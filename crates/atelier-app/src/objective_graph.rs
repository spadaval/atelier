use anyhow::{bail, Context, Result};
use atelier_core::{Issue, RecordLink};
use atelier_sqlite::Database;
use std::collections::{BTreeSet, HashSet};

use crate::workflow_policy::WorkflowPolicy;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct DependencyClosure {
    pub open_paths: Vec<Vec<String>>,
    pub cycle_paths: Vec<Vec<String>>,
}

impl DependencyClosure {
    pub fn is_ready(&self) -> bool {
        self.open_paths.is_empty() && self.cycle_paths.is_empty()
    }

    pub fn blocking_ids(&self) -> Vec<String> {
        let mut ids = self
            .open_paths
            .iter()
            .filter_map(|path| path.last().cloned())
            .collect::<BTreeSet<_>>();
        for path in &self.cycle_paths {
            if let Some(id) = path.last() {
                ids.insert(id.clone());
            }
        }
        ids.into_iter().collect()
    }

    pub fn failure_reason(&self) -> Option<String> {
        if self.is_ready() {
            return None;
        }
        let mut reasons = Vec::new();
        if !self.open_paths.is_empty() {
            reasons.push(format!(
                "incomplete dependency path(s): {}",
                format_dependency_paths(&self.open_paths)
            ));
        }
        if !self.cycle_paths.is_empty() {
            reasons.push(format!(
                "dependency cycle(s): {}; repair the declared blocked_by relationships",
                format_dependency_paths(&self.cycle_paths)
            ));
        }
        Some(reasons.join("; "))
    }
}

pub fn dependency_closure(
    db: &Database,
    policy: &WorkflowPolicy,
    issue_id: &str,
) -> Result<DependencyClosure> {
    db.require_issue(issue_id)?;
    evaluate_dependency_closure(
        issue_id,
        |id| db.get_blockers(id),
        |id| {
            let issue = db
                .require_issue(id)
                .with_context(|| format!("dependency path references missing issue {id}"))?;
            policy.issue_status_is_terminal(&issue.issue_type, &issue.status)
        },
    )
}

fn evaluate_dependency_closure(
    issue_id: &str,
    mut blockers_for: impl FnMut(&str) -> Result<Vec<String>>,
    mut is_terminal: impl FnMut(&str) -> Result<bool>,
) -> Result<DependencyClosure> {
    fn visit(
        current: &str,
        blockers_for: &mut impl FnMut(&str) -> Result<Vec<String>>,
        is_terminal: &mut impl FnMut(&str) -> Result<bool>,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        closure: &mut DependencyClosure,
    ) -> Result<()> {
        visited.insert(current.to_string());
        let mut blockers = blockers_for(current)?;
        blockers.sort();
        blockers.dedup();
        for blocker in blockers {
            if path.iter().any(|id| id == &blocker) {
                let mut cycle_path = path[..].to_vec();
                cycle_path.push(blocker);
                closure.cycle_paths.push(cycle_path);
                continue;
            }

            path.push(blocker.clone());
            if !is_terminal(&blocker)? {
                closure.open_paths.push(path.clone());
            }
            if !visited.contains(&blocker) {
                visit(&blocker, blockers_for, is_terminal, visited, path, closure)?;
            }
            path.pop();
        }
        Ok(())
    }

    let mut closure = DependencyClosure::default();
    let mut visited = HashSet::new();
    let mut path = vec![issue_id.to_string()];
    visit(
        issue_id,
        &mut blockers_for,
        &mut is_terminal,
        &mut visited,
        &mut path,
        &mut closure,
    )?;
    closure.open_paths.sort();
    closure.open_paths.dedup();
    closure.cycle_paths.sort();
    closure.cycle_paths.dedup();
    Ok(closure)
}

fn format_dependency_paths(paths: &[Vec<String>]) -> String {
    paths
        .iter()
        .map(|path| path.join(" -> "))
        .collect::<Vec<_>>()
        .join(", ")
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn dependency_closure_reports_complete_direct_and_transitive_paths() {
        let graph = BTreeMap::from([
            ("mission", vec!["direct"]),
            ("direct", vec!["transitive"]),
            ("transitive", Vec::new()),
        ]);
        let terminal = BTreeSet::from(["direct"]);

        let closure = evaluate_dependency_closure(
            "mission",
            |id| {
                Ok(graph
                    .get(id)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .map(str::to_string)
                    .collect())
            },
            |id| Ok(terminal.contains(id)),
        )
        .unwrap();

        assert_eq!(
            closure.open_paths,
            vec![vec![
                "mission".to_string(),
                "direct".to_string(),
                "transitive".to_string(),
            ]]
        );
        assert!(closure.cycle_paths.is_empty());
    }

    #[test]
    fn dependency_closure_fails_safely_with_an_actionable_cycle_path() {
        let graph = BTreeMap::from([
            ("consumer", vec!["first"]),
            ("first", vec!["second"]),
            ("second", vec!["first"]),
        ]);

        let closure = evaluate_dependency_closure(
            "consumer",
            |id| {
                Ok(graph
                    .get(id)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .map(str::to_string)
                    .collect())
            },
            |_id| Ok(true),
        )
        .unwrap();

        assert!(!closure.is_ready());
        assert_eq!(
            closure.cycle_paths,
            vec![vec![
                "consumer".to_string(),
                "first".to_string(),
                "second".to_string(),
                "first".to_string(),
            ]]
        );
        assert_eq!(
            closure.failure_reason().as_deref(),
            Some(
                "dependency cycle(s): consumer -> first -> second -> first; repair the declared blocked_by relationships"
            )
        );
    }
}
