use anyhow::Result;
use std::collections::BTreeMap;
use std::path::Path;

use atelier_core::Issue;
use atelier_sqlite::Database;

use crate::workflow_policy::WorkflowPolicy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkRow {
    pub id: String,
    pub title: String,
    pub issue_type: String,
    pub status: String,
    pub status_category: Option<String>,
    pub priority: String,
    pub open_blockers: Vec<String>,
}

impl WorkRow {
    pub fn state_label(&self) -> &'static str {
        if self.status_category.as_deref() == Some("active") {
            "active"
        } else if !self.open_blockers.is_empty() {
            "blocked"
        } else if self.status_category.as_deref() == Some("todo") {
            "ready"
        } else if self.status_category.as_deref() == Some("done") {
            "done"
        } else {
            "backlog"
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorkBuckets {
    pub active: Vec<WorkRow>,
    pub ready: Vec<WorkRow>,
    pub blocked: Vec<WorkRow>,
    pub backlog: Vec<WorkRow>,
    pub done: Vec<WorkRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionSummary {
    pub id: String,
    pub title: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StatusNextAction {
    InspectReadyWork { count: usize },
    InspectBlockedWork,
    InspectHealth { stale_records: usize },
    NoSpecificAction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusView {
    pub tracker_state: String,
    pub stale_records: usize,
    pub work: WorkBuckets,
    pub current_missions: Vec<MissionSummary>,
    pub active_role_counts: BTreeMap<String, usize>,
    pub next_action: StatusNextAction,
}

pub fn status_view(
    db: &Database,
    state_dir: &Path,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<StatusView> {
    let work = work_buckets(db, workflow_policy)?;
    let current_missions = current_missions(db, workflow_policy)?;
    let active_role_counts = active_role_counts(&work.active, workflow_policy);
    let stale_records = crate::export::canonical_stale_entries(db, state_dir)?.len();
    let tracker_state = if stale_records == 0 {
        "current".to_string()
    } else {
        "stale".to_string()
    };
    let next_action = if stale_records > 0 {
        StatusNextAction::InspectHealth { stale_records }
    } else if !work.ready.is_empty() {
        StatusNextAction::InspectReadyWork {
            count: work.ready.len(),
        }
    } else if !work.blocked.is_empty() {
        StatusNextAction::InspectBlockedWork
    } else {
        StatusNextAction::NoSpecificAction
    };

    Ok(StatusView {
        tracker_state,
        stale_records,
        work,
        current_missions,
        active_role_counts,
        next_action,
    })
}

pub fn work_buckets(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<WorkBuckets> {
    let mut buckets = WorkBuckets::default();
    for issue in db.list_issues(Some("all"), None, None)? {
        let row = work_row(db, workflow_policy, issue)?;
        match row.state_label() {
            "active" => buckets.active.push(row),
            "ready" => buckets.ready.push(row),
            "blocked" => buckets.blocked.push(row),
            "done" => buckets.done.push(row),
            _ => buckets.backlog.push(row),
        }
    }
    sort_work_rows(&mut buckets.active);
    sort_work_rows(&mut buckets.ready);
    sort_work_rows(&mut buckets.blocked);
    sort_work_rows(&mut buckets.backlog);
    sort_work_rows(&mut buckets.done);
    Ok(buckets)
}

fn work_row(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: Issue,
) -> Result<WorkRow> {
    let status_category = workflow_policy
        .and_then(|policy| policy.status_category(&issue.status))
        .map(str::to_string);
    Ok(WorkRow {
        id: issue.id.clone(),
        title: issue.title,
        issue_type: issue.issue_type,
        status: issue.status,
        status_category,
        priority: issue.priority,
        open_blockers: open_blockers(db, workflow_policy, &issue.id)?,
    })
}

fn open_blockers(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issue_id: &str,
) -> Result<Vec<String>> {
    let mut blockers = db
        .get_blockers(issue_id)?
        .into_iter()
        .filter_map(|id| db.require_issue(&id).ok())
        .filter(|issue| issue_blocks_work(workflow_policy, issue))
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    blockers.sort();
    Ok(blockers)
}

fn issue_blocks_work(workflow_policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    workflow_policy.and_then(|policy| policy.status_category(&issue.status)) != Some("done")
}

fn current_missions(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<Vec<MissionSummary>> {
    let mut missions = db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| issue.issue_type == "mission")
        .filter(|issue| {
            workflow_policy.and_then(|policy| policy.status_category(&issue.status)) != Some("done")
        })
        .map(|issue| MissionSummary {
            id: issue.id,
            title: issue.title,
            status: issue.status,
        })
        .collect::<Vec<_>>();
    missions.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(missions)
}

fn active_role_counts(
    active: &[WorkRow],
    workflow_policy: Option<&WorkflowPolicy>,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in active {
        let role = workflow_policy
            .and_then(|policy| policy.status_role(&row.status))
            .unwrap_or("unconfigured")
            .to_string();
        *counts.entry(role).or_insert(0) += 1;
    }
    counts
}

fn sort_work_rows(rows: &mut [WorkRow]) {
    rows.sort_by(|left, right| {
        priority_rank(&right.priority)
            .cmp(&priority_rank(&left.priority))
            .then_with(|| right.status.cmp(&left.status))
            .then_with(|| left.id.cmp(&right.id))
    });
}

fn priority_rank(priority: &str) -> u8 {
    match priority {
        "critical" | "P0" => 4,
        "high" | "P1" => 3,
        "medium" | "P2" => 2,
        "low" | "P3" => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_core::Issue;
    use atelier_sqlite::Database;
    use chrono::Utc;
    use tempfile::tempdir;

    fn issue(id: &str, title: &str, status: &str, priority: &str) -> Issue {
        let now = Utc::now();
        Issue {
            id: id.to_string(),
            title: title.to_string(),
            description: None,
            status: status.to_string(),
            priority: priority.to_string(),
            issue_type: "task".to_string(),
            fields: BTreeMap::new(),
            created_at: now,
            updated_at: now,
            closed_at: None,
            parent_id: None,
        }
    }

    fn db_with_rows() -> Database {
        let dir = tempdir().unwrap();
        let db = Database::open(&dir.path().join("state.db")).unwrap();
        db.insert_issue_rebuild(&issue("atelier-ready", "Ready", "ready", "high"))
            .unwrap();
        db.insert_issue_rebuild(&issue("atelier-active", "Active", "in_progress", "medium"))
            .unwrap();
        db.insert_issue_rebuild(&issue("atelier-done", "Done", "done", "low"))
            .unwrap();
        db.insert_issue_rebuild(&issue("atelier-blocked", "Blocked", "todo", "medium"))
            .unwrap();
        db.add_dependency("atelier-blocked", "atelier-ready")
            .unwrap();
        db
    }

    fn test_policy() -> WorkflowPolicy {
        let mut statuses = BTreeMap::new();
        statuses.insert(
            "ready".to_string(),
            crate::workflow_policy::StatusDefinition {
                category: "todo".to_string(),
                role: None,
            },
        );
        statuses.insert(
            "todo".to_string(),
            crate::workflow_policy::StatusDefinition {
                category: "todo".to_string(),
                role: None,
            },
        );
        statuses.insert(
            "in_progress".to_string(),
            crate::workflow_policy::StatusDefinition {
                category: "active".to_string(),
                role: Some("worker".to_string()),
            },
        );
        statuses.insert(
            "done".to_string(),
            crate::workflow_policy::StatusDefinition {
                category: "done".to_string(),
                role: None,
            },
        );
        WorkflowPolicy {
            schema_version: 3,
            branch_policy: crate::workflow_policy::BranchLifecycleConfig::default(),
            issue_types: BTreeMap::new(),
            workflow_by_issue_type: BTreeMap::new(),
            statuses,
            workflows: BTreeMap::new(),
        }
    }

    #[test]
    fn work_buckets_classify_rows_without_cli_renderers() {
        let db = db_with_rows();
        let policy = test_policy();

        let buckets = work_buckets(&db, Some(&policy)).unwrap();

        assert_eq!(buckets.ready[0].id, "atelier-ready");
        assert_eq!(buckets.active[0].id, "atelier-active");
        assert_eq!(buckets.blocked[0].id, "atelier-blocked");
        assert_eq!(buckets.done[0].id, "atelier-done");
    }

    #[test]
    fn status_view_selects_ready_work_before_generic_guidance() {
        let db = db_with_rows();
        let dir = tempdir().unwrap();
        let policy = test_policy();

        let view = status_view(&db, dir.path(), Some(&policy)).unwrap();

        assert_eq!(
            view.next_action,
            StatusNextAction::InspectReadyWork { count: 1 }
        );
        assert_eq!(view.tracker_state, "current");
        assert_eq!(view.active_role_counts.get("worker"), Some(&1));
    }
}
