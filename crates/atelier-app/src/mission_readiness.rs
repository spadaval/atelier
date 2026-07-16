use anyhow::Result;
use atelier_records::mission_plan_review::{
    mission_plan_review_state, mission_plan_review_state_from_records, MissionPlanFindingSeverity,
    MissionPlanReviewFreshness,
};
use atelier_records::{CanonicalIssueRecord, RecordStore};
use atelier_sqlite::Database;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::objective_graph::dependency_closure;
use crate::workflow_policy::WorkflowPolicy;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MissionReadinessKind {
    ReviewNotRequested,
    ApprovalMissing,
    BlockingFinding,
    ChangeRequest,
    StaleApproval,
    DirectBlocker,
    TransitiveBlocker,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MissionReadinessDiagnosis {
    pub kind: MissionReadinessKind,
    pub mission_id: String,
    pub summary: String,
    pub next_command: String,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct MissionExecutionAllowances {
    pub allowed_by_issue: BTreeMap<String, bool>,
    pub missions_evaluated: usize,
    pub canonical_records_loaded: usize,
}

/// Build review allowances only for candidate work in one request.
///
/// Mission membership and dependency edges come from the decision-safe cache;
/// material revision content is loaded canonically only for relevant mission
/// scopes and dependency endpoints.
pub fn execution_allowances_for_candidates(
    db: &Database,
    state_dir: &Path,
    policy: &WorkflowPolicy,
    candidate_ids: &BTreeSet<String>,
) -> Result<MissionExecutionAllowances> {
    if candidate_ids.is_empty() {
        return Ok(MissionExecutionAllowances::default());
    }
    let dependency_edges = db.list_issue_dependencies()?;
    let store = RecordStore::new(state_dir);
    let mut record_cache = BTreeMap::<String, CanonicalIssueRecord>::new();
    let mut result = MissionExecutionAllowances::default();

    for mission in db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| issue.issue_type == "mission")
    {
        if policy.issue_status_is_terminal(&mission.issue_type, &mission.status)? {
            continue;
        }
        let scope = crate::objective_graph::mission_issue_ids(db, &mission.id)?;
        let applies_to = candidate_ids
            .iter()
            .filter(|id| **id == mission.id || scope.contains(*id))
            .cloned()
            .collect::<Vec<_>>();
        if applies_to.is_empty() {
            continue;
        }

        let mut reviewed = scope;
        reviewed.insert(mission.id.clone());
        let mut required = reviewed.clone();
        for (blocked, blocker) in &dependency_edges {
            if reviewed.contains(blocked) || reviewed.contains(blocker) {
                required.insert(blocked.clone());
                required.insert(blocker.clone());
            }
        }
        for id in &reviewed {
            for relation in db.get_typed_relations(id)? {
                if relation.relation_type == "blocked_by" {
                    required.insert(relation.issue_id_1);
                    required.insert(relation.issue_id_2);
                }
            }
        }
        for id in &required {
            if !record_cache.contains_key(id) {
                record_cache.insert(id.clone(), store.load_issue_by_id(id)?);
            }
        }
        let records = required
            .iter()
            .filter_map(|id| record_cache.get(id).cloned())
            .collect::<Vec<_>>();
        let state = mission_plan_review_state_from_records(state_dir, &mission.id, &records)?;
        let allowed = match state.freshness {
            MissionPlanReviewFreshness::FreshApproval => true,
            MissionPlanReviewFreshness::FreshGrandfather => {
                policy.status_category(&mission.status) == Some("active")
            }
            _ => false,
        };
        for id in applies_to {
            result
                .allowed_by_issue
                .entry(id)
                .and_modify(|current| *current &= allowed)
                .or_insert(allowed);
        }
        result.missions_evaluated += 1;
    }
    result.canonical_records_loaded = record_cache.len();
    Ok(result)
}

impl MissionReadinessDiagnosis {
    pub fn normal_lines(&self) -> [String; 2] {
        [
            format!("Readiness: {}", self.summary),
            format!("Next: {}", self.next_command),
        ]
    }
}

pub fn review_diagnosis(
    state_dir: &Path,
    mission_id: &str,
) -> Result<Option<MissionReadinessDiagnosis>> {
    let state = mission_plan_review_state(state_dir, mission_id)?;
    let revision = &state.current_graph_revision;
    let diagnosis = match state.freshness {
        MissionPlanReviewFreshness::FreshApproval
        | MissionPlanReviewFreshness::FreshGrandfather => None,
        MissionPlanReviewFreshness::ProvenanceIncomplete
            if state.authors.is_empty()
                && state.material_editors.is_empty()
                && state.authorization.is_none() =>
        {
            Some(MissionReadinessDiagnosis {
                kind: MissionReadinessKind::ReviewNotRequested,
                mission_id: mission_id.to_string(),
                summary: format!(
                    "mission {mission_id}: mission-plan review was not requested for graph {revision}"
                ),
                next_command: format!("atelier issue plan-review {mission_id} request"),
            })
        }
        MissionPlanReviewFreshness::ProvenanceIncomplete => Some(MissionReadinessDiagnosis {
            kind: MissionReadinessKind::ReviewNotRequested,
            mission_id: mission_id.to_string(),
            summary: format!(
                "mission {mission_id} review provenance is incomplete for graph {revision}"
            ),
            next_command: format!("atelier issue plan-review {mission_id} request"),
        }),
        MissionPlanReviewFreshness::Unapproved => Some(MissionReadinessDiagnosis {
            kind: MissionReadinessKind::ApprovalMissing,
            mission_id: mission_id.to_string(),
            summary: format!(
                "mission {mission_id} graph {revision} has no independent approval"
            ),
            next_command: format!("atelier issue plan-review {mission_id} approve"),
        }),
        MissionPlanReviewFreshness::Stale => Some(MissionReadinessDiagnosis {
            kind: MissionReadinessKind::StaleApproval,
            mission_id: mission_id.to_string(),
            summary: format!(
                "mission {mission_id}: mission-plan approval is stale for current graph {revision}"
            ),
            next_command: format!("atelier issue plan-review {mission_id} rework"),
        }),
        MissionPlanReviewFreshness::BlockedByReview => {
            if let Some(finding) = state.findings.iter().find(|finding| {
                finding.graph_revision == *revision
                    && finding.severity == MissionPlanFindingSeverity::Blocking
                    && finding.resolution.is_none()
            }) {
                let scope = decision_scope(&finding.dependency_path, &finding.affected_issue_ids);
                Some(MissionReadinessDiagnosis {
                    kind: MissionReadinessKind::BlockingFinding,
                    mission_id: mission_id.to_string(),
                    summary: format!(
                        "mission {mission_id} has unresolved blocking findings: {}{scope}",
                        finding.id
                    ),
                    next_command: format!(
                        "atelier issue plan-review {mission_id} resolve {} --disposition \"<resolution>\"",
                        finding.id
                    ),
                })
            } else if let Some(request) = state
                .change_requests
                .iter()
                .find(|request| request.graph_revision == *revision && request.resolution.is_none())
            {
                let scope = decision_scope(&request.dependency_path, &request.affected_issue_ids);
                Some(MissionReadinessDiagnosis {
                    kind: MissionReadinessKind::ChangeRequest,
                    mission_id: mission_id.to_string(),
                    summary: format!(
                        "mission {mission_id} has unresolved change requests: {}; rework required{scope}",
                        request.id
                    ),
                    next_command: format!(
                        "atelier issue plan-review {mission_id} resolve {} --disposition \"<resolution>\"",
                        request.id
                    ),
                })
            } else {
                None
            }
        }
    };
    Ok(diagnosis)
}

pub fn dependency_diagnosis(
    db: &Database,
    policy: &WorkflowPolicy,
    mission_id: &str,
) -> Result<Option<MissionReadinessDiagnosis>> {
    let closure = dependency_closure(db, policy, mission_id)?;
    let Some(path) = closure
        .open_paths
        .iter()
        .chain(closure.cycle_paths.iter())
        .max_by_key(|path| path.len())
    else {
        return Ok(None);
    };
    let terminal = path
        .last()
        .cloned()
        .unwrap_or_else(|| mission_id.to_string());
    let kind = if path.len() <= 2 {
        MissionReadinessKind::DirectBlocker
    } else {
        MissionReadinessKind::TransitiveBlocker
    };
    let label = if kind == MissionReadinessKind::DirectBlocker {
        "direct blocker"
    } else {
        "transitive blocker"
    };
    Ok(Some(MissionReadinessDiagnosis {
        kind,
        mission_id: mission_id.to_string(),
        summary: format!(
            "mission {mission_id} has {label} path {}",
            path.join(" -> ")
        ),
        next_command: format!("atelier issue show {terminal}"),
    }))
}

pub fn mission_diagnosis(
    db: &Database,
    state_dir: &Path,
    policy: &WorkflowPolicy,
    mission_id: &str,
) -> Result<Option<MissionReadinessDiagnosis>> {
    if let Some(review) = review_diagnosis(state_dir, mission_id)? {
        return Ok(Some(review));
    }
    dependency_diagnosis(db, policy, mission_id)
}

fn decision_scope(path: &[String], affected: &[String]) -> String {
    if !path.is_empty() {
        format!(" on path {}", path.join(" -> "))
    } else if !affected.is_empty() {
        format!(" affecting {}", affected.join(", "))
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow_policy::{
        BranchLifecycleConfig, StatusDefinition, WorkflowDefinition, WorkflowPolicy,
    };
    use atelier_core::{Issue, RelatesRelationship, RelationshipTarget, Relationships};
    use atelier_records::IssueSections;
    use atelier_sqlite::{IssueCacheRow, RecordSourceCacheRow};
    use chrono::Utc;
    use tempfile::tempdir;

    fn policy() -> WorkflowPolicy {
        let statuses = BTreeMap::from([
            (
                "draft".to_string(),
                StatusDefinition {
                    category: "todo".to_string(),
                    role: None,
                },
            ),
            (
                "todo".to_string(),
                StatusDefinition {
                    category: "todo".to_string(),
                    role: None,
                },
            ),
            (
                "archived".to_string(),
                StatusDefinition {
                    category: "done".to_string(),
                    role: None,
                },
            ),
        ]);
        let mission = WorkflowDefinition {
            applies_to: vec!["mission".to_string()],
            initial_status: "draft".to_string(),
            done_statuses: vec!["archived".to_string()],
            transitions: BTreeMap::new(),
        };
        let task = WorkflowDefinition {
            applies_to: vec!["task".to_string()],
            initial_status: "todo".to_string(),
            done_statuses: vec!["archived".to_string()],
            transitions: BTreeMap::new(),
        };
        WorkflowPolicy {
            schema_version: 3,
            branch_policy: BranchLifecycleConfig::default(),
            issue_types: BTreeMap::new(),
            workflow_by_issue_type: BTreeMap::from([
                ("mission".to_string(), "mission".to_string()),
                ("task".to_string(), "task".to_string()),
            ]),
            statuses,
            workflows: BTreeMap::from([
                ("mission".to_string(), mission),
                ("task".to_string(), task),
            ]),
        }
    }

    fn index_issue(
        db: &Database,
        id: &str,
        issue_type: &str,
        status: &str,
        parent_id: Option<&str>,
    ) {
        let now = Utc::now();
        db.index_issue(
            &IssueCacheRow {
                id: id.to_string(),
                title: id.to_string(),
                status: status.to_string(),
                issue_type: issue_type.to_string(),
                priority: "medium".to_string(),
                fields: BTreeMap::new(),
                parent_id: parent_id.map(str::to_string),
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

    fn canonical_issue(
        id: &str,
        issue_type: &str,
        status: &str,
        parent_id: Option<&str>,
        relationships: Relationships,
    ) -> CanonicalIssueRecord {
        let now = Utc::now();
        CanonicalIssueRecord {
            issue: Issue {
                id: id.to_string(),
                title: id.to_string(),
                description: None,
                status: status.to_string(),
                issue_type: issue_type.to_string(),
                priority: "medium".to_string(),
                fields: BTreeMap::new(),
                parent_id: parent_id.map(str::to_string),
                created_at: now,
                updated_at: now,
                closed_at: None,
            },
            labels: Vec::new(),
            sections: IssueSections::unchecked_from_body(Some(
                "## Description\n\nFixture\n\n## Outcome\n\nBounded readiness\n\n## Evidence\n\nTest",
            )),
            relationships,
        }
    }

    #[test]
    fn configured_terminal_mission_with_scope_never_receives_execution_gate() {
        let dir = tempdir().unwrap();
        let db = Database::open(&dir.path().join("state.db")).unwrap();
        index_issue(&db, "atelier-m100", "mission", "archived", None);
        index_issue(&db, "atelier-r100", "task", "todo", None);
        db.add_typed_relation("atelier-m100", "atelier-r100", "advances")
            .unwrap();

        let result = execution_allowances_for_candidates(
            &db,
            &dir.path().join("canonical-does-not-exist"),
            &policy(),
            &BTreeSet::from(["atelier-r100".to_string()]),
        )
        .unwrap();

        assert!(result.allowed_by_issue.is_empty());
        assert_eq!(result.missions_evaluated, 0);
        assert_eq!(result.canonical_records_loaded, 0);
    }

    #[test]
    fn candidate_context_loads_only_relevant_canonical_graph_slice() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join("canonical");
        let db = Database::open(&dir.path().join("state.db")).unwrap();
        index_issue(&db, "atelier-m200", "mission", "draft", None);
        index_issue(&db, "atelier-r200", "task", "todo", None);
        index_issue(&db, "atelier-c200", "task", "todo", Some("atelier-r200"));
        db.add_typed_relation("atelier-m200", "atelier-r200", "advances")
            .unwrap();
        for index in 0..700 {
            index_issue(&db, &format!("atelier-u{index:04}"), "task", "todo", None);
        }

        let store = RecordStore::new(&state_dir);
        store
            .write_issue_atomic(&canonical_issue(
                "atelier-m200",
                "mission",
                "draft",
                None,
                Relationships {
                    relates: vec![RelatesRelationship {
                        kind: "issue".to_string(),
                        id: "atelier-r200".to_string(),
                        relation_type: "advances".to_string(),
                    }],
                    ..Relationships::default()
                },
            ))
            .unwrap();
        store
            .write_issue_atomic(&canonical_issue(
                "atelier-r200",
                "task",
                "todo",
                None,
                Relationships {
                    children: vec![RelationshipTarget {
                        kind: "issue".to_string(),
                        id: "atelier-c200".to_string(),
                    }],
                    ..Relationships::default()
                },
            ))
            .unwrap();
        store
            .write_issue_atomic(&canonical_issue(
                "atelier-c200",
                "task",
                "todo",
                Some("atelier-r200"),
                Relationships::default(),
            ))
            .unwrap();

        let result = execution_allowances_for_candidates(
            &db,
            &state_dir,
            &policy(),
            &BTreeSet::from(["atelier-c200".to_string()]),
        )
        .unwrap();

        assert_eq!(result.allowed_by_issue.get("atelier-c200"), Some(&false));
        assert_eq!(result.missions_evaluated, 1);
        assert_eq!(result.canonical_records_loaded, 3);
    }
}
