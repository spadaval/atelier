use anyhow::Result;
use chrono::{DateTime, Utc};
use std::cmp::Reverse;
use std::collections::BTreeSet;

use crate::workflow_policy::WorkflowPolicy;
use atelier_core::{Issue, IssuePriority, RecordLink};
use atelier_sqlite::Database;

const OBJECTIVE_ROLLUP_LIMIT: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveIssueBucket {
    Active,
    Ready,
    Blocked,
    Done,
    Backlog,
}

impl ObjectiveIssueBucket {
    pub fn label(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Ready => "ready",
            Self::Blocked => "blocked",
            Self::Done => "done",
            Self::Backlog => "backlog",
        }
    }
}

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
    pub bucket: ObjectiveIssueBucket,
    pub open_blockers: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectiveScopeSummary {
    pub issue_ids: BTreeSet<String>,
    pub totals: ObjectiveBucketTotals,
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
    db: &Database,
    objective_id: &str,
    workflow_policy: Option<&WorkflowPolicy>,
    active_issue_ids: &BTreeSet<&str>,
) -> Result<Option<ObjectiveReadSummary>> {
    let Some(objective) = db.get_issue(objective_id)? else {
        return Ok(None);
    };
    let is_objective = matches!(objective.issue_type.as_str(), "epic" | "mission");
    let scope_ids = issue_descendant_ids(db, &objective)?;
    if !is_objective && scope_ids.is_empty() {
        return Ok(None);
    }

    let relationships = relationship_summary(db, &objective, &scope_ids, workflow_policy)?;
    let scope = scope_summary(db, &scope_ids, workflow_policy, active_issue_ids)?;
    let evidence = evidence_gate_summary(db, objective_id, &scope_ids)?;
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
    db: &Database,
    scope_ids: &BTreeSet<String>,
    workflow_policy: Option<&WorkflowPolicy>,
    active_issue_ids: &BTreeSet<&str>,
) -> Result<ObjectiveScopeSummary> {
    let mut summary = ObjectiveScopeSummary {
        issue_ids: scope_ids.clone(),
        ..ObjectiveScopeSummary::default()
    };
    let mut done = Vec::new();

    for issue_id in scope_ids {
        let Some(issue) = db.get_issue(issue_id)? else {
            continue;
        };
        let open_blockers = open_issue_blockers(db, &issue.id, workflow_policy)?;
        let bucket = issue_bucket(&issue, &open_blockers, workflow_policy, active_issue_ids);
        let issue_summary = ObjectiveIssueSummary {
            id: issue.id,
            title: issue.title,
            status: issue.status,
            issue_type: issue.issue_type,
            priority: issue.priority,
            parent_id: issue.parent_id,
            bucket,
            open_blockers,
            updated_at: issue.updated_at,
        };
        match bucket {
            ObjectiveIssueBucket::Active => summary.totals.active += 1,
            ObjectiveIssueBucket::Ready => {
                summary.totals.ready += 1;
                summary.ready.push(issue_summary);
            }
            ObjectiveIssueBucket::Blocked => {
                summary.totals.blocked += 1;
                summary.blocked.push(issue_summary);
            }
            ObjectiveIssueBucket::Done => {
                summary.totals.done += 1;
                done.push(issue_summary);
            }
            ObjectiveIssueBucket::Backlog => summary.totals.backlog += 1,
        }
    }

    summary.ready = order_issue_summaries(summary.ready);
    summary.blocked = order_issue_summaries(summary.blocked);
    summary.done = order_issue_summaries(done);
    summary.ready.truncate(OBJECTIVE_ROLLUP_LIMIT);
    summary.blocked.truncate(OBJECTIVE_ROLLUP_LIMIT);
    summary.done.truncate(OBJECTIVE_ROLLUP_LIMIT);
    Ok(summary)
}

fn issue_bucket(
    issue: &Issue,
    open_blockers: &[String],
    workflow_policy: Option<&WorkflowPolicy>,
    active_issue_ids: &BTreeSet<&str>,
) -> ObjectiveIssueBucket {
    if active_issue_ids.contains(issue.id.as_str()) {
        return ObjectiveIssueBucket::Active;
    }
    if issue_is_done(workflow_policy, issue) {
        return ObjectiveIssueBucket::Done;
    }
    if !open_blockers.is_empty() {
        return ObjectiveIssueBucket::Blocked;
    }
    if issue_status_category(workflow_policy, &issue.status).as_deref() == Some("todo") {
        ObjectiveIssueBucket::Ready
    } else {
        ObjectiveIssueBucket::Backlog
    }
}

fn relationship_summary(
    db: &Database,
    objective: &Issue,
    scope_ids: &BTreeSet<String>,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<ObjectiveRelationshipSummary> {
    let mut summary = ObjectiveRelationshipSummary::default();
    let mut direct_blockers = BTreeSet::new();

    for relation in db.get_typed_relations(&objective.id)? {
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

    for link in db.list_record_links("issue", &objective.id)? {
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
        for blocker_id in db.get_blockers(issue_id)? {
            blocker_ids.insert(blocker_id);
        }
    }

    summary.direct_blockers = direct_blockers.into_iter().collect();
    summary.open_blockers = blocker_ids
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter(|issue| issue_blocks_work(workflow_policy, issue))
        .map(|issue| issue.id)
        .collect();
    summary.advances_roots.sort();
    summary.advances_roots.dedup();
    summary.open_blockers.sort();
    Ok(summary)
}

fn evidence_gate_summary(
    db: &Database,
    objective_id: &str,
    scope_ids: &BTreeSet<String>,
) -> Result<ObjectiveEvidenceGateSummary> {
    let mut linked_validating_evidence = 0usize;
    let mut scoped_issues_without_evidence = 0usize;
    let mut issue_ids = scope_ids.iter().map(String::as_str).collect::<Vec<_>>();
    issue_ids.push(objective_id);

    for issue_id in issue_ids {
        let count = validating_evidence_count(db, issue_id)?;
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

fn validating_evidence_count(db: &Database, issue_id: &str) -> Result<usize> {
    Ok(db
        .list_record_links("issue", issue_id)?
        .into_iter()
        .filter(|link| link.relation_type == "validates" && link_touches_evidence(link))
        .count())
}

fn recent_activity_facts(scope: &ObjectiveScopeSummary) -> ObjectiveRecentActivityFacts {
    let mut recently_updated = scope
        .ready
        .iter()
        .chain(scope.blocked.iter())
        .chain(scope.done.iter())
        .cloned()
        .collect::<Vec<_>>();
    recently_updated.sort_by_key(|issue| (Reverse(issue.updated_at), issue.id.clone()));
    recently_updated.truncate(OBJECTIVE_ROLLUP_LIMIT);
    ObjectiveRecentActivityFacts { recently_updated }
}

pub fn issue_descendant_ids(db: &Database, objective: &Issue) -> Result<BTreeSet<String>> {
    let mut issue_ids = BTreeSet::new();
    for child in db.get_subissues(&objective.id)? {
        collect_issue_and_descendants(db, &child.id, &mut issue_ids)?;
    }
    if objective.issue_type != "mission" {
        return Ok(issue_ids);
    }
    for relation in db.get_typed_relations(&objective.id)? {
        if relation.relation_type != "advances" {
            continue;
        }
        let linked_id = if relation.issue_id_1 == objective.id {
            relation.issue_id_2
        } else {
            relation.issue_id_1
        };
        collect_issue_and_descendants(db, &linked_id, &mut issue_ids)?;
    }
    Ok(issue_ids)
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

fn open_issue_blockers(
    db: &Database,
    issue_id: &str,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<Vec<String>> {
    let mut blockers = db
        .get_blockers(issue_id)?
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter(|issue| issue_blocks_work(workflow_policy, issue))
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    blockers.sort();
    Ok(blockers)
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

fn bucket_rank(bucket: ObjectiveIssueBucket) -> u8 {
    match bucket {
        ObjectiveIssueBucket::Ready => 0,
        ObjectiveIssueBucket::Active => 1,
        ObjectiveIssueBucket::Blocked => 2,
        ObjectiveIssueBucket::Done => 3,
        ObjectiveIssueBucket::Backlog => 4,
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
    use crate::workflow_policy::{BranchLifecycleConfig, StatusDefinition, WorkflowPolicy};
    use atelier_core::Issue;
    use chrono::{TimeZone, Utc};
    use serde_json::Value;
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    fn test_policy() -> WorkflowPolicy {
        let mut statuses = BTreeMap::new();
        statuses.insert(
            "todo".to_string(),
            StatusDefinition {
                category: "todo".to_string(),
                role: None,
            },
        );
        statuses.insert(
            "in_progress".to_string(),
            StatusDefinition {
                category: "active".to_string(),
                role: None,
            },
        );
        statuses.insert(
            "blocked".to_string(),
            StatusDefinition {
                category: "blocked".to_string(),
                role: None,
            },
        );
        statuses.insert(
            "done".to_string(),
            StatusDefinition {
                category: "done".to_string(),
                role: None,
            },
        );
        WorkflowPolicy {
            schema_version: 3,
            branch_policy: BranchLifecycleConfig::default(),
            issue_types: BTreeMap::new(),
            workflow_by_issue_type: BTreeMap::new(),
            statuses,
            workflows: BTreeMap::new(),
        }
    }

    fn setup_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db = Database::open(&dir.path().join("state.db")).unwrap();
        (db, dir)
    }

    fn insert_issue(
        db: &Database,
        id: &str,
        title: &str,
        issue_type: &str,
        status: &str,
        parent_id: Option<&str>,
    ) {
        db.insert_issue_rebuild(&Issue {
            id: id.to_string(),
            title: title.to_string(),
            description: None,
            status: status.to_string(),
            issue_type: issue_type.to_string(),
            priority: "medium".to_string(),
            fields: BTreeMap::<String, Value>::new(),
            parent_id: parent_id.map(str::to_string),
            created_at: Utc.with_ymd_and_hms(2026, 6, 24, 12, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 6, 24, 12, 0, 0).unwrap(),
            closed_at: None,
        })
        .unwrap();
    }

    #[test]
    fn objective_scope_buckets_ready_blocked_and_done_children() {
        let (db, _dir) = setup_db();
        let policy = test_policy();
        insert_issue(&db, "atelier-obj", "Objective", "epic", "todo", None);
        insert_issue(
            &db,
            "atelier-ready",
            "Ready",
            "task",
            "todo",
            Some("atelier-obj"),
        );
        insert_issue(
            &db,
            "atelier-blocked",
            "Blocked",
            "task",
            "todo",
            Some("atelier-obj"),
        );
        insert_issue(
            &db,
            "atelier-blocker",
            "Blocker",
            "task",
            "todo",
            Some("atelier-obj"),
        );
        insert_issue(
            &db,
            "atelier-done",
            "Done",
            "task",
            "done",
            Some("atelier-obj"),
        );
        db.add_dependency("atelier-blocked", "atelier-blocker")
            .unwrap();

        let summary = objective_read_summary(&db, "atelier-obj", Some(&policy), &BTreeSet::new())
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
        let (db, _dir) = setup_db();
        let policy = test_policy();
        insert_issue(&db, "atelier-mission", "Mission", "mission", "todo", None);
        insert_issue(&db, "atelier-root", "Linked root", "epic", "todo", None);
        insert_issue(
            &db,
            "atelier-child",
            "Linked child",
            "task",
            "todo",
            Some("atelier-root"),
        );
        db.add_typed_relation("atelier-mission", "atelier-root", "advances")
            .unwrap();

        let summary =
            objective_read_summary(&db, "atelier-mission", Some(&policy), &BTreeSet::new())
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
        let (db, _dir) = setup_db();
        let policy = test_policy();
        insert_issue(&db, "atelier-obj", "Objective", "epic", "todo", None);
        insert_issue(
            &db,
            "atelier-ready",
            "Ready",
            "task",
            "todo",
            Some("atelier-obj"),
        );
        let evidence_id = db
            .create_record("evidence", "Validation proof", "recorded")
            .unwrap();
        db.add_record_link(
            "evidence",
            &evidence_id,
            "issue",
            "atelier-ready",
            "validates",
        )
        .unwrap();

        let summary = objective_read_summary(&db, "atelier-obj", Some(&policy), &BTreeSet::new())
            .unwrap()
            .unwrap();

        assert_eq!(summary.evidence.linked_validating_evidence, 1);
        assert_eq!(summary.evidence.scoped_issues_without_evidence, 0);
    }
}
