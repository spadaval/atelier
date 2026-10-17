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
    /// Ready work only the human can do (labelled `needs-human`).
    NeedsHuman,
    /// A mission, or a parent with open children: coordination rather than
    /// something to start, since work happens through its children.
    Context,
    Blocked,
    Backlog,
    Done,
}

impl WorkClassification {
    pub fn label(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Ready => "ready",
            Self::NeedsHuman => "needs human",
            Self::Context => "context",
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkRow {
    pub id: String,
    pub title: String,
    pub status: String,
    pub status_category: Option<String>,
    pub priority: String,
    pub open_blockers: Vec<String>,
    pub classification: WorkClassification,
}

/// The reserved label that marks work only the human can do (being at a
/// machine, an account approval, a judgement call). Agents should not pick
/// such work from the ready list.
pub const NEEDS_HUMAN_LABEL: &str = "needs-human";

pub fn needs_human(labels: &[String]) -> bool {
    labels.iter().any(|label| label == NEEDS_HUMAN_LABEL)
}

/// Open children of `owner` labelled needs-human. A parent integrates only
/// once every child is terminal, so such a child holds the owner's branch off
/// the base until the human acts (atelier-1jll).
pub fn open_human_children(
    index: &RecordIndex,
    policy: &WorkflowPolicy,
    owner: &str,
) -> Vec<String> {
    let Some(record) = index.issue_record(owner) else {
        return Vec::new();
    };
    record
        .relationships
        .children
        .iter()
        .filter_map(|child| index.issue_record(&child.id))
        .filter(|child| {
            needs_human(&child.labels)
                && !policy
                    .issue_status_is_terminal(&child.issue.issue_type, &child.issue.status)
                    .unwrap_or(false)
        })
        .map(|child| child.issue.id.clone())
        .collect()
}

/// What to do about [`open_human_children`].
pub fn human_children_note(owner: &str, children: &[String]) -> String {
    let commands = children
        .iter()
        .map(|child| {
            format!("atelier issue update {child} --no-parent, then atelier issue link {child} {owner} --role blocked_by")
        })
        .collect::<Vec<_>>()
        .join("; ");
    let (verb, pronoun, shape) = if children.len() == 1 {
        ("is", "it", "a standalone validation")
    } else {
        ("are each", "them", "standalone validations")
    };
    format!(
        "{} labelled needs-human {verb} a child of {owner}, and atelier merge needs every child closed, so the branch cannot integrate until the human finishes {pronoun}. If a check runs after the merge (a game, device or release check), make {pronoun} {shape} instead: {commands}.",
        children.join(", "),
    )
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
    /// Ready work only the human can do.
    pub human: Vec<WorkRow>,
    /// Missions and parents with open children, shown for orientation.
    pub context: Vec<WorkRow>,
    pub blocked: Vec<WorkRow>,
    pub backlog: Vec<WorkRow>,
    pub done: Vec<WorkRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusView {
    pub work: WorkBuckets,
    pub current_mission_count: usize,
}

pub fn status_view(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<StatusView> {
    Ok(StatusView {
        work: work_buckets(index, workflow_policy)?,
        current_mission_count: current_mission_count(index, workflow_policy),
    })
}

pub fn work_buckets(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<WorkBuckets> {
    work_buckets_for(index, workflow_policy, index.issues())
}

pub fn work_buckets_for<'a>(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issues: impl IntoIterator<Item = &'a Issue>,
) -> Result<WorkBuckets> {
    let mut buckets = WorkBuckets::default();
    for issue in issues {
        let row = work_row(index, workflow_policy, issue.clone())?;
        match row.state_label() {
            "active" => buckets.active.push(row),
            "ready" => buckets.ready.push(row),
            "needs human" => buckets.human.push(row),
            "context" => buckets.context.push(row),
            "blocked" => buckets.blocked.push(row),
            "done" => buckets.done.push(row),
            _ => buckets.backlog.push(row),
        }
    }
    sort_work_rows(&mut buckets.active);
    sort_work_rows(&mut buckets.ready);
    sort_work_rows(&mut buckets.human);
    sort_work_rows(&mut buckets.context);
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
        status: issue.status,
        status_category: classified.status_category,
        priority: issue.priority,
        open_blockers: classified.open_blockers,
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
    // Only unstarted children are parked; work already in flight stays visible.
    let retired = terminal_ancestor(index, workflow_policy, issue)?.is_some()
        || status_category.as_deref() == Some("todo")
            && parked_parent(index, workflow_policy, issue)?.is_some();

    let classification = if status_category.as_deref() == Some("done") {
        WorkClassification::Done
    } else if retired {
        WorkClassification::Backlog
    } else if issue.issue_type == "decision" && status_category.as_deref() == Some("todo") {
        // Decisions are the human's by default: an open decision has no start
        // transition, but it is waiting on the human, not parked.
        if open_blockers.is_empty() {
            WorkClassification::NeedsHuman
        } else {
            WorkClassification::Blocked
        }
    } else {
        match status_category.as_deref() {
            Some("active") => WorkClassification::Active,
            Some("blocked") => WorkClassification::Blocked,
            Some("todo") => {
                let can_enter_active = match workflow_policy {
                    Some(policy) => policy.can_enter_active(&issue.issue_type, &issue.status)?,
                    None => issue.status == "todo",
                };
                // Parked statuses (draft, backlog) stay backlog even when they
                // have blockers; the blocker count still shows.
                if !can_enter_active {
                    WorkClassification::Backlog
                } else if !open_blockers.is_empty() {
                    WorkClassification::Blocked
                } else {
                    WorkClassification::Ready
                }
            }
            _ if !open_blockers.is_empty() => WorkClassification::Blocked,
            _ if workflow_policy.is_none() && issue.status == "todo" => WorkClassification::Ready,
            _ => WorkClassification::Backlog,
        }
    };
    let classification = if classification != WorkClassification::Ready {
        classification
    } else if is_context(index, workflow_policy, issue) {
        WorkClassification::Context
    } else if index
        .issue_record(&issue.id)
        .is_some_and(|record| needs_human(&record.labels))
    {
        WorkClassification::NeedsHuman
    } else {
        classification
    };
    Ok(ClassifiedWork {
        classification,
        status_category,
        open_blockers,
    })
}

/// Missions coordinate work and own no branch, and a parent with open children
/// is worked through them, so neither is something to pick from a ready list.
fn is_context(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> bool {
    issue.issue_type == "mission"
        || index.issue_record(&issue.id).is_some_and(|record| {
            record.relationships.children.iter().any(|child| {
                index.issue_record(&child.id).is_some_and(|child| {
                    workflow_policy.and_then(|policy| policy.status_category(&child.issue.status))
                        != Some("done")
                })
            })
        })
}

/// The parent, when it is parked: in a todo-category status it cannot start
/// from but can be planned out of (such as `backlog`, which `plan` moves to
/// `todo`). Its unstarted children are parked with it (atelier-28mz, decided
/// by Sumeet 2026-10-15). A parent that can never start, such as an open
/// decision, does not park its children.
pub fn parked_parent(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> Result<Option<String>> {
    let (Some(policy), Some(parent_id)) = (workflow_policy, issue.parent_id.as_deref()) else {
        return Ok(None);
    };
    let parent = index.require_issue(parent_id)?;
    if policy.status_category(&parent.status) != Some("todo")
        || policy.can_enter_active(&parent.issue_type, &parent.status)?
    {
        return Ok(None);
    }
    for (_, transition) in policy.transitions_from_status(&parent.issue_type, &parent.status)? {
        if policy.status_category(&transition.to) == Some("todo")
            && policy.can_enter_active(&parent.issue_type, &transition.to)?
        {
            return Ok(Some(parent.id));
        }
    }
    Ok(None)
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
    Ok(crate::objective_graph::effective_blockers(index, issue_id))
}

fn current_mission_count(index: &RecordIndex, workflow_policy: Option<&WorkflowPolicy>) -> usize {
    index
        .issues()
        .filter(|issue| issue.issue_type == "mission")
        .filter(|issue| {
            workflow_policy.and_then(|policy| policy.status_category(&issue.status)) != Some("done")
        })
        .count()
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
        workflow_by_issue_type.insert("feature".to_string(), "task".to_string());
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
            merge_checks: Vec::new(),
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
    fn current_mission_count_uses_custom_terminal_categories_and_complete_scope() {
        let mut records = records_with_rows();
        records.issue("atelier-live-mission", "mission", "in_progress");
        records.issue("atelier-retired-mission", "mission", "retired");
        let index = records.index();
        let mut policy = test_policy();
        policy.statuses.insert(
            "retired".into(),
            crate::workflow_policy::StatusDefinition {
                category: "done".into(),
            },
        );
        let view = status_view(&index, Some(&policy)).unwrap();
        assert_eq!(view.current_mission_count, 1);
        assert_eq!(view.work.active.len(), 2);
        assert_eq!(view.work.done.len(), 2);
        // Without policy all mission statuses are nonterminal, as before.
        assert_eq!(status_view(&index, None).unwrap().current_mission_count, 2);
    }

    #[test]
    fn status_view_retains_complete_bucket_facts() {
        let index = records_with_rows().index();
        let policy = test_policy();

        let view = status_view(&index, Some(&policy)).unwrap();

        assert_eq!(view.work.ready.len(), 1);
        assert_eq!(view.work.blocked.len(), 1);
        assert_eq!(view.work.active.len(), 1);
        assert_eq!(view.work.done.len(), 1);
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
        records.issue("atelier-epic", "feature", "superseded");
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
