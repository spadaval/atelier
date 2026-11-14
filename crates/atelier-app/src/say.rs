//! What the acts that live here say: their results and refusals, and the
//! wording of a refusal of any act and of what it needs, which `status`
//! and the commands of `atelier-cli` print too. Moved from `atelier-cli`'s
//! `output.rs`, which keeps the rest.

use crate::acts::{row, Act, Condition, Refusal, Role};
use crate::skip::Refused;
use crate::Entry;
use atelier_core::{Decision, Outcome, Settlement, Status, Thread, Verdict};
use atelier_records::Problem;
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
        Act::Ready => format!("atelier ready {id} --by <name>"),
        Act::StartNew => "atelier start --new \"<title>\"".into(),
        Act::StartMission => "atelier start --mission \"<title>\"".into(),
        Act::StartBacklog => "atelier start --backlog <slug>".into(),
        Act::MergeTrivial => "atelier merge --trivial \"<why>\"".into(),
        Act::Drop => format!("atelier drop {id} --reason \"<why>\""),
        Act::Close => format!("atelier close {id} \"<result>\""),
        Act::CloseMission => format!("atelier close {id}"),
        Act::Decide => format!("atelier decide {id} \"<answer>\""),
        _ => format!("atelier {}", row(act).command),
    }
}

pub fn goal(act: Act, status: Status) -> &'static str {
    match act {
        Act::Ready => "make it ready",
        Act::Start | Act::StartNew | Act::StartMission | Act::StartBacklog => "start it",
        Act::SubmitBacklog | Act::SubmitMission => "submit it",
        Act::MergeBacklog | Act::MergeMission => "merge it",
        Act::TakeOver => "take it over",
        Act::Submit if status == Status::ChangesRequested => "return it to review",
        Act::Submit => "submit it for review",
        Act::Review => "review it",
        Act::VerdictChanges | Act::VerdictApprove => "record your verdict",
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
        Condition::MissionNotOpen { id: mission, status: Some(Status::Draft) } => format!(
            "Mission {mission} is agreed (its record on the base is an old `draft`, which is proposed: its plan merges once two plan reviews approve it)"
        ),
        Condition::MissionNotOpen { id: mission, status: Some(Status::Dropped) } => format!(
            "Mission {mission} is not dropped (it was stopped, so none of its items starts or merges)"
        ),
        Condition::MissionNotOpen { id: mission, status: Some(status) } => {
            format!("Mission {mission} is not dropped (it is {})", status_name(*status))
        }
        Condition::MissionNotOpen { id: mission, status: None } => format!(
            "Mission {mission} is agreed (no mission {mission} is in the base's last commit: it is on the base once its plan merges)"
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
            "Finding {finding} (fix now): {title} (a reviewer kept its rebuttal, and a rebuttal gets one try: fix it)"
        ),
        Condition::FormerImplementer => format!("You have never implemented {id}"),
        Condition::ReviewedFinding(finding) => format!("You have never reviewed finding {finding}"),
        Condition::NoQuestion => "A question waits under `## Decisions` (none does)".into(),
        Condition::Decided(finding) => format!(
            "Finding {finding} has no `Decided:` line and carries no finding a decision settled (nothing decides a finding now: close a carried one with `Now: withdrawn`, or keep it at defer or no action and say why)"
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
/// What an entry needs and the command that follows, or what to do when no
/// command of the caller's follows. `dropped` is why the item was dropped,
/// from its record.
pub fn next(entry: &Entry, dropped: Option<&str>) -> String {
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
                Some(why) => format!("It was dropped: {why}\nNothing is left to do on it.\n"),
                None => "It was dropped; nothing is left to do on it.\n".into(),
            },
            _ => "Nothing is left to do on it.\n".into(),
        };
    };
    needs(act, entry.status, id, &entry.unmet)
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
/// starting it is the caller's own job and part of its task, so the caller
/// starts it unasked: told less, a Codex thread waited for its user's word,
/// and a Claude Code thread started as T3's child, told to start no agent
/// unless its task said so, stopped for its lead. A task has no backticks:
/// pasted into a double-quoted shell argument, they would run the command
/// as the caller.
pub fn handoff(who: &str, tasks: &[String], then: &str) -> String {
    let (start, give) = match tasks.len() {
        1 => ("one", "this task"),
        _ => ("one for each task", "its task"),
    };
    let mut text = format!(
        "{who} must be a thread other than yours, and starting it is your job: it is part of the task you were given, even as another thread's child or subagent, and nobody else will start it. So start {start} yourself now, without waiting to be asked. Under Codex a subagent you start is one; under Claude Code a subagent runs as your thread and is refused, so use your harness's tool for a new agent thread. Give it fresh context, not a fork of yours, and give it {give}:\n"
    );
    for task in tasks {
        text += &format!("  {task}\n");
    }
    text + then + "\n"
}

/// What a thread that holds nothing on item `id` is told on its branch:
/// `offer`, what it is told otherwise (to take it over, or, for a reviewer
/// whose review is done, to stop), except that the lead of its mission,
/// `leads`, is not offered the item.
pub fn visiting(id: &str, leads: Option<&str>, offer: &str) -> String {
    match leads {
        Some(mission) => format!("You lead mission {mission}, which {id} is in: its implementer builds it, so you do not take it over.\n"),
        None => offer.to_string(),
    }
}

/// The conditions of `act` that do not hold, then its command.
pub fn needs(act: Act, status: Status, id: &str, unmet: &[Condition]) -> String {
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
            text += &format!("Answer each finding with one line under `### Responses` in the Report, and nothing else there, such as `{finding}: fixed in <commit>` or `{finding}: rebutted: <fact>`. A rebuttal goes to the next round's reviewer, who keeps the finding or withdraws it.\n");
        }
        if unmet
            .iter()
            .any(|c| matches!(c, Condition::OpenFinding { .. }))
        {
            text += "A finding that stands needs `atelier verdict changes`. A carried one closes when your file says `Now: fixed` or `Now: withdrawn` for it, or keeps it at `Recommendation: defer` or `no action` with a line that says why.\n";
        }
        if unmet.iter().any(|c| matches!(c, Condition::Uncommitted(_))) {
            text += "Commit them, or set them aside with `git stash`.\n";
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
            "You are the implementer of {id}. Implementers run `start`, `submit` and `merge`.\n"
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
                next(shown(), dropped),
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
            needs(act, shown().status, id, unmet)
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
    guidance: &Guidance,
    at: Option<&Path>,
    paths: &[String],
    message: &str,
) -> String {
    let mut text = format!("{said}Wrote, and did not commit:\n");
    for path in paths {
        text += &format!("  {path}\n");
    }
    text += &match guidance {
        Guidance::Branch => "It counts once it is committed and its branch is merged: commit it, then run `atelier submit` and `atelier merge` on that branch.\n".into(),
        Guidance::Settle(id) => format!(
            "Once committed, it reaches the base when {id} merges. It is outside {id}'s directory, so a review of {id} must see it: an approval from before it is stale, and `atelier merge` will say to run `atelier submit`.\n"
        ),
        Guidance::Own => String::new(),
    };
    text + &format!("Then: {}\n", commit_line(at, paths, message))
}

/// What a record write says of how its change reaches the base.
pub enum Guidance {
    /// By the merge of the backlog or mission branch it is on.
    Branch,
    /// In the merge of item `0`, whose branch it is on, outside its
    /// directory.
    Settle(String),
    /// It is the item's own, on its branch: what the command said already
    /// says what follows.
    Own,
}

pub fn not_committed() -> String {
    "The commit failed, so the record is as it was. Fix what Git says below, then run the command again".into()
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

pub fn on_base_checkout(command: &str, base: &str) -> Refused {
    Refused::then(
        format!(
            "`atelier {command}` refused; nothing written.\nThis checkout has the base branch {base} out, and nothing is written there: only a merge writes {base}, and the pre-commit hook refuses a record change committed on it.\n"
        ),
        "Make the branch the change belongs on, and run this there: `atelier start --backlog <slug>` for a record change tied to no item or mission, `atelier start --mission \"<title>\"` for a new mission, or `atelier start <id>` for an item's own work. Run in this checkout, `start` makes the branch without switching it, and prints the `git worktree add` that gives the branch a checkout of its own.".to_string(),
    )
}

pub fn rule_on_base(base: &str) -> String {
    format!("Nothing is written in the checkout of the base branch {base}")
}

/// A record change that belongs on a backlog branch, run on the branch of
/// item `id`.
pub fn belongs_on_backlog(command: &str, id: &str, branch: &str) -> Refused {
    Refused::then(
        format!(
            "`atelier {command}` refused; nothing written.\nThis checkout is on {branch}, the branch of item {id}, and this change is not {id}'s: it would reach the base in {id}'s merge, and make a review of it stale.\n"
        ),
        "Run it on a backlog branch, made with `atelier start --backlog <slug>`, or on a mission's branch for that mission's plan.".to_string(),
    )
}

pub fn rule_belongs_on_backlog(command: &str, branch: &str) -> String {
    format!("`atelier {command}` writes on a backlog or mission branch, not on the item branch {branch}")
}

/// A drop of started item `id`, run anywhere but a checkout of its branch
/// `branch`, which `checkout`, if any, has out.
pub fn drop_off_branch(
    id: &str,
    branch: &str,
    here: Option<&str>,
    checkout: Option<&Path>,
) -> Refused {
    let there = match checkout {
        Some(path) => format!(
            "Run it in {}, which has {branch} checked out.",
            path.display()
        ),
        None => format!(
            "Run it in a checkout of {branch}, as after `git worktree add ../{id} {branch}`."
        ),
    };
    Refused::then(
        format!(
            "`atelier drop {id}` refused; nothing written.\n{id} is started, and a started item's record changes only on its branch {branch}; this checkout is on {}.\n",
            place(here)
        ),
        there,
    )
}

pub fn rule_drop_off_branch(id: &str, branch: &str) -> String {
    format!("A started item {id} is dropped in a checkout of its branch {branch}")
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

pub fn ready_refused(
    unknown_ids: &[&str],
    refused: &[(&str, Status)],
    missions: &[&str],
) -> Refused {
    let mut text = "`atelier ready` refused; nothing changed.\n".to_string();
    for id in unknown_ids {
        text += &unknown(id);
    }
    for id in missions {
        text += &format!(
            "  {id}: it is a mission, and a mission needs no go-ahead: its plan reaches the base once two plan reviews approve it, and its items are made ready one by one\n"
        );
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
        "`atelier ready` names no item to make ready; nothing changed.\n".into(),
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

/// A mission's status, written in the act table as an item's, by the name
/// its record stores.
pub fn mission_status_name(status: Status) -> &'static str {
    atelier_core::MissionStatus::of(status).map_or(status_name(status), |s| s.name())
}

/// A mission's act refused for `refusal`.
pub fn mission_refused(act: Act, id: &str, refusal: &Refusal) -> String {
    match refusal {
        Refusal::From { status, .. } => {
            let from: Vec<&str> = row(act)
                .from
                .iter()
                .filter(|s| atelier_core::MissionStatus::of(**s).is_some())
                .map(|s| mission_status_name(*s))
                .collect();
            let then = match status {
                Status::Draft if act == Act::MergeMission => "Its record is an old `draft`: run `atelier submit` here, which writes it open, and start its two plan reviews.\n".to_string(),
                Status::Draft => "Its record is an old `draft`, which is proposed: its plan reaches the base once two plan reviews approve it and its branch merges.\n".to_string(),
                Status::Dropped if matches!(act, Act::SubmitMission | Act::MergeMission) => "A plan dropped on its branch is not merged: push the branch and close its pull request, if it has one.\n".to_string(),
                _ => "Nothing is left to do on it.\n".into(),
            };
            format!(
                "Mission {id} is {}, and `atelier {}` runs on a mission that is {}. Nothing changed.\n{then}",
                mission_status_name(*status),
                row(act).command,
                from.join(" or ")
            )
        }
        Refusal::NotHolder { holder, .. } => {
            let by = holder
                .as_ref()
                .map_or(String::new(), |t| format!(", last {t}"));
            format!(
                "`atelier {}` on mission {id}'s branch is a planner's, and you have not planned it{by}. Nothing changed.\nTo take the plan over, run `atelier submit` here, which makes you one of its planners.\n",
                row(act).command
            )
        }
        Refusal::Unmet(unmet) => format!(
            "`{}` refused; nothing changed.\n{}",
            command(act, id),
            needs(act, Status::Ready, id, unmet)
        ),
        Refusal::Role(_) | Refusal::Record(_) => {
            unreachable!("a mission's act is refused only for its status, planner or conditions")
        }
    }
}

pub fn verdict_name(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Approve => "approved",
        Verdict::Changes => "changes requested",
    }
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

/// The Desk's `decide` refused for who the caller is, or for no question
/// waiting.
pub fn not_decider(id: &str, caller: &Thread, unmet: &[Condition]) -> Refused {
    let mut text = format!("`atelier decide {id}` refused; nothing changed.\n");
    for condition in unmet {
        text += &match condition {
            Condition::FormerImplementer => format!("You (thread {caller}) implemented {id}, and none of its implementers, past or present, decides its questions.\n"),
            Condition::ReviewedFinding(finding) => format!("You (thread {caller}) reviewed finding {finding}, which its question is about, and none of its reviewers decides it.\n"),
            Condition::NoQuestion => format!("{id} has no unanswered question under `## Decisions`.\n"),
            other => unreachable!("decide has no condition {other:?}"),
        };
    }
    text.into()
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
        // A thread waits for its user's or its lead's word to start
        // another unless told the start is its own job, in its task.
        for said in [
            "starting it is your job: it is part of the task you were given",
            "even as another thread's child or subagent, and nobody else will start it",
            "start one yourself now, without waiting to be asked",
        ] {
            assert!(first.contains(said), "{first}");
        }
        // True under either harness.
        assert!(first.contains("Under Codex a subagent you start is one"));
        assert!(first.contains("under Claude Code a subagent runs as your thread"));
        assert!(!first.contains('`'), "{first}");
        assert!(!line.lines().nth(1).unwrap().contains('`'));
    }
}
