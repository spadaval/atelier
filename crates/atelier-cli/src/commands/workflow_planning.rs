use anyhow::{bail, Result};
use atelier_app::project_config;
use atelier_app::workflow_policy::{
    BranchLifecycleResolution, MergeStrategy, WorkflowForgejoRoleAuthors,
};
use atelier_core::Issue;
use atelier_sqlite::Database;

use crate::commands::workflow::ValidatorResult;
use crate::human_output;

#[derive(Debug, Clone)]
pub struct IssueTransitionOption {
    pub name: String,
    pub from: Vec<String>,
    pub to: String,
    pub allowed: bool,
    pub blockers: Vec<String>,
    pub validator_results: Vec<ValidatorResult>,
    pub planned_actions: Vec<PlannedAction>,
    pub descriptions: Vec<String>,
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct PlannedAction {
    pub order: usize,
    pub name: String,
    pub target_issue_id: String,
    pub branch_owner_id: String,
    pub expected_branch: String,
    pub base_branch: String,
    pub merge_strategy: MergeStrategy,
    pub merge_owned: bool,
    pub review_artifact_target: Option<String>,
    pub review_artifact_provider: Option<String>,
    pub review_artifact_role: Option<String>,
    pub forgejo_role_authors: Option<project_config::ForgejoRoleAuthors>,
    pub confirmation_required: bool,
    pub skip_reason: Option<String>,
    pub block_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct BranchLifecycleContext {
    pub resolution: BranchLifecycleResolution,
    pub current_branch: Option<String>,
    pub expected_branch_exists: bool,
    pub base_branch_exists: bool,
    pub dirty_entries: Vec<String>,
}

pub fn issue_transition_options(
    db: &Database,
    issue_ref: &str,
) -> Result<Vec<IssueTransitionOption>> {
    let issue_id = crate::commands::issue::resolve_id(db, issue_ref)?;
    let repo_root = crate::commands::workflow::repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let issue = db.require_issue(&issue_id)?;
    crate::commands::workflow::ensure_transitionable_status(&policy, &issue)?;
    let state_dir = atelier_app::storage_layout::StorageLayout::new(&repo_root).canonical_dir();
    let record = atelier_app::use_cases::load_canonical_issue(&state_dir, &issue.id)?;
    let mut options = Vec::new();

    for (name, transition) in policy.transitions_from_status(&issue.issue_type, &issue.status)? {
        let mut blockers =
            crate::commands::workflow::required_field_failures(&record, transition, None)?;
        let planned_actions = plan_transition_actions(db, &issue, name, transition)?;
        blockers.extend(branch_context_blockers(
            db,
            &issue,
            name,
            transition,
            &planned_actions,
        )?);
        let validator_results = crate::commands::workflow::evaluate_policy_transition(
            db,
            &policy,
            "issue",
            &issue.id,
            name,
            &transition.validators,
        )?;
        blockers.extend(
            validator_results
                .iter()
                .filter(|result| !result.passed)
                .map(|result| format!("validator {} failed: {}", result.validator, result.reason)),
        );
        let mut descriptions = transition_descriptions(transition);
        descriptions.extend(branch_context_guidance(db, &issue, name, &planned_actions)?);
        blockers.extend(crate::commands::workflow::action_preflight_blockers(
            &repo_root,
            &planned_actions,
        ));
        options.push(IssueTransitionOption {
            name: name.to_string(),
            from: transition.from.clone(),
            to: transition.to.clone(),
            allowed: blockers.is_empty(),
            blockers,
            validator_results,
            planned_actions,
            descriptions,
            command: transition_command(&issue.id, name, transition),
        });
    }

    if options.is_empty() {
        bail!(
            "Issue {} has no configured transitions from status '{}'",
            issue.id,
            issue.status
        );
    }

    Ok(options)
}

pub(crate) fn plan_transition_actions(
    db: &Database,
    issue: &Issue,
    transition_name: &str,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
) -> Result<Vec<PlannedAction>> {
    if transition.actions.is_empty() {
        return Ok(Vec::new());
    }
    let repo_root = crate::commands::workflow::repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let resolution =
        atelier_app::workflow_policy::resolve_branch_lifecycle(&policy, db, &issue.id)?;
    let _ = transition_name;
    Ok(plan_actions_for_resolution(
        issue,
        &resolution,
        &transition.actions,
        1,
    ))
}

pub(crate) fn plan_actions_for_resolution(
    issue: &Issue,
    resolution: &BranchLifecycleResolution,
    actions: &[atelier_app::workflow_policy::ActionDefinition],
    start_order: usize,
) -> Vec<PlannedAction> {
    actions
        .iter()
        .enumerate()
        .map(|(index, action)| {
            let review_artifact = review_artifact_action_plan(action, resolution);
            PlannedAction {
                order: start_order + index,
                name: action.builtin.clone(),
                target_issue_id: issue.id.clone(),
                branch_owner_id: resolution.owner_id.clone(),
                expected_branch: resolution.expected_branch.clone(),
                base_branch: resolution.base_branch.clone(),
                merge_strategy: resolution.merge_strategy,
                merge_owned: resolution.merge_owned,
                review_artifact_target: review_artifact
                    .as_ref()
                    .map(|review| review.target_issue_id.clone()),
                review_artifact_provider: review_artifact
                    .as_ref()
                    .and_then(|review| review.provider.clone()),
                review_artifact_role: review_artifact.as_ref().map(|review| review.role.clone()),
                forgejo_role_authors: review_artifact.and_then(|review| review.role_authors),
                confirmation_required: action.builtin == "branch_integrate",
                skip_reason: None,
                block_reason: None,
            }
        })
        .collect()
}

struct ReviewArtifactActionPlan {
    target_issue_id: String,
    provider: Option<String>,
    role: String,
    role_authors: Option<project_config::ForgejoRoleAuthors>,
}

fn review_artifact_action_plan(
    action: &atelier_app::workflow_policy::ActionDefinition,
    resolution: &BranchLifecycleResolution,
) -> Option<ReviewArtifactActionPlan> {
    if action.builtin != "review.open" {
        return None;
    }
    let Some(atelier_app::workflow_policy::ActionParams::ReviewArtifact(params)) =
        action.params.as_ref()
    else {
        return None;
    };
    Some(ReviewArtifactActionPlan {
        target_issue_id: resolution.owner_id.clone(),
        provider: params.provider.clone(),
        role: params.role.clone(),
        role_authors: params
            .role_authors
            .as_ref()
            .map(workflow_role_authors_to_project),
    })
}

fn workflow_role_authors_to_project(
    role_authors: &WorkflowForgejoRoleAuthors,
) -> project_config::ForgejoRoleAuthors {
    project_config::ForgejoRoleAuthors {
        worker: role_authors.worker.clone(),
        reviewer: role_authors.reviewer.clone(),
        validator: role_authors.validator.clone(),
        manager: role_authors.manager.clone(),
    }
}

pub(crate) fn branch_lifecycle_context(
    db: &Database,
    issue_id: &str,
) -> Result<BranchLifecycleContext> {
    let repo_root = crate::commands::workflow::repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let resolution = atelier_app::workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?;
    Ok(BranchLifecycleContext {
        current_branch: current_git_branch()?,
        expected_branch_exists: crate::commands::workflow::branch_exists_at(
            &repo_root,
            &resolution.expected_branch,
        )?,
        base_branch_exists: crate::commands::workflow::branch_exists_at(
            &repo_root,
            &resolution.base_branch,
        )?,
        dirty_entries: crate::commands::workflow::git_dirty_entries(&repo_root)?,
        resolution,
    })
}

pub(crate) fn current_git_branch() -> Result<Option<String>> {
    let repo_root = crate::commands::workflow::repo_root()?;
    Ok(crate::commands::workflow::git_stdout(
        &repo_root,
        &["branch", "--show-current"],
        "read current branch",
    )
    .ok()
    .map(|value| value.trim().to_string())
    .filter(|value| !value.is_empty()))
}

pub(crate) fn branch_owner_label(
    owner_kind: &atelier_app::workflow_policy::BranchOwnerKind,
) -> &'static str {
    match owner_kind {
        atelier_app::workflow_policy::BranchOwnerKind::Epic => "epic",
        atelier_app::workflow_policy::BranchOwnerKind::StandaloneIssue => "issue",
    }
}

pub(crate) fn branch_lifecycle_state_line(context: &BranchLifecycleContext) -> String {
    if !context.dirty_entries.is_empty() {
        return format!(
            "dirty checkout: {}",
            human_output::path_summary(&context.dirty_entries, 3)
        );
    }
    match context.current_branch.as_deref() {
        Some(current) if current == context.resolution.expected_branch => {
            "current branch matches expected branch".to_string()
        }
        Some(current) if context.expected_branch_exists => {
            format!(
                "current branch {current}; expected branch {} exists",
                context.resolution.expected_branch
            )
        }
        Some(current) if context.base_branch_exists => format!(
            "current branch {current}; expected branch {} can be created from {}",
            context.resolution.expected_branch, context.resolution.base_branch
        ),
        Some(current) => format!(
            "current branch {current}; expected branch {} and base {} are missing",
            context.resolution.expected_branch, context.resolution.base_branch
        ),
        None => "detached HEAD; inspect git status".to_string(),
    }
}

fn branch_context_blockers(
    db: &Database,
    issue: &Issue,
    transition_name: &str,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
    planned_actions: &[PlannedAction],
) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    if !planned_actions_need_branch_context(planned_actions) {
        return Ok(blockers);
    }
    let context = branch_lifecycle_context(db, &issue.id)?;
    if !context.dirty_entries.is_empty() {
        blockers.push(format!(
            "branch context: checkout has uncommitted changes; inspect `git status --short --branch`, then rerun `{}`",
            transition_command(&issue.id, transition_name, transition)
        ));
        return Ok(blockers);
    }
    if transition_name == "start"
        && context.current_branch.as_deref() != Some(context.resolution.expected_branch.as_str())
        && !context.expected_branch_exists
        && !context.base_branch_exists
    {
        blockers.push(format!(
            "branch context: expected branch '{}' cannot be created because base branch '{}' is missing",
            context.resolution.expected_branch, context.resolution.base_branch
        ));
    }
    if transition_name == "close" && context.resolution.merge_owned && !context.base_branch_exists {
        blockers.push(format!(
            "branch context: base branch '{}' is missing for close integration",
            context.resolution.base_branch
        ));
    }
    Ok(blockers)
}

fn branch_context_guidance(
    db: &Database,
    issue: &Issue,
    transition_name: &str,
    planned_actions: &[PlannedAction],
) -> Result<Vec<String>> {
    if !planned_actions_need_branch_context(planned_actions) {
        return Ok(Vec::new());
    }
    let context = branch_lifecycle_context(db, &issue.id)?;
    let mut guidance = Vec::new();
    let is_start = transition_name == "start";
    let is_close = transition_name == "close";
    guidance.push(format!(
        "Branch owner: {} {} ({})",
        branch_owner_label(&context.resolution.owner_kind),
        context.resolution.owner_id,
        context.resolution.owner_issue_type
    ));
    guidance.push(format!(
        "Expected branch: {}",
        context.resolution.expected_branch
    ));
    guidance.push(format!("Base branch: {}", context.resolution.base_branch));
    guidance.push(branch_lifecycle_state_line(&context));
    if is_start {
        guidance.push(format!(
            "Corrective lifecycle command: atelier issue transition {} start",
            issue.id
        ));
    }
    if is_close {
        guidance.push(format!(
            "Close lifecycle command: atelier issue transition {} close --reason \"...\"",
            issue.id
        ));
    }
    Ok(guidance)
}

pub(crate) fn planned_actions_need_branch_context(planned_actions: &[PlannedAction]) -> bool {
    planned_actions.iter().any(|action| {
        matches!(
            action.name.as_str(),
            "branch.prepare"
                | "tracker.commit"
                | "branch.push"
                | "review.merge"
                | "base.sync"
                | "branch_integrate"
                | "review.open"
        )
    })
}

pub(crate) fn transition_descriptions(
    transition: &atelier_app::workflow_policy::TransitionDefinition,
) -> Vec<String> {
    transition
        .description
        .iter()
        .map(|description| description.trim().to_string())
        .filter(|description| !description.is_empty())
        .collect()
}

pub(crate) fn transition_command(
    issue_id: &str,
    transition_name: &str,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
) -> String {
    let mut command = format!("atelier issue transition {issue_id} {transition_name}");
    if transition
        .required_fields
        .iter()
        .any(|field| field == "close_reason")
    {
        command.push_str(" --reason \"...\"");
    }
    command
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_app::workflow_policy::{ActionDefinition, BranchOwnerKind};
    use std::collections::BTreeMap;

    fn test_issue(id: &str) -> Issue {
        Issue {
            id: id.to_string(),
            title: "Issue".to_string(),
            description: None,
            status: "in_progress".to_string(),
            priority: "medium".to_string(),
            issue_type: "epic".to_string(),
            fields: BTreeMap::new(),
            parent_id: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            closed_at: None,
        }
    }

    fn action(name: &str) -> ActionDefinition {
        ActionDefinition {
            builtin: name.to_string(),
            params: None,
        }
    }

    fn review_action() -> ActionDefinition {
        ActionDefinition {
            builtin: "review.open".to_string(),
            params: Some(atelier_app::workflow_policy::ActionParams::ReviewArtifact(
                atelier_app::workflow_policy::ReviewArtifactActionParams {
                    provider: None,
                    role: "worker".to_string(),
                    role_authors: None,
                },
            )),
        }
    }

    #[test]
    fn transition_action_plan_is_ordered_and_side_effect_free() {
        let issue = test_issue("atelier-epic1");
        let resolution = BranchLifecycleResolution {
            issue_id: "atelier-epic1".to_string(),
            owner_id: "atelier-epic1".to_string(),
            owner_issue_type: "epic".to_string(),
            owner_kind: BranchOwnerKind::Epic,
            expected_branch: "epic/atelier-epic1".to_string(),
            base_branch: "master".to_string(),
            merge_strategy: MergeStrategy::Squash,
            merge_owned: true,
            nested_under_epic: false,
        };
        let actions = vec![review_action(), action("branch_integrate")];

        let plan = plan_actions_for_resolution(&issue, &resolution, &actions, 1);

        assert_eq!(issue.status, "in_progress");
        assert_eq!(plan.len(), 2);
        assert_eq!(plan[0].order, 1);
        assert_eq!(plan[0].name, "review.open");
        assert_eq!(plan[0].target_issue_id, "atelier-epic1");
        assert_eq!(plan[0].branch_owner_id, "atelier-epic1");
        assert_eq!(
            plan[0].review_artifact_target.as_deref(),
            Some("atelier-epic1")
        );
        assert_eq!(plan[0].review_artifact_provider.as_deref(), None);
        assert_eq!(plan[0].review_artifact_role.as_deref(), Some("worker"));
        assert!(!plan[0].confirmation_required);
        assert_eq!(plan[1].order, 2);
        assert_eq!(plan[1].name, "branch_integrate");
        assert!(plan[1].review_artifact_target.is_none());
        assert!(plan[1].review_artifact_role.is_none());
        assert!(plan[1].confirmation_required);
        assert!(plan.iter().all(|action| action.skip_reason.is_none()));
        assert!(plan.iter().all(|action| action.block_reason.is_none()));
    }

    #[test]
    fn branch_prepare_is_explicit_planned_action() {
        let issue = test_issue("atelier-epic1");
        let resolution = BranchLifecycleResolution {
            issue_id: "atelier-epic1".to_string(),
            owner_id: "atelier-epic1".to_string(),
            owner_issue_type: "epic".to_string(),
            owner_kind: BranchOwnerKind::Epic,
            expected_branch: "epic/atelier-epic1".to_string(),
            base_branch: "master".to_string(),
            merge_strategy: MergeStrategy::Squash,
            merge_owned: true,
            nested_under_epic: false,
        };

        let empty_plan = plan_actions_for_resolution(&issue, &resolution, &[], 1);
        assert!(empty_plan.is_empty());
        assert!(!planned_actions_need_branch_context(&empty_plan));

        let plan = plan_actions_for_resolution(&issue, &resolution, &[action("branch.prepare")], 1);
        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].name, "branch.prepare");
        assert!(planned_actions_need_branch_context(&plan));
    }

    #[test]
    fn branch_lifecycle_state_bounds_dirty_path_summary() {
        let context = BranchLifecycleContext {
            resolution: BranchLifecycleResolution {
                issue_id: "atelier-epic1".to_string(),
                owner_id: "atelier-epic1".to_string(),
                owner_issue_type: "epic".to_string(),
                owner_kind: BranchOwnerKind::Epic,
                expected_branch: "epic/atelier-epic1".to_string(),
                base_branch: "master".to_string(),
                merge_strategy: MergeStrategy::Squash,
                merge_owned: true,
                nested_under_epic: false,
            },
            current_branch: Some("master".to_string()),
            expected_branch_exists: false,
            base_branch_exists: true,
            dirty_entries: vec![
                "M first.txt".to_string(),
                "M second.txt".to_string(),
                "M third.txt".to_string(),
                "M fourth.txt".to_string(),
                "M fifth.txt".to_string(),
            ],
        };

        let summary = branch_lifecycle_state_line(&context);

        assert!(summary.contains("dirty checkout: 5 paths:"));
        assert!(summary.contains("M first.txt"));
        assert!(summary.contains("2 more omitted"));
        assert!(!summary.contains("M fifth.txt"));
    }

    #[test]
    fn provider_terminal_actions_plan_without_local_branch_integrate() {
        let issue = test_issue("atelier-epic1");
        let resolution = BranchLifecycleResolution {
            issue_id: "atelier-epic1".to_string(),
            owner_id: "atelier-epic1".to_string(),
            owner_issue_type: "epic".to_string(),
            owner_kind: BranchOwnerKind::Epic,
            expected_branch: "epic/atelier-epic1".to_string(),
            base_branch: "master".to_string(),
            merge_strategy: MergeStrategy::Squash,
            merge_owned: true,
            nested_under_epic: false,
        };
        let actions = vec![
            action("tracker.commit"),
            action("branch.push"),
            action("review.merge"),
            action("base.sync"),
        ];

        let plan = plan_actions_for_resolution(&issue, &resolution, &actions, 1);

        assert_eq!(
            plan.iter()
                .map(|action| action.name.as_str())
                .collect::<Vec<_>>(),
            vec!["tracker.commit", "branch.push", "review.merge", "base.sync"]
        );
        assert!(plan.iter().all(|action| !action.confirmation_required));
        assert!(plan.iter().all(|action| action.name != "branch_integrate"));
    }
}
