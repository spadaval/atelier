//! Everything `status` and the commands print. The wording lives here, in
//! one place, so it is cheap to change.

use crate::forge::{CiRun, ForgeError, PullRequest};
use crate::merge::Moved;
use crate::setup::{Harness, Step};
use atelier_app::acts::{row, Act, Condition, Refusal, Role};
use atelier_app::{Carried, Entry, Overview};
use atelier_core::{
    Config, Decision, Finding, FindingName, Outcome, Priority, Response, Settlement, Status,
    Thread, Verdict,
};
use atelier_records::{Problem, ReviewFile};
use std::path::Path;
use std::time::Duration;

pub(crate) fn status_name(status: Status) -> &'static str {
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

pub(crate) fn priority_name(priority: Priority) -> &'static str {
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
        Act::Ready | Act::ReadyMission => format!("atelier ready {id} --by <name>"),
        Act::StartNew => "atelier start --new \"<title>\"".into(),
        Act::MergeTrivial => "atelier merge --trivial \"<why>\"".into(),
        Act::Drop => format!("atelier drop {id} --reason \"<why>\""),
        Act::Close => format!("atelier close {id} \"<result>\""),
        Act::CloseMission => format!("atelier close {id}"),
        Act::Escalate => "atelier escalate \"<question>\"".into(),
        Act::Decide => format!("atelier decide {id} \"<answer>\""),
        _ => format!("atelier {}", row(act).command),
    }
}

fn goal(act: Act, status: Status) -> &'static str {
    match act {
        Act::Ready | Act::ReadyMission => "make it ready",
        Act::Start | Act::StartNew => "start it",
        Act::TakeOver => "take it over",
        Act::Submit if status == Status::ChangesRequested => "return it to review",
        Act::Submit => "submit it for review",
        Act::Review => "review it",
        Act::VerdictChanges | Act::VerdictApprove => "record your verdict",
        Act::Escalate => "ask a question",
        Act::Decide => "decide its question",
        Act::Merge | Act::MergeTrivial => "merge it",
        Act::Drop => "drop it",
        Act::Close | Act::CloseMission => "close it",
        Act::PlanVerdictChanges | Act::PlanVerdictApprove => "record your verdict",
        Act::Lead => "lead it",
        Act::Settle => "settle it",
    }
}

pub(crate) fn condition(id: &str, condition: &Condition) -> String {
    format!("  [ ] {}", condition_text(id, condition))
}

/// What `condition` needs, and what stands instead, in one line: also the
/// rule a skip of it names.
fn condition_text(id: &str, condition: &Condition) -> String {
    match condition {
        Condition::Blocked {
            id: blocker,
            status: Some(status),
        } => format!("{blocker} is done (it is {})", status_name(*status)),
        Condition::Blocked { id: blocker, status: None } => {
            format!("{blocker} is done (no item {blocker} is found)")
        }
        Condition::MissionNotReady { id: mission, status: Some(Status::Draft) } => format!(
            "Mission {mission} is ready (it is draft; a planner makes it ready with: atelier ready {mission} --by <name>)"
        ),
        Condition::MissionNotReady { id: mission, status: Some(Status::Dropped) } => format!(
            "Mission {mission} is ready (it was dropped, so none of its items starts)"
        ),
        Condition::MissionNotReady { id: mission, status: Some(status) } => {
            format!("Mission {mission} is ready (it is {})", status_name(*status))
        }
        Condition::MissionNotReady { id: mission, status: None } => format!(
            "Mission {mission} is ready (no mission {mission} is in the base's last commit)"
        ),
        Condition::BranchExists => format!(
            "No branch {id} exists yet (one does; if nothing on it is needed, delete it with `git branch -D {id}`)"
        ),
        Condition::NoBranch => format!("Branch {id} exists"),
        Condition::Unticked(line) => format!("Done when: {line}"),
        Condition::NoReport => "Report: written under `## Report`".into(),
        Condition::NoResponse { finding, title, rebutted: false } => {
            format!("Finding {finding} (fix now): {title}")
        }
        Condition::NoResponse { finding, title, rebutted: true } => format!(
            "Finding {finding} (fix now): {title} (its rebuttal was kept, and a rebuttal gets one try: fix it, or escalate it)"
        ),
        Condition::Unanswered(question) => format!("Question answered: {question}"),
        Condition::FormerImplementer => format!("You have never implemented {id}"),
        Condition::ReviewedFinding(finding) => format!("You have never reviewed finding {finding}"),
        Condition::NoQuestion => "A question waits under `## Decisions` (none does)".into(),
        Condition::DecisionDropped { finding, outcome } => format!(
            "Your file carries {finding} with the `Decided: {outcome}` line the tool wrote (a decision settled it; put that finding back as it was)"
        ),
        Condition::DecisionChanged(finding) => format!(
            "Finding {finding} has no `Decided:` line the tool did not write, and carries no finding a decision settled (put it back as the tool wrote it, or remove it)"
        ),
        Condition::NoOpenFinding => "Your review file holds an open fix-now finding (it holds none; with nothing to fix, run `atelier verdict approve`)".into(),
        Condition::OpenFinding { finding, title } => {
            format!("No fix-now finding is open; {finding} is: {title}")
        }
        Condition::NoApproval => "A review of the latest round approves it (none does)".into(),
        Condition::Stale { round, commit, changed } => format!(
            "The approval still covers the head: since {}, which round {round} reviewed, nothing changed outside this item's directory, nor its Goal, Done when or Scope (changed: {})",
            short(commit),
            some(changed)
        ),
        Condition::Uncommitted(paths) => format!(
            "Every change to a tracked file or to this item's directory is committed (these are not: {})",
            some(paths)
        ),
        Condition::Reviewed => {
            "Nobody has reviewed it (it has a review file, so it merges once approved, with `atelier merge`)".into()
        }
        Condition::ItemOpen { id: item, status } => {
            format!("{item} is done or dropped (it is {})", status_name(*status))
        }
        Condition::NoResult => "Result: written under `## Result`".into(),
        Condition::NoFinding(finding) => {
            format!("A review of {id} has a finding {finding} (none does)")
        }
        Condition::NotDeferred(finding) => format!(
            "Finding {id}/{finding} is deferred, by its recommendation or by a decision (it is not)"
        ),
        Condition::Settled {
            finding,
            settlement,
        } => format!(
            "Finding {id}/{finding} is not settled yet (it was {})",
            settled(settlement)
        ),
        Condition::Invalid(problem) => valid(problem),
    }
}

/// The rule that every record is valid, as `problem` breaks it.
pub(crate) fn valid(problem: &Problem) -> String {
    format!("Every record is valid ({problem})")
}

fn role_name(role: Role) -> &'static str {
    match role {
        Role::Planner => "planner",
        Role::Implementer => "implementer",
        Role::Reviewer => "reviewer",
        Role::Decider => "decider",
        Role::Lead => "lead",
    }
}

/// The rules `refusal` of `act` on `id` says are broken, each in one line,
/// as a skip names them.
pub(crate) fn rules(act: Act, id: &str, refusal: &Refusal) -> Vec<String> {
    vec![match refusal {
        Refusal::Role(role) => format!(
            "`atelier {}` is run by {id}'s {}, not its {}",
            row(act).command,
            role_name(row(act).role),
            role_name(*role)
        ),
        Refusal::From { act, status } => rule_from(id, row(*act).from, *status),
        Refusal::NotHolder { role, holder } => format!(
            "You hold {id} as its {} ({} does)",
            role_name(*role),
            holder
                .as_ref()
                .map_or("nobody".to_string(), Thread::to_string)
        ),
        Refusal::Unmet(unmet) => return unmet.iter().map(|c| condition_text(id, c)).collect(),
        Refusal::Record(problem) => problem.to_string(),
    }]
}

/// The rule that `id` is one of `from`, which it is not.
pub(crate) fn rule_from(id: &str, from: &[Status], status: Status) -> String {
    let from: Vec<&str> = from.iter().map(|s| status_name(*s)).collect();
    format!(
        "{id} is {} (it is {})",
        from.join(" or "),
        status_name(status)
    )
}

/// The rule that planning, or with `settle` the settling of a finding,
/// happens on the base `base`, not on `branch`.
pub(crate) fn rule_off_base(base: &str, branch: Option<&str>, settle: bool) -> String {
    let or = if settle {
        " or an item's you implement"
    } else {
        ""
    };
    format!(
        "This checkout is on the base branch {base}{or} (it is on {})",
        place(branch)
    )
}

/// What an entry needs and the command that follows, or what to do when no
/// command of the caller's follows. `dropped` is why the item was dropped,
/// from its record.
fn next(entry: &Entry, dropped: Option<&str>, decider: Option<&str>) -> String {
    let id = &entry.id;
    let Some(act) = entry.next else {
        if entry.reviewed.is_some() && !entry.holds {
            return "Your review is done, and nothing on this item is yours to do. Say so to whoever started you, and stop.\n".into();
        }
        return match entry.status {
            Status::InReview => match &entry.reviewer {
                Some(reviewer) => format!(
                    "Thread {reviewer} holds this round's review. Wait for its verdict, then run `atelier status`.\n"
                ),
                None => format!("No reviewer has opened this round's review yet.\n{}", start_reviewer(id)),
            },
            Status::Draft => format!(
                "It is a draft. A planner makes it ready with: atelier ready {id} --by <name>\n"
            ),
            Status::Dropped => match dropped {
                Some(why) => format!("It was dropped on the base: {why}\nNothing is left to do on it.\n"),
                None => "It was dropped on the base; nothing is left to do on it.\n".into(),
            },
            _ => "Nothing is left to do on it.\n".into(),
        };
    };
    needs(act, entry.status, id, &entry.unmet, decider)
}

/// How an implementer gets its item reviewed.
fn start_reviewer(id: &str) -> String {
    let task = format!("Run atelier review {id} and follow what it prints.");
    handoff(
        "A reviewer",
        &[task],
        "Then wait for its verdict, and run `atelier status`.",
    )
}

/// The line that has `who` started as another thread with each of
/// `tasks`, then says what to do meanwhile. A task has no backticks:
/// pasted into a double-quoted shell argument, they would run the command
/// as the caller.
pub(crate) fn handoff(who: &str, tasks: &[String], then: &str) -> String {
    let (start, give) = match tasks.len() {
        1 => ("one", "this task"),
        _ => ("one for each task", "its task"),
    };
    let mut text = format!(
        "{who} must be another thread: not you, and not a subagent of yours, which runs as your thread. Start {start} with your harness's tool for a new agent thread, and give it {give}:\n"
    );
    for task in tasks {
        text += &format!("  {task}\n");
    }
    text + then + "\n"
}

/// The conditions of `act` that do not hold, then its command.
/// `decider` says who answers a question waiting on the item, where the
/// caller knows.
fn needs(act: Act, status: Status, id: &str, unmet: &[Condition], decider: Option<&str>) -> String {
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
            text += "Answer each finding under `### Responses` in the Report: `<finding>: fixed in <commit>` or `<finding>: rebutted: <fact>`. Or, when it is right but not worth fixing now, or its fix leaves the item's scope, escalate it: atelier escalate --finding <finding> \"<question>\"\n";
        }
        if unmet
            .iter()
            .any(|c| matches!(c, Condition::OpenFinding { .. }))
        {
            text += "A finding that stands needs `atelier verdict changes`. A carried one closes when your file says `Now: fixed` or `Now: withdrawn` for it.\n";
        }
        if unmet.iter().any(|c| matches!(c, Condition::Uncommitted(_))) {
            text += "Commit them, or set them aside with `git stash`.\n";
        }
        if unmet.iter().any(|c| matches!(c, Condition::Unanswered(_))) {
            text += &decider.map_or_else(
                || format!("A decider answers each question with `atelier decide {id}`: a thread that never implemented this item nor reviewed the finding a question is about. `atelier status` says who.\n"),
                str::to_string,
            );
        }
        // A stale approval is answered by a fresh review, not by merging.
        if unmet.iter().any(|c| matches!(c, Condition::Stale { .. })) {
            text += "What changed needs a fresh review, so send it back to review.\n";
            return text + &format!("Then: {}\n", command(Act::Submit, id));
        }
    }
    let or = match act {
        Act::VerdictApprove | Act::PlanVerdictApprove => {
            " (or `atelier verdict changes`, once your file holds a fix-now finding)"
        }
        _ => "",
    };
    if act == Act::Submit {
        // The reviewer reviews the branch's last commit.
        text +=
            "`atelier submit` commits only the item's directory, so commit your change first.\n";
    }
    if act == Act::TakeOver {
        // Another thread holds it; taking it over is a choice, not a next step.
        return text
            + &format!(
                "Only if you mean to take it over from its holder: {}\n",
                command(act, id)
            );
    }
    text + &format!("Then: {}{or}\n", command(act, id))
}

pub(crate) fn holder(entry: &Entry) -> String {
    match &entry.holder {
        Some(thread) => format!("held by {thread}"),
        None => "held by nobody".into(),
    }
}

/// `status` on an item's branch. `config` is the base's, whose merge
/// checks and forge `merge` goes through, `dropped` why the item was
/// dropped, and `decider` who answers a question waiting on it.
pub fn item(
    caller: &Thread,
    entry: &Entry,
    config: &Config,
    dropped: Option<&str>,
    decider: Option<&str>,
) -> String {
    let id = &entry.id;
    let who = if let Some(review) = &entry.review {
        format!(
            "You are the reviewer of {id} (thread {caller}), round {}.\nReview file: {}\n`atelier review {id}` prints the brief again.\n",
            review.round,
            review_path(id, &review.to_string())
        )
    } else if entry.holds {
        format!(
            "You are the implementer of {id} (thread {caller}).\nItem: {}\n",
            crate::repo::item_path(id)
        )
    } else if let (Some((name, verdict)), false) = (&entry.reviewed, entry.holds) {
        let verdict = verdict.map_or("no verdict", verdict_name);
        format!(
            "You reviewed {id} (thread {caller}): review {name}, {verdict}. It is {}.\n",
            holder(entry)
        )
    } else if entry.status == Status::Dropped {
        // The base's record is the item's, and names no holder.
        format!("You are thread {caller}, on the branch of {id}.\n")
    } else {
        format!(
            "You are thread {caller}, on the branch of {id}, which is {}.\n",
            holder(entry)
        )
    };
    let mut next = next(entry, dropped, decider.filter(|_| entry.holds));
    // Before `merge`, what it will run and wait for; a stale approval is
    // sent to `submit` instead.
    let stale = entry
        .unmet
        .iter()
        .any(|c| matches!(c, Condition::Stale { .. }));
    if entry.next == Some(Act::Merge) && !stale {
        let then = next.rfind("Then: ").expect("it ends with a Then line");
        next.insert_str(then, &gates(config));
    }
    let code = outside(id, &entry.uncommitted);
    if entry.next == Some(Act::Submit) && !code.is_empty() {
        let then = next.rfind("Then: ").expect("it ends with a Then line");
        next.insert_str(then, &format!("Not committed: {}\n", some(&code)));
    }
    let undefined = match entry.undefined {
        true => format!(
            "Write its Goal and its Done when lines in {} first: they say what the change must do, and the reviewer judges it against them.\n",
            crate::repo::item_path(id)
        ),
        false => String::new(),
    };
    let since = match &entry.approved {
        Some(time) => format!(" since {time}"),
        None => String::new(),
    };
    format!(
        "{who}Title: {}\nStatus: {}{since}\n{undefined}{next}",
        entry.title,
        status_name(entry.status),
    )
}

/// What `merge` runs and waits for, from the base's config.
fn gates(config: &Config) -> String {
    let mut text = String::new();
    if !config.merge_checks.is_empty() {
        text += "`atelier merge` runs these merge checks on the tree merging makes:\n";
        for check in &config.merge_checks {
            text += &format!("  {}\n", check.run);
        }
    }
    if let Some(forge) = &config.forge {
        text += &format!(
            "It pushes the branch to {} and opens its pull request where `atelier submit` did not",
            forge.remote
        );
        text += &match &forge.ci_workflow {
            Some(workflow) => format!(
                ", waits up to {} for {workflow} to pass on the branch's latest code, then merges and pushes the base.\n",
                span(crate::merge::WAIT)
            ),
            None => ", then merges and pushes the base.\n".into(),
        };
    }
    text
}

/// `status` away from an item's branch. `dropped` is each item dropped on
/// the base while its branch is there, with why. `planning` is the record
/// changes the base's checkout has not committed, and `at` that checkout,
/// as for [`commit_line`].
pub fn overview(
    caller: &Thread,
    branch: Option<&str>,
    base: &str,
    overview: &Overview,
    dropped: &[(String, Option<String>)],
    planning: &[String],
    at: Option<&Path>,
) -> String {
    let place = match branch {
        Some(branch) if branch == base => format!("the base branch {base}"),
        Some(branch) => format!("branch {branch}, which is no item's"),
        None => "a detached HEAD".into(),
    };
    let mut text = format!("You are thread {caller}, on {place}.\n");
    if !overview.held.is_empty() {
        text += "You hold:\n";
        for entry in &overview.held {
            let since = entry.approved.as_ref();
            text += &format!(
                "  {} ({}{}): {}\n",
                entry.id,
                status_name(entry.status),
                since.map_or(String::new(), |time| format!(" since {time}")),
                entry.title
            );
            for question in &entry.questions {
                text += &format!("    A question waits on a decider: {question}\n");
            }
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
    if !dropped.is_empty() {
        text += "Dropped on the base while started; nothing is left to do on them:\n";
        for (id, why) in dropped {
            text += &format!("  {id}: {}\n", why.as_deref().unwrap_or("no reason given"));
        }
    }
    text += &crate::view_output::overview(overview);
    if !planning.is_empty() {
        text += &format!(
            "Planning not committed yet, which `atelier start` does not read until it is: {}\nCommit it with: {}\n",
            some(planning),
            commit_line(at, planning, "Plan")
        );
    }
    text += &crate::view_output::led(&overview.led);
    // A lead holding nothing hands its mission's items to other threads,
    // and is not pointed at one itself.
    if overview.held.is_empty() && !overview.led.is_empty() {
        return text;
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

/// `entry` is what `status` shows the caller, which a refusal of the
/// item's status or of the act's conditions ends with, and `dropped` why
/// the item was dropped.
pub fn refusal(
    act: Act,
    refusal: &Refusal,
    caller: &Thread,
    id: &str,
    entry: Option<&Entry>,
    dropped: Option<&str>,
) -> String {
    let shown = || entry.expect("a refusal that says what comes next has the entry");
    match refusal {
        Refusal::Role(Role::Reviewer) => {
            format!("You are the reviewer of {id}. Reviewers run `review` and `verdict`.\n")
        }
        Refusal::Role(Role::Implementer) => format!(
            "You are the implementer of {id}. Implementers run `start`, `submit`, `escalate` and `merge`.\n"
        ),
        Refusal::Role(Role::Planner | Role::Lead | Role::Decider) => {
            unreachable!("no thread holds a planner's, a lead's or a decider's role on an item")
        }
        Refusal::From { act, status } => {
            let from: Vec<&str> = row(*act).from.iter().map(|s| status_name(*s)).collect();
            format!(
                "{id} is {}, and `atelier {}` runs on an item that is {}. Nothing changed.\n{}",
                status_name(*status),
                row(*act).command,
                from.join(", "),
                next(shown(), dropped, None)
            )
        }
        Refusal::NotHolder {
            role: Role::Reviewer,
            holder: Some(holder),
        } => format!(
            "You (thread {caller}) do not hold this round's review of {id}; {holder} does. Nothing changed.\nTo take it over: atelier review {id}\n"
        ),
        Refusal::NotHolder {
            role: Role::Reviewer,
            holder: None,
        } => format!("No review of {id} is open in this round. Nothing changed.\nTo open one: atelier review {id}\n"),
        Refusal::NotHolder { holder, .. } => {
            let holder = match holder {
                Some(thread) => format!("{thread} does"),
                None => "nobody does".into(),
            };
            format!(
                "You (thread {caller}) do not hold {id}; {holder}. Nothing changed.\nTo take it over: atelier start {id}\n"
            )
        }
        Refusal::Unmet(unmet) => format!(
            "`{}` refused; nothing changed.\n{}",
            command(act, id),
            needs(act, shown().status, id, unmet, None)
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

pub fn started_new(id: &str) -> String {
    format!("Created {id} and started it on branch {id}.\n")
}

/// `word` as one shell word: as it is when it holds nothing a shell reads,
/// else in single quotes.
fn quoted(word: &str) -> String {
    let plain = !word.is_empty()
        && word
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "_-./:@%+=,".contains(c));
    match plain {
        true => word.to_string(),
        false => format!("'{}'", word.replace('\'', "'\\''")),
    }
}

/// The Git command that commits `paths`, and only them, with `message`.
/// `at` is the checkout they are in, which the command names with `-C`
/// when the caller is not at its root (see [`crate::repo::git_at`]).
pub(crate) fn commit_line(at: Option<&Path>, paths: &[String], message: &str) -> String {
    let git = match at {
        Some(at) => format!("git -C {}", quoted(&at.display().to_string())),
        None => "git".into(),
    };
    let paths: Vec<String> = paths.iter().map(|path| quoted(path)).collect();
    let paths = paths.join(" ");
    format!("{git} add -- {paths} && {git} commit -m \"{message}\" -- {paths}")
}

/// What a planning command prints once it has written `paths`, which are
/// in the checkout `at` names, as for [`commit_line`]. `item` is the item
/// whose branch it ran on, when it settled a finding there.
pub fn planned(
    said: &str,
    item: Option<&str>,
    at: Option<&Path>,
    paths: &[String],
    message: &str,
) -> String {
    let mut text = format!("{said}Wrote, and did not commit:\n");
    for path in paths {
        text += &format!("  {path}\n");
    }
    text += &match item {
        None => "Planning counts once it is committed on the base.\n".into(),
        Some(id) => format!(
            "Once committed, it reaches the base when {id} merges. It is outside {id}'s directory, so a review of {id} must see it: an approval from before it is stale, and `atelier merge` will say to run `atelier submit`.\n"
        ),
    };
    text + &format!("Then: {}\n", commit_line(at, paths, message))
}

pub fn created_item(id: &str) -> String {
    format!(
        "Created draft item {id}, with empty Goal, Done when and Scope sections to write.\nA planner makes it ready with: atelier ready {id} --by <name>\n"
    )
}

pub fn created_mission(id: &str) -> String {
    format!(
        "Created draft mission {id}, with empty sections to write.\nIts items are created with: atelier new \"<title>\" --in {id}\nA planner makes it ready with: atelier ready {id} --by <name>\n"
    )
}

/// Where to run a command that runs on the base branch `base`, which
/// `checkout`, if any, has checked out.
fn on_base(base: &str, checkout: Option<&Path>) -> String {
    match checkout {
        Some(path) => format!(
            "Run it in {}, which has {base} checked out.",
            path.display()
        ),
        None => format!("Run it in a checkout of {base}, as after `git switch {base}`."),
    }
}

pub fn off_base(
    command: &str,
    base: &str,
    branch: Option<&str>,
    checkout: Option<&Path>,
) -> String {
    format!(
        "`atelier {command}` writes planning, which happens on the base branch {base}, and this checkout is on {}. Nothing changed.\n{}\n",
        place(branch),
        on_base(base, checkout)
    )
}

/// `new --from` or `drop` on a finding, run on a branch that is not the
/// base's nor that of an item the caller implements.
pub fn settle_elsewhere(
    command: &str,
    base: &str,
    branch: Option<&str>,
    checkout: Option<&Path>,
) -> String {
    format!(
        "`atelier {command}` settles a finding on the base branch {base}, or on the branch of an item you implement, and this checkout is on {}. Nothing changed.\n{}\n",
        place(branch),
        on_base(base, checkout)
    )
}

/// `name`, given to `new --from` or `drop`, is no finding's name.
pub fn not_a_finding(command: &str, name: &str) -> String {
    format!(
        "`atelier {command} {name}` refused; nothing changed.\n`{name}` is not a finding's name, <item>/<review>.<number>, as `atelier list --deferred` prints it.\n"
    )
}

/// `new --from` or `drop` on finding `name` of item `id`, refused for its
/// item's status, for the finding, or both.
pub fn settle_refused(command: &str, name: &str, id: &str, refusals: &[Refusal]) -> String {
    let mut text = format!("`atelier {command} {name}` refused; nothing changed.\n");
    for refusal in refusals {
        text += &match refusal {
            Refusal::From { status, .. } => format!(
                "  [ ] {id} is done (it is {}; only a done item's findings are settled)\n",
                status_name(*status)
            ),
            Refusal::Unmet(unmet) => unmet.iter().map(|c| condition(id, c) + "\n").collect(),
            Refusal::Role(_) | Refusal::NotHolder { .. } | Refusal::Record(_) => {
                unreachable!("settling a finding needs no role, and a record problem is an error")
            }
        };
    }
    text + "`atelier list --deferred` lists the deferred findings nobody has settled.\n"
}

/// How a finding was settled, after "it was".
pub(crate) fn settled(settlement: &Settlement) -> String {
    match settlement {
        Settlement::Taken { item, by } => format!("taken into {item} by {by}"),
        Settlement::Dropped { by, reason } => format!("dropped by {by}: {reason}"),
    }
}

pub fn finding_taken(name: &str, id: &str) -> String {
    format!(
        "Created draft item {id} from finding {name}: its Goal quotes the finding, and its Done when and Scope are empty to write.\nMarked finding {name} taken into {id}.\nA planner makes {id} ready with: atelier ready {id} --by <name>\n"
    )
}

/// A planning command's write failed. `created` says it had written a new
/// item, which it removes, and `left` names the directories of those it
/// could not remove.
pub fn write_failed(command: &str, created: bool, left: &[String]) -> String {
    let failed = format!("`atelier {command}` could not write its records");
    match (created, left) {
        (false, _) => failed,
        (true, []) => {
            format!("{failed}, so it removed the new item it had written; nothing changed")
        }
        (true, left) => format!(
            "{failed}, and could not remove the new item it had written; remove {} by hand",
            left.join(" and ")
        ),
    }
}

pub fn finding_dropped(name: &str, reason: &str) -> String {
    format!("Dropped finding {name}: {reason}\n")
}

/// The problems a planning command found in what it would write, with
/// paths shown from the checkout's root.
pub fn invalid(command: &str, root: &Path, problems: &[Problem]) -> String {
    let mut text = format!("`atelier {command}` refused; nothing changed.\n");
    for problem in problems {
        text += &format!("  {}\n", from_root(root, problem));
    }
    text + "Correct what each names, in the command or in the record, then run it again.\n"
}

/// `problem` with its file's path from the checkout's root.
pub(crate) fn from_root(root: &Path, problem: &Problem) -> Problem {
    let file = problem.file.strip_prefix(root).unwrap_or(&problem.file);
    Problem {
        file: file.to_path_buf(),
        ..problem.clone()
    }
}

/// "2 items, 1 mission and config.toml".
fn records((items, missions): (usize, usize)) -> String {
    let plural = |n: usize, word: &str| match n {
        1 => format!("1 {word}"),
        n => format!("{n} {word}s"),
    };
    format!(
        "{}, {} and config.toml",
        plural(items, "item"),
        plural(missions, "mission")
    )
}

/// Records that do not read, and records that break the rule that every
/// record is valid, each with its file and field.
fn problems(unread: &[Problem], invalid: &[Problem]) -> String {
    let mut text = String::new();
    if !unread.is_empty() {
        text += "These do not read as records, so no rule can be checked on them:\n";
    }
    for problem in unread {
        text += &format!("  {problem}\n");
    }
    if !invalid.is_empty() {
        text += "Rule: every record is valid. These are not:\n";
    }
    for problem in invalid {
        text += &format!("  {problem}\n");
    }
    text
}

fn not_committed_records(paths: &[String]) -> String {
    match paths.is_empty() {
        true => String::new(),
        false => format!("Record changes not committed yet: {}\n", some(paths)),
    }
}

pub fn checked(counts: (usize, usize), uncommitted: &[String]) -> String {
    format!(
        "Every record is valid: {}.\n{}",
        records(counts),
        not_committed_records(uncommitted)
    )
}

pub fn check_found(
    counts: (usize, usize),
    unread: &[Problem],
    invalid: &[Problem],
    uncommitted: &[String],
) -> String {
    let found = unread.len() + invalid.len();
    let plural = if found == 1 { "" } else { "s" };
    format!(
        "`atelier check` found {found} problem{plural} in {}.\n{}{}Fix each in the file named, then run atelier check again.\n",
        records(counts),
        problems(unread, invalid),
        not_committed_records(uncommitted)
    )
}

/// The pre-commit hook's refusal: records that do not read or validate as
/// staged, and each status changed as no act changes it, with the status
/// `HEAD` has and the one staged.
pub fn commit_refused(
    unread: &[Problem],
    invalid: &[Problem],
    acts: &[(String, Status, Status)],
) -> String {
    let mut text = "Atelier's pre-commit hook refused this commit; nothing was committed.\n"
        .to_string()
        + &problems(unread, invalid);
    if !acts.is_empty() {
        text += "Rule: a status changes only as an act of the lifecycle changes it. These change it as no act does:\n";
    }
    for (path, before, now) in acts {
        text += &format!(
            "  {path}: status: {} to {}. Put back the status HEAD has; atelier status names the command for what comes next.\n",
            status_name(*before),
            status_name(*now)
        );
    }
    text + "Fix each file named, stage it with git add, and commit again.\n"
}

/// The rule that a status changes only as an act changes it, as the
/// record at `path` breaks it.
pub(crate) fn rule_status(path: &str, from: Status, to: Status) -> String {
    format!(
        "A status changes only as an act of the lifecycle changes it ({path}: {} to {})",
        status_name(from),
        status_name(to)
    )
}

/// The rule that code reaches `base` only through `atelier merge`, as
/// `commit` breaks it.
pub(crate) fn rule_code(base: &str, commit: &str, subject: &str) -> String {
    format!(
        "Code and config.toml reach {base} only through atelier merge ({} {subject})",
        short(commit)
    )
}

/// The rule that an item with a branch is done only through `atelier
/// merge`, as `commit` breaks it for item `id`.
pub(crate) fn rule_done(base: &str, commit: &str, id: &str) -> String {
    format!(
        "An item whose branch exists is done on {base} only through atelier merge ({} sets {id} done)",
        short(commit)
    )
}

/// How many commits a refused push lists.
const LISTED: usize = 10;

fn commits<T>(found: &[(String, String, T)], what: impl Fn(&T) -> String) -> String {
    let mut text = String::new();
    for (commit, subject, detail) in found.iter().take(LISTED) {
        text += &format!("  {} {subject}: {}\n", short(commit), what(detail));
    }
    if found.len() > LISTED {
        text += &format!("  and {} more\n", found.len() - LISTED);
    }
    text
}

/// The pre-push hook's refusal of a push of `base`, whose commit on the
/// remote, if it has one this clone has, is `old`: records that do not
/// read, commits that bring code without an Atelier merge, with the paths
/// they change outside planning, and commits that set an item done while
/// its branch exists, with the item.
pub fn push_refused(
    base: &str,
    old: Option<&str>,
    unread: &[Problem],
    code: &[(String, String, Vec<String>)],
    done: &[(String, String, String)],
) -> String {
    // With no commit of the remote's to name, the commands are described.
    let (back, drop) = match old.map(short) {
        Some(old) => (
            format!("git branch -f {base} {old}"),
            format!("git rebase -i {old}"),
        ),
        None => (
            format!("moving {base} back to the remote's commit"),
            "git rebase -i".to_string(),
        ),
    };
    let mut text = "Atelier's pre-push hook refused this push; nothing was pushed.\n".to_string()
        + &problems(unread, &[]);
    if !unread.is_empty() {
        text += "Fix each record in the commit that broke it: git commit --amend for the last commit, git rebase -i for an earlier one. A later commit that fixes it leaves the broken one in the push.\n";
    }
    if !code.is_empty() {
        text += &format!(
            "Rule: code and config.toml reach {base} only through atelier merge. Each new commit on {base}'s first-parent line is an Atelier merge, or changes only .atelier/missions/ and .atelier/items/. These are neither:\n"
        );
        text += &commits(code, |paths| format!("changes {}", some(paths)));
        text += &format!(
            "Move them to an item's branch: atelier start --new \"<title>\" starts one that holds them. Then {back} takes them off {base}, and atelier merge brings them in once the item is approved.\n"
        );
    }
    if !done.is_empty() {
        text += &format!(
            "Rule: an item whose branch exists is done on {base} only through atelier merge. These set one done:\n"
        );
        text += &commits(done, |id| {
            format!("sets {id} done while branch {id} exists")
        });
        text += &format!(
            "Take each such commit off {base}: git reset --keep HEAD^ when it is the last, else {drop} and drop it. Then run atelier merge on the item's branch.\n"
        );
    }
    text
}

/// The pre-push hook's refusal when the checkout's `config.toml`, which
/// names the base it guards, does not read.
pub fn push_no_config(problem: &Problem) -> String {
    format!(
        "Atelier's pre-push hook refused this push; nothing was pushed.\n{}Fix it in this checkout, then push again.\n",
        problems(std::slice::from_ref(problem), &[])
    )
}

/// What the post-checkout hook prints when `status` fails. The checkout
/// itself went ahead.
pub fn no_status(error: &anyhow::Error) -> String {
    format!("Switched; `atelier status` cannot say more here: {error:#}\n")
}

/// The pre-commit or pre-push hook's refusal when the clone's own hook,
/// kept as `<name>.local`, failed after Atelier's rule passed.
pub fn kept_refused(name: &str, why: &str) -> String {
    let what = match name {
        "pre-push" => "pushed",
        _ => "committed",
    };
    format!(
        "This clone's own {name} hook, which atelier setup kept as {name}.local in its hooks directory, refused: {why}. Nothing was {what}.\nIts output above says why; do what it asks, then run the Git command again.\n"
    )
}

/// What the post-checkout hook prints when the clone's own hook failed.
/// The checkout itself went ahead.
pub fn kept_failed(name: &str, why: &str) -> String {
    format!("Switched; this clone's own {name} hook, kept as {name}.local, failed: {why}.\n")
}

/// The line `status` gives a clone that has none of Atelier's Git hooks.
pub fn no_hooks() -> String {
    "This clone has none of Atelier's Git hooks, so nothing checks its commits and pushes. Install them with: atelier setup\n".into()
}

/// `setup` refused because Git runs the clone's hooks from `dir`, which
/// `core.hooksPath` sets, outside the Git directory its worktrees share,
/// whose hooks directory is `shared`.
pub fn hooks_elsewhere(dir: &Path, shared: &Path) -> String {
    let shared = quoted(&shared.display().to_string());
    format!(
        "`atelier setup` refused; nothing changed.\nGit runs this clone's hooks from {}, as core.hooksPath says, and setup installs hooks only in the Git directory every worktree of the clone shares.\nCopy any hook this clone needs from there into {shared}, then run: git config core.hooksPath {shared} && atelier setup\n",
        dir.display()
    )
}

/// `setup` refused because each hook of `names` in `dir` is not Atelier's
/// and `<name>.local`, where it would be kept, is taken.
pub fn hooks_taken(dir: &Path, names: &[&str]) -> String {
    let mut text = format!(
        "`atelier setup` refused; nothing changed.\nsetup keeps a hook of this clone's own by moving it to <name>.local, and in {} that name is taken:\n",
        dir.display()
    );
    for name in names {
        text += &format!("  {name}, and {name}.local\n");
    }
    text + "For each, put what both files do into <name>.local and remove <name>, then run: atelier setup\n"
}

/// What `setup` did to each hook in `dir`.
pub fn hooks_set(dir: &Path, steps: &[(&str, Step)]) -> String {
    if steps.iter().all(|(_, step)| *step == Step::Unchanged) {
        return format!(
            "Atelier's Git hooks are in {} already; nothing changed.\n",
            dir.display()
        );
    }
    let mut text = format!(
        "Installed Atelier's Git hooks in {}, which every worktree of this clone shares:\n",
        dir.display()
    );
    for (name, step) in steps {
        text += &match step {
            Step::Unchanged => format!("  {name}: Atelier's already\n"),
            Step::Written => format!("  {name}: written\n"),
            Step::Kept { runs: true } => format!(
                "  {name}: written; this clone's own {name} hook was moved to {name}.local, and runs after Atelier's rule passes\n"
            ),
            Step::Kept { runs: false } => format!(
                "  {name}: written; this clone's own {name} hook, which is not executable and so did not run, was moved to {name}.local\n"
            ),
        };
    }
    text
}

fn harness_name(harness: Harness) -> &'static str {
    match harness {
        Harness::Claude => "claude",
        Harness::Codex => "codex",
    }
}

/// `setup claude` or `setup codex` refused, for `why`, to change `file`.
pub fn harness_refused(harness: Harness, file: &str, why: &str) -> String {
    format!(
        "`atelier setup {0}` refused; nothing changed.\n{why}.\nFix {file}, then run: atelier setup {0}\n",
        harness_name(harness)
    )
}

/// What `setup claude` or `setup codex` did to `file`.
pub fn harness_set(harness: Harness, file: &str, changed: bool) -> String {
    let what = match harness {
        Harness::Claude => {
            "A Claude Code session started in this repository begins with atelier status, and each turn ends with atelier check."
        }
        Harness::Codex => {
            "A Codex session started in this repository begins with atelier status, once Codex trusts the project and you have trusted the hook in its /hooks list."
        }
    };
    match changed {
        true => format!("Wrote Atelier's hooks into {file}, keeping every other hook there.\n{what}\nCommit {file} so that every clone has it.\n"),
        false => format!("Atelier's hooks are in {file} already; nothing changed.\n{what}\n"),
    }
}

pub fn no_mission(id: &str) -> String {
    format!(
        "`atelier new` refused; nothing changed.\nNo mission {id} is in this checkout. `atelier new --mission \"<title>\"` creates one.\n"
    )
}

fn unknown(id: &str) -> String {
    format!("  {id}: no item or mission has this ID in this checkout or on a branch\n")
}

pub fn set_unknown(id: &str) -> String {
    format!(
        "`atelier set {id}` refused; nothing changed.\n{}",
        unknown(id)
    )
}

pub fn item_only(id: &str, flags: &[&str]) -> String {
    format!(
        "`atelier set {id}` refused; nothing changed.\n{id} is a mission, and {} only an item has. A mission's fields are --title and --labels.\n",
        flags.join(", ")
    )
}

pub fn not_planned(id: &str, status: Status) -> String {
    format!(
        "`atelier set {id}` refused; nothing changed.\n{id} is {}, and its record no longer changes.\n",
        status_name(status)
    )
}

pub fn set_started(id: &str, status: Status) -> String {
    format!(
        "`atelier set {id}` refused; nothing changed.\n{id} is {}: it is started, and its record now changes only on branch {id}, by its implementer.\n",
        status_name(status)
    )
}

pub fn ready_refused(unknown_ids: &[&str], refused: &[(&str, Status)]) -> String {
    let mut text = "`atelier ready` refused; nothing changed.\n".to_string();
    for id in unknown_ids {
        text += &unknown(id);
    }
    for (id, status) in refused {
        text += &format!(
            "  {id}: it is {}, and `atelier ready` readies a draft\n",
            status_name(*status)
        );
    }
    text + "Run it again with only the drafts to make ready.\n"
}

pub fn readied(ids: &[&str], by: &str) -> String {
    format!("Made ready, as {by} said: {}.\n", ids.join(", "))
}

/// A `start` refused because `ready`, the planning it needs, is in the
/// base's checkout and not committed. `other` is that checkout when it is
/// not the caller's, `at` names it as for [`commit_line`], and `paths` are
/// its uncommitted planning.
pub fn not_committed_ready(
    id: &str,
    ready: &[String],
    base: &str,
    other: Option<&Path>,
    at: Option<&Path>,
    paths: &[String],
) -> String {
    let checkout = match other {
        Some(path) => format!("the checkout of {base} at {}", path.display()),
        None => "this checkout".into(),
    };
    let is = if ready.len() > 1 { "are" } else { "is" };
    format!(
        "`atelier start {id}` refused; nothing changed.\n{} {is} ready in {checkout}, but not in {base}'s last commit, which is what `atelier start` reads: the change is not committed yet.\nCommit the planning with: {}\nThen: atelier start {id}\n",
        ready.join(" and "),
        commit_line(at, paths, "Plan")
    )
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

/// The paths of `uncommitted` outside the item's directory.
fn outside(id: &str, uncommitted: &[String]) -> Vec<String> {
    let dir = format!(".atelier/items/{id}/");
    let outside = uncommitted.iter().filter(|path| !path.starts_with(&dir));
    outside.cloned().collect()
}

/// `forge` is what `submit` did on the forge, if one is configured.
pub fn submitted(id: &str, uncommitted: &[String], forge: &str) -> String {
    let code = outside(id, uncommitted);
    let mut text = format!("Submitted {id} for review.\n{forge}");
    if !code.is_empty() {
        text += &format!(
            "These changes are not committed, so the reviewer will not see them: {}. Commit them now, before the reviewer starts.\n",
            some(&code)
        );
    }
    text + &start_reviewer(id)
}

pub fn submit_not_pushed(id: &str, remote: &str, error: &anyhow::Error) -> String {
    format!(
        "Warning: pushing branch {id} to {remote} failed, so CI has not started: {error:#}. The item is in review all the same; `atelier merge` will push the branch.\n"
    )
}

pub fn submit_no_pull_request(id: &str, error: &ForgeError, pushed: bool) -> String {
    let (reason, _) = forge_reason(error);
    let (pushed, push) = if pushed {
        (format!("Pushed branch {id}, but "), "")
    } else {
        (String::new(), "push the branch and ")
    };
    format!(
        "Warning: {pushed}opening its pull request failed, so CI has not started: {reason}. The item is in review all the same; `atelier merge` will {push}open the pull request.\n"
    )
}

fn place(branch: Option<&str>) -> String {
    branch.map_or("a detached HEAD".to_string(), |b| format!("branch {b}"))
}

pub fn not_on_item(command: &str, branch: Option<&str>) -> String {
    format!(
        "`atelier {command}` runs on an item's branch, and this checkout is on {}. Nothing changed.\n`atelier status` lists what you hold; `atelier start <id>` switches to its branch.\n",
        place(branch)
    )
}

fn review_path(id: &str, name: &str) -> String {
    format!(".atelier/items/{id}/reviews/{name}.md")
}

/// `command` on item `id` refused in a checkout on `branch`; `checkout` has
/// the item's branch, if one does.
pub fn elsewhere(command: &str, id: &str, branch: Option<&str>, checkout: Option<&Path>) -> String {
    let there = match checkout {
        Some(path) => format!(
            "Run it in {}, which has branch {id} checked out.",
            path.display()
        ),
        None => {
            format!("Run it there, or `git switch {id}` here if no other checkout has that branch.")
        }
    };
    format!(
        "`atelier {command} {id}` runs in the checkout on branch {id}, where its implementer works, and this checkout is on {}. Nothing changed.\n{there}\n",
        place(branch)
    )
}

/// `id` is the item, or the mission whose plan is reviewed.
pub fn review_taken_over(id: &str, round: u32, before: Option<&Thread>) -> String {
    match before {
        Some(thread) => format!(
            "Took over round {round}'s review of {id}; {thread} held it. The file keeps what {thread} wrote.\n\n"
        ),
        None => format!("Took over round {round}'s review of {id}, which nobody held.\n\n"),
    }
}

pub fn author(id: &str, caller: &Thread) -> String {
    format!(
        "You (thread {caller}) have implemented {id}, and an author does not review its own work. Nothing changed.\n{}What a reviewer is shown:\n\n",
        start_reviewer(id)
    )
}

fn response(response: Option<&Response>) -> String {
    match response {
        Some(Response::Fixed { commit, .. }) => format!("fixed in {commit}"),
        Some(Response::Rebutted { fact, .. }) => format!("rebutted: {fact}"),
        None => "none in the Report".into(),
    }
}

/// The body of a new review file: a Summary to write, then each finding
/// it carries, kept until the reviewer says otherwise. An item's carried
/// finding shows the author's response; a plan has no Responses.
pub fn template(carried: &[Carried], item: bool) -> String {
    let mut text =
        "\n## Summary\n\n<!-- What you checked and how, and what you could not check. -->\n"
            .to_string();
    for (number, carried) in (1..).zip(carried) {
        let (finding, title) = (&carried.finding.name, &carried.finding.title);
        if let Some(outcome) = carried.decision.as_ref().and_then(|d| d.outcome) {
            text += &format!(
                "\n## Finding {number}: {title}\nRecommendation: fix now\nWas: {finding}\nDecided: {outcome}\n<!-- A decision settled it, and the tool wrote `Decided:`. Leave this finding as it is. -->\n"
            );
            continue;
        }
        let response = match item {
            true => format!("Response: {}\n", response(carried.response.as_ref())),
            false => String::new(),
        };
        text += &format!(
            "\n## Finding {number}: {title}\nRecommendation: fix now\nWas: {finding}\n{response}Now: kept\n<!-- Set Now: to fixed, withdrawn or kept. -->\n"
        );
    }
    text
}

/// What `review` prints: the item, the change, what earlier rounds left
/// open, and, for a reviewer, its file and the verdict commands.
pub struct Brief<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub caller: &'a Thread,
    pub round: u32,
    /// Goal, Done when, Scope and Report, those the item has.
    pub sections: Vec<(&'a str, &'a str)>,
    pub base: &'a str,
    pub commit: &'a str,
    /// The commit the round before reviewed.
    pub previous: Option<&'a str>,
    pub carried: &'a [Carried],
    /// The caller's review file; `None` for an author.
    pub path: Option<&'a str>,
}

pub fn brief(brief: &Brief) -> String {
    let Brief { id, round, .. } = *brief;
    let mut text = String::new();
    if brief.path.is_some() {
        text += &format!(
            "You are the reviewer of {id} (thread {}), round {round}.\n\
             Review the change on branch {id} as someone who did not write it: check that it meets each Done when line, that it is correct, and that the Report's claims hold. Read the code, run what proves it, and write down each problem you find. The author's reasoning is left out on purpose; judge what is there.\n\n",
            brief.caller
        );
    }
    text += &format!("Item {id}: {}\n", brief.title);
    for (heading, content) in &brief.sections {
        text += &format!("\n## {heading}\n{}", content.trim_end());
        text += "\n";
    }
    text += &format!(
        "\nThe change, at commit {commit}:\n  git diff {base}...{commit}\n",
        commit = brief.commit,
        base = brief.base
    );
    if let Some(previous) = brief.previous {
        text += &format!(
            "What changed since the last round's review:\n  git diff {previous} {}\n",
            brief.commit
        );
    }
    text += &carried_findings(brief.carried, brief.path.is_some(), true);
    let Some(path) = brief.path else {
        return text;
    };
    text + &review_file(
        path,
        brief.carried.len() + 1,
        "the change merges",
        "commits it with the item's new status",
    ) + "Your review is then done.\n"
}

/// The open findings of earlier rounds that a review carries, with the
/// author's response on an item's; how to judge each for a reviewer.
fn carried_findings(carried: &[Carried], reviewer: bool, item: bool) -> String {
    if carried.is_empty() {
        return String::new();
    }
    let mut text = "\nOpen findings from earlier rounds:\n".to_string();
    if reviewer {
        let with = if item {
            ", with the author's response"
        } else {
            ""
        };
        text += &format!("Your review file carries each as a finding of its own{with}. Judge it, then set the finding's `Now:` line to `fixed`, to `withdrawn` (the rebuttal holds, or the finding was wrong), or leave it `kept` (it still stands, and stays a fix-now finding).\n");
        if carried.iter().any(|c| c.decision.is_some()) {
            text += "One a decision settled has a `Decided:` line instead, which the tool wrote: leave it as it is.\n";
        }
    }
    for (number, carried) in (1..).zip(carried) {
        text += &format!(
            "\n{}, carried as Finding {number}: {}\n",
            carried.finding.name, carried.finding.title
        );
        for (name, written) in &carried.texts {
            text += &format!("  As {name}:\n");
            for line in written.trim().lines() {
                text += &format!("    {line}\n");
            }
        }
        if let Some(decision) = &carried.decision {
            let outcome = decision.outcome.map_or(String::new(), |o| o.to_string());
            text += &format!(
                "  Decided: {outcome}. The question: {}\n",
                decision.question
            );
            if let Some(answer) = &decision.answer {
                text += &format!("  Answer by {}: {}\n", answer.by, answer.text);
            }
        } else if item {
            text += &format!(
                "  The author's response: {}\n",
                response(carried.response.as_ref())
            );
        }
    }
    text
}

/// How a reviewer writes its file at `path`, whose findings number on from
/// `next`, and records its verdict, which `commits` it. `before` is what a
/// fix-now finding must be fixed before.
fn review_file(path: &str, next: usize, before: &str, commits: &str) -> String {
    format!(
        "\nYour review file: {path}\n\
         Write your review there, and change no other file. Leave its front matter, between the `---` lines, as it is.\n\
         Under `## Summary`, say what you checked and how, and what you could not check. Then add a section for each problem, numbered on from {next}:\n\n\
         ## Finding {next}: <a one-line title>\n\
         Recommendation: fix now\n\
         <What is wrong, where (file and line), and how you know.>\n\n\
         The Recommendation is `fix now` (it must be fixed before {before}), `defer` (real, but for later work) or `no action` (a note; nothing to do).\n\n\
         When the file is written, record your verdict. It checks the file, then {commits}:\n\
         \x20 atelier verdict changes --model <your model>   if the file holds an open fix-now finding\n\
         \x20 atelier verdict approve --model <your model>   if it holds none\n\
         Give the name of the model you run on, such as gpt-6.1-sol, or leave out `--model` if you do not know it. "
    )
}

/// What `review` prints for a mission's plan: the mission, its documents
/// and items, what earlier rounds left open, and the reviewer's file.
pub struct PlanBrief<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub caller: &'a Thread,
    pub round: u32,
    /// The sections of `mission.md` it has.
    pub sections: Vec<(&'a str, &'a str)>,
    /// The commit the plan is reviewed at.
    pub commit: &'a str,
    /// The paths of the mission's other documents.
    pub documents: Vec<String>,
    /// Each item of the mission: its path, status and title.
    pub items: Vec<(String, Status, String)>,
    pub carried: &'a [Carried],
    pub path: &'a str,
    pub base: &'a str,
}

pub fn plan_brief(brief: &PlanBrief) -> String {
    let PlanBrief { id, round, .. } = *brief;
    let mut text = format!(
        "You are a reviewer of the plan of mission {id} (thread {}), round {round}.\n\
         Review the plan as commit {} holds it, as someone who did not write it: the mission below, its other documents and its items. Read them, and write down each problem you find. Another reviewer may review the same plan blind, in a checkout of its own.\n\n\
         Mission {id}: {}\n",
        brief.caller,
        short(brief.commit),
        brief.title
    );
    for (heading, content) in &brief.sections {
        text += &format!("\n## {heading}\n{}\n", content.trim_end());
    }
    text += "\nIts other documents:\n";
    for path in &brief.documents {
        text += &format!("  {path}\n");
    }
    if brief.documents.is_empty() {
        text += "  (none)\n";
    }
    text += "Its items:\n";
    for (path, status, title) in &brief.items {
        text += &format!("  {path} ({}): {title}\n", status_name(*status));
    }
    if brief.items.is_empty() {
        text += "  (none)\n";
    }
    text += &carried_findings(brief.carried, true, false);
    text + &review_file(
        brief.path,
        brief.carried.len() + 1,
        "the mission starts",
        "commits it alone, changing no status",
    ) + &format!(
        "Then tell whoever started you the commit it prints, which the planner cherry-picks onto {}. Your review is then done.\n",
        brief.base
    )
}

pub fn plan_review_on_item(id: &str, branch: Option<&str>, base: &str) -> String {
    format!(
        "`atelier review {id}` reviews a mission's plan, and this checkout is on {}, an item's branch. Nothing changed.\nReview it in a detached worktree of {base}: git worktree add --detach <path> {base}\nThen: atelier review {id}\n",
        place(branch)
    )
}

pub fn plan_verdict_recorded(
    id: &str,
    round: u32,
    verdict: Verdict,
    path: &str,
    commit: &str,
    base: &str,
    on_base: bool,
) -> String {
    let then = match on_base {
        true => format!("It is on {base}."),
        false => format!("The planner puts it on {base} with: git cherry-pick {commit}"),
    };
    format!(
        "Recorded your verdict on the plan of mission {id}, round {round}: {}. Committed {path} alone, in commit {commit}.\n{then}\nYour review is done: tell whoever started you that commit, and stop.\n",
        verdict_name(verdict)
    )
}

/// `held` is the paths of the plan reviews without a verdict the caller
/// holds in this checkout, when there is more than one.
pub fn no_review_held(branch: Option<&str>, held: &[String]) -> String {
    let text = "`atelier verdict` records the verdict of one review you hold: on an item's branch, or of a mission's plan in this checkout";
    if !held.is_empty() {
        return format!(
            "{text}, and you hold more than one plan review without a verdict here: {}. Nothing changed.\nEach plan reviewer works in a detached worktree of its own (`git worktree add --detach <path>`); move all but one of these files to another, and run `atelier verdict` in each.\n",
            some(held)
        );
    }
    format!(
        "{text}. This checkout is on {}, and holds no plan review of yours without a verdict. Nothing changed.\n`atelier status` lists what you hold; `atelier review <id>` opens a review.\n",
        place(branch)
    )
}

/// A mission's act refused for `refusal`.
pub fn mission_refused(act: Act, id: &str, refusal: &Refusal) -> String {
    match refusal {
        Refusal::From { status, .. } => {
            let from: Vec<&str> = row(act)
                .from
                .iter()
                .filter(|s| atelier_core::MissionStatus::of(**s).is_some())
                .map(|s| status_name(*s))
                .collect();
            let then = match status {
                Status::Draft => {
                    format!("A planner makes it ready with: atelier ready {id} --by <name>\n")
                }
                _ => "Nothing is left to do on it.\n".into(),
            };
            format!(
                "Mission {id} is {}, and `atelier {}` runs on a mission that is {}. Nothing changed.\n{then}",
                status_name(*status),
                row(act).command,
                from.join(", ")
            )
        }
        Refusal::Unmet(unmet) => format!(
            "`{}` refused; nothing changed.\n{}",
            command(act, id),
            needs(act, Status::Ready, id, unmet, None)
        ),
        Refusal::Role(_) | Refusal::NotHolder { .. } | Refusal::Record(_) => {
            unreachable!("a mission's act is refused only for its status or conditions")
        }
    }
}

pub fn no_drop_reason(id: &str) -> String {
    format!(
        "`atelier drop` needs a reason, on one line. Nothing changed.\nThen: atelier drop {id} --reason \"<why it will not be done>\"\n"
    )
}

pub fn no_result(id: &str) -> String {
    format!(
        "`atelier close {id}` needs the item's result, the text its Report gets. Nothing changed.\nThen: atelier close {id} \"<what came of it>\"\n"
    )
}

pub fn close_mission_text(id: &str) -> String {
    format!(
        "`atelier close {id}` refused; nothing changed.\n{id} is a mission, whose result is written in {} under `## Result`. Write it there.\nThen: atelier close {id}\n",
        crate::repo::mission_path(id)
    )
}

pub fn close_started(id: &str) -> String {
    format!(
        "`atelier close {id}` refused; nothing changed.\nBranch {id} exists, so {id} was started, and a started item ends when `atelier merge` merges it from that branch. If it will not be done, drop it instead.\nThen: atelier drop {id} --reason \"<why>\"\n"
    )
}

pub fn unknown_refused(command: &str, id: &str) -> String {
    format!(
        "`atelier {command} {id}` refused; nothing changed.\n{}",
        unknown(id)
    )
}

/// `kind` is "item" or "mission". `started` says the item had a branch.
pub fn dropped(id: &str, kind: &str, reason: &str, started: bool) -> String {
    let mut text = format!("Dropped {kind} {id}: {reason}\n");
    if started {
        text += &format!("Its branch {id} is left as it is. Every command reads the base's record first, so once this is committed nobody can submit, merge or take it over.\n");
    }
    text
}

pub fn closed(id: &str, kind: &str) -> String {
    format!("Closed {kind} {id}: it is done.\n")
}

pub fn no_plan_review(id: &str) -> String {
    format!("Mission {id}'s plan has no review.\n")
}

/// What `ready` says of a mission's plan reviews: `latest` are those of
/// round `round`, and `changed` what changed in the plan since each commit
/// they reviewed, `None` where this clone lacks the commit.
pub fn plan_reviews(
    id: &str,
    round: u32,
    latest: &[&ReviewFile],
    changed: &[(&str, Option<Vec<String>>)],
) -> String {
    let mut text = format!("Mission {id}'s plan reviews, round {round}:\n");
    for file in latest {
        let verdict = file.review.verdict.map_or("no verdict yet", verdict_name);
        let by = file
            .review
            .reviewers
            .holder()
            .map_or("nobody".into(), |t| t.to_string());
        text += &format!(
            "  {} by {by}: {verdict}, of commit {}\n",
            file.name,
            short(&file.review.commit)
        );
    }
    for (commit, paths) in changed {
        text += &match paths {
            None => format!(
                "Commit {} is not in this clone, so what changed since is not known.\n",
                short(commit)
            ),
            Some(paths) if paths.is_empty() => format!(
                "Since {}, nothing in the mission's directory or its items changed.\n",
                short(commit)
            ),
            Some(paths) => format!("Since {}, these changed: {}\n", short(commit), some(paths)),
        };
    }
    text
}

pub fn verdict_name(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Approve => "approved",
        Verdict::Changes => "changes requested",
    }
}

pub fn verdict_recorded(
    id: &str,
    round: u32,
    verdict: Verdict,
    path: &str,
    implementer: Option<&Thread>,
) -> String {
    let to = match verdict {
        Verdict::Approve => "to merge",
        Verdict::Changes => "to answer your open findings",
    };
    let implementer = implementer.map_or("nobody yet".to_string(), |t| t.to_string());
    format!(
        "Recorded your verdict on {id}, round {round}: {}. Committed {path} with it.\n{id} is back with its implementer, {implementer}, {to}. Your review is done: say so to whoever started you, and stop.\n",
        verdict_name(verdict)
    )
}

/// Who answers `question`, the oldest waiting on item `id`: `lead`, a
/// mission and its lead, when it may decide it, else a new thread, which
/// reads the question and, when it is about a finding, the `review` file
/// that holds the finding. `commit` commits the question, when the lead
/// cannot see it yet because it is not committed.
pub fn decider(
    id: &str,
    question: &Decision,
    review: Option<&str>,
    lead: Option<(&str, &Thread)>,
    commit: Option<&str>,
) -> String {
    if let Some((mission, lead)) = lead {
        let commit = commit.map_or(String::new(), |line| {
            format!("{lead}'s `atelier status` reads this branch's last commit, so commit the question first: {line}\n")
        });
        return format!(
            "Thread {lead}, the lead of mission {mission}, decides it: {lead}'s `atelier status` lists the question.\n{commit}Tell whoever started you that it waits, and run `atelier status` once it is answered.\n"
        );
    }
    let task = match (&question.finding, review) {
        (Some(finding), Some(review)) => format!(
            "Run atelier show {id}, and read the question waiting under its Decisions and finding {finding} in {review}. Answer it with atelier decide {id} '<your answer>', adding --defer to defer the finding or --no-action to close it without a fix, or neither to leave it to be fixed."
        ),
        _ => format!(
            "Run atelier show {id}, and read the question waiting under its Decisions. Answer it with atelier decide {id} '<your answer>'."
        ),
    };
    handoff(
        "A decider",
        &[task],
        "Then wait for its answer, and run `atelier status`.",
    )
}

pub fn escalated(id: &str, question: &str, finding: Option<&FindingName>, decider: &str) -> String {
    let about = finding.map_or(String::new(), |f| format!("It is about finding {f}.\n"));
    format!(
        "Asked under `## Decisions` in {}, and did not commit it:\n  {question}\n{about}`atelier submit` and `atelier merge` wait until it is answered, and `atelier submit` commits it with the item.\n{decider}",
        crate::repo::item_path(id)
    )
}

/// `answered` is the question `decide` answered, `waiting` the next one,
/// and `approved` whether the item is approved, so its implementer merges
/// next rather than submits.
pub fn decided(
    id: &str,
    answered: &Decision,
    waiting: Option<&Decision>,
    implementer: Option<&Thread>,
    approved: bool,
) -> String {
    let answer = answered.answer.as_ref().expect("it was just answered");
    let mut text = format!(
        "Answered a question on {id}, as thread {}:\n  {}\n  Answer: {}\n",
        answer.by, answered.question, answer.text
    );
    if let (Some(finding), Some(outcome)) = (&answered.finding, answered.outcome) {
        text += &format!("Finding {finding} is settled, `Decided: {outcome}`: the next review carries it so, and no review reopens it.\n");
    }
    text += &format!(
        "Wrote {}, and did not commit it.\n",
        crate::repo::item_path(id)
    );
    if let Some(waiting) = waiting {
        return text
            + &format!(
                "Another question waits on {id}:\n  {}\nThen: {}\n",
                waiting.question,
                command(Act::Decide, id)
            );
    }
    let implementer = implementer.map_or("nobody yet".to_string(), |t| format!("thread {t}"));
    let next = match approved {
        true => "commits your answer, then runs `atelier merge`".to_string(),
        false => "runs `atelier submit`, which commits your answer with the item".into(),
    };
    text + &format!(
        "No other question waits. Its implementer, {implementer}, {next}. Your decision is done: say so to whoever started you, and stop.\n"
    )
}

/// `act`'s `what`, a question or an answer, was empty or had a line break.
pub fn not_one_line(act: Act, id: &str, what: &str) -> String {
    format!(
        "`{}` needs its {what} on one line. Nothing changed.\n",
        command(act, id)
    )
}

/// `escalate --finding <named>` on a finding that is not open; `known`
/// when a review of the item has it, and `open` the item's open fix-now
/// findings.
pub fn not_open(id: &str, named: &str, known: bool, open: &[Finding]) -> String {
    let mut text = "`atelier escalate` refused; nothing changed.\n".to_string();
    text += &match known {
        true => format!("Finding {named} of {id} is not an open fix-now finding.\n"),
        false => format!("No review of {id} has a finding {named}.\n"),
    };
    if open.is_empty() {
        return text + "It has no open fix-now finding. A question about anything else is asked without --finding.\n";
    }
    let names: Vec<String> = open.iter().map(|f| f.name.to_string()).collect();
    text + &format!("Its open fix-now findings: {}\n", names.join(", "))
}

/// `decide` given `settled` on its question `asked`, whose finding there is
/// none of, or is not open.
pub fn not_settled(id: &str, asked: &Decision, settled: Outcome) -> String {
    let flag = match settled {
        Outcome::Defer => "--defer",
        Outcome::NoAction => "--no-action",
    };
    let why = match &asked.finding {
        None => "is about no finding".to_string(),
        Some(finding) => format!("is about finding {finding}, which is no longer open"),
    };
    format!(
        "`atelier decide {id} {flag}` refused; nothing changed.\nIts oldest question, which it answers, {why}, so there is nothing to settle:\n  {}\nThen: {}\n",
        asked.question,
        command(Act::Decide, id)
    )
}

/// `decide` refused for who the caller is, or for no question waiting;
/// `handoff` says who decides the question that waits, which an
/// implementer is shown.
pub fn not_decider(
    id: &str,
    caller: &Thread,
    unmet: &[Condition],
    handoff: Option<&str>,
) -> String {
    let mut text = format!("`atelier decide {id}` refused; nothing changed.\n");
    for condition in unmet {
        text += &match condition {
            Condition::FormerImplementer => format!("You (thread {caller}) implemented {id}, and none of its implementers, past or present, decides its questions.\n"),
            Condition::ReviewedFinding(finding) => format!("You (thread {caller}) reviewed finding {finding}, which its question is about, and none of its reviewers decides it.\n"),
            Condition::NoQuestion => format!("{id} has no unanswered question under `## Decisions`.\n"),
            other => unreachable!("decide has no condition {other:?}"),
        };
    }
    if unmet.contains(&Condition::FormerImplementer) {
        return text + handoff.unwrap_or_default();
    }
    match unmet
        .iter()
        .any(|c| matches!(c, Condition::ReviewedFinding(_)))
    {
        true => text + "Say so to whoever started you, and stop.\n",
        false => text,
    }
}

pub fn unreadable(path: &str) -> String {
    format!("Written so, `## Decisions` in {path} would not read back as asked, so nothing was written. Look there for a code fence that is not closed")
}

fn short(commit: &str) -> &str {
    commit.get(..7).unwrap_or(commit)
}

/// The first five of `names`, and how many more.
fn some(names: &[String]) -> String {
    let mut text = names.iter().take(5).cloned().collect::<Vec<_>>().join(", ");
    if names.len() > 5 {
        text += &format!(" and {} more", names.len() - 5);
    }
    text
}

pub fn no_reason() -> String {
    "`atelier merge --trivial` needs a reason: `atelier merge --trivial \"<why the change is evidently correct>\"`. Nothing changed.\n".into()
}

pub fn waiting() -> String {
    "Another `atelier merge` is running in this clone; this one waits for it to finish.\n".into()
}

pub fn base_dirty(base: &str, path: &Path, dirty: &[String]) -> String {
    format!(
        "`atelier merge` refused: {base} is checked out at {}, where tracked files have uncommitted changes: {}. Nothing changed.\nMerging moves that checkout to the merge commit. Commit them or set them aside with `git stash` there, then run `atelier merge` again.\n",
        path.display(),
        some(dirty)
    )
}

/// The rule that the checkout of `base` at `path` has nothing uncommitted,
/// as `dirty` breaks it.
pub(crate) fn rule_base_dirty(base: &str, path: &Path, dirty: &[String]) -> String {
    format!(
        "Every change to a tracked file in the checkout of {base} at {} is committed (these are not: {})",
        path.display(),
        some(dirty)
    )
}

pub fn conflicts(id: &str, base: &str, paths: &[String]) -> String {
    format!(
        "`atelier merge` refused: {id} does not merge cleanly into {base}; these conflict: {}. Nothing changed.\nMerge {base} in with `git merge {base}` on branch {id}, resolve the conflicts and commit. `atelier status` then says what comes next.\n",
        some(paths)
    )
}

/// The merge commit's message. Its last line is the trailer the pre-push
/// hook knows an Atelier merge by.
pub fn merge_message(id: &str, title: &str, base: &str, config: &Config) -> String {
    let mut text =
        format!("Merge {id}: {title}\n\nMerges branch {id} into {base} and marks {id} done.\n");
    if !config.merge_checks.is_empty() {
        text += "These merge checks passed on this tree:\n";
        for check in &config.merge_checks {
            text += &format!("- {}\n", check.run);
        }
    }
    text + &format!("\nAtelier-Merge: {id}\n")
}

fn exit_status(code: Option<i32>) -> String {
    code.map_or("killed by a signal".to_string(), |code| {
        format!("exit {code}")
    })
}

/// The rule that the merge check `run` passes, which it failed with `code`.
pub(crate) fn rule_check(run: &str, code: Option<i32>) -> String {
    format!(
        "The merge check `{run}` passes (it failed: {})",
        exit_status(code)
    )
}

/// The rule that CI passed on the head, where `run` is the run that
/// failed, or the newest that has not finished.
pub(crate) fn rule_ci(workflow: &str, run: Option<&CiRun>) -> String {
    let state = match run {
        None => format!("no run of {workflow} has started"),
        Some(CiRun {
            conclusion: Some(conclusion),
            url,
            ..
        }) => format!("the latest run of {workflow} was {conclusion}: {url}"),
        Some(run) => format!(
            "{workflow} is {}: {}",
            run.status.replace('_', " "),
            run.url
        ),
    };
    format!("CI passed on the head ({state})")
}

pub fn check_failed(id: &str, base: &str, run: &str, code: Option<i32>, tail: &str) -> String {
    let code = exit_status(code);
    let mut text = format!(
        "`atelier merge` refused: the merge check `{run}` failed ({code}) on {id} merged into {base}. Nothing changed.\nThe end of its output:\n"
    );
    for line in tail.lines() {
        text += &format!("  {line}\n");
    }
    text + &format!(
        "Fix it on branch {id} and commit; if {base} broke it, bring {base} in first with `git merge {base}`. `atelier status` then says what comes next.\n"
    )
}

/// Why the forge did not answer, and what to do about it.
fn forge_reason(error: &ForgeError) -> (String, String) {
    match error {
        ForgeError::GhMissing => (
            "gh, the GitHub CLI, is not on PATH".to_string(),
            "Install it (https://cli.github.com) and run `gh auth login`".to_string(),
        ),
        ForgeError::SignedOut { host } => (
            format!("gh is not signed in to {host}"),
            "Run `gh auth login`".to_string(),
        ),
        ForgeError::Unreachable(message) => (
            format!("gh failed: {message}"),
            "Fix what it says (`gh auth status` shows the sign-in)".to_string(),
        ),
    }
}

pub fn forge_failed(error: &ForgeError, pushed: bool) -> String {
    let (reason, next) = forge_reason(error);
    let pushed = if pushed {
        " The branch was pushed."
    } else {
        ""
    };
    format!(
        "`atelier merge` refused: {reason}. Nothing was merged.{pushed}\n{next}, then run `atelier merge` again.\n"
    )
}

pub fn branch_push_failed(id: &str, remote: &str, error: &anyhow::Error) -> String {
    format!(
        "`atelier merge` refused: pushing branch {id} to {remote} failed: {error:#}. Nothing was merged.\nIf {remote} has commits on {id} that this branch lacks, bring them in with `git pull {remote} {id}`; otherwise fix what Git says. Then run `atelier merge` again.\n"
    )
}

/// The pull request, after the branch was pushed when `pushed`.
pub fn pull_request(id: &str, remote: &str, pull: &PullRequest, pushed: bool) -> String {
    let pushed = if pushed {
        format!("Pushed branch {id} to {remote}. ")
    } else {
        String::new()
    };
    format!("{pushed}Pull request #{}: {}\n", pull.number, pull.url)
}

/// The title and body of the pull request `merge` opens.
pub fn pull_request_text(id: &str, title: &str, base: &str) -> (String, String) {
    (
        format!("{id}: {title}"),
        format!(
            "Atelier item {id}. Its review is recorded in `.atelier/items/{id}/reviews/` on the branch, and `atelier merge` merges it into {base}."
        ),
    )
}

fn span(wait: Duration) -> String {
    match wait.as_secs() {
        seconds if seconds >= 60 && seconds % 60 == 0 => format!("{} minutes", seconds / 60),
        seconds => format!("{seconds} seconds"),
    }
}

pub fn waiting_for_ci(workflow: &str, pull: &PullRequest, wait: Duration) -> String {
    format!(
        "Waiting up to {} for {workflow} to finish on pull request #{}. Nothing is written while it waits, so this may be interrupted and `atelier merge` run again.\n",
        span(wait),
        pull.number
    )
}

/// `pending` is the newest run that counts, if any, or why the forge did
/// not answer.
pub fn ci_pending(
    workflow: &str,
    head: &str,
    pending: &Result<Option<CiRun>, ForgeError>,
) -> String {
    let state = match pending {
        Err(error) => format!("the forge did not answer, as {}", forge_reason(error).0),
        Ok(Some(run)) => match &run.conclusion {
            Some(conclusion) => format!("the latest was {conclusion} ({})", run.url),
            None => format!("it is {} ({})", run.status.replace('_', " "), run.url),
        },
        Ok(None) => "no run of it has started".into(),
    };
    format!(
        "CI has not finished on {}, the branch's head: {state}. Nothing else changed.\nRun `atelier merge` again once {workflow} finishes.\n",
        short(head)
    )
}

pub fn ci_failed(workflow: &str, run: &CiRun) -> String {
    let conclusion = run.conclusion.as_deref().unwrap_or("without a conclusion");
    format!(
        "`atelier merge` refused: CI failed: {workflow} finished {conclusion} on {} ({}). Nothing was merged.\nIf it failed for a reason outside the change, rerun it with `gh run rerun {id}` and run `atelier merge` again once it passes. Otherwise read the log with `gh run view {id} --log-failed`, fix the change on the branch and commit; `atelier status` then says what comes next.\n",
        short(&run.head_sha),
        run.url,
        id = run.id
    )
}

pub fn moved(moved: &Moved, id: &str, base: &str) -> String {
    let what = match moved {
        Moved::Branch => format!("branch {id}"),
        Moved::Base => format!("the base branch {base}"),
        Moved::Checkout(path) => format!("the checkout at {}", path.display()),
    };
    format!(
        "`atelier merge` wrote nothing: {what} changed while its checks ran, so they do not cover what would merge now.\nRun `atelier merge` again.\n"
    )
}

pub fn not_merged(base: &str) -> String {
    format!(
        "Moving {base} to the merge commit failed, so {base} is as it was. Run `atelier merge` again once the cause below is fixed"
    )
}

pub fn merged(id: &str, base: &str, commit: &str) -> String {
    format!(
        "Merged {id} into {base} with merge commit {}, which marks it done.\n",
        short(commit)
    )
}

pub fn base_pushed(base: &str, remote: &str) -> String {
    format!("Pushed {base} to {remote}.\n")
}

pub fn base_push_failed(base: &str, remote: &str, error: &anyhow::Error) -> String {
    format!(
        "Pushing {base} to {remote} failed: {error:#}. The merge stays on the local {base}.\nIf {remote}'s {base} has commits the local one lacks, bring them into the local {base} first. Then run `atelier merge` again on this branch to push it.\n"
    )
}

pub fn already_merged(id: &str, base: &str) -> String {
    format!("{id} is already merged into {base}; nothing is left to do on it.\n")
}

/// What ends every refusal of a rule, after the way out.
pub fn skip_offer() -> String {
    "If the rule is wrong for this case, go past it on purpose: run the same command again with ATELIER_SKIP=\"<why>\" in front of it.\n".into()
}

/// Each rule `ATELIER_SKIP` passed, as `<rule>: <reason>`.
pub fn skipped(passed: &[String]) -> String {
    let lines = passed
        .iter()
        .map(|line| format!("Skipped with ATELIER_SKIP: {line}\n"));
    lines.collect()
}

/// What ends a refusal of anything but a rule while `ATELIER_SKIP` is set.
pub fn not_skippable() -> String {
    "ATELIER_SKIP passes only a rule, and what refused this is not one, so it cannot be skipped.\n"
        .into()
}

/// `ATELIER_SKIP` set and blank; `refused` when the command stops for it.
pub fn blank_skip(refused: bool) -> String {
    let text = "ATELIER_SKIP is set and blank, and a skip needs a reason: ATELIER_SKIP=\"<why>\".";
    match refused {
        true => format!("{text} Nothing changed.\n"),
        false => format!("{text}\n"),
    }
}
