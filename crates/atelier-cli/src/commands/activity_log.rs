use anyhow::Result;
use chrono::Utc;
use std::path::{Path, PathBuf};

use atelier_records::activity::{
    create_issue_activity_with_metadata, ActivityEventType, ActivityPrAttribution,
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
    record(
        issue_id,
        ActivityEventType::WorkStarted,
        "Started work",
        &work_body(branch, worktree_path),
    )
}

pub fn record_pr_action(
    issue_id: &str,
    role: &str,
    action: &str,
    pull_request: &str,
    remote_author: Option<&str>,
) -> Result<()> {
    let Some(state_dir) = current_state_dir_for_issue(issue_id) else {
        return Ok(());
    };
    record_pr_action_in_state_dir(
        &state_dir,
        issue_id,
        role,
        action,
        pull_request,
        remote_author,
    )
}

pub fn record_pr_action_in_state_dir(
    state_dir: &Path,
    issue_id: &str,
    role: &str,
    action: &str,
    pull_request: &str,
    remote_author: Option<&str>,
) -> Result<()> {
    create_issue_activity_with_metadata(
        state_dir,
        issue_id,
        ActivityEventType::Comment,
        &current_actor(),
        Utc::now(),
        &format!("Recorded PR {action}"),
        Some(ActivityPrAttribution {
            action: action.to_string(),
            role: role.to_string(),
            pull_request: Some(pull_request.to_string()),
            remote_author: remote_author.map(str::to_string),
        }),
        &pr_action_body(role, action, pull_request, remote_author),
    )?;
    Ok(())
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
    if transition == "request_review" || transition == "request_validation" || done_status(to) {
        record(
            issue_id,
            ActivityEventType::WorkFinished,
            &format!("Finished work before transition {transition}"),
            &transition_body(transition, from, Some(to), None),
        )?;
    }
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
    create_issue_activity_with_metadata(
        &state_dir,
        issue_id,
        event_type,
        &current_actor(),
        Utc::now(),
        summary,
        None,
        body,
    )?;
    Ok(())
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
    record(mission_id, event_type, summary, body)
}

fn current_state_dir_for_issue(issue_id: &str) -> Option<PathBuf> {
    let state_dir = atelier_app::storage_layout::find_canonical_dir_from_cwd().ok()??;
    let issue_file = state_dir.join("issues").join(format!("{issue_id}.md"));
    issue_file.is_file().then_some(state_dir)
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

fn pr_action_body(
    role: &str,
    action: &str,
    pull_request: &str,
    remote_author: Option<&str>,
) -> String {
    format!(
        "role: {}\naction: {}\npull_request: {}\nremote_author: {}",
        scalar(role),
        scalar(action),
        scalar(pull_request),
        option_scalar(remote_author)
    )
}

fn option_scalar(value: Option<&str>) -> String {
    value.map(scalar).unwrap_or_else(|| "null".to_string())
}

fn scalar(value: &str) -> String {
    serde_json::to_string(value).expect("string serialization cannot fail")
}
