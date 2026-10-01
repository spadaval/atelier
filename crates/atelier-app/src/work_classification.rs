//! Executable-work classification (active, ready, blocked, backlog, done)
//! shared by status, work queues, and objective rollups.

use anyhow::Result;
use std::collections::BTreeSet;

use atelier_core::Issue;

use crate::record_index::RecordIndex;
use crate::workflow_policy::WorkflowPolicy;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkClassification {
    Active,
    Ready,
    Blocked,
    Backlog,
    Done,
}

impl WorkClassification {
    pub fn label(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Ready => "ready",
            Self::Blocked => "blocked",
            Self::Backlog => "backlog",
            Self::Done => "done",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassifiedWork {
    pub classification: WorkClassification,
    pub status_category: Option<String>,
    pub open_blockers: Vec<String>,
    pub can_enter_active: bool,
    pub terminal_ancestor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkRow {
    pub id: String,
    pub title: String,
    pub issue_type: String,
    pub status: String,
    pub status_category: Option<String>,
    pub priority: String,
    pub open_blockers: Vec<String>,
    pub can_enter_active: bool,
    pub classification: WorkClassification,
}

impl WorkRow {
    pub fn state_label(&self) -> &'static str {
        self.classification.label()
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
    NoSpecificAction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusView {
    pub work: WorkBuckets,
    pub current_missions: Vec<MissionSummary>,
    pub next_action: StatusNextAction,
}

pub fn status_view(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<StatusView> {
    let work = work_buckets(index, workflow_policy)?;
    let current_missions = current_missions(index, workflow_policy);
    let next_action = if !work.ready.is_empty() {
        StatusNextAction::InspectReadyWork {
            count: work.ready.len(),
        }
    } else if !work.blocked.is_empty() {
        StatusNextAction::InspectBlockedWork
    } else {
        StatusNextAction::NoSpecificAction
    };

    Ok(StatusView {
        work,
        current_missions,
        next_action,
    })
}

pub fn work_buckets(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<WorkBuckets> {
    let mut buckets = WorkBuckets::default();
    for issue in index.issues() {
        let row = work_row(index, workflow_policy, issue.clone())?;
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
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: Issue,
) -> Result<WorkRow> {
    let classified = classify_issue(index, workflow_policy, &issue)?;
    Ok(WorkRow {
        id: issue.id.clone(),
        title: issue.title,
        issue_type: issue.issue_type,
        status: issue.status,
        status_category: classified.status_category,
        priority: issue.priority,
        open_blockers: classified.open_blockers,
        can_enter_active: classified.can_enter_active,
        classification: classified.classification,
    })
}

/// Classifies one issue for every executable-work projection.
///
/// Canonical status categories are authoritative. A completed record remains
/// historical evidence; a nonterminal record below a terminal parent becomes
/// context-only backlog; and only todo-category records are refined by
/// blocker and direct-start eligibility.
pub fn classify_issue(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> Result<ClassifiedWork> {
    let status_category = workflow_policy
        .and_then(|policy| policy.status_category(&issue.status))
        .map(str::to_string);
    let open_blockers = open_blockers(index, workflow_policy, &issue.id)?;

    if status_category.as_deref() == Some("done") {
        return Ok(ClassifiedWork {
            classification: WorkClassification::Done,
            status_category,
            open_blockers,
            can_enter_active: false,
            terminal_ancestor: None,
        });
    }

    let terminal_ancestor = terminal_ancestor(index, workflow_policy, issue)?;
    if terminal_ancestor.is_some() {
        return Ok(ClassifiedWork {
            classification: WorkClassification::Backlog,
            status_category,
            open_blockers,
            can_enter_active: false,
            terminal_ancestor,
        });
    }

    let classification = match status_category.as_deref() {
        Some("active") => WorkClassification::Active,
        Some("blocked") => WorkClassification::Blocked,
        Some("todo") => {
            if !open_blockers.is_empty() {
                WorkClassification::Blocked
            } else {
                let can_enter_active = match workflow_policy {
                    Some(policy) => policy.can_enter_active(&issue.issue_type, &issue.status)?,
                    None => issue.status == "todo",
                };
                if can_enter_active {
                    WorkClassification::Ready
                } else {
                    WorkClassification::Backlog
                }
            }
        }
        _ if !open_blockers.is_empty() => WorkClassification::Blocked,
        _ if workflow_policy.is_none() && issue.status == "todo" => WorkClassification::Ready,
        _ => WorkClassification::Backlog,
    };
    let can_enter_active = classification == WorkClassification::Ready;
    Ok(ClassifiedWork {
        classification,
        status_category,
        open_blockers,
        can_enter_active,
        terminal_ancestor: None,
    })
}

/// Returns the nearest done-category hierarchy ancestor, if one exists.
///
/// Completed descendants remain independently terminal and inspectable; callers
/// use this fact only to retire nonterminal descendants from execution.
pub fn terminal_ancestor(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> Result<Option<String>> {
    let mut parent_id = issue.parent_id.clone();
    let mut visited = BTreeSet::new();
    while let Some(id) = parent_id {
        if !visited.insert(id.clone()) {
            anyhow::bail!(
                "issue hierarchy cycle encountered while classifying {}",
                issue.id
            );
        }
        let parent = index.require_issue(&id)?;
        if workflow_policy.and_then(|policy| policy.status_category(&parent.status)) == Some("done")
        {
            return Ok(Some(parent.id));
        }
        parent_id = parent.parent_id;
    }
    Ok(None)
}

fn open_blockers(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issue_id: &str,
) -> Result<Vec<String>> {
    if let Some(policy) = workflow_policy {
        return Ok(
            crate::objective_graph::dependency_closure(index, policy, issue_id)?.blocking_ids(),
        );
    }
    // Without a policy no status is terminal, so every blocker is open.
    Ok(index.get_blockers(issue_id))
}

fn current_missions(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Vec<MissionSummary> {
    let mut missions = index
        .issues()
        .filter(|issue| issue.issue_type == "mission")
        .filter(|issue| {
            workflow_policy.and_then(|policy| policy.status_category(&issue.status)) != Some("done")
        })
        .map(|issue| MissionSummary {
            id: issue.id.clone(),
            title: issue.title.clone(),
            status: issue.status.clone(),
        })
        .collect::<Vec<_>>();
    missions.sort_by(|left, right| left.id.cmp(&right.id));
    missions
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
    use crate::test_records::Records;
    use std::collections::BTreeMap;

    fn records_with_rows() -> Records {
        let mut records = Records::default();
        records
            .issue("atelier-ready", "task", "ready")
            .issue
            .priority = "high".to_string();
        records.issue("atelier-active", "task", "in_progress");
        records.issue("atelier-done", "task", "done").issue.priority = "low".to_string();
        records.issue("atelier-blocked", "task", "todo");
        records.block("atelier-blocked", "atelier-ready");
        records
    }

    fn test_policy() -> WorkflowPolicy {
        let mut statuses = BTreeMap::new();
        statuses.insert(
            "ready".to_string(),
            crate::workflow_policy::StatusDefinition {
                category: "todo".to_string(),
            },
        );
        statuses.insert(
            "todo".to_string(),
            crate::workflow_policy::StatusDefinition {
                category: "todo".to_string(),
            },
        );
        statuses.insert(
            "in_progress".to_string(),
            crate::workflow_policy::StatusDefinition {
                category: "active".to_string(),
            },
        );
        statuses.insert(
            "done".to_string(),
            crate::workflow_policy::StatusDefinition {
                category: "done".to_string(),
            },
        );
        statuses.insert(
            "blocked".to_string(),
            crate::workflow_policy::StatusDefinition {
                category: "blocked".to_string(),
            },
        );
        statuses.insert(
            "superseded".to_string(),
            crate::workflow_policy::StatusDefinition {
                category: "done".to_string(),
            },
        );
        let mut workflow_by_issue_type = BTreeMap::new();
        workflow_by_issue_type.insert("task".to_string(), "task".to_string());
        workflow_by_issue_type.insert("epic".to_string(), "task".to_string());
        let mut workflows = BTreeMap::new();
        workflows.insert(
            "task".to_string(),
            crate::workflow_policy::WorkflowDefinition {
                applies_to: vec!["task".to_string()],
                initial_status: "todo".to_string(),
                done_statuses: vec!["done".to_string()],
                transitions: BTreeMap::from([(
                    "start".to_string(),
                    crate::workflow_policy::TransitionDefinition {
                        from: vec![
                            "ready".to_string(),
                            "todo".to_string(),
                            "blocked".to_string(),
                        ],
                        to: "in_progress".to_string(),
                        required_fields: Vec::new(),
                        description: None,
                        validators: Vec::new(),
                    },
                )]),
            },
        );
        WorkflowPolicy {
            schema_version: 3,
            branch_policy: crate::workflow_policy::BranchLifecycleConfig::default(),
            issue_types: BTreeMap::new(),
            workflow_by_issue_type,
            statuses,
            workflows,
        }
    }

    #[test]
    fn work_buckets_classify_rows_without_cli_renderers() {
        let index = records_with_rows().index();
        let policy = test_policy();

        let buckets = work_buckets(&index, Some(&policy)).unwrap();

        assert_eq!(buckets.ready[0].id, "atelier-ready");
        assert_eq!(buckets.active[0].id, "atelier-active");
        assert_eq!(buckets.blocked[0].id, "atelier-blocked");
        assert_eq!(buckets.done[0].id, "atelier-done");
    }

    #[test]
    fn status_view_selects_ready_work_before_generic_guidance() {
        let index = records_with_rows().index();
        let policy = test_policy();

        let view = status_view(&index, Some(&policy)).unwrap();

        assert_eq!(
            view.next_action,
            StatusNextAction::InspectReadyWork { count: 1 }
        );
    }

    #[test]
    fn explicit_blocked_status_wins_over_direct_start_eligibility() {
        let mut records = Records::default();
        records.issue("atelier-explicit-blocked", "task", "blocked");

        let buckets = work_buckets(&records.index(), Some(&test_policy())).unwrap();

        assert!(buckets.ready.is_empty());
        assert_eq!(buckets.blocked[0].id, "atelier-explicit-blocked");
    }

    #[test]
    fn terminal_ancestor_retires_open_descendant_but_preserves_done_descendant() {
        let mut records = Records::default();
        records.issue("atelier-epic", "epic", "superseded");
        records.issue("atelier-open", "task", "todo");
        records.issue("atelier-history", "task", "done");
        records.parent("atelier-open", "atelier-epic");
        records.parent("atelier-history", "atelier-epic");

        let buckets = work_buckets(&records.index(), Some(&test_policy())).unwrap();

        assert!(buckets.ready.is_empty());
        assert_eq!(buckets.backlog[0].id, "atelier-open");
        assert!(buckets.done.iter().any(|row| row.id == "atelier-history"));
    }
}
