use anyhow::{bail, Result};
use atelier::{commands, telemetry};
use atelier_app::command_storage::{
    canonical_mutation_db, command_storage, degraded_projection_query_db, existing_projection_db,
    lint_db, projection_query_db, state_and_db_paths, CommandStorageAccess,
};
use atelier_app::use_cases;
use atelier_core::IssuePriority;
use atelier_records::RecordStore;
use atelier_sqlite::Database;
use chrono::Utc;
use clap::{Parser, Subcommand};
use std::env;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "atelier")]
#[command(about = "Mission and proof oriented work coordination for agents")]
#[command(help_template = "{about-section}\nUsage: {usage}\n\n{after-help}\nOptions:\n{options}")]
#[command(after_help = "Setup:
  init          Initialize Atelier in the current repository

Orientation:
  man           Show role-specific operating guidance
  status        Show checkout, mission, work, and tracker signposts
  start         Start tracked work on an issue

Issues:
  issue         Create, list, show, update, close, and manage blockers
  search        Search issue text
  graph         Inspect mission and issue hierarchy and impact

Missions and planning:
  mission       Create, list, show, status, close, and update durable missions
  bundle        Preview and apply one-shot graph bundle files

Records:
  evidence      Capture validation evidence
  session       Inspect derived issue attempts
  pr            Manage Forgejo pull request review artifacts
  forgejo       Configure and verify Forgejo integration
  history       Inspect canonical repo, mission, issue, or epic activity

Advanced work:
  worktree      Create, inspect, merge, and remove mission or issue worktrees
  branch        Inspect and repair epic review branches

Maintenance:
  maintenance   Run explicit destructive maintenance commands
  lint          Validate tracker records
  doctor        Check runtime and derived-state health; use --fix for local repair

Common commands:
  atelier man
  atelier man worker
  atelier man reviewer
  atelier man validator
  atelier man manager
  atelier man admin
  atelier status
  atelier issue list
  atelier issue list --ready
  atelier issue list --blocked
  atelier issue show <id>
  atelier issue block <blocked-id> <blocker-id>
  atelier issue unblock <blocked-id> <blocker-id>
  atelier issue blocked [<id>]
  atelier mission list
  atelier mission show <id>
  atelier mission status
  atelier mission close <id> --reason \"...\"
  atelier session list --active
  atelier forgejo roles check
  atelier history --mission <id>
  atelier history --issue <id>
  atelier start <issue-id>
  atelier issue transition <issue-id> --options
  atelier issue close <issue-id> --reason \"...\"
  atelier doctor
  atelier doctor --fix
  atelier help <command>
")]
#[command(version = option_env!("ATELIER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")))]
struct Cli {
    /// Quiet mode: only output essential data (IDs, counts)
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Log level for diagnostic output (error, warn, info, debug, trace)
    #[arg(long, global = true, default_value = "warn", env = "ATELIER_LOG")]
    log_level: String,

    /// Log format (text, json)
    #[arg(
        long,
        global = true,
        default_value = "text",
        env = "ATELIER_LOG_FORMAT"
    )]
    log_format: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize atelier in the current directory
    Init {
        /// Reconcile core tracker state even if already initialized
        #[arg(short, long)]
        force: bool,
        /// Import the repo-local Beads migration file at .beads/issues.manual.jsonl
        #[arg(long)]
        import_beads: bool,
    },

    /// Show role-specific operating guidance
    Man {
        /// Role guide to print: worker, reviewer, validator, manager, or admin
        role: Option<String>,
    },

    /// Show checkout, mission, work, and tracker signposts
    Status,

    /// Start tracked work on an issue
    Start {
        id: String,
        /// Do not emit legacy session output for this start
        #[arg(long, hide = true)]
        no_session: bool,
        /// Legacy option retained only to reject stale scripts with a direct error
        #[arg(long, hide = true)]
        reuse_session: Option<String>,
    },

    /// Issue lifecycle commands (create, show, list, close, ...)
    Issue {
        #[command(subcommand)]
        action: IssueCommands,
    },

    /// Search issue text
    Search {
        /// Search query
        query: String,
    },

    /// Mission and issue graph commands
    Graph {
        #[command(subcommand)]
        action: GraphCommands,
    },

    /// Advanced deterministic-renderer diagnostic; normal health uses lint and doctor
    #[command(hide = true)]
    Export {
        /// State directory for canonical export diagnostics
        #[arg(short, long)]
        output: Option<String>,
        /// Check deterministic renderer/projection freshness without writing tracked records
        #[arg(long)]
        check: bool,
    },

    /// Advanced projection diagnostic; normal local repair uses doctor --fix
    #[command(hide = true)]
    Rebuild {
        /// Canonical state directory to rebuild from
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Import Beads JSONL backup into Atelier runtime and canonical state
    #[command(hide = true)]
    ImportBeads {
        /// Beads JSONL backup path from an external source
        input: String,
        /// Canonical state directory to write after import
        #[arg(short, long)]
        output: Option<String>,
    },

    /// First-class mission records
    Mission {
        #[command(subcommand)]
        action: MissionCommands,
    },

    /// One-shot graph bundle files
    Bundle {
        #[command(subcommand)]
        action: BundleCommands,
    },

    /// First-class evidence records
    Evidence {
        #[command(subcommand)]
        action: EvidenceCommands,
    },

    /// Inspect derived issue-scoped worker, reviewer, and validator attempts
    Session {
        #[command(subcommand)]
        action: SessionCommands,
    },

    /// Forgejo pull request review artifacts
    Pr {
        #[command(subcommand)]
        action: PrCommands,
    },

    /// Configure and verify Forgejo integration
    Forgejo {
        #[command(subcommand)]
        action: ForgejoCommands,
    },

    /// Inspect canonical repo, mission, issue, or epic activity
    History {
        /// Scope to one mission and linked work
        #[arg(long)]
        mission: Option<String>,
        /// Scope to one issue
        #[arg(long)]
        issue: Option<String>,
        /// Scope to one epic and its descendants
        #[arg(long)]
        epic: Option<String>,
        /// Include subissues when using --issue
        #[arg(long)]
        include_descendants: bool,
        /// Filter by event kind, such as note or evidence_attached
        #[arg(long)]
        event_kind: Option<String>,
        /// Filter by actor exactly as recorded
        #[arg(long)]
        actor: Option<String>,
        /// Filter to events since a duration like 7d, a YYYY-MM-DD date, or RFC3339
        #[arg(long)]
        since: Option<String>,
        /// Maximum number of matching events to print
        #[arg(long, default_value_t = commands::history::DEFAULT_LIMIT)]
        limit: usize,
    },

    /// Advanced/debug workflow policy diagnostics
    #[command(hide = true)]
    Workflow {
        #[command(subcommand)]
        action: WorkflowCommands,
    },

    /// Git worktree helpers for tracked work
    Worktree {
        #[command(subcommand)]
        action: WorktreeCommands,
    },

    /// Git branch helpers for epic review branches
    Branch {
        #[command(subcommand)]
        action: BranchCommands,
    },

    /// Advanced local command diagnostics; JSON is local-only telemetry, not workflow state
    #[command(hide = true)]
    Diagnostics {
        #[command(subcommand)]
        action: DiagnosticsCommands,
    },

    /// Destructive maintenance commands
    Maintenance {
        #[command(subcommand)]
        action: MaintenanceCommands,
    },

    /// Validate tracker records
    Lint {
        /// Optional issue ID or imported source ID
        id: Option<String>,
    },

    /// Check tracker runtime and derived-state health
    Doctor {
        /// Repair ignored local runtime/cache/projection state; never edits tracked canonical records
        #[arg(long)]
        fix: bool,
    },
}

// ============================================================================
// Issue subcommands (canonical path: `atelier issue <command>`)
// ============================================================================

#[derive(Subcommand)]
enum IssueCommands {
    /// Create a new issue
    Create {
        /// Issue title
        title: String,
        /// Priority (low, medium, high, critical)
        #[arg(short, long, default_value = "medium")]
        priority: String,
        /// Work type/body preset (bug, feature, refactor, research)
        #[arg(short, long)]
        template: Option<String>,
        /// Add labels to the issue
        #[arg(short, long)]
        label: Vec<String>,
        /// Explicit work type (bug, epic, feature, spike, task, validation)
        #[arg(long)]
        issue_type: Option<String>,
        /// Parent issue ID or imported source ID
        #[arg(long)]
        parent: Option<String>,
    },

    /// List issues
    List {
        /// Filter by exact workflow status, or all
        #[arg(short, long, default_value = "todo")]
        status: String,
        /// Filter by derived workflow category
        #[arg(long)]
        category: Option<String>,
        /// Filter by label
        #[arg(short, long)]
        label: Option<String>,
        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,
        /// Show only ready work
        #[arg(long)]
        ready: bool,
        /// Show only blocked work
        #[arg(long)]
        blocked: bool,
    },

    /// Show issue details
    Show {
        /// Issue ID
        id: String,
    },

    /// Show issue transition options and blockers
    Transition {
        /// Issue ID
        id: String,
        /// Transition name to execute
        transition: Option<String>,
        /// Show the full option list
        #[arg(long)]
        options: bool,
        /// Close reason used by transitions that require it
        #[arg(long = "reason")]
        close_reason: Option<String>,
    },

    /// Update an issue
    Update {
        /// Issue ID
        id: String,
        /// New title
        #[arg(short, long)]
        title: Option<String>,
        /// New priority
        #[arg(short, long)]
        priority: Option<String>,
        /// New issue type (bug, epic, feature, spike, task, validation)
        #[arg(long)]
        issue_type: Option<String>,
        /// Add labels to the issue
        #[arg(short, long)]
        label: Vec<String>,
        /// Remove labels from the issue
        #[arg(long = "remove-label")]
        remove_label: Vec<String>,
        /// Set parent issue ID or imported source ID
        #[arg(long)]
        parent: Option<String>,
        /// Clear parent issue
        #[arg(long)]
        no_parent: bool,
    },

    /// Add an activity note to an issue
    Note {
        /// Issue ID
        id: String,
        /// Note text
        text: String,
        /// Note kind (note, plan, observation, blocker, resolution, result, handoff, human)
        #[arg(long, default_value = "note")]
        kind: String,
    },

    /// Close an issue
    Close {
        /// Issue ID
        id: String,
        /// Explicit terminal workflow status when multiple done targets are available
        #[arg(long)]
        to: Option<String>,
        /// Close reason recorded in issue activity
        #[arg(short, long)]
        reason: String,
    },

    /// Mark an issue as blocked by another
    Block {
        /// Issue ID that is blocked
        id: String,
        /// Issue ID that is blocking
        blocker: String,
    },

    /// Remove a blocking relationship
    Unblock {
        /// Issue ID that was blocked
        id: String,
        /// Issue ID that was blocking
        blocker: String,
    },

    /// List blocked issues, or show blockers for one issue
    Blocked {
        /// Issue ID to inspect instead of the blocked-work queue
        id: Option<String>,
    },
}

#[derive(Subcommand)]
enum GraphCommands {
    /// Show downstream impact across mission work, hierarchy, and impact-bearing links
    Impact {
        /// Mission or issue ID
        id: String,
    },
    /// Show missions and issues as a tree hierarchy
    Tree {
        /// Filter by status (todo, done, all)
        #[arg(short, long, default_value = "all")]
        status: String,
        /// Show a bounded, scan-friendly hierarchy instead of the full tree
        #[arg(long)]
        compact: bool,
    },
}

#[derive(Subcommand)]
enum MaintenanceCommands {
    /// Delete a record with an explicit target kind
    Delete {
        target_kind: String,
        target_id: String,
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum MissionCommands {
    /// Create a mission with generated Intent, Constraints, Risks, and Validation sections
    Create {
        title: String,
        /// Intent section text; this does not replace the full mission Markdown body
        #[arg(short, long)]
        body: Option<String>,
        /// Add one Constraints section bullet; repeat for multiple constraints
        #[arg(long)]
        constraint: Vec<String>,
        /// Add one Risks section bullet; repeat for multiple risks
        #[arg(long)]
        risk: Vec<String>,
        /// Add one Validation section bullet; repeat for multiple validation criteria
        #[arg(long)]
        validation: Vec<String>,
    },
    /// Show a mission with linked work, blockers, and evidence
    Show { id: String },
    /// Focus a mission as the active orchestration context
    Start {
        id: String,
        /// Replace any currently active mission focus
        #[arg(long = "switch")]
        switch_active: bool,
    },
    /// Show mission-control status for one mission or all current missions
    Status {
        /// Show verbose validator detail in the status summary
        #[arg(long)]
        verbose: bool,
        id: Option<String>,
    },
    /// Close a mission after terminal checks pass
    Close {
        id: String,
        /// Mission close reason recorded in the mission terminal notes
        #[arg(long)]
        reason: String,
    },
    /// List missions
    List {
        /// Filter missions by status (default: current; use all to include closed/history)
        #[arg(short, long)]
        status: Option<String>,
    },
    /// Update mission fields
    Update {
        id: String,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        status: Option<String>,
        #[arg(short, long)]
        body: Option<String>,
        #[arg(long)]
        constraint: Vec<String>,
        #[arg(long)]
        risk: Vec<String>,
        #[arg(long)]
        validation: Vec<String>,
    },
    /// Add an activity note to a mission
    Note {
        id: String,
        text: String,
        /// Note kind (note, plan, observation, blocker, resolution, result, handoff, human)
        #[arg(long, default_value = "note")]
        kind: String,
    },
    /// Add issue work to a mission
    AddWork { id: String, issue: String },
    /// Remove issue work from a mission
    Unlink { id: String, issue: String },
    /// Add an issue blocker to a mission
    AddBlocker { id: String, issue: String },
}

#[derive(Subcommand)]
enum BundleCommands {
    /// Preview an authored bundle JSON file without mutating tracker state
    Preview { input: String },
    /// Apply an authored bundle JSON file
    Apply {
        input: String,
        #[arg(long)]
        yes: bool,
    },
}

#[derive(Subcommand)]
enum EvidenceCommands {
    /// Record proof manually or by capturing a command transcript
    #[command(after_help = "Examples:
  atelier evidence record --target issue/<id> --kind validation \"summary\"
  atelier evidence record --target issue/<id> --kind test -- <command>

Use `evidence attach` only when you need to reuse an existing evidence record on
another target.")]
    Record {
        #[arg(long = "kind")]
        evidence_kind: String,
        /// Accountable target using kind/id syntax, for example issue/atelier-1234
        #[arg(long)]
        target: Option<String>,
        #[arg(long, default_value = "validates")]
        role: String,
        #[arg(long)]
        summary: Option<String>,
        #[arg(long)]
        path: Option<String>,
        #[arg(long)]
        uri: Option<String>,
        #[arg(long)]
        producer: Option<String>,
        /// Manual evidence summary. Command-backed evidence should pass commands after `--`.
        summary_text: Option<String>,
        #[arg(last = true, num_args = 0..)]
        command: Vec<String>,
    },
    /// Show an evidence record
    Show { id: String },
    /// Reuse an existing evidence record on another target
    Attach {
        id: String,
        target_kind: String,
        target_id: String,
        #[arg(long, default_value = "validates")]
        role: String,
    },
    /// List evidence records
    List {
        #[arg(long)]
        status: Option<String>,
    },
}

#[derive(Subcommand)]
enum SessionCommands {
    /// Show a derived issue attempt
    Show { id: String },
    /// List derived issue attempts
    List {
        /// Show only active attempts
        #[arg(long)]
        active: bool,
    },
}

#[derive(Subcommand)]
enum PrCommands {
    /// Open or confirm the active Forgejo pull request for an issue owner
    Open {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        role: String,
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "")]
        body: String,
        #[arg(long)]
        source_branch: String,
        #[arg(long, default_value = "master")]
        target_branch: String,
    },
    /// Link an existing Forgejo PR by number or URL
    Link {
        #[arg(long)]
        issue: Option<String>,
        pull_request: String,
    },
    /// Show concise linked PR status
    Status {
        #[arg(long)]
        issue: Option<String>,
    },
    /// Show linked PR details
    Show {
        #[arg(long)]
        issue: Option<String>,
    },
    /// Merge or confirm the linked Forgejo PR without changing Atelier workflow state
    Merge {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        role: String,
    },
    /// List live PR comments and review comments
    Comments {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        unresolved: bool,
    },
    /// Add a Forgejo PR comment
    Comment {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        role: String,
        body: String,
    },
    /// Submit a Forgejo PR review
    Review {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        role: String,
        #[arg(long)]
        event: String,
        #[arg(long, default_value = "")]
        body: String,
    },
}

#[derive(Subcommand)]
enum ForgejoCommands {
    /// Provision and verify Forgejo role author accounts
    Roles {
        #[command(subcommand)]
        action: ForgejoRolesCommands,
    },
}

#[derive(Subcommand)]
enum ForgejoRolesCommands {
    /// Verify configured role author users, repo permissions, and sudo access
    Check,
    /// Create missing role author users and grant repository access
    Provision {
        /// Persist the role author mapping in .atelier/config.toml
        #[arg(long)]
        write_config: bool,
    },
}

#[derive(Subcommand)]
enum WorkflowCommands {
    /// Run raw workflow-policy diagnostics; normal operator checks use lint and status surfaces
    Check,
}

#[derive(Subcommand)]
enum WorktreeCommands {
    /// Create or locate a worktree for a mission
    ForMission {
        id: String,
        #[arg(long)]
        path: Option<String>,
    },
    /// Create or locate a worktree for an issue
    For {
        id: String,
        #[arg(long)]
        path: Option<String>,
    },
    /// Show scan-friendly worktree status
    Status,
    /// Merge the associated work branch into the current branch
    Merge { id: String },
    /// Remove the associated worktree
    Remove {
        id: String,
        #[arg(long)]
        force: bool,
    },
    /// Clear a stale local worktree association after interrupted setup/removal
    Repair { id: String },
}

#[derive(Subcommand)]
enum BranchCommands {
    /// Create or switch to the review branch for an epic
    ForEpic { id: String },
    /// Show local epic review branches
    Status,
    /// Merge the review branch for an epic into the current branch
    Merge { id: String },
}

#[derive(Subcommand)]
enum DiagnosticsCommands {
    /// Summarize slow command telemetry as stable local-only JSON for performance analysis
    Slow {
        /// Time window in UTC days, where 0 means today only
        #[arg(long, default_value_t = 7)]
        days: u64,
        /// Minimum duration in milliseconds
        #[arg(long, default_value_t = 1000)]
        threshold_ms: u64,
    },
}

// ============================================================================
// Helpers
// ============================================================================

fn issue_create_parts(
    priority: &str,
    template: Option<&str>,
    labels: &[String],
    issue_type: Option<&str>,
) -> Result<(String, Option<String>, Vec<String>, String)> {
    let mut labels = labels.to_vec();
    let (final_priority, final_description, template_issue_type) =
        if let Some(template_name) = template {
            let template = commands::create::get_template(template_name).ok_or_else(|| {
                anyhow::anyhow!(
                    "Unknown template '{}'. Available: {}",
                    template_name,
                    commands::create::list_templates().join(", ")
                )
            })?;
            if !labels.iter().any(|label| label == template.label) {
                labels.push(template.label.to_string());
            }
            let priority = if priority != "medium" {
                priority
            } else {
                template.priority
            };
            (
                priority.to_string(),
                template.description_prefix.map(str::to_string),
                Some(template_default_issue_type(template_name)),
            )
        } else {
            (priority.to_string(), None, None)
        };

    IssuePriority::from_cli_input(&final_priority)?;
    let final_issue_type = match (issue_type, template_issue_type) {
        (Some(explicit), Some(default)) if explicit != default => {
            bail!(
                "Conflicting work type options: --issue-type {explicit} does not match --template {} (default type {default}). Choose one work type or use a matching template.",
                template.unwrap_or("(none)")
            );
        }
        (Some(explicit), _) => explicit.to_string(),
        (None, Some(default)) => default.to_string(),
        (None, None) => "task".to_string(),
    };
    Ok((final_priority, final_description, labels, final_issue_type))
}

fn template_default_issue_type(template: &str) -> &'static str {
    match template {
        "bug" => "bug",
        "feature" => "feature",
        "research" | "investigation" => "spike",
        "audit" => "validation",
        _ => "task",
    }
}

fn resolve_issue_arg(db: &Database, issue_ref: &str) -> Result<String> {
    match commands::agent_factory::resolve_id(db, issue_ref) {
        Ok(id) => Ok(id),
        Err(error) => match db.record_kind_for_id(issue_ref)? {
            Some(actual_kind) if actual_kind != "issue" => {
                bail!("{}", wrong_kind_message("issue", &actual_kind, issue_ref));
            }
            _ => Err(error),
        },
    }
}

fn resolve_record_arg(db: &Database, kind: &str, id: &str) -> Result<String> {
    if kind == "issue" {
        resolve_issue_arg(db, id)
    } else if db.get_record(kind, id)?.is_some() {
        Ok(id.to_string())
    } else if let Some(actual_kind) = db.record_kind_for_id(id)? {
        bail!("{}", wrong_kind_message(kind, &actual_kind, id));
    } else {
        Ok(id.to_string())
    }
}

fn resolve_graph_record_arg(db: &Database, id: &str) -> Result<(String, String)> {
    match commands::agent_factory::resolve_id(db, id) {
        Ok(issue_id) => Ok(("issue".to_string(), issue_id)),
        Err(issue_error) => match db.record_kind_for_id(id)? {
            Some(kind) if kind == "mission" => Ok((kind, id.to_string())),
            Some(kind) => bail!(
                "{id} is a {kind} record; `atelier graph impact` supports mission and issue records."
            ),
            None => Err(issue_error),
        },
    }
}

fn wrong_kind_message(expected_kind: &str, actual_kind: &str, id: &str) -> String {
    let suggested = show_command_for_kind(actual_kind)
        .map(|command| format!(" Use `{command} {id}`."))
        .unwrap_or_default();
    format!("{id} is a {actual_kind} record, not a {expected_kind} record.{suggested}")
}

fn show_command_for_kind(kind: &str) -> Option<&'static str> {
    match kind {
        "issue" | "epic" => Some("atelier issue show"),
        "mission" => Some("atelier mission show"),
        "evidence" => Some("atelier evidence show"),
        _ => None,
    }
}

fn require_issue_kind(kind: &str, command: &str) -> Result<()> {
    if kind != "issue" {
        bail!("{command} currently supports issue records only; got '{kind}'");
    }
    Ok(())
}

fn init_tracing(log_level: &str, log_format: &str) {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
    let filter = EnvFilter::try_new(log_level).unwrap_or_else(|_| EnvFilter::new("warn"));
    if log_format == "json" {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().json().with_writer(std::io::stderr))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().with_target(false).with_writer(std::io::stderr))
            .init();
    }
}

// ============================================================================
// Dispatch helpers for canonical subcommands
// ============================================================================

fn dispatch_issue(action: IssueCommands, quiet: bool) -> Result<()> {
    match action {
        IssueCommands::Create {
            title,
            priority,
            template,
            label,
            issue_type,
            parent,
        } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            let (final_priority, final_description, labels, issue_type) = issue_create_parts(
                &priority,
                template.as_deref(),
                &label,
                issue_type.as_deref(),
            )?;
            commands::agent_factory::create_lifecycle(
                &state_dir,
                &db_path,
                commands::agent_factory::LifecycleCreateInput {
                    title: &title,
                    description: final_description.as_deref(),
                    priority: &final_priority,
                    issue_type: &issue_type,
                    labels: &labels,
                    parent: parent.as_deref(),
                    quiet,
                },
            )
        }

        IssueCommands::List {
            status,
            category,
            label,
            priority,
            ready,
            blocked,
        } => {
            let db = degraded_projection_query_db()?;
            if blocked {
                if ready {
                    bail!("--blocked cannot be combined with --ready");
                }
                if status != "todo" || category.is_some() || label.is_some() || priority.is_some() {
                    bail!("--blocked cannot be combined with --status, --category, --label, or --priority");
                }
                commands::deps::list_blocked(&db, quiet)
            } else {
                commands::agent_factory::list(
                    &db,
                    Some(&status),
                    category.as_deref(),
                    label.as_deref(),
                    priority.as_deref(),
                    ready,
                    quiet,
                )
            }
        }

        IssueCommands::Show { id } => {
            let db = degraded_projection_query_db()?;
            commands::agent_factory::show(&db, &id)
        }

        IssueCommands::Transition {
            id,
            transition,
            options,
            close_reason,
        } => {
            if options {
                if transition.is_some() {
                    bail!("--options cannot be combined with a transition name");
                }
                let db = degraded_projection_query_db()?;
                commands::agent_factory::transition_options(&db, &id)
            } else {
                let transition = transition.ok_or_else(|| {
                    anyhow::anyhow!(
                        "Specify a transition name or rerun with `atelier issue transition {} --options`",
                        id
                    )
                })?;
                let (state_dir, db_path) = state_and_db_paths()?;
                let db = canonical_mutation_db()?;
                commands::workflow::transition_issue(
                    &db,
                    &state_dir,
                    &db_path,
                    &id,
                    &transition,
                    close_reason.as_deref(),
                )
            }
        }

        IssueCommands::Update {
            id,
            title,
            priority,
            issue_type,
            label,
            remove_label,
            parent,
            no_parent,
        } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            commands::agent_factory::update_lifecycle(
                &state_dir,
                &db_path,
                commands::agent_factory::UpdateInput {
                    issue_ref: &id,
                    title: title.as_deref(),
                    priority: priority.as_deref(),
                    issue_type: issue_type.as_deref(),
                    labels: &label,
                    remove_labels: &remove_label,
                    parent: if no_parent {
                        Some(None)
                    } else {
                        parent.as_deref().map(Some)
                    },
                    append_notes: None,
                },
            )
        }

        IssueCommands::Note { id, text, kind } => {
            let db = canonical_mutation_db()?;
            let id = resolve_issue_arg(&db, &id)?;
            commands::comment::run_issue_note(&db, &id, &text, &kind)
        }

        IssueCommands::Close { id, to, reason } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            let _ = quiet;
            commands::agent_factory::close_lifecycle(
                &state_dir,
                &db_path,
                &id,
                &reason,
                to.as_deref(),
            )
        }

        IssueCommands::Block { id, blocker } => {
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            commands::agent_factory::dep_add_canonical(&db, &store, &id, &blocker)?;
            drop(db);
            atelier_app::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        IssueCommands::Unblock { id, blocker } => {
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            commands::agent_factory::dep_remove_canonical(&db, &store, &id, &blocker)?;
            drop(db);
            atelier_app::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        IssueCommands::Blocked { id } => {
            let db = projection_query_db()?;
            if let Some(id) = id {
                commands::agent_factory::dep_list(&db, Some(&id))
            } else {
                commands::deps::list_blocked(&db, quiet)
            }
        }
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() -> Result<()> {
    load_dotenv()?;
    run()
}

fn load_dotenv() -> Result<()> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

fn run() -> Result<()> {
    let cli = parse_cli_or_exit();
    init_tracing(&cli.log_level, &cli.log_format);
    let quiet = cli.quiet;
    let command_name = command_identity(&cli.command);
    let started_at = Utc::now();
    let started = Instant::now();

    let result = match cli.command {
        Commands::Init {
            force,
            import_beads,
        } => {
            let cwd = env::current_dir()?;
            commands::init::run(&cwd, force, import_beads)
        }

        Commands::Man { role } => commands::man::run(role),

        Commands::Status => {
            let storage = use_cases::status_storage()?;
            commands::status::run(storage.db(), &storage.state_dir(), quiet)
        }

        Commands::Start {
            id,
            no_session,
            reuse_session,
        } => {
            let db = projection_query_db()?;
            let id = resolve_issue_arg(&db, &id)?;
            let (state_dir, db_path) = state_and_db_paths()?;
            commands::work::start_lifecycle(
                &state_dir,
                &db_path,
                &id,
                commands::work::StartSessionOptions {
                    no_session,
                    reuse_session: reuse_session.as_deref(),
                },
            )
        }

        Commands::Issue { action } => dispatch_issue(action, quiet),

        Commands::Search { query } => {
            let db = degraded_projection_query_db()?;
            commands::agent_factory::search(&db, &query, quiet)
        }

        Commands::Graph { action } => match action {
            GraphCommands::Impact { id } => {
                let db = projection_query_db()?;
                let (kind, id) = resolve_graph_record_arg(&db, &id)?;
                commands::relate::impact(&db, &kind, &id)
            }
            GraphCommands::Tree { status, compact } => {
                let db = projection_query_db()?;
                if compact {
                    commands::tree::run_compact(&db, Some(&status))
                } else {
                    commands::tree::run(&db, Some(&status))
                }
            }
        },

        Commands::Export { output, check } => {
            let storage = command_storage(CommandStorageAccess::HealthRepair)?;
            let state_dir = output
                .as_deref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| storage.state_dir());
            commands::agent_factory::export_canonical(storage.db(), &state_dir, check)
        }

        Commands::Rebuild { input } => {
            let storage = command_storage(CommandStorageAccess::HealthRepair)?;
            let state_dir = input
                .as_deref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| storage.state_dir());
            let db_path = storage.db_path();
            commands::agent_factory::rebuild(&state_dir, &db_path)
        }

        Commands::ImportBeads { input, output } => {
            let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
            let state_dir = output
                .as_deref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| storage.state_dir());
            commands::import::run_beads_jsonl(
                storage.db(),
                std::path::Path::new(&input),
                &state_dir,
            )
        }

        Commands::Mission { action } => match action {
            MissionCommands::Create {
                title,
                body,
                constraint,
                risk,
                validation,
            } => {
                let storage = use_cases::mission_mutation_storage()?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                commands::mission::create(
                    &state_dir,
                    &db_path,
                    &title,
                    body.as_deref(),
                    constraint,
                    risk,
                    validation,
                )
            }
            MissionCommands::Show { id } => {
                let storage = use_cases::mission_query_storage()?;
                let db = storage.db();
                let id = use_cases::resolve_record_ref(&storage, "mission", &id)?;
                commands::mission::show(db, &id)
            }
            MissionCommands::Start { id, switch_active } => {
                let storage = use_cases::mission_mutation_storage()?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = use_cases::resolve_record_ref(&storage, "mission", &id)?;
                commands::mission::start(&state_dir, &db_path, &id, switch_active)
            }
            MissionCommands::Status { id, verbose } => {
                let storage = use_cases::mission_query_storage()?;
                let id = use_cases::resolve_optional_record_ref(&storage, "mission", id)?;
                commands::mission::status(
                    storage.db(),
                    &storage.state_dir(),
                    id.as_deref(),
                    quiet,
                    verbose,
                )
            }
            MissionCommands::Close { id, reason } => {
                let storage = use_cases::mission_mutation_storage()?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = use_cases::resolve_record_ref(&storage, "mission", &id)?;
                commands::mission::close(&state_dir, &db_path, &id, &reason)
            }
            MissionCommands::List { status } => {
                let storage = use_cases::mission_query_storage()?;
                let db = storage.db();
                commands::mission::list(&db, status.as_deref())
            }
            MissionCommands::Update {
                id,
                title,
                status,
                body,
                constraint,
                risk,
                validation,
            } => {
                let storage = use_cases::mission_mutation_storage()?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = use_cases::resolve_record_ref(&storage, "mission", &id)?;
                commands::mission::update(
                    &state_dir,
                    &db_path,
                    &id,
                    title.as_deref(),
                    status.as_deref(),
                    body.as_deref(),
                    constraint,
                    risk,
                    validation,
                )
            }
            MissionCommands::Note { id, text, kind } => {
                let storage = use_cases::mission_mutation_storage()?;
                let id = use_cases::resolve_record_ref(&storage, "mission", &id)?;
                commands::comment::run_mission_note(storage.db(), &id, &text, &kind)
            }
            MissionCommands::AddWork { id, issue } => {
                let storage = use_cases::mission_mutation_storage()?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = use_cases::resolve_record_ref(&storage, "mission", &id)?;
                let issue = use_cases::resolve_issue_ref(&storage, &issue)?;
                commands::mission::add_work(&state_dir, &db_path, &id, &issue)
            }
            MissionCommands::Unlink { id, issue } => {
                let storage = use_cases::mission_mutation_storage()?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = use_cases::resolve_record_ref(&storage, "mission", &id)?;
                let issue = use_cases::resolve_issue_ref(&storage, &issue)?;
                commands::mission::unlink(&state_dir, &db_path, &id, &issue)
            }
            MissionCommands::AddBlocker { id, issue } => {
                let storage = use_cases::mission_mutation_storage()?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = use_cases::resolve_record_ref(&storage, "mission", &id)?;
                let issue = use_cases::resolve_issue_ref(&storage, &issue)?;
                commands::mission::add_blocker(&state_dir, &db_path, &id, &issue)
            }
        },

        Commands::Bundle { action } => match action {
            BundleCommands::Preview { input } => {
                let storage = command_storage(CommandStorageAccess::ProjectionQuery)?;
                commands::bundle::preview(storage.db(), &input)
            }
            BundleCommands::Apply { input, yes } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                commands::bundle::apply(
                    storage.db(),
                    &storage.state_dir(),
                    &storage.db_path(),
                    &input,
                    yes,
                )
            }
        },

        Commands::Evidence { action } => match action {
            EvidenceCommands::Record {
                evidence_kind,
                target,
                role,
                summary,
                path,
                uri,
                producer,
                summary_text,
                command,
            } => {
                let storage = use_cases::evidence_mutation_storage()?;
                let parsed_target = match target.as_deref() {
                    Some(target) => {
                        let target = use_cases::parse_evidence_target_arg(target)?;
                        let id = use_cases::resolve_evidence_target_ref(
                            &storage,
                            &target.kind,
                            &target.id,
                        )?;
                        Some((target.kind, id))
                    }
                    None => None,
                };
                if command.is_empty() {
                    let summary = match (summary.as_deref(), summary_text.as_deref()) {
                        (Some(_), Some(_)) => {
                            bail!("use either --summary or a positional summary, not both")
                        }
                        (Some(summary), None) | (None, Some(summary)) => summary,
                        (None, None) => {
                            bail!("evidence record without a command requires a summary")
                        }
                    };
                    let evidence_id = commands::evidence::add_returning_id(
                        &storage.state_dir(),
                        &storage.db_path(),
                        &evidence_kind,
                        summary,
                        path.as_deref(),
                        uri.as_deref(),
                        producer.as_deref(),
                        parsed_target.as_ref().map(|(kind, id)| {
                            commands::evidence::TargetMetadata {
                                kind,
                                id,
                                role: &role,
                            }
                        }),
                    )?;
                    if let Some((kind, id)) = parsed_target {
                        commands::evidence::attach(
                            &storage.state_dir(),
                            &storage.db_path(),
                            &evidence_id,
                            &kind,
                            &id,
                            &role,
                        )?;
                    }
                    let db = use_cases::refreshed_mutation_db(&storage)?;
                    commands::evidence::show(&db, &evidence_id)
                } else {
                    let command_summary = match (summary.as_deref(), summary_text.as_deref()) {
                        (Some(_), Some(_)) => {
                            bail!("use either --summary or a positional summary, not both")
                        }
                        (Some(summary), None) | (None, Some(summary)) => Some(summary),
                        (None, None) => None,
                    };
                    commands::evidence::capture(
                        &storage.state_dir(),
                        &storage.db_path(),
                        commands::evidence::CaptureOptions {
                            evidence_kind: &evidence_kind,
                            summary: command_summary,
                            path: path.as_deref(),
                            uri: uri.as_deref(),
                            producer: producer.as_deref(),
                            target_kind: parsed_target.as_ref().map(|(kind, _)| kind.as_str()),
                            target_id: parsed_target.as_ref().map(|(_, id)| id.as_str()),
                            role: &role,
                            command: &command,
                        },
                    )
                }
            }
            EvidenceCommands::Show { id } => {
                let storage = use_cases::evidence_query_storage()?;
                let db = storage.db();
                commands::evidence::show(&db, &id)
            }
            EvidenceCommands::Attach {
                id,
                target_kind,
                target_id,
                role,
            } => {
                let storage = use_cases::evidence_mutation_storage()?;
                let target_id =
                    use_cases::resolve_evidence_target_ref(&storage, &target_kind, &target_id)?;
                commands::evidence::attach(
                    &storage.state_dir(),
                    &storage.db_path(),
                    &id,
                    &target_kind,
                    &target_id,
                    &role,
                )
            }
            EvidenceCommands::List { status } => {
                let storage = use_cases::evidence_query_storage()?;
                let db = storage.db();
                commands::evidence::list(&db, status.as_deref())
            }
        },

        Commands::Session { action } => match action {
            SessionCommands::Show { id } => {
                let storage = use_cases::mission_query_storage()?;
                commands::session::show(&storage.state_dir(), &id)
            }
            SessionCommands::List { active } => {
                let storage = use_cases::mission_query_storage()?;
                commands::session::list(&storage.state_dir(), active)
            }
        },

        Commands::Pr { action } => {
            let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
            match action {
                PrCommands::Open {
                    issue,
                    role,
                    title,
                    body,
                    source_branch,
                    target_branch,
                } => commands::pr::open(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    &storage.db_path(),
                    issue.as_deref(),
                    &role,
                    &title,
                    &body,
                    &source_branch,
                    &target_branch,
                ),
                PrCommands::Link {
                    issue,
                    pull_request,
                } => commands::pr::link(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    &storage.db_path(),
                    issue.as_deref(),
                    &pull_request,
                ),
                PrCommands::Status { issue } => commands::pr::status(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    issue.as_deref(),
                ),
                PrCommands::Show { issue } => commands::pr::show(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    issue.as_deref(),
                ),
                PrCommands::Merge { issue, role } => commands::pr::merge(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    &storage.db_path(),
                    issue.as_deref(),
                    &role,
                ),
                PrCommands::Comments { issue, unresolved } => commands::pr::comments(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    issue.as_deref(),
                    unresolved,
                ),
                PrCommands::Comment { issue, role, body } => commands::pr::comment(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    issue.as_deref(),
                    &role,
                    &body,
                ),
                PrCommands::Review {
                    issue,
                    role,
                    event,
                    body,
                } => commands::pr::review(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    issue.as_deref(),
                    &role,
                    &event,
                    &body,
                ),
            }
        }

        Commands::Forgejo { action } => {
            let repo_root = atelier_app::storage_layout::find_repo_root()?;
            match action {
                ForgejoCommands::Roles { action } => match action {
                    ForgejoRolesCommands::Check => commands::forgejo::roles_check(&repo_root),
                    ForgejoRolesCommands::Provision { write_config } => {
                        commands::forgejo::roles_provision(&repo_root, write_config)
                    }
                },
            }
        }

        Commands::History {
            mission,
            issue,
            epic,
            include_descendants,
            event_kind,
            actor,
            since,
            limit,
        } => {
            let storage = command_storage(CommandStorageAccess::ProjectionQuery)?;
            let mission = mission
                .as_deref()
                .map(|id| resolve_record_arg(storage.db(), "mission", id))
                .transpose()?;
            let issue = issue
                .as_deref()
                .map(|id| resolve_issue_arg(storage.db(), id))
                .transpose()?;
            let epic = epic
                .as_deref()
                .map(|id| resolve_issue_arg(storage.db(), id))
                .transpose()?;
            commands::history::run(
                storage.db(),
                &storage.state_dir(),
                commands::history::HistoryOptions {
                    mission,
                    issue,
                    epic,
                    include_descendants,
                    event_kind,
                    actor,
                    since,
                    limit,
                },
            )
        }

        Commands::Workflow { action } => match action {
            WorkflowCommands::Check => {
                let storage = use_cases::workflow_query_storage()?;
                let db = storage.db();
                commands::workflow::check(&db)
            }
        },

        Commands::Worktree { action } => {
            let db = existing_projection_db()?;
            match action {
                WorktreeCommands::ForMission { id, path } => {
                    commands::work::worktree_for_mission(&db, &id, path.as_deref())
                }
                WorktreeCommands::For { id, path } => {
                    let id = resolve_issue_arg(&db, &id)?;
                    commands::work::worktree_for(&db, &id, path.as_deref())
                }
                WorktreeCommands::Status => commands::work::worktree_status(&db),
                WorktreeCommands::Merge { id } => {
                    let id = resolve_issue_arg(&db, &id)?;
                    commands::work::worktree_merge(&db, &id)
                }
                WorktreeCommands::Remove { id, force } => {
                    let id = resolve_issue_arg(&db, &id)?;
                    commands::work::worktree_remove(&db, &id, force)
                }
                WorktreeCommands::Repair { id } => {
                    let id = resolve_issue_arg(&db, &id)?;
                    commands::work::worktree_repair(&db, &id)
                }
            }
        }

        Commands::Branch { action } => {
            let db = existing_projection_db()?;
            match action {
                BranchCommands::ForEpic { id } => {
                    let id = resolve_issue_arg(&db, &id)?;
                    commands::work::branch_for_epic(&db, &id)
                }
                BranchCommands::Status => commands::work::branch_status(&db),
                BranchCommands::Merge { id } => {
                    let id = resolve_issue_arg(&db, &id)?;
                    commands::work::branch_merge(&db, &id)
                }
            }
        }

        Commands::Diagnostics { action } => match action {
            DiagnosticsCommands::Slow { days, threshold_ms } => {
                let summary = telemetry::slow_command_summary(days, threshold_ms)?;
                println!("{}", serde_json::to_string_pretty(&summary)?);
                Ok(())
            }
        },

        Commands::Maintenance { action } => match action {
            MaintenanceCommands::Delete {
                target_kind,
                target_id,
                force,
            } => {
                require_issue_kind(&target_kind, "atelier maintenance delete")?;
                let (state_dir, db_path) = state_and_db_paths()?;
                let db = canonical_mutation_db()?;
                let target_id = resolve_issue_arg(&db, &target_id)?;
                drop(db);
                commands::delete::run_lifecycle(&state_dir, &db_path, &target_id, force)
            }
        },

        Commands::Lint { id } => {
            let db = lint_db()?;
            commands::agent_factory::lint(&db, id.as_deref())
        }

        Commands::Doctor { fix } => {
            let storage = command_storage(CommandStorageAccess::HealthRepair)?;
            commands::agent_factory::doctor(
                storage.db(),
                storage.repo_root(),
                &storage.state_dir(),
                &storage.db_path(),
                storage.projection_db_existed,
                fix,
            )
        }
    };

    let success = result.is_ok();
    telemetry::record_command_event(
        command_name,
        started_at,
        started.elapsed(),
        if success { Some(0) } else { Some(1) },
        success,
    );
    result
}

fn parse_cli_or_exit() -> Cli {
    match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            let exit_code = error.exit_code();
            if let Err(print_error) = error.print() {
                eprintln!("{print_error}");
            }
            std::process::exit(exit_code);
        }
    }
}

fn command_identity(command: &Commands) -> &'static str {
    match command {
        Commands::Init { .. } => "init",
        Commands::Man { .. } => "man",
        Commands::Status => "status",
        Commands::Start { .. } => "start",
        Commands::Issue { action } => match action {
            IssueCommands::Create { .. } => "issue create",
            IssueCommands::List { .. } => "issue list",
            IssueCommands::Show { .. } => "issue show",
            IssueCommands::Transition { .. } => "issue transition",
            IssueCommands::Update { .. } => "issue update",
            IssueCommands::Note { .. } => "issue note",
            IssueCommands::Close { .. } => "issue close",
            IssueCommands::Block { .. } => "issue block",
            IssueCommands::Unblock { .. } => "issue unblock",
            IssueCommands::Blocked { .. } => "issue blocked",
        },
        Commands::Search { .. } => "search",
        Commands::Graph { action } => match action {
            GraphCommands::Impact { .. } => "graph impact",
            GraphCommands::Tree { .. } => "graph tree",
        },
        Commands::Export { check, .. } => {
            if *check {
                "export --check"
            } else {
                "export"
            }
        }
        Commands::Rebuild { .. } => "rebuild",
        Commands::ImportBeads { .. } => "import-beads",
        Commands::Mission { action } => match action {
            MissionCommands::Create { .. } => "mission create",
            MissionCommands::Show { .. } => "mission show",
            MissionCommands::Start { .. } => "mission start",
            MissionCommands::Status { .. } => "mission status",
            MissionCommands::Close { .. } => "mission close",
            MissionCommands::List { .. } => "mission list",
            MissionCommands::Update { .. } => "mission update",
            MissionCommands::Note { .. } => "mission note",
            MissionCommands::AddWork { .. } => "mission add-work",
            MissionCommands::Unlink { .. } => "mission unlink",
            MissionCommands::AddBlocker { .. } => "mission add-blocker",
        },
        Commands::Bundle { action } => match action {
            BundleCommands::Preview { .. } => "bundle preview",
            BundleCommands::Apply { .. } => "bundle apply",
        },
        Commands::Evidence { action } => match action {
            EvidenceCommands::Record { .. } => "evidence record",
            EvidenceCommands::Show { .. } => "evidence show",
            EvidenceCommands::Attach { .. } => "evidence attach",
            EvidenceCommands::List { .. } => "evidence list",
        },
        Commands::Session { action } => match action {
            SessionCommands::Show { .. } => "session show",
            SessionCommands::List { .. } => "session list",
        },
        Commands::Pr { action } => match action {
            PrCommands::Open { .. } => "pr open",
            PrCommands::Link { .. } => "pr link",
            PrCommands::Status { .. } => "pr status",
            PrCommands::Show { .. } => "pr show",
            PrCommands::Merge { .. } => "pr merge",
            PrCommands::Comments { .. } => "pr comments",
            PrCommands::Comment { .. } => "pr comment",
            PrCommands::Review { .. } => "pr review",
        },
        Commands::Forgejo { action } => match action {
            ForgejoCommands::Roles { action } => match action {
                ForgejoRolesCommands::Check => "forgejo roles check",
                ForgejoRolesCommands::Provision { .. } => "forgejo roles provision",
            },
        },
        Commands::History { .. } => "history",
        Commands::Workflow { action } => match action {
            WorkflowCommands::Check => "workflow check",
        },
        Commands::Worktree { action } => match action {
            WorktreeCommands::ForMission { .. } => "worktree for-mission",
            WorktreeCommands::For { .. } => "worktree for",
            WorktreeCommands::Status => "worktree status",
            WorktreeCommands::Merge { .. } => "worktree merge",
            WorktreeCommands::Remove { .. } => "worktree remove",
            WorktreeCommands::Repair { .. } => "worktree repair",
        },
        Commands::Branch { action } => match action {
            BranchCommands::ForEpic { .. } => "branch for-epic",
            BranchCommands::Status => "branch status",
            BranchCommands::Merge { .. } => "branch merge",
        },
        Commands::Diagnostics { action } => match action {
            DiagnosticsCommands::Slow { .. } => "diagnostics slow",
        },
        Commands::Maintenance { action } => match action {
            MaintenanceCommands::Delete { .. } => "maintenance delete",
        },
        Commands::Lint { .. } => "lint",
        Commands::Doctor { .. } => "doctor",
    }
}
