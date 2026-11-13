//! `log`: the history of acts that `atelier-app` returns, one line each
//! (docs/design.md, The log). The layout lives here.

use crate::commands::Outcome;
use crate::view_output;
use anyhow::Result;
use atelier_app::{history, Happened, Logged, Move, Repo};
use atelier_core::{Settlement, Status, Verdict};
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
    Ok(Outcome::Done(history.acts.iter().map(line).collect()))
}

/// `<time> <commit> <id> <act> <thread>`, then what changed, then each
/// skip, the last two in parts separated by `; `.
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
        logged.act.name(),
    );
    let mut parts = changed(&logged.act);
    parts.extend(logged.skips.iter().map(|skip| format!("skip {skip}")));
    let parts: Vec<String> = parts.iter().map(|part| one_line(part)).collect();
    if !parts.is_empty() {
        line += " ";
        line += &parts.join("; ");
    }
    line + "\n"
}

/// What changed, in the parts its line shows.
fn changed(act: &Happened) -> Vec<String> {
    let moved = |status: &Option<Move>| status.iter().map(shown).collect::<Vec<_>>();
    match act {
        Happened::Start { status } => moved(status),
        Happened::TakeOver { from, status } => {
            [vec![format!("from {from}")], moved(status)].concat()
        }
        Happened::Submit { status } | Happened::Close { status } => vec![shown(status)],
        Happened::Review { review } => vec![review.to_string()],
        Happened::ReviewTakeOver { review, from } => {
            vec![review.to_string(), format!("from {from}")]
        }
        Happened::Verdict {
            review,
            verdict,
            status,
        } => {
            let verdict = match verdict {
                Verdict::Approve => "approve",
                Verdict::Changes => "changes",
            };
            [vec![format!("{review} {verdict}")], moved(status)].concat()
        }
        Happened::Merge { status, trivial } => {
            let trivial = trivial.iter().map(|why| format!("trivial: {why}"));
            [shown(status)].into_iter().chain(trivial).collect()
        }
        Happened::Settle {
            finding,
            settlement,
        } => vec![match settlement {
            Settlement::Taken { item, .. } => format!("{finding} taken into {item}"),
            Settlement::Dropped { reason, .. } => format!("{finding} dropped: {reason}"),
        }],
        Happened::Lead { from } => from.iter().map(|from| format!("from {from}")).collect(),
        Happened::Ready { by, status } => {
            let by = by.iter().map(|by| format!("by {by}"));
            [shown(status)].into_iter().chain(by).collect()
        }
        Happened::Drop { status, reason } => {
            [shown(status)].into_iter().chain(reason.clone()).collect()
        }
    }
}

/// A status move, `<from> -> <to>`, each as a record spells it; `new` for
/// a record the commit added.
fn shown(status: &Move) -> String {
    let from = status.from.map_or("new", spelled);
    format!("{from} -> {}", spelled(status.to))
}

fn spelled(status: Status) -> &'static str {
    match status {
        Status::Draft => "draft",
        Status::Ready => "ready",
        Status::Building => "building",
        Status::InReview => "in_review",
        Status::ChangesRequested => "changes_requested",
        Status::Approved => "approved",
        Status::Done => "done",
        Status::Dropped => "dropped",
    }
}

/// `text` with its whitespace made single spaces, so a line stays one.
fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
