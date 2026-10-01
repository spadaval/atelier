use anyhow::Result;

use crate::objective_graph::dependency_closure;
use crate::record_index::RecordIndex;
use crate::workflow_policy::WorkflowPolicy;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MissionReadinessKind {
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

pub fn dependency_diagnosis(
    index: &RecordIndex,
    policy: &WorkflowPolicy,
    mission_id: &str,
) -> Result<Option<MissionReadinessDiagnosis>> {
    let closure = dependency_closure(index, policy, mission_id)?;
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
