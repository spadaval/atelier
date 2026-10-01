use anyhow::{anyhow, Result};
use std::collections::BTreeSet;

use atelier_core::Issue;

use crate::record_index::RecordIndex;

pub use atelier_workflow::{
    configured_initial_status, load, validate_issue_against_policy, BranchLifecycleConfig,
    MergeStrategy, StatusDefinition, TransitionDefinition, ValidatorDefinition, ValidatorParams,
    WorkflowDefinition, WorkflowPolicy, STARTER_POLICY_YAML, WORKFLOW_BRANCH_FIELD,
    WORKFLOW_POLICY_PATH,
};

pub fn validate_issue_hierarchy(
    index: &RecordIndex,
    issue: &Issue,
    parent_id: Option<&str>,
) -> Result<()> {
    if issue.issue_type == "mission" {
        if let Some(parent_id) = parent_id {
            return Err(anyhow!(
                "workflow_issue_hierarchy_invalid: mission issue {} cannot have parent {}; link mission work with `atelier issue link <mission-id> <issue-id> --role advances`",
                issue.id,
                parent_id
            ));
        }
    }
    if issue.issue_type == "epic" {
        if let Some(parent_id) = parent_id {
            return Err(anyhow!(
                "workflow_issue_hierarchy_invalid: epic issue {} cannot have parent {}; epics are root work packages",
                issue.id,
                parent_id
            ));
        }
    }
    if let Some(parent_id) = parent_id {
        let parent = index.get_issue(parent_id).ok_or_else(|| {
            anyhow!(
                "workflow_issue_hierarchy_invalid: issue {} references missing parent {}",
                issue.id,
                parent_id
            )
        })?;
        if parent.issue_type != "epic" {
            return Err(anyhow!(
                "workflow_issue_hierarchy_invalid: issue {} cannot be child of {} {}; only epics can own child work",
                issue.id,
                parent.issue_type,
                parent.id
            ));
        }
    }
    let children = index.get_subissues(&issue.id);
    if !children.is_empty() && issue.issue_type != "epic" {
        let child_ids = children
            .iter()
            .map(|child| child.id.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(anyhow!(
            "workflow_issue_hierarchy_invalid: {} issue {} cannot own child work {}; only epics can own child work",
            issue.issue_type,
            issue.id,
            child_ids
        ));
    }
    Ok(())
}

/// The branch that holds an issue's work, derived from its branch owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkBranch {
    /// The epic or standalone issue that owns the branch.
    pub owner_id: String,
    pub branch: String,
    /// The base recorded by `atelier start` on the owner, if it was started.
    pub recorded_base: Option<String>,
}

/// Resolves the branch owner and branch for an issue: a child issue works on
/// its nearest parent epic's branch, an epic owns `epic/<id>`, and a
/// standalone issue owns `<issue_type>/<id>`. A branch recorded on the owner
/// by `atelier start` wins over the derived name. Missions have no branch.
pub fn resolve_work_branch(
    policy: &WorkflowPolicy,
    index: &RecordIndex,
    issue_id: &str,
) -> Result<WorkBranch> {
    let issue = index.require_issue(issue_id)?;
    if issue.issue_type == "mission" {
        return Err(anyhow!(
            "workflow_branch_mission: {} is a mission; missions are coordinated, not worked on a branch. Start one of its epics or issues instead.",
            issue.id
        ));
    }
    let owner = if issue.issue_type == "epic" || issue.parent_id.is_none() {
        issue
    } else {
        nearest_parent_epic(index, &issue)?
    };
    let (branch, recorded_base) = match recorded_workflow_branch(&owner)? {
        Some((branch, base)) => (branch, Some(base)),
        None => (policy.branch_name_for_owner(&owner)?, None),
    };
    Ok(WorkBranch {
        owner_id: owner.id,
        branch,
        recorded_base,
    })
}

/// Returns the `(work_branch, branch_base)` pair recorded on a branch owner.
fn recorded_workflow_branch(issue: &Issue) -> Result<Option<(String, String)>> {
    Ok(issue
        .workflow_branch
        .as_ref()
        .map(|branch| (branch.work_branch.clone(), branch.branch_base.clone())))
}

fn nearest_parent_epic(index: &RecordIndex, issue: &Issue) -> Result<Issue> {
    let mut parent_id = issue.parent_id.clone();
    let mut seen = BTreeSet::new();
    let mut parent_chain = Vec::new();
    while let Some(current_id) = parent_id {
        if !seen.insert(current_id.clone()) {
            return Err(anyhow!(
                "workflow_branch_policy_invalid_graph: issue {} has a cyclic parent graph while resolving branch owner",
                issue.id
            ));
        }
        let parent = index.get_issue(&current_id).ok_or_else(|| {
            anyhow!(
                "workflow_branch_policy_invalid_graph: issue {} references missing parent issue {}",
                issue.id,
                current_id
            )
        })?;
        parent_chain.push((parent.id.clone(), parent.issue_type.clone()));
        if parent.issue_type == "epic" {
            return Ok(parent);
        }
        parent_id = parent.parent_id;
    }
    Err(missing_parent_epic_branch_owner_error(issue, &parent_chain))
}

fn missing_parent_epic_branch_owner_error(
    issue: &Issue,
    parent_chain: &[(String, String)],
) -> anyhow::Error {
    let parent_chain = if parent_chain.is_empty() {
        "(none)".to_string()
    } else {
        parent_chain
            .iter()
            .map(|(id, issue_type)| format!("{id} ({issue_type})"))
            .collect::<Vec<_>>()
            .join(" -> ")
    };
    let mission_link_guidance = parent_chain
        .split(" -> ")
        .find(|entry| entry.ends_with(" (mission)"))
        .map(|entry| {
            let mission_id = entry.split_once(' ').map(|(id, _)| id).unwrap_or(entry);
            format!(
                "\nIf this work should stay directly under the mission, make it standalone and link it to the mission instead:\n  atelier issue update {} --no-parent\n  atelier issue link {} {}",
                issue.id, mission_id, issue.id
            )
        })
        .unwrap_or_default();
    anyhow!(
        "workflow_branch_policy_invalid_graph: issue {} is nested but has no parent epic branch owner\nParent chain: {}\nBranch policy requires nested issues to inherit branch ownership from a parent epic. Standalone issues own their own issue branch.\nFix: move {} under an epic with:\n  atelier issue update {} --parent <epic-id>{}",
        issue.id,
        parent_chain,
        issue.id,
        issue.id,
        mission_link_guidance
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_records::Records;
    use std::collections::BTreeMap;

    fn policy() -> WorkflowPolicy {
        WorkflowPolicy {
            schema_version: 3,
            branch_policy: BranchLifecycleConfig::default(),
            issue_types: BTreeMap::new(),
            workflow_by_issue_type: BTreeMap::new(),
            statuses: BTreeMap::new(),
            workflows: BTreeMap::new(),
        }
    }

    #[test]
    fn child_works_on_parent_epic_branch() {
        let mut records = Records::default();
        records.issue("atelier-epic", "epic", "todo");
        records.issue("atelier-child", "task", "todo");
        records.parent("atelier-child", "atelier-epic");

        let work_branch =
            resolve_work_branch(&policy(), &records.index(), "atelier-child").unwrap();

        assert_eq!(work_branch.owner_id, "atelier-epic");
        assert_eq!(work_branch.branch, "epic/atelier-epic");
        assert_eq!(work_branch.recorded_base, None);
    }

    #[test]
    fn standalone_issue_and_epic_own_their_branches() {
        let mut records = Records::default();
        records.issue("atelier-bug1", "bug", "todo");
        records.issue("atelier-epic", "epic", "todo");
        let index = records.index();

        let bug = resolve_work_branch(&policy(), &index, "atelier-bug1").unwrap();
        let epic = resolve_work_branch(&policy(), &index, "atelier-epic").unwrap();

        assert_eq!(
            (bug.owner_id.as_str(), bug.branch.as_str()),
            ("atelier-bug1", "bug/atelier-bug1")
        );
        assert_eq!(
            (epic.owner_id.as_str(), epic.branch.as_str()),
            ("atelier-epic", "epic/atelier-epic")
        );
    }

    #[test]
    fn recorded_owner_branch_wins_over_derived_name() {
        let mut records = Records::default();
        records
            .issue("atelier-epic", "epic", "todo")
            .issue
            .workflow_branch = Some(atelier_core::WorkflowBranch {
            work_branch: "epic/renamed".to_string(),
            branch_base: "trunk".to_string(),
        });
        records.issue("atelier-child", "task", "todo");
        records.parent("atelier-child", "atelier-epic");

        let work_branch =
            resolve_work_branch(&policy(), &records.index(), "atelier-child").unwrap();

        assert_eq!(work_branch.branch, "epic/renamed");
        assert_eq!(work_branch.recorded_base.as_deref(), Some("trunk"));
    }

    #[test]
    fn missions_have_no_branch() {
        let mut records = Records::default();
        records.issue("atelier-mission", "mission", "todo");

        let error = resolve_work_branch(&policy(), &records.index(), "atelier-mission")
            .unwrap_err()
            .to_string();

        assert!(error.contains("missions are coordinated, not worked on a branch"));
    }
}
