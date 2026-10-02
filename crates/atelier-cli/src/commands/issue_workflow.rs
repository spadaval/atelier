use anyhow::Result;

use atelier_app::workflow_policy::WorkflowPolicy;
use atelier_core::Issue;

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
        Some(_) => status.to_string(),
        None => format!("{status} (unknown category)"),
    }
}

pub(crate) fn issue_is_done(policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    issue_status_category(policy, &issue.status).as_deref() == Some("done")
}
