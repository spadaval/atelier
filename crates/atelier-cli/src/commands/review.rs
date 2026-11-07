//! Creation and inspection of self-declared review events, and of the
//! dispositions findings carry (ADR 0025).
use crate::commands::git::{resolve, text};
use crate::human_output::{count_noun, display_timestamp, local_date, short_commit};
use anyhow::{bail, Result};
use atelier_app::record_ref::RecordKind;
use atelier_app::review_gate::{self, ApprovalState, NotCurrent};
use atelier_app::{git_freshness as git, record_index::RecordIndex, storage_layout::StorageLayout};
use atelier_core::{BaseExemption, Disposition, FindingDecision, ReviewEvent, ReviewRecord};
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
    // Read before the record exists, so a failure here writes nothing.
    let authors = authors_at_head(repo, &index, &owner)?;
    let created = atelier_app::project_config::record_store(&layout.canonical_dir())?
        .create_review(&owner, identity, &pin, event, summary)?;
    let warning = (created.event == ReviewEvent::Approval
        && !review_gate::independent(&created, &authors))
    .then(|| author_warning("review.current"));
    confirm(&created.header.id, &confirmation(&created), warning, quiet);
    Ok(())
}

/// Prints a recording command's one-line confirmation and any warning;
/// quiet output is `id`, with the warning on stderr.
fn confirm(id: &str, line: &str, warning: Option<String>, quiet: bool) {
    if quiet {
        println!("{id}");
        if let Some(warning) = warning {
            eprintln!("{warning}");
        }
        return;
    }
    println!("{line}");
    if let Some(warning) = warning {
        println!("{warning}");
    }
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

/// The warning for a record the gates will refuse because its identity is
/// one of the work's authors: an approval (`review.current`), or a defer or
/// no_action disposition (`findings.disposed`).
fn author_warning(gate: &str) -> String {
    format!("⚠ identity matches a Git author; this will not satisfy {gate}")
}

/// One line saying what was recorded: the kind, ID, owner and short pin,
/// or the step a finding needs next.
fn confirmation(record: &ReviewRecord) -> String {
    let id = &record.header.id;
    let on = format!("on {} @{}", record.target, short_commit(&record.commit));
    match &record.event {
        ReviewEvent::Review => format!("Review account {id} {on}"),
        ReviewEvent::Approval => format!("Approval {id} {on}"),
        ReviewEvent::TrivialExemption => format!("Trivial exemption {id} {on}"),
        ReviewEvent::Finding { recommendation, .. } => {
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
    }
}

/// `, fixed at <commit>` and `, follow-up <id>`, as far as a disposition
/// has them.
fn disposition_extras(disposition: &Disposition) -> String {
    let mut extras = String::new();
    if let Some(commit) = &disposition.resolution_commit {
        extras.push_str(&format!(", fixed at {}", short_commit(commit)));
    }
    if let Some(issue) = &disposition.follow_up {
        extras.push_str(&format!(", follow-up {issue}"));
    }
    extras
}

/// A disposition as a finding's line shows it:
/// `<decision> by <identity> @<pin>[, fixed at <commit>][, follow-up <id>]`.
fn disposition_step(disposition: &Disposition) -> String {
    format!(
        "{} by {} @{}{}",
        disposition.decision.as_str(),
        disposition.identity,
        short_commit(&disposition.commit),
        disposition_extras(disposition)
    )
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

/// Sets a finding's disposition, pinned at HEAD, replacing any earlier one;
/// Git keeps the earlier ones in the finding file's history (ADR 0025).
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
    let repo = layout.repo_root();
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
    let pin = git::head_commit(repo)?;
    if let Some(commit) = &resolution {
        atelier_records::review::validate_commit(commit)?;
        require_on_branch(repo, commit)?;
        if commit == &original.commit
            || git::git_output(
                repo,
                &["merge-base", "--is-ancestor", &original.commit, commit],
            )
            .is_err()
        {
            bail!(
                "resolution commit {commit} does not follow {}, the commit finding {} reviewed. Next: git log --format=%H {}..HEAD",
                short_commit(&original.commit),
                original.header.id,
                original.commit
            );
        }
    }
    let follow_up = follow_up
        .map(|id| index.resolve(RecordKind::Issue, &id))
        .transpose()?;
    let disposition = Disposition {
        decision,
        identity: identity.to_owned(),
        commit: pin,
        recorded_at: chrono::Utc::now(),
        resolution_commit: resolution,
        follow_up,
        reason: summary.to_owned(),
        migrated_from: None,
    };
    atelier_records::review::validate_disposition(&disposition)?;
    // Read before the record changes, so a failure here writes nothing.
    let authors = authors_at_head(repo, &index, &original.target)?;
    // The disposition an older Atelier recorded as separate events is read
    // as folded into the finding (ADR 0025); writing the field removes them.
    let previous = original.disposition().cloned();
    let store = atelier_app::project_config::record_store(&layout.canonical_dir())?;
    let (updated, _) = store.set_disposition(&finding, disposition.clone())?;
    for event in index
        .retired_dispositions()
        .filter(|event| event.finding == finding)
    {
        store.remove_retired_disposition(&event.id)?;
    }
    let mut line = format!(
        "Disposition {} of {} on {} @{}: \"{}\"{}",
        decision.as_str(),
        updated.header.id,
        updated.target,
        short_commit(&disposition.commit),
        first_line(&updated.summary),
        disposition_extras(&disposition)
    );
    if let Some(previous) = previous {
        line.push_str(&format!(
            ", replacing {} by {}",
            previous.decision.as_str(),
            previous.identity
        ));
    }
    let warning = (decision != FindingDecision::FixNow
        && !review_gate::identity_independent(identity, &authors))
    .then(|| author_warning("findings.disposed"));
    confirm(&updated.header.id, &line, warning, quiet);
    Ok(())
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
    for line in finding_lines(&records) {
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

/// What an event is, as a row names it: its stored kind, with a finding's
/// recommendation.
fn event_kind(event: &ReviewEvent) -> String {
    match event {
        ReviewEvent::Finding { recommendation, .. } => {
            format!("finding {}", recommendation.as_str())
        }
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
/// branch `review_gate::approval_standing`, which `review.current` also
/// explains, with the commits that made a stale approval stale or the
/// author an approval's identity matches; on the base, the newest approval
/// `review_gate::approval_accepted` accepts.
fn approval_lines(repo: &Path, index: &RecordIndex, owner: &str) -> Result<Vec<String>> {
    const MISSING: &str = "Approval: ⚠ missing — no independent approval or trivial exemption";
    let mut judge = Judge::new(repo, index, owner)?;
    let Judge::Branch { read, fork, .. } = &mut judge else {
        return base_approval_line(&mut judge, index, owner).map(|line| vec![line]);
    };
    let mut lines = Vec::new();
    let standing = review_gate::approval_standing(index, read, owner, fork)?;
    match (standing.state, standing.record, &standing.not_current) {
        (ApprovalState::Current, Some(record), _) => {
            lines.push(format!("Approval: ✓ current — {}", approval_name(record)));
        }
        (ApprovalState::Stale, Some(record), Some(NotCurrent::Stale(edges))) => {
            lines.push(format!(
                "Approval: ⚠ stale — {}; changed since by {}:",
                approval_name(record),
                count_noun(edges.len(), "commit")
            ));
            for edge in edges {
                lines.push(format!(
                    "  {} {}: {}",
                    short_commit(&edge.commit),
                    edge.subject,
                    listed_paths(&edge.paths)
                ));
            }
        }
        (ApprovalState::Stale, Some(record), _) => {
            lines.push(format!(
                "Approval: ⚠ stale — {}; it predates the point where the branch left the base",
                approval_name(record)
            ));
        }
        (_, Some(record), Some(NotCurrent::Author)) => lines.push(format!(
            "Approval: ⚠ missing — {} is not independent: the identity matches a Git author of the branch",
            approval_name(record)
        )),
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

/// The findings among `records`, open ones first, each with its
/// disposition: `⚠ <id> <recommendation>, no disposition  <summary>` or
/// `✓ <id> <recommendation> → <decision> by <identity> @<pin>...  <summary>`.
fn finding_lines(records: &[&ReviewRecord]) -> Vec<String> {
    let mut rows = records
        .iter()
        .filter_map(|record| match &record.event {
            ReviewEvent::Finding {
                recommendation,
                disposition,
            } => {
                let summary = first_line(&record.summary);
                let id = &record.header.id;
                let recommendation = recommendation.as_str();
                Some(match disposition {
                    None => (
                        false,
                        format!("  ⚠ {id} {recommendation}, no disposition  {summary}"),
                    ),
                    Some(disposition) => (
                        true,
                        format!(
                            "  ✓ {id} {recommendation} → {}  {summary}",
                            disposition_step(disposition)
                        ),
                    ),
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
        ReviewEvent::Finding {
            disposition: None, ..
        } => links.push("Disposition: ⚠ none".to_owned()),
        ReviewEvent::Finding {
            disposition: Some(disposition),
            ..
        } => {
            links.push(format!(
                "Disposition: {} | {}: {}",
                disposition_step(disposition),
                display_timestamp(disposition.recorded_at),
                first_line(&disposition.reason)
            ));
            if let Some(event) = &disposition.migrated_from {
                links.push(format!("Migrated from: {event}"));
            }
            links.push(format!(
                "Earlier dispositions: git log -p -- .atelier/reviews/{}.md",
                record.header.id
            ));
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
