//! What the acts that live here say: their results and refusals, and the
//! wording of a refusal of any act and of what it needs, which `status`
//! and the commands of `atelier-cli` print too. Moved from `atelier-cli`'s
//! `output.rs`, which keeps the rest.

use crate::acts::{row, Act, Condition, Refusal, Role};
use crate::skip::Refused;
use crate::Entry;
use atelier_core::{Decision, Outcome, Settlement, Status, Thread, Verdict};
use atelier_records::{Problem, ReviewFile};
use std::path::Path;

pub fn status_name(status: Status) -> &'static str {
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

pub fn command(act: Act, id: &str) -> String {
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

pub fn goal(act: Act, status: Status) -> &'static str {
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

pub fn condition(id: &str, condition: &Condition) -> String {
    format!("  [ ] {}", condition_text(id, condition))
}

/// What `condition` needs, and what stands instead, in one line: also the
/// rule a skip of it names.
pub fn condition_text(id: &str, condition: &Condition) -> String {
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
pub fn valid(problem: &Problem) -> String {
    format!("Every record is valid ({problem})")
}

pub fn role_name(role: Role) -> &'static str {
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
pub fn rules(act: Act, id: &str, refusal: &Refusal) -> Vec<String> {
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
pub fn rule_from(id: &str, from: &[Status], status: Status) -> String {
    let from: Vec<&str> = from.iter().map(|s| status_name(*s)).collect();
    format!(
        "{id} is {} (it is {})",
        from.join(" or "),
        status_name(status)
    )
}

/// The rule that planning, or with `settle` the settling of a finding,
/// happens on the base `base`, not on `branch`.
pub fn rule_off_base(base: &str, branch: Option<&str>, settle: bool) -> String {
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
pub fn next(entry: &Entry, dropped: Option<&str>, decider: Option<&str>) -> String {
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
pub fn start_reviewer(id: &str) -> String {
    let task = format!("Run atelier review {id} and follow what it prints.");
    handoff(
        "A reviewer",
        &[task],
        "Then wait for its verdict, and run `atelier status`.",
    )
}

/// The line that has `who` started as another thread with each of
/// `tasks`, then says what to do meanwhile. A Codex subagent has an
/// identity of its own and a Claude Code one has its parent's
/// (`atelier_app::identity`), so the line says which may be used. It says
/// the caller starts it unasked, since a Codex thread otherwise waits for
/// its user's word to start a subagent. A task has no backticks: pasted
/// into a double-quoted shell argument, they would run the command as the
/// caller.
pub fn handoff(who: &str, tasks: &[String], then: &str) -> String {
    let (start, give) = match tasks.len() {
        1 => ("one", "this task"),
        _ => ("one for each task", "its task"),
    };
    let mut text = format!(
        "{who} must be a thread other than yours. Starting it is part of the work you were given, so start {start} yourself now, without waiting to be asked. Under Codex a subagent you start is one; under Claude Code a subagent runs as your thread and is refused, so use your harness's tool for a new agent thread. Give it fresh context, not a fork of yours, and give it {give}:\n"
    );
    for task in tasks {
        text += &format!("  {task}\n");
    }
    text + then + "\n"
}

/// What a thread that holds nothing on item `id` is told on its branch,
/// before `offer`, the offer to take it over: each of `questions`, which
/// wait unanswered, and how the oldest is answered, by the caller when
/// `decides`. The lead of its mission, `leads`, is not offered the item.
pub fn visiting(
    id: &str,
    questions: &[String],
    decides: bool,
    leads: Option<&str>,
    offer: &str,
) -> String {
    let mut text = match leads {
        Some(mission) => format!("You lead mission {mission}, which {id} is in: its implementer builds it, so you do not take it over.\n"),
        None => String::new(),
    };
    for question in questions {
        text += &format!("A question waits on a decider: {question}\n");
    }
    if !questions.is_empty() {
        text += &match decides {
            true => format!("Then: {}\n", command(Act::Decide, id)),
            false => format!("You may not decide it. A decider answers it with `atelier decide {id}`: a thread that never implemented this item nor reviewed the finding a question is about.\n"),
        };
    }
    match leads {
        Some(_) => text,
        None => text + offer,
    }
}

/// The conditions of `act` that do not hold, then its command.
/// `decider` says who answers a question waiting on the item, where the
/// caller knows.
pub fn needs(
    act: Act,
    status: Status,
    id: &str,
    unmet: &[Condition],
    decider: Option<&str>,
) -> String {
    let mut text = String::new();
    if !unmet.is_empty() {
        text += &format!("To {}:\n", goal(act, status));
        for unmet in unmet {
            text += &condition(id, unmet);
            text += "\n";
        }
        // A concrete line: agents copied "Finding <name>" from the list
        // above, or wrote prose under the line, and every command then
        // failed to read the item.
        if let Some(finding) = unmet.iter().find_map(|c| match c {
            Condition::NoResponse { finding, .. } => Some(finding),
            _ => None,
        }) {
            text += &format!("Answer each finding with one line under `### Responses` in the Report, and nothing else there, such as `{finding}: fixed in <commit>` or `{finding}: rebutted: <fact>`. Or, when it is right but not worth fixing now, or its fix leaves the item's scope, escalate it: atelier escalate --finding <finding> \"<question>\"\n");
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
) -> Refused {
    let shown = || entry.expect("a refusal that says what comes next has the entry");
    match refusal {
        Refusal::Role(Role::Reviewer) => {
            format!("You are the reviewer of {id}. Reviewers run `review` and `verdict`.\n").into()
        }
        Refusal::Role(Role::Implementer) => format!(
            "You are the implementer of {id}. Implementers run `start`, `submit`, `escalate` and `merge`.\n"
        ).into(),
        Refusal::Role(Role::Planner | Role::Lead | Role::Decider) => {
            unreachable!("no thread holds a planner's, a lead's or a decider's role on an item")
        }
        Refusal::From { act, status } => {
            let from: Vec<&str> = row(*act).from.iter().map(|s| status_name(*s)).collect();
            Refused::then(
                format!(
                    "{id} is {}, and `atelier {}` runs on an item that is {}. Nothing changed.\n",
                    status_name(*status),
                    row(*act).command,
                    from.join(", "),
                ),
                next(shown(), dropped, None),
            )
        }
        Refusal::NotHolder {
            role: Role::Reviewer,
            holder: Some(holder),
        } => Refused::then(
            format!("You (thread {caller}) do not hold this round's review of {id}; {holder} does. Nothing changed.\n"),
            format!("To take it over: atelier review {id}"),
        ),
        Refusal::NotHolder {
            role: Role::Reviewer,
            holder: None,
        } => Refused::then(
            format!("No review of {id} is open in this round. Nothing changed.\n"),
            format!("To open one: atelier review {id}"),
        ),
        Refusal::NotHolder { holder, .. } => {
            let holder = match holder {
                Some(thread) => format!("{thread} does"),
                None => "nobody does".into(),
            };
            Refused::then(
                format!("You (thread {caller}) do not hold {id}; {holder}. Nothing changed.\n"),
                format!("To take it over: atelier start {id}"),
            )
        }
        Refusal::Unmet(unmet) => format!(
            "`{}` refused; nothing changed.\n{}",
            command(act, id),
            needs(act, shown().status, id, unmet, None)
        )
        .into(),
        Refusal::Record(problem) => format!("{problem}\n").into(),
    }
}

pub fn no_item(id: &str, base: &str) -> Refused {
    Refused::then(
        format!("No item {id} is on the base branch {base} or on a branch named {id}. Nothing changed.\n"),
        "`atelier status` lists the items you can start.".into(),
    )
}

/// `word` as one shell word: as it is when it holds nothing a shell reads,
/// else in single quotes.
pub fn quoted(word: &str) -> String {
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
/// when the caller is not at its root (see [`crate::Repo::git_at`]).
pub fn commit_line(at: Option<&Path>, paths: &[String], message: &str) -> String {
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

pub fn not_committed() -> String {
    "The commit failed, so the record is as it was. Fix what Git says below, then run the command again".into()
}

/// What a planning command that commits what it writes prints once it has
/// committed `paths` as `commit`, with `message`.
pub fn committed(said: &str, paths: &[String], commit: &str, message: &str) -> String {
    let mut text = format!("{said}Committed {}, \"{message}\":\n", short(commit));
    for path in paths {
        text += &format!("  {path}\n");
    }
    text
}

/// Where to run a command that runs on the base branch `base`, which
/// `checkout`, if any, has checked out.
pub fn on_base(base: &str, checkout: Option<&Path>) -> String {
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
) -> Refused {
    Refused::then(
        format!(
            "`atelier {command}` writes planning, which happens on the base branch {base}, and this checkout is on {}. Nothing changed.\n",
            place(branch),
        ),
        on_base(base, checkout),
    )
}

/// How a finding was settled, after "it was".
pub fn settled(settlement: &Settlement) -> String {
    match settlement {
        Settlement::Taken { item, by } => format!("taken into {item} by {by}"),
        Settlement::Dropped { by, reason } => format!("dropped by {by}: {reason}"),
    }
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

/// The problems a planning command found in what it would write, with
/// paths shown from the checkout's root.
pub fn invalid(command: &str, root: &Path, problems: &[Problem]) -> Refused {
    let mut text = format!("`atelier {command}` refused; nothing changed.\n");
    for problem in problems {
        text += &format!("  {}\n", from_root(root, problem));
    }
    let next = "Correct what each names, in the command or in the record, then run it again.";
    Refused::then(text, next.into())
}

/// `problem` with its file's path from the checkout's root.
pub fn from_root(root: &Path, problem: &Problem) -> Problem {
    let file = problem.file.strip_prefix(root).unwrap_or(&problem.file);
    Problem {
        file: file.to_path_buf(),
        ..problem.clone()
    }
}

pub fn unknown(id: &str) -> String {
    format!("  {id}: no item or mission has this ID in this checkout or on a branch\n")
}

pub fn ready_refused(unknown_ids: &[&str], refused: &[(&str, Status)]) -> Refused {
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
    Refused::then(
        text,
        "Run it again with only the drafts to make ready.".into(),
    )
}

/// `ready` given no ID, which only a program calling it can do.
pub fn ready_nothing() -> Refused {
    Refused::then(
        "`atelier ready` names no item or mission to make ready; nothing changed.\n".into(),
        "Then: atelier ready <id>... --by <name>".into(),
    )
}

pub fn readied(ids: &[&str], by: &str) -> String {
    format!("Made ready, as {by} said: {}.\n", ids.join(", "))
}

pub fn place(branch: Option<&str>) -> String {
    branch.map_or("a detached HEAD".to_string(), |b| format!("branch {b}"))
}

/// `command` on item `id` refused in a checkout on `branch`; `checkout` has
/// the item's branch, if one does.
pub fn elsewhere(
    command: &str,
    id: &str,
    branch: Option<&str>,
    checkout: Option<&Path>,
) -> Refused {
    let there = match checkout {
        Some(path) => format!(
            "Run it in {}, which has branch {id} checked out.",
            path.display()
        ),
        None => {
            format!("Run it there, or `git switch {id}` here if no other checkout has that branch.")
        }
    };
    Refused::then(
        format!(
            "`atelier {command} {id}` runs in the checkout on branch {id}, where its implementer works, and this checkout is on {}. Nothing changed.\n",
            place(branch)
        ),
        there,
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

/// `decide` given `settled` on its question `asked`, whose finding there is
/// none of, or is not open.
pub fn not_settled(id: &str, asked: &Decision, settled: Outcome) -> Refused {
    let flag = match settled {
        Outcome::Defer => "--defer",
        Outcome::NoAction => "--no-action",
    };
    let why = match &asked.finding {
        None => "is about no finding".to_string(),
        Some(finding) => format!("is about finding {finding}, which is no longer open"),
    };
    Refused::then(
        format!(
            "`atelier decide {id} {flag}` refused; nothing changed.\nIts oldest question, which it answers, {why}, so there is nothing to settle:\n  {}\n",
            asked.question,
        ),
        format!("Then: {}", command(Act::Decide, id)),
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
) -> Refused {
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
        return match handoff {
            Some(handoff) => Refused::then(text, handoff.to_string()),
            None => text.into(),
        };
    }
    match unmet
        .iter()
        .any(|c| matches!(c, Condition::ReviewedFinding(_)))
    {
        true => Refused::then(text, "Say so to whoever started you, and stop.".into()),
        false => text.into(),
    }
}

pub fn unreadable(path: &str) -> String {
    format!("Written so, `## Decisions` in {path} would not read back as asked, so nothing was written. Look there for a code fence that is not closed")
}

pub fn short(commit: &str) -> &str {
    commit.get(..7).unwrap_or(commit)
}

/// The first five of `names`, and how many more.
pub fn some(names: &[String]) -> String {
    let mut text = names.iter().take(5).cloned().collect::<Vec<_>>().join(", ");
    if names.len() > 5 {
        text += &format!(" and {} more", names.len() - 5);
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_handoff_says_to_start_the_thread_unasked_and_holds_no_backtick() {
        let tasks = ["Run atelier review demo-ab12 and follow what it prints.".to_string()];
        let line = handoff("A reviewer", &tasks, "Then wait.");
        let first = line.lines().next().unwrap();
        // Codex waits for its user's word to start a subagent unless told
        // the start is its own to make.
        assert!(
            first.contains("start one yourself now, without waiting to be asked"),
            "{first}"
        );
        // True under either harness.
        assert!(first.contains("Under Codex a subagent you start is one"));
        assert!(first.contains("under Claude Code a subagent runs as your thread"));
        assert!(!first.contains('`'), "{first}");
        assert!(!line.lines().nth(1).unwrap().contains('`'));
    }
}
