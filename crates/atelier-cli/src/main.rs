use anyhow::{bail, Context, Result};
use atelier::commands;
use atelier_app::record_index::RecordIndex;
use atelier_app::storage_layout::StorageLayout;
use atelier_app::use_cases::{self, load_repository};
use clap::{Parser, Subcommand};
use std::env;

mod issue_cli;

#[derive(Parser)]
#[command(name = "atelier")]
#[command(about = "Mission and proof oriented work coordination for agents")]
#[command(help_template = "{about-section}\nUsage: {usage}\n\n{after-help}\nOptions:\n{options}")]
#[command(after_help = "Setup:
  init          Initialize Atelier in the current repository
  setup         Install supported harness hooks (claude)

Orientation:
  prime         Load fresh session context
  land          Check committed records and completion invariants
  status        Show checkout, mission, work, and tracker signposts
  work          Show operational views, including the plural Mission Overview

Issues:
  issue         Create, list, show, update, transition, note, and manage links
  start         Check out an issue's owner branch and start it

Planning:
  bundle        Preview and apply one-shot graph bundle files

Records:
  evidence      Capture validation evidence
  review        Record independent reviews, findings, and decisions
  merge         Check gates and fast-forward a reviewed work branch

Maintenance:
  check         Validate every tracker record
  prune         Prune accumulated artifacts safely

Common commands:
  atelier status
  atelier work ready
  atelier work blocked
  atelier work missions
  atelier issue list
  atelier work mission <mission-id>
  atelier work parent <parent-id>
  atelier issue show <id>
  atelier issue link <blocked-id> <blocker-id> --role blocked_by
  atelier issue unlink <blocked-id> <blocker-id> --role blocked_by
  atelier issue create \"...\" --issue-type mission
  atelier issue show <mission-id>
  atelier bundle preview <file>
  atelier bundle apply <file> --yes
  atelier issue transition <issue-id>
  atelier start <issue-id>
  atelier issue transition <issue-id> close --reason \"...\"
  atelier check
  atelier check <issue-id>
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

#[derive(clap::Args)]
struct ReviewArgs {
    id: String,
    #[arg(long)]
    identity: String,
    #[arg(long)]
    commit: Option<String>,
    summary: String,
}
#[derive(Subcommand)]
enum ReviewCommands {
    Record(ReviewArgs),
    Approve(ReviewArgs),
    Finding {
        #[command(flatten)]
        args: ReviewArgs,
        #[arg(long)]
        recommendation: String,
    },
    Trivial(ReviewArgs),
    Disposition {
        finding: String,
        #[arg(long)]
        identity: String,
        #[arg(long)]
        decision: String,
        #[arg(long)]
        resolution_commit: Option<String>,
        #[arg(long)]
        follow_up: Option<String>,
        summary: String,
    },
    List {
        id: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Show {
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

    /// Show checkout, mission, work, and tracker signposts
    Status,

    /// Load fresh checkout and work context without writing session state
    Prime {
        /// Consume Claude Code SessionStart JSON and emit hook context
        #[arg(long, value_parser = ["claude"])]
        hook: Option<String>,
    },
    /// Verify committed records and applicable completion invariants
    Land {
        /// Issue or branch owner (otherwise inferred from the checkout)
        id: Option<String>,
        /// Consume Claude Code Stop JSON and emit its decision
        #[arg(long, value_parser = ["claude"])]
        hook: Option<String>,
    },
    /// Install supported harness session hooks
    Setup {
        #[arg(value_parser = ["claude"])]
        harness: String,
        /// Explicit executable for local dogfooding; otherwise use atelier on PATH
        #[arg(long)]
        binary: Option<std::path::PathBuf>,
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
    },

    /// Append-only in-repository review events
    Review {
        #[command(subcommand)]
        action: ReviewCommands,
    },
    /// Check merge gates and fast-forward without creating commits
    Merge {
        id: String,
        #[arg(long)]
        check: bool,
        #[arg(long)]
        json: bool,
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

    /// Prune accumulated artifacts safely
    Prune {
        /// Apply eligible cleanup; without this flag the command only reports candidates
        #[arg(long)]
        apply: bool,
        /// Override canonical record retention for this prune pass
        #[arg(long)]
        retention_days: Option<u64>,
    },

    /// One-shot record migrations
    Migrate {
        #[command(subcommand)]
        action: MigrateCommands,
    },

    /// Validate tracker records and their graph
    Check {
        /// Optional issue ID
        id: Option<String>,
    },
}

#[derive(Subcommand)]
enum MigrateCommands {
    /// Retype retired epic records as parent issues: Outcome becomes Goal,
    /// Proof and Delivers become Done when, draft becomes backlog (or the
    /// type's initial status), and the policy loses the epic type. Dry run
    /// unless --apply
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
    Mission {
        id: String,
        /// Show mission-scoped ready work only
        #[arg(long)]
        ready: bool,
        /// Show mission-scoped blocked work only
        #[arg(long)]
        blocked: bool,
        /// Show mission-scoped active work only
        #[arg(long)]
        active: bool,
        /// Show mission-scoped done work only
        #[arg(long)]
        done: bool,
        /// Show all mission-scoped work
        #[arg(long)]
        all: bool,
    },
    /// Show one parent issue's dashboard: the parent and its children
    Parent { id: String },
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
        /// Include done missions in the Mission Overview
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
        /// Priority (low, medium, high, critical)
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
        /// Initial todo-category status, such as backlog for someday work;
        /// defaults to the workflow's initial status
        #[arg(long)]
        status: Option<String>,
        /// Repository-relative strategy file a mission links; the mission's
        /// path is the canonical link
        #[arg(long)]
        strategy: Option<String>,
    },

    /// Show issue details
    Show {
        /// Issue ID
        id: String,
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
        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,
        /// Maximum number of matching records to show
        #[arg(long, default_value_t = 50)]
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
        /// Close reason used by transitions that require it
        #[arg(long = "reason")]
        close_reason: Option<String>,
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
        /// New priority
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
        #[arg(long)]
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
    },

    /// Append a dated note to the issue record's Notes section
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
  atelier evidence record --target issue/<id> --kind validation --result pass \"checked claim X\"
  atelier evidence record --target issue/<id> --kind test -- <command>

Use `evidence attach` only when you need to reuse an existing evidence record on
another target.")]
    Record {
        #[arg(long = "kind")]
        evidence_kind: String,
        /// Explicit manual result; command capture derives it from the exit status.
        #[arg(long)]
        result: Option<atelier_core::EvidenceResult>,
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
    /// List evidence records, optionally filtering their explicit result
    List {
        #[arg(long)]
        result: Option<atelier_core::EvidenceResult>,
    },
}

// ============================================================================
// Helpers
// ============================================================================

fn resolve_issue_arg(index: &RecordIndex, issue_ref: &str) -> Result<String> {
    match commands::issue::resolve_id(index, issue_ref) {
        Ok(id) => Ok(id),
        Err(error) => match index.record_kind_for_id(issue_ref) {
            Some(actual_kind) if actual_kind != "issue" => {
                bail!("{}", wrong_kind_message("issue", actual_kind, issue_ref));
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
        "issue" => Some("atelier issue show"),
        "mission" => Some("atelier issue show"),
        "evidence" => Some("atelier evidence show"),
        _ => None,
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

    if !matches!(&cli.command, Commands::Init { .. }) {
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
            commands::init::run(&cwd, force, slug)
        }

        Commands::Prime { hook } => commands::session::run(false, None, hook.is_some(), quiet),
        Commands::Land { id, hook } => {
            commands::session::run(true, id.as_deref(), hook.is_some(), quiet)
        }
        Commands::Setup { binary, .. } => commands::setup::run(binary.as_deref(), quiet),

        Commands::Status => {
            let (_, index) = load_repository()?;
            commands::status::run(&index, quiet)
        }

        Commands::Work { action } => {
            let (_, index) = load_repository()?;
            match action {
                None => commands::work::dashboards(quiet),
                Some(WorkCommands::Mission {
                    id,
                    ready,
                    blocked,
                    active,
                    done,
                    all,
                }) => commands::work::mission_dashboard(
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
                Some(WorkCommands::Parent { id }) => {
                    commands::work::parent_dashboard(&index, &id, quiet)
                }
                Some(WorkCommands::Ready) => commands::work::list(&index, "ready", quiet),
                Some(WorkCommands::Blocked) => commands::work::list(&index, "blocked", quiet),
                Some(WorkCommands::Human) => commands::work::list(&index, "human", quiet),
                Some(WorkCommands::Active) => commands::work::list(&index, "active", quiet),
                Some(WorkCommands::All) => commands::work::list(&index, "all", quiet),
                Some(WorkCommands::Missions { all }) => {
                    commands::work::missions(&index, all, quiet)
                }
            }
        }

        Commands::Issue { action } => issue_cli::dispatch(action, quiet),

        Commands::Start { id } => {
            let (state_dir, index) = load_repository()?;
            commands::start::run(&index, &state_dir, &id)
        }

        Commands::Merge { id, check, json } => commands::merge::run(&id, check, json),
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
                } => append(
                    args,
                    ReviewEvent::Finding {
                        recommendation: commands::review::decision(&recommendation)?,
                    },
                ),
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
                    &decision,
                    resolution_commit,
                    follow_up,
                    &summary,
                    quiet,
                ),
                ReviewCommands::List { id, json } => commands::review::list(id.as_deref(), json),
                ReviewCommands::Show { id } => commands::review::show(&id),
            }
        }
        Commands::Bundle { action } => {
            let (state_dir, index) = load_repository()?;
            match action {
                BundleCommands::Preview { input } => commands::bundle::preview(&index, &input),
                BundleCommands::Apply { input, yes } => {
                    commands::bundle::apply(&index, &state_dir, &input, yes)
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
                    if result.is_some() {
                        bail!("command capture derives its result; omit --result when passing a command");
                    }
                    let command_summary = match (summary.as_deref(), summary_text.as_deref()) {
                        (Some(_), Some(_)) => {
                            bail!("use either --summary or a positional summary, not both")
                        }
                        (Some(summary), None) | (None, Some(summary)) => Some(summary),
                        (None, None) => None,
                    };
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
        } => commands::migrate::epics(&to, apply),

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
            commands::issue::lint(&state_dir, id.as_deref())
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
        // rather than hold a lock across arbitrary child execution.
        Commands::Merge { .. }
        | Commands::Prime { .. }
        | Commands::Land { .. }
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
