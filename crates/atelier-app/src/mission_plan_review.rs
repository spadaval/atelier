use anyhow::{bail, Context, Result};
use atelier_records::activity::{
    allocate_activity_id, list_issue_activities, write_record_activity, ActivityEventType,
    IssueActivity,
};
use atelier_records::mission_plan_review::{
    load_mission_plan_review_cutover_manifest, mission_graph_revision, project_mission_plan_review,
    validate_stable_actor_identity, MissionPlanFindingSeverity, MissionPlanReviewEvent,
};
use atelier_records::RecordStore;
use chrono::{Timelike, Utc};
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MissionPlanReviewMutation {
    Request,
    Finding {
        finding_id: String,
        severity: MissionPlanFindingSeverity,
        affected_issue_ids: Vec<String>,
        dependency_path: Vec<String>,
    },
    ChangeRequest {
        request_id: String,
        affected_issue_ids: Vec<String>,
        dependency_path: Vec<String>,
    },
    Resolve {
        target_id: String,
        disposition: String,
    },
    Approve,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MissionPlanReviewMutationResult {
    pub activity_id: String,
    pub graph_revision: String,
}

pub fn mutate(
    state_dir: &Path,
    mission_id: &str,
    authenticated_actor: &str,
    mutation: MissionPlanReviewMutation,
) -> Result<MissionPlanReviewMutationResult> {
    validate_stable_actor_identity(authenticated_actor)
        .context("ATELIER_AUTHENTICATED_ACTOR is invalid")?;
    let mission = RecordStore::new(state_dir).load_issue_by_id(mission_id)?;
    if mission.issue.issue_type != "mission" {
        bail!(
            "mission-plan review requires a mission issue, found {}",
            mission.issue.issue_type
        );
    }
    let expected_status = match mutation {
        MissionPlanReviewMutation::Request => "draft",
        _ => "plan_review",
    };
    if mission.issue.status != expected_status {
        bail!(
            "mission-plan review mutation requires mission {} to be '{}', found '{}'",
            mission_id,
            expected_status,
            mission.issue.status
        );
    }

    let revision = mission_graph_revision(state_dir, mission_id)?;
    let event = match mutation {
        MissionPlanReviewMutation::Request => MissionPlanReviewEvent::Request {
            graph_revision: revision.clone(),
            authors: vec![authenticated_actor.to_string()],
            material_editors: vec![authenticated_actor.to_string()],
        },
        MissionPlanReviewMutation::Finding {
            finding_id,
            severity,
            mut affected_issue_ids,
            dependency_path,
        } => {
            affected_issue_ids.sort();
            affected_issue_ids.dedup();
            MissionPlanReviewEvent::Finding {
                graph_revision: revision.clone(),
                finding_id,
                severity,
                affected_issue_ids,
                dependency_path,
            }
        }
        MissionPlanReviewMutation::ChangeRequest {
            request_id,
            mut affected_issue_ids,
            dependency_path,
        } => {
            affected_issue_ids.sort();
            affected_issue_ids.dedup();
            MissionPlanReviewEvent::ChangeRequest {
                graph_revision: revision.clone(),
                request_id,
                affected_issue_ids,
                dependency_path,
            }
        }
        MissionPlanReviewMutation::Resolve {
            target_id,
            disposition,
        } => MissionPlanReviewEvent::Resolution {
            graph_revision: revision.clone(),
            target_id,
            disposition,
        },
        MissionPlanReviewMutation::Approve => MissionPlanReviewEvent::Approval {
            graph_revision: revision.clone(),
        },
    };
    event.validate()?;

    let now = Utc::now();
    let created_at = now
        .with_nanosecond((now.nanosecond() / 1_000) * 1_000)
        .expect("truncated nanoseconds remain valid");
    let activity = IssueActivity {
        id: allocate_activity_id(state_dir, "issue", mission_id, created_at)?,
        subject_kind: "issue".to_string(),
        subject_id: mission_id.to_string(),
        event_type: ActivityEventType::MissionPlanReview,
        actor: authenticated_actor.to_string(),
        created_at,
        summary: mutation_summary(&event).to_string(),
        pr_attribution: None,
        workflow_transition: None,
        mission_plan_review: Some(event),
        body: String::new(),
    };
    let mut activities = list_issue_activities(state_dir, mission_id)?;
    activities.push(activity.clone());
    project_mission_plan_review(
        mission_id,
        &mission.issue.status,
        revision.clone(),
        &activities,
        load_mission_plan_review_cutover_manifest(state_dir)?.as_ref(),
    )?;
    write_record_activity(state_dir, &activity)?;
    Ok(MissionPlanReviewMutationResult {
        activity_id: activity.id,
        graph_revision: revision.to_string(),
    })
}

fn mutation_summary(event: &MissionPlanReviewEvent) -> &'static str {
    match event {
        MissionPlanReviewEvent::Request { .. } => "Requested independent mission-plan review",
        MissionPlanReviewEvent::Finding { .. } => "Recorded mission-plan review finding",
        MissionPlanReviewEvent::ChangeRequest { .. } => "Requested mission-plan changes",
        MissionPlanReviewEvent::Resolution { .. } => "Resolved mission-plan review decision",
        MissionPlanReviewEvent::Approval { .. } => "Approved exact mission graph",
        MissionPlanReviewEvent::MaterialEditAttribution { .. }
        | MissionPlanReviewEvent::LegacyGrandfather { .. } => "Recorded mission-plan review event",
    }
}
