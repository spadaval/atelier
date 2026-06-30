use anyhow::{bail, Result};
use atelier_core::{EvidenceRecord, Issue, Record};
use atelier_records::IssueSections;
use atelier_sqlite::Database;
use serde::Serialize;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use crate::forgejo::{ForgejoClient, UreqForgejoTransport};
use crate::lint::{self, LintRequest};
use crate::pr as app_pr;
use crate::project_config::{ProjectConfig, ReviewConfig, ReviewProviderKind};
use crate::review_room;
use crate::user_config::forgejo_admin_token;
use crate::workflow_policy::{ValidatorDefinition, ValidatorParams, WorkflowPolicy};

pub struct ValidatorRequest<'a> {
    pub db: &'a Database,
    pub repo_root: &'a Path,
    pub policy: &'a WorkflowPolicy,
    pub target_kind: &'a str,
    pub target_id: &'a str,
    pub transition: &'a str,
    pub validators: &'a [ValidatorDefinition],
}

pub struct TransitionReadinessRequest<'a> {
    pub db: &'a Database,
    pub repo_root: &'a Path,
    pub policy: &'a WorkflowPolicy,
    pub issue_id: &'a str,
    pub transition_name: &'a str,
    pub validators: &'a [ValidatorDefinition],
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TransitionReadinessView {
    pub issue_id: String,
    pub transition_name: String,
    pub state: TransitionReadinessState,
    pub summary: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TransitionReadinessState {
    Allowed,
    Blocked,
    NotAvailable,
    Terminal,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidatorResult {
    pub target_kind: String,
    pub target_id: String,
    pub transition: String,
    pub validator: String,
    pub passed: bool,
    pub reason: String,
    pub help: Option<String>,
    pub elapsed_ms: u128,
}

pub fn evaluate_policy_transition(request: ValidatorRequest<'_>) -> Result<Vec<ValidatorResult>> {
    ensure_target_exists(request.db, request.target_kind, request.target_id)?;
    let mut results = Vec::new();
    for definition in request.validators {
        let started = Instant::now();
        let (passed, reason, help) = evaluate_builtin_with_params(
            request.db,
            request.repo_root,
            request.policy,
            request.target_kind,
            request.target_id,
            request.transition,
            &definition.builtin,
            definition.params.as_ref(),
        )?;
        results.push(ValidatorResult {
            target_kind: request.target_kind.to_string(),
            target_id: request.target_id.to_string(),
            transition: request.transition.to_string(),
            validator: definition.builtin.clone(),
            passed,
            reason,
            help,
            elapsed_ms: started.elapsed().as_millis(),
        });
    }
    Ok(results)
}

fn ensure_target_exists(db: &Database, kind: &str, id: &str) -> Result<()> {
    match kind {
        "issue" => {
            db.require_issue(id)?;
        }
        "tracker" => {
            if id != "health" {
                bail!("unsupported tracker validation target {id}; expected health");
            }
        }
        _ => {
            db.require_record(kind, id)?;
        }
    }
    Ok(())
}

fn evaluate_builtin_with_params(
    db: &Database,
    repo_root: &Path,
    policy: &WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validator: &str,
    params: Option<&ValidatorParams>,
) -> Result<(bool, String, Option<String>)> {
    match validator {
        "tracker.current" => {
            let state_dir = crate::storage_layout::StorageLayout::new(repo_root).canonical_dir();
            let stale = crate::export::canonical_stale_entries(db, &state_dir)?;
            if stale.is_empty() {
                Ok((true, "canonical export is current".to_string(), None))
            } else {
                Ok((
                    false,
                    format!("canonical export is stale: {}", stale.join("; ")),
                    None,
                ))
            }
        }
        "evidence.attached" => evidence_attached(db, repo_root, target_kind, target_id, params),
        "blockers.none_open" => {
            let open = open_blockers(db, policy, target_kind, target_id)?;
            if open.is_empty() {
                Ok((true, "no open blockers".to_string(), None))
            } else {
                Ok((false, format!("open blockers: {}", open.join(", ")), None))
            }
        }
        "no_open_work" => {
            let open = open_work(db, policy, target_kind, target_id)?;
            if open.is_empty() {
                Ok((true, "no open linked work".to_string(), None))
            } else {
                Ok((
                    false,
                    format!("open linked work: {}", open.join(", ")),
                    None,
                ))
            }
        }
        "git.on_base_branch" => git_on_base_branch(repo_root, policy).map(without_validator_help),
        "git.worktree_clean" => git_worktree_clean(repo_root).map(without_validator_help),
        "lint.none_blocking" => lint_none_blocking(db).map(without_validator_help),
        "ignored_tests_reviewed" => ignored_tests_reviewed(repo_root).map(without_validator_help),
        "command_surface_current" => command_surface_current(repo_root).map(without_validator_help),
        "issue.sections_parseable" => {
            issue_sections_parseable(db, repo_root, target_kind, target_id)
                .map(without_validator_help)
        }
        "validation.criteria_satisfied" => {
            validation_criteria_satisfied(db, repo_root, target_kind, target_id)
                .map(without_validator_help)
        }
        "objective.work_present" => {
            objective_work_present(db, target_kind, target_id).map(without_validator_help)
        }
        "objective.work_terminal" => {
            objective_work_terminal(db, policy, target_kind, target_id).map(without_validator_help)
        }
        "objective.blockers_none_open" => {
            objective_direct_blockers_none_open(db, policy, target_kind, target_id)
                .map(without_validator_help)
        }
        "review.linked_pr_merged" => {
            linked_pr_merged(db, repo_root, target_kind, target_id).map(without_validator_help)
        }
        "review.complete" => review_complete(db, repo_root, target_kind, target_id, transition)
            .map(without_validator_help),
        "children.proof_complete" => {
            epic_child_proof_complete(db, repo_root, policy, target_kind, target_id)
                .map(without_validator_help)
        }
        other => Ok((
            false,
            format!("unsupported builtin validator: {other}"),
            None,
        )),
    }
}

fn without_validator_help((passed, reason): (bool, String)) -> (bool, String, Option<String>) {
    (passed, reason, None)
}

fn evidence_attached(
    db: &Database,
    repo_root: &Path,
    target_kind: &str,
    target_id: &str,
    params: Option<&ValidatorParams>,
) -> Result<(bool, String, Option<String>)> {
    if target_kind == "issue" {
        let issue = db.require_issue(target_id)?;
        let state_dir = crate::storage_layout::StorageLayout::new(repo_root).canonical_dir();
        let record = crate::use_cases::load_canonical_issue(&state_dir, target_id)?;
        let gate = issue_evidence_gate_status(db, repo_root, &issue, Some(&record.sections))?;
        if let Some(ValidatorParams::EvidenceAttached { min_count, kind }) = params {
            let linked = linked_evidence_records(db, repo_root, target_id, kind.as_deref())?;
            let validating_count = linked.len();
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
        return Ok((gate.passed, gate.reason, gate.help));
    }
    let attached = db
        .list_record_links(target_kind, target_id)?
        .into_iter()
        .any(|link| {
            link.relation_type == "validates"
                && (link.source_kind == "evidence" || link.target_kind == "evidence")
        });
    if attached {
        Ok((true, "validating evidence is linked".to_string(), None))
    } else {
        Ok((
            false,
            "no validating evidence link found".to_string(),
            Some(evidence_help_hint()),
        ))
    }
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
    issue: &Issue,
    _sections: Option<&IssueSections>,
) -> Result<EvidenceGateStatus> {
    let evidence = linked_evidence_records(db, repo_root, &issue.id, None)?;
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
    Ok(
        match crate::use_cases::load_canonical_record(&state_dir, "evidence", id) {
            Ok(Record::Evidence(record)) => Some(record),
            Ok(_) | Err(_) => None,
        },
    )
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

fn validation_criteria_satisfied(
    db: &Database,
    repo_root: &Path,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    if target_kind != "mission" {
        return Ok((
            true,
            format!("validation criteria closeout does not apply to {target_kind} records"),
        ));
    }
    let state_dir = crate::storage_layout::StorageLayout::new(repo_root).canonical_dir();
    let record = crate::use_cases::load_canonical_issue(&state_dir, target_id)?;
    if record.sections.evidence.trim().is_empty() {
        return Ok((
            true,
            "no mission validation criteria are declared".to_string(),
        ));
    }
    let evidence = linked_evidence_records(db, repo_root, target_id, Some("validation"))?;
    if evidence.is_empty() {
        return Ok((
            false,
            "mission validation criteria require linked validation evidence".to_string(),
        ));
    }
    Ok((
        true,
        format!(
            "mission validation criteria covered by {} validation evidence record(s)",
            evidence.len()
        ),
    ))
}

fn issue_sections_parseable(
    db: &Database,
    repo_root: &Path,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    let issue_ids = match target_kind {
        "issue" => {
            let mut ids = BTreeSet::new();
            ids.insert(target_id.to_string());
            ids
        }
        "mission" => crate::objective_graph::mission_issue_ids(db, target_id)?,
        _ => {
            return Ok((
                true,
                format!("issue sections do not apply to {target_kind} records"),
            ))
        }
    };
    if issue_ids.is_empty() {
        return Ok((true, "no linked issues require section checks".to_string()));
    }

    let state_dir = crate::storage_layout::StorageLayout::new(repo_root).canonical_dir();
    let mut checked = 0;
    for issue_id in issue_ids {
        let record = match crate::use_cases::load_canonical_issue(&state_dir, &issue_id) {
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
        checked += 1;
    }

    Ok((
        true,
        format!(
            "parsed required sections {} are present and non-empty for {checked} issue(s)",
            IssueSections::REQUIRED_NAMES
                .into_iter()
                .map(|name| name.title())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    ))
}

fn objective_work_ids(
    db: &Database,
    target_kind: &str,
    target_id: &str,
) -> Result<BTreeSet<String>> {
    match target_kind {
        "mission" => crate::objective_graph::mission_issue_ids(db, target_id),
        "issue" => crate::objective_graph::issue_descendant_ids(db, target_id),
        _ => Ok(BTreeSet::new()),
    }
}

fn objective_work_present(
    db: &Database,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    let work = objective_work_ids(db, target_kind, target_id)?;
    if work.is_empty() {
        Ok((
            false,
            format!(
                "no advancing work linked to {target_kind} {target_id}; run `atelier issue link {target_id} <issue-id> --role advances`"
            ),
        ))
    } else {
        Ok((
            true,
            format!(
                "advancing work linked via advances: {}",
                work.into_iter().collect::<Vec<_>>().join(", ")
            ),
        ))
    }
}

fn objective_work_terminal(
    db: &Database,
    policy: &WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    let mut open = objective_work_ids(db, target_kind, target_id)?
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter_map(|issue| match issue_is_open_for_workflow(policy, &issue) {
            Ok(true) => Some(Ok(issue.id)),
            Ok(false) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>>>()?;
    open.sort();
    if open.is_empty() {
        Ok((true, "all advancing work is terminal".to_string()))
    } else {
        Ok((
            false,
            format!(
                "open advancing work via advances: {}; inspect `atelier issue transition {}`",
                open.join(", "),
                open.first()
                    .cloned()
                    .unwrap_or_else(|| "<issue-id>".to_string())
            ),
        ))
    }
}

fn objective_direct_blockers_none_open(
    db: &Database,
    policy: &WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    let mut open = crate::objective_graph::direct_blocker_ids(db, target_kind, target_id)?
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter_map(|issue| match issue_is_open_for_workflow(policy, &issue) {
            Ok(true) => Some(Ok(issue.id)),
            Ok(false) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>>>()?;
    open.sort();
    if open.is_empty() {
        Ok((true, "no open direct objective blockers".to_string()))
    } else {
        Ok((
            false,
            format!(
                "open direct objective blockers via blocked_by: {}; inspect `atelier issue show {target_id}`",
                open.join(", ")
            ),
        ))
    }
}

fn epic_child_proof_complete(
    db: &Database,
    repo_root: &Path,
    policy: &WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    if target_kind != "issue" {
        return Ok((
            true,
            format!("epic child proof does not apply to {target_kind} records"),
        ));
    }
    let issue = db.require_issue(target_id)?;
    if issue.issue_type != "epic" {
        return Ok((
            true,
            "epic child proof does not apply to non-epic issues".to_string(),
        ));
    }
    let mut missing = Vec::new();
    for child in db.get_subissues(target_id)? {
        collect_missing_child_proof(db, repo_root, policy, &child.id, &mut missing)?;
    }
    if missing.is_empty() {
        Ok((
            true,
            "all epic child issues are closed with validating proof".to_string(),
        ))
    } else {
        Ok((
            false,
            format!("epic child proof incomplete: {}", missing.join(", ")),
        ))
    }
}

fn collect_missing_child_proof(
    db: &Database,
    repo_root: &Path,
    policy: &WorkflowPolicy,
    issue_id: &str,
    missing: &mut Vec<String>,
) -> Result<()> {
    let issue = db.require_issue(issue_id)?;
    if issue_is_open_for_workflow(policy, &issue)? {
        missing.push(format!("{issue_id} is open"));
        return Ok(());
    }
    let gate = issue_evidence_gate_status(db, repo_root, &issue, None)?;
    if !gate.passed {
        missing.push(format!("{issue_id}: {}", gate.reason));
    }
    for child in db.get_subissues(issue_id)? {
        collect_missing_child_proof(db, repo_root, policy, &child.id, missing)?;
    }
    Ok(())
}

fn open_blockers(
    db: &Database,
    policy: &WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
) -> Result<Vec<String>> {
    let mut blocker_ids = BTreeSet::new();
    match target_kind {
        "issue" => {
            for blocker in db.get_blockers(target_id)? {
                blocker_ids.insert(blocker);
            }
        }
        "mission" => {
            for blocker in crate::objective_graph::direct_blocker_ids(db, "issue", target_id)? {
                blocker_ids.insert(blocker);
            }
            for issue_id in crate::objective_graph::mission_issue_ids(db, target_id)? {
                for blocker in db.get_blockers(&issue_id)? {
                    blocker_ids.insert(blocker);
                }
            }
        }
        _ => return Ok(Vec::new()),
    }
    let mut open = blocker_ids
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter_map(|issue| match issue_is_open_for_workflow(policy, &issue) {
            Ok(true) => Some(Ok(issue.id)),
            Ok(false) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>>>()?;
    open.sort();
    Ok(open)
}

fn open_work(
    db: &Database,
    policy: &WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
) -> Result<Vec<String>> {
    if target_kind != "mission" {
        return Ok(Vec::new());
    }
    let mut open = crate::objective_graph::mission_issue_ids(db, target_id)?
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter_map(|issue| match issue_is_open_for_workflow(policy, &issue) {
            Ok(true) => Some(Ok(issue.id)),
            Ok(false) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>>>()?;
    open.sort();
    Ok(open)
}

fn issue_is_open_for_workflow(policy: &WorkflowPolicy, issue: &Issue) -> Result<bool> {
    crate::workflow_policy::validate_issue_against_policy(
        policy,
        issue,
        &repo_root()?.join(crate::workflow_policy::WORKFLOW_POLICY_PATH),
    )?;
    Ok(policy.status_category(&issue.status) != Some("done"))
}

fn linked_pr_merged(
    db: &Database,
    repo_root: &Path,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    if target_kind != "issue" {
        return Ok((
            true,
            format!("linked PR merge state does not apply to {target_kind} records"),
        ));
    }

    let config_path = repo_root.join(".atelier/config.toml");
    let forgejo = match ProjectConfig::load(repo_root)
        .and_then(|config| config.require_forgejo(&config_path).cloned())
    {
        Ok(forgejo) => forgejo,
        Err(error) => {
            return Ok((
                false,
                format!(
                    "{}; configure Forgejo, then run `atelier review open --issue {}` or `atelier review status --issue {}`",
                    error,
                    target_id,
                    target_id
                ),
            ));
        }
    };
    let token = match forgejo_admin_token() {
        Ok(token) => token,
        Err(error) => {
            return Ok((
                false,
                format!(
                    "{error:#}; run `atelier review status --issue {}` after configuring Forgejo credentials",
                    target_id
                ),
            ));
        }
    };
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    app_pr::linked_pull_request_merge_status_with_client(db, repo_root, target_id, &client)
}

fn review_complete(
    db: &Database,
    repo_root: &Path,
    target_kind: &str,
    target_id: &str,
    _transition: &str,
) -> Result<(bool, String)> {
    if target_kind != "issue" {
        return Ok((
            true,
            format!("review completion does not apply to {target_kind}"),
        ));
    }
    match ProjectConfig::load(repo_root) {
        Ok(ProjectConfig {
            review: ReviewConfig::Room,
            ..
        }) => room_review_complete(db, repo_root, target_id),
        Ok(ProjectConfig {
            review:
                ReviewConfig::Provider(crate::project_config::ReviewProviderConfig {
                    provider: ReviewProviderKind::Forgejo(_),
                }),
            ..
        }) => linked_pr_merged(db, repo_root, target_kind, target_id),
        Err(error) => Ok((
            false,
            format!(
                "{}; run `atelier review status --issue {}`",
                error, target_id
            ),
        )),
    }
}

fn room_review_complete(db: &Database, repo_root: &Path, issue_id: &str) -> Result<(bool, String)> {
    let state_dir = crate::storage_layout::StorageLayout::new(repo_root).canonical_dir();
    let outcome = match review_room::status(
        db,
        review_room::RoomStatusRequest {
            repo_root,
            state_dir: &state_dir,
            issue_ref: Some(issue_id),
        },
    ) {
        Ok(outcome) => outcome,
        Err(error) => {
            return Ok((
                false,
                format!(
                    "{}; run `atelier review status --issue {}`",
                    error, issue_id
                ),
            ))
        }
    };

    if outcome.status == "merged" {
        Ok((true, format!("review room {} is merged", outcome.review_id)))
    } else {
        Ok((
            false,
            format!(
                "review room {} is {}; run `atelier review status --issue {}`",
                outcome.review_id, outcome.status, issue_id
            ),
        ))
    }
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

fn git_on_base_branch(repo_root: &Path, policy: &WorkflowPolicy) -> Result<(bool, String)> {
    let expected = &policy.branch_policy.base_branch;
    match current_git_branch(repo_root)? {
        Some(current) if &current == expected => Ok((
            true,
            format!("current branch is configured base branch {expected}"),
        )),
        Some(current) => Ok((
            false,
            format!("current branch is {current}; expected configured base branch {expected}"),
        )),
        None => Ok((
            false,
            format!("detached HEAD; expected configured base branch {expected}"),
        )),
    }
}

fn current_git_branch(repo_root: &Path) -> Result<Option<String>> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(repo_root)
        .output()?;
    if !output.status.success() {
        return Ok(None);
    }
    Ok(
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            .filter(|value| !value.is_empty()),
    )
}

fn ignored_tests_reviewed(repo_root: &Path) -> Result<(bool, String)> {
    let inventory = crate::test_inventory::IgnoredTestInventory::scan_repo(repo_root)?;
    Ok(inventory.status_reason())
}

fn command_surface_current(repo_root: &Path) -> Result<(bool, String)> {
    crate::command_surface::status_reason(repo_root)
}

fn repo_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;
    if output.status.success() {
        Ok(Path::new(String::from_utf8_lossy(&output.stdout).trim()).to_path_buf())
    } else {
        Ok(std::env::current_dir()?)
    }
}
