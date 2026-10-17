use anyhow::{bail, Context, Result};
use atelier_core::Issue;
use std::collections::{BTreeSet, HashSet};

use crate::record_index::RecordIndex;
use crate::workflow_policy::WorkflowPolicy;

/// The parent's blockers a child also waits on before it can start, since it
/// works on the parent's branch (atelier-e4c5). A parent blocked by one of its
/// own children sequences its children; that does not block the children.
pub fn inherited_blockers(index: &RecordIndex, issue_id: &str) -> Option<(String, Vec<String>)> {
    let parent = index.get_issue(issue_id)?.parent_id?;
    let blockers = index.get_blockers(&parent);
    if blockers.is_empty() {
        return None;
    }
    let siblings = index
        .issue_record(&parent)
        .map(|record| {
            record
                .relationships
                .children
                .iter()
                .map(|child| child.id.as_str())
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let blockers = blockers
        .into_iter()
        .filter(|blocker| blocker != issue_id && !siblings.contains(blocker.as_str()))
        .collect::<Vec<_>>();
    (!blockers.is_empty()).then_some((parent, blockers))
}

/// An issue's own blockers plus those it inherits from its parent.
pub fn effective_blockers(index: &RecordIndex, issue_id: &str) -> Vec<String> {
    let mut blockers = index.get_blockers(issue_id);
    if let Some((_, inherited)) = inherited_blockers(index, issue_id) {
        blockers.extend(inherited);
    }
    blockers.sort();
    blockers.dedup();
    blockers
}

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

/// Evaluate dependency readiness for many objectives from one record index.
/// Shared dependency tails are memoized, so each graph node is evaluated at
/// most once for a request.
pub fn dependency_readiness_batch(
    index: &RecordIndex,
    policy: &WorkflowPolicy,
    issue_ids: impl IntoIterator<Item = String>,
) -> Result<DependencyReadinessBatch> {
    use std::collections::{BTreeMap, BTreeSet};

    let terminal = index
        .issues()
        .map(|issue| {
            Ok((
                issue.id.clone(),
                policy.issue_status_is_terminal(&issue.issue_type, &issue.status)?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    let blockers = terminal
        .keys()
        .map(|id| (id.clone(), index.get_blockers(id)))
        .collect::<BTreeMap<_, _>>();

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
        let mut ready = visit(
            &id,
            &terminal,
            &blockers,
            &mut memo,
            &mut BTreeSet::new(),
            &mut nodes_evaluated,
        )?;
        // Only the issue itself inherits its parent's blockers; the walk
        // beyond them uses each blocker's own.
        for blocker in inherited_blockers(index, &id)
            .into_iter()
            .flat_map(|(_, b)| b)
        {
            let blocker_terminal = terminal.get(&blocker).copied().ok_or_else(|| {
                anyhow::anyhow!("dependency path references missing issue {blocker}")
            })?;
            if !blocker_terminal
                || !visit(
                    &blocker,
                    &terminal,
                    &blockers,
                    &mut memo,
                    &mut BTreeSet::new(),
                    &mut nodes_evaluated,
                )?
            {
                ready = false;
            }
        }
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
                "{}: {}",
                atelier_core::count_noun(self.open_paths.len(), "incomplete dependency path"),
                format_dependency_paths(&self.open_paths)
            ));
        }
        if !self.cycle_paths.is_empty() {
            reasons.push(format!(
                "{}: {}; repair the declared blocked_by relationships",
                atelier_core::count_noun(self.cycle_paths.len(), "dependency cycle"),
                format_dependency_paths(&self.cycle_paths)
            ));
        }
        Some(reasons.join("; "))
    }
}

pub fn dependency_closure(
    index: &RecordIndex,
    policy: &WorkflowPolicy,
    issue_id: &str,
) -> Result<DependencyClosure> {
    index.require_issue(issue_id)?;
    let is_terminal = |id: &str| {
        let issue = index
            .issue_record(id)
            .with_context(|| format!("dependency path references missing issue {id}"))?;
        policy.issue_status_is_terminal(&issue.issue.issue_type, &issue.issue.status)
    };
    let mut closure =
        evaluate_dependency_closure(issue_id, |id| Ok(index.get_blockers(id)), is_terminal)?;
    // Inherited blockers are reported through the parent: child -> parent -> blocker.
    if let Some((parent, inherited)) = inherited_blockers(index, issue_id) {
        let via = evaluate_dependency_closure(
            &parent,
            |id| {
                Ok(if id == parent {
                    inherited.clone()
                } else {
                    index.get_blockers(id)
                })
            },
            is_terminal,
        )?;
        let prefixed = |paths: Vec<Vec<String>>| {
            paths.into_iter().map(|path| {
                std::iter::once(issue_id.to_string())
                    .chain(path)
                    .collect::<Vec<_>>()
            })
        };
        closure.open_paths.extend(prefixed(via.open_paths));
        closure.cycle_paths.extend(prefixed(via.cycle_paths));
    }
    Ok(closure)
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

pub fn mission_issue_ids(index: &RecordIndex, mission_id: &str) -> Result<BTreeSet<String>> {
    mission_objective_kind(index, mission_id)?;
    issue_descendant_ids(index, mission_id)
}

/// The first mission, in index order, whose `advances` scope contains
/// `issue_id`.
pub fn containing_mission(index: &RecordIndex, issue_id: &str) -> Result<Option<String>> {
    for mission in index.issues() {
        if mission.issue_type == "mission"
            && mission_issue_ids(index, &mission.id)?.contains(issue_id)
        {
            return Ok(Some(mission.id.clone()));
        }
    }
    Ok(None)
}

pub fn mission_objective_kind(index: &RecordIndex, mission_id: &str) -> Result<&'static str> {
    if index
        .issue_record(mission_id)
        .is_some_and(|record| record.issue.issue_type == "mission")
    {
        Ok("issue")
    } else {
        bail!("{mission_id} is not a mission objective issue")
    }
}

pub fn issue_descendant_ids(index: &RecordIndex, issue_id: &str) -> Result<BTreeSet<String>> {
    let mut issue_ids = BTreeSet::new();
    for child_id in index.child_ids(issue_id) {
        collect_issue_and_descendants(index, child_id, &mut issue_ids)?;
    }
    let follows_advances = index
        .issue_record(issue_id)
        .is_some_and(|record| record.issue.issue_type == "mission");
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

pub fn issue_blocks_work(workflow_policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    workflow_policy.and_then(|policy| policy.status_category(&issue.status)) != Some("done")
}

fn collect_issue_and_descendants(
    index: &RecordIndex,
    issue_id: &str,
    issue_ids: &mut BTreeSet<String>,
) -> Result<()> {
    if !issue_ids.insert(issue_id.to_string()) {
        return Ok(());
    }
    for child_id in index.child_ids(issue_id) {
        collect_issue_and_descendants(index, child_id, issue_ids)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_records::Records;
    use crate::workflow_policy::{
        BranchLifecycleConfig, StatusDefinition, WorkflowDefinition, WorkflowPolicy,
    };
    use std::collections::{BTreeMap, BTreeSet};

    fn batch_policy() -> WorkflowPolicy {
        let statuses = BTreeMap::from([
            (
                "todo".to_string(),
                StatusDefinition {
                    category: "todo".to_string(),
                },
            ),
            (
                "done".to_string(),
                StatusDefinition {
                    category: "done".to_string(),
                },
            ),
        ]);
        WorkflowPolicy {
            merge_checks: Vec::new(),
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
                "1 dependency cycle: consumer -> first -> second -> first; repair the declared blocked_by relationships"
            )
        );
    }

    #[test]
    fn batch_dependency_readiness_memoizes_shared_graph_for_large_candidate_set() {
        let mut records = Records::default();
        let policy = batch_policy();
        records.issue("atelier-shared", "task", "done");
        let candidates = (0..600)
            .map(|index| format!("atelier-c{index:04}"))
            .collect::<Vec<_>>();
        for id in &candidates {
            records.issue(id, "task", "todo");
            records.block(id, "atelier-shared");
        }
        for index in 0..400 {
            records.issue(&format!("atelier-u{index:04}"), "task", "todo");
        }

        let batch =
            dependency_readiness_batch(&records.index(), &policy, candidates.clone()).unwrap();

        assert_eq!(batch.ready_by_issue.len(), 600);
        assert!(batch.ready_by_issue.values().all(|ready| *ready));
        assert_eq!(batch.nodes_evaluated, 601);
    }
}
