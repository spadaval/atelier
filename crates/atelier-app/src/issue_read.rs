use anyhow::Result;
use std::cmp::Reverse;
use std::collections::BTreeSet;

use crate::objective_graph::issue_blocks_work;
use crate::record_index::RecordIndex;
use crate::work_classification::{work_buckets_for, WorkBuckets, WorkRow};
use crate::workflow_policy::WorkflowPolicy;
use atelier_core::{Issue, IssuePriority, RecordLink};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectiveBucketTotals {
    pub active: usize,
    pub ready: usize,
    pub human: usize,
    pub context: usize,
    pub blocked: usize,
    pub done: usize,
    pub backlog: usize,
}

impl From<&WorkBuckets> for ObjectiveBucketTotals {
    fn from(work: &WorkBuckets) -> Self {
        Self {
            active: work.active.len(),
            ready: work.ready.len(),
            human: work.human.len(),
            context: work.context.len(),
            blocked: work.blocked.len(),
            done: work.done.len(),
            backlog: work.backlog.len(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectiveScopeSummary {
    pub issue_ids: BTreeSet<String>,
    pub work: WorkBuckets,
}

impl ObjectiveScopeSummary {
    pub fn total(&self) -> usize {
        self.issue_ids.len()
    }

    pub fn health(&self, open_objective_blockers: usize) -> &'static str {
        if open_objective_blockers > 0 || !self.work.blocked.is_empty() {
            "blocked"
        } else if !self.work.active.is_empty() {
            "active"
        } else if !self.work.ready.is_empty() {
            "ready"
        } else if !self.work.human.is_empty() {
            "waiting on the human"
        } else if !self.work.done.is_empty() {
            "done"
        } else {
            "steady"
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectiveRelationshipSummary {
    pub advances_roots: Vec<String>,
    pub open_blockers: Vec<String>,
    pub other_links: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectiveEvidenceGateSummary {
    pub linked_validating_evidence: usize,
    pub scoped_issues_without_evidence: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectiveReadSummary {
    pub scope: ObjectiveScopeSummary,
    pub relationships: ObjectiveRelationshipSummary,
    pub evidence: ObjectiveEvidenceGateSummary,
    pub recently_updated: Vec<WorkRow>,
}

pub fn objective_read_summary(
    index: &RecordIndex,
    objective_id: &str,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<Option<ObjectiveReadSummary>> {
    let Some(objective) = index.get_issue(objective_id) else {
        return Ok(None);
    };
    let is_objective = objective.issue_type == "mission";
    let scope_ids = crate::objective_graph::issue_descendant_ids(index, &objective.id)?;
    if !is_objective && scope_ids.is_empty() {
        return Ok(None);
    }

    let relationships = relationship_summary(index, &objective, &scope_ids, workflow_policy)?;
    let scope = scope_summary(index, &scope_ids, workflow_policy)?;
    let evidence = evidence_gate_summary(index, objective_id, &scope_ids)?;
    let mut recently_updated = scope
        .work
        .active
        .iter()
        .chain(&scope.work.backlog)
        .chain(&scope.work.ready)
        .chain(&scope.work.human)
        .chain(&scope.work.context)
        .chain(&scope.work.blocked)
        .chain(&scope.work.done)
        .cloned()
        .collect::<Vec<_>>();
    recently_updated.sort_by_key(|row| {
        (
            Reverse(
                index
                    .issue_record(&row.id)
                    .expect("indexed work row")
                    .issue
                    .updated_at,
            ),
            row.id.clone(),
        )
    });

    Ok(Some(ObjectiveReadSummary {
        scope,
        relationships,
        evidence,
        recently_updated,
    }))
}

fn scope_summary(
    index: &RecordIndex,
    scope_ids: &BTreeSet<String>,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<ObjectiveScopeSummary> {
    let mut work = work_buckets_for(
        index,
        workflow_policy,
        index.issues().filter(|issue| scope_ids.contains(&issue.id)),
    )?;
    for rows in [
        &mut work.active,
        &mut work.ready,
        &mut work.human,
        &mut work.context,
        &mut work.blocked,
        &mut work.backlog,
        &mut work.done,
    ] {
        rows.sort_by_key(|row| {
            (
                IssuePriority::from_label(&row.priority)
                    .map(|priority| priority.sort_rank())
                    .unwrap_or(4),
                Reverse(
                    index
                        .issue_record(&row.id)
                        .expect("indexed work row")
                        .issue
                        .updated_at,
                ),
                row.id.clone(),
            )
        });
    }
    Ok(ObjectiveScopeSummary {
        issue_ids: scope_ids.clone(),
        work,
    })
}

fn relationship_summary(
    index: &RecordIndex,
    objective: &Issue,
    scope_ids: &BTreeSet<String>,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<ObjectiveRelationshipSummary> {
    let mut summary = ObjectiveRelationshipSummary::default();
    let mut blocker_ids = BTreeSet::new();

    for relation in index.get_typed_relations(&objective.id) {
        let linked_id = if relation.issue_id_1 == objective.id {
            relation.issue_id_2
        } else {
            relation.issue_id_1
        };
        if relation.relation_type == "advances" {
            summary.advances_roots.push(linked_id);
        } else {
            summary.other_links += 1;
        }
    }

    for link in index.list_record_links("issue", &objective.id)? {
        if !(link.relation_type == "validates" && link_touches_evidence(&link)) {
            summary.other_links += 1;
        }
    }

    for issue_id in scope_ids.iter().chain([&objective.id]) {
        for blocker_id in index.get_blockers(issue_id) {
            blocker_ids.insert(blocker_id);
        }
    }

    summary.open_blockers = blocker_ids
        .into_iter()
        .filter_map(|id| index.get_issue(&id))
        .filter(|issue| issue_blocks_work(workflow_policy, issue))
        .map(|issue| issue.id)
        .collect();
    summary.advances_roots.sort();
    summary.advances_roots.dedup();
    summary.open_blockers.sort();
    Ok(summary)
}

fn evidence_gate_summary(
    index: &RecordIndex,
    objective_id: &str,
    scope_ids: &BTreeSet<String>,
) -> Result<ObjectiveEvidenceGateSummary> {
    let mut linked_validating_evidence = 0usize;
    let mut scoped_issues_without_evidence = 0usize;
    let mut issue_ids = scope_ids.iter().map(String::as_str).collect::<Vec<_>>();
    issue_ids.push(objective_id);

    for issue_id in issue_ids {
        let count = validating_evidence_count(index, issue_id)?;
        linked_validating_evidence += count;
        if issue_id != objective_id && count == 0 {
            scoped_issues_without_evidence += 1;
        }
    }

    Ok(ObjectiveEvidenceGateSummary {
        linked_validating_evidence,
        scoped_issues_without_evidence,
    })
}

fn validating_evidence_count(index: &RecordIndex, issue_id: &str) -> Result<usize> {
    Ok(index
        .list_record_links("issue", issue_id)?
        .into_iter()
        .filter(|link| link.relation_type == "validates" && link_touches_evidence(link))
        .count())
}

fn link_touches_evidence(link: &RecordLink) -> bool {
    link.source_kind == "evidence" || link.target_kind == "evidence"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_records::{evidence_record, Records};
    use crate::workflow_policy::{
        BranchLifecycleConfig, StatusDefinition, TransitionDefinition, WorkflowDefinition,
        WorkflowPolicy,
    };
    use chrono::Utc;
    use std::collections::BTreeMap;

    fn test_policy() -> WorkflowPolicy {
        let mut statuses = BTreeMap::new();
        statuses.insert(
            "todo".to_string(),
            StatusDefinition {
                category: "todo".to_string(),
            },
        );
        statuses.insert(
            "in_progress".to_string(),
            StatusDefinition {
                category: "active".to_string(),
            },
        );
        statuses.insert(
            "blocked".to_string(),
            StatusDefinition {
                category: "blocked".to_string(),
            },
        );
        statuses.insert(
            "done".to_string(),
            StatusDefinition {
                category: "done".to_string(),
            },
        );
        let workflow_by_issue_type = ["mission", "feature", "task"]
            .into_iter()
            .map(|issue_type| (issue_type.to_string(), "work".to_string()))
            .collect();
        let workflows = BTreeMap::from([(
            "work".to_string(),
            WorkflowDefinition {
                applies_to: vec![
                    "mission".to_string(),
                    "epic".to_string(),
                    "task".to_string(),
                ],
                initial_status: "todo".to_string(),
                done_statuses: vec!["done".to_string()],
                transitions: BTreeMap::from([(
                    "start".to_string(),
                    TransitionDefinition {
                        from: vec!["todo".to_string()],
                        to: "in_progress".to_string(),
                        required_fields: Vec::new(),
                        description: None,
                        validators: Vec::new(),
                    },
                )]),
            },
        )]);
        WorkflowPolicy {
            merge_checks: Vec::new(),
            schema_version: 3,
            branch_policy: BranchLifecycleConfig::default(),
            issue_types: BTreeMap::new(),
            workflow_by_issue_type,
            statuses,
            workflows,
        }
    }

    #[test]
    fn objective_scope_buckets_ready_blocked_and_done_children() {
        let mut records = Records::default();
        let policy = test_policy();
        records.issue("atelier-obj", "feature", "todo");
        for (id, status) in [
            ("atelier-ready", "todo"),
            ("atelier-blocked", "todo"),
            ("atelier-blocker", "todo"),
            ("atelier-done", "done"),
        ] {
            records.issue(id, "task", status);
            records.parent(id, "atelier-obj");
        }
        records.block("atelier-blocked", "atelier-blocker");

        let summary = objective_read_summary(&records.index(), "atelier-obj", Some(&policy))
            .unwrap()
            .unwrap();

        assert_eq!(summary.scope.total(), 4);
        assert_eq!(summary.scope.work.ready.len(), 2);
        assert_eq!(summary.scope.work.blocked.len(), 1);
        assert_eq!(summary.scope.work.done.len(), 1);
        assert_eq!(summary.relationships.open_blockers, vec!["atelier-blocker"]);
        assert_eq!(
            summary
                .scope
                .health(summary.relationships.open_blockers.len()),
            "blocked"
        );
    }

    #[test]
    fn mission_scope_follows_advances_linked_roots_and_descendants() {
        let mut records = Records::default();
        let policy = test_policy();
        records.issue("atelier-mission", "mission", "todo");
        records.issue("atelier-root", "feature", "todo");
        records.issue("atelier-child", "task", "todo");
        records.parent("atelier-child", "atelier-root");
        records.relate("atelier-mission", "atelier-root", "advances");

        let summary = objective_read_summary(&records.index(), "atelier-mission", Some(&policy))
            .unwrap()
            .unwrap();

        assert_eq!(
            summary.scope.issue_ids,
            BTreeSet::from(["atelier-child".to_string(), "atelier-root".to_string()])
        );
        assert_eq!(summary.relationships.advances_roots, vec!["atelier-root"]);
        // The parent with an open child is context; its child is the ready work.
        assert_eq!(summary.scope.work.ready.len(), 1);
        assert_eq!(summary.scope.work.context.len(), 1);
    }

    #[test]
    fn evidence_gate_counts_validating_evidence_without_renderer() {
        let mut records = Records::default();
        let policy = test_policy();
        records.issue("atelier-obj", "feature", "todo");
        records.issue("atelier-ready", "task", "todo");
        records.parent("atelier-ready", "atelier-obj");
        records.evidence(evidence_record(
            "atelier-proof",
            "atelier-ready",
            "validates",
        ));

        let summary = objective_read_summary(&records.index(), "atelier-obj", Some(&policy))
            .unwrap()
            .unwrap();

        assert_eq!(summary.evidence.linked_validating_evidence, 1);
        assert_eq!(summary.evidence.scoped_issues_without_evidence, 0);
    }

    #[test]
    fn objective_scope_keeps_all_rows_before_rendering_and_recent_activity_includes_active_work() {
        let mut records = Records::default();
        records.issue("atelier-obj", "feature", "todo");
        for number in 0..7 {
            let id = format!("atelier-ready{number}");
            records.issue(&id, "task", "todo");
            records.parent(&id, "atelier-obj");
        }
        records.issue("atelier-active", "task", "in_progress");
        records.parent("atelier-active", "atelier-obj");
        records.get("atelier-active").issue.updated_at = Utc::now() + chrono::Duration::days(1);

        let summary = objective_read_summary(&records.index(), "atelier-obj", Some(&test_policy()))
            .unwrap()
            .unwrap();

        assert_eq!(summary.scope.work.ready.len(), 7);
        assert_eq!(summary.scope.work.ready.len(), 7);
        assert_eq!(summary.scope.work.active.len(), 1);
        assert_eq!(summary.recently_updated[0].id, "atelier-active");
        assert_eq!(summary.recently_updated.len(), 8);
    }
    #[test]
    fn objective_rows_keep_priority_update_and_identity_order() {
        let mut records = Records::default();
        records.issue("atelier-obj", "feature", "todo");
        let now = Utc::now();
        for (id, priority, age) in [
            ("atelier-low", "low", 0),
            ("atelier-old", "high", 10),
            ("atelier-newb", "high", 0),
            ("atelier-newa", "high", 0),
        ] {
            let record = records.issue(id, "task", "todo");
            record.issue.priority = priority.into();
            record.issue.updated_at = now - chrono::Duration::days(age);
            records.parent(id, "atelier-obj");
        }
        let summary = objective_read_summary(&records.index(), "atelier-obj", Some(&test_policy()))
            .unwrap()
            .unwrap();
        assert_eq!(
            summary
                .scope
                .work
                .ready
                .iter()
                .map(|row| row.id.as_str())
                .collect::<Vec<_>>(),
            ["atelier-newa", "atelier-newb", "atelier-old", "atelier-low"]
        );
        assert_eq!(
            summary
                .recently_updated
                .iter()
                .map(|row| row.id.as_str())
                .collect::<Vec<_>>(),
            ["atelier-low", "atelier-newa", "atelier-newb", "atelier-old"]
        );
    }
}
