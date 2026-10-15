use std::path::Path;

use anyhow::{anyhow, bail, Result};
use atelier_records::{issue_record_path, CanonicalIssueRecord, IssueSectionName};

use crate::record_index::{read_record_files, RecordIndex};

pub struct LintRequest<'a> {
    pub state_dir: &'a Path,
    pub issue_ref: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LintView {
    pub findings: Vec<LintFinding>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LintFinding {
    pub id: String,
    pub message: String,
}

/// Reports every invalid record file, record-graph integrity errors, and
/// per-issue findings. Unlike other commands, lint never stops at the first
/// invalid record file.
pub fn lint(request: LintRequest<'_>) -> Result<LintView> {
    let LintRequest {
        state_dir,
        issue_ref,
    } = request;
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "Cannot determine repository root for {}",
            state_dir.display()
        )
    })?;
    let files = read_record_files(state_dir)?;
    let policy_error = atelier_workflow::load(repo_root).err();
    let scoped_id = issue_ref
        .map(|issue_ref| resolve_issue_id(&files, issue_ref))
        .transpose()?;

    let mut findings = Vec::new();
    if let Some(error) = policy_error {
        findings.push(LintFinding {
            id: "(policy)".into(),
            message: format!("Invalid workflow policy: {error:#}"),
        });
    }
    let mut other_file_errors = Vec::new();
    for file_error in &files.errors {
        let stem = file_error
            .path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("(unknown)")
            .to_string();
        let finding = LintFinding {
            message: format!("Tracker record files are invalid: {:#}", file_error.error),
            id: stem,
        };
        match &scoped_id {
            Some(id) if file_error.path != issue_record_path(id) => other_file_errors.push(finding),
            _ => findings.push(finding),
        }
    }
    let scoped_record_missing = scoped_id.as_ref().is_some_and(|id| {
        !files.issues.iter().any(|record| &record.issue.id == id)
            && !files
                .errors
                .iter()
                .any(|error| error.path == issue_record_path(id))
    });
    if let (Some(id), true) = (&scoped_id, scoped_record_missing) {
        findings.push(LintFinding {
            id: id.clone(),
            message: format!(
                "Tracker record files are invalid: Missing record file .atelier/{}",
                issue_record_path(id).display()
            ),
        });
    }
    if findings.is_empty() {
        let integrity_errors = match other_file_errors.into_iter().next() {
            Some(first) => vec![first.message],
            None => RecordIndex::validation_errors(
                state_dir,
                files.issues.clone(),
                files.evidence.clone(),
                files.reviews.clone(),
            )
            .into_iter()
            .map(|error| format!("Tracker record files are invalid: {error:#}"))
            .collect(),
        };
        findings.extend(integrity_errors.into_iter().map(|message| LintFinding {
            id: "(canonical)".to_string(),
            message,
        }));
    }

    let scoped_records = files
        .issues
        .iter()
        .filter(|record| scoped_id.as_ref().is_none_or(|id| &record.issue.id == id));
    findings.extend(issue_findings(
        scoped_records,
        repo_root,
        scoped_id.is_some(),
    ));

    Ok(LintView { findings })
}

/// Per-issue findings: empty titles, workflow-policy violations, and
/// required sections. `scoped` adds the stricter single-issue checks
/// (placeholder acceptance sections).
pub fn issue_findings<'a>(
    records: impl IntoIterator<Item = &'a CanonicalIssueRecord>,
    repo_root: &Path,
    scoped: bool,
) -> Vec<LintFinding> {
    let policy_path = repo_root.join(crate::workflow_policy::WORKFLOW_POLICY_PATH);
    let workflow_policy = crate::workflow_policy::load(repo_root).ok();
    let mut findings = Vec::new();
    for record in records {
        let issue = &record.issue;
        let finding = |message: String| LintFinding {
            id: issue.id.clone(),
            message,
        };
        if issue.title.trim().is_empty() {
            findings.push(finding("Issue title must not be empty".to_string()));
        }
        if let Some(policy) = &workflow_policy {
            if let Err(error) =
                crate::workflow_policy::validate_issue_against_policy(policy, issue, &policy_path)
            {
                findings.push(finding(error.to_string()));
            }
        }
        for state in record.sections.section_states() {
            if state.required && (!state.present || state.empty) {
                findings.push(finding(format!(
                    "Issue section {} must be present and non-empty",
                    state.name.title()
                )));
            }
        }
        if issue.issue_type == "mission"
            && issue.strategy.is_none()
            && record
                .sections
                .section(IssueSectionName::SuccessCriteria)
                .is_none()
        {
            findings.push(finding(
                "Mission requires Success criteria or a linked strategy".to_string(),
            ));
        }
        if issue.issue_type == "epic"
            && issue.status != "draft"
            && record.sections.section(IssueSectionName::Proof).is_none()
        {
            findings.push(finding("Ready epic requires Proof".to_string()));
        }
        if !scoped {
            continue;
        }
        let mut required = record.sections.required_names();
        if issue.issue_type == "epic" && issue.status != "draft" {
            required.push(IssueSectionName::Proof);
        }
        if issue.issue_type == "mission" && issue.strategy.is_none() {
            required.push(IssueSectionName::SuccessCriteria);
        }
        for name in required {
            if record.sections.section(name).is_none_or(|value| {
                value.trim().ends_with("was not specified.")
                    || value.trim().ends_with("were not specified.")
            }) {
                findings.push(finding(format!(
                    "Issue section {} must be present and non-empty",
                    name.title()
                )));
            }
        }
    }
    findings
}

/// Resolves an issue reference against the record files, including files
/// that failed to parse, so a broken issue can still be checked by id.
fn resolve_issue_id(files: &crate::record_index::RecordFiles, issue_ref: &str) -> Result<String> {
    let issue_ref = issue_ref.trim();
    let mut ids = files
        .issues
        .iter()
        .map(|record| record.issue.id.clone())
        .chain(files.errors.iter().filter_map(|error| {
            error
                .path
                .strip_prefix("issues")
                .ok()
                .and_then(|path| path.file_stem())
                .and_then(|stem| stem.to_str())
                .map(str::to_string)
        }))
        .collect::<Vec<_>>();
    ids.sort();
    ids.dedup();
    if ids.iter().any(|id| id == issue_ref) {
        return Ok(issue_ref.to_string());
    }
    if !issue_ref.is_empty()
        && issue_ref
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
    {
        let suffix = format!("-{issue_ref}");
        let matches = ids
            .iter()
            .filter(|id| id.ends_with(&suffix))
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [id] => return Ok((*id).clone()),
            [] => {}
            _ => bail!(
                "Issue key {issue_ref} is ambiguous: {}",
                matches
                    .iter()
                    .map(|id| id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
    if files
        .evidence
        .iter()
        .any(|record| record.header.id == issue_ref)
    {
        bail!(
            "{issue_ref} is a evidence record, not an issue record. Use `atelier evidence show {issue_ref}`."
        );
    }
    if issue_ref.contains('-')
        && issue_ref.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
    {
        return Ok(issue_ref.to_string());
    }
    Err(anyhow!("Issue {issue_ref} was not found"))
}
