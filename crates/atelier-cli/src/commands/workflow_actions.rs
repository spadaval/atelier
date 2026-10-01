use std::path::Path;

use crate::commands::workflow::PlannedAction;

pub(crate) fn action_preflight_blockers(
    repo_root: &Path,
    planned_actions: &[PlannedAction],
) -> Vec<String> {
    planned_actions
        .iter()
        .filter_map(|action| match action.name.as_str() {
            "git.prepare_branch" => branch_prepare_preflight(repo_root, action),
            "tracker.commit" | "git.push" | "git.sync" | "git.integrate" => {
                branch_post_action_preflight(repo_root, action)
            }
            other => Some(format!(
                "action {other} failed preflight: action execution is not implemented yet; retry after the owning action issue lands"
            )),
        })
        .collect()
}

fn branch_prepare_preflight(repo_root: &Path, action: &PlannedAction) -> Option<String> {
    if let Err(error) = ensure_git_action_repo(repo_root, action) {
        return Some(error);
    }
    match crate::commands::workflow::non_tracker_dirty_entries(repo_root) {
        Ok(dirty) if !dirty.is_empty() => {
            return Some(format!(
                "action {} failed preflight: checkout has uncommitted non-tracker changes:\n{}",
                action.name,
                dirty.join("\n")
            ));
        }
        Ok(_) => {}
        Err(error) => {
            return Some(format!(
                "action {} failed preflight: {error:#}",
                action.name
            ))
        }
    }
    if crate::commands::workflow::git_current_branch(repo_root)
        .ok()
        .as_deref()
        != Some(action.expected_branch.as_str())
        && !crate::commands::workflow::branch_exists_at(repo_root, &action.expected_branch)
            .unwrap_or(false)
        && !crate::commands::workflow::branch_exists_at(repo_root, &action.base_branch)
            .unwrap_or(false)
    {
        return Some(format!(
            "action {} failed preflight: configured base branch '{}' is missing; create or fetch it, then retry `atelier issue transition {} start`",
            action.name, action.base_branch, action.target_issue_id
        ));
    }
    None
}

fn branch_post_action_preflight(repo_root: &Path, action: &PlannedAction) -> Option<String> {
    if let Err(error) = ensure_git_action_repo(repo_root, action) {
        return Some(error);
    }
    match crate::commands::workflow::non_tracker_dirty_entries(repo_root) {
        Ok(dirty) if !dirty.is_empty() => Some(format!(
            "action {} failed preflight: checkout has uncommitted non-tracker changes:\n{}",
            action.name,
            dirty.join("\n")
        )),
        Err(error) => Some(format!("action {} failed preflight: {error:#}", action.name)),
        _ if action.name == "git.integrate"
            && action.merge_owned
            && !crate::commands::workflow::branch_exists_at(repo_root, &action.base_branch)
                .unwrap_or(false) =>
        {
            Some(format!(
                "action {} failed preflight: configured base branch '{}' is missing; create or fetch it, then retry the transition",
                action.name, action.base_branch
            ))
        }
        _ => None,
    }
}

fn ensure_git_action_repo(repo_root: &Path, action: &PlannedAction) -> Result<(), String> {
    match crate::commands::workflow::is_git_repo(repo_root) {
        Ok(true) => Ok(()),
        Ok(false) => Err(format!(
            "action {} failed preflight: git repository is required",
            action.name
        )),
        Err(error) => Err(format!(
            "action {} failed preflight: {error:#}",
            action.name
        )),
    }
}
