use anyhow::Result;

use crate::objective_graph::dependency_closure;
use crate::record_index::RecordIndex;
use crate::workflow_policy::WorkflowPolicy;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MissionReadinessDiagnosis {
    pub summary: String,
    pub next_command: String,
}

pub fn dependency_diagnosis(
    index: &RecordIndex,
    policy: &WorkflowPolicy,
    issue_id: &str,
) -> Result<Option<MissionReadinessDiagnosis>> {
    let closure = dependency_closure(index, policy, issue_id)?;
    let Some(path) = closure
        .open_paths
        .iter()
        .chain(closure.cycle_paths.iter())
        .max_by_key(|path| path.len())
    else {
        return Ok(None);
    };
    let terminal = path.last().cloned().unwrap_or_else(|| issue_id.to_string());
    let label = if path.len() <= 2 {
        "direct blocker"
    } else {
        "transitive blocker"
    };
    let record_type = index
        .get_issue(issue_id)
        .map_or_else(|| "issue".to_string(), |record| record.issue_type.clone());
    Ok(Some(MissionReadinessDiagnosis {
        summary: format!(
            "{record_type} {issue_id} has {label} path {}",
            path.join(" -> ")
        ),
        next_command: format!("atelier issue show {terminal}"),
    }))
}
