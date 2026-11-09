//! `merge --push`: merge, then publish the base through the policy's forge
//! (ADR 0031; docs/product/reviews-and-merge.md, "`merge --push`, step by
//! step"). It checks `gh`, fetches the base from `forge.remote`, merges as
//! `merge` does, judges the unpushed base commits as `land` does, pushes the
//! base and confirms the pull request merged, so one exit status covers
//! integrating and publishing.
//!
//! Pushing the branch and opening its pull request (steps 2 and 3) are not
//! built yet (atelier-vqoy): the branch must already be pushed with an open
//! pull request.
use super::git::{resolve, text};
use super::merge::{attempts, base_checkout, rerun};
use anyhow::{anyhow, bail, Context, Result};
use atelier_app::{
    base_policy, committed_snapshot,
    forge::{ForgeError, Github},
    git_freshness as git,
    storage_layout::StorageLayout,
};
use std::path::Path;
use std::time::{Duration, Instant};

/// How long `merge --push` waits for GitHub to mark the pull request merged
/// after the base is pushed, and how often it asks.
const PULL_REQUEST_WAIT: Duration = Duration::from_secs(60);
const PULL_REQUEST_POLL: Duration = Duration::from_secs(5);

/// Runs `merge --push` and returns what it published, for the verdict line.
pub(super) fn run(issue_ref: Option<&str>, quiet: bool) -> Result<String> {
    let say = |line: &str| {
        if quiet {
            eprintln!("{line}");
        } else {
            println!("{line}");
        }
    };
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    let base = base_policy::from_ref(repo, "HEAD")?
        .policy
        .branch_policy
        .base_branch;
    let Some(branch) = super::git::checkout_branch(repo)?.filter(|branch| *branch != base) else {
        // On the base or a detached HEAD, merge refuses before any gate, or
        // finds the work already merged; nothing is fetched or pushed.
        attempts(issue_ref, false, false, quiet, true)?;
        bail!("merge_push_base_branch: merge --push runs from the work branch. Next: check out the work branch, then `{}`", rerun(issue_ref, true));
    };
    let rerun = rerun(issue_ref, true);
    let no_forge = || {
        anyhow!("merge_push_no_forge: the {base} policy names no forge, so merge --push has nowhere to publish; nothing was pushed. Next: `{}`, then `atelier land --push` from the {base} checkout", super::merge::rerun(issue_ref, false))
    };
    let Some(forge) = base_forge(repo, &format!("refs/heads/{base}"))? else {
        return Err(no_forge());
    };
    let remote = forge.remote.as_str();

    // 1. gh, before anything is pushed.
    let forge_refusal = |error: ForgeError| {
        anyhow!(
            "{}: {}; nothing was pushed. Next: {}",
            error.code(),
            error.reason(),
            error.next()
        )
    };
    let github = Github::open(repo, &forge).map_err(forge_refusal)?;
    github.signed_in().map_err(forge_refusal)?;

    // 4. The base as the forge has it.
    let remote_base = fetch_base(repo, remote, &base)?;
    let local_base = resolve(repo, &format!("refs/heads/{base}"))?;
    if local_base != remote_base {
        if is_ancestor(repo, &local_base, &remote_base) {
            fast_forward(repo, &base, &local_base, &remote_base)?;
            say(&format!(
                "Fast-forwarded {base} to {} from {remote}.",
                short(&remote_base)
            ));
        } else if !is_ancestor(repo, &remote_base, &local_base) {
            bail!(
                "merge_push_base_diverged: local {base} at {} and {base} on {remote} at {} have diverged, so a merge onto local {base} could not be pushed; nothing was merged or pushed. Next: in the {base} checkout, `git fetch {remote} {base} && git merge --no-edit FETCH_HEAD` and `atelier land`, then `git push {remote} {base}`, then back on {branch}, `{rerun}`",
                short(&local_base),
                short(&remote_base)
            );
        }
    }
    // The base's committed policy names the forge, so the base just fetched
    // decides where it is published (ADR 0031, decision 3).
    let changed = |now: Option<&atelier_app::workflow_policy::ForgeConfig>,
                   at: &str,
                   after: &str| {
        match now {
            None => no_forge(),
            Some(now) => anyhow!(
                "merge_push_forge_changed: {base} at {} names forge remote {} with workflow {}, not remote {remote} with workflow {} as {base} did before {after}; nothing was pushed. Next: rerun `{rerun}`, which publishes through the forge {base} names now",
                short(at),
                now.remote,
                now.ci_workflow,
                forge.ci_workflow
            ),
        }
    };
    let fetched = resolve(repo, &format!("refs/heads/{base}"))?;
    let now = base_forge(repo, &fetched)?;
    if now.as_ref() != Some(&forge) {
        return Err(changed(now.as_ref(), &fetched, "the fetch"));
    }

    // 5 and 6. The gates and the merge, as `merge` runs them.
    let integrated = attempts(issue_ref, false, false, quiet, true)?
        .context("merge integrated nothing to push")?;
    let target = integrated.base_checkout.as_path();
    let merged = short(&integrated.commit);
    // The merge may have run on a base that moved again, by records only,
    // while its gates ran; the base it merged onto still names this forge.
    let onto = text(target, &["rev-parse", &format!("{}^1", integrated.commit)])?;
    let now = base_forge(target, &onto)?;
    if now.as_ref() != Some(&forge) {
        return Err(anyhow!(
            "{} Merge commit {merged} stays on local {base}.",
            changed(now.as_ref(), &onto, "the merge")
        ));
    }
    // The pull request is read while it is still open, by its branch: the
    // branch head can be a record-only commit GitHub has never seen.
    let pull = github.open_pull_request(&branch, &base);

    // 7. Judge the unpushed base commits as `land` does on the base, from
    // the base the forge has, then push.
    let (_records, index) =
        super::git::Snapshot::at(repo, &layout.canonical_dir(), &integrated.commit)?.into_parts();
    let merged_policy = base_policy::from_ref(target, &integrated.commit)?.policy;
    let judged = super::git::check_base_commits_since(
        target,
        &index,
        &merged_policy,
        &integrated.commit,
        &remote_base,
    )
    .map_err(|error| {
        anyhow!("{error:#} Nothing was pushed; merge commit {merged} stays on {base}.")
    })?;
    say(&judged);
    let pushed = git::command(target)
        .args([
            "push",
            "--quiet",
            remote,
            &format!("{}:refs/heads/{base}", integrated.commit),
        ])
        .output()
        .context("could not run git push")?;
    if !pushed.status.success() {
        bail!(
            "merge_push_base_rejected: git push to {remote} refused {base}: {}. Merge commit {merged} stays on local {base}, and nothing was undone. Next: in the {base} checkout, `git fetch {remote} {base} && git merge --no-edit FETCH_HEAD`, then `atelier land`, then `git push {remote} {base}`",
            one_line(&pushed.stderr)
        );
    }
    let published = format!("pushed {base} to {remote}");

    // 8. GitHub marks the pull request merged once its head is on the base.
    let number = match pull {
        Ok(Some(number)) => number,
        Ok(None) => {
            say(&format!(
                "Note: {published} at {merged}; GitHub has no open pull request from {branch} into {base}, so none was confirmed merged."
            ));
            return Ok(published);
        }
        Err(error) => {
            say(&format!(
                "Note: {published} at {merged}; Atelier could not read the pull request from {branch} into {base}, so it did not confirm one merged: {}. Next: gh pr list --head {branch} --state all",
                error.reason()
            ));
            return Ok(published);
        }
    };
    if merged_within(&github, number, pull_request_wait()) {
        return Ok(format!("{published}, pull request #{number} merged"));
    }
    say(&format!(
        "Note: {published} at {merged}; pull request #{number} is still open because GitHub has not marked it merged. Atelier does not close it, since closed is not merged. Next: gh pr view {number}"
    ));
    Ok(published)
}

/// Fetches the base from `remote` into `FETCH_HEAD`, which needs no
/// remote-tracking branch, so a URL remote works too.
fn fetch_base(repo: &Path, remote: &str, base: &str) -> Result<String> {
    let fetched = git::command(repo)
        .args([
            "fetch",
            "--quiet",
            "--no-tags",
            remote,
            &format!("refs/heads/{base}"),
        ])
        .output()
        .context("could not run git fetch")?;
    if !fetched.status.success() {
        bail!(
            "merge_push_fetch_failed: git fetch from {remote} could not read {base}: {}; nothing was pushed. Next: check that `git fetch {remote} {base}` works, then retry",
            one_line(&fetched.stderr)
        );
    }
    text(repo, &["rev-parse", "--verify", "FETCH_HEAD^{commit}"])
}

/// Moves a local base that is behind the forge's up to it: its checkout
/// with a fast-forward, or the branch alone when none has it checked out.
fn fast_forward(repo: &Path, base: &str, from: &str, to: &str) -> Result<()> {
    match base_checkout(repo, base)? {
        Some(path) => {
            // As `merge` guards a checked-out base: its records and code
            // match its commit, and no untracked path is in the way.
            committed_snapshot::CommittedSnapshot::new(from)?
                .require_clean(&path)
                .map_err(|error| {
                    anyhow!("{error:#} The {base} checkout at {} must be clean before merge --push fast-forwards it; nothing was merged or pushed.", path.display())
                })?;
            let overwritten = super::git::untracked_overwritten(&path, &[to])?;
            if !overwritten.is_empty() {
                bail!(
                    "merge_untracked_overwritten: {} has {} where {base} on the forge has files, so fast-forwarding would overwrite them: {}; nothing was merged or pushed. Next: move them out of the way, or commit them, then retry",
                    path.display(),
                    crate::human_output::count_noun(overwritten.len(), "untracked path"),
                    crate::human_output::first_five(
                        overwritten.iter().map(|path| path.display().to_string()),
                        ", "
                    )
                );
            }
            git::git_output(
                &path,
                &[
                    "-c",
                    "core.hooksPath=/dev/null",
                    "merge",
                    "--ff-only",
                    "--quiet",
                    to,
                ],
            )
            .with_context(|| format!("could not fast-forward {base} in {}", path.display()))?;
        }
        None => {
            git::git_output(
                repo,
                &["update-ref", &format!("refs/heads/{base}"), to, from],
            )
            .with_context(|| format!("could not fast-forward {base}"))?;
        }
    }
    Ok(())
}

/// The forge the base policy at `reference` names.
fn base_forge(
    repo: &Path,
    reference: &str,
) -> Result<Option<atelier_app::workflow_policy::ForgeConfig>> {
    Ok(base_policy::from_ref(repo, reference)?.policy.forge)
}

fn is_ancestor(repo: &Path, ancestor: &str, of: &str) -> bool {
    git::git_output(repo, &["merge-base", "--is-ancestor", ancestor, of]).is_ok()
}

/// Whether GitHub marks pull request `number` merged within `wait`. A read
/// that fails counts as not merged yet.
fn merged_within(github: &Github, number: u64, wait: Duration) -> bool {
    let deadline = Instant::now() + wait;
    loop {
        if github.pull_request_merged(number).unwrap_or(false) {
            return true;
        }
        let now = Instant::now();
        if now >= deadline {
            return false;
        }
        std::thread::sleep(PULL_REQUEST_POLL.min(deadline - now));
    }
}

/// Tests set the wait with `ATELIER_TEST_MERGE_PR_WAIT_SECS`.
fn pull_request_wait() -> Duration {
    #[cfg(feature = "test-hooks")]
    if let Some(seconds) = std::env::var("ATELIER_TEST_MERGE_PR_WAIT_SECS")
        .ok()
        .and_then(|value| value.parse().ok())
    {
        return Duration::from_secs(seconds);
    }
    PULL_REQUEST_WAIT
}

fn one_line(stderr: &[u8]) -> String {
    String::from_utf8_lossy(stderr)
        .lines()
        .map(str::trim)
        // git's hints suggest `git pull`, which the Next replaces.
        .filter(|line| !line.is_empty() && !line.starts_with("hint:"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn short(commit: &str) -> &str {
    crate::human_output::short_commit(commit)
}
