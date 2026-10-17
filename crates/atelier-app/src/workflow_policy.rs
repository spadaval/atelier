use anyhow::{anyhow, Result};

use atelier_core::{InvalidValue, Issue};

use crate::record_index::RecordIndex;

pub use atelier_workflow::{
    configured_initial_status, validate_issue_against_policy, BranchLifecycleConfig, MergeStrategy,
    StatusDefinition, TransitionDefinition, ValidatorDefinition, ValidatorParams,
    WorkflowDefinition, WorkflowPolicy, STARTER_POLICY_YAML, WORKFLOW_BRANCH_FIELD,
    WORKFLOW_POLICY_PATH,
};

/// An issue type argument the workflow does not declare.
pub fn invalid_issue_type(policy: &WorkflowPolicy, issue_type: &str) -> InvalidValue {
    let valid = policy.issue_types.keys().collect::<Vec<_>>();
    InvalidValue::new("issue type", issue_type, &valid).with_hint(&format!(
        "Issue types are declared under issue_types in {WORKFLOW_POLICY_PATH}."
    ))
}

/// Statuses new work may be created in: the workflow's initial status, or a
/// todo-category status one unchecked transition away from it, such as
/// `backlog` via `defer`. A status guarded by checks or required fields (an
/// objective's `ready`) or outside the todo category needs its transition.
/// `issue create --status` and bundles share this rule (atelier-ha6k,
/// atelier-9co2).
pub fn creatable_statuses(
    policy: &WorkflowPolicy,
    issue_type: &str,
) -> Result<std::collections::BTreeSet<String>> {
    let workflow = policy.workflow_for_issue_type(issue_type)?;
    Ok(std::iter::once(workflow.initial_status.clone())
        .chain(
            workflow
                .transitions
                .values()
                .filter(|transition| {
                    transition.from.contains(&workflow.initial_status)
                        && transition.validators.is_empty()
                        && transition.required_fields.is_empty()
                        && policy.status_category(&transition.to) == Some("todo")
                        && policy
                            .transitions_from_status(issue_type, &transition.to)
                            .is_ok_and(|exits| !exits.is_empty())
                })
                .map(|transition| transition.to.clone()),
        )
        .collect())
}

/// `advances` joins a mission to work in its scope. Either endpoint may
/// declare it, and only the declaring record stores it; exactly one must be a
/// mission.
pub fn validate_advances_endpoints(
    source: &str,
    source_type: &str,
    target_type: &str,
) -> Result<()> {
    if (source_type == "mission") == (target_type == "mission") {
        // Only the source is known to be wanted; the other endpoint must change.
        let example = if source_type == "mission" {
            format!("atelier issue link <issue-id> {source} --role advances")
        } else {
            format!("atelier issue link {source} <mission-id> --role advances")
        };
        return Err(anyhow!(
            "workflow_issue_link_type_invalid: {source} ({source_type}) cannot advance {}; advances links one mission to one non-mission issue, e.g. `{example}`",
            crate::record_ref::with_article(target_type)
        ));
    }
    Ok(())
}

pub fn validate_issue_hierarchy(
    index: &RecordIndex,
    issue: &Issue,
    parent_id: Option<&str>,
) -> Result<()> {
    // An issue being created has no ID to link yet: the hint creates it
    // without the parent first.
    let (link_id, create_first) = if index.get_issue(&issue.id).is_some() {
        (issue.id.as_str(), "")
    } else {
        ("<new-id>", "create it without --parent, then ")
    };
    if issue.issue_type == "mission" {
        if let Some(parent_id) = parent_id {
            // A mission parent cannot advance a mission either, so only
            // non-mission work fills the source slot.
            let work = match index.get_issue(parent_id) {
                Some(parent) if parent.issue_type != "mission" => parent_id,
                _ => "<issue-id>",
            };
            return Err(anyhow!(
                "workflow_issue_hierarchy_invalid: mission issue {mission} cannot have parent {parent_id}; {create_first}link mission work with `atelier issue link {work} {link_id} --role advances`",
                mission = issue.id,
            ));
        }
    }
    if issue.issue_type == "mission" {
        let children = index.get_subissues(&issue.id);
        if !children.is_empty() {
            return Err(anyhow!(
                "workflow_issue_hierarchy_invalid: mission {} cannot have children ({}); missions are not parents, so link the work with advances",
                issue.id,
                children.iter().map(|child| child.id.as_str()).collect::<Vec<_>>().join(", ")
            ));
        }
    }
    // One level: any non-mission issue may be a parent, and a parent's
    // children are leaves (atelier-9vq3).
    if let Some(parent_id) = parent_id {
        let parent = index.get_issue(parent_id).ok_or_else(|| {
            anyhow!(
                "workflow_issue_hierarchy_invalid: issue {} references missing parent {}",
                issue.id,
                parent_id
            )
        })?;
        if parent.issue_type == "mission" {
            return Err(anyhow!(
                "workflow_issue_hierarchy_invalid: issue {child} cannot be child of mission {mission}; {create_first}link mission work with `atelier issue link {link_id} {mission} --role advances`",
                child = issue.id,
                mission = parent.id,
            ));
        }
        if let Some(grandparent) = &parent.parent_id {
            return Err(anyhow!(
                "workflow_issue_hierarchy_invalid: issue {} cannot be child of {}, which is itself a child of {}; parents are one level deep",
                issue.id,
                parent.id,
                grandparent
            ));
        }
        let children = index.get_subissues(&issue.id);
        if !children.is_empty() {
            return Err(anyhow!(
                "workflow_issue_hierarchy_invalid: issue {} has children ({}) and cannot also have parent {}; parents are one level deep",
                issue.id,
                children
                    .iter()
                    .map(|child| child.id.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                parent_id
            ));
        }
    }
    Ok(())
}

/// The branch that holds an issue's work, derived from its branch owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkBranch {
    /// The parent or standalone issue that owns the branch.
    pub owner_id: String,
    pub branch: String,
    /// The base recorded by `atelier start` on the owner, if it was started.
    pub recorded_base: Option<String>,
}

/// Resolves the branch owner and branch for an issue: a child works on its
/// parent's branch, and a parent or standalone issue owns `<issue_type>/<id>`. A branch recorded on the owner
/// by `atelier start` wins over the derived name. Missions have no branch.
pub fn resolve_work_branch(
    policy: &WorkflowPolicy,
    index: &RecordIndex,
    issue_id: &str,
) -> Result<WorkBranch> {
    let issue = index.require_issue(issue_id)?;
    if issue.issue_type == "mission" {
        return Err(anyhow!(
            "workflow_branch_mission: {} is a mission; missions are coordinated, not worked on a branch. Start one of its issues instead.",
            issue.id
        ));
    }
    if issue.issue_type == "decision" {
        return Err(anyhow!(
            "workflow_branch_decision: {} is a decision; decisions need no branch. Record the Decision and run `atelier issue transition {} decide` on the base branch.",
            issue.id,
            issue.id
        ));
    }
    // Loaded graphs already guarantee that any parent is a direct, one-level owner.
    let owner = index.require_issue(issue.parent_id.as_deref().unwrap_or(issue_id))?;
    let (branch, recorded_base) = match &owner.workflow_branch {
        Some(branch) => (branch.work_branch.clone(), Some(branch.branch_base.clone())),
        None => (policy.branch_name_for_owner(&owner)?, None),
    };
    Ok(WorkBranch {
        owner_id: owner.id,
        branch,
        recorded_base,
    })
}

pub fn load(repo: &std::path::Path) -> anyhow::Result<WorkflowPolicy> {
    Ok(crate::base_policy::working(repo)?.policy)
}

/// Guards for started work use the committed integration base's policy.
/// Unstarted planning records retain ordinary working-policy semantics.
pub fn action_policy(
    repo: &std::path::Path,
    index: &RecordIndex,
    issue_id: &str,
) -> anyhow::Result<crate::base_policy::PolicySource> {
    let issue = index.require_issue(issue_id)?;
    let owner = index.require_issue(issue.parent_id.as_deref().unwrap_or(issue_id))?;
    if let Some(branch) = &owner.workflow_branch {
        crate::base_policy::from_ref(repo, &format!("refs/heads/{}", branch.branch_base))
    } else {
        crate::base_policy::working(repo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_records::Records;
    use std::collections::BTreeMap;

    fn policy() -> WorkflowPolicy {
        WorkflowPolicy {
            merge_checks: Vec::new(),
            schema_version: 3,
            branch_policy: BranchLifecycleConfig::default(),
            issue_types: BTreeMap::new(),
            workflow_by_issue_type: BTreeMap::new(),
            statuses: BTreeMap::new(),
            workflows: BTreeMap::new(),
        }
    }

    #[test]
    fn advances_between_non_missions_gives_the_target_type_its_article() {
        let error = validate_advances_endpoints("atelier-a1b2", "task", "incident")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("atelier-a1b2 (task) cannot advance an incident;"),
            "{error}"
        );
    }

    #[test]
    fn child_works_on_its_parent_branch() {
        let mut records = Records::default();
        records.issue("atelier-epic", "feature", "todo");
        records.issue("atelier-child", "task", "todo");
        records.parent("atelier-child", "atelier-epic");

        let work_branch =
            resolve_work_branch(&policy(), &records.index(), "atelier-child").unwrap();

        assert_eq!(work_branch.owner_id, "atelier-epic");
        assert_eq!(work_branch.branch, "feature/atelier-epic");
        assert_eq!(work_branch.recorded_base, None);
    }

    #[test]
    fn standalone_issue_and_parent_own_their_branches() {
        let mut records = Records::default();
        records.issue("atelier-bug1", "bug", "todo");
        records.issue("atelier-task", "task", "todo");
        records.issue("atelier-epic", "feature", "todo");
        let index = records.index();

        let bug = resolve_work_branch(&policy(), &index, "atelier-bug1").unwrap();
        let epic = resolve_work_branch(&policy(), &index, "atelier-epic").unwrap();

        assert_eq!(
            (bug.owner_id.as_str(), bug.branch.as_str()),
            ("atelier-bug1", "bug/atelier-bug1")
        );
        let task = resolve_work_branch(&policy(), &index, "atelier-task").unwrap();
        assert_eq!(task.owner_id, "atelier-task");
        assert_eq!(task.branch, "task/atelier-task");
        assert_eq!(
            (epic.owner_id.as_str(), epic.branch.as_str()),
            ("atelier-epic", "feature/atelier-epic")
        );
    }

    #[test]
    fn recorded_owner_branch_wins_over_derived_name() {
        let mut records = Records::default();
        records
            .issue("atelier-epic", "feature", "todo")
            .issue
            .workflow_branch = Some(atelier_core::WorkflowBranch {
            work_branch: "feature/renamed".to_string(),
            branch_base: "trunk".to_string(),
        });
        records.issue("atelier-child", "task", "todo");
        records.parent("atelier-child", "atelier-epic");

        let work_branch =
            resolve_work_branch(&policy(), &records.index(), "atelier-child").unwrap();

        assert_eq!(work_branch.branch, "feature/renamed");
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

#[cfg(test)]
mod hierarchy_contract_tests {
    use super::*;
    use crate::test_records::{issue_record, link};

    #[test]
    fn loaded_graph_rejects_mission_parents_and_missing_hierarchy_targets() {
        let mut parent = issue_record("atelier-parent", "mission", "todo");
        parent.relationships.children.push(link("atelier-child"));
        let child = issue_record("atelier-child", "task", "todo");
        let error = RecordIndex::from_records(vec![parent, child], Vec::new(), Vec::new())
            .unwrap_err()
            .to_string();
        assert!(error.contains("cannot be child of mission"), "{error}");
        let mut parent = issue_record("atelier-parent", "feature", "todo");
        parent.relationships.children.push(link("atelier-missing"));
        let error = RecordIndex::from_records(vec![parent], Vec::new(), Vec::new())
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("atelier-parent") && error.contains("missing issue atelier-missing"),
            "{error}"
        );
    }

    #[test]
    fn assigning_missing_parent_retains_actionable_error() {
        let child = issue_record("atelier-child", "task", "todo");
        let index = RecordIndex::from_records(vec![child.clone()], Vec::new(), Vec::new()).unwrap();
        let error = validate_issue_hierarchy(&index, &child.issue, Some("atelier-missing"))
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("issue atelier-child references missing parent atelier-missing"),
            "{error}"
        );
    }
}
