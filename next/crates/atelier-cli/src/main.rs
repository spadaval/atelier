//! The `atelier` command (docs/strategies/redesign-product.md, Commands).

mod check;
mod commands;
mod forge;
mod git;
mod merge;
mod output;
mod plan;
mod questions;
mod repo;
mod review;
mod setup;
mod skip;
mod view_output;
mod views;

use atelier_core::{Priority, Verdict};
use clap::builder::NonEmptyStringValueParser;
use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use commands::Outcome;
use std::io::Write;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "atelier",
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
    /// One item in full with its reviews, or one mission on one page with its items and questions.
    Show { id: String },
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
    /// Mark each named draft item or mission ready, recording who said so, without committing it.
    Ready {
        #[arg(required = true)]
        ids: Vec<String>,
        /// Who said it should start now.
        #[arg(long, required = true, value_parser = NonEmptyStringValueParser::new())]
        by: String,
    },
    /// End a draft, ready or started item, or a draft or ready mission, that will not be done, on the base and without committing it. Given a deferred finding of a done item, named as `atelier list --deferred` prints it, mark it dropped; also on the branch of an item you implement.
    Drop {
        /// An item, a mission, or a finding.
        id: String,
        /// Why it will not be done, on one line.
        #[arg(long)]
        reason: Option<String>,
    },
    /// End a ready mission whose items are done or dropped and whose Result is written, or a draft or ready item that will not be built, with its result; on the base and without committing it.
    Close {
        id: String,
        /// The item's result, written into its Report. A mission's is written in its file.
        result: Option<String>,
    },
    /// Start a ready item on a new branch named for it, or take over a started one.
    #[command(group(ArgGroup::new("what").required(true).args(["id", "new"])))]
    Start {
        id: Option<String>,
        /// Create an item already started, for work asked for directly, with this title.
        #[arg(long, value_name = "TITLE", value_parser = NonEmptyStringValueParser::new())]
        new: Option<String>,
    },
    /// Send this branch's item to review, once its Done when lines are ticked and its Report is written.
    Submit,
    /// Ask a question about this branch's item that a decider answers, as when a finding is right but not worth fixing now, without committing it.
    Escalate {
        /// The question, on one line, with your position.
        #[arg(value_parser = NonEmptyStringValueParser::new())]
        question: String,
        /// The open fix-now finding it is about, as in 2-m2bn.1.
        #[arg(long, value_name = "FINDING")]
        finding: Option<String>,
    },
    /// Open this round's review file of an item in review, or of a mission's plan, or take it over, and print the brief.
    Review { id: String },
    /// Record your verdict in your review file and commit it.
    Verdict {
        verdict: VerdictArg,
        /// The model you run on, such as gpt-6.1-sol; written into the review file.
        #[arg(long)]
        model: Option<String>,
    },
    /// Answer the oldest question waiting on an item, in the checkout on its branch, without committing it.
    Decide {
        id: String,
        /// The answer, on one line.
        #[arg(value_parser = NonEmptyStringValueParser::new())]
        answer: String,
        /// Settle the question's finding as deferred: real, but for later work.
        #[arg(long, conflicts_with = "no_action")]
        defer: bool,
        /// Settle the question's finding with no action.
        #[arg(long)]
        no_action: bool,
    },
    /// Lead a ready mission, in this checkout of the base without committing it, and see what each of its items needs.
    Lead { mission: String },
    /// Merge this branch's approved item into the base, once its checks pass, in one commit that marks it done.
    Merge {
        /// Merge an item nobody reviewed, whose correctness is evident from its diff, giving why.
        #[arg(long, value_name = "WHY")]
        trivial: Option<String>,
    },
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
            if error.use_stderr() && skip::reason().is_some() {
                eprint!("{}", output::not_skippable());
            }
            return ExitCode::from(error.exit_code() as u8);
        }
    };
    // A skip needs its reason. A checkout cannot be refused, so the
    // post-checkout hook only says so.
    if skip::variable().is_some_and(|reason| reason.is_empty()) {
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
    // `check` reads the records itself, and reports a config that does not
    // read rather than stopping at it.
    if let Command::Check { hook, args } = &cli.command {
        return exit(match hook {
            None => check::check(),
            Some(hook) => check::hook(*hook, args),
        });
    }
    let result = repo::Repo::open().and_then(|mut repo| match cli.command {
        Command::Status => commands::status(&mut repo).map(Outcome::Done),
        Command::Check { .. } => unreachable!("check runs before the repository is opened"),
        Command::Setup { harness: None } => setup::git_hooks(&repo.root),
        Command::Setup {
            harness: Some(harness),
        } => setup::harness(&repo.root, harness),
        Command::Show { id } => views::show(&mut repo, &id),
        Command::List { deferred: true, .. } => views::deferred(&mut repo),
        Command::List {
            ready,
            blocked,
            needs_human,
            mission,
            label,
            mine,
            done,
            deferred: false,
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
        Command::Ready { ids, by } => plan::ready(&mut repo, &ids, &by),
        Command::Drop { id, reason } => plan::drop(&mut repo, &id, reason.as_deref()),
        Command::Close { id, result } => plan::close(&mut repo, &id, result.as_deref()),
        Command::Start { id: Some(id), .. } => commands::start(&mut repo, &id),
        Command::Start {
            new: Some(title), ..
        } => commands::start_new(&mut repo, &title),
        Command::Start { .. } => unreachable!("clap requires an ID or --new"),
        Command::Submit => commands::submit(&mut repo),
        Command::Escalate { question, finding } => {
            questions::escalate(&mut repo, &question, finding.as_deref())
        }
        Command::Review { id } => review::review(&mut repo, &id),
        Command::Verdict { verdict, model } => {
            let verdict = match verdict {
                VerdictArg::Approve => Verdict::Approve,
                VerdictArg::Changes => Verdict::Changes,
            };
            review::verdict(&mut repo, verdict, model)
        }
        Command::Decide {
            id,
            answer,
            defer,
            no_action,
        } => {
            let settled = match (defer, no_action) {
                (true, _) => Some(atelier_core::Outcome::Defer),
                (_, true) => Some(atelier_core::Outcome::NoAction),
                _ => None,
            };
            questions::decide(&mut repo, &id, &answer, settled)
        }
        Command::Lead { mission } => views::lead(&mut repo, &mission),
        Command::Merge { trivial } => merge::merge(&repo.root, trivial),
    });
    exit(result)
}

/// Prints the outcome, after each rule `ATELIER_SKIP` passed. Every refusal
/// of a rule was passed or offered the skip, so a refusal or an error
/// while a reason is given is of something that cannot be skipped.
fn exit(result: anyhow::Result<Outcome>) -> ExitCode {
    let (out, err, code) = match result {
        Ok(Outcome::Done(text)) => (text, String::new(), 0),
        Ok(Outcome::Refused(text)) => (String::new(), text, 1),
        Ok(Outcome::NotYet(text)) => (text, String::new(), 3),
        Err(error) => (String::new(), format!("atelier: {error:#}\n"), 2),
    };
    let mut err = err;
    if code == 1 || code == 2 {
        err += &skip::reason().map_or(String::new(), |_| output::not_skippable());
    }
    // A closed stdout, as under `| head`, is no failure of the command.
    let out = output::skipped(&skip::passed()) + &out;
    let _ = std::io::stdout().write_all(out.as_bytes());
    let _ = std::io::stderr().write_all(err.as_bytes());
    ExitCode::from(code)
}
