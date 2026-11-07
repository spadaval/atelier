//! Refuses a write of records that `atelier check` would refuse, before
//! anything is written (atelier-ilba).
//!
//! A record that broke a graph rule used to be written without complaint,
//! after which every command that loads the records refused it, and the
//! append-only rules kept its author from fixing it. Now every write passes
//! the rules `atelier check` applies first: the records the write would
//! leave are checked, and a finding they have that the records before it do
//! not have is refused, with where a missing record is instead. Comparing
//! with the records before means a write is never refused for a problem it
//! did not make, and a write that fixes one passes.
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use atelier_records::guard::Change;

use crate::lint::{lint_files, LintFinding};
use crate::record_index::{missing_reference_note, read_changed_record_files, read_record_files};

/// Installs [`check_changes`] as the guard every record write in this
/// process passes.
pub fn install() {
    atelier_records::guard::install(check_changes);
}

/// Refuses `changes` to the records under `state_dir` when the records
/// they would leave have a finding `atelier check` reports that the records
/// before them do not.
pub fn check_changes(state_dir: &Path, changes: &[Change]) -> Result<()> {
    let after = lint_files(
        state_dir,
        read_changed_record_files(state_dir, changes)?,
        None,
        false,
    )?;
    if after.is_empty() {
        return Ok(());
    }
    let before = lint_files(state_dir, read_record_files(state_dir)?, None, false)?;
    refuse_new(
        state_dir.parent(),
        &before,
        after,
        "writing these records",
        "nothing was written",
    )
}

/// Refuses merging `tip` into the base at commit `base` when the records
/// `tip` holds have a finding `atelier check` reports that `base`'s do
/// not: the records a merge puts on the base pass the check every other
/// write passes.
pub fn check_merge(repo: &Path, base: &str, tip: &str) -> Result<()> {
    let after = commit_findings(repo, tip)?;
    if after.is_empty() {
        return Ok(());
    }
    let before = commit_findings(repo, base)?;
    refuse_new(
        Some(repo),
        &before,
        after,
        "merging puts records on the base that",
        "nothing was merged",
    )
}

/// The one rule `atelier check` applies that a write does not: atelier-md0v
/// removes the deferral source rule, so it gets no write-time version, and
/// this exception goes with it.
const NOT_CHECKED_AT_WRITE: &str = "review_follow_up_source_mismatch";

/// Fails naming each finding in `after` that `before` lacks, with where a
/// record it names as missing is instead.
fn refuse_new(
    repo_root: Option<&Path>,
    before: &[LintFinding],
    after: Vec<LintFinding>,
    what: &str,
    outcome: &str,
) -> Result<()> {
    let new = after
        .into_iter()
        .filter(|finding| !before.contains(finding))
        .filter(|finding| !finding.message.contains(NOT_CHECKED_AT_WRITE))
        .collect::<Vec<_>>();
    if new.is_empty() {
        return Ok(());
    }
    let lines = new
        .iter()
        .map(|finding| {
            let note = repo_root
                .and_then(|root| missing_reference_note(root, &finding.message))
                .map(|note| format!("; {note}"))
                .unwrap_or_default();
            format!("\n  {}: {}{note}", finding.id, finding.message)
        })
        .collect::<String>();
    bail!(
        "record_invalid: {what} would fail `atelier check` with {}, so {outcome}:{lines}\nNext: fix what each line names, then rerun",
        atelier_core::count_noun(new.len(), "new finding")
    )
}

/// The findings `atelier check` reports for the records at `commit`, read
/// with the documents they link (decision ADRs and mission strategies) from
/// that commit alone. A commit without records has none. A workflow policy
/// that does not parse is left to merge's own check of it, which names its
/// fix.
fn commit_findings(repo: &Path, commit: &str) -> Result<Vec<LintFinding>> {
    let dir = tempfile::tempdir().context("cannot create a directory to check records in")?;
    extract(repo, commit, &[".atelier".to_owned()], dir.path())?;
    let state_dir = dir.path().join(".atelier");
    if !state_dir.is_dir() {
        return Ok(Vec::new());
    }
    let files = read_record_files(&state_dir)?;
    let documents = files
        .issues
        .iter()
        .flat_map(|record| {
            [
                record.issue.adr.as_ref().map(|adr| adr.path.clone()),
                record
                    .issue
                    .strategy
                    .as_ref()
                    .map(|strategy| strategy.path.clone()),
            ]
        })
        .flatten()
        .collect::<Vec<_>>();
    extract(repo, commit, &documents, dir.path())?;
    let mut findings = lint_files(&state_dir, files, None, false)?;
    findings.retain(|finding| finding.id != "policy");
    Ok(findings)
}

/// Writes the files under `paths` at `commit` into `dir`, skipping paths
/// the commit does not hold.
fn extract(repo: &Path, commit: &str, paths: &[String], dir: &Path) -> Result<()> {
    if paths.is_empty() {
        return Ok(());
    }
    let mut args = vec!["ls-tree", "-z", "--name-only", commit, "--"];
    args.extend(paths.iter().map(String::as_str));
    let held = crate::git_freshness::git_output(repo, &args)?;
    let held = held
        .split(|byte| *byte == 0)
        .filter(|name| !name.is_empty())
        .map(|name| String::from_utf8_lossy(name).into_owned())
        .collect::<Vec<_>>();
    if held.is_empty() {
        return Ok(());
    }
    let mut archive = crate::git_freshness::command(repo)
        .args(["archive", "--format=tar", commit, "--"])
        .args(&held)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let unpack = Command::new("tar")
        .current_dir(dir)
        .args(["-xf", "-"])
        .stdin(archive.stdout.take().context("git archive has no output")?)
        .output()?;
    let archive = archive.wait_with_output()?;
    if !archive.status.success() || !unpack.status.success() {
        bail!(
            "cannot read the records at {commit}: {}{}",
            String::from_utf8_lossy(&archive.stderr),
            String::from_utf8_lossy(&unpack.stderr)
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_records::issue_record;
    use atelier_records::{issue_relates_relationship, render_issue_record, CanonicalIssueRecord};
    use std::fs;
    use std::path::PathBuf;

    /// A version 1 review event about `atelier-real`, as an older Atelier
    /// wrote it, with the separate disposition events ADR 0025 retired.
    fn v1_review(id: &str, event: &str) -> String {
        format!(
            "---\nschema: atelier.review\nschema_version: 1\nid: {id}\ntitle: t\ntarget: atelier-real\nidentity: reviewer\ncommit: {}\ncreated_at: 2026-10-17T00:00:00Z\nevent:\n{event}---\n\n{id} text\n",
            "a".repeat(40)
        )
    }

    fn disposition(supersedes: Option<&str>) -> String {
        format!(
            "  event: disposition\n  finding: atelier-fnd1\n  decision: no_action\n  supersedes: {}\n  resolution_commit: null\n  follow_up: null\n",
            supersedes.unwrap_or("null")
        )
    }

    /// Records under the starter policy: an issue, and a finding on it that
    /// one separate disposition event settles.
    fn records() -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let state = dir.path().join(".atelier");
        fs::create_dir_all(state.join("reviews")).unwrap();
        fs::write(
            state.join("workflow.yaml"),
            atelier_workflow::STARTER_POLICY_YAML,
        )
        .unwrap();
        atelier_records::RecordStore::new(&state)
            .write_issue_atomic(&issue_record("atelier-real", "task", "todo"))
            .unwrap();
        for (id, event) in [
            (
                "atelier-fnd1",
                "  event: finding\n  recommendation: no_action\n".to_owned(),
            ),
            ("atelier-dsp1", disposition(None)),
        ] {
            fs::write(
                state.join(format!("reviews/{id}.md")),
                v1_review(id, &event),
            )
            .unwrap();
        }
        (dir, state)
    }

    fn write(record: &CanonicalIssueRecord) -> Change {
        Change {
            path: atelier_records::issue_record_path(&record.issue.id),
            text: Some(render_issue_record(record).unwrap()),
        }
    }

    fn linking(id: &str, missing: &str) -> CanonicalIssueRecord {
        let mut record = issue_record(id, "task", "todo");
        record
            .relationships
            .relates
            .push(issue_relates_relationship(missing, "related"));
        record
    }

    #[test]
    fn a_write_that_gives_a_finding_competing_disposition_events_is_refused() {
        let (_dir, state) = records();
        let competing = Change {
            path: PathBuf::from("reviews/atelier-dsp2.md"),
            text: Some(v1_review("atelier-dsp2", &disposition(None))),
        };
        let error = format!("{:#}", check_changes(&state, &[competing]).unwrap_err());
        assert!(
            error.starts_with("record_invalid: writing these records would fail `atelier check` with 2 new findings, so nothing was written:")
                && error.contains("atelier-dsp1: Tracker record files are invalid: Invalid record file .atelier/reviews/atelier-dsp1.md: finding atelier-fnd1 has competing")
                && error.contains("atelier-dsp2: Tracker record files are invalid: Invalid record file .atelier/reviews/atelier-dsp2.md: finding atelier-fnd1 has competing"),
            "{error}"
        );
        // A successor that continues the event it supersedes passes.
        let successor = Change {
            path: PathBuf::from("reviews/atelier-dsp2.md"),
            text: Some(v1_review(
                "atelier-dsp2",
                &disposition(Some("atelier-dsp1")),
            )),
        };
        check_changes(&state, &[successor]).unwrap();
    }

    #[test]
    fn a_write_is_refused_only_for_the_findings_it_adds() {
        let (_dir, state) = records();
        // A link to a missing issue, written past the guard.
        atelier_records::RecordStore::new(&state)
            .write_issue_atomic(&linking("atelier-brkn", "atelier-gone"))
            .unwrap();
        // An unrelated write is not refused for it, nor is the write that
        // fixes it, nor removing the record that holds it.
        check_changes(
            &state,
            &[write(&issue_record("atelier-new1", "task", "todo"))],
        )
        .unwrap();
        check_changes(
            &state,
            &[write(&issue_record("atelier-brkn", "task", "todo"))],
        )
        .unwrap();
        check_changes(
            &state,
            &[Change {
                path: PathBuf::from("issues/atelier-brkn.md"),
                text: None,
            }],
        )
        .unwrap();
        // A second link to a missing issue is refused.
        let error = check_changes(&state, &[write(&linking("atelier-new2", "atelier-lost"))])
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("reference to missing issue atelier-lost")
                && !error.contains("atelier-gone"),
            "{error}"
        );
        // Removing the issue a review event is about is refused too.
        let error = check_changes(
            &state,
            &[Change {
                path: PathBuf::from("issues/atelier-real.md"),
                text: None,
            }],
        )
        .unwrap_err()
        .to_string();
        assert!(
            error.contains("review target reference to missing issue atelier-real"),
            "{error}"
        );
    }
}
