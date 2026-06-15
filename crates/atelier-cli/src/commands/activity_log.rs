use anyhow::Result;
use chrono::Utc;
use std::path::PathBuf;

use crate::activity::{create_issue_activity, create_mission_activity, ActivityEventType};

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
    record(
        issue_id,
        ActivityEventType::WorkStarted,
        "Started work",
        &work_body(branch, worktree_path),
    )
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
    let state_dir = crate::storage_layout::find_canonical_dir_from_cwd().ok()??;
    let issue_file = state_dir.join("issues").join(format!("{issue_id}.md"));
    issue_file.is_file().then_some(state_dir)
}

fn current_state_dir_for_mission(mission_id: &str) -> Option<PathBuf> {
    let state_dir = crate::storage_layout::find_canonical_dir_from_cwd().ok()??;
    let mission_file = state_dir.join("missions").join(format!("{mission_id}.md"));
    mission_file.is_file().then_some(state_dir)
}

fn current_actor() -> String {
    std::env::var("ATELIER_AGENT")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "agent".to_string())
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
