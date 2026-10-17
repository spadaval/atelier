//! Gated fast-forward integration. Creates no commits or tracker records.
use crate::commands::git::text;
use anyhow::{bail, Context, Result};
use atelier_app::{
    base_policy, committed_snapshot, git_freshness as git, review_gate,
    storage_layout::StorageLayout,
};
use std::path::{Path, PathBuf};

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
    git::git_output(repo, &["merge-base", "--is-ancestor", &base_head, &head]).map_err(|_| anyhow::anyhow!("merge_diverged: {base} is not an ancestor of {branch}. Merge it in with `git merge {base}` on {branch}, record fresh proof, then retry; a clean merge keeps the approval. Do not rebase once review records exist: rebasing rewrites the commits they pin."))?;
    let source = base_policy::from_captured(&mut read, &base_head)?;
    for diagnostic in &source.diagnostics {
        eprintln!("{diagnostic}");
    }
    if source.policy.branch_policy.base_branch != *base
        || source.policy.branch_policy.merge_strategy
            != atelier_app::workflow_policy::MergeStrategy::FastForwardOnly
    {
        bail!("merge requires base policy for {base} with merge_strategy: fast_forward_only; review the policy change first");
    }
    // Candidate syntax is checked only after trusted base resolution, so an
    // invalid branch cannot prevent a visible fallback or become base policy.
    atelier_workflow::load(repo)
        .context("candidate workflow policy is invalid; fix it before merging")?;
    index.validate_policy(&source.policy, repo)?;
    review_gate::append_only(&mut read, &base_head)?;
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
    let nodes = review_gate::evaluate(&index, &mut read, &source.policy, owner, &base_head)?;
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
    if !failed.is_empty() {
        bail!("merge blocked: {}", failed.join(", "));
    }
    if check_only {
        return Ok(());
    }
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
            &head,
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
    println!("Merged {branch} into {base} at {}; deleted local work branch. No commits or tracker records were created.", crate::human_output::short_commit(&head));
    if base_tree.is_some() {
        println!("The work checkout is detached; when finished, leave it and run git worktree remove {} from the base checkout.", repo.display());
    }
    Ok(())
}
