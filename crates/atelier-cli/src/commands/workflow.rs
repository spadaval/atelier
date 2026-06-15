use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use crate::commands::agent_factory::issue_evidence_gate_status;
use crate::db::Database;
use atelier_core::{EvidenceRecordData, Issue};
use atelier_records::{CanonicalIssueRecord, IssueSections, RecordStore};

#[derive(Debug, Clone, Serialize)]
pub struct ValidatorResult {
    pub target_kind: String,
    pub target_id: String,
    pub transition: String,
    pub validator: String,
    pub passed: bool,
    pub reason: String,
    pub elapsed_ms: u128,
}

#[derive(Debug, Clone)]
pub struct IssueTransitionOption {
    pub name: String,
    pub from: Vec<String>,
    pub to: String,
    pub allowed: bool,
    pub blockers: Vec<String>,
    pub validator_results: Vec<ValidatorResult>,
    pub guidance: Vec<String>,
    pub command: String,
}

pub fn check(db: &Database) -> Result<()> {
    let repo_root = repo_root()?;
    let report = crate::workflow_policy::check(db, &repo_root)?;
    println!("Workflow Check");
    println!("==============");
    println!(
        "Path:           {}",
        crate::workflow_policy::WORKFLOW_POLICY_PATH
    );
    println!("Policy:         pass");
    println!("Issue Types:    {}", report.policy.issue_types.len());
    println!("Statuses:       {}", report.policy.statuses.len());
    println!("Validators:     {}", report.policy.validators.len());
    println!("Workflows:      {}", report.policy.workflows.len());
    println!("Record Health:  pass");
    println!("Issues Checked: {}", report.issue_count);
    let (command_surface_passed, command_surface_reason) =
        crate::command_surface::status_reason(&repo_root)?;
    if command_surface_passed {
        println!("Docs/Help Drift: clear");
    } else {
        println!("Docs/Help Drift: detected");
        println!("{command_surface_reason}");
        bail!("workflow_command_surface_drift: {command_surface_reason}");
    }
    Ok(())
}

pub fn issue_transition_options(
    db: &Database,
    issue_ref: &str,
) -> Result<Vec<IssueTransitionOption>> {
    let issue_id = crate::commands::agent_factory::resolve_id(db, issue_ref)?;
    let repo_root = repo_root()?;
    let policy = crate::workflow_policy::load(&repo_root)?;
    let issue = db.require_issue(&issue_id)?;
    ensure_transitionable_status(&policy, &issue)?;
    let workflow = policy.workflow_for_issue_type(&issue.issue_type)?;
    let state_dir = crate::storage_layout::StorageLayout::new(&repo_root).canonical_dir();
    let store = RecordStore::new(&state_dir);
    let record = store.load_issue_by_id(&issue.id)?;
    let mut options = Vec::new();

    for (name, transition) in &workflow.transitions {
        if !transition.from.iter().any(|from| from == &issue.status) {
            continue;
        }
        let mut blockers = required_field_failures(&record, transition, None)?;
        let validator_results = evaluate_policy_transition(
            db,
            &policy,
            "issue",
            &issue.id,
            name,
            &transition.validators,
        )?;
        blockers.extend(
            validator_results
                .iter()
                .filter(|result| !result.passed)
                .map(|result| format!("validator {} failed: {}", result.validator, result.reason)),
        );
        let guidance = render_transition_guidance(&policy, &issue, name, transition)?;
        options.push(IssueTransitionOption {
            name: name.clone(),
            from: transition.from.clone(),
            to: transition.to.clone(),
            allowed: blockers.is_empty(),
            blockers,
            validator_results,
            guidance,
            command: transition_command(&issue.id, name, transition),
        });
    }

    if options.is_empty() {
        bail!(
            "Issue {} has no configured transitions from status '{}'",
            issue.id,
            issue.status
        );
    }

    Ok(options)
}

pub fn transition_issue(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: &str,
    transition_name: &str,
    close_reason: Option<&str>,
) -> Result<()> {
    let issue_id = crate::commands::agent_factory::resolve_id(db, issue_ref)?;
    let repo_root = repo_root()?;
    let policy = crate::workflow_policy::load(&repo_root)?;
    let before = db.require_issue(&issue_id)?;
    ensure_transitionable_status(&policy, &before)?;
    let workflow = policy.workflow_for_issue_type(&before.issue_type)?;
    let transition = resolve_issue_transition(workflow, &before, transition_name)?;
    ensure_transition_available(&before, transition_name, transition)?;

    let store = RecordStore::new(state_dir);
    let mut record = store.load_issue_by_id(&before.id)?;
    let (blockers, validator_results) = transition_blockers(
        db,
        &policy,
        &record,
        transition_name,
        transition,
        close_reason,
    )?;
    if !blockers.is_empty() {
        report_blocked_transition(
            &policy,
            &before,
            transition_name,
            transition,
            &validator_results,
            &blockers,
        )?;
    }

    apply_transition_record(&policy, &store, &mut record, transition, close_reason)?;
    record_applied_transition(&before, transition_name, transition)?;

    super::projection::refresh_after_canonical_write(state_dir, db_path)?;
    let refreshed = Database::open(db_path)?;
    let issue = refreshed.require_issue(&before.id)?;
    println!("Applied transition {} to {}", transition_name, issue.id);
    println!("From:     {}", before.status);
    println!("To:       {}", issue.status);
    print_heading("Next Commands");
    println!("  atelier issue show {}", issue.id);
    println!("  atelier issue transition {} --options", issue.id);
    Ok(())
}

fn resolve_issue_transition<'a>(
    workflow: &'a crate::workflow_policy::WorkflowDefinition,
    issue: &Issue,
    transition_name: &str,
) -> Result<&'a crate::workflow_policy::TransitionDefinition> {
    if let Some(transition) = workflow.transitions.get(transition_name) {
        return Ok(transition);
    }
    let available = workflow
        .transitions
        .iter()
        .filter(|(_, candidate)| candidate.from.iter().any(|from| from == &issue.status))
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    if available.is_empty() {
        bail!(
            "Unknown transition '{}' for issue {}; no transitions are configured from status '{}'",
            transition_name,
            issue.id,
            issue.status
        );
    }
    bail!(
        "Unknown transition '{}' for issue {}; available from '{}' are: {}",
        transition_name,
        issue.id,
        issue.status,
        available.join(", ")
    )
}

fn ensure_transition_available(
    issue: &Issue,
    transition_name: &str,
    transition: &crate::workflow_policy::TransitionDefinition,
) -> Result<()> {
    if transition.from.iter().any(|from| from == &issue.status) {
        return Ok(());
    }
    let reason = format!(
        "transition '{}' is not available from status '{}'",
        transition_name, issue.status
    );
    crate::commands::activity_log::record_transition_blocked(
        &issue.id,
        transition_name,
        &issue.status,
        Some(&transition.to),
        &reason,
    )?;
    bail!(
        "Transition '{}' is not available from status '{}' for issue {}",
        transition_name,
        issue.status,
        issue.id
    )
}

fn transition_blockers(
    db: &Database,
    policy: &crate::workflow_policy::WorkflowPolicy,
    record: &CanonicalIssueRecord,
    transition_name: &str,
    transition: &crate::workflow_policy::TransitionDefinition,
    close_reason: Option<&str>,
) -> Result<(Vec<String>, Vec<ValidatorResult>)> {
    let mut blockers = required_field_failures(record, transition, close_reason)?;
    let validator_results = evaluate_policy_transition(
        db,
        policy,
        "issue",
        &record.issue.id,
        transition_name,
        &transition.validators,
    )?;
    blockers.extend(
        validator_results
            .iter()
            .filter(|result| !result.passed)
            .map(|result| format!("validator {} failed: {}", result.validator, result.reason)),
    );
    Ok((blockers, validator_results))
}

fn report_blocked_transition(
    policy: &crate::workflow_policy::WorkflowPolicy,
    issue: &Issue,
    transition_name: &str,
    transition: &crate::workflow_policy::TransitionDefinition,
    validator_results: &[ValidatorResult],
    blockers: &[String],
) -> Result<()> {
    let reason = blockers.join("; ");
    crate::commands::activity_log::record_transition_blocked(
        &issue.id,
        transition_name,
        &issue.status,
        Some(&transition.to),
        &reason,
    )?;
    print_transition_attempt(
        issue,
        transition_name,
        &transition.to,
        validator_results,
        blockers,
        &render_transition_guidance(policy, issue, transition_name, transition)?,
        &transition_command(&issue.id, transition_name, transition),
    );
    bail!(
        "Transition '{}' is blocked for issue {}: {}",
        transition_name,
        issue.id,
        reason
    )
}

fn apply_transition_record(
    policy: &crate::workflow_policy::WorkflowPolicy,
    store: &RecordStore,
    record: &mut CanonicalIssueRecord,
    transition: &crate::workflow_policy::TransitionDefinition,
    close_reason: Option<&str>,
) -> Result<()> {
    let now = Utc::now();
    record.issue.status = transition.to.clone();
    record.issue.updated_at = now;
    record.issue.closed_at = if policy.status_category(&transition.to) == Some("done") {
        Some(now)
    } else {
        None
    };
    store.write_issue_atomic(record)?;
    if let Some(reason) = close_reason {
        crate::commands::activity_log::record_close_reason(&record.issue.id, reason)?;
    }
    Ok(())
}

fn record_applied_transition(
    issue: &Issue,
    transition_name: &str,
    transition: &crate::workflow_policy::TransitionDefinition,
) -> Result<()> {
    crate::commands::activity_log::record_transition_applied(
        &issue.id,
        transition_name,
        &issue.status,
        &transition.to,
    )
}

pub fn close_issue(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: &str,
    to_status: Option<&str>,
    close_reason: &str,
) -> Result<()> {
    let issue_id = crate::commands::agent_factory::resolve_id(db, issue_ref)?;
    let repo_root = repo_root()?;
    let policy = crate::workflow_policy::load(&repo_root)?;
    let issue = db.require_issue(&issue_id)?;
    ensure_transitionable_status(&policy, &issue)?;
    let workflow = policy.workflow_for_issue_type(&issue.issue_type)?;

    let mut candidates = workflow
        .transitions
        .iter()
        .filter(|(_, transition)| {
            transition.from.iter().any(|from| from == &issue.status)
                && policy.status_category(&transition.to) == Some("done")
        })
        .map(|(name, transition)| (name.as_str(), transition.to.as_str()))
        .collect::<Vec<_>>();

    if candidates.is_empty() {
        bail!(
            "Issue {} has no terminal done-category transitions from status '{}'; inspect `atelier issue transition {} --options`",
            issue.id,
            issue.status,
            issue.id
        );
    }

    if let Some(to_status) = to_status {
        candidates.retain(|(_, destination)| *destination == to_status);
        if candidates.is_empty() {
            let available = workflow
                .transitions
                .iter()
                .filter(|(_, transition)| {
                    transition.from.iter().any(|from| from == &issue.status)
                        && policy.status_category(&transition.to) == Some("done")
                })
                .map(|(_, transition)| transition.to.as_str())
                .collect::<BTreeSet<_>>();
            bail!(
                "Issue {} cannot close to status '{}'; available done targets from '{}' are: {}",
                issue.id,
                to_status,
                issue.status,
                available.into_iter().collect::<Vec<_>>().join(", ")
            );
        }
    } else {
        let destinations = candidates
            .iter()
            .map(|(_, destination)| *destination)
            .collect::<BTreeSet<_>>();
        if destinations.len() > 1 {
            bail!(
                "Issue {} has multiple terminal done targets from '{}'; rerun with `atelier issue close {} --to <status> --reason \"...\"` (available: {})",
                issue.id,
                issue.status,
                issue.id,
                destinations.into_iter().collect::<Vec<_>>().join(", ")
            );
        }
    }

    if candidates.len() > 1 {
        let transitions = candidates
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
            .join(", ");
        bail!(
            "Issue {} has multiple terminal transitions to status '{}': {}; use `atelier issue transition {} <transition> --reason \"...\"`",
            issue.id,
            candidates[0].1,
            transitions,
            issue.id
        );
    }

    transition_issue(
        db,
        state_dir,
        db_path,
        &issue.id,
        candidates[0].0,
        Some(close_reason),
    )?;

    let _ = Database::open(db_path)?;
    Ok(())
}

pub fn print_issue_transition_options(issue: &Issue, options: &[IssueTransitionOption]) {
    println!("Issue Transitions {} - {}", issue.id, issue.title);
    println!("{}", "=".repeat(issue.id.len() + issue.title.len() + 21));
    print_heading("State");
    println!("Status:   {}", issue.status);
    println!("Type:     {}", issue.issue_type);
    println!("Options:  {}", options.len());
    for option in options {
        println!();
        println!(
            "{} [{}]",
            option.name,
            if option.allowed { "allowed" } else { "blocked" }
        );
        println!("  From: {}", option.from.join(", "));
        println!("  To:   {}", option.to);
        println!("  Command: {}", option.command);
        print_transition_detail("Validators", &option.validator_results);
        print_text_list("Blockers", &option.blockers);
        print_text_list("Guidance", &option.guidance);
    }
}

pub fn evaluate(
    db: &Database,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validators: Vec<String>,
) -> Result<Vec<ValidatorResult>> {
    ensure_target_exists(db, target_kind, target_id)?;
    let validators = if validators.is_empty() {
        default_validators(target_kind, transition)
    } else {
        validators
    };
    let mut results = Vec::new();
    for validator in validators {
        let started = Instant::now();
        let (passed, reason) = evaluate_builtin_with_params(
            db,
            &crate::workflow_policy::load(&repo_root()?)?,
            target_kind,
            target_id,
            transition,
            &validator,
            None,
        )?;
        let elapsed_ms = started.elapsed().as_millis();
        results.push(ValidatorResult {
            target_kind: target_kind.to_string(),
            target_id: target_id.to_string(),
            transition: transition.to_string(),
            validator,
            passed,
            reason,
            elapsed_ms,
        });
    }

    Ok(results)
}

fn print_transition_attempt(
    issue: &Issue,
    transition_name: &str,
    destination: &str,
    validator_results: &[ValidatorResult],
    blockers: &[String],
    guidance: &[String],
    command: &str,
) {
    println!("Issue Transition {} - {}", issue.id, issue.title);
    println!("{}", "=".repeat(issue.id.len() + issue.title.len() + 20));
    println!("Transition: {}", transition_name);
    println!("From:       {}", issue.status);
    println!("To:         {}", destination);
    println!("Command:    {}", command);
    print_transition_detail("Validators", validator_results);
    print_text_list("Blockers", blockers);
    print_text_list("Guidance", guidance);
}

fn print_transition_detail(title: &str, results: &[ValidatorResult]) {
    print_heading(title);
    if results.is_empty() {
        println!("(none)");
        return;
    }
    for result in results {
        println!(
            "  {}  {}",
            if result.passed { "pass" } else { "fail" },
            result.validator
        );
        println!("      {}", result.reason);
    }
}

fn print_text_list(title: &str, values: &[String]) {
    print_heading(title);
    if values.is_empty() {
        println!("(none)");
        return;
    }
    for value in values {
        println!("  {value}");
    }
}

pub fn default_validators(target_kind: &str, transition: &str) -> Vec<String> {
    let names: &[&str] = match (target_kind, transition) {
        ("issue", "start") => &[
            "durable_state_current",
            "issue_sections_parseable",
            "no_open_blockers",
        ],
        ("issue", "close") => &[
            "durable_state_current",
            "issue_sections_parseable",
            "no_open_blockers",
            "evidence_attached",
        ],
        ("mission", "close") => &[
            "durable_state_current",
            "issue_sections_parseable",
            "no_open_work",
            "no_open_blockers",
            "validation_criteria_satisfied",
            "no_blocking_lints",
            "command_surface_current",
            "ignored_tests_reviewed",
            "git_worktree_clean",
        ],
        ("mission", _) => &[
            "durable_state_current",
            "issue_sections_parseable",
            "no_open_blockers",
        ],
        ("evidence", _) => &["durable_state_current"],
        ("tracker", "health") => &[
            "durable_state_current",
            "no_blocking_lints",
            "command_surface_current",
            "ignored_tests_reviewed",
            "git_worktree_clean",
        ],
        _ => &["durable_state_current"],
    };
    names.iter().map(|name| (*name).to_string()).collect()
}

fn print_heading(title: &str) {
    println!("{title}");
    println!("{}", "-".repeat(title.len()));
}

fn ensure_transitionable_status(
    policy: &crate::workflow_policy::WorkflowPolicy,
    issue: &Issue,
) -> Result<()> {
    if policy.workflow_allows_status(&issue.issue_type, &issue.status)? {
        return Ok(());
    }
    bail!(
        "Issue {} has status '{}' that is not allowed by the workflow policy for issue_type '{}'",
        issue.id,
        issue.status,
        issue.issue_type
    )
}

fn required_field_failures(
    _record: &CanonicalIssueRecord,
    transition: &crate::workflow_policy::TransitionDefinition,
    close_reason: Option<&str>,
) -> Result<Vec<String>> {
    let mut failures = Vec::new();
    for field in &transition.required_fields {
        match field.as_str() {
            "close_reason" => {
                if close_reason.is_none_or(|value| value.trim().is_empty()) {
                    failures.push(
                        "missing required field close_reason; rerun with `--reason \"...\"`"
                            .to_string(),
                    );
                }
            }
            other => {
                return Err(anyhow!("unsupported required field '{other}'"));
            }
        }
    }
    Ok(failures)
}

fn transition_command(
    issue_id: &str,
    transition_name: &str,
    transition: &crate::workflow_policy::TransitionDefinition,
) -> String {
    let mut command = format!("atelier issue transition {issue_id} {transition_name}");
    if transition
        .required_fields
        .iter()
        .any(|field| field == "close_reason")
    {
        command.push_str(" --reason \"...\"");
    }
    command
}

fn evaluate_policy_transition(
    db: &Database,
    policy: &crate::workflow_policy::WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validators: &[String],
) -> Result<Vec<ValidatorResult>> {
    ensure_target_exists(db, target_kind, target_id)?;
    let mut results = Vec::new();
    for validator_name in validators {
        let definition = policy.validators.get(validator_name).ok_or_else(|| {
            anyhow!(
                "workflow policy references unknown validator '{}'",
                validator_name
            )
        })?;
        let started = Instant::now();
        let (passed, reason) = evaluate_builtin_with_params(
            db,
            policy,
            target_kind,
            target_id,
            transition,
            &definition.builtin,
            definition.params.as_ref(),
        )?;
        results.push(ValidatorResult {
            target_kind: target_kind.to_string(),
            target_id: target_id.to_string(),
            transition: transition.to_string(),
            validator: validator_name.clone(),
            passed,
            reason,
            elapsed_ms: started.elapsed().as_millis(),
        });
    }
    Ok(results)
}

fn render_transition_guidance(
    policy: &crate::workflow_policy::WorkflowPolicy,
    issue: &Issue,
    transition_name: &str,
    transition: &crate::workflow_policy::TransitionDefinition,
) -> Result<Vec<String>> {
    let mut rendered = Vec::new();
    for guidance_name in &transition.guidance {
        let template = policy
            .guidance_templates
            .get(guidance_name)
            .ok_or_else(|| {
                anyhow!(
                    "workflow policy references undefined guidance template '{}'",
                    guidance_name
                )
            })?;
        rendered.push(
            template
                .template
                .replace("{{ issue.id }}", &issue.id)
                .replace("{{ issue.type }}", &issue.issue_type)
                .replace("{{ transition.name }}", transition_name)
                .replace("{{ transition.from }}", &issue.status)
                .replace("{{ transition.to }}", &transition.to)
                .trim()
                .to_string(),
        );
    }
    Ok(rendered)
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
    policy: &crate::workflow_policy::WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validator: &str,
    params: Option<&crate::workflow_policy::ValidatorParams>,
) -> Result<(bool, String)> {
    match validator {
        "durable_state_current" => {
            let state_dir = crate::storage_layout::StorageLayout::new(repo_root()?).canonical_dir();
            let stale = crate::commands::export::canonical_stale_entries(db, &state_dir)?;
            if stale.is_empty() {
                Ok((true, "canonical export is current".to_string()))
            } else {
                Ok((
                    false,
                    format!("canonical export is stale: {}", stale.join("; ")),
                ))
            }
        }
        "evidence_attached" => {
            if target_kind == "issue" {
                let issue = db.require_issue(target_id)?;
                let state_dir =
                    crate::storage_layout::StorageLayout::new(repo_root()?).canonical_dir();
                let store = RecordStore::new(&state_dir);
                let record = store.load_issue_by_id(target_id)?;
                let gate = issue_evidence_gate_status(db, &issue, Some(&record.sections))?;
                if let Some(crate::workflow_policy::ValidatorParams::EvidenceAttached {
                    min_count,
                    kind,
                }) = params
                {
                    let linked = linked_evidence_records(db, target_id, kind.as_deref())?;
                    let passing_count = linked
                        .iter()
                        .filter(|record| record.status == "pass")
                        .count();
                    if passing_count < *min_count as usize {
                        return Ok((
                            false,
                            format!(
                                "expected at least {} passing evidence record(s){}; found {}",
                                min_count,
                                kind.as_deref()
                                    .map(|value| format!(" of kind {}", value))
                                    .unwrap_or_default(),
                                passing_count
                            ),
                        ));
                    }
                }
                return Ok((gate.passed, gate.reason));
            }
            let attached = db
                .list_record_links(target_kind, target_id)?
                .into_iter()
                .any(|link| {
                    link.relation_type == "validates"
                        && (link.source_kind == "evidence" || link.target_kind == "evidence")
                });
            if attached {
                Ok((true, "validating evidence is linked".to_string()))
            } else {
                Ok((false, "no validating evidence link found".to_string()))
            }
        }
        "no_open_blockers" => {
            let open = open_blockers(db, policy, target_kind, target_id)?;
            if open.is_empty() {
                Ok((true, "no open blockers".to_string()))
            } else {
                Ok((false, format!("open blockers: {}", open.join(", "))))
            }
        }
        "no_open_work" => {
            let open = open_work(db, policy, target_kind, target_id)?;
            if open.is_empty() {
                Ok((true, "no open linked work".to_string()))
            } else {
                Ok((false, format!("open linked work: {}", open.join(", "))))
            }
        }
        "git_worktree_clean" => git_worktree_clean(),
        "no_blocking_lints" => {
            let status = Command::new(std::env::current_exe()?)
                .arg("lint")
                .status()?;
            if status.success() {
                Ok((true, "lint passed".to_string()))
            } else {
                Ok((false, "atelier lint failed".to_string()))
            }
        }
        "ignored_tests_reviewed" => ignored_tests_reviewed(),
        "command_surface_current" => command_surface_current(),
        "issue_sections_parseable" => issue_sections_parseable(db, target_kind, target_id),
        "validation_criteria_satisfied" => {
            validation_criteria_satisfied(db, target_kind, target_id)
        }
        "review_complete" => review_complete(db, policy, target_kind, target_id, transition),
        "epic_child_proof_complete" => {
            epic_child_proof_complete(db, policy, target_kind, target_id)
        }
        other => Ok((false, format!("unsupported builtin validator: {other}"))),
    }
}

fn epic_child_proof_complete(
    db: &Database,
    policy: &crate::workflow_policy::WorkflowPolicy,
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
        collect_missing_child_proof(db, policy, &child.id, &mut missing)?;
    }
    if missing.is_empty() {
        Ok((
            true,
            "all epic child issues are closed with passing proof".to_string(),
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
    policy: &crate::workflow_policy::WorkflowPolicy,
    issue_id: &str,
    missing: &mut Vec<String>,
) -> Result<()> {
    let issue = db.require_issue(issue_id)?;
    if issue_is_open_for_workflow(policy, &issue)? {
        missing.push(format!("{issue_id} open"));
    } else if !linked_evidence_records(db, issue_id, None)?
        .iter()
        .any(|record| record.status == "pass")
    {
        missing.push(format!("{issue_id} missing passing proof"));
    }
    for child in db.get_subissues(issue_id)? {
        collect_missing_child_proof(db, policy, &child.id, missing)?;
    }
    Ok(())
}

fn validation_criteria_satisfied(
    db: &Database,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    if target_kind == "mission" {
        return crate::commands::mission::mission_validation_criteria_gate(db, target_id);
    }
    Ok((
        true,
        format!("validation criteria closeout does not apply to {target_kind} records"),
    ))
}

fn issue_sections_parseable(
    db: &Database,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    let issue_ids = match target_kind {
        "issue" => {
            let mut ids = BTreeSet::new();
            ids.insert(target_id.to_string());
            ids
        }
        "mission" => mission_issue_ids(db, target_id)?,
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

    let state_dir = crate::storage_layout::StorageLayout::new(repo_root()?).canonical_dir();
    let store = RecordStore::new(&state_dir);
    let mut checked = 0;
    for issue_id in issue_ids {
        let record = match store.load_issue_by_id(&issue_id) {
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

fn linked_evidence_records(
    db: &Database,
    issue_id: &str,
    required_kind: Option<&str>,
) -> Result<Vec<atelier_core::DomainRecord>> {
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
        let record = db.require_record("evidence", &evidence_id)?;
        if let Some(required_kind) = required_kind {
            let data = serde_json::from_str::<EvidenceRecordData>(&record.data_json)
                .unwrap_or_else(|_| EvidenceRecordData {
                    evidence_type: String::new(),
                    captured_at: chrono::Utc::now(),
                    command: None,
                    path: None,
                    uri: None,
                    producer: None,
                    proof_scope: None,
                    agent_identity: None,
                    independence_level: None,
                    residual_risks: Vec::new(),
                    follow_up_ids: Vec::new(),
                    exit_code: None,
                    exit_status: None,
                    success: None,
                    spawn_error: None,
                    output: None,
                    target: None,
                });
            if data.evidence_type != required_kind {
                continue;
            }
        }
        records.push(record);
    }
    Ok(records)
}

fn review_complete(
    db: &Database,
    policy: &crate::workflow_policy::WorkflowPolicy,
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
    let issue = db.require_issue(target_id)?;
    match policy.status_category(&issue.status) {
        Some("review") | Some("validation") | Some("done") => Ok((
            true,
            format!(
                "issue {} has completed review state {}",
                issue.id, issue.status
            ),
        )),
        _ => Ok((
            false,
            format!(
                "issue {} must reach a review status before this transition; current status is {}",
                issue.id, issue.status
            ),
        )),
    }
}

fn issue_is_open_for_workflow(
    policy: &crate::workflow_policy::WorkflowPolicy,
    issue: &Issue,
) -> Result<bool> {
    ensure_transitionable_status(policy, issue)?;
    Ok(policy.status_category(&issue.status) != Some("done"))
}

fn open_blockers(
    db: &Database,
    policy: &crate::workflow_policy::WorkflowPolicy,
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
            for blocker in mission_direct_blockers(db, target_id)? {
                blocker_ids.insert(blocker);
            }
            for issue_id in mission_issue_ids(db, target_id)? {
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
    policy: &crate::workflow_policy::WorkflowPolicy,
    target_kind: &str,
    target_id: &str,
) -> Result<Vec<String>> {
    if target_kind != "mission" {
        return Ok(Vec::new());
    }
    let mut open = mission_issue_ids(db, target_id)?
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

fn mission_direct_blockers(db: &Database, mission_id: &str) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for link in db.list_record_links("mission", mission_id)? {
        if link.relation_type != "blocked_by" {
            continue;
        }
        if link.source_kind == "issue"
            && link.target_kind == "mission"
            && link.target_id == mission_id
        {
            blockers.push(link.source_id);
        } else if link.target_kind == "issue"
            && link.source_kind == "mission"
            && link.source_id == mission_id
        {
            blockers.push(link.target_id);
        }
    }
    Ok(blockers)
}

fn mission_issue_ids(db: &Database, mission_id: &str) -> Result<BTreeSet<String>> {
    let mut issue_ids = BTreeSet::new();
    for link in db.list_record_links("mission", mission_id)? {
        if link.relation_type != "advances" {
            continue;
        }
        let linked_id = if link.source_kind == "issue"
            && link.target_kind == "mission"
            && link.target_id == mission_id
        {
            Some(link.source_id)
        } else if link.target_kind == "issue"
            && link.source_kind == "mission"
            && link.source_id == mission_id
        {
            Some(link.target_id)
        } else {
            None
        };
        if let Some(linked_id) = linked_id {
            collect_issue_and_descendants(db, &linked_id, &mut issue_ids)?;
        }
    }
    Ok(issue_ids)
}

fn collect_issue_and_descendants(
    db: &Database,
    issue_id: &str,
    issue_ids: &mut BTreeSet<String>,
) -> Result<()> {
    if !issue_ids.insert(issue_id.to_string()) {
        return Ok(());
    }
    for child in db.get_subissues(issue_id)? {
        collect_issue_and_descendants(db, &child.id, issue_ids)?;
    }
    Ok(())
}

fn git_worktree_clean() -> Result<(bool, String)> {
    let root = repo_root()?;
    let output = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=all"])
        .current_dir(&root)
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.contains("not a git repository") {
            return Ok((
                true,
                "not a git repository; git worktree check skipped".to_string(),
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
        .filter_map(parse_git_dirty_entry)
        .collect::<Vec<_>>();
    if dirty.is_empty() {
        Ok((true, "git worktree is clean".to_string()))
    } else {
        let classified = classify_git_dirty_entries(&root, &dirty)?;
        if classified.blocking_entries.is_empty() {
            if classified.tracker_generated_entries.is_empty() {
                return Ok((true, "git worktree is clean".to_string()));
            }
            return Ok((
                true,
                format!(
                    "ignored {} tracker-generated canonical {}: {}",
                    classified.tracker_generated_entries.len(),
                    if classified.tracker_generated_entries.len() == 1 {
                        "entry"
                    } else {
                        "entries"
                    },
                    summarize_git_dirty_entries(&classified.tracker_generated_entries)
                ),
            ));
        }
        let sample = summarize_git_dirty_entries(&classified.blocking_entries);
        let suffix = if classified.blocking_entries.len() > 8 {
            format!("; ... and {} more", classified.blocking_entries.len() - 8)
        } else {
            String::new()
        };
        Ok((
            false,
            format!(
                "git worktree has {} dirty {}: {sample}{suffix}",
                classified.blocking_entries.len(),
                if classified.blocking_entries.len() == 1 {
                    "entry"
                } else {
                    "entries"
                }
            ),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GitDirtyEntry {
    raw: String,
    repo_path: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ClassifiedGitDirtyEntries {
    blocking_entries: Vec<String>,
    tracker_generated_entries: Vec<String>,
}

fn parse_git_dirty_entry(line: &str) -> Option<GitDirtyEntry> {
    let raw = line.trim_end();
    if raw.trim().is_empty() || raw.len() < 4 {
        return None;
    }
    let repo_path = raw
        .get(3..)
        .map(str::trim)
        .filter(|value| !value.is_empty())?;
    let repo_path = repo_path
        .rsplit_once(" -> ")
        .map(|(_, target)| target)
        .unwrap_or(repo_path)
        .to_string();
    Some(GitDirtyEntry {
        raw: raw.to_string(),
        repo_path,
    })
}

fn summarize_git_dirty_entries(entries: &[String]) -> String {
    entries
        .iter()
        .take(8)
        .cloned()
        .collect::<Vec<_>>()
        .join("; ")
}

fn classify_git_dirty_entries(
    repo_root: &Path,
    entries: &[GitDirtyEntry],
) -> Result<ClassifiedGitDirtyEntries> {
    let tracker_activity_issue_ids = entries
        .iter()
        .filter_map(|entry| atelier_relative_path(&entry.repo_path))
        .filter(|relative| is_tracker_generated_activity_path(relative))
        .filter_map(issue_id_from_activity_path)
        .collect::<BTreeSet<_>>();

    let mut blocking_entries = Vec::new();
    let mut tracker_generated_entries = Vec::new();
    for entry in entries {
        let Some(relative) = atelier_relative_path(&entry.repo_path) else {
            blocking_entries.push(entry.raw.clone());
            continue;
        };
        if crate::storage_layout::is_local_atelier_path(relative) {
            continue;
        }
        if is_tracker_generated_activity_path(relative) {
            tracker_generated_entries.push(entry.raw.clone());
            continue;
        }
        if is_tracker_generated_issue_bookkeeping(
            repo_root,
            relative,
            &entry.repo_path,
            &tracker_activity_issue_ids,
        )? {
            tracker_generated_entries.push(entry.raw.clone());
            continue;
        }
        blocking_entries.push(entry.raw.clone());
    }
    Ok(ClassifiedGitDirtyEntries {
        blocking_entries,
        tracker_generated_entries,
    })
}

fn atelier_relative_path(repo_path: &str) -> Option<&Path> {
    repo_path
        .strip_prefix(".atelier/")
        .map(|relative| Path::new(relative))
}

fn is_tracker_generated_activity_path(relative: &Path) -> bool {
    let mut components = relative.components();
    let Some(std::path::Component::Normal(root)) = components.next() else {
        return false;
    };
    if root != "issues" && root != "missions" {
        return false;
    }
    let Some(std::path::Component::Normal(dir)) = components.next() else {
        return false;
    };
    if !dir.to_string_lossy().ends_with(".activity") {
        return false;
    }
    let Some(std::path::Component::Normal(file)) = components.next() else {
        return false;
    };
    components.next().is_none() && file.to_string_lossy().ends_with(".md")
}

fn issue_id_from_activity_path(relative: &Path) -> Option<String> {
    let mut components = relative.components();
    let root = components.next()?.as_os_str();
    if root != "issues" {
        return None;
    }
    let dir = components.next()?.as_os_str().to_string_lossy();
    dir.strip_suffix(".activity").map(ToOwned::to_owned)
}

fn is_tracker_generated_issue_bookkeeping(
    repo_root: &Path,
    relative: &Path,
    repo_path: &str,
    tracker_activity_issue_ids: &BTreeSet<String>,
) -> Result<bool> {
    let Some(issue_id) = issue_id_from_canonical_issue_path(relative) else {
        return Ok(false);
    };
    if !tracker_activity_issue_ids.contains(&issue_id) {
        return Ok(false);
    }
    let current_text = fs::read_to_string(repo_root.join(repo_path))?;
    let Some(front_matter_end_line) = front_matter_end_line(&current_text) else {
        return Ok(false);
    };
    let diff = git_diff_against_head(repo_root, repo_path)?;
    if diff.trim().is_empty() {
        return Ok(false);
    }
    let mut saw_allowed_change = false;
    let mut current_line = None;
    for line in diff.lines() {
        if line.starts_with("diff --git")
            || line.starts_with("index ")
            || line.starts_with("--- ")
            || line.starts_with("+++ ")
        {
            continue;
        }
        if line.starts_with("@@ ") {
            current_line = parse_new_hunk_start(line);
            continue;
        }
        let Some(line_no) = current_line.as_mut() else {
            continue;
        };
        match line.chars().next() {
            Some('+') => {
                saw_allowed_change = true;
                if *line_no > front_matter_end_line
                    || !is_allowed_issue_bookkeeping_line(&line[1..])
                {
                    return Ok(false);
                }
                *line_no += 1;
            }
            Some('-') => {
                saw_allowed_change = true;
                if *line_no > front_matter_end_line
                    || !is_allowed_issue_bookkeeping_line(&line[1..])
                {
                    return Ok(false);
                }
            }
            Some(' ') => *line_no += 1,
            _ => {}
        }
    }
    Ok(saw_allowed_change)
}

fn issue_id_from_canonical_issue_path(relative: &Path) -> Option<String> {
    let mut components = relative.components();
    let root = components.next()?.as_os_str();
    if root != "issues" {
        return None;
    }
    let file = components.next()?.as_os_str().to_string_lossy();
    if components.next().is_some() || !file.ends_with(".md") || file.ends_with(".activity") {
        return None;
    }
    file.strip_suffix(".md").map(ToOwned::to_owned)
}

fn front_matter_end_line(text: &str) -> Option<usize> {
    let mut fence_count = 0;
    for (index, line) in text.lines().enumerate() {
        if line == "---" {
            fence_count += 1;
            if fence_count == 2 {
                return Some(index + 1);
            }
        }
    }
    None
}

fn parse_new_hunk_start(line: &str) -> Option<usize> {
    let (_, rest) = line.split_once('+')?;
    let digits = rest
        .chars()
        .take_while(|char| char.is_ascii_digit())
        .collect::<String>();
    digits.parse().ok()
}

fn is_allowed_issue_bookkeeping_line(line: &str) -> bool {
    line.starts_with("status: ")
        || line.starts_with("updated_at: ")
        || line.starts_with("closed_at: ")
}

fn git_diff_against_head(repo_root: &Path, repo_path: &str) -> Result<String> {
    let output = Command::new("git")
        .args([
            "diff",
            "--no-ext-diff",
            "--unified=0",
            "HEAD",
            "--",
            repo_path,
        ])
        .current_dir(repo_root)
        .output()?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).to_string());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    bail!("git diff HEAD -- {repo_path} failed: {}", stderr.trim())
}

fn ignored_tests_reviewed() -> Result<(bool, String)> {
    let inventory = crate::test_inventory::IgnoredTestInventory::scan_repo(&repo_root()?)?;
    Ok(inventory.status_reason())
}

fn command_surface_current() -> Result<(bool, String)> {
    crate::command_surface::status_reason(&repo_root()?)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_validators_are_target_and_transition_aware() {
        assert_eq!(
            default_validators("issue", "start"),
            vec![
                "durable_state_current",
                "issue_sections_parseable",
                "no_open_blockers"
            ]
        );
        assert_eq!(
            default_validators("issue", "close"),
            vec![
                "durable_state_current",
                "issue_sections_parseable",
                "no_open_blockers",
                "evidence_attached"
            ]
        );
        assert_eq!(
            default_validators("mission", "close"),
            vec![
                "durable_state_current",
                "issue_sections_parseable",
                "no_open_work",
                "no_open_blockers",
                "validation_criteria_satisfied",
                "no_blocking_lints",
                "command_surface_current",
                "ignored_tests_reviewed",
                "git_worktree_clean"
            ]
        );
        assert_eq!(
            default_validators("evidence", "attach"),
            vec!["durable_state_current"]
        );
        assert_eq!(
            default_validators("tracker", "health"),
            vec![
                "durable_state_current",
                "no_blocking_lints",
                "command_surface_current",
                "ignored_tests_reviewed",
                "git_worktree_clean"
            ]
        );
    }
}
