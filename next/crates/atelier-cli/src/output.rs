//! Everything `status` and the commands print. The wording lives here, in
//! one place, so it is cheap to change, but for what the acts that live in
//! `atelier-app` say and the wording of a refusal, which live in
//! `atelier_app::say` and are used from here.

use crate::forge::{CiRun, ForgeError, PullRequest};
use crate::merge::Moved;
use crate::setup::{Harness, Step};
use atelier_app::acts::{Act, Condition, Refusal};
use atelier_app::{Carried, Entry, Overview};
use atelier_core::{Config, Finding, FindingName, Priority, Response, Status, Thread, Verdict};
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
        "{who}Title: {}\nStatus: {}{}\n{}{undefined}{next}",
        entry.title,
        status_name(entry.status),
        since(entry),
        escalate(entry, ""),
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

/// The Review section's rule after three rounds without an approval, for
/// the implementer, indented by `indent`; nothing refuses a fourth round.
/// In review, `escalate` waits for the round's verdict.
fn escalate(entry: &Entry, indent: &str) -> String {
    let when = match entry.status {
        Status::InReview => "if this round asks for changes too, escalate once its verdict is in",
        _ => "so escalate now",
    };
    entry.unapproved.map_or(String::new(), |rounds| {
        format!("{indent}{rounds} rounds have ended without an approval, and the Review section requires escalation after three, {when}: atelier escalate \"<question>\"\n")
    })
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
            text += &format!(
                "  {} ({}{}): {}\n",
                entry.id,
                status_name(entry.status),
                since(entry),
                entry.title
            );
            text += &escalate(entry, "    ");
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
        }
    }
    text += &cannot_start(&overview.cannot_start, !overview.ready.is_empty());
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
        (None, None) if overview.cannot_start.is_empty() => {
            text += "Nothing is ready to start.\n";
        }
        (None, None) => text += "No ready item can start now.\n",
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
            with(|c| matches!(c, Condition::MissionNotReady { .. })),
            "is in a mission that is not ready",
            "are in a mission that is not ready",
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

pub fn started(id: &str) -> String {
    format!("Started {id} on branch {id}.\n")
}

pub fn started_new(id: &str) -> String {
    format!("Created {id} and started it on branch {id}.\n")
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
    /// The paths of the plan that differ in this checkout from `commit`.
    pub changed: Vec<String>,
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
        "commits it alone, changing no status",
    ) + &format!(
        "Then tell whoever started you the commit it prints, which the planner cherry-picks onto {}. Your review is then done.\n",
        brief.base
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

pub fn escalated(id: &str, question: &str, finding: Option<&FindingName>, decider: &str) -> String {
    let about = finding.map_or(String::new(), |f| format!("It is about finding {f}.\n"));
    format!(
        "Asked under `## Decisions` in {}, and did not commit it:\n  {question}\n{about}`atelier submit` and `atelier merge` wait until it is answered, and `atelier submit` commits it with the item.\n{decider}",
        atelier_app::repo::item_path(id)
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
