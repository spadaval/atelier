//! The light path (ADR 0028 decision 6, atelier-4nsl): a branch no issue
//! owns, such as a release, a docs edit or a mockup, is reviewed and merged
//! on an independent approval of its head, under the same merge checks, with
//! no issue, start or Report around it.
//!
//! A review record about such a branch names the branch as its target and is
//! kept apart from the records about issues (`RecordIndex::branch_reviews`),
//! so nothing that judges an issue reads it. It counts only for the branch's
//! current run: pinned to a commit the branch holds and the point where it
//! left the base does not, so a later branch with a reused name (`release`)
//! inherits none of an earlier one's findings or approvals. An approval of a
//! branch covers its code exactly as an approval of an issue covers the
//! issue's, and never a done claim.
use crate::{
    git_freshness as git,
    record_index::RecordIndex,
    review_gate::{self, ancestor, gate, Gap, GateNode, Settlement, Standing},
    workflow_policy::WorkflowPolicy,
};
use anyhow::{Context, Result};
use atelier_core::{ReviewEvent, ReviewRecord};
use std::{
    collections::{BTreeSet, HashMap},
    path::Path,
};

/// The issues that own `branch` in `index`, this checkout's records: those
/// that record it as their work branch and are not done on the base. An
/// owner already done on the base no longer owns the name, so a branch that
/// reuses it is on the light path. An issue the base does not hold yet is
/// not done there.
pub fn owners(
    repo: &Path,
    index: &RecordIndex,
    policy: &WorkflowPolicy,
    branch: &str,
) -> Vec<String> {
    let base = format!("refs/heads/{}", policy.branch_policy.base_branch);
    index
        .issues()
        .filter(|issue| {
            issue
                .workflow_branch
                .as_ref()
                .is_some_and(|owner| owner.work_branch == branch)
        })
        .filter(|issue| !done_at(repo, policy, &base, &issue.id))
        .map(|issue| issue.id.clone())
        .collect()
}

/// Whether `revision` holds `id` in a done status.
pub fn done_at(repo: &Path, policy: &WorkflowPolicy, revision: &str, id: &str) -> bool {
    let path = format!(".atelier/issues/{id}.md");
    let Some(commit) = git::git_output(
        repo,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{revision}^{{commit}}"),
        ],
    )
    .ok()
    .map(|out| String::from_utf8_lossy(&out).trim().to_owned()) else {
        return false;
    };
    git::blob(repo, &commit, Path::new(&path))
        .ok()
        .and_then(|text| {
            atelier_records::parse_issue_record(
                &text,
                Path::new(&path).strip_prefix(".atelier").ok()?,
            )
            .ok()
        })
        .is_some_and(|record| policy.status_category(&record.issue.status) == Some("done"))
}

/// The records of `branch`'s current run among `records`: those about the
/// branch pinned to a commit `read`'s HEAD holds and `fork_point`, where the
/// branch left the base, does not. Records of an earlier branch with the same
/// name are on the base, so behind the fork point, and records pinned to
/// commits a rebase left behind are not on the branch; neither counts.
pub fn run<'r>(
    records: impl IntoIterator<Item = &'r ReviewRecord>,
    read: &mut git::ReadContext<'_>,
    branch: &str,
    fork_point: &str,
) -> Vec<&'r ReviewRecord> {
    let mut run = records
        .into_iter()
        .filter(|record| {
            record.target == branch
                && ancestor(read, &record.commit, "HEAD")
                && !ancestor(read, &record.commit, fork_point)
        })
        .collect::<Vec<_>>();
    run.sort_by_key(|record| (record.header.created_at, record.header.id.clone()));
    run
}

/// Where the branch's review stands, judged as `review.current` judges an
/// issue's: its run's current approval, else the newest stale one.
pub fn standing<'r>(
    run: &[&'r ReviewRecord],
    read: &mut git::ReadContext<'_>,
    fork_point: &str,
) -> Result<Standing<'r>> {
    review_gate::standing_among(
        run.iter()
            .copied()
            .filter(|record| record.event == ReviewEvent::Approval),
        read,
        fork_point,
    )
}

/// The approval in `run` that resolves the fix_now `finding`, by the rule
/// `findings.disposed` applies to an issue's: of a later commit holding the
/// finding's record, and recorded after it. `None` for a finding that is not
/// fix_now, or that no approval since resolves; whether that approval is
/// still current is `review.current`'s question.
pub fn fix_approval<'r>(
    run: &[&'r ReviewRecord],
    read: &mut git::ReadContext<'_>,
    finding: &ReviewRecord,
) -> Option<&'r ReviewRecord> {
    if review_gate::settlement(finding) != Settlement::Fix {
        return None;
    }
    run.iter()
        .copied()
        .filter(|record| record.event == ReviewEvent::Approval)
        .find(|approval| review_gate::fix_resolved(read, finding, approval))
}

/// The gates for a branch on the light path, at `read`'s HEAD, which left
/// the base at `fork_point`: `review.current` (a current independent
/// approval of its run), `findings.disposed` (its run's findings, judged as
/// for an issue) and, with `checks`, the policy's `merge_checks` run
/// there. A check that reads an issue passes, since there is none. Each node
/// is about `branch`.
pub fn evaluate(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    branch: &str,
    fork_point: &str,
    checks: Option<(&Path, crate::workflow_validation::MergeHeads<'_>)>,
) -> Result<Vec<GateNode>> {
    let run = run(index.branch_reviews(), read, branch, fork_point);
    let current = run
        .iter()
        .copied()
        .filter(|record| record.event == ReviewEvent::Approval)
        .filter(|record| review_gate::current_on_branch(read, record, fork_point))
        .collect::<Vec<_>>();
    let mut nodes = vec![gate(
        branch,
        "review.current",
        !current.is_empty(),
        match current.first() {
            Some(record) => format!("current independent approval {}", record.header.id),
            None => match standing(&run, read, fork_point)? {
                standing if standing.record.is_some() => review_gate::not_current_reason(&standing),
                _ => "no independent approval of this branch".to_owned(),
            },
        },
        current
            .iter()
            .map(|record| record.header.id.clone())
            .collect(),
    )];
    let mut unresolved = Vec::new();
    let mut gaps = Vec::new();
    let mut disposed = Vec::new();
    for finding in run
        .iter()
        .filter(|record| matches!(record.event, ReviewEvent::Finding { .. }))
    {
        let id = finding.header.id.clone();
        match review_gate::settlement(finding) {
            Settlement::Decided => disposed.push(id),
            Settlement::Undecided => {
                unresolved.push(format!(
                    "{id} needs a decision: a finding recommended defer, no_action or wont_fix records one"
                ));
                gaps.push(Gap::Undisposed(id));
            }
            Settlement::Fix => {
                if current
                    .iter()
                    .any(|approval| review_gate::fix_resolved(read, finding, approval))
                {
                    disposed.push(id);
                } else {
                    unresolved.push(format!(
                        "{id}: fix_now needs a current approval after its fix"
                    ));
                    gaps.push(Gap::AwaitingApproval(id));
                }
            }
        }
    }
    nodes.push(
        gate(
            branch,
            "findings.disposed",
            unresolved.is_empty(),
            if unresolved.is_empty() {
                "every finding is decided, or fixed and approved since".into()
            } else {
                unresolved.join("; ")
            },
            disposed,
        )
        .with_gaps(gaps),
    );
    if let Some((root, heads)) = checks {
        let checks = crate::workflow_validation::evaluate_without_issue(
            index,
            root,
            policy,
            &policy.merge_checks,
            Some(heads),
        )?;
        for (check, definition) in checks.into_iter().zip(&policy.merge_checks) {
            nodes.push(review_gate::validator_gate(
                index,
                policy,
                branch,
                check,
                Some(definition),
            )?);
        }
    }
    Ok(nodes)
}

/// The commits of `tip`, the base, not on `trusted` that an approval of a
/// light-path branch covers, as `land` on the base counts them reviewed.
/// `read` is pinned at `tip`. An approval covers its pin's history as an
/// issue's does, once it is one `merge` would have accepted:
///
/// - It reached the base through a merge, and its branch left the base at
///   that merge's first parent's merge base with it; a pin on the base's own
///   first-parent line was never on a branch.
/// - Its branch was not owned when it merged: no issue the merged branch
///   records as its work branch was open on the base before the merge.
/// - It is as first committed, and every finding of its run pinned at or
///   before it is settled as `findings.disposed` settles one.
///
/// Its commits are still checked for done claims, which no approval of a
/// branch covers.
pub fn covered_on_base(
    repo: &Path,
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    tip: &str,
    trusted: &str,
) -> Result<BTreeSet<String>> {
    let mut covered = BTreeSet::new();
    let mut line: Option<HashMap<String, usize>> = None;
    for approval in index
        .branch_reviews()
        .filter(|record| record.event == ReviewEvent::Approval)
    {
        let Ok(pin) = read.commit_id(&approval.commit) else {
            continue;
        };
        if !ancestor(read, &pin, tip) || ancestor(read, &pin, trusted) {
            continue;
        }
        let line = match &mut line {
            Some(line) => line,
            None => line.insert(first_parent_line(repo, tip)?),
        };
        let Some(fork) = joined_fork(repo, read, policy, &approval.target, &pin, tip, line)? else {
            continue;
        };
        if !accepted(index, read, approval, &pin, &fork)? {
            continue;
        }
        let listed = git::git_output(repo, &["rev-list", &pin, "--not", trusted])?;
        covered.extend(String::from_utf8(listed)?.lines().map(str::to_owned));
    }
    Ok(covered)
}

/// The commits on `tip`'s first-parent line, each with its distance from
/// `tip`.
fn first_parent_line(repo: &Path, tip: &str) -> Result<HashMap<String, usize>> {
    let listed = String::from_utf8(git::git_output(repo, &["rev-list", "--first-parent", tip])?)?;
    Ok(listed
        .lines()
        .enumerate()
        .map(|(distance, commit)| (commit.to_owned(), distance))
        .collect())
}

/// Where `branch`, whose approval is pinned at `pin`, left the base, read
/// from the merge on `tip`'s first-parent line that brought `pin` in; `None`
/// when `pin` is on that line itself, or the branch was owned when it merged.
#[allow(clippy::too_many_arguments)]
fn joined_fork(
    repo: &Path,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    branch: &str,
    pin: &str,
    tip: &str,
    line: &HashMap<String, usize>,
) -> Result<Option<String>> {
    if line.contains_key(pin) {
        return Ok(None);
    }
    let descendants = String::from_utf8(git::git_output(
        repo,
        &["rev-list", "--ancestry-path", &format!("{pin}..{tip}")],
    )?)?;
    let Some(merge) = descendants
        .lines()
        .filter_map(|commit| line.get(commit).map(|distance| (*distance, commit)))
        .max_by_key(|(distance, _)| *distance)
        .map(|(_, commit)| commit.to_owned())
    else {
        return Ok(None);
    };
    let parents = String::from_utf8(git::git_output(
        repo,
        &["rev-list", "--parents", "-n", "1", &merge],
    )?)?;
    let parents = parents.split_whitespace().skip(1).collect::<Vec<_>>();
    let Some((first, sides)) = parents.split_first() else {
        return Ok(None);
    };
    let Some(side) = sides
        .iter()
        .find(|side| ancestor(read, pin, side))
        .map(|side| side.to_string())
    else {
        return Ok(None);
    };
    if owned_when_merged(repo, policy, branch, &side, first)? {
        return Ok(None);
    }
    let fork = String::from_utf8(git::git_output(repo, &["merge-base", first, pin])?)?;
    Ok(Some(fork.trim().to_owned()).filter(|fork| !fork.is_empty()))
}

/// Whether an issue the branch's merged head `side` records as owning
/// `branch` was not done on the base at `before`, the merge's first parent:
/// the branch then followed the issue path, where its own records count for
/// nothing. Every record that names a work branch is parsed and its branch
/// compared, since a name YAML would read as another type, such as `1.0`, is
/// written quoted (finding atelier-lezj). A record that does not parse but
/// mentions the branch counts as an owner, so the approval covers nothing
/// rather than too much.
fn owned_when_merged(
    repo: &Path,
    policy: &WorkflowPolicy,
    branch: &str,
    side: &str,
    before: &str,
) -> Result<bool> {
    let listed = git::command(repo)
        .args([
            "grep",
            "-l",
            "-F",
            "-e",
            "work_branch:",
            side,
            "--",
            ".atelier/issues/",
        ])
        .output()
        .context("git grep could not read the merged branch's issues")?;
    // `git grep` exits 1 when nothing matches.
    if !listed.status.success() && listed.status.code() != Some(1) {
        anyhow::bail!(
            "git grep could not read the issues of {side}: {}",
            String::from_utf8_lossy(&listed.stderr).trim()
        );
    }
    for found in String::from_utf8_lossy(&listed.stdout).lines() {
        let Some((_, path)) = found.split_once(':') else {
            continue;
        };
        let path = Path::new(path);
        let text = git::blob(repo, side, path)?;
        let Some(record) = path
            .strip_prefix(".atelier")
            .ok()
            .and_then(|relative| atelier_records::parse_issue_record(&text, relative).ok())
        else {
            if text.contains(branch) {
                return Ok(true);
            }
            continue;
        };
        let names_branch = record
            .issue
            .workflow_branch
            .as_ref()
            .is_some_and(|owner| owner.work_branch == branch);
        if names_branch && !done_at(repo, policy, before, &record.issue.id) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Whether `approval`, pinned at `pin` on a branch that left the base at
/// `fork`, is one `merge` would have accepted: as first committed, with
/// every finding of its run pinned at or before it settled.
fn accepted(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    approval: &ReviewRecord,
    pin: &str,
    fork: &str,
) -> Result<bool> {
    if !review_gate::original_intact(read, approval).unwrap_or(false) {
        return Ok(false);
    }
    for finding in index.branch_reviews().filter(|record| {
        record.target == approval.target && matches!(record.event, ReviewEvent::Finding { .. })
    }) {
        if !ancestor(read, &finding.commit, pin) || ancestor(read, &finding.commit, fork) {
            continue;
        }
        let settled = match review_gate::settlement(finding) {
            Settlement::Decided => true,
            Settlement::Fix => review_gate::fix_resolved(read, finding, approval),
            Settlement::Undecided => false,
        };
        if !settled {
            return Ok(false);
        }
    }
    Ok(true)
}
