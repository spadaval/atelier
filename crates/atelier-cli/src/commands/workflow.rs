use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use serde::Serialize;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use crate::commands::agent_factory::issue_evidence_gate_status;
use atelier_app::forgejo::{ForgejoClient, ForgejoTransport, UreqForgejoTransport};
use atelier_app::pr as app_pr;
use atelier_app::project_config::{ProjectConfig, ReviewConfig, ReviewProviderKind};
use atelier_app::review_room;
use atelier_app::use_cases as app_use_cases;
use atelier_app::workflow_policy::{
    ActionParams, BranchLifecycleResolution, MergeStrategy, WorkflowForgejoRoleAuthors,
};
use atelier_core::{EvidenceRecord, Issue, Record};
use atelier_records::{CanonicalIssueRecord, IssueSections};
use atelier_sqlite::Database;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct ValidatorResult {
    pub target_kind: String,
    pub target_id: String,
    pub transition: String,
    pub validator: String,
    pub passed: bool,
    pub reason: String,
    pub elapsed_ms: u128,
}

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
    pub forgejo_role_authors: Option<atelier_app::project_config::ForgejoRoleAuthors>,
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

pub fn check(db: &Database) -> Result<()> {
    let repo_root = repo_root()?;
    let report = atelier_app::workflow_policy::check(db, &repo_root)?;
    println!("Workflow Check");
    println!("==============");
    println!(
        "Path:           {}",
        atelier_app::workflow_policy::WORKFLOW_POLICY_PATH
    );
    println!("Policy:         pass");
    println!(
        "Applicability:  {}",
        report.policy.workflow_by_issue_type.len()
    );
    println!("Statuses:       {}", report.policy.statuses.len());
    println!("Workflows:      {}", report.policy.workflows.len());
    println!("Record Health:  pass");
    println!("Issues Checked: {}", report.issue_count);
    let (command_surface_passed, command_surface_reason) =
        crate::command_surface::status_reason(&repo_root)?;
    if command_surface_passed {
        println!("Docs/Help Drift: clear");
    } else {
        println!("Docs/Help Drift: detected");
        println!("{command_surface_reason}");
        bail!("workflow_command_surface_drift: {command_surface_reason}");
    }
    Ok(())
}

pub fn issue_transition_options(
    db: &Database,
    issue_ref: &str,
) -> Result<Vec<IssueTransitionOption>> {
    let issue_id = crate::commands::agent_factory::resolve_id(db, issue_ref)?;
    let repo_root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let issue = db.require_issue(&issue_id)?;
    ensure_transitionable_status(&policy, &issue)?;
    let state_dir = atelier_app::storage_layout::StorageLayout::new(&repo_root).canonical_dir();
    let record = app_use_cases::load_canonical_issue(&state_dir, &issue.id)?;
    let mut options = Vec::new();

    for (name, transition) in policy.transitions_from_status(&issue.issue_type, &issue.status)? {
        let mut blockers = required_field_failures(&record, transition, None)?;
        blockers.extend(branch_context_blockers(db, &issue, name, transition)?);
        let validator_results = evaluate_policy_transition(
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
        descriptions.extend(branch_context_guidance(db, &issue, name, transition)?);
        let planned_actions = plan_transition_actions(db, &issue, transition)?;
        blockers.extend(action_preflight_blockers(&repo_root, &planned_actions));
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

fn plan_transition_actions(
    db: &Database,
    issue: &Issue,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
) -> Result<Vec<PlannedAction>> {
    let repo_root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let resolution =
        atelier_app::workflow_policy::resolve_branch_lifecycle(&policy, db, &issue.id)?;
    Ok(plan_actions_for_resolution(
        issue,
        &resolution,
        &transition.actions,
    ))
}

fn plan_actions_for_resolution(
    issue: &Issue,
    resolution: &BranchLifecycleResolution,
    actions: &[atelier_app::workflow_policy::ActionDefinition],
) -> Vec<PlannedAction> {
    actions
        .iter()
        .enumerate()
        .map(|(index, action)| {
            let review_artifact = review_artifact_action_plan(action, resolution);
            PlannedAction {
                order: index + 1,
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
    role_authors: Option<atelier_app::project_config::ForgejoRoleAuthors>,
}

fn review_artifact_action_plan(
    action: &atelier_app::workflow_policy::ActionDefinition,
    resolution: &BranchLifecycleResolution,
) -> Option<ReviewArtifactActionPlan> {
    if !matches!(action.builtin.as_str(), "review.open" | "review.link") {
        return None;
    }
    let Some(ActionParams::ReviewArtifact(params)) = action.params.as_ref() else {
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
) -> atelier_app::project_config::ForgejoRoleAuthors {
    atelier_app::project_config::ForgejoRoleAuthors {
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
    let repo_root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let resolution = atelier_app::workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?;
    Ok(BranchLifecycleContext {
        expected_branch_exists: branch_exists_at(&repo_root, &resolution.expected_branch)?,
        base_branch_exists: branch_exists_at(&repo_root, &resolution.base_branch)?,
        current_branch: git_current_branch(&repo_root)
            .ok()
            .filter(|branch| !branch.is_empty()),
        dirty_entries: git_dirty_entries(&repo_root)?,
        resolution,
    })
}

pub(crate) fn known_branch_owner(
    db: &Database,
    branch: &str,
) -> Result<Option<BranchLifecycleResolution>> {
    let repo_root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let mut owner_ids = BTreeSet::new();
    for issue in db.list_issues(Some("all"), None, None)? {
        let resolution =
            atelier_app::workflow_policy::resolve_branch_lifecycle(&policy, db, &issue.id)?;
        if resolution.expected_branch == branch && owner_ids.insert(resolution.owner_id.clone()) {
            return Ok(Some(resolution));
        }
    }
    Ok(None)
}

pub(crate) fn configured_base_branch() -> Result<String> {
    Ok(atelier_app::workflow_policy::load(&repo_root()?)?
        .branch_policy
        .base_branch)
}

pub(crate) fn current_git_branch() -> Result<Option<String>> {
    Ok(git_current_branch(&repo_root()?)
        .ok()
        .filter(|branch| !branch.is_empty()))
}

pub(crate) fn branch_ahead_count(branch: &str, base_branch: &str) -> Result<Option<usize>> {
    let repo_root = repo_root()?;
    if !branch_exists_at(&repo_root, branch)? || !branch_exists_at(&repo_root, base_branch)? {
        return Ok(None);
    }
    let output = git_stdout(
        &repo_root,
        &["rev-list", "--count", &format!("{base_branch}..{branch}")],
        "count branch commits ahead of base",
    )?;
    Ok(output.trim().parse::<usize>().ok())
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
    match context.current_branch.as_deref() {
        Some(current) if current == context.resolution.expected_branch => {
            "current branch matches expected branch".to_string()
        }
        Some(current) => format!(
            "mismatch - current branch {current}; run `atelier start {}` before continuing work",
            context.resolution.issue_id
        ),
        None => format!(
            "detached or unknown - run `atelier start {}` before continuing work",
            context.resolution.issue_id
        ),
    }
}

pub(crate) fn branch_lifecycle_scope_line(context: &BranchLifecycleContext) -> &'static str {
    if context.resolution.nested_under_epic {
        "nested under epic; merge is deferred to epic close"
    } else {
        "owns its merge branch"
    }
}

fn branch_context_blockers(
    db: &Database,
    issue: &Issue,
    transition_name: &str,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
) -> Result<Vec<String>> {
    let policy = atelier_app::workflow_policy::load(&repo_root()?)?;
    let is_start = transition_name == "start";
    let is_close = policy.status_category(&transition.to) == Some("done");
    if !is_start && !is_close {
        return Ok(Vec::new());
    }

    let context = branch_lifecycle_context(db, &issue.id)?;
    let mut blockers = Vec::new();
    if is_start {
        if !context.dirty_entries.is_empty() {
            blockers.push(format!(
                "branch context: worktree has uncommitted changes; inspect `git status --short --branch`, then rerun `atelier start {}`",
                issue.id
            ));
        }
        if context.current_branch.as_deref() != Some(context.resolution.expected_branch.as_str())
            && !context.expected_branch_exists
            && !context.base_branch_exists
        {
            blockers.push(format!(
                "branch context: configured base branch '{}' is missing; create or fetch it, then rerun `atelier start {}`",
                context.resolution.base_branch, issue.id
            ));
        }
    }
    if is_close && context.resolution.merge_owned && !context.base_branch_exists {
        blockers.push(format!(
            "branch context: configured base branch '{}' is missing; create or fetch it, then rerun `atelier issue close {} --reason \"...\"`",
            context.resolution.base_branch, issue.id
        ));
    }
    Ok(blockers)
}

fn branch_context_guidance(
    db: &Database,
    issue: &Issue,
    transition_name: &str,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
) -> Result<Vec<String>> {
    let policy = atelier_app::workflow_policy::load(&repo_root()?)?;
    let is_start = transition_name == "start";
    let is_close = policy.status_category(&transition.to) == Some("done");
    if !is_start && !is_close {
        return Ok(Vec::new());
    }

    let context = branch_lifecycle_context(db, &issue.id)?;
    let mut guidance = Vec::new();
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
            "Corrective lifecycle command: atelier start {}",
            issue.id
        ));
    }
    if is_close {
        guidance.push(format!(
            "Close lifecycle command: atelier issue close {} --reason \"...\"",
            issue.id
        ));
    }
    Ok(guidance)
}

pub fn transition_issue(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: &str,
    transition_name: &str,
    close_reason: Option<&str>,
) -> Result<()> {
    let issue_id = crate::commands::agent_factory::resolve_id(db, issue_ref)?;
    let repo_root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let before = db.require_issue(&issue_id)?;
    ensure_transitionable_status(&policy, &before)?;
    let transition = resolve_issue_transition(&policy, &before, transition_name)?;
    ensure_transition_available(&before, transition_name, transition)?;

    let mut record = app_use_cases::load_canonical_issue(state_dir, &before.id)?;
    let (mut blockers, validator_results) = transition_blockers(
        db,
        &policy,
        &record,
        transition_name,
        transition,
        close_reason,
    )?;
    let planned_actions = plan_transition_actions(db, &before, transition)?;
    blockers.extend(action_preflight_blockers(&repo_root, &planned_actions));
    if !blockers.is_empty() {
        report_blocked_transition(
            &policy,
            &before,
            transition_name,
            transition,
            &validator_results,
            &blockers,
            &planned_actions,
        )?;
    }

    let git_rollback = TransitionGitRollback::snapshot_if_needed(
        &repo_root,
        &before,
        transition_name,
        &planned_actions,
    )?;
    let mut action_results = execute_pre_transition_actions(
        db,
        state_dir,
        db_path,
        &repo_root,
        &before,
        transition_name,
        &planned_actions,
    )?;
    apply_transition_record(&policy, state_dir, &mut record, transition, close_reason)?;
    record_applied_actions(&before.id, transition_name, &planned_actions)?;
    record_applied_transition(&before, transition_name, transition)?;
    app_use_cases::refresh_after_canonical_write(state_dir, db_path)?;
    match execute_post_transition_actions(&repo_root, &before, transition_name, &planned_actions) {
        Ok(mut results) => action_results.append(&mut results),
        Err(error) => {
            if let Some(rollback) = git_rollback {
                rollback.rollback_after_post_action_failure(state_dir, db_path)?;
            }
            bail!("{error:#}");
        }
    }
    let refreshed = app_use_cases::open_database(db_path)?;
    let issue = refreshed.require_issue(&before.id)?;
    println!("Applied transition {} to {}", transition_name, issue.id);
    println!("From:     {}", before.status);
    println!("To:       {}", issue.status);
    for result in action_results {
        println!("Action:   {} {}", result.name, result.detail);
    }
    print_heading("Next Commands");
    println!("  atelier issue show {}", issue.id);
    println!("  atelier issue transition {} --options", issue.id);
    Ok(())
}

fn resolve_issue_transition<'a>(
    policy: &'a atelier_app::workflow_policy::WorkflowPolicy,
    issue: &Issue,
    transition_name: &str,
) -> Result<&'a atelier_app::workflow_policy::TransitionDefinition> {
    if let Ok(transition) = policy.transition_for_issue_type(&issue.issue_type, transition_name) {
        return Ok(transition);
    }
    let available = policy
        .transitions_from_status(&issue.issue_type, &issue.status)?
        .into_iter()
        .map(|(name, _)| name)
        .collect::<Vec<_>>();
    if available.is_empty() {
        bail!(
            "Unknown transition '{}' for issue {}; no transitions are configured from status '{}'",
            transition_name,
            issue.id,
            issue.status
        );
    }
    bail!(
        "Unknown transition '{}' for issue {}; available from '{}' are: {}",
        transition_name,
        issue.id,
        issue.status,
        available.join(", ")
    )
}

fn ensure_transition_available(
    issue: &Issue,
    transition_name: &str,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
) -> Result<()> {
    if transition.from.iter().any(|from| from == &issue.status) {
        return Ok(());
    }
    let reason = format!(
        "transition '{}' is not available from status '{}'",
        transition_name, issue.status
    );
    crate::commands::activity_log::record_transition_blocked(
        &issue.id,
        transition_name,
        &issue.status,
        Some(&transition.to),
        &reason,
    )?;
    bail!(
        "Transition '{}' is not available from status '{}' for issue {}",
        transition_name,
        issue.status,
        issue.id
    )
}

fn transition_blockers(
    db: &Database,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    record: &CanonicalIssueRecord,
    transition_name: &str,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
    close_reason: Option<&str>,
) -> Result<(Vec<String>, Vec<ValidatorResult>)> {
    let mut blockers = required_field_failures(record, transition, close_reason)?;
    let validator_results = evaluate_policy_transition(
        db,
        policy,
        "issue",
        &record.issue.id,
        transition_name,
        &transition.validators,
    )?;
    blockers.extend(
        validator_results
            .iter()
            .filter(|result| !result.passed)
            .map(|result| format!("validator {} failed: {}", result.validator, result.reason)),
    );
    Ok((blockers, validator_results))
}

fn report_blocked_transition(
    _policy: &atelier_app::workflow_policy::WorkflowPolicy,
    issue: &Issue,
    transition_name: &str,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
    validator_results: &[ValidatorResult],
    blockers: &[String],
    planned_actions: &[PlannedAction],
) -> Result<()> {
    let reason = blockers.join("; ");
    crate::commands::activity_log::record_transition_blocked(
        &issue.id,
        transition_name,
        &issue.status,
        Some(&transition.to),
        &reason,
    )?;
    print_transition_attempt(
        issue,
        transition_name,
        &transition.to,
        validator_results,
        blockers,
        planned_actions,
        &transition_descriptions(transition),
        &transition_command(&issue.id, transition_name, transition),
    );
    bail!(
        "Transition '{}' is blocked for issue {}: {}",
        transition_name,
        issue.id,
        reason
    )
}

#[derive(Debug, Clone)]
struct AppliedAction {
    name: String,
    detail: String,
}

fn action_preflight_blockers(repo_root: &Path, planned_actions: &[PlannedAction]) -> Vec<String> {
    planned_actions
        .iter()
        .filter_map(|action| {
            match action.name.as_str() {
                "branch_prepare" => branch_prepare_preflight(repo_root, action),
                "branch_commit" | "branch_integrate" => branch_post_action_preflight(repo_root, action),
                "review.open" => review_open_preflight(repo_root, action),
                other => Some(format!(
                    "action {other} failed preflight: action execution is not implemented yet; retry after the owning action issue lands"
                )),
            }
        })
        .collect()
}

fn branch_prepare_preflight(repo_root: &Path, action: &PlannedAction) -> Option<String> {
    if let Err(error) = ensure_git_action_repo(repo_root, action) {
        return Some(error);
    }
    match non_tracker_dirty_entries(repo_root) {
        Ok(dirty) if !dirty.is_empty() => {
            return Some(format!(
                "action {} failed preflight: worktree has uncommitted non-tracker changes:\n{}",
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
    if git_current_branch(repo_root).ok().as_deref() != Some(action.expected_branch.as_str())
        && !branch_exists_at(repo_root, &action.expected_branch).unwrap_or(false)
        && !branch_exists_at(repo_root, &action.base_branch).unwrap_or(false)
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
    match non_tracker_dirty_entries(repo_root) {
        Ok(dirty) if !dirty.is_empty() => Some(format!(
            "action {} failed preflight: worktree has uncommitted non-tracker changes:\n{}",
            action.name,
            dirty.join("\n")
        )),
        Err(error) => Some(format!("action {} failed preflight: {error:#}", action.name)),
        _ if action.name == "branch_integrate"
            && action.merge_owned
            && !branch_exists_at(repo_root, &action.base_branch).unwrap_or(false) =>
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
    match is_git_repo(repo_root) {
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
            ReviewProviderKind::Forgejo(forgejo) => {
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
                env::var(&forgejo.admin_token_env).err().map(|_| {
                    format!(
                        "action {} failed preflight: environment variable {} is required for provider review open",
                        action.name, forgejo.admin_token_env
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

fn execute_pre_transition_actions(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    repo_root: &Path,
    issue: &Issue,
    transition_name: &str,
    planned_actions: &[PlannedAction],
) -> Result<Vec<AppliedAction>> {
    let mut applied = Vec::new();
    for action in planned_actions {
        match action.name.as_str() {
            "branch_prepare" => {
                let detail = prepare_branch_action(repo_root, issue, action)?;
                applied.push(AppliedAction {
                    name: action.name.clone(),
                    detail,
                });
            }
            "branch_commit" | "branch_integrate" => {}
            "review.open" => {
                let detail = open_review_artifact_action(
                    db,
                    state_dir,
                    db_path,
                    repo_root,
                    issue,
                    transition_name,
                    action,
                )?;
                applied.push(AppliedAction {
                    name: action.name.clone(),
                    detail,
                });
            }
            other => bail!(
                "action {other} failed: action execution is not implemented; status was not changed"
            ),
        }
    }
    Ok(applied)
}

fn execute_post_transition_actions(
    repo_root: &Path,
    issue: &Issue,
    transition_name: &str,
    planned_actions: &[PlannedAction],
) -> Result<Vec<AppliedAction>> {
    let mut applied = Vec::new();
    for action in planned_actions {
        match action.name.as_str() {
            "branch_commit" => {
                let detail = commit_branch_action(repo_root, issue, transition_name, action)?;
                applied.push(AppliedAction {
                    name: action.name.clone(),
                    detail,
                });
            }
            "branch_integrate" => {
                let detail = integrate_branch_action(repo_root, issue, action)?;
                applied.push(AppliedAction {
                    name: action.name.clone(),
                    detail,
                });
            }
            _ => {}
        }
    }
    Ok(applied)
}

fn prepare_branch_action(
    repo_root: &Path,
    issue: &Issue,
    action: &PlannedAction,
) -> Result<String> {
    ensure_non_tracker_clean_for_action(repo_root, action, issue, "before workflow transition")?;
    let current = git_current_branch(repo_root).unwrap_or_default();
    if current == action.expected_branch {
        return Ok(format!("already on branch {}", action.expected_branch));
    }
    if branch_exists_at(repo_root, &action.expected_branch)? {
        git_checked(repo_root, &["switch", &action.expected_branch], "checkout action branch")
            .with_context(|| {
                format!(
                    "action {} failed while switching to branch '{}'.\nRecovery: inspect `git status --short --branch`, then retry `atelier issue transition {} start`.",
                    action.name, action.expected_branch, issue.id
                )
            })?;
        return Ok(format!("checked out branch {}", action.expected_branch));
    }
    ensure_branch_exists(repo_root, &action.base_branch).with_context(|| {
        format!(
            "action {} failed because configured base branch '{}' is missing.\nRecovery: create or fetch the base branch, then retry `atelier issue transition {} start`.",
            action.name, action.base_branch, issue.id
        )
    })?;
    git_checked(
        repo_root,
        &["switch", "-c", &action.expected_branch, &action.base_branch],
        "create action branch",
    )
    .with_context(|| {
        format!(
            "action {} failed while creating branch '{}' from '{}'.\nRecovery: inspect `git status --short --branch`, then retry `atelier issue transition {} start`.",
            action.name, action.expected_branch, action.base_branch, issue.id
        )
    })?;
    Ok(format!(
        "created branch {} from {}",
        action.expected_branch, action.base_branch
    ))
}

fn commit_branch_action(
    repo_root: &Path,
    issue: &Issue,
    transition_name: &str,
    action: &PlannedAction,
) -> Result<String> {
    ensure_expected_branch_checked_out(repo_root, issue, action)?;
    git_checked(repo_root, &["add", "-A", ".atelier"], "stage transition tracker state")
        .with_context(|| {
            format!(
                "action {} failed while staging tracker state.\nRecovery: tracker state was restored when possible; inspect `git status --short --branch`, then retry `atelier issue transition {} {}`.",
                action.name, issue.id, transition_name
            )
        })?;
    if git_checked(
        repo_root,
        &["diff", "--cached", "--quiet"],
        "inspect staged tracker state",
    )
    .is_ok()
    {
        return Ok("no tracker changes to commit".to_string());
    }
    let message = format!(
        "Transition {} {}: {}",
        issue.id, transition_name, issue.title
    );
    git_checked(
        repo_root,
        &["commit", "-m", &message],
        "commit transition tracker state",
    )
    .with_context(|| {
        format!(
            "action {} failed while committing tracker state.\nRecovery: tracker state was restored when possible; inspect `git status --short --branch`, then retry `atelier issue transition {} {}`.",
            action.name, issue.id, transition_name
        )
    })?;
    let sha = git_stdout(
        repo_root,
        &["rev-parse", "--short", "HEAD"],
        "read action commit",
    )?;
    Ok(format!("committed tracker state {}", sha.trim()))
}

fn integrate_branch_action(
    repo_root: &Path,
    issue: &Issue,
    action: &PlannedAction,
) -> Result<String> {
    if !action.merge_owned {
        return Ok("deferred to parent branch close".to_string());
    }
    ensure_branch_exists(repo_root, &action.base_branch).with_context(|| {
        format!(
            "action {} failed because configured base branch '{}' is missing.\nRecovery: create or fetch the base branch, then retry the transition for {}.",
            action.name, action.base_branch, issue.id
        )
    })?;
    git_checked(
        repo_root,
        &["switch", &action.base_branch],
        "checkout action integration target",
    )
    .with_context(|| {
        format!(
            "action {} failed while switching to base branch '{}'.\nRecovery: source branch '{}' contains transition work; inspect `git status --short --branch`, switch to the base branch, and retry integration or the transition after repair.",
            action.name, action.base_branch, action.expected_branch
        )
    })?;
    match action.merge_strategy {
        MergeStrategy::Squash => {
            git_checked(
                repo_root,
                &["merge", "--squash", &action.expected_branch],
                "squash merge action branch",
            )
            .with_context(|| {
                format!(
                    "action {} failed during squash merge from '{}' to '{}'.\nRecovery: merge state was aborted when possible and issue status was restored; inspect `git status --short --branch`, then retry the transition for {}.",
                    action.name, action.expected_branch, action.base_branch, issue.id
                )
            })?;
            let message = format!(
                "Squash merge {} into {}",
                action.expected_branch, action.base_branch
            );
            git_checked(repo_root, &["commit", "-m", &message], "commit action squash merge")
                .with_context(|| {
                    format!(
                        "action {} failed while committing squash merge on '{}'.\nRecovery: merge state was aborted when possible and issue status was restored; inspect `git status --short --branch`, then retry the transition for {}.",
                        action.name, action.base_branch, issue.id
                    )
                })?;
            let sha = git_stdout(
                repo_root,
                &["rev-parse", "--short", "HEAD"],
                "read squash commit",
            )?;
            Ok(format!("squash commit {}", sha.trim()))
        }
        MergeStrategy::MergeCommit => {
            let message = format!(
                "Merge {} into {}",
                action.expected_branch, action.base_branch
            );
            git_checked(
                repo_root,
                &["merge", "--no-ff", &action.expected_branch, "-m", &message],
                "merge action branch",
            )
            .with_context(|| {
                format!(
                    "action {} failed during merge from '{}' to '{}'.\nRecovery: merge state was aborted when possible and issue status was restored; inspect `git status --short --branch`, then retry the transition for {}.",
                    action.name, action.expected_branch, action.base_branch, issue.id
                )
            })?;
            let sha = git_stdout(
                repo_root,
                &["rev-parse", "--short", "HEAD"],
                "read merge commit",
            )?;
            Ok(format!("merge commit {}", sha.trim()))
        }
        MergeStrategy::FastForwardOnly => {
            git_checked(
                repo_root,
                &["merge", "--ff-only", &action.expected_branch],
                "fast-forward action branch",
            )
            .with_context(|| {
                format!(
                    "action {} failed during fast-forward from '{}' to '{}'.\nRecovery: merge state was aborted when possible and issue status was restored; inspect `git status --short --branch`, then retry the transition for {}.",
                    action.name, action.expected_branch, action.base_branch, issue.id
                )
            })?;
            let sha = git_stdout(
                repo_root,
                &["rev-parse", "--short", "HEAD"],
                "read fast-forward head",
            )?;
            Ok(format!("fast-forward to {}", sha.trim()))
        }
    }
}

fn ensure_expected_branch_checked_out(
    repo_root: &Path,
    issue: &Issue,
    action: &PlannedAction,
) -> Result<()> {
    let current = git_current_branch(repo_root).unwrap_or_default();
    if current == action.expected_branch {
        return Ok(());
    }
    ensure_branch_exists(repo_root, &action.expected_branch).with_context(|| {
        format!(
            "action {} failed because source branch '{}' is missing.\nRecovery: run the transition with `branch_prepare` first, then retry the transition for {}.",
            action.name, action.expected_branch, issue.id
        )
    })?;
    git_checked(
        repo_root,
        &["switch", &action.expected_branch],
        "checkout action source branch",
    )
    .with_context(|| {
        format!(
            "action {} failed while switching to source branch '{}'.\nRecovery: inspect `git status --short --branch`, then retry the transition for {}.",
            action.name, action.expected_branch, issue.id
        )
    })
}

fn open_review_artifact_action(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    repo_root: &Path,
    issue: &Issue,
    transition_name: &str,
    action: &PlannedAction,
) -> Result<String> {
    let policy = atelier_app::workflow_policy::load(repo_root)?;
    let resolution =
        atelier_app::workflow_policy::resolve_branch_lifecycle(&policy, db, &issue.id)?;
    if let Some(detail) = existing_review_artifact_detail(state_dir, &resolution.owner_id)? {
        return Ok(detail);
    }
    let title = format!("Review {} {}", resolution.owner_id, transition_name);
    let body = format!(
        "Opened by transition action `{}` for issue {}.",
        action.name, issue.id
    );
    let role = action.review_artifact_role.as_deref().ok_or_else(|| {
        anyhow!(
            "action {} failed: missing workflow action role",
            action.name
        )
    })?;
    match ProjectConfig::load(repo_root)?.review {
        ReviewConfig::Room => {
            if action.review_artifact_provider.is_some() {
                bail!(
                    "action {} failed: provider action config is only valid when review.mode = \"provider\"",
                    action.name
                );
            }
            let outcome = review_room::open(
                db,
                review_room::RoomOpenRequest {
                    repo_root,
                    state_dir,
                    db_path,
                    issue_ref: Some(&resolution.owner_id),
                    role,
                    title: &title,
                    body: &body,
                    source_branch: &resolution.expected_branch,
                    target_branch: &resolution.base_branch,
                },
            )?;
            Ok(format!("opened room {}", outcome.review_id))
        }
        ReviewConfig::Provider(provider) => match provider.provider {
            ReviewProviderKind::Forgejo(mut forgejo) => {
                if action.review_artifact_provider.as_deref() != Some("forgejo") {
                    bail!(
                        "action {} failed: provider review open requires workflow action provider: forgejo",
                        action.name
                    );
                }
                forgejo.role_authors = Some(action.forgejo_role_authors.clone().ok_or_else(|| {
                    anyhow!(
                        "action {} failed: provider review open requires workflow action role_authors",
                        action.name
                    )
                })?);
                let token = env::var(&forgejo.admin_token_env).with_context(|| {
                    format!(
                        "action {} failed: environment variable {} is required for provider review open",
                        action.name, forgejo.admin_token_env
                    )
                })?;
                let client = ForgejoClient::new(
                    forgejo.clone(),
                    UreqForgejoTransport::new(&forgejo.host, token),
                );
                let outcome = app_pr::open_with_client(
                    db,
                    app_pr::PrOpenRequest {
                        repo_root,
                        state_dir,
                        db_path,
                        issue_ref: Some(&resolution.owner_id),
                        role,
                        title: &title,
                        body: &body,
                        source_branch: &resolution.expected_branch,
                        target_branch: &resolution.base_branch,
                    },
                    &forgejo,
                    &client,
                )?;
                Ok(format!("opened provider review {}", outcome.pull.url))
            }
        },
    }
}

fn existing_review_artifact_detail(state_dir: &Path, owner_id: &str) -> Result<Option<String>> {
    let owner = app_use_cases::load_canonical_issue(state_dir, owner_id)?;
    let Some(review) = owner.issue.fields.get("review") else {
        return Ok(None);
    };
    let kind = review
        .get("kind")
        .and_then(Value::as_str)
        .unwrap_or("review");
    let detail = match kind {
        "room" => review
            .get("id")
            .and_then(Value::as_str)
            .map(|id| format!("reused room {id}"))
            .unwrap_or_else(|| "reused room review".to_string()),
        "pull_request" => {
            let provider = review
                .get("provider")
                .and_then(Value::as_str)
                .unwrap_or("provider");
            let number = review
                .get("number")
                .and_then(Value::as_i64)
                .map(|number| format!("#{number}"))
                .unwrap_or_else(|| "review".to_string());
            format!("reused provider review {provider}{number}")
        }
        other => format!("reused {other} review"),
    };
    Ok(Some(detail))
}

fn record_applied_actions(
    issue_id: &str,
    transition_name: &str,
    planned_actions: &[PlannedAction],
) -> Result<()> {
    for action in planned_actions {
        crate::commands::activity_log::record_note(
            issue_id,
            &format!(
                "transition: {}\naction: {}\norder: {}\nstatus: applied\ntarget_issue: {}\nbranch_owner: {}\nreview_artifact_target: {}\nreview_artifact_provider: {}\nreview_artifact_role: {}",
                transition_name,
                action.name,
                action.order,
                action.target_issue_id,
                action.branch_owner_id,
                action
                    .review_artifact_target
                    .as_deref()
                    .unwrap_or("(none)"),
                action
                    .review_artifact_provider
                    .as_deref()
                    .unwrap_or("(none)"),
                action.review_artifact_role.as_deref().unwrap_or("(none)")
            ),
        )?;
    }
    Ok(())
}

fn apply_transition_record(
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    state_dir: &Path,
    record: &mut CanonicalIssueRecord,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
    close_reason: Option<&str>,
) -> Result<()> {
    let now = Utc::now();
    record.issue.status = transition.to.clone();
    record.issue.updated_at = now;
    record.issue.closed_at = if policy.status_category(&transition.to) == Some("done") {
        Some(now)
    } else {
        None
    };
    app_use_cases::write_canonical_issue(state_dir, record)?;
    if let Some(reason) = close_reason {
        crate::commands::activity_log::record_close_reason(&record.issue.id, reason)?;
    }
    Ok(())
}

fn record_applied_transition(
    issue: &Issue,
    transition_name: &str,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
) -> Result<()> {
    crate::commands::activity_log::record_transition_applied(
        &issue.id,
        transition_name,
        &issue.status,
        &transition.to,
    )
}

pub fn close_issue(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: &str,
    to_status: Option<&str>,
    close_reason: &str,
) -> Result<()> {
    let issue_id = crate::commands::agent_factory::resolve_id(db, issue_ref)?;
    let repo_root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let issue = db.require_issue(&issue_id)?;
    ensure_transitionable_status(&policy, &issue)?;
    let mut candidates = policy
        .transitions_from_status(&issue.issue_type, &issue.status)?
        .into_iter()
        .filter(|(_, transition)| policy.status_category(&transition.to) == Some("done"))
        .map(|(name, transition)| (name, transition.to.as_str()))
        .collect::<Vec<_>>();

    if candidates.is_empty() {
        bail!(
            "Issue {} has no terminal done-category transitions from status '{}'; inspect `atelier issue transition {} --options`",
            issue.id,
            issue.status,
            issue.id
        );
    }

    if let Some(to_status) = to_status {
        candidates.retain(|(_, destination)| *destination == to_status);
        if candidates.is_empty() {
            let available = policy
                .transitions_from_status(&issue.issue_type, &issue.status)?
                .into_iter()
                .filter(|(_, transition)| policy.status_category(&transition.to) == Some("done"))
                .map(|(_, transition)| transition.to.as_str())
                .collect::<BTreeSet<_>>();
            bail!(
                "Issue {} cannot close to status '{}'; available done targets from '{}' are: {}",
                issue.id,
                to_status,
                issue.status,
                available.into_iter().collect::<Vec<_>>().join(", ")
            );
        }
    } else {
        let destinations = candidates
            .iter()
            .map(|(_, destination)| *destination)
            .collect::<BTreeSet<_>>();
        if destinations.len() > 1 {
            bail!(
                "Issue {} has multiple terminal done targets from '{}'; rerun with `atelier issue close {} --to <status> --reason \"...\"` (available: {})",
                issue.id,
                issue.status,
                issue.id,
                destinations.into_iter().collect::<Vec<_>>().join(", ")
            );
        }
    }

    if candidates.len() > 1 {
        let transitions = candidates
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
            .join(", ");
        bail!(
            "Issue {} has multiple terminal transitions to status '{}': {}; use `atelier issue transition {} <transition> --reason \"...\"`",
            issue.id,
            candidates[0].1,
            transitions,
            issue.id
        );
    }

    let transition = resolve_issue_transition(&policy, &issue, candidates[0].0)?;
    let close_git = if transition_declares_branch_git_actions(transition) {
        None
    } else {
        CloseGitIntegration::prepare(db, &policy, &issue)?
    };
    transition_issue(
        db,
        state_dir,
        db_path,
        &issue.id,
        candidates[0].0,
        Some(close_reason),
    )?;

    if let Some(mut close_git) = close_git {
        if let Err(error) = close_git.integrate(state_dir, db_path) {
            bail!("{error:#}");
        }
    } else {
        let _ = app_use_cases::open_database(db_path)?;
    }
    Ok(())
}

fn transition_declares_branch_git_actions(
    transition: &atelier_app::workflow_policy::TransitionDefinition,
) -> bool {
    transition.actions.iter().any(|action| {
        matches!(
            action.builtin.as_str(),
            "branch_commit" | "branch_integrate"
        )
    })
}

struct CloseGitIntegration {
    repo_root: PathBuf,
    issue_id: String,
    issue_title: String,
    resolution: BranchLifecycleResolution,
    source_pre_head: String,
    tracker_patch_before_close: Vec<u8>,
}

struct TransitionGitRollback {
    repo_root: PathBuf,
    issue_id: String,
    transition_name: String,
    expected_branch: String,
    source_pre_head: String,
    tracker_patch_before_transition: Vec<u8>,
}

impl TransitionGitRollback {
    fn snapshot_if_needed(
        repo_root: &Path,
        issue: &Issue,
        transition_name: &str,
        planned_actions: &[PlannedAction],
    ) -> Result<Option<Self>> {
        if !planned_actions
            .iter()
            .any(|action| matches!(action.name.as_str(), "branch_commit" | "branch_integrate"))
        {
            return Ok(None);
        }
        if !is_git_repo(repo_root)? {
            return Ok(None);
        }
        let Some(action) = planned_actions
            .iter()
            .find(|action| matches!(action.name.as_str(), "branch_commit" | "branch_integrate"))
        else {
            return Ok(None);
        };
        let source_pre_head = git_stdout(
            repo_root,
            &["rev-parse", "HEAD"],
            "read pre-transition HEAD",
        )?;
        let tracker_patch_before_transition = git_binary_stdout(
            repo_root,
            &["diff", "--binary", "--", ".atelier"],
            "snapshot tracker changes before transition action",
        )?;
        Ok(Some(Self {
            repo_root: repo_root.to_path_buf(),
            issue_id: issue.id.clone(),
            transition_name: transition_name.to_string(),
            expected_branch: action.expected_branch.clone(),
            source_pre_head: source_pre_head.trim().to_string(),
            tracker_patch_before_transition,
        }))
    }

    fn rollback_after_post_action_failure(&self, state_dir: &Path, db_path: &Path) -> Result<()> {
        if git_checked(
            &self.repo_root,
            &["merge", "--abort"],
            "abort failed action merge",
        )
        .is_err()
        {
            git_checked(
                &self.repo_root,
                &["reset", "--hard", "HEAD"],
                "reset failed action merge state",
            )?;
        }
        if branch_exists_at(&self.repo_root, &self.expected_branch)? {
            git_checked(
                &self.repo_root,
                &["switch", &self.expected_branch],
                "return to action source branch for rollback",
            )?;
        }
        git_checked(
            &self.repo_root,
            &["reset", "--hard", &self.source_pre_head],
            "restore pre-transition action HEAD",
        )
        .with_context(|| {
            format!(
                "action rollback failed for {} {} while restoring git HEAD.\nRecovery: inspect `git status --short --branch` before retrying.",
                self.issue_id, self.transition_name
            )
        })?;
        if !self.tracker_patch_before_transition.is_empty() {
            let mut child = Command::new("git")
                .current_dir(&self.repo_root)
                .args(["apply", "--binary", "--whitespace=nowarn"])
                .stdin(std::process::Stdio::piped())
                .spawn()
                .context(
                    "failed to run git apply while restoring pre-transition tracker changes",
                )?;
            {
                let stdin = child
                    .stdin
                    .as_mut()
                    .context("failed to open git apply stdin")?;
                use std::io::Write;
                stdin.write_all(&self.tracker_patch_before_transition)?;
            }
            let status = child.wait().context("failed to wait for git apply")?;
            if !status.success() {
                bail!("failed to restore pre-transition tracker changes after action rollback");
            }
        }
        app_use_cases::refresh_after_canonical_write(state_dir, db_path)?;
        Ok(())
    }
}

impl CloseGitIntegration {
    fn prepare(
        db: &Database,
        policy: &atelier_app::workflow_policy::WorkflowPolicy,
        issue: &Issue,
    ) -> Result<Option<Self>> {
        let repo_root = repo_root()?;
        if !is_git_repo(&repo_root)? {
            return Ok(None);
        }
        let resolution = atelier_app::workflow_policy::resolve_branch_lifecycle(&policy, db, &issue.id).with_context(|| {
            format!(
                "Close Git integration could not resolve branch policy for {}. Inspect parent links with `atelier issue show {}`.",
                issue.id, issue.id
            )
        })?;
        ensure_no_non_tracker_dirty(&repo_root)?;
        ensure_close_branch_ready(&repo_root, &resolution)?;
        if resolution.merge_owned {
            ensure_branch_exists(&repo_root, &resolution.base_branch).with_context(|| {
                format!(
                    "Close Git integration cannot find configured base branch '{}'.\nRecovery: create or fetch the base branch, then retry `atelier issue close {} --reason \"...\"`.",
                    resolution.base_branch, issue.id
                )
            })?;
        }
        let current = git_current_branch(&repo_root)?;
        if current != resolution.expected_branch {
            git_checked(
                &repo_root,
                &["switch", &resolution.expected_branch],
                "checkout source branch before close",
            )
            .with_context(|| {
                format!(
                    "Close Git integration could not switch to source branch '{}'.\nRecovery: inspect `git status --short --branch`, then retry `atelier issue close {} --reason \"...\"`.",
                    resolution.expected_branch, issue.id
                )
            })?;
        }
        let source_pre_head = git_stdout(&repo_root, &["rev-parse", "HEAD"], "read source HEAD")?;
        let tracker_patch_before_close = git_binary_stdout(
            &repo_root,
            &["diff", "--binary", "--", ".atelier"],
            "snapshot tracker changes before close",
        )?;
        Ok(Some(Self {
            repo_root,
            issue_id: issue.id.clone(),
            issue_title: issue.title.clone(),
            resolution,
            source_pre_head: source_pre_head.trim().to_string(),
            tracker_patch_before_close,
        }))
    }

    fn integrate(&mut self, state_dir: &Path, db_path: &Path) -> Result<()> {
        println!();
        println!("Close Git Integration");
        println!("---------------------");
        println!("Target:        issue/{}", self.issue_id);
        println!(
            "Branch owner:  {} {} ({})",
            branch_owner_label(&self.resolution.owner_kind),
            self.resolution.owner_id,
            self.resolution.owner_issue_type
        );
        println!("Source branch: {}", self.resolution.expected_branch);
        println!("Base branch:   {}", self.resolution.base_branch);
        println!(
            "Merge strategy: {}",
            self.resolution.merge_strategy.as_str()
        );

        let close_commit = self.commit_tracker_state(state_dir, db_path)?;
        println!("Tracker commit: {close_commit}");
        if self.resolution.merge_owned {
            match self.merge_to_base() {
                Ok(result) => println!("Merge result:   {result}"),
                Err(error) => {
                    self.rollback_after_integration_failure(state_dir, db_path)?;
                    bail!("{error}");
                }
            }
        } else {
            println!("Merge result:   deferred to epic close");
        }
        println!(
            "Recovery:      rerun `atelier issue close {} --reason \"...\"` only if a later step reports failure",
            self.issue_id
        );
        let _ = app_use_cases::open_database(db_path)?;
        Ok(())
    }

    fn commit_tracker_state(&mut self, state_dir: &Path, db_path: &Path) -> Result<String> {
        git_checked(
            &self.repo_root,
            &["add", "-A", ".atelier"],
            "stage tracker close state",
        )
        .or_else(|error| {
            self.rollback_tracker_state(state_dir, db_path)?;
            Err(error)
        })?;
        let message = format!("Close {}: {}", self.issue_id, self.issue_title);
        git_checked(
            &self.repo_root,
            &["commit", "-m", &message],
            "commit tracker close state",
        )
        .or_else(|error| {
            self.rollback_tracker_state(state_dir, db_path)?;
            Err(error)
        })
        .with_context(|| {
            format!(
                "Close Git integration failed while committing tracker state for {}.\nRecovery: tracker files were restored to their pre-close state; inspect `git status --short --branch`, then retry `atelier issue close {} --reason \"...\"`.",
                self.issue_id, self.issue_id
            )
        })?;
        git_stdout(
            &self.repo_root,
            &["rev-parse", "--short", "HEAD"],
            "read close commit",
        )
        .map(|value| value.trim().to_string())
    }

    fn merge_to_base(&self) -> Result<String> {
        git_checked(
            &self.repo_root,
            &["switch", &self.resolution.base_branch],
            "checkout base branch before merge",
        )
        .with_context(|| {
            format!(
                "Close Git integration failed while switching to base branch '{}'.\nRecovery: source branch '{}' contains the close commit; inspect `git status --short --branch`, switch to the base branch, and retry the merge or rerun close after repair.",
                self.resolution.base_branch, self.resolution.expected_branch
            )
        })?;
        match self.resolution.merge_strategy {
            MergeStrategy::Squash => {
                git_checked(
                    &self.repo_root,
                    &["merge", "--squash", &self.resolution.expected_branch],
                    "squash merge source branch",
                )
                .with_context(|| {
                    format!(
                        "Close Git integration failed during squash merge from '{}' to '{}'.\nRecovery: merge state was aborted when possible and the source close commit was rolled back; inspect `git status --short --branch`, then retry `atelier issue close {} --reason \"...\"`.",
                        self.resolution.expected_branch, self.resolution.base_branch, self.issue_id
                    )
                })?;
                let message = format!(
                    "Squash merge {} into {}",
                    self.resolution.expected_branch, self.resolution.base_branch
                );
                git_checked(
                    &self.repo_root,
                    &["commit", "-m", &message],
                    "commit squash merge",
                )
                .with_context(|| {
                    format!(
                        "Close Git integration failed while committing squash merge on '{}'.\nRecovery: merge state was aborted when possible and the source close commit was rolled back; inspect `git status --short --branch`, then retry `atelier issue close {} --reason \"...\"`.",
                        self.resolution.base_branch, self.issue_id
                    )
                })?;
                let sha = git_stdout(
                    &self.repo_root,
                    &["rev-parse", "--short", "HEAD"],
                    "read squash commit",
                )?;
                Ok(format!("squash commit {}", sha.trim()))
            }
            MergeStrategy::MergeCommit => {
                let message = format!(
                    "Merge {} into {}",
                    self.resolution.expected_branch, self.resolution.base_branch
                );
                git_checked(
                    &self.repo_root,
                    &["merge", "--no-ff", &self.resolution.expected_branch, "-m", &message],
                    "merge source branch",
                )
                .with_context(|| {
                    format!(
                        "Close Git integration failed during merge from '{}' to '{}'.\nRecovery: merge state was aborted when possible and the source close commit was rolled back; inspect `git status --short --branch`, then retry `atelier issue close {} --reason \"...\"`.",
                        self.resolution.expected_branch, self.resolution.base_branch, self.issue_id
                    )
                })?;
                let sha = git_stdout(
                    &self.repo_root,
                    &["rev-parse", "--short", "HEAD"],
                    "read merge commit",
                )?;
                Ok(format!("merge commit {}", sha.trim()))
            }
            MergeStrategy::FastForwardOnly => {
                git_checked(
                    &self.repo_root,
                    &["merge", "--ff-only", &self.resolution.expected_branch],
                    "fast-forward source branch",
                )
                .with_context(|| {
                    format!(
                        "Close Git integration failed during fast-forward from '{}' to '{}'.\nRecovery: merge state was aborted when possible and the source close commit was rolled back; inspect `git status --short --branch`, then retry `atelier issue close {} --reason \"...\"`.",
                        self.resolution.expected_branch, self.resolution.base_branch, self.issue_id
                    )
                })?;
                let sha = git_stdout(
                    &self.repo_root,
                    &["rev-parse", "--short", "HEAD"],
                    "read fast-forward head",
                )?;
                Ok(format!("fast-forward to {}", sha.trim()))
            }
        }
    }

    fn rollback_after_integration_failure(&self, state_dir: &Path, db_path: &Path) -> Result<()> {
        if git_checked(&self.repo_root, &["merge", "--abort"], "abort failed merge").is_err() {
            git_checked(
                &self.repo_root,
                &["reset", "--hard", "HEAD"],
                "reset failed merge state",
            )?;
        }
        git_checked(
            &self.repo_root,
            &["switch", &self.resolution.expected_branch],
            "return to source branch for rollback",
        )?;
        self.rollback_tracker_state(state_dir, db_path)
    }

    fn rollback_tracker_state(&self, state_dir: &Path, db_path: &Path) -> Result<()> {
        git_checked(
            &self.repo_root,
            &["reset", "--hard", &self.source_pre_head],
            "move source branch back to pre-close HEAD",
        )?;
        if !self.tracker_patch_before_close.is_empty() {
            let mut child = Command::new("git")
                .current_dir(&self.repo_root)
                .args(["apply", "--binary", "--whitespace=nowarn"])
                .stdin(std::process::Stdio::piped())
                .spawn()
                .context("failed to run git apply while restoring pre-close tracker changes")?;
            {
                let stdin = child
                    .stdin
                    .as_mut()
                    .context("failed to open git apply stdin")?;
                use std::io::Write;
                stdin.write_all(&self.tracker_patch_before_close)?;
            }
            let status = child.wait().context("failed to wait for git apply")?;
            if !status.success() {
                bail!("failed to restore pre-close tracker changes after rollback");
            }
        }
        app_use_cases::refresh_after_canonical_write(state_dir, db_path)?;
        Ok(())
    }
}

fn is_git_repo(root: &Path) -> Result<bool> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .context("failed to inspect git repository")?;
    Ok(output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true")
}

fn ensure_close_branch_ready(root: &Path, resolution: &BranchLifecycleResolution) -> Result<()> {
    if git_current_branch(root)? == resolution.expected_branch {
        return Ok(());
    }
    if ensure_branch_exists(root, &resolution.expected_branch).is_ok() {
        return Ok(());
    }
    git_checked(
        root,
        &["switch", "-c", &resolution.expected_branch],
        "create source branch before close",
    )
    .with_context(|| {
        format!(
            "Close Git integration could not create source branch '{}'.\nRecovery: run `atelier start {}` to prepare the branch, then retry `atelier issue close {} --reason \"...\"`.",
            resolution.expected_branch, resolution.issue_id, resolution.issue_id
        )
    })
}

fn ensure_branch_exists(root: &Path, branch: &str) -> Result<()> {
    if branch_exists_at(root, branch)? {
        Ok(())
    } else {
        bail!("branch '{}' does not exist", branch)
    }
}

fn ensure_no_non_tracker_dirty(root: &Path) -> Result<()> {
    let dirty = non_tracker_dirty_entries(root)?;
    if !dirty.is_empty() {
        bail!(
            "Close Git integration requires non-tracker files to be clean before close:\n{}\nRecovery: commit or stash these paths, then retry `atelier issue close <issue-id> --reason \"...\"`.",
            dirty.join("\n")
        );
    }
    Ok(())
}

fn ensure_non_tracker_clean_for_action(
    root: &Path,
    action: &PlannedAction,
    issue: &Issue,
    phase: &str,
) -> Result<()> {
    let dirty = non_tracker_dirty_entries(root)?;
    if !dirty.is_empty() {
        bail!(
            "action {} failed {phase}: worktree has uncommitted non-tracker changes:\n{}\nRecovery: commit or stash these paths, then retry the transition for {}.",
            action.name,
            dirty.join("\n"),
            issue.id
        );
    }
    Ok(())
}

fn non_tracker_dirty_entries(root: &Path) -> Result<Vec<String>> {
    let status = git_stdout(
        root,
        &["status", "--porcelain"],
        "inspect worktree dirtiness",
    )?;
    Ok(status
        .lines()
        .filter_map(git_status_path)
        .filter(|path| !path.starts_with(".atelier/"))
        .collect::<Vec<_>>())
}

fn git_status_path(line: &str) -> Option<String> {
    let path = line.get(3..)?.trim();
    let path = path.split(" -> ").last().unwrap_or(path);
    if path.is_empty() {
        None
    } else {
        Some(path.to_string())
    }
}

fn git_current_branch(root: &Path) -> Result<String> {
    git_stdout(root, &["branch", "--show-current"], "read current branch")
        .map(|value| value.trim().to_string())
}

fn branch_exists_at(root: &Path, branch: &str) -> Result<bool> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["rev-parse", "--verify", "--quiet", branch])
        .output()
        .with_context(|| format!("failed to inspect git branch {branch}"))?;
    Ok(output.status.success())
}

fn git_dirty_entries(root: &Path) -> Result<Vec<String>> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["status", "--short", "--untracked-files=all"])
        .output()
        .context("failed to inspect git dirty state")?;
    if !output.status.success() {
        bail!(
            "git dirty state failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(ToString::to_string)
        .collect())
}

fn git_checked(root: &Path, args: &[&str], action: &str) -> Result<()> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .with_context(|| format!("failed to run git for {action}"))?;
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    bail!(
        "git {action} failed: {}{}{}",
        stderr.trim(),
        if stderr.trim().is_empty() || stdout.trim().is_empty() {
            ""
        } else {
            "\n"
        },
        stdout.trim()
    )
}

fn git_stdout(root: &Path, args: &[&str], action: &str) -> Result<String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .with_context(|| format!("failed to run git for {action}"))?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).to_string());
    }
    bail!(
        "git {action} failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    )
}

fn git_binary_stdout(root: &Path, args: &[&str], action: &str) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .with_context(|| format!("failed to run git for {action}"))?;
    if output.status.success() {
        return Ok(output.stdout);
    }
    bail!(
        "git {action} failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    )
}

pub fn print_issue_transition_options(
    db: &Database,
    issue: &Issue,
    options: &[IssueTransitionOption],
) {
    println!("Issue Transitions {} - {}", issue.id, issue.title);
    println!("{}", "=".repeat(issue.id.len() + issue.title.len() + 21));
    print_heading("State");
    println!("Status:   {}", issue.status);
    println!("Type:     {}", issue.issue_type);
    println!("Options:  {}", options.len());
    if let Ok(context) = branch_lifecycle_context(db, &issue.id) {
        print_heading("Branch Context");
        println!(
            "Owner:    {} {} ({})",
            branch_owner_label(&context.resolution.owner_kind),
            context.resolution.owner_id,
            context.resolution.owner_issue_type
        );
        println!("Expected: {}", context.resolution.expected_branch);
        println!("Base:     {}", context.resolution.base_branch);
        println!(
            "Current:  {}",
            context.current_branch.as_deref().unwrap_or("(detached)")
        );
        println!("State:    {}", branch_lifecycle_state_line(&context));
    }
    for option in options {
        println!();
        println!(
            "{} [{}]",
            option.name,
            if option.allowed { "allowed" } else { "blocked" }
        );
        println!("  From: {}", option.from.join(", "));
        println!("  To:   {}", option.to);
        println!("  Command: {}", option.command);
        print_transition_detail("Validators", &option.validator_results);
        print_text_list("Blockers", &option.blockers);
        print_text_list(
            "Planned Actions",
            &planned_action_lines(&option.planned_actions),
        );
        print_text_list("Description", &option.descriptions);
    }
}

pub fn evaluate(
    db: &Database,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validators: Vec<String>,
) -> Result<Vec<ValidatorResult>> {
    ensure_target_exists(db, target_kind, target_id)?;
    let validators = if validators.is_empty() {
        default_validators(target_kind, transition)
    } else {
        validators
    };
    let mut results = Vec::new();
    for validator in validators {
        let started = Instant::now();
        let (passed, reason) = evaluate_builtin_with_params(
            db,
            &atelier_app::workflow_policy::load(&repo_root()?)?,
            target_kind,
            target_id,
            transition,
            &validator,
            None,
        )?;
        let elapsed_ms = started.elapsed().as_millis();
        results.push(ValidatorResult {
            target_kind: target_kind.to_string(),
            target_id: target_id.to_string(),
            transition: transition.to_string(),
            validator,
            passed,
            reason,
            elapsed_ms,
        });
    }

    Ok(results)
}

fn print_transition_attempt(
    issue: &Issue,
    transition_name: &str,
    destination: &str,
    validator_results: &[ValidatorResult],
    blockers: &[String],
    planned_actions: &[PlannedAction],
    descriptions: &[String],
    command: &str,
) {
    println!("Issue Transition {} - {}", issue.id, issue.title);
    println!("{}", "=".repeat(issue.id.len() + issue.title.len() + 20));
    println!("Transition: {}", transition_name);
    println!("From:       {}", issue.status);
    println!("To:         {}", destination);
    println!("Command:    {}", command);
    print_transition_detail("Validators", validator_results);
    print_text_list("Blockers", blockers);
    print_text_list("Planned Actions", &planned_action_lines(planned_actions));
    print_text_list("Description", descriptions);
}

fn planned_action_lines(planned_actions: &[PlannedAction]) -> Vec<String> {
    planned_actions
        .iter()
        .map(|action| {
            let mut line = format!(
                "{}. {} target={} owner={}",
                action.order, action.name, action.target_issue_id, action.branch_owner_id
            );
            if let Some(review_target) = &action.review_artifact_target {
                line.push_str(&format!(" review_target={review_target}"));
            }
            if let Some(provider) = &action.review_artifact_provider {
                line.push_str(&format!(" provider={provider}"));
            }
            if let Some(role) = &action.review_artifact_role {
                line.push_str(&format!(" role={role}"));
            }
            if action.confirmation_required {
                line.push_str(" confirmation=required");
            }
            if let Some(skip_reason) = &action.skip_reason {
                line.push_str(&format!(" skip={skip_reason}"));
            }
            if let Some(block_reason) = &action.block_reason {
                line.push_str(&format!(" block={block_reason}"));
            }
            line
        })
        .collect()
}

fn print_transition_detail(title: &str, results: &[ValidatorResult]) {
    print_heading(title);
    if results.is_empty() {
        println!("(none)");
        return;
    }
    for result in results {
        println!(
            "  {}  {}",
            if result.passed { "pass" } else { "fail" },
            result.validator
        );
        println!("      {}", result.reason);
    }
}

fn print_text_list(title: &str, values: &[String]) {
    print_heading(title);
    if values.is_empty() {
        println!("(none)");
        return;
    }
    for value in values {
        println!("  {value}");
    }
}

pub fn default_validators(target_kind: &str, transition: &str) -> Vec<String> {
    let names: &[&str] = match (target_kind, transition) {
        ("issue", "start") => &[
            "durable_state_current",
            "issue_sections_parseable",
            "no_open_blockers",
        ],
        ("issue", "close") => &[
            "durable_state_current",
            "issue_sections_parseable",
            "no_open_blockers",
            "evidence_attached",
        ],
        ("mission", "close") => mission_terminal_validators(),
        ("mission", _) => &[
            "durable_state_current",
            "issue_sections_parseable",
            "no_open_blockers",
        ],
        ("evidence", _) => &["durable_state_current"],
        ("tracker", "health") => &[
            "durable_state_current",
            "no_blocking_lints",
            "command_surface_current",
            "ignored_tests_reviewed",
            "git_worktree_clean",
        ],
        _ => &["durable_state_current"],
    };
    names.iter().map(|name| (*name).to_string()).collect()
}

pub(crate) fn mission_terminal_validators() -> &'static [&'static str] {
    &[
        "durable_state_current",
        "issue_sections_parseable",
        "no_open_work",
        "no_open_blockers",
        "validation_criteria_satisfied",
        "no_blocking_lints",
        "command_surface_current",
        "ignored_tests_reviewed",
        "git_worktree_clean",
    ]
}

fn print_heading(title: &str) {
    println!("{title}");
    println!("{}", "-".repeat(title.len()));
}

fn ensure_transitionable_status(
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    issue: &Issue,
) -> Result<()> {
    if policy.workflow_allows_status(&issue.issue_type, &issue.status)? {
        return Ok(());
    }
    bail!(
        "Issue {} has status '{}' that is not allowed by the workflow policy for issue_type '{}'",
        issue.id,
        issue.status,
        issue.issue_type
    )
}

fn required_field_failures(
    _record: &CanonicalIssueRecord,
    transition: &atelier_app::workflow_policy::TransitionDefinition,
    close_reason: Option<&str>,
) -> Result<Vec<String>> {
    let mut failures = Vec::new();
    for field in &transition.required_fields {
        match field.as_str() {
            "close_reason" => {
                if close_reason.is_none_or(|value| value.trim().is_empty()) {
                    failures.push(
                        "missing required field close_reason; rerun with `--reason \"...\"`"
                            .to_string(),
                    );
                }
            }
            other => {
                return Err(anyhow!("unsupported required field '{other}'"));
            }
        }
    }
    Ok(failures)
}

fn transition_command(
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

fn evaluate_policy_transition(
    db: &Database,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validators: &[atelier_app::workflow_policy::ValidatorDefinition],
) -> Result<Vec<ValidatorResult>> {
    ensure_target_exists(db, target_kind, target_id)?;
    let mut results = Vec::new();
    for definition in validators {
        let started = Instant::now();
        let (passed, reason) = evaluate_builtin_with_params(
            db,
            policy,
            target_kind,
            target_id,
            transition,
            &definition.builtin,
            definition.params.as_ref(),
        )?;
        results.push(ValidatorResult {
            target_kind: target_kind.to_string(),
            target_id: target_id.to_string(),
            transition: transition.to_string(),
            validator: definition.builtin.clone(),
            passed,
            reason,
            elapsed_ms: started.elapsed().as_millis(),
        });
    }
    Ok(results)
}

fn transition_descriptions(
    transition: &atelier_app::workflow_policy::TransitionDefinition,
) -> Vec<String> {
    transition
        .description
        .iter()
        .map(|description| description.trim().to_string())
        .filter(|description| !description.is_empty())
        .collect()
}

fn ensure_target_exists(db: &Database, kind: &str, id: &str) -> Result<()> {
    match kind {
        "issue" => {
            db.require_issue(id)?;
        }
        "tracker" => {
            if id != "health" {
                bail!("unsupported tracker validation target {id}; expected health");
            }
        }
        _ => {
            db.require_record(kind, id)?;
        }
    }
    Ok(())
}

fn evaluate_builtin_with_params(
    db: &Database,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validator: &str,
    params: Option<&atelier_app::workflow_policy::ValidatorParams>,
) -> Result<(bool, String)> {
    match validator {
        "durable_state_current" => {
            let state_dir =
                atelier_app::storage_layout::StorageLayout::new(repo_root()?).canonical_dir();
            let stale = atelier_app::export::canonical_stale_entries(db, &state_dir)?;
            if stale.is_empty() {
                Ok((true, "canonical export is current".to_string()))
            } else {
                Ok((
                    false,
                    format!("canonical export is stale: {}", stale.join("; ")),
                ))
            }
        }
        "evidence_attached" => {
            if target_kind == "issue" {
                let issue = db.require_issue(target_id)?;
                let state_dir =
                    atelier_app::storage_layout::StorageLayout::new(repo_root()?).canonical_dir();
                let record = app_use_cases::load_canonical_issue(&state_dir, target_id)?;
                let gate = issue_evidence_gate_status(db, &issue, Some(&record.sections))?;
                if let Some(atelier_app::workflow_policy::ValidatorParams::EvidenceAttached {
                    min_count,
                    kind,
                }) = params
                {
                    let linked = linked_evidence_records(db, target_id, kind.as_deref())?;
                    let validating_count = linked.len();
                    if validating_count < *min_count as usize {
                        return Ok((
                            false,
                            format!(
                                "expected at least {} validating evidence record(s){}; found {}",
                                min_count,
                                kind.as_deref()
                                    .map(|value| format!(" of kind {}", value))
                                    .unwrap_or_default(),
                                validating_count
                            ),
                        ));
                    }
                }
                return Ok((gate.passed, gate.reason));
            }
            let attached = db
                .list_record_links(target_kind, target_id)?
                .into_iter()
                .any(|link| {
                    link.relation_type == "validates"
                        && (link.source_kind == "evidence" || link.target_kind == "evidence")
                });
            if attached {
                Ok((true, "validating evidence is linked".to_string()))
            } else {
                Ok((false, "no validating evidence link found".to_string()))
            }
        }
        "no_open_blockers" => {
            let open = open_blockers(db, policy, target_kind, target_id)?;
            if open.is_empty() {
                Ok((true, "no open blockers".to_string()))
            } else {
                Ok((false, format!("open blockers: {}", open.join(", "))))
            }
        }
        "no_open_work" => {
            let open = open_work(db, policy, target_kind, target_id)?;
            if open.is_empty() {
                Ok((true, "no open linked work".to_string()))
            } else {
                Ok((false, format!("open linked work: {}", open.join(", "))))
            }
        }
        "git_worktree_clean" => git_worktree_clean(),
        "no_blocking_lints" => {
            let status = Command::new(std::env::current_exe()?)
                .arg("lint")
                .status()?;
            if status.success() {
                Ok((true, "lint passed".to_string()))
            } else {
                Ok((false, "atelier lint failed".to_string()))
            }
        }
        "ignored_tests_reviewed" => ignored_tests_reviewed(),
        "command_surface_current" => command_surface_current(),
        "issue_sections_parseable" => issue_sections_parseable(db, target_kind, target_id),
        "validation_criteria_satisfied" => {
            validation_criteria_satisfied(db, target_kind, target_id)
        }
        "linked_pr_merged" => linked_pr_merged(db, target_kind, target_id),
        "review_complete" => review_complete(db, policy, target_kind, target_id, transition),
        "epic_child_proof_complete" => {
            epic_child_proof_complete(db, policy, target_kind, target_id)
        }
        other => Ok((false, format!("unsupported builtin validator: {other}"))),
    }
}

fn linked_pr_merged(db: &Database, target_kind: &str, target_id: &str) -> Result<(bool, String)> {
    if target_kind != "issue" {
        return Ok((
            true,
            format!("linked PR merge state does not apply to {target_kind} records"),
        ));
    }

    let field = atelier_app::workflow_policy::effective_pull_request_field(db, target_id)?;
    if field.is_none() {
        return Ok((
            false,
            format!("no linked review field; run `atelier review open --issue {target_id}`"),
        ));
    }
    let repo_root = repo_root()?;
    let config_path = repo_root.join(".atelier/config.toml");
    let forgejo = match ProjectConfig::load(&repo_root)
        .and_then(|config| config.require_forgejo(&config_path).cloned())
    {
        Ok(forgejo) => forgejo,
        Err(error) => {
            return Ok((
                false,
                format!(
                    "{}; configure Forgejo, then run `atelier review open --issue {}` or `atelier review status --issue {}`",
                    error,
                    target_id,
                    target_id
                ),
            ));
        }
    };
    let token = match std::env::var(&forgejo.admin_token_env) {
        Ok(token) => token,
        Err(_) => {
            return Ok((
                false,
                format!(
                    "forgejo_config_missing_token: environment variable {} is required for review validators; run `atelier review status --issue {}` after configuring it",
                    forgejo.admin_token_env,
                    target_id
                ),
            ));
        }
    };
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    linked_pr_merged_with_client(db, &repo_root, field, &client, target_id)
}

fn linked_pr_merged_with_client<T: ForgejoTransport>(
    db: &Database,
    repo_root: &Path,
    field: Option<Value>,
    client: &ForgejoClient<T>,
    issue_id: &str,
) -> Result<(bool, String)> {
    let Some(field) = field else {
        return Ok((
            false,
            format!("no linked review field; run `atelier review open --issue {issue_id}`"),
        ));
    };
    let number = field
        .as_object()
        .filter(|object| {
            object.get("kind").and_then(Value::as_str) == Some("pull_request")
                && object.get("provider").and_then(Value::as_str) == Some("forgejo")
        })
        .and_then(|object| object.get("number"))
        .and_then(Value::as_u64)
        .filter(|number| *number > 0)
        .ok_or_else(|| {
            anyhow!("pull_request_invalid: field review must be a provider pull_request object")
        })?;
    let pull = client.show_pull(number)?;
    let policy = atelier_app::workflow_policy::load(repo_root)?;
    let resolution = atelier_app::workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?;
    if pull.source_branch != resolution.expected_branch
        || pull.target_branch != resolution.base_branch
    {
        return Ok((
            false,
            format!(
                "linked PR branches are {} -> {}, but issue {} expects {} -> {}; run `atelier review status --issue {}`",
                pull.source_branch,
                pull.target_branch,
                resolution.owner_id,
                resolution.expected_branch,
                resolution.base_branch,
                issue_id
            ),
        ));
    }
    if pull.merged {
        Ok((true, format!("linked PR {} is merged", pull.number)))
    } else {
        Ok((
            false,
            format!(
                "linked PR {} is {} and not merged; run `atelier review status --issue {}`",
                pull.number, pull.state, issue_id
            ),
        ))
    }
}

fn epic_child_proof_complete(
    db: &Database,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    if target_kind != "issue" {
        return Ok((
            true,
            format!("epic child proof does not apply to {target_kind} records"),
        ));
    }
    let issue = db.require_issue(target_id)?;
    if issue.issue_type != "epic" {
        return Ok((
            true,
            "epic child proof does not apply to non-epic issues".to_string(),
        ));
    }
    let mut missing = Vec::new();
    for child in db.get_subissues(target_id)? {
        collect_missing_child_proof(db, policy, &child.id, &mut missing)?;
    }
    if missing.is_empty() {
        Ok((
            true,
            "all epic child issues are closed with validating proof".to_string(),
        ))
    } else {
        Ok((
            false,
            format!("epic child proof incomplete: {}", missing.join(", ")),
        ))
    }
}

fn collect_missing_child_proof(
    db: &Database,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    issue_id: &str,
    missing: &mut Vec<String>,
) -> Result<()> {
    let issue = db.require_issue(issue_id)?;
    if issue_is_open_for_workflow(policy, &issue)? {
        missing.push(format!("{issue_id} open"));
    } else if linked_evidence_records(db, issue_id, None)?.is_empty() {
        missing.push(format!("{issue_id} missing validating proof"));
    }
    for child in db.get_subissues(issue_id)? {
        collect_missing_child_proof(db, policy, &child.id, missing)?;
    }
    Ok(())
}

fn validation_criteria_satisfied(
    db: &Database,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    if target_kind == "mission" {
        return crate::commands::mission::mission_validation_criteria_gate(db, target_id);
    }
    Ok((
        true,
        format!("validation criteria closeout does not apply to {target_kind} records"),
    ))
}

fn issue_sections_parseable(
    db: &Database,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    let issue_ids = match target_kind {
        "issue" => {
            let mut ids = BTreeSet::new();
            ids.insert(target_id.to_string());
            ids
        }
        "mission" => mission_issue_ids(db, target_id)?,
        _ => {
            return Ok((
                true,
                format!("issue sections do not apply to {target_kind} records"),
            ))
        }
    };
    if issue_ids.is_empty() {
        return Ok((true, "no linked issues require section checks".to_string()));
    }

    let state_dir = atelier_app::storage_layout::StorageLayout::new(repo_root()?).canonical_dir();
    let mut checked = 0;
    for issue_id in issue_ids {
        let record = match app_use_cases::load_canonical_issue(&state_dir, &issue_id) {
            Ok(record) => record,
            Err(error) => return Ok((false, error.to_string())),
        };
        let invalid = record
            .sections
            .section_states()
            .into_iter()
            .filter(|state| state.required && (!state.present || state.empty))
            .map(|state| state.name.title().to_string())
            .collect::<Vec<_>>();
        if !invalid.is_empty() {
            let path = state_dir.join("issues").join(format!("{issue_id}.md"));
            return Ok((
                false,
                format!(
                    "issue {issue_id} has invalid sections {} in {}",
                    invalid.join(", "),
                    path.display()
                ),
            ));
        }
        checked += 1;
    }

    Ok((
        true,
        format!(
            "parsed required sections {} are present and non-empty for {checked} issue(s)",
            IssueSections::REQUIRED_NAMES
                .into_iter()
                .map(|name| name.title())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    ))
}

fn linked_evidence_records(
    db: &Database,
    issue_id: &str,
    required_kind: Option<&str>,
) -> Result<Vec<EvidenceRecord>> {
    let mut records = Vec::new();
    for link in db.list_record_links("issue", issue_id)? {
        if link.relation_type != "validates" {
            continue;
        }
        let evidence_id = if link.source_kind == "evidence" {
            Some(link.source_id)
        } else if link.target_kind == "evidence" {
            Some(link.target_id)
        } else {
            None
        };
        let Some(evidence_id) = evidence_id else {
            continue;
        };
        db.require_record("evidence", &evidence_id)?;
        let Some(record) = canonical_evidence_record(&evidence_id)? else {
            continue;
        };
        if let Some(required_kind) = required_kind {
            if record.data.evidence_type != required_kind {
                continue;
            }
        }
        records.push(record);
    }
    Ok(records)
}

fn canonical_evidence_record(id: &str) -> Result<Option<EvidenceRecord>> {
    let Some(state_dir) = atelier_app::storage_layout::find_canonical_dir_from_cwd()? else {
        return Ok(None);
    };
    Ok(
        match app_use_cases::load_canonical_record(&state_dir, "evidence", id) {
            Ok(Record::Evidence(record)) => Some(record),
            Ok(_) | Err(_) => None,
        },
    )
}

fn review_complete(
    db: &Database,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
    _transition: &str,
) -> Result<(bool, String)> {
    if target_kind != "issue" {
        return Ok((
            true,
            format!("review completion does not apply to {target_kind}"),
        ));
    }
    let issue = db.require_issue(target_id)?;
    match policy.status_category(&issue.status) {
        Some("review") | Some("validation") | Some("done") => Ok((
            true,
            format!(
                "issue {} has completed review state {}",
                issue.id, issue.status
            ),
        )),
        _ => Ok((
            false,
            format!(
                "issue {} must reach a review status before this transition; current status is {}",
                issue.id, issue.status
            ),
        )),
    }
}

fn issue_is_open_for_workflow(
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    issue: &Issue,
) -> Result<bool> {
    ensure_transitionable_status(policy, issue)?;
    Ok(policy.status_category(&issue.status) != Some("done"))
}

fn open_blockers(
    db: &Database,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
) -> Result<Vec<String>> {
    let mut blocker_ids = BTreeSet::new();
    match target_kind {
        "issue" => {
            for blocker in db.get_blockers(target_id)? {
                blocker_ids.insert(blocker);
            }
        }
        "mission" => {
            for blocker in mission_direct_blockers(db, target_id)? {
                blocker_ids.insert(blocker);
            }
            for issue_id in mission_issue_ids(db, target_id)? {
                for blocker in db.get_blockers(&issue_id)? {
                    blocker_ids.insert(blocker);
                }
            }
        }
        _ => return Ok(Vec::new()),
    }
    let mut open = blocker_ids
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter_map(|issue| match issue_is_open_for_workflow(policy, &issue) {
            Ok(true) => Some(Ok(issue.id)),
            Ok(false) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>>>()?;
    open.sort();
    Ok(open)
}

fn open_work(
    db: &Database,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
) -> Result<Vec<String>> {
    if target_kind != "mission" {
        return Ok(Vec::new());
    }
    let mut open = mission_issue_ids(db, target_id)?
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter_map(|issue| match issue_is_open_for_workflow(policy, &issue) {
            Ok(true) => Some(Ok(issue.id)),
            Ok(false) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>>>()?;
    open.sort();
    Ok(open)
}

fn mission_direct_blockers(db: &Database, mission_id: &str) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for link in db.list_record_links("mission", mission_id)? {
        if link.relation_type != "blocked_by" {
            continue;
        }
        if link.source_kind == "issue"
            && link.target_kind == "mission"
            && link.target_id == mission_id
        {
            blockers.push(link.source_id);
        } else if link.target_kind == "issue"
            && link.source_kind == "mission"
            && link.source_id == mission_id
        {
            blockers.push(link.target_id);
        }
    }
    Ok(blockers)
}

fn mission_issue_ids(db: &Database, mission_id: &str) -> Result<BTreeSet<String>> {
    let mut issue_ids = BTreeSet::new();
    for link in db.list_record_links("mission", mission_id)? {
        if link.relation_type != "advances" {
            continue;
        }
        let linked_id = if link.source_kind == "issue"
            && link.target_kind == "mission"
            && link.target_id == mission_id
        {
            Some(link.source_id)
        } else if link.target_kind == "issue"
            && link.source_kind == "mission"
            && link.source_id == mission_id
        {
            Some(link.target_id)
        } else {
            None
        };
        if let Some(linked_id) = linked_id {
            collect_issue_and_descendants(db, &linked_id, &mut issue_ids)?;
        }
    }
    Ok(issue_ids)
}

fn collect_issue_and_descendants(
    db: &Database,
    issue_id: &str,
    issue_ids: &mut BTreeSet<String>,
) -> Result<()> {
    if !issue_ids.insert(issue_id.to_string()) {
        return Ok(());
    }
    for child in db.get_subissues(issue_id)? {
        collect_issue_and_descendants(db, &child.id, issue_ids)?;
    }
    Ok(())
}

fn git_worktree_clean() -> Result<(bool, String)> {
    let root = repo_root()?;
    let output = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=all"])
        .current_dir(&root)
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.contains("not a git repository") {
            return Ok((
                true,
                "not a git repository; git worktree check skipped".to_string(),
            ));
        }
        let message = if stderr.is_empty() {
            "git status failed".to_string()
        } else {
            format!("git status failed: {stderr}")
        };
        return Ok((false, message));
    }
    let dirty = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(parse_git_dirty_entry)
        .collect::<Vec<_>>();
    if dirty.is_empty() {
        Ok((true, "git worktree is clean".to_string()))
    } else {
        let classified = classify_git_dirty_entries(&root, &dirty)?;
        if classified.blocking_entries.is_empty() {
            if classified.tracker_generated_entries.is_empty() {
                return Ok((true, "git worktree is clean".to_string()));
            }
            return Ok((
                true,
                format!(
                    "ignored {} tracker-generated canonical {}: {}",
                    classified.tracker_generated_entries.len(),
                    if classified.tracker_generated_entries.len() == 1 {
                        "entry"
                    } else {
                        "entries"
                    },
                    summarize_git_dirty_entries(&classified.tracker_generated_entries)
                ),
            ));
        }
        let sample = summarize_git_dirty_entries(&classified.blocking_entries);
        let suffix = if classified.blocking_entries.len() > 8 {
            format!("; ... and {} more", classified.blocking_entries.len() - 8)
        } else {
            String::new()
        };
        Ok((
            false,
            format!(
                "git worktree has {} dirty {}: {sample}{suffix}",
                classified.blocking_entries.len(),
                if classified.blocking_entries.len() == 1 {
                    "entry"
                } else {
                    "entries"
                }
            ),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GitDirtyEntry {
    raw: String,
    repo_path: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ClassifiedGitDirtyEntries {
    blocking_entries: Vec<String>,
    tracker_generated_entries: Vec<String>,
}

fn parse_git_dirty_entry(line: &str) -> Option<GitDirtyEntry> {
    let raw = line.trim_end();
    if raw.trim().is_empty() || raw.len() < 4 {
        return None;
    }
    let repo_path = raw
        .get(3..)
        .map(str::trim)
        .filter(|value| !value.is_empty())?;
    let repo_path = repo_path
        .rsplit_once(" -> ")
        .map(|(_, target)| target)
        .unwrap_or(repo_path)
        .to_string();
    Some(GitDirtyEntry {
        raw: raw.to_string(),
        repo_path,
    })
}

fn summarize_git_dirty_entries(entries: &[String]) -> String {
    entries
        .iter()
        .take(8)
        .cloned()
        .collect::<Vec<_>>()
        .join("; ")
}

fn classify_git_dirty_entries(
    repo_root: &Path,
    entries: &[GitDirtyEntry],
) -> Result<ClassifiedGitDirtyEntries> {
    let tracker_activity_issue_ids = entries
        .iter()
        .filter_map(|entry| atelier_relative_path(&entry.repo_path))
        .filter(|relative| is_tracker_generated_activity_path(relative))
        .filter_map(issue_id_from_activity_path)
        .collect::<BTreeSet<_>>();

    let mut blocking_entries = Vec::new();
    let mut tracker_generated_entries = Vec::new();
    for entry in entries {
        let Some(relative) = atelier_relative_path(&entry.repo_path) else {
            blocking_entries.push(entry.raw.clone());
            continue;
        };
        if atelier_app::storage_layout::is_local_atelier_path(relative) {
            continue;
        }
        if is_tracker_generated_evidence_path(relative) {
            tracker_generated_entries.push(entry.raw.clone());
            continue;
        }
        if is_tracker_generated_activity_path(relative) {
            tracker_generated_entries.push(entry.raw.clone());
            continue;
        }
        if is_tracker_generated_issue_bookkeeping(
            repo_root,
            relative,
            &entry.repo_path,
            &tracker_activity_issue_ids,
        )? {
            tracker_generated_entries.push(entry.raw.clone());
            continue;
        }
        blocking_entries.push(entry.raw.clone());
    }
    Ok(ClassifiedGitDirtyEntries {
        blocking_entries,
        tracker_generated_entries,
    })
}

fn atelier_relative_path(repo_path: &str) -> Option<&Path> {
    repo_path
        .strip_prefix(".atelier/")
        .map(|relative| Path::new(relative))
}

fn is_tracker_generated_evidence_path(relative: &Path) -> bool {
    let mut components = relative.components();
    let Some(std::path::Component::Normal(root)) = components.next() else {
        return false;
    };
    if root != "evidence" {
        return false;
    }
    let Some(std::path::Component::Normal(file)) = components.next() else {
        return false;
    };
    components.next().is_none() && file.to_string_lossy().ends_with(".md")
}

fn is_tracker_generated_activity_path(relative: &Path) -> bool {
    let mut components = relative.components();
    let Some(std::path::Component::Normal(root)) = components.next() else {
        return false;
    };
    if root != "issues" && root != "missions" {
        return false;
    }
    let Some(std::path::Component::Normal(dir)) = components.next() else {
        return false;
    };
    if !dir.to_string_lossy().ends_with(".activity") {
        return false;
    }
    let Some(std::path::Component::Normal(file)) = components.next() else {
        return false;
    };
    components.next().is_none() && file.to_string_lossy().ends_with(".md")
}

fn issue_id_from_activity_path(relative: &Path) -> Option<String> {
    let mut components = relative.components();
    let root = components.next()?.as_os_str();
    if root != "issues" {
        return None;
    }
    let dir = components.next()?.as_os_str().to_string_lossy();
    dir.strip_suffix(".activity").map(ToOwned::to_owned)
}

fn is_tracker_generated_issue_bookkeeping(
    repo_root: &Path,
    relative: &Path,
    repo_path: &str,
    tracker_activity_issue_ids: &BTreeSet<String>,
) -> Result<bool> {
    let Some(issue_id) = issue_id_from_canonical_issue_path(relative) else {
        return Ok(false);
    };
    if !tracker_activity_issue_ids.contains(&issue_id) {
        return Ok(false);
    }
    let current_text = fs::read_to_string(repo_root.join(repo_path))?;
    let Some(front_matter_end_line) = front_matter_end_line(&current_text) else {
        return Ok(false);
    };
    let diff = git_diff_against_head(repo_root, repo_path)?;
    if diff.trim().is_empty() {
        return Ok(false);
    }
    let mut saw_allowed_change = false;
    let mut current_line = None;
    for line in diff.lines() {
        if line.starts_with("diff --git")
            || line.starts_with("index ")
            || line.starts_with("--- ")
            || line.starts_with("+++ ")
        {
            continue;
        }
        if line.starts_with("@@ ") {
            current_line = parse_new_hunk_start(line);
            continue;
        }
        let Some(line_no) = current_line.as_mut() else {
            continue;
        };
        match line.chars().next() {
            Some('+') => {
                saw_allowed_change = true;
                if *line_no > front_matter_end_line
                    || !is_allowed_issue_bookkeeping_line(&line[1..])
                {
                    return Ok(false);
                }
                *line_no += 1;
            }
            Some('-') => {
                saw_allowed_change = true;
                if *line_no > front_matter_end_line
                    || !is_allowed_issue_bookkeeping_line(&line[1..])
                {
                    return Ok(false);
                }
            }
            Some(' ') => *line_no += 1,
            _ => {}
        }
    }
    Ok(saw_allowed_change)
}

fn issue_id_from_canonical_issue_path(relative: &Path) -> Option<String> {
    let mut components = relative.components();
    let root = components.next()?.as_os_str();
    if root != "issues" {
        return None;
    }
    let file = components.next()?.as_os_str().to_string_lossy();
    if components.next().is_some() || !file.ends_with(".md") || file.ends_with(".activity") {
        return None;
    }
    file.strip_suffix(".md").map(ToOwned::to_owned)
}

fn front_matter_end_line(text: &str) -> Option<usize> {
    let mut fence_count = 0;
    for (index, line) in text.lines().enumerate() {
        if line == "---" {
            fence_count += 1;
            if fence_count == 2 {
                return Some(index + 1);
            }
        }
    }
    None
}

fn parse_new_hunk_start(line: &str) -> Option<usize> {
    let (_, rest) = line.split_once('+')?;
    let digits = rest
        .chars()
        .take_while(|char| char.is_ascii_digit())
        .collect::<String>();
    digits.parse().ok()
}

fn is_allowed_issue_bookkeeping_line(line: &str) -> bool {
    line.starts_with("status: ")
        || line.starts_with("updated_at: ")
        || line.starts_with("closed_at: ")
}

fn git_diff_against_head(repo_root: &Path, repo_path: &str) -> Result<String> {
    let output = Command::new("git")
        .args([
            "diff",
            "--no-ext-diff",
            "--unified=0",
            "HEAD",
            "--",
            repo_path,
        ])
        .current_dir(repo_root)
        .output()?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).to_string());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    bail!("git diff HEAD -- {repo_path} failed: {}", stderr.trim())
}

fn ignored_tests_reviewed() -> Result<(bool, String)> {
    let inventory = crate::test_inventory::IgnoredTestInventory::scan_repo(&repo_root()?)?;
    Ok(inventory.status_reason())
}

fn command_surface_current() -> Result<(bool, String)> {
    crate::command_surface::status_reason(&repo_root()?)
}

fn repo_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;
    if output.status.success() {
        Ok(Path::new(String::from_utf8_lossy(&output.stdout).trim()).to_path_buf())
    } else {
        Ok(std::env::current_dir()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use atelier_app::forgejo::{ForgejoRequest, ForgejoResponse};
    use atelier_app::project_config::{ForgejoConfig, ForgejoRoleAuthors};
    use atelier_app::workflow_policy::ReviewArtifactActionParams;
    use atelier_records::{RecordStore, Relationships};
    use chrono::Utc;
    use serde_json::json;
    use std::collections::BTreeMap;
    use tempfile::{tempdir, TempDir};

    struct FakeForgejoTransport {
        state: &'static str,
        merged: bool,
        source_branch: &'static str,
        target_branch: &'static str,
    }

    impl FakeForgejoTransport {
        fn merged() -> Self {
            Self {
                state: "closed",
                merged: true,
                source_branch: "epic/atelier-hw9t",
                target_branch: "master",
            }
        }
    }

    impl ForgejoTransport for FakeForgejoTransport {
        fn send(&self, request: ForgejoRequest) -> Result<ForgejoResponse> {
            assert_eq!(request.method, "GET");
            assert_eq!(request.path, "/api/v1/repos/tools/atelier/pulls/42");
            Ok(ForgejoResponse {
                status: 200,
                body: format!(
                    r#"{{
                        "number": 42,
                        "url": "https://forge.example.test/tools/atelier/pulls/42",
                        "state": "{}",
                        "merged": {},
                        "head": {{ "ref": "{}" }},
                        "base": {{ "ref": "{}" }}
                    }}"#,
                    self.state, self.merged, self.source_branch, self.target_branch
                ),
            })
        }
    }

    fn forgejo_config() -> ForgejoConfig {
        ForgejoConfig {
            host: "https://forge.example.test".to_string(),
            owner: "tools".to_string(),
            repo: "atelier".to_string(),
            admin_token_env: "FORGEJO_ADMIN_TOKEN".to_string(),
            role_authors: Some(ForgejoRoleAuthors {
                worker: "forge-worker".to_string(),
                reviewer: "forge-reviewer".to_string(),
                validator: "forge-validator".to_string(),
                manager: "forge-manager".to_string(),
            }),
        }
    }

    fn pull_request_field() -> Value {
        json!({"kind": "pull_request", "provider": "forgejo", "number": 42})
    }

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
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
        }
    }

    fn action(name: &str) -> atelier_app::workflow_policy::ActionDefinition {
        atelier_app::workflow_policy::ActionDefinition {
            builtin: name.to_string(),
            params: None,
        }
    }

    fn review_action() -> atelier_app::workflow_policy::ActionDefinition {
        atelier_app::workflow_policy::ActionDefinition {
            builtin: "review.open".to_string(),
            params: Some(ActionParams::ReviewArtifact(ReviewArtifactActionParams {
                provider: None,
                role: "worker".to_string(),
                role_authors: None,
            })),
        }
    }

    fn forgejo_review_action() -> atelier_app::workflow_policy::ActionDefinition {
        atelier_app::workflow_policy::ActionDefinition {
            builtin: "review.open".to_string(),
            params: Some(ActionParams::ReviewArtifact(ReviewArtifactActionParams {
                provider: Some("forgejo".to_string()),
                role: "worker".to_string(),
                role_authors: Some(WorkflowForgejoRoleAuthors {
                    worker: "forge-worker".to_string(),
                    reviewer: "forge-reviewer".to_string(),
                    validator: "forge-validator".to_string(),
                    manager: "forge-manager".to_string(),
                }),
            })),
        }
    }

    fn write_room_config_and_workflow(dir: &TempDir) {
        std::fs::create_dir_all(dir.path().join(".atelier/runtime")).unwrap();
        std::fs::write(
            dir.path().join(".atelier/config.toml"),
            r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier-test"

[paths]
state_root = ".atelier"
runtime_dir = ".atelier/runtime"
runtime_database = ".atelier/runtime/state.db"
cache_dir = ".atelier/cache"

[review]
mode = "room"
"#,
        )
        .unwrap();
        std::fs::write(
            dir.path().join(".atelier/workflow.yaml"),
            atelier_app::workflow_policy::STARTER_POLICY_YAML
                .replace("base_branch: main", "base_branch: master"),
        )
        .unwrap();
    }

    fn write_provider_config_without_role_authors(dir: &TempDir) {
        std::fs::create_dir_all(dir.path().join(".atelier/runtime")).unwrap();
        std::fs::write(
            dir.path().join(".atelier/config.toml"),
            r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier-test"

[paths]
state_root = ".atelier"
runtime_dir = ".atelier/runtime"
runtime_database = ".atelier/runtime/state.db"
cache_dir = ".atelier/cache"

[review]
mode = "provider"
provider = "forgejo"

[review.providers.forgejo]
host = "https://forge.example.test"
owner = "tools"
repo = "atelier"
admin_token_env = "ATELIER_TEST_FORGEJO_TOKEN"
"#,
        )
        .unwrap();
    }

    fn insert_canonical_issue(db: &Database, state_dir: &Path, issue: Issue) {
        db.insert_issue_rebuild(&issue).unwrap();
        let record = CanonicalIssueRecord {
            issue,
            labels: Vec::new(),
            sections: IssueSections::unchecked_from_body(Some(
                "## Description\n\nbody\n\n## Outcome\n\nworks\n\n## Evidence\n\nproof",
            )),
            relationships: Relationships::default(),
        };
        RecordStore::new(state_dir)
            .write_issue_atomic(&record)
            .unwrap();
    }

    fn setup_pr_validator_repo() -> (TempDir, Database) {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".atelier/runtime")).unwrap();
        std::fs::write(
            dir.path().join(".atelier/workflow.yaml"),
            atelier_app::workflow_policy::STARTER_POLICY_YAML
                .replace("base_branch: main", "base_branch: master"),
        )
        .unwrap();
        let db = Database::open(&dir.path().join(".atelier/runtime/state.db")).unwrap();
        let now = Utc::now();
        db.insert_issue_rebuild(&Issue {
            id: "atelier-hw9t".to_string(),
            title: "Epic".to_string(),
            description: None,
            status: "validation".to_string(),
            issue_type: "epic".to_string(),
            priority: "medium".to_string(),
            fields: BTreeMap::new(),
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
        })
        .unwrap();
        db.insert_issue_rebuild(&Issue {
            id: "atelier-val1".to_string(),
            title: "Validation".to_string(),
            description: None,
            status: "validation".to_string(),
            issue_type: "validation".to_string(),
            priority: "medium".to_string(),
            fields: BTreeMap::new(),
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
        })
        .unwrap();
        (dir, db)
    }

    #[test]
    fn default_validators_are_target_and_transition_aware() {
        assert_eq!(
            default_validators("issue", "start"),
            vec![
                "durable_state_current",
                "issue_sections_parseable",
                "no_open_blockers"
            ]
        );
        assert_eq!(
            default_validators("issue", "close"),
            vec![
                "durable_state_current",
                "issue_sections_parseable",
                "no_open_blockers",
                "evidence_attached"
            ]
        );
        assert_eq!(
            default_validators("mission", "close"),
            mission_terminal_validators()
                .iter()
                .map(|name| (*name).to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            default_validators("evidence", "attach"),
            vec!["durable_state_current"]
        );
        assert_eq!(
            default_validators("tracker", "health"),
            vec![
                "durable_state_current",
                "no_blocking_lints",
                "command_surface_current",
                "ignored_tests_reviewed",
                "git_worktree_clean"
            ]
        );
    }

    #[test]
    fn transition_action_plan_is_ordered_and_side_effect_free() {
        let issue = test_issue("atelier-epic1");
        let resolution = BranchLifecycleResolution {
            issue_id: "atelier-epic1".to_string(),
            owner_id: "atelier-epic1".to_string(),
            owner_issue_type: "epic".to_string(),
            owner_kind: atelier_app::workflow_policy::BranchOwnerKind::Epic,
            expected_branch: "epic/atelier-epic1".to_string(),
            base_branch: "master".to_string(),
            merge_strategy: MergeStrategy::Squash,
            merge_owned: true,
            nested_under_epic: false,
        };
        let actions = vec![review_action(), action("branch_integrate")];

        let plan = plan_actions_for_resolution(&issue, &resolution, &actions);

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
    fn action_preflight_checks_git_actions_before_execution() {
        let issue = test_issue("atelier-epic1");
        let resolution = BranchLifecycleResolution {
            issue_id: "atelier-epic1".to_string(),
            owner_id: "atelier-epic1".to_string(),
            owner_issue_type: "epic".to_string(),
            owner_kind: atelier_app::workflow_policy::BranchOwnerKind::Epic,
            expected_branch: "epic/atelier-epic1".to_string(),
            base_branch: "master".to_string(),
            merge_strategy: MergeStrategy::Squash,
            merge_owned: true,
            nested_under_epic: false,
        };
        let dir = tempdir().unwrap();
        assert!(action_preflight_blockers(dir.path(), &[]).is_empty());

        let unsupported =
            plan_actions_for_resolution(&issue, &resolution, &[action("branch_integrate")]);
        let blockers = action_preflight_blockers(dir.path(), &unsupported);
        assert_eq!(blockers.len(), 1);
        assert!(blockers[0].contains("branch_integrate"));
        assert!(blockers[0].contains("git repository is required"));
    }

    #[test]
    fn provider_review_action_preflight_uses_workflow_role_authors_and_env_secret() {
        let issue = test_issue("atelier-epic1");
        let resolution = BranchLifecycleResolution {
            issue_id: "atelier-epic1".to_string(),
            owner_id: "atelier-epic1".to_string(),
            owner_issue_type: "epic".to_string(),
            owner_kind: atelier_app::workflow_policy::BranchOwnerKind::Epic,
            expected_branch: "epic/atelier-epic1".to_string(),
            base_branch: "master".to_string(),
            merge_strategy: MergeStrategy::Squash,
            merge_owned: true,
            nested_under_epic: false,
        };
        let dir = tempdir().unwrap();
        write_provider_config_without_role_authors(&dir);

        let actions = plan_actions_for_resolution(&issue, &resolution, &[forgejo_review_action()]);
        let blockers = action_preflight_blockers(dir.path(), &actions);

        assert_eq!(blockers.len(), 1);
        assert!(blockers[0].contains("ATELIER_TEST_FORGEJO_TOKEN"));
        assert!(!blockers[0].contains("role_authors"));
        assert_eq!(
            actions[0].review_artifact_provider.as_deref(),
            Some("forgejo")
        );
        assert_eq!(actions[0].review_artifact_role.as_deref(), Some("worker"));
        assert_eq!(
            actions[0].forgejo_role_authors.as_ref().unwrap().worker,
            "forge-worker"
        );
    }

    #[test]
    fn review_open_action_persists_room_review_field() {
        let dir = tempdir().unwrap();
        write_room_config_and_workflow(&dir);
        let state_dir = dir.path().join(".atelier");
        let db_path = dir.path().join(".atelier/runtime/state.db");
        let db = Database::open(&db_path).unwrap();
        let issue = test_issue("atelier-epic1");
        insert_canonical_issue(&db, &state_dir, issue.clone());
        let resolution = BranchLifecycleResolution {
            issue_id: "atelier-epic1".to_string(),
            owner_id: "atelier-epic1".to_string(),
            owner_issue_type: "epic".to_string(),
            owner_kind: atelier_app::workflow_policy::BranchOwnerKind::Epic,
            expected_branch: "epic/atelier-epic1".to_string(),
            base_branch: "master".to_string(),
            merge_strategy: MergeStrategy::Squash,
            merge_owned: true,
            nested_under_epic: false,
        };
        let action = plan_actions_for_resolution(&issue, &resolution, &[review_action()]).remove(0);

        let detail = open_review_artifact_action(
            &db,
            &state_dir,
            &db_path,
            dir.path(),
            &issue,
            "request_review",
            &action,
        )
        .unwrap();

        assert!(detail.contains("opened room"));
        let owner = app_use_cases::load_canonical_issue(&state_dir, "atelier-epic1").unwrap();
        let review = owner.issue.fields.get("review").unwrap();
        assert_eq!(review["kind"], "room");
        assert!(review["id"].as_str().unwrap().starts_with("atelier-"));

        let second_detail = open_review_artifact_action(
            &db,
            &state_dir,
            &db_path,
            dir.path(),
            &issue,
            "request_review",
            &action,
        )
        .unwrap();
        assert!(second_detail.contains("reused room"));
    }

    #[test]
    fn linked_pr_merged_validator_reports_required_states() {
        let (dir, db) = setup_pr_validator_repo();
        let forgejo = forgejo_config();
        let merged_client = ForgejoClient::new(forgejo.clone(), FakeForgejoTransport::merged());

        let (passed, reason) =
            linked_pr_merged_with_client(&db, dir.path(), None, &merged_client, "atelier-hw9t")
                .unwrap();
        assert!(!passed);
        assert!(reason.contains("atelier review open --issue atelier-hw9t"));

        let open_client = ForgejoClient::new(
            forgejo.clone(),
            FakeForgejoTransport {
                state: "open",
                merged: false,
                source_branch: "epic/atelier-hw9t",
                target_branch: "master",
            },
        );
        let (passed, reason) = linked_pr_merged_with_client(
            &db,
            dir.path(),
            Some(pull_request_field()),
            &open_client,
            "atelier-hw9t",
        )
        .unwrap();
        assert!(!passed);
        assert!(reason.contains("not merged"));
        assert!(reason.contains("atelier review status --issue atelier-hw9t"));

        let closed_client = ForgejoClient::new(
            forgejo.clone(),
            FakeForgejoTransport {
                state: "closed",
                merged: false,
                source_branch: "epic/atelier-hw9t",
                target_branch: "master",
            },
        );
        let (passed, reason) = linked_pr_merged_with_client(
            &db,
            dir.path(),
            Some(pull_request_field()),
            &closed_client,
            "atelier-hw9t",
        )
        .unwrap();
        assert!(!passed);
        assert!(reason.contains("closed and not merged"));

        let (passed, reason) = linked_pr_merged_with_client(
            &db,
            dir.path(),
            Some(pull_request_field()),
            &merged_client,
            "atelier-hw9t",
        )
        .unwrap();
        assert!(passed);
        assert_eq!(reason, "linked PR 42 is merged");
    }

    #[test]
    fn linked_pr_merged_is_not_in_starter_close_policy() {
        let (dir, db) = setup_pr_validator_repo();
        let policy = atelier_app::workflow_policy::load(dir.path()).unwrap();
        let epic_workflow = policy.workflow_by_issue_type.get("epic").unwrap();
        let epic_close = &policy.workflows[epic_workflow].transitions["close"];
        let linked_pr_validators = epic_close
            .validators
            .iter()
            .filter(|validator| validator.builtin == "linked_pr_merged")
            .cloned()
            .collect::<Vec<_>>();
        assert!(linked_pr_validators.is_empty());

        let results = evaluate_policy_transition(
            &db,
            &policy,
            "issue",
            "atelier-hw9t",
            "close",
            &linked_pr_validators,
        )
        .unwrap();
        assert!(results.is_empty());

        let validation_workflow = policy.workflow_by_issue_type.get("validation").unwrap();
        let validation_close = &policy.workflows[validation_workflow].transitions["close"];
        assert!(!validation_close
            .validators
            .iter()
            .any(|validator| validator.builtin == "linked_pr_merged"));
    }

    #[test]
    fn linked_pr_merged_validator_rejects_branch_mismatch() {
        let (dir, db) = setup_pr_validator_repo();
        let forgejo = forgejo_config();
        let wrong_branch_client = ForgejoClient::new(
            forgejo.clone(),
            FakeForgejoTransport {
                state: "closed",
                merged: true,
                source_branch: "epic/other",
                target_branch: "master",
            },
        );
        let (passed, reason) = linked_pr_merged_with_client(
            &db,
            dir.path(),
            Some(pull_request_field()),
            &wrong_branch_client,
            "atelier-hw9t",
        )
        .unwrap();
        assert!(!passed);
        assert!(reason.contains("linked PR branches"));
        assert!(reason.contains("atelier review status --issue atelier-hw9t"));
    }
}
