//! `log`: the history of actions that `atelier-core` returns, one line each
//! (docs/design.md, The log). The line's layout lives here, and the order
//! and limit a reader gets; the words for what changed are the core's, which
//! the Desk shows too.

use crate::commands::Outcome;
use crate::view_output;
use anyhow::Result;
use atelier_core::model::Thread;
use atelier_core::{history, Logged, Repo};
use chrono::{DateTime, Utc};

/// How many actions `log` prints with no `--all`, newest first: at most
/// 9,000 characters of actions at [`WIDTH`] a line, and one line more.
pub const LIMIT: usize = 30;
/// The most characters a line holds with no `--all`.
pub const WIDTH: usize = 300;

pub fn log(
    repo: &mut Repo,
    id: Option<&str>,
    since: Option<DateTime<Utc>>,
    all: bool,
) -> Result<Outcome> {
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
    if all {
        return Ok(Outcome::Done(history.actions.iter().map(line).collect()));
    }
    let mut out: String = history
        .actions
        .iter()
        .rev()
        .take(LIMIT)
        .map(|logged| cut(line(logged)))
        .collect();
    let older = history.actions.len().saturating_sub(LIMIT);
    if older > 0 {
        let actions = if older == 1 { "action" } else { "actions" };
        out += &format!("{older} older {actions} not shown. {WHOLE}\n");
    }
    Ok(Outcome::Done(out))
}

/// What a reader adds to see everything whole. It names no ID, so no line
/// that says it outgrows [`WIDTH`].
const WHOLE: &str = "Add --all to print every action, oldest first, each line whole.";

/// `line`, a printed line ending in a line break, held to [`WIDTH`]
/// characters: one longer is cut and ends saying where it is whole.
fn cut(line: String) -> String {
    let text = line.trim_end_matches('\n');
    if text.chars().count() <= WIDTH {
        return line;
    }
    let tail = " [cut; add --all to print it whole]";
    let kept: String = text.chars().take(WIDTH - tail.chars().count()).collect();
    format!("{}{tail}\n", kept.trim_end())
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

#[cfg(test)]
mod tests {
    use super::*;

    /// No ID or thread, however long, takes a printed line past the width:
    /// the cut's ending and the closing line name none.
    #[test]
    fn a_cut_line_and_the_closing_line_stay_within_the_width() {
        let long = format!("{} {}\n", "a".repeat(250), "b".repeat(5000));
        let cut = cut(long);
        assert!(cut.trim_end().chars().count() <= WIDTH, "{cut}");
        assert!(cut.ends_with(" [cut; add --all to print it whole]\n"));
        let closing = format!("{} older actions not shown. {WHOLE}", usize::MAX);
        assert!(closing.chars().count() <= WIDTH, "{closing}");
    }
}
