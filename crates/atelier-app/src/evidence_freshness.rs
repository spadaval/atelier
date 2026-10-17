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
pub fn completion_claim_matches(a: &CanonicalIssueRecord, b: &CanonicalIssueRecord) -> bool {
    acceptance_matches(a, b) && a.issue.status == b.issue.status
}
/// A single read phase; create a new context after mutations or external checks.
pub type EvidenceReadContext<'a> = git::ReadContext<'a>;
impl EvidenceReadContext<'_> {
    pub fn issue_at(&mut self, commit: &str, issue_id: &str) -> Result<CanonicalIssueRecord> {
        let relative = issue_record_path(issue_id);
        parse_issue_record(
            &self.blob(commit, &Path::new(".atelier").join(&relative))?,
            &relative,
        )
    }
    /// The record at `commit`, read as `like`'s type when it still has the
    /// retired epic shape, so an `atelier migrate epics` rewrite does not
    /// look like a new completion claim.
    fn issue_at_migrated(
        &mut self,
        commit: &str,
        issue_id: &str,
        like: &CanonicalIssueRecord,
    ) -> Result<CanonicalIssueRecord> {
        let error = match self.issue_at(commit, issue_id) {
            Ok(record) => return Ok(record),
            Err(error) => error,
        };
        let relative = issue_record_path(issue_id);
        let text = self.blob(commit, &Path::new(".atelier").join(&relative))?;
        // Keep a draft status as it was; only the shape changes here.
        match crate::epic_migration::migrate_epic_text(&text, &like.issue.issue_type, "draft") {
            Ok(Some(migrated)) => parse_issue_record(&migrated.text, &relative),
            _ => Err(error),
        }
    }
    pub fn evidence_is_fresh(
        &mut self,
        issue: &CanonicalIssueRecord,
        record: &EvidenceRecord,
    ) -> Result<bool> {
        Ok(self.matches_commits_to_head(issue, record)? && self.code_clean()?)
    }
    /// Whether no commit since the pin changed what `record` proves; fresh
    /// proof also needs a clean checkout.
    fn matches_commits_to_head(
        &mut self,
        issue: &CanonicalIssueRecord,
        record: &EvidenceRecord,
    ) -> Result<bool> {
        let edges = self.edges(&record.data.commit, "HEAD")?;
        evidence_matches_edges(self, issue, record, &edges)
    }
    pub fn issue_gate(
        &mut self,
        index: &RecordIndex,
        issue_id: &str,
        min_count: usize,
        kind: Option<&str>,
    ) -> Result<EvidenceGateStatus> {
        self.issue_gate_with_freshness(index, issue_id, min_count, kind, true)
    }
    pub(crate) fn issue_gate_with_freshness(
        &mut self,
        index: &RecordIndex,
        issue_id: &str,
        min_count: usize,
        kind: Option<&str>,
        fresh: bool,
    ) -> Result<EvidenceGateStatus> {
        evaluate_issue_gate(index, issue_id, min_count, kind, fresh, |issue, record| {
            self.proof_matches(issue, record, fresh)
        })
    }
}
fn evidence_matches_edges(
    read: &mut EvidenceReadContext<'_>,
    issue: &CanonicalIssueRecord,
    record: &EvidenceRecord,
    edges: &[git::CommitEdge],
) -> Result<bool> {
    let original = read.issue_at(&record.data.commit, &issue.issue.id)?;
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
                &read.issue_at(&edge.parent, &issue.issue.id)?,
                &read.issue_at(&edge.commit, &issue.issue.id)?,
            )
        {
            return Ok(false);
        }
    }
    Ok(true)
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
        .ok_or_else(|| index.not_found(crate::record_ref::RecordKind::Issue, issue_id))?;
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
                "{} linked",
                atelier_core::count_noun(
                    passing,
                    if fresh {
                        "fresh passing evidence record"
                    } else {
                        "valid historical passing evidence record"
                    }
                )
            )
        } else {
            format!("expected at least {}; found {passing}; older evidence, blocked/not-applicable results and passes superseded by a newer failure do not satisfy proof", atelier_core::count_noun(min_count.max(1), "fresh passing evidence record"))
        },
    })
}

/// A recorded fail newer than every explicit pass blocks successful completion.
pub fn newest_failure(index: &RecordIndex, repo: &Path, issue_id: &str) -> Result<Option<String>> {
    Ok(EvidenceReadContext::new(repo)
        .newest_failure(index, issue_id, true)?
        .map(|failure| failure.id))
}

/// A failing evidence record that no current pass supersedes.
pub(crate) struct NewestFailure {
    pub(crate) id: String,
    /// A newer pass that does not prove the current checkout or commit,
    /// such as one made stale by uncommitted changes (atelier-749l).
    pub(crate) stale_pass: Option<String>,
    /// Whether that pass would still prove the claim with uncommitted
    /// changes set aside, so the checkout alone makes it stale (atelier-x3bj).
    pub(crate) stale_from_checkout: bool,
}

impl EvidenceReadContext<'_> {
    pub(crate) fn newest_failure(
        &mut self,
        index: &RecordIndex,
        issue_id: &str,
        fresh: bool,
    ) -> Result<Option<NewestFailure>> {
        let issue = index
            .issue_record(issue_id)
            .ok_or_else(|| index.not_found(crate::record_ref::RecordKind::Issue, issue_id))?;
        let mut newest_fail = None;
        let mut newest_pass = None;
        let mut newest_stale_pass: Option<(_, String, bool)> = None;
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
                EvidenceResult::Pass if newest_pass.as_ref().is_none_or(|old| &candidate > old) => {
                    if self.proof_matches(issue, record, fresh).unwrap_or(false) {
                        newest_pass = Some(candidate)
                    } else if newest_stale_pass
                        .as_ref()
                        .is_none_or(|old| (&candidate.0, &candidate.1) > (&old.0, &old.1))
                    {
                        // Stale only because of the checkout: no commit since
                        // the pin changed what it proves, but code is dirty.
                        let from_checkout = fresh
                            && self.matches_commits_to_head(issue, record).unwrap_or(false)
                            && !self.code_clean().unwrap_or(true);
                        newest_stale_pass = Some((candidate.0, candidate.1, from_checkout))
                    }
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
            .map(|(failed_at, id)| {
                let stale = newest_stale_pass.filter(|(passed_at, _, _)| *passed_at > failed_at);
                NewestFailure {
                    id,
                    stale_from_checkout: stale.as_ref().is_some_and(|(_, _, checkout)| *checkout),
                    stale_pass: stale.map(|(_, id, _)| id),
                }
            }))
    }

    fn proof_matches(
        &mut self,
        issue: &CanonicalIssueRecord,
        record: &EvidenceRecord,
        fresh: bool,
    ) -> Result<bool> {
        if fresh {
            return self.evidence_is_fresh(issue, record);
        }
        if !self.ancestor(&record.data.commit, "HEAD")? {
            return Ok(false);
        }
        let end = self.claim_endpoint(&issue.issue.id, &record.data.commit)?;
        let edges = self.edges(&record.data.commit, &end)?;
        evidence_matches_edges(self, issue, record, &edges)
    }
    pub(crate) fn claim_endpoint(&mut self, issue_id: &str, pin: &str) -> Result<String> {
        let head = self.pinned("HEAD")?;
        let query = claim_history_query(&head, issue_id);
        let changes = self.history(&query.iter().map(String::as_str).collect::<Vec<_>>())?;
        let current = self.issue_at(&head, issue_id)?;
        let mut claim = None;
        for commit in changes.lines() {
            let after = self.issue_at_migrated(commit, issue_id, &current)?;
            let parent = self.issue_at_migrated(&format!("{commit}^"), issue_id, &current);
            if parent.is_err() || !completion_claim_matches(&after, &parent?) {
                claim = Some(commit);
                break;
            }
        }
        let claim = claim.ok_or_else(|| anyhow::anyhow!("terminal claim history is missing"))?;
        let end = if self.ancestor(claim, pin)? {
            pin
        } else {
            claim
        };
        Ok(end.to_owned())
    }
}

/// The `git log` behind [`EvidenceReadContext::claim_endpoint`]: every commit
/// that touched the issue record, newest first. Shared so callers can
/// prefetch it (atelier-v5tt).
pub(crate) fn claim_history_query(head: &str, issue_id: &str) -> Vec<String> {
    [
        "log".to_owned(),
        "--root".to_owned(),
        "--format=%H".to_owned(),
        head.to_owned(),
        "--".to_owned(),
        format!(".atelier/issues/{issue_id}.md"),
    ]
    .to_vec()
}

#[cfg(test)]
pub(crate) mod tests {
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
            assert!(EvidenceReadContext::new(repo.path())
                .evidence_is_fresh(issue, &evidence)
                .unwrap());
        }
        let index =
            RecordIndex::from_records(issues.to_vec(), vec![evidence.clone()], Vec::new()).unwrap();
        for (min_count, kind) in [(1, None), (2, None), (1, Some("other"))] {
            let shared = reads
                .issue_gate(&index, "atelier-a001", min_count, kind)
                .unwrap();
            let separate = EvidenceReadContext::new(repo.path())
                .issue_gate(&index, "atelier-a001", min_count, kind)
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
        assert!(!EvidenceReadContext::new(repo.path())
            .evidence_is_fresh(&issues[0], &evidence)
            .unwrap());
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
    fn code_clean_filter_follows_git_filemode_but_keeps_flagged_policy() {
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
        // core.filemode=false: Git ignores execute bits, and so do these paths.
        fs::set_permissions(&record, fs::Permissions::from_mode(0o755)).unwrap();
        let code = repo.path().join("code.rs");
        fs::set_permissions(&code, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(git::working_paths(repo.path()).unwrap().is_empty());
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

    /// A done record in the retired epic shape, and its `migrate epics` form.
    pub(crate) fn approved_epic() -> (String, String) {
        let mut record = issue_record("atelier-e001", "feature", "done");
        record.sections.close_reason = Some("Shipped.".into());
        let rendered = atelier_records::render_issue_record(&record).unwrap();
        let (front, _) = atelier_records::split_document(&rendered).unwrap();
        let epic = format!(
            "---\n{}\n---\n\n## Outcome\n\nShipped.\n\n## Proof\n\nChecked.\n\n## Close Reason\n\nShipped.\n",
            front.replace("issue_type: \"feature\"", "issue_type: \"epic\"")
        );
        let migrated = crate::epic_migration::migrate_epic_text(&epic, "feature", "backlog")
            .unwrap()
            .unwrap()
            .text;
        (epic, migrated)
    }

    #[test]
    fn epic_migration_is_not_a_new_completion_claim() {
        let repo = tempfile::tempdir().unwrap();
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.name", "Author"],
            vec!["config", "user.email", "author@example.test"],
        ] {
            git::git_output(repo.path(), &args).unwrap();
        }
        let (epic, migrated) = approved_epic();
        let path = repo.path().join(".atelier/issues/atelier-e001.md");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut commits = Vec::new();
        for text in [&epic, &migrated] {
            fs::write(&path, text).unwrap();
            git::git_output(repo.path(), &["add", "."]).unwrap();
            git::git_output(repo.path(), &["commit", "-qm", "record"]).unwrap();
            commits.push(git::head_commit(repo.path()).unwrap());
        }
        let mut read = EvidenceReadContext::new(repo.path());
        assert_eq!(
            read.claim_endpoint("atelier-e001", &commits[0]).unwrap(),
            commits[0],
            "the claim is the epic's done commit, not its migration"
        );
        // A real content change after the migration is a new claim.
        fs::write(
            &path,
            migrated.replace("Shipped.\n\n## Done when", "Changed.\n\n## Done when"),
        )
        .unwrap();
        git::git_output(repo.path(), &["commit", "-qam", "edit"]).unwrap();
        let edit = git::head_commit(repo.path()).unwrap();
        let mut read = EvidenceReadContext::new(repo.path());
        assert_eq!(
            read.claim_endpoint("atelier-e001", &commits[0]).unwrap(),
            edit
        );
    }
}
