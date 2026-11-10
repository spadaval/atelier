//! The `atelier` command (docs/strategies/redesign-product.md, Commands).

mod commands;
mod forge;
mod git;
mod merge;
mod output;
mod plan;
mod repo;
mod review;
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
    /// Open this round's review file of an item in review, or take it over, and print the brief.
    Review { id: String },
    /// Record your verdict in your review file and commit it.
    Verdict {
        verdict: VerdictArg,
        /// The model you run on, such as gpt-6.1-sol; written into the review file.
        #[arg(long)]
        model: Option<String>,
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
    let cli = Cli::parse();
    let result = repo::Repo::open().and_then(|mut repo| match cli.command {
        Command::Status => commands::status(&mut repo).map(Outcome::Done),
        Command::Show { id } => views::show(&mut repo, &id),
        Command::List {
            ready,
            blocked,
            needs_human,
            mission,
            label,
            mine,
            done,
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
        } => plan::new(&mut repo, &title, into, mission),
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
        Command::Start { id: Some(id), .. } => commands::start(&mut repo, &id),
        Command::Start {
            new: Some(title), ..
        } => commands::start_new(&mut repo, &title),
        Command::Start { .. } => unreachable!("clap requires an ID or --new"),
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
        Command::Merge { trivial } => merge::merge(&repo.root, trivial),
    });
    // A closed stdout, as under `| head`, is no failure of the command.
    match result {
        Ok(Outcome::Done(text)) => {
            let _ = std::io::stdout().write_all(text.as_bytes());
            ExitCode::SUCCESS
        }
        Ok(Outcome::Refused(text)) => {
            let _ = std::io::stderr().write_all(text.as_bytes());
            ExitCode::from(1)
        }
        Ok(Outcome::NotYet(text)) => {
            let _ = std::io::stdout().write_all(text.as_bytes());
            ExitCode::from(3)
        }
        Err(error) => {
            let _ = writeln!(std::io::stderr(), "atelier: {error:#}");
            ExitCode::from(2)
        }
    }
}
