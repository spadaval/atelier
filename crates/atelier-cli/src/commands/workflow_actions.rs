use std::path::Path;

use atelier_app::project_config::{ProjectConfig, ReviewConfig, ReviewProviderKind};
use atelier_app::user_config::forgejo_admin_token;

use crate::commands::workflow::PlannedAction;

pub(crate) fn action_preflight_blockers(
    repo_root: &Path,
    planned_actions: &[PlannedAction],
) -> Vec<String> {
    planned_actions
        .iter()
        .filter_map(|action| match action.name.as_str() {
            "branch.prepare" => branch_prepare_preflight(repo_root, action),
            "tracker.commit" | "branch.push" | "review.merge" | "base.sync"
            | "branch_integrate" => branch_post_action_preflight(repo_root, action),
            "review.open" => review_open_preflight(repo_root, action),
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
        _ if provider_action_names(action.name.as_str()) && !review_config_is_provider(repo_root) => {
            Some(format!(
                "action {} failed preflight: provider terminal action requires review.mode = \"provider\"",
                action.name
            ))
        }
        _ if action.name == "branch_integrate" && review_config_is_provider(repo_root) => {
            Some(
                "action branch_integrate failed preflight: local branch integration is only valid for room review workflows"
                    .to_string(),
            )
        }
        _ if action.name == "branch_integrate"
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

fn provider_action_names(name: &str) -> bool {
    matches!(name, "branch.push" | "review.merge" | "base.sync")
}

fn review_config_is_provider(repo_root: &Path) -> bool {
    matches!(
        ProjectConfig::load(repo_root).map(|config| config.review),
        Ok(ReviewConfig::Provider(_))
    )
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

fn review_open_preflight(repo_root: &Path, action: &PlannedAction) -> Option<String> {
    review_open_preflight_with_token_check(repo_root, action, || forgejo_admin_token().is_ok())
}

fn review_open_preflight_with_token_check<TokenAvailable>(
    repo_root: &Path,
    action: &PlannedAction,
    token_available: TokenAvailable,
) -> Option<String>
where
    TokenAvailable: FnOnce() -> bool,
{
    if action.review_artifact_target.is_none() {
        return Some(format!(
            "action {} failed preflight: missing review artifact target",
            action.name
        ));
    }
    let Some(role) = &action.review_artifact_role else {
        return Some(format!(
            "action {} failed preflight: missing review artifact role",
            action.name
        ));
    };
    match ProjectConfig::load(repo_root).map(|config| config.review) {
        Ok(ReviewConfig::Room) => {
            if action.review_artifact_provider.is_some() {
                return Some(format!(
                    "action {} failed preflight: provider action config is only valid when review.mode = \"provider\"",
                    action.name
                ));
            }
            let _ = role;
            None
        }
        Ok(ReviewConfig::Provider(provider)) => match provider.provider {
            ReviewProviderKind::Forgejo(_) => {
                if action.review_artifact_provider.as_deref() != Some("forgejo") {
                    return Some(format!(
                        "action {} failed preflight: provider review open requires workflow action provider: forgejo",
                        action.name
                    ));
                }
                if action.forgejo_role_authors.is_none() {
                    return Some(format!(
                        "action {} failed preflight: provider review open requires workflow action role_authors",
                        action.name
                    ));
                }
                (!token_available()).then(|| {
                    format!(
                        "action {} failed preflight: Forgejo admin token is required for provider review open; set forgejo.admin_token in ~/.config/atelier.toml",
                        action.name
                    )
                })
            }
        },
        Err(error) => Some(format!(
            "action {} failed preflight: {}",
            action.name, error
        )),
    }
}

#[cfg(test)]
pub(crate) fn review_open_preflight_for_test(
    repo_root: &Path,
    action: &PlannedAction,
    token_available: bool,
) -> Option<String> {
    review_open_preflight_with_token_check(repo_root, action, || token_available)
}
