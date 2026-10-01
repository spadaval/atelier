use anyhow::Result;
use chrono::{DateTime, Utc};
use std::cmp::Reverse;
use std::collections::BTreeSet;

use crate::record_index::RecordIndex;
use crate::work_classification::{classify_issue, WorkClassification};
use crate::workflow_policy::WorkflowPolicy;
use atelier_core::{Issue, IssuePriority, RecordLink};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectiveBucketTotals {
    pub active: usize,
    pub ready: usize,
    pub blocked: usize,
    pub done: usize,
    pub backlog: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectiveIssueSummary {
    pub id: String,
    pub title: String,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
    pub parent_id: Option<String>,
    pub bucket: WorkClassification,
    pub open_blockers: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectiveScopeSummary {
    pub issue_ids: BTreeSet<String>,
    pub totals: ObjectiveBucketTotals,
    pub active: Vec<ObjectiveIssueSummary>,
    pub backlog: Vec<ObjectiveIssueSummary>,
    pub ready: Vec<ObjectiveIssueSummary>,
    pub blocked: Vec<ObjectiveIssueSummary>,
    pub done: Vec<ObjectiveIssueSummary>,
}

impl ObjectiveScopeSummary {
    pub fn total(&self) -> usize {
        self.issue_ids.len()
    }

    pub fn health(&self, open_objective_blockers: usize) -> &'static str {
        if open_objective_blockers > 0 || self.totals.blocked > 0 {
            "blocked"
        } else if self.totals.active > 0 {
            "active"
        } else if self.totals.ready > 0 {
            "ready"
        } else if self.totals.done > 0 {
            "terminal"
        } else {
            "steady"
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectiveRelationshipSummary {
    pub advances_roots: Vec<String>,
    pub open_blockers: Vec<String>,
    pub direct_blockers: Vec<String>,
    pub validating_evidence: usize,
    pub other_links: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectiveEvidenceGateSummary {
    pub linked_validating_evidence: usize,
    pub scoped_issues_without_evidence: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectiveRecentActivityFacts {
    pub recently_updated: Vec<ObjectiveIssueSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectiveReadSummary {
    pub objective_id: String,
    pub objective_type: String,
    pub scope: ObjectiveScopeSummary,
    pub relationships: ObjectiveRelationshipSummary,
    pub evidence: ObjectiveEvidenceGateSummary,
    pub recent_activity: ObjectiveRecentActivityFacts,
}

pub fn objective_read_summary(
    index: &RecordIndex,
    objective_id: &str,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<Option<ObjectiveReadSummary>> {
    let Some(objective) = index.get_issue(objective_id) else {
        return Ok(None);
    };
    let is_objective = matches!(objective.issue_type.as_str(), "epic" | "mission");
    let scope_ids = crate::objective_graph::issue_descendant_ids(index, &objective.id)?;
    if !is_objective && scope_ids.is_empty() {
        return Ok(None);
    }

    let relationships = relationship_summary(index, &objective, &scope_ids, workflow_policy)?;
    let scope = scope_summary(index, &scope_ids, workflow_policy)?;
    let evidence = evidence_gate_summary(index, objective_id, &scope_ids)?;
    let recent_activity = recent_activity_facts(&scope);

    Ok(Some(ObjectiveReadSummary {
        objective_id: objective.id,
        objective_type: objective.issue_type,
        scope,
        relationships,
        evidence,
        recent_activity,
    }))
}

fn scope_summary(
    index: &RecordIndex,
    scope_ids: &BTreeSet<String>,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<ObjectiveScopeSummary> {
    let mut summary = ObjectiveScopeSummary {
        issue_ids: scope_ids.clone(),
        ..ObjectiveScopeSummary::default()
    };
    let mut done = Vec::new();

    for issue_id in scope_ids {
        let Some(issue) = index.get_issue(issue_id) else {
            continue;
        };
        let classified = classify_issue(index, workflow_policy, &issue)?;
        let bucket = classified.classification;
        let issue_summary = ObjectiveIssueSummary {
            id: issue.id,
            title: issue.title,
            status: issue.status,
            issue_type: issue.issue_type,
            priority: issue.priority,
            parent_id: issue.parent_id,
            bucket,
            open_blockers: classified.open_blockers,
            updated_at: issue.updated_at,
        };
        match bucket {
            WorkClassification::Active => {
                summary.totals.active += 1;
                summary.active.push(issue_summary);
            }
            WorkClassification::Ready => {
                summary.totals.ready += 1;
                summary.ready.push(issue_summary);
            }
            WorkClassification::Blocked => {
                summary.totals.blocked += 1;
                summary.blocked.push(issue_summary);
            }
            WorkClassification::Done => {
                summary.totals.done += 1;
                done.push(issue_summary);
            }
            WorkClassification::Backlog => {
                summary.totals.backlog += 1;
                summary.backlog.push(issue_summary);
            }
        }
    }

    summary.active = order_issue_summaries(summary.active);
    summary.backlog = order_issue_summaries(summary.backlog);
    summary.ready = order_issue_summaries(summary.ready);
    summary.blocked = order_issue_summaries(summary.blocked);
    summary.done = order_issue_summaries(done);
    Ok(summary)
}

fn relationship_summary(
    index: &RecordIndex,
    objective: &Issue,
    scope_ids: &BTreeSet<String>,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<ObjectiveRelationshipSummary> {
    let mut summary = ObjectiveRelationshipSummary::default();
    let mut direct_blockers = BTreeSet::new();

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
        match link.relation_type.as_str() {
            "blocked_by" => {
                if let Some(issue_id) = linked_issue_side(&link, &objective.id) {
                    direct_blockers.insert(issue_id.to_string());
                }
            }
            "validates" if link_touches_evidence(&link) => summary.validating_evidence += 1,
            _ => summary.other_links += 1,
        }
    }

    let mut blocker_ids = direct_blockers.clone();
    for issue_id in scope_ids {
        for blocker_id in index.get_blockers(issue_id) {
            blocker_ids.insert(blocker_id);
        }
    }

    summary.direct_blockers = direct_blockers.into_iter().collect();
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

fn recent_activity_facts(scope: &ObjectiveScopeSummary) -> ObjectiveRecentActivityFacts {
    let mut recently_updated = scope
        .active
        .iter()
        .chain(scope.backlog.iter())
        .chain(scope.ready.iter())
        .chain(scope.blocked.iter())
        .chain(scope.done.iter())
        .cloned()
        .collect::<Vec<_>>();
    recently_updated.sort_by_key(|issue| (Reverse(issue.updated_at), issue.id.clone()));
    ObjectiveRecentActivityFacts { recently_updated }
}

fn order_issue_summaries(mut issues: Vec<ObjectiveIssueSummary>) -> Vec<ObjectiveIssueSummary> {
    issues.sort_by_key(|issue| {
        (
            bucket_rank(issue.bucket),
            priority_rank(&issue.priority),
            Reverse(issue.updated_at),
            issue.id.clone(),
        )
    });
    issues
}

fn bucket_rank(bucket: WorkClassification) -> u8 {
    match bucket {
        WorkClassification::Ready => 0,
        WorkClassification::Active => 1,
        WorkClassification::Blocked => 2,
        WorkClassification::Done => 3,
        WorkClassification::Backlog => 4,
    }
}

fn priority_rank(priority: &str) -> u8 {
    IssuePriority::from_label(priority)
        .map(|priority| priority.sort_rank())
        .unwrap_or(4)
}

fn issue_status_category(policy: Option<&WorkflowPolicy>, status: &str) -> Option<String> {
    policy
        .and_then(|policy| policy.status_category(status))
        .map(str::to_string)
}

fn issue_is_done(policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    issue_status_category(policy, &issue.status).as_deref() == Some("done")
}

fn issue_blocks_work(policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    !issue_is_done(policy, issue)
}

fn linked_issue_side<'a>(link: &'a RecordLink, objective_id: &str) -> Option<&'a str> {
    if link.source_kind == "issue" && link.source_id == objective_id && link.target_kind == "issue"
    {
        Some(&link.target_id)
    } else if link.target_kind == "issue"
        && link.target_id == objective_id
        && link.source_kind == "issue"
    {
        Some(&link.source_id)
    } else {
        None
    }
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
        let workflow_by_issue_type = ["mission", "epic", "task"]
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
        records.issue("atelier-obj", "epic", "todo");
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
        assert_eq!(summary.scope.totals.ready, 2);
        assert_eq!(summary.scope.totals.blocked, 1);
        assert_eq!(summary.scope.totals.done, 1);
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
        records.issue("atelier-root", "epic", "todo");
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
        assert_eq!(summary.scope.totals.ready, 2);
    }

    #[test]
    fn evidence_gate_counts_validating_evidence_without_renderer() {
        let mut records = Records::default();
        let policy = test_policy();
        records.issue("atelier-obj", "epic", "todo");
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
        records.issue("atelier-obj", "epic", "todo");
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

        assert_eq!(summary.scope.totals.ready, 7);
        assert_eq!(summary.scope.ready.len(), 7);
        assert_eq!(summary.scope.totals.active, 1);
        assert_eq!(
            summary.recent_activity.recently_updated[0].id,
            "atelier-active"
        );
        assert_eq!(summary.recent_activity.recently_updated.len(), 8);
    }
}
