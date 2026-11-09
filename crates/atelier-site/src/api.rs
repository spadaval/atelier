//! The JSON the API returns (atelier-zpmb). Each body is an `atelier-app`
//! view model as the CLI builds it, plus the base commit it was read from
//! and links; the server decides nothing about work except which review a
//! mission opens on, read from the workflow policy. TypeScript types for
//! these are generated with `ts-rs` into `web/src/api/`.

use atelier_app::issue_read::ObjectiveBucketTotals;
use atelier_app::mission_review::{Brief, Debrief};
use atelier_app::workflow_policy::WorkflowPolicy;
use atelier_core::Issue;
use serde::Serialize;
use ts_rs::TS;

/// `GET /api/repos`.
#[derive(Debug, Clone, Serialize, TS)]
pub struct ReposResponse {
    pub repos: Vec<RepoSummary>,
}

/// One repository: its base and what its records hold, or why they cannot
/// be read.
#[derive(Debug, Clone, Serialize, TS)]
pub struct RepoSummary {
    /// The name in its URLs: its `project_slug`, else its directory name; a
    /// second repository with the same name gets `-2`, and so on.
    pub repo: String,
    pub base_branch: Option<String>,
    pub base_commit: Option<String>,
    /// Why the base or the records could not be read; the repository has no
    /// counts or missions then.
    pub error: Option<String>,
    pub counts: Option<RepoCounts>,
    pub missions: Vec<MissionSummary>,
}

/// What `atelier status` counts on the base branch.
#[derive(Debug, Clone, Serialize, TS)]
pub struct RepoCounts {
    pub work: ObjectiveBucketTotals,
    pub current_missions: usize,
}

/// Which review a mission's page opens on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum Review {
    /// The mission is in its workflow's initial status: its plan is
    /// reviewed.
    Brief,
    /// It has left its initial status: what it did is reviewed.
    Debrief,
}

impl Review {
    /// The brief while the mission is in its workflow's initial status, the
    /// debrief afterwards.
    pub fn for_mission(policy: &WorkflowPolicy, mission: &Issue) -> Self {
        match policy.workflow_for_issue_type(&mission.issue_type) {
            Ok(workflow) if workflow.initial_status != mission.status => Self::Debrief,
            _ => Self::Brief,
        }
    }
}

/// One mission, as `/api/repos` lists it and `/api/<repo>/mission/<id>`
/// returns it.
#[derive(Debug, Clone, Serialize, TS)]
pub struct MissionSummary {
    pub id: String,
    pub title: String,
    pub status: String,
    pub status_category: Option<String>,
    pub review: Review,
    pub links: MissionLinks,
}

/// The API routes for one mission.
#[derive(Debug, Clone, Serialize, TS)]
pub struct MissionLinks {
    pub summary: String,
    pub brief: String,
    pub debrief: String,
    /// `None` when the mission links no strategy.
    pub strategy: Option<String>,
}

/// `GET /api/<repo>/mission/<id>`.
#[derive(Debug, Clone, Serialize, TS)]
pub struct MissionResponse {
    pub repo: String,
    pub base_commit: String,
    pub mission: MissionSummary,
}

/// `GET /api/<repo>/mission/<id>/brief`: the view `atelier brief` prints.
#[derive(Debug, Clone, Serialize, TS)]
pub struct BriefResponse {
    pub base_commit: String,
    pub brief: Brief,
}

/// `GET /api/<repo>/mission/<id>/debrief`: the view `atelier debrief`
/// prints.
#[derive(Debug, Clone, Serialize, TS)]
pub struct DebriefResponse {
    pub base_commit: String,
    pub debrief: Debrief,
}

/// `GET /api/<repo>/mission/<id>/strategy`: the strategy as committed.
#[derive(Debug, Clone, Serialize, TS)]
pub struct StrategyResponse {
    pub base_commit: String,
    pub mission: String,
    pub path: String,
    pub markdown: String,
}

/// The body of every response that is not a 200.
#[derive(Debug, Clone, Serialize, TS)]
pub struct ApiError {
    pub error: String,
}

/// Writes the TypeScript type of every response, and of everything they
/// hold, into `dir`, one file per type.
pub fn export_types(dir: &std::path::Path) -> Result<(), ts_rs::ExportError> {
    let config = ts_rs::Config::new().with_out_dir(dir);
    ReposResponse::export_all(&config)?;
    MissionResponse::export_all(&config)?;
    BriefResponse::export_all(&config)?;
    DebriefResponse::export_all(&config)?;
    StrategyResponse::export_all(&config)?;
    ApiError::export_all(&config)?;
    Ok(())
}
