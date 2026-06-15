mod activity;
mod command_storage;
mod command_surface;
mod commands;
mod db;
#[cfg(test)]
mod identity;
mod models;
mod projection_index;
mod record_id;
mod record_store;
mod storage_layout;
mod telemetry;
mod test_inventory;
mod utils;
mod workflow_policy;

use anyhow::{bail, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use std::env;
use std::time::Instant;

use command_storage::{
    canonical_mutation_db, command_storage, degraded_projection_query_db, lint_db,
    projection_query_db, runtime_db, state_and_db_paths, CommandStorageAccess,
};
use db::Database;
use record_store::RecordStore;

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
  abandon       Clear active local work without changing issue status
  repair        Clear stale active local work after interrupted worktree cleanup

Issues:
  issue         Create, list, show, update, close, and manage blockers
  search        Search issue text
  graph         Inspect mission and issue hierarchy and impact

Missions and planning:
  mission       Create, list, show, status, close, and update durable missions
  plan          Create, apply, revise, list, and link durable plans

Records:
  evidence      Capture validation evidence
  history       Inspect canonical repo, mission, issue, or epic activity

Advanced work:
  worktree      Create, inspect, merge, and remove mission or issue worktrees
  branch        Create, inspect, and merge epic review branches

Maintenance:
  maintenance   Run explicit destructive maintenance commands
  lint          Validate tracker records
  doctor        Check runtime and derived-state health; use --fix for local repair

Common commands:
  atelier man
  atelier man worker
  atelier man reviewer
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
  atelier history --mission <id>
  atelier history --issue <id>
  atelier start <issue-id>
  atelier abandon [issue-id] --reason \"...\"
  atelier repair [issue-id]
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
        /// Role guide to print: worker, reviewer, manager, or admin
        role: Option<String>,
    },

    /// Show checkout, mission, work, and tracker signposts
    Status,

    /// Start tracked work on an issue
    Start { id: String },

    /// Clear active local work without changing issue status
    Abandon {
        /// Issue ID; defaults to the active work association
        id: Option<String>,
        /// Reason for abandoning the local work association
        #[arg(long)]
        reason: String,
    },

    /// Clear stale active local work after interrupted worktree cleanup
    Repair {
        /// Issue ID; defaults to the active work association
        id: Option<String>,
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

    /// Removed generic activity note commands
    #[command(hide = true)]
    Note {
        #[command(subcommand)]
        action: NoteCommands,
    },

    /// Export canonical state
    Export {
        /// State directory for canonical export
        #[arg(short, long)]
        output: Option<String>,
        /// Check whether canonical tracker records are current
        #[arg(long)]
        check: bool,
    },

    /// Rebuild local SQLite runtime state from canonical tracker records
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

    /// First-class durable plan records
    Plan {
        #[command(subcommand)]
        action: PlanCommands,
    },

    /// First-class evidence records
    Evidence {
        #[command(subcommand)]
        action: EvidenceCommands,
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
        /// Issue description
        #[arg(short, long)]
        description: Option<String>,
        /// Priority (low, medium, high, critical)
        #[arg(short, long, default_value = "medium")]
        priority: String,
        /// Work type/body preset (bug, feature, refactor, research)
        #[arg(short, long)]
        template: Option<String>,
        /// Add labels to the issue
        #[arg(short, long)]
        label: Vec<String>,
        /// Explicit work type (bug, closeout, epic, feature, spike, task, validation)
        #[arg(long)]
        issue_type: Option<String>,
        /// Parent issue ID or imported source ID
        #[arg(long)]
        parent: Option<String>,
        /// Set as current session work item
        #[arg(short, long)]
        work: bool,
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
        /// New issue type (bug, closeout, epic, feature, spike, task, validation)
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
        /// Claim this issue for the current agent/user
        #[arg(long, hide = true)]
        claim: bool,
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
enum NoteCommands {
    /// Removed: use record-specific note commands
    Add {
        target_kind: String,
        target_id: String,
        text: String,
        /// Note kind (note, plan, observation, blocker, resolution, result, handoff, human)
        #[arg(long, default_value = "note")]
        kind: String,
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
    /// Show a mission with linked plans, work, blockers, and evidence
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
        /// Show closeout audit detail for the mission
        #[arg(long)]
        closeout: bool,
        /// Show verbose validator detail in the status summary
        #[arg(long)]
        verbose: bool,
        id: Option<String>,
    },
    /// Audit mission shell closeout and explicit workflow approval
    Audit { id: String },
    /// Close a mission after all closeout gates pass
    Close {
        id: String,
        /// Mission closeout reason recorded in the mission closeout notes
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
enum PlanCommands {
    /// Create a durable plan
    Create {
        title: String,
        #[arg(short, long)]
        body: Option<String>,
        #[arg(long)]
        reason: Option<String>,
    },
    /// Show a plan
    Show { id: String },
    /// Apply an authored bulk plan JSON file
    Apply {
        input: String,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        validate_only: bool,
    },
    /// List plans
    List {
        #[arg(short, long)]
        status: Option<String>,
    },
    /// Add a new plan revision
    Revise {
        id: String,
        body: String,
        #[arg(long)]
        reason: Option<String>,
    },
    /// Link a plan to a target record
    Link {
        id: String,
        target_kind: String,
        target_id: String,
        #[arg(short = 't', long = "type", default_value = "planned_by")]
        relation_type: String,
    },
}

#[derive(Subcommand)]
enum EvidenceCommands {
    /// Record proof manually or by capturing a command transcript
    #[command(after_help = "Examples:
  atelier evidence record --target issue/<id> --kind validation --result pass \"summary\"
  atelier evidence record --target issue/<id> --kind test --result pass -- <command>

Use `evidence attach` only when you need to reuse an existing evidence record on
another target.")]
    Record {
        #[arg(long = "kind")]
        evidence_kind: String,
        #[arg(long)]
        result: String,
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
        result: Option<String>,
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
    description: Option<&str>,
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
            let description = match (template.description_prefix, description) {
                (Some(prefix), Some(user_description)) => {
                    Some(format!("{prefix}\n\n{user_description}"))
                }
                (Some(prefix), None) => Some(prefix.to_string()),
                (None, description) => description.map(str::to_string),
            };
            (
                priority.to_string(),
                description,
                Some(template_default_issue_type(template_name)),
            )
        } else {
            (priority.to_string(), description.map(str::to_string), None)
        };

    if !commands::create::validate_priority(&final_priority) {
        bail!(
            "Invalid priority '{}'. Must be one of: low, medium, high, critical",
            final_priority
        );
    }
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

fn resolve_optional_record_arg(
    db: &Database,
    kind: &str,
    id: Option<String>,
) -> Result<Option<String>> {
    id.map(|id| resolve_record_arg(db, kind, &id)).transpose()
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
        "plan" => Some("atelier plan show"),
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

fn parse_evidence_target(target: &str) -> Result<(&str, &str)> {
    let Some((kind, id)) = target.split_once('/') else {
        bail!("--target must use kind/id syntax, for example issue/atelier-1234");
    };
    if kind.trim().is_empty() || id.trim().is_empty() {
        bail!("--target must use kind/id syntax, for example issue/atelier-1234");
    }
    Ok((kind, id))
}

fn resolve_evidence_target_arg(db: &Database, kind: &str, id: &str) -> Result<String> {
    if matches!(kind, "issue" | "epic") {
        resolve_issue_arg(db, id)
    } else {
        Ok(id.to_string())
    }
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
            description,
            priority,
            template,
            label,
            issue_type,
            parent,
            work,
        } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            let (final_priority, final_description, labels, issue_type) = issue_create_parts(
                &priority,
                description.as_deref(),
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
                    work,
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
                commands::deps::list_blocked(&db)
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
            claim,
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
                    claim,
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
            commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        IssueCommands::Unblock { id, blocker } => {
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            commands::agent_factory::dep_remove_canonical(&db, &store, &id, &blocker)?;
            drop(db);
            commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        IssueCommands::Blocked { id } => {
            let db = projection_query_db()?;
            if let Some(id) = id {
                commands::agent_factory::dep_list(&db, Some(&id))
            } else {
                commands::deps::list_blocked(&db)
            }
        }
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() -> Result<()> {
    run()
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
            let storage = command_storage(CommandStorageAccess::DegradedProjectionQuery)?;
            commands::status::run(storage.db(), &storage.state_dir(), quiet)
        }

        Commands::Start { id } => {
            let db = projection_query_db()?;
            let id = resolve_issue_arg(&db, &id)?;
            let (state_dir, db_path) = state_and_db_paths()?;
            commands::work::start_lifecycle(&state_dir, &db_path, &id)
        }

        Commands::Abandon { id, reason } => {
            let db = runtime_db()?;
            let id = match id {
                Some(id) => resolve_issue_arg(&db, &id)?,
                None => db
                    .get_active_work_association()?
                    .map(|work| work.issue_id)
                    .ok_or_else(|| {
                        anyhow::anyhow!("No active work. Use `atelier start <issue-id>` first.")
                    })?,
            };
            commands::work::abandon(&db, &id, &reason)
        }

        Commands::Repair { id } => {
            let db = runtime_db()?;
            let id = match id {
                Some(id) => Some(resolve_issue_arg(&db, &id)?),
                None => None,
            };
            commands::work::repair_active(&db, id.as_deref())
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

        Commands::Note { action } => match action {
            NoteCommands::Add {
                target_kind,
                target_id,
                text,
                kind,
            } => {
                let replacement = match target_kind.as_str() {
                    "issue" => format!("atelier issue note {target_id} {text:?} --kind {kind}"),
                    "mission" => {
                        format!("atelier mission note {target_id} {text:?} --kind {kind}")
                    }
                    _ => "atelier issue note <id> \"...\" or atelier mission note <id> \"...\""
                        .to_string(),
                };
                bail!(
                    "`atelier note add <kind> <id>` was removed. Use `{}` instead.",
                    replacement
                )
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
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
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
                let db = degraded_projection_query_db()?;
                let id = resolve_record_arg(&db, "mission", &id)?;
                commands::mission::show(&db, &id)
            }
            MissionCommands::Start { id, switch_active } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = resolve_record_arg(storage.db(), "mission", &id)?;
                commands::mission::start(&state_dir, &db_path, &id, switch_active)
            }
            MissionCommands::Status {
                id,
                closeout,
                verbose,
            } => {
                let storage = command_storage(CommandStorageAccess::DegradedProjectionQuery)?;
                let id = resolve_optional_record_arg(storage.db(), "mission", id)?;
                commands::mission::status(
                    storage.db(),
                    &storage.state_dir(),
                    id.as_deref(),
                    quiet,
                    closeout,
                    verbose,
                )
            }
            MissionCommands::Audit { id } => {
                let storage = command_storage(CommandStorageAccess::ProjectionQuery)?;
                let id = resolve_record_arg(storage.db(), "mission", &id)?;
                commands::mission::audit(storage.db(), &storage.state_dir(), &id, quiet)
            }
            MissionCommands::Close { id, reason } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = resolve_record_arg(storage.db(), "mission", &id)?;
                commands::mission::close(&state_dir, &db_path, &id, &reason)
            }
            MissionCommands::List { status } => {
                let db = degraded_projection_query_db()?;
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
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = resolve_record_arg(storage.db(), "mission", &id)?;
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
                let db = canonical_mutation_db()?;
                let id = resolve_record_arg(&db, "mission", &id)?;
                commands::comment::run_mission_note(&db, &id, &text, &kind)
            }
            MissionCommands::AddWork { id, issue } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = resolve_record_arg(storage.db(), "mission", &id)?;
                let issue = resolve_issue_arg(storage.db(), &issue)?;
                commands::mission::add_work(&state_dir, &db_path, &id, &issue)
            }
            MissionCommands::Unlink { id, issue } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = resolve_record_arg(storage.db(), "mission", &id)?;
                let issue = resolve_issue_arg(storage.db(), &issue)?;
                commands::mission::unlink(&state_dir, &db_path, &id, &issue)
            }
            MissionCommands::AddBlocker { id, issue } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = resolve_record_arg(storage.db(), "mission", &id)?;
                let issue = resolve_issue_arg(storage.db(), &issue)?;
                commands::mission::add_blocker(&state_dir, &db_path, &id, &issue)
            }
        },

        Commands::Plan { action } => match action {
            PlanCommands::Create {
                title,
                body,
                reason,
            } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                commands::plan::create(
                    &state_dir,
                    &db_path,
                    &title,
                    body.as_deref(),
                    reason.as_deref(),
                )
            }
            PlanCommands::Show { id } => {
                let db = projection_query_db()?;
                commands::plan::show(&db, &id)
            }
            PlanCommands::Apply {
                input,
                dry_run,
                validate_only,
            } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                commands::plan::apply(
                    storage.db(),
                    &storage.state_dir(),
                    &storage.db_path(),
                    &input,
                    dry_run,
                    validate_only,
                )
            }
            PlanCommands::List { status } => {
                let db = projection_query_db()?;
                commands::plan::list(&db, status.as_deref())
            }
            PlanCommands::Revise { id, body, reason } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                commands::plan::revise(
                    &storage.state_dir(),
                    &storage.db_path(),
                    &id,
                    &body,
                    reason.as_deref(),
                )
            }
            PlanCommands::Link {
                id,
                target_kind,
                target_id,
                relation_type,
            } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let target_id = resolve_record_arg(storage.db(), &target_kind, &target_id)?;
                commands::plan::link(
                    &storage.state_dir(),
                    &storage.db_path(),
                    &id,
                    &target_kind,
                    &target_id,
                    &relation_type,
                )
            }
        },

        Commands::Evidence { action } => match action {
            EvidenceCommands::Record {
                evidence_kind,
                result,
                target,
                role,
                summary,
                path,
                uri,
                producer,
                summary_text,
                command,
            } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let parsed_target = match target.as_deref() {
                    Some(target) => {
                        let (kind, id) = parse_evidence_target(target)?;
                        let id = resolve_evidence_target_arg(storage.db(), kind, id)?;
                        Some((kind.to_string(), id))
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
                        &result,
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
                    let db = Database::open(&storage.db_path())?;
                    let record = db.require_record("evidence", &evidence_id)?;
                    commands::evidence::print_record(&db, &record)
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
                            result: &result,
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
                let db = projection_query_db()?;
                commands::evidence::show(&db, &id)
            }
            EvidenceCommands::Attach {
                id,
                target_kind,
                target_id,
                role,
            } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let target_id =
                    resolve_evidence_target_arg(storage.db(), &target_kind, &target_id)?;
                commands::evidence::attach(
                    &storage.state_dir(),
                    &storage.db_path(),
                    &id,
                    &target_kind,
                    &target_id,
                    &role,
                )
            }
            EvidenceCommands::List { result } => {
                let db = projection_query_db()?;
                commands::evidence::list(&db, result.as_deref())
            }
        },

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
                let db = projection_query_db()?;
                commands::workflow::check(&db)
            }
        },

        Commands::Worktree { action } => {
            let db = runtime_db()?;
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
            let db = runtime_db()?;
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
                storage.runtime_db_existed,
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
            let args = env::args().skip(1).collect::<Vec<_>>();
            let removed_guidance = removed_command_guidance(&args);
            let exit_code = error.exit_code();
            if let Err(print_error) = error.print() {
                eprintln!("{print_error}");
            }
            if let Some(guidance) = removed_guidance {
                eprintln!();
                eprintln!("{guidance}");
            }
            std::process::exit(exit_code);
        }
    }
}

fn removed_command_guidance(args: &[String]) -> Option<&'static str> {
    let path = command_path_tokens(args);
    match path.as_slice() {
        ["workflow", "check", ..] => Some(
            "`atelier workflow check` is not the normal workflow-readiness path; use `atelier issue transition <id> --options`, `atelier mission status [<id>]`, `atelier lint`, or `atelier doctor`.",
        ),
        ["workflow", "init", ..] => Some(
            "`atelier workflow init` was removed; use root `atelier init` to create `.atelier/workflow.yaml` during tracker setup.",
        ),
        ["finish", ..] => Some(
            "`atelier finish` was removed; use `atelier issue close <id> --reason \"...\"` or inspect `atelier issue transition <id> --options`.",
        ),
        ["current-work", ..] => Some(
            "`atelier current-work` was removed; use `atelier status` for current-work orientation or `atelier issue transition <id> --options` for next steps.",
        ),
        ["issue", "new", ..] => Some(
            "`atelier issue new` was removed; use `atelier issue create \"title\"`.",
        ),
        ["work", "start", ..] => Some(
            "`atelier work start` was removed; use root `atelier start <issue-id>` for tracked work or `atelier worktree for <issue-id>` for an issue worktree.",
        ),
        ["work", ..] => Some(
            "`atelier work` was removed; use root `atelier start <issue-id>`, `atelier status`, `atelier abandon`, or `atelier worktree ...`.",
        ),
        ["integrations", ..] => Some(
            "`atelier integrations` was removed; external assistant hooks are not an Atelier product feature.",
        ),
        ["link", ..] => Some(
            "`atelier link` was removed. Use record-specific commands: `atelier mission add-work` or `atelier mission unlink` for mission work, `atelier issue block` or `atelier issue unblock` for blockers, `atelier evidence attach` for evidence, and `atelier graph impact` or `atelier graph tree` for inspection.",
        ),
        ["archive", ..] => Some(
            "`atelier archive` was removed; use workflow-backed `atelier issue close <id> --to archived --reason \"...\"` when the configured workflow allows archive.",
        ),
        ["session", ..] => Some(
            "`atelier session` was removed; use `atelier start <issue-id>` to begin tracked work, `atelier status` for orientation, and `atelier issue note <id> \"...\"` for handoff context.",
        ),
        ["timer", ..] => Some(
            "`atelier timer` was removed; use `atelier status` and `atelier history --issue <id>` for work orientation and activity history.",
        ),
        _ => None,
    }
}

fn command_path_tokens(args: &[String]) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        match arg {
            "-q" | "--quiet" => {
                index += 1;
            }
            "--log-level" | "--log-format" => {
                index += 2;
            }
            _ if arg.starts_with("--log-level=") || arg.starts_with("--log-format=") => {
                index += 1;
            }
            _ if arg.starts_with('-') => {
                index += 1;
            }
            _ => {
                tokens.push(arg);
                index += 1;
            }
        }
    }
    tokens
}

fn command_identity(command: &Commands) -> &'static str {
    match command {
        Commands::Init { .. } => "init",
        Commands::Man { .. } => "man",
        Commands::Status => "status",
        Commands::Start { .. } => "start",
        Commands::Abandon { .. } => "abandon",
        Commands::Repair { .. } => "repair",
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
        Commands::Note { action } => match action {
            NoteCommands::Add { .. } => "note add",
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
            MissionCommands::Audit { .. } => "mission audit",
            MissionCommands::Close { .. } => "mission close",
            MissionCommands::List { .. } => "mission list",
            MissionCommands::Update { .. } => "mission update",
            MissionCommands::Note { .. } => "mission note",
            MissionCommands::AddWork { .. } => "mission add-work",
            MissionCommands::Unlink { .. } => "mission unlink",
            MissionCommands::AddBlocker { .. } => "mission add-blocker",
        },
        Commands::Plan { action } => match action {
            PlanCommands::Create { .. } => "plan create",
            PlanCommands::Show { .. } => "plan show",
            PlanCommands::Apply { .. } => "plan apply",
            PlanCommands::List { .. } => "plan list",
            PlanCommands::Revise { .. } => "plan revise",
            PlanCommands::Link { .. } => "plan link",
        },
        Commands::Evidence { action } => match action {
            EvidenceCommands::Record { .. } => "evidence record",
            EvidenceCommands::Show { .. } => "evidence show",
            EvidenceCommands::Attach { .. } => "evidence attach",
            EvidenceCommands::List { .. } => "evidence list",
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
