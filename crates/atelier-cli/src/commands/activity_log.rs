use anyhow::Result;
use chrono::Utc;
use std::path::PathBuf;

use atelier_records::activity::{
    create_issue_activity_with_metadata, create_mission_activity, derive_issue_attempts,
    list_issue_activities, ActivityAttemptLifecycle, ActivityAttemptMetadata, ActivityAttemptRole,
    ActivityEventType, DerivedIssueAttemptState,
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
    record_with_attempt(
        issue_id,
        ActivityEventType::WorkStarted,
        "Started work",
        Some(attempt_for_role(
            issue_id,
            ActivityAttemptRole::Worker,
            ActivityAttemptLifecycle::Started,
            SerialMode::ActiveOrNext,
        )?),
        &work_body(branch, worktree_path),
    )
}

pub fn record_evidence_attached(
    issue_id: &str,
    evidence_id: &str,
    result: Option<&str>,
) -> Result<()> {
    record_with_attempt(
        issue_id,
        ActivityEventType::EvidenceAttached,
        &format!("Attached evidence {evidence_id}"),
        Some(attempt_for_role(
            issue_id,
            ActivityAttemptRole::Validator,
            active_update_lifecycle(issue_id, ActivityAttemptRole::Validator)?,
            SerialMode::ActiveOrNext,
        )?),
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
    if let Some(attempt) = transition_completion_attempt(issue_id, transition, from)? {
        record_with_attempt(
            issue_id,
            ActivityEventType::WorkFinished,
            &format!("Finished role attempt before transition {transition}"),
            Some(attempt),
            &transition_body(transition, from, Some(to), None),
        )?;
    }
    record_with_attempt(
        issue_id,
        ActivityEventType::TransitionApplied,
        &format!("Applied transition {transition} ({from} -> {to})"),
        transition_attempt(issue_id, transition, from, to)?,
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
    record_with_attempt(issue_id, event_type, summary, None, body)
}

fn record_with_attempt(
    issue_id: &str,
    event_type: ActivityEventType,
    summary: &str,
    attempt: Option<ActivityAttemptMetadata>,
    body: &str,
) -> Result<()> {
    let Some(state_dir) = current_state_dir_for_issue(issue_id) else {
        return Ok(());
    };
    create_issue_activity_with_metadata(
        &state_dir,
        issue_id,
        event_type,
        &current_actor(),
        Utc::now(),
        summary,
        attempt,
        None,
        body,
    )?;
    Ok(())
}

fn transition_attempt(
    issue_id: &str,
    transition: &str,
    from: &str,
    to: &str,
) -> Result<Option<ActivityAttemptMetadata>> {
    if transition == "start" {
        return Ok(Some(attempt_for_role(
            issue_id,
            ActivityAttemptRole::Worker,
            ActivityAttemptLifecycle::Started,
            SerialMode::Next,
        )?));
    }
    if transition == "request_review" {
        return Ok(Some(attempt_for_role(
            issue_id,
            ActivityAttemptRole::Reviewer,
            ActivityAttemptLifecycle::Started,
            SerialMode::Next,
        )?));
    }
    if transition == "request_validation" {
        return Ok(Some(attempt_for_role(
            issue_id,
            ActivityAttemptRole::Validator,
            ActivityAttemptLifecycle::Started,
            SerialMode::Next,
        )?));
    }
    if done_status(to) {
        return Ok(Some(attempt_for_role(
            issue_id,
            role_for_status(from),
            ActivityAttemptLifecycle::Finished,
            SerialMode::ActiveOrLatest,
        )?));
    }
    if to == "blocked" {
        return Ok(Some(attempt_for_role(
            issue_id,
            role_for_status(from),
            ActivityAttemptLifecycle::Abandoned,
            SerialMode::ActiveOrLatest,
        )?));
    }
    Ok(None)
}

fn transition_completion_attempt(
    issue_id: &str,
    transition: &str,
    from: &str,
) -> Result<Option<ActivityAttemptMetadata>> {
    let role = match transition {
        "request_review" => ActivityAttemptRole::Worker,
        "request_validation" if from == "review" => ActivityAttemptRole::Reviewer,
        "request_validation" => ActivityAttemptRole::Worker,
        _ => return Ok(None),
    };
    Ok(Some(attempt_for_role(
        issue_id,
        role,
        ActivityAttemptLifecycle::Finished,
        SerialMode::ActiveOrLatest,
    )?))
}

#[derive(Clone, Copy)]
enum SerialMode {
    Next,
    ActiveOrNext,
    ActiveOrLatest,
}

fn attempt_for_role(
    issue_id: &str,
    role: ActivityAttemptRole,
    lifecycle: ActivityAttemptLifecycle,
    mode: SerialMode,
) -> Result<ActivityAttemptMetadata> {
    let serial = match mode {
        SerialMode::Next => next_serial(issue_id, role)?,
        SerialMode::ActiveOrNext => {
            active_serial(issue_id, role)?.unwrap_or(next_serial(issue_id, role)?)
        }
        SerialMode::ActiveOrLatest => active_serial(issue_id, role)?
            .or(latest_serial(issue_id, role)?)
            .unwrap_or(1),
    };
    Ok(ActivityAttemptMetadata {
        role,
        serial,
        lifecycle,
        agent: current_agent(),
        subskill: current_subskill(),
    })
}

fn active_update_lifecycle(
    issue_id: &str,
    role: ActivityAttemptRole,
) -> Result<ActivityAttemptLifecycle> {
    if active_serial(issue_id, role)?.is_some() {
        Ok(ActivityAttemptLifecycle::Updated)
    } else {
        Ok(ActivityAttemptLifecycle::Started)
    }
}

fn active_serial(issue_id: &str, role: ActivityAttemptRole) -> Result<Option<u32>> {
    Ok(derived_attempts(issue_id)?
        .into_iter()
        .filter(|attempt| attempt.role == role && attempt.state == DerivedIssueAttemptState::Active)
        .map(|attempt| attempt.serial)
        .max())
}

fn latest_serial(issue_id: &str, role: ActivityAttemptRole) -> Result<Option<u32>> {
    Ok(serials(issue_id, role)?.into_iter().max())
}

fn next_serial(issue_id: &str, role: ActivityAttemptRole) -> Result<u32> {
    Ok(latest_serial(issue_id, role)?.unwrap_or(0) + 1)
}

fn serials(issue_id: &str, role: ActivityAttemptRole) -> Result<Vec<u32>> {
    Ok(list_issue_activities_for_attempts(issue_id)?
        .into_iter()
        .filter_map(|activity| activity.attempt)
        .filter(|attempt| attempt.role == role)
        .map(|attempt| attempt.serial)
        .collect())
}

fn derived_attempts(issue_id: &str) -> Result<Vec<atelier_records::activity::DerivedIssueAttempt>> {
    derive_issue_attempts(list_issue_activities_for_attempts(issue_id)?)
}

fn list_issue_activities_for_attempts(
    issue_id: &str,
) -> Result<Vec<atelier_records::activity::IssueActivity>> {
    let Some(state_dir) = current_state_dir_for_issue(issue_id) else {
        return Ok(Vec::new());
    };
    list_issue_activities(&state_dir, issue_id)
}

fn role_for_status(status: &str) -> ActivityAttemptRole {
    match status {
        "review" => ActivityAttemptRole::Reviewer,
        "validation" => ActivityAttemptRole::Validator,
        _ => ActivityAttemptRole::Worker,
    }
}

fn done_status(status: &str) -> bool {
    matches!(status, "done" | "archived")
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
    std::env::var("ATELIER_AGENT_ID")
        .or_else(|_| std::env::var("ATELIER_AGENT"))
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn current_subskill() -> Option<String> {
    std::env::var("ATELIER_SUBSKILL")
        .or_else(|_| std::env::var("ATELIER_AGENT_SUBSKILL"))
        .ok()
        .filter(|value| !value.trim().is_empty())
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

fn option_scalar(value: Option<&str>) -> String {
    value.map(scalar).unwrap_or_else(|| "null".to_string())
}

fn scalar(value: &str) -> String {
    serde_json::to_string(value).expect("string serialization cannot fail")
}
