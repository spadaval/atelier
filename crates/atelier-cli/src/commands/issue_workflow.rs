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

pub(crate) fn format_status_with_category(category: Option<&str>, status: &str) -> String {
    match category {
        Some(_) => status.to_string(),
        None => format!("{status} (unknown category)"),
    }
}

pub(crate) fn issue_is_done(policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    issue_status_category(policy, &issue.status).as_deref() == Some("done")
}

/// Whether the working policy configures any transition out of the issue's
/// status; most done statuses have none.
pub(crate) fn issue_has_transitions(policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    policy
        .and_then(|policy| {
            policy
                .transitions_from_status(&issue.issue_type, &issue.status)
                .ok()
        })
        .is_some_and(|transitions| !transitions.is_empty())
}
