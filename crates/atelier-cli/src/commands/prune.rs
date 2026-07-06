use anyhow::{bail, Context, Result};
use atelier_app::project_config::DEFAULT_CANONICAL_PRUNE_RETENTION_DAYS;
use atelier_app::workflow_policy::MergeStrategy;
use atelier_core::{Issue, RecordLink};
use atelier_sqlite::{Database, RecordSummary};
use chrono::{DateTime, Days, NaiveDate, Utc};
use fs2::FileExt;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};

use crate::telemetry::{self, DiagnosticsPruneSummary};

pub struct TrackerContext {
    pub db: Database,
    pub repo_root: PathBuf,
    pub state_dir: PathBuf,
    pub db_path: PathBuf,
    pub canonical_retention_days: u64,
}

#[derive(Debug, Clone)]
struct CanonicalPruneSummary {
    retention_days: u64,
    cutoff: NaiveDate,
    issues: Vec<CanonicalCandidate>,
    evidence: Vec<CanonicalCandidate>,
    removed: Vec<CanonicalRemoval>,
    failures: Vec<(PathBuf, String)>,
    unavailable: Option<String>,
    rebuilt_projection: bool,
}

#[derive(Debug, Clone)]
struct CanonicalCandidate {
    kind: &'static str,
    id: String,
    title: String,
    status: String,
    path: PathBuf,
    latest_at: DateTime<Utc>,
    activity_count: usize,
    protection: Option<String>,
}

#[derive(Debug, Clone)]
struct CanonicalRemoval {
    kind: &'static str,
    id: String,
    activity_removed: bool,
}

#[derive(Debug, Clone)]
struct LocalCandidate {
    class: &'static str,
    path: PathBuf,
    protection: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct LocalPruneSummary {
    candidates: Vec<LocalCandidate>,
    removed: Vec<PathBuf>,
    failures: Vec<(PathBuf, String)>,
    unavailable: Option<String>,
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
}

impl CanonicalCandidate {
    fn eligible(&self) -> bool {
        self.protection.is_none()
    }
}

pub fn run(
    tracker: Option<TrackerContext>,
    apply: bool,
    retention_days: Option<u64>,
) -> Result<()> {
    let diagnostics = telemetry::prune_diagnostics_logs(retention_days, apply)?;
    // Local artifacts are explicitly independent of canonical health.  Do this
    // before opening or validating canonical state so a broken projection never
    // prevents removal of an abandoned temporary file.
    let local = prune_local_artifacts(tracker.as_ref(), retention_days, apply)?;
    let git = prune_git_artifacts(tracker.as_ref(), retention_days, apply)?;
    let canonical = prune_canonical_records(tracker, retention_days, apply)?;

    println!("Prune");
    println!("=====");
    println!("Mode: {}", if apply { "apply" } else { "dry-run" });
    println!();

    print_diagnostics(&diagnostics, apply);
    println!();
    print_local(&local, apply);
    println!();
    print_git(&git, apply);
    println!();
    print_canonical(&canonical, apply);
    println!();
    print_deferred_classes();

    if !apply {
        println!();
        println!("Next: atelier prune --apply");
    }

    Ok(())
}

fn prune_git_artifacts(
    tracker: Option<&TrackerContext>,
    retention_days_override: Option<u64>,
    apply: bool,
) -> Result<GitPruneSummary> {
    let Some(tracker) = tracker else {
        return Ok(GitPruneSummary {
            unavailable: Some("tracker unavailable in this directory".to_string()),
            ..Default::default()
        });
    };
    let git_repository = Command::new("git")
        .current_dir(&tracker.repo_root)
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
    let (_, cutoff) = canonical_retention_cutoff(Some(tracker), retention_days_override);
    let mut owner_candidates = issue_candidates(tracker, cutoff)?;
    apply_git_history_protection(&tracker.repo_root, &mut owner_candidates)?;
    for candidate in owner_candidates
        .iter_mut()
        .filter(|candidate| candidate.eligible())
    {
        if canonical_path_is_dirty(&tracker.repo_root, &candidate.path)? {
            candidate.protection = Some("owner record has uncommitted changes".to_string());
        }
    }
    let owner_candidates = owner_candidates
        .into_iter()
        .map(|candidate| (candidate.id.clone(), candidate))
        .collect::<BTreeMap<_, _>>();
    let mut branch_owners = BTreeMap::<String, Vec<GitBranchOwner>>::new();
    for issue in tracker.db.list_issues(Some("all"), None, None)? {
        if let Ok(resolution) =
            atelier_app::workflow_policy::resolve_branch_lifecycle(&policy, &tracker.db, &issue.id)
        {
            if resolution.owner_id == issue.id {
                let protection =
                    if !crate::commands::issue_workflow::issue_is_done(Some(&policy), &issue) {
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
                    .entry(resolution.expected_branch)
                    .or_default()
                    .push(GitBranchOwner {
                        id: issue.id,
                        base: resolution.base_branch,
                        merge_strategy: resolution.merge_strategy,
                        protection,
                    });
            }
        }
    }
    let current = git_stdout_trimmed(&tracker.repo_root, &["branch", "--show-current"])?;
    let worktrees = git_worktrees(&tracker.repo_root)?;
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
    let branches = git_stdout_trimmed(
        &tracker.repo_root,
        &["for-each-ref", "--format=%(refname:short)", "refs/heads"],
    )?;
    for branch in branches.lines().filter(|branch| !branch.is_empty()) {
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
        let dirty = !git_stdout_trimmed(&worktree.path, &["status", "--porcelain"])?.is_empty();
        let protection = if is_current {
            Some("current checkout".to_string())
        } else if worktree.locked {
            Some("locked worktree".to_string())
        } else if dirty {
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
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(["merge-base", "--is-ancestor", branch, base])
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
    let merge_base = git_stdout_trimmed(repo_root, &["merge-base", branch, base])?;
    let branch_diff = git_diff(repo_root, &merge_base, branch)?;
    if branch_diff.is_empty() {
        return Ok(false);
    }
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(["log", "-z", "--format=%H%x00%s", base])
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
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(["diff", "--binary", "--full-index", "--no-renames", from, to])
        .output()
        .context("failed to compare Git integration patches for prune candidate")?;
    if !output.status.success() {
        bail!("git diff failed while inspecting prune candidate integration");
    }
    Ok(output.stdout)
}

fn branch_push_protection(repo_root: &Path, branch: &str) -> Result<Option<String>> {
    let upstream = Command::new("git")
        .current_dir(repo_root)
        .args([
            "rev-parse",
            "--abbrev-ref",
            &format!("{branch}@{{upstream}}"),
        ])
        .output()
        .context("failed to inspect Git upstream for prune candidate")?;
    if !upstream.status.success() {
        return Ok(Some(
            "has no upstream; push or preserve it before pruning".to_string(),
        ));
    }
    let upstream = String::from_utf8_lossy(&upstream.stdout).trim().to_string();
    let unpushed = git_stdout_trimmed(repo_root, &["rev-list", &format!("{upstream}..{branch}")])?;
    if unpushed.is_empty() {
        Ok(None)
    } else {
        Ok(Some(format!(
            "contains commits not present in upstream {upstream}"
        )))
    }
}

fn git_stdout_trimmed(repo_root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .current_dir(repo_root)
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
    let output = Command::new("git")
        .current_dir(repo_root)
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

fn prune_local_artifacts(
    tracker: Option<&TrackerContext>,
    retention_days: Option<u64>,
    apply: bool,
) -> Result<LocalPruneSummary> {
    let Some(tracker) = tracker else {
        return Ok(LocalPruneSummary {
            unavailable: Some("tracker unavailable in this directory".to_string()),
            ..Default::default()
        });
    };
    let cutoff = SystemTime::now()
        .checked_sub(Duration::from_secs(
            retention_days.unwrap_or(30).saturating_mul(86_400),
        ))
        .unwrap_or(SystemTime::UNIX_EPOCH);
    let mut summary = LocalPruneSummary::default();
    for root in [
        tracker.state_dir.join("runtime"),
        tracker.state_dir.join("cache"),
        tracker.state_dir.join(".cache"),
    ] {
        collect_local_candidates(
            &tracker.repo_root,
            &tracker.state_dir,
            &root,
            cutoff,
            &mut summary.candidates,
        )?;
    }
    summary.candidates.sort_by(|a, b| a.path.cmp(&b.path));
    if apply {
        for candidate in summary.candidates.iter().filter(|c| c.protection.is_none()) {
            let absolute = tracker.state_dir.join(&candidate.path);
            match fs::remove_file(&absolute) {
                Ok(()) => summary.removed.push(candidate.path.clone()),
                Err(error) => summary
                    .failures
                    .push((candidate.path.clone(), error.to_string())),
            }
        }
    }
    Ok(summary)
}

fn collect_local_candidates(
    repo_root: &Path,
    state_dir: &Path,
    root: &Path,
    cutoff: SystemTime,
    candidates: &mut Vec<LocalCandidate>,
) -> Result<()> {
    if !root.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(root)
        .with_context(|| format!("failed to inspect local prune directory {}", root.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_local_candidates(repo_root, state_dir, &path, cutoff, candidates)?;
            continue;
        }
        let relative = match path.strip_prefix(state_dir) {
            Ok(path) => path.to_path_buf(),
            Err(_) => continue,
        };
        if !is_ignored_local_path(repo_root, &path)? {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        let is_lock = name.ends_with(".lock");
        let is_temp = name.ends_with(".tmp")
            || name.contains(".rebuild-tmp")
            || name.ends_with("-journal")
            || name.ends_with("-wal")
            || name.ends_with("-shm")
            || name.ends_with(".journal");
        let is_cache = relative
            .components()
            .next()
            .map(|part| part.as_os_str() == "cache" || part.as_os_str() == ".cache")
            .unwrap_or(false);
        if !is_temp && !is_cache && !is_lock {
            continue;
        }
        let stale = entry
            .metadata()?
            .modified()
            .map(|modified| modified < cutoff)
            .unwrap_or(true);
        if !is_temp && !is_lock && !stale {
            continue;
        }
        let class = if is_cache {
            "stale-cache"
        } else if is_temp {
            "orphaned-temp"
        } else {
            "runtime-lock"
        };
        let protection = if is_lock {
            Some("locked by a running or interrupted command; inspect before removal".to_string())
        } else if is_runtime_projection_artifact(&relative) {
            projection_artifact_protection(state_dir, &path)?
        } else {
            None
        };
        candidates.push(LocalCandidate {
            class,
            path: relative,
            protection,
        });
    }
    Ok(())
}

fn is_runtime_projection_artifact(relative: &Path) -> bool {
    relative
        .components()
        .next()
        .map(|part| part.as_os_str() == "runtime")
        .unwrap_or(false)
}

fn projection_artifact_protection(state_dir: &Path, artifact: &Path) -> Result<Option<String>> {
    let name = artifact
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    // SQLite sidecars are tied to an open projection database. They cannot be
    // distinguished safely from live state without coordinating with SQLite.
    if name.ends_with("-wal")
        || name.ends_with("-shm")
        || name.ends_with("-journal")
        || name.ends_with(".journal")
    {
        return Ok(Some(
            "runtime projection sidecar may belong to an open database".to_string(),
        ));
    }
    if !name.contains(".rebuild-tmp") {
        return Ok(None);
    }
    let lock = state_dir.join("runtime/.state.db.rebuild.lock");
    let file = match OpenOptions::new().read(true).write(true).open(&lock) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "failed to inspect projection rebuild lock {}",
                    lock.display()
                )
            })
        }
    };
    match file.try_lock_exclusive() {
        Ok(()) => {
            let _ = file.unlock();
            Ok(Some(
                "projection rebuild artifact is protected outside a coordinated rebuild; run `atelier check --fix`"
                    .to_string(),
            ))
        }
        Err(_) => Ok(Some(
            "projection rebuild lock is active; rebuild artifact may be live".to_string(),
        )),
    }
}

fn is_ignored_local_path(repo_root: &Path, path: &Path) -> Result<bool> {
    let output = Command::new("git")
        .current_dir(repo_root)
        .args([
            "check-ignore",
            "--quiet",
            "--",
            path.to_string_lossy().as_ref(),
        ])
        .output()
        .context("failed to inspect Git ignore state for local prune candidate")?;
    Ok(output.status.success())
}

fn prune_canonical_records(
    tracker: Option<TrackerContext>,
    retention_days_override: Option<u64>,
    apply: bool,
) -> Result<CanonicalPruneSummary> {
    let (retention_days, cutoff) =
        canonical_retention_cutoff(tracker.as_ref(), retention_days_override);
    let Some(tracker) = tracker else {
        return Ok(CanonicalPruneSummary {
            retention_days,
            cutoff,
            issues: Vec::new(),
            evidence: Vec::new(),
            removed: Vec::new(),
            failures: Vec::new(),
            unavailable: Some("tracker unavailable in this directory".to_string()),
            rebuilt_projection: false,
        });
    };

    let mut issues = issue_candidates(&tracker, cutoff)?;
    let evidence = evidence_candidates(&tracker, cutoff, &issues)?;
    apply_git_history_protection(&tracker.repo_root, &mut issues)?;
    let mut evidence = evidence;
    apply_git_history_protection(&tracker.repo_root, &mut evidence)?;

    let mut removed = Vec::new();
    let mut failures = Vec::new();
    if apply {
        let has_eligible = issues.iter().any(CanonicalCandidate::eligible)
            || evidence.iter().any(CanonicalCandidate::eligible);
        if has_eligible {
            ensure_clean_for_canonical_prune(&tracker.repo_root)?;
        }
        for candidate in evidence
            .iter()
            .chain(issues.iter())
            .filter(|c| c.eligible())
        {
            remove_candidate(&tracker.state_dir, candidate, &mut removed, &mut failures);
        }
        if !removed.is_empty() {
            drop(tracker.db);
            atelier_app::rebuild::run(&tracker.state_dir, &tracker.db_path)
                .context("failed to rebuild local projection after canonical prune")?;
            return Ok(CanonicalPruneSummary {
                retention_days,
                cutoff,
                issues,
                evidence,
                removed,
                failures,
                unavailable: None,
                rebuilt_projection: true,
            });
        }
    }

    Ok(CanonicalPruneSummary {
        retention_days,
        cutoff,
        issues,
        evidence,
        removed,
        failures,
        unavailable: None,
        rebuilt_projection: false,
    })
}

fn canonical_retention_cutoff(
    tracker: Option<&TrackerContext>,
    retention_days_override: Option<u64>,
) -> (u64, NaiveDate) {
    let retention_days = retention_days_override
        .or_else(|| tracker.map(|tracker| tracker.canonical_retention_days))
        .unwrap_or(DEFAULT_CANONICAL_PRUNE_RETENTION_DAYS);
    let cutoff = Utc::now()
        .date_naive()
        .checked_sub_days(Days::new(retention_days))
        .unwrap_or_else(|| Utc::now().date_naive());
    (retention_days, cutoff)
}

fn issue_candidates(
    tracker: &TrackerContext,
    cutoff: NaiveDate,
) -> Result<Vec<CanonicalCandidate>> {
    let policy = atelier_app::workflow_policy::load(&tracker.repo_root).ok();
    let all_issues = tracker.db.list_issues(Some("all"), None, None)?;
    let all_issue_ids = all_issues
        .iter()
        .map(|issue| issue.id.clone())
        .collect::<BTreeSet<_>>();
    let activity_latest = issue_activity_latest(&tracker.state_dir)?;
    let links = tracker.db.list_all_record_links()?;
    let retained_records = retained_record_ids(&tracker.db, cutoff)?;

    let mut old_terminal = BTreeSet::new();
    for issue in &all_issues {
        if !crate::commands::issue_workflow::issue_is_done(policy.as_ref(), issue) {
            continue;
        }
        let latest_at = latest_issue_time(issue, activity_latest.get(&issue.id));
        if latest_at.date_naive() < cutoff {
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
        let latest_at = latest_issue_time(&issue, activity_latest.get(&issue.id));
        let activity_count = activity_latest
            .get(&issue.id)
            .map(|activity| activity.count)
            .unwrap_or(0);
        let protection = crossing_link_reason(
            &links,
            "issue",
            &issue.id,
            &retained_issues,
            &retained_records,
        );
        candidates.push(CanonicalCandidate {
            kind: "issue",
            id: issue.id.clone(),
            title: issue.title,
            status: issue.status,
            path: PathBuf::from("issues").join(format!("{}.md", issue.id)),
            latest_at,
            activity_count,
            protection,
        });
    }
    candidates.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(candidates)
}

fn evidence_candidates(
    tracker: &TrackerContext,
    cutoff: NaiveDate,
    issue_candidates: &[CanonicalCandidate],
) -> Result<Vec<CanonicalCandidate>> {
    let links = tracker.db.list_all_record_links()?;
    let issue_candidates = issue_candidates
        .iter()
        .filter(|candidate| candidate.eligible())
        .map(|candidate| candidate.id.clone())
        .collect::<BTreeSet<_>>();
    let retained_issues = tracker
        .db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .map(|issue| issue.id)
        .filter(|id| !issue_candidates.contains(id))
        .collect::<BTreeSet<_>>();
    let all_evidence = tracker.db.list_records("evidence", None)?;
    let old_evidence = all_evidence
        .iter()
        .filter(|record| record.updated_at.date_naive() < cutoff)
        .map(|record| record.id.clone())
        .collect::<BTreeSet<_>>();
    let retained_records = all_evidence
        .iter()
        .filter(|record| !old_evidence.contains(&record.id))
        .map(|record| ("evidence".to_string(), record.id.clone()))
        .collect::<BTreeSet<_>>();

    let mut candidates = Vec::new();
    for record in all_evidence {
        if !old_evidence.contains(&record.id) {
            continue;
        }
        let path = record_path(&record, "evidence");
        let protection = crossing_link_reason(
            &links,
            "evidence",
            &record.id,
            &retained_issues,
            &retained_records,
        );
        candidates.push(CanonicalCandidate {
            kind: "evidence-record",
            id: record.id.clone(),
            title: record.title,
            status: record.status,
            path,
            latest_at: record.updated_at,
            activity_count: 0,
            protection,
        });
    }
    candidates.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(candidates)
}

fn latest_issue_time(issue: &Issue, activity: Option<&IssueActivityLatest>) -> DateTime<Utc> {
    let mut latest = issue.closed_at.unwrap_or(issue.updated_at);
    if let Some(activity) = activity {
        latest = latest.max(activity.latest_at);
    }
    latest
}

#[derive(Debug, Clone)]
struct IssueActivityLatest {
    latest_at: DateTime<Utc>,
    count: usize,
}

fn issue_activity_latest(state_dir: &Path) -> Result<BTreeMap<String, IssueActivityLatest>> {
    let mut latest = BTreeMap::<String, IssueActivityLatest>::new();
    for activity in atelier_records::activity::list_all_issue_activities(state_dir)? {
        latest
            .entry(activity.subject_id)
            .and_modify(|entry| {
                entry.latest_at = entry.latest_at.max(activity.created_at);
                entry.count += 1;
            })
            .or_insert(IssueActivityLatest {
                latest_at: activity.created_at,
                count: 1,
            });
    }
    Ok(latest)
}

fn retained_record_ids(db: &Database, cutoff: NaiveDate) -> Result<BTreeSet<(String, String)>> {
    let mut retained = BTreeSet::new();
    for record in db.list_records("evidence", None)? {
        if record.updated_at.date_naive() >= cutoff {
            retained.insert((record.kind, record.id));
        }
    }
    Ok(retained)
}

fn record_path(record: &RecordSummary, directory: &str) -> PathBuf {
    if record.source_path.trim().is_empty() {
        PathBuf::from(directory).join(format!("{}.md", record.id))
    } else {
        PathBuf::from(&record.source_path)
    }
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
    repo_root: &Path,
    candidates: &mut [CanonicalCandidate],
) -> Result<()> {
    for candidate in candidates
        .iter_mut()
        .filter(|candidate| candidate.eligible())
    {
        if !path_exists_in_head(repo_root, &candidate.path)? {
            candidate.protection = Some("not present in Git history at HEAD".to_string());
        }
    }
    Ok(())
}

fn path_exists_in_head(repo_root: &Path, path: &Path) -> Result<bool> {
    let spec = format!("HEAD:{}", display_git_path(path));
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(["cat-file", "-e", &spec])
        .output()
        .context("failed to inspect Git history for prune candidate")?;
    Ok(output.status.success())
}

fn canonical_path_is_dirty(repo_root: &Path, path: &Path) -> Result<bool> {
    let output = Command::new("git")
        .current_dir(repo_root)
        .args([
            "status",
            "--porcelain",
            "--untracked-files=all",
            "--",
            &display_git_path(path),
        ])
        .output()
        .context("failed to inspect branch owner record state for prune candidate")?;
    if !output.status.success() {
        bail!("git status failed while inspecting branch owner record");
    }
    Ok(!output.stdout.is_empty())
}

fn ensure_clean_for_canonical_prune(repo_root: &Path) -> Result<()> {
    let tracked = Command::new("git")
        .current_dir(repo_root)
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .context("failed to inspect tracked checkout state before canonical prune")?;
    if !tracked.status.success() {
        bail!("git status failed before canonical prune");
    }
    let canonical = Command::new("git")
        .current_dir(repo_root)
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

fn remove_candidate(
    state_dir: &Path,
    candidate: &CanonicalCandidate,
    removed: &mut Vec<CanonicalRemoval>,
    failures: &mut Vec<(PathBuf, String)>,
) {
    let absolute = state_dir.join(&candidate.path);
    match fs::remove_file(&absolute) {
        Ok(()) => {
            let mut activity_removed = false;
            if candidate.kind == "issue" {
                let activity_dir = state_dir
                    .join("issues")
                    .join(format!("{}.activity", candidate.id));
                if activity_dir.exists() {
                    match fs::remove_dir_all(&activity_dir) {
                        Ok(()) => activity_removed = true,
                        Err(error) => failures.push((
                            PathBuf::from("issues").join(format!("{}.activity", candidate.id)),
                            error.to_string(),
                        )),
                    }
                }
            }
            removed.push(CanonicalRemoval {
                kind: candidate.kind,
                id: candidate.id.clone(),
                activity_removed,
            });
        }
        Err(error) => failures.push((candidate.path.clone(), error.to_string())),
    }
}

fn print_diagnostics(summary: &DiagnosticsPruneSummary, apply: bool) {
    println!("Diagnostics Logs");
    println!("----------------");
    println!("Retention: {} day(s)", summary.retention_days);
    println!("Cutoff:    before {}", summary.cutoff);
    match &summary.commands_dir {
        Some(path) => println!("Path:      {}", path.display()),
        None => println!("Path:      disabled or unavailable"),
    }

    if summary.candidates.is_empty() {
        println!("Candidates: none");
        return;
    }

    println!("Candidates: {}", summary.candidates.len());
    for candidate in &summary.candidates {
        let status = if apply && summary.removed.contains(&candidate.path) {
            "removed"
        } else if apply {
            "failed"
        } else {
            "eligible"
        };
        println!(
            "  {status} diagnostics-log {} (date {}, {} bytes)",
            candidate.path.display(),
            candidate.date,
            candidate.size_bytes
        );
    }

    if !summary.failures.is_empty() {
        println!("Failures:");
        for (path, error) in &summary.failures {
            println!("  {} - {}", path.display(), error);
        }
    }
}

fn print_local(summary: &LocalPruneSummary, apply: bool) {
    println!("Ignored Runtime, Cache, And Projection Artifacts");
    println!("-----------------------------------------------");
    if let Some(reason) = &summary.unavailable {
        println!("Status:    unavailable - {reason}");
        return;
    }
    if summary.candidates.is_empty() {
        println!("Candidates: none");
    } else {
        println!("Candidates: {}", summary.candidates.len());
        for candidate in &summary.candidates {
            let removed = summary.removed.contains(&candidate.path);
            if let Some(reason) = &candidate.protection {
                println!(
                    "  protected {} {} - {}",
                    candidate.class,
                    display_state_path(&candidate.path),
                    reason
                );
            } else if removed {
                println!(
                    "  removed {} {}",
                    candidate.class,
                    display_state_path(&candidate.path)
                );
            } else if apply {
                println!(
                    "  failed {} {}",
                    candidate.class,
                    display_state_path(&candidate.path)
                );
            } else {
                println!(
                    "  eligible {} {}",
                    candidate.class,
                    display_state_path(&candidate.path)
                );
            }
        }
    }
    if !summary.removed.is_empty() {
        println!("Projection: local state changed; run `atelier check --fix` if projection health is stale");
    }
    if !summary.failures.is_empty() {
        println!("Failures:");
        for (path, error) in &summary.failures {
            println!("  {} - {}", display_state_path(path), error);
        }
    }
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

fn print_canonical(summary: &CanonicalPruneSummary, apply: bool) {
    println!("Canonical Records");
    println!("-----------------");
    println!("Retention: {} day(s)", summary.retention_days);
    println!("Cutoff:    before {}", summary.cutoff);
    if let Some(reason) = &summary.unavailable {
        println!("Status:    unavailable - {reason}");
        return;
    }

    let total_candidates = summary.issues.len() + summary.evidence.len();
    if total_candidates == 0 {
        println!("Candidates: none");
    } else {
        println!("Candidates: {total_candidates}");
        for candidate in summary.issues.iter().chain(summary.evidence.iter()) {
            print_canonical_candidate(candidate, apply, summary);
        }
    }

    if summary.rebuilt_projection {
        println!("Projection: rebuilt after canonical prune");
    }

    if !summary.failures.is_empty() {
        println!("Failures:");
        for (path, error) in &summary.failures {
            println!("  {} - {}", path.display(), error);
        }
    }
}

fn print_canonical_candidate(
    candidate: &CanonicalCandidate,
    apply: bool,
    summary: &CanonicalPruneSummary,
) {
    let removed = summary
        .removed
        .iter()
        .any(|removed| removed.kind == candidate.kind && removed.id == candidate.id);
    let failed = summary
        .failures
        .iter()
        .any(|(path, _)| path == &candidate.path);
    let status = if removed {
        "removed"
    } else if failed {
        "failed"
    } else if let Some(reason) = &candidate.protection {
        println!(
            "  protected {} {} ({}, latest {}, path {}) - {}",
            candidate.kind,
            candidate.id,
            candidate.status,
            candidate.latest_at.date_naive(),
            display_state_path(&candidate.path),
            reason
        );
        return;
    } else if apply {
        "eligible"
    } else {
        "eligible"
    };

    println!(
        "  {status} {} {} ({}, latest {}, path {})",
        candidate.kind,
        candidate.id,
        candidate.status,
        candidate.latest_at.date_naive(),
        display_state_path(&candidate.path)
    );
    if candidate.activity_count > 0 {
        println!("{}", canonical_activity_line(candidate, summary));
    }
    println!(
        "    recover: git log --all -- {}; git show <commit>:{}",
        display_git_path(&candidate.path),
        display_git_path(&candidate.path)
    );
    if !candidate.title.trim().is_empty() {
        println!("    title: {}", candidate.title);
    }
}

fn canonical_activity_line(
    candidate: &CanonicalCandidate,
    summary: &CanonicalPruneSummary,
) -> String {
    let activity_path = PathBuf::from("issues").join(format!("{}.activity", candidate.id));
    let activity_removed = summary
        .removed
        .iter()
        .find(|removal| removal.kind == candidate.kind && removal.id == candidate.id)
        .map(|removal| removal.activity_removed)
        .unwrap_or(false);
    let activity_failed = summary
        .failures
        .iter()
        .any(|(path, _)| path == &activity_path);
    let activity_status = if activity_removed {
        "removed"
    } else if activity_failed {
        "failed"
    } else {
        "eligible"
    };
    format!(
        "    {activity_status} activity-sidecars {} ({} file(s))",
        display_state_path(&activity_path),
        candidate.activity_count
    )
}

fn print_deferred_classes() {
    println!("Deferred Cleanup Classes");
    println!("------------------------");
    println!("  none");
}

fn display_state_path(path: &Path) -> String {
    display_git_path(path)
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
        let output = Command::new("git")
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
    fn failed_activity_sidecar_removal_is_not_reported_as_removed() {
        let state = tempdir().unwrap();
        let issues = state.path().join("issues");
        fs::create_dir_all(&issues).unwrap();
        let id = "atelier-sidecar";
        fs::write(issues.join(format!("{id}.md")), "record").unwrap();
        // A file at the activity-directory path makes remove_dir_all fail on
        // every platform, exercising the independent sidecar failure state.
        let activity = issues.join(format!("{id}.activity"));
        fs::write(&activity, "not a directory").unwrap();
        let candidate = CanonicalCandidate {
            kind: "issue",
            id: id.to_string(),
            title: String::new(),
            status: "done".to_string(),
            path: PathBuf::from("issues").join(format!("{id}.md")),
            latest_at: Utc::now(),
            activity_count: 1,
            protection: None,
        };
        let mut removed = Vec::new();
        let mut failures = Vec::new();

        remove_candidate(state.path(), &candidate, &mut removed, &mut failures);

        assert_eq!(removed.len(), 1);
        assert!(!removed[0].activity_removed);
        let relative_activity = PathBuf::from("issues").join(format!("{id}.activity"));
        assert!(failures.iter().any(|(path, _)| path == &relative_activity));
        let summary = CanonicalPruneSummary {
            retention_days: 7,
            cutoff: Utc::now().date_naive(),
            issues: vec![candidate.clone()],
            evidence: Vec::new(),
            removed,
            failures,
            unavailable: None,
            rebuilt_projection: false,
        };
        assert!(canonical_activity_line(&candidate, &summary)
            .contains("failed activity-sidecars .atelier/issues/atelier-sidecar.activity"));
    }
}
