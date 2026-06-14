use anyhow::Result;

use crate::db::Database;
use crate::models::Issue;
use crate::utils::format_issue_id;
use crate::workflow_policy::WorkflowPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IssueStartReadiness {
    Ready,
    Blocked,
    NotReady,
}

pub(crate) fn load_issue_workflow_policy() -> Result<Option<WorkflowPolicy>> {
    let repo_root = crate::storage_layout::find_repo_root()?;
    let policy_path = repo_root.join(crate::workflow_policy::WORKFLOW_POLICY_PATH);
    if !policy_path.exists() {
        return Ok(None);
    }
    crate::workflow_policy::load(&repo_root).map(Some)
}

pub(crate) fn issue_status_category(
    policy: Option<&WorkflowPolicy>,
    status: &str,
) -> Option<String> {
    policy
        .and_then(|policy| policy.status_category(status))
        .map(str::to_string)
}

pub(crate) fn issue_status_label(policy: Option<&WorkflowPolicy>, status: &str) -> String {
    format_status_with_category(issue_status_category(policy, status).as_deref(), status)
}

pub(crate) fn format_status_with_category(category: Option<&str>, status: &str) -> String {
    match category {
        Some(category) => format!("{category}/{status}"),
        None => format!("unknown/{status}"),
    }
}

pub(crate) fn issue_is_done(policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    issue_status_category(policy, &issue.status).as_deref() == Some("done")
}

pub(crate) fn issue_blocks_work(policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    !issue_is_done(policy, issue)
}

pub(crate) fn issue_start_readiness(
    db: &Database,
    policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> Result<IssueStartReadiness> {
    if policy.is_none() {
        return if issue_status_category(None, &issue.status).as_deref() == Some("todo") {
            if open_blocker_ids_with_policy(db, None, &issue.id)?.is_empty() {
                Ok(IssueStartReadiness::Ready)
            } else {
                Ok(IssueStartReadiness::Blocked)
            }
        } else {
            Ok(IssueStartReadiness::NotReady)
        };
    }
    let Some(policy) = policy else {
        unreachable!("handled missing policy above")
    };
    let options = match crate::commands::workflow::issue_transition_options(db, &issue.id) {
        Ok(options) => options,
        Err(_) => return Ok(IssueStartReadiness::NotReady),
    };
    let mut has_start_target = false;
    let mut blocked = false;
    for option in options {
        if issue_status_category(Some(policy), &option.to).as_deref() != Some("active") {
            continue;
        }
        has_start_target = true;
        if option.allowed {
            return Ok(IssueStartReadiness::Ready);
        }
        blocked = true;
    }
    if blocked {
        Ok(IssueStartReadiness::Blocked)
    } else if has_start_target {
        Ok(IssueStartReadiness::NotReady)
    } else {
        Ok(IssueStartReadiness::NotReady)
    }
}

pub(crate) fn open_blocker_ids_with_policy(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issue_id: &str,
) -> Result<Vec<String>> {
    let mut blockers = db
        .get_blockers(issue_id)?
        .into_iter()
        .filter_map(|id| db.require_issue(&id).ok())
        .filter(|issue| issue_blocks_work(workflow_policy, issue))
        .map(|issue| format_issue_id(&issue.id))
        .collect::<Vec<_>>();
    blockers.sort();
    Ok(blockers)
}
