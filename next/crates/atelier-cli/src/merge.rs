//! `merge`, the one command that writes to the base
//! (docs/strategies/redesign-architecture.md, Merge and The forge).
//!
//! Copied from `crates/atelier-cli/src/commands/merge.rs` and trimmed to
//! what this command calls: the merged-tree checkout (`MergedCheckout` and
//! its sweep of checkouts a killed merge left, without the lock file each
//! held, since the merge lock covers everything after the wait); the merge
//! commit, built in a scratch index on the merged tree
//! (`git::commit_file`); the fast-forward of the base's checkout; and the
//! confirmation, after the checks and before any write, that the branch
//! tip, the base tip and the checkouts are the ones the checks ran on.
//! From `merge_push.rs`: pushing the branch, finding or opening its pull
//! request, and pushing the base. From
//! `crates/atelier-records/src/mutation_lock.rs`: the exclusive lock,
//! without its shared form and re-entry, which nothing here needs.
//! `committed_snapshot.rs`'s checks of file modes and index flags are not
//! carried over. The wait for CI before the lock, and which runs count for
//! the head, are new.

use crate::commands::{facts, gate, hold, refusal, refused, Outcome};
use crate::forge::{CiRun, ForgeError, Github, PullRequest};
use crate::git;
use crate::repo::{item_dir, item_path, Repo};
use crate::{output, skip};
use anyhow::{Context, Result};
use atelier_app::acts::{check, Act, Refusal};
use atelier_core::{Config, Forge, Status};
use atelier_records::{parse_item, render_item};
use std::collections::HashMap;
use std::fs::{File, OpenOptions, TryLockError};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// How long `merge` waits for CI: under the 10 minutes a harness such as
/// Claude Code allows a command in the foreground.
pub(crate) const WAIT: Duration = Duration::from_secs(9 * 60);
/// How often it asks the forge while it waits.
const POLL: Duration = Duration::from_secs(15);
/// How long it gives the forge to answer under the lock, where it holds up
/// every other merge, and never longer than the wait.
const LOCKED: Duration = Duration::from_secs(60);

/// Merges the item of the checked-out branch into the base, in one merge
/// commit that marks it done, once its checks pass. With `trivial`, an
/// item nobody reviewed merges on that reason. Run again, it picks up
/// where the facts now stand.
pub fn merge(root: &Path, trivial: Option<String>) -> Result<Outcome> {
    if trivial.as_ref().is_some_and(|why| why.trim().is_empty()) {
        return Ok(Outcome::Refused(output::no_reason()));
    }
    // Until CI passes nothing is locked and nothing in the clone written,
    // so a merge interrupted then leaves it as it was; everything read
    // before is read again under the lock.
    let judged = match before_lock(trivial.is_some())? {
        Ok(id) => id,
        Err(outcome) => return Ok(outcome),
    };
    // Everything is read under the lock, so a merge that waited for
    // another reads the base that one left.
    let _lock = lock(root)?;
    let repo = &mut Repo::open()?;
    // The item it waited for is the one it merges.
    if repo.branch.as_deref() != Some(&judged) {
        let moved = Moved::Checkout(repo.root.clone());
        return Ok(Outcome::Refused(output::moved(&moved, &judged, &repo.base)));
    }
    let Judged {
        head,
        id,
        config,
        merged,
        uncommitted,
        ..
    } = match read(repo, trivial.is_some())? {
        Ok(read) => read,
        Err(outcome) => return Ok(outcome),
    };
    if merged {
        return publish(repo, &config, &id);
    }

    // What the checks run on, confirmed unchanged before anything is
    // written.
    hold("ATELIER_TEST_HOLD_AFTER_READ");
    let root = repo.root.clone();
    let base = repo.base.clone();
    let base_tip = repo.base_tip.clone();
    let base_checkout = git::checkout_of(&root, &base)?;
    if let Some(path) = &base_checkout {
        let dirty = git::uncommitted(path)?;
        if !dirty.is_empty() {
            return Ok(Outcome::Refused(output::base_dirty(&base, path, &dirty)));
        }
    }

    let tree = match git::merge_tree(&root, &base_tip, &head)? {
        Ok(tree) => tree,
        Err(conflicts) => return Ok(Outcome::Refused(output::conflicts(&id, &base, &conflicts))),
    };
    // A passing run must still count for the head as it now is.
    if let Some(forge) = &config.forge {
        if let Some(workflow) = &forge.ci_workflow {
            let github = match Github::open(&root, forge) {
                Ok(github) => github.until(Instant::now() + LOCKED.min(wait_times().0)),
                Err(error) => return Ok(Outcome::Refused(output::forge_failed(&error, false))),
            };
            let mut known = HashMap::new();
            let now = ci(&github, workflow, &root, &id, &head, &mut known)?;
            if let Some(stopped) = ci_stops(now, workflow, &head) {
                return Ok(stopped);
            }
        }
    }
    // The item as the merged tree holds it, marked done.
    let path = item_path(&id);
    let text = git::run(&root, &["cat-file", "blob", &format!("{tree}:{path}")])?;
    let mut item = parse_item(Path::new(&path), &text)?;
    item.status = Status::Done;
    if trivial.is_some() {
        item.trivial = trivial;
    }
    let message = output::merge_message(&id, &item.title, &base, &config);
    // Its message carries each rule skipped so far.
    let commit_merge = || {
        let item = render_item(&item);
        git::commit_file(&root, &tree, &[&base_tip, &head], &path, &item, &message)
    };
    let mut commit = commit_merge()?;

    if !config.merge_checks.is_empty() {
        let checkout = MergedCheckout::add(&root, &commit)?;
        let mut skipped = false;
        for check in &config.merge_checks {
            if let Some((code, tail)) = run_check(&checkout.path, &check.run)? {
                let text = output::check_failed(&id, &base, &check.run, code, &tail);
                let rule = output::rule_check(&check.run, code);
                if let Some(refused) = skip::refuse([rule], text) {
                    return Ok(refused);
                }
                skipped = true;
            }
        }
        // The same merge, with the checks it skipped in its message.
        if skipped {
            commit = commit_merge()?;
        }
    }

    let read = (head.as_str(), base_tip.as_str(), uncommitted.as_slice());
    if let Some(moved) = moved(repo, read, base_checkout.as_deref())? {
        return Ok(Outcome::Refused(output::moved(&moved, &id, &base)));
    }
    match &base_checkout {
        Some(path) => git::run(
            path,
            &[
                "-c",
                "core.hooksPath=/dev/null",
                "merge",
                "--ff-only",
                // A local file Git ignores is still someone's work.
                "--no-overwrite-ignore",
                "--quiet",
                &commit,
            ],
        ),
        None => git::run(
            &root,
            &[
                "update-ref",
                &format!("refs/heads/{base}"),
                &commit,
                &base_tip,
            ],
        ),
    }
    .with_context(|| output::not_merged(&base))?;
    let mut said = output::merged(&id, &base, &commit);
    if let Some(forge) = &config.forge {
        if let Err(error) = push(&root, &forge.remote, &commit, &base) {
            return Ok(Outcome::Refused(
                said + &output::base_push_failed(&base, &forge.remote, &error),
            ));
        }
        said += &output::base_pushed(&base, &forge.remote);
    }
    Ok(Outcome::Done(said))
}

/// What `merge` judges, read from the clone as it now stands.
struct Judged {
    /// The commit every later step is checked against.
    head: String,
    id: String,
    title: String,
    /// The base's.
    config: Config,
    /// The item is done and its branch merged into the base.
    merged: bool,
    /// What the checkout holds that no commit does, which a skip let stay.
    uncommitted: Vec<String>,
}

/// Reads the head, then the item as the head holds it and the base's
/// config, and checks the act's conditions on them. The outcome is a
/// refusal.
fn read(repo: &mut Repo, trivial: bool) -> Result<Result<Judged, Outcome>> {
    // Read first: a commit made after this is caught before anything is
    // written.
    let head = git::head(&repo.root)?;
    let Some(mut located) = repo.current()? else {
        return Ok(Err(Outcome::Refused(output::not_on_item(
            "merge",
            repo.branch.as_deref(),
        ))));
    };
    let act = if trivial {
        Act::MergeTrivial
    } else {
        Act::Merge
    };
    // A reviewer is known by the review file it holds, which it may not
    // have committed, so its role is told from the checkout, as every
    // command tells it.
    let links = repo.links(located.record().expect("the current item has a record"))?;
    let seen = facts(repo, &located, &links, None);
    if let Err(role @ Refusal::Role(_)) = check(act, &seen) {
        let text = refused(act, &role, &seen)?;
        if let Some(refused) = gate(act, &located.id, &[role], text) {
            return Ok(Err(refused));
        }
    }
    // What merges is the head, so the item is judged as it holds it, not
    // as the checkout does, whose uncommitted changes are a condition of
    // their own.
    if let Some(branch) = located.branch.as_mut() {
        branch.record = repo.item_at(&head, &located.id)?;
    }
    let config = repo.base_config()?;
    let Some(record) = located.record() else {
        // Its record is in the checkout and in no commit.
        return Ok(Err(Outcome::Refused(output::not_on_item(
            "merge",
            repo.branch.as_deref(),
        ))));
    };
    let id = record.item.id.clone();
    let title = record.item.title.clone();
    let merged = record.item.status == Status::Done
        && located.branch.as_ref().is_some_and(|branch| branch.merged);
    let uncommitted = repo.uncommitted(&id)?;
    if !merged {
        let checkout = repo.checkout(record, &head)?;
        let facts = facts(repo, &located, &links, Some(&checkout));
        if let Some(refused) = refusal(act, &facts)? {
            return Ok(Err(refused));
        }
    }
    Ok(Ok(Judged {
        head,
        id,
        title,
        config,
        merged,
        uncommitted,
    }))
}

/// What comes before the lock where a forge is configured: the branch
/// pushed and its pull request opened, each only when missing, then the
/// wait for CI. It prints as it goes, since the wait is long. It returns
/// the item it judged, or the outcome when `merge` stops here.
fn before_lock(trivial: bool) -> Result<Result<String, Outcome>> {
    let repo = &mut Repo::open()?;
    let judged = match read(repo, trivial)? {
        Ok(judged) => judged,
        Err(outcome) => return Ok(Err(outcome)),
    };
    let Some(forge) = judged.config.forge.as_ref().filter(|_| !judged.merged) else {
        return Ok(Ok(judged.id));
    };
    let (root, base) = (&repo.root, &repo.base);
    let Judged { head, id, .. } = &judged;
    // CI does not run on a pull request that conflicts.
    if let Err(conflicts) = git::merge_tree(root, &repo.base_tip, head)? {
        return Ok(Err(Outcome::Refused(output::conflicts(
            id, base, &conflicts,
        ))));
    }
    let github = match Github::open(root, forge).and_then(|g| g.signed_in().map(|()| g)) {
        Ok(github) => github,
        Err(error) => return Ok(Err(Outcome::Refused(output::forge_failed(&error, false)))),
    };
    // A push would start CI again, and cancel a run in progress, so the
    // branch is pushed only when no run on the forge's copy would count.
    let mut known = HashMap::new();
    let pushed = match git::remote_tip(root, &forge.remote, id)? {
        Some(tip) if counts(root, id, head, &tip, &mut known)? => false,
        _ => {
            if let Err(error) = push(root, &forge.remote, head, id) {
                let failed = output::branch_push_failed(id, &forge.remote, &error);
                return Ok(Err(Outcome::Refused(failed)));
            }
            true
        }
    };
    let pull = match pull_request(&github, id, &judged.title, base) {
        Ok(pull) => pull,
        Err(error) => return Ok(Err(Outcome::Refused(output::forge_failed(&error, pushed)))),
    };
    say(&output::pull_request(id, &forge.remote, &pull, pushed));
    let Some(workflow) = &forge.ci_workflow else {
        return Ok(Ok(judged.id.clone()));
    };
    let (wait, poll) = wait_times();
    let deadline = Instant::now() + wait;
    let github = github.until(deadline);
    // Said before the first poll, which may itself take the whole bound.
    say(&output::waiting_for_ci(workflow, &pull, wait));
    let mut last = None;
    loop {
        let now = match ci(&github, workflow, root, id, head, &mut known)? {
            // A skip passes a run that has not finished without waiting
            // for it; a forge that does not answer is waited for.
            Ci::Pending(now) if now.is_err() || skip::reason().is_none() => now,
            now => {
                return Ok(match ci_stops(now, workflow, head) {
                    Some(stopped) => Err(stopped),
                    None => Ok(judged.id.clone()),
                })
            }
        };
        // A poll that fails, or that the bound ends, does not replace what
        // an earlier one said: the last answer, or until one comes, the
        // first failure.
        let pending = match (last.take(), now) {
            (Some(kept), Err(_)) => kept,
            (_, now) => now,
        };
        let left = deadline.saturating_duration_since(Instant::now());
        std::thread::sleep(poll.min(left));
        if left <= poll {
            let pending = output::ci_pending(workflow, head, &pending);
            return Ok(Err(Outcome::NotYet(pending)));
        }
        last = Some(pending);
    }
}

/// What stops `merge` where CI stands `now`: nothing once it passed, or
/// once `ATELIER_SKIP` passed the rule that it pass; else a refusal when it
/// failed, and "not yet" when it has not finished or the forge did not
/// answer, which is no rule.
fn ci_stops(now: Ci, workflow: &str, head: &str) -> Option<Outcome> {
    let pending = |now| Some(Outcome::NotYet(output::ci_pending(workflow, head, &now)));
    match now {
        Ci::Passed => None,
        Ci::Failed(run) => {
            let rule = output::rule_ci(workflow, Some(&run));
            skip::refuse([rule], output::ci_failed(workflow, &run))
        }
        Ci::Pending(Ok(run)) if skip::pass([output::rule_ci(workflow, run.as_ref())]) => None,
        Ci::Pending(now) => pending(now),
    }
}

/// Prints `text` at once, while `merge` goes on.
fn say(text: &str) {
    let mut stdout = std::io::stdout();
    let _ = stdout.write_all(text.as_bytes());
    let _ = stdout.flush();
}

/// The bound of the wait and the time between polls. Tests shorten them
/// through `ATELIER_TEST_CI_WAIT`, `<wait ms>,<poll ms>`, which only debug
/// builds read.
fn wait_times() -> (Duration, Duration) {
    #[cfg(debug_assertions)]
    if let Some((wait, poll)) = std::env::var("ATELIER_TEST_CI_WAIT")
        .ok()
        .as_deref()
        .and_then(|times| times.split_once(','))
        .and_then(|(wait, poll)| Some((wait.parse().ok()?, poll.parse().ok()?)))
    {
        return (Duration::from_millis(wait), Duration::from_millis(poll));
    }
    (WAIT, POLL)
}

/// Where CI stands for the head.
enum Ci {
    Passed,
    Failed(CiRun),
    /// No run that counts has finished, or the latest finished one failed
    /// and a newer one is running: the newest that counts. Or the forge did
    /// not answer, and why.
    Pending(Result<Option<CiRun>, ForgeError>),
}

/// Where CI stands for `head`, from the runs that count for it: the latest
/// finished one decides, unless it failed and a newer one is running.
/// `known` keeps what [`counts`] found for each commit.
fn ci(
    github: &Github,
    workflow: &str,
    root: &Path,
    id: &str,
    head: &str,
    known: &mut HashMap<String, bool>,
) -> Result<Ci> {
    let runs = match github.branch_runs(workflow, id) {
        Ok(runs) => runs,
        Err(error) => return Ok(Ci::Pending(Err(error))),
    };
    let mut counting = Vec::new();
    for run in runs {
        if counts(root, id, head, &run.head_sha, known)? {
            counting.push(run);
        }
    }
    counting.sort_by(|a, b| (&a.created_at, a.id).cmp(&(&b.created_at, b.id)));
    let latest = counting.iter().rposition(CiRun::finished);
    Ok(match latest {
        Some(at) if counting[at].passed() => Ci::Passed,
        Some(at) if !counting[at + 1..].iter().any(CiRun::running) => {
            Ci::Failed(counting.swap_remove(at))
        }
        _ => Ci::Pending(Ok(counting.pop())),
    })
}

/// Whether a run on `commit` counts for `head`: `commit` is `head`, or an
/// earlier commit of it from which `head` differs only inside the item's
/// own directory, as when `verdict approve` committed after CI started.
fn counts(
    root: &Path,
    id: &str,
    head: &str,
    commit: &str,
    known: &mut HashMap<String, bool>,
) -> Result<bool> {
    if let Some(counts) = known.get(commit) {
        return Ok(*counts);
    }
    let own = format!("{}/", item_dir(id));
    let counts = commit == head
        || (git::has_commit(root, commit)?
            && git::is_ancestor(root, commit, head)?
            && git::changed(root, commit, head)?
                .iter()
                .all(|path| path.starts_with(&own)));
    known.insert(commit.to_string(), counts);
    Ok(counts)
}

/// `merge` run again on an item it merged: pushes the base where a forge
/// is configured, since a run that stopped after the merge commit left
/// that undone.
fn publish(repo: &Repo, config: &Config, id: &str) -> Result<Outcome> {
    let base = &repo.base;
    let mut said = output::already_merged(id, base);
    if let Some(Forge { remote, .. }) = &config.forge {
        if let Err(error) = push(&repo.root, remote, &repo.base_tip, base) {
            return Ok(Outcome::Refused(
                said + &output::base_push_failed(base, remote, &error),
            ));
        }
        said += &output::base_pushed(base, remote);
    }
    Ok(Outcome::Done(said))
}

/// What moved while the checks ran.
pub enum Moved {
    Branch,
    Base,
    Checkout(PathBuf),
}

/// The first input of the merge that is no longer what the checks ran on:
/// the branch's tip, the base's tip, or a checkout of either.
fn moved(
    repo: &Repo,
    (head, base_tip, uncommitted): (&str, &str, &[String]),
    base_checkout: Option<&Path>,
) -> Result<Option<Moved>> {
    let root = &repo.root;
    let branch = repo.branch.as_deref().expect("merge runs on a branch");
    if git::branch_tip(root, branch)?.as_deref() != Some(head) {
        return Ok(Some(Moved::Branch));
    }
    if git::branch_tip(root, &repo.base)?.as_deref() != Some(base_tip) {
        return Ok(Some(Moved::Base));
    }
    let unchanged = |path: &Path, branch: &str, tip: &str| -> Result<bool> {
        Ok(git::current_branch(path)?.as_deref() == Some(branch) && git::head(path)? == tip)
    };
    // What a skip let the checkout keep uncommitted stays as it was read.
    if !unchanged(root, branch, head)? || repo.uncommitted(branch)? != uncommitted {
        return Ok(Some(Moved::Checkout(root.clone())));
    }
    let now = git::checkout_of(root, &repo.base)?;
    if now.as_deref() != base_checkout {
        let path = now.or_else(|| base_checkout.map(Path::to_path_buf));
        return Ok(Some(Moved::Checkout(path.expect("one of them is a path"))));
    }
    match base_checkout {
        Some(path)
            if !unchanged(path, &repo.base, base_tip)? || !git::uncommitted(path)?.is_empty() =>
        {
            Ok(Some(Moved::Checkout(path.to_path_buf())))
        }
        _ => Ok(None),
    }
}

/// Pushes `commit` to the branch `branch` of `remote`, never forcing.
pub(crate) fn push(root: &Path, remote: &str, commit: &str, branch: &str) -> Result<()> {
    git::run(
        root,
        &[
            "push",
            "--quiet",
            remote,
            &format!("{commit}:refs/heads/{branch}"),
        ],
    )
    .map(drop)
}

/// The open pull request from `branch` into `base`, opened when there is
/// none.
pub(crate) fn pull_request(
    github: &Github,
    branch: &str,
    title: &str,
    base: &str,
) -> Result<PullRequest, ForgeError> {
    if let Some(pull) = github.open_pull_request(branch, base)? {
        return Ok(pull);
    }
    let (title, body) = output::pull_request_text(branch, title, base);
    github.create_pull_request(branch, base, &title, &body)
}

/// Runs one merge check in `dir`. When it fails, returns its exit code and
/// the end of its output.
fn run_check(dir: &Path, run: &str) -> Result<Option<(Option<i32>, String)>> {
    let (mut reader, writer) = std::io::pipe()?;
    let mut child = Command::new("sh")
        .args(["-c", run])
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(writer.try_clone()?)
        .stderr(writer)
        .spawn()
        .with_context(|| format!("cannot start the merge check `{run}`"))?;
    let mut text = Vec::new();
    // The command and its writers hold the pipe's other end; reading ends
    // when the last of them exits.
    reader.read_to_end(&mut text)?;
    let status = child.wait()?;
    if status.success() {
        return Ok(None);
    }
    let text = String::from_utf8_lossy(&text);
    let lines: Vec<&str> = text.lines().collect();
    let tail = lines[lines.len().saturating_sub(20)..].join("\n");
    Ok(Some((status.code(), tail)))
}

/// The lock that serialises merges in a clone: an exclusive lock on a file
/// in the Git directory all its worktrees share, held from the read after
/// the wait for CI to the end of the merge.
/// A second merge waits for it; the operating system releases it when a
/// merge ends, killed or not.
fn lock(root: &Path) -> Result<File> {
    let path = git::common_dir(root)?.join("atelier-merge.lock");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&path)
        .with_context(|| format!("cannot open {}", path.display()))?;
    match file.try_lock() {
        Ok(()) => return Ok(file),
        Err(TryLockError::WouldBlock) => eprint!("{}", output::waiting()),
        Err(TryLockError::Error(error)) => return Err(error.into()),
    }
    file.lock()
        .with_context(|| format!("cannot lock {}", path.display()))?;
    Ok(file)
}

const MERGE_DIR_PREFIX: &str = "atelier-merge-";

/// A detached checkout of the merge commit in the Git directory, outside
/// every working tree, so the merge checks run on exactly the tree merging
/// puts on the base. `Drop` removes it; a killed merge cannot, and the next
/// merge, which holds the lock, removes what it left.
struct MergedCheckout {
    repo: PathBuf,
    path: PathBuf,
}

impl MergedCheckout {
    fn add(repo: &Path, commit: &str) -> Result<Self> {
        let common = git::common_dir(repo)?;
        Self::sweep(repo, &common)?;
        let path = common.join(format!("{MERGE_DIR_PREFIX}{}", std::process::id()));
        let shown = path.to_string_lossy();
        git::run(
            repo,
            &[
                "-c",
                "core.hooksPath=/dev/null",
                "worktree",
                "add",
                "--quiet",
                "--detach",
                &shown,
                commit,
            ],
        )
        .context("cannot check out the merged tree for the merge checks")?;
        Ok(Self {
            repo: repo.to_path_buf(),
            path,
        })
    }

    /// Removes the checkouts earlier merges left when they were killed
    /// during their checks. Under the lock, none belongs to a live merge.
    fn sweep(repo: &Path, common: &Path) -> Result<()> {
        for entry in std::fs::read_dir(common)? {
            let dir = entry?.path();
            let leftover = dir
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(MERGE_DIR_PREFIX) && !name.ends_with(".lock"));
            if leftover {
                std::fs::remove_dir_all(&dir)
                    .with_context(|| format!("cannot remove {}", dir.display()))?;
            }
        }
        git::run(repo, &["worktree", "prune"]).map(drop)
    }
}

impl Drop for MergedCheckout {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
        let _ = git::run(&self.repo, &["worktree", "prune"]);
    }
}
