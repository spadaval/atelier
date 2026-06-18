use anyhow::{anyhow, Result};
use std::collections::BTreeSet;
use std::path::Path;

use atelier_core::Issue;
use atelier_sqlite::Database;
use serde_json::Value;

pub use atelier_workflow::{
    configured_initial_status, load, validate_issue_against_policy, BranchLifecycleConfig,
    BranchLifecycleResolution, BranchOwnerKind, BranchTemplates, GuidanceTemplate, MergeStrategy,
    StatusDefinition, TransitionDefinition, ValidatorDefinition, ValidatorParams,
    WorkflowDefinition, WorkflowPolicy, WORKFLOW_POLICY_PATH,
};

pub use atelier_workflow::STARTER_POLICY_YAML;

pub const FORGE_PR_FIELD: &str = "forge_pr";

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
        base_branch: policy.branch_lifecycle.base_branch.clone(),
        merge_strategy: policy.branch_lifecycle.merge_strategy,
        merge_owned,
        nested_under_epic,
    })
}

pub fn effective_forge_pr_field(db: &Database, issue_id: &str) -> Result<Option<Value>> {
    let issue = db.require_issue(issue_id)?;
    if issue.parent_id.is_some() && issue.fields.contains_key(FORGE_PR_FIELD) {
        return Err(anyhow!(
            "workflow_issue_field_invalid: issue {} defines forge_pr directly, but child issues inherit forge_pr from the nearest parent epic; move the field to the owning epic or remove it from the child",
            issue.id
        ));
    }
    if let Some(value) = issue.fields.get(FORGE_PR_FIELD) {
        return Ok(Some(value.clone()));
    }

    let mut parent_id = issue.parent_id.clone();
    let mut seen = BTreeSet::new();
    while let Some(current_id) = parent_id {
        if !seen.insert(current_id.clone()) {
            return Err(anyhow!(
                "workflow_forge_pr_invalid_graph: issue {} has a cyclic parent graph while resolving forge_pr",
                issue.id
            ));
        }
        let parent = db.get_issue(&current_id)?.ok_or_else(|| {
            anyhow!(
                "workflow_forge_pr_invalid_graph: issue {} references missing parent issue {}",
                issue.id,
                current_id
            )
        })?;
        if parent.issue_type == "epic" {
            return Ok(parent.fields.get(FORGE_PR_FIELD).cloned());
        }
        parent_id = parent.parent_id;
    }
    Ok(None)
}

fn nearest_parent_epic(db: &Database, issue: &Issue) -> Result<Issue> {
    let mut parent_id = issue.parent_id.clone();
    let mut seen = BTreeSet::new();
    while let Some(current_id) = parent_id {
        if !seen.insert(current_id.clone()) {
            return Err(anyhow!(
                "workflow_branch_lifecycle_invalid_graph: issue {} has a cyclic parent graph while resolving branch owner",
                issue.id
            ));
        }
        let parent = db.get_issue(&current_id)?.ok_or_else(|| {
            anyhow!(
                "workflow_branch_lifecycle_invalid_graph: issue {} references missing parent issue {}",
                issue.id,
                current_id
            )
        })?;
        if parent.issue_type == "epic" {
            return Ok(parent);
        }
        parent_id = parent.parent_id;
    }
    Err(anyhow!(
        "workflow_branch_lifecycle_invalid_graph: issue {} is nested but has no parent epic branch owner",
        issue.id
    ))
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

    fn forge_pr_field() -> BTreeMap<String, Value> {
        let mut fields = BTreeMap::new();
        fields.insert(
            FORGE_PR_FIELD.to_string(),
            serde_json::json!({
                "provider": "forgejo",
                "host": "forge.example.test",
                "owner": "tools",
                "repo": "atelier",
                "number": 42,
                "url": "https://forge.example.test/tools/atelier/pulls/42",
                "source_branch": "codex/atelier-fpr1",
                "target_branch": "master"
            }),
        );
        fields
    }

    #[test]
    fn effective_forge_pr_field_inherits_from_nearest_parent_epic() {
        let (db, _dir) = setup_test_db();
        let parent_fields = forge_pr_field();
        let expected = parent_fields.get(FORGE_PR_FIELD).unwrap().clone();
        insert_issue(&db, "atelier-epic", "epic", None, parent_fields);
        insert_issue(
            &db,
            "atelier-child",
            "task",
            Some("atelier-epic"),
            BTreeMap::new(),
        );

        let inherited = effective_forge_pr_field(&db, "atelier-child").unwrap();

        assert_eq!(inherited, Some(expected));
    }

    #[test]
    fn effective_forge_pr_field_rejects_child_duplicate() {
        let (db, _dir) = setup_test_db();
        insert_issue(&db, "atelier-epic", "epic", None, forge_pr_field());
        insert_issue(
            &db,
            "atelier-child",
            "task",
            Some("atelier-epic"),
            forge_pr_field(),
        );

        let error = effective_forge_pr_field(&db, "atelier-child")
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_issue_field_invalid"));
        assert!(error.contains("child issues inherit forge_pr"));
    }
}
