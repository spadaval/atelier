mod commands;
mod daemon;
mod db;
mod identity;
mod lock_check;
mod locks;
mod models;
mod record_id;
mod sync;
mod token_usage;
mod utils;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;

use db::Database;

#[derive(Parser)]
#[command(name = "atelier")]
#[command(about = "A simple, lean issue tracker CLI")]
#[command(version = option_env!("ATELIER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")))]
struct Cli {
    /// Quiet mode: only output essential data (IDs, counts)
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Output as JSON (supported by list, show, search, session status)
    #[arg(long, global = true)]
    json: bool,

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

    /// Time tracking (start, stop, show)
    #[command(hide = true)]
    Timer {
        #[command(subcommand)]
        action: Option<TimerCommands>,
    },

    /// Export canonical state, or backup issues to JSON/markdown with --format
    Export {
        /// Output file path for backup exports, or state directory for canonical export
        #[arg(short, long)]
        output: Option<String>,
        /// Backup format (json, markdown). Omit for canonical .atelier-state export.
        #[arg(short, long)]
        format: Option<String>,
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

    /// Import issues from JSON file
    #[command(hide = true)]
    Import {
        /// Input file path
        input: String,
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

    /// Validate tracker records
    Lint {
        /// Optional issue ID or imported source ID
        id: Option<String>,
    },

    /// Check tracker runtime and exported-state health
    Doctor,

    /// Archive management
    #[command(hide = true)]
    Archive {
        #[command(subcommand)]
        action: ArchiveCommands,
    },

    /// Milestone management
    #[command(hide = true)]
    Milestone {
        #[command(subcommand)]
        action: MilestoneCommands,
    },

    /// Session management
    #[command(hide = true)]
    Session {
        #[command(subcommand)]
        action: SessionCommands,
    },

    /// Daemon management
    #[command(hide = true)]
    Daemon {
        #[command(subcommand)]
        action: DaemonCommands,
    },

    /// Code clone detection via cpitd
    #[command(hide = true)]
    Cpitd {
        #[command(subcommand)]
        action: CpitdCommands,
    },

    /// Token usage tracking and cost monitoring
    #[command(hide = true)]
    Usage {
        #[command(subcommand)]
        action: UsageCommands,
    },

    /// Agent identity management
    #[command(hide = true)]
    Agent {
        #[command(subcommand)]
        action: AgentCommands,
    },

    /// Lock management for multi-agent coordination
    #[command(hide = true)]
    Locks {
        #[command(subcommand)]
        action: LocksCommands,
    },

    /// Fetch locks and report coordination status
    #[command(hide = true)]
    Sync,

    // ========================================================================
    // Hidden backward-compatible aliases (flat commands)
    // ========================================================================
    /// Create a new issue (shortcut for `issue create`)
    #[command(hide = true)]
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

    /// Quick-create an issue (shortcut for `issue quick`)
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

    /// Create a subissue (shortcut for `issue subissue`)
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

    /// List issues (shortcut for `issue list`)
    #[command(hide = true)]
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

    /// Search issues (shortcut for `issue search`)
    #[command(hide = true)]
    Search {
        /// Search query
        query: String,
    },

    /// Show issue details (shortcut for `issue show`)
    #[command(hide = true)]
    Show {
        /// Issue ID
        id: String,
    },

    /// Update an issue (shortcut for `issue update`)
    #[command(hide = true)]
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

    /// Close an issue (shortcut for `issue close`)
    #[command(hide = true)]
    Close {
        /// Issue ID
        id: String,
        /// Closure reason
        #[arg(short, long)]
        reason: Option<String>,
    },

    /// Close all issues matching filters (shortcut for `issue close-all`)
    #[command(hide = true)]
    CloseAll {
        /// Filter by label
        #[arg(short, long)]
        label: Option<String>,
        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,
    },

    /// Reopen an issue (shortcut for `issue reopen`)
    #[command(hide = true)]
    Reopen {
        /// Issue ID
        id: String,
    },

    /// Delete an issue (shortcut for `issue delete`)
    #[command(hide = true)]
    Delete {
        /// Issue ID
        id: String,
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Add a comment (shortcut for `issue comment`)
    #[command(hide = true)]
    Comment {
        /// Issue ID
        id: String,
        /// Comment text
        text: String,
        /// Comment kind (note, plan, decision, observation, blocker, resolution, result, handoff, human)
        #[arg(long, default_value = "note")]
        kind: String,
    },

    /// Add a label (shortcut for `issue label`)
    #[command(hide = true)]
    Label {
        /// Issue ID
        id: String,
        /// Label name
        label: String,
    },

    /// Remove a label (shortcut for `issue unlabel`)
    #[command(hide = true)]
    Unlabel {
        /// Issue ID
        id: String,
        /// Label name
        label: String,
    },

    /// Block an issue (shortcut for `issue block`)
    #[command(hide = true)]
    Block {
        /// Issue ID that is blocked
        id: String,
        /// Issue ID that is blocking
        blocker: String,
    },

    /// Unblock an issue (shortcut for `issue unblock`)
    #[command(hide = true)]
    Unblock {
        /// Issue ID that was blocked
        id: String,
        /// Issue ID that was blocking
        blocker: String,
    },

    /// List blocked issues (shortcut for `issue blocked`)
    #[command(hide = true)]
    Blocked,

    /// List ready issues (shortcut for `issue ready`)
    #[command(hide = true)]
    Ready,

    /// Link related issues (shortcut for `issue relate`)
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

    /// Remove a relation (shortcut for `issue unrelate`)
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

    /// List related issues (shortcut for `issue related`)
    #[command(hide = true)]
    Related {
        /// Issue ID
        id: String,
    },

    /// Suggest next issue (shortcut for `issue next`)
    #[command(hide = true)]
    Next,

    /// Show issue tree (shortcut for `issue tree`)
    #[command(hide = true)]
    Tree {
        /// Filter by status (open, closed, all)
        #[arg(short, long, default_value = "all")]
        status: String,
    },

    /// Mark tests as run (shortcut for `issue tested`)
    #[command(hide = true)]
    Tested,

    /// Start a timer (shortcut for `timer start`)
    #[command(hide = true, name = "start")]
    TimerStart {
        /// Issue ID
        id: String,
    },

    /// Stop the timer (shortcut for `timer stop`)
    #[command(hide = true, name = "stop")]
    TimerStop,
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
    },

    /// Mark tests as run (resets test reminder)
    Tested,
}

// ============================================================================
// Timer subcommands (canonical path: `atelier timer <command>`)
// ============================================================================

#[derive(Subcommand)]
enum TimerCommands {
    /// Start a timer for an issue
    Start {
        /// Issue ID
        id: String,
    },
    /// Stop the current timer
    Stop,
    /// Show current timer status
    Show,
}

// ============================================================================
// Other subcommand enums
// ============================================================================

#[derive(Subcommand)]
enum UsageCommands {
    /// Record a token usage entry
    Record {
        /// Agent ID
        #[arg(long)]
        agent: String,
        /// Model name (e.g., claude-opus-4-6)
        #[arg(long)]
        model: String,
        /// Input tokens consumed
        #[arg(long)]
        input_tokens: i64,
        /// Output tokens produced
        #[arg(long)]
        output_tokens: i64,
        /// Cache read tokens
        #[arg(long)]
        cache_read: Option<i64>,
        /// Cache creation tokens
        #[arg(long)]
        cache_creation: Option<i64>,
        /// Session ID to associate with
        #[arg(long)]
        session: Option<i64>,
    },
    /// Show details of a single usage record
    Show {
        /// Usage record ID
        id: i64,
    },
    /// List usage records
    List {
        /// Filter by agent ID
        #[arg(long)]
        agent: Option<String>,
        /// Filter by model
        #[arg(long)]
        model: Option<String>,
        /// Max records to show
        #[arg(long, default_value = "20")]
        limit: i64,
    },
    /// Show aggregated usage summary
    Summary {
        /// Filter by agent ID
        #[arg(long)]
        agent: Option<String>,
    },
}

#[derive(Subcommand)]
enum ArchiveCommands {
    /// Archive a closed issue
    Add {
        /// Issue ID
        id: String,
    },
    /// Unarchive an issue (restore to closed)
    Remove {
        /// Issue ID
        id: String,
    },
    /// List archived issues
    List,
    /// Archive all issues closed more than N days ago
    Older {
        /// Days threshold
        days: i64,
    },
}

#[derive(Subcommand)]
enum MilestoneCommands {
    /// Create a new milestone
    Create {
        /// Milestone name
        name: String,
        /// Description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// List milestones
    List {
        /// Filter by status (open, closed, all)
        #[arg(short, long, default_value = "open")]
        status: String,
    },
    /// Show milestone details
    Show {
        /// Milestone ID
        id: i64,
    },
    /// Add issues to a milestone
    Add {
        /// Milestone ID
        id: i64,
        /// Issue IDs to add
        issues: Vec<String>,
    },
    /// Remove an issue from a milestone
    Remove {
        /// Milestone ID
        id: i64,
        /// Issue ID to remove
        issue: String,
    },
    /// Close a milestone
    Close {
        /// Milestone ID
        id: i64,
    },
    /// Delete a milestone
    Delete {
        /// Milestone ID
        id: i64,
    },
}

#[derive(Subcommand)]
enum SessionCommands {
    /// Start a new session
    Start,
    /// End the current session
    End {
        /// Handoff notes for the next session
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// Show current session status
    Status,
    /// Set the issue being worked on
    Work {
        /// Issue ID
        id: String,
    },
    /// Show handoff notes from the previous session
    LastHandoff,
    /// Record last action for context compression breadcrumbs
    Action {
        /// Description of what you just did or are doing
        text: String,
    },
}

#[derive(Subcommand)]
enum CpitdCommands {
    /// Scan for code clones and create issues
    Scan {
        /// Paths to scan (defaults to current directory)
        paths: Vec<String>,
        /// Minimum token sequence length to report
        #[arg(long, default_value = "50")]
        min_tokens: u32,
        /// Glob patterns to exclude (repeatable)
        #[arg(long)]
        ignore: Vec<String>,
        /// Show what would be created without creating issues
        #[arg(long)]
        dry_run: bool,
    },
    /// Show open clone issues
    Status,
    /// Close all open clone issues
    Clear,
}

#[derive(Subcommand)]
enum AgentCommands {
    /// Initialize agent identity for this machine
    Init {
        /// Agent ID (e.g. "worker-1")
        agent_id: String,
        /// Description of this agent
        #[arg(short, long)]
        description: Option<String>,
        /// Overwrite existing configuration
        #[arg(short, long)]
        force: bool,
    },
    /// Show current agent identity and lock status
    Status,
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
enum LocksCommands {
    /// List all active locks
    List,
    /// Check if a specific issue is locked
    Check {
        /// Issue ID
        id: String,
    },
    /// Claim a lock on an issue
    Claim {
        /// Issue ID
        id: String,
        /// Branch name associated with the lock
        #[arg(short, long)]
        branch: Option<String>,
    },
    /// Release a lock on an issue
    Release {
        /// Issue ID
        id: String,
    },
    /// Force-steal a stale lock
    Steal {
        /// Issue ID
        id: String,
    },
}

#[derive(Subcommand)]
enum DaemonCommands {
    /// Start the background daemon
    Start,
    /// Stop the background daemon
    Stop,
    /// Check daemon status
    Status,
    /// Internal: run the daemon loop (used by start)
    #[command(hide = true)]
    Run {
        #[arg(long)]
        dir: PathBuf,
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

fn dispatch_issue(action: IssueCommands, quiet: bool, json: bool) -> Result<()> {
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
            if json || issue_type.is_some() || parent.is_some() {
                return commands::agent_factory::create(
                    &db,
                    commands::agent_factory::CreateInput {
                        title: &title,
                        description: description.as_deref(),
                        priority: &priority,
                        issue_type: issue_type.as_deref().or(template.as_deref()),
                        labels: &label,
                        parent: parent.as_deref(),
                    },
                    json,
                );
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
            )
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
            )
        }

        IssueCommands::List {
            status,
            label,
            priority,
        } => {
            let db = get_db()?;
            if json {
                commands::agent_factory::list(
                    &db,
                    Some(&status),
                    label.as_deref(),
                    priority.as_deref(),
                    true,
                )
            } else {
                commands::agent_factory::list(
                    &db,
                    Some(&status),
                    label.as_deref(),
                    priority.as_deref(),
                    false,
                )
            }
        }

        IssueCommands::Search { query } => {
            let db = get_db()?;
            if json {
                commands::agent_factory::search(&db, &query, true)
            } else {
                commands::agent_factory::search(&db, &query, false)
            }
        }

        IssueCommands::Show { id } => {
            let db = get_db()?;
            commands::agent_factory::show(&db, &id, json)
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
                json,
            )
        }

        IssueCommands::Close { id, reason } => {
            let db = get_db()?;
            let _ = quiet;
            commands::agent_factory::close(&db, &id, reason.as_deref(), json)
        }

        IssueCommands::CloseAll { label, priority } => {
            let db = get_db()?;
            commands::status::close_all(&db, label.as_deref(), priority.as_deref())
        }

        IssueCommands::Reopen { id } => {
            let db = get_db()?;
            commands::agent_factory::reopen(&db, &id, json)
        }

        IssueCommands::Delete { id, force } => {
            let db = get_db()?;
            commands::delete::run(&db, &id, force)
        }

        IssueCommands::Comment { id, text, kind } => {
            let db = get_db()?;
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            commands::comment::run(&db, &resolved, &text, &kind)
        }

        IssueCommands::Label { id, label } => {
            let db = get_db()?;
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            commands::label::add(&db, &resolved, &label)
        }

        IssueCommands::Unlabel { id, label } => {
            let db = get_db()?;
            let resolved = commands::agent_factory::resolve_id(&db, &id)?;
            commands::label::remove(&db, &resolved, &label)
        }

        IssueCommands::Block { id, blocker } => {
            let db = get_db()?;
            commands::agent_factory::dep_add(&db, &id, &blocker, json)
        }

        IssueCommands::Unblock { id, blocker } => {
            let db = get_db()?;
            commands::agent_factory::dep_remove(&db, &id, &blocker, json)
        }

        IssueCommands::Blocked => {
            let db = get_db()?;
            if json {
                let items = db
                    .list_blocked_issues()?
                    .into_iter()
                    .map(|issue| commands::agent_factory::issue_object(&db, issue))
                    .collect::<Result<Vec<_>>>()?;
                commands::agent_factory::print_success(
                    "issue.blocked",
                    serde_json::json!({ "items": items, "count": items.len() }),
                )
            } else {
                commands::deps::list_blocked(&db)
            }
        }

        IssueCommands::Ready => {
            let db = get_db()?;
            commands::agent_factory::ready(&db, json)
        }

        IssueCommands::Relate {
            id,
            related,
            relation_type,
        } => {
            let db = get_db()?;
            commands::relate::add_typed(&db, &id, &related, &relation_type)
        }

        IssueCommands::Unrelate {
            id,
            related,
            relation_type,
        } => {
            let db = get_db()?;
            commands::relate::remove_typed(&db, &id, &related, &relation_type)
        }

        IssueCommands::Related { id } => {
            let db = get_db()?;
            commands::relate::list(&db, &id)
        }

        IssueCommands::Impact { id } => {
            let db = get_db()?;
            commands::relate::impact(&db, &id)
        }

        IssueCommands::Next => {
            let db = get_db()?;
            let atelier_dir = find_atelier_dir()?;
            commands::next::run(&db, &atelier_dir)
        }

        IssueCommands::Tree { status } => {
            let db = get_db()?;
            commands::tree::run(&db, Some(&status))
        }

        IssueCommands::Tested => {
            let atelier_dir = find_atelier_dir()?;
            commands::tested::run(&atelier_dir)
        }
    }
}

fn dispatch_timer(action: Option<TimerCommands>) -> Result<()> {
    let db = get_db()?;
    match action {
        Some(TimerCommands::Start { id }) => commands::timer::start(&db, &id),
        Some(TimerCommands::Stop) => commands::timer::stop(&db),
        Some(TimerCommands::Show) | None => commands::timer::status(&db),
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() -> Result<()> {
    // The Commands enum is large (many hidden aliases for backward compat).
    // In debug builds the unoptimized match overflows the default 1 MB stack
    // on Windows, so we run the real logic on a thread with a 4 MB stack.
    let result = std::thread::Builder::new()
        .name("main".into())
        .stack_size(4 * 1024 * 1024)
        .spawn(run)
        .map_err(|e| anyhow::anyhow!("failed to spawn main thread: {e}"))?
        .join();

    match result {
        Ok(r) => r,
        Err(e) => std::panic::resume_unwind(e),
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    init_tracing(&cli.log_level, &cli.log_format);
    let quiet = cli.quiet;
    let json = cli.json;

    let result = match cli.command {
        Commands::Init { force } => {
            let cwd = env::current_dir()?;
            commands::init::run(&cwd, force)
        }

        // ====== Canonical hierarchical commands ======
        Commands::Issue { action } => dispatch_issue(action, quiet, json),
        Commands::Timer { action } => dispatch_timer(action),

        // ====== Hidden backward-compatible aliases ======
        Commands::Create {
            title,
            description,
            priority,
            template,
            label,
            issue_type,
            parent,
            work,
        } => dispatch_issue(
            IssueCommands::Create {
                title,
                description,
                priority,
                template,
                label,
                issue_type,
                parent,
                work,
            },
            quiet,
            json,
        ),

        Commands::Quick {
            title,
            description,
            priority,
            template,
            label,
        } => dispatch_issue(
            IssueCommands::Quick {
                title,
                description,
                priority,
                template,
                label,
            },
            quiet,
            json,
        ),

        Commands::Subissue {
            parent,
            title,
            description,
            priority,
            label,
            work,
        } => dispatch_issue(
            IssueCommands::Subissue {
                parent,
                title,
                description,
                priority,
                label,
                work,
            },
            quiet,
            json,
        ),

        Commands::List {
            status,
            label,
            priority,
        } => dispatch_issue(
            IssueCommands::List {
                status,
                label,
                priority,
            },
            quiet,
            json,
        ),

        Commands::Search { query } => dispatch_issue(IssueCommands::Search { query }, quiet, json),

        Commands::Show { id } => dispatch_issue(IssueCommands::Show { id }, quiet, json),

        Commands::Update {
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
        } => dispatch_issue(
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
            },
            quiet,
            json,
        ),

        Commands::Close { id, reason } => {
            dispatch_issue(IssueCommands::Close { id, reason }, quiet, json)
        }

        Commands::CloseAll { label, priority } => {
            dispatch_issue(IssueCommands::CloseAll { label, priority }, quiet, json)
        }

        Commands::Reopen { id } => dispatch_issue(IssueCommands::Reopen { id }, quiet, json),

        Commands::Delete { id, force } => {
            dispatch_issue(IssueCommands::Delete { id, force }, quiet, json)
        }

        Commands::Comment { id, text, kind } => {
            dispatch_issue(IssueCommands::Comment { id, text, kind }, quiet, json)
        }

        Commands::Label { id, label } => {
            dispatch_issue(IssueCommands::Label { id, label }, quiet, json)
        }

        Commands::Unlabel { id, label } => {
            dispatch_issue(IssueCommands::Unlabel { id, label }, quiet, json)
        }

        Commands::Block { id, blocker } => {
            dispatch_issue(IssueCommands::Block { id, blocker }, quiet, json)
        }

        Commands::Unblock { id, blocker } => {
            dispatch_issue(IssueCommands::Unblock { id, blocker }, quiet, json)
        }

        Commands::Blocked => dispatch_issue(IssueCommands::Blocked, quiet, json),

        Commands::Ready => dispatch_issue(IssueCommands::Ready, quiet, json),

        Commands::Relate {
            id,
            related,
            relation_type,
        } => dispatch_issue(
            IssueCommands::Relate {
                id,
                related,
                relation_type,
            },
            quiet,
            json,
        ),

        Commands::Unrelate {
            id,
            related,
            relation_type,
        } => dispatch_issue(
            IssueCommands::Unrelate {
                id,
                related,
                relation_type,
            },
            quiet,
            json,
        ),

        Commands::Related { id } => dispatch_issue(IssueCommands::Related { id }, quiet, json),

        Commands::Next => dispatch_issue(IssueCommands::Next, quiet, json),

        Commands::Tree { status } => dispatch_issue(IssueCommands::Tree { status }, quiet, json),

        Commands::Tested => dispatch_issue(IssueCommands::Tested, quiet, json),

        Commands::TimerStart { id } => dispatch_timer(Some(TimerCommands::Start { id })),

        Commands::TimerStop => dispatch_timer(Some(TimerCommands::Stop)),

        // ====== Non-issue, non-timer commands ======
        Commands::Export {
            output,
            format,
            check,
        } => {
            let db = get_db()?;
            match format.as_deref() {
                Some("json") => {
                    if check {
                        bail!("--check is only supported for canonical export");
                    }
                    commands::export::run_json(&db, output.as_deref())
                }
                Some("markdown") | Some("md") => {
                    if check {
                        bail!("--check is only supported for canonical export");
                    }
                    commands::export::run_markdown(&db, output.as_deref())
                }
                Some(format) => {
                    bail!(
                        "Unknown format '{}'. Use 'json', 'markdown', or omit --format for canonical export",
                        format
                    );
                }
                None => {
                    let atelier_dir = find_atelier_dir()?;
                    let repo_root = atelier_dir
                        .parent()
                        .ok_or_else(|| anyhow::anyhow!("Cannot determine repository root"))?;
                    let state_dir = output
                        .as_deref()
                        .map(std::path::PathBuf::from)
                        .unwrap_or_else(|| repo_root.join(".atelier-state"));
                    commands::agent_factory::export_canonical(&db, &state_dir, check, json)
                }
            }
        }

        Commands::Rebuild { input } => {
            let repo_root = find_repo_root_for_rebuild()?;
            let state_dir = input
                .as_deref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| repo_root.join(".atelier-state"));
            let db_path = repo_root.join(".atelier").join("state.db");
            commands::agent_factory::rebuild(&state_dir, &db_path, json)
        }

        Commands::Import { input } => {
            let db = get_db()?;
            let path = std::path::Path::new(&input);
            commands::import::run_json(&db, path)
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
            commands::import::run_beads_jsonl(&db, std::path::Path::new(&input), &state_dir, json)
        }

        Commands::Dep { action } => {
            let db = get_db()?;
            match action {
                DepCommands::Add { blocked, blocker } => {
                    commands::agent_factory::dep_add(&db, &blocked, &blocker, json)
                }
                DepCommands::Remove { blocked, blocker } => {
                    commands::agent_factory::dep_remove(&db, &blocked, &blocker, json)
                }
                DepCommands::List { issue } => {
                    commands::agent_factory::dep_list(&db, issue.as_deref(), json)
                }
            }
        }

        Commands::Lint { id } => {
            let db = get_db()?;
            commands::agent_factory::lint(&db, id.as_deref(), json)
        }

        Commands::Doctor => {
            let db = get_db()?;
            let atelier_dir = find_atelier_dir()?;
            let repo_root = atelier_dir
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Cannot determine repository root"))?;
            let state_dir = repo_root.join(".atelier-state");
            commands::agent_factory::doctor(&db, repo_root, &state_dir, json)
        }

        Commands::Archive { action } => {
            let db = get_db()?;
            match action {
                ArchiveCommands::Add { id } => commands::archive::archive(&db, &id),
                ArchiveCommands::Remove { id } => commands::archive::unarchive(&db, &id),
                ArchiveCommands::List => commands::archive::list(&db),
                ArchiveCommands::Older { days } => commands::archive::archive_older(&db, days),
            }
        }

        Commands::Milestone { action } => {
            let db = get_db()?;
            match action {
                MilestoneCommands::Create { name, description } => {
                    commands::milestone::create(&db, &name, description.as_deref())
                }
                MilestoneCommands::List { status } => commands::milestone::list(&db, Some(&status)),
                MilestoneCommands::Show { id } => commands::milestone::show(&db, id),
                MilestoneCommands::Add { id, issues } => commands::milestone::add(&db, id, &issues),
                MilestoneCommands::Remove { id, issue } => {
                    commands::milestone::remove(&db, id, &issue)
                }
                MilestoneCommands::Close { id } => commands::milestone::close(&db, id),
                MilestoneCommands::Delete { id } => commands::milestone::delete(&db, id),
            }
        }

        Commands::Session { action } => {
            let db = get_db()?;
            match action {
                SessionCommands::Start => {
                    let atelier_dir = find_atelier_dir()?;
                    commands::session::start(&db, &atelier_dir)
                }
                SessionCommands::End { notes } => commands::session::end(&db, notes.as_deref()),
                SessionCommands::Status => {
                    if json {
                        commands::session::status_json(&db)
                    } else {
                        commands::session::status(&db)
                    }
                }
                SessionCommands::Work { id } => {
                    let atelier_dir = find_atelier_dir()?;
                    commands::session::work(&db, &id, &atelier_dir)
                }
                SessionCommands::LastHandoff => commands::session::last_handoff(&db),
                SessionCommands::Action { text } => commands::session::action(&db, &text),
            }
        }

        Commands::Daemon { action } => match action {
            DaemonCommands::Start => {
                let atelier_dir = find_atelier_dir()?;
                daemon::start(&atelier_dir)
            }
            DaemonCommands::Stop => {
                let atelier_dir = find_atelier_dir()?;
                daemon::stop(&atelier_dir)
            }
            DaemonCommands::Status => {
                let atelier_dir = find_atelier_dir()?;
                daemon::status(&atelier_dir)
            }
            DaemonCommands::Run { dir } => daemon::run_daemon(&dir),
        },

        Commands::Cpitd { action } => {
            let db = get_db()?;
            match action {
                CpitdCommands::Scan {
                    paths,
                    min_tokens,
                    ignore,
                    dry_run,
                } => commands::cpitd::scan(&db, &paths, min_tokens, &ignore, dry_run, quiet),
                CpitdCommands::Status => commands::cpitd::status(&db),
                CpitdCommands::Clear => commands::cpitd::clear(&db),
            }
        }

        Commands::Usage { action } => {
            let db = get_db()?;
            match action {
                UsageCommands::Record {
                    agent,
                    model,
                    input_tokens,
                    output_tokens,
                    cache_read,
                    cache_creation,
                    session,
                } => {
                    let raw = token_usage::RawTokenUsage {
                        input_tokens,
                        output_tokens,
                        cache_read_input_tokens: cache_read,
                        cache_creation_input_tokens: cache_creation,
                    };
                    let parsed = token_usage::parse_api_usage(&raw, &model, &agent, session);
                    commands::usage::record(&db, &parsed)
                }
                UsageCommands::Show { id } => commands::usage::show(&db, id, json),
                UsageCommands::List {
                    agent,
                    model,
                    limit,
                } => commands::usage::list(
                    &db,
                    agent.as_deref(),
                    model.as_deref(),
                    Some(limit),
                    json,
                ),
                UsageCommands::Summary { agent } => {
                    commands::usage::summary(&db, agent.as_deref(), json)
                }
            }
        }

        Commands::Agent { action } => {
            let atelier_dir = find_atelier_dir()?;
            match action {
                AgentCommands::Init {
                    agent_id,
                    description,
                    force,
                } => commands::agent::init(&atelier_dir, &agent_id, description.as_deref(), force),
                AgentCommands::Status => commands::agent::status(&atelier_dir),
            }
        }

        Commands::Locks { action } => {
            let atelier_dir = find_atelier_dir()?;
            match action {
                LocksCommands::List => {
                    let db = get_db()?;
                    commands::locks_cmd::list(&atelier_dir, &db, json)
                }
                LocksCommands::Check { id } => commands::locks_cmd::check(&atelier_dir, &id),
                LocksCommands::Claim { id, branch } => {
                    commands::locks_cmd::claim(&atelier_dir, &id, branch.as_deref())
                }
                LocksCommands::Release { id } => commands::locks_cmd::release(&atelier_dir, &id),
                LocksCommands::Steal { id } => commands::locks_cmd::steal(&atelier_dir, &id),
            }
        }

        Commands::Sync => {
            let atelier_dir = find_atelier_dir()?;
            commands::locks_cmd::sync_cmd(&atelier_dir)
        }
    };

    if json {
        if let Err(error) = result {
            let code = commands::agent_factory::classify_error(&error);
            commands::agent_factory::print_error(
                "atelier",
                code,
                &error.to_string(),
                serde_json::json!({}),
            )?;
            std::process::exit(1);
        }
        Ok(())
    } else {
        result
    }
}
