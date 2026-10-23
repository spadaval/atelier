//! Creation and inspection of immutable, self-declared review events.
use crate::commands::git::{resolve, text};
use crate::human_output::{count_noun, display_timestamp, local_date, short_commit};
use anyhow::{bail, Result};
use atelier_app::record_ref::RecordKind;
use atelier_app::review_gate::{self, ApprovalState};
use atelier_app::{git_freshness as git, record_index::RecordIndex, storage_layout::StorageLayout};
use atelier_core::{BaseExemption, FindingDecision, ReviewEvent, ReviewRecord};
use std::collections::BTreeSet;
use std::path::Path;

/// The longest summary line a row or confirmation shows; `review show`
/// prints the whole text.
const SUMMARY_LIMIT: usize = 100;
/// The paths a stale approval's edge lists before `and <n> more`.
const LISTED_PATHS: usize = 5;

pub fn append(
    owner_ref: &str,
    identity: &str,
    commit: Option<&str>,
    summary: &str,
    event: ReviewEvent,
    quiet: bool,
) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    let index = RecordIndex::load_structural(&layout.canonical_dir())?;
    let owner = index.resolve(RecordKind::Issue, owner_ref)?;
    let pin = commit.map(str::to_owned).unwrap_or(git::head_commit(repo)?);
    atelier_records::review::validate_commit(&pin)?;
    require_on_branch(repo, &pin)?;
    let mut finding_line = None;
    if let ReviewEvent::Disposition {
        finding,
        supersedes,
        resolution_commit,
        follow_up,
        ..
    } = &event
    {
        let record = index.require_review(finding)?;
        if !matches!(record.event, ReviewEvent::Finding { .. }) || record.target != owner {
            bail!("disposition must target the finding's owner");
        }
        if supersedes.as_ref().map(String::as_str)
            != index
                .latest_disposition(finding)
                .map(|record| record.header.id.as_str())
        {
            bail!("disposition successor is not current");
        }
        if let Some(commit) = resolution_commit {
            atelier_records::review::validate_commit(commit)?;
            require_on_branch(repo, commit)?;
            if commit == &record.commit
                || git::git_output(
                    repo,
                    &["merge-base", "--is-ancestor", &record.commit, commit],
                )
                .is_err()
            {
                bail!(
                    "resolution commit {commit} does not follow {}, the commit finding {} reviewed. Next: git log --format=%H {}..HEAD",
                    short_commit(&record.commit),
                    record.header.id,
                    record.commit
                );
            }
        }
        if let Some(id) = follow_up {
            index.require_issue(id)?;
        }
        finding_line = Some(first_line(&record.summary));
    }
    // Read before the record exists, so a failure here writes nothing.
    let authors = authors_at_head(repo, &index, &owner)?;
    let created = atelier_app::project_config::record_store(&layout.canonical_dir())?
        .create_review(&owner, identity, &pin, event, summary)?;
    let warning = author_warning(&created, &authors);
    if quiet {
        println!("{}", created.header.id);
        if let Some(warning) = warning {
            eprintln!("{warning}");
        }
        return Ok(());
    }
    println!("{}", confirmation(&created, finding_line.as_deref()));
    if let Some(warning) = warning {
        println!("{warning}");
    }
    Ok(())
}

/// A pin given on the command line must be a commit this branch holds.
fn require_on_branch(repo: &Path, commit: &str) -> Result<()> {
    if git::git_output(repo, &["merge-base", "--is-ancestor", commit, "HEAD"]).is_err() {
        bail!(
            "commit {commit} is not on this branch: Git does not know it, or HEAD does not contain it. Next: git log --format=%H"
        );
    }
    Ok(())
}

/// The substantive Git authors of the work on this branch since it left the
/// base, as `review.current` counts them; none on the base itself, or where
/// the base cannot be found, since they only decide a warning.
fn authors_at_head(repo: &Path, index: &RecordIndex, target: &str) -> Result<BTreeSet<String>> {
    let Ok(policy) = atelier_app::workflow_policy::load(repo) else {
        return Ok(BTreeSet::new());
    };
    let base = policy.branch_policy.base_branch;
    let Ok(fork) = text(repo, &["merge-base", &format!("refs/heads/{base}"), "HEAD"]) else {
        return Ok(BTreeSet::new());
    };
    let owner = branch_owner(index, target);
    let ids = review_gate::scope(index, &owner)?;
    let mut read = git::ReadContext::new(repo);
    review_gate::author_identities(&mut read, &fork, &ids)
}

/// The issue whose branch carries `id`'s work: `id` itself when it has
/// branch ownership or no parent, otherwise its parent.
fn branch_owner(index: &RecordIndex, id: &str) -> String {
    match index.issue_record(id) {
        Some(record) if record.issue.workflow_branch.is_none() => record
            .issue
            .parent_id
            .clone()
            .unwrap_or_else(|| id.to_owned()),
        _ => id.to_owned(),
    }
}

/// The warning for an event the gates will refuse because its identity is
/// one of the work's authors: an approval (`review.current`), or a defer or
/// no_action disposition (`findings.disposed`).
fn author_warning(record: &ReviewRecord, authors: &BTreeSet<String>) -> Option<String> {
    let gate = match &record.event {
        ReviewEvent::Approval => "review.current",
        ReviewEvent::Disposition {
            decision: FindingDecision::Defer | FindingDecision::NoAction,
            ..
        } => "findings.disposed",
        _ => return None,
    };
    (!review_gate::independent(record, authors))
        .then(|| format!("⚠ identity matches a Git author; this will not satisfy {gate}"))
}

/// One line saying what was recorded: the kind, ID, owner and short pin,
/// then what a disposition settled, or the step a finding needs next.
fn confirmation(record: &ReviewRecord, finding_line: Option<&str>) -> String {
    let id = &record.header.id;
    let on = format!("on {} @{}", record.target, short_commit(&record.commit));
    match &record.event {
        ReviewEvent::Review => format!("Review account {id} {on}"),
        ReviewEvent::Approval => format!("Approval {id} {on}"),
        ReviewEvent::TrivialExemption => format!("Trivial exemption {id} {on}"),
        ReviewEvent::Finding { recommendation } => {
            let decision = recommendation.as_str();
            let resolution = if *recommendation == FindingDecision::FixNow {
                " --resolution-commit ..."
            } else if *recommendation == FindingDecision::Defer {
                " --follow-up ..."
            } else {
                ""
            };
            format!(
                "Finding {id} {decision} {on}. Dispose: atelier review disposition {id} --identity \"...\" --decision {decision}{resolution} \"...\""
            )
        }
        ReviewEvent::Disposition {
            finding,
            decision,
            supersedes,
            resolution_commit,
            follow_up,
        } => {
            let mut line = format!(
                "Disposition {id} {} {on}: finding {finding} \"{}\"",
                decision.as_str(),
                finding_line.unwrap_or_default()
            );
            line.push_str(&disposition_extras(
                resolution_commit.as_deref(),
                follow_up.as_deref(),
                supersedes.as_deref(),
            ));
            line
        }
    }
}

/// `, fixed at <commit>`, `, follow-up <id>` and `, supersedes <id>`, as
/// far as a disposition has them.
fn disposition_extras(
    resolution: Option<&str>,
    follow_up: Option<&str>,
    supersedes: Option<&str>,
) -> String {
    let mut extras = String::new();
    if let Some(commit) = resolution {
        extras.push_str(&format!(", fixed at {}", short_commit(commit)));
    }
    if let Some(issue) = follow_up {
        extras.push_str(&format!(", follow-up {issue}"));
    }
    if let Some(previous) = supersedes {
        extras.push_str(&format!(", supersedes {previous}"));
    }
    extras
}

/// Records a trivial exemption of one commit on the base branch, with no
/// owner issue (atelier-mbrf). It is recorded on the base, for a commit the
/// base already holds, and `land` counts exactly that commit as covered.
pub fn exempt_base(commit: &str, identity: &str, summary: &str, quiet: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    // Loading checks the records are valid before one is added.
    RecordIndex::load_structural(&layout.canonical_dir())?;
    let base = atelier_app::workflow_policy::load(repo)?
        .branch_policy
        .base_branch;
    let branch = crate::commands::git::current_branch(repo)?;
    if branch.as_deref() != Some(base.as_str()) {
        bail!(
            "review_base_not_checked_out: a base exemption is recorded on {base}, for a commit {base} already holds; run it from a checkout of {base}"
        );
    }
    let pin = resolve(repo, commit)
        .map_err(|_| anyhow::anyhow!("review_base_commit: {commit} is not a commit"))?;
    if git::git_output(repo, &["merge-base", "--is-ancestor", &pin, "HEAD"]).is_err() {
        bail!("review_base_commit: {commit} is not on {base}; a base exemption covers a commit the base already holds");
    }
    let created = atelier_app::project_config::record_store(&layout.canonical_dir())?
        .create_base_exemption(&base, identity, &pin, summary)?;
    if quiet {
        println!("{}", created.header.id);
    } else {
        println!(
            "Trivial exemption {} of {} @{}; it covers this commit alone",
            created.header.id,
            created.base,
            short_commit(&created.commit)
        );
    }
    Ok(())
}

pub fn disposition(
    finding: &str,
    identity: &str,
    decision: FindingDecision,
    resolution: Option<String>,
    follow_up: Option<String>,
    summary: &str,
    quiet: bool,
) -> Result<()> {
    if decision == FindingDecision::Defer && follow_up.is_none() {
        bail!("DEFER requires --follow-up <issue-id>");
    }
    let layout = StorageLayout::discover()?;
    let index = RecordIndex::load_structural(&layout.canonical_dir())?;
    let finding = index.resolve(RecordKind::Review, finding)?;
    let original = index.require_review(&finding)?;
    // A swapped ID is the usual cause: say what the event is and where the
    // findings are listed (atelier-jiz0).
    if !matches!(original.event, ReviewEvent::Finding { .. }) {
        let kind = original.event.name().replace('_', " ");
        let article = if kind.starts_with(['a', 'e', 'i', 'o', 'u']) {
            "an"
        } else {
            "a"
        };
        bail!(
            "{finding} is {article} {kind}, not a finding; a disposition settles a finding. Next: atelier review list {}",
            original.target
        );
    }
    let follow_up = follow_up
        .map(|id| index.resolve(RecordKind::Issue, &id))
        .transpose()?;
    let event = ReviewEvent::Disposition {
        decision,
        supersedes: index
            .latest_disposition(&finding)
            .map(|record| record.header.id.clone()),
        finding,
        resolution_commit: resolution,
        follow_up,
    };
    append(&original.target, identity, None, summary, event, quiet)
}

/// Lists review events oldest first, one row each. For an issue, the rows
/// are its branch owner's (the parent's, for a child without its own
/// branch), followed by the owner's approval state and its findings with
/// their dispositions. Without an issue, base exemptions are listed too.
/// Quiet output is the listed event IDs, one per line; `--json` keeps the
/// records of the issue alone, as stored.
pub fn list(reference: Option<&str>, json: bool, quiet: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let index = RecordIndex::load_structural(&layout.canonical_dir())?;
    let Some(reference) = reference else {
        return list_all(&index, json, quiet);
    };
    let id = index.resolve(RecordKind::Issue, reference)?;
    if json {
        let values = index
            .reviews()
            .filter(|record| record.target == id)
            .map(serde_json::to_value)
            .collect::<serde_json::Result<Vec<_>>>()?;
        println!("{}", serde_json::to_string_pretty(&values)?);
        return Ok(());
    }
    let owner = branch_owner(&index, &id);
    let scope = review_gate::scope(&index, &owner)?;
    let mut records = index
        .reviews()
        .filter(|record| scope.contains(&record.target))
        .collect::<Vec<_>>();
    records.sort_by_key(|record| (record.header.created_at, record.header.id.clone()));
    if quiet {
        for record in records {
            println!("{}", record.header.id);
        }
        return Ok(());
    }
    if owner != id {
        println!("{id} is reviewed with its branch owner {owner}.");
    }
    if records.is_empty() {
        println!("No review events for {owner}.");
        return Ok(());
    }
    for record in &records {
        println!("{}", event_row(record, Some(&owner)));
    }
    println!();
    let repo = layout.repo_root();
    for line in approval_lines(repo, &index, &owner)? {
        println!("{line}");
    }
    for line in finding_lines(&index, &records) {
        println!("{line}");
    }
    Ok(())
}

fn list_all(index: &RecordIndex, json: bool, quiet: bool) -> Result<()> {
    if json {
        let mut values = index
            .reviews()
            .map(serde_json::to_value)
            .collect::<serde_json::Result<Vec<_>>>()?;
        for exemption in index.base_exemptions() {
            values.push(serde_json::to_value(exemption)?);
        }
        println!("{}", serde_json::to_string_pretty(&values)?);
        return Ok(());
    }
    let mut rows = index
        .reviews()
        .map(|record| (&record.header, event_row(record, None)))
        .chain(
            index
                .base_exemptions()
                .map(|exemption| (&exemption.header, exemption_row(exemption))),
        )
        .collect::<Vec<_>>();
    rows.sort_by_key(|(header, _)| (header.created_at, header.id.clone()));
    if rows.is_empty() && !quiet {
        println!("No review events.");
    }
    for (header, row) in rows {
        println!("{}", if quiet { &header.id } else { &row });
    }
    Ok(())
}

/// The summary's first non-empty line, cut to [`SUMMARY_LIMIT`].
fn first_line(summary: &str) -> String {
    let line = summary
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or_default();
    crate::utils::truncate(line, SUMMARY_LIMIT)
}

/// What an event is, as a row names it: its stored kind, a finding's
/// recommendation, or a disposition's decision and finding.
fn event_kind(event: &ReviewEvent) -> String {
    match event {
        ReviewEvent::Finding { recommendation } => format!("finding {}", recommendation.as_str()),
        ReviewEvent::Disposition {
            finding, decision, ..
        } => format!("disposition {} → {finding}", decision.as_str()),
        other => other.name().to_owned(),
    }
}

/// One review event as a list row:
/// `<id>  <kind>[  on <target>]  @<pin>  <identity>  <date>  <summary line>`.
/// The target is left out when it is `owner`, the issue the list is for.
/// `status` lists its Reviews with the same row.
pub(crate) fn event_row(record: &ReviewRecord, owner: Option<&str>) -> String {
    let target = if owner == Some(record.target.as_str()) {
        String::new()
    } else {
        format!("  on {}", record.target)
    };
    format!(
        "{}  {}{target}  @{}  {}  {}  {}",
        record.header.id,
        event_kind(&record.event),
        short_commit(&record.commit),
        record.identity,
        local_date(record.header.created_at),
        first_line(&record.summary)
    )
}

fn exemption_row(exemption: &BaseExemption) -> String {
    format!(
        "{}  trivial_exemption  of {}  @{}  {}  {}  {}",
        exemption.header.id,
        exemption.base,
        short_commit(&exemption.commit),
        exemption.identity,
        local_date(exemption.header.created_at),
        first_line(&exemption.summary)
    )
}

/// Where the owner's approval is judged. On its work branch, while the
/// branch holds commits the base does not, it is judged as `merge`'s
/// `review.current` judges it at the branch tip. Otherwise, as for merged
/// work, it is judged on the base as `land` judges history.
enum Judge<'r> {
    Branch {
        read: git::ReadContext<'r>,
        fork: String,
        base_head: String,
        ids: BTreeSet<String>,
        authors: BTreeSet<String>,
    },
    Base {
        read: git::ReadContext<'r>,
        base: String,
    },
}

impl<'r> Judge<'r> {
    fn new(repo: &'r Path, index: &RecordIndex, owner: &str) -> Result<Self> {
        let base = atelier_app::workflow_policy::load(repo)?
            .branch_policy
            .base_branch;
        let base_head = resolve(repo, &format!("refs/heads/{base}")).ok();
        let tip = index
            .issue_record(owner)
            .and_then(|record| record.issue.workflow_branch.as_ref())
            .and_then(|branch| resolve(repo, &format!("refs/heads/{}", branch.work_branch)).ok());
        if let (Some(tip), Some(base_head)) = (tip, base_head) {
            let fork = text(repo, &["merge-base", &base_head, &tip])?;
            if fork != tip {
                let mut read = git::ReadContext::from_snapshot(repo, tip, &[]);
                let ids = review_gate::scope(index, owner)?;
                let authors = review_gate::author_identities(&mut read, &fork, &ids)?;
                return Ok(Judge::Branch {
                    read,
                    fork,
                    base_head,
                    ids,
                    authors,
                });
            }
        }
        Ok(Judge::Base {
            read: git::ReadContext::new(repo),
            base,
        })
    }

    /// One approval or trivial exemption's standing: `✓ current` or
    /// `⚠ not current` on the work branch, `✓ accepted on <base>` or
    /// `⚠ not accepted on <base>` there.
    fn standing(&mut self, index: &RecordIndex, record: &ReviewRecord) -> Result<String> {
        Ok(match self {
            Judge::Branch {
                read,
                fork,
                ids,
                authors,
                ..
            } => {
                let identity = match record.event {
                    ReviewEvent::Approval => review_gate::independent(record, authors),
                    _ => true,
                };
                if identity && review_gate::current_on_branch(read, record, ids, fork) {
                    "✓ current".into()
                } else {
                    "⚠ not current".into()
                }
            }
            Judge::Base { read, base } => {
                if review_gate::approval_accepted(index, read, record)? {
                    format!("✓ accepted on {base}")
                } else {
                    format!("⚠ not accepted on {base}")
                }
            }
        })
    }
}

/// The approval or exemption a line names: `<id> @<pin> by <identity>`,
/// led by `trivial exemption` for an exemption.
fn approval_name(record: &ReviewRecord) -> String {
    format!(
        "{}{} @{} by {}",
        if record.event == ReviewEvent::TrivialExemption {
            "trivial exemption "
        } else {
            ""
        },
        record.header.id,
        short_commit(&record.commit),
        record.identity
    )
}

/// The owner's approval state, from the gates' own functions: on its work
/// branch `review_gate::approval_standing`, with the commits that made a
/// stale approval stale from `review_gate::approval_stale_edges`; on the
/// base, the newest approval `review_gate::approval_accepted` accepts.
fn approval_lines(repo: &Path, index: &RecordIndex, owner: &str) -> Result<Vec<String>> {
    const MISSING: &str = "Approval: ⚠ missing — no independent approval or trivial exemption";
    let mut judge = Judge::new(repo, index, owner)?;
    let Judge::Branch {
        read,
        fork,
        base_head,
        ids,
        ..
    } = &mut judge
    else {
        return base_approval_line(&mut judge, index, owner).map(|line| vec![line]);
    };
    let mut lines = Vec::new();
    match review_gate::approval_standing(index, read, owner, fork)? {
        (ApprovalState::Current, Some(record)) => {
            lines.push(format!("Approval: ✓ current — {}", approval_name(record)));
        }
        (ApprovalState::Stale, Some(record)) => {
            let edges =
                review_gate::approval_stale_edges(read, &record.commit, ids, "HEAD", base_head)?;
            if edges.is_empty() {
                lines.push(format!(
                    "Approval: ⚠ stale — {}; it predates the point where the branch left the base",
                    approval_name(record)
                ));
            } else {
                lines.push(format!(
                    "Approval: ⚠ stale — {}; changed since by {}:",
                    approval_name(record),
                    count_noun(edges.len(), "commit")
                ));
                for (commit, paths) in edges {
                    let subject = text(repo, &["log", "-1", "--format=%s", &commit])?;
                    lines.push(format!(
                        "  {} {subject}: {}",
                        short_commit(&commit),
                        listed_paths(&paths)
                    ));
                }
            }
        }
        _ => lines.push(MISSING.into()),
    }
    Ok(lines)
}

/// The approval line for work judged on the base: the newest approval or
/// exemption `land` accepts, or else the newest one it does not.
fn base_approval_line(judge: &mut Judge<'_>, index: &RecordIndex, owner: &str) -> Result<String> {
    let base = match judge {
        Judge::Base { base, .. } => base.clone(),
        Judge::Branch { .. } => unreachable!("only base judgement reaches here"),
    };
    let mut candidates = index
        .reviews()
        .filter(|record| {
            record.target == owner
                && matches!(
                    record.event,
                    ReviewEvent::Approval | ReviewEvent::TrivialExemption
                )
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|record| {
        std::cmp::Reverse((record.header.created_at, record.header.id.clone()))
    });
    for record in &candidates {
        if judge.standing(index, record)?.starts_with('✓') {
            return Ok(format!(
                "Approval: ✓ accepted on {base} — {}",
                approval_name(record)
            ));
        }
    }
    Ok(match candidates.first() {
        Some(newest) => format!(
            "Approval: ⚠ not accepted on {base} — newest {}",
            approval_name(newest)
        ),
        None => "Approval: ⚠ missing — no independent approval or trivial exemption".into(),
    })
}

/// Paths as a stale edge lists them: up to [`LISTED_PATHS`], then
/// `and <n> more`.
fn listed_paths(paths: &[std::path::PathBuf]) -> String {
    let mut listed = paths
        .iter()
        .take(LISTED_PATHS)
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    if paths.len() > LISTED_PATHS {
        listed.push(format!("and {} more", paths.len() - LISTED_PATHS));
    }
    listed.join(", ")
}

/// A finding's dispositions, oldest first, following each successor's
/// `supersedes` back from the latest.
fn disposition_chain<'i>(index: &'i RecordIndex, finding: &str) -> Vec<&'i ReviewRecord> {
    let mut chain = Vec::new();
    let mut next = index.latest_disposition(finding);
    while let Some(record) = next {
        chain.push(record);
        next = match &record.event {
            ReviewEvent::Disposition {
                supersedes: Some(previous),
                ..
            } => index.reviews().find(|record| &record.header.id == previous),
            _ => None,
        };
    }
    chain.reverse();
    chain
}

/// A disposition as a finding's chain shows it:
/// `<decision> <id>[, fixed at <commit>][, follow-up <id>]`.
fn disposition_step(record: &ReviewRecord) -> String {
    match &record.event {
        ReviewEvent::Disposition {
            decision,
            resolution_commit,
            follow_up,
            ..
        } => format!(
            "{} {}{}",
            decision.as_str(),
            record.header.id,
            disposition_extras(resolution_commit.as_deref(), follow_up.as_deref(), None)
        ),
        _ => record.header.id.clone(),
    }
}

/// The findings among `records`, open ones first, each with its
/// disposition chain: `⚠ <id> <recommendation>, no disposition  <summary>`
/// or `✓ <id> <recommendation> → <decision> <id> → ...  <summary>`.
fn finding_lines(index: &RecordIndex, records: &[&ReviewRecord]) -> Vec<String> {
    let mut rows = records
        .iter()
        .filter_map(|record| match &record.event {
            ReviewEvent::Finding { recommendation } => {
                let chain = disposition_chain(index, &record.header.id);
                let summary = first_line(&record.summary);
                Some(if chain.is_empty() {
                    (
                        false,
                        format!(
                            "  ⚠ {} {}, no disposition  {summary}",
                            record.header.id,
                            recommendation.as_str()
                        ),
                    )
                } else {
                    let steps = chain
                        .iter()
                        .map(|step| format!(" → {}", disposition_step(step)))
                        .collect::<String>();
                    (
                        true,
                        format!(
                            "  ✓ {} {}{steps}  {summary}",
                            record.header.id,
                            recommendation.as_str()
                        ),
                    )
                })
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    if rows.is_empty() {
        return Vec::new();
    }
    // Open findings first; the rest keep their order.
    rows.sort_by_key(|(disposed, _)| *disposed);
    let disposed = rows.iter().filter(|(disposed, _)| *disposed).count();
    let open = rows.len() - disposed;
    let counts = [(disposed, "disposed"), (open, "open")]
        .into_iter()
        .filter(|(count, _)| *count > 0)
        .map(|(count, word)| format!("{count} {word}"))
        .collect::<Vec<_>>()
        .join(", ");
    std::iter::once(format!("Findings: {counts}"))
        .chain(rows.into_iter().map(|(_, row)| row))
        .collect()
}

/// A brief view of one event: a header line, what it links to, then its
/// text once. Quiet output is `<id> <event>`, the event as the record
/// stores it.
pub fn show(id: &str, quiet: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let index = RecordIndex::load_structural(&layout.canonical_dir())?;
    let id = index.resolve(RecordKind::Review, id)?;
    if let Some(exemption) = index.base_exemption(&id) {
        if quiet {
            println!("{} trivial_exemption", exemption.header.id);
        } else {
            println!(
                "{} trivial_exemption of {} @{} by {} | {}\n\n{}",
                exemption.header.id,
                exemption.base,
                short_commit(&exemption.commit),
                exemption.identity,
                display_timestamp(exemption.header.created_at),
                exemption.summary.trim()
            );
        }
        return Ok(());
    }
    let record = index.require_review(&id)?;
    if quiet {
        println!("{} {}", record.header.id, record.event.name());
        return Ok(());
    }
    let mut header = format!(
        "{} {} on {} @{} by {} | {}",
        record.header.id,
        event_kind(&record.event),
        record.target,
        short_commit(&record.commit),
        record.identity,
        display_timestamp(record.header.created_at)
    );
    let mut links = Vec::new();
    match &record.event {
        ReviewEvent::Approval | ReviewEvent::TrivialExemption => {
            let owner = branch_owner(&index, &record.target);
            let mut judge = Judge::new(layout.repo_root(), &index, &owner)?;
            header.push_str(&format!(" | {}", judge.standing(&index, record)?));
        }
        ReviewEvent::Finding { .. } => {
            let chain = disposition_chain(&index, &record.header.id);
            if chain.is_empty() {
                links.push("Dispositions: ⚠ none".to_owned());
            } else {
                links.push("Dispositions:".to_owned());
                for step in chain {
                    links.push(format!(
                        "  → {} by {}: {}",
                        disposition_step(step),
                        step.identity,
                        first_line(&step.summary)
                    ));
                }
            }
        }
        ReviewEvent::Disposition {
            finding,
            resolution_commit,
            follow_up,
            supersedes,
            ..
        } => {
            if let Ok(original) = index.require_review(finding) {
                let recommendation = match &original.event {
                    ReviewEvent::Finding { recommendation } => recommendation.as_str(),
                    _ => "",
                };
                links.push(format!(
                    "Finding: {} {recommendation} by {}: {}",
                    original.header.id,
                    original.identity,
                    first_line(&original.summary)
                ));
            }
            if let Some(commit) = resolution_commit {
                links.push(format!("Fixed at: {}", short_commit(commit)));
            }
            if let Some(issue) = follow_up {
                links.push(format!("Follow-up: {issue}"));
            }
            if let Some(previous) = supersedes {
                links.push(format!("Supersedes: {previous}"));
            }
        }
        ReviewEvent::Review => {}
    }
    println!("{header}");
    for line in links {
        println!("{line}");
    }
    println!("\n{}", record.summary.trim());
    Ok(())
}
