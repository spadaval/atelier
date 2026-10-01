use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::human_output::{self, DecisionState, StylePolicy};
use atelier_app::use_cases as app_use_cases;
use atelier_app::workflow_policy::{TransitionDefinition, WorkflowPolicy};
use atelier_core::Issue;
use atelier_records::CanonicalIssueRecord;
use atelier_sqlite::Database;

pub use atelier_app::workflow_validation::ValidatorResult;

#[derive(Debug, Clone)]
pub struct IssueTransitionOption {
    pub name: String,
    pub from: Vec<String>,
    pub to: String,
    pub allowed: bool,
    pub blockers: Vec<String>,
    pub validator_results: Vec<ValidatorResult>,
    pub descriptions: Vec<String>,
    pub command: String,
}

/// Applies a workflow transition: checks required fields and read-only
/// validators, then writes the issue record. Never touches Git.
pub fn transition_issue(
    db: &Database,
    state_dir: &Path,
    issue_ref: &str,
    transition_name: &str,
    close_reason: Option<&str>,
) -> Result<()> {
    let issue_id = crate::commands::issue::resolve_id(db, issue_ref)?;
    if let Some(reason) = close_reason {
        atelier_records::validate_issue_section_text(
            &issue_id,
            atelier_records::IssueSectionName::CloseReason,
            reason,
        )?;
    }
    let repo_root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let before = db.require_issue(&issue_id)?;
    ensure_transitionable_status(&policy, &before)?;
    let transition = resolve_issue_transition(&policy, &before, transition_name)?;
    ensure_transition_available(&before, transition_name, transition)?;
    ensure_transition_allowed(
        db,
        &policy,
        &repo_root,
        &before,
        transition_name,
        transition,
        close_reason,
    )?;

    let mut record = app_use_cases::load_canonical_issue(state_dir, &before.id)?;
    apply_transition_record(&policy, state_dir, &mut record, transition, close_reason)?;
    print_applied_transition(transition_name, &before.id, &before.status, &transition.to);
    print_heading("Next Commands");
    println!("  atelier issue show {}", before.id);
    println!("  atelier issue transition {}", before.id);
    Ok(())
}

pub(crate) fn print_applied_transition(
    transition_name: &str,
    issue_id: &str,
    from: &str,
    to: &str,
) {
    println!("Applied transition {transition_name} to {issue_id}");
    println!("From:     {from}");
    println!("To:       {to}");
}

/// Fails with the full transition report when required fields, ancestry, or
/// validators block the transition.
pub(crate) fn ensure_transition_allowed(
    db: &Database,
    policy: &WorkflowPolicy,
    repo_root: &Path,
    issue: &Issue,
    transition_name: &str,
    transition: &TransitionDefinition,
    close_reason: Option<&str>,
) -> Result<()> {
    let (blockers, validator_results) =
        transition_blockers(db, policy, repo_root, issue, transition, close_reason)?;
    if blockers.is_empty() {
        return Ok(());
    }
    print_transition_attempt(
        issue,
        transition_name,
        &transition.to,
        &validator_results,
        &blockers,
        &transition_descriptions(transition),
        &transition_command(&issue.id, transition_name, transition),
    );
    bail!(
        "Transition '{}' is blocked for issue {}: {}",
        transition_name,
        issue.id,
        blockers.join("; ")
    )
}

fn resolve_issue_transition<'a>(
    policy: &'a WorkflowPolicy,
    issue: &Issue,
    transition_name: &str,
) -> Result<&'a TransitionDefinition> {
    if let Ok(transition) = policy.transition_for_issue_type(&issue.issue_type, transition_name) {
        return Ok(transition);
    }
    let available = policy
        .transitions_from_status(&issue.issue_type, &issue.status)?
        .into_iter()
        .map(|(name, _)| name)
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
    transition: &TransitionDefinition,
) -> Result<()> {
    if transition.from.iter().any(|from| from == &issue.status) {
        return Ok(());
    }
    bail!(
        "Transition '{}' is not available from status '{}' for issue {}",
        transition_name,
        issue.status,
        issue.id
    )
}

fn transition_blockers(
    db: &Database,
    policy: &WorkflowPolicy,
    repo_root: &Path,
    issue: &Issue,
    transition: &TransitionDefinition,
    close_reason: Option<&str>,
) -> Result<(Vec<String>, Vec<ValidatorResult>)> {
    let mut blockers = required_field_failures(transition, close_reason)?;
    if let Some(blocker) = active_transition_ancestor_blocker(db, policy, issue, transition)? {
        blockers.push(blocker);
    }
    let validator_results =
        evaluate_policy_transition(db, policy, repo_root, &issue.id, transition)?;
    blockers.extend(
        validator_results
            .iter()
            .filter(|result| !result.passed)
            .map(|result| format!("validator {} failed: {}", result.validator, result.reason)),
    );
    Ok((blockers, validator_results))
}

pub fn issue_transition_options(
    db: &Database,
    issue_ref: &str,
) -> Result<Vec<IssueTransitionOption>> {
    let issue_id = crate::commands::issue::resolve_id(db, issue_ref)?;
    let repo_root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let issue = db.require_issue(&issue_id)?;
    ensure_transitionable_status(&policy, &issue)?;
    let mut options = Vec::new();
    for (name, transition) in policy.transitions_from_status(&issue.issue_type, &issue.status)? {
        let (blockers, validator_results) =
            transition_blockers(db, &policy, &repo_root, &issue, transition, None)?;
        options.push(IssueTransitionOption {
            name: name.to_string(),
            from: transition.from.clone(),
            to: transition.to.clone(),
            allowed: blockers.is_empty(),
            blockers,
            validator_results,
            descriptions: transition_descriptions(transition),
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

fn active_transition_ancestor_blocker(
    db: &Database,
    policy: &WorkflowPolicy,
    issue: &Issue,
    transition: &TransitionDefinition,
) -> Result<Option<String>> {
    if policy.status_category(&issue.status) == Some("done")
        || policy.status_category(&transition.to) != Some("active")
    {
        return Ok(None);
    }
    Ok(
        atelier_app::read_pipeline::terminal_ancestor(db, Some(policy), issue)?.map(|ancestor_id| {
            format!(
                "terminal ancestor {ancestor_id} retires nonterminal descendant {}; cannot enter active-category status '{}'; inspect `atelier issue show {ancestor_id}`",
                issue.id, transition.to
            )
        }),
    )
}

pub(crate) fn apply_transition_record(
    policy: &WorkflowPolicy,
    state_dir: &Path,
    record: &mut CanonicalIssueRecord,
    transition: &TransitionDefinition,
    close_reason: Option<&str>,
) -> Result<()> {
    let now = Utc::now();
    record.issue.status = transition.to.clone();
    record.issue.updated_at = now;
    let closing = policy.status_category(&transition.to) == Some("done");
    record.issue.closed_at = closing.then_some(now);
    if let Some(reason) = close_reason
        .map(str::trim)
        .filter(|reason| closing && !reason.is_empty())
    {
        record.sections.close_reason = Some(reason.to_string());
    }
    app_use_cases::write_canonical_issue(state_dir, record)
}

pub fn print_issue_transition_options(
    issue: &Issue,
    options: &[IssueTransitionOption],
    verbose: bool,
) {
    println!(
        "{}",
        render_issue_transition_options(issue, options, StylePolicy::for_stdout(), verbose)
    );
}

fn render_issue_transition_options(
    issue: &Issue,
    options: &[IssueTransitionOption],
    style_policy: StylePolicy,
    verbose: bool,
) -> String {
    let mut lines = vec![
        human_output::heading(&format!("Issue Transitions {} - {}", issue.id, issue.title)),
        human_output::section_heading("State"),
        format!("Status:   {}", issue.status),
        format!("Type:     {}", issue.issue_type),
        format!("Options:  {}", options.len()),
    ];
    for option in options {
        let decision = if option.allowed {
            DecisionState::Allowed
        } else {
            DecisionState::Blocked
        };
        lines.push(String::new());
        lines.push(format!(
            "{} [{}]",
            option.name,
            decision.render(style_policy)
        ));
        lines.push(format!("  From: {}", option.from.join(", ")));
        lines.push(format!("  To:   {}", option.to));
        if option.allowed {
            lines.push("  Requirements: satisfied".to_string());
        } else {
            if let Some(result) = option
                .validator_results
                .iter()
                .find(|result| !result.passed && result.help.is_some())
            {
                lines.push(format!("  Readiness: {}", result.reason));
                lines.push(format!("  {}", result.help.as_deref().unwrap_or_default()));
            }
            lines.extend(render_text_list(
                "Failed Requirements",
                &failed_requirement_lines(&option.blockers),
            ));
        }
        if verbose {
            lines.push(format!("  Decision: {}", decision.render(style_policy)));
            lines.extend(render_transition_detail(
                "Validators",
                &option.validator_results,
                style_policy,
            ));
            lines.extend(render_text_list("Description", &option.descriptions));
        }
        lines.extend(render_text_list("Commands", &[option.command.clone()]));
    }
    lines.join("\n")
}

fn failed_requirement_lines(blockers: &[String]) -> Vec<String> {
    blockers
        .iter()
        .map(|blocker| {
            if let Some(rest) = blocker.strip_prefix("validator ") {
                if let Some((name, _)) = rest.split_once(" failed:") {
                    return format!("validator {name}");
                }
            }
            if let Some(rest) = blocker.strip_prefix("missing required field ") {
                if let Some((field, _)) = rest.split_once(';') {
                    return format!("required field {field}");
                }
            }
            if let Some((requirement, _)) = blocker.split_once(':') {
                return requirement.to_string();
            }
            blocker.clone()
        })
        .collect()
}

fn print_transition_attempt(
    issue: &Issue,
    transition_name: &str,
    destination: &str,
    validator_results: &[ValidatorResult],
    blockers: &[String],
    descriptions: &[String],
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
    print_text_list("Description", descriptions);
}

fn print_transition_detail(title: &str, results: &[ValidatorResult]) {
    for line in render_transition_detail(title, results, StylePolicy::for_stdout()) {
        println!("{line}");
    }
}

fn render_transition_detail(
    title: &str,
    results: &[ValidatorResult],
    style_policy: StylePolicy,
) -> Vec<String> {
    let mut lines = vec![human_output::section_heading(title)];
    if results.is_empty() {
        lines.push("(none)".to_string());
        return lines;
    }
    for result in results {
        let decision = if result.passed {
            DecisionState::Pass
        } else {
            DecisionState::Fail
        };
        lines.push(format!(
            "  {}  {}",
            decision.render(style_policy),
            result.validator
        ));
        lines.push(format!("      {}", result.reason));
        if let Some(help) = &result.help {
            lines.push(format!("      Hint: {help}"));
        }
    }
    lines
}

fn print_text_list(title: &str, values: &[String]) {
    for line in render_text_list(title, values) {
        println!("{line}");
    }
}

fn render_text_list(title: &str, values: &[String]) -> Vec<String> {
    let mut lines = vec![human_output::section_heading(title)];
    if values.is_empty() {
        lines.push("(none)".to_string());
        return lines;
    }
    for value in values {
        lines.push(format!("  {value}"));
    }
    lines
}

fn print_heading(title: &str) {
    human_output::print_section_heading(title);
}

fn ensure_transitionable_status(policy: &WorkflowPolicy, issue: &Issue) -> Result<()> {
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
    transition: &TransitionDefinition,
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
    transition: &TransitionDefinition,
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
    policy: &WorkflowPolicy,
    repo_root: &Path,
    issue_id: &str,
    transition: &TransitionDefinition,
) -> Result<Vec<ValidatorResult>> {
    atelier_app::workflow_validation::evaluate_policy_transition(
        atelier_app::workflow_validation::ValidatorRequest {
            db,
            repo_root,
            policy,
            issue_id,
            validators: &transition.validators,
        },
    )
}

fn transition_descriptions(transition: &TransitionDefinition) -> Vec<String> {
    transition
        .description
        .iter()
        .map(|description| description.trim().to_string())
        .filter(|description| !description.is_empty())
        .collect()
}

pub(crate) fn repo_root() -> Result<PathBuf> {
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
    use crate::commands::test_support::DomainCacheFixture;
    use crate::human_output::ColorChoice;
    use atelier_records::{IssueSections, RecordStore, Relationships};
    use std::collections::BTreeMap;
    use tempfile::{tempdir, TempDir};

    fn test_issue(id: &str) -> Issue {
        Issue {
            id: id.to_string(),
            title: "Issue".to_string(),
            description: None,
            status: "in_progress".to_string(),
            priority: "medium".to_string(),
            issue_type: "epic".to_string(),
            fields: BTreeMap::new(),
            parent_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
        }
    }

    fn transition_option(allowed: bool, passed: bool) -> IssueTransitionOption {
        IssueTransitionOption {
            name: "start".to_string(),
            from: vec!["todo".to_string()],
            to: "in_progress".to_string(),
            allowed,
            blockers: Vec::new(),
            validator_results: vec![ValidatorResult {
                validator: "issue.sections_parseable".to_string(),
                passed,
                reason: "sections are parseable".to_string(),
                help: None,
                elapsed_ms: 1,
            }],
            descriptions: vec!["Begin work.".to_string()],
            command: "atelier issue transition atelier-test start".to_string(),
        }
    }

    #[test]
    fn transition_options_use_color_when_interactive_context_allows_it() {
        let issue = test_issue("atelier-test");
        let policy = StylePolicy::from_context(ColorChoice::Auto, true, false);

        let output =
            render_issue_transition_options(&issue, &[transition_option(true, true)], policy, true);

        assert!(output.contains("\u{1b}[32mallowed\u{1b}[0m"));
        assert!(output.contains("Decision: \u{1b}[32mallowed\u{1b}[0m"));
        assert!(output.contains("issue.sections_parseable"));
    }

    #[test]
    fn transition_options_stay_plain_when_no_color_is_set() {
        let issue = test_issue("atelier-test");
        let policy = StylePolicy::from_context(ColorChoice::Auto, true, true);

        let output = render_issue_transition_options(
            &issue,
            &[transition_option(false, false)],
            policy,
            true,
        );

        assert!(!output.contains("\u{1b}["));
        assert!(output.contains("start [blocked]"));
        assert!(output.contains("Decision: blocked"));
        assert!(output.contains("fail  issue.sections_parseable"));
    }

    #[test]
    fn transition_options_stay_plain_when_stdout_is_not_interactive() {
        let issue = test_issue("atelier-test");
        let policy = StylePolicy::from_context(ColorChoice::Auto, false, false);

        let output =
            render_issue_transition_options(&issue, &[transition_option(true, true)], policy, true);

        assert!(!output.contains("\u{1b}["));
        assert!(output.contains("start [allowed]"));
        assert!(output.contains("Decision: allowed"));
        assert!(output.contains("pass  issue.sections_parseable"));
    }

    fn write_config_and_workflow(dir: &TempDir) {
        std::fs::create_dir_all(dir.path().join(".atelier/runtime")).unwrap();
        std::fs::write(
            dir.path().join(".atelier/config.toml"),
            r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier-test"

[paths]
state_root = ".atelier"
"#,
        )
        .unwrap();
        std::fs::write(
            dir.path().join(".atelier/workflow.yaml"),
            atelier_app::workflow_policy::STARTER_POLICY_YAML,
        )
        .unwrap();
    }

    fn insert_canonical_issue(db: &Database, state_dir: &Path, issue: Issue) {
        db.cache_fixture_insert(&issue).unwrap();
        let record = CanonicalIssueRecord {
            issue,
            labels: Vec::new(),
            sections: IssueSections::unchecked_from_body(Some(
                "## Description\n\nbody\n\n## Outcome\n\nworks\n\n## Evidence\n\nproof",
            )),
            relationships: Relationships::default(),
        };
        RecordStore::new(state_dir)
            .write_issue_atomic(&record)
            .unwrap();
    }

    #[test]
    fn transition_record_stores_close_reason_in_issue_record() {
        let dir = tempdir().unwrap();
        write_config_and_workflow(&dir);
        let state_dir = dir.path().join(".atelier");
        let db = Database::open(&dir.path().join(".atelier/runtime/state.db")).unwrap();
        let mut issue = test_issue("atelier-epic1");
        issue.status = "draft".to_string();
        insert_canonical_issue(&db, &state_dir, issue.clone());
        let policy = atelier_app::workflow_policy::load(dir.path()).unwrap();
        let transition = resolve_issue_transition(&policy, &issue, "supersede").unwrap();

        let mut record = app_use_cases::load_canonical_issue(&state_dir, "atelier-epic1").unwrap();
        apply_transition_record(
            &policy,
            &state_dir,
            &mut record,
            transition,
            Some("  Replaced by a smaller epic.  "),
        )
        .unwrap();

        let stored = app_use_cases::load_canonical_issue(&state_dir, "atelier-epic1").unwrap();
        assert_eq!(stored.issue.status, "superseded");
        assert!(stored.issue.closed_at.is_some());
        assert_eq!(
            stored.sections.close_reason.as_deref(),
            Some("Replaced by a smaller epic.")
        );
    }

    #[test]
    fn non_terminal_transition_does_not_store_close_reason() {
        let dir = tempdir().unwrap();
        write_config_and_workflow(&dir);
        let state_dir = dir.path().join(".atelier");
        let db = Database::open(&dir.path().join(".atelier/runtime/state.db")).unwrap();
        let issue = test_issue("atelier-epic1");
        insert_canonical_issue(&db, &state_dir, issue.clone());
        let policy = atelier_app::workflow_policy::load(dir.path()).unwrap();
        let transition = resolve_issue_transition(&policy, &issue, "block").unwrap();

        let mut record = app_use_cases::load_canonical_issue(&state_dir, "atelier-epic1").unwrap();
        apply_transition_record(
            &policy,
            &state_dir,
            &mut record,
            transition,
            Some("waiting"),
        )
        .unwrap();

        let stored = app_use_cases::load_canonical_issue(&state_dir, "atelier-epic1").unwrap();
        assert_eq!(stored.issue.status, "blocked");
        assert!(stored.issue.closed_at.is_none());
        assert_eq!(stored.sections.close_reason, None);
    }
}
