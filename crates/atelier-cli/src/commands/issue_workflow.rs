use anyhow::Result;

use crate::utils::format_issue_id;
use atelier_app::workflow_policy::WorkflowPolicy;
use atelier_core::Issue;
use atelier_sqlite::Database;

pub(crate) fn load_issue_workflow_policy() -> Result<Option<WorkflowPolicy>> {
    let repo_root = atelier_app::storage_layout::find_repo_root()?;
    let policy_path = repo_root.join(atelier_app::workflow_policy::WORKFLOW_POLICY_PATH);
    if !policy_path.exists() {
        return Ok(None);
    }
    atelier_app::workflow_policy::load(&repo_root).map(Some)
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
