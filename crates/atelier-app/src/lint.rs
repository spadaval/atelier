use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Result};
use atelier_core::Issue;
use atelier_records::{issue_record_path, issue_section_diagnostic, IssueSectionName, RecordStore};
use atelier_sqlite::Database;

pub struct LintRequest<'a> {
    pub db: &'a Database,
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

pub fn lint(
    request: crate::Request<LintRequest<'_>>,
) -> Result<crate::Outcome<crate::ViewModel<LintView>>> {
    let input = request.input;
    let issues = if let Some(issue_ref) = input.issue_ref {
        let id = resolve_issue_id(input.db, issue_ref)?;
        vec![input.db.require_issue(&id)?]
    } else {
        input.db.list_issues(Some("all"), None, None)?
    };
    let canonical_state_dir = crate::storage_layout::find_canonical_dir_from_cwd()?;
    let (canonical_issues, canonical_findings) = if let Some(state_dir) = &canonical_state_dir {
        let store = RecordStore::new(&state_dir);
        let mut records = BTreeMap::new();
        let mut findings = Vec::new();
        let paths = if let Some(issue_ref) = input.issue_ref {
            let id = resolve_issue_id(input.db, issue_ref)?;
            vec![issue_record_path(&id)]
        } else {
            match store.discover_issue_paths() {
                Ok(paths) => paths,
                Err(error) => {
                    findings.push(LintFinding {
                        id: "(canonical)".to_string(),
                        message: format!("Canonical tracker Markdown is invalid: {error:#}"),
                    });
                    Vec::new()
                }
            }
        };
        for relative in paths {
            match store.load_issue(&relative) {
                Ok(record) => {
                    records.insert(record.issue.id.clone(), record);
                }
                Err(error) => {
                    findings.push(LintFinding {
                        id: relative
                            .file_stem()
                            .and_then(|stem| stem.to_str())
                            .unwrap_or("(unknown)")
                            .to_string(),
                        message: format!("Canonical tracker Markdown is invalid: {error:#}"),
                    });
                }
            }
        }
        if findings.is_empty() {
            if let Err(error) = crate::rebuild::validate_canonical_state(state_dir) {
                findings.push(LintFinding {
                    id: "(canonical)".to_string(),
                    message: format!("Canonical tracker Markdown is invalid: {error:#}"),
                });
            }
        }
        (records, findings)
    } else {
        (BTreeMap::new(), Vec::new())
    };

    let mut findings = canonical_findings;
    for issue in issues {
        if issue.title.trim().is_empty() {
            findings.push(LintFinding {
                id: issue_id_for_agent(&issue),
                message: "Issue title must not be empty".to_string(),
            });
        }
        if !atelier_sqlite::VALID_ISSUE_TYPES.contains(&issue.issue_type.as_str()) {
            findings.push(LintFinding {
                id: issue_id_for_agent(&issue),
                message: format!("Issue type '{}' is not valid", issue.issue_type),
            });
        }
        for blocker_id in input.db.get_blockers(&issue.id)? {
            if input.db.get_issue(&blocker_id)?.is_none() {
                findings.push(LintFinding {
                    id: issue_id_for_agent(&issue),
                    message: format!("Dependency references missing issue {blocker_id}"),
                });
            }
        }
        if let Some(record) = canonical_issues.get(&issue.id) {
            for state in record.sections.section_states() {
                if state.required && (!state.present || state.empty) {
                    findings.push(LintFinding {
                        id: issue_id_for_agent(&issue),
                        message: format!(
                            "Issue section {} must be present and non-empty",
                            state.name.title()
                        ),
                    });
                }
            }
            if input.issue_ref.is_some() {
                for (name, value) in [
                    (
                        IssueSectionName::Description,
                        record.sections.description.as_str(),
                    ),
                    (IssueSectionName::Outcome, record.sections.outcome.as_str()),
                    (
                        IssueSectionName::Evidence,
                        record.sections.evidence.as_str(),
                    ),
                ] {
                    if issue_section_placeholder(name, value) {
                        findings.push(LintFinding {
                            id: issue_id_for_agent(&issue),
                            message: format!(
                                "Issue section {} must be present and non-empty",
                                name.title()
                            ),
                        });
                    }
                }
            }
            if issue_requires_concrete_evidence(&issue) {
                for (index, entry) in evidence_entries(&record.sections.evidence)
                    .iter()
                    .enumerate()
                {
                    if !evidence_entry_names_observable_target(entry) {
                        let relative = issue_record_path(&issue.id);
                        let message = issue_section_diagnostic(
                            Some(&issue.id),
                            IssueSectionName::Evidence.title(),
                            &relative,
                            &format!(
                                "Issue section Evidence entry {} must name an observable proof target ({})",
                                index + 1,
                                EVIDENCE_PROOF_TARGET_HINT
                            ),
                        );
                        findings.push(LintFinding {
                            id: issue_id_for_agent(&issue),
                            message,
                        });
                    }
                }
            }
        }
    }

    Ok(crate::Outcome {
        value: crate::ViewModel {
            data: LintView { findings },
        },
    })
}

fn issue_section_placeholder(name: IssueSectionName, value: &str) -> bool {
    matches!(
        (name, value.trim()),
        (IssueSectionName::Description, "No description provided.")
            | (IssueSectionName::Outcome, "Outcome was not specified.")
            | (IssueSectionName::Evidence, "Evidence was not specified.")
    )
}

pub fn resolve_issue_id(db: &Database, issue_ref: &str) -> Result<String> {
    if let Some(id) = db.resolve_issue_ref(issue_ref)? {
        return Ok(id);
    }

    if let Some(actual_kind) = db.record_kind_for_id(issue_ref)? {
        bail!(
            "{} is a {} record, not an issue record. Use `{}`.",
            issue_ref,
            actual_kind,
            show_command_for_kind(&actual_kind, issue_ref)
        );
    }

    Err(anyhow!("Issue {issue_ref} was not found"))
}

fn show_command_for_kind(kind: &str, id: &str) -> String {
    match kind {
        "mission" => format!("atelier mission show {id}"),
        "evidence" => format!("atelier evidence show {id}"),
        _ => format!("atelier {kind} show {id}"),
    }
}

fn issue_id_for_agent(issue: &Issue) -> String {
    issue.id.clone()
}

fn issue_requires_concrete_evidence(issue: &Issue) -> bool {
    !matches!(issue.status.as_str(), "done" | "archived") && issue.issue_type != "epic"
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

#[allow(dead_code)]
fn canonical_issue_path_from_state(state_dir: &Path, issue_id: &str) -> PathBuf {
    state_dir.join("issues").join(format!("{issue_id}.md"))
}
