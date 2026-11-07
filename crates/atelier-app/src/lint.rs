use std::path::Path;

use anyhow::{anyhow, Result};
use atelier_records::{issue_record_path, CanonicalIssueRecord};

use crate::record_index::{read_record_files, RecordFiles, RecordIndex};
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
    /// Old record forms Atelier still reads, each naming the migration that
    /// rewrites them; they pass the check (atelier-l457).
    pub notes: Vec<String>,
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
    let files = read_record_files(state_dir)?;
    let scoped_id = issue_ref
        .map(|issue_ref| resolve_issue_id(&files, issue_ref))
        .transpose()?;
    let (findings, index) = lint_records(state_dir, files, scoped_id.as_deref(), true)?;
    let notes = index
        .map(|index| old_form_notes(&index, state_dir, scoped_id.as_deref()))
        .unwrap_or_default();
    Ok(LintView {
        scoped_id,
        findings,
        notes,
    })
}

/// The old record forms still read, each with the migration that rewrites
/// it: separate disposition events (before ADR 0025) and open After merge
/// lines (before loose ends, ADR 0028). Version 1 reviews are read as
/// written and get no note, since approvals stay as committed.
fn old_form_notes(index: &RecordIndex, state_dir: &Path, scoped_id: Option<&str>) -> Vec<String> {
    let mut notes = Vec::new();
    let events = index
        .retired_dispositions()
        .filter(|event| scoped_id.is_none_or(|id| event.target == id))
        .count();
    if events > 0 {
        notes.push(format!(
            "Note: {} in the form before ADR 0025, read folded into their findings. `atelier migrate dispositions --apply` writes the fold.",
            atelier_core::count_noun(events, "separate disposition event")
        ));
    }
    let policy = state_dir
        .parent()
        .and_then(|root| crate::workflow_policy::load(root).ok());
    let done = |record: &CanonicalIssueRecord| {
        crate::workflow_policy::status_is_done(policy.as_ref(), &record.issue.status)
    };
    let lines = crate::after_merge::open_lines(index, done)
        .into_iter()
        .filter(|open| scoped_id.is_none_or(|id| open.item.issue.id == id))
        .count();
    if lines > 0 {
        notes.push(format!(
            "Note: {} in the form before loose ends. `atelier migrate after-merge` turns each into a loose end.",
            atelier_core::count_noun(lines, "open After merge line")
        ));
    }
    notes
}

/// The findings `atelier check` reports for `files`, the records read from
/// `state_dir` or as a write would leave them (atelier-ilba). `explain`
/// says where a record missing from them is instead; without it, two sets
/// of records compare by their findings alone.
pub(crate) fn lint_files(
    state_dir: &Path,
    files: RecordFiles,
    scoped_id: Option<&str>,
    explain: bool,
) -> Result<Vec<LintFinding>> {
    lint_records(state_dir, files, scoped_id, explain).map(|(findings, _)| findings)
}

/// [`lint_files`], with the record index when every record loaded.
fn lint_records(
    state_dir: &Path,
    files: RecordFiles,
    scoped_id: Option<&str>,
    explain: bool,
) -> Result<(Vec<LintFinding>, Option<RecordIndex>)> {
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "Cannot determine repository root for {}",
            state_dir.display()
        )
    })?;
    let policy_error = atelier_workflow::load(repo_root).err();

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
        match scoped_id {
            Some(id) if file_error.path != issue_record_path(id) => other_file_errors.push(finding),
            _ => findings.push(finding),
        }
    }
    let scoped_record_missing = scoped_id.is_some_and(|id| {
        !files.issues.iter().any(|record| record.issue.id == id)
            && !files
                .errors
                .iter()
                .any(|error| error.path == issue_record_path(id))
    });
    if let (Some(id), true) = (scoped_id, scoped_record_missing) {
        findings.push(LintFinding {
            id: id.to_owned(),
            message: format!(
                "Tracker record files are invalid: Missing record file .atelier/{}",
                issue_record_path(id).display()
            ),
        });
    }
    // The record graph, when every file parsed, which the receipt criteria
    // read once it is valid.
    let mut graph = None;
    if findings.is_empty() {
        let integrity_errors = match other_file_errors.into_iter().next() {
            Some(first) => vec![first.message],
            None => {
                let (index, errors) = RecordIndex::checked(
                    state_dir,
                    files.issues.clone(),
                    files.evidence.clone(),
                    files.reviews.clone(),
                    explain,
                );
                graph = Some(index);
                errors
                    .into_iter()
                    .map(|error| format!("Tracker record files are invalid: {error:#}"))
                    .collect()
            }
        };
        findings.extend(integrity_errors.into_iter().map(|message| LintFinding {
            id: "records".to_string(),
            message,
        }));
    }

    // Receipt criteria need a loadable index, so only invalid records hold
    // them back, not unrelated issue findings.
    let records_valid = findings.is_empty();
    let scoped_records = files
        .issues
        .iter()
        .filter(|record| scoped_id.is_none_or(|id| record.issue.id == id));
    findings.extend(issue_findings(
        scoped_records.clone(),
        repo_root,
        scoped_id.is_some(),
    ));
    findings.extend(strategy_findings(&files.issues, scoped_records, repo_root));
    findings.extend(loose_end_findings(&files.issues, scoped_id));
    let mut loaded = None;
    if let (true, Some(index)) = (records_valid, graph) {
        let index = index
            .with_base_exemptions(files.base_exemptions)?
            .with_retired_dispositions(files.retired_dispositions)?;
        findings.extend(criteria_findings(&index, repo_root, scoped_id)?);
        loaded = Some(index);
    }
    Ok((findings, loaded))
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
        if !scoped {
            continue;
        }
        for name in record.sections.required_names() {
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

/// A loose end waits on what it follows through `blocked_by` and is never
/// part of a mission: it is no issue's child and advances no mission
/// (atelier-8bh4). The record guard refuses the write that links one, since
/// it would add a finding. Scoped to one issue, only the links it is an end
/// of.
fn loose_end_findings(all: &[CanonicalIssueRecord], scoped_id: Option<&str>) -> Vec<LintFinding> {
    let loose_end = |id: &str| {
        all.iter().any(|record| {
            record.issue.id == id && record.issue.issue_type == atelier_workflow::LOOSE_END_TYPE
        })
    };
    let mut findings = Vec::new();
    for record in all {
        let id = &record.issue.id;
        for child in &record.relationships.children {
            if child.kind == "issue"
                && loose_end(&child.id)
                && scoped_id.is_none_or(|scoped| scoped == id || scoped == child.id)
            {
                findings.push(LintFinding {
                    id: child.id.clone(),
                    message: format!(
                        "Loose end {child} is a child of {id}, but a loose end is never part of another issue's work; it waits on what it follows. Next: `atelier issue update {child} --no-parent`, then `atelier issue link {child} {id} --role blocked_by`",
                        child = child.id
                    ),
                });
            }
        }
        for relation in record
            .relationships
            .relates
            .iter()
            .filter(|relation| relation.kind == "issue" && relation.relation_type == "advances")
        {
            let (end, other) = if loose_end(id) {
                (id.as_str(), relation.id.as_str())
            } else if loose_end(&relation.id) {
                (relation.id.as_str(), id.as_str())
            } else {
                continue;
            };
            if scoped_id.is_some_and(|scoped| scoped != end && scoped != other) {
                continue;
            }
            findings.push(LintFinding {
                id: end.to_owned(),
                message: format!(
                    "Loose end {end} advances {other}, but a loose end is never part of a mission, so it never holds one open. Next: `atelier issue unlink {id} {} --role advances`, and link the loose end to the work it waits on with `--role blocked_by`",
                    relation.id
                ),
            });
        }
    }
    findings
}

/// Receipts that name mission criteria (atelier-et6q): each named criterion
/// must be one the receipt's stored `criteria_mission` defines, so a typo or
/// a renamed slug is never silent. Scoped to one issue, only receipts
/// targeting it or stored against it as their mission; for a mission, also
/// receipts targeting an item in its scope.
fn criteria_findings(
    index: &RecordIndex,
    repo_root: &Path,
    scoped_id: Option<&str>,
) -> Result<Vec<LintFinding>> {
    let scope = match scoped_id {
        Some(id)
            if index
                .issue_record(id)
                .is_some_and(|record| record.issue.issue_type == "mission") =>
        {
            let mut scope = crate::objective_graph::mission_issue_ids(index, id)?;
            scope.insert(id.to_string());
            Some(scope)
        }
        Some(id) => Some(std::collections::BTreeSet::from([id.to_string()])),
        None => None,
    };
    let mut findings = Vec::new();
    for record in index.evidence_records() {
        let Some(mission) = record.data.criteria_mission.as_deref() else {
            continue;
        };
        let target = record
            .data
            .target
            .as_ref()
            .filter(|target| target.kind == "issue")
            .map(|target| target.id.as_str());
        let in_scope = |id: &str| scope.as_ref().is_none_or(|scope| scope.contains(id));
        if !in_scope(mission) && !target.is_some_and(in_scope) {
            continue;
        }
        let slugs = record
            .data
            .criteria
            .iter()
            .map(|criterion| criterion.slug.as_str())
            .collect::<Vec<_>>();
        for problem in crate::criteria::undefined_criteria(index, repo_root, mission, &slugs)? {
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
