use anyhow::{bail, Result};
use atelier_core::EvidenceRecord;
use atelier_records::IssueSections;
use serde::Serialize;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use crate::record_index::RecordIndex;
use crate::workflow_policy::{ValidatorDefinition, ValidatorParams, WorkflowPolicy};

pub struct ValidatorRequest<'a> {
    pub index: &'a RecordIndex,
    pub repo_root: &'a Path,
    pub policy: &'a WorkflowPolicy,
    pub issue_id: &'a str,
    pub validators: &'a [ValidatorDefinition],
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidatorResult {
    pub validator: String,
    pub passed: bool,
    pub reason: String,
    pub help: Option<String>,
    pub elapsed_ms: u128,
}

/// Runs a transition's read-only validators against one issue.
pub fn evaluate_policy_transition(request: ValidatorRequest<'_>) -> Result<Vec<ValidatorResult>> {
    request.index.require_issue(request.issue_id)?;
    let mut results = Vec::new();
    for definition in request.validators {
        let started = Instant::now();
        let (passed, reason, help) = evaluate_builtin(&request, definition)?;
        results.push(ValidatorResult {
            validator: definition.builtin.clone(),
            passed,
            reason,
            help,
            elapsed_ms: started.elapsed().as_millis(),
        });
    }
    Ok(results)
}

fn evaluate_builtin(
    request: &ValidatorRequest<'_>,
    definition: &ValidatorDefinition,
) -> Result<(bool, String, Option<String>)> {
    let ValidatorRequest {
        index,
        repo_root,
        policy,
        issue_id,
        ..
    } = *request;
    match definition.builtin.as_str() {
        "issue.sections_parseable" => {
            issue_sections_parseable(index, repo_root, issue_id).map(without_validator_help)
        }
        "blockers.none_open" => {
            let open = open_blockers(index, policy, repo_root, issue_id)?;
            if open.is_empty() {
                Ok((true, "no open blockers".to_string(), None))
            } else {
                Ok((false, format!("open blockers: {}", open.join(", ")), None))
            }
        }
        "blockers.transitive_none_open" => transitive_blockers_none_open(index, policy, issue_id),
        "evidence.attached" => evidence_attached(index, issue_id, definition.params.as_ref()),
        "lint.none_blocking" => Ok(without_validator_help(lint_none_blocking(index, repo_root))),
        "git.worktree_clean" => git_worktree_clean(repo_root).map(without_validator_help),
        "work.terminal" => work_terminal(index, policy, issue_id).map(without_validator_help),
        other => bail!("unsupported builtin validator: {other}"),
    }
}

fn without_validator_help((passed, reason): (bool, String)) -> (bool, String, Option<String>) {
    (passed, reason, None)
}

fn evidence_attached(
    index: &RecordIndex,
    issue_id: &str,
    params: Option<&ValidatorParams>,
) -> Result<(bool, String, Option<String>)> {
    if let Some(ValidatorParams::EvidenceAttached { min_count, kind }) = params {
        let validating_count = linked_evidence_records(index, issue_id, kind.as_deref())?.len();
        if validating_count < *min_count as usize {
            return Ok((
                false,
                format!(
                    "expected at least {} validating evidence record(s){}; found {}",
                    min_count,
                    kind.as_deref()
                        .map(|value| format!(" of kind {}", value))
                        .unwrap_or_default(),
                    validating_count
                ),
                Some(evidence_help_hint()),
            ));
        }
    }
    let gate = issue_evidence_gate_status(index, issue_id)?;
    Ok((gate.passed, gate.reason, gate.help))
}

#[derive(Debug, Clone)]
struct EvidenceGateStatus {
    passed: bool,
    reason: String,
    help: Option<String>,
}

fn issue_evidence_gate_status(index: &RecordIndex, issue_id: &str) -> Result<EvidenceGateStatus> {
    let evidence = linked_evidence_records(index, issue_id, None)?;
    if evidence.is_empty() {
        return Ok(EvidenceGateStatus {
            passed: false,
            reason: "no validating evidence link found".to_string(),
            help: Some(evidence_help_hint()),
        });
    }
    let passing = evidence.iter().any(|record| {
        record.data.success.unwrap_or_else(|| {
            !matches!(record.header.status.as_str(), "blocked" | "fail" | "failed")
        })
    });
    if passing {
        Ok(EvidenceGateStatus {
            passed: true,
            reason: "passing validating evidence is linked".to_string(),
            help: None,
        })
    } else {
        Ok(EvidenceGateStatus {
            passed: false,
            reason: "expected at least 1 passing evidence record".to_string(),
            help: Some(evidence_help_hint()),
        })
    }
}

fn evidence_help_hint() -> String {
    "record proof with `atelier evidence record --target issue/<id> --kind validation \"...\"` or attach existing proof with `atelier evidence attach <evidence-id> issue <issue-id>`".to_string()
}

fn linked_evidence_records<'a>(
    index: &'a RecordIndex,
    issue_id: &str,
    required_kind: Option<&str>,
) -> Result<Vec<&'a EvidenceRecord>> {
    let mut records = Vec::new();
    for link in index.list_record_links("issue", issue_id)? {
        if link.relation_type != "validates" {
            continue;
        }
        let evidence_id = if link.source_kind == "evidence" {
            link.source_id
        } else if link.target_kind == "evidence" {
            link.target_id
        } else {
            continue;
        };
        let record = index.require_evidence(&evidence_id)?;
        if required_kind.is_some_and(|kind| record.data.evidence_type != kind) {
            continue;
        }
        records.push(record);
    }
    Ok(records)
}

fn lint_none_blocking(index: &RecordIndex, repo_root: &Path) -> (bool, String) {
    let findings = crate::lint::issue_findings(index.issue_records(), repo_root, false);
    if findings.is_empty() {
        (true, "lint passed".to_string())
    } else {
        (
            false,
            format!("atelier check failed with {} finding(s)", findings.len()),
        )
    }
}

fn issue_sections_parseable(
    index: &RecordIndex,
    repo_root: &Path,
    issue_id: &str,
) -> Result<(bool, String)> {
    let state_dir = crate::storage_layout::StorageLayout::new(repo_root).canonical_dir();
    let record = index
        .issue_record(issue_id)
        .ok_or_else(|| anyhow::anyhow!("Issue {issue_id} not found"))?;
    let invalid = record
        .sections
        .section_states()
        .into_iter()
        .filter(|state| state.required && (!state.present || state.empty))
        .map(|state| state.name.title().to_string())
        .collect::<Vec<_>>();
    if !invalid.is_empty() {
        let path = state_dir.join("issues").join(format!("{issue_id}.md"));
        return Ok((
            false,
            format!(
                "issue {issue_id} has invalid sections {} in {}",
                invalid.join(", "),
                path.display()
            ),
        ));
    }
    Ok((
        true,
        format!(
            "parsed required sections {} are present and non-empty",
            IssueSections::REQUIRED_NAMES
                .into_iter()
                .map(|name| name.title())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    ))
}

/// Passes when every child (recursively) and, for missions, all work linked
/// by `advances` is in a done-category status.
fn work_terminal(
    index: &RecordIndex,
    policy: &WorkflowPolicy,
    issue_id: &str,
) -> Result<(bool, String)> {
    let mut open = Vec::new();
    for id in crate::objective_graph::issue_descendant_ids(index, issue_id)? {
        let issue = index.require_issue(&id)?;
        if policy.status_category(&issue.status) != Some("done") {
            open.push(format!("{} [{}]", issue.id, issue.status));
        }
    }
    if open.is_empty() {
        Ok((true, "all child and advancing work is done".to_string()))
    } else {
        Ok((false, format!("open work: {}", open.join(", "))))
    }
}

fn transitive_blockers_none_open(
    index: &RecordIndex,
    policy: &WorkflowPolicy,
    issue_id: &str,
) -> Result<(bool, String, Option<String>)> {
    match crate::mission_readiness::dependency_diagnosis(index, policy, issue_id)? {
        Some(diagnosis) => Ok((
            false,
            diagnosis.summary,
            Some(format!("Next: {}", diagnosis.next_command)),
        )),
        None => Ok((
            true,
            "declared dependency closure is terminal".to_string(),
            None,
        )),
    }
}

fn open_blockers(
    index: &RecordIndex,
    policy: &WorkflowPolicy,
    repo_root: &Path,
    issue_id: &str,
) -> Result<Vec<String>> {
    let policy_path = repo_root.join(crate::workflow_policy::WORKFLOW_POLICY_PATH);
    let mut open = Vec::new();
    for blocker_id in index.get_blockers(issue_id) {
        let blocker = index.require_issue(&blocker_id)?;
        crate::workflow_policy::validate_issue_against_policy(policy, &blocker, &policy_path)?;
        if policy.status_category(&blocker.status) != Some("done") {
            open.push(blocker.id);
        }
    }
    Ok(open)
}

fn git_worktree_clean(repo_root: &Path) -> Result<(bool, String)> {
    let output = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=all"])
        .current_dir(repo_root)
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.contains("not a git repository") {
            return Ok((
                true,
                "not a git repository; git checkout check skipped".to_string(),
            ));
        }
        let message = if stderr.is_empty() {
            "git status failed".to_string()
        } else {
            format!("git status failed: {stderr}")
        };
        return Ok((false, message));
    }
    let dirty = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    if dirty.is_empty() {
        Ok((true, "git checkout is clean".to_string()))
    } else {
        Ok((
            false,
            format!(
                "git checkout has {} dirty {}: {}",
                dirty.len(),
                if dirty.len() == 1 { "entry" } else { "entries" },
                dirty.into_iter().take(8).collect::<Vec<_>>().join(", ")
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::work_terminal;
    use crate::test_records::Records;
    use crate::workflow_policy::{BranchLifecycleConfig, StatusDefinition, WorkflowPolicy};
    use std::collections::BTreeMap;

    fn policy() -> WorkflowPolicy {
        let statuses = [
            ("todo", "todo"),
            ("in_progress", "active"),
            ("done", "done"),
        ]
        .into_iter()
        .map(|(name, category)| {
            (
                name.to_string(),
                StatusDefinition {
                    category: category.to_string(),
                },
            )
        })
        .collect();
        WorkflowPolicy {
            schema_version: 3,
            branch_policy: BranchLifecycleConfig::default(),
            issue_types: BTreeMap::new(),
            workflow_by_issue_type: BTreeMap::new(),
            statuses,
            workflows: BTreeMap::new(),
        }
    }

    #[test]
    fn work_terminal_requires_every_child_done() {
        let mut records = Records::default();
        records.issue("atelier-epic", "epic", "in_progress");
        records.issue("atelier-done", "task", "done");
        records.issue("atelier-open", "task", "in_progress");
        records.parent("atelier-done", "atelier-epic");
        records.parent("atelier-open", "atelier-epic");

        let (passed, reason) = work_terminal(&records.index(), &policy(), "atelier-epic").unwrap();
        assert!(!passed);
        assert_eq!(reason, "open work: atelier-open [in_progress]");

        records.get("atelier-open").issue.status = "done".to_string();
        let (passed, reason) = work_terminal(&records.index(), &policy(), "atelier-epic").unwrap();
        assert!(passed, "{reason}");
    }

    #[test]
    fn work_terminal_follows_mission_advances_links() {
        let mut records = Records::default();
        records.issue("atelier-mission", "mission", "in_progress");
        records.issue("atelier-epic", "epic", "done");
        records.issue("atelier-child", "task", "todo");
        records.parent("atelier-child", "atelier-epic");
        records.relate("atelier-mission", "atelier-epic", "advances");

        let (passed, reason) =
            work_terminal(&records.index(), &policy(), "atelier-mission").unwrap();
        assert!(!passed);
        assert_eq!(reason, "open work: atelier-child [todo]");

        records.get("atelier-child").issue.status = "done".to_string();
        let (passed, reason) =
            work_terminal(&records.index(), &policy(), "atelier-mission").unwrap();
        assert!(passed, "{reason}");
    }
}
