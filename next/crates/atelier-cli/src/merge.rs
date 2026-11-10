//! `merge`, the one command that writes to the base
//! (docs/strategies/redesign-architecture.md, Merge and The forge).
//!
//! Copied from `crates/atelier-cli/src/commands/merge.rs` and trimmed to
//! what this command calls: the merged-tree checkout (`MergedCheckout` and
//! its sweep of checkouts a killed merge left, without the lock file each
//! held, since the merge lock now covers the whole run); the merge commit,
//! built in a scratch index on the merged tree (`git::commit_file`); the
//! fast-forward of the base's checkout; and the confirmation, after the
//! checks and before any write, that the branch tip, the base tip and the
//! checkouts are the ones the checks ran on. From `merge_push.rs`: pushing
//! the branch, finding or opening its pull request, and pushing the base.
//! From `crates/atelier-records/src/mutation_lock.rs`: the exclusive lock,
//! without its shared form and re-entry, which nothing here needs.
//! `committed_snapshot.rs`'s checks of file modes and index flags are not
//! carried over.

use crate::commands::{facts, hold, refusal, refused, Outcome};
use crate::forge::{ForgeError, Github, PullRequest};
use crate::git;
use crate::output;
use crate::repo::{item_path, Repo};
use anyhow::{Context, Result};
use atelier_app::acts::{check, Act, Refusal};
use atelier_core::{Config, Forge, Status};
use atelier_records::{parse_item, render_item};
use std::fs::{File, OpenOptions, TryLockError};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Merges the item of the checked-out branch into the base, in one merge
/// commit that marks it done, once its checks pass. With `trivial`, an
/// item nobody reviewed merges on that reason. Run again, it picks up
/// where the facts now stand.
pub fn merge(root: &Path, trivial: Option<String>) -> Result<Outcome> {
    if trivial.as_ref().is_some_and(|why| why.trim().is_empty()) {
        return Ok(Outcome::Refused(output::no_reason()));
    }
    // Everything is read under the lock, so a merge that waited for
    // another reads the base that one left.
    let _lock = lock(root)?;
    let repo = &mut Repo::open()?;
    // The commit everything below is checked against, read first: a commit
    // made after this is caught before anything is written.
    let head = git::head(&repo.root)?;
    let Some(mut located) = repo.current()? else {
        return Ok(Outcome::Refused(output::not_on_item(
            "merge",
            repo.branch.as_deref(),
        )));
    };
    let act = if trivial.is_some() {
        Act::MergeTrivial
    } else {
        Act::Merge
    };
    // A reviewer is known by the review file it holds, which it may not
    // have committed, so its role is told from the checkout, as every
    // command tells it.
    let blockers = repo.blockers(located.record().expect("the current item has a record"))?;
    let seen = facts(repo, &located, &blockers, None);
    if let Err(Refusal::Role(role)) = check(act, &seen) {
        return Ok(Outcome::Refused(refused(act, Refusal::Role(role), &seen)?));
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
        return Ok(Outcome::Refused(output::not_on_item(
            "merge",
            repo.branch.as_deref(),
        )));
    };
    let id = record.item.id.clone();
    let merged = located.branch.as_ref().is_some_and(|branch| branch.merged);
    if record.item.status == Status::Done && merged {
        return publish(repo, &config, &id);
    }
    let checkout = repo.checkout(record, &head)?;
    let facts = facts(repo, &located, &blockers, Some(&checkout));
    if let Some(refused) = refusal(act, &facts)? {
        return Ok(Outcome::Refused(refused));
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
    // The item as the merged tree holds it, marked done.
    let path = item_path(&id);
    let text = git::run(&root, &["cat-file", "blob", &format!("{tree}:{path}")])?;
    let mut item = parse_item(Path::new(&path), &text)?;
    item.status = Status::Done;
    if trivial.is_some() {
        item.trivial = trivial;
    }
    let message = output::merge_message(&id, &item.title, &base, &config);
    let commit = git::commit_file(
        &root,
        &tree,
        &[&base_tip, &head],
        &path,
        &render_item(&item),
        &message,
    )?;

    if !config.merge_checks.is_empty() {
        let checkout = MergedCheckout::add(&root, &commit)?;
        for check in &config.merge_checks {
            if let Some((code, tail)) = run_check(&checkout.path, &check.run)? {
                return Ok(Outcome::Refused(output::check_failed(
                    &id, &base, &check.run, code, &tail,
                )));
            }
        }
    }

    let mut said = String::new();
    if let Some(forge) = &config.forge {
        let github = match Github::open(&root, forge).and_then(|g| g.signed_in().map(|()| g)) {
            Ok(github) => github,
            Err(error) => return Ok(Outcome::Refused(output::forge_failed(&error, false))),
        };
        if let Err(error) = push(&root, &forge.remote, &head, &id) {
            return Ok(Outcome::Refused(output::branch_push_failed(
                &id,
                &forge.remote,
                &error,
            )));
        }
        let pull = match pull_request(&github, &id, &item.title, &base) {
            Ok(pull) => pull,
            Err(error) => return Ok(Outcome::Refused(output::forge_failed(&error, true))),
        };
        said += &output::pull_request(&id, &forge.remote, &pull);
        if let Some(workflow) = &forge.ci_workflow {
            let runs = match github.pull_request_runs(workflow, &head) {
                Ok(runs) => runs,
                Err(error) => return Ok(Outcome::Refused(output::forge_failed(&error, true))),
            };
            let newest = runs.iter().max_by(|a, b| a.created_at.cmp(&b.created_at));
            match newest {
                Some(run) if run.passed() => {}
                Some(run) if run.status == "completed" => {
                    let failed = output::ci_failed(workflow, &head, run);
                    return Ok(Outcome::Refused(said + &failed));
                }
                _ => {
                    let pending = output::ci_pending(workflow, &head, newest);
                    return Ok(Outcome::NotYet(said + &pending));
                }
            }
        }
    }

    if let Some(moved) = moved(repo, &head, &base_tip, base_checkout.as_deref())? {
        return Ok(Outcome::Refused(said + &output::moved(&moved, &id, &base)));
    }
    match &base_checkout {
        Some(path) => git::run(
            path,
            &[
                "-c",
                "core.hooksPath=/dev/null",
                "merge",
                "--ff-only",
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
    said += &output::merged(&id, &base, &commit);
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
    head: &str,
    base_tip: &str,
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
        Ok(git::current_branch(path)?.as_deref() == Some(branch)
            && git::head(path)? == tip
            && git::uncommitted(path)?.is_empty())
    };
    if !unchanged(root, branch, head)? {
        return Ok(Some(Moved::Checkout(root.clone())));
    }
    let now = git::checkout_of(root, &repo.base)?;
    if now.as_deref() != base_checkout {
        let path = now.or_else(|| base_checkout.map(Path::to_path_buf));
        return Ok(Some(Moved::Checkout(path.expect("one of them is a path"))));
    }
    match base_checkout {
        Some(path) if !unchanged(path, &repo.base, base_tip)? => {
            Ok(Some(Moved::Checkout(path.to_path_buf())))
        }
        _ => Ok(None),
    }
}

/// Pushes `commit` to the branch `branch` of `remote`, never forcing.
fn push(root: &Path, remote: &str, commit: &str, branch: &str) -> Result<()> {
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
fn pull_request(
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
/// in the Git directory all its worktrees share, held for the whole merge.
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
