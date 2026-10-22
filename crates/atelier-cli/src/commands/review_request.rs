//! `atelier review request <id>`: the reviewer brief an orchestrator hands to
//! an independent reviewer (atelier-rc8p). Everything in it comes from
//! records, Git and policy; it carries the item's requirements and the review
//! history, never the author's rationale (no notes, no Close Reason).
use crate::commands::git::{resolve, text};
use anyhow::{bail, Context, Result};
use atelier_app::project_config::ProjectConfig;
use atelier_app::record_index::RecordIndex;
use atelier_app::record_ref::RecordKind;
use atelier_app::storage_layout::StorageLayout;
use atelier_app::workflow_policy::ValidatorParams;
use atelier_core::{FindingDecision, ReviewEvent};
use atelier_records::IssueSectionName;

/// Longest summary the brief quotes; `review show` has the rest.
const SUMMARY_LIMIT: usize = 300;
/// Receipts listed, newest first.
const PROOF_LIMIT: usize = 8;
const REVIEW_PROCEDURE: &str = ".agents/skills/agent-factory/procedures/review.md";

/// Commits to review, from `from` (exclusive) to the brief's head.
#[derive(Debug, PartialEq)]
pub struct Range {
    pub from: String,
    pub commits: usize,
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub id: String,
    pub status: String,
    pub title: String,
    pub done_when: Option<String>,
}

/// A line of review history: a finding with its latest disposition, or an
/// account or approval.
#[derive(Debug, PartialEq)]
pub struct HistoryLine {
    pub id: String,
    pub what: String,
    pub commit: String,
    pub identity: String,
    pub summary: String,
}

#[derive(Debug, PartialEq)]
pub struct Receipt {
    pub id: String,
    pub result: String,
    pub kind: String,
    pub commit: String,
    pub target: Option<String>,
    pub summary: String,
}

/// Everything the brief says, gathered; [`render`] only lays it out.
#[derive(Debug, PartialEq)]
pub struct Facts {
    pub owner: Item,
    pub issue_type: String,
    pub goal: Option<String>,
    pub children: Vec<Item>,
    pub branch: String,
    pub base: String,
    pub head: String,
    /// Changes since the last review or account, when there was one.
    pub since: Option<(Range, String)>,
    pub whole: Range,
    pub uncommitted: usize,
    pub findings: Vec<HistoryLine>,
    pub reviews: Vec<HistoryLine>,
    pub proof: Vec<Receipt>,
    pub checks: Vec<String>,
    pub rules: Option<String>,
    pub procedure: bool,
    pub identity: Option<String>,
}

/// Quiet output is the range to review, `<from>..<head>` in full commits.
pub fn run(id: &str, identity: Option<&str>, from: Option<&str>, quiet: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let facts = gather(&layout, id, identity, from)?;
    if quiet {
        println!("{}..{}", facts.review_range().from, facts.head);
    } else {
        print!("{}", render(&facts));
    }
    Ok(())
}

impl Facts {
    /// The commits to review: those since the last review when there was one.
    fn review_range(&self) -> &Range {
        self.since.as_ref().map_or(&self.whole, |(range, _)| range)
    }
}

fn gather(
    layout: &StorageLayout,
    id: &str,
    identity: Option<&str>,
    from: Option<&str>,
) -> Result<Facts> {
    let repo = layout.repo_root();
    let index = RecordIndex::load_structural(&layout.canonical_dir())?;
    let id = index.resolve(RecordKind::Issue, id)?;
    let issue = index.require_issue(&id)?;
    let owner_id = issue.parent_id.clone().unwrap_or_else(|| id.clone());
    let owner = index
        .issue_record(&owner_id)
        .with_context(|| format!("{owner_id} is not an issue"))?;
    let branch = owner.issue.workflow_branch.as_ref().with_context(|| {
        format!("review_request_unstarted: {owner_id} has no branch ownership, so there is no work to review. Next: run `atelier start {owner_id}`")
    })?;
    let head = resolve(repo, "HEAD")?;
    let current = crate::commands::git::checkout_branch(repo)?.unwrap_or_default();
    if current != branch.work_branch {
        bail!(
            "review_request_branch: {owner_id} works on {} but this checkout is on {current}. Next: run the request from the owner's worktree",
            branch.work_branch
        );
    }
    let base_head = resolve(repo, &format!("refs/heads/{}", branch.branch_base))
        .with_context(|| format!("base branch {} not found", branch.branch_base))?;
    let fork = text(repo, &["merge-base", &base_head, &head])?;
    let count = |from: &str| -> Result<usize> {
        Ok(text(repo, &["rev-list", "--count", &format!("{from}..{head}")])?.parse()?)
    };
    let whole = Range {
        commits: count(&fork)?,
        from: fork,
    };

    let scope = atelier_app::review_gate::scope(&index, &owner_id)?;
    let mut events = index
        .reviews()
        .filter(|record| scope.contains(&record.target))
        .collect::<Vec<_>>();
    events.sort_by(|a, b| {
        (a.header.created_at, &a.header.id).cmp(&(b.header.created_at, &b.header.id))
    });

    let mut findings = Vec::new();
    let mut reviews = Vec::new();
    let mut last_account = None;
    for record in &events {
        let line = |what: String| HistoryLine {
            id: record.header.id.clone(),
            what,
            commit: record.commit.clone(),
            identity: record.identity.clone(),
            summary: record.summary.clone(),
        };
        match &record.event {
            ReviewEvent::Finding { recommendation } => {
                let state = match index.latest_disposition(&record.header.id) {
                    None => "open, no disposition".to_owned(),
                    Some(disposition) => match &disposition.event {
                        ReviewEvent::Disposition {
                            decision,
                            resolution_commit,
                            follow_up,
                            ..
                        } => {
                            let mut state =
                                format!("{} by {}", decision.as_str(), disposition.identity);
                            if let Some(commit) = resolution_commit {
                                state.push_str(&format!(" at {}", short(commit)));
                            }
                            if let Some(follow_up) = follow_up {
                                state.push_str(&format!(" to {follow_up}"));
                            }
                            state
                        }
                        _ => "open, no disposition".to_owned(),
                    },
                };
                findings.push(line(format!("{} | {state}", recommendation.as_str())));
            }
            ReviewEvent::Review | ReviewEvent::Approval | ReviewEvent::TrivialExemption => {
                let what = match &record.event {
                    ReviewEvent::Review => "review",
                    ReviewEvent::Approval => "approval",
                    _ => "trivial exemption",
                };
                if matches!(record.event, ReviewEvent::Review | ReviewEvent::Approval) {
                    last_account = Some(record);
                }
                reviews.push(line(what.to_owned()));
            }
            ReviewEvent::Disposition { .. } => {}
        }
    }
    // Open findings first; the rest keep their order.
    findings.sort_by_key(|finding| !finding.what.contains("open, no disposition"));

    let since = match from {
        Some(reference) => {
            let from = resolve(repo, reference)
                .with_context(|| format!("--from {reference} is not a commit"))?;
            atelier_app::git_freshness::git_output(
                repo,
                &["merge-base", "--is-ancestor", &from, &head],
            )
            .with_context(|| format!("--from {reference} is not an ancestor of HEAD"))?;
            Some((
                Range {
                    commits: count(&from)?,
                    from,
                },
                "from --from".to_owned(),
            ))
        }
        None => match last_account {
            Some(record) if record.commit != head => {
                let reachable = atelier_app::git_freshness::git_output(
                    repo,
                    &["merge-base", "--is-ancestor", &record.commit, &head],
                )
                .is_ok();
                reachable
                    .then(|| -> Result<_> {
                        Ok((
                            Range {
                                commits: count(&record.commit)?,
                                from: record.commit.clone(),
                            },
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
                        ))
                    })
                    .transpose()?
            }
            _ => None,
        },
    };

    let mut children = Vec::new();
    for child in index.child_ids(&owner_id) {
        if let Some(record) = index.issue_record(child) {
            children.push(item(record));
        }
    }
    let mut proof = index
        .evidence_records()
        .into_iter()
        .filter(|record| {
            record
                .data
                .target
                .as_ref()
                .is_some_and(|target| target.kind == "issue" && scope.contains(&target.id))
        })
        .collect::<Vec<_>>();
    proof.sort_by(|a, b| {
        (b.header.created_at, &b.header.id).cmp(&(a.header.created_at, &a.header.id))
    });
    let proof = proof
        .into_iter()
        .take(PROOF_LIMIT)
        .map(|record| Receipt {
            id: record.header.id.clone(),
            result: record.data.result.as_str().to_owned(),
            kind: record.data.evidence_type.clone(),
            commit: record.data.commit.clone(),
            target: record.data.target.as_ref().map(|target| target.id.clone()),
            summary: record.header.title.clone(),
        })
        .collect();

    let policy = atelier_app::workflow_policy::load(repo)?;
    let checks = policy
        .merge_checks
        .iter()
        .filter_map(|check| match &check.params {
            Some(ValidatorParams::ShellCheck { command }) => Some(command.clone()),
            _ => None,
        })
        .collect();
    let rules = match ProjectConfig::load(repo)?.review.rules_file {
        Some(path) => Some(
            std::fs::read_to_string(repo.join(&path))
                .with_context(|| format!("review.rules_file {path} cannot be read"))?,
        ),
        None => None,
    };
    let uncommitted = text(repo, &["status", "--porcelain", "--untracked-files=no"])?
        .lines()
        .count();
    Ok(Facts {
        owner: item(owner),
        issue_type: owner.issue.issue_type.clone(),
        goal: owner
            .sections
            .section(IssueSectionName::Goal)
            .map(str::to_owned),
        children,
        branch: branch.work_branch.clone(),
        base: branch.branch_base.clone(),
        head,
        since,
        whole,
        uncommitted,
        findings,
        reviews,
        proof,
        checks,
        rules,
        procedure: repo.join(REVIEW_PROCEDURE).is_file(),
        identity: identity.map(str::to_owned),
    })
}

fn item(record: &atelier_records::CanonicalIssueRecord) -> Item {
    Item {
        id: record.issue.id.clone(),
        status: record.issue.status.clone(),
        title: record.issue.title.clone(),
        done_when: record
            .sections
            .section(IssueSectionName::DoneWhen)
            .map(str::to_owned),
    }
}

fn short(commit: &str) -> &str {
    commit.get(..8).unwrap_or(commit)
}

fn flat(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// One line of at most [`SUMMARY_LIMIT`] characters.
fn one_line(text: &str) -> String {
    let flat = flat(text);
    if flat.chars().count() <= SUMMARY_LIMIT {
        return flat;
    }
    let cut = flat.chars().take(SUMMARY_LIMIT).collect::<String>();
    format!("{}... (atelier review show)", cut.trim_end())
}

/// The brief as dense Markdown (ADR 0022): short headers, one line per fact.
pub fn render(facts: &Facts) -> String {
    let mut out = String::new();
    let owner = &facts.owner;
    out.push_str(&format!(
        "# Review request: {} {} {} {}\n\n",
        owner.id, facts.issue_type, owner.status, owner.title
    ));
    out.push_str("You are an independent reviewer. You did not write this code. Judge it against the requirements below, not the author's intent");
    if facts.procedure {
        out.push_str(&format!(", following `{REVIEW_PROCEDURE}`"));
    }
    out.push_str(".\n\n");

    out.push_str("## Range\n\n");
    let whole_label = match &facts.since {
        Some((range, why)) => {
            out.push_str(&format!(
                "- Review: `{}..{}` ({}, {why})\n",
                short(&range.from),
                short(&facts.head),
                count(range.commits)
            ));
            "Whole branch"
        }
        None => "Review: whole branch",
    };
    out.push_str(&format!(
        "- {whole_label}: `{}..{}` ({}) on `{}`, forked from `{}`\n",
        short(&facts.whole.from),
        short(&facts.head),
        count(facts.whole.commits),
        facts.branch,
        facts.base
    ));
    out.push_str(&format!(
        "- Head, to pin records to: `{}`\n- Inspect: `git log --oneline {}..{}`, `git diff {}..{}`\n",
        facts.head,
        short(&facts.review_range().from),
        short(&facts.head),
        short(&facts.review_range().from),
        short(&facts.head),
    ));
    if facts.uncommitted > 0 {
        out.push_str(&format!(
            "- Checkout: {} with changes not committed; review commits only, and do not commit changes you did not make\n",
            atelier_core::count_noun(facts.uncommitted, "path")
        ));
    }

    if let Some(goal) = &facts.goal {
        out.push_str(&format!("\n## Goal\n\n{}\n", goal.trim()));
    }
    if let Some(done) = &owner.done_when {
        out.push_str(&format!("\n## Done when\n\n{}\n", done.trim()));
    }
    if !facts.children.is_empty() {
        out.push_str("\n## Items\n\n");
        for child in &facts.children {
            out.push_str(&format!(
                "- {} {} {}\n",
                child.id, child.status, child.title
            ));
            if let Some(done) = &child.done_when {
                out.push_str(&format!("  Done when: {}\n", flat(done)));
            }
        }
    }

    out.push_str("\n## Findings so far\n\n");
    if facts.findings.is_empty() {
        out.push_str("None.\n");
    }
    for finding in &facts.findings {
        out.push_str(&format!(
            "- {} {} @ {} {}: {}\n",
            finding.id,
            finding.what,
            short(&finding.commit),
            finding.identity,
            one_line(&finding.summary)
        ));
    }
    if !facts.reviews.is_empty() {
        out.push_str("\n## Earlier reviews\n\n");
        for review in &facts.reviews {
            out.push_str(&format!(
                "- {} {} @ {} {}: {}\n",
                review.id,
                review.what,
                short(&review.commit),
                review.identity,
                one_line(&review.summary)
            ));
        }
    }
    if !facts.proof.is_empty() {
        out.push_str("\n## Proof recorded\n\n");
        for receipt in &facts.proof {
            out.push_str(&format!(
                "- {} {} {} @ {}{}: {}\n",
                receipt.id,
                receipt.result,
                receipt.kind,
                short(&receipt.commit),
                receipt
                    .target
                    .as_ref()
                    .map(|target| format!(" on {target}"))
                    .unwrap_or_default(),
                one_line(&receipt.summary)
            ));
        }
    }

    out.push_str("\n## Working rules\n\n");
    out.push_str("- Stay read-only on code: do not edit it, commit code, switch branches, rebase, merge or push. Your only writes are review records.\n");
    out.push_str("- Judge in order: does it do what Done when says, then is it fit to integrate. Check the proof exercises the claims.\n");
    if facts.checks.is_empty() {
        out.push_str("- Run the proof yourself and read the full output.\n");
    } else {
        out.push_str("- Run the proof yourself and read the full output. The merge checks are:\n");
        for check in &facts.checks {
            out.push_str(&format!("  - `{check}`\n"));
        }
    }
    out.push_str("- Approve only when you recommend no `fix_now` finding and the proof passes. An approval identity must differ from the code's Git authors.\n");
    if let Some(rules) = &facts.rules {
        out.push_str(&format!(
            "\n### This repository's rules\n\n{}\n",
            rules.trim()
        ));
    }

    let who = facts.identity.as_deref().unwrap_or("<your identity>");
    out.push_str(&format!(
        "\n## Record\n\nPin every record to the head. `--commit-records` commits only the record it writes.\n\n```\natelier review finding {id} --identity \"{who}\" --commit {head} --recommendation {rec} --commit-records \"<severity>: <path:line> <problem, impact, suggested fix>\"\natelier review approve {id} --identity \"{who}\" --commit {head} --commit-records \"<what you checked>\"\n```\n",
        id = owner.id,
        head = facts.head,
        rec = [
            FindingDecision::FixNow,
            FindingDecision::Defer,
            FindingDecision::NoAction
        ]
        .map(FindingDecision::as_str)
        .join("|"),
    ));
    out.push_str("\nReport: Contract and Fitness verdicts, findings (R-number, severity, recommendation), open questions, residual risk, and the IDs of the records you created.\n");
    out
}

fn count(commits: usize) -> String {
    atelier_core::count_noun(commits, "commit")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts() -> Facts {
        let head = "2".repeat(40);
        Facts {
            owner: Item {
                id: "atelier-own1".into(),
                status: "review".into(),
                title: "Do the thing".into(),
                done_when: Some("A test shows it.\nSecond line.".into()),
            },
            issue_type: "feature".into(),
            goal: Some("Do the thing.".into()),
            children: vec![Item {
                id: "atelier-chd1".into(),
                status: "done".into(),
                title: "Part one".into(),
                done_when: Some("Part one works.".into()),
            }],
            branch: "feature/atelier-own1".into(),
            base: "main".into(),
            head: head.clone(),
            since: Some((
                Range {
                    from: "1".repeat(40),
                    commits: 1,
                },
                "since the last approval by Rae (atelier-rev1)".into(),
            )),
            whole: Range {
                from: "0".repeat(40),
                commits: 4,
            },
            uncommitted: 0,
            findings: vec![
                HistoryLine {
                    id: "atelier-fnd1".into(),
                    what: "fix_now | open, no disposition".into(),
                    commit: "1".repeat(40),
                    identity: "Rae".into(),
                    summary: "high: src/a.rs:3 panics\non empty input".into(),
                },
                HistoryLine {
                    id: "atelier-fnd2".into(),
                    what: "defer | defer by Lee to atelier-fol1".into(),
                    commit: "1".repeat(40),
                    identity: "Rae".into(),
                    summary: "low: nit".into(),
                },
            ],
            reviews: vec![HistoryLine {
                id: "atelier-rev1".into(),
                what: "approval".into(),
                commit: "1".repeat(40),
                identity: "Rae".into(),
                summary: "Read the diff.".into(),
            }],
            proof: vec![Receipt {
                id: "atelier-evd1".into(),
                result: "pass".into(),
                kind: "test".into(),
                commit: head,
                target: Some("atelier-chd1".into()),
                summary: "Focused tests pass".into(),
            }],
            checks: vec!["cargo test".into()],
            rules: Some("Use the repo's scratch directory.\n".into()),
            procedure: true,
            identity: Some("Independent reviewer".into()),
        }
    }

    #[test]
    fn the_brief_renders_as_dense_markdown() {
        let expected = "\
# Review request: atelier-own1 feature review Do the thing

You are an independent reviewer. You did not write this code. Judge it against the requirements below, not the author's intent, following `.agents/skills/agent-factory/procedures/review.md`.

## Range

- Review: `11111111..22222222` (1 commit, since the last approval by Rae (atelier-rev1))
- Whole branch: `00000000..22222222` (4 commits) on `feature/atelier-own1`, forked from `main`
- Head, to pin records to: `2222222222222222222222222222222222222222`
- Inspect: `git log --oneline 11111111..22222222`, `git diff 11111111..22222222`

## Goal

Do the thing.

## Done when

A test shows it.
Second line.

## Items

- atelier-chd1 done Part one
  Done when: Part one works.

## Findings so far

- atelier-fnd1 fix_now | open, no disposition @ 11111111 Rae: high: src/a.rs:3 panics on empty input
- atelier-fnd2 defer | defer by Lee to atelier-fol1 @ 11111111 Rae: low: nit

## Earlier reviews

- atelier-rev1 approval @ 11111111 Rae: Read the diff.

## Proof recorded

- atelier-evd1 pass test @ 22222222 on atelier-chd1: Focused tests pass

## Working rules

- Stay read-only on code: do not edit it, commit code, switch branches, rebase, merge or push. Your only writes are review records.
- Judge in order: does it do what Done when says, then is it fit to integrate. Check the proof exercises the claims.
- Run the proof yourself and read the full output. The merge checks are:
  - `cargo test`
- Approve only when you recommend no `fix_now` finding and the proof passes. An approval identity must differ from the code's Git authors.

### This repository's rules

Use the repo's scratch directory.

## Record

Pin every record to the head. `--commit-records` commits only the record it writes.

```
atelier review finding atelier-own1 --identity \"Independent reviewer\" --commit 2222222222222222222222222222222222222222 --recommendation fix_now|defer|no_action --commit-records \"<severity>: <path:line> <problem, impact, suggested fix>\"
atelier review approve atelier-own1 --identity \"Independent reviewer\" --commit 2222222222222222222222222222222222222222 --commit-records \"<what you checked>\"
```

Report: Contract and Fitness verdicts, findings (R-number, severity, recommendation), open questions, residual risk, and the IDs of the records you created.
";
        assert_eq!(render(&facts()), expected);
    }
}
