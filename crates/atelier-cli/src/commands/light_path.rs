//! The light path from the checkout (ADR 0028 decision 6, atelier-4nsl): a
//! branch no issue owns, such as a release, docs or a mockup, is reviewed
//! with `atelier review request` and merged with `atelier merge`, both with
//! no ID, on an independent approval of its head. `atelier_app::light_path`
//! judges it; this module says which path the checkout is on and writes the
//! reviewer's brief.
use crate::commands::git::{resolve, text};
use anyhow::{bail, Context, Result};
use atelier_app::{
    git_freshness as git, project_config::ProjectConfig, record_index::RecordIndex,
    workflow_policy::ValidatorParams,
};
use atelier_core::{FindingDecision, ReviewEvent, ReviewRecord};
use std::path::Path;

/// Longest summary the brief quotes; `review show` has the rest.
const SUMMARY_LIMIT: usize = 300;
/// Commit subjects the brief lists before `and <n> more`.
const COMMITS_SHOWN: usize = 30;

/// Which path the checked-out branch takes.
pub(crate) enum Checkout {
    /// A detached HEAD, which has no branch to review or merge.
    Detached,
    /// The base branch, which is never a light-path branch.
    Base(String),
    /// A branch an open issue owns: the issue path.
    Owned(Vec<String>),
    /// A branch no issue owns: the light path.
    Light(LightBranch),
}

/// A branch on the light path and where it stands against the base.
pub(crate) struct LightBranch {
    pub branch: String,
    pub base: String,
    pub base_head: String,
    pub head: String,
    /// Where the branch left the base: the merge base of the base's tip and
    /// HEAD.
    pub fork: String,
}

/// Classifies the checkout, whose records `index` holds, against the base
/// branch `base`.
pub(crate) fn checkout(repo: &Path, index: &RecordIndex, base: &str) -> Result<Checkout> {
    let Some(branch) = crate::commands::git::checkout_branch(repo)? else {
        return Ok(Checkout::Detached);
    };
    if branch == base {
        return Ok(Checkout::Base(branch));
    }
    let owners = crate::commands::session::branch_owners(repo, index, &branch);
    if !owners.is_empty() {
        return Ok(Checkout::Owned(owners));
    }
    let base_head = resolve(repo, &format!("refs/heads/{base}"))
        .with_context(|| format!("base branch {base} not found locally. Next: git fetch"))?;
    let head = resolve(repo, "HEAD")?;
    let fork = text(repo, &["merge-base", &base_head, &head]).with_context(|| {
        format!("{branch} shares no history with {base}, so it has no commits to review against it")
    })?;
    Ok(Checkout::Light(LightBranch {
        branch,
        base: base.to_owned(),
        base_head,
        head,
        fork,
    }))
}

/// The one owner of an owned branch, or the refusal naming several.
pub(crate) fn single_owner(owners: &[String], branch_hint: &str) -> Result<String> {
    match owners {
        [owner] => Ok(owner.clone()),
        _ => bail!(
            "{branch_hint}: the checked-out branch has several owners: {}. Next: name the issue",
            owners.join(", ")
        ),
    }
}

/// `atelier review request` on a branch no issue owns: the brief for the
/// branch's commits since it left the base, with no Goal or Done when, since
/// no issue states them (atelier-4nsl). Quiet output is the range.
pub(crate) fn request(
    repo: &Path,
    index: &RecordIndex,
    light: &LightBranch,
    identity: Option<&str>,
    from: Option<&str>,
    quiet: bool,
) -> Result<()> {
    let LightBranch {
        branch,
        base,
        base_head,
        head,
        fork,
    } = light;
    // Only the branch's own commits: those the base has merged in are not.
    let count = |from: &str| -> Result<usize> {
        Ok(text(
            repo,
            &[
                "rev-list",
                "--count",
                &format!("{from}..{head}"),
                &format!("^{base_head}"),
            ],
        )?
        .parse()?)
    };
    let whole = count(fork)?;
    if whole == 0 {
        bail!("review_request_empty: {branch} has no commits since it left {base}, so there is nothing to review. Next: commit the change on {branch}, then rerun `atelier review request`");
    }
    let mut read = git::ReadContext::from_snapshot(repo, head.clone(), &[]);
    let run = atelier_app::light_path::run(index.branch_reviews(), &mut read, branch, fork);
    let last_account = run
        .iter()
        .rev()
        .find(|record| matches!(record.event, ReviewEvent::Review | ReviewEvent::Approval));
    let since = match from {
        Some(reference) => {
            let from = resolve(repo, reference)
                .with_context(|| format!("--from {reference} is not a commit"))?;
            git::git_output(repo, &["merge-base", "--is-ancestor", &from, head])
                .with_context(|| format!("--from {reference} is not an ancestor of HEAD"))?;
            Some((count(&from)?, from, "from --from".to_owned()))
        }
        None => match last_account {
            Some(record) if record.commit != *head => Some((
                count(&record.commit)?,
                record.commit.clone(),
                format!(
                    "since the last {} by {} ({})",
                    if record.event == ReviewEvent::Approval {
                        "approval"
                    } else {
                        "review"
                    },
                    record.identity,
                    record.header.id
                ),
            )),
            _ => None,
        },
    };
    let from = since.as_ref().map_or(fork.as_str(), |(_, from, _)| from);
    if quiet {
        println!("{from}..{head}");
        return Ok(());
    }
    let mut out = format!("# Review request: branch {branch}\n\n");
    out.push_str("You are an independent reviewer. You did not write this code. No issue states requirements for this branch: judge whether each change is correct and fit to integrate, from the commits themselves.\n\n");
    out.push_str("## Range\n\n");
    let whole_label = match &since {
        Some((commits, from, why)) => {
            out.push_str(&format!(
                "- Review: `{}..{}` ({}, {why})\n",
                short(from),
                short(head),
                count_noun(*commits)
            ));
            "Whole branch"
        }
        None => "Review: whole branch",
    };
    out.push_str(&format!(
        "- {whole_label}: `{}..{}` ({}) on `{branch}`, forked from `{base}`; no issue owns it, so it merges on an approval of its head\n",
        short(fork),
        short(head),
        count_noun(whole),
    ));
    out.push_str(&format!(
        "- Head, to pin records to: `{head}`\n- Inspect: `git log -p --reverse {}..{} ^{base}`, the branch's own commits without ones merged from the base\n",
        short(from),
        short(head),
    ));
    let uncommitted = text(repo, &["status", "--porcelain", "--untracked-files=no"])?
        .lines()
        .count();
    if uncommitted > 0 {
        out.push_str(&format!(
            "- Checkout: {} with changes not committed; review commits only, and do not commit changes you did not make\n",
            atelier_core::count_noun(uncommitted, "path")
        ));
    }
    let subjects = text(
        repo,
        &[
            "log",
            "--reverse",
            "--format=%h %s",
            &format!("{from}..{head}"),
            &format!("^{base_head}"),
        ],
    )?;
    let subjects = subjects.lines().collect::<Vec<_>>();
    out.push_str("\n## Commits\n\n");
    for subject in subjects.iter().take(COMMITS_SHOWN) {
        out.push_str(&format!("- {subject}\n"));
    }
    if subjects.len() > COMMITS_SHOWN {
        out.push_str(&format!(
            "- and {} more (`git log {}..{}`)\n",
            subjects.len() - COMMITS_SHOWN,
            short(from),
            short(head)
        ));
    }
    // Against the fork point, so the base's own changes a sync merged in
    // are not counted as the branch's.
    let stat = text(repo, &["diff", "--stat", &format!("{fork}..{head}")])?;
    out.push_str("\n## Diff stat (whole branch)\n\n");
    if stat.is_empty() {
        out.push_str("No file changes.\n");
    } else {
        out.push_str(&format!("```\n{stat}\n```\n"));
    }
    out.push_str("\n## Findings so far\n\n");
    let findings = run
        .iter()
        .filter(|record| matches!(record.event, ReviewEvent::Finding { .. }))
        .map(|record| (finding_state(&run, &mut read, record), *record))
        .collect::<Vec<_>>();
    if findings.is_empty() {
        out.push_str("None.\n");
    }
    // Open findings first; the rest keep their order.
    for open in [true, false] {
        for ((is_open, what), record) in &findings {
            if *is_open == open {
                out.push_str(&history_line(record, what));
            }
        }
    }
    let reviews = run
        .iter()
        .filter(|record| matches!(record.event, ReviewEvent::Review | ReviewEvent::Approval))
        .collect::<Vec<_>>();
    if !reviews.is_empty() {
        out.push_str("\n## Earlier reviews\n\n");
        for record in reviews {
            out.push_str(&history_line(record, record.event.name()));
        }
    }
    let policy = atelier_app::workflow_policy::load(repo)?;
    let checks = policy
        .merge_checks
        .iter()
        .filter_map(|check| match &check.params {
            Some(ValidatorParams::ShellCheck { command }) => Some(command.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    out.push_str("\n## Working rules\n\n");
    out.push_str("- Stay read-only on code: do not edit it, commit code, switch branches, rebase, merge or push. Your only writes are review records.\n");
    out.push_str("- Judge whether the change does what its commit messages say, then whether it is fit to integrate.\n");
    if checks.is_empty() {
        out.push_str("- Run the proof yourself and read the full output.\n");
    } else {
        out.push_str("- Run the proof yourself and read the full output. The merge checks are:\n");
        for check in &checks {
            out.push_str(&format!("  - `{check}`\n"));
        }
    }
    out.push_str("- Approve only when you recommend no `fix_now` finding and the proof passes. Your approval of a later commit resolves the `fix_now` findings recorded before it.\n");
    if let Some(path) = ProjectConfig::load(repo)?.review.rules_file {
        let rules = std::fs::read_to_string(repo.join(&path))
            .with_context(|| format!("review.rules_file {path} cannot be read"))?;
        out.push_str(&format!(
            "\n### This repository's rules\n\n{}\n",
            rules.trim()
        ));
    }
    let who = identity.unwrap_or("<your identity>");
    out.push_str(&format!(
        "\n## Record\n\nPin every record to the head. `--commit-records` commits the record it writes, with any records staged earlier by `--stage-records`.\n\n```\natelier review finding --branch {branch} --identity \"{who}\" --commit {head} --recommendation {rec} --commit-records \"<severity>: <path:line> <problem, impact, suggested fix>\"\natelier review approve --branch {branch} --identity \"{who}\" --commit {head} --commit-records \"<what you checked>\"\n```\n",
        rec = FindingDecision::ALL.map(FindingDecision::as_str).join("|"),
    ));
    out.push_str("\nReport: Correctness and Fitness verdicts, findings (R-number, severity, recommendation), open questions, residual risk, and the IDs of the records you created.\n");
    print!("{out}");
    Ok(())
}

/// Whether a finding is open, and what settles it, as the brief shows it.
fn finding_state(
    run: &[&ReviewRecord],
    read: &mut git::ReadContext<'_>,
    finding: &ReviewRecord,
) -> (bool, String) {
    let ReviewEvent::Finding {
        recommendation,
        disposition,
    } = &finding.event
    else {
        return (false, String::new());
    };
    let fixed = atelier_app::light_path::fix_approval(run, read, finding);
    let fix_now = disposition
        .as_ref()
        .map_or(*recommendation == FindingDecision::FixNow, |disposition| {
            disposition.decision == FindingDecision::FixNow
        });
    let mut parts = Vec::new();
    if let Some(disposition) = disposition {
        let mut state = format!(
            "{} by {}",
            disposition.decision.as_str(),
            disposition.identity
        );
        if let Some(follow_up) = &disposition.follow_up {
            state.push_str(&format!(" to {follow_up}"));
        }
        parts.push(state);
    }
    let open = match (fix_now, fixed) {
        (true, Some(approval)) => {
            parts.push(format!(
                "approved since by {} @ {}",
                approval.header.id,
                short(&approval.commit)
            ));
            false
        }
        (true, None) => {
            parts.push("open until an approval after its fix".to_owned());
            true
        }
        (false, _) if disposition.is_none() => {
            parts.push("open, no decision".to_owned());
            true
        }
        (false, _) => false,
    };
    (
        open,
        format!("{} | {}", recommendation.as_str(), parts.join(", ")),
    )
}

fn history_line(record: &ReviewRecord, what: &str) -> String {
    format!(
        "- {} {what} @ {} {}: {}\n",
        record.header.id,
        short(&record.commit),
        record.identity,
        one_line(&record.summary)
    )
}

/// Where a light-path branch's review stands, as a status or land line:
/// `✓ approved`, `⚠ approval stale` or `⚠ not approved`, with the commits
/// it holds since the base.
pub(crate) fn state_facts(
    repo: &Path,
    index: &RecordIndex,
    light: &LightBranch,
) -> Result<(atelier_app::review_gate::ApprovalState, usize)> {
    let mut read = git::ReadContext::from_snapshot(repo, light.head.clone(), &[]);
    let run = atelier_app::light_path::run(
        index.branch_reviews(),
        &mut read,
        &light.branch,
        &light.fork,
    );
    let state = atelier_app::light_path::standing(&run, &mut read, &light.fork)?.state;
    let commits = text(
        repo,
        &[
            "rev-list",
            "--count",
            &format!("{}..{}", light.fork, light.head),
            &format!("^{}", light.base_head),
        ],
    )?
    .parse()
    .context("git rev-list --count printed no count")?;
    Ok((state, commits))
}

/// The words for an approval state, as status rows mark one.
pub(crate) fn approval_words(state: atelier_app::review_gate::ApprovalState) -> &'static str {
    match state {
        atelier_app::review_gate::ApprovalState::Current => "✓ approved",
        atelier_app::review_gate::ApprovalState::Stale => "⚠ approval stale",
        atelier_app::review_gate::ApprovalState::Missing => "⚠ not approved",
    }
}

fn count_noun(commits: usize) -> String {
    atelier_core::count_noun(commits, "commit")
}

fn short(commit: &str) -> &str {
    commit.get(..8).unwrap_or(commit)
}

/// One line of at most [`SUMMARY_LIMIT`] characters.
fn one_line(text: &str) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= SUMMARY_LIMIT {
        return flat;
    }
    let cut = flat.chars().take(SUMMARY_LIMIT).collect::<String>();
    format!("{}... (atelier review show)", cut.trim_end())
}
