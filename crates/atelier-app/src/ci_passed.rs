//! `ci.passed`, the merge check that reads CI through the policy's forge
//! (ADR 0031; docs/product/reviews-and-merge.md, CI on a forge).
//!
//! It passes when the branch changes no code, or when the trusted workflow
//! passed a `pull_request` run for a branch commit whose code matches the
//! head's. That run tested the head merged with the base as it was then, so
//! a base that moved since needs no new run, and the pass line says what the
//! run tested.
use anyhow::{Context, Result};
use std::path::Path;

use crate::forge::{CiRun, ForgeError, Github};
use crate::workflow_policy::WorkflowPolicy;
use crate::workflow_validation::MergeHeads;

/// Code is everything outside these record directories; the workflow policy
/// and config count as code.
pub const CODE: [&str; 5] = [
    ".",
    ":(exclude).atelier/issues",
    ":(exclude).atelier/reviews",
    ":(exclude).atelier/evidence",
    ":(exclude).atelier/runtime",
];

/// What CI says about the code that merges, for `merge --wait` to act on:
/// the refusal table's codes, and a pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CiState {
    Passed,
    /// `ci_no_run`: no open pull request into the base has the head.
    NoPullRequest,
    /// `ci_no_run`: a pull request has the head, but no run started for its
    /// code.
    NoRun,
    /// `ci_pending`: the newest run is queued or in progress.
    Pending,
    /// `ci_failed`: the newest run finished without passing.
    Failed,
    /// `forge_*`: `gh` could not answer.
    Unreachable,
}

/// The check's verdict; `next` is the whole Next step of a refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub state: CiState,
    pub reason: String,
    pub next: Option<String>,
}
impl Verdict {
    pub fn passed(&self) -> bool {
        self.state == CiState::Passed
    }
    fn pass(reason: String) -> Self {
        Self {
            state: CiState::Passed,
            reason,
            next: None,
        }
    }
    fn refuse(state: CiState, reason: String, next: String) -> Self {
        Self {
            state,
            reason,
            next: Some(next),
        }
    }
}

/// Judges `ci.passed` in `checkout`. A merge passes the heads it
/// integrates; elsewhere, such as `land` on a completed work branch, HEAD is
/// the branch head and the base is the policy's base branch. The heads are
/// never read from HEAD's parents: a work branch's own HEAD can be a merge
/// of the base. `issue_id` names the item a Next reruns `merge` for, empty
/// on the light path.
pub fn evaluate(
    checkout: &Path,
    policy: &WorkflowPolicy,
    issue_id: &str,
    merge: Option<MergeHeads<'_>>,
) -> Result<Verdict> {
    let forge = policy
        .forge
        .as_ref()
        .context("ci.passed needs a forge block in the base policy")?;
    let base = policy.branch_policy.base_branch.as_str();
    let git = |args: &[&str]| -> Result<String> {
        Ok(
            String::from_utf8(crate::git_freshness::git_output(checkout, args)?)?
                .trim()
                .to_owned(),
        )
    };
    let (base_head, head) = match merge {
        Some(heads) => (heads.base.to_owned(), heads.head.to_owned()),
        None => (
            git(&["rev-parse", &format!("refs/heads/{base}")])?,
            git(&["rev-parse", "HEAD"])?,
        ),
    };
    let fork = git(&["merge-base", &base_head, &head])?;
    let code_changed = |from: &str, to: &str| -> Result<bool> {
        let mut args = vec!["diff", "--name-only", from, to, "--"];
        args.extend(CODE);
        Ok(!git(&args)?.is_empty())
    };
    if !code_changed(&fork, &head)? {
        return Ok(Verdict::pass(
            "the branch changes only records, so it needs no CI run.".to_owned(),
        ));
    }
    let workflow = forge.ci_workflow.as_str();
    // A Next that reruns `merge` waits for CI. Code that must reach the
    // forge first pushes; a rerun that only waits keeps `--push` when this
    // run had it, since `merge --wait` alone publishes nothing.
    let merge_command = if issue_id.is_empty() {
        "atelier merge".to_owned()
    } else {
        format!("atelier merge {issue_id}")
    };
    let push_and_wait = format!("{merge_command} --push --wait");
    let wait_again = if merge.is_some_and(|heads| heads.push) {
        push_and_wait.clone()
    } else {
        format!("{merge_command} --wait")
    };
    let refused =
        |error: ForgeError| Verdict::refuse(CiState::Unreachable, error.reason(), error.next());
    let github = match Github::open(checkout, forge) {
        Ok(github) => github,
        Err(error) => return Ok(refused(error)),
    };
    // Walk back from the head while each commit's code is the head's.
    let mut seen = Vec::<CiRun>::new();
    for commit in git(&["rev-list", &format!("{fork}..{head}")])?.lines() {
        if code_changed(commit, &head)? {
            break;
        }
        let runs = match github.pull_request_runs(commit) {
            Ok(runs) => runs,
            Err(error) => return Ok(refused(error)),
        };
        if let Some(run) = runs.iter().find(|run| run.passed()) {
            let tested = format!(
                "{workflow} passed on {} for {}, which has the branch head's code",
                pull_request(run),
                short(commit)
            );
            return Ok(Verdict::pass(if code_changed(&fork, &base_head)? {
                format!(
                    "{tested}: {}. That run tested the head merged with {base} as it was when the run started, not the tree this merge makes. {base} has gained code since the branch left it, and CI on {base} after the push covers the combination.",
                    run.url
                )
            } else {
                format!(
                    "{tested}, and {base} has gained no code since the branch left it: {}",
                    run.url
                )
            }));
        }
        seen.extend(runs);
    }
    let Some(newest) = seen
        .iter()
        .max_by(|left, right| left.created_at.cmp(&right.created_at))
    else {
        // A head GitHub has never seen, because it was not pushed, fails
        // the lookup and is in no pull request.
        let has_pull_request = github.has_pull_request(&head, base).unwrap_or(false);
        return Ok(if has_pull_request {
            Verdict::refuse(
                CiState::NoRun,
                format!(
                    "no pull_request run of {workflow} exists for {}, the branch head, or an earlier branch commit with its code",
                    short(&head)
                ),
                push_and_wait,
            )
        } else {
            Verdict::refuse(
                CiState::NoPullRequest,
                format!(
                    "no open pull request into {base} has {}, the branch head, so CI has not run on it",
                    short(&head)
                ),
                push_and_wait,
            )
        });
    };
    let on = format!(
        "on {} for {}: {}",
        pull_request(newest),
        short(&newest.head_sha),
        newest.url
    );
    if newest.status != "completed" {
        return Ok(Verdict::refuse(
            CiState::Pending,
            format!("{workflow} is {} {on}", newest.status.replace('_', " ")),
            wait_again,
        ));
    }
    let conclusion = newest.conclusion.as_deref().unwrap_or("unknown");
    Ok(match conclusion {
        "cancelled" | "timed_out" => Verdict::refuse(
            CiState::Failed,
            format!(
                "{workflow} {} {on}",
                if conclusion == "cancelled" {
                    "was cancelled"
                } else {
                    "timed out"
                }
            ),
            format!("`gh run rerun {}`, then `{wait_again}`", newest.id),
        ),
        _ => Verdict::refuse(
            CiState::Failed,
            format!(
                "{workflow} {} {on}",
                if conclusion == "failure" {
                    "failed".to_owned()
                } else {
                    format!("finished {conclusion}")
                }
            ),
            format!(
                "`gh run view {} --log-failed`; fix it on the branch, commit, then `{push_and_wait}`",
                newest.id
            ),
        ),
    })
}

/// The note `merge` shows when the trusted workflow's newest run on the
/// base failed. It refuses nothing, since the merge may be the fix, and a
/// forge that cannot answer adds no note.
pub fn base_note(repo: &Path, policy: &WorkflowPolicy) -> Option<String> {
    let forge = policy.forge.as_ref()?;
    let base = policy.branch_policy.base_branch.as_str();
    let run = Github::open(repo, forge)
        .ok()?
        .newest_push_run(base)
        .ok()??;
    let failed = run.status == "completed"
        && matches!(
            run.conclusion.as_deref(),
            Some("failure" | "timed_out" | "startup_failure")
        );
    failed.then(|| {
        format!(
            "Note: {} failed on {base} at {}: {}. It refuses nothing.",
            forge.ci_workflow,
            short(&run.head_sha),
            run.url
        )
    })
}

fn pull_request(run: &CiRun) -> String {
    match run.pull_request {
        Some(number) => format!("pull request #{number}"),
        None => "a pull request".to_owned(),
    }
}

fn short(commit: &str) -> &str {
    commit.get(..8).unwrap_or(commit)
}
