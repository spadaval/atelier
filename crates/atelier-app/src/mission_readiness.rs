use anyhow::Result;
use atelier_records::mission_plan_review::{
    mission_plan_review_state, MissionPlanFindingSeverity, MissionPlanReviewFreshness,
};
use atelier_sqlite::Database;
use std::path::Path;

use crate::objective_graph::dependency_closure;
use crate::workflow_policy::WorkflowPolicy;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MissionReadinessKind {
    ReviewNotRequested,
    ApprovalMissing,
    BlockingFinding,
    ChangeRequest,
    StaleApproval,
    DirectBlocker,
    TransitiveBlocker,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MissionReadinessDiagnosis {
    pub kind: MissionReadinessKind,
    pub mission_id: String,
    pub summary: String,
    pub next_command: String,
}

impl MissionReadinessDiagnosis {
    pub fn normal_lines(&self) -> [String; 2] {
        [
            format!("Readiness: {}", self.summary),
            format!("Next: {}", self.next_command),
        ]
    }
}

pub fn review_diagnosis(
    state_dir: &Path,
    mission_id: &str,
) -> Result<Option<MissionReadinessDiagnosis>> {
    let state = mission_plan_review_state(state_dir, mission_id)?;
    let revision = &state.current_graph_revision;
    let diagnosis = match state.freshness {
        MissionPlanReviewFreshness::FreshApproval
        | MissionPlanReviewFreshness::FreshGrandfather => None,
        MissionPlanReviewFreshness::ProvenanceIncomplete
            if state.authors.is_empty()
                && state.material_editors.is_empty()
                && state.authorization.is_none() =>
        {
            Some(MissionReadinessDiagnosis {
                kind: MissionReadinessKind::ReviewNotRequested,
                mission_id: mission_id.to_string(),
                summary: format!(
                    "mission {mission_id}: mission-plan review was not requested for graph {revision}"
                ),
                next_command: format!("atelier issue plan-review {mission_id} request"),
            })
        }
        MissionPlanReviewFreshness::ProvenanceIncomplete => Some(MissionReadinessDiagnosis {
            kind: MissionReadinessKind::ReviewNotRequested,
            mission_id: mission_id.to_string(),
            summary: format!(
                "mission {mission_id} review provenance is incomplete for graph {revision}"
            ),
            next_command: format!("atelier issue plan-review {mission_id} request"),
        }),
        MissionPlanReviewFreshness::Unapproved => Some(MissionReadinessDiagnosis {
            kind: MissionReadinessKind::ApprovalMissing,
            mission_id: mission_id.to_string(),
            summary: format!(
                "mission {mission_id} graph {revision} has no independent approval"
            ),
            next_command: format!("atelier issue plan-review {mission_id} approve"),
        }),
        MissionPlanReviewFreshness::Stale => Some(MissionReadinessDiagnosis {
            kind: MissionReadinessKind::StaleApproval,
            mission_id: mission_id.to_string(),
            summary: format!(
                "mission {mission_id}: mission-plan approval is stale for current graph {revision}"
            ),
            next_command: format!("atelier issue plan-review {mission_id} rework"),
        }),
        MissionPlanReviewFreshness::BlockedByReview => {
            if let Some(finding) = state.findings.iter().find(|finding| {
                finding.graph_revision == *revision
                    && finding.severity == MissionPlanFindingSeverity::Blocking
                    && finding.resolution.is_none()
            }) {
                let scope = decision_scope(&finding.dependency_path, &finding.affected_issue_ids);
                Some(MissionReadinessDiagnosis {
                    kind: MissionReadinessKind::BlockingFinding,
                    mission_id: mission_id.to_string(),
                    summary: format!(
                        "mission {mission_id} has unresolved blocking findings: {}{scope}",
                        finding.id
                    ),
                    next_command: format!(
                        "atelier issue plan-review {mission_id} resolve {} --disposition \"<resolution>\"",
                        finding.id
                    ),
                })
            } else if let Some(request) = state
                .change_requests
                .iter()
                .find(|request| request.graph_revision == *revision && request.resolution.is_none())
            {
                let scope = decision_scope(&request.dependency_path, &request.affected_issue_ids);
                Some(MissionReadinessDiagnosis {
                    kind: MissionReadinessKind::ChangeRequest,
                    mission_id: mission_id.to_string(),
                    summary: format!(
                        "mission {mission_id} has unresolved change requests: {}; rework required{scope}",
                        request.id
                    ),
                    next_command: format!(
                        "atelier issue plan-review {mission_id} resolve {} --disposition \"<resolution>\"",
                        request.id
                    ),
                })
            } else {
                None
            }
        }
    };
    Ok(diagnosis)
}

pub fn dependency_diagnosis(
    db: &Database,
    policy: &WorkflowPolicy,
    mission_id: &str,
) -> Result<Option<MissionReadinessDiagnosis>> {
    let closure = dependency_closure(db, policy, mission_id)?;
    let Some(path) = closure
        .open_paths
        .iter()
        .chain(closure.cycle_paths.iter())
        .max_by_key(|path| path.len())
    else {
        return Ok(None);
    };
    let terminal = path
        .last()
        .cloned()
        .unwrap_or_else(|| mission_id.to_string());
    let kind = if path.len() <= 2 {
        MissionReadinessKind::DirectBlocker
    } else {
        MissionReadinessKind::TransitiveBlocker
    };
    let label = if kind == MissionReadinessKind::DirectBlocker {
        "direct blocker"
    } else {
        "transitive blocker"
    };
    Ok(Some(MissionReadinessDiagnosis {
        kind,
        mission_id: mission_id.to_string(),
        summary: format!(
            "mission {mission_id} has {label} path {}",
            path.join(" -> ")
        ),
        next_command: format!("atelier issue show {terminal}"),
    }))
}

pub fn mission_diagnosis(
    db: &Database,
    state_dir: &Path,
    policy: &WorkflowPolicy,
    mission_id: &str,
) -> Result<Option<MissionReadinessDiagnosis>> {
    if let Some(review) = review_diagnosis(state_dir, mission_id)? {
        return Ok(Some(review));
    }
    dependency_diagnosis(db, policy, mission_id)
}

fn decision_scope(path: &[String], affected: &[String]) -> String {
    if !path.is_empty() {
        format!(" on path {}", path.join(" -> "))
    } else if !affected.is_empty() {
        format!(" affecting {}", affected.join(", "))
    } else {
        String::new()
    }
}
