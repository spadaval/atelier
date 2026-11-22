//! Everything `status` and the commands print. The wording lives here, in
//! one place, so it is cheap to change, but for what the actions that live in
//! `atelier-core` say and the wording of a refusal, which live in
//! `atelier_core::say` and are used from here.

use crate::merge::Moved;
use crate::setup::{Harness, Step};
use atelier_core::actions::{Action, Condition, Refusal};
use atelier_core::forge::{CiRun, ForgeError, MergeState, PullRequest};
use atelier_core::model::{Config, Priority, Response, Status, Thread, Verdict};
use atelier_core::plan::{Approvals, NotCounted, PlanDelta, APPROVALS, SPLITS};
use atelier_core::records::Problem;
use atelier_core::{Carried, Entry, Overview, Refused};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub(crate) use atelier_core::say::*;

pub(crate) fn priority_name(priority: Priority) -> &'static str {
    match priority {
        Priority::Critical => "critical",
        Priority::High => "high",
        Priority::Medium => "medium",
        Priority::Low => "low",
    }
}

pub(crate) fn holder(entry: &Entry) -> String {
    match &entry.holder {
        Some(thread) => format!("held by {thread}"),
        None => "held by nobody".into(),
    }
}

/// `status` on an item's branch. `config` is the base's, whose merge
/// checks and forge `merge` goes through, `dropped` why the item was
/// dropped, and, for a thread that holds nothing on it, `leads` names the
/// item's mission when it leads it.
pub fn item(
    caller: &Thread,
    entry: &Entry,
    config: &Config,
    dropped: Option<&str>,
    leads: Option<&str>,
    unsaved: &str,
    unreached: bool,
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
            atelier_core::repo::item_path(id)
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
    let mut next = next(entry, dropped);
    if unreached {
        next = next.replace(
            "Nothing is left to do on it.\n",
            &format!("The drop has not reached the base: once it is committed and pushed, drop the base's copy on a backlog branch with `atelier start --backlog drop-{id}`, then there `atelier drop {id} --reason \"<why>\"`, commit, `atelier submit` and `atelier merge`.\n"),
        );
    }
    // A thread holding nothing on it, a reviewer whose review is done
    // among them.
    let done = entry.next.is_none() && entry.reviewed.is_some() && !entry.holds;
    if entry.next == Some(Action::TakeOver) || done {
        next = visiting(id, leads, &next);
        if let (Some(_), Some(round)) = (leads, entry.capped) {
            next += &cap_for_lead(id, round);
        }
    }
    // Before `merge`, what it will run and wait for; a stale approval is
    // sent to `submit` instead.
    let stale = entry
        .unmet
        .iter()
        .any(|c| matches!(c, Condition::Stale { .. }));
    if entry.next == Some(Action::Merge) && !stale {
        let then = next.rfind("Then: ").expect("it ends with a Then line");
        next.insert_str(then, &gates(config));
    }
    let code = outside(id, &entry.uncommitted);
    if entry.next == Some(Action::Submit) && !code.is_empty() {
        // At the round cap the lead's ruling takes the Then line's place.
        let then = next.rfind("Then: ").unwrap_or(next.len());
        next.insert_str(then, &format!("Not committed: {}\n", some(&code)));
    }
    let undefined = match entry.undefined {
        Some(unwritten) => format!(
            "Write its {} in {} first{}: they say what the change must do, and the reviewer judges it against them.\n",
            match unwritten {
                (true, true) => "Goal and its Done when lines",
                (true, false) => "Goal",
                (false, _) => "Done when lines",
            },
            atelier_core::repo::item_path(id),
            match unwritten {
                (false, _) => ", each as `- [ ] <result>`",
                _ => "",
            }
        ),
        None => String::new(),
    };
    format!(
        "{who}Title: {}\nStatus: {}{}\n{undefined}{unsaved}{}{next}",
        entry.title,
        status_name(entry.status),
        since(entry),
        malformed(entry),
    )
}

/// What follows the status of an item its implementer holds: since when
/// it is approved, or which round it is in.
fn since(entry: &Entry) -> String {
    match (&entry.approved, entry.round) {
        (Some(time), _) => format!(" since {time}"),
        (None, Some(round)) if entry.status == Status::InReview => format!(", round {round}"),
        (None, Some(round)) => format!(" in round {round}"),
        (None, None) => String::new(),
    }
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
            "It pushes the branch to {} where the forge's copy is behind and opens its pull request where `atelier submit` did not",
            forge.remote
        );
        text += &match &forge.ci_workflow {
            Some(workflow) => format!(
                ", waits up to {} for {workflow} to pass on the branch's latest code, then has the forge merge the pull request.\n",
                span(crate::merge::WAIT)
            ),
            None => ", then has the forge merge the pull request.\n".into(),
        };
    }
    text
}

/// `status` away from an item's branch. `dropped` is each item dropped on
/// the base while its branch is there, with why, and `unreached` each item
/// dropped on its branch whose drop has not reached the base. `unsaved` is
/// what [`unsaved`] says of the checkout. `plans` is the plans the
/// checked-out records branch changes, with where their review stands.
/// `there` is, in the base's checkout, how the caller gets to the first item
/// it holds, since `start` does not switch that checkout.
#[allow(clippy::too_many_arguments)]
pub fn overview(
    caller: &Thread,
    branch: Option<&str>,
    base: &str,
    kind: Option<&str>,
    overview: &Overview,
    plans: &Result<Vec<Approvals>, Problem>,
    dropped: &[(String, Option<String>)],
    unreached: &[(String, Option<String>)],
    unsaved: &str,
    there: Option<&str>,
) -> String {
    let place = match (branch, kind) {
        (Some(branch), _) if branch == base => format!("the base branch {base}"),
        (Some(branch), Some(kind)) => format!("branch {branch}, {kind}"),
        (Some(branch), None) => format!("branch {branch}, which is no item's"),
        (None, _) => "a detached HEAD".into(),
    };
    let mut text = format!("You are thread {caller}, on {place}.\n");
    text += &match plans {
        Ok(plans) => plan_standing(plans),
        Err(problem) => {
            format!("Plan review not shown, since a committed record does not read: {problem}\n")
        }
    };
    if !overview.held.is_empty() {
        text += "You hold:\n";
        for entry in &overview.held {
            text += &format!(
                "  {} ({}{}): {}\n",
                entry.id,
                status_name(entry.status),
                since(entry),
                entry.title
            );
            for problem in &entry.malformed {
                text += &format!("  {}\n", condition(&entry.id, problem));
            }
            if let Some(round) = entry.capped {
                text += &format!("    Round {round} reached the review cap: its mission's lead rules before it is submitted again.\n");
            }
        }
    }
    if !overview.startable.is_empty() {
        text += "Can start now:\n";
        for entry in &overview.startable {
            text += &format!(
                "  {} ({}): {}\n",
                entry.id,
                priority_name(entry.priority),
                entry.title
            );
        }
    }
    text += &cannot_start(&overview.cannot_start, !overview.startable.is_empty());
    if !overview.started.is_empty() {
        text += "Held by other threads:\n";
        for entry in &overview.started {
            // Read from a remote-tracking branch: held in another clone.
            let on = (entry.on.as_ref()).map_or(String::new(), |on| format!(", on {on}"));
            text += &format!(
                "  {} ({}, {}{on}): {}\n",
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
    if !unreached.is_empty() {
        text += "Dropped on their branch, and the drop has not reached the base; carry it on a backlog branch with `atelier drop <id> --reason \"<why>\"` there, then `submit` and `merge`:\n";
        for (id, why) in unreached {
            text += &format!("  {id}: {}\n", why.as_deref().unwrap_or("no reason given"));
        }
    }
    text += &crate::view_output::overview(overview);
    text += unsaved;
    text += &crate::view_output::led(&overview.led);
    // A planner stopped for the human's ruling is pointed at no other work.
    if plans.iter().flatten().any(Approvals::stopped) {
        return text;
    }
    // A lead holding nothing hands its mission's items to other threads,
    // and is not pointed at one itself.
    if overview.held.is_empty() && !overview.led.is_empty() {
        return text;
    }
    match overview.held.first() {
        Some(held) => match there {
            Some(there) => text += there,
            None => text += &format!("Then: atelier start {} (switches to its branch)\n", held.id),
        },
        // Which to start is the task's to say: a thread given one task
        // takes a named item for its next step.
        None if !overview.startable.is_empty() => {
            text += "Start one only if that is your task: atelier start <id>\n";
        }
        None if overview.cannot_start.is_empty() => text += "Nothing is open to start.\n",
        None => text += "No open item can start now.\n",
    }
    text
}

/// How `status` in the base's checkout sends the holder of `id` to `path`,
/// the checkout that has its branch.
pub fn work_in(id: &str, path: &Path) -> String {
    format!(
        "{id}'s branch is checked out in {0}; work there.\nThen: cd {0}\n",
        quoted(&path.display().to_string())
    )
}

/// How `status` in the base's checkout sends the holder of `id` to a new
/// checkout of `branch` at `path`. `stale` says Git still lists a checkout
/// of it whose directory is gone, which it will not add another beside.
pub fn work_in_new(id: &str, branch: &str, path: &Path, stale: bool) -> String {
    let prune = if stale { "git worktree prune && " } else { "" };
    format!(
        "{id}'s branch needs a checkout of its own; work there.\nThen: {prune}git worktree add {} {branch}\n",
        quoted(&path.display().to_string())
    )
}

/// What `status` says of what the checkout `at` names, as for
/// [`commit_line`], has not saved: `records`, the record changes not
/// committed, and `unpushed`, the commits of `branch` not on `remote`,
/// oldest first, each its short ID and subject.
pub fn unsaved(
    at: Option<&Path>,
    records: &[String],
    branch: Option<&str>,
    remote: Option<&str>,
    unpushed: &[String],
) -> String {
    let mut text = String::new();
    if !records.is_empty() {
        text += "Record changes not committed, which count only once committed and merged:\n";
        for path in records {
            text += &format!("  {path}\n");
        }
        text += &format!(
            "Commit them with: {}\n",
            commit_line(at, records, "<what they change>", &[])
        );
    }
    if let (Some(branch), Some(remote), false) = (branch, remote, unpushed.is_empty()) {
        let git = git_command(at);
        let commits = match unpushed.len() {
            1 => "1 commit".to_string(),
            n => format!("{n} commits"),
        };
        text += &format!("{commits} of {branch} not pushed to {remote}:\n");
        for commit in unpushed {
            text += &format!("  {commit}\n");
        }
        text += &format!("Push with: {git} push {remote} {branch}\n");
    }
    text
}

/// One line counting the open items `start` would refuse, by each cause
/// among their unmet conditions, after `others` that can start, if any.
fn cannot_start(entries: &[Entry], others: bool) -> String {
    if entries.is_empty() {
        return String::new();
    }
    let with = |cause: fn(&Condition) -> bool| {
        let entries = entries.iter().filter(|e| e.unmet.iter().any(cause));
        entries.count()
    };
    let causes = [
        (
            with(|c| matches!(c, Condition::Blocked { .. })),
            "waits on a blocker that is not done",
            "wait on a blocker that is not done",
        ),
        (
            with(|c| matches!(c, Condition::MissionNotOpen { .. })),
            "is in a mission that is dropped or not agreed",
            "are in a mission that is dropped or not agreed",
        ),
        (
            with(|c| matches!(c, Condition::BranchExists)),
            "has a branch of its name already",
            "have a branch of their name already",
        ),
    ];
    let counts: Vec<String> = causes
        .into_iter()
        .filter(|(count, ..)| *count > 0)
        .map(|(count, one, many)| format!("{count} {}", if count == 1 { one } else { many }))
        .collect();
    let (item, them) = match entries.len() {
        1 => ("item", "it"),
        _ => ("items", "them"),
    };
    let other = if others { " other" } else { "" };
    format!(
        "{}{other} open {item} cannot start yet: {}. See {them} with: atelier list\n",
        entries.len(),
        counts.join(", ")
    )
}

pub fn lost_race(id: &str, holder: Option<&Thread>) -> String {
    let holder = holder.map_or("another thread".to_string(), |t| t.to_string());
    format!(
        "{holder} started {id} first and holds it. Nothing changed.\nTo take it over, run `atelier start {id}` again.\n"
    )
}

pub fn not_switched(branch: &str) -> String {
    format!(
        "Started: branch {branch} holds its start, but this checkout could not switch to it. Run `git switch {branch}` once the cause below is fixed"
    )
}

pub fn switch_failed(id: &str, branch: &str) -> String {
    format!(
        "Could not switch to branch {branch}, so {id} was not taken over. If another worktree has it checked out, run `atelier start {id}` there"
    )
}

pub fn checked_out_elsewhere(id: &str, branch: &str, path: &Path) -> String {
    format!(
        "`atelier start {id}` refused; nothing changed.\nBranch {branch} is checked out in {}, and a branch can be checked out in one worktree at a time. Run `atelier start {id}` there.\n",
        path.display()
    )
}

pub fn take_over_not_pushed(id: &str, branch: &str, remote: &str, error: &anyhow::Error) -> String {
    format!(
        "Took over {id} on branch {branch} here, but the push to {remote} failed, so other clones still see the last holder: {error:#}\nFix the cause, then push it with: git push {remote} {branch}\n"
    )
}

pub fn started(id: &str, branch: &str) -> String {
    format!("Started {id} on branch {branch}.\n")
}

pub fn started_new(id: &str, branch: &str) -> String {
    format!("Created {id} and started it on branch {branch}.\n")
}

pub fn started_mission(id: &str, branch: &str) -> String {
    format!(
        "Created proposed mission {id} on branch {branch}, with empty sections to write and you as its planner. It is agreed once that branch merges.\nWrite its plan there, starting with whose direction it started from in its Why; create its items there with `atelier new \"<title>\" --in {id}`, and commit with Git. Then run `atelier submit` here for its two plan reviews.\n"
    )
}

pub fn started_backlog(branch: &str) -> String {
    format!(
        "Made branch {branch} at the base's tip, for record changes tied to no item or mission.\nMake them with `new`, `set`, `drop`, `close` and `lead`, commit them with Git, then run `atelier submit` and `atelier merge` here.\n"
    )
}

/// What `start` prints in the base's own checkout, which it does not
/// switch: how to give `branch` a checkout of its own at `path`.
pub fn worktree(branch: &str, path: &Path) -> String {
    format!(
        "This checkout has the base out, so `start` left it as it is. Give {branch} a checkout of its own, and work there:\n  git worktree add {} {branch}\n",
        quoted(&path.display().to_string())
    )
}

pub fn bad_backlog_slug(slug: &str, message: &str) -> String {
    format!("`atelier start --backlog {slug}` refused; nothing changed.\n{message}.\n")
}

pub fn backlog_exists(branch: &str, remote: Option<&str>) -> String {
    let place = match remote {
        Some(remote) => format!("on {remote}"),
        None => "in this clone".to_string(),
    };
    format!(
        "`atelier start --backlog` refused; nothing changed.\nBranch {branch} exists {place}. Pick another slug, or work on that branch.\n"
    )
}

pub fn rule_backlog_exists(branch: &str) -> String {
    format!("No branch {branch} exists, here or on the remote")
}

pub fn unreachable(branch: &str, remote: &str, error: &anyhow::Error) -> String {
    format!(
        "`atelier start` refused; nothing changed.\n{remote} cannot be reached, and a branch is claimed by pushing it there, so {branch} was not made: {error:#}\nRun it again once {remote} answers.\n"
    )
}

pub fn claim_push_refused(branch: &str, remote: &str, error: &anyhow::Error) -> String {
    format!(
        "`atelier start` refused; nothing changed.\n{remote} refused the push of {branch}, which is the claim, so the branch was not kept: {error:#}\n"
    )
}

/// `start` lost the claim to `branch`, which `remote` already has, held
/// by `holder` as its record names it.
pub fn lost_claim(what: &str, branch: &str, remote: &str, holder: Option<&Thread>) -> String {
    let holder = holder.map_or("Another thread".to_string(), |t| t.to_string());
    format!(
        "`atelier start` refused; nothing changed here.\n{holder} holds {what}: {remote} already has branch {branch}, so this clone's claim lost, and the branch it made is removed. It fetched {remote}/{branch}.\nTo take it over, run `atelier start {what}` again.\n"
    )
}

/// `start <id>` could not fetch the base, which it judges by.
pub fn base_unread(id: &str, base: &str, remote: &str, error: &anyhow::Error) -> String {
    format!(
        "`atelier start` refused; nothing changed.\n{remote} cannot be reached to fetch {base}, which says whether {id} is already done or dropped: {error:#}\nRun it again once {remote} answers.\n"
    )
}

/// The claim was pushed and the base could not be fetched to confirm it.
pub fn claim_unconfirmed(
    id: &str,
    branch: &str,
    base: &str,
    remote: &str,
    error: &anyhow::Error,
) -> String {
    format!(
        "`atelier start {id}` pushed branch {branch}, then could not fetch {base} from {remote} to confirm {id} is still open: {error:#}\nRun `atelier start {id}` again once {remote} answers, before working on it.\n"
    )
}

/// `id` became `status` on the base while `start` claimed it.
pub fn start_finished(id: &str, branch: &str, base: &str, remote: &str, status: Status) -> String {
    format!(
        "`atelier start {id}` refused: {id} became {} on {base} while it started. The local branch is removed, and this checkout is as it was.\nWarning: the branch {branch} it pushed stays on {remote}, where it counts for nothing, since {base}'s record wins.\n",
        status_name(status)
    )
}

/// `id` became `status` on the base while it was taken over; `here` is
/// the branch this checkout was switched to for it.
pub fn take_over_finished(
    id: &str,
    branch: &str,
    base: &str,
    remote: &str,
    status: Status,
    here: Option<&str>,
) -> String {
    let here = here.map_or(String::new(), |here| {
        format!(" This checkout is on {here}.")
    });
    format!(
        "`atelier start {id}` refused: {id} became {} on {base} while it was taken over, so nothing is left to do on it.\nBranch {branch} is kept here and on {remote}, with the work already on it.{here}\n",
        status_name(status)
    )
}

/// The commits only a detached HEAD holds, at most the first ten.
fn loose(commits: &[String]) -> String {
    let mut text = String::new();
    for commit in commits.iter().take(10) {
        text += &format!("  {commit}\n");
    }
    if commits.len() > 10 {
        text += &format!("  and {} more\n", commits.len() - 10);
    }
    text
}

/// How to keep commits only a detached HEAD holds.
const KEEP_LOOSE: &str = "Keep them with `git branch <name> HEAD`, then `git cherry-pick` them onto the branch they belong on.";

/// What `status` says of commits only a detached HEAD holds.
pub fn unattached(commits: &[String]) -> String {
    if commits.is_empty() {
        return String::new();
    }
    format!(
        "Commits of this detached HEAD on no branch, remote-tracking branch or tag, which a switch away would lose:\n{}{KEEP_LOOSE}\n",
        loose(commits)
    )
}

/// `start` refused where it would switch away from such commits.
pub fn start_strands(commits: &[String]) -> Refused {
    Refused::then(
        format!(
            "`atelier start` refused; nothing changed.\nThis detached HEAD holds commits on no branch, remote-tracking branch or tag, which switching away would lose:\n{}",
            loose(commits)
        ),
        format!("{KEEP_LOOSE} Then run `atelier start` again."),
    )
}

pub fn rule_start_strands() -> String {
    "`atelier start` does not switch away from commits on no branch".into()
}

/// `status`'s line where merged branches or detached worktrees piled up.
pub fn piled_up(base: &str, merged: usize, detached: usize) -> String {
    format!(
        "Piled up in this clone: {merged} local branches merged into {base}, {detached} detached worktrees. See them with `git branch --merged {base}` and `git worktree list`.\n"
    )
}

pub fn created_item(id: &str) -> String {
    format!("Created open item {id}, with empty Goal, Done when and Scope sections to write.\n")
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
            Refusal::Role(_)
            | Refusal::FromMission { .. }
            | Refusal::NotHolder { .. }
            | Refusal::Record(_) => {
                unreachable!("settling a finding needs no role, and a record problem is an error")
            }
        };
    }
    text + "`atelier list --deferred` lists the deferred findings nobody has settled.\n"
}

pub fn finding_taken(name: &str, id: &str) -> String {
    format!(
        "Created open item {id} from finding {name}: its Goal quotes the finding, and its Done when and Scope are empty to write.\nMarked finding {name} taken into {id}.\n"
    )
}

pub fn finding_dropped(name: &str, reason: &str) -> String {
    format!("Dropped finding {name}: {reason}\n")
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
/// staged, and each status changed as no action changes it, with the status
/// `HEAD` has and the one staged.
pub fn commit_refused(
    unread: &[Problem],
    invalid: &[Problem],
    actions: &[(String, &str, &str)],
    base: &str,
    on_base: &[String],
) -> String {
    let mut text = "Atelier's pre-commit hook refused this commit; nothing was committed.\n"
        .to_string()
        + &problems(unread, invalid);
    if !on_base.is_empty() {
        text += &format!("Rule: only a merge writes the base branch {base}, so no record changes in a commit on it. This commit changes:\n");
        for path in on_base {
            text += &format!("  {path}\n");
        }
        text += "Make the branch the change belongs on and commit it there: `atelier start --backlog <slug>` for a record change tied to no item or mission, or `atelier start --mission \"<title>\"` for a new mission. `git stash` carries the change across.\n";
    }
    if !actions.is_empty() {
        text += "Rule: a status changes only as an action of the lifecycle changes it. These change it as no action does:\n";
    }
    for (path, before, now) in actions {
        text += &format!(
            "  {path}: status: {} to {}. Put back the status HEAD has; atelier status names the command for what comes next.\n",
            before, now
        );
    }
    text + "Fix each file named, stage it with git add, and commit again.\n"
}

/// The rule that a status changes only as an action changes it, as the
/// record at `path` breaks it.
pub(crate) fn rule_status(path: &str, from: &str, to: &str) -> String {
    format!(
        "A status changes only as an action of the lifecycle changes it ({path}: {from} to {to})"
    )
}

/// A commit refused because no config, `HEAD`'s or the checkout's, names
/// the base.
pub fn commit_no_base() -> String {
    let config = atelier_core::records::CONFIG;
    format!("Atelier's pre-commit hook refused this commit; nothing was committed.\nNeither HEAD's {config} nor this checkout's reads, so the hook cannot tell whether this commit is on the base, where no record changes. Fix {config}, stage it with git add, and commit again.\n")
}

pub(crate) fn rule_no_base() -> String {
    "The config names the base branch".into()
}

/// The rule that no commit on the base changes a record.
pub(crate) fn rule_record_on_base(base: &str) -> String {
    format!("Only a merge writes the base branch {base}, so no commit on it changes a record")
}

/// The rule that the base receives only merges, as `commit`, which is not
/// an Atelier merge, breaks it.
pub(crate) fn rule_merge(base: &str, commit: &str, subject: &str) -> String {
    format!(
        "{base} receives only merges made by atelier merge ({} {subject} is not one)",
        short(commit)
    )
}

/// The rule that with a forge configured no push moves the base, since
/// the forge makes every merge.
pub(crate) fn rule_base_moved(base: &str) -> String {
    format!("{base} is moved only by the forge's merge of a pull request, through atelier merge")
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

/// How to take refused commits off the base, where one command line can: the
/// base's tip is the commit pushed, and a branch `start` makes there holds
/// them all.
pub struct WayBack {
    /// The remote's commit, which the base goes back to; whole, since the
    /// command is run as printed.
    pub old: String,
    pub base_checkout: BaseCheckout,
}

/// Which checkout has the base out, for how it can be moved.
pub enum BaseCheckout {
    Here,
    At(PathBuf),
    /// None has, so Git lets `branch -f` move it.
    Nowhere,
}

/// What the pre-push hook says to do with the commits it refused. The
/// steps are one `&&` line, so a `start` that fails leaves the base, which
/// is then all that holds them, where it is.
fn off_base(base: &str, back: Option<&WayBack>) -> String {
    let Some(WayBack { old, base_checkout }) = back else {
        return format!(
            "Move them to a branch: `atelier start --new \"<title>\"` starts an item that holds code, `atelier start --backlog <slug>` a branch for record changes. Once that branch holds them, move {base} back to the remote's commit, and atelier merge brings the branch in.\n"
        );
    };
    let at = match base_checkout {
        BaseCheckout::At(path) => Some(path.as_path()),
        BaseCheckout::Here | BaseCheckout::Nowhere => None,
    };
    let git = git_command(at);
    let (cd, reset) = (
        at.map_or(String::new(), |path| {
            format!("cd {} && ", quoted(&path.display().to_string()))
        }),
        format!("{git} reset --keep {old}"),
    );
    let (back, refused) = match base_checkout {
        BaseCheckout::Nowhere => (format!("git branch -f {base} {old}"), String::new()),
        BaseCheckout::Here | BaseCheckout::At(_) => (
            reset.clone(),
            format!(
                "If uncommitted changes make the reset refuse, nothing is lost and the new branch already has the commits: save the changes with `{git} stash push -u`, finish with `{reset}`, then run `git stash apply` in the new branch's checkout.\n"
            ),
        ),
    };
    format!(
        "Move them to a new item's branch and put {base} back with:\n  {cd}atelier start --new \"<title>\" && {back}\n{refused}For record changes alone, use `atelier start --backlog <slug>` in place of `start --new`. atelier merge then brings the branch in.\n"
    )
}

/// The pre-push hook's refusal of a push of `base`: the new commits on its
/// first-parent line that are not Atelier merges, and the way `back`.
pub fn push_refused(base: &str, back: Option<&WayBack>, plain: &[(String, String, ())]) -> String {
    let mut text = format!(
        "Atelier's pre-push hook refused this push; nothing was pushed.\nRule: {base} receives only merges made by atelier merge. Each new commit on {base}'s first-parent line is one. These are not:\n"
    );
    text += &commits(plain, |()| "not an Atelier merge".to_string());
    text + &off_base(base, back)
}

/// The pre-push hook's refusal of a push that moves `base` where a forge is
/// configured, and the way `back`.
pub fn push_moves_base(base: &str, back: Option<&WayBack>) -> String {
    format!(
        "Atelier's pre-push hook refused this push; nothing was pushed.\nRule: {base} is moved only by the forge's merge of a pull request. With a forge configured, atelier merge has the forge merge each branch's pull request and never pushes {base}, so no push may move it.\n{}",
        off_base(base, back)
    )
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
        "`atelier new` refused; nothing changed.\nNo mission {id} is in this checkout. `atelier start --mission \"<title>\"` creates one, on its own branch.\n"
    )
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

pub fn not_planned(id: &str, status: &str) -> String {
    format!(
        "`atelier set {id}` refused; nothing changed.\n{id} is {status}, and its record no longer changes.\n"
    )
}

/// `ready`, which is no command since items are open from the start.
pub fn no_go_ahead() -> String {
    "`atelier ready` refused; nothing changed.\nItems need no go-ahead, and nor does a mission: `atelier start <id>` takes any open item that is on the base and whose blockers are done, and a mission is agreed when its plan merges.\n".into()
}

pub fn set_started(id: &str, status: Status) -> String {
    format!(
        "`atelier set {id}` refused; nothing changed.\n{id} is {}: it is started, and its record now changes only on branch {id}, by its implementer.\n",
        status_name(status)
    )
}

pub fn taken_over(id: &str, before: Option<&Thread>) -> String {
    match before {
        Some(thread) => format!("Took over {id}; {thread} held it.\n"),
        None => format!("Took over {id}, which nobody held.\n"),
    }
}

pub fn already_held(id: &str, branch: &str) -> String {
    format!("You already hold {id}, on its branch {branch}.\n")
}

/// The paths of `uncommitted` outside the item's directory.
fn outside(id: &str, uncommitted: &[String]) -> Vec<String> {
    let dir = format!(".atelier/items/{id}/");
    let outside = uncommitted.iter().filter(|path| !path.starts_with(&dir));
    outside.cloned().collect()
}

/// `checkout` is the one submitted from, and `forge` what `submit` did on
/// the forge, if one is configured.
pub fn submitted(id: &str, checkout: &Path, uncommitted: &[String], forge: &str) -> String {
    let code = outside(id, uncommitted);
    let mut text = format!("Submitted {id} for review.\n{forge}");
    if !code.is_empty() {
        text += &format!(
            "These changes are not committed, so the reviewer will not see them: {}. Commit them now, before the reviewer starts.\n",
            some(&code)
        );
    }
    text + &start_reviewer(id, Some(checkout))
}

/// `submit` on a records branch `branch` while `paths` are uncommitted.
pub fn records_uncommitted(branch: &str, paths: &[String], line: &str) -> String {
    let mut text = format!("`atelier submit` refused; nothing changed.\nThese record changes on {branch} are not committed, and a submit publishes only what is:\n");
    for path in paths {
        text += &format!("  {path}\n");
    }
    text + &format!("Commit them, or remove them, and submit again.\nThen: {line}\n")
}

pub fn records_submitted(branch: &str, forge: &str) -> String {
    let forge = match forge.is_empty() {
        true => "No forge is configured, so nothing was pushed.\n".to_string(),
        false => forge.to_string(),
    };
    format!(
        "Submitted {branch}, which changes records alone.\n{forge}It merges once `atelier check` passes on the merged tree, with no review.\nThen: atelier merge\n"
    )
}

/// What `check` says of the missions `ids`, whose Why is empty.
pub fn why_unwritten(ids: &[&str]) -> String {
    ids.iter()
        .map(|id| format!("Mission {id}'s Why is empty: it says why the mission is wanted and whose direction it started from: the conversation or the agreed proposal it came from.\n"))
        .collect()
}

/// `submit` on records branch `branch`, whose copy fetched from `remote`
/// at `tip` has commits this checkout lacks.
pub fn records_behind(branch: &str, remote: &str, tip: &str) -> String {
    format!(
        "`atelier submit` refused; nothing changed.\n{remote}/{branch} is at {}, which this checkout does not have, as after a reviewer pushed its review file there. A submit from here would leave it out.\nThen: git pull --no-rebase {remote} {branch}\n",
        short(tip)
    )
}

/// The title and body of the pull request of `branch`, which proposes or
/// changes the plans of `plans`.
pub fn plan_pull_request_text(
    branch: &str,
    plans: &[(String, u32)],
    base: &str,
) -> (String, String) {
    let ids: Vec<&str> = plans.iter().map(|(id, _)| id.as_str()).collect();
    (
        format!("{branch}: the plan of {}", ids.join(", ")),
        format!("The plan of mission {}, on {branch}. `atelier merge` merges it into {base} once two plan reviews, by threads that are not its planners, approve the plan its head holds.", ids.join(", ")),
    )
}

/// `submit` of `branch`, at commit `head`, which proposes or changes the
/// plans of `plans`, each with the round of plan review it opens.
pub fn plan_submitted(branch: &str, plans: &[(String, u32)], head: &str, forge: &str) -> String {
    let forge = match forge.is_empty() {
        true => "No forge is configured, so nothing was pushed.\n".to_string(),
        false => forge.to_string(),
    };
    let mut text = String::new();
    let mut tasks = Vec::new();
    for (id, round) in plans {
        text += &format!("Submitted the plan of mission {id} on {branch}, at commit {}, for round {round} of plan review. You are one of its planners, so you do not review it.\n", short(head));
        let task = format!("In a checkout of your own detached at commit {head}, run atelier review {id} and follow what it prints.");
        tasks.push(task.clone());
        tasks.push(task);
    }
    text += &forge;
    text += &format!("Give each reviewer its own checkout at that commit, as `git worktree add --detach <path> {head}` makes one, so that neither sees the other's review.\n");
    let then = format!("Then wait for both verdicts. Each reviewer pushes its review file to {branch}; once two approve, bring them here with `git pull`, and run `atelier merge`. If either asks for changes, change the plan, commit, and run `atelier submit` again for the next round, unless that makes {SPLITS} rounds in a row on this change where one approves and the other asks for changes: review stops there, and you bring the open findings to the human to rule.");
    text + &handoff("Each of two reviewers", &tasks, &then)
}

/// The title and body of a records branch's pull request.
pub fn records_pull_request_text(branch: &str, base: &str) -> (String, String) {
    (
        format!("{branch}: record changes"),
        format!("Atelier record changes on {branch}, under `.atelier/` alone. `atelier merge` merges them into {base} once `atelier check` passes on the merged tree."),
    )
}

/// `submit` or `merge` of item `id`, whose branch `branch` the forge's
/// `remote` has ahead of this checkout, at `tip`.
pub fn behind(id: &str, branch: &str, remote: &str, tip: &str) -> String {
    format!(
        "Refused; nothing changed.\n{remote}/{branch} is at {}, ahead of this checkout, so another clone changed {id} since: run `git pull --ff-only {remote} {branch}`, then `atelier status`.\n",
        short(tip)
    )
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

/// `submit` could not fetch the base, so judged by the last fetch.
pub fn submit_base_unread(id: &str, base: &str, remote: &str, error: &anyhow::Error) -> String {
    format!(
        "Warning: fetching {base} from {remote} failed, so whether {id} is done or dropped was judged by the last fetch: {error:#}\n"
    )
}

pub fn not_on_item(command: &str, branch: Option<&str>) -> String {
    format!(
        "`atelier {command}` runs on an item's branch, and this checkout is on {}. Nothing changed.\n`atelier status` lists what you hold and says how to get to its branch.\n",
        place(branch)
    )
}

fn review_path(id: &str, name: &str) -> String {
    format!(".atelier/items/{id}/reviews/{name}.md")
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

pub fn author(id: &str, caller: &Thread, checkout: Option<&Path>) -> String {
    format!(
        "You (thread {caller}) have implemented {id}, and an author does not review its own work. Nothing changed.\n{}What a reviewer is shown:\n\n",
        start_reviewer(id, checkout)
    )
}

fn response(response: Option<&Response>) -> String {
    match response {
        Some(Response::Fixed { commit, .. }) => format!("fixed in {commit}"),
        Some(Response::Rebutted { fact, .. }) => format!("rebutted: {fact}"),
        Some(Response::NotBuilt { reason, .. }) => format!("not built: {reason}"),
        None => "none in the Report".into(),
    }
}

/// Whether the author answered `carried` `not built`, so it waits on the
/// lead and not on the reviewer.
fn waits(carried: &Carried) -> bool {
    matches!(carried.response, Some(Response::NotBuilt { .. }))
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
        let response = match item {
            true => format!("Response: {}\n", response(carried.response.as_ref())),
            false => String::new(),
        };
        let comment = match waits(carried) {
            true => "It waits on the mission's lead, who rules on it: leave it as it is.",
            false => "Set Now: to fixed, withdrawn or kept. To keep it for later work instead, set Recommendation: to defer or no action and add a line that says why.",
        };
        text += &format!(
            "\n## Finding {number}: {title}\nRecommendation: fix now\nWas: {finding}\n{response}Now: kept\n<!-- {comment} -->\n"
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
    /// The round before's, from round 2 on.
    pub previous: Option<Previous>,
    /// The checkout, where a printed Git command must name it.
    pub at: Option<&'a Path>,
    pub carried: &'a [Carried],
    /// The caller's review file; `None` for an author.
    pub path: Option<&'a str>,
    /// What [`brief_pull_request`] says, where a forge is configured.
    pub pull_request: Option<String>,
}

/// The commit the round before reviewed, and the diff that can be given
/// since it.
pub enum Previous {
    /// The branch alone changed since: the whole diff.
    All(String),
    /// The base was merged in since: the paths the branch changes.
    Scoped(String, Vec<String>),
    /// It is not in the history of the commit under review: no diff.
    Lost(String),
}

/// The command that shows what changed from commit `from` to `to`, in
/// `paths` alone where any are given.
fn diff(at: Option<&Path>, from: &str, to: &str, paths: &[String]) -> String {
    let mut line = format!("  {} diff {from} {to}", git_command(at));
    if !paths.is_empty() {
        let paths: Vec<String> = paths.iter().map(|path| quoted(path)).collect();
        line += &format!(" -- {}", paths.join(" "));
    }
    line + "\n"
}

/// The brief's line on branch `id`'s pull request, as the forge `found` it.
pub fn brief_pull_request(id: &str, found: &Result<Option<PullRequest>, ForgeError>) -> String {
    match found {
        Ok(Some(pull)) => format!(
            "Pull request #{}, where CI's result on the branch can be read: {}\n",
            pull.number, pull.url
        ),
        Ok(None) => format!("The forge has no open pull request for branch {id}, so CI's result cannot be read there.\n"),
        Err(error) => format!(
            "The forge did not say which pull request branch {id} has: {}.\n",
            forge_reason(error).0
        ),
    }
}

pub fn brief(brief: &Brief) -> String {
    let Brief {
        id, round, commit, ..
    } = *brief;
    let mut text = String::new();
    if brief.path.is_some() {
        text += &format!(
            "You are the reviewer of {id} (thread {}), round {round}.\n",
            brief.caller
        );
        text += &match &brief.previous {
            None => format!("{}\n\n", whole_change(id)),
            Some(Previous::Lost(reviewed)) => format!(
                "The last round reviewed commit {}, which is not in this commit's history, so there is no diff since then. {} Judge each open finding below too.\n\n",
                short(reviewed),
                whole_change(id)
            ),
            Some(_) => later_round(round),
        };
    }
    text += &format!("Item {id}: {}\n", brief.title);
    for (heading, content) in &brief.sections {
        text += &format!("\n## {heading}\n{}", content.trim_end());
        text += "\n";
    }
    let whole = format!("  git diff {}...{commit}\n", brief.base);
    let since = match &brief.previous {
        None | Some(Previous::Lost(_)) => None,
        Some(Previous::All(from)) => Some(format!(":\n{}", diff(brief.at, from, commit, &[]))),
        Some(Previous::Scoped(_, paths)) if paths.is_empty() => Some(format!(
            ": nothing of this branch's, only {} merged in.\n",
            brief.base
        )),
        Some(Previous::Scoped(from, paths)) => Some(format!(
            ", in the paths this branch changes, since {} was merged in between:\n{}",
            brief.base,
            diff(brief.at, from, commit, paths)
        )),
    };
    text += &match since {
        None => format!("\nThe change, at commit {commit}:\n{whole}"),
        Some(since) => format!(
            "\nWhat changed since the last round's review, at commit {commit}{since}\
             The whole change:\n{whole}"
        ),
    };
    text += brief.pull_request.as_deref().unwrap_or_default();
    text += &carried_findings(brief.carried, brief.path.is_some(), true);
    let Some(path) = brief.path else {
        return text;
    };
    text + &review_file(
        path,
        brief.carried.len() + 1,
        "the change merges",
        "writes it and the item's new status and prints the Git command that commits the two",
    ) + "Run that command; your review is then done.\n"
}

/// How every round's reviewer is to look.
const LOOK: &str = "Read the code, run what proves it, and write down each problem you find. The author's reasoning is left out on purpose; judge what is there.";

/// What a reviewer of the whole change on item `id`'s branch is told.
fn whole_change(id: &str) -> String {
    format!("Review the change on branch {id} as someone who did not write it: check that it meets each Done when line, that it is correct, and that the Report's claims hold. {LOOK}")
}

/// What a reviewer in `round`, after the first, is told the round is for:
/// the earlier rounds read the whole change, so this one checks their
/// findings and what changed since, and a new finding in code that did
/// not change blocks only where the work would come out wrong without it.
fn later_round(round: u32) -> String {
    let earlier = match round - 1 {
        1 => "One round has".to_string(),
        n => format!("{n} rounds have"),
    };
    format!(
        "{earlier} reviewed the whole change before this one. This round checks the earlier findings and what changed since the last round's review: judge each open finding below, and check that the commits since then are correct, that the change still meets each Done when line, and that the Report's claims hold. {LOOK}\n\
         A new finding in code that did not change since the last round's review blocks only if the work would come out wrong without it: record that one as `fix now`, and any other as `defer`, for later work.\n\n"
    )
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
        text += &format!("Your review file carries each as a finding of its own{with}. Judge it, then set the finding's `Now:` line to `fixed`, to `withdrawn` (the rebuttal holds, or the finding was wrong), or leave it `kept` (it still stands, and stays a fix-now finding). A finding that is right but not for this item stays `kept` with its `Recommendation:` lowered to `defer` or `no action` and a line that says why.\n");
        if carried.iter().any(waits) {
            text += "A finding the author answered `not built` is real by the author's own word and was taken to the mission's lead, who rules on it. Leave it as your file has it and do not judge it again: it does not stop `atelier verdict approve`, and `atelier merge` holds it until the lead rules.\n";
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
        if item {
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
    /// The paths of the plan that differ in this checkout from `commit`.
    pub changed: Vec<String>,
    /// What a later round checks; `None` for a review of the whole plan.
    pub delta: Option<PlanDelta>,
    /// The checkout, where a printed Git command must name it.
    pub at: Option<&'a Path>,
    pub carried: &'a [Carried],
    pub path: &'a str,
    /// Where the review file goes once it is committed.
    pub landing: &'a Landing,
}

/// Where a plan review is pushed once committed: the branch of the plan,
/// on the forge's remote, or in this clone, where that branch may be
/// checked out; `None` for an amendment, whose backlog branch the review
/// cannot know.
pub struct Landing {
    pub branch: Option<String>,
    pub remote: Option<String>,
    /// The checkout of the branch, in a clone with no remote.
    pub checkout: Option<std::path::PathBuf>,
}

impl Landing {
    /// The commands that put the committed review on its branch, the push
    /// first and then the one for a push the other reviewer got ahead of.
    pub fn commands(&self) -> String {
        let Some(branch) = &self.branch else {
            return "Push it to the backlog branch the amendment of the plan is on: git push <remote> HEAD:refs/heads/<branch>\nIf the push is refused because the other reviewer pushed first, take theirs and push again: git pull --rebase <remote> <branch> && git push <remote> HEAD:refs/heads/<branch>\n".into();
        };
        let (push, behind) = match (&self.remote, &self.checkout) {
            (Some(remote), _) => (
                format!("git push {remote} HEAD:refs/heads/{branch}"),
                format!("git pull --rebase {remote} {branch} && git push {remote} HEAD:refs/heads/{branch}"),
            ),
            (None, Some(path)) => {
                let land = format!("git -C {} merge --ff-only \"$(git rev-parse HEAD)\"", quoted(&path.display().to_string()));
                (land.clone(), format!("git rebase {branch} && {land}"))
            }
            (None, None) => (
                format!("git push . HEAD:refs/heads/{branch}"),
                format!("git rebase {branch} && git push . HEAD:refs/heads/{branch}"),
            ),
        };
        format!("Push it to {branch}: {push}\nIf that is refused because the other reviewer got there first, put your commit after theirs and push again; your files differ, so this cannot conflict: {behind}\n")
    }
}

pub fn plan_brief(brief: &PlanBrief) -> String {
    let PlanBrief { id, round, .. } = *brief;
    let task = match &brief.delta {
        Some(delta) => later_plan_round(delta, brief.commit, brief.at),
        None => format!(
            "Review the plan as commit {} holds it, as someone who did not write it: the mission below, its other documents and its items. Read them, and write down each problem you find.",
            short(brief.commit)
        ),
    };
    let mut text = format!(
        "You are a reviewer of the plan of mission {id} (thread {}), round {round}.\n\
         {task} Another reviewer may review the same plan blind, in a checkout of its own.\n\n\
         Mission {id}: {}\n",
        brief.caller, brief.title
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
    if !brief.changed.is_empty() {
        text += &format!(
            "This checkout changes the plan from commit {}, and this review does not cover those changes: {}\n",
            short(brief.commit),
            brief.changed.join(", ")
        );
    }
    text += &carried_findings(brief.carried, true, false);
    text + &review_file(
        brief.path,
        brief.carried.len() + 1,
        "the mission starts",
        "writes it, changing no status, and prints the Git command that commits it alone",
    ) + "No command commits or pushes your review: you do, with Git. Run the command `verdict` prints, which commits the file alone, then push the commit to the branch of the plan:\n" + &brief.landing.commands() + "Tell whoever started you the commit. Your review is then done.\n"
}

/// What a plan's reviewer after the first round is told, with the diff
/// first: check `delta` at `commit` and the findings carried, and block on
/// what did not change only for a serious defect.
fn later_plan_round(delta: &PlanDelta, commit: &str, at: Option<&Path>) -> String {
    let PlanDelta { round, from, paths } = delta;
    let reviewed = format!("since round {round} reviewed it at commit {}", short(from));
    let what = match paths.is_empty() {
        true => format!(
            "The plan is unchanged {reviewed}, so there is no diff to read.\nThis round judges each open finding carried from earlier rounds, below."
        ),
        false => format!(
            "What changed in the plan {reviewed}:\n{}This round checks those changes and judges each open finding carried from earlier rounds, below.",
            diff(at, from, commit, paths)
        ),
    };
    format!(
        "{what} A new finding in a part of the plan that did not change blocks only if it is a serious defect the earlier rounds missed: record that one as `fix now`, and any other as `defer`.\nThe plan as commit {} holds it follows, for reference.",
        short(commit)
    )
}

pub fn plan_not_committed(id: &str, commit: &str, commit_line: &str) -> String {
    format!(
        "`atelier review {id}` reviews the plan as a commit holds it, and commit {} holds no mission {id}. Nothing changed.\nCommit the plan first: {commit_line}\nThen: atelier review {id}\n",
        short(commit)
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
    commit_line: &str,
    landing: &Landing,
) -> String {
    format!(
        "Recorded your verdict on the plan of mission {id}, round {round}: {}. Wrote, and did not commit:\n  {path}\nCommit it alone, and push it to the branch of the plan you reviewed; your review is done once it is there. Tell whoever started you the commit, and stop.\nThen: {commit_line}\n{}",
        verdict_name(verdict),
        landing.commands(),
    )
}

/// `review` of mission `id`'s plan by one of its planners.
pub fn planner_reviews(id: &str, caller: &Thread) -> String {
    format!(
        "`atelier review {id}` refused; nothing changed.\nYou ({caller}) are a planner of mission {id}, and a planner of a mission never reviews its plan: its two reviews are what let it merge, so each comes from a thread that did not write it.\nStart two other threads to review it, as `atelier submit` on the mission's branch says.\n"
    )
}

pub fn rule_planner_reviews(id: &str) -> String {
    format!("A planner of mission {id} does not review its plan")
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
        atelier_core::repo::mission_path(id)
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
pub fn dropped(id: &str, kind: &str, reason: &str) -> String {
    format!("Dropped {kind} {id}: {reason}\n")
}

/// `command` on mission `id`, which is proposed and not on the base `base`.
pub fn mission_proposed(command: &str, id: &str, base: &str) -> String {
    format!(
        "`atelier {command} {id}` refused; nothing written.\nMission {id} is proposed: {base}'s last commit does not have it open. `atelier {command}` runs on an agreed mission, whose plan has merged after two approving plan reviews.\n"
    )
}

pub fn rule_agreed(command: &str, id: &str) -> String {
    format!("`atelier {command}` runs on an agreed mission, and {id} is proposed")
}

/// `drop` of started item `id`, written on its branch `branch`, which is
/// pushed to `remote` where one is configured.
pub fn dropped_started(id: &str, branch: &str, reason: &str, remote: Option<&str>) -> String {
    let push = match remote {
        Some(remote) => format!("Once it is committed and pushed (`git push {remote} {branch}`), every clone that has fetched refuses {id}'s `submit`, `merge` and take-over, and the forge refuses a push of the branch from one that has not."),
        None => format!("Once it is committed, `submit`, `merge` and a take-over of {id} are refused."),
    };
    format!(
        "Dropped item {id} on its branch {branch}: {reason}\n{push}\nThe base's copy of {id} is dropped on a backlog branch after that: `atelier start --backlog drop-{id}`, then there `atelier drop {id} --reason \"{reason}\"`, commit, `atelier submit` and `atelier merge`. Until then `status` and `list` say the drop has not reached the base.\n"
    )
}

/// `drop` of started item `id` written on a backlog branch. `branch` is
/// the item's, whose pull request `merge` closes where there is a
/// `remote`.
pub fn dropped_on_backlog(id: &str, branch: &str, reason: &str, remote: Option<&str>) -> String {
    let closes = match remote {
        Some(_) => " That merge closes its pull request, if one is open.",
        None => "",
    };
    format!(
        "Dropped started item {id}: {reason}\nOnce this branch merges, {id}'s `submit`, `merge` and take-over are refused. Branch {branch} is left as it is.{closes}\n"
    )
}

/// The items `added`, each its ID, title and the ref that holds it, which
/// the branch of dropped `id` filed and the base does not hold.
pub fn dropped_adds(
    id: &str,
    branch: &str,
    base: &str,
    added: &[(String, String, String)],
) -> String {
    if added.is_empty() {
        return String::new();
    }
    let mut text = format!(
        "Branch {branch} also adds items that are not on {base}, and dropping {id} leaves them behind. For each still wanted, run its command on a backlog branch and commit it:\n"
    );
    for (other, title, on) in added {
        let dir = atelier_core::repo::item_dir(other);
        text += &format!("  {other}: {title}\n    git checkout {on} -- {dir}\n");
    }
    text
}

/// `drop` of an item the base holds done, whose branch may be gone.
pub fn drop_done(id: &str, base: &str) -> String {
    format!("`atelier drop {id}` refused; nothing written.\n{id} is done on {base}, so it cannot be dropped. Merge {base} into this branch to see it done.\n")
}

/// `drop` of an item whose branch the base, or the forge's copy of it,
/// has merged.
pub fn drop_merged(id: &str) -> String {
    format!("`atelier drop {id}` refused; nothing written.\n{id} is merged: its branch is in the history of the base, here or on the forge, so it is done and cannot be dropped. Fetch and pull the base to see it done.\n")
}

/// `drop` of an item dropped on its branch whose base copy says so too.
pub fn drop_reached_base(id: &str) -> String {
    format!("`atelier drop {id}` refused; nothing written.\n{id} is dropped already, on its branch and in this checkout's copy.\n")
}

/// `new --mission`, which `start --mission` replaced.
pub fn new_mission(title: &str) -> String {
    format!("`atelier new --mission` refused; nothing written.\nA mission is made on a branch of its own, named for it, with: atelier start --mission \"{title}\"\n")
}

pub fn closed(id: &str, kind: &str) -> String {
    format!("Closed {kind} {id}: it is done.\n")
}

pub fn verdict_recorded(
    id: &str,
    round: u32,
    verdict: Verdict,
    paths: &[String],
    implementer: Option<&Thread>,
    commit_line: &str,
) -> String {
    let to = match verdict {
        Verdict::Approve => "to merge",
        Verdict::Changes => "to answer your open findings",
    };
    let implementer = implementer.map_or("nobody yet".to_string(), |t| t.to_string());
    let mut text = format!(
        "Recorded your verdict on {id}, round {round}: {}. Wrote, and did not commit:\n",
        verdict_name(verdict)
    );
    for path in paths {
        text += &format!("  {path}\n");
    }
    text + &format!(
        "Commit the two together; once that commit is made, {id} is back with its implementer, {implementer}, {to}, and your review is done: say so to whoever started you, and stop.\nThen: {commit_line}\n"
    )
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
    let mut text = format!(
        "Merge {id}: {title}\n\nMerges branch {id} into {base}, where its record reads as done.\n"
    );
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

/// What `merge` says once each of the `count` merge checks in the base's
/// config passed on the merged tree.
pub fn checks_passed(base: &str, count: usize) -> String {
    match count {
        0 => format!("{base}'s config.toml names no merge check, so none ran.\n"),
        1 => {
            format!("Ran 1 merge check from {base}'s config.toml on the merged tree; it passed.\n")
        }
        _ => format!(
            "Ran {count} merge checks from {base}'s config.toml on the merged tree; each passed.\n"
        ),
    }
}

pub fn merged(id: &str, base: &str, commit: &str) -> String {
    format!(
        "Merged {id} into {base} with merge commit {}; it is done.\n",
        short(commit)
    )
}

/// The forge merged `id`'s pull request `pull` into `base`.
pub fn forge_merged(id: &str, base: &str, pull: &PullRequest) -> String {
    format!(
        "The forge merged pull request #{} into {base}; {id} is done.\n",
        pull.number
    )
}

/// The forge did not merge `id`'s pull request `pull`, for `error`.
pub fn forge_merge_failed(id: &str, base: &str, pull: &PullRequest, error: &ForgeError) -> String {
    let (reason, next) = forge_reason(error);
    format!(
        "`atelier merge` refused: the forge did not merge pull request #{} of {id} into {base}: {reason}. Nothing was merged.\n{next}, then run `atelier merge` again.\n",
        pull.number
    )
}

/// The forge will not merge `id`'s pull request `pull`, since its branch
/// conflicts with the forge's `base`.
pub fn forge_conflicts(id: &str, base: &str, remote: &str, pull: &PullRequest) -> String {
    format!(
        "`atelier merge` refused: pull request #{} does not merge cleanly into {remote}'s {base}, which has moved since this clone fetched it. Nothing was merged.\nBring it in on branch {id} with `git fetch {remote} {base}` and `git merge {remote}/{base}`, resolve the conflicts and commit, then run `atelier merge` again.\n",
        pull.number
    )
}

/// The forge will not merge `id`'s pull request `pull` until its branch is
/// up to date with `base`, as the base's protection requires.
pub fn forge_behind(id: &str, base: &str, remote: &str, pull: &PullRequest) -> String {
    format!(
        "The forge will not merge pull request #{} yet: {base}'s protection requires the branch to be up to date with {base}, and it is not (mergeStateStatus BEHIND). Nothing was merged.\nBring {base} in on branch {id} with `git fetch {remote} {base}` and `git merge {remote}/{base}`, commit, and run `atelier merge` again.\n",
        pull.number
    )
}

/// The forge will not merge pull request `pull` until a review or a check
/// its base's protection requires is given, as `state` says.
pub fn forge_blocked(pull: &PullRequest, state: &MergeState) -> String {
    let review = match state.review_decision.as_str() {
        "REVIEW_REQUIRED" => "a review the base's protection requires has not been given",
        "CHANGES_REQUESTED" => "a reviewer on the forge requested changes",
        _ => "a check the base's protection requires has not passed",
    };
    let decision = match state.review_decision.as_str() {
        "" => "none".to_string(),
        decision => decision.to_string(),
    };
    format!(
        "The forge will not merge pull request #{} yet: {review} (reviewDecision {decision}, mergeStateStatus {}). Nothing was merged.\nRun `atelier merge` again once it is given: {}\n",
        pull.number, state.merge_state_status, pull.url
    )
}

/// The fetch of `base` after the forge's merge failed.
pub fn not_fetched(remote: &str, base: &str, error: &anyhow::Error) -> String {
    format!(
        "Fetching {base} from {remote} afterwards failed: {error:#}. This clone does not show the merge until `git fetch {remote}` and a fast-forward of {base} bring it in.\n"
    )
}

/// The local `base` holds commits the forge's does not, so it was not
/// brought up to date with the merge.
pub fn base_diverged(remote: &str, base: &str) -> String {
    format!(
        "The local {base} has commits {remote}/{base} does not, so it was left as it was; no merge made them. Move it to {remote}/{base} once they are on a branch.\n"
    )
}

/// The checkout of `base` at `path` has uncommitted changes, so it was not
/// brought up to date with the merge.
pub fn base_left(remote: &str, base: &str, path: &Path) -> String {
    format!(
        "{base} is checked out at {}, where tracked files have uncommitted changes, so it was left as it was. Bring it up to date there with `git merge --ff-only {remote}/{base}`.\n",
        path.display()
    )
}

/// The fast-forward of the checkout of `base` failed.
pub fn base_not_moved(remote: &str, base: &str, error: &anyhow::Error) -> String {
    format!(
        "Bringing the checkout of {base} up to date failed: {error:#}. Do it there with `git merge --ff-only {remote}/{base}`.\n"
    )
}

/// `merge --trivial` where the item's record has uncommitted changes, which
/// would be committed with the reason.
pub fn trivial_uncommitted(path: &str) -> String {
    format!(
        "`atelier merge --trivial` refused: {path} has uncommitted changes, and the reason is committed in that file. Nothing changed.\nCommit them or undo them, then run it again.\n"
    )
}

/// The message of the commit that records why item `id` merges with no
/// review.
pub fn trivial_message(id: &str, why: &str) -> String {
    format!("Merge {id} as trivial: {why}\n")
}

pub fn records_merged(branch: &str, base: &str, commit: &str) -> String {
    format!(
        "Merged {branch} into {base} with merge commit {}.\n",
        short(commit)
    )
}

pub fn records_checked(base: &str) -> String {
    format!("The branch changes records alone, so it merges with no review once its records are valid: `atelier check` passed on the merged tree of {base}.\n")
}

/// The merge message of the records branch `branch`.
pub fn records_merge_message(branch: &str, base: &str) -> String {
    format!("Merge {branch}\n\nMerges record changes on {branch} into {base}; every record of the merged tree is valid.\n\nAtelier-Merge: {branch}\n")
}

/// `merge` of a records branch whose merged tree has invalid records.
pub fn records_invalid(branch: &str, base: &str, problems: &[String]) -> String {
    let mut text = format!("`atelier merge` refused; nothing changed.\nMerged into {base}, {branch} leaves records that are not valid:\n");
    for problem in problems {
        text += &format!("  {problem}\n");
    }
    text + "Fix them on the branch, as `atelier check` there says, commit, and merge again.\n"
}

/// `merge --trivial` on a records branch.
pub fn trivial_records(branch: &str) -> String {
    format!("`atelier merge --trivial` refused; nothing changed.\n{branch} changes records alone, and merges with no review already: run `atelier merge`.\n")
}

/// `merge` of a records branch that changes `code`, paths outside
/// `.atelier/` or the config.
pub fn records_code(branch: &str, base: &str, code: &[String]) -> String {
    let mut text = format!("`atelier merge` refused; nothing changed.\n{branch} merges with no review only while it changes records alone, under `.atelier/` and not its config.toml, and it changes these against {base}:\n");
    for path in code {
        text += &format!("  {path}\n");
    }
    text + "A change to code needs an independent review: make it on an item's branch (`atelier start <id>`, or `atelier start --new \"<title>\"`), and take it off this one.\n"
}

/// `merge` of a records branch that adds a mission or changes a plan.
/// `merge` of `branch`, which adds a mission or changes a plan, refused
/// for each mission in `unapproved`, with what the branch changes of it,
/// while fewer than two plan reviews approve the plan its head holds.
pub fn plan_unapproved(
    branch: &str,
    unapproved: &[(String, Approvals)],
    remote: Option<&str>,
) -> String {
    let mut text = format!("`atelier merge` refused; nothing changed.\nA new mission or a changed plan merges once {APPROVALS} plan reviews of its current round, by threads that are not its planners, approve the plan the branch's head holds, and {branch} has fewer:\n");
    for (what, approvals) in unapproved {
        let id = &approvals.id;
        text += &format!("  Mission {id}: {branch} {what}.\n");
        if approvals.reviews.is_empty() {
            text += "    Its plan has no review.\n";
        }
        for review in &approvals.reviews {
            let by = review
                .by
                .as_ref()
                .map_or("nobody".to_string(), |t| t.to_string());
            let why = match &review.not {
                None => "approves, and counts".to_string(),
                Some(NotCounted::NoVerdict) => "has no verdict yet".into(),
                Some(NotCounted::Changes) => "asks for changes".into(),
                Some(NotCounted::Planner) => "approves, and does not count: its reviewer is a planner of the mission".into(),
                Some(NotCounted::SameReviewer) => "approves, and does not count: its reviewer's other approval of the round counts already".into(),
                Some(NotCounted::Unknown) => format!("approves {}, which is not in this clone, so it does not count", short(&review.commit)),
                Some(NotCounted::Stale(changed)) => format!("approves {}, and does not count: the plan has changed since ({})", short(&review.commit), changed.join(", ")),
            };
            text += &format!(
                "    Round {}, review {} by {by}: {why}.\n",
                approvals.round, review.name
            );
        }
        text += &format!("    {} of {APPROVALS} count.\n", approvals.counted());
    }
    let fetch = match remote {
        Some(remote) => format!(" Reviewers push their files to {branch}; fetch them with `git pull {remote} {branch}` first if they are there."),
        None => String::new(),
    };
    let stopped = split_exits(unapproved.iter().map(|(_, approvals)| approvals));
    let next = match stopped.is_empty() {
        true => "Run `atelier submit` here to open the next round of plan reviews, and start two reviewers, each in a checkout of its own at the commit submitted.",
        false => stopped.trim_end(),
    };
    text + &format!("{next}{fetch}\n")
}

/// The human's ruling as the way out, for each of `plans` whose review
/// stops at the split rounds.
fn split_exits<'a>(plans: impl IntoIterator<Item = &'a Approvals>) -> String {
    (plans.into_iter())
        .filter(|plan| plan.stopped())
        .map(|plan| plan_split_exit(&plan.id, plan.splits))
        .collect()
}

/// `submit` refused, since review of some of `plans` stops for the human.
pub fn plan_split_refused(plans: &[Approvals]) -> String {
    format!(
        "`atelier submit` refused; nothing changed.\n{}",
        split_exits(plans)
    )
}

pub fn rule_plan_split(id: &str) -> String {
    format!("Plan review of a change to mission {id} stops after {SPLITS} split rounds in a row")
}

/// Where the review of each plan a records branch changes stands.
fn plan_standing(plans: &[Approvals]) -> String {
    let mut text = String::new();
    for plan in plans {
        let id = &plan.id;
        text += &match plan.reviewed {
            true => format!(
                "Plan review of mission {id}, round {}: {} of {APPROVALS} approvals count.\n",
                plan.round,
                plan.counted()
            ),
            false => format!("Plan review of mission {id}: no review of this change here yet.\n"),
        };
    }
    text + &split_exits(plans)
}

/// `merge` of item `id`'s branch, which adds or changes the plans of
/// `planned`, each with what it changes.
pub fn item_changes_plan(id: &str, planned: &[(String, String)]) -> String {
    let mut text = format!("`atelier merge` refused; nothing changed.\nAn item's reviewer judges its change against the plan, so an item's branch changes no mission's plan, and {id}'s does:\n");
    for (mission, what) in planned {
        text += &format!("  Mission {mission}: it {what}.\n");
    }
    text + "Take those changes off this branch. A new mission is planned with `atelier start --mission \"<title>\"`, and an amendment of an agreed mission's plan goes on a backlog branch, `atelier start --backlog <slug>`, where two plan reviews approve it.\n"
}

pub fn rule_item_changes_plan(id: &str) -> String {
    format!("{id}'s branch changes no mission's plan")
}

pub fn rule_plan_approved(id: &str) -> String {
    format!("{APPROVALS} plan reviews by threads that are not its planners approve the plan of mission {id} its head holds")
}

/// `merge` of a records branch that brings `started`, items in a started
/// status, to the base.
pub fn records_started(branch: &str, started: &[(String, Status)]) -> String {
    let mut text = format!("`atelier merge` refused; nothing changed.\nA started item's record changes on its own branch and reaches the base in its own merge, and {branch} brings these to the base started:\n");
    for (id, status) in started {
        text += &format!("  {id}: {}\n", status_name(*status));
    }
    text + "Take those changes off this branch.\n"
}

pub fn rule_records_started(branch: &str) -> String {
    format!("{branch} brings no item to the base in a started status")
}

pub fn records_dirty(branch: &str, paths: &[String]) -> String {
    let mut text = format!("`atelier merge` refused; nothing changed.\nThese changes on {branch} are not committed, and would be left behind:\n");
    for path in paths {
        text += &format!("  {path}\n");
    }
    text + "Commit them, or remove them, and merge again.\n"
}

pub fn rule_records_dirty(branch: &str) -> String {
    format!("Nothing on {branch} is uncommitted")
}

/// A problem of a record, on one line.
pub fn problem_line(problem: &Problem) -> String {
    problem.to_string()
}

pub fn already_merged(id: &str, base: &str) -> String {
    format!("{id} is already merged into {base}; nothing is left to do on it.\n")
}

/// The comment on the pull request of dropped `id` that `merge` closes.
pub fn dropped_comment(id: &str, base: &str, by: &str) -> String {
    format!("{id} is dropped on {base}, by the merge of {by}.")
}

pub fn dropped_closed(id: &str, pull: &PullRequest) -> String {
    format!("Closed pull request #{} of dropped {id}.\n", pull.number)
}

pub fn dropped_not_closed(id: &str, error: &ForgeError) -> String {
    let (reason, _) = forge_reason(error);
    format!(
        "Warning: could not close an open pull request of dropped {id}: {reason}. Close it on the forge.\n"
    )
}

/// The branch stays, since the base as fetched does not hold its merge.
pub fn branch_kept(branch: &str, base: &str, remote: Option<&str>) -> String {
    let everywhere = remote.map_or(String::new(), |remote| format!(" here and on {remote}"));
    format!(
        "Kept branch {branch}{everywhere}, since {base} as this clone has it does not hold the merge yet. Fetch, then run `atelier merge` again to delete it.\n"
    )
}

pub fn remote_deleted(branch: &str, remote: &str) -> String {
    format!("Deleted branch {branch} on {remote}.\n")
}

pub fn remote_already_deleted(branch: &str, remote: &str) -> String {
    format!("Warning: {remote} had already deleted branch {branch}.\n")
}

pub fn remote_not_deleted(branch: &str, remote: &str, error: &anyhow::Error) -> String {
    format!(
        "Warning: branch {branch} stays on {remote}, which refused its deletion at the merged commit, as after a later push to it: {error:#}\n"
    )
}

pub fn branch_deleted(branch: &str, head: &str) -> String {
    format!(
        "Deleted branch {branch} here; this checkout is on a detached HEAD at {}. For further changes, run `atelier start --backlog <slug>` or `atelier start <id>`.\n",
        short(head)
    )
}

pub fn branch_attached(branch: &str) -> String {
    format!(
        "Left branch {branch} checked out, since this checkout holds changes the merge does not. Take them to a branch made with `atelier start --backlog <slug>` or `atelier start <id>`.\n"
    )
}

pub fn cleanup_failed(branch: &str, error: &anyhow::Error) -> String {
    format!("Warning: the merge stands, and cleaning up branch {branch} failed: {error:#}\n")
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
