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
use std::path::{Path, PathBuf};

/// A detached checkout of the merge commit, outside every working tree, so
/// `merge_checks` run on exactly the tree merging would put on the base.
struct MergedCheckout {
    repo: PathBuf,
    path: PathBuf,
    _dir: tempfile::TempDir,
}
impl MergedCheckout {
    fn add(repo: &Path, commit: &str) -> Result<Self> {
        let common = text(
            repo,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?;
        let dir = tempfile::Builder::new()
            .prefix("atelier-merge-")
            .tempdir_in(common)?;
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
        })
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

/// The merge commit object; it reaches the base only if its tree passes.
fn merge_commit(
    repo: &Path,
    policy: &WorkflowPolicy,
    tree: &str,
    (base, base_head): (&str, &str),
    (branch, head): (&str, &str),
) -> Result<String> {
    let checks = policy
        .merge_checks
        .iter()
        .map(|check| match &check.params {
            Some(ValidatorParams::ShellCheck { command }) => {
                format!("- {}: {command}", check.builtin)
            }
            _ => format!("- {}", check.builtin),
        })
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
            "merge_base_records: {base} changed records about {owner} since {branch} left it: {}. The merge commit would carry them onto {base} unchecked. Merge it in with `git merge {base}` on {branch} so the gates judge them, then retry; a clean merge keeps the approval.",
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
            if block.lines().any(|line| line.starts_with("locked")) {
                bail!("base worktree is locked; unlock it before merging");
            }
            return Ok(block
                .lines()
                .find_map(|line| line.strip_prefix("worktree "))
                .map(PathBuf::from));
        }
    }
    Ok(None)
}

pub fn run(issue_ref: &str, check_only: bool, json: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    let head = git::head_commit(repo)?;
    let branch = crate::commands::git::checkout_branch(repo)?
        .context("merge requires a local work branch")?;
    let snapshot = committed_snapshot::CommittedSnapshot::new(&head)?;
    let index = {
        let _transaction =
            atelier_records::mutation_lock::CanonicalMutationLock::shared(&layout.canonical_dir())?;
        snapshot.read_record_files(repo, true)?.0.into_index()?
    };
    let id = index
        .resolve_issue_ref(issue_ref)?
        .ok_or_else(|| anyhow::anyhow!("issue {issue_ref} not found"))?;
    let issue = index.require_issue(&id)?;
    let owner = issue.parent_id.as_deref().unwrap_or(&id);
    let owner_record = index.require_issue(owner)?;
    if owner_record.issue_type == "mission" {
        bail!("missions have no work branch; merge one of their issues");
    }
    let recorded = owner_record.workflow_branch.as_ref().context(
        "merge requires committed branch ownership; run atelier start and commit its metadata",
    )?;
    if recorded.work_branch != branch || recorded.branch_base == branch {
        bail!(
            "merge_branch_mismatch: expected work branch {} with distinct base {}",
            recorded.work_branch,
            recorded.branch_base
        );
    }
    let base = &recorded.branch_base;
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
        bail!("merge requires base policy for {base} with merge_strategy: fast_forward_only or merge_commit; review the policy change first");
    }
    let tree = if strategy == MergeStrategy::MergeCommit {
        // Evidence at the branch head does not prove a tree the base has
        // changed; only checks run on the merged tree can.
        if source.policy.merge_checks.is_empty() {
            bail!("merge_unproven: merge_strategy merge_commit needs merge_checks in the base policy for {base}; they run on the merged tree, which evidence at the branch head does not prove");
        }
        Some(merged_tree(repo, base, &base_head, &branch, &head)?)
    } else {
        git::git_output(repo, &["merge-base", "--is-ancestor", &base_head, &head]).map_err(|_| anyhow::anyhow!("merge_diverged: {base} is not an ancestor of {branch}. Merge it in with `git merge {base}` on {branch}, record fresh proof, then retry; a clean merge keeps the approval. Do not rebase once review records exist: rebasing rewrites the commits they pin."))?;
        None
    };
    // Where the branch left the base; the base itself under fast-forward.
    let fork_point = text(repo, &["merge-base", &base_head, &head])?;
    // Candidate syntax is checked only after trusted base resolution, so an
    // invalid branch cannot prevent a visible fallback or become base policy.
    atelier_workflow::load(repo)
        .context("candidate workflow policy is invalid; fix it before merging")?;
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
    if let Some((tree, _, _, _)) = &merge {
        nodes.push(review_gate::GateNode {
            kind: "workflow_validator",
            id: format!("{owner}/merge.merged_tree"),
            target: owner.into(),
            check: "merge.merged_tree".into(),
            passed: true,
            reason: format!("merge_checks ran on merged tree {tree} of {branch} into {base} at {base_head}; evidence at the branch head does not prove it"),
            supports: Vec::new(),
            stale_from_checkout: false,
        });
    }
    if json {
        println!("{}", serde_json::to_string_pretty(&nodes)?);
    } else {
        for node in &nodes {
            println!(
                "{} {}: {}",
                if node.passed { "pass" } else { "FAIL" },
                node.check,
                node.reason
            );
        }
    }
    let failed = nodes
        .iter()
        .filter(|node| !node.passed)
        .map(|node| node.check.as_str())
        .collect::<Vec<_>>();
    // Fence cooperative writers only after arbitrary checks have returned.
    // Lock both checkouts in path order to avoid cross-worktree lock inversion.
    let mut state_paths = vec![layout.canonical_dir()];
    if let Some(path) = &base_tree {
        state_paths.push(path.join(".atelier"));
    }
    state_paths.sort();
    state_paths.dedup();
    let _transactions = state_paths
        .iter()
        .map(|path| atelier_records::mutation_lock::CanonicalMutationLock::exclusive(path))
        .collect::<Result<Vec<_>>>()?;
    // Shell checks cannot authorize a different tree or leave dirty records.
    snapshot.require_clean(repo)?;
    if git::head_commit(repo)? != head
        || crate::commands::git::checkout_branch(repo)?.as_deref() != Some(branch.as_str())
        || text(
            repo,
            &["rev-parse", "--verify", &format!("refs/heads/{branch}")],
        )? != head
        || crate::commands::git::resolve(repo, &base_ref)
            .map_or(true, |current| current != base_head)
    {
        bail!("merge_snapshot_changed: Git refs changed while checks ran; retry with fresh proof and review");
    }
    if base_checkout(repo, base)? != base_tree {
        bail!("base checkout ownership changed while checks ran");
    }
    if let Some(path) = &base_tree {
        base_snapshot
            .as_ref()
            .expect("base snapshot")
            .require_clean(path)?;
        if git::head_commit(path)? != base_head
            || crate::commands::git::checkout_branch(path)?.as_deref() != Some(base.as_str())
        {
            bail!("base checkout changed while checks ran");
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
        bail!("merge blocked: {}", failed.join(", "));
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
    println!("Git hooks are disabled during gated integration and cleanup; put required read-only checks in the base policy's merge_checks.");
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
    match &landed {
        Some(commit) => println!("Merged {branch} into {base} with merge commit {commit}; deleted local work branch. No other commits or tracker records were created."),
        None => println!("Merged {branch} into {base} at {head}; deleted local work branch. No commits or tracker records were created."),
    }
    if base_tree.is_some() {
        println!("The work checkout is detached; when finished, leave it and run git worktree remove {} from the base checkout.", repo.display());
    }
    Ok(())
}
