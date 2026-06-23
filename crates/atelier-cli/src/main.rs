use anyhow::{bail, Result};
use atelier::{commands, telemetry};
use atelier_app::command_storage::{
    canonical_mutation_db, command_storage, degraded_projection_query_db, existing_projection_db,
    lint_db, state_and_db_paths, CommandStorageAccess,
};
use atelier_app::use_cases;
use atelier_sqlite::Database;
use chrono::Utc;
use clap::{Parser, Subcommand};
use std::env;
use std::time::Instant;

mod issue_cli;

#[derive(Parser)]
#[command(name = "atelier")]
#[command(about = "Mission and proof oriented work coordination for agents")]
#[command(help_template = "{about-section}\nUsage: {usage}\n\n{after-help}\nOptions:\n{options}")]
#[command(after_help = "Setup:
  init          Initialize Atelier in the current repository

Orientation:
  man           Show role-specific operating guidance
  status        Show checkout, mission, work, and tracker signposts

Issues:
  issue         Create, list, show, update, close, and manage blockers
  search        Search issue text

Planning:
  bundle        Preview and apply one-shot graph bundle files

Records:
  evidence      Capture validation evidence
  review        Manage configured review artifacts
  forgejo       Configure and verify Forgejo integration
  history       Inspect canonical repo, mission, issue, or epic activity

Advanced work:
  branch        Inspect and repair epic review branches

Maintenance:
  prune         Prune accumulated artifacts safely
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
  atelier issue create \"...\" --issue-type mission
  atelier issue show <mission-id>
  atelier issue table --kind mission
  atelier issue transition <mission-id> close --reason \"...\"
  atelier bundle preview <file>
  atelier bundle apply <file> --yes
  atelier forgejo roles check
  atelier history --mission <id>
  atelier history --issue <id>
  atelier issue transition <issue-id> --options
  atelier issue transition <issue-id> start
  atelier issue transition <issue-id> close --reason \"...\"
  atelier prune
  atelier prune --apply
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

    /// Issue lifecycle commands (create, show, list, transition, ...)
    Issue {
        #[command(subcommand)]
        action: IssueCommands,
    },

    /// Search issue text
    Search {
        /// Search query
        query: String,
    },

    /// Advanced deterministic-renderer diagnostic; normal health uses lint and status
    #[command(hide = true)]
    Export {
        /// State directory for canonical export diagnostics
        #[arg(short, long)]
        output: Option<String>,
        /// Check deterministic renderer/projection freshness without writing tracked records
        #[arg(long)]
        check: bool,
    },

    /// Advanced projection diagnostic; explicit local repair uses doctor --fix
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

    /// Configured review artifacts
    Review {
        #[command(subcommand)]
        action: ReviewCommands,
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

    /// Prune accumulated artifacts safely
    Prune {
        /// Apply eligible cleanup; without this flag the command only reports candidates
        #[arg(long)]
        apply: bool,
        /// Retain diagnostics logs for this many UTC days
        #[arg(long)]
        retention_days: Option<u64>,
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
        /// Initial Markdown description/body text
        #[arg(long)]
        description: Option<String>,
        /// Mission intent/body text; requires --issue-type mission
        #[arg(long)]
        body: Option<String>,
        /// Add one mission Constraints section bullet; repeat for multiple constraints
        #[arg(long)]
        constraint: Vec<String>,
        /// Add one mission Risks section bullet; repeat for multiple risks
        #[arg(long)]
        risk: Vec<String>,
        /// Add one mission Validation section bullet; repeat for multiple validation criteria
        #[arg(long)]
        validation: Vec<String>,
        /// Priority (low, medium, high, critical)
        #[arg(short, long, default_value = "medium")]
        priority: String,
        /// Work type/body preset (bug, feature, refactor, research)
        #[arg(short, long)]
        template: Option<String>,
        /// Add labels to the issue
        #[arg(short, long)]
        label: Vec<String>,
        /// Explicit work type from .atelier/workflow.yaml issue_types, or built-in mission
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

    /// Show a homogeneous objective inventory table
    Table {
        /// Record kind to inventory: mission or issue
        #[arg(long, default_value = "mission")]
        kind: String,
        /// Filter by exact record/workflow status, or all
        #[arg(long, default_value = "current")]
        status: String,
        /// Filter issue rows by issue type, such as epic
        #[arg(long)]
        issue_type: Option<String>,
    },

    /// Show issue details
    Show {
        /// Issue ID
        id: String,
    },

    /// Show type-aware issue status for objective records
    Status {
        /// Issue ID
        id: String,
        /// Show verbose validator detail for mission objective records
        #[arg(long)]
        verbose: bool,
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
        /// New issue type from .atelier/workflow.yaml issue_types
        #[arg(long)]
        issue_type: Option<String>,
        /// New status for mission objective records
        #[arg(long)]
        status: Option<String>,
        /// Mission intent/body text; requires a mission objective record
        #[arg(long)]
        body: Option<String>,
        /// Add one mission Constraints section bullet; repeat for multiple constraints
        #[arg(long)]
        constraint: Vec<String>,
        /// Add one mission Risks section bullet; repeat for multiple risks
        #[arg(long)]
        risk: Vec<String>,
        /// Add one mission Validation section bullet; repeat for multiple validation criteria
        #[arg(long)]
        validation: Vec<String>,
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

    /// Add a typed link from one issue to another
    Link {
        /// Source issue ID
        id: String,
        /// Target issue ID
        target: String,
        /// Relationship role, such as advances or blocked_by
        #[arg(long, default_value = "advances")]
        role: String,
    },

    /// Remove a typed link from one issue to another
    Unlink {
        /// Source issue ID
        id: String,
        /// Target issue ID
        target: String,
        /// Relationship role, such as advances or blocked_by
        #[arg(long, default_value = "advances")]
        role: String,
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
enum ReviewCommands {
    /// Open or confirm the active review artifact for an issue owner
    Open {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        role: Option<String>,
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "")]
        body: String,
        #[arg(long)]
        source_branch: String,
        #[arg(long, default_value = "master")]
        target_branch: String,
    },
    /// Link an existing review artifact by number or URL
    Link {
        #[arg(long)]
        issue: Option<String>,
        pull_request: String,
    },
    /// Show concise linked review status
    Status {
        #[arg(long)]
        issue: Option<String>,
    },
    /// Show linked review details
    Show {
        #[arg(long)]
        issue: Option<String>,
    },
    /// Merge or confirm the linked review artifact without changing Atelier workflow state
    Merge {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        role: Option<String>,
    },
    /// List live review comments
    Comments {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        unresolved: bool,
    },
    /// Add a review artifact comment
    Comment {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        role: Option<String>,
        /// Record this room comment as a finding instead of a plain timeline comment
        #[arg(long)]
        finding: bool,
        /// Finding severity for native room mode
        #[arg(long, default_value = "blocking")]
        severity: String,
        body: String,
    },
    /// Approve a review artifact
    Approve {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        role: Option<String>,
        #[arg(long, default_value = "")]
        body: String,
    },
    /// Request changes on a review artifact
    RequestChanges {
        #[arg(long)]
        issue: Option<String>,
        #[arg(long)]
        role: Option<String>,
        #[arg(long, default_value = "")]
        body: String,
    },
    /// Resolve a native room finding
    Resolve {
        #[arg(long)]
        issue: Option<String>,
        finding: String,
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
    Provision,
}

#[derive(Subcommand)]
enum WorkflowCommands {
    /// Run raw workflow-policy diagnostics; normal operator checks use lint and status surfaces
    Check,
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

fn resolve_issue_arg(db: &Database, issue_ref: &str) -> Result<String> {
    match commands::issue::resolve_id(db, issue_ref) {
        Ok(id) => Ok(id),
        Err(error) => match db.record_kind_for_id(issue_ref)? {
            Some(actual_kind) if actual_kind != "issue" => {
                bail!("{}", wrong_kind_message("issue", &actual_kind, issue_ref));
            }
            _ => Err(error),
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
        "mission" => Some("atelier issue show"),
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

        Commands::Issue { action } => issue_cli::dispatch(action, quiet),

        Commands::Search { query } => {
            let db = degraded_projection_query_db()?;
            commands::issue::search(&db, &query, quiet)
        }

        Commands::Export { output, check } => {
            let storage = command_storage(CommandStorageAccess::HealthRepair)?;
            let state_dir = output
                .as_deref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| storage.state_dir());
            commands::issue::export_canonical(storage.db(), &state_dir, check)
        }

        Commands::Rebuild { input } => {
            let storage = command_storage(CommandStorageAccess::HealthRepair)?;
            let state_dir = input
                .as_deref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| storage.state_dir());
            let db_path = storage.db_path();
            commands::issue::rebuild(&state_dir, &db_path)
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

        Commands::Review { action } => {
            let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
            match action {
                ReviewCommands::Open {
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
                    role.as_deref(),
                    &title,
                    &body,
                    &source_branch,
                    &target_branch,
                ),
                ReviewCommands::Link {
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
                ReviewCommands::Status { issue } => commands::pr::status(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    issue.as_deref(),
                ),
                ReviewCommands::Show { issue } => commands::pr::show(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    issue.as_deref(),
                ),
                ReviewCommands::Merge { issue, role } => commands::pr::merge(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    &storage.db_path(),
                    issue.as_deref(),
                    role.as_deref(),
                ),
                ReviewCommands::Comments { issue, unresolved } => commands::pr::comments(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    issue.as_deref(),
                    unresolved,
                ),
                ReviewCommands::Comment {
                    issue,
                    role,
                    finding,
                    severity,
                    body,
                } => commands::pr::comment(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    &storage.db_path(),
                    issue.as_deref(),
                    role.as_deref(),
                    &body,
                    finding,
                    finding.then_some(severity.as_str()),
                ),
                ReviewCommands::Approve { issue, role, body } => commands::pr::review(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    &storage.db_path(),
                    issue.as_deref(),
                    role.as_deref(),
                    "approve",
                    &body,
                ),
                ReviewCommands::RequestChanges { issue, role, body } => commands::pr::review(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    &storage.db_path(),
                    issue.as_deref(),
                    role.as_deref(),
                    "request-changes",
                    &body,
                ),
                ReviewCommands::Resolve { issue, finding } => commands::pr::resolve(
                    storage.db(),
                    storage.repo_root(),
                    &storage.state_dir(),
                    &storage.db_path(),
                    issue.as_deref(),
                    &finding,
                ),
            }
        }

        Commands::Forgejo { action } => {
            let repo_root = atelier_app::storage_layout::find_repo_root()?;
            match action {
                ForgejoCommands::Roles { action } => match action {
                    ForgejoRolesCommands::Check => commands::forgejo::roles_check(&repo_root),
                    ForgejoRolesCommands::Provision => {
                        commands::forgejo::roles_provision(&repo_root)
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
                .map(|id| resolve_issue_arg(storage.db(), id))
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

        Commands::Prune {
            apply,
            retention_days,
        } => {
            let tracker = match command_storage(CommandStorageAccess::CanonicalMutation) {
                Ok(storage) => {
                    let repo_root = storage.repo_root().to_path_buf();
                    let config = atelier_app::project_config::ProjectConfig::load(&repo_root)?;
                    let state_dir = storage.state_dir();
                    let db_path = storage.db_path();
                    Some(commands::prune::TrackerContext {
                        db: storage.into_db(),
                        repo_root,
                        state_dir,
                        db_path,
                        canonical_retention_days: config.prune.canonical_retention_days,
                    })
                }
                Err(error) => {
                    if atelier_app::command_storage::find_atelier_dir().is_ok() {
                        return Err(error);
                    }
                    None
                }
            };
            commands::prune::run(tracker, apply, retention_days)
        }

        Commands::Lint { id } => {
            let db = lint_db()?;
            commands::issue::lint(&db, id.as_deref())
        }

        Commands::Doctor { fix } => {
            let storage = command_storage(CommandStorageAccess::HealthRepair)?;
            commands::issue::doctor(
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
        Commands::Issue { action } => match action {
            IssueCommands::Create { .. } => "issue create",
            IssueCommands::List { .. } => "issue list",
            IssueCommands::Table { .. } => "issue table",
            IssueCommands::Show { .. } => "issue show",
            IssueCommands::Status { .. } => "issue status",
            IssueCommands::Transition { .. } => "issue transition",
            IssueCommands::Update { .. } => "issue update",
            IssueCommands::Note { .. } => "issue note",
            IssueCommands::Link { .. } => "issue link",
            IssueCommands::Unlink { .. } => "issue unlink",
            IssueCommands::Block { .. } => "issue block",
            IssueCommands::Unblock { .. } => "issue unblock",
            IssueCommands::Blocked { .. } => "issue blocked",
        },
        Commands::Search { .. } => "search",
        Commands::Export { check, .. } => {
            if *check {
                "export --check"
            } else {
                "export"
            }
        }
        Commands::Rebuild { .. } => "rebuild",
        Commands::ImportBeads { .. } => "import-beads",
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
        Commands::Review { action } => match action {
            ReviewCommands::Open { .. } => "review open",
            ReviewCommands::Link { .. } => "review link",
            ReviewCommands::Status { .. } => "review status",
            ReviewCommands::Show { .. } => "review show",
            ReviewCommands::Merge { .. } => "review merge",
            ReviewCommands::Comments { .. } => "review comments",
            ReviewCommands::Comment { .. } => "review comment",
            ReviewCommands::Approve { .. } => "review approve",
            ReviewCommands::RequestChanges { .. } => "review request-changes",
            ReviewCommands::Resolve { .. } => "review resolve",
        },
        Commands::Forgejo { action } => match action {
            ForgejoCommands::Roles { action } => match action {
                ForgejoRolesCommands::Check => "forgejo roles check",
                ForgejoRolesCommands::Provision => "forgejo roles provision",
            },
        },
        Commands::History { .. } => "history",
        Commands::Workflow { action } => match action {
            WorkflowCommands::Check => "workflow check",
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
        Commands::Prune { apply, .. } => {
            if *apply {
                "prune --apply"
            } else {
                "prune"
            }
        }
        Commands::Lint { .. } => "lint",
        Commands::Doctor { .. } => "doctor",
    }
}
