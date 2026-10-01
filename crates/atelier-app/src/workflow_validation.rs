use anyhow::{bail, Result};
use atelier_core::EvidenceRecord;
use atelier_records::IssueSections;
use atelier_sqlite::Database;
use serde::Serialize;
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use crate::lint::{self, LintRequest};
use crate::workflow_policy::{ValidatorDefinition, ValidatorParams, WorkflowPolicy};

pub struct ValidatorRequest<'a> {
    pub db: &'a Database,
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
    request.db.require_issue(request.issue_id)?;
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
        db,
        repo_root,
        policy,
        issue_id,
        ..
    } = *request;
    match definition.builtin.as_str() {
        "issue.sections_parseable" => {
            issue_sections_parseable(repo_root, issue_id).map(without_validator_help)
        }
        "blockers.none_open" => {
            let open = open_blockers(db, policy, repo_root, issue_id)?;
            if open.is_empty() {
                Ok((true, "no open blockers".to_string(), None))
            } else {
                Ok((false, format!("open blockers: {}", open.join(", ")), None))
            }
        }
        "blockers.transitive_none_open" => transitive_blockers_none_open(db, policy, issue_id),
        "evidence.attached" => {
            evidence_attached(db, repo_root, issue_id, definition.params.as_ref())
        }
        "lint.none_blocking" => lint_none_blocking(db).map(without_validator_help),
        "git.worktree_clean" => git_worktree_clean(repo_root).map(without_validator_help),
        "work.terminal" => work_terminal(db, policy, issue_id).map(without_validator_help),
        other => bail!("unsupported builtin validator: {other}"),
    }
}

fn without_validator_help((passed, reason): (bool, String)) -> (bool, String, Option<String>) {
    (passed, reason, None)
}

fn evidence_attached(
    db: &Database,
    repo_root: &Path,
    issue_id: &str,
    params: Option<&ValidatorParams>,
) -> Result<(bool, String, Option<String>)> {
    if let Some(ValidatorParams::EvidenceAttached { min_count, kind }) = params {
        let validating_count =
            linked_evidence_records(db, repo_root, issue_id, kind.as_deref())?.len();
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
    let gate = issue_evidence_gate_status(db, repo_root, issue_id)?;
    Ok((gate.passed, gate.reason, gate.help))
}

#[derive(Debug, Clone)]
struct EvidenceGateStatus {
    passed: bool,
    reason: String,
    help: Option<String>,
}

fn issue_evidence_gate_status(
    db: &Database,
    repo_root: &Path,
    issue_id: &str,
) -> Result<EvidenceGateStatus> {
    let evidence = linked_evidence_records(db, repo_root, issue_id, None)?;
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

fn linked_evidence_records(
    db: &Database,
    repo_root: &Path,
    issue_id: &str,
    required_kind: Option<&str>,
) -> Result<Vec<EvidenceRecord>> {
    let mut records = Vec::new();
    for link in db.list_record_links("issue", issue_id)? {
        if link.relation_type != "validates" {
            continue;
        }
        let evidence_id = if link.source_kind == "evidence" {
            Some(link.source_id)
        } else if link.target_kind == "evidence" {
            Some(link.target_id)
        } else {
            None
        };
        let Some(evidence_id) = evidence_id else {
            continue;
        };
        db.require_record("evidence", &evidence_id)?;
        let Some(record) = canonical_evidence_record(repo_root, &evidence_id)? else {
            continue;
        };
        if let Some(required_kind) = required_kind {
            if record.data.evidence_type != required_kind {
                continue;
            }
        }
        records.push(record);
    }
    Ok(records)
}

fn canonical_evidence_record(repo_root: &Path, id: &str) -> Result<Option<EvidenceRecord>> {
    let state_dir = crate::storage_layout::StorageLayout::new(repo_root).canonical_dir();
    Ok(crate::use_cases::load_canonical_evidence(&state_dir, id).ok())
}

fn lint_none_blocking(db: &Database) -> Result<(bool, String)> {
    let outcome = lint::lint(crate::Request {
        input: LintRequest {
            db,
            issue_ref: None,
        },
    })?;
    let findings = outcome.value.data.findings;
    if findings.is_empty() {
        Ok((true, "lint passed".to_string()))
    } else {
        Ok((
            false,
            format!("atelier lint failed with {} finding(s)", findings.len()),
        ))
    }
}

fn issue_sections_parseable(repo_root: &Path, issue_id: &str) -> Result<(bool, String)> {
    let state_dir = crate::storage_layout::StorageLayout::new(repo_root).canonical_dir();
    let record = match crate::use_cases::load_canonical_issue(&state_dir, issue_id) {
        Ok(record) => record,
        Err(error) => return Ok((false, error.to_string())),
    };
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
fn work_terminal(db: &Database, policy: &WorkflowPolicy, issue_id: &str) -> Result<(bool, String)> {
    let mut open = Vec::new();
    for id in crate::objective_graph::issue_descendant_ids(db, issue_id)? {
        let issue = db.require_issue(&id)?;
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
    db: &Database,
    policy: &WorkflowPolicy,
    issue_id: &str,
) -> Result<(bool, String, Option<String>)> {
    match crate::mission_readiness::dependency_diagnosis(db, policy, issue_id)? {
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
    db: &Database,
    policy: &WorkflowPolicy,
    repo_root: &Path,
    issue_id: &str,
) -> Result<Vec<String>> {
    let policy_path = repo_root.join(crate::workflow_policy::WORKFLOW_POLICY_PATH);
    let mut open = Vec::new();
    for blocker_id in db
        .get_blockers(issue_id)?
        .into_iter()
        .collect::<BTreeSet<_>>()
    {
        let Some(blocker) = db.get_issue(&blocker_id)? else {
            continue;
        };
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
    use crate::workflow_policy::{BranchLifecycleConfig, StatusDefinition, WorkflowPolicy};
    use atelier_sqlite::{Database, IssueCacheRow, RecordSourceCacheRow};
    use chrono::Utc;
    use std::collections::BTreeMap;
    use tempfile::tempdir;

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

    fn index(db: &Database, id: &str, issue_type: &str, status: &str, parent: Option<&str>) {
        let now = Utc::now();
        db.index_issue(
            &IssueCacheRow {
                id: id.to_string(),
                title: id.to_string(),
                status: status.to_string(),
                issue_type: issue_type.to_string(),
                priority: "medium".to_string(),
                fields: BTreeMap::new(),
                parent_id: parent.map(str::to_string),
                created_at: now,
                updated_at: now,
                closed_at: None,
            },
            &[],
            &[],
            &[],
            &RecordSourceCacheRow {
                path: format!("issues/{id}.md"),
                record_kind: "issue".to_string(),
                record_id: id.to_string(),
                size_bytes: 0,
                modified_micros: None,
                content_hash: None,
                indexed_at: now,
            },
        )
        .unwrap();
    }

    #[test]
    fn work_terminal_requires_every_child_done() {
        let dir = tempdir().unwrap();
        let db = Database::open(&dir.path().join("test.db")).unwrap();
        index(&db, "atelier-epic", "epic", "in_progress", None);
        index(&db, "atelier-done", "task", "done", Some("atelier-epic"));
        index(
            &db,
            "atelier-open",
            "task",
            "in_progress",
            Some("atelier-epic"),
        );

        let (passed, reason) = work_terminal(&db, &policy(), "atelier-epic").unwrap();
        assert!(!passed);
        assert_eq!(reason, "open work: atelier-open [in_progress]");

        index(&db, "atelier-open", "task", "done", Some("atelier-epic"));
        let (passed, reason) = work_terminal(&db, &policy(), "atelier-epic").unwrap();
        assert!(passed, "{reason}");
    }

    #[test]
    fn work_terminal_follows_mission_advances_links() {
        let dir = tempdir().unwrap();
        let db = Database::open(&dir.path().join("test.db")).unwrap();
        index(&db, "atelier-mission", "mission", "in_progress", None);
        index(&db, "atelier-epic", "epic", "done", None);
        index(&db, "atelier-child", "task", "todo", Some("atelier-epic"));
        db.add_typed_relation("atelier-mission", "atelier-epic", "advances")
            .unwrap();

        let (passed, reason) = work_terminal(&db, &policy(), "atelier-mission").unwrap();
        assert!(!passed);
        assert_eq!(reason, "open work: atelier-child [todo]");

        index(&db, "atelier-child", "task", "done", Some("atelier-epic"));
        let (passed, reason) = work_terminal(&db, &policy(), "atelier-mission").unwrap();
        assert!(passed, "{reason}");
    }
}
