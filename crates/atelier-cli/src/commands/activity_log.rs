use anyhow::Result;
use chrono::Utc;
use std::path::{Path, PathBuf};

use atelier_records::activity::{
    create_issue_activity, create_issue_activity_with_metadata, create_mission_activity,
    list_issue_activities, ActivityAttemptLifecycle, ActivityAttemptMetadata, ActivityAttemptRole,
    ActivityEventType, ActivityPrAttribution,
};

pub fn record_comment(issue_id: &str, kind: &str, body: &str) -> Result<()> {
    let (event_type, summary) = match kind {
        "note" => (ActivityEventType::Note, "Added note"),
        "handoff" => (ActivityEventType::Handoff, "Added handoff"),
        "plan" => (ActivityEventType::Plan, "Added plan"),
        _ => (ActivityEventType::Comment, "Added comment"),
    };
    record(issue_id, event_type, summary, body)
}

pub fn record_mission_comment(mission_id: &str, kind: &str, body: &str) -> Result<()> {
    let (event_type, summary) = match kind {
        "note" => (ActivityEventType::Note, "Added note"),
        "handoff" => (ActivityEventType::Handoff, "Added handoff"),
        "plan" => (ActivityEventType::Plan, "Added plan"),
        _ => (ActivityEventType::Comment, "Added comment"),
    };
    record_mission(mission_id, event_type, summary, body)
}

pub fn record_note(issue_id: &str, body: &str) -> Result<()> {
    record(issue_id, ActivityEventType::Note, "Added note", body)
}

pub fn record_close_reason(issue_id: &str, reason: &str) -> Result<()> {
    record(
        issue_id,
        ActivityEventType::CloseReason,
        "Recorded close reason",
        reason,
    )
}

pub fn record_field_changed(
    issue_id: &str,
    field: &str,
    old: Option<&str>,
    new: Option<&str>,
) -> Result<()> {
    if old == new {
        return Ok(());
    }
    record(
        issue_id,
        ActivityEventType::FieldChanged,
        &format!("Changed {field}"),
        &field_change_body(field, old, new),
    )
}

pub fn record_work_started(
    issue_id: &str,
    branch: Option<&str>,
    worktree_path: Option<&str>,
) -> Result<()> {
    let Some(state_dir) = current_state_dir_for_issue(issue_id) else {
        return Ok(());
    };
    record_work_started_in_state_dir(&state_dir, issue_id, branch, worktree_path)
}

pub fn record_work_started_in_state_dir(
    state_dir: &Path,
    issue_id: &str,
    branch: Option<&str>,
    worktree_path: Option<&str>,
) -> Result<()> {
    let role = ActivityAttemptRole::Worker;
    create_issue_activity_with_metadata(
        state_dir,
        issue_id,
        ActivityEventType::WorkStarted,
        &current_actor(),
        Utc::now(),
        "Started work",
        Some(ActivityAttemptMetadata {
            role,
            serial: next_attempt_serial(state_dir, issue_id, role)?,
            lifecycle: ActivityAttemptLifecycle::Started,
            agent: current_agent(),
            subskill: current_subskill(),
        }),
        None,
        &work_body(branch, worktree_path),
    )?;
    Ok(())
}

pub fn record_pr_action(
    issue_id: &str,
    role: ActivityAttemptRole,
    action: &str,
    forge_pr: &str,
    remote_author: Option<&str>,
) -> Result<()> {
    let Some(state_dir) = current_state_dir_for_issue(issue_id) else {
        return Ok(());
    };
    record_pr_action_in_state_dir(&state_dir, issue_id, role, action, forge_pr, remote_author)
}

pub fn record_pr_action_in_state_dir(
    state_dir: &Path,
    issue_id: &str,
    role: ActivityAttemptRole,
    action: &str,
    forge_pr: &str,
    remote_author: Option<&str>,
) -> Result<()> {
    let serial = current_attempt_serial(state_dir, issue_id, role)?.unwrap_or(1);
    create_issue_activity_with_metadata(
        state_dir,
        issue_id,
        ActivityEventType::Comment,
        &current_actor(),
        Utc::now(),
        &format!("Recorded PR {action}"),
        Some(ActivityAttemptMetadata {
            role,
            serial,
            lifecycle: ActivityAttemptLifecycle::Updated,
            agent: current_agent(),
            subskill: current_subskill(),
        }),
        Some(ActivityPrAttribution {
            action: action.to_string(),
            forge_pr: Some(forge_pr.to_string()),
            remote_author: remote_author.map(str::to_string),
        }),
        &pr_action_body(role, action, forge_pr, remote_author),
    )?;
    Ok(())
}

pub fn attempt_role_from_cli(role: &str) -> Option<ActivityAttemptRole> {
    match role {
        "worker" => Some(ActivityAttemptRole::Worker),
        "reviewer" => Some(ActivityAttemptRole::Reviewer),
        "validator" => Some(ActivityAttemptRole::Validator),
        _ => None,
    }
}

fn current_attempt_serial(
    state_dir: &Path,
    issue_id: &str,
    role: ActivityAttemptRole,
) -> Result<Option<u32>> {
    Ok(list_issue_activities(state_dir, issue_id)?
        .into_iter()
        .filter_map(|activity| activity.attempt)
        .filter(|attempt| attempt.role == role)
        .map(|attempt| attempt.serial)
        .max())
}

fn next_attempt_serial(state_dir: &Path, issue_id: &str, role: ActivityAttemptRole) -> Result<u32> {
    Ok(current_attempt_serial(state_dir, issue_id, role)?.unwrap_or(0) + 1)
}

pub fn record_evidence_attached(
    issue_id: &str,
    evidence_id: &str,
    result: Option<&str>,
) -> Result<()> {
    record(
        issue_id,
        ActivityEventType::EvidenceAttached,
        &format!("Attached evidence {evidence_id}"),
        &format!(
            "evidence_id: {}\nresult: {}",
            scalar(evidence_id),
            option_scalar(result)
        ),
    )
}

pub fn record_transition_applied(
    issue_id: &str,
    transition: &str,
    from: &str,
    to: &str,
) -> Result<()> {
    record(
        issue_id,
        ActivityEventType::TransitionApplied,
        &format!("Applied transition {transition} ({from} -> {to})"),
        &transition_body(transition, from, Some(to), None),
    )
}

pub fn record_transition_blocked(
    issue_id: &str,
    transition: &str,
    from: &str,
    to: Option<&str>,
    reason: &str,
) -> Result<()> {
    record(
        issue_id,
        ActivityEventType::TransitionBlocked,
        &format!("Blocked transition {transition} from {from}"),
        &transition_body(transition, from, to, Some(reason)),
    )
}

fn record(issue_id: &str, event_type: ActivityEventType, summary: &str, body: &str) -> Result<()> {
    let Some(state_dir) = current_state_dir_for_issue(issue_id) else {
        return Ok(());
    };
    create_issue_activity(
        &state_dir,
        issue_id,
        event_type,
        &current_actor(),
        Utc::now(),
        summary,
        body,
    )?;
    Ok(())
}

fn record_mission(
    mission_id: &str,
    event_type: ActivityEventType,
    summary: &str,
    body: &str,
) -> Result<()> {
    let Some(state_dir) = current_state_dir_for_mission(mission_id) else {
        return Ok(());
    };
    create_mission_activity(
        &state_dir,
        mission_id,
        event_type,
        &current_actor(),
        Utc::now(),
        summary,
        body,
    )?;
    Ok(())
}

fn current_state_dir_for_issue(issue_id: &str) -> Option<PathBuf> {
    let state_dir = atelier_app::storage_layout::find_canonical_dir_from_cwd().ok()??;
    let issue_file = state_dir.join("issues").join(format!("{issue_id}.md"));
    issue_file.is_file().then_some(state_dir)
}

fn current_state_dir_for_mission(mission_id: &str) -> Option<PathBuf> {
    let state_dir = atelier_app::storage_layout::find_canonical_dir_from_cwd().ok()??;
    let mission_file = state_dir.join("missions").join(format!("{mission_id}.md"));
    mission_file.is_file().then_some(state_dir)
}

fn current_actor() -> String {
    std::env::var("ATELIER_AGENT")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "agent".to_string())
}

fn current_agent() -> Option<String> {
    std::env::var("ATELIER_AGENT").ok()
}

fn current_subskill() -> Option<String> {
    std::env::var("ATELIER_SUBSKILL").ok()
}

fn field_change_body(field: &str, old: Option<&str>, new: Option<&str>) -> String {
    format!(
        "field: {}\nold: {}\nnew: {}",
        scalar(field),
        option_scalar(old),
        option_scalar(new)
    )
}

fn work_body(branch: Option<&str>, worktree_path: Option<&str>) -> String {
    format!(
        "branch: {}\nworktree_path: {}",
        option_scalar(branch),
        option_scalar(worktree_path)
    )
}

fn transition_body(transition: &str, from: &str, to: Option<&str>, reason: Option<&str>) -> String {
    format!(
        "transition: {}\nfrom: {}\nto: {}\nreason: {}",
        scalar(transition),
        scalar(from),
        option_scalar(to),
        option_scalar(reason)
    )
}

fn pr_action_body(
    role: ActivityAttemptRole,
    action: &str,
    forge_pr: &str,
    remote_author: Option<&str>,
) -> String {
    format!(
        "role: {}\naction: {}\nforge_pr: {}\nremote_author: {}",
        scalar(role.as_str()),
        scalar(action),
        scalar(forge_pr),
        option_scalar(remote_author)
    )
}

fn option_scalar(value: Option<&str>) -> String {
    value.map(scalar).unwrap_or_else(|| "null".to_string())
}

fn scalar(value: &str) -> String {
    serde_json::to_string(value).expect("string serialization cannot fail")
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_records::activity::{
        list_derived_issue_attempts, list_issue_activities, ActivityAttemptLifecycle,
        ActivityAttemptRole, DerivedIssueAttemptState,
    };
    use tempfile::tempdir;

    #[test]
    fn work_started_records_worker_attempt_metadata() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path();
        let issue_id = "atelier-work";

        record_work_started_in_state_dir(
            state_dir,
            issue_id,
            Some("codex/work"),
            Some("/tmp/work"),
        )
        .unwrap();

        let activities = list_issue_activities(state_dir, issue_id).unwrap();
        assert_eq!(activities.len(), 1);
        let attempt = activities[0].attempt.as_ref().unwrap();
        assert_eq!(attempt.role, ActivityAttemptRole::Worker);
        assert_eq!(attempt.serial, 1);
        assert_eq!(attempt.lifecycle, ActivityAttemptLifecycle::Started);
        assert!(activities[0].body.contains("branch: \"codex/work\""));
    }

    #[test]
    fn pr_actions_record_structured_attempt_and_pr_attribution() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path();
        let issue_id = "atelier-pr";

        record_work_started_in_state_dir(state_dir, issue_id, Some("codex/pr"), None).unwrap();
        record_pr_action_in_state_dir(
            state_dir,
            issue_id,
            ActivityAttemptRole::Worker,
            "open",
            "forgejo/tools/atelier#42",
            Some("forge-worker"),
        )
        .unwrap();
        record_pr_action_in_state_dir(
            state_dir,
            issue_id,
            ActivityAttemptRole::Reviewer,
            "review",
            "forgejo/tools/atelier#42",
            Some("forge-reviewer"),
        )
        .unwrap();
        record_pr_action_in_state_dir(
            state_dir,
            issue_id,
            ActivityAttemptRole::Validator,
            "comment",
            "forgejo/tools/atelier#42",
            Some("forge-validator"),
        )
        .unwrap();

        let activities = list_issue_activities(state_dir, issue_id).unwrap();
        let pr_actions = activities
            .iter()
            .filter(|activity| activity.pr_attribution.is_some())
            .collect::<Vec<_>>();
        assert_eq!(pr_actions.len(), 3);
        assert!(pr_actions.iter().any(|activity| {
            activity.attempt.as_ref().unwrap().role == ActivityAttemptRole::Worker
                && activity.attempt.as_ref().unwrap().serial == 1
                && activity.pr_attribution.as_ref().unwrap().action == "open"
                && activity
                    .pr_attribution
                    .as_ref()
                    .unwrap()
                    .forge_pr
                    .as_deref()
                    == Some("forgejo/tools/atelier#42")
                && activity
                    .pr_attribution
                    .as_ref()
                    .unwrap()
                    .remote_author
                    .as_deref()
                    == Some("forge-worker")
        }));
        assert!(pr_actions.iter().any(|activity| {
            activity.attempt.as_ref().unwrap().role == ActivityAttemptRole::Reviewer
                && activity.pr_attribution.as_ref().unwrap().action == "review"
        }));
        assert!(pr_actions.iter().any(|activity| {
            activity.attempt.as_ref().unwrap().role == ActivityAttemptRole::Validator
                && activity.pr_attribution.as_ref().unwrap().action == "comment"
        }));

        let attempts = list_derived_issue_attempts(state_dir).unwrap();
        assert_eq!(attempts.len(), 3);
        assert!(attempts.iter().any(|attempt| {
            attempt.id == "atelier-pr/worker/1"
                && attempt.state == DerivedIssueAttemptState::Active
                && attempt.activity_ids.len() == 2
        }));
        assert!(attempts
            .iter()
            .any(|attempt| attempt.id == "atelier-pr/reviewer/1"));
        assert!(attempts
            .iter()
            .any(|attempt| attempt.id == "atelier-pr/validator/1"));
    }
}
