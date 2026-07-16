use anyhow::{bail, Context, Result};
use atelier_records::activity::{
    allocate_activity_id, list_issue_activities, write_record_activity, ActivityEventType,
    IssueActivity,
};
use atelier_records::mission_plan_review::{
    load_mission_plan_review_cutover_manifest, mission_graph_revision, project_mission_plan_review,
    validate_mission_plan_review_event_references, validate_stable_actor_identity,
    MissionGraphRevision, MissionPlanFindingSeverity, MissionPlanReviewEvent,
};
use atelier_records::RecordStore;
use chrono::{Timelike, Utc};
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MissionPlanReviewMutation {
    Request,
    Rework,
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
    let expected_status = match &mutation {
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
    let activities = list_issue_activities(state_dir, mission_id)?;
    let event = match mutation {
        MissionPlanReviewMutation::Request => MissionPlanReviewEvent::Request {
            graph_revision: revision.clone(),
            authors: vec![authenticated_actor.to_string()],
            material_editors: vec![authenticated_actor.to_string()],
        },
        MissionPlanReviewMutation::Rework => {
            let previous_graph_revision =
                latest_provenance_revision(&activities).ok_or_else(|| {
                    anyhow::anyhow!("mission {mission_id} has no prior review request to rework")
                })?;
            if previous_graph_revision == revision {
                bail!("mission {mission_id} has no unattributed material graph edit to rework");
            }
            MissionPlanReviewEvent::MaterialEditAttribution {
                previous_graph_revision,
                graph_revision: revision.clone(),
                editors: vec![authenticated_actor.to_string()],
            }
        }
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
        } => {
            let target_revision = activities
                .iter()
                .find_map(|activity| match activity.mission_plan_review.as_ref() {
                    Some(MissionPlanReviewEvent::Finding {
                        graph_revision,
                        finding_id,
                        ..
                    }) if finding_id == &target_id => Some(graph_revision.clone()),
                    Some(MissionPlanReviewEvent::ChangeRequest {
                        graph_revision,
                        request_id,
                        ..
                    }) if request_id == &target_id => Some(graph_revision.clone()),
                    _ => None,
                })
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "resolution references unknown mission-plan decision {target_id}"
                    )
                })?;
            MissionPlanReviewEvent::Resolution {
                graph_revision: target_revision,
                target_id,
                disposition,
            }
        }
        MissionPlanReviewMutation::Approve => MissionPlanReviewEvent::Approval {
            graph_revision: revision.clone(),
        },
    };
    event.validate()?;
    validate_mission_plan_review_event_references(state_dir, mission_id, &event)?;

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
    let mut activities = activities;
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

fn latest_provenance_revision(activities: &[IssueActivity]) -> Option<MissionGraphRevision> {
    activities
        .iter()
        .filter_map(|activity| match activity.mission_plan_review.as_ref() {
            Some(MissionPlanReviewEvent::Request { graph_revision, .. })
            | Some(MissionPlanReviewEvent::MaterialEditAttribution { graph_revision, .. }) => {
                Some((activity.created_at, activity.id.as_str(), graph_revision))
            }
            _ => None,
        })
        .max_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(right.1)))
        .map(|(_, _, revision)| revision.clone())
}

fn mutation_summary(event: &MissionPlanReviewEvent) -> &'static str {
    match event {
        MissionPlanReviewEvent::Request { .. } => "Requested independent mission-plan review",
        MissionPlanReviewEvent::MaterialEditAttribution { .. } => {
            "Attributed material edits and resubmitted mission-plan review"
        }
        MissionPlanReviewEvent::Finding { .. } => "Recorded mission-plan review finding",
        MissionPlanReviewEvent::ChangeRequest { .. } => "Requested mission-plan changes",
        MissionPlanReviewEvent::Resolution { .. } => "Resolved mission-plan review decision",
        MissionPlanReviewEvent::Approval { .. } => "Approved exact mission graph",
        MissionPlanReviewEvent::LegacyGrandfather { .. } => "Recorded mission-plan review event",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_core::Issue;
    use atelier_records::mission_plan_review::validate_mission_plan_reviews;
    use atelier_records::{CanonicalIssueRecord, IssueSections, Relationships};
    use chrono::TimeZone;
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    const PLANNER: &str = "actor-v1:tests.atelier.local/planner";
    const REVIEWER: &str = "actor-v1:tests.atelier.local/reviewer";

    fn write_issue(state_dir: &Path, id: &str, issue_type: &str, status: &str) {
        let timestamp = Utc.with_ymd_and_hms(2026, 7, 16, 12, 0, 0).unwrap();
        let record = CanonicalIssueRecord {
            issue: Issue {
                id: id.to_string(),
                title: id.to_string(),
                description: None,
                status: status.to_string(),
                issue_type: issue_type.to_string(),
                priority: "high".to_string(),
                fields: BTreeMap::new(),
                parent_id: None,
                created_at: timestamp,
                updated_at: timestamp,
                closed_at: None,
            },
            labels: Vec::new(),
            sections: IssueSections::unchecked_from_body(Some(
                "## Description\n\nFixture\n\n## Outcome\n\nReviewed result\n\n## Evidence\n\nCloseout",
            )),
            relationships: Relationships::default(),
        };
        RecordStore::new(state_dir)
            .write_issue_atomic(&record)
            .unwrap();
    }

    #[test]
    fn invalid_graph_references_fail_before_append_and_leave_rebuild_valid() {
        let directory = tempdir().unwrap();
        let state_dir = directory.path().join(".atelier");
        std::fs::create_dir_all(&state_dir).unwrap();
        write_issue(&state_dir, "atelier-m100", "mission", "draft");
        write_issue(&state_dir, "atelier-x100", "task", "todo");
        mutate(
            &state_dir,
            "atelier-m100",
            PLANNER,
            MissionPlanReviewMutation::Request,
        )
        .unwrap();
        let store = RecordStore::new(&state_dir);
        let mut mission = store.load_issue_by_id("atelier-m100").unwrap();
        mission.issue.status = "plan_review".to_string();
        store.write_issue_atomic(&mission).unwrap();
        let baseline = list_issue_activities(&state_dir, "atelier-m100")
            .unwrap()
            .len();

        for mutation in [
            MissionPlanReviewMutation::Finding {
                finding_id: "missing".to_string(),
                severity: MissionPlanFindingSeverity::Blocking,
                affected_issue_ids: vec!["atelier-does-not-exist".to_string()],
                dependency_path: Vec::new(),
            },
            MissionPlanReviewMutation::ChangeRequest {
                request_id: "missing-path".to_string(),
                affected_issue_ids: vec!["atelier-m100".to_string()],
                dependency_path: vec![
                    "atelier-m100".to_string(),
                    "atelier-does-not-exist".to_string(),
                ],
            },
            MissionPlanReviewMutation::Finding {
                finding_id: "outside".to_string(),
                severity: MissionPlanFindingSeverity::Blocking,
                affected_issue_ids: vec!["atelier-x100".to_string()],
                dependency_path: Vec::new(),
            },
        ] {
            let error = mutate(&state_dir, "atelier-m100", REVIEWER, mutation)
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("outside the current mission graph"),
                "{error}"
            );
            assert_eq!(
                list_issue_activities(&state_dir, "atelier-m100")
                    .unwrap()
                    .len(),
                baseline
            );
            validate_mission_plan_reviews(&state_dir).unwrap();
        }
    }
}
