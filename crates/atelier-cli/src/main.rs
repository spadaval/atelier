use anyhow::{Context, Result};
use atelier::commands;
use atelier_app::storage_layout::StorageLayout;
use atelier_app::use_cases::{self, load_repository};
use atelier_core::{EvidenceResult, FindingDecision, InvalidValue};
use clap::builder::TypedValueParser;
use clap::{Parser, Subcommand};
use std::env;

mod issue_cli;
mod workflow_help;

/// Help lists global options after every command's own options.
const GLOBAL_OPTION_ORDER: usize = 1000;

#[derive(Parser)]
#[command(name = "atelier")]
#[command(about = "Mission and proof oriented work coordination for agents")]
#[command(after_help = "Common commands:
  atelier status
  atelier work ready
  atelier work mission <id>
  atelier issue show <id>
  atelier start <id>
  atelier evidence record --target issue/<id> --kind test -- <command>
  atelier issue transition <id> close --reason \"...\"
  atelier review approve <id> --identity <name> \"...\"
  atelier land
  atelier merge <id> --check")]
#[command(version = option_env!("ATELIER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")))]
struct Cli {
    /// Quiet mode: only output essential data (IDs, counts)
    #[arg(short, long, global = true, display_order = GLOBAL_OPTION_ORDER)]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Args)]
struct ReviewArgs {
    /// Branch owner, parent issue or standalone issue the event is about
    id: String,
    /// Who records the event; approvals must not match a substantive Git author
    #[arg(long)]
    identity: String,
    /// Full commit the event pins, an ancestor on the work branch [default: HEAD]
    #[arg(long)]
    commit: Option<String>,
    /// What was checked, found or decided
    summary: String,
}
#[derive(Subcommand)]
enum ReviewCommands {
    /// Record a review account: what was checked and its limits
    Record(ReviewArgs),
    /// Record an independent approval of the work at the pinned commit
    Approve(ReviewArgs),
    /// Record a finding; every finding needs a disposition before merge
    Finding {
        #[command(flatten)]
        args: ReviewArgs,
        /// Recommended decision
        #[arg(long, value_parser = named_values(FindingDecision::ALL, FindingDecision::as_str))]
        recommendation: FindingDecision,
    },
    /// Record an explicit trivial exemption, with why correctness is evident
    Trivial(ReviewArgs),
    /// Record the decision on a finding
    Disposition {
        /// The finding's review event ID
        finding: String,
        /// Who decides; defer and no_action need a non-author identity
        #[arg(long)]
        identity: String,
        /// The decision on the finding
        #[arg(long, value_parser = named_values(FindingDecision::ALL, FindingDecision::as_str))]
        decision: FindingDecision,
        /// For fix_now: the full commit that resolves the finding, later than its pin
        #[arg(long)]
        resolution_commit: Option<String>,
        /// For defer: the existing follow-up issue
        #[arg(long)]
        follow_up: Option<String>,
        /// Why the decision was taken
        summary: String,
    },
    /// List review events, for one owner or all
    List {
        /// Owner or issue ID; omit to list every event
        id: Option<String>,
        /// Print JSON
        #[arg(long)]
        json: bool,
    },
    /// Show one review event
    Show {
        /// Review event ID
        id: String,
    },
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize atelier in the current directory
    Init {
        /// Reconcile core tracker state even if already initialized
        #[arg(short, long)]
        force: bool,
        /// Project slug for new record IDs [default: repository directory name]
        #[arg(long)]
        slug: Option<String>,
    },

    /// Install supported harness session hooks
    Setup {
        /// Harness whose hooks to install
        #[arg(value_parser = ["claude"])]
        harness: String,
        /// Explicit executable for local dogfooding; otherwise use atelier on PATH
        #[arg(long)]
        binary: Option<std::path::PathBuf>,
    },

    /// Orient a session: checkout, owner, work, missions, and other branches
    Status {
        /// Consume Claude Code SessionStart JSON and emit hook context
        #[arg(long, value_parser = ["claude"])]
        hook: Option<String>,
    },

    /// Deprecated alias for status
    #[command(hide = true)]
    Prime {
        /// Consume Claude Code SessionStart JSON and emit hook context
        #[arg(long, value_parser = ["claude"])]
        hook: Option<String>,
    },

    /// Operational multi-issue work views
    Work {
        #[command(subcommand)]
        action: Option<WorkCommands>,
    },

    /// Issue lifecycle commands (create, show, transition, note, link, ...)
    Issue {
        #[command(subcommand)]
        action: IssueCommands,
    },

    /// Check out an issue's owner branch, record it, and apply its start transition
    Start {
        /// Issue ID
        id: String,
        /// Create the owner branch from this local branch, from any checkout
        /// including a detached HEAD; it must be the policy's base branch
        #[arg(long)]
        base: Option<String>,
    },

    /// First-class evidence records
    Evidence {
        #[command(subcommand)]
        action: EvidenceCommands,
    },

    /// Append-only in-repository review events
    Review {
        #[command(subcommand)]
        action: ReviewCommands,
    },

    /// Verify committed records and applicable completion invariants
    Land {
        /// Issue or branch owner (otherwise inferred from the checkout)
        id: Option<String>,
        /// Consume Claude Code Stop JSON and emit its decision
        #[arg(long, value_parser = ["claude"])]
        hook: Option<String>,
        /// Confirm that this unowned branch installs Atelier: the base has no
        /// workflow policy yet, so it lands through the repository's usual review
        #[arg(long)]
        install: bool,
        /// After the checks pass, push the current branch to its configured
        /// upstream; a refused land or a failed push exits non-zero
        #[arg(long, conflicts_with = "hook")]
        push: bool,
    },

    /// Check merge gates and fast-forward, or create only a merge commit under merge_commit
    Merge {
        /// Issue, or child of the parent, that owns the checked-out work branch
        id: String,
        /// Check the merge gates only; merge nothing
        #[arg(long)]
        check: bool,
        /// Print the gate results as JSON
        #[arg(long)]
        json: bool,
    },

    /// One-shot graph bundle files
    Bundle {
        #[command(subcommand)]
        action: BundleCommands,
    },

    /// Validate tracker records and their graph
    Check {
        /// Optional issue ID
        id: Option<String>,
    },

    /// Check every done claim and the whole review history
    ///
    /// `land` checks only what changed since the base.
    Doctor,

    /// Review a mission's plan before work starts: outcome, criteria, scope
    /// with sizes, blockers, unknowns, human decisions, risks and approvals
    Brief {
        /// Mission ID
        id: String,
        /// Write a self-contained HTML page under .atelier/runtime/reviews/
        /// instead of printing Markdown
        #[arg(long)]
        html: bool,
    },

    /// Review a finished mission: criteria and proof, each item's report,
    /// decisions, derived deviations and follow-ups
    Debrief {
        /// Mission ID
        id: String,
        /// Write a self-contained HTML page under .atelier/runtime/reviews/
        /// instead of printing Markdown
        #[arg(long)]
        html: bool,
    },

    /// Prune accumulated artifacts safely
    Prune {
        /// Apply eligible cleanup; without this flag the command only reports candidates
        #[arg(long)]
        apply: bool,
        /// Override record retention, in days, for this prune pass
        #[arg(long)]
        retention_days: Option<u64>,
    },

    /// One-shot record migrations
    Migrate {
        #[command(subcommand)]
        action: MigrateCommands,
    },
}

#[derive(Subcommand)]
enum MigrateCommands {
    /// Retype retired epic records as parent issues; dry run unless --apply
    ///
    /// Outcome becomes Goal, Proof and Delivers become Done when, draft
    /// becomes backlog (or the type's initial status), and the policy loses
    /// the epic type.
    Epics {
        /// Issue type for migrated epics
        #[arg(long, default_value = "feature")]
        to: String,
        /// Write the changes; without this flag the command only reports them
        #[arg(long)]
        apply: bool,
    },
}

// ============================================================================
// Issue subcommands (canonical path: `atelier issue <command>`)
// ============================================================================

#[derive(Subcommand)]
enum WorkCommands {
    /// Show one mission dashboard
    #[command(group(clap::ArgGroup::new("filter").multiple(false)))]
    Mission {
        /// Mission ID
        id: String,
        /// Show mission-scoped ready work only
        #[arg(long, group = "filter")]
        ready: bool,
        /// Show mission-scoped blocked work only
        #[arg(long, group = "filter")]
        blocked: bool,
        /// Show mission-scoped active work only
        #[arg(long, group = "filter")]
        active: bool,
        /// Show mission-scoped done work only
        #[arg(long, group = "filter")]
        done: bool,
        /// Show all mission-scoped work
        #[arg(long, group = "filter")]
        all: bool,
    },
    /// Show one parent issue's dashboard: the parent and its children
    Parent {
        /// Parent issue ID
        id: String,
        /// Show every item in the parent's scope, none omitted
        #[arg(long)]
        all: bool,
    },
    /// Show the decisions across a mission's scope: one-way decision notes
    /// first, then decision records, then reversible notes
    Decisions {
        /// Mission ID
        id: String,
    },
    /// Show ready work an agent can pick up; work labelled needs-human is
    /// listed separately
    Ready,
    /// Show ready work only the human can do (labelled needs-human)
    Human,
    /// Show blocked work
    Blocked,
    /// Show active work
    Active,
    /// Show all operational work buckets
    All,
    /// Show the Mission Overview across current missions
    Missions {
        /// Include done missions, and show every mission and unassigned item
        #[arg(long)]
        all: bool,
    },
}

#[derive(Subcommand)]
enum IssueCommands {
    /// Create a new issue
    Create {
        /// Issue title
        title: String,
        /// Markdown body with the type's `## ` sections: Goal and Done when
        /// (optionally Scope, Context, Unknowns, Design, Background) for work
        /// items; Outcome for missions, plus Success criteria without a
        /// strategy, optionally Background; Question (optionally Options,
        /// Decision, Context, Background) for decisions. Plain text becomes
        /// the Goal, Outcome, or Question
        #[arg(long)]
        body: Option<String>,
        /// Priority: P0-P3 or critical, high, medium, low
        #[arg(short, long, default_value = "medium")]
        priority: String,
        /// Add labels to the issue
        #[arg(short, long)]
        label: Vec<String>,
        /// Explicit work type from .atelier/workflow.yaml issue_types, or built-in mission
        #[arg(long)]
        issue_type: Option<String>,
        /// Parent issue ID
        #[arg(long)]
        parent: Option<String>,
        /// Initial status for work not yet started, such as backlog for
        /// someday work; which statuses are allowed depends on the issue
        /// type. Defaults to the workflow's initial status
        #[arg(long)]
        status: Option<String>,
        /// Repository-relative strategy file a mission links; this path,
        /// not the strategy's front matter, is the link
        #[arg(long)]
        strategy: Option<String>,
    },

    /// Show a brief view of one or more issues; --full shows the whole record
    Show {
        /// Issue IDs, each shown in turn
        #[arg(required = true)]
        ids: Vec<String>,
        /// Show the full record: metadata, every section, links, rollups, and
        /// transition commands
        #[arg(long)]
        full: bool,
    },

    /// List issue records as generic inventory
    List {
        /// Filter by exact workflow status, or all
        #[arg(short, long, default_value = "all")]
        status: String,
        /// Filter by derived workflow category
        #[arg(long)]
        category: Option<String>,
        /// Filter by issue type from .atelier/workflow.yaml
        #[arg(long = "issue-type")]
        issue_type: Option<String>,
        /// Filter by label
        #[arg(short, long)]
        label: Option<String>,
        /// Filter by priority: P0-P3 or critical, high, medium, low
        #[arg(short, long)]
        priority: Option<String>,
        /// Maximum number of matching records to show
        #[arg(
            long,
            default_value_t = 50,
            value_parser = atelier_app::issue_inventory::parse_limit
        )]
        limit: usize,
    },

    /// Show or execute issue transitions
    Transition {
        /// Issue ID
        id: String,
        /// Transition name to execute; omit to inspect options and blockers.
        /// To begin work, use `atelier start <id>`, which also creates and
        /// records the owner branch; `start` here only changes the record
        transition: Option<String>,
        /// Close reason used by transitions that require it: the Outcome,
        /// kept above any report parts the record's Close Reason already has
        #[arg(long = "reason", conflicts_with = "reason_file")]
        close_reason: Option<String>,
        /// File holding the whole Close Reason: the Outcome, then report parts
        /// as `### Deviations`, `### Decisions`, `### Surprises` and
        /// `### Open items`. With neither flag, the record's own Close Reason
        /// is used
        #[arg(long)]
        reason_file: Option<std::path::PathBuf>,
        /// Show validator passes, action preflight detail, descriptions, and debug context
        #[arg(long)]
        verbose: bool,
    },

    /// Update an issue
    Update {
        /// Issue ID
        id: String,
        /// New title
        #[arg(short, long)]
        title: Option<String>,
        /// New priority: P0-P3 or critical, high, medium, low
        #[arg(short, long)]
        priority: Option<String>,
        /// New issue type from .atelier/workflow.yaml issue_types
        #[arg(long)]
        issue_type: Option<String>,
        /// Replace the typed body sections, given as `## ` headings for the
        /// record's type and checked like `atelier check`; Notes and Close
        /// Reason are kept, and done records cannot be revised. Change
        /// status with `atelier issue transition`
        #[arg(long)]
        body: Option<String>,
        /// Add labels to the issue
        #[arg(short, long)]
        label: Vec<String>,
        /// Remove labels from the issue
        #[arg(long = "remove-label")]
        remove_label: Vec<String>,
        /// Set the parent issue ID
        #[arg(long, conflicts_with = "no_parent")]
        parent: Option<String>,
        /// Clear parent issue
        #[arg(long)]
        no_parent: bool,
        /// Set the repository-relative strategy file a mission links
        #[arg(long, conflicts_with = "no_strategy")]
        strategy: Option<String>,
        /// Clear a mission's strategy link
        #[arg(long)]
        no_strategy: bool,
        /// Set the repository-relative ADR a decision record produced
        #[arg(long, conflicts_with = "no_adr")]
        adr: Option<String>,
        /// Clear a decision record's ADR link
        #[arg(long)]
        no_adr: bool,
    },

    /// Append a dated note to the issue record's Notes section
    Note {
        /// Issue ID
        id: String,
        /// Note text; for a decision, the choice
        text: String,
        /// Note kind (note, plan, observation, blocker, resolution, result, handoff, human, decision)
        #[arg(long, default_value = "note")]
        kind: String,
        /// Why the decision was made; required with --kind decision
        #[arg(long)]
        reason: Option<String>,
        /// An alternative the decision rejected; repeat for several
        #[arg(long)]
        rejected: Vec<String>,
        /// Mark the decision one-way: it cannot be reversed cheaply
        #[arg(long)]
        one_way: bool,
    },

    /// Add a typed link from one issue to another
    Link {
        /// Source issue ID
        id: String,
        /// Target issue ID
        target: String,
        /// Relationship role. Readiness: blocked_by (the source waits for the
        /// target) and blocks (the target waits for the source). Mission scope:
        /// advances (stored under relates). Context only, no effect on
        /// readiness: related, part_of, implements, supersedes, duplicates,
        /// derived_from, contributes_to, assumption, falsifies, derived, and
        /// custom roles from .atelier/config.toml; these are stored under the
        /// record's relates field. Only the source record is written; the
        /// target reads the link from it
        #[arg(long, default_value = "advances")]
        role: String,
    },

    /// Remove a typed link from one issue to another
    Unlink {
        /// Source issue ID
        id: String,
        /// Target issue ID
        target: String,
        /// Relationship role. Readiness: blocked_by (the source waits for the
        /// target) and blocks (the target waits for the source). Mission scope:
        /// advances (stored under relates). Context only, no effect on
        /// readiness: related, part_of, implements, supersedes, duplicates,
        /// derived_from, contributes_to, assumption, falsifies, derived, and
        /// custom roles from .atelier/config.toml; these are stored under the
        /// record's relates field
        #[arg(long, default_value = "advances")]
        role: String,
    },
}

#[derive(Subcommand)]
enum BundleCommands {
    /// Preview an authored bundle JSON file without mutating tracker state
    Preview {
        /// Path to the bundle JSON file
        input: String,
    },
    /// Apply an authored bundle JSON file
    Apply {
        /// Path to the bundle JSON file
        input: String,
        /// Confirm the apply; without it the command refuses
        #[arg(long)]
        yes: bool,
    },
}

#[derive(Subcommand)]
enum EvidenceCommands {
    /// Record proof manually or by capturing a command transcript
    #[command(after_help = "Examples:
  atelier evidence record --target issue/<id> --kind validation --result pass \"checked claim X\"
  atelier evidence record --target issue/<id> --kind test -- <command>

Use `evidence attach` only when you need to reuse an existing evidence record on
another target.")]
    Record {
        /// Kind of proof, such as test or validation
        #[arg(long = "kind")]
        evidence_kind: String,
        /// Explicit manual result; command capture derives it from the exit status
        #[arg(
            long,
            conflicts_with = "command",
            value_parser = named_values(EvidenceResult::ALL, EvidenceResult::as_str)
        )]
        result: Option<atelier_core::EvidenceResult>,
        /// Accountable target using kind/id syntax, for example issue/atelier-1234
        #[arg(long)]
        target: Option<String>,
        /// Relation to the target
        #[arg(long, default_value = "validates", value_parser = EVIDENCE_ROLES)]
        role: String,
        /// What the evidence shows; the same as the positional summary
        #[arg(long, conflicts_with = "summary_text")]
        summary: Option<String>,
        /// Path of an artifact the evidence refers to
        #[arg(long)]
        path: Option<String>,
        /// URI of an artifact the evidence refers to
        #[arg(long)]
        uri: Option<String>,
        /// Who or what produced the evidence, such as an agent name
        #[arg(long)]
        producer: Option<String>,
        /// A mission success criterion this receipt judges, as <slug>=<result>
        /// for manual proof, or a bare <slug> that takes a captured command's
        /// result; repeat for several. Needs --target on the mission, or on an
        /// item that only that mission holds
        #[arg(long = "criterion", value_name = "CRITERION")]
        criteria: Vec<String>,
        /// Manual evidence summary; command-backed evidence passes its command after `--`
        summary_text: Option<String>,
        /// Command to run and capture; its exit status sets the result
        #[arg(last = true, num_args = 0..)]
        command: Vec<String>,
    },
    /// Show an evidence record
    Show {
        /// Evidence record ID
        id: String,
    },
    /// Reuse an existing evidence record on another target
    Attach {
        /// Evidence record ID
        id: String,
        /// Target record kind
        #[arg(value_parser = ["issue", "evidence"])]
        target_kind: String,
        /// Target record ID
        target_id: String,
        /// Relation to the target
        #[arg(long, default_value = "validates", value_parser = EVIDENCE_ROLES)]
        role: String,
    },
    /// List evidence records, optionally filtering their explicit result
    List {
        /// Show only records with this result
        #[arg(long, value_parser = named_values(EvidenceResult::ALL, EvidenceResult::as_str))]
        result: Option<atelier_core::EvidenceResult>,
    },
}

/// The relation roles evidence may have to its target.
const EVIDENCE_ROLES: [&str; 1] = ["validates"];

/// Lists `values` by name in help and parses a name back to its value, so
/// any other name is a usage error.
fn named_values<T, const N: usize>(
    values: [T; N],
    name: fn(T) -> &'static str,
) -> impl TypedValueParser<Value = T>
where
    T: Copy + Send + Sync + 'static,
{
    clap::builder::PossibleValuesParser::new(values.map(name)).map(move |chosen| {
        values
            .into_iter()
            .find(|value| name(*value) == chosen)
            .expect("the parser accepts only listed names")
    })
}

// ============================================================================
// Main
// ============================================================================

/// The exit status of a usage error, which clap also uses.
const USAGE_ERROR: u8 = 2;

fn main() -> std::process::ExitCode {
    restore_default_sigpipe();
    match load_dotenv().and_then(|()| run()) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) if error.is::<commands::session::Reported>() => std::process::ExitCode::from(1),
        Err(error) => {
            eprintln!("Error: {error:?}");
            // An invalid value is a usage error wherever it is checked.
            let invalid_value = error.chain().any(|cause| cause.is::<InvalidValue>());
            std::process::ExitCode::from(if invalid_value { USAGE_ERROR } else { 1 })
        }
    }
}

/// Rust ignores SIGPIPE, so a write to a reader that closed early, as `head`
/// does, fails and `println!` panics. Restoring the default ends the process
/// quietly instead, with the conventional status 141, as other command-line
/// tools do.
fn restore_default_sigpipe() {
    #[cfg(unix)]
    // SAFETY: called first in `main`, before any thread or I/O exists;
    // SIG_DFL is a valid disposition for SIGPIPE.
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
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
    // Hold an association transaction across every repository command's
    // complete read/validate/write lifetime. Reads share it; every writer owns
    // it exclusively so read-modify-write commands cannot lose concurrent edits.
    let _canonical_transaction_lock = match canonical_transaction_mode(&cli.command) {
        CanonicalTransactionMode::None => None,
        mode => {
            let state_dir = StorageLayout::discover()?.canonical_dir();
            Some(match mode {
                CanonicalTransactionMode::Shared => {
                    atelier_records::mutation_lock::CanonicalMutationLock::shared(&state_dir)?
                }
                CanonicalTransactionMode::Exclusive => {
                    atelier_records::mutation_lock::CanonicalMutationLock::exclusive(&state_dir)?
                }
                CanonicalTransactionMode::None => unreachable!(),
            })
        }
    };
    let quiet = cli.quiet;

    // Status renders policy diagnostics in its own page.
    if !matches!(
        &cli.command,
        Commands::Init { .. } | Commands::Status { .. } | Commands::Prime { .. }
    ) {
        if let Ok(layout) = StorageLayout::discover() {
            if let Ok(source) = atelier_app::base_policy::working(layout.repo_root()) {
                for diagnostic in source.diagnostics {
                    eprintln!("{diagnostic}");
                }
            }
        }
    }
    match cli.command {
        Commands::Init { force, slug } => {
            let cwd = env::current_dir()?;
            commands::init::run(&cwd, force, slug, quiet)
        }

        Commands::Status { hook } => commands::status::run(hook.is_some(), quiet, false),
        Commands::Prime { hook } => commands::status::run(hook.is_some(), quiet, true),
        Commands::Land {
            id,
            hook,
            install,
            push,
        } => commands::session::land(id.as_deref(), hook.is_some(), install, push, quiet),
        Commands::Setup { binary, .. } => commands::setup::run(binary.as_deref(), quiet),
        Commands::Doctor => commands::session::doctor(quiet),
        Commands::Brief { id, html } => {
            let (_, index) = load_repository()?;
            commands::mission_review::brief(&index, &id, html, quiet)
        }
        Commands::Debrief { id, html } => {
            let (_, index) = load_repository()?;
            commands::mission_review::debrief(&index, &id, html, quiet)
        }

        Commands::Work { action: None } => {
            work_index(quiet);
            Ok(())
        }
        Commands::Work {
            action: Some(action),
        } => {
            let (_, index) = load_repository()?;
            match action {
                WorkCommands::Mission {
                    id,
                    ready,
                    blocked,
                    active,
                    done,
                    all,
                } => commands::work::mission_dashboard(
                    &index,
                    &id,
                    commands::work::MissionDashboardOptions {
                        ready,
                        blocked,
                        active,
                        done,
                        all,
                    },
                    quiet,
                ),
                WorkCommands::Parent { id, all } => {
                    commands::work::parent_dashboard(&index, &id, all, quiet)
                }
                WorkCommands::Decisions { id } => commands::work::decisions(&index, &id, quiet),
                WorkCommands::Ready => commands::work::list(&index, "ready", quiet),
                WorkCommands::Blocked => commands::work::list(&index, "blocked", quiet),
                WorkCommands::Human => commands::work::list(&index, "human", quiet),
                WorkCommands::Active => commands::work::list(&index, "active", quiet),
                WorkCommands::All => commands::work::list(&index, "all", quiet),
                WorkCommands::Missions { all } => commands::work::missions(&index, all, quiet),
            }
        }

        Commands::Issue { action } => issue_cli::dispatch(action, quiet),

        Commands::Start { id, base } => {
            let (state_dir, index) = load_repository()?;
            commands::start::run(&index, &state_dir, &id, base.as_deref(), quiet)
        }

        Commands::Merge { id, check, json } => commands::merge::run(&id, check, json, quiet),
        Commands::Review { action } => {
            use atelier_core::ReviewEvent;
            let append = |args: ReviewArgs, event| {
                commands::review::append(
                    &args.id,
                    &args.identity,
                    args.commit.as_deref(),
                    &args.summary,
                    event,
                    quiet,
                )
            };
            match action {
                ReviewCommands::Record(args) => append(args, ReviewEvent::Review),
                ReviewCommands::Approve(args) => append(args, ReviewEvent::Approval),
                ReviewCommands::Trivial(args) => append(args, ReviewEvent::TrivialExemption),
                ReviewCommands::Finding {
                    args,
                    recommendation,
                } => append(args, ReviewEvent::Finding { recommendation }),
                ReviewCommands::Disposition {
                    finding,
                    identity,
                    decision,
                    resolution_commit,
                    follow_up,
                    summary,
                } => commands::review::disposition(
                    &finding,
                    &identity,
                    decision,
                    resolution_commit,
                    follow_up,
                    &summary,
                    quiet,
                ),
                ReviewCommands::List { id, json } => {
                    commands::review::list(id.as_deref(), json, quiet)
                }
                ReviewCommands::Show { id } => commands::review::show(&id, quiet),
            }
        }
        Commands::Bundle { action } => {
            let (state_dir, index) = load_repository()?;
            match action {
                BundleCommands::Preview { input } => {
                    commands::bundle::preview(&index, &input, quiet)
                }
                BundleCommands::Apply { input, yes } => {
                    commands::bundle::apply(&index, &state_dir, &input, yes, quiet)
                }
            }
        }

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
                criteria,
                summary_text,
                command,
            } => {
                let parsed_target = match target.as_deref() {
                    Some(target) => Some(use_cases::parse_evidence_target_arg(target)?),
                    None => None,
                };
                if command.is_empty() {
                    let (state_dir, index) = load_repository()?;
                    let resolved_target = parsed_target
                        .as_ref()
                        .map(|target| {
                            use_cases::resolve_evidence_target_ref(&index, &target.kind, &target.id)
                                .map(|id| (target.kind.clone(), id))
                        })
                        .transpose()?;
                    let summary = summary
                        .as_deref()
                        .or(summary_text.as_deref())
                        .context("evidence record without a command requires a summary")?;
                    let evidence_id = commands::evidence::add_returning_id(
                        &index,
                        &state_dir,
                        &evidence_kind,
                        summary,
                        result.context(
                            "manual evidence requires --result pass|fail|blocked|not-applicable",
                        )?,
                        path.as_deref(),
                        uri.as_deref(),
                        producer.as_deref(),
                        resolved_target.as_ref().map(|(kind, id)| {
                            commands::evidence::TargetMetadata {
                                kind,
                                id,
                                role: &role,
                            }
                        }),
                        &commands::evidence::manual_criteria(&criteria)?,
                    )?;
                    if let Some((kind, id)) = resolved_target {
                        commands::evidence::attach_silently(
                            &index,
                            &state_dir,
                            &evidence_id,
                            &kind,
                            &id,
                            &role,
                        )?;
                    }
                    let evidence = atelier_records::RecordStore::new(&state_dir)
                        .load_evidence_by_id(&evidence_id)?;
                    commands::evidence::print_new_record(&evidence, quiet)
                } else {
                    let command_summary = summary.as_deref().or(summary_text.as_deref());
                    commands::evidence::capture(commands::evidence::CaptureOptions {
                        evidence_kind: &evidence_kind,
                        summary: command_summary,
                        path: path.as_deref(),
                        uri: uri.as_deref(),
                        producer: producer.as_deref(),
                        target_kind: parsed_target.as_ref().map(|target| target.kind.as_str()),
                        target_id: parsed_target.as_ref().map(|target| target.id.as_str()),
                        role: &role,
                        command: &command,
                        criteria: &commands::evidence::captured_criteria(&criteria)?,
                        quiet,
                    })
                }
            }
            EvidenceCommands::Show { id } => {
                let (_, index) = load_repository()?;
                commands::evidence::show(&index, &id, quiet)
            }
            EvidenceCommands::Attach {
                id,
                target_kind,
                target_id,
                role,
            } => {
                let (state_dir, index) = load_repository()?;
                let id = index.resolve(atelier_app::record_ref::RecordKind::Evidence, &id)?;
                let target_id =
                    use_cases::resolve_evidence_target_ref(&index, &target_kind, &target_id)?;
                commands::evidence::attach(
                    &index,
                    &state_dir,
                    &id,
                    &target_kind,
                    &target_id,
                    &role,
                    quiet,
                )
            }
            EvidenceCommands::List { result } => {
                let (_, index) = load_repository()?;
                commands::evidence::list(&index, result, quiet)
            }
        },

        Commands::Migrate {
            action: MigrateCommands::Epics { to, apply },
        } => commands::migrate::epics(&to, apply, quiet),

        Commands::Prune {
            apply,
            retention_days,
        } => {
            let (state_dir, index) = load_repository()?;
            let repo_root = state_dir
                .parent()
                .expect("state directory has a repository root")
                .to_path_buf();
            let config = atelier_app::project_config::ProjectConfig::load(&repo_root)?;
            commands::prune::run(
                commands::prune::TrackerContext {
                    index,
                    repo_root,
                    state_dir,
                    canonical_retention_days: config.prune.canonical_retention_days,
                },
                apply,
                retention_days,
                quiet,
            )
        }

        Commands::Check { id } => {
            let state_dir = StorageLayout::discover()?.canonical_dir();
            commands::issue::lint(&state_dir, id.as_deref(), quiet)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CanonicalTransactionMode {
    None,
    Shared,
    Exclusive,
}

fn canonical_transaction_mode(command: &Commands) -> CanonicalTransactionMode {
    match command {
        // Fresh initialization has no repository association to lock. Forced
        // reconciliation of an existing tracker does.
        Commands::Init { .. } if std::path::Path::new(".atelier").exists() => {
            CanonicalTransactionMode::Exclusive
        }
        Commands::Init { .. } => CanonicalTransactionMode::None,
        // Bare `work` only lists the work views.
        Commands::Work { action: None } => CanonicalTransactionMode::None,
        Commands::Bundle {
            action: BundleCommands::Apply { .. },
        } => CanonicalTransactionMode::Exclusive,
        // Note append is a read-modify-write of one record; it owns an
        // exclusive transaction so concurrent notes cannot lose updates.
        Commands::Issue {
            action: IssueCommands::Note { .. },
        } => CanonicalTransactionMode::Exclusive,
        // Start reads records, switches branches, and rewrites the owner and
        // issue records; no other writer may interleave.
        Commands::Start { .. } => CanonicalTransactionMode::Exclusive,
        Commands::Prune { apply: true, .. } => CanonicalTransactionMode::Exclusive,
        Commands::Migrate {
            action: MigrateCommands::Epics { apply: true, .. },
        } => CanonicalTransactionMode::Exclusive,
        Commands::Review {
            action:
                ReviewCommands::Record(_)
                | ReviewCommands::Approve(_)
                | ReviewCommands::Finding { .. }
                | ReviewCommands::Trivial(_)
                | ReviewCommands::Disposition { .. },
        } => CanonicalTransactionMode::Exclusive,
        // Shell checks may run Atelier read commands; revalidate snapshots
        // rather than hold a lock across arbitrary child execution. Status
        // locks for itself once a hook payload has named the checkout.
        Commands::Merge { .. }
        | Commands::Status { .. }
        | Commands::Prime { .. }
        | Commands::Land { .. }
        | Commands::Doctor
        | Commands::Setup { .. } => CanonicalTransactionMode::None,
        // Command-backed evidence deliberately executes its arbitrary child
        // before acquiring an association transaction for the canonical
        // append. The child may itself be an Atelier writer or migration.
        Commands::Evidence {
            action: EvidenceCommands::Record { command, .. },
        } if !command.is_empty() => CanonicalTransactionMode::None,
        Commands::Issue {
            action:
                IssueCommands::Create { .. }
                | IssueCommands::Update { .. }
                | IssueCommands::Link { .. }
                | IssueCommands::Unlink { .. }
                | IssueCommands::Transition { .. },
        } => CanonicalTransactionMode::Exclusive,
        Commands::Evidence {
            action: EvidenceCommands::Record { .. } | EvidenceCommands::Attach { .. },
        } => CanonicalTransactionMode::Exclusive,
        _ => CanonicalTransactionMode::Shared,
    }
}

#[cfg(test)]
fn command_uses_canonical_transaction(command: &Commands) -> bool {
    canonical_transaction_mode(command) != CanonicalTransactionMode::None
}

/// Bare `atelier work` lists the work views from the command tree, as
/// `atelier work --help` does; `--quiet` prints only their names.
fn work_index(quiet: bool) {
    use clap::CommandFactory;
    if quiet {
        let root = Cli::command();
        let work = root.find_subcommand("work").expect("work subcommand");
        for view in work.get_subcommands().filter(|view| !view.is_hide_set()) {
            if view.get_name() != "help" {
                println!("{}", view.get_name());
            }
        }
        return;
    }
    if let Err(help) = Cli::command().try_get_matches_from(["atelier", "work", "--help"]) {
        let _ = help.print();
    }
}

/// `--quiet` and `--json` are two script formats, so asking for both is a
/// usage error. Clap does not check conflicts against a global flag, so the
/// conflict is raised here as the same parser error.
fn reject_quiet_json(cli: Cli) -> Result<Cli, clap::Error> {
    use clap::CommandFactory;
    let json = matches!(
        cli.command,
        Commands::Review {
            action: ReviewCommands::List { json: true, .. }
        } | Commands::Merge { json: true, .. }
    );
    if cli.quiet && json {
        return Err(Cli::command().error(
            clap::error::ErrorKind::ArgumentConflict,
            "the argument '--quiet' cannot be used with '--json'",
        ));
    }
    Ok(cli)
}

fn parse_cli_or_exit() -> Cli {
    match Cli::try_parse().and_then(reject_quiet_json) {
        Ok(cli) => cli,
        Err(error) => {
            if let Some(invalid) = invalid_argument_value(&error) {
                eprintln!("Error: {invalid}");
                std::process::exit(error.exit_code());
            }
            let error = workflow_help::with_workflow_values(error);
            let exit_code = error.exit_code();
            if let Err(print_error) = error.print() {
                eprintln!("{print_error}");
            }
            std::process::exit(exit_code);
        }
    }
}

/// Words a value the parser rejects in the form every invalid value takes:
/// one outside an option's possible values, or one a value parser refused
/// with an [`InvalidValue`].
fn invalid_argument_value(error: &clap::Error) -> Option<InvalidValue> {
    use clap::error::{ContextKind, ContextValue, ErrorKind};
    match error.kind() {
        ErrorKind::InvalidValue => {
            let string = |kind| match error.get(kind) {
                Some(ContextValue::String(value)) => Some(value.as_str()),
                _ => None,
            };
            let Some(ContextValue::Strings(valid)) = error.get(ContextKind::ValidValue) else {
                return None;
            };
            let invalid = InvalidValue::new(
                &argument_field(string(ContextKind::InvalidArg)?),
                string(ContextKind::InvalidValue)?,
                valid,
            );
            // Keep clap's typo suggestion as the guidance sentence.
            Some(match string(ContextKind::SuggestedValue) {
                Some(suggested) => invalid.with_hint(&format!("Did you mean '{suggested}'?")),
                None => invalid,
            })
        }
        ErrorKind::ValueValidation => std::error::Error::source(error)?
            .downcast_ref::<InvalidValue>()
            .cloned(),
        _ => None,
    }
}

/// The field an argument names, as `issue type` for `--issue-type
/// <ISSUE_TYPE>` and `target kind` for `<TARGET_KIND>`.
fn argument_field(argument: &str) -> String {
    let name = argument.trim_start_matches('-');
    let name = name.split([' ', '=']).next().unwrap_or(name);
    name.trim_matches(['<', '>', '[', ']', '.'])
        .to_lowercase()
        .replace(['-', '_'], " ")
}

#[cfg(test)]
mod help_tests {
    use super::Cli;
    use clap::{Command, CommandFactory, Parser};

    /// Every command in the tree, with its path, after globals propagate.
    /// clap's generated `help` subcommands take no arguments and are skipped.
    fn commands() -> Vec<(String, Command)> {
        fn walk(path: String, command: &Command, out: &mut Vec<(String, Command)>) {
            out.push((path.clone(), command.clone()));
            for sub in command
                .get_subcommands()
                .filter(|sub| sub.get_name() != "help")
            {
                walk(format!("{path} {}", sub.get_name()), sub, out);
            }
        }
        let mut root = Cli::command();
        root.build();
        let mut out = Vec::new();
        walk("atelier".into(), &root, &mut out);
        out
    }

    #[test]
    fn every_argument_has_help() {
        let mut missing = Vec::new();
        for (path, command) in commands() {
            for arg in command.get_arguments() {
                if arg
                    .get_help()
                    .is_none_or(|help| help.to_string().trim().is_empty())
                {
                    missing.push(format!("{path} {}", arg.get_id()));
                }
            }
        }
        assert!(missing.is_empty(), "arguments without help: {missing:?}");
    }

    /// Help uses CONTEXT.md terms (docs/product/human-cli-output.md, Errors):
    /// "lint" and "canonical" appear only joined into a code or key by `_`
    /// or `.`; a hyphen does not join, so "non-canonical" is caught.
    #[test]
    fn no_help_page_uses_internal_words() {
        let mut found = Vec::new();
        for (path, mut command) in commands() {
            let help = command.render_long_help().to_string();
            for word in help
                .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.'))
                .map(|word| word.trim_matches('.').to_lowercase())
            {
                if matches!(word.as_str(), "lint" | "canonical") {
                    found.push(format!("{path}: {word}"));
                }
            }
        }
        assert!(found.is_empty(), "internal words in help: {found:?}");
    }

    #[test]
    fn every_visible_subcommand_is_listed_in_its_parent_help() {
        let mut missing = Vec::new();
        for (path, mut command) in commands() {
            let help = command.render_help().to_string();
            for sub in command.get_subcommands().filter(|sub| !sub.is_hide_set()) {
                let about = sub.get_about().map(ToString::to_string).unwrap_or_default();
                let listed = help.lines().any(|line| {
                    let line = line.trim_start();
                    line.strip_prefix(sub.get_name())
                        .is_some_and(|rest| rest.trim_start() == about)
                });
                if !listed {
                    missing.push(format!("{path} {}", sub.get_name()));
                }
            }
        }
        assert!(
            missing.is_empty(),
            "subcommands missing from help: {missing:?}"
        );
    }

    #[test]
    fn migrate_epics_summary_fits_one_line() {
        let root = Cli::command();
        let epics = root
            .find_subcommand("migrate")
            .and_then(|migrate| migrate.find_subcommand("epics"))
            .expect("migrate epics");
        let about = epics.get_about().expect("about").to_string();
        assert!(about.chars().count() <= 80, "{about}");
        assert!(
            epics.get_long_about().is_some(),
            "detail moved to long help"
        );
    }

    #[test]
    fn fixed_values_are_listed_and_others_are_usage_errors() {
        let cases: [(&[&str], &str); 7] = [
            (
                &[
                    "review",
                    "finding",
                    "x",
                    "s",
                    "--identity",
                    "i",
                    "--recommendation",
                ],
                "fix_now, defer, no_action",
            ),
            (
                &[
                    "review",
                    "disposition",
                    "x",
                    "s",
                    "--identity",
                    "i",
                    "--decision",
                ],
                "fix_now, defer, no_action",
            ),
            (
                &["evidence", "record", "--kind", "test", "s", "--result"],
                "pass, fail, blocked, not-applicable",
            ),
            (
                &["evidence", "list", "--result"],
                "pass, fail, blocked, not-applicable",
            ),
            (
                &["evidence", "record", "--kind", "test", "s", "--role"],
                "validates",
            ),
            (
                &["evidence", "attach", "e", "issue", "t", "--role"],
                "validates",
            ),
            (&["evidence", "attach", "e", "t"], "issue, evidence"),
        ];
        for (args, values) in cases {
            let args = std::iter::once("atelier")
                .chain(args.iter().copied())
                .chain(["bogus"]);
            let error = match Cli::try_parse_from(args) {
                Ok(_) => panic!("an option with values {values} accepted bogus"),
                Err(error) => error,
            };
            assert_eq!(error.kind(), clap::error::ErrorKind::InvalidValue);
            assert_eq!(error.exit_code(), 2);
            assert!(
                error
                    .to_string()
                    .contains(&format!("[possible values: {values}]")),
                "{error}"
            );
        }
    }

    #[test]
    fn global_options_follow_command_options() {
        for (path, mut command) in commands() {
            let help = command.render_help().to_string();
            let quiet = help
                .lines()
                .position(|line| line.trim_start().starts_with("-q, --quiet"))
                .unwrap_or_else(|| panic!("{path} help lists no --quiet:\n{help}"));
            let later_option = help
                .lines()
                .skip(quiet + 1)
                .find(|line| line.trim_start().starts_with('-'));
            assert!(
                later_option.is_none(),
                "{path} lists {later_option:?} after the global --quiet:\n{help}"
            );
        }
    }
}

#[cfg(test)]
mod transaction_tests {
    use super::{
        canonical_transaction_mode, command_uses_canonical_transaction, CanonicalTransactionMode,
        Cli,
    };
    use clap::Parser;

    #[test]
    fn command_backed_evidence_defers_association_transaction_until_after_child() {
        let capture = Cli::try_parse_from([
            "atelier", "evidence", "record", "--kind", "test", "--", "atelier", "status",
        ])
        .expect("command-backed evidence CLI");
        assert!(!command_uses_canonical_transaction(&capture.command));

        let manual = Cli::try_parse_from([
            "atelier",
            "evidence",
            "record",
            "--kind",
            "test",
            "manual proof",
        ])
        .expect("manual evidence CLI");
        assert!(command_uses_canonical_transaction(&manual.command));
    }

    #[test]
    fn doctor_takes_no_transaction_like_land() {
        // A passing land fences its result with an exclusive transaction,
        // which a shared one held around it cannot upgrade to (atelier-37uq).
        for command in [["atelier", "land"], ["atelier", "doctor"]] {
            let cli = Cli::try_parse_from(command).expect("landing CLI");
            assert!(!command_uses_canonical_transaction(&cli.command));
        }
    }

    #[test]
    fn canonical_transaction_lock_precedes_repository_dispatch() {
        let main = include_str!("main.rs");
        let production = main
            .split("mod transaction_tests")
            .next()
            .expect("production dispatch source");
        let lock = production
            .find("let _canonical_transaction_lock")
            .expect("canonical association lock");
        let dispatch = production
            .find("    match cli.command {")
            .expect("central command dispatch");

        assert!(lock < dispatch, "association lock must precede dispatch");
        assert!(
            production[dispatch..].contains("commands::bundle::apply"),
            "missing bulk canonical mutation surface commands::bundle::apply"
        );
        let prune = Cli::try_parse_from(["atelier", "prune", "--apply"]).expect("prune");
        assert_eq!(
            canonical_transaction_mode(&prune.command),
            CanonicalTransactionMode::Exclusive
        );
    }

    #[test]
    fn bulk_snapshot_swap_commands_take_exclusive_canonical_transactions() {
        let apply = Cli::try_parse_from(["atelier", "bundle", "apply", "bundle.json", "--yes"])
            .expect("bundle apply");
        assert_eq!(
            canonical_transaction_mode(&apply.command),
            CanonicalTransactionMode::Exclusive
        );

        let preview = Cli::try_parse_from(["atelier", "bundle", "preview", "bundle.json"])
            .expect("bundle preview");
        assert_eq!(
            canonical_transaction_mode(&preview.command),
            CanonicalTransactionMode::Shared
        );
    }
}
