//! Everything `status` and the commands print. The wording lives here, in
//! one place, so it is cheap to change.

use atelier_app::acts::{row, Act, Condition, Refusal};
use atelier_app::{Entry, Overview};
use atelier_core::{Priority, Status, Thread};

fn status_name(status: Status) -> &'static str {
    match status {
        Status::Draft => "draft",
        Status::Ready => "ready",
        Status::Building => "building",
        Status::InReview => "in review",
        Status::ChangesRequested => "changes requested",
        Status::Approved => "approved",
        Status::Done => "done",
        Status::Dropped => "dropped",
    }
}

fn priority_name(priority: Priority) -> &'static str {
    match priority {
        Priority::Critical => "critical",
        Priority::High => "high",
        Priority::Medium => "medium",
        Priority::Low => "low",
    }
}

fn command(act: Act, id: &str) -> String {
    match act {
        Act::Start | Act::TakeOver | Act::Review => format!("atelier {} {id}", row(act).command),
        Act::MergeTrivial => "atelier merge --trivial \"<why>\"".into(),
        _ => format!("atelier {}", row(act).command),
    }
}

fn goal(act: Act, status: Status) -> &'static str {
    match act {
        Act::Start => "start it",
        Act::TakeOver => "take it over",
        Act::Submit if status == Status::ChangesRequested => "return it to review",
        Act::Submit => "submit it for review",
        Act::Review => "review it",
        Act::VerdictChanges | Act::VerdictApprove => "record your verdict",
        Act::Merge | Act::MergeTrivial => "merge it",
    }
}

fn condition(id: &str, condition: &Condition) -> String {
    let text = match condition {
        Condition::Blocked {
            id: blocker,
            status: Some(status),
        } => format!("{blocker} is done (it is {})", status_name(*status)),
        Condition::Blocked { id: blocker, status: None } => {
            format!("{blocker} is done (no item {blocker} is found)")
        }
        Condition::BranchExists => format!(
            "No branch {id} exists yet (one does; if nothing on it is needed, delete it with `git branch -D {id}`)"
        ),
        Condition::NoBranch => format!("Branch {id} exists"),
        Condition::Unticked(line) => format!("Done when: {line}"),
        Condition::NoReport => "Report: written under `## Report`".into(),
        Condition::NoResponse { finding, title } => format!("Finding {finding} (fix now): {title}"),
        Condition::Unanswered(question) => format!("Question answered: {question}"),
        Condition::FormerImplementer => format!("You have never implemented {id}"),
    };
    format!("  [ ] {text}")
}

/// What an entry needs and the command that follows, or what to do when no
/// command of the caller's follows.
fn next(entry: &Entry) -> String {
    let id = &entry.id;
    let Some(act) = entry.next else {
        return match entry.status {
            Status::InReview => format!(
                "Waiting for a reviewer's verdict. If no reviewer is working on it, start one with: Run `atelier review {id}` and follow what it prints.\n"
            ),
            Status::Draft => format!(
                "It is a draft. A planner makes it ready with: atelier ready {id} --by <name>\n"
            ),
            Status::Dropped => "It was dropped on the base; nothing is left to do on it.\n".into(),
            _ => "Nothing is left to do on it.\n".into(),
        };
    };
    needs(act, entry.status, id, &entry.unmet)
}

/// The conditions of `act` that do not hold, then its command.
fn needs(act: Act, status: Status, id: &str, unmet: &[Condition]) -> String {
    let mut text = String::new();
    if !unmet.is_empty() {
        text += &format!("To {}:\n", goal(act, status));
        for unmet in unmet {
            text += &condition(id, unmet);
            text += "\n";
        }
        if unmet
            .iter()
            .any(|c| matches!(c, Condition::NoResponse { .. }))
        {
            text += "Answer each finding under `### Responses` in the Report: `<finding>: fixed in <commit>` or `<finding>: rebutted: <fact>`.\n";
        }
    }
    text + &format!("Then: {}\n", command(act, id))
}

fn holder(entry: &Entry) -> String {
    match &entry.holder {
        Some(thread) => format!("held by {thread}"),
        None => "held by nobody".into(),
    }
}

/// `status` on an item's branch.
pub fn item(caller: &Thread, entry: &Entry) -> String {
    let id = &entry.id;
    let who = if entry.holds {
        format!(
            "You are the implementer of {id} (thread {caller}).\nItem: {}\n",
            crate::repo::item_path(id)
        )
    } else {
        format!(
            "You are thread {caller}, on the branch of {id}, which is {}.\n",
            holder(entry)
        )
    };
    format!(
        "{who}Title: {}\nStatus: {}\n{}",
        entry.title,
        status_name(entry.status),
        next(entry)
    )
}

/// `status` away from an item's branch.
pub fn overview(caller: &Thread, branch: Option<&str>, base: &str, overview: &Overview) -> String {
    let place = match branch {
        Some(branch) if branch == base => format!("the base branch {base}"),
        Some(branch) => format!("branch {branch}, which is no item's"),
        None => "a detached HEAD".into(),
    };
    let mut text = format!("You are thread {caller}, on {place}.\n");
    if !overview.held.is_empty() {
        text += "You hold:\n";
        for entry in &overview.held {
            text += &format!(
                "  {} ({}): {}\n",
                entry.id,
                status_name(entry.status),
                entry.title
            );
        }
    }
    if !overview.ready.is_empty() {
        text += "Ready to start:\n";
        for entry in &overview.ready {
            text += &format!(
                "  {} ({}): {}\n",
                entry.id,
                priority_name(entry.priority),
                entry.title
            );
            for unmet in &entry.unmet {
                text += &format!("  {}\n", condition(&entry.id, unmet));
            }
        }
    }
    if !overview.started.is_empty() {
        text += "Held by other threads:\n";
        for entry in &overview.started {
            text += &format!(
                "  {} ({}, {}): {}\n",
                entry.id,
                status_name(entry.status),
                holder(entry),
                entry.title
            );
        }
    }
    let startable = overview.ready.iter().find(|entry| entry.unmet.is_empty());
    match (overview.held.first(), startable) {
        (Some(held), _) => {
            text += &format!("Then: atelier start {} (switches to its branch)\n", held.id)
        }
        (None, Some(ready)) => text += &format!("Then: atelier start {}\n", ready.id),
        (None, None) if overview.ready.is_empty() => {
            text += "Nothing is ready to start.\n";
        }
        (None, None) => text += "Nothing is ready to start until what blocks it is done.\n",
    }
    text
}

pub fn refusal(act: Act, refusal: &Refusal, caller: &Thread, entry: &Entry) -> String {
    let id = &entry.id;
    match refusal {
        Refusal::From { act, status } => {
            let from: Vec<&str> = row(*act).from.iter().map(|s| status_name(*s)).collect();
            format!(
                "{id} is {}, and `atelier {}` runs on an item that is {}. Nothing changed.\n{}",
                status_name(*status),
                row(*act).command,
                from.join(", "),
                next(entry)
            )
        }
        Refusal::NotHolder { holder } => {
            let holder = match holder {
                Some(thread) => format!("{thread} does"),
                None => "nobody does".into(),
            };
            format!(
                "You (thread {caller}) do not hold {id}; {holder}. Nothing changed.\nTo take it over: atelier start {id}\n"
            )
        }
        Refusal::Unmet(_) => format!(
            "`{}` refused; nothing changed.\n{}",
            command(act, id),
            next(entry)
        ),
        Refusal::Record(problem) => format!("{problem}\n"),
    }
}

pub fn no_item(id: &str, base: &str) -> String {
    format!(
        "No item {id} is on the base branch {base} or on a branch named {id}. Nothing changed.\n`atelier status` lists the items you can start.\n"
    )
}

pub fn lost_race(id: &str, holder: Option<&Thread>) -> String {
    let holder = holder.map_or("another thread".to_string(), |t| t.to_string());
    format!(
        "{holder} started {id} first and holds it. Nothing changed.\nTo take it over, run `atelier start {id}` again.\n"
    )
}

pub fn not_switched(id: &str) -> String {
    format!(
        "Started {id}: branch {id} holds its start, but this checkout could not switch to it. Run `git switch {id}` once the cause below is fixed"
    )
}

pub fn switch_failed(id: &str) -> String {
    format!(
        "Could not switch to branch {id}, so nothing changed. If another worktree has it checked out, run `atelier start {id}` there"
    )
}

pub fn not_committed() -> String {
    "The commit failed, so the record is as it was. Fix what Git says below, then run the command again".into()
}

pub fn started(id: &str) -> String {
    format!("Started {id} on branch {id}.\n")
}

pub fn taken_over(id: &str, before: Option<&Thread>) -> String {
    match before {
        Some(thread) => format!("Took over {id}; {thread} held it.\n"),
        None => format!("Took over {id}, which nobody held.\n"),
    }
}

pub fn already_held(id: &str) -> String {
    format!("You already hold {id}; this checkout is on its branch.\n")
}

pub fn submitted(id: &str) -> String {
    format!(
        "Submitted {id} for review.\nStart a reviewer with: Run `atelier review {id}` and follow what it prints.\n"
    )
}

pub fn not_on_item(branch: Option<&str>) -> String {
    let place = branch.map_or("a detached HEAD".to_string(), |b| format!("branch {b}"));
    format!(
        "`atelier submit` runs on an item's branch, and this checkout is on {place}. Nothing changed.\n`atelier status` lists what you hold; `atelier start <id>` switches to its branch.\n"
    )
}
