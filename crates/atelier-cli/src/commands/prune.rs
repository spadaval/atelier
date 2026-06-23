use anyhow::{bail, Context, Result};
use atelier_app::project_config::DEFAULT_CANONICAL_PRUNE_RETENTION_DAYS;
use atelier_core::{Issue, RecordLink};
use atelier_sqlite::{Database, RecordSummary};
use chrono::{DateTime, Days, NaiveDate, Utc};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    let canonical = prune_canonical_records(tracker, retention_days, apply)?;

    println!("Prune");
    println!("=====");
    println!("Mode: {}", if apply { "apply" } else { "dry-run" });
    println!();

    print_diagnostics(&diagnostics, apply);
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

fn prune_canonical_records(
    tracker: Option<TrackerContext>,
    retention_days_override: Option<u64>,
    apply: bool,
) -> Result<CanonicalPruneSummary> {
    let retention_days = retention_days_override
        .or_else(|| {
            tracker
                .as_ref()
                .map(|tracker| tracker.canonical_retention_days)
        })
        .unwrap_or(DEFAULT_CANONICAL_PRUNE_RETENTION_DAYS);
    let cutoff = Utc::now()
        .date_naive()
        .checked_sub_days(Days::new(retention_days))
        .unwrap_or_else(|| Utc::now().date_naive());
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
            if candidate.kind == "issue" {
                let activity_dir = state_dir
                    .join("issues")
                    .join(format!("{}.activity", candidate.id));
                if activity_dir.exists() {
                    if let Err(error) = fs::remove_dir_all(&activity_dir) {
                        failures.push((activity_dir, error.to_string()));
                    }
                }
            }
            removed.push(CanonicalRemoval {
                kind: candidate.kind,
                id: candidate.id.clone(),
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
    let status = if removed {
        "removed"
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
        let activity_status = if removed { "removed" } else { "eligible" };
        println!(
            "    {activity_status} activity-sidecars {} ({} file(s))",
            display_state_path(&PathBuf::from("issues").join(format!("{}.activity", candidate.id))),
            candidate.activity_count
        );
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

fn print_deferred_classes() {
    println!("Deferred Cleanup Classes");
    println!("------------------------");
    println!(
        "  report-only ignored runtime/cache projection artifacts - local safety contract pending"
    );
    println!("  report-only branches and worktrees - Git safety contract pending");
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
