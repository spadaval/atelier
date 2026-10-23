//! Gated integration: a fast-forward, or under `merge_strategy:
//! merge_commit` a merge commit whose tree passed `merge_checks`. Creates no
//! other commits and no tracker records.
use crate::commands::git::text;
use anyhow::{bail, Context, Result};
use atelier_app::{
    base_policy, committed_snapshot, git_freshness as git,
    record_index::RecordIndex,
    review_gate,
    storage_layout::StorageLayout,
    workflow_policy::{MergeStrategy, ValidatorParams, WorkflowPolicy},
};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

/// A detached checkout of the merge commit, outside every working tree, so
/// `merge_checks` run on exactly the tree merging would put on the base.
///
/// Each checkout sits in its own `atelier-merge-*` directory in the Git common
/// directory and holds an exclusive lock on that directory's `lock` file for
/// its life. `Drop` removes the checkout, but a killed merge cannot run it:
/// the operating system releases the lock instead, which is how the next merge
/// tells a leftover checkout from one a concurrent merge is still using.
struct MergedCheckout {
    repo: PathBuf,
    path: PathBuf,
    _dir: tempfile::TempDir,
    _lock: File,
}
impl MergedCheckout {
    fn add(repo: &Path, commit: &str) -> Result<Self> {
        let common = PathBuf::from(text(
            repo,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?);
        // Sweeping and creating are serialized, so the sweep never meets a
        // directory whose lock file does not exist yet.
        let creating = lock_file(&common.join("atelier-merge.lock"))?;
        creating.lock()?;
        Self::sweep(repo, &common);
        let dir = tempfile::Builder::new()
            .prefix(MERGE_DIR_PREFIX)
            .tempdir_in(&common)?;
        let lock = lock_file(&dir.path().join("lock"))?;
        lock.lock()?;
        let path = dir.path().join("tree");
        git::command(repo)
            .args([
                "-c",
                "core.hooksPath=/dev/null",
                "worktree",
                "add",
                "--quiet",
                "--detach",
            ])
            .arg(&path)
            .arg(commit)
            .output()
            .ok()
            .filter(|output| output.status.success())
            .context("could not check out the merged tree for merge_checks")?;
        Ok(Self {
            repo: repo.to_owned(),
            path,
            _dir: dir,
            _lock: lock,
        })
    }

    /// Remove the checkouts earlier merges left behind when they were killed
    /// during their checks. A directory whose lock is held belongs to a live
    /// merge and stays.
    fn sweep(repo: &Path, common: &Path) {
        let Ok(entries) = std::fs::read_dir(common) else {
            return;
        };
        for dir in entries.filter_map(|entry| entry.ok().map(|entry| entry.path())) {
            let leftover = dir
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(MERGE_DIR_PREFIX))
                && lock_file(&dir.join("lock")).is_ok_and(|lock| lock.try_lock().is_ok());
            if leftover {
                let _ = git::command(repo)
                    .args(["worktree", "remove", "--force"])
                    .arg(dir.join("tree"))
                    .output();
                if let Err(error) = discard(&dir) {
                    eprintln!(
                        "warning: could not remove leftover merge checkout {}: {error}",
                        dir.display()
                    );
                }
            }
        }
        let _ = git::git_output(repo, &["worktree", "prune"]);
    }
}
/// Remove a leftover directory; one already gone, such as to a concurrent
/// sweep, counts as removed.
fn discard(dir: &Path) -> std::io::Result<()> {
    match std::fs::remove_dir_all(dir) {
        Err(error) if error.kind() != std::io::ErrorKind::NotFound => Err(error),
        _ => Ok(()),
    }
}
impl Drop for MergedCheckout {
    fn drop(&mut self) {
        let _ = git::command(&self.repo)
            .args(["worktree", "remove", "--force"])
            .arg(&self.path)
            .output();
    }
}

const MERGE_DIR_PREFIX: &str = "atelier-merge-";

fn lock_file(path: &Path) -> Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .with_context(|| format!("could not open {}", path.display()))
}

/// Git's merge of the work branch into the base, refused on any conflict.
fn merged_tree(
    repo: &Path,
    base: &str,
    base_head: &str,
    branch: &str,
    head: &str,
) -> Result<String> {
    let (tree, conflicted) = git::merge_tree(repo, base_head, head)?
        .with_context(|| format!("git merge-tree could not merge {branch} into {base}"))?;
    if !conflicted.is_empty() {
        bail!(
            "merge_conflict: {branch} does not merge cleanly into {base}; conflicts in {}. Merge it in with `git merge {base}` on {branch}, resolve the conflicts, record fresh proof and get the resolution reviewed, then retry.",
            conflicted
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(tree)
}

/// The commands of the `merge_checks` that run on the merged tree; the other
/// checks read records, so they prove nothing about what merging builds.
fn tree_checks(policy: &WorkflowPolicy) -> impl Iterator<Item = &str> {
    policy
        .merge_checks
        .iter()
        .filter_map(|check| match &check.params {
            Some(ValidatorParams::ShellCheck { command }) => Some(command.as_str()),
            _ => None,
        })
}

/// The merge commit object; it reaches the base only if its tree passes.
fn merge_commit(
    repo: &Path,
    policy: &WorkflowPolicy,
    tree: &str,
    (base, base_head): (&str, &str),
    (branch, head): (&str, &str),
) -> Result<String> {
    let checks = tree_checks(policy)
        .map(|command| format!("- shell.check: {command}"))
        .collect::<Vec<_>>()
        .join("\n");
    let message = format!(
        "Merge {branch} into {base}\n\nThe base policy's merge_checks passed on this tree before atelier merge\nput it on {base}:\n{checks}\n"
    );
    text(
        repo,
        &[
            "commit-tree",
            tree,
            "-p",
            base_head,
            "-p",
            head,
            "-m",
            &message,
        ],
    )
}

/// Refuse records the base changed since the work branch left it that bear
/// on the item: in-scope issues, and reviews or evidence about them. The
/// merge commit would carry them onto the base, but review and evidence pins
/// are judged only along the work branch, so merging the base into the
/// branch is what lets the gates judge them. Other base records, such as a
/// blocker, are judged by the gates reading the merged records.
fn base_records(
    repo: &Path,
    (branch_index, merged): (&RecordIndex, &RecordIndex),
    owner: &str,
    (base, branch): (&str, &str),
    (head, commit): (&str, &str),
) -> Result<()> {
    let mut ids = review_gate::scope(branch_index, owner)?;
    ids.extend(review_gate::scope(merged, owner)?);
    let bears = |index: &RecordIndex, kind: &str, id: &str| match kind {
        "issues" => ids.contains(id),
        "reviews" => index
            .require_review(id)
            .is_ok_and(|review| ids.contains(&review.target)),
        "evidence" => index.list_record_links("evidence", id).is_ok_and(|links| {
            links
                .iter()
                .any(|link| ids.contains(&link.source_id) || ids.contains(&link.target_id))
        }),
        _ => false,
    };
    // Where the merged records differ from the branch's, the base changed them.
    let changed = git::git_output(
        repo,
        &[
            "diff",
            "--name-only",
            "-z",
            "--no-renames",
            head,
            commit,
            "--",
            ".atelier/",
        ],
    )?;
    let relevant = changed
        .split(|byte| *byte == 0)
        .filter_map(|path| std::str::from_utf8(path).ok())
        .filter(|path| {
            let mut parts = Path::new(path).components().skip(1);
            let (Some(kind), Some(file)) = (parts.next(), parts.next()) else {
                return false;
            };
            let (Some(kind), Some(id)) = (
                kind.as_os_str().to_str(),
                Path::new(file.as_os_str())
                    .file_stem()
                    .and_then(|id| id.to_str()),
            ) else {
                return false;
            };
            parts.next().is_none() && (bears(merged, kind, id) || bears(branch_index, kind, id))
        })
        .collect::<Vec<_>>();
    if !relevant.is_empty() {
        bail!(
            "merge_base_records: {base} changed records about {owner} since {branch} left it: {}. The merge commit would carry them onto {base} unchecked. Merge it in with `git merge {base}` on {branch} so the gates judge them, then retry; a clean merge keeps the approval unless it changes an in-scope issue record beyond its status, and then review.current names the merge and the record.",
            relevant.join(", ")
        );
    }
    Ok(())
}

fn clean(repo: &Path) -> Result<()> {
    let dirty = text(repo, &["status", "--porcelain", "--untracked-files=all"])?;
    if !dirty.is_empty() {
        bail!("merge_checkout_dirty: {} has uncommitted changes:\n{dirty}\nCommit or stash them before retrying.", repo.display());
    }
    Ok(())
}
fn base_checkout(repo: &Path, base: &str) -> Result<Option<PathBuf>> {
    let worktrees =
        String::from_utf8(git::git_output(repo, &["worktree", "list", "--porcelain"])?)?;
    for block in worktrees.split("\n\n") {
        if block
            .lines()
            .any(|line| line == format!("branch refs/heads/{base}"))
        {
            let path = block
                .lines()
                .find_map(|line| line.strip_prefix("worktree "))
                .map(PathBuf::from);
            if block.lines().any(|line| line.starts_with("locked")) {
                let shown = path
                    .as_ref()
                    .map_or("<path>".to_string(), |path| path.display().to_string());
                bail!("merge_base_locked: the {base} worktree is locked. Next: unlock it with `git worktree unlock {shown}`, then retry");
            }
            return Ok(path);
        }
    }
    Ok(None)
}

/// How many times one `atelier merge` re-runs its gates against a base that
/// moved by records-only commits while the checks ran.
const MAX_ATTEMPTS: usize = 3;

/// The base moved during the checks only by records-only commits, so the
/// gates are worth running again against the new base.
#[derive(Debug)]
struct RetryOnMovedBase(String);
impl std::fmt::Display for RetryOnMovedBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for RetryOnMovedBase {}

/// The first of the commits `from..to` that changes anything but records, or
/// `None` when they are all records-only. Records the item itself owns are
/// judged by the next attempt's gates, not here.
fn first_code_commit(repo: &Path, from: &str, to: &str) -> Result<Option<(String, PathBuf)>> {
    let commits = text(repo, &["rev-list", "--reverse", &format!("{from}..{to}")])?;
    for commit in commits.lines() {
        let parent = format!("{commit}^1");
        if let Some(path) = git::changed_paths(repo, &parent, commit)?
            .into_iter()
            .find(|path| !git::is_record_path(path))
        {
            return Ok(Some((commit.to_owned(), path)));
        }
    }
    Ok(None)
}

/// The error for a base that moved while checks ran: a retry marker when the
/// move is records-only and the strategy can land on the new base without
/// creating commits other than the merge commit, else a refusal naming what
/// prevents the retry.
fn base_moved(
    repo: &Path,
    id: &str,
    strategy: MergeStrategy,
    base: &str,
    (from, to): (&str, &str),
) -> anyhow::Error {
    let named = |commit: &str, path: &Path| {
        anyhow::anyhow!("merge_snapshot_changed: {base} moved while checks ran, and {} changes {}, which is not a record. Next: rerun `atelier merge {id}` with fresh proof and review", crate::human_output::short_commit(commit), path.display())
    };
    let ancestor = git::git_output(repo, &["merge-base", "--is-ancestor", from, to]).is_ok();
    if !ancestor {
        return anyhow::anyhow!("merge_snapshot_changed: {base} was rewritten while checks ran, from {} to {}. Next: rerun `atelier merge {id}` with fresh proof and review", crate::human_output::short_commit(from), crate::human_output::short_commit(to));
    }
    match first_code_commit(repo, from, to) {
        Err(error) => error,
        Ok(Some((commit, path))) => named(&commit, &path),
        Ok(None) if strategy == MergeStrategy::FastForwardOnly => anyhow::anyhow!("merge_diverged: {base} gained only records-only commits ({} to {}) while checks ran, but merge_strategy fast_forward_only creates no merge commit. Merge it in with `git merge {base}` on the work branch, record fresh proof, then retry; a clean merge keeps the approval unless it changes an in-scope issue record beyond its status, and then review.current names the merge and the record.", crate::human_output::short_commit(from), crate::human_output::short_commit(to)),
        Ok(None) => RetryOnMovedBase(format!("{base} moved from {} to {} while checks ran, only by records-only commits", crate::human_output::short_commit(from), crate::human_output::short_commit(to))).into(),
    }
}

/// Lets a test move the base onto the branch named by
/// `ATELIER_TEST_MERGE_MOVE_BASE_TO` after the checks and before the locks.
#[cfg(feature = "test-hooks")]
fn test_move_base_before_locks(repo: &Path, base: &str) {
    if let Some(target) = std::env::var_os("ATELIER_TEST_MERGE_MOVE_BASE_TO") {
        let target = format!("refs/heads/{}", target.to_string_lossy());
        let _ = git::git_output(
            repo,
            &["update-ref", &format!("refs/heads/{base}"), &target],
        );
    }
}

/// Quiet output is `passed` once the gates pass (and, without `--check`, the
/// merge is done); a failed gate's line goes to stderr instead. A base that
/// moves by records-only commits while checks run is merged again, up to
/// `MAX_ATTEMPTS` times; any other move refuses.
pub fn run(issue_ref: &str, check_only: bool, json: bool, quiet: bool) -> Result<()> {
    let result = attempts(issue_ref, check_only, json, quiet);
    // `--json` is a script's format and keeps its errors (ADR 0022).
    if json {
        return result;
    }
    let say = |line: &str| {
        if quiet {
            eprintln!("{line}");
        } else {
            println!("{line}");
        }
    };
    match result {
        Ok(()) => {
            println!("{}", if quiet { "passed" } else { "merge: passed" });
            Ok(())
        }
        Err(error) => {
            // The gates' lines are printed; a precondition's error is not.
            let failed = match error.downcast_ref::<crate::gate_report::GatesRefused>() {
                Some(refused) => refused.report.failed.clone(),
                None => {
                    eprintln!("Error: {error:?}");
                    refusal_code(&error).into_iter().collect()
                }
            };
            say(&if failed.is_empty() {
                "merge: refused".to_owned()
            } else {
                crate::gate_report::verdict("merge", &failed)
            });
            Err(super::session::Reported.into())
        }
    }
}

/// The code of a refusal by one of merge's own checks or the record fences
/// it runs, if it has one.
fn refusal_code(error: &anyhow::Error) -> Option<String> {
    let message = error.to_string();
    let (code, _) = message.split_once(':')?;
    (code.chars().all(|c| c.is_ascii_lowercase() || c == '_')
        && (code.starts_with("merge_")
            || [
                "canonical_snapshot_uncommitted",
                "tracked_snapshot_uncommitted",
                "review_append_only",
            ]
            .contains(&code)))
    .then(|| code.to_owned())
}

fn attempts(issue_ref: &str, check_only: bool, json: bool, quiet: bool) -> Result<()> {
    for attempt in 1..=MAX_ATTEMPTS {
        match merge_once(issue_ref, check_only, json, quiet) {
            Err(error) => match error.downcast_ref::<RetryOnMovedBase>() {
                Some(moved) if attempt < MAX_ATTEMPTS => {
                    eprintln!("merge_base_moved: {moved}; running the gates again on the new base")
                }
                Some(moved) => bail!("merge_snapshot_changed: {moved}, and it kept moving across {MAX_ATTEMPTS} attempts. Next: rerun `atelier merge {issue_ref}` once the base is quiet"),
                None => return Err(error),
            },
            done => return done,
        }
    }
    unreachable!("the last attempt returns")
}

/// The checked-out work branch of the item's owner, read from its committed
/// records: what `merge` and `merge --sync` both start from.
pub(super) struct Owned {
    pub layout: StorageLayout,
    pub head: String,
    pub branch: String,
    pub snapshot: committed_snapshot::CommittedSnapshot,
    pub index: RecordIndex,
    pub id: String,
    pub owner: String,
    pub base: String,
}

pub(super) fn owned_branch(issue_ref: &str) -> Result<Owned> {
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    let head = git::head_commit(repo)?;
    let branch = crate::commands::git::checkout_branch(repo)?.context(
        "merge_detached_head: merge requires a local work branch. Next: check out the work branch, then retry",
    )?;
    let snapshot = committed_snapshot::CommittedSnapshot::new(&head)?;
    let index = {
        let _transaction =
            atelier_records::mutation_lock::CanonicalMutationLock::shared(&layout.canonical_dir())?;
        snapshot.read_record_files(repo, true)?.0.into_index()?
    };
    let id = index.resolve(atelier_app::record_ref::RecordKind::Issue, issue_ref)?;
    let issue = index.require_issue(&id)?;
    let owner = issue.parent_id.clone().unwrap_or_else(|| id.clone());
    let owner_record = index.require_issue(&owner)?;
    if owner_record.issue_type == "mission" {
        bail!("merge_mission: {owner} is a mission, and missions have no work branch. Next: merge one of its issues, listed by `atelier work mission {owner}`");
    }
    let recorded = owner_record.workflow_branch.as_ref().with_context(|| {
        format!("merge_owner_unstarted: owner {owner} has no committed branch ownership. Next: run `atelier start {owner}` and commit its metadata")
    })?;
    if recorded.work_branch != branch || recorded.branch_base == branch {
        bail!(
            "merge_branch_mismatch: expected work branch {} with distinct base {}",
            recorded.work_branch,
            recorded.branch_base
        );
    }
    let base = recorded.branch_base.clone();
    Ok(Owned {
        layout,
        head,
        branch,
        snapshot,
        index,
        id,
        owner,
        base,
    })
}

fn merge_once(issue_ref: &str, check_only: bool, json: bool, quiet: bool) -> Result<()> {
    let Owned {
        layout,
        head,
        branch,
        snapshot,
        index,
        id,
        owner,
        base,
    } = owned_branch(issue_ref)?;
    let repo = layout.repo_root();
    let owner = owner.as_str();
    let base = &base;
    let base_ref = format!("refs/heads/{base}");
    let mut read = git::ReadContext::from_snapshot(repo, head.clone(), &[]);
    let base_head = crate::commands::git::resolve(repo, &base_ref)?;
    let source = base_policy::from_captured(&mut read, &base_head)?;
    for diagnostic in &source.diagnostics {
        eprintln!("{diagnostic}");
    }
    let strategy = source.policy.branch_policy.merge_strategy;
    if source.policy.branch_policy.base_branch != *base
        || !matches!(
            strategy,
            MergeStrategy::FastForwardOnly | MergeStrategy::MergeCommit
        )
    {
        bail!("merge_base_policy_unsupported: merge requires base policy for {base} with merge_strategy: fast_forward_only or merge_commit. Next: review and merge the policy change on {base} first");
    }
    let tree = if strategy == MergeStrategy::MergeCommit {
        // Evidence at the branch head does not prove a tree the base has
        // changed; only checks run on the merged tree can.
        if tree_checks(&source.policy).next().is_none() {
            bail!("merge_unproven: merge_strategy merge_commit needs a shell.check in the base policy's merge_checks for {base}; it runs on the merged tree, which evidence at the branch head and record checks do not prove");
        }
        Some(merged_tree(repo, base, &base_head, &branch, &head)?)
    } else {
        git::git_output(repo, &["merge-base", "--is-ancestor", &base_head, &head]).map_err(|_| {
            let fork = text(repo, &["merge-base", &base_head, &head]).unwrap_or_default();
            let moved = match first_code_commit(repo, &fork, &base_head) {
                Ok(None) => format!("{base} gained only records-only commits since {branch} left it, but merge_strategy fast_forward_only creates no merge commit."),
                Ok(Some((commit, path))) => format!("{base} gained {}, which changes {}, not a record.", crate::human_output::short_commit(&commit), path.display()),
                Err(_) => format!("{base} is not an ancestor of {branch}."),
            };
            anyhow::anyhow!("merge_diverged: {moved} Merge it in with `git merge {base}` on {branch}, record fresh proof, then retry; a clean merge keeps the approval unless it changes an in-scope issue record beyond its status, and then review.current names the merge and the record. Do not rebase once review records exist: rebasing rewrites the commits they pin.")
        })?;
        None
    };
    // Where the branch left the base; the base itself under fast-forward.
    let fork_point = text(repo, &["merge-base", &base_head, &head])?;
    // Candidate syntax is checked only after trusted base resolution, so an
    // invalid branch cannot prevent a visible fallback or become base policy.
    atelier_workflow::load(repo).with_context(|| {
        format!("candidate workflow policy is invalid. Next: fix .atelier/workflow.yaml, commit it, then rerun `atelier merge {id}`")
    })?;
    index.validate_policy(&source.policy, repo)?;
    review_gate::append_only(&mut read, &fork_point)?;
    let base_tree = base_checkout(repo, base)?;
    let base_snapshot = base_tree
        .as_ref()
        .map(|_| committed_snapshot::CommittedSnapshot::new(&base_head))
        .transpose()?;
    if let Some(path) = &base_tree {
        base_snapshot
            .as_ref()
            .expect("base snapshot")
            .require_clean(path)?;
    }
    crate::commands::git::require_committed_config(repo, &head, "merge")?;
    let merge = tree
        .map(|tree| -> Result<_> {
            let commit = merge_commit(
                repo,
                &source.policy,
                &tree,
                (base, &base_head),
                (&branch, &head),
            )?;
            let checkout = MergedCheckout::add(repo, &commit)?;
            let merged = committed_snapshot::CommittedSnapshot::new(&commit)?
                .read_record_files(&checkout.path, true)?
                .0
                .into_index()?;
            base_records(
                repo,
                (&index, &merged),
                owner,
                (base, &branch),
                (&head, &commit),
            )?;
            Ok((tree, commit, checkout, merged))
        })
        .transpose()?;
    // The gates read the records merging would put on the base.
    let mut nodes = review_gate::evaluate(
        merge.as_ref().map_or(&index, |(_, _, _, merged)| merged),
        &mut read,
        &source.policy,
        owner,
        &fork_point,
        merge
            .as_ref()
            .map_or(repo, |(_, _, checkout, _)| checkout.path.as_path()),
    )?;
    // The commit merging would put on the base, with its records: the merge
    // commit, or the branch head itself under fast-forward.
    let (tip, tip_index) = merge
        .as_ref()
        .map_or((&head, &index), |(_, commit, _, merged)| (commit, merged));
    // The records merging puts on the base can break a done claim outside
    // the item, such as one the base completed after the fork point, which no
    // checkout held before (atelier-m4gv). Those claims are read at that
    // commit, which holds the base commits their reviews pin (atelier-goom),
    // under either strategy, as `land` there reads them (atelier-bu66).
    nodes.extend(crate::commands::session::merge_claim_nodes(
        repo,
        &mut git::ReadContext::from_snapshot(repo, tip.clone(), &[]),
        tip_index,
        &source.policy,
        owner,
        &base_head,
        tip,
    )?);
    if let Some((tree, _, _, _)) = &merge {
        // Only the shell checks examined the merged tree; it is proven when
        // at least one ran there and every one passed.
        let shell = nodes
            .iter()
            .filter(|node| node.check == "shell.check")
            .collect::<Vec<_>>();
        let passed = !shell.is_empty() && shell.iter().all(|node| node.passed);
        let commands = tree_checks(&source.policy).collect::<Vec<_>>().join("; ");
        nodes.push(review_gate::gate(
            owner,
            "merge.merged_tree",
            passed,
            if passed {
                format!("merge_checks ran on merged tree {tree} of {branch} into {base} at {base_head} and passed: {commands}; evidence at the branch head does not prove it")
            } else {
                format!("merged tree {tree} of {branch} into {base} at {base_head} is unproven: not every shell.check passed on it; shell.check names the failure")
            },
            Vec::new(),
        ));
    }
    // Every done claim merging would put on the base must be one `land`
    // there accepts, judged by the same code, so a refusal comes before
    // integration rather than after it (atelier-uw4x).
    let unreviewed = crate::commands::git::first_unreviewed_claim(
        repo,
        tip_index,
        &source.policy,
        tip,
        &base_head,
    )?;
    let mut claims_reviewed = review_gate::gate(
        owner,
        "claims.reviewed",
        unreviewed.is_none(),
        match &unreviewed {
            None => "every done claim merging adds is covered by a current accepted approval".into(),
            Some((commit, claim)) => format!(
                "commit {} marks {claim} done outside every current approval, so `land` on {base} would refuse it with land_base_done_claim",
                crate::human_output::short_commit(commit),
            ),
        },
        Vec::new(),
    );
    if let Some((commit, claim)) = &unreviewed {
        // An approval covers a done claim in its scope while it is still
        // current at the claim's commit.
        claims_reviewed.fix = Some(if review_gate::scope(tip_index, owner)?.contains(claim) {
            format!("atelier review request {owner}")
        } else {
            format!("edit .atelier/issues/{claim}.md back to its earlier status on this branch, commit, and close {claim} on its own branch with `atelier start {claim}`")
        });
        claims_reviewed = claims_reviewed.with_gaps(vec![review_gate::Gap::UnreviewedClaim {
            claim: claim.clone(),
            commit: commit.clone(),
        }]);
    }
    nodes.push(claims_reviewed);
    let gates = crate::gate_report::Gates {
        index: tip_index,
        policy: &source.policy,
        owner: Some(owner),
    }
    .report(&nodes);
    // Failures print with the passes; a passing check prints its passes and
    // next step, and a passing merge only what it did (atelier-2tux).
    let report = || -> Result<()> {
        if json {
            println!("{}", serde_json::to_string_pretty(&nodes)?);
        } else if gates.failed.is_empty() {
            if check_only && !quiet {
                for line in &gates.passes {
                    println!("{line}");
                }
                println!("Next: atelier merge {id}");
            }
        } else {
            for line in gates.failures.iter().chain(&gates.passes) {
                if quiet {
                    eprintln!("{line}");
                } else {
                    println!("{line}");
                }
            }
        }
        Ok(())
    };
    // A failed check about another item names it.
    let failed = nodes
        .iter()
        .filter(|node| !node.passed)
        .map(|node| match node.target.as_str() {
            target if target == owner => node.check.clone(),
            target => format!("{target} {}", node.check),
        })
        .collect::<Vec<_>>();
    // Fence cooperative writers only after arbitrary checks have returned.
    // Lock both checkouts in path order to avoid cross-worktree lock inversion.
    let mut state_paths = vec![layout.canonical_dir()];
    if let Some(path) = &base_tree {
        state_paths.push(path.join(".atelier"));
    }
    state_paths.sort();
    state_paths.dedup();
    #[cfg(feature = "test-hooks")]
    test_move_base_before_locks(repo, base);
    let _transactions = state_paths
        .iter()
        .map(|path| atelier_records::mutation_lock::CanonicalMutationLock::exclusive(path))
        .collect::<Result<Vec<_>>>()?;
    // The base is read under the locks, then the report prints once: a base
    // that moves by records-only commits retries without having printed, so
    // only the attempt that returns reports (atelier-1hjj).
    let moved = match crate::commands::git::resolve(repo, &base_ref) {
        Ok(current) if current == base_head => None,
        Ok(current) => Some(base_moved(repo, &id, strategy, base, (&base_head, &current))),
        Err(_) => Some(anyhow::anyhow!("merge_snapshot_changed: {base} disappeared while checks ran. Next: rerun `atelier merge {id}` with fresh proof and review")),
    };
    if let Some(moved) = moved {
        if moved.downcast_ref::<RetryOnMovedBase>().is_none() {
            report()?;
        }
        return Err(moved);
    }
    report()?;
    // Shell checks cannot authorize a different tree or leave dirty records.
    snapshot.require_clean(repo)?;
    if git::head_commit(repo)? != head
        || crate::commands::git::checkout_branch(repo)?.as_deref() != Some(branch.as_str())
        || text(
            repo,
            &["rev-parse", "--verify", &format!("refs/heads/{branch}")],
        )? != head
    {
        bail!("merge_snapshot_changed: Git refs changed while checks ran. Next: rerun `atelier merge {id}` with fresh proof and review");
    }
    if base_checkout(repo, base)? != base_tree {
        bail!("merge_snapshot_changed: base checkout ownership changed while checks ran. Next: rerun `atelier merge {id}` with fresh proof and review");
    }
    if let Some(path) = &base_tree {
        base_snapshot
            .as_ref()
            .expect("base snapshot")
            .require_clean(path)?;
        if git::head_commit(path)? != base_head
            || crate::commands::git::checkout_branch(path)?.as_deref() != Some(base.as_str())
        {
            bail!("merge_snapshot_changed: base checkout changed while checks ran. Next: rerun `atelier merge {id}` with fresh proof and review");
        }
    }
    if let Some((_, commit, checkout, _)) = &merge {
        if !text(
            &checkout.path,
            &["status", "--porcelain", "--untracked-files=all"],
        )?
        .is_empty()
            || git::head_commit(&checkout.path)? != *commit
        {
            bail!("merge_checkout_dirty: merge_checks changed the merged checkout; checks must be read-only");
        }
    }
    if !failed.is_empty() {
        if json {
            bail!("merge blocked: {}", failed.join(", "));
        }
        return Err(crate::gate_report::GatesRefused {
            report: gates,
            notes: Vec::new(),
        }
        .into());
    }
    if check_only {
        return Ok(());
    }
    let landed = match merge {
        Some((_, commit, checkout, _)) => {
            drop(checkout);
            Some(commit)
        }
        None => None,
    };
    // A separately checked-out base can be updated without touching it from
    // this worktree. Detach the source so its now-integrated branch can go.
    let target = base_tree.as_deref().unwrap_or(repo);
    if !quiet && hooks_configured(repo) {
        println!("Git hooks did not run: merge integrates with hooks disabled.");
    }
    if base_tree.is_none() {
        git::git_output(repo, &["-c", "core.hooksPath=/dev/null", "switch", base])?;
    }
    git::git_output(
        target,
        &[
            "-c",
            "core.hooksPath=/dev/null",
            "merge",
            "--ff-only",
            landed.as_deref().unwrap_or(&head),
        ],
    )
    .context("fast-forward failed; no tracker state was written")?;
    if base_tree.is_some() {
        git::git_output(
            repo,
            &[
                "-c",
                "core.hooksPath=/dev/null",
                "switch",
                "--detach",
                &head,
            ],
        )
        .context(
            "integration succeeded; detach the work checkout and delete its integrated branch",
        )?;
    }
    // Delete only the reviewed tip, atomically. Git branch -d instead tests
    // an upstream when configured, even when this base contains the full tip.
    let work_ref = format!("refs/heads/{branch}");
    git::git_output(target, &["update-ref", "--no-deref", "-d", &work_ref, &head]).with_context(|| format!("integration succeeded; cleanup refused because {branch} changed. Inspect the branch before deleting it."))?;
    let metadata = atelier_app::git_freshness::command(target)
        .args([
            "config",
            "--local",
            "--name-only",
            "--get-regexp",
            "^branch\\.",
        ])
        .output()?;
    if metadata.status.success() {
        if String::from_utf8_lossy(&metadata.stdout)
            .lines()
            .any(|key| key.starts_with(&format!("branch.{branch}.")))
        {
            git::git_output(target, &["config", "--local", "--remove-section", &format!("branch.{branch}")]).context("integration and branch deletion succeeded; remove the branch's remaining local config section")?;
        }
    } else if metadata.status.code() != Some(1) {
        bail!(
            "integration succeeded; could not inspect branch cleanup metadata: {}",
            String::from_utf8_lossy(&metadata.stderr)
        );
    }
    clean(target)?;
    clean(repo)?;
    let detached = base_tree.is_some().then(|| format!("The work checkout is detached; when finished, leave it and run `git worktree remove {}` from the base checkout.", repo.display()));
    if quiet {
        if let Some(note) = detached {
            eprintln!("{note}");
        }
        return Ok(());
    }
    println!(
        "✓ Merged {branch} into {base} {}; branch deleted. Next: atelier land, then git push.",
        match &landed {
            Some(commit) => format!(
                "with merge commit {}",
                crate::human_output::short_commit(commit)
            ),
            None => format!("at {}", crate::human_output::short_commit(&head)),
        }
    );
    if let Some(note) = detached {
        println!("{note}");
    }
    Ok(())
}

/// Whether the repository runs Git hooks of its own: a `core.hooksPath`, or
/// an executable hook in the default hooks directory.
fn hooks_configured(repo: &Path) -> bool {
    if text(repo, &["config", "--get", "core.hooksPath"]).is_ok() {
        return true;
    }
    let Ok(dir) = text(repo, &["rev-parse", "--git-path", "hooks"]) else {
        return false;
    };
    std::fs::read_dir(repo.join(dir)).is_ok_and(|entries| {
        entries.flatten().any(|entry| {
            let name = entry.file_name();
            !name.to_string_lossy().ends_with(".sample")
                && entry.metadata().is_ok_and(|metadata| {
                    use std::os::unix::fs::PermissionsExt;
                    metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
                })
        })
    })
}

#[cfg(test)]
mod tests {
    use super::discard;

    #[test]
    fn a_leftover_directory_already_gone_counts_as_removed() {
        let root = tempfile::tempdir().unwrap();
        let leftover = root.path().join("atelier-merge-gone");
        assert!(discard(&leftover).is_ok());
        std::fs::create_dir(&leftover).unwrap();
        assert!(discard(&leftover).is_ok() && !leftover.exists());
    }
}
