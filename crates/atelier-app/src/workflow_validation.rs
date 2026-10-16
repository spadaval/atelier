use anyhow::{bail, Result};
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
    let mut read = crate::git_freshness::ReadContext::new(request.repo_root);
    evaluate_with_context(request, true, &mut read)
}
pub(crate) fn evaluate_with_context(
    request: ValidatorRequest<'_>,
    fresh: bool,
    read: &mut crate::git_freshness::ReadContext<'_>,
) -> Result<Vec<ValidatorResult>> {
    request.index.require_issue(request.issue_id)?;
    let mut results = Vec::new();
    for definition in request.validators {
        if !fresh && definition.builtin == "git.worktree_clean" {
            continue;
        }
        let started = Instant::now();
        let (passed, reason, help) = evaluate_builtin(&request, definition, fresh, read)?;
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
    fresh: bool,
    read: &mut crate::git_freshness::ReadContext<'_>,
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
        "evidence.attached" => {
            evidence_attached(index, read, issue_id, definition.params.as_ref(), fresh)
        }
        "lint.none_blocking" => Ok(without_validator_help(lint_none_blocking(index, repo_root))),
        "git.worktree_clean" => git_worktree_clean(repo_root).map(without_validator_help),
        "shell.check" => {
            let Some(ValidatorParams::ShellCheck { command }) = &definition.params else {
                bail!("shell.check requires command");
            };
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(repo_root)
                .output()?;
            *read = crate::git_freshness::ReadContext::new(read.repo);
            let reason = format!(
                "shell check {}: exit {:?}\n{}{}",
                command,
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            Ok((output.status.success(), reason, None))
        }
        "work.terminal" => work_terminal(index, policy, issue_id).map(without_validator_help),
        "decision.recorded" => decision_recorded(index, issue_id),
        other => bail!("unsupported builtin validator: {other}"),
    }
}

fn without_validator_help((passed, reason): (bool, String)) -> (bool, String, Option<String>) {
    (passed, reason, None)
}

fn evidence_attached(
    index: &RecordIndex,
    read: &mut crate::git_freshness::ReadContext<'_>,
    issue_id: &str,
    params: Option<&ValidatorParams>,
    fresh: bool,
) -> Result<(bool, String, Option<String>)> {
    let (min_count, kind) = match params {
        Some(ValidatorParams::EvidenceAttached { min_count, kind }) => {
            (*min_count as usize, kind.as_deref())
        }
        _ => (1, None),
    };
    let gate = read.issue_gate_with_freshness(index, issue_id, min_count, kind, fresh)?;
    Ok((gate.passed, gate.reason, (!gate.passed).then(|| "Record fresh proof with `atelier evidence record --target issue/<id> --kind test -- <command>`".to_string())))
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
    let mut required = record.sections.required_names();
    if record.issue.issue_type == "mission" && record.issue.strategy.is_none() {
        required.push(atelier_core::IssueSectionName::SuccessCriteria);
    }
    let invalid = required
        .iter()
        .filter(|name| {
            record.sections.section(**name).is_none_or(|text| {
                text.trim().is_empty()
                    || text.trim().ends_with("was not specified.")
                    || text.trim().ends_with("were not specified.")
            })
        })
        .map(|name| name.title().to_string())
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
            required
                .into_iter()
                .map(|name| name.title())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    ))
}

/// Passes when every child (recursively) and, for missions, all work linked
/// by `advances` is in a done-category status.
/// A decision closes only once its Decision section says what was decided.
fn decision_recorded(
    index: &RecordIndex,
    issue_id: &str,
) -> Result<(bool, String, Option<String>)> {
    let record = index
        .issue_record(issue_id)
        .ok_or_else(|| anyhow::anyhow!("Issue {issue_id} not found"))?;
    match record
        .sections
        .section(atelier_core::IssueSectionName::Decision)
        .map(str::trim)
        .filter(|text| !text.is_empty())
    {
        Some(_) => Ok((true, "Decision section is filled".to_string(), None)),
        None => Ok((
            false,
            "Decision section is missing".to_string(),
            Some(format!(
                "Add a ## Decision section saying what was decided and why, for example with `atelier issue update {issue_id} --body`, then decide"
            )),
        )),
    }
}

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
    let output = crate::git_freshness::command(repo_root)
        .args(["status", "--porcelain", "--untracked-files=all"])
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
            merge_checks: Vec::new(),
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
        records.issue("atelier-epic", "feature", "in_progress");
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
        records.issue("atelier-epic", "feature", "done");
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
