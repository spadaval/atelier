//! `merge --push`: merge, then publish the base through the policy's forge
//! (ADR 0031; docs/product/reviews-and-merge.md, "`merge --push`, step by
//! step"). It checks `gh`, pushes the work branch and makes sure it has a
//! pull request, fetches the base from `forge.remote`, merges as `merge`
//! does, judges the unpushed base commits as `land` does, pushes the base
//! and confirms the pull request merged, so one exit status covers
//! integrating and publishing.
use super::git::{resolve, text};
use super::merge::{attempts, base_checkout, rerun};
use anyhow::{anyhow, bail, Context, Result};
use atelier_app::{
    base_policy, committed_snapshot,
    forge::{ForgeError, Github, PullRequest},
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
/// `unpushed` ends the verdict of a refusal: it says the base was not
/// pushed once the branch was.
pub(super) fn run(
    issue_ref: Option<&str>,
    quiet: bool,
    wait: Option<Duration>,
    unpushed: &mut String,
) -> Result<String> {
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
        attempts(issue_ref, false, false, quiet, true, None)?;
        bail!("merge_push_base_branch: merge --push runs from the work branch. Next: check out the work branch, then `{}`", rerun(issue_ref, true));
    };
    let rerun = rerun(issue_ref, true);
    // Before step 2 nothing is pushed; after it, the branch is.
    let no_forge = |pushed: &str| {
        anyhow!("merge_push_no_forge: the {base} policy names no forge, so merge --push has nowhere to publish; {pushed}. Next: `{}`, then `atelier land --push` from the {base} checkout", super::merge::rerun(issue_ref, false))
    };
    let Some(forge) = base_forge(repo, &format!("refs/heads/{base}"))? else {
        return Err(no_forge("nothing was pushed"));
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

    // 2 and 3. The branch and its pull request, whatever the gates say
    // next: pushing a work branch is always allowed, and starts CI early.
    // A checkout whose owner cannot be read refuses in the gates, which
    // read it the same way, and pushes nothing.
    let owned = match super::merge::committed_branch(issue_ref) {
        Ok(owned) => owned,
        Err(error) => {
            attempts(issue_ref, true, false, quiet, true, None)?;
            return Err(error);
        }
    };
    push_branch(repo, remote, &branch, &base, &rerun)?;
    *unpushed = format!(", {base} not pushed");
    let (pull, opened) = ensure_pull_request(&github, repo, &owned, &branch, &base)?;
    say(&format!(
        "Pull request #{}: {} ({})",
        pull.number,
        pull.url,
        if opened { "opened now" } else { "already open" }
    ));

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
                "merge_push_base_diverged: local {base} at {} and {base} on {remote} at {} have diverged, so a merge onto local {base} could not be pushed; nothing was merged, and {base} was not pushed. Next: in the {base} checkout, `git fetch {remote} {base} && git merge --no-edit FETCH_HEAD` and `atelier land`, then `git push {remote} {base}`, then back on {branch}, `{rerun}`",
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
            None => no_forge(&format!("{branch} was pushed, and {base} was not")),
            Some(now) => anyhow!(
                "merge_push_forge_changed: {base} at {} names forge remote {} with workflow {}, not remote {remote} with workflow {} as {base} did before {after}; {base} was not pushed. Next: rerun `{rerun}`, which publishes through the forge {base} names now",
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
    let integrated = attempts(issue_ref, false, false, quiet, true, wait)?
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
        anyhow!("{error:#} {base} was not pushed; merge commit {merged} stays on {base}.")
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

    // 8. GitHub marks the pull request step 3 ensured merged once its head
    // is on the base.
    let number = pull.number;
    if merged_within(&github, number, pull_request_wait()) {
        return Ok(format!("{published}, pull request #{number} merged"));
    }
    say(&format!(
        "Note: {published} at {merged}; pull request #{number} is still open because GitHub has not marked it merged. Atelier does not close it, since closed is not merged. Next: gh pr view {number}"
    ));
    Ok(published)
}

/// Pushes the committed branch head to `remote` under the branch's name,
/// never forcing, and sets the branch's upstream when it has none and the
/// remote is a name rather than a URL.
fn push_branch(repo: &Path, remote: &str, branch: &str, base: &str, rerun: &str) -> Result<()> {
    let has_upstream = git::git_output(
        repo,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}@{{upstream}}"),
        ],
    )
    .is_ok();
    let named = git::git_output(repo, &["remote", "get-url", "--", remote]).is_ok();
    let mut push = git::command(repo);
    push.args(["push", "--quiet"]);
    if named && !has_upstream {
        push.arg("--set-upstream");
    }
    let pushed = push
        .args([remote, &format!("refs/heads/{branch}:refs/heads/{branch}")])
        .output()
        .context("could not run git push")?;
    if pushed.status.success() {
        return Ok(());
    }
    // git marks a push the remote branch's own commits refuse `[rejected]`;
    // anything else, such as a remote that wants credentials, is no
    // reason to merge the remote branch in.
    let stderr = String::from_utf8_lossy(&pushed.stderr);
    if stderr.contains("[rejected]") {
        bail!(
            "merge_push_branch_rejected: git push to {remote} refused {branch}: {}. The remote branch has commits this one lacks; nothing was pushed. Next: `git fetch {remote} {branch} && git merge --no-edit FETCH_HEAD`, then `{rerun}`",
            one_line(&pushed.stderr)
        );
    }
    bail!(
        "merge_push_branch_failed: git push to {remote} could not push {branch}: {}; nothing was pushed. Next: check that `git push {remote} {branch}` works, then `{rerun}`",
        one_line(&pushed.stderr)
    )
}

/// The open pull request from `branch` into `base`, opened when there is
/// none, and whether this call opened it. Its title is `<id>: <title>`, or
/// on the light path the branch's newest commit subject.
fn ensure_pull_request(
    github: &Github,
    repo: &Path,
    owned: &super::merge::Owned,
    branch: &str,
    base: &str,
) -> Result<(PullRequest, bool)> {
    let refusal = |error: ForgeError, doing: &str| {
        let reason = match &error {
            ForgeError::Unreachable(message) => {
                format!(
                    "gh could not {doing} the pull request from {branch} into {base}: {message}"
                )
            }
            other => other.reason(),
        };
        anyhow!(
            "{}: {reason}; {branch} was pushed, and {base} was not. Next: {}",
            error.code(),
            error.next()
        )
    };
    if let Some(pull) = github
        .open_pull_request(branch, base)
        .map_err(|error| refusal(error, "look up"))?
    {
        return Ok((pull, false));
    }
    let (title, body) = if owned.light {
        (
            light_title(repo, &owned.head, base)?,
            format!("Branch {branch}, which no Atelier item owns."),
        )
    } else {
        let issue = owned.index.require_issue(&owned.id)?;
        (
            format!("{}: {}", owned.id, issue.title),
            format!("Atelier item {}.", owned.id),
        )
    };
    let body = format!(
        "{body}\n\nReview is recorded in `.atelier/reviews` on the branch, not on this pull request. `atelier merge --push` merges it and publishes {base}."
    );
    let pull = github
        .create_pull_request(branch, base, &title, &body)
        .map_err(|error| refusal(error, "open"))?;
    Ok((pull, true))
}

/// A light-path pull request's title: the subject of the branch's newest
/// commit that changes code, since its head is often a record-only
/// approval, or of its newest commit when every commit changes only records.
fn light_title(repo: &Path, head: &str, base: &str) -> Result<String> {
    let fork = text(repo, &["merge-base", &format!("refs/heads/{base}"), head])?;
    let range = format!("{fork}..{head}");
    let mut args = vec!["log", "-1", "--format=%s", range.as_str(), "--"];
    args.extend(atelier_app::ci_passed::CODE);
    let subject = text(repo, &args)?;
    if !subject.is_empty() {
        return Ok(subject);
    }
    text(repo, &["log", "-1", "--format=%s", head])
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
            "merge_push_fetch_failed: git fetch from {remote} could not read {base}: {}; {base} was not pushed. Next: check that `git fetch {remote} {base}` works, then retry",
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
                    anyhow!("{error:#} The {base} checkout at {} must be clean before merge --push fast-forwards it; nothing was merged, and {base} was not pushed.", path.display())
                })?;
            let overwritten = super::git::untracked_overwritten(&path, &[to])?;
            if !overwritten.is_empty() {
                bail!(
                    "merge_untracked_overwritten: {} has {} where {base} on the forge has files, so fast-forwarding would overwrite them: {}; nothing was merged, and {base} was not pushed. Next: move them out of the way, or commit them, then retry",
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
