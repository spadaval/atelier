use crate::human_output::{local_date, print_footer, FooterAction};
use anyhow::{bail, Context, Result};
use atelier_app::record_index::RecordIndex;
use atelier_app::workflow_policy::MergeStrategy;
use atelier_core::{Issue, RecordLink, Relationships};
use chrono::{DateTime, Days, NaiveDate, Utc};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

pub struct TrackerContext {
    pub index: RecordIndex,
    pub repo_root: PathBuf,
    pub state_dir: PathBuf,
    pub canonical_retention_days: u64,
}

#[derive(Debug, Clone)]
struct CanonicalPruneSummary {
    retention_days: u64,
    cutoff: NaiveDate,
    issues: Vec<CanonicalCandidate>,
    evidence: Vec<CanonicalCandidate>,
    reviews: Vec<CanonicalCandidate>,
    removed: Vec<CanonicalRemoval>,
}

#[derive(Debug, Clone)]
struct CanonicalCandidate {
    kind: &'static str,
    id: String,
    title: String,
    status: String,
    path: PathBuf,
    latest_at: DateTime<Utc>,
    protection: Option<String>,
}

#[derive(Debug, Clone)]
struct CanonicalRemoval {
    kind: &'static str,
    id: String,
}

#[derive(Debug, Clone)]
struct GitCandidate {
    class: &'static str,
    label: String,
    path: Option<PathBuf>,
    protection: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct GitPruneSummary {
    candidates: Vec<GitCandidate>,
    removed: Vec<(String, String)>,
    failures: Vec<(String, String)>,
    unavailable: Option<String>,
}

#[derive(Debug, Clone)]
struct GitWorktree {
    path: PathBuf,
    branch: Option<String>,
    locked: bool,
    prunable: bool,
}

#[derive(Debug, Clone)]
struct GitBranchOwner {
    id: String,
    base: String,
    merge_strategy: MergeStrategy,
    protection: Option<String>,
    active_descendants: BTreeSet<String>,
}

impl CanonicalCandidate {
    fn eligible(&self) -> bool {
        self.protection.is_none()
    }
}

pub fn run(
    tracker: TrackerContext,
    apply: bool,
    retention_days: Option<u64>,
    quiet: bool,
) -> Result<()> {
    // Plan and validate canonical removal before any Git cleanup, so a
    // refused canonical prune never follows branch or worktree removal.
    let head_paths = canonical_paths_at_head(&tracker.repo_root)?;
    let mut canonical = plan_canonical_records(&tracker, retention_days, apply, &head_paths)?;
    // Git cleanup runs before record removal: it treats an owner whose record
    // file is dirty as protected, and removing records would make them dirty.
    let git = prune_git_artifacts(&tracker, retention_days, apply, &head_paths)?;
    if apply {
        if let Err(error) = remove_canonical_records(&tracker.state_dir, &mut canonical) {
            bail!("{error:#}\n{}.", git_removals_note(&git));
        }
    }

    if quiet {
        print_quiet_summary(&git, &canonical, apply);
        return Ok(());
    }

    println!("Prune");
    println!("=====");
    println!("Mode: {}", if apply { "apply" } else { "dry-run" });
    println!();

    print_git(&git, apply);
    println!();
    print_canonical(&canonical);
    println!();
    print_deferred_classes();

    if !apply {
        print_footer([FooterAction::new(
            "Apply the prune",
            "atelier prune --apply",
        )]);
    }

    Ok(())
}

fn prune_git_artifacts(
    tracker: &TrackerContext,
    retention_days_override: Option<u64>,
    apply: bool,
    head_paths: &BTreeSet<PathBuf>,
) -> Result<GitPruneSummary> {
    let git_repository = atelier_app::git_freshness::command(&tracker.repo_root)
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .context("failed to inspect Git repository for prune")?;
    if !git_repository.status.success() {
        return Ok(GitPruneSummary {
            unavailable: Some("not a Git checkout; Git cleanup is unavailable".to_string()),
            ..Default::default()
        });
    }
    let policy = match atelier_app::workflow_policy::load(&tracker.repo_root) {
        Ok(policy) => policy,
        Err(error) => {
            return Ok(GitPruneSummary {
                unavailable: Some(format!(
                    "workflow policy unavailable; Git cleanup is protected ({error})"
                )),
                ..Default::default()
            })
        }
    };
    let base = policy.branch_policy.base_branch.clone();
    let current = git_stdout_trimmed(&tracker.repo_root, &["branch", "--show-current"])?;
    let worktrees = git_worktrees(&tracker.repo_root)?;
    let branches = git_stdout_trimmed(
        &tracker.repo_root,
        &["for-each-ref", "--format=%(refname)", "refs/heads"],
    )?;
    let (_, cutoff) = canonical_retention_cutoff(tracker, retention_days_override);
    let mut owner_candidates = issue_candidates(tracker, cutoff);
    apply_git_history_protection(head_paths, &mut owner_candidates);
    let dirty_paths = dirty_canonical_paths(&tracker.repo_root)?;
    for candidate in owner_candidates
        .iter_mut()
        .filter(|candidate| candidate.eligible())
    {
        if dirty_paths.contains(&PathBuf::from(display_git_path(&candidate.path))) {
            candidate.protection = Some("owner record has uncommitted changes".to_string());
        }
    }
    let owner_candidates = owner_candidates
        .into_iter()
        .map(|candidate| (candidate.id.clone(), candidate))
        .collect::<BTreeMap<_, _>>();
    let all_issues = tracker.index.issues().collect::<Vec<_>>();
    let mut branch_owners = BTreeMap::<String, Vec<GitBranchOwner>>::new();
    for issue in &all_issues {
        if let Ok(resolution) =
            atelier_app::workflow_policy::resolve_work_branch(&policy, &tracker.index, &issue.id)
        {
            if resolution.owner_id == issue.id {
                let protection =
                    if !crate::commands::issue_workflow::issue_is_done(Some(&policy), issue) {
                        Some(format!("owner {} has active workflow state", issue.id))
                    } else if let Some(candidate) = owner_candidates.get(&issue.id) {
                        candidate.protection.as_ref().map(|reason| {
                            format!("terminal owner {} is protected: {reason}", issue.id)
                        })
                    } else {
                        Some(format!(
                            "terminal owner {} is within retention window",
                            issue.id
                        ))
                    };
                branch_owners
                    .entry(resolution.branch)
                    .or_default()
                    .push(GitBranchOwner {
                        id: issue.id.clone(),
                        base: resolution
                            .recorded_base
                            .unwrap_or_else(|| policy.branch_policy.base_branch.clone()),
                        merge_strategy: policy.branch_policy.merge_strategy,
                        protection,
                        active_descendants: BTreeSet::new(),
                    });
            }
        }
    }
    for issue in &all_issues {
        if crate::commands::issue_workflow::issue_is_done(Some(&policy), issue) {
            continue;
        }
        let Ok(resolution) =
            atelier_app::workflow_policy::resolve_work_branch(&policy, &tracker.index, &issue.id)
        else {
            continue;
        };
        if resolution.owner_id == issue.id {
            continue;
        }
        if let Some(owner) = branch_owners
            .get_mut(&resolution.branch)
            .and_then(|owners| {
                owners
                    .iter_mut()
                    .find(|owner| owner.id == resolution.owner_id)
            })
        {
            owner.active_descendants.insert(issue.id.clone());
        }
    }
    let mut worktree_branch_blockers = BTreeMap::new();
    for worktree in &worktrees {
        if worktree.prunable || worktree.branch.is_none() {
            continue;
        }
        let reason = if worktree.path == tracker.repo_root {
            Some("current checkout".to_string())
        } else if worktree.locked {
            Some("locked worktree".to_string())
        } else if !git_stdout_trimmed(&worktree.path, &["status", "--porcelain"])?.is_empty() {
            Some("checked out by a dirty worktree".to_string())
        } else {
            None
        };
        if let Some(reason) = reason {
            worktree_branch_blockers
                .insert(worktree.branch.clone().expect("checked above"), reason);
        }
    }
    let mut summary = GitPruneSummary::default();
    for branch in branches
        .lines()
        .filter_map(|branch| branch.strip_prefix("refs/heads/"))
    {
        let protection = if branch == current {
            Some("current checkout".to_string())
        } else if branch == base {
            Some("configured base branch".to_string())
        } else if let Some(reason) = worktree_branch_blockers.get(branch) {
            Some(reason.clone())
        } else if let Some(owners) = branch_owners.get(branch) {
            git_owner_protection(&tracker.repo_root, branch, owners)?
        } else {
            Some("no terminal owner record association".to_string())
        };
        summary.candidates.push(GitCandidate {
            class: "branch",
            label: branch.to_string(),
            path: None,
            protection,
        });
    }
    for worktree in &worktrees {
        if worktree.prunable {
            summary.candidates.push(GitCandidate {
                class: "worktree-registration",
                label: worktree
                    .branch
                    .clone()
                    .unwrap_or_else(|| "detached HEAD".to_string()),
                path: Some(worktree.path.clone()),
                protection: if worktree.locked {
                    Some("locked stale worktree registration".to_string())
                } else {
                    None
                },
            });
            continue;
        }
        let is_current = worktree.path == tracker.repo_root;
        let protection = if is_current {
            Some("current checkout".to_string())
        } else if worktree.locked {
            Some("locked worktree".to_string())
        } else if !git_stdout_trimmed(&worktree.path, &["status", "--porcelain"])?.is_empty() {
            Some("dirty worktree".to_string())
        } else if let Some(branch) = &worktree.branch {
            if branch == &base {
                Some("configured base branch".to_string())
            } else if let Some(owners) = branch_owners.get(branch) {
                git_owner_protection(&tracker.repo_root, branch, owners)?.map(|reason| {
                    if reason.starts_with("contains commits not integrated") {
                        format!("branch {reason}")
                    } else {
                        reason
                    }
                })
            } else {
                Some("no terminal owner record association".to_string())
            }
        } else {
            Some("detached worktree has no safely removable owner branch".to_string())
        };
        summary.candidates.push(GitCandidate {
            class: "worktree",
            label: worktree
                .branch
                .clone()
                .unwrap_or_else(|| "detached HEAD".to_string()),
            path: Some(worktree.path.clone()),
            protection,
        });
    }
    summary.candidates.sort_by(|left, right| {
        (left.class, &left.label, &left.path).cmp(&(right.class, &right.label, &right.path))
    });
    if apply {
        let prune_registrations = summary.candidates.iter().any(|candidate| {
            candidate.class == "worktree-registration" && candidate.protection.is_none()
        });
        if prune_registrations {
            match git_run(&tracker.repo_root, &["worktree", "prune"]) {
                Ok(()) => {
                    for candidate in summary.candidates.iter().filter(|candidate| {
                        candidate.class == "worktree-registration" && candidate.protection.is_none()
                    }) {
                        let path = candidate.path.as_ref().expect("worktree registration path");
                        summary
                            .removed
                            .push((candidate.class.to_string(), path.display().to_string()));
                    }
                }
                Err(error) => summary
                    .failures
                    .push(("worktree registrations".to_string(), error.to_string())),
            }
        }
        // Worktrees must go first; Git refuses to delete their checked-out branch.
        for candidate in summary
            .candidates
            .iter()
            .filter(|candidate| candidate.class == "worktree" && candidate.protection.is_none())
        {
            let path = candidate.path.as_ref().expect("worktree candidate path");
            match git_run(
                &tracker.repo_root,
                &["worktree", "remove", path.to_string_lossy().as_ref()],
            ) {
                Ok(()) => summary
                    .removed
                    .push((candidate.class.to_string(), path.display().to_string())),
                Err(error) => summary
                    .failures
                    .push((path.display().to_string(), error.to_string())),
            }
        }
        for candidate in summary
            .candidates
            .iter()
            .filter(|candidate| candidate.class == "branch" && candidate.protection.is_none())
        {
            match git_run(&tracker.repo_root, &["branch", "-d", &candidate.label]) {
                Ok(()) => summary
                    .removed
                    .push((candidate.class.to_string(), candidate.label.clone())),
                Err(error) => summary
                    .failures
                    .push((candidate.label.clone(), error.to_string())),
            }
        }
    }
    Ok(summary)
}

fn git_owner_protection(
    repo_root: &Path,
    branch: &str,
    owners: &[GitBranchOwner],
) -> Result<Option<String>> {
    if owners.len() != 1 {
        let ids = owners
            .iter()
            .map(|owner| owner.id.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        return Ok(Some(format!("ambiguous owner record associations: {ids}")));
    }
    let owner = &owners[0];
    if !owner.active_descendants.is_empty() {
        return Ok(Some(format!(
            "owner {} has active descendant {}",
            owner.id,
            owner
                .active_descendants
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }
    if let Some(reason) = &owner.protection {
        return Ok(Some(reason.clone()));
    }
    if !branch_is_integrated(repo_root, branch, &owner.base, owner.merge_strategy)? {
        return Ok(Some(format!(
            "contains commits not integrated into owner base {}",
            owner.base
        )));
    }
    branch_push_protection(repo_root, branch)
}

fn git_worktrees(repo_root: &Path) -> Result<Vec<GitWorktree>> {
    let output = git_stdout_trimmed(repo_root, &["worktree", "list", "--porcelain"])?;
    let mut worktrees = Vec::new();
    let mut path = None;
    let mut branch = None;
    let mut locked = false;
    let mut prunable = false;
    let finish = |worktrees: &mut Vec<GitWorktree>,
                  path: &mut Option<PathBuf>,
                  branch: &mut Option<String>,
                  locked: &mut bool,
                  prunable: &mut bool| {
        if let Some(path) = path.take() {
            worktrees.push(GitWorktree {
                path,
                branch: branch.take(),
                locked: std::mem::take(locked),
                prunable: std::mem::take(prunable),
            });
        }
    };
    for line in output.lines().chain(std::iter::once("")) {
        if line.is_empty() {
            finish(
                &mut worktrees,
                &mut path,
                &mut branch,
                &mut locked,
                &mut prunable,
            );
        } else if let Some(value) = line.strip_prefix("worktree ") {
            path = Some(PathBuf::from(value));
        } else if let Some(value) = line.strip_prefix("branch refs/heads/") {
            branch = Some(value.to_string());
        } else if line.starts_with("locked") {
            locked = true;
        } else if line.starts_with("prunable") {
            prunable = true;
        }
    }
    Ok(worktrees)
}

fn branch_is_integrated(
    repo_root: &Path,
    branch: &str,
    base: &str,
    merge_strategy: MergeStrategy,
) -> Result<bool> {
    let output = atelier_app::git_freshness::command(repo_root)
        .args([
            "merge-base",
            "--is-ancestor",
            &format!("refs/heads/{branch}"),
            &format!("refs/heads/{base}"),
        ])
        .output()
        .context("failed to inspect Git branch integration for prune candidate")?;
    if output.status.success() {
        return Ok(true);
    }
    if merge_strategy != MergeStrategy::Squash {
        return Ok(false);
    }
    branch_has_equivalent_squash(repo_root, branch, base)
}

fn branch_has_equivalent_squash(repo_root: &Path, branch: &str, base: &str) -> Result<bool> {
    let branch_ref = format!("refs/heads/{branch}");
    let base_ref = format!("refs/heads/{base}");
    let merge_base = git_stdout_trimmed(repo_root, &["merge-base", &branch_ref, &base_ref])?;
    let branch_diff = git_diff(repo_root, &merge_base, &branch_ref)?;
    if branch_diff.is_empty() {
        return Ok(false);
    }
    let output = atelier_app::git_freshness::command(repo_root)
        .args([
            "log",
            "-z",
            "--format=%H%x00%s",
            &format!("{merge_base}..{base_ref}"),
        ])
        .output()
        .context("failed to inspect squash integration history for prune candidate")?;
    if !output.status.success() {
        return Ok(false);
    }
    let fields = output.stdout.split(|byte| *byte == 0).collect::<Vec<_>>();
    let expected_subject = format!("Squash merge {branch} into {base}");
    for pair in fields.chunks(2) {
        let [commit, subject] = pair else {
            continue;
        };
        if String::from_utf8_lossy(subject) != expected_subject {
            continue;
        }
        let commit = String::from_utf8_lossy(commit);
        let parent = match git_stdout_trimmed(repo_root, &["rev-parse", &format!("{commit}^")]) {
            Ok(parent) => parent,
            Err(_) => continue,
        };
        if git_diff(repo_root, &parent, &commit)? == branch_diff {
            return Ok(true);
        }
    }
    Ok(false)
}

fn git_diff(repo_root: &Path, from: &str, to: &str) -> Result<Vec<u8>> {
    let output = atelier_app::git_freshness::command(repo_root)
        .args(["diff", "--binary", "--full-index", "--no-renames", from, to])
        .output()
        .context("failed to compare Git integration patches for prune candidate")?;
    if !output.status.success() {
        bail!("git diff failed while inspecting prune candidate integration");
    }
    Ok(output.stdout)
}

fn branch_push_protection(repo_root: &Path, branch: &str) -> Result<Option<String>> {
    let upstream = atelier_app::git_freshness::command(repo_root)
        .args([
            "for-each-ref",
            "--format=%(upstream)",
            &format!("refs/heads/{branch}"),
        ])
        .output()
        .context("failed to inspect Git upstream for prune candidate")?;
    if !upstream.status.success() || upstream.stdout.iter().all(u8::is_ascii_whitespace) {
        return Ok(Some(
            "has no upstream; push or preserve it before pruning".to_string(),
        ));
    }
    let upstream = String::from_utf8_lossy(&upstream.stdout).trim().to_string();
    let unpushed = git_stdout_trimmed(
        repo_root,
        &["rev-list", &format!("{upstream}..refs/heads/{branch}")],
    )?;
    if unpushed.is_empty() {
        Ok(None)
    } else {
        let label = upstream
            .strip_prefix("refs/remotes/")
            .or_else(|| upstream.strip_prefix("refs/heads/"))
            .unwrap_or(&upstream);
        Ok(Some(format!(
            "contains commits not present in upstream {label}"
        )))
    }
}

fn git_stdout_trimmed(repo_root: &Path, args: &[&str]) -> Result<String> {
    let output = atelier_app::git_freshness::command(repo_root)
        .args(args)
        .output()
        .context("failed to inspect Git state for prune")?;
    if !output.status.success() {
        bail!(
            "git {} failed while inspecting prune candidates",
            args.join(" ")
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn git_run(repo_root: &Path, args: &[&str]) -> Result<()> {
    let output = atelier_app::git_freshness::command(repo_root)
        .args(args)
        .output()
        .context("failed to remove Git prune candidate")?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

/// Finds canonical candidates and, for `--apply`, checks that removing the
/// eligible ones is safe: a clean checkout and surviving records that load.
fn plan_canonical_records(
    tracker: &TrackerContext,
    retention_days_override: Option<u64>,
    apply: bool,
    head_paths: &BTreeSet<PathBuf>,
) -> Result<CanonicalPruneSummary> {
    let (retention_days, cutoff) = canonical_retention_cutoff(tracker, retention_days_override);
    let mut issues = issue_candidates(tracker, cutoff);
    let mut evidence = evidence_candidates(tracker, cutoff, &issues);
    apply_git_history_protection(head_paths, &mut issues);
    apply_git_history_protection(head_paths, &mut evidence);
    let mut reviews = review_candidates(tracker, cutoff);
    apply_git_history_protection(head_paths, &mut reviews);
    protect_referenced_candidates(&tracker.index, &mut issues, &mut evidence, &mut reviews);

    let has_eligible = issues.iter().any(CanonicalCandidate::eligible)
        || evidence.iter().any(CanonicalCandidate::eligible)
        || reviews.iter().any(CanonicalCandidate::eligible);
    if apply && has_eligible {
        ensure_clean_for_canonical_prune(&tracker.repo_root)?;
        ensure_survivors_load(tracker, &issues, &evidence, &reviews)?;
    }

    Ok(CanonicalPruneSummary {
        retention_days,
        cutoff,
        issues,
        evidence,
        reviews,
        removed: Vec::new(),
    })
}

/// Names the Git artifacts already removed in this run, which
/// `git restore` cannot bring back.
fn git_removals_note(git: &GitPruneSummary) -> String {
    let mut note = if git.removed.is_empty() {
        "Git cleanup removed nothing in this run".to_string()
    } else {
        format!(
            "Git cleanup already removed {}; `git restore` does not restore branches or worktrees",
            git.removed
                .iter()
                .map(|(class, label)| format!("{class} {label}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    if !git.failures.is_empty() {
        note.push_str(&format!(
            "; Git cleanup failures: {}",
            git.failures
                .iter()
                .map(|(target, error)| format!("{target} ({error})"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    note
}

/// Removes eligible candidates, stopping at the first failure. The checkout
/// was clean before removal, so `git restore .atelier` undoes a partial run.
fn remove_canonical_records(state_dir: &Path, summary: &mut CanonicalPruneSummary) -> Result<()> {
    let eligible = summary
        .reviews
        .iter()
        .chain(summary.evidence.iter())
        .chain(summary.issues.iter())
        .filter(|candidate| candidate.eligible())
        .collect::<Vec<_>>();
    let runtime = state_dir.join("runtime");
    fs::create_dir_all(&runtime)?;
    let staged = tempfile::tempdir_in(&runtime)
        .context("failed to create reversible prune staging directory")?;
    let mut moved: Vec<(&CanonicalCandidate, PathBuf)> = Vec::new();
    for (position, candidate) in eligible.iter().enumerate() {
        let target = staged.path().join(position.to_string());
        #[cfg(feature = "test-hooks")]
        let result = if std::env::var("ATELIER_TEST_PRUNE_REMOVE_FAILURE").as_deref()
            == Ok(candidate.id.as_str())
        {
            Err(anyhow::anyhow!("injected prune removal failure"))
        } else {
            fs::rename(state_dir.join(&candidate.path), &target).map_err(Into::into)
        };
        #[cfg(not(feature = "test-hooks"))]
        let result: Result<()> =
            fs::rename(state_dir.join(&candidate.path), &target).map_err(Into::into);
        if let Err(error) = result {
            let mut recovery = Vec::new();
            for (original, staged_path) in moved.iter().rev() {
                if let Err(restore) = fs::rename(staged_path, state_dir.join(&original.path)) {
                    recovery.push(format!("{}: {restore}", original.path.display()));
                }
            }
            if recovery.is_empty() {
                bail!("prune --apply stopped: failed to remove {}: {error:#}; every staged record was restored; no canonical records removed. Next: fix the filesystem failure and rerun atelier prune.", display_git_path(&candidate.path));
            }
            let recovery_dir = staged.keep();
            bail!("prune rollback failed: {}. Record backups remain in {}. Next: restore these paths before any tracker command.", recovery.join("; "), recovery_dir.display());
        }
        moved.push((candidate, target));
    }
    // Publication is all-or-restored under the command's exclusive lock;
    // staged files can now be discarded without exposing an invalid graph.
    summary
        .removed
        .extend(eligible.into_iter().map(|candidate| CanonicalRemoval {
            kind: candidate.kind,
            id: candidate.id.clone(),
        }));
    Ok(())
}

/// Every relationship a record declares, as `(target kind, target id, role)`.
fn declared_references(relationships: &Relationships) -> Vec<(&str, &str, &str)> {
    let mut references = Vec::new();
    for (role, targets) in [
        ("blocks", &relationships.blocks),
        ("children", &relationships.children),
    ] {
        references.extend(
            targets
                .iter()
                .map(|target| (target.kind.as_str(), target.id.as_str(), role)),
        );
    }
    references.extend(relationships.attachments.iter().map(|attachment| {
        (
            attachment.kind.as_str(),
            attachment.id.as_str(),
            attachment.role.as_str(),
        )
    }));
    references.extend(relationships.relates.iter().map(|relation| {
        (
            relation.kind.as_str(),
            relation.id.as_str(),
            relation.relation_type.as_str(),
        )
    }));
    references
}

/// Protects every eligible candidate that a kept record references, or that
/// references a kept record, through any relationship. Protecting one
/// candidate keeps it, so this repeats until nothing changes.
fn protect_referenced_candidates(
    index: &RecordIndex,
    issues: &mut [CanonicalCandidate],
    evidence: &mut [CanonicalCandidate],
    reviews: &mut [CanonicalCandidate],
) {
    let mut edges = Vec::new();
    for record in index.issue_records() {
        for (kind, id, role) in declared_references(&record.relationships) {
            edges.push((("issue", record.issue.id.as_str()), (kind, id), role));
        }
    }
    for record in index.evidence_records() {
        for (kind, id, role) in declared_references(&record.header.relationships) {
            edges.push((("evidence", record.header.id.as_str()), (kind, id), role));
        }
    }
    for record in index.reviews() {
        for (kind, id, role) in declared_references(&record.header.relationships) {
            edges.push((("review", record.header.id.as_str()), (kind, id), role));
        }
    }
    loop {
        let eligible = issues
            .iter()
            .map(|candidate| ("issue", candidate))
            .chain(evidence.iter().map(|candidate| ("evidence", candidate)))
            .chain(reviews.iter().map(|candidate| ("review", candidate)))
            .filter(|(_, candidate)| candidate.eligible())
            .map(|(kind, candidate)| (kind, candidate.id.clone()))
            .collect::<BTreeSet<_>>();
        let is_eligible = |(kind, id): (&str, &str)| eligible.contains(&(kind, id.to_string()));
        let mut reasons = BTreeMap::new();
        for (source, target, role) in &edges {
            match (is_eligible(*source), is_eligible(*target)) {
                (true, false) => {
                    reasons
                        .entry((source.0, source.1.to_string()))
                        .or_insert_with(|| {
                            format!("references retained {} {} ({role})", target.0, target.1)
                        });
                }
                (false, true) => {
                    reasons
                        .entry((target.0, target.1.to_string()))
                        .or_insert_with(|| {
                            format!("referenced by retained {} {} ({role})", source.0, source.1)
                        });
                }
                _ => {}
            }
        }
        if reasons.is_empty() {
            return;
        }
        for (kind, candidates) in [
            ("issue", &mut *issues),
            ("evidence", &mut *evidence),
            ("review", &mut *reviews),
        ] {
            for candidate in candidates.iter_mut() {
                if let Some(reason) = reasons.remove(&(kind, candidate.id.clone())) {
                    candidate.protection = Some(reason);
                }
            }
        }
    }
}

/// Refuses an apply whose surviving records would not load.
fn ensure_survivors_load(
    tracker: &TrackerContext,
    issues: &[CanonicalCandidate],
    evidence: &[CanonicalCandidate],
    reviews: &[CanonicalCandidate],
) -> Result<()> {
    let removed = |candidates: &[CanonicalCandidate], id: &str| {
        candidates
            .iter()
            .any(|candidate| candidate.eligible() && candidate.id == id)
    };
    let surviving_issues = tracker
        .index
        .issue_records()
        .filter(|record| !removed(issues, &record.issue.id))
        .cloned()
        .collect();
    let surviving_evidence = tracker
        .index
        .evidence_records()
        .into_iter()
        .filter(|record| !removed(evidence, &record.header.id))
        .cloned()
        .collect();
    RecordIndex::from_project_records(
        &tracker.state_dir,
        surviving_issues,
        surviving_evidence,
        tracker
            .index
            .reviews()
            .filter(|record| !removed(reviews, &record.header.id))
            .cloned()
            .collect(),
    )
    .map(|_| ())
    .context("prune refused: the records left after pruning would not load; nothing was removed")
}

fn canonical_retention_cutoff(
    tracker: &TrackerContext,
    retention_days_override: Option<u64>,
) -> (u64, NaiveDate) {
    let retention_days = retention_days_override.unwrap_or(tracker.canonical_retention_days);
    let today = local_date(Utc::now());
    let cutoff = today
        .checked_sub_days(Days::new(retention_days))
        .unwrap_or(today);
    (retention_days, cutoff)
}

fn issue_candidates(tracker: &TrackerContext, cutoff: NaiveDate) -> Vec<CanonicalCandidate> {
    let policy = atelier_app::workflow_policy::load(&tracker.repo_root).ok();
    let all_issues = tracker.index.issues().cloned().collect::<Vec<_>>();
    let all_issue_ids = all_issues
        .iter()
        .map(|issue| issue.id.clone())
        .collect::<BTreeSet<_>>();
    let links = tracker.index.record_links();
    let retained_records = retained_record_ids(&tracker.index, cutoff);
    let active_descendants = active_descendants_by_ancestor(policy.as_ref(), &all_issues);

    let mut old_terminal = BTreeSet::new();
    for issue in &all_issues {
        if !crate::commands::issue_workflow::issue_is_done(policy.as_ref(), issue) {
            continue;
        }
        let latest_at = issue.closed_at.unwrap_or(issue.updated_at);
        if local_date(latest_at) < cutoff {
            old_terminal.insert(issue.id.clone());
        }
    }

    let retained_issues = all_issue_ids
        .difference(&old_terminal)
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut candidates = Vec::new();
    for issue in all_issues {
        if !old_terminal.contains(&issue.id) {
            continue;
        }
        let latest_at = issue.closed_at.unwrap_or(issue.updated_at);
        let protection = active_descendants
            .get(&issue.id)
            .map(|descendant| format!("terminal record has active descendant {descendant}"))
            .or_else(|| kept_content(&tracker.index, &issue))
            .or_else(|| {
                crossing_link_reason(
                    links,
                    "issue",
                    &issue.id,
                    &retained_issues,
                    &retained_records,
                )
            });
        candidates.push(CanonicalCandidate {
            kind: "issue",
            id: issue.id.clone(),
            title: issue.title,
            status: issue.status,
            path: PathBuf::from("issues").join(format!("{}.md", issue.id)),
            latest_at,
            protection,
        });
    }
    candidates.sort_by(|left, right| left.id.cmp(&right.id));
    candidates
}

/// Decisions and research outlive routine work (atelier-r7cd, decided by
/// Sumeet 2026-10-15): prune keeps decision records and records with a
/// Decision or Background section, which Git history alone would bury.
fn kept_content(index: &RecordIndex, issue: &Issue) -> Option<String> {
    use atelier_core::IssueSectionName::{Background, Decision};
    let record = index.issue_record(&issue.id)?;
    if issue.issue_type == "decision" || record.sections.section(Decision).is_some() {
        Some("keeps decision content".to_string())
    } else if record.sections.section(Background).is_some() {
        Some("keeps research content (Background)".to_string())
    } else {
        None
    }
}

fn active_descendants_by_ancestor(
    policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
    issues: &[Issue],
) -> BTreeMap<String, String> {
    let by_id = issues
        .iter()
        .map(|issue| (issue.id.as_str(), issue))
        .collect::<BTreeMap<_, _>>();
    let mut descendants = BTreeMap::new();
    for issue in issues {
        if crate::commands::issue_workflow::issue_is_done(policy, issue) {
            continue;
        }
        let mut parent = issue.parent_id.as_deref();
        let mut visited = BTreeSet::new();
        while let Some(parent_id) = parent {
            if !visited.insert(parent_id) {
                break;
            }
            descendants
                .entry(parent_id.to_string())
                .or_insert_with(|| issue.id.clone());
            parent = by_id
                .get(parent_id)
                .and_then(|parent_issue| parent_issue.parent_id.as_deref());
        }
    }
    descendants
}

fn review_candidates(tracker: &TrackerContext, cutoff: NaiveDate) -> Vec<CanonicalCandidate> {
    tracker
        .index
        .reviews()
        .filter(|record| local_date(record.header.created_at) < cutoff)
        .map(|record| CanonicalCandidate {
            kind: "review-record",
            id: record.header.id.clone(),
            title: record.header.title.clone(),
            status: record.header.status.clone(),
            path: PathBuf::from("reviews").join(format!("{}.md", record.header.id)),
            latest_at: record.header.created_at,
            protection: None,
        })
        .collect()
}

fn evidence_candidates(
    tracker: &TrackerContext,
    cutoff: NaiveDate,
    issue_candidates: &[CanonicalCandidate],
) -> Vec<CanonicalCandidate> {
    let links = tracker.index.record_links();
    let issue_candidates = issue_candidates
        .iter()
        .filter(|candidate| candidate.eligible())
        .map(|candidate| candidate.id.clone())
        .collect::<BTreeSet<_>>();
    let retained_issues = tracker
        .index
        .issues()
        .map(|issue| issue.id.clone())
        .filter(|id| !issue_candidates.contains(id))
        .collect::<BTreeSet<_>>();
    let all_evidence = tracker.index.evidence_records();
    let old_evidence = all_evidence
        .iter()
        .filter(|record| local_date(record.header.updated_at) < cutoff)
        .map(|record| record.header.id.clone())
        .collect::<BTreeSet<_>>();
    let retained_records = all_evidence
        .iter()
        .filter(|record| !old_evidence.contains(&record.header.id))
        .map(|record| ("evidence".to_string(), record.header.id.clone()))
        .collect::<BTreeSet<_>>();

    let mut candidates = Vec::new();
    for record in all_evidence {
        let header = &record.header;
        if !old_evidence.contains(&header.id) {
            continue;
        }
        let protection = crossing_link_reason(
            links,
            "evidence",
            &header.id,
            &retained_issues,
            &retained_records,
        );
        candidates.push(CanonicalCandidate {
            kind: "evidence-record",
            id: header.id.clone(),
            title: header.title.clone(),
            status: header.status.clone(),
            path: PathBuf::from("evidence").join(format!("{}.md", header.id)),
            latest_at: header.updated_at,
            protection,
        });
    }
    candidates.sort_by(|left, right| left.id.cmp(&right.id));
    candidates
}

fn retained_record_ids(index: &RecordIndex, cutoff: NaiveDate) -> BTreeSet<(String, String)> {
    index
        .evidence_records()
        .into_iter()
        .filter(|record| local_date(record.header.updated_at) >= cutoff)
        .map(|record| (record.header.kind.clone(), record.header.id.clone()))
        .collect()
}

fn crossing_link_reason(
    links: &[RecordLink],
    kind: &str,
    id: &str,
    retained_issues: &BTreeSet<String>,
    retained_records: &BTreeSet<(String, String)>,
) -> Option<String> {
    links.iter().find_map(|link| {
        if link.source_kind == kind && link.source_id == id {
            if link.target_kind == "issue" && retained_issues.contains(&link.target_id) {
                if kind == "evidence" {
                    return Some(format!(
                        "attached to retained issue {} ({})",
                        link.target_id, link.relation_type
                    ));
                }
                return Some(format!(
                    "linked to retained issue {} ({})",
                    link.target_id, link.relation_type
                ));
            }
            if retained_records.contains(&(link.target_kind.clone(), link.target_id.clone())) {
                return Some(format!(
                    "linked to retained {} {} ({})",
                    link.target_kind, link.target_id, link.relation_type
                ));
            }
        }
        if link.target_kind == kind && link.target_id == id {
            if link.source_kind == "issue" && retained_issues.contains(&link.source_id) {
                return Some(format!(
                    "attached to retained issue {} ({})",
                    link.source_id, link.relation_type
                ));
            }
            if retained_records.contains(&(link.source_kind.clone(), link.source_id.clone())) {
                return Some(format!(
                    "linked from retained {} {} ({})",
                    link.source_kind, link.source_id, link.relation_type
                ));
            }
        }
        None
    })
}

fn apply_git_history_protection(
    head_paths: &BTreeSet<PathBuf>,
    candidates: &mut [CanonicalCandidate],
) {
    for candidate in candidates
        .iter_mut()
        .filter(|candidate| candidate.eligible())
    {
        if !head_paths.contains(&PathBuf::from(display_git_path(&candidate.path))) {
            candidate.protection = Some("not present in Git history at HEAD".to_string());
        }
    }
}

/// One invocation's planning facts; apply's checkout check remains fresh.
fn canonical_paths_at_head(repo_root: &Path) -> Result<BTreeSet<PathBuf>> {
    let output = atelier_app::git_freshness::command(repo_root)
        .args(["ls-tree", "-rz", "--name-only", "HEAD", "--", ".atelier"])
        .output()
        .context("failed to inspect Git history for prune candidates")?;
    // An unborn HEAD (or absent repository) has no recoverable candidates.
    Ok(if output.status.success() {
        output
            .stdout
            .split(|byte| *byte == 0)
            .filter(|path| !path.is_empty())
            .map(|path| PathBuf::from(String::from_utf8_lossy(path).as_ref()))
            .collect()
    } else {
        BTreeSet::new()
    })
}

fn dirty_canonical_paths(repo_root: &Path) -> Result<BTreeSet<PathBuf>> {
    let output = atelier_app::git_freshness::command(repo_root)
        .args([
            "status",
            "--porcelain",
            "-z",
            "--untracked-files=all",
            "--",
            ".atelier",
        ])
        .output()
        .context("failed to inspect branch owner record state for prune candidates")?;
    if !output.status.success() {
        bail!("git status failed while inspecting branch owner records");
    }
    let mut paths = BTreeSet::new();
    let mut entries = output.stdout.split(|byte| *byte == 0);
    while let Some(entry) = entries.next().filter(|entry| entry.len() >= 3) {
        paths.insert(PathBuf::from(String::from_utf8_lossy(&entry[3..]).as_ref()));
        // Porcelain -z includes both destination and source for a rename/copy.
        if entry[..2]
            .iter()
            .any(|status| matches!(status, b'R' | b'C'))
        {
            if let Some(source) = entries.next() {
                paths.insert(PathBuf::from(String::from_utf8_lossy(source).as_ref()));
            }
        }
    }
    Ok(paths)
}

fn ensure_clean_for_canonical_prune(repo_root: &Path) -> Result<()> {
    let tracked = atelier_app::git_freshness::command(repo_root)
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .context("failed to inspect tracked checkout state before canonical prune")?;
    if !tracked.status.success() {
        bail!("git status failed before canonical prune");
    }
    let canonical = atelier_app::git_freshness::command(repo_root)
        .args([
            "status",
            "--porcelain",
            "--untracked-files=all",
            "--",
            ".atelier",
        ])
        .output()
        .context("failed to inspect canonical checkout state before canonical prune")?;
    if !canonical.status.success() {
        bail!("git status failed before canonical prune");
    }
    if !tracked.stdout.is_empty() || !canonical.stdout.is_empty() {
        bail!(
            "canonical prune requires a clean tracked checkout and no untracked `.atelier` records; inspect `git status --short --branch`"
        );
    }
    Ok(())
}

fn print_quiet_summary(git: &GitPruneSummary, canonical: &CanonicalPruneSummary, apply: bool) {
    let eligible = git
        .candidates
        .iter()
        .filter(|candidate| candidate.protection.is_none())
        .count()
        + canonical
            .issues
            .iter()
            .chain(canonical.evidence.iter())
            .chain(canonical.reviews.iter())
            .filter(|candidate| candidate.eligible())
            .count();
    let protected = git
        .candidates
        .iter()
        .filter(|candidate| candidate.protection.is_some())
        .count()
        + canonical
            .issues
            .iter()
            .chain(canonical.evidence.iter())
            .chain(canonical.reviews.iter())
            .filter(|candidate| !candidate.eligible())
            .count();
    let removed = git.removed.len() + canonical.removed.len();
    let failures = git.failures.len();
    println!(
        "mode={} eligible={eligible} protected={protected} removed={removed} failures={failures}",
        if apply { "apply" } else { "dry-run" }
    );
}

fn print_git(summary: &GitPruneSummary, apply: bool) {
    println!("Git Branches And Worktrees");
    println!("--------------------------");
    if let Some(reason) = &summary.unavailable {
        println!("Status:    unavailable - {reason}");
        return;
    }
    if summary.candidates.is_empty() {
        println!("Candidates: none");
    } else {
        println!("Candidates: {}", summary.candidates.len());
        for candidate in &summary.candidates {
            let target = candidate
                .path
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| candidate.label.clone());
            if let Some(reason) = &candidate.protection {
                println!("  protected {} {} - {}", candidate.class, target, reason);
            } else if summary
                .removed
                .iter()
                .any(|(class, removed)| class == candidate.class && removed == &target)
            {
                println!("  removed {} {}", candidate.class, target);
            } else if apply {
                println!("  failed {} {}", candidate.class, target);
            } else {
                println!("  eligible {} {}", candidate.class, target);
            }
        }
    }
    if !summary.failures.is_empty() {
        println!("Failures:");
        for (target, error) in &summary.failures {
            println!("  {target} - {error}");
        }
    }
}

fn print_canonical(summary: &CanonicalPruneSummary) {
    println!("Canonical Records");
    println!("-----------------");
    println!("Retention: {} day(s)", summary.retention_days);
    println!("Cutoff:    before {}", summary.cutoff);

    let total_candidates = summary.issues.len() + summary.evidence.len() + summary.reviews.len();
    if total_candidates == 0 {
        println!("Candidates: none");
    } else {
        println!("Candidates: {total_candidates}");
        for candidate in summary
            .issues
            .iter()
            .chain(summary.evidence.iter())
            .chain(summary.reviews.iter())
        {
            print_canonical_candidate(candidate, summary);
        }
    }
}

fn print_canonical_candidate(candidate: &CanonicalCandidate, summary: &CanonicalPruneSummary) {
    let removed = summary
        .removed
        .iter()
        .any(|removed| removed.kind == candidate.kind && removed.id == candidate.id);
    let status = if removed {
        "removed"
    } else if let Some(reason) = &candidate.protection {
        println!(
            "  protected {} {} ({}, latest {}, path {}) - {}",
            candidate.kind,
            candidate.id,
            candidate.status,
            local_date(candidate.latest_at),
            display_git_path(&candidate.path),
            reason
        );
        return;
    } else {
        "eligible"
    };

    println!(
        "  {status} {} {} ({}, latest {}, path {})",
        candidate.kind,
        candidate.id,
        candidate.status,
        local_date(candidate.latest_at),
        display_git_path(&candidate.path)
    );
    println!(
        "    recover: git log --all -- {}; git show <commit>:{}",
        display_git_path(&candidate.path),
        display_git_path(&candidate.path)
    );
    if !candidate.title.trim().is_empty() {
        println!("    title: {}", candidate.title);
    }
}

fn print_deferred_classes() {
    println!("Deferred Cleanup Classes");
    println!("------------------------");
    println!("  none");
}

fn display_git_path(path: &Path) -> String {
    let relative = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");
    format!(".atelier/{relative}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn git(dir: &Path, args: &[&str]) {
        let output = std::process::Command::new("git")
            .current_dir(dir)
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn canonical_status_preserves_rename_paths_and_untracked_files() {
        let repo = tempdir().unwrap();
        git(repo.path(), &["init", "-b", "main"]);
        git(repo.path(), &["config", "user.email", "test@example.com"]);
        git(repo.path(), &["config", "user.name", "Test"]);
        fs::create_dir_all(repo.path().join(".atelier/issues")).unwrap();
        let original = ".atelier/issues/old.md";
        let renamed = ".atelier/issues/new \"name\"\n.md";
        let untracked = ".atelier/issues/untracked.md";
        fs::write(repo.path().join(original), "unchanged record bytes").unwrap();
        git(repo.path(), &["add", ".atelier"]);
        git(repo.path(), &["commit", "-m", "record"]);
        git(repo.path(), &["mv", original, renamed]);
        fs::write(repo.path().join(untracked), "untracked record").unwrap();
        let dirty = dirty_canonical_paths(repo.path()).unwrap();
        for path in [original, renamed, untracked] {
            assert!(
                dirty.contains(Path::new(path)),
                "missing dirty path {path:?}"
            );
        }
        let history = canonical_paths_at_head(repo.path()).unwrap();
        assert!(history.contains(Path::new(original)));
        assert!(!history.contains(Path::new(renamed)));
        assert!(!history.contains(Path::new(untracked)));
    }

    #[test]
    fn branch_with_unpushed_commits_is_protected() {
        let repo = tempdir().unwrap();
        let remote = tempdir().unwrap();
        git(repo.path(), &["init", "-b", "main"]);
        git(repo.path(), &["config", "user.email", "test@example.com"]);
        git(repo.path(), &["config", "user.name", "Test"]);
        fs::write(repo.path().join("base"), "base").unwrap();
        git(repo.path(), &["add", "base"]);
        git(repo.path(), &["commit", "-m", "base"]);
        git(remote.path(), &["init", "--bare"]);
        git(
            repo.path(),
            &["remote", "add", "origin", remote.path().to_str().unwrap()],
        );
        git(repo.path(), &["push", "-u", "origin", "main"]);
        git(repo.path(), &["checkout", "-b", "task/owner"]);
        git(repo.path(), &["push", "-u", "origin", "task/owner"]);
        fs::write(repo.path().join("unpushed"), "work").unwrap();
        git(repo.path(), &["add", "unpushed"]);
        git(repo.path(), &["commit", "-m", "unpushed"]);

        let reason = branch_push_protection(repo.path(), "task/owner").unwrap();
        assert!(reason
            .unwrap()
            .contains("not present in upstream origin/task/owner"));
    }

    #[test]
    fn branch_without_upstream_is_protected() {
        let repo = tempdir().unwrap();
        git(repo.path(), &["init", "-b", "main"]);
        git(repo.path(), &["config", "user.email", "test@example.com"]);
        git(repo.path(), &["config", "user.name", "Test"]);
        fs::write(repo.path().join("base"), "base").unwrap();
        git(repo.path(), &["add", "base"]);
        git(repo.path(), &["commit", "-m", "base"]);

        let reason = branch_push_protection(repo.path(), "main").unwrap();
        assert!(reason.unwrap().contains("has no upstream"));
    }

    #[test]
    fn shadow_tags_cannot_prove_branch_integration() {
        let repo = tempdir().unwrap();
        git(repo.path(), &["init", "-b", "main"]);
        git(repo.path(), &["config", "user.email", "test@example.com"]);
        git(repo.path(), &["config", "user.name", "Test"]);
        fs::write(repo.path().join("base"), "base").unwrap();
        git(repo.path(), &["add", "base"]);
        git(repo.path(), &["commit", "-m", "base"]);
        git(repo.path(), &["switch", "-c", "task/owner"]);
        fs::write(repo.path().join("work"), "unintegrated").unwrap();
        git(repo.path(), &["add", "work"]);
        git(repo.path(), &["commit", "-m", "work"]);
        git(repo.path(), &["tag", "main", "HEAD"]);
        git(repo.path(), &["tag", "task/owner", "refs/heads/main"]);
        assert!(!branch_is_integrated(
            repo.path(),
            "task/owner",
            "main",
            MergeStrategy::FastForwardOnly
        )
        .unwrap());
    }

    #[test]
    fn squash_subject_without_equivalent_patch_does_not_prove_integration() {
        let repo = tempdir().unwrap();
        git(repo.path(), &["init", "-b", "main"]);
        git(repo.path(), &["config", "user.email", "test@example.com"]);
        git(repo.path(), &["config", "user.name", "Test"]);
        fs::write(repo.path().join("base"), "base").unwrap();
        git(repo.path(), &["add", "base"]);
        git(repo.path(), &["commit", "-m", "base"]);
        git(repo.path(), &["checkout", "-b", "task/owner"]);
        fs::write(repo.path().join("owner-work"), "owner").unwrap();
        git(repo.path(), &["add", "owner-work"]);
        git(repo.path(), &["commit", "-m", "owner work"]);
        git(repo.path(), &["checkout", "main"]);
        fs::write(repo.path().join("different-work"), "different").unwrap();
        git(repo.path(), &["add", "different-work"]);
        git(
            repo.path(),
            &["commit", "-m", "Squash merge task/owner into main"],
        );

        assert!(
            !branch_is_integrated(repo.path(), "task/owner", "main", MergeStrategy::Squash)
                .unwrap()
        );
    }

    #[test]
    fn historical_reverted_squash_patch_does_not_prove_later_integration() {
        let repo = tempdir().unwrap();
        git(repo.path(), &["init", "-b", "main"]);
        git(repo.path(), &["config", "user.email", "test@example.com"]);
        git(repo.path(), &["config", "user.name", "Test"]);
        fs::write(repo.path().join("base"), "base").unwrap();
        git(repo.path(), &["add", "base"]);
        git(repo.path(), &["commit", "-m", "base"]);
        fs::write(repo.path().join("replayed-work"), "same patch").unwrap();
        git(repo.path(), &["add", "replayed-work"]);
        git(
            repo.path(),
            &["commit", "-m", "Squash merge task/owner into main"],
        );
        git(repo.path(), &["revert", "--no-edit", "HEAD"]);
        git(repo.path(), &["checkout", "-b", "task/owner"]);
        fs::write(repo.path().join("replayed-work"), "same patch").unwrap();
        git(repo.path(), &["add", "replayed-work"]);
        git(repo.path(), &["commit", "-m", "later owner work"]);

        assert!(
            !branch_is_integrated(repo.path(), "task/owner", "main", MergeStrategy::Squash)
                .unwrap()
        );
    }
}
