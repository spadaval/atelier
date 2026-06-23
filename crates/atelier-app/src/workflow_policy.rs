use anyhow::{anyhow, Result};
use std::collections::BTreeSet;
use std::path::Path;

use atelier_core::Issue;
use atelier_sqlite::Database;
use serde_json::Value;

pub use atelier_workflow::{
    configured_initial_status, load, validate_issue_against_policy, ActionDefinition, ActionParams,
    BranchLifecycleConfig, BranchLifecycleResolution, BranchOwnerKind, BranchTemplates,
    GuidanceTemplate, MergeStrategy, ReviewArtifactActionParams, StatusDefinition,
    TransitionDefinition, ValidatorDefinition, ValidatorParams, WorkflowDefinition,
    WorkflowForgejoRoleAuthors, WorkflowPolicy, WORKFLOW_POLICY_PATH,
};

pub use atelier_workflow::STARTER_POLICY_YAML;

pub const REVIEW_FIELD: &str = "review";

#[derive(Debug, Clone)]
pub struct WorkflowCheckReport {
    pub issue_count: usize,
    pub policy: WorkflowPolicy,
}

pub fn check(db: &Database, repo_root: &Path) -> Result<WorkflowCheckReport> {
    let policy_path = repo_root.join(WORKFLOW_POLICY_PATH);
    let policy = load(repo_root)?;
    let issues = db.list_issues(Some("all"), None, None)?;
    for issue in &issues {
        validate_issue_against_policy(&policy, issue, &policy_path)?;
    }
    Ok(WorkflowCheckReport {
        issue_count: issues.len(),
        policy,
    })
}

pub fn resolve_branch_lifecycle(
    policy: &WorkflowPolicy,
    db: &Database,
    issue_id: &str,
) -> Result<BranchLifecycleResolution> {
    let issue = db.require_issue(issue_id)?;
    let (owner, owner_kind, nested_under_epic) = if issue.issue_type == "epic" {
        (issue.clone(), BranchOwnerKind::Epic, false)
    } else if issue.parent_id.is_none() {
        (issue.clone(), BranchOwnerKind::StandaloneIssue, false)
    } else {
        let owner = nearest_parent_epic(db, &issue)?;
        (owner, BranchOwnerKind::Epic, true)
    };
    let expected_branch = policy.branch_name_for_owner(&owner, &owner_kind)?;
    let merge_owned = !nested_under_epic;
    Ok(BranchLifecycleResolution {
        issue_id: issue.id,
        owner_id: owner.id,
        owner_issue_type: owner.issue_type,
        owner_kind,
        expected_branch,
        base_branch: policy.branch_policy.base_branch.clone(),
        merge_strategy: policy.branch_policy.merge_strategy,
        merge_owned,
        nested_under_epic,
    })
}

pub fn effective_pull_request_field(db: &Database, issue_id: &str) -> Result<Option<Value>> {
    effective_review_field(db, issue_id)
}

pub fn effective_review_field(db: &Database, issue_id: &str) -> Result<Option<Value>> {
    let issue = db.require_issue(issue_id)?;
    if issue.parent_id.is_some() && issue.fields.contains_key(REVIEW_FIELD) {
        return Err(anyhow!(
            "workflow_issue_field_invalid: issue {} defines review directly, but child issues inherit review from the nearest parent epic; move the field to the owning epic or remove it from the child",
            issue.id
        ));
    }
    if let Some(value) = issue.fields.get(REVIEW_FIELD) {
        return Ok(Some(value.clone()));
    }

    let mut parent_id = issue.parent_id.clone();
    let mut seen = BTreeSet::new();
    while let Some(current_id) = parent_id {
        if !seen.insert(current_id.clone()) {
            return Err(anyhow!(
                "workflow_review_invalid_graph: issue {} has a cyclic parent graph while resolving review",
                issue.id
            ));
        }
        let parent = db.get_issue(&current_id)?.ok_or_else(|| {
            anyhow!(
                "workflow_review_invalid_graph: issue {} references missing parent issue {}",
                issue.id,
                current_id
            )
        })?;
        if parent.issue_type == "epic" {
            return Ok(parent.fields.get(REVIEW_FIELD).cloned());
        }
        parent_id = parent.parent_id;
    }
    Ok(None)
}

fn nearest_parent_epic(db: &Database, issue: &Issue) -> Result<Issue> {
    let mut parent_id = issue.parent_id.clone();
    let mut seen = BTreeSet::new();
    let mut parent_chain = Vec::new();
    while let Some(current_id) = parent_id {
        if !seen.insert(current_id.clone()) {
            return Err(anyhow!(
                "workflow_branch_policy_invalid_graph: issue {} has a cyclic parent graph while resolving branch owner",
                issue.id
            ));
        }
        let parent = db.get_issue(&current_id)?.ok_or_else(|| {
            anyhow!(
                "workflow_branch_policy_invalid_graph: issue {} references missing parent issue {}",
                issue.id,
                current_id
            )
        })?;
        parent_chain.push((parent.id.clone(), parent.issue_type.clone()));
        if parent.issue_type == "epic" {
            return Ok(parent);
        }
        parent_id = parent.parent_id;
    }
    Err(missing_parent_epic_branch_owner_error(issue, &parent_chain))
}

fn missing_parent_epic_branch_owner_error(
    issue: &Issue,
    parent_chain: &[(String, String)],
) -> anyhow::Error {
    let parent_chain = if parent_chain.is_empty() {
        "(none)".to_string()
    } else {
        parent_chain
            .iter()
            .map(|(id, issue_type)| format!("{id} ({issue_type})"))
            .collect::<Vec<_>>()
            .join(" -> ")
    };
    let mission_link_guidance = parent_chain
        .split(" -> ")
        .find(|entry| entry.ends_with(" (mission)"))
        .map(|entry| {
            let mission_id = entry.split_once(' ').map(|(id, _)| id).unwrap_or(entry);
            format!(
                "\nIf this work should stay directly under the mission, make it standalone and link it to the mission instead:\n  atelier issue update {} --no-parent\n  atelier issue link {} {}",
                issue.id, mission_id, issue.id
            )
        })
        .unwrap_or_default();
    anyhow!(
        "workflow_branch_policy_invalid_graph: issue {} is nested but has no parent epic branch owner\nParent chain: {}\nBranch policy requires nested issues to inherit branch ownership from a parent epic. Standalone issues own their own issue branch.\nFix: move {} under an epic with:\n  atelier issue update {} --parent <epic-id>{}",
        issue.id,
        parent_chain,
        issue.id,
        issue.id,
        mission_link_guidance
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    fn setup_test_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::open(&db_path).unwrap();
        (db, dir)
    }

    fn insert_issue(
        db: &Database,
        id: &str,
        issue_type: &str,
        parent_id: Option<&str>,
        fields: BTreeMap<String, Value>,
    ) {
        let now = Utc::now();
        db.insert_issue_rebuild(&Issue {
            id: id.to_string(),
            title: id.to_string(),
            description: None,
            status: "todo".to_string(),
            issue_type: issue_type.to_string(),
            priority: "medium".to_string(),
            fields,
            parent_id: parent_id.map(str::to_string),
            created_at: now,
            updated_at: now,
            closed_at: None,
        })
        .unwrap();
    }

    fn review_field() -> BTreeMap<String, Value> {
        let mut fields = BTreeMap::new();
        fields.insert(
            REVIEW_FIELD.to_string(),
            serde_json::json!({"kind": "room", "id": "atelier-rvw1"}),
        );
        fields
    }

    #[test]
    fn effective_review_field_inherits_from_nearest_parent_epic() {
        let (db, _dir) = setup_test_db();
        let parent_fields = review_field();
        let expected = parent_fields.get(REVIEW_FIELD).unwrap().clone();
        insert_issue(&db, "atelier-epic", "epic", None, parent_fields);
        insert_issue(
            &db,
            "atelier-child",
            "task",
            Some("atelier-epic"),
            BTreeMap::new(),
        );

        let inherited = effective_review_field(&db, "atelier-child").unwrap();

        assert_eq!(inherited, Some(expected));
    }

    #[test]
    fn effective_review_field_rejects_child_duplicate() {
        let (db, _dir) = setup_test_db();
        insert_issue(&db, "atelier-epic", "epic", None, review_field());
        insert_issue(
            &db,
            "atelier-child",
            "task",
            Some("atelier-epic"),
            review_field(),
        );

        let error = effective_review_field(&db, "atelier-child")
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_issue_field_invalid"));
        assert!(error.contains("child issues inherit review"));
    }

    #[test]
    fn branch_lifecycle_error_explains_nested_issue_without_parent_epic_fix() {
        let (db, _dir) = setup_test_db();
        insert_issue(&db, "atelier-mission", "mission", None, BTreeMap::new());
        insert_issue(
            &db,
            "atelier-child",
            "task",
            Some("atelier-mission"),
            BTreeMap::new(),
        );

        let policy = WorkflowPolicy {
            schema_version: 3,
            branch_policy: BranchLifecycleConfig::default(),
            issue_types: BTreeMap::new(),
            workflow_by_issue_type: BTreeMap::new(),
            statuses: BTreeMap::new(),
            workflows: BTreeMap::new(),
        };
        let error = resolve_branch_lifecycle(&policy, &db, "atelier-child")
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_branch_policy_invalid_graph"));
        assert!(error.contains("Parent chain: atelier-mission (mission)"));
        assert!(error.contains("nested issues to inherit branch ownership from a parent epic"));
        assert!(error.contains("Standalone issues own their own issue branch"));
        assert!(error.contains("atelier issue update atelier-child --parent <epic-id>"));
        assert!(error.contains("atelier issue update atelier-child --no-parent"));
        assert!(error.contains("atelier issue link atelier-mission atelier-child"));
    }
}
