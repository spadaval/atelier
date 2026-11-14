//! Everything `status` and the commands print. The wording lives here, in
//! one place, so it is cheap to change, but for what the acts that live in
//! `atelier-app` say and the wording of a refusal, which live in
//! `atelier_app::say` and are used from here.

use crate::forge::{CiRun, ForgeError, MergeState, PullRequest};
use crate::merge::Moved;
use crate::setup::{Harness, Step};
use atelier_app::acts::{Act, Condition, Refusal};
use atelier_app::plan::{Approvals, NotCounted, APPROVALS};
use atelier_app::{Carried, Entry, Overview};
use atelier_core::{Config, Priority, Response, Status, Thread, Verdict};
use atelier_records::Problem;
use std::path::Path;
use std::time::Duration;

pub(crate) use atelier_app::say::*;

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
            atelier_app::repo::item_path(id)
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
    if entry.next == Some(Act::TakeOver) || done {
        next = visiting(id, leads, &next);
    }
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
        Some(unwritten) => format!(
            "Write its {} in {} first{}: they say what the change must do, and the reviewer judges it against them.\n",
            match unwritten {
                (true, true) => "Goal and its Done when lines",
                (true, false) => "Goal",
                (false, _) => "Done when lines",
            },
            atelier_app::repo::item_path(id),
            match unwritten {
                (false, _) => ", each as `- [ ] <result>`",
                _ => "",
            }
        ),
        None => String::new(),
    };
    format!(
        "{who}Title: {}\nStatus: {}{}\n{undefined}{unsaved}{next}",
        entry.title,
        status_name(entry.status),
        since(entry),
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
/// what [`unsaved`] says of the checkout.
#[allow(clippy::too_many_arguments)]
pub fn overview(
    caller: &Thread,
    branch: Option<&str>,
    base: &str,
    kind: Option<&str>,
    overview: &Overview,
    dropped: &[(String, Option<String>)],
    unreached: &[(String, Option<String>)],
    unsaved: &str,
) -> String {
    let place = match (branch, kind) {
        (Some(branch), _) if branch == base => format!("the base branch {base}"),
        (Some(branch), Some(kind)) => format!("branch {branch}, {kind}"),
        (Some(branch), None) => format!("branch {branch}, which is no item's"),
        (None, _) => "a detached HEAD".into(),
    };
    let mut text = format!("You are thread {caller}, on {place}.\n");
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
        }
    }
    text += &cannot_start(&overview.cannot_start, !overview.ready.is_empty());
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
        (None, None) if overview.cannot_start.is_empty() => {
            text += "Nothing is ready to start.\n";
        }
        (None, None) => text += "No ready item can start now.\n",
    }
    text
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
            commit_line(at, records, "<what they change>")
        );
    }
    if let (Some(branch), Some(remote), false) = (branch, remote, unpushed.is_empty()) {
        let git = match at {
            Some(at) => format!("git -C {}", quoted(&at.display().to_string())),
            None => "git".into(),
        };
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

/// One line counting the ready items `start` would refuse, by each cause
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
        "{}{other} ready {item} cannot start yet: {}. See {them} with: atelier list\n",
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
        "Made branch {branch} at the base's tip, for record changes tied to no item or mission.\nMake them with `new`, `set`, `ready`, `drop`, `close` and `lead`, commit them with Git, then run `atelier submit` and `atelier merge` here.\n"
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

pub fn created_item(id: &str) -> String {
    format!(
        "Created draft item {id}, with empty Goal, Done when and Scope sections to write.\nA planner makes it ready with: atelier ready {id} --by <name>\n"
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

pub fn finding_taken(name: &str, id: &str) -> String {
    format!(
        "Created draft item {id} from finding {name}: its Goal quotes the finding, and its Done when and Scope are empty to write.\nMarked finding {name} taken into {id}.\nA planner makes {id} ready with: atelier ready {id} --by <name>\n"
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
/// staged, and each status changed as no act changes it, with the status
/// `HEAD` has and the one staged.
pub fn commit_refused(
    unread: &[Problem],
    invalid: &[Problem],
    acts: &[(String, Status, Status)],
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

/// A commit refused because no config, `HEAD`'s or the checkout's, names
/// the base.
pub fn commit_no_base() -> String {
    let config = atelier_records::CONFIG;
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

/// The pre-push hook's refusal of a push of `base`, whose commit on the
/// remote, if it has one this clone has, is `old`: the new commits on its
/// first-parent line that are not Atelier merges.
pub fn push_refused(base: &str, old: Option<&str>, plain: &[(String, String, ())]) -> String {
    let back = match old.map(short) {
        Some(old) => format!("git branch -f {base} {old}"),
        None => format!("moving {base} back to the remote's commit"),
    };
    let mut text = format!(
        "Atelier's pre-push hook refused this push; nothing was pushed.\nRule: {base} receives only merges made by atelier merge. Each new commit on {base}'s first-parent line is one. These are not:\n"
    );
    text += &commits(plain, |()| "not an Atelier merge".to_string());
    text + &format!(
        "Move them to a branch: atelier start --new \"<title>\" starts an item that holds code, atelier start --backlog <slug> a branch for record changes. Then {back} takes them off {base}, and atelier merge brings the branch in.\n"
    )
}

/// The pre-push hook's refusal of a push that moves `base` where a forge
/// on `remote` is configured; `old` is the remote's commit, if this clone
/// has it.
pub fn push_moves_base(base: &str, remote: &str, old: Option<&str>) -> String {
    let back = match old.map(short) {
        Some(old) => format!("`git branch -f {base} {old}` puts {base} back"),
        None => format!("moving {base} back to {remote}'s commit puts it back"),
    };
    format!(
        "Atelier's pre-push hook refused this push; nothing was pushed.\nRule: {base} is moved only by the forge's merge of a pull request. With a forge configured, atelier merge has the forge merge each branch's pull request and never pushes {base}, so no push may move it.\nPut the commits on a branch and run atelier merge there; {back}.\n"
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
    let then = format!("Then wait for both verdicts. Each reviewer pushes its review file to {branch}; once two approve, bring them here with `git pull`, and run `atelier merge`. If either asks for changes, change the plan, commit, and run `atelier submit` again for the next round.");
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

pub fn not_on_item(command: &str, branch: Option<&str>) -> String {
    format!(
        "`atelier {command}` runs on an item's branch, and this checkout is on {}. Nothing changed.\n`atelier status` lists what you hold; `atelier start <id>` switches to its branch.\n",
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
        let response = match item {
            true => format!("Response: {}\n", response(carried.response.as_ref())),
            false => String::new(),
        };
        text += &format!(
            "\n## Finding {number}: {title}\nRecommendation: fix now\nWas: {finding}\n{response}Now: kept\n<!-- Set Now: to fixed, withdrawn or kept. To keep it for later work instead, set Recommendation: to defer or no action and add a line that says why. -->\n"
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
    /// What [`brief_pull_request`] says, where a forge is configured.
    pub pull_request: Option<String>,
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
        atelier_app::repo::mission_path(id)
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
    text + &format!("Run `atelier submit` here to open the next round of plan reviews, and start two reviewers, each in a checkout of its own at the commit submitted.{fetch}\n")
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
