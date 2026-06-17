use anyhow::{anyhow, Result};
use std::collections::BTreeSet;
use std::path::Path;

use atelier_core::Issue;
use atelier_sqlite::Database;

pub use atelier_workflow::{
    configured_initial_status, load, validate_issue_against_policy, BranchLifecycleConfig,
    BranchLifecycleResolution, BranchOwnerKind, BranchTemplates, GuidanceTemplate, MergeStrategy,
    StatusDefinition, TransitionDefinition, ValidatorDefinition, ValidatorParams,
    WorkflowDefinition, WorkflowPolicy, WORKFLOW_POLICY_PATH,
};

pub use atelier_workflow::STARTER_POLICY_YAML;

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
