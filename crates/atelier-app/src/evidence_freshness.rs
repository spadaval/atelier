//! Honest evidence decisions used by transition checks and issue displays.
use crate::{git_freshness as git, record_index::RecordIndex};
use anyhow::Result;
use atelier_core::{EvidenceRecord, EvidenceResult};
use atelier_records::{issue_record_path, parse_issue_record, CanonicalIssueRecord};
use std::path::Path;

#[derive(Debug)]
pub struct EvidenceGateStatus {
    pub passed: bool,
    pub reason: String,
}

fn acceptance_matches(a: &CanonicalIssueRecord, b: &CanonicalIssueRecord) -> bool {
    a.issue.title == b.issue.title
        && a.issue.issue_type == b.issue.issue_type
        && a.issue.strategy == b.issue.strategy
        && a.sections.body == b.sections.body
}
fn at_commit(repo: &Path, commit: &str, issue_id: &str) -> Result<CanonicalIssueRecord> {
    let relative = issue_record_path(issue_id);
    parse_issue_record(
        &git::blob(repo, commit, &Path::new(".atelier").join(&relative))?,
        &relative,
    )
}
pub fn evidence_is_fresh(
    repo: &Path,
    issue: &CanonicalIssueRecord,
    record: &EvidenceRecord,
) -> Result<bool> {
    let original = at_commit(repo, &record.data.commit, &issue.issue.id)?;
    if !acceptance_matches(&original, issue) {
        return Ok(false);
    }
    let target = Path::new(".atelier").join(issue_record_path(&issue.issue.id));
    for edge in git::commit_edges(repo, &record.data.commit)? {
        if edge.paths.iter().any(|path| !git::is_record_path(path)) {
            return Ok(false);
        }
        if edge.paths.contains(&target)
            && !acceptance_matches(
                &at_commit(repo, &edge.parent, &issue.issue.id)?,
                &at_commit(repo, &edge.commit, &issue.issue.id)?,
            )
        {
            return Ok(false);
        }
    }
    Ok(!git::working_paths(repo)?
        .iter()
        .any(|path| !git::is_record_path(path)))
}

pub fn issue_evidence_gate_status(
    index: &RecordIndex,
    repo: &Path,
    issue_id: &str,
    min_count: usize,
    kind: Option<&str>,
) -> Result<EvidenceGateStatus> {
    let issue = index
        .issue_record(issue_id)
        .ok_or_else(|| anyhow::anyhow!("Issue {issue_id} not found"))?;
    let mut records = Vec::new();
    for link in index.list_record_links("issue", issue_id)? {
        if link.relation_type != "validates" {
            continue;
        }
        let evidence_id = if link.source_kind == "evidence" {
            link.source_id
        } else if link.target_kind == "evidence" {
            link.target_id
        } else {
            continue;
        };
        let record = index.require_evidence(&evidence_id)?;
        if kind.is_none_or(|kind| record.data.evidence_type == kind) {
            records.push(record);
        }
    }
    records.sort_by_key(|record| (record.data.captured_at, &record.header.id));
    // A new failure supersedes earlier passing proof. Blocked and N/A never count.
    let last_failure = records
        .iter()
        .filter(|record| record.data.result == EvidenceResult::Fail)
        .map(|record| record.data.captured_at)
        .max();
    let mut passing = 0;
    for record in &records {
        if record.data.result == EvidenceResult::Pass
            && last_failure.is_none_or(|failed_at| record.data.captured_at > failed_at)
            && evidence_is_fresh(repo, issue, record).unwrap_or(false)
        {
            passing += 1;
        }
    }
    let passed = passing >= min_count.max(1);
    Ok(EvidenceGateStatus {
        passed,
        reason: if passed {
            format!("{passing} fresh passing evidence record(s) linked")
        } else {
            format!("expected at least {} fresh passing evidence record(s); found {passing}; older evidence, blocked/not-applicable results and passes superseded by a newer failure do not satisfy proof", min_count.max(1))
        },
    })
}

/// A recorded fail newer than every explicit pass blocks successful completion.
pub fn newest_failure(index: &RecordIndex, repo: &Path, issue_id: &str) -> Result<Option<String>> {
    let issue = index
        .issue_record(issue_id)
        .ok_or_else(|| anyhow::anyhow!("Issue {issue_id} not found"))?;
    let mut newest_fail = None;
    let mut newest_pass = None;
    for link in index.list_record_links("issue", issue_id)? {
        if link.relation_type != "validates" {
            continue;
        }
        let id = if link.source_kind == "evidence" {
            &link.source_id
        } else if link.target_kind == "evidence" {
            &link.target_id
        } else {
            continue;
        };
        let record = index.require_evidence(id)?;
        let candidate = (record.data.captured_at, record.header.id.clone());
        match record.data.result {
            EvidenceResult::Fail if newest_fail.as_ref().is_none_or(|old| &candidate > old) => {
                newest_fail = Some(candidate)
            }
            EvidenceResult::Pass
                if newest_pass.as_ref().is_none_or(|old| &candidate > old)
                    && evidence_is_fresh(repo, issue, record).unwrap_or(false) =>
            {
                newest_pass = Some(candidate)
            }
            _ => {}
        }
    }
    Ok(newest_fail
        .filter(|failed| {
            newest_pass
                .as_ref()
                .is_none_or(|passed| passed.0 <= failed.0)
        })
        .map(|(_, id)| id))
}
