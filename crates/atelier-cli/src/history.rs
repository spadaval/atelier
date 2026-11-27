//! `log`: the history of actions that `atelier-core` returns, one line each
//! (docs/design.md, The log). The line's layout lives here; the words for
//! what changed are the core's, which the Desk shows too.

use crate::commands::Outcome;
use crate::view_output;
use anyhow::Result;
use atelier_core::model::Thread;
use atelier_core::{history, Logged, Repo};
use chrono::{DateTime, Utc};

pub fn log(repo: &mut Repo, id: Option<&str>, since: Option<DateTime<Utc>>) -> Result<Outcome> {
    let Some(history) = history(repo, id, since)? else {
        let id = id.expect("only an ID can name no record");
        return Ok(Outcome::Refused(view_output::unknown(id, &repo.base)));
    };
    for passed in &history.passed_over {
        eprintln!(
            "atelier log: passed over {} in commit {}, which does not read: {}",
            passed.path,
            passed.commit,
            one_line(&passed.problem)
        );
    }
    Ok(Outcome::Done(history.actions.iter().map(line).collect()))
}

/// `<time> <commit> <id> <action> <thread>`, then what changed, then each
/// skip with the thread that passed it, the last two in parts separated by
/// `; `.
pub fn line(logged: &Logged) -> String {
    let thread = logged
        .thread
        .as_ref()
        .map_or("-", |thread| thread.0.as_str());
    let mut line = format!(
        "{} {} {} {} {thread}",
        logged.time.format("%Y-%m-%dT%H:%M:%SZ"),
        logged.commit,
        logged.id,
        logged.action.name(),
    );
    let mut parts = logged.action.changed();
    let by = passed_by(logged.skipped_by.as_ref());
    parts.extend(
        logged
            .skips
            .iter()
            .map(|skip| format!("skip {skip} ({by})")),
    );
    let parts: Vec<String> = parts.iter().map(|part| one_line(part)).collect();
    if !parts.is_empty() {
        line += " ";
        line += &parts.join("; ");
    }
    line + "\n"
}

/// Who passed a commit's rules, as `log` and `show` say it: `by <thread>`,
/// or that a commit from before the `Atelier-Skip-By` trailer does not say.
pub fn passed_by(thread: Option<&Thread>) -> String {
    match thread {
        Some(thread) => format!("by {thread}"),
        None => "thread not recorded".into(),
    }
}

/// `text` with its whitespace made single spaces, so a line stays one.
pub fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
