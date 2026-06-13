mod activity;
mod command_surface;
mod commands;
mod db;
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

use anyhow::{bail, Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;
use std::time::Instant;

use db::Database;
use record_store::RecordStore;

#[derive(Parser)]
#[command(name = "atelier")]
#[command(about = "A simple, lean issue tracker CLI")]
#[command(help_template = "{about-section}\nUsage: {usage}\n\n{after-help}\nOptions:\n{options}")]
#[command(after_help = "Setup:
  init          Initialize Atelier in the current repository

Orientation:
  prime         Show repository operating guidance for recovery and onboarding
  status        Show checkout, mission, work, and tracker signposts
  start         Start tracked work on an issue
  abandon       Clear active local work without changing issue status

Issues:
  issue         Create, list, show, update, and close issues
  dep           Manage issue blockers with add, remove, and list
  search        Search issue text
  link          Manage typed issue links
  graph         Inspect issue hierarchy and impact
  note          Add issue activity notes

Missions and planning:
  mission       Create, list, show, status, and update durable missions
  plan          Create, apply, revise, list, and link durable plans

Records:
  evidence      Capture validation evidence
  history       Inspect canonical repo, mission, issue, or epic activity

Advanced work:
  worktree      Create, inspect, merge, and remove issue worktrees

State management:
  export        Write or check canonical tracker records
  rebuild       Rebuild local SQLite state from canonical tracker records
  import-beads  Import an external Beads JSONL backup

Integrations:
  integrations  Install optional integrations such as Claude hooks

Maintenance:
  maintenance   Run explicit destructive maintenance commands
  diagnostics   Inspect local command diagnostics
  lint          Validate tracker records
  doctor        Check runtime and exported-state health

Common commands:
  atelier prime
  atelier status
  atelier issue list
  atelier issue list --ready
  atelier issue show <id>
  atelier mission list
  atelier mission show <id>
  atelier mission status
  atelier history --mission <id>
  atelier history --issue <id>
  atelier start <issue-id>
  atelier abandon [issue-id] --reason \"...\"
  atelier issue transition <issue-id> --options
  atelier issue close <issue-id> --reason \"...\"
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
        /// Reconcile core tracker state even if already initialized
        #[arg(short, long)]
        force: bool,
    },

    /// Show repository operating guidance for recovery and onboarding
    Prime,

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

    /// Typed issue link commands
    Link {
        #[command(subcommand)]
        action: LinkCommands,
    },

    /// Issue graph and hierarchy commands
    Graph {
        #[command(subcommand)]
        action: GraphCommands,
    },

    /// Issue activity note commands
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
    ImportBeads {
        /// Beads JSONL backup path from an external source
        input: String,
        /// Canonical state directory to write after import
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Optional external tool integrations
    Integrations {
        #[command(subcommand)]
        action: IntegrationCommands,
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

    /// Advanced/internal workflow policy diagnostics
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

    /// Local command diagnostics
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

    /// Check tracker runtime and exported-state health
    Doctor,
}

#[derive(Subcommand)]
enum IntegrationCommands {
    /// Claude Code hooks and MCP setup
    Claude {
        #[command(subcommand)]
        action: ClaudeIntegrationCommands,
    },
}

#[derive(Subcommand)]
enum ClaudeIntegrationCommands {
    /// Install or update the optional Claude Code integration
    Install {
        /// Overwrite Atelier-managed Claude integration files
        #[arg(short, long)]
        force: bool,
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

    /// Quick-create an issue and start working on it (create + label + session work)
    #[command(hide = true)]
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
    #[command(hide = true)]
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

    /// Search issues by text
    #[command(hide = true)]
    Search {
        /// Search query
        query: String,
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
        /// Explicit terminal workflow status when multiple done targets are available
        #[arg(long)]
        to: Option<String>,
        /// Close reason recorded in issue activity
        #[arg(short, long)]
        reason: String,
    },

    /// Close all issues matching filters
    #[command(hide = true)]
    CloseAll {
        /// Filter by label
        #[arg(short, long)]
        label: Option<String>,
        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,
    },

    /// Delete an issue
    #[command(hide = true)]
    Delete {
        /// Issue ID
        id: String,
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Add a comment to an issue
    #[command(hide = true)]
    Comment {
        /// Issue ID
        id: String,
        /// Comment text
        text: String,
        /// Comment kind (note, plan, observation, blocker, resolution, result, handoff, human)
        #[arg(long, default_value = "note")]
        kind: String,
    },

    /// Add a label to an issue
    #[command(hide = true)]
    Label {
        /// Issue ID
        id: String,
        /// Label name
        label: String,
    },

    /// Remove a label from an issue
    #[command(hide = true)]
    Unlabel {
        /// Issue ID
        id: String,
        /// Label name
        label: String,
    },

    /// Mark an issue as blocked by another
    #[command(hide = true)]
    Block {
        /// Issue ID that is blocked
        id: String,
        /// Issue ID that is blocking
        blocker: String,
    },

    /// Remove a blocking relationship
    #[command(hide = true)]
    Unblock {
        /// Issue ID that was blocked
        id: String,
        /// Issue ID that was blocking
        blocker: String,
    },

    /// List blocked issues
    #[command(hide = true)]
    Blocked,

    /// Link two related issues
    #[command(hide = true)]
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
    #[command(hide = true)]
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
    #[command(hide = true)]
    Related {
        /// Issue ID
        id: String,
    },

    /// Show downstream issue impact from hierarchy and impact-bearing links
    #[command(hide = true)]
    Impact {
        /// Issue ID to check impact from
        id: String,
    },

    /// Suggest the next issue to work on
    #[command(hide = true)]
    Next,

    /// Show issues as a tree hierarchy
    #[command(hide = true)]
    Tree {
        /// Filter by status (todo, done, all)
        #[arg(short, long, default_value = "all")]
        status: String,
        /// Show a bounded, scan-friendly hierarchy instead of the full tree
        #[arg(long)]
        compact: bool,
    },

    /// Mark tests as run (resets test reminder)
    #[command(hide = true)]
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
enum LinkCommands {
    /// Add a typed link between two records
    Add {
        source_kind: String,
        source_id: String,
        target_kind: String,
        target_id: String,
        /// Relation type
        #[arg(short = 't', long = "type", default_value = "related")]
        relation_type: String,
    },
    /// Remove a typed link between two records
    Remove {
        source_kind: String,
        source_id: String,
        target_kind: String,
        target_id: String,
        /// Relation type
        #[arg(short = 't', long = "type", default_value = "related")]
        relation_type: String,
    },
    /// List typed links for a record
    List {
        target_kind: String,
        target_id: String,
    },
}

#[derive(Subcommand)]
enum GraphCommands {
    /// Show downstream impact from hierarchy and impact-bearing links
    Impact {
        /// Issue ID
        id: String,
    },
    /// Show issues as a tree hierarchy
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
    /// Add an activity note to a target
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
    /// Focus a mission as the active orchestration context
    Start {
        id: String,
        /// Replace any currently active mission focus
        #[arg(long = "switch")]
        switch_active: bool,
    },
    /// Show mission-control status for one mission or all current missions
    Status {
        /// Show closeout contract audit detail for the mission
        #[arg(long)]
        closeout: bool,
        /// Show verbose validator detail in the status summary
        #[arg(long)]
        verbose: bool,
        id: Option<String>,
    },
    /// Audit mission validation and linked epic outcomes against proof
    Audit { id: String },
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
    /// Add issue work to a mission
    AddWork { id: String, issue: String },
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
    Record {
        #[arg(long = "kind")]
        evidence_kind: String,
        #[arg(long)]
        result: String,
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
    /// Add validation evidence
    #[command(hide = true)]
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
    /// Capture a command transcript as validation evidence
    #[command(hide = true)]
    Capture {
        #[arg(long = "kind")]
        evidence_kind: String,
        #[arg(long)]
        result: String,
        #[arg(long)]
        summary: Option<String>,
        #[arg(long)]
        path: Option<String>,
        #[arg(long)]
        uri: Option<String>,
        #[arg(long)]
        producer: Option<String>,
        #[arg(long)]
        target_kind: Option<String>,
        #[arg(long)]
        target_id: Option<String>,
        #[arg(long, default_value = "validates")]
        role: String,
        #[arg(last = true, required = true, num_args = 1..)]
        command: Vec<String>,
    },
    /// Show an evidence record
    Show { id: String },
    /// Attach evidence to a target record
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
    /// Write the starter .atelier/workflow.yaml policy
    Init {
        #[arg(long)]
        force: bool,
    },
    /// Validate .atelier/workflow.yaml policy and current issue-record health
    Check,
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
    /// Clear a stale local worktree association after interrupted setup/removal
    Repair { id: String },
}

#[derive(Subcommand)]
enum DiagnosticsCommands {
    /// Summarize slow command telemetry as stable JSON
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

fn find_atelier_dir() -> Result<PathBuf> {
    storage_layout::find_atelier_dir()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandStorageAccess {
    /// Read/query commands that depend on a fresh SQLite projection.
    ProjectionQuery,
    /// Orientation reads that can fall back to the existing projection when
    /// canonical records are malformed, as long as the degraded state is named.
    DegradedProjectionQuery,
    /// Commands that write canonical Markdown and then refresh the projection.
    CanonicalMutation,
    /// Runtime-local commands that must not refresh or mutate canonical state.
    RuntimeOnly,
    /// Diagnostics, export, rebuild, and repair flows that own freshness policy.
    HealthRepair,
}

impl CommandStorageAccess {
    fn requires_fresh_projection(self) -> bool {
        matches!(
            self,
            CommandStorageAccess::ProjectionQuery
                | CommandStorageAccess::DegradedProjectionQuery
                | CommandStorageAccess::CanonicalMutation
        )
    }

    fn allows_degraded_projection(self) -> bool {
        matches!(self, CommandStorageAccess::DegradedProjectionQuery)
    }
}

struct CommandStorage {
    layout: storage_layout::StorageLayout,
    db: Database,
    runtime_db_existed: bool,
}

impl CommandStorage {
    fn db(&self) -> &Database {
        &self.db
    }

    fn into_db(self) -> Database {
        self.db
    }

    fn state_dir(&self) -> PathBuf {
        self.layout.canonical_dir()
    }

    fn db_path(&self) -> PathBuf {
        self.layout.runtime_db_path()
    }

    fn state_and_db_paths(&self) -> (PathBuf, PathBuf) {
        (self.state_dir(), self.db_path())
    }

    fn repo_root(&self) -> &std::path::Path {
        self.layout.repo_root()
    }
}

fn command_storage(mode: CommandStorageAccess) -> Result<CommandStorage> {
    let layout = storage_layout::StorageLayout::discover()?;
    let runtime_db_existed = layout.runtime_db_path().exists();
    let db = Database::open(&layout.runtime_db_path()).context("Failed to open database")?;
    let db = if mode.requires_fresh_projection() {
        ensure_fresh_projection_db(
            db,
            &layout,
            runtime_db_existed,
            mode.allows_degraded_projection(),
        )?
    } else {
        db
    };
    Ok(CommandStorage {
        layout,
        db,
        runtime_db_existed,
    })
}

fn ensure_fresh_projection_db(
    db: Database,
    layout: &storage_layout::StorageLayout,
    runtime_db_existed: bool,
    allow_degraded_projection: bool,
) -> Result<Database> {
    let state_dir = layout.canonical_dir();
    if state_dir.is_dir() {
        if !runtime_db_existed {
            commands::rebuild::validate_canonical_state(&state_dir).map_err(|error| {
                projection_validation_error(error, "Runtime projection database is missing")
            })?;
            let db_path = layout.runtime_db_path();
            drop(db);
            commands::rebuild::run(&state_dir, &db_path).with_context(|| {
                format!(
                    "Runtime projection database is missing and automatic rebuild failed for {}",
                    state_dir.display()
                )
            })?;
            eprintln!(
                "Runtime projection database was missing; rebuilt local SQLite projection from {}",
                state_dir.display()
            );
            return Database::open(&db_path).context("Failed to open database");
        }

        let report = projection_index::check(&db, &state_dir)?;
        if !report.is_fresh() {
            if let Err(error) = commands::rebuild::validate_canonical_state(&state_dir) {
                if allow_degraded_projection {
                    eprintln!(
                        "Tracker degraded: canonical tracker Markdown is invalid; using existing local projection for orientation only."
                    );
                    eprintln!("Repair: run `atelier lint` for record diagnostics, then fix the named Markdown before closing or mutating work.");
                    eprintln!(
                        "Projection freshness: {}",
                        report.problem_messages().join("; ")
                    );
                    eprintln!("Canonical diagnostic: {error:#}");
                    return Ok(db);
                }
                return Err(projection_validation_error(
                    error,
                    "Canonical tracker Markdown is invalid",
                ));
            }
            let db_path = layout.runtime_db_path();
            drop(db);
            commands::rebuild::run(&state_dir, &db_path).with_context(|| {
                format!(
                    "Projection index is stale and automatic rebuild failed for {}\n{}",
                    state_dir.display(),
                    report.problem_messages().join("\n")
                )
            })?;
            eprintln!(
                "Projection index was stale; rebuilt local SQLite projection from {}",
                state_dir.display()
            );
            return Database::open(&db_path).context("Failed to open database");
        }
    }
    Ok(db)
}

fn projection_validation_error(error: anyhow::Error, prefix: &str) -> anyhow::Error {
    let detail = format!("{error:#}");
    if looks_like_schema_drift(&detail) {
        error.context(format!(
            "{prefix}: canonical tracker records use a schema this atelier binary does not understand. \
             Rebuild and use `target/debug/atelier` when testing local CLI changes, or update the installed `atelier` binary before continuing."
        ))
    } else {
        error.context(format!(
            "{prefix}; run `atelier lint` for details, then fix canonical tracker records before querying."
        ))
    }
}

fn looks_like_schema_drift(detail: &str) -> bool {
    detail.contains("Unsupported schema") || detail.contains("Unsupported schema_version")
}

fn state_and_db_paths() -> Result<(PathBuf, PathBuf)> {
    Ok(command_storage(CommandStorageAccess::CanonicalMutation)?.state_and_db_paths())
}

fn runtime_db() -> Result<Database> {
    Ok(command_storage(CommandStorageAccess::RuntimeOnly)?.into_db())
}

fn projection_query_db() -> Result<Database> {
    Ok(command_storage(CommandStorageAccess::ProjectionQuery)?.into_db())
}

fn degraded_projection_query_db() -> Result<Database> {
    Ok(command_storage(CommandStorageAccess::DegradedProjectionQuery)?.into_db())
}

fn lint_db() -> Result<Database> {
    let layout = storage_layout::StorageLayout::discover()?;
    if layout.runtime_db_path().exists() {
        Ok(command_storage(CommandStorageAccess::RuntimeOnly)?.into_db())
    } else {
        projection_query_db()
    }
}

fn canonical_mutation_db() -> Result<Database> {
    Ok(command_storage(CommandStorageAccess::CanonicalMutation)?.into_db())
}

#[cfg(test)]
mod command_storage_tests {
    use super::CommandStorageAccess;

    #[test]
    fn access_modes_declare_projection_freshness_policy() {
        assert!(CommandStorageAccess::ProjectionQuery.requires_fresh_projection());
        assert!(CommandStorageAccess::DegradedProjectionQuery.requires_fresh_projection());
        assert!(CommandStorageAccess::CanonicalMutation.requires_fresh_projection());
        assert!(!CommandStorageAccess::RuntimeOnly.requires_fresh_projection());
        assert!(!CommandStorageAccess::HealthRepair.requires_fresh_projection());
        assert!(!CommandStorageAccess::ProjectionQuery.allows_degraded_projection());
        assert!(CommandStorageAccess::DegradedProjectionQuery.allows_degraded_projection());
        assert!(!CommandStorageAccess::CanonicalMutation.allows_degraded_projection());
    }
}

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

        IssueCommands::Quick {
            title,
            description,
            priority,
            template,
            label,
        } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            let (final_priority, final_description, labels, issue_type) = issue_create_parts(
                &priority,
                description.as_deref(),
                template.as_deref(),
                &label,
                None,
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
                    parent: None,
                    work: true,
                    quiet,
                },
            )
        }

        IssueCommands::Subissue {
            parent,
            title,
            description,
            priority,
            label,
            work,
        } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            commands::agent_factory::create_lifecycle(
                &state_dir,
                &db_path,
                commands::agent_factory::LifecycleCreateInput {
                    title: &title,
                    description: description.as_deref(),
                    priority: &priority,
                    issue_type: "task",
                    labels: &label,
                    parent: Some(&parent),
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

        IssueCommands::Search { query } => {
            let db = degraded_projection_query_db()?;
            commands::agent_factory::search(&db, &query, quiet)
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
            append_notes,
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
                    append_notes: append_notes.as_deref(),
                },
            )
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

        IssueCommands::CloseAll { label, priority } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            commands::status::close_all_lifecycle(
                &state_dir,
                &db_path,
                label.as_deref(),
                priority.as_deref(),
            )
        }

        IssueCommands::Delete { id, force } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            let db = canonical_mutation_db()?;
            let id = resolve_issue_arg(&db, &id)?;
            drop(db);
            commands::delete::run_lifecycle(&state_dir, &db_path, &id, force)
        }

        IssueCommands::Comment { id, text, kind } => {
            let db = canonical_mutation_db()?;
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            commands::comment::run_canonical(&db, &resolved, &text, &kind)
        }

        IssueCommands::Label { id, label } => {
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            commands::label::add_canonical(&db, &store, &resolved, &label)?;
            drop(db);
            commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        IssueCommands::Unlabel { id, label } => {
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            commands::label::remove_canonical(&db, &store, &resolved, &label)?;
            drop(db);
            commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
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

        IssueCommands::Blocked => {
            let db = projection_query_db()?;
            commands::deps::list_blocked(&db)
        }

        IssueCommands::Relate {
            id,
            related,
            relation_type,
        } => {
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            let id = resolve_issue_arg(&db, &id)?;
            let related = resolve_issue_arg(&db, &related)?;
            commands::relate::add_typed_canonical(&db, &store, &id, &related, &relation_type)?;
            drop(db);
            commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        IssueCommands::Unrelate {
            id,
            related,
            relation_type,
        } => {
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            let id = resolve_issue_arg(&db, &id)?;
            let related = resolve_issue_arg(&db, &related)?;
            commands::relate::remove_typed_canonical(&db, &store, &id, &related, &relation_type)?;
            drop(db);
            commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        IssueCommands::Related { id } => {
            let db = projection_query_db()?;
            let id = resolve_issue_arg(&db, &id)?;
            commands::relate::list(&db, &id)
        }

        IssueCommands::Impact { id } => {
            let db = projection_query_db()?;
            let id = resolve_issue_arg(&db, &id)?;
            commands::relate::impact(&db, &id)
        }

        IssueCommands::Next => {
            let db = projection_query_db()?;
            commands::next::run(&db)
        }

        IssueCommands::Tree { status, compact } => {
            let db = projection_query_db()?;
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
    let command_name = command_identity(&cli.command);
    let started_at = Utc::now();
    let started = Instant::now();

    let result = match cli.command {
        Commands::Init { force } => {
            let cwd = env::current_dir()?;
            commands::init::run(&cwd, force)
        }

        Commands::Prime => {
            let storage = command_storage(CommandStorageAccess::DegradedProjectionQuery)?;
            let repo_root = storage.repo_root().to_path_buf();
            commands::prime::run(storage.db(), &storage.state_dir(), &repo_root)
        }

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

        Commands::Issue { action } => dispatch_issue(action, quiet),

        Commands::Search { query } => {
            let db = degraded_projection_query_db()?;
            commands::agent_factory::search(&db, &query, quiet)
        }

        Commands::Link { action } => match action {
            LinkCommands::Add {
                source_kind,
                source_id,
                target_kind,
                target_id,
                relation_type,
            } => {
                require_issue_kind(&source_kind, "atelier link add")?;
                require_issue_kind(&target_kind, "atelier link add")?;
                let db = canonical_mutation_db()?;
                let (state_dir, db_path) = state_and_db_paths()?;
                let store = RecordStore::new(&state_dir);
                let source_id = resolve_issue_arg(&db, &source_id)?;
                let target_id = resolve_issue_arg(&db, &target_id)?;
                commands::relate::add_typed_canonical(
                    &db,
                    &store,
                    &source_id,
                    &target_id,
                    &relation_type,
                )?;
                drop(db);
                commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
            }
            LinkCommands::Remove {
                source_kind,
                source_id,
                target_kind,
                target_id,
                relation_type,
            } => {
                require_issue_kind(&source_kind, "atelier link remove")?;
                require_issue_kind(&target_kind, "atelier link remove")?;
                let db = canonical_mutation_db()?;
                let (state_dir, db_path) = state_and_db_paths()?;
                let store = RecordStore::new(&state_dir);
                let source_id = resolve_issue_arg(&db, &source_id)?;
                let target_id = resolve_issue_arg(&db, &target_id)?;
                commands::relate::remove_typed_canonical(
                    &db,
                    &store,
                    &source_id,
                    &target_id,
                    &relation_type,
                )?;
                drop(db);
                commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
            }
            LinkCommands::List {
                target_kind,
                target_id,
            } => {
                require_issue_kind(&target_kind, "atelier link list")?;
                let db = projection_query_db()?;
                let target_id = resolve_issue_arg(&db, &target_id)?;
                commands::relate::list(&db, &target_id)
            }
        },

        Commands::Graph { action } => match action {
            GraphCommands::Impact { id } => {
                let db = projection_query_db()?;
                let id = resolve_issue_arg(&db, &id)?;
                commands::relate::impact(&db, &id)
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
                require_issue_kind(&target_kind, "atelier note add")?;
                let db = canonical_mutation_db()?;
                let target_id = resolve_issue_arg(&db, &target_id)?;
                commands::comment::run_canonical(&db, &target_id, &text, &kind)
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

        Commands::Integrations { action } => match action {
            IntegrationCommands::Claude { action } => match action {
                ClaudeIntegrationCommands::Install { force } => {
                    let repo_root = storage_layout::find_repo_root()?;
                    commands::integrations::install_claude(&repo_root, force)
                }
            },
        },

        Commands::Dep { action } => match action {
            DepCommands::Add { blocked, blocker } => {
                let db = canonical_mutation_db()?;
                let (state_dir, db_path) = state_and_db_paths()?;
                let store = RecordStore::new(&state_dir);
                commands::agent_factory::dep_add_canonical(&db, &store, &blocked, &blocker)?;
                drop(db);
                commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
            }
            DepCommands::Remove { blocked, blocker } => {
                let db = canonical_mutation_db()?;
                let (state_dir, db_path) = state_and_db_paths()?;
                let store = RecordStore::new(&state_dir);
                commands::agent_factory::dep_remove_canonical(&db, &store, &blocked, &blocker)?;
                drop(db);
                commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
            }
            DepCommands::List { issue } => {
                let db = projection_query_db()?;
                commands::agent_factory::dep_list(&db, issue.as_deref())
            }
        },

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
                commands::mission::audit(storage.db(), &storage.state_dir(), &id, quiet)
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
            MissionCommands::AddWork { id, issue } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let issue = resolve_issue_arg(storage.db(), &issue)?;
                commands::mission::add_work(&state_dir, &db_path, &id, &issue)
            }
            MissionCommands::AddBlocker { id, issue } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
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
            EvidenceCommands::Add {
                evidence_kind,
                result,
                summary,
                path,
                uri,
                producer,
            } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                commands::evidence::add(
                    &storage.state_dir(),
                    &storage.db_path(),
                    &evidence_kind,
                    &result,
                    &summary,
                    path.as_deref(),
                    uri.as_deref(),
                    producer.as_deref(),
                )
            }
            EvidenceCommands::Capture {
                evidence_kind,
                result,
                summary,
                path,
                uri,
                producer,
                target_kind,
                target_id,
                role,
                command,
            } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let target_id = match (target_kind.as_deref(), target_id.as_deref()) {
                    (Some(kind), Some(id)) => {
                        Some(resolve_evidence_target_arg(storage.db(), kind, id)?)
                    }
                    (None, None) => None,
                    _ => bail!("--target-kind and --target-id must be supplied together"),
                };
                commands::evidence::capture(
                    &storage.state_dir(),
                    &storage.db_path(),
                    commands::evidence::CaptureOptions {
                        evidence_kind: &evidence_kind,
                        result: &result,
                        summary: summary.as_deref(),
                        path: path.as_deref(),
                        uri: uri.as_deref(),
                        producer: producer.as_deref(),
                        target_kind: target_kind.as_deref(),
                        target_id: target_id.as_deref(),
                        role: &role,
                        command: &command,
                    },
                )
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
            WorkflowCommands::Init { force } => {
                let repo_root = storage_layout::find_repo_root()?;
                commands::workflow::init(&repo_root, force)
            }
            WorkflowCommands::Check => {
                let db = projection_query_db()?;
                commands::workflow::check(&db)
            }
        },

        Commands::Worktree { action } => {
            let db = runtime_db()?;
            match action {
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

        Commands::Doctor => {
            let storage = command_storage(CommandStorageAccess::HealthRepair)?;
            commands::agent_factory::doctor(
                storage.db(),
                storage.repo_root(),
                &storage.state_dir(),
                storage.runtime_db_existed,
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

fn command_identity(command: &Commands) -> &'static str {
    match command {
        Commands::Init { .. } => "init",
        Commands::Prime => "prime",
        Commands::Status => "status",
        Commands::Start { .. } => "start",
        Commands::Abandon { .. } => "abandon",
        Commands::Issue { action } => match action {
            IssueCommands::Create { .. } => "issue create",
            IssueCommands::Quick { .. } => "issue quick",
            IssueCommands::Subissue { .. } => "issue subissue",
            IssueCommands::List { .. } => "issue list",
            IssueCommands::Search { .. } => "issue search",
            IssueCommands::Show { .. } => "issue show",
            IssueCommands::Transition { .. } => "issue transition",
            IssueCommands::Update { .. } => "issue update",
            IssueCommands::Close { .. } => "issue close",
            IssueCommands::CloseAll { .. } => "issue close-all",
            IssueCommands::Delete { .. } => "issue delete",
            IssueCommands::Comment { .. } => "issue comment",
            IssueCommands::Label { .. } => "issue label",
            IssueCommands::Unlabel { .. } => "issue unlabel",
            IssueCommands::Block { .. } => "issue block",
            IssueCommands::Unblock { .. } => "issue unblock",
            IssueCommands::Blocked => "issue blocked",
            IssueCommands::Relate { .. } => "issue relate",
            IssueCommands::Unrelate { .. } => "issue unrelate",
            IssueCommands::Related { .. } => "issue related",
            IssueCommands::Impact { .. } => "issue impact",
            IssueCommands::Next => "issue next",
            IssueCommands::Tree { .. } => "issue tree",
            IssueCommands::Tested => "issue tested",
        },
        Commands::Search { .. } => "search",
        Commands::Link { action } => match action {
            LinkCommands::Add { .. } => "link add",
            LinkCommands::Remove { .. } => "link remove",
            LinkCommands::List { .. } => "link list",
        },
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
        Commands::Integrations { action } => match action {
            IntegrationCommands::Claude { action } => match action {
                ClaudeIntegrationCommands::Install { .. } => "integrations claude install",
            },
        },
        Commands::Dep { action } => match action {
            DepCommands::Add { .. } => "dep add",
            DepCommands::Remove { .. } => "dep remove",
            DepCommands::List { .. } => "dep list",
        },
        Commands::Mission { action } => match action {
            MissionCommands::Create { .. } => "mission create",
            MissionCommands::Show { .. } => "mission show",
            MissionCommands::Start { .. } => "mission start",
            MissionCommands::Status { .. } => "mission status",
            MissionCommands::Audit { .. } => "mission audit",
            MissionCommands::List { .. } => "mission list",
            MissionCommands::Update { .. } => "mission update",
            MissionCommands::AddWork { .. } => "mission add-work",
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
            EvidenceCommands::Add { .. } => "evidence add",
            EvidenceCommands::Record { .. } => "evidence record",
            EvidenceCommands::Capture { .. } => "evidence capture",
            EvidenceCommands::Show { .. } => "evidence show",
            EvidenceCommands::Attach { .. } => "evidence attach",
            EvidenceCommands::List { .. } => "evidence list",
        },
        Commands::History { .. } => "history",
        Commands::Workflow { action } => match action {
            WorkflowCommands::Init { .. } => "workflow init",
            WorkflowCommands::Check => "workflow check",
        },
        Commands::Worktree { action } => match action {
            WorktreeCommands::For { .. } => "worktree for",
            WorktreeCommands::Status => "worktree status",
            WorktreeCommands::Merge { .. } => "worktree merge",
            WorktreeCommands::Remove { .. } => "worktree remove",
            WorktreeCommands::Repair { .. } => "worktree repair",
        },
        Commands::Diagnostics { action } => match action {
            DiagnosticsCommands::Slow { .. } => "diagnostics slow",
        },
        Commands::Maintenance { action } => match action {
            MaintenanceCommands::Delete { .. } => "maintenance delete",
        },
        Commands::Lint { .. } => "lint",
        Commands::Doctor => "doctor",
    }
}
