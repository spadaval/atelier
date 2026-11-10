//! The `atelier` command (docs/strategies/redesign-product.md, Commands).

mod commands;
mod git;
mod output;
mod repo;

use clap::{Parser, Subcommand};
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
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = repo::Repo::open().and_then(|mut repo| match cli.command {
        Command::Status => commands::status(&mut repo).map(Outcome::Done),
        Command::Start { id } => commands::start(&mut repo, &id),
        Command::Submit => commands::submit(&mut repo),
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
        Err(error) => {
            let _ = writeln!(std::io::stderr(), "atelier: {error:#}");
            ExitCode::from(2)
        }
    }
}
