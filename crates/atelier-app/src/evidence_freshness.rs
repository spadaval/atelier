//! Honest evidence decisions used by transition checks and issue displays.
use crate::{git_freshness as git, record_index::RecordIndex};
use anyhow::Result;
use atelier_core::{EvidenceRecord, EvidenceResult};
use atelier_records::{issue_record_path, parse_issue_record, CanonicalIssueRecord};
use std::{collections::HashMap, path::Path};

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
pub fn completion_claim_matches(a: &CanonicalIssueRecord, b: &CanonicalIssueRecord) -> bool {
    acceptance_matches(a, b) && a.issue.status == b.issue.status
}
pub(crate) fn at_commit(repo: &Path, commit: &str, issue_id: &str) -> Result<CanonicalIssueRecord> {
    let relative = issue_record_path(issue_id);
    parse_issue_record(
        &git::blob(repo, commit, &Path::new(".atelier").join(&relative))?,
        &relative,
    )
}
/// A single immutable read phase. Create a new context after any mutation or
/// external command; mutable filesystem checks must never cross those fences.
pub struct EvidenceReadContext<'a> {
    repo: &'a Path,
    head: Option<String>,
    clean: Option<bool>,
    ranges: HashMap<String, Vec<git::CommitEdge>>,
}
impl<'a> EvidenceReadContext<'a> {
    pub fn new(repo: &'a Path) -> Self {
        Self {
            repo,
            head: None,
            clean: None,
            ranges: HashMap::new(),
        }
    }
    /// Reuse HEAD and dirty paths just inspected in this same read phase.
    pub fn from_snapshot(repo: &'a Path, head: String, dirty: &[std::path::PathBuf]) -> Self {
        Self {
            repo,
            head: Some(head),
            clean: Some(!dirty.iter().any(|path| !git::is_record_path(path))),
            ranges: HashMap::new(),
        }
    }
    pub fn evidence_is_fresh(
        &mut self,
        issue: &CanonicalIssueRecord,
        record: &EvidenceRecord,
    ) -> Result<bool> {
        let pin = &record.data.commit;
        if !self.ranges.contains_key(pin) {
            if self.head.is_none() {
                self.head = Some(git::head_commit(self.repo)?);
            }
            self.ranges.insert(
                pin.clone(),
                git::commit_edges_to(
                    self.repo,
                    pin,
                    self.head.as_deref().expect("read HEAD pinned"),
                )?,
            );
        }
        if !evidence_matches_edges(self.repo, issue, record, &self.ranges[pin])? {
            return Ok(false);
        }
        let clean = match self.clean {
            Some(clean) => clean,
            None => {
                let clean = git::working_code_paths(self.repo)?.is_empty();
                self.clean = Some(clean);
                clean
            }
        };
        Ok(clean)
    }
    pub fn issue_gate(
        &mut self,
        index: &RecordIndex,
        issue_id: &str,
        min_count: usize,
        kind: Option<&str>,
    ) -> Result<EvidenceGateStatus> {
        evaluate_issue_gate(index, issue_id, min_count, kind, true, |issue, record| {
            self.evidence_is_fresh(issue, record)
        })
    }
}
pub fn evidence_is_fresh(
    repo: &Path,
    issue: &CanonicalIssueRecord,
    record: &EvidenceRecord,
) -> Result<bool> {
    EvidenceReadContext::new(repo).evidence_is_fresh(issue, record)
}
fn evidence_valid_through(
    repo: &Path,
    issue: &CanonicalIssueRecord,
    record: &EvidenceRecord,
    end: &str,
) -> Result<bool> {
    evidence_matches_edges(
        repo,
        issue,
        record,
        &git::commit_edges_to(repo, &record.data.commit, end)?,
    )
}
fn evidence_matches_edges(
    repo: &Path,
    issue: &CanonicalIssueRecord,
    record: &EvidenceRecord,
    edges: &[git::CommitEdge],
) -> Result<bool> {
    let original = at_commit(repo, &record.data.commit, &issue.issue.id)?;
    if !acceptance_matches(&original, issue) {
        return Ok(false);
    }
    let target = Path::new(".atelier").join(issue_record_path(&issue.issue.id));
    for edge in edges {
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
    Ok(true)
}

pub fn issue_evidence_gate_status(
    index: &RecordIndex,
    repo: &Path,
    issue_id: &str,
    min_count: usize,
    kind: Option<&str>,
) -> Result<EvidenceGateStatus> {
    issue_evidence_gate_status_with_freshness(index, repo, issue_id, min_count, kind, true)
}
pub(crate) fn issue_evidence_gate_status_with_freshness(
    index: &RecordIndex,
    repo: &Path,
    issue_id: &str,
    min_count: usize,
    kind: Option<&str>,
    fresh: bool,
) -> Result<EvidenceGateStatus> {
    evaluate_issue_gate(index, issue_id, min_count, kind, fresh, |issue, record| {
        proof_matches(repo, issue, record, fresh)
    })
}
fn evaluate_issue_gate(
    index: &RecordIndex,
    issue_id: &str,
    min_count: usize,
    kind: Option<&str>,
    fresh: bool,
    mut proof_matches: impl FnMut(&CanonicalIssueRecord, &EvidenceRecord) -> Result<bool>,
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
            && proof_matches(issue, record).unwrap_or(false)
        {
            passing += 1;
        }
    }
    let passed = passing >= min_count.max(1);
    Ok(EvidenceGateStatus {
        passed,
        reason: if passed {
            format!(
                "{passing} {} passing evidence record(s) linked",
                if fresh { "fresh" } else { "valid historical" }
            )
        } else {
            format!("expected at least {} fresh passing evidence record(s); found {passing}; older evidence, blocked/not-applicable results and passes superseded by a newer failure do not satisfy proof", min_count.max(1))
        },
    })
}

/// A recorded fail newer than every explicit pass blocks successful completion.
pub fn newest_failure(index: &RecordIndex, repo: &Path, issue_id: &str) -> Result<Option<String>> {
    newest_failure_with_freshness(index, repo, issue_id, true)
}
pub(crate) fn newest_failure_with_freshness(
    index: &RecordIndex,
    repo: &Path,
    issue_id: &str,
    fresh: bool,
) -> Result<Option<String>> {
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
                    && proof_matches(repo, issue, record, fresh).unwrap_or(false) =>
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

fn proof_matches(
    repo: &Path,
    issue: &CanonicalIssueRecord,
    record: &EvidenceRecord,
    fresh: bool,
) -> Result<bool> {
    if fresh {
        return evidence_is_fresh(repo, issue, record);
    }
    git::git_output(
        repo,
        &["merge-base", "--is-ancestor", &record.data.commit, "HEAD"],
    )?;
    let end = claim_endpoint(repo, &issue.issue.id, &record.data.commit)?;
    evidence_valid_through(repo, issue, record, &end)
}
pub(crate) fn claim_endpoint(repo: &Path, issue_id: &str, pin: &str) -> Result<String> {
    let path = format!(".atelier/issues/{issue_id}.md");
    let changes = String::from_utf8(git::git_output(repo, &["log", "--format=%H", "--", &path])?)?;
    let mut claim = None;
    for commit in changes.lines() {
        let after = at_commit(repo, commit, issue_id)?;
        let parent = at_commit(repo, &format!("{commit}^"), issue_id);
        if parent.is_err() || !completion_claim_matches(&after, &parent?) {
            claim = Some(commit);
            break;
        }
    }
    let claim = claim.ok_or_else(|| anyhow::anyhow!("terminal claim history is missing"))?;
    let end = if git::git_output(repo, &["merge-base", "--is-ancestor", claim, pin]).is_ok() {
        pin
    } else {
        claim
    };
    Ok(end.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_records::{evidence_record, issue_record};
    use std::{fs, path::PathBuf};

    #[test]
    fn shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation() {
        let repo = tempfile::tempdir().unwrap();
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.name", "Author"],
            vec!["config", "user.email", "author@example.test"],
        ] {
            git::git_output(repo.path(), &args).unwrap();
        }
        fs::create_dir_all(repo.path().join(".atelier/issues")).unwrap();
        let issues = [
            issue_record("atelier-a001", "task", "todo"),
            issue_record("atelier-a002", "task", "todo"),
        ];
        for issue in &issues {
            fs::write(
                repo.path()
                    .join(".atelier")
                    .join(issue_record_path(&issue.issue.id)),
                atelier_records::render_issue_record(issue).unwrap(),
            )
            .unwrap();
        }
        fs::write(repo.path().join("code.rs"), "original code").unwrap();
        git::git_output(repo.path(), &["add", "."]).unwrap();
        git::git_output(repo.path(), &["commit", "-qm", "proof pin"]).unwrap();
        let pin = git::head_commit(repo.path()).unwrap();
        let mut evidence = evidence_record("atelier-v001", "atelier-a001", "validates");
        evidence.data.commit = pin;
        let mut reads = EvidenceReadContext::new(repo.path());
        for issue in &issues {
            assert!(reads.evidence_is_fresh(issue, &evidence).unwrap());
            assert!(evidence_is_fresh(repo.path(), issue, &evidence).unwrap());
        }
        assert_eq!(reads.ranges.len(), 1);
        let index =
            RecordIndex::from_records(issues.to_vec(), vec![evidence.clone()], Vec::new()).unwrap();
        for (min_count, kind) in [(1, None), (2, None), (1, Some("other"))] {
            let shared = reads
                .issue_gate(&index, "atelier-a001", min_count, kind)
                .unwrap();
            let separate =
                issue_evidence_gate_status(&index, repo.path(), "atelier-a001", min_count, kind)
                    .unwrap();
            assert_eq!(
                (shared.passed, shared.reason),
                (separate.passed, separate.reason)
            );
        }
        let mut failure = evidence.clone();
        failure.header.id = "atelier-v002".into();
        failure.header.status = "fail".into();
        failure.data.result = EvidenceResult::Fail;
        failure.data.captured_at += chrono::Duration::seconds(1);
        let failed =
            RecordIndex::from_records(issues.to_vec(), vec![evidence.clone(), failure], Vec::new())
                .unwrap();
        assert!(
            !reads
                .issue_gate(&failed, "atelier-a001", 1, None)
                .unwrap()
                .passed
        );
        fs::write(repo.path().join("code.rs"), "changed code").unwrap();
        assert!(!evidence_is_fresh(repo.path(), &issues[0], &evidence).unwrap());
        let dirty = git::working_paths(repo.path()).unwrap();
        let mut snapshot_read = EvidenceReadContext::from_snapshot(
            repo.path(),
            git::head_commit(repo.path()).unwrap(),
            &dirty,
        );
        assert!(!snapshot_read
            .evidence_is_fresh(&issues[0], &evidence)
            .unwrap());
        let mut next_read = EvidenceReadContext::new(repo.path());
        assert!(!next_read.evidence_is_fresh(&issues[0], &evidence).unwrap());
    }

    #[cfg(unix)]
    #[test]
    fn code_clean_filter_keeps_hidden_modes_and_policy_but_skips_record_modes() {
        use std::os::unix::fs::PermissionsExt;
        let repo = tempfile::tempdir().unwrap();
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.name", "Author"],
            vec!["config", "user.email", "author@example.test"],
            vec!["config", "core.filemode", "false"],
        ] {
            git::git_output(repo.path(), &args).unwrap();
        }
        fs::create_dir_all(repo.path().join(".atelier/issues")).unwrap();
        for path in [
            "code.rs",
            ".atelier/workflow.yaml",
            ".atelier/issues/atelier-a001.md",
        ] {
            fs::write(repo.path().join(path), "original bytes").unwrap();
        }
        git::git_output(repo.path(), &["add", "."]).unwrap();
        git::git_output(repo.path(), &["commit", "-qm", "initial modes"]).unwrap();
        let record = repo.path().join(".atelier/issues/atelier-a001.md");
        fs::set_permissions(&record, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(git::working_code_paths(repo.path()).unwrap().is_empty());
        assert!(git::working_paths(repo.path())
            .unwrap()
            .contains(&PathBuf::from(".atelier/issues/atelier-a001.md")));
        let code = repo.path().join("code.rs");
        fs::set_permissions(&code, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(git::working_code_paths(repo.path())
            .unwrap()
            .contains(&PathBuf::from("code.rs")));
        git::git_output(
            repo.path(),
            &[
                "update-index",
                "--assume-unchanged",
                ".atelier/workflow.yaml",
            ],
        )
        .unwrap();
        fs::write(repo.path().join(".atelier/workflow.yaml"), "changed policy").unwrap();
        assert!(git::working_code_paths(repo.path())
            .unwrap()
            .contains(&PathBuf::from(".atelier/workflow.yaml")));
    }
}
