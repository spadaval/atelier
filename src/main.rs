mod activity;
mod command_surface;
mod commands;
mod db;
mod identity;
mod lock_check;
mod locks;
mod models;
mod projection_index;
mod record_id;
mod record_store;
mod storage_layout;
mod sync;
mod telemetry;
mod test_inventory;
mod utils;

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
  finish        Finish tracked work, defaulting to active work

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
  atelier finish [issue-id]
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

    /// Finish tracked work, defaulting to active work
    Finish { id: Option<String> },

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

    /// Work lifecycle helpers
    #[command(hide = true)]
    Work {
        #[command(subcommand)]
        action: WorkCommands,
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
        /// Filter by status (open, closed, all)
        #[arg(short, long, default_value = "open")]
        status: String,
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
        /// Show the full option list
        #[arg(long)]
        options: bool,
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
        /// Closure reason
        #[arg(short, long)]
        reason: Option<String>,
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

    /// Reopen a closed issue
    #[command(hide = true)]
    Reopen {
        /// Issue ID
        id: String,
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
        /// Filter by status (open, closed, all)
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
        /// Filter by status (open, closed, all)
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
    Status { id: Option<String> },
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
    /// Capture a command transcript as validation evidence
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
    /// Evaluate workflow validators as an advanced diagnostic without mutating record state
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
            CommandStorageAccess::ProjectionQuery | CommandStorageAccess::CanonicalMutation
        )
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
        ensure_fresh_projection_db(db, &layout, runtime_db_existed)?
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
) -> Result<Database> {
    let state_dir = layout.canonical_dir();
    if state_dir.is_dir() {
        if !runtime_db_existed {
            commands::rebuild::validate_canonical_state(&state_dir).with_context(|| {
                "Runtime projection database is missing and canonical tracker Markdown is not rebuild-ready; \
                 run `atelier lint` for details, then fix canonical tracker records before querying."
                    .to_string()
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
            commands::rebuild::validate_canonical_state(&state_dir).with_context(|| {
                "Canonical tracker Markdown is invalid; run `atelier lint` for details, \
                 then fix canonical tracker records before querying."
                    .to_string()
            })?;
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

fn state_and_db_paths() -> Result<(PathBuf, PathBuf)> {
    Ok(command_storage(CommandStorageAccess::CanonicalMutation)?.state_and_db_paths())
}

fn runtime_db() -> Result<Database> {
    Ok(command_storage(CommandStorageAccess::RuntimeOnly)?.into_db())
}

fn projection_query_db() -> Result<Database> {
    Ok(command_storage(CommandStorageAccess::ProjectionQuery)?.into_db())
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
        assert!(CommandStorageAccess::CanonicalMutation.requires_fresh_projection());
        assert!(!CommandStorageAccess::RuntimeOnly.requires_fresh_projection());
        assert!(!CommandStorageAccess::HealthRepair.requires_fresh_projection());
    }
}

fn issue_create_parts(
    priority: &str,
    description: Option<&str>,
    template: Option<&str>,
    labels: &[String],
) -> Result<(String, Option<String>, Vec<String>, &'static str)> {
    let mut labels = labels.to_vec();
    let (final_priority, final_description) = if let Some(template_name) = template {
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
        (priority.to_string(), description)
    } else {
        (priority.to_string(), description.map(str::to_string))
    };

    if !commands::create::validate_priority(&final_priority) {
        bail!(
            "Invalid priority '{}'. Must be one of: low, medium, high, critical",
            final_priority
        );
    }
    Ok((final_priority, final_description, labels, "task"))
}

fn resolve_issue_arg(db: &Database, issue_ref: &str) -> Result<String> {
    commands::agent_factory::resolve_id(db, issue_ref)
}

fn resolve_record_arg(db: &Database, kind: &str, id: &str) -> Result<String> {
    if kind == "issue" {
        resolve_issue_arg(db, id)
    } else {
        Ok(id.to_string())
    }
}

fn require_issue_kind(kind: &str, command: &str) -> Result<()> {
    if kind != "issue" {
        bail!("{command} currently supports issue records only; got '{kind}'");
    }
    Ok(())
}

fn issue_compat_guidance(replacement: &str) {
    eprintln!("Compatibility: this hidden issue helper remains callable; use `{replacement}`.");
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
            let atelier_dir = find_atelier_dir().ok();
            if issue_type.is_some() || parent.is_some() {
                commands::agent_factory::create_lifecycle(
                    &state_dir,
                    &db_path,
                    commands::agent_factory::LifecycleCreateInput {
                        title: &title,
                        description: description.as_deref(),
                        priority: &priority,
                        issue_type: issue_type
                            .as_deref()
                            .or(template.as_deref())
                            .unwrap_or("task"),
                        labels: &label,
                        parent: parent.as_deref(),
                        work,
                        quiet,
                        atelier_dir: atelier_dir.as_deref(),
                    },
                )
            } else {
                let (final_priority, final_description, labels, issue_type) = issue_create_parts(
                    &priority,
                    description.as_deref(),
                    template.as_deref(),
                    &label,
                )?;
                commands::agent_factory::create_lifecycle(
                    &state_dir,
                    &db_path,
                    commands::agent_factory::LifecycleCreateInput {
                        title: &title,
                        description: final_description.as_deref(),
                        priority: &final_priority,
                        issue_type,
                        labels: &labels,
                        parent: None,
                        work,
                        quiet,
                        atelier_dir: atelier_dir.as_deref(),
                    },
                )
            }
        }

        IssueCommands::Quick {
            title,
            description,
            priority,
            template,
            label,
        } => {
            issue_compat_guidance("atelier issue create <title> --work");
            let (state_dir, db_path) = state_and_db_paths()?;
            let atelier_dir = find_atelier_dir().ok();
            let (final_priority, final_description, labels, issue_type) = issue_create_parts(
                &priority,
                description.as_deref(),
                template.as_deref(),
                &label,
            )?;
            commands::agent_factory::create_lifecycle(
                &state_dir,
                &db_path,
                commands::agent_factory::LifecycleCreateInput {
                    title: &title,
                    description: final_description.as_deref(),
                    priority: &final_priority,
                    issue_type,
                    labels: &labels,
                    parent: None,
                    work: true,
                    quiet,
                    atelier_dir: atelier_dir.as_deref(),
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
            issue_compat_guidance("atelier issue create <title> --parent <id>");
            let (state_dir, db_path) = state_and_db_paths()?;
            let atelier_dir = find_atelier_dir().ok();
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
                    atelier_dir: atelier_dir.as_deref(),
                },
            )
        }

        IssueCommands::List {
            status,
            label,
            priority,
            ready,
            blocked,
        } => {
            let db = projection_query_db()?;
            if blocked {
                if ready {
                    bail!("--blocked cannot be combined with --ready");
                }
                if status != "open" || label.is_some() || priority.is_some() {
                    bail!("--blocked cannot be combined with --status, --label, or --priority");
                }
                commands::deps::list_blocked(&db)
            } else {
                commands::agent_factory::list(
                    &db,
                    Some(&status),
                    label.as_deref(),
                    priority.as_deref(),
                    ready,
                    quiet,
                )
            }
        }

        IssueCommands::Search { query } => {
            issue_compat_guidance("atelier search <query>");
            let db = projection_query_db()?;
            commands::agent_factory::search(&db, &query, quiet)
        }

        IssueCommands::Show { id } => {
            let db = projection_query_db()?;
            commands::agent_factory::show(&db, &id)
        }

        IssueCommands::Transition { id, options } => {
            let db = projection_query_db()?;
            let _ = options;
            commands::agent_factory::transition_options(&db, &id)
        }

        IssueCommands::Update {
            id,
            title,
            description,
            priority,
            status,
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
                    description: description.as_deref(),
                    priority: priority.as_deref(),
                    status: status.as_deref(),
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

        IssueCommands::Close { id, reason } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            let _ = quiet;
            commands::agent_factory::close_lifecycle(&state_dir, &db_path, &id, reason.as_deref())
        }

        IssueCommands::CloseAll { label, priority } => {
            issue_compat_guidance("atelier issue close <id> --reason \"...\"");
            let (state_dir, db_path) = state_and_db_paths()?;
            commands::status::close_all_lifecycle(
                &state_dir,
                &db_path,
                label.as_deref(),
                priority.as_deref(),
            )
        }

        IssueCommands::Reopen { id } => {
            issue_compat_guidance("atelier issue update <id> --status open");
            let (state_dir, db_path) = state_and_db_paths()?;
            commands::agent_factory::reopen_lifecycle(&state_dir, &db_path, &id)
        }

        IssueCommands::Delete { id, force } => {
            issue_compat_guidance("atelier maintenance delete issue <id> --force");
            let (state_dir, db_path) = state_and_db_paths()?;
            let db = canonical_mutation_db()?;
            let id = resolve_issue_arg(&db, &id)?;
            drop(db);
            commands::delete::run_lifecycle(&state_dir, &db_path, &id, force)
        }

        IssueCommands::Comment { id, text, kind } => {
            issue_compat_guidance("atelier note add issue <id> \"...\" --kind <kind>");
            let db = canonical_mutation_db()?;
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            commands::comment::run_canonical(&db, &resolved, &text, &kind)
        }

        IssueCommands::Label { id, label } => {
            issue_compat_guidance("atelier issue update <id> --label <label>");
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            commands::label::add_canonical(&db, &store, &resolved, &label)?;
            drop(db);
            commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        IssueCommands::Unlabel { id, label } => {
            issue_compat_guidance("atelier issue update <id> --remove-label <label>");
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            commands::label::remove_canonical(&db, &store, &resolved, &label)?;
            drop(db);
            commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        IssueCommands::Block { id, blocker } => {
            issue_compat_guidance("atelier dep add <blocked> <blocker>");
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            commands::agent_factory::dep_add_canonical(&db, &store, &id, &blocker)?;
            drop(db);
            commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        IssueCommands::Unblock { id, blocker } => {
            issue_compat_guidance("atelier dep remove <blocked> <blocker>");
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            commands::agent_factory::dep_remove_canonical(&db, &store, &id, &blocker)?;
            drop(db);
            commands::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        IssueCommands::Blocked => {
            issue_compat_guidance("atelier issue list --blocked");
            let db = projection_query_db()?;
            commands::deps::list_blocked(&db)
        }

        IssueCommands::Relate {
            id,
            related,
            relation_type,
        } => {
            issue_compat_guidance("atelier link add issue <id> issue <related> --type <type>");
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
            issue_compat_guidance("atelier link remove issue <id> issue <related> --type <type>");
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
            issue_compat_guidance("atelier link list issue <id>");
            let db = projection_query_db()?;
            let id = resolve_issue_arg(&db, &id)?;
            commands::relate::list(&db, &id)
        }

        IssueCommands::Impact { id } => {
            issue_compat_guidance("atelier graph impact <id>");
            let db = projection_query_db()?;
            let id = resolve_issue_arg(&db, &id)?;
            commands::relate::impact(&db, &id)
        }

        IssueCommands::Next => {
            issue_compat_guidance("atelier status");
            let db = projection_query_db()?;
            let atelier_dir = find_atelier_dir()?;
            commands::next::run(&db, &atelier_dir)
        }

        IssueCommands::Tree { status, compact } => {
            issue_compat_guidance("atelier graph tree");
            let db = projection_query_db()?;
            if compact {
                commands::tree::run_compact(&db, Some(&status))
            } else {
                commands::tree::run(&db, Some(&status))
            }
        }

        IssueCommands::Tested => {
            issue_compat_guidance("atelier evidence add --kind validation --result pass \"...\"");
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
            let storage = command_storage(CommandStorageAccess::ProjectionQuery)?;
            let repo_root = storage.repo_root().to_path_buf();
            commands::prime::run(storage.db(), &storage.state_dir(), &repo_root)
        }

        Commands::Status => {
            let storage = command_storage(CommandStorageAccess::ProjectionQuery)?;
            commands::status::run(storage.db(), &storage.state_dir(), quiet)
        }

        Commands::Start { id } => {
            let db = runtime_db()?;
            let id = resolve_issue_arg(&db, &id)?;
            commands::work::start(&db, &id)
        }

        Commands::Finish { id } => {
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
            commands::work::finish(&db, &id)
        }

        Commands::Issue { action } => dispatch_issue(action, quiet),

        Commands::Search { query } => {
            let db = projection_query_db()?;
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
                let db = projection_query_db()?;
                commands::mission::show(&db, &id)
            }
            MissionCommands::Start { id, switch_active } => {
                let storage = command_storage(CommandStorageAccess::CanonicalMutation)?;
                let db_path = storage.db_path();
                let state_dir = storage.state_dir();
                let id = resolve_record_arg(storage.db(), "mission", &id)?;
                commands::mission::start(&state_dir, &db_path, &id, switch_active)
            }
            MissionCommands::Status { id } => {
                let storage = command_storage(CommandStorageAccess::ProjectionQuery)?;
                commands::mission::status(storage.db(), &storage.state_dir(), id.as_deref(), quiet)
            }
            MissionCommands::Audit { id } => {
                let storage = command_storage(CommandStorageAccess::ProjectionQuery)?;
                commands::mission::audit(storage.db(), &storage.state_dir(), &id, quiet)
            }
            MissionCommands::List { status } => {
                let db = projection_query_db()?;
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

        Commands::Workflow { action } => {
            let db = projection_query_db()?;
            match action {
                WorkflowCommands::Validate {
                    target_kind,
                    target_id,
                    transition,
                    validator,
                } => {
                    let target_id = if target_kind == "tracker" {
                        target_id
                    } else {
                        resolve_record_arg(&db, &target_kind, &target_id)?
                    };
                    commands::workflow::validate(
                        &db,
                        &target_kind,
                        &target_id,
                        &transition,
                        validator,
                    )
                }
            }
        }

        Commands::Work { action } => {
            let db = runtime_db()?;
            match action {
                WorkCommands::Start { id } => {
                    let id = resolve_issue_arg(&db, &id)?;
                    commands::work::start(&db, &id)
                }
                WorkCommands::Finish { id } => {
                    let id = resolve_issue_arg(&db, &id)?;
                    commands::work::finish(&db, &id)
                }
                WorkCommands::Status => commands::work::status(&db),
            }
        }

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
        Commands::Finish { .. } => "finish",
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
            IssueCommands::Reopen { .. } => "issue reopen",
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
            EvidenceCommands::Capture { .. } => "evidence capture",
            EvidenceCommands::Show { .. } => "evidence show",
            EvidenceCommands::Attach { .. } => "evidence attach",
            EvidenceCommands::List { .. } => "evidence list",
        },
        Commands::History { .. } => "history",
        Commands::Workflow { action } => match action {
            WorkflowCommands::Validate { .. } => "workflow validate",
        },
        Commands::Work { action } => match action {
            WorkCommands::Start { .. } => "work start",
            WorkCommands::Finish { .. } => "work finish",
            WorkCommands::Status => "work status",
        },
        Commands::Worktree { action } => match action {
            WorktreeCommands::For { .. } => "worktree for",
            WorktreeCommands::Status => "worktree status",
            WorktreeCommands::Merge { .. } => "worktree merge",
            WorktreeCommands::Remove { .. } => "worktree remove",
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
