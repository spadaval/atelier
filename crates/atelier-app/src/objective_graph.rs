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

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct DependencyReadinessBatch {
    pub ready_by_issue: std::collections::BTreeMap<String, bool>,
    pub nodes_evaluated: usize,
}

/// Evaluate dependency readiness for many objectives from one cache snapshot.
/// Shared dependency tails are memoized, so each graph node is evaluated at
/// most once for a request.
pub fn dependency_readiness_batch(
    db: &Database,
    policy: &WorkflowPolicy,
    issue_ids: impl IntoIterator<Item = String>,
) -> Result<DependencyReadinessBatch> {
    use std::collections::{BTreeMap, BTreeSet};

    let issues = db.list_issues(Some("all"), None, None)?;
    let terminal = issues
        .iter()
        .map(|issue| {
            Ok((
                issue.id.clone(),
                policy.issue_status_is_terminal(&issue.issue_type, &issue.status)?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    let mut blockers = BTreeMap::<String, Vec<String>>::new();
    for (blocked, blocker) in db.list_issue_dependencies()? {
        if !terminal.contains_key(&blocked) {
            bail!("dependency closure references missing issue {blocked}");
        }
        if !terminal.contains_key(&blocker) {
            bail!("dependency path references missing issue {blocker}");
        }
        blockers.entry(blocked).or_default().push(blocker);
    }

    fn visit(
        id: &str,
        terminal: &BTreeMap<String, bool>,
        blockers: &BTreeMap<String, Vec<String>>,
        memo: &mut BTreeMap<String, bool>,
        visiting: &mut BTreeSet<String>,
        nodes_evaluated: &mut usize,
    ) -> Result<bool> {
        if let Some(ready) = memo.get(id) {
            return Ok(*ready);
        }
        if !visiting.insert(id.to_string()) {
            return Ok(false);
        }
        *nodes_evaluated += 1;
        let mut ready = true;
        for blocker in blockers.get(id).into_iter().flatten() {
            let blocker_terminal = terminal.get(blocker).copied().ok_or_else(|| {
                anyhow::anyhow!("dependency path references missing issue {blocker}")
            })?;
            if !blocker_terminal
                || !visit(blocker, terminal, blockers, memo, visiting, nodes_evaluated)?
            {
                ready = false;
            }
        }
        visiting.remove(id);
        memo.insert(id.to_string(), ready);
        Ok(ready)
    }

    let mut memo = BTreeMap::new();
    let mut ready_by_issue = BTreeMap::new();
    let mut nodes_evaluated = 0;
    for id in issue_ids {
        if !terminal.contains_key(&id) {
            bail!("dependency closure references missing issue {id}");
        }
        let ready = visit(
            &id,
            &terminal,
            &blockers,
            &mut memo,
            &mut BTreeSet::new(),
            &mut nodes_evaluated,
        )?;
        ready_by_issue.insert(id, ready);
    }
    Ok(DependencyReadinessBatch {
        ready_by_issue,
        nodes_evaluated,
    })
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

pub fn dependency_closure_from_canonical(
    policy: &WorkflowPolicy,
    issues: &[Issue],
    dependency_edges: &[(String, String)],
    issue_id: &str,
) -> Result<DependencyClosure> {
    let by_id = issues
        .iter()
        .map(|issue| (issue.id.as_str(), issue))
        .collect::<std::collections::BTreeMap<_, _>>();
    if !by_id.contains_key(issue_id) {
        bail!("dependency closure references missing issue {issue_id}");
    }
    evaluate_dependency_closure(
        issue_id,
        |blocked_id| {
            Ok(dependency_edges
                .iter()
                .filter(|(blocked, _)| blocked == blocked_id)
                .map(|(_, blocker)| blocker.clone())
                .collect())
        },
        |id| {
            let issue = by_id
                .get(id)
                .ok_or_else(|| anyhow::anyhow!("dependency path references missing issue {id}"))?;
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
    use crate::workflow_policy::{
        BranchLifecycleConfig, StatusDefinition, WorkflowDefinition, WorkflowPolicy,
    };
    use atelier_sqlite::{IssueCacheRow, RecordSourceCacheRow};
    use chrono::Utc;
    use std::collections::{BTreeMap, BTreeSet};
    use tempfile::tempdir;

    fn batch_policy() -> WorkflowPolicy {
        let statuses = BTreeMap::from([
            (
                "todo".to_string(),
                StatusDefinition {
                    category: "todo".to_string(),
                    role: None,
                },
            ),
            (
                "done".to_string(),
                StatusDefinition {
                    category: "done".to_string(),
                    role: None,
                },
            ),
        ]);
        WorkflowPolicy {
            schema_version: 3,
            branch_policy: BranchLifecycleConfig::default(),
            issue_types: BTreeMap::new(),
            workflow_by_issue_type: BTreeMap::from([("task".to_string(), "task".to_string())]),
            statuses,
            workflows: BTreeMap::from([(
                "task".to_string(),
                WorkflowDefinition {
                    applies_to: vec!["task".to_string()],
                    initial_status: "todo".to_string(),
                    done_statuses: vec!["done".to_string()],
                    transitions: BTreeMap::new(),
                },
            )]),
        }
    }

    fn index_batch_issue(db: &Database, id: &str, status: &str) {
        let now = Utc::now();
        db.index_issue(
            &IssueCacheRow {
                id: id.to_string(),
                title: id.to_string(),
                status: status.to_string(),
                issue_type: "task".to_string(),
                priority: "medium".to_string(),
                fields: BTreeMap::new(),
                parent_id: None,
                created_at: now,
                updated_at: now,
                closed_at: None,
            },
            &[],
            &[],
            &[],
            &RecordSourceCacheRow {
                path: format!("issues/{id}.md"),
                record_kind: "issue".to_string(),
                record_id: id.to_string(),
                size_bytes: 0,
                modified_micros: None,
                content_hash: None,
                indexed_at: now,
            },
        )
        .unwrap();
    }

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

    #[test]
    fn batch_dependency_readiness_memoizes_shared_graph_for_large_candidate_set() {
        let dir = tempdir().unwrap();
        let db = Database::open(&dir.path().join("state.db")).unwrap();
        let policy = batch_policy();
        index_batch_issue(&db, "atelier-shared", "done");
        let candidates = (0..600)
            .map(|index| format!("atelier-c{index:04}"))
            .collect::<Vec<_>>();
        for id in &candidates {
            index_batch_issue(&db, id, "todo");
            db.add_dependency(id, "atelier-shared").unwrap();
        }
        for index in 0..400 {
            index_batch_issue(&db, &format!("atelier-u{index:04}"), "todo");
        }

        let batch = dependency_readiness_batch(&db, &policy, candidates.clone()).unwrap();

        assert_eq!(batch.ready_by_issue.len(), 600);
        assert!(batch.ready_by_issue.values().all(|ready| *ready));
        assert_eq!(batch.nodes_evaluated, 601);
    }
}
