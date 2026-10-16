use std::path::Path;

use anyhow::{anyhow, bail, Result};
use atelier_records::{issue_record_path, CanonicalIssueRecord, IssueSectionName};

use crate::record_index::{read_record_files, RecordIndex};

/// Reserved size labels (atelier-b7hb): `size:s`, `size:m`, `size:l`, `size:xl`.
pub const SIZE_LABEL_PREFIX: &str = "size:";
const SIZES: [&str; 4] = ["s", "m", "l", "xl"];

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
        scoped_records.clone(),
        repo_root,
        scoped_id.is_some(),
    ));
    findings.extend(strategy_findings(&files.issues, scoped_records, repo_root));

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
        // Size is the reserved label size:s|m|l|xl, at most one per record.
        let sizes = record
            .labels
            .iter()
            .filter_map(|label| label.strip_prefix(SIZE_LABEL_PREFIX))
            .collect::<Vec<_>>();
        if let Some(size) = sizes.iter().find(|size| !SIZES.contains(size)) {
            findings.push(finding(format!(
                "Size label size:{size} is not one of size:s, size:m, size:l, size:xl"
            )));
        }
        if sizes.len() > 1 {
            findings.push(finding(format!(
                "Issue has {} size labels; keep one of size:s, size:m, size:l, size:xl",
                sizes.len()
            )));
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
        if !scoped {
            continue;
        }
        let mut required = record.sections.required_names();
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

/// Mission strategy links: each mission's strategy path is the canonical
/// link, so the file must exist, and front matter naming a mission must name
/// the one mission linking it.
fn strategy_findings<'a>(
    all: &[CanonicalIssueRecord],
    scoped: impl IntoIterator<Item = &'a CanonicalIssueRecord>,
    repo_root: &Path,
) -> Vec<LintFinding> {
    let normalized = |path: &str| {
        Path::new(path)
            .components()
            .filter(|part| !matches!(part, std::path::Component::CurDir))
            .collect::<std::path::PathBuf>()
    };
    let linking = |path: &str| {
        all.iter()
            .filter(|record| record.issue.issue_type == "mission")
            .filter(|record| {
                record
                    .issue
                    .strategy
                    .as_ref()
                    .is_some_and(|strategy| normalized(&strategy.path) == normalized(path))
            })
            .map(|record| record.issue.id.clone())
            .collect::<Vec<_>>()
    };
    let mut findings = Vec::new();
    for record in scoped {
        let issue = &record.issue;
        let Some(strategy) = issue
            .strategy
            .as_ref()
            .filter(|_| issue.issue_type == "mission")
        else {
            continue;
        };
        let path = &strategy.path;
        let message = match std::fs::read_to_string(repo_root.join(path)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                Some(format!("Mission strategy {path} does not exist"))
            }
            Err(error) => Some(format!("Mission strategy {path} cannot be read: {error}")),
            Ok(text) => strategy_named_mission(&text).and_then(|named| {
                let missions = linking(path);
                if missions.len() > 1 {
                    Some(format!(
                        "Strategy {path} front matter names mission {named}, but missions {} all link it; remove the `mission:` key, since each mission's strategy path is the canonical link",
                        missions.join(", ")
                    ))
                } else {
                    (named != issue.id).then(|| format!(
                        "Strategy {path} front matter names mission {named}, but {} links it; the mission's strategy path is canonical, so set `mission: {}` in the strategy or remove the key",
                        issue.id, issue.id
                    ))
                }
            }),
        };
        if let Some(message) = message {
            findings.push(LintFinding {
                id: issue.id.clone(),
                message,
            });
        }
    }
    findings
}

/// The mission a strategy's front matter names; a missing or blank key names none.
fn strategy_named_mission(text: &str) -> Option<String> {
    let text = text.replace("\r\n", "\n");
    let front_matter = atelier_records::split_document(&text).and_then(|(front_matter, _)| {
        serde_yaml::from_str::<serde_yaml::Value>(front_matter).ok()
    })?;
    match front_matter.get("mission")? {
        serde_yaml::Value::Null => None,
        serde_yaml::Value::String(named) if named.trim().is_empty() => None,
        serde_yaml::Value::String(named) => Some(named.trim().to_string()),
        serde_yaml::Value::Number(number) => Some(number.to_string()),
        serde_yaml::Value::Bool(value) => Some(value.to_string()),
        _ => Some("(a non-scalar value)".to_string()),
    }
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
