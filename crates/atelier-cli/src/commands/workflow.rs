use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use std::collections::BTreeSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::human_output::{self, DecisionState, StylePolicy};
use atelier_app::forgejo::{ForgejoClient, UreqForgejoTransport};
use atelier_app::pr as app_pr;
use atelier_app::project_config::{ProjectConfig, ReviewConfig, ReviewProviderKind};
use atelier_app::review_room;
use atelier_app::use_cases as app_use_cases;
use atelier_app::workflow_policy::{BranchLifecycleResolution, MergeStrategy};
use atelier_core::Issue;
use atelier_records::CanonicalIssueRecord;
use atelier_sqlite::Database;
use serde_json::Value;

pub(crate) use crate::commands::workflow_actions::action_preflight_blockers;
pub(crate) use crate::commands::workflow_planning::{
    branch_lifecycle_context, branch_lifecycle_state_line, branch_owner_label,
    issue_transition_options, plan_transition_actions, IssueTransitionOption, PlannedAction,
};

pub use atelier_app::workflow_validation::ValidatorResult;

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
        atelier_app::command_surface::status_reason(&repo_root)?;
    if command_surface_passed {
        println!("Docs/Help Drift: clear");
    } else {
        println!("Docs/Help Drift: detected");
        println!("{command_surface_reason}");
        bail!("workflow_command_surface_drift: {command_surface_reason}");
    }
    Ok(())
}

pub fn transition_issue(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: &str,
    transition_name: &str,
    close_reason: Option<&str>,
) -> Result<()> {
    let issue_id = crate::commands::issue::resolve_id(db, issue_ref)?;
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
    let planned_actions = plan_transition_actions(db, &before, transition_name, transition)?;
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
    record = app_use_cases::load_canonical_issue(state_dir, &before.id)?;
    apply_transition_record(&policy, state_dir, &mut record, transition, close_reason)?;
    record_applied_actions(&before.id, transition_name, &planned_actions)?;
    record_applied_transition(&before, transition_name, transition)?;
    app_use_cases::refresh_after_canonical_write(state_dir, db_path)?;
    match execute_post_transition_actions(
        &repo_root,
        state_dir,
        db_path,
        &before,
        transition_name,
        &planned_actions,
    ) {
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
    if transition_name == "start" {
        print_start_context_and_record(&refreshed, &issue)?;
    }
    println!("Applied transition {} to {}", transition_name, issue.id);
    println!("From:     {}", before.status);
    println!("To:       {}", issue.status);
    for result in action_results {
        println!("Action:   {} {}", result.name, result.detail);
    }
    print_heading("Next Commands");
    if transition_name == "start" {
        println!("  Inspect checkout status: atelier status");
        if let Some(mission_id) = containing_mission(&refreshed, &issue.id)? {
            println!("  Inspect objective selection and blockers: atelier issue show {mission_id}");
        }
        println!(
            "  Inspect work transitions: atelier issue transition {}",
            issue.id
        );
        println!(
            "  Record proof: atelier evidence record --target issue/{} --kind test \"...\"",
            issue.id
        );
    } else {
        println!("  atelier issue show {}", issue.id);
        println!("  atelier issue transition {}", issue.id);
    }
    Ok(())
}

fn print_start_context_and_record(db: &Database, issue: &Issue) -> Result<()> {
    print_active_mission_context(db, &issue.id)?;
    let branch = git_current_branch(&repo_root()?).ok();
    let path = env::current_dir()?.to_string_lossy().to_string();
    crate::commands::activity_log::record_work_started(&issue.id, branch.as_deref(), Some(&path))?;
    println!("Started work on {} {}", issue.id, issue.title);
    if let Ok(context) = branch_lifecycle_context(db, &issue.id) {
        println!(
            "Branch owner: {} {} ({})",
            branch_owner_label(&context.resolution.owner_kind),
            context.resolution.owner_id,
            context.resolution.owner_issue_type
        );
        println!("Effective branch: {}", context.resolution.expected_branch);
        println!("Base branch: {}", context.resolution.base_branch);
    }
    if let Some(branch) = branch {
        println!("Branch: {branch}");
    }
    println!("Checkout: {path}");
    Ok(())
}

fn print_active_mission_context(db: &Database, issue_id: &str) -> Result<()> {
    if let Some(mission_id) = containing_mission(db, issue_id)? {
        println!("Mission: {mission_id} (linked)");
    }
    Ok(())
}

fn containing_mission(db: &Database, issue_id: &str) -> Result<Option<String>> {
    for mission in db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| issue.issue_type == "mission")
    {
        if mission.status == "closed" {
            continue;
        }
        if crate::commands::mission::issue_advances_mission(db, &mission.id, issue_id)? {
            return Ok(Some(mission.id));
        }
    }
    Ok(None)
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
            "branch.prepare" => {
                let detail = prepare_branch_action(repo_root, issue, action)?;
                applied.push(AppliedAction {
                    name: action.name.clone(),
                    detail,
                });
            }
            "tracker.commit" | "branch.push" | "review.merge" | "base.sync"
            | "branch_integrate" => {}
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
    state_dir: &Path,
    db_path: &Path,
    issue: &Issue,
    transition_name: &str,
    planned_actions: &[PlannedAction],
) -> Result<Vec<AppliedAction>> {
    let mut applied = Vec::new();
    for action in planned_actions {
        match action.name.as_str() {
            "tracker.commit" => {
                let detail = commit_branch_action(repo_root, issue, transition_name, action)?;
                applied.push(AppliedAction {
                    name: action.name.clone(),
                    detail,
                });
            }
            "branch.push" => {
                let detail = push_branch_action(repo_root, issue, action)?;
                applied.push(AppliedAction {
                    name: action.name.clone(),
                    detail,
                });
            }
            "review.merge" => {
                let detail = merge_review_action(repo_root, state_dir, db_path, issue, action)?;
                applied.push(AppliedAction {
                    name: action.name.clone(),
                    detail,
                });
            }
            "base.sync" => {
                let detail = sync_base_action(repo_root, action)?;
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

fn push_branch_action(repo_root: &Path, issue: &Issue, action: &PlannedAction) -> Result<String> {
    ensure_expected_branch_checked_out(repo_root, issue, action)?;
    git_checked(
        repo_root,
        &["push", "origin", &action.expected_branch],
        "push action branch",
    )
    .with_context(|| {
        format!(
            "action {} failed while pushing branch '{}'.\nRecovery: inspect `git status --short --branch`, provider remote configuration, and retry the transition for {}.",
            action.name, action.expected_branch, issue.id
        )
    })?;
    Ok(format!("pushed {}", action.expected_branch))
}

fn merge_review_action(
    repo_root: &Path,
    state_dir: &Path,
    db_path: &Path,
    issue: &Issue,
    _action: &PlannedAction,
) -> Result<String> {
    let db = app_use_cases::open_database(db_path)?;
    let policy = atelier_app::workflow_policy::load(repo_root)?;
    let role = policy.status_role(&issue.status).ok_or_else(|| {
        anyhow::anyhow!(
            "review_role_missing: issue {} is in status '{}' and that status has no role; configure statuses.{}.role before using review.merge as a transition action",
            issue.id,
            issue.status,
            issue.status
        )
    })?;
    crate::commands::pr::merge(
        &db,
        repo_root,
        &state_dir,
        &db_path,
        Some(&issue.id),
        Some(role),
    )?;
    Ok("provider review merged".to_string())
}

fn sync_base_action(repo_root: &Path, action: &PlannedAction) -> Result<String> {
    git_checked(repo_root, &["fetch", "origin", &action.base_branch], "fetch base branch")
        .with_context(|| {
            format!(
                "action {} failed while fetching base branch '{}'.\nRecovery: inspect the configured provider remote and retry the transition.",
                action.name, action.base_branch
            )
        })?;
    git_checked(repo_root, &["switch", &action.base_branch], "checkout base branch")
        .with_context(|| {
            format!(
                "action {} failed while switching to base branch '{}'.\nRecovery: inspect `git status --short --branch` before retrying.",
                action.name, action.base_branch
            )
        })?;
    git_checked(
        repo_root,
        &["merge", "--ff-only", &format!("origin/{}", action.base_branch)],
        "fast-forward base branch",
    )
    .with_context(|| {
        format!(
            "action {} failed while syncing base branch '{}'.\nRecovery: inspect local/base divergence before retrying.",
            action.name, action.base_branch
        )
    })?;
    Ok(format!("synced {}", action.base_branch))
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
            "action {} failed because source branch '{}' is missing.\nRecovery: run the transition with `branch.prepare` first, then retry the transition for {}.",
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
    let issue_id = crate::commands::issue::resolve_id(db, issue_ref)?;
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
            "Issue {} has no terminal done-category transitions from status '{}'; inspect `atelier issue transition {}`",
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
                "Issue {} has multiple terminal done targets from '{}'; rerun with `atelier issue transition {} <transition> --reason \"...\"` (available done statuses: {})",
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
            "tracker.commit" | "branch.push" | "review.merge" | "base.sync" | "branch_integrate"
        )
    })
}

fn transition_git_action_names(name: &str) -> bool {
    matches!(
        name,
        "tracker.commit" | "branch.push" | "review.merge" | "base.sync" | "branch_integrate"
    )
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
            .any(|action| transition_git_action_names(action.name.as_str()))
        {
            return Ok(None);
        }
        if !is_git_repo(repo_root)? {
            return Ok(None);
        }
        let Some(action) = planned_actions
            .iter()
            .find(|action| transition_git_action_names(action.name.as_str()))
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
                    "Close Git integration cannot find configured base branch '{}'.\nRecovery: create or fetch the base branch, then retry `atelier issue transition {} close --reason \"...\"`.",
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
                    "Close Git integration could not switch to source branch '{}'.\nRecovery: inspect `git status --short --branch`, then retry `atelier issue transition {} close --reason \"...\"`.",
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
            "Recovery:      rerun `atelier issue transition {} close --reason \"...\"` only if a later step reports failure",
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
                "Close Git integration failed while committing tracker state for {}.\nRecovery: tracker files were restored to their pre-close state; inspect `git status --short --branch`, then retry `atelier issue transition {} close --reason \"...\"`.",
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
                        "Close Git integration failed during squash merge from '{}' to '{}'.\nRecovery: merge state was aborted when possible and the source close commit was rolled back; inspect `git status --short --branch`, then retry `atelier issue transition {} close --reason \"...\"`.",
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
                        "Close Git integration failed while committing squash merge on '{}'.\nRecovery: merge state was aborted when possible and the source close commit was rolled back; inspect `git status --short --branch`, then retry `atelier issue transition {} close --reason \"...\"`.",
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
                        "Close Git integration failed during merge from '{}' to '{}'.\nRecovery: merge state was aborted when possible and the source close commit was rolled back; inspect `git status --short --branch`, then retry `atelier issue transition {} close --reason \"...\"`.",
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
                        "Close Git integration failed during fast-forward from '{}' to '{}'.\nRecovery: merge state was aborted when possible and the source close commit was rolled back; inspect `git status --short --branch`, then retry `atelier issue transition {} close --reason \"...\"`.",
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

pub(crate) fn is_git_repo(root: &Path) -> Result<bool> {
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
            "Close Git integration could not create source branch '{}'.\nRecovery: run `atelier issue transition {} start` to prepare the branch, then retry `atelier issue transition {} close --reason \"...\"`.",
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
            "Close Git integration requires non-tracker files to be clean before close:\n{}\nRecovery: commit or stash these paths, then retry `atelier issue transition <issue-id> close --reason \"...\"`.",
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
            "action {} failed {phase}: checkout has uncommitted non-tracker changes:\n{}\nRecovery: commit or stash these paths, then retry the transition for {}.",
            action.name,
            dirty.join("\n"),
            issue.id
        );
    }
    Ok(())
}

pub(crate) fn non_tracker_dirty_entries(root: &Path) -> Result<Vec<String>> {
    let status = git_stdout(
        root,
        &["status", "--porcelain"],
        "inspect checkout dirtiness",
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

pub(crate) fn git_current_branch(root: &Path) -> Result<String> {
    git_stdout(root, &["branch", "--show-current"], "read current branch")
        .map(|value| value.trim().to_string())
}

pub(crate) fn branch_exists_at(root: &Path, branch: &str) -> Result<bool> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["rev-parse", "--verify", "--quiet", branch])
        .output()
        .with_context(|| format!("failed to inspect git branch {branch}"))?;
    Ok(output.status.success())
}

pub(crate) fn git_dirty_entries(root: &Path) -> Result<Vec<String>> {
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

pub(crate) fn git_stdout(root: &Path, args: &[&str], action: &str) -> Result<String> {
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
    println!(
        "{}",
        render_issue_transition_options(db, issue, options, StylePolicy::for_stdout())
    );
}

fn render_issue_transition_options(
    db: &Database,
    issue: &Issue,
    options: &[IssueTransitionOption],
    style_policy: StylePolicy,
) -> String {
    let mut lines = vec![
        human_output::heading(&format!("Issue Transitions {} - {}", issue.id, issue.title)),
        human_output::section_heading("State"),
        format!("Status:   {}", issue.status),
        format!("Type:     {}", issue.issue_type),
        format!("Options:  {}", options.len()),
    ];
    let needs_branch_context = options.iter().any(|option| {
        crate::commands::workflow_planning::planned_actions_need_branch_context(
            &option.planned_actions,
        )
    });
    if needs_branch_context {
        if let Ok(context) = branch_lifecycle_context(db, &issue.id) {
            lines.push(human_output::section_heading("Branch Context"));
            lines.push(format!(
                "Owner:    {} {} ({})",
                branch_owner_label(&context.resolution.owner_kind),
                context.resolution.owner_id,
                context.resolution.owner_issue_type
            ));
            lines.push(format!("Expected: {}", context.resolution.expected_branch));
            lines.push(format!("Base:     {}", context.resolution.base_branch));
            lines.push(format!(
                "Current:  {}",
                context.current_branch.as_deref().unwrap_or("(detached)")
            ));
            lines.push(format!(
                "State:    {}",
                branch_lifecycle_state_line(&context)
            ));
        }
    }
    for option in options {
        let decision = if option.allowed {
            DecisionState::Allowed
        } else {
            DecisionState::Blocked
        };
        lines.push(String::new());
        lines.push(format!(
            "{} [{}]",
            option.name,
            decision.render(style_policy)
        ));
        lines.push(format!("  Decision: {}", decision.render(style_policy)));
        lines.push(format!("  From: {}", option.from.join(", ")));
        lines.push(format!("  To:   {}", option.to));
        lines.extend(render_transition_detail(
            "Validators",
            &option.validator_results,
            style_policy,
        ));
        lines.extend(render_text_list("Blockers", &option.blockers));
        lines.extend(render_text_list(
            "Planned Actions",
            &planned_action_lines(&option.planned_actions),
        ));
        lines.extend(render_text_list("Description", &option.descriptions));
        lines.extend(render_text_list("Commands", &[option.command.clone()]));
    }
    lines.join("\n")
}

pub fn evaluate(
    db: &Database,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validators: Vec<String>,
) -> Result<Vec<ValidatorResult>> {
    ensure_target_exists(db, target_kind, target_id)?;
    let policy = atelier_app::workflow_policy::load(&repo_root()?)?;
    let definitions = if validators.is_empty() && target_kind == "mission" {
        policy
            .transition_for_issue_type("mission", transition)
            .with_context(|| {
                format!(
                    "mission terminal checks require issue_type 'mission' and transition '{}' in {}",
                    transition,
                    atelier_app::workflow_policy::WORKFLOW_POLICY_PATH
                )
            })?
            .validators
            .clone()
    } else if validators.is_empty() {
        default_validator_definitions(target_kind, transition)
    } else {
        validators
            .into_iter()
            .map(
                |builtin| atelier_app::workflow_policy::ValidatorDefinition {
                    builtin,
                    params: None,
                },
            )
            .collect()
    };
    evaluate_policy_transition(
        db,
        &policy,
        target_kind,
        target_id,
        transition,
        &definitions,
    )
}

fn default_validator_definitions(
    target_kind: &str,
    transition: &str,
) -> Vec<atelier_app::workflow_policy::ValidatorDefinition> {
    default_validators(target_kind, transition)
        .into_iter()
        .map(
            |builtin| atelier_app::workflow_policy::ValidatorDefinition {
                builtin,
                params: None,
            },
        )
        .collect()
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
    for line in render_transition_detail(title, results, StylePolicy::for_stdout()) {
        println!("{line}");
    }
}

fn render_transition_detail(
    title: &str,
    results: &[ValidatorResult],
    style_policy: StylePolicy,
) -> Vec<String> {
    let mut lines = vec![human_output::section_heading(title)];
    if results.is_empty() {
        lines.push("(none)".to_string());
        return lines;
    }
    for result in results {
        let decision = if result.passed {
            DecisionState::Pass
        } else {
            DecisionState::Fail
        };
        lines.push(format!(
            "  {}  {}",
            decision.render(style_policy),
            result.validator
        ));
        lines.push(format!("      {}", result.reason));
        if let Some(help) = &result.help {
            lines.push(format!("      Hint: {help}"));
        }
    }
    lines
}

fn print_text_list(title: &str, values: &[String]) {
    for line in render_text_list(title, values) {
        println!("{line}");
    }
}

fn render_text_list(title: &str, values: &[String]) -> Vec<String> {
    let mut lines = vec![human_output::section_heading(title)];
    if values.is_empty() {
        lines.push("(none)".to_string());
        return lines;
    }
    for value in values {
        lines.push(format!("  {value}"));
    }
    lines
}

pub fn default_validators(target_kind: &str, transition: &str) -> Vec<String> {
    let names: &[&str] = match (target_kind, transition) {
        ("issue", "start") => &[
            "tracker.current",
            "issue.sections_parseable",
            "blockers.none_open",
        ],
        ("issue", "close") => &[
            "tracker.current",
            "issue.sections_parseable",
            "blockers.none_open",
            "evidence.attached",
        ],
        ("mission", "close") => mission_terminal_validators(),
        ("mission", _) => &[
            "tracker.current",
            "issue.sections_parseable",
            "blockers.none_open",
        ],
        ("evidence", _) => &["tracker.current"],
        ("tracker", "health") => &[
            "tracker.current",
            "lint.none_blocking",
            "command_surface_current",
            "ignored_tests_reviewed",
            "git.worktree_clean",
        ],
        _ => &["tracker.current"],
    };
    names.iter().map(|name| (*name).to_string()).collect()
}

pub(crate) fn mission_terminal_validators() -> &'static [&'static str] {
    &[
        "tracker.current",
        "issue.sections_parseable",
        "no_open_work",
        "blockers.none_open",
        "validation.criteria_satisfied",
        "lint.none_blocking",
        "command_surface_current",
        "ignored_tests_reviewed",
        "git.on_base_branch",
        "git.worktree_clean",
    ]
}

fn print_heading(title: &str) {
    human_output::print_section_heading(title);
}

pub(crate) fn ensure_transitionable_status(
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

pub(crate) fn required_field_failures(
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

pub(crate) fn evaluate_policy_transition(
    db: &Database,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validators: &[atelier_app::workflow_policy::ValidatorDefinition],
) -> Result<Vec<ValidatorResult>> {
    atelier_app::workflow_validation::evaluate_policy_transition(
        atelier_app::workflow_validation::ValidatorRequest {
            db,
            repo_root: &repo_root()?,
            policy,
            target_kind,
            target_id,
            transition,
            validators,
        },
    )
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

pub(crate) fn repo_root() -> Result<PathBuf> {
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
    use crate::commands::workflow_planning::plan_actions_for_resolution;
    use crate::human_output::ColorChoice;
    use atelier_app::workflow_policy::{
        ActionParams, ReviewArtifactActionParams, WorkflowForgejoRoleAuthors,
    };
    use atelier_records::{IssueSections, RecordStore, Relationships};
    use chrono::Utc;
    use std::collections::BTreeMap;
    use tempfile::{tempdir, TempDir};

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

    fn setup_test_db() -> (Database, TempDir) {
        let dir = tempdir().unwrap();
        let db = Database::open(&dir.path().join("test.db")).unwrap();
        (db, dir)
    }

    fn transition_option(allowed: bool, passed: bool) -> IssueTransitionOption {
        IssueTransitionOption {
            name: "start".to_string(),
            from: vec!["todo".to_string()],
            to: "in_progress".to_string(),
            allowed,
            blockers: Vec::new(),
            validator_results: vec![ValidatorResult {
                target_kind: "issue".to_string(),
                target_id: "atelier-test".to_string(),
                transition: "start".to_string(),
                validator: "tracker.current".to_string(),
                passed,
                reason: "tracker is current".to_string(),
                help: None,
                elapsed_ms: 1,
            }],
            planned_actions: Vec::new(),
            descriptions: vec!["Begin work.".to_string()],
            command: "atelier issue transition atelier-test start".to_string(),
        }
    }

    #[test]
    fn transition_options_use_color_when_interactive_context_allows_it() {
        let (db, _dir) = setup_test_db();
        let issue = test_issue("atelier-test");
        let policy = StylePolicy::from_context(ColorChoice::Auto, true, false);

        let output =
            render_issue_transition_options(&db, &issue, &[transition_option(true, true)], policy);

        assert!(output.contains("\u{1b}[32mallowed\u{1b}[0m"));
        assert!(output.contains("Decision: \u{1b}[32mallowed\u{1b}[0m"));
        assert!(output.contains("tracker.current"));
    }

    #[test]
    fn transition_options_stay_plain_when_no_color_is_set() {
        let (db, _dir) = setup_test_db();
        let issue = test_issue("atelier-test");
        let policy = StylePolicy::from_context(ColorChoice::Auto, true, true);

        let output = render_issue_transition_options(
            &db,
            &issue,
            &[transition_option(false, false)],
            policy,
        );

        assert!(!output.contains("\u{1b}["));
        assert!(output.contains("start [blocked]"));
        assert!(output.contains("Decision: blocked"));
        assert!(output.contains("fail  tracker.current"));
    }

    #[test]
    fn transition_options_stay_plain_when_stdout_is_not_interactive() {
        let (db, _dir) = setup_test_db();
        let issue = test_issue("atelier-test");
        let policy = StylePolicy::from_context(ColorChoice::Auto, false, false);

        let output =
            render_issue_transition_options(&db, &issue, &[transition_option(true, true)], policy);

        assert!(!output.contains("\u{1b}["));
        assert!(output.contains("start [allowed]"));
        assert!(output.contains("Decision: allowed"));
        assert!(output.contains("pass  tracker.current"));
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
                "tracker.current",
                "issue.sections_parseable",
                "blockers.none_open"
            ]
        );
        assert_eq!(
            default_validators("issue", "close"),
            vec![
                "tracker.current",
                "issue.sections_parseable",
                "blockers.none_open",
                "evidence.attached"
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
            vec!["tracker.current"]
        );
        assert_eq!(
            default_validators("tracker", "health"),
            vec![
                "tracker.current",
                "lint.none_blocking",
                "command_surface_current",
                "ignored_tests_reviewed",
                "git.worktree_clean"
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
    fn provider_terminal_actions_plan_without_local_branch_integrate() {
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
            plan_actions_for_resolution(&issue, &resolution, &[action("branch_integrate")], 1);
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

        let actions =
            plan_actions_for_resolution(&issue, &resolution, &[forgejo_review_action()], 1);
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
        let action =
            plan_actions_for_resolution(&issue, &resolution, &[review_action()], 1).remove(0);

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
    fn transition_status_write_preserves_review_field_from_pre_action_reload() {
        let dir = tempdir().unwrap();
        write_room_config_and_workflow(&dir);
        let state_dir = dir.path().join(".atelier");
        let db_path = dir.path().join(".atelier/runtime/state.db");
        let db = Database::open(&db_path).unwrap();
        let issue = test_issue("atelier-epic1");
        insert_canonical_issue(&db, &state_dir, issue.clone());
        let policy = atelier_app::workflow_policy::load(dir.path()).unwrap();
        let transition = resolve_issue_transition(&policy, &issue, "request_review").unwrap();
        let stale_record =
            app_use_cases::load_canonical_issue(&state_dir, "atelier-epic1").unwrap();
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
        let action =
            plan_actions_for_resolution(&issue, &resolution, &[review_action()], 1).remove(0);

        open_review_artifact_action(
            &db,
            &state_dir,
            &db_path,
            dir.path(),
            &issue,
            "request_review",
            &action,
        )
        .unwrap();
        assert!(!stale_record.issue.fields.contains_key("review"));

        let mut reloaded_record =
            app_use_cases::load_canonical_issue(&state_dir, "atelier-epic1").unwrap();
        apply_transition_record(&policy, &state_dir, &mut reloaded_record, transition, None)
            .unwrap();

        let final_record =
            app_use_cases::load_canonical_issue(&state_dir, "atelier-epic1").unwrap();
        assert_eq!(final_record.issue.status, "review");
        let review = final_record.issue.fields.get("review").unwrap();
        assert_eq!(review["kind"], "room");
        assert!(review["id"].as_str().unwrap().starts_with("atelier-"));
    }

    #[test]
    fn room_review_complete_requires_merged_room_artifact() {
        let dir = tempdir().unwrap();
        write_room_config_and_workflow(&dir);
        let state_dir = dir.path().join(".atelier");
        let db_path = dir.path().join(".atelier/runtime/state.db");
        let db = Database::open(&db_path).unwrap();
        let mut issue = test_issue("atelier-epic1");
        issue.status = "review".to_string();
        insert_canonical_issue(&db, &state_dir, issue);

        let outcome = review_room::open(
            &db,
            review_room::RoomOpenRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-epic1"),
                role: "worker",
                title: "Review atelier-epic1",
                body: "Please review.",
                source_branch: "epic/atelier-epic1",
                target_branch: "master",
            },
        )
        .unwrap();
        drop(db);
        let db = Database::open(&db_path).unwrap();

        let policy = atelier_app::workflow_policy::load(dir.path()).unwrap();
        let validators = vec![atelier_app::workflow_policy::ValidatorDefinition {
            builtin: "review.complete".to_string(),
            params: None,
        }];
        let results = atelier_app::workflow_validation::evaluate_policy_transition(
            atelier_app::workflow_validation::ValidatorRequest {
                db: &db,
                repo_root: dir.path(),
                policy: &policy,
                target_kind: "issue",
                target_id: "atelier-epic1",
                transition: "close",
                validators: &validators,
            },
        )
        .unwrap();
        let result = results.first().unwrap();
        let passed = result.passed;
        let reason = result.reason.clone();
        assert!(!passed);
        assert!(
            reason.contains(&format!("review room {}", outcome.review_id)),
            "{reason}"
        );
        assert!(
            reason.contains("atelier review status --issue atelier-epic1"),
            "{reason}"
        );

        review_room::approve(
            &db,
            review_room::RoomDecisionRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-epic1"),
                role: "reviewer",
                body: "Approved.",
            },
        )
        .unwrap();
        review_room::merge(
            &db,
            review_room::RoomMergeRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-epic1"),
                role: "manager",
            },
        )
        .unwrap();

        let results = atelier_app::workflow_validation::evaluate_policy_transition(
            atelier_app::workflow_validation::ValidatorRequest {
                db: &db,
                repo_root: dir.path(),
                policy: &policy,
                target_kind: "issue",
                target_id: "atelier-epic1",
                transition: "close",
                validators: &validators,
            },
        )
        .unwrap();
        let result = results.first().unwrap();
        let passed = result.passed;
        let reason = result.reason.clone();
        assert!(passed);
        assert_eq!(
            reason,
            format!("review room {} is merged", outcome.review_id)
        );
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
            .filter(|validator| validator.builtin == "review.linked_pr_merged")
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
            .any(|validator| validator.builtin == "review.linked_pr_merged"));
    }
}
