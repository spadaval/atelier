use std::path::Path;

use anyhow::{anyhow, bail, Result};
use atelier_records::{
    issue_record_path, issue_section_diagnostic, CanonicalIssueRecord, IssueSectionName,
};

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

const EVIDENCE_PROOF_TARGET_HINT: &str = "command, transcript, evidence record, test, \
review artifact, file change, or manual check";

const CONCRETE_EVIDENCE_MARKERS: &[&str] = &[
    "command",
    "transcript",
    "evidence record",
    "evidence id",
    "test",
    "tests",
    "nextest",
    "lint",
    "doctor",
    "export",
    "review artifact",
    "review",
    "artifact",
    "file change",
    "file diff",
    "manual check",
    "manual validation",
    "screenshot",
    "stdout",
    "stderr",
    "command output",
    "help text",
    "atelier ",
    "`atelier ",
    "cargo ",
    "git diff",
    "target/debug/atelier",
    ".rs",
    ".md",
    ".toml",
    ".json",
    ".yaml",
    ".yml",
];

const VAGUE_EVIDENCE_MARKERS: &[&str] = &[
    "not specified",
    "to be determined",
    "tbd",
    "todo",
    "none yet",
    "will be added",
    "add later",
    "later",
];

/// Reports every invalid record file, record-graph integrity errors, and
/// per-issue findings. Unlike other commands, lint never stops at the first
/// invalid record file.
pub fn lint(
    request: crate::Request<LintRequest<'_>>,
) -> Result<crate::Outcome<crate::ViewModel<LintView>>> {
    let LintRequest {
        state_dir,
        issue_ref,
    } = request.input;
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "Cannot determine repository root for {}",
            state_dir.display()
        )
    })?;
    let files = read_record_files(state_dir)?;
    let scoped_id = issue_ref
        .map(|issue_ref| resolve_issue_id(&files, issue_ref))
        .transpose()?;

    let mut findings = Vec::new();
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
        let integrity_error = match other_file_errors.into_iter().next() {
            Some(first) => Some(first.message),
            None => RecordIndex::from_project_records(
                state_dir,
                files.issues.clone(),
                files.evidence.clone(),
            )
            .err()
            .map(|error| format!("Tracker record files are invalid: {error:#}")),
        };
        if let Some(message) = integrity_error {
            findings.push(LintFinding {
                id: "(canonical)".to_string(),
                message,
            });
        }
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

    Ok(crate::Outcome {
        value: crate::ViewModel {
            data: LintView { findings },
        },
    })
}

/// Per-issue findings: empty titles, workflow-policy violations, and
/// required sections. `scoped` adds the stricter single-issue checks
/// (placeholder sections and unobservable evidence entries).
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
        if !scoped {
            continue;
        }
        for (name, value) in [
            (
                IssueSectionName::Description,
                record.sections.description.as_str(),
            ),
            (IssueSectionName::Outcome, record.sections.outcome.as_str()),
        ] {
            if issue_section_placeholder(name, value) {
                findings.push(finding(format!(
                    "Issue section {} must be present and non-empty",
                    name.title()
                )));
            }
        }
        let Some(evidence) = record.sections.section(IssueSectionName::Evidence) else {
            continue;
        };
        if issue_section_placeholder(IssueSectionName::Evidence, evidence) {
            continue;
        }
        for (position, entry) in evidence_entries(evidence).iter().enumerate() {
            if !evidence_entry_names_observable_target(entry) {
                findings.push(finding(issue_section_diagnostic(
                    Some(&issue.id),
                    IssueSectionName::Evidence.title(),
                    &issue_record_path(&issue.id),
                    &format!(
                        "Issue section Evidence entry {} must name an observable proof target ({})",
                        position + 1,
                        EVIDENCE_PROOF_TARGET_HINT
                    ),
                )));
            }
        }
    }
    findings
}

fn issue_section_placeholder(name: IssueSectionName, value: &str) -> bool {
    matches!(
        (name, value.trim()),
        (IssueSectionName::Description, "No description provided.")
            | (IssueSectionName::Outcome, "Outcome was not specified.")
            | (IssueSectionName::Evidence, "Evidence was not specified.")
    )
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

fn evidence_entries(evidence: &str) -> Vec<String> {
    if evidence
        .lines()
        .any(|line| strip_markdown_list_marker(line.trim()).is_some())
    {
        let mut entries = Vec::new();
        let mut current = String::new();
        for line in evidence.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(item) = strip_markdown_list_marker(trimmed) {
                if !current.trim().is_empty() {
                    entries.push(current.trim().to_string());
                }
                current = item.trim().to_string();
            } else if current.trim().is_empty() {
                current = trimmed.to_string();
            } else {
                current.push(' ');
                current.push_str(trimmed);
            }
        }
        if !current.trim().is_empty() {
            entries.push(current.trim().to_string());
        }
        entries
    } else {
        evidence
            .split("\n\n")
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
            .map(str::to_string)
            .collect()
    }
}

fn strip_markdown_list_marker(line: &str) -> Option<&str> {
    for prefix in ["- ", "* ", "+ "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some(rest);
        }
    }

    let (digits, rest) = line.split_once('.')?;
    if !digits.is_empty()
        && digits.chars().all(|character| character.is_ascii_digit())
        && rest.starts_with(' ')
    {
        Some(rest.trim_start())
    } else {
        None
    }
}

fn evidence_entry_names_observable_target(entry: &str) -> bool {
    let lower = entry.to_lowercase();
    if VAGUE_EVIDENCE_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
    {
        return false;
    }
    CONCRETE_EVIDENCE_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
}
