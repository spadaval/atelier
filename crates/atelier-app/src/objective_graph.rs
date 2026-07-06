use anyhow::{bail, Result};
use atelier_core::{Issue, RecordLink};
use atelier_sqlite::Database;
use std::collections::BTreeSet;

use crate::workflow_policy::WorkflowPolicy;

pub fn mission_issue_ids(db: &Database, mission_id: &str) -> Result<BTreeSet<String>> {
    mission_objective_kind(db, mission_id)?;
    issue_descendant_ids(db, mission_id)
}

pub fn mission_objective_kind(db: &Database, mission_id: &str) -> Result<&'static str> {
    if db
        .get_issue(mission_id)?
        .is_some_and(|issue| issue.issue_type == "mission")
    {
        Ok("issue")
    } else {
        bail!("{mission_id} is not a mission objective issue")
    }
}

pub fn issue_descendant_ids(db: &Database, issue_id: &str) -> Result<BTreeSet<String>> {
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

pub fn direct_blocker_ids(db: &Database, kind: &str, id: &str) -> Result<Vec<String>> {
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

pub fn open_issue_blockers(
    db: &Database,
    issue_id: &str,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for blocker_id in db.get_blockers(issue_id)? {
        if issue_blocks_work(workflow_policy, &db.require_issue(&blocker_id)?) {
            blockers.push(blocker_id);
        }
    }
    blockers.sort();
    Ok(blockers)
}

pub fn open_objective_blockers(
    db: &Database,
    objective_kind: &str,
    objective_id: &str,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<Vec<String>> {
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
        .filter(|issue| issue_blocks_work(workflow_policy, issue))
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    open.sort();
    Ok(open)
}

pub fn issue_blocks_work(workflow_policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    workflow_policy.and_then(|policy| policy.status_category(&issue.status)) != Some("done")
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
