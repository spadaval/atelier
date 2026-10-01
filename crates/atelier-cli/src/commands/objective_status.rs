use anyhow::{bail, Result};
use atelier_app::record_index::RecordIndex;
use std::collections::BTreeSet;

use crate::commands;
use crate::commands::work_order::WorkOrderRow;
use atelier_app::workflow_policy::WorkflowPolicy;
use atelier_core::Issue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectiveIssueBucket {
    Active,
    Ready,
    Blocked,
    Done,
    Backlog,
}

#[derive(Default)]
pub(crate) struct ObjectiveStatusSnapshot {
    pub(crate) issue_ids: BTreeSet<String>,
    pub(crate) active_issues: Vec<Issue>,
    pub(crate) ready_issues: Vec<Issue>,
    pub(crate) selectable_issues: Vec<Issue>,
    pub(crate) blocked_issues: Vec<Issue>,
    pub(crate) backlog_issues: Vec<Issue>,
    pub(crate) open_blockers: Vec<String>,
    pub(crate) active: usize,
    pub(crate) ready: usize,
    pub(crate) blocked: usize,
    pub(crate) done: usize,
    pub(crate) backlog: usize,
}

impl ObjectiveStatusSnapshot {
    pub(crate) fn health(&self) -> &'static str {
        if !self.open_blockers.is_empty() || self.blocked > 0 {
            "blocked"
        } else if self.active > 0 {
            "active"
        } else if self.ready > 0 {
            "ready"
        } else if self.done > 0 {
            "terminal"
        } else {
            "steady"
        }
    }
}

pub(crate) fn snapshot_for_issue_objective(
    index: &RecordIndex,
    issue_id: &str,
    active_issue_ids: &BTreeSet<&str>,
) -> Result<ObjectiveStatusSnapshot> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let mut snapshot = ObjectiveStatusSnapshot {
        issue_ids: issue_descendant_ids(index, issue_id)?,
        open_blockers: open_issue_objective_blockers(index, issue_id)?,
        ..ObjectiveStatusSnapshot::default()
    };

    for child_id in &snapshot.issue_ids {
        let Some(issue) = index.get_issue(child_id) else {
            continue;
        };
        match issue_bucket(index, &issue, active_issue_ids, workflow_policy.as_ref())? {
            ObjectiveIssueBucket::Active => {
                snapshot.active += 1;
                snapshot.active_issues.push(issue);
            }
            ObjectiveIssueBucket::Ready => {
                snapshot.ready += 1;
                if is_selectable_work(index, &issue)? {
                    snapshot.selectable_issues.push(issue.clone());
                }
                snapshot.ready_issues.push(issue);
            }
            ObjectiveIssueBucket::Blocked => {
                snapshot.blocked += 1;
                snapshot.blocked_issues.push(issue);
            }
            ObjectiveIssueBucket::Done => snapshot.done += 1,
            ObjectiveIssueBucket::Backlog => {
                snapshot.backlog += 1;
                snapshot.backlog_issues.push(issue);
            }
        }
    }

    snapshot.active_issues =
        order_issues_by_work(index, workflow_policy.as_ref(), snapshot.active_issues)?;
    snapshot.ready_issues =
        order_issues_by_work(index, workflow_policy.as_ref(), snapshot.ready_issues)?;
    snapshot.selectable_issues =
        order_issues_by_work(index, workflow_policy.as_ref(), snapshot.selectable_issues)?;
    snapshot.blocked_issues =
        order_issues_by_work(index, workflow_policy.as_ref(), snapshot.blocked_issues)?;
    snapshot.backlog_issues =
        order_issues_by_work(index, workflow_policy.as_ref(), snapshot.backlog_issues)?;
    Ok(snapshot)
}

pub(crate) fn issue_bucket(
    index: &RecordIndex,
    issue: &Issue,
    active_issue_ids: &BTreeSet<&str>,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<ObjectiveIssueBucket> {
    if active_issue_ids.contains(issue.id.as_str()) {
        return Ok(ObjectiveIssueBucket::Active);
    }
    if commands::issue_workflow::issue_is_done(workflow_policy, issue) {
        return Ok(ObjectiveIssueBucket::Done);
    }
    if !open_issue_blockers(index, &issue.id, workflow_policy)?.is_empty() {
        return Ok(ObjectiveIssueBucket::Blocked);
    }
    match issue_state(index, workflow_policy, issue)? {
        "ready" => Ok(ObjectiveIssueBucket::Ready),
        _ => Ok(ObjectiveIssueBucket::Backlog),
    }
}

pub(crate) fn issue_state(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> Result<&'static str> {
    Ok(work_order_row_for_issue(index, workflow_policy, issue)?
        .state()
        .label())
}

pub(crate) fn order_issues_by_work(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issues: Vec<Issue>,
) -> Result<Vec<Issue>> {
    let rows = issues
        .iter()
        .map(|issue| work_order_row_for_issue(index, workflow_policy, issue))
        .collect::<Result<Vec<_>>>()?;
    let mut keyed = issues.into_iter().map(Some).collect::<Vec<_>>();
    Ok(commands::work_order::ordered_work_indices(&rows)
        .into_iter()
        .filter_map(|index| keyed[index].take())
        .collect())
}

pub(crate) fn work_order_row_for_issue(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> Result<WorkOrderRow> {
    Ok(WorkOrderRow {
        id: issue.id.clone(),
        status_category: commands::issue_workflow::issue_status_category(
            workflow_policy,
            &issue.status,
        ),
        priority: issue.priority.clone(),
        updated_at: issue.updated_at,
        open_blockers: open_issue_blockers(index, &issue.id, workflow_policy)?,
    })
}

pub(crate) fn open_issue_blockers(
    index: &RecordIndex,
    issue_id: &str,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for blocker_id in index.get_blockers(issue_id) {
        if commands::issue_workflow::issue_blocks_work(
            workflow_policy,
            &index.require_issue(&blocker_id)?,
        ) {
            blockers.push(blocker_id);
        }
    }
    blockers.sort();
    Ok(blockers)
}

pub(crate) fn open_issue_blockers_with_default(
    index: &RecordIndex,
    issue_id: &str,
) -> Result<Vec<String>> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    open_issue_blockers(index, issue_id, workflow_policy.as_ref())
}

pub(crate) fn open_issue_objective_blockers(
    index: &RecordIndex,
    issue_id: &str,
) -> Result<Vec<String>> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let mut blocker_ids = index
        .get_blockers(issue_id)
        .into_iter()
        .collect::<BTreeSet<_>>();
    for child_id in issue_descendant_ids(index, issue_id)? {
        for blocker_id in index.get_blockers(&child_id) {
            blocker_ids.insert(blocker_id);
        }
    }
    let mut open = blocker_ids
        .into_iter()
        .filter_map(|id| index.get_issue(&id))
        .filter(|issue| {
            commands::issue_workflow::issue_blocks_work(workflow_policy.as_ref(), issue)
        })
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    open.sort();
    Ok(open)
}

pub(crate) fn mission_issue_ids(index: &RecordIndex, mission_id: &str) -> Result<BTreeSet<String>> {
    mission_objective_kind(index, mission_id)?;
    issue_descendant_ids(index, mission_id)
}

pub(crate) fn mission_objective_kind(
    index: &RecordIndex,
    mission_id: &str,
) -> Result<&'static str> {
    if index
        .get_issue(mission_id)
        .is_some_and(|issue| issue.issue_type == "mission")
    {
        Ok("issue")
    } else {
        bail!("{mission_id} is not a mission objective issue")
    }
}

pub(crate) fn issue_descendant_ids(
    index: &RecordIndex,
    issue_id: &str,
) -> Result<BTreeSet<String>> {
    let mut issue_ids = BTreeSet::new();
    for child in index.get_subissues(issue_id) {
        collect_issue_and_descendants(index, &child.id, &mut issue_ids)?;
    }
    let follows_advances = index
        .get_issue(issue_id)
        .is_some_and(|issue| issue.issue_type == "mission");
    if !follows_advances {
        return Ok(issue_ids);
    }
    for relation in index.get_typed_relations(issue_id) {
        if relation.relation_type != "advances" {
            continue;
        }
        let linked_id = if relation.issue_id_1 == issue_id {
            relation.issue_id_2
        } else {
            relation.issue_id_1
        };
        collect_issue_and_descendants(index, &linked_id, &mut issue_ids)?;
    }
    Ok(issue_ids)
}

pub(crate) fn is_selectable_work(index: &RecordIndex, issue: &Issue) -> Result<bool> {
    Ok(issue.issue_type != "epic" || index.get_subissues(&issue.id).is_empty())
}

pub(crate) fn parent_context(issue: &Issue) -> String {
    match issue.parent_id.as_deref() {
        Some(parent_id) => format!("parent {parent_id}"),
        None => "mission-linked root".to_string(),
    }
}

pub(crate) fn proof_context(_index: &RecordIndex, _issue_id: &str) -> Result<&'static str> {
    Ok("proof checked by workflow validators")
}

fn collect_issue_and_descendants(
    index: &RecordIndex,
    issue_id: &str,
    issue_ids: &mut BTreeSet<String>,
) -> Result<()> {
    if !issue_ids.insert(issue_id.to_string()) {
        return Ok(());
    }
    for child in index.get_subissues(issue_id) {
        collect_issue_and_descendants(index, &child.id, issue_ids)?;
    }
    Ok(())
}
