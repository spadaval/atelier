use std::path::Path;

use anyhow::{anyhow, Result};
use atelier_records::{issue_record_path, CanonicalIssueRecord, IssueSectionName};

use crate::record_index::{read_record_files, RecordIndex};
use crate::record_ref::{match_id, not_found, wrong_kind, RecordKind};

/// Reserved size labels (atelier-b7hb): `size-s`, `size-m`, `size-l`,
/// `size-xl`, the form adopters already use.
pub const SIZE_LABEL_PREFIX: &str = "size-";
pub const SIZES: [&str; 4] = ["s", "m", "l", "xl"];

/// Normalizes and checks the size labels a command adds: sizes are
/// lowercase, one of the four, and a record has at most one.
pub fn size_labels(labels: &[String]) -> anyhow::Result<Vec<String>> {
    let mut sizes = 0;
    let labels = labels
        .iter()
        .map(|label| match label.strip_prefix(SIZE_LABEL_PREFIX) {
            Some(size) => {
                let size = size.to_ascii_lowercase();
                if !SIZES.contains(&size.as_str()) {
                    anyhow::bail!(
                        "{label} is not a size; use one of size-s, size-m, size-l, size-xl"
                    );
                }
                sizes += 1;
                Ok(format!("{SIZE_LABEL_PREFIX}{size}"))
            }
            None => Ok(label.clone()),
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    if sizes > 1 {
        anyhow::bail!("a record has one size; give one of size-s, size-m, size-l, size-xl");
    }
    Ok(labels)
}

pub struct LintRequest<'a> {
    pub state_dir: &'a Path,
    pub issue_ref: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LintView {
    /// The checked issue's full ID, when the check was scoped to one.
    pub scoped_id: Option<String>,
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
            id: "policy".into(),
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
            id: "records".to_string(),
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
    if findings.is_empty() {
        findings.extend(criteria_findings(
            state_dir,
            repo_root,
            scoped_id.as_deref(),
        )?);
    }

    Ok(LintView {
        scoped_id,
        findings,
    })
}

/// Per-issue findings: empty titles, workflow-policy violations, and
/// required sections. `scoped` adds the stricter single-issue checks
/// (placeholder acceptance sections).
pub fn issue_findings<'a>(
    records: impl IntoIterator<Item = &'a CanonicalIssueRecord>,
    repo_root: &Path,
    scoped: bool,
) -> Vec<LintFinding> {
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
        // Size is the reserved label size-s|m|l|xl, at most one per record.
        let sizes = record
            .labels
            .iter()
            .filter_map(|label| label.strip_prefix(SIZE_LABEL_PREFIX))
            .collect::<Vec<_>>();
        if let Some(size) = sizes.iter().find(|size| !SIZES.contains(size)) {
            findings.push(finding(format!(
                "Size label size-{size} is not one of size-s, size-m, size-l, size-xl"
            )));
        }
        if sizes.len() > 1 {
            findings.push(finding(format!(
                "Issue has {} size labels; keep one of size-s, size-m, size-l, size-xl",
                sizes.len()
            )));
        }
        if let Some(policy) = &workflow_policy {
            if let Err(error) = crate::workflow_policy::validate_issue_against_policy(
                policy,
                issue,
                &crate::record_index::record_file("issue", &issue.id),
            ) {
                findings.push(finding(error.to_string()));
            }
        }
        if let Some(adr) = &issue.adr {
            if !repo_root.join(&adr.path).is_file() {
                findings.push(finding(format!("Decision ADR {} does not exist", adr.path)));
            }
        }
        let notes = record.sections.note_entries();
        let decisions = notes
            .iter()
            .filter_map(|note| Some((note, atelier_core::DecisionNote::from_note(note)?)));
        for (number, (note, decision)) in decisions.enumerate() {
            if let Err(defect) = decision {
                findings.push(finding(format!(
                    "Decision note {} ({}) {defect}",
                    number + 1,
                    note.date
                )));
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

/// Receipts that name mission criteria (atelier-et6q): each named criterion
/// must be one the target's mission defines, so a typo or a renamed slug is
/// never silent. Scoped to one issue, only receipts targeting it.
fn criteria_findings(
    state_dir: &Path,
    repo_root: &Path,
    scoped_id: Option<&str>,
) -> Result<Vec<LintFinding>> {
    let index = RecordIndex::load(state_dir)?;
    let mut findings = Vec::new();
    for record in index.evidence_records() {
        let Some(target) = record
            .data
            .target
            .as_ref()
            .filter(|target| target.kind == "issue")
        else {
            if !record.data.criteria.is_empty() {
                findings.push(LintFinding {
                    id: record.header.id.clone(),
                    message: "Evidence names criteria but has no issue target".to_string(),
                });
            }
            continue;
        };
        if scoped_id.is_some_and(|id| id != target.id) {
            continue;
        }
        let slugs = record
            .data
            .criteria
            .iter()
            .map(|criterion| criterion.slug.as_str())
            .collect::<Vec<_>>();
        for problem in
            crate::criteria::receipt_criteria_problems(&index, repo_root, &target.id, &slugs)?
        {
            findings.push(LintFinding {
                id: record.header.id.clone(),
                message: format!("Evidence {problem}"),
            });
        }
    }
    Ok(findings)
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
                        "Strategy {path} front matter names mission {named}, but missions {} all link it; remove the `mission:` key, since each mission's strategy path is the link that counts",
                        missions.join(", ")
                    ))
                } else {
                    (named != issue.id).then(|| format!(
                        "Strategy {path} front matter names mission {named}, but {} links it; the mission's strategy path is the link that counts, so set `mission: {}` in the strategy or remove the key",
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
    if let Some(id) = match_id(RecordKind::Issue, issue_ref, ids.iter().map(String::as_str))? {
        return Ok(id);
    }
    let evidence = files
        .evidence
        .iter()
        .map(|record| record.header.id.as_str());
    let reviews = files.reviews.iter().map(|record| record.header.id.as_str());
    for (kind, other_ids) in [
        (RecordKind::Evidence, evidence.collect::<Vec<_>>()),
        (RecordKind::Review, reviews.collect::<Vec<_>>()),
    ] {
        if let Ok(Some(id)) = match_id(kind, issue_ref, other_ids) {
            return Err(wrong_kind(RecordKind::Issue, kind, &id));
        }
    }
    let lowered = issue_ref.to_ascii_lowercase();
    if lowered.contains('-')
        && lowered.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
    {
        return Ok(lowered);
    }
    Err(not_found(
        RecordKind::Issue,
        issue_ref,
        ids.iter().map(String::as_str),
    ))
}
