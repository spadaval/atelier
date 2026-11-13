//! The `atelier` command (docs/product.md, Commands).

mod check;
mod commands;
mod forge;
mod git;
mod history;
mod merge;
mod output;
mod plan;
mod review;
mod setup;
mod view_output;
mod views;

use atelier_app::{identity, Repo, Skip, SKIP_VARIABLE as SKIP};
use atelier_core::{Priority, Verdict};
use clap::builder::NonEmptyStringValueParser;
use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use commands::Outcome;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

/// `ATELIER_SKIP`'s value, if it is set; blank when it is set to nothing
/// but whitespace, which every command refuses.
fn skip_variable() -> Option<String> {
    let value = std::env::var_os(SKIP)?;
    Some(value.to_str().unwrap_or("?").to_string())
}

/// The repository of the checkout this command runs in, opened for the
/// calling thread with this invocation's skip.
pub(crate) fn open<'s>(dir: &Path, skip: &'s Skip) -> anyhow::Result<Repo<'s>> {
    Repo::open(dir, identity(|name| std::env::var(name).ok()), skip)
}

#[derive(Parser)]
#[command(
    name = "atelier",
    version,
    about = "Tracks work in this repository. Start with `atelier status`."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Who this thread is, what it holds, what each thing needs, and the next command.
    Status,
    /// Validate every record in this checkout, and list record changes not committed yet.
    Check {
        /// Run as this Git hook, with the arguments Git gave the hook.
        #[arg(long, value_name = "HOOK")]
        hook: Option<check::Hook>,
        #[arg(
            requires = "hook",
            trailing_var_arg = true,
            allow_hyphen_values = true,
            hide = true
        )]
        args: Vec<String>,
    },
    /// Install Atelier's Git hooks in this clone, keeping any hook it has as <name>.local; or with a harness, its session hooks in this repository.
    Setup { harness: Option<setup::Harness> },
    /// One item in full with its reviews, or one mission on one page with its items and disputed findings.
    Show { id: String },
    /// The history of acts, read from Git, one line each, oldest first: of everything, of one item, or of one mission with its plan reviews and its items.
    ///
    /// Each line is the commit's time in UTC, the commit, the item or mission, the act, the thread that acted or `-` where no record holds one, then what changed, and last each rule its commit skipped as `skip <rule>: <reason>`. The first five hold no space; what changed and the skips are parts separated by `; `.
    ///
    /// The acts are start, take-over, submit, review, verdict, merge, settle, lead, ready, drop and close. A status move reads `<from> -> <to>`, and `new` is a record the commit added. `log` sees commits, not commands: two acts on one record in one commit show as their net change. It adds nothing up; narrow its lines with grep.
    Log {
        /// An item or a mission.
        id: Option<String>,
        /// Only acts committed at or after this RFC 3339 time, or this date's start in UTC (YYYY-MM-DD).
        #[arg(long, value_name = "TIME", value_parser = since)]
        since: Option<chrono::DateTime<chrono::Utc>>,
    },
    /// All open work, grouped by mission and then by label; each flag given narrows it.
    List {
        /// Only items a thread could start now: ready, their blockers done, their mission ready.
        #[arg(long)]
        ready: bool,
        /// Only items with a blocker that is not done.
        #[arg(long)]
        blocked: bool,
        /// Only items labelled needs-human.
        #[arg(long)]
        needs_human: bool,
        /// Only this mission's items.
        #[arg(long, value_name = "ID")]
        mission: Option<String>,
        /// Only items, and missions, with this label.
        #[arg(long, value_name = "LABEL")]
        label: Option<String>,
        /// Only what you hold as implementer, reviewer or lead.
        #[arg(long)]
        mine: bool,
        /// Finished missions and items, done or dropped, instead of open ones.
        #[arg(long)]
        done: bool,
        /// Instead, the deferred findings of done items that nobody has taken into an item or dropped.
        #[arg(long, conflicts_with_all = ["ready", "blocked", "needs_human", "mission", "label", "mine", "done"])]
        deferred: bool,
        /// Instead, each rebutted finding of an open or done item with what the next round made of it, and each plan-review round whose reviewers' verdicts differ; with --mission, only that mission's.
        #[arg(long, conflicts_with_all = ["ready", "blocked", "needs_human", "label", "mine", "done", "deferred"])]
        disputed: bool,
    },
    /// Create a draft item, or with --mission a draft mission, in this checkout of the base, without committing it.
    New {
        #[arg(value_parser = NonEmptyStringValueParser::new())]
        title: String,
        /// Make the item part of this mission.
        #[arg(long = "in", value_name = "MISSION", conflicts_with = "mission")]
        into: Option<String>,
        /// Create a mission instead of an item.
        #[arg(long)]
        mission: bool,
        /// Make the item from a deferred finding of a done item, named as `atelier list --deferred` prints it, and mark the finding taken; also on the branch of an item you implement.
        #[arg(long, value_name = "FINDING", conflicts_with = "mission")]
        from: Option<String>,
    },
    /// Change front-matter fields of a draft or ready item or mission, without committing them.
    #[command(group(ArgGroup::new("fields").required(true).multiple(true)))]
    Set {
        id: String,
        #[arg(long, group = "fields", value_parser = NonEmptyStringValueParser::new())]
        title: Option<String>,
        /// The item's priority.
        #[arg(long, group = "fields")]
        priority: Option<PriorityArg>,
        /// Comma-separated; replaces the labels, and "" clears them.
        #[arg(long, group = "fields", value_name = "LABELS")]
        labels: Option<String>,
        /// The item's mission; "" takes it out of its mission.
        #[arg(long, group = "fields", value_name = "MISSION")]
        mission: Option<String>,
        /// Comma-separated item IDs; replaces the item's blockers, and "" clears them.
        #[arg(long, group = "fields", value_name = "IDS")]
        blocked_by: Option<String>,
        /// On a bug, the item ID or commit hash that introduced it; "" clears it.
        #[arg(long, group = "fields", value_name = "CAUSE")]
        caused_by: Option<String>,
    },
    /// Mark each named draft item or mission ready, recording who said so, and commit that alone.
    Ready {
        #[arg(required = true)]
        ids: Vec<String>,
        /// Who said it should start now.
        #[arg(long, required = true, value_parser = NonEmptyStringValueParser::new())]
        by: String,
    },
    /// End a draft, ready or started item, or a draft or ready mission, that will not be done, on the base, and commit that alone. Given a deferred finding of a done item, named as `atelier list --deferred` prints it, mark it dropped without committing it; also on the branch of an item you implement.
    Drop {
        /// An item, a mission, or a finding.
        id: String,
        /// Why it will not be done, on one line.
        #[arg(long)]
        reason: Option<String>,
    },
    /// End a ready mission whose items are done or dropped and whose Result is written, or a draft or ready item that will not be built, with its result; on the base, and commit that alone.
    Close {
        id: String,
        /// The item's result, written into its Report. A mission's is written in its file.
        result: Option<String>,
    },
    /// Make a branch and claim it: start a ready item on a branch named for it, or take over a started one; or make a new mission's branch, or a backlog branch. The only command that makes a branch. Where a remote is configured the push of the branch is the claim. In the base's own checkout it does not switch, and prints the `git worktree add` to run.
    #[command(group(ArgGroup::new("what").required(true).args(["id", "new", "mission", "backlog"])))]
    Start {
        id: Option<String>,
        /// Create an item already started, for work asked for directly, with this title.
        #[arg(long, value_name = "TITLE", value_parser = NonEmptyStringValueParser::new())]
        new: Option<String>,
        /// Create a draft mission with this title on a new branch named for it, where its plan is written.
        #[arg(long, value_name = "TITLE", value_parser = NonEmptyStringValueParser::new())]
        mission: Option<String>,
        /// Make the branch backlog/<SLUG> at the base's tip, for record changes tied to no item or mission.
        #[arg(long, value_name = "SLUG")]
        backlog: Option<String>,
    },
    /// Send this branch's item to review, once its Done when lines are ticked and its Report is written.
    Submit,
    /// Open this round's review file of an item in review, or of a mission's plan, or take it over, and print the brief.
    Review { id: String },
    /// Record your verdict in your review file and commit it.
    Verdict {
        verdict: VerdictArg,
        /// The model you run on, such as gpt-6.1-sol; written into the review file.
        #[arg(long)]
        model: Option<String>,
    },
    /// Lead a ready mission, in this checkout of the base, commit that alone, and see what each of its items needs.
    Lead { mission: String },
    /// Merge this branch's approved item into the base, once its checks pass, in one commit that marks it done.
    Merge {
        /// Merge an item nobody reviewed, whose correctness is evident from its diff, giving why.
        #[arg(long, value_name = "WHY")]
        trivial: Option<String>,
    },
}

fn since(text: &str) -> Result<chrono::DateTime<chrono::Utc>, String> {
    atelier_app::log::since(text)
}

#[derive(Clone, Copy, ValueEnum)]
enum PriorityArg {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Clone, Copy, ValueEnum)]
enum VerdictArg {
    /// No fix-now finding is open.
    Approve,
    /// Your review file holds an open fix-now finding.
    Changes,
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        // A usage error, or the help asked for.
        Err(error) => {
            let _ = error.print();
            let skip = Skip::new(skip_variable().as_deref());
            if error.use_stderr() && skip.reason().is_some() {
                eprint!("{}", output::not_skippable());
            }
            return ExitCode::from(error.exit_code() as u8);
        }
    };
    // A skip needs its reason. A checkout cannot be refused, so the
    // post-checkout hook only says so.
    let variable = skip_variable();
    if variable
        .as_deref()
        .is_some_and(|reason| reason.trim().is_empty())
    {
        let hook = match &cli.command {
            Command::Check { hook, .. } => *hook,
            _ => None,
        };
        if !matches!(hook, Some(check::Hook::PostCheckout)) {
            let _ = std::io::stderr().write_all(output::blank_skip(true).as_bytes());
            return ExitCode::from(1);
        }
        print!("{}", output::blank_skip(false));
    }
    let skip = Skip::new(variable.as_deref());
    // `check` reads the records itself, and reports a config that does not
    // read rather than stopping at it.
    if let Command::Check { hook, args } = &cli.command {
        let checked = match hook {
            None => check::check(&skip),
            Some(hook) => check::hook(*hook, args, &skip),
        };
        return exit(checked, &skip);
    }
    let here = std::env::current_dir();
    let result = here
        .map_err(anyhow::Error::from)
        .and_then(|here| open(&here, &skip))
        .and_then(|mut repo| match cli.command {
            Command::Status => commands::status(&mut repo).map(Outcome::Done),
            Command::Check { .. } => unreachable!("check runs before the repository is opened"),
            Command::Setup { harness: None } => setup::git_hooks(&repo.root),
            Command::Setup {
                harness: Some(harness),
            } => setup::harness(&repo.root, harness),
            Command::Show { id } => views::show(&mut repo, &id),
            Command::Log { id, since } => history::log(&mut repo, id.as_deref(), since),
            Command::List { deferred: true, .. } => views::deferred(&mut repo),
            Command::List {
                disputed: true,
                mission,
                ..
            } => views::disputed(&mut repo, mission.as_deref()),
            Command::List {
                ready,
                blocked,
                needs_human,
                mission,
                label,
                mine,
                done,
                deferred: false,
                disputed: false,
            } => {
                let filter = atelier_app::Filter {
                    ready,
                    blocked,
                    needs_human,
                    mission,
                    label,
                    mine,
                    done,
                };
                views::list(&mut repo, &filter)
            }
            Command::New {
                title,
                into,
                mission,
                from,
            } => plan::new(&mut repo, &title, into, mission, from.as_deref()),
            Command::Set {
                id,
                title,
                priority,
                labels,
                mission,
                blocked_by,
                caused_by,
            } => {
                let priority = priority.map(|priority| match priority {
                    PriorityArg::Critical => Priority::Critical,
                    PriorityArg::High => Priority::High,
                    PriorityArg::Medium => Priority::Medium,
                    PriorityArg::Low => Priority::Low,
                });
                let fields = plan::Fields {
                    title,
                    priority,
                    labels,
                    mission,
                    blocked_by,
                    caused_by,
                };
                plan::set(&mut repo, &id, fields)
            }
            Command::Ready { ids, by } => {
                atelier_app::ready(&mut repo, &ids, &by).map(Outcome::from)
            }
            Command::Drop { id, reason } => plan::drop(&mut repo, &id, reason.as_deref()),
            Command::Close { id, result } => plan::close(&mut repo, &id, result.as_deref()),
            Command::Start { id: Some(id), .. } => commands::start(&mut repo, &id),
            Command::Start {
                new: Some(title), ..
            } => commands::start_new(&mut repo, &title),
            Command::Start {
                mission: Some(title),
                ..
            } => commands::start_mission(&mut repo, &title),
            Command::Start {
                backlog: Some(slug),
                ..
            } => commands::start_backlog(&mut repo, &slug),
            Command::Start { .. } => {
                unreachable!("clap requires an ID, --new, --mission or --backlog")
            }
            Command::Submit => commands::submit(&mut repo),
            Command::Review { id } => review::review(&mut repo, &id),
            Command::Verdict { verdict, model } => {
                let verdict = match verdict {
                    VerdictArg::Approve => Verdict::Approve,
                    VerdictArg::Changes => Verdict::Changes,
                };
                review::verdict(&mut repo, verdict, model)
            }
            Command::Lead { mission } => views::lead(&mut repo, &mission),
            Command::Merge { trivial } => {
                merge::merge(&repo.root, repo.caller.clone(), trivial, &skip)
            }
        });
    exit(result, &skip)
}

/// Prints the outcome, after each rule `skip` passed. Every refusal of a
/// rule was passed or offered the skip, so a refusal or an error while a
/// reason is given is of something that cannot be skipped.
fn exit(result: anyhow::Result<Outcome>, skip: &Skip) -> ExitCode {
    let (out, err, code) = match result {
        Ok(Outcome::Done(text)) => (text, String::new(), 0),
        Ok(Outcome::Refused(text)) => (String::new(), text, 1),
        Ok(Outcome::NotYet(text)) => (text, String::new(), 3),
        Err(error) => (String::new(), format!("atelier: {error:#}\n"), 2),
    };
    let mut err = err;
    if code == 1 || code == 2 {
        err += &skip
            .reason()
            .map_or(String::new(), |_| output::not_skippable());
    }
    // A closed stdout, as under `| head`, is no failure of the command.
    let out = output::skipped(&skip.passed()) + &out;
    let _ = std::io::stdout().write_all(out.as_bytes());
    let _ = std::io::stderr().write_all(err.as_bytes());
    ExitCode::from(code)
}
