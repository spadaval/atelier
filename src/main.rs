mod activity;
mod commands;
mod db;
mod identity;
mod lock_check;
mod locks;
mod models;
mod projection_index;
mod record_id;
mod record_store;
mod sync;
mod utils;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;

use db::Database;

#[derive(Parser)]
#[command(name = "atelier")]
#[command(about = "A simple, lean issue tracker CLI")]
#[command(help_template = "{about-section}\nUsage: {usage}\n\n{after-help}\nOptions:\n{options}")]
#[command(after_help = "Setup:
  init          Initialize Atelier in the current repository

Issues:
  issue         Create, list, show, update, close, and relate issues
  dep           Manage issue blockers with add, remove, and list

Missions and planning:
  mission       Create, list, show, and update durable missions
  plan          Create, apply, revise, list, and link durable plans

Records:
  evidence      Capture validation evidence
  link          Manage typed links across tracker records

Work:
  work          Start, finish, and inspect tracked work
  worktree      Create, inspect, merge, and remove issue worktrees
  workflow      Validate workflow policy for records

State management:
  export        Write or check canonical .atelier-state projection
  rebuild       Rebuild local SQLite state from .atelier-state
  import-beads  Import an external Beads JSONL backup

Maintenance:
  lint          Validate tracker records
  doctor        Check runtime and exported-state health

Common commands:
  atelier issue list
  atelier issue ready
  atelier issue show <id>
  atelier issue update <id> --claim
  atelier mission list
  atelier mission show <id>
  atelier work status
  atelier doctor
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
        /// Force update hooks even if already initialized
        #[arg(short, long)]
        force: bool,
    },

    /// Issue lifecycle commands (create, show, list, close, ...)
    Issue {
        #[command(subcommand)]
        action: IssueCommands,
    },

    /// Export canonical state
    Export {
        /// State directory for canonical export
        #[arg(short, long)]
        output: Option<String>,
        /// Check whether the canonical .atelier-state projection is current
        #[arg(long)]
        check: bool,
    },

    /// Rebuild local SQLite runtime state from canonical .atelier-state files
    Rebuild {
        /// Canonical state directory to rebuild from
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Import Beads JSONL backup into Atelier runtime and canonical state
    ImportBeads {
        /// Beads JSONL backup path from an external source
        input: String,
        /// Canonical state directory to write after import
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Dependency aliases for Agent Factory (`dep add/remove/list`)
    Dep {
        #[command(subcommand)]
        action: DepCommands,
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

    /// Typed links across records
    Link {
        #[command(subcommand)]
        action: LinkCommands,
    },

    /// First-class evidence records
    Evidence {
        #[command(subcommand)]
        action: EvidenceCommands,
    },

    /// Workflow policy and validator helpers
    Workflow {
        #[command(subcommand)]
        action: WorkflowCommands,
    },

    /// Work lifecycle and worktree helpers
    Work {
        #[command(subcommand)]
        action: WorkCommands,
    },

    /// Git worktree helpers for tracked work
    Worktree {
        #[command(subcommand)]
        action: WorktreeCommands,
    },

    /// Validate tracker records
    Lint {
        /// Optional issue ID or imported source ID
        id: Option<String>,
    },

    /// Check tracker runtime and exported-state health
    Doctor,
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
        /// Template (bug, feature, refactor, research)
        #[arg(short, long)]
        template: Option<String>,
        /// Add labels to the issue
        #[arg(short, long)]
        label: Vec<String>,
        /// Issue type for Agent Factory parity
        #[arg(long)]
        issue_type: Option<String>,
        /// Parent issue ID or imported source ID
        #[arg(long)]
        parent: Option<String>,
        /// Set as current session work item
        #[arg(short, long)]
        work: bool,
    },

    /// Quick-create an issue and start working on it (create + label + session work)
    Quick {
        /// Issue title
        title: String,
        /// Issue description
        #[arg(short, long)]
        description: Option<String>,
        /// Priority (low, medium, high, critical)
        #[arg(short, long, default_value = "medium")]
        priority: String,
        /// Template (bug, feature, refactor, research)
        #[arg(short, long)]
        template: Option<String>,
        /// Add labels to the issue
        #[arg(short, long)]
        label: Vec<String>,
    },

    /// Create a subissue under a parent issue
    Subissue {
        /// Parent issue ID
        parent: String,
        /// Subissue title
        title: String,
        /// Subissue description
        #[arg(short, long)]
        description: Option<String>,
        /// Priority (low, medium, high, critical)
        #[arg(short, long, default_value = "medium")]
        priority: String,
        /// Add labels to the subissue
        #[arg(short, long)]
        label: Vec<String>,
        /// Set as current session work item
        #[arg(short, long)]
        work: bool,
    },

    /// List issues
    List {
        /// Filter by status (open, closed, all)
        #[arg(short, long, default_value = "open")]
        status: String,
        /// Filter by label
        #[arg(short, long)]
        label: Option<String>,
        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,
    },

    /// Search issues by text
    Search {
        /// Search query
        query: String,
    },

    /// Show issue details
    Show {
        /// Issue ID
        id: String,
    },

    /// Update an issue
    Update {
        /// Issue ID
        id: String,
        /// New title
        #[arg(short, long)]
        title: Option<String>,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
        /// New priority
        #[arg(short, long)]
        priority: Option<String>,
        /// New status (open, in_progress, closed)
        #[arg(short, long)]
        status: Option<String>,
        /// Add labels to the issue
        #[arg(short, long)]
        label: Vec<String>,
        /// Set parent issue ID or imported source ID
        #[arg(long)]
        parent: Option<String>,
        /// Clear parent issue
        #[arg(long)]
        no_parent: bool,
        /// Claim this issue for the current agent/user
        #[arg(long)]
        claim: bool,
        /// Append durable notes without opening an editor
        #[arg(long)]
        append_notes: Option<String>,
    },

    /// Close an issue
    Close {
        /// Issue ID
        id: String,
        /// Closure reason
        #[arg(short, long)]
        reason: Option<String>,
    },

    /// Close all issues matching filters
    CloseAll {
        /// Filter by label
        #[arg(short, long)]
        label: Option<String>,
        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,
    },

    /// Reopen a closed issue
    Reopen {
        /// Issue ID
        id: String,
    },

    /// Delete an issue
    Delete {
        /// Issue ID
        id: String,
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Add a comment to an issue
    Comment {
        /// Issue ID
        id: String,
        /// Comment text
        text: String,
        /// Comment kind (note, plan, decision, observation, blocker, resolution, result, handoff, human)
        #[arg(long, default_value = "note")]
        kind: String,
    },

    /// Add a label to an issue
    Label {
        /// Issue ID
        id: String,
        /// Label name
        label: String,
    },

    /// Remove a label from an issue
    Unlabel {
        /// Issue ID
        id: String,
        /// Label name
        label: String,
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

    /// List blocked issues
    Blocked,

    /// List issues ready to work on (no open blockers)
    Ready,

    /// Link two related issues
    Relate {
        /// First issue ID
        id: String,
        /// Second issue ID
        related: String,
        /// Relation type (any string, e.g. related, assumption, falsifies, derived, caused-by)
        #[arg(short = 't', long = "type", default_value = "related")]
        relation_type: String,
    },

    /// Remove a relation between issues
    Unrelate {
        /// First issue ID
        id: String,
        /// Second issue ID
        related: String,
        /// Relation type to remove
        #[arg(short = 't', long = "type", default_value = "related")]
        relation_type: String,
    },

    /// List related issues
    Related {
        /// Issue ID
        id: String,
    },

    /// Show downstream issue impact from hierarchy and impact-bearing links
    Impact {
        /// Issue ID to check impact from
        id: String,
    },

    /// Suggest the next issue to work on
    Next,

    /// Show issues as a tree hierarchy
    Tree {
        /// Filter by status (open, closed, all)
        #[arg(short, long, default_value = "all")]
        status: String,
        /// Show a bounded, scan-friendly hierarchy instead of the full tree
        #[arg(long)]
        compact: bool,
    },

    /// Mark tests as run (resets test reminder)
    Tested,
}

#[derive(Subcommand)]
enum DepCommands {
    /// Add a blocking dependency: <blocked> is blocked by <blocker>
    Add { blocked: String, blocker: String },
    /// Remove a blocking dependency
    Remove { blocked: String, blocker: String },
    /// List blocking dependencies
    List { issue: Option<String> },
}

#[derive(Subcommand)]
enum MissionCommands {
    /// Create a mission
    Create {
        title: String,
        #[arg(short, long)]
        body: Option<String>,
        #[arg(long)]
        constraint: Vec<String>,
        #[arg(long)]
        risk: Vec<String>,
        #[arg(long)]
        validation: Vec<String>,
    },
    /// Show a mission with linked plans, work, blockers, and evidence
    Show { id: String },
    /// List missions
    List {
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
enum LinkCommands {
    /// Add a typed link
    Add {
        source_kind: String,
        source_id: String,
        target_kind: String,
        target_id: String,
        #[arg(short = 't', long = "type", default_value = "related")]
        relation_type: String,
    },
    /// Remove a typed link
    Remove {
        source_kind: String,
        source_id: String,
        target_kind: String,
        target_id: String,
        #[arg(short = 't', long = "type", default_value = "related")]
        relation_type: String,
    },
    /// List typed links for a record
    List { kind: String, id: String },
}

#[derive(Subcommand)]
enum EvidenceCommands {
    /// Add validation evidence
    Add {
        #[arg(long = "kind")]
        evidence_kind: String,
        #[arg(long)]
        result: String,
        summary: String,
        #[arg(long)]
        path: Option<String>,
        #[arg(long)]
        uri: Option<String>,
        #[arg(long)]
        producer: Option<String>,
    },
    /// Show an evidence record
    Show { id: String },
    /// List evidence records
    List {
        #[arg(long)]
        result: Option<String>,
    },
}

#[derive(Subcommand)]
enum WorkflowCommands {
    /// Evaluate workflow validators without mutating record state
    Validate {
        target_kind: String,
        target_id: String,
        #[arg(long, default_value = "close")]
        transition: String,
        #[arg(long)]
        validator: Vec<String>,
    },
}

#[derive(Subcommand)]
enum WorkCommands {
    /// Start tracked work on an issue
    Start { id: String },
    /// Finish tracked work on an issue
    Finish { id: String },
    /// Show current work association
    Status,
    /// Create or locate a worktree for an issue
    Worktree {
        #[command(subcommand)]
        action: WorktreeCommands,
    },
}

#[derive(Subcommand)]
enum WorktreeCommands {
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
}

// ============================================================================
// Helpers
// ============================================================================

fn find_atelier_dir() -> Result<PathBuf> {
    let mut current = env::current_dir()?;

    loop {
        let candidate = current.join(".atelier");
        if candidate.is_dir() {
            return Ok(candidate);
        }

        if !current.pop() {
            bail!("Not an Atelier repository (or any parent). Run 'atelier init' first.");
        }
    }
}

fn find_repo_root_for_rebuild() -> Result<PathBuf> {
    let mut current = env::current_dir()?;

    loop {
        if current.join(".atelier-state").is_dir() || current.join(".atelier").is_dir() {
            return Ok(current);
        }

        if !current.pop() {
            bail!(
                "Not an Atelier repository (or any parent). Run from a checkout with .atelier-state/."
            );
        }
    }
}

fn get_db() -> Result<Database> {
    let atelier_dir = find_atelier_dir()?;
    let db_path = atelier_dir.join("state.db");
    Database::open(&db_path).context("Failed to open database")
}

fn get_fresh_projection_db() -> Result<Database> {
    let db = get_db()?;
    ensure_projection_fresh_for_query(&db)?;
    Ok(db)
}

fn ensure_projection_fresh_for_query(db: &Database) -> Result<()> {
    let repo_root = find_repo_root_for_rebuild()?;
    let state_dir = repo_root.join(".atelier-state");
    if state_dir.is_dir() {
        projection_index::ensure_fresh(db, &state_dir)?;
    }
    Ok(())
}

fn export_current_state(db: &Database) -> Result<()> {
    let root = find_repo_root_for_rebuild()?;
    commands::export::write_canonical_from_db(db, &root.join(".atelier-state"))
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
            let db = get_db()?;
            if issue_type.is_some() || parent.is_some() {
                commands::agent_factory::create(
                    &db,
                    commands::agent_factory::CreateInput {
                        title: &title,
                        description: description.as_deref(),
                        priority: &priority,
                        issue_type: issue_type.as_deref().or(template.as_deref()),
                        labels: &label,
                        parent: parent.as_deref(),
                    },
                )?;
                return export_current_state(&db);
            }
            let atelier_dir = find_atelier_dir().ok();
            let opts = commands::create::CreateOpts {
                labels: &label,
                work,
                quiet,
                atelier_dir: atelier_dir.as_deref(),
            };
            commands::create::run(
                &db,
                &title,
                description.as_deref(),
                &priority,
                template.as_deref(),
                &opts,
            )?;
            export_current_state(&db)
        }

        IssueCommands::Quick {
            title,
            description,
            priority,
            template,
            label,
        } => {
            let db = get_db()?;
            let atelier_dir = find_atelier_dir().ok();
            let opts = commands::create::CreateOpts {
                labels: &label,
                work: true,
                quiet,
                atelier_dir: atelier_dir.as_deref(),
            };
            commands::create::run(
                &db,
                &title,
                description.as_deref(),
                &priority,
                template.as_deref(),
                &opts,
            )?;
            export_current_state(&db)
        }

        IssueCommands::Subissue {
            parent,
            title,
            description,
            priority,
            label,
            work,
        } => {
            let db = get_db()?;
            let atelier_dir = find_atelier_dir().ok();
            let opts = commands::create::CreateOpts {
                labels: &label,
                work,
                quiet,
                atelier_dir: atelier_dir.as_deref(),
            };
            commands::create::run_subissue(
                &db,
                &parent,
                &title,
                description.as_deref(),
                &priority,
                &opts,
            )?;
            export_current_state(&db)
        }

        IssueCommands::List {
            status,
            label,
            priority,
        } => {
            let db = get_fresh_projection_db()?;
            commands::agent_factory::list(
                &db,
                Some(&status),
                label.as_deref(),
                priority.as_deref(),
                quiet,
            )
        }

        IssueCommands::Search { query } => {
            let db = get_fresh_projection_db()?;
            commands::agent_factory::search(&db, &query, quiet)
        }

        IssueCommands::Show { id } => {
            let db = get_fresh_projection_db()?;
            commands::agent_factory::show(&db, &id)
        }

        IssueCommands::Update {
            id,
            title,
            description,
            priority,
            status,
            label,
            parent,
            no_parent,
            claim,
            append_notes,
        } => {
            let db = get_db()?;
            commands::agent_factory::update(
                &db,
                commands::agent_factory::UpdateInput {
                    issue_ref: &id,
                    title: title.as_deref(),
                    description: description.as_deref(),
                    priority: priority.as_deref(),
                    status: status.as_deref(),
                    labels: &label,
                    parent: if no_parent {
                        Some(None)
                    } else {
                        parent.as_deref().map(Some)
                    },
                    claim,
                    append_notes: append_notes.as_deref(),
                },
            )?;
            export_current_state(&db)
        }

        IssueCommands::Close { id, reason } => {
            let db = get_db()?;
            let _ = quiet;
            commands::agent_factory::close(&db, &id, reason.as_deref())?;
            export_current_state(&db)
        }

        IssueCommands::CloseAll { label, priority } => {
            let db = get_db()?;
            commands::status::close_all(&db, label.as_deref(), priority.as_deref())
        }

        IssueCommands::Reopen { id } => {
            let db = get_db()?;
            commands::agent_factory::reopen(&db, &id)?;
            export_current_state(&db)
        }

        IssueCommands::Delete { id, force } => {
            let db = get_db()?;
            commands::delete::run(&db, &id, force)
        }

        IssueCommands::Comment { id, text, kind } => {
            let db = get_db()?;
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            export_current_state(&db)?;
            commands::comment::run(&db, &resolved, &text, &kind)?;
            export_current_state(&db)
        }

        IssueCommands::Label { id, label } => {
            let db = get_db()?;
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            commands::label::add(&db, &resolved, &label)?;
            export_current_state(&db)
        }

        IssueCommands::Unlabel { id, label } => {
            let db = get_db()?;
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            commands::label::remove(&db, &resolved, &label)?;
            export_current_state(&db)
        }

        IssueCommands::Block { id, blocker } => {
            let db = get_db()?;
            commands::agent_factory::dep_add(&db, &id, &blocker)?;
            export_current_state(&db)
        }

        IssueCommands::Unblock { id, blocker } => {
            let db = get_db()?;
            commands::agent_factory::dep_remove(&db, &id, &blocker)?;
            export_current_state(&db)
        }

        IssueCommands::Blocked => {
            let db = get_fresh_projection_db()?;
            commands::deps::list_blocked(&db)
        }

        IssueCommands::Ready => {
            let db = get_fresh_projection_db()?;
            commands::agent_factory::ready(&db, quiet)
        }

        IssueCommands::Relate {
            id,
            related,
            relation_type,
        } => {
            let db = get_db()?;
            commands::relate::add_typed(&db, &id, &related, &relation_type)?;
            export_current_state(&db)
        }

        IssueCommands::Unrelate {
            id,
            related,
            relation_type,
        } => {
            let db = get_db()?;
            commands::relate::remove_typed(&db, &id, &related, &relation_type)?;
            export_current_state(&db)
        }

        IssueCommands::Related { id } => {
            let db = get_fresh_projection_db()?;
            commands::relate::list(&db, &id)
        }

        IssueCommands::Impact { id } => {
            let db = get_fresh_projection_db()?;
            commands::relate::impact(&db, &id)
        }

        IssueCommands::Next => {
            let db = get_fresh_projection_db()?;
            let atelier_dir = find_atelier_dir()?;
            commands::next::run(&db, &atelier_dir)
        }

        IssueCommands::Tree { status, compact } => {
            let db = get_fresh_projection_db()?;
            if compact {
                commands::tree::run_compact(&db, Some(&status))
            } else {
                commands::tree::run(&db, Some(&status))
            }
        }

        IssueCommands::Tested => {
            let atelier_dir = find_atelier_dir()?;
            commands::tested::run(&atelier_dir)
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
    let cli = Cli::parse();
    init_tracing(&cli.log_level, &cli.log_format);
    let quiet = cli.quiet;

    let result = match cli.command {
        Commands::Init { force } => {
            let cwd = env::current_dir()?;
            commands::init::run(&cwd, force)
        }

        Commands::Issue { action } => dispatch_issue(action, quiet),

        Commands::Export { output, check } => {
            let db = get_db()?;
            let atelier_dir = find_atelier_dir()?;
            let repo_root = atelier_dir
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Cannot determine repository root"))?;
            let state_dir = output
                .as_deref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| repo_root.join(".atelier-state"));
            commands::agent_factory::export_canonical(&db, &state_dir, check)
        }

        Commands::Rebuild { input } => {
            let repo_root = find_repo_root_for_rebuild()?;
            let state_dir = input
                .as_deref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| repo_root.join(".atelier-state"));
            let db_path = repo_root.join(".atelier").join("state.db");
            commands::agent_factory::rebuild(&state_dir, &db_path)
        }

        Commands::ImportBeads { input, output } => {
            let db = get_db()?;
            let atelier_dir = find_atelier_dir()?;
            let repo_root = atelier_dir
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Cannot determine repository root"))?;
            let state_dir = output
                .as_deref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| repo_root.join(".atelier-state"));
            commands::import::run_beads_jsonl(&db, std::path::Path::new(&input), &state_dir)
        }

        Commands::Dep { action } => match action {
            DepCommands::Add { blocked, blocker } => {
                let db = get_db()?;
                commands::agent_factory::dep_add(&db, &blocked, &blocker)
            }
            DepCommands::Remove { blocked, blocker } => {
                let db = get_db()?;
                commands::agent_factory::dep_remove(&db, &blocked, &blocker)
            }
            DepCommands::List { issue } => {
                let db = get_fresh_projection_db()?;
                commands::agent_factory::dep_list(&db, issue.as_deref())
            }
        },

        Commands::Mission { action } => {
            let db = get_db()?;
            match action {
                MissionCommands::Create {
                    title,
                    body,
                    constraint,
                    risk,
                    validation,
                } => {
                    commands::mission::create(
                        &db,
                        &title,
                        body.as_deref(),
                        constraint,
                        risk,
                        validation,
                    )?;
                    export_current_state(&db)
                }
                MissionCommands::Show { id } => commands::mission::show(&db, &id),
                MissionCommands::List { status } => commands::mission::list(&db, status.as_deref()),
                MissionCommands::Update {
                    id,
                    title,
                    status,
                    body,
                    constraint,
                    risk,
                    validation,
                } => {
                    commands::mission::update(
                        &db,
                        &id,
                        title.as_deref(),
                        status.as_deref(),
                        body.as_deref(),
                        constraint,
                        risk,
                        validation,
                    )?;
                    export_current_state(&db)
                }
            }
        }

        Commands::Plan { action } => {
            let db = get_db()?;
            match action {
                PlanCommands::Create {
                    title,
                    body,
                    reason,
                } => {
                    commands::plan::create(&db, &title, body.as_deref(), reason.as_deref())?;
                    export_current_state(&db)
                }
                PlanCommands::Show { id } => commands::plan::show(&db, &id),
                PlanCommands::Apply {
                    input,
                    dry_run,
                    validate_only,
                } => commands::plan::apply(&db, &input, dry_run, validate_only),
                PlanCommands::List { status } => commands::plan::list(&db, status.as_deref()),
                PlanCommands::Revise { id, body, reason } => {
                    commands::plan::revise(&db, &id, &body, reason.as_deref())?;
                    export_current_state(&db)
                }
                PlanCommands::Link {
                    id,
                    target_kind,
                    target_id,
                    relation_type,
                } => {
                    commands::plan::link(&db, &id, &target_kind, &target_id, &relation_type)?;
                    export_current_state(&db)
                }
            }
        }

        Commands::Link { action } => {
            let db = get_db()?;
            match action {
                LinkCommands::Add {
                    source_kind,
                    source_id,
                    target_kind,
                    target_id,
                    relation_type,
                } => {
                    commands::link::add(
                        &db,
                        &source_kind,
                        &source_id,
                        &target_kind,
                        &target_id,
                        &relation_type,
                    )?;
                    export_current_state(&db)
                }
                LinkCommands::Remove {
                    source_kind,
                    source_id,
                    target_kind,
                    target_id,
                    relation_type,
                } => {
                    commands::link::remove(
                        &db,
                        &source_kind,
                        &source_id,
                        &target_kind,
                        &target_id,
                        &relation_type,
                    )?;
                    export_current_state(&db)
                }
                LinkCommands::List { kind, id } => commands::link::list(&db, &kind, &id),
            }
        }

        Commands::Evidence { action } => {
            let db = get_db()?;
            match action {
                EvidenceCommands::Add {
                    evidence_kind,
                    result,
                    summary,
                    path,
                    uri,
                    producer,
                } => {
                    commands::evidence::add(
                        &db,
                        &evidence_kind,
                        &result,
                        &summary,
                        path.as_deref(),
                        uri.as_deref(),
                        producer.as_deref(),
                    )?;
                    export_current_state(&db)
                }
                EvidenceCommands::Show { id } => commands::evidence::show(&db, &id),
                EvidenceCommands::List { result } => {
                    commands::evidence::list(&db, result.as_deref())
                }
            }
        }

        Commands::Workflow { action } => {
            let db = get_db()?;
            match action {
                WorkflowCommands::Validate {
                    target_kind,
                    target_id,
                    transition,
                    validator,
                } => commands::workflow::validate(
                    &db,
                    &target_kind,
                    &target_id,
                    &transition,
                    validator,
                ),
            }
        }

        Commands::Work { action } => {
            let db = get_db()?;
            match action {
                WorkCommands::Start { id } => commands::work::start(&db, &id),
                WorkCommands::Finish { id } => commands::work::finish(&db, &id),
                WorkCommands::Status => commands::work::status(&db),
                WorkCommands::Worktree { action } => match action {
                    WorktreeCommands::For { id, path } => {
                        commands::work::worktree_for(&db, &id, path.as_deref())
                    }
                    WorktreeCommands::Status => commands::work::worktree_status(&db),
                    WorktreeCommands::Merge { id } => commands::work::worktree_merge(&db, &id),
                    WorktreeCommands::Remove { id, force } => {
                        commands::work::worktree_remove(&db, &id, force)
                    }
                },
            }
        }

        Commands::Worktree { action } => {
            let db = get_db()?;
            match action {
                WorktreeCommands::For { id, path } => {
                    commands::work::worktree_for(&db, &id, path.as_deref())
                }
                WorktreeCommands::Status => commands::work::worktree_status(&db),
                WorktreeCommands::Merge { id } => commands::work::worktree_merge(&db, &id),
                WorktreeCommands::Remove { id, force } => {
                    commands::work::worktree_remove(&db, &id, force)
                }
            }
        }

        Commands::Lint { id } => {
            let db = get_fresh_projection_db()?;
            commands::agent_factory::lint(&db, id.as_deref())
        }

        Commands::Doctor => {
            let db = get_db()?;
            let atelier_dir = find_atelier_dir()?;
            let repo_root = atelier_dir
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Cannot determine repository root"))?;
            let state_dir = repo_root.join(".atelier-state");
            commands::agent_factory::doctor(&db, repo_root, &state_dir)
        }
    };

    result
}
