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

/// Code is everything outside these record directories; the workflow policy
/// and config count as code.
const CODE: [&str; 5] = [
    ".",
    ":(exclude).atelier/issues",
    ":(exclude).atelier/reviews",
    ":(exclude).atelier/evidence",
    ":(exclude).atelier/runtime",
];

/// The check's verdict; `next` is the whole Next step of a refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub passed: bool,
    pub reason: String,
    pub next: Option<String>,
}
impl Verdict {
    fn pass(reason: String) -> Self {
        Self {
            passed: true,
            reason,
            next: None,
        }
    }
    fn refuse(reason: String, next: String) -> Self {
        Self {
            passed: false,
            reason,
            next: Some(next),
        }
    }
}

/// Judges `ci.passed` in `checkout`. In `merge`'s checkout of the merge
/// commit, the first parent is the base head and the second the branch head;
/// elsewhere HEAD is the branch head and the base is the policy's base
/// branch. `rerun` is the command that merges again.
pub fn evaluate(checkout: &Path, policy: &WorkflowPolicy, rerun: &str) -> Result<Verdict> {
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
    let (base_head, head) = match git(&["rev-parse", "-q", "--verify", "HEAD^2"]) {
        Ok(head) => (git(&["rev-parse", "HEAD^1"])?, head),
        Err(_) => (
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
    let refused = |error: ForgeError| Verdict::refuse(error.reason(), error.next());
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
                format!(
                    "no pull_request run of {workflow} exists for {}, the branch head, or an earlier branch commit with its code",
                    short(&head)
                ),
                format!("push the branch so CI runs on its pull request, then `{rerun}` once CI passes"),
            )
        } else {
            Verdict::refuse(
                format!(
                    "no open pull request into {base} has {}, the branch head, so CI has not run on it",
                    short(&head)
                ),
                format!("push the branch and open a pull request into {base} (`gh pr create --base {base}`), then `{rerun}` once CI passes"),
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
            format!("{workflow} is {} {on}", newest.status.replace('_', " ")),
            format!("`gh run watch {}`, then `{rerun}`", newest.id),
        ));
    }
    let conclusion = newest.conclusion.as_deref().unwrap_or("unknown");
    Ok(match conclusion {
        "cancelled" | "timed_out" => Verdict::refuse(
            format!(
                "{workflow} {} {on}",
                if conclusion == "cancelled" {
                    "was cancelled"
                } else {
                    "timed out"
                }
            ),
            format!("`gh run rerun {}`, then `{rerun}` once CI passes", newest.id),
        ),
        _ => Verdict::refuse(
            format!(
                "{workflow} {} {on}",
                if conclusion == "failure" {
                    "failed".to_owned()
                } else {
                    format!("finished {conclusion}")
                }
            ),
            format!(
                "`gh run view {} --log-failed`; fix it on the branch, commit and push, then `{rerun}` once CI passes",
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
