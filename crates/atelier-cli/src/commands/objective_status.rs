use anyhow::{bail, Result};
use std::collections::BTreeSet;

use crate::commands;
use crate::commands::work_order::WorkOrderRow;
use atelier_app::workflow_policy::WorkflowPolicy;
use atelier_core::{Issue, RecordLink};
use atelier_sqlite::Database;

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
    db: &Database,
    issue_id: &str,
    active_issue_ids: &BTreeSet<&str>,
) -> Result<ObjectiveStatusSnapshot> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let mut snapshot = ObjectiveStatusSnapshot {
        issue_ids: issue_descendant_ids(db, issue_id)?,
        open_blockers: open_issue_objective_blockers(db, issue_id)?,
        ..ObjectiveStatusSnapshot::default()
    };

    for child_id in &snapshot.issue_ids {
        let Some(issue) = db.get_issue(child_id)? else {
            continue;
        };
        match issue_bucket(db, &issue, active_issue_ids, workflow_policy.as_ref())? {
            ObjectiveIssueBucket::Active => {
                snapshot.active += 1;
                snapshot.active_issues.push(issue);
            }
            ObjectiveIssueBucket::Ready => {
                snapshot.ready += 1;
                if is_selectable_work(db, &issue)? {
                    snapshot.selectable_issues.push(issue.clone());
                }
                snapshot.ready_issues.push(issue);
            }
            ObjectiveIssueBucket::Blocked => {
                snapshot.blocked += 1;
                snapshot.blocked_issues.push(issue);
            }
            ObjectiveIssueBucket::Done => snapshot.done += 1,
            ObjectiveIssueBucket::Backlog => snapshot.backlog += 1,
        }
    }

    snapshot.active_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.active_issues)?;
    snapshot.ready_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.ready_issues)?;
    snapshot.selectable_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.selectable_issues)?;
    snapshot.blocked_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.blocked_issues)?;
    Ok(snapshot)
}

pub(crate) fn snapshot_for_mission(
    db: &Database,
    mission_id: &str,
    active_issue_ids: &BTreeSet<&str>,
) -> Result<ObjectiveStatusSnapshot> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let objective_kind = mission_objective_kind(db, mission_id)?;
    let mut snapshot = ObjectiveStatusSnapshot {
        issue_ids: mission_issue_ids(db, mission_id)?,
        open_blockers: open_objective_blockers(db, objective_kind, mission_id)?,
        ..ObjectiveStatusSnapshot::default()
    };

    for issue_id in &snapshot.issue_ids {
        let Some(issue) = db.get_issue(issue_id)? else {
            continue;
        };
        match issue_bucket(db, &issue, active_issue_ids, workflow_policy.as_ref())? {
            ObjectiveIssueBucket::Active => {
                snapshot.active += 1;
                snapshot.active_issues.push(issue);
            }
            ObjectiveIssueBucket::Ready => {
                snapshot.ready += 1;
                if is_selectable_work(db, &issue)? {
                    snapshot.selectable_issues.push(issue.clone());
                }
                snapshot.ready_issues.push(issue);
            }
            ObjectiveIssueBucket::Blocked => {
                snapshot.blocked += 1;
                snapshot.blocked_issues.push(issue);
            }
            ObjectiveIssueBucket::Done => snapshot.done += 1,
            ObjectiveIssueBucket::Backlog => snapshot.backlog += 1,
        }
    }

    snapshot.active_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.active_issues)?;
    snapshot.ready_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.ready_issues)?;
    snapshot.selectable_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.selectable_issues)?;
    snapshot.blocked_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.blocked_issues)?;
    Ok(snapshot)
}

pub(crate) fn issue_bucket(
    db: &Database,
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
    if !open_issue_blockers(db, &issue.id, workflow_policy)?.is_empty() {
        return Ok(ObjectiveIssueBucket::Blocked);
    }
    match issue_state(db, workflow_policy, issue)? {
        "ready" => Ok(ObjectiveIssueBucket::Ready),
        _ => Ok(ObjectiveIssueBucket::Backlog),
    }
}

pub(crate) fn issue_state(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> Result<&'static str> {
    Ok(work_order_row_for_issue(db, workflow_policy, issue)?
        .state()
        .label())
}

pub(crate) fn order_issues_by_work(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issues: Vec<Issue>,
) -> Result<Vec<Issue>> {
    let rows = issues
        .iter()
        .map(|issue| work_order_row_for_issue(db, workflow_policy, issue))
        .collect::<Result<Vec<_>>>()?;
    let mut keyed = issues.into_iter().map(Some).collect::<Vec<_>>();
    Ok(commands::work_order::ordered_work_indices(&rows)
        .into_iter()
        .filter_map(|index| keyed[index].take())
        .collect())
}

pub(crate) fn order_issues_by_work_with_default(
    db: &Database,
    issues: Vec<Issue>,
) -> Result<Vec<Issue>> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    order_issues_by_work(db, workflow_policy.as_ref(), issues)
}

pub(crate) fn work_order_row_for_issue(
    db: &Database,
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
        open_blockers: open_issue_blockers(db, &issue.id, workflow_policy)?,
    })
}

pub(crate) fn work_order_row_for_issue_with_default(
    db: &Database,
    issue: &Issue,
) -> Result<WorkOrderRow> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    work_order_row_for_issue(db, workflow_policy.as_ref(), issue)
}

pub(crate) fn open_issue_blockers(
    db: &Database,
    issue_id: &str,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for blocker_id in db.get_blockers(issue_id)? {
        if commands::issue_workflow::issue_blocks_work(
            workflow_policy,
            &db.require_issue(&blocker_id)?,
        ) {
            blockers.push(blocker_id);
        }
    }
    blockers.sort();
    Ok(blockers)
}

pub(crate) fn open_issue_blockers_with_default(
    db: &Database,
    issue_id: &str,
) -> Result<Vec<String>> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    open_issue_blockers(db, issue_id, workflow_policy.as_ref())
}

pub(crate) fn open_objective_blockers(
    db: &Database,
    objective_kind: &str,
    objective_id: &str,
) -> Result<Vec<String>> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let mut blocker_ids = direct_blocker_ids(db, objective_kind, objective_id)?
        .into_iter()
        .collect::<BTreeSet<_>>();
    for issue_id in mission_issue_ids(db, objective_id)? {
        for blocker_id in db.get_blockers(&issue_id)? {
            blocker_ids.insert(blocker_id);
        }
    }
    let mut open = blocker_ids
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter(|issue| {
            commands::issue_workflow::issue_blocks_work(workflow_policy.as_ref(), issue)
        })
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    open.sort();
    Ok(open)
}

pub(crate) fn open_issue_objective_blockers(db: &Database, issue_id: &str) -> Result<Vec<String>> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let mut blocker_ids = db
        .get_blockers(issue_id)?
        .into_iter()
        .collect::<BTreeSet<_>>();
    for child_id in issue_descendant_ids(db, issue_id)? {
        for blocker_id in db.get_blockers(&child_id)? {
            blocker_ids.insert(blocker_id);
        }
    }
    let mut open = blocker_ids
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter(|issue| {
            commands::issue_workflow::issue_blocks_work(workflow_policy.as_ref(), issue)
        })
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    open.sort();
    Ok(open)
}

pub(crate) fn open_objective_work(db: &Database, mission_id: &str) -> Result<Vec<String>> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let mut open = mission_issue_ids(db, mission_id)?
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter(|issue| {
            commands::issue_workflow::issue_blocks_work(workflow_policy.as_ref(), issue)
        })
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    open.sort();
    Ok(open)
}

pub(crate) fn mission_issue_ids(db: &Database, mission_id: &str) -> Result<BTreeSet<String>> {
    mission_objective_kind(db, mission_id)?;
    issue_descendant_ids(db, mission_id)
}

pub(crate) fn mission_objective_kind(db: &Database, mission_id: &str) -> Result<&'static str> {
    if db
        .get_issue(mission_id)?
        .is_some_and(|issue| issue.issue_type == "mission")
    {
        Ok("issue")
    } else {
        bail!("{mission_id} is not a mission objective issue")
    }
}

pub(crate) fn issue_descendant_ids(db: &Database, issue_id: &str) -> Result<BTreeSet<String>> {
    let mut issue_ids = BTreeSet::new();
    for child in db.get_subissues(issue_id)? {
        collect_issue_and_descendants(db, &child.id, &mut issue_ids)?;
    }
    let follows_advances = db
        .get_issue(issue_id)?
        .is_some_and(|issue| issue.issue_type == "mission");
    if !follows_advances {
        return Ok(issue_ids);
    }
    for relation in db.get_typed_relations(issue_id)? {
        if relation.relation_type != "advances" {
            continue;
        }
        let linked_id = if relation.issue_id_1 == issue_id {
            relation.issue_id_2
        } else {
            relation.issue_id_1
        };
        collect_issue_and_descendants(db, &linked_id, &mut issue_ids)?;
    }
    Ok(issue_ids)
}

pub(crate) fn direct_blocker_ids(db: &Database, kind: &str, id: &str) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for link in db.list_record_links(kind, id)? {
        if link.relation_type != "blocked_by" {
            continue;
        }
        let Some((linked_kind, linked_id)) = other_side(&link, kind, id) else {
            continue;
        };
        if linked_kind == "issue" {
            blockers.push(linked_id.to_string());
        }
    }
    Ok(blockers)
}

pub(crate) fn is_selectable_work(db: &Database, issue: &Issue) -> Result<bool> {
    Ok(issue.issue_type != "epic" || db.get_subissues(&issue.id)?.is_empty())
}

pub(crate) fn parent_context(issue: &Issue) -> String {
    match issue.parent_id.as_deref() {
        Some(parent_id) => format!("parent {parent_id}"),
        None => "mission-linked root".to_string(),
    }
}

pub(crate) fn proof_context(_db: &Database, _issue_id: &str) -> Result<&'static str> {
    Ok("proof checked by workflow validators")
}

pub(crate) fn has_validating_evidence(db: &Database, issue_id: &str) -> Result<bool> {
    for link in db.list_record_links("issue", issue_id)? {
        if link.relation_type != "validates" {
            continue;
        }
        if link.source_kind == "evidence" || link.target_kind == "evidence" {
            return Ok(true);
        }
    }
    Ok(false)
}

fn collect_issue_and_descendants(
    db: &Database,
    issue_id: &str,
    issue_ids: &mut BTreeSet<String>,
) -> Result<()> {
    if !issue_ids.insert(issue_id.to_string()) {
        return Ok(());
    }
    for child in db.get_subissues(issue_id)? {
        collect_issue_and_descendants(db, &child.id, issue_ids)?;
    }
    Ok(())
}

fn other_side<'a>(link: &'a RecordLink, kind: &str, id: &str) -> Option<(&'a str, &'a str)> {
    if link.source_kind == kind && link.source_id == id {
        Some((&link.target_kind, &link.target_id))
    } else if link.target_kind == kind && link.target_id == id {
        Some((&link.source_kind, &link.source_id))
    } else {
        None
    }
}
