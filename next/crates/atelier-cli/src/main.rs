//! The `atelier` command (docs/strategies/redesign-product.md, Commands).

mod commands;
mod forge;
mod git;
mod merge;
mod output;
mod repo;
mod review;

use atelier_core::Verdict;
use clap::{Parser, Subcommand, ValueEnum};
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
    /// Start a ready item on a new branch named for it, or take over a started one.
    Start { id: String },
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
    /// Merge this branch's approved item into the base, once its checks pass, in one commit that marks it done.
    Merge {
        /// Merge an item nobody reviewed, whose correctness is evident from its diff, giving why.
        #[arg(long, value_name = "WHY")]
        trivial: Option<String>,
    },
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
        Command::Start { id } => commands::start(&mut repo, &id),
        Command::Submit => commands::submit(&mut repo),
        Command::Review { id } => review::review(&mut repo, &id),
        Command::Verdict { verdict, model } => {
            let verdict = match verdict {
                VerdictArg::Approve => Verdict::Approve,
                VerdictArg::Changes => Verdict::Changes,
            };
            review::verdict(&mut repo, verdict, model)
        }
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
