//! Creation and inspection of immutable, self-declared review events.
use anyhow::{bail, Result};
use atelier_app::record_ref::RecordKind;
use atelier_app::{git_freshness as git, record_index::RecordIndex, storage_layout::StorageLayout};
use atelier_core::{FindingDecision, ReviewEvent};

pub fn append(
    owner_ref: &str,
    identity: &str,
    commit: Option<&str>,
    summary: &str,
    event: ReviewEvent,
    quiet: bool,
) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let index = RecordIndex::load_structural(&layout.canonical_dir())?;
    let owner = index.resolve(RecordKind::Issue, owner_ref)?;
    let pin = commit
        .map(str::to_owned)
        .unwrap_or(git::head_commit(layout.repo_root())?);
    atelier_records::review::validate_commit(&pin)?;
    git::git_output(
        layout.repo_root(),
        &["merge-base", "--is-ancestor", &pin, "HEAD"],
    )?;
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
            git::git_output(
                layout.repo_root(),
                &["merge-base", "--is-ancestor", commit, "HEAD"],
            )?;
            git::git_output(
                layout.repo_root(),
                &["merge-base", "--is-ancestor", &record.commit, commit],
            )?;
            if commit == &record.commit {
                bail!("resolution must follow the finding's reviewed commit");
            }
        }
        if let Some(id) = follow_up {
            index.require_issue(id)?;
        }
    }
    let created = atelier_app::project_config::record_store(&layout.canonical_dir())?
        .create_review(&owner, identity, &pin, event, summary)?;
    if quiet {
        println!("{}", created.header.id);
    } else {
        println!("Review event: {}\nIdentity: {}\nCommit: {}\n{}\nCommit this record with the work it describes.", created.header.id, created.identity, created.commit, created.summary);
    }
    Ok(())
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
    let pin = crate::commands::git::resolve(repo, commit)
        .map_err(|_| anyhow::anyhow!("review_base_commit: {commit} is not a commit"))?;
    if git::git_output(repo, &["merge-base", "--is-ancestor", &pin, "HEAD"]).is_err() {
        bail!("review_base_commit: {commit} is not on {base}; a base exemption covers a commit the base already holds");
    }
    let created = atelier_app::project_config::record_store(&layout.canonical_dir())?
        .create_base_exemption(&base, identity, &pin, summary)?;
    if quiet {
        println!("{}", created.header.id);
    } else {
        println!("Base exemption: {}\nIdentity: {}\nCommit: {} on {}\n{}\nIt covers this commit alone. Commit this record on {}.", created.header.id, created.identity, created.commit, created.base, created.summary, created.base);
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
    let layout = StorageLayout::discover()?;
    let index = RecordIndex::load_structural(&layout.canonical_dir())?;
    let finding = index.resolve(RecordKind::Review, finding)?;
    let original = index.require_review(&finding)?;
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

/// Quiet output is the event IDs, one per line. Without an owner, base
/// exemptions are listed too, after the events about issues.
pub fn list(owner: Option<&str>, json: bool, quiet: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let index = RecordIndex::load_structural(&layout.canonical_dir())?;
    let owner = owner
        .map(|id| index.resolve(RecordKind::Issue, id))
        .transpose()?;
    let records = index
        .reviews()
        .filter(|record| owner.as_ref().is_none_or(|id| &record.target == id))
        .collect::<Vec<_>>();
    let exemptions = index
        .base_exemptions()
        .filter(|_| owner.is_none())
        .collect::<Vec<_>>();
    if json {
        let mut values = records
            .iter()
            .map(serde_json::to_value)
            .collect::<serde_json::Result<Vec<_>>>()?;
        for exemption in &exemptions {
            values.push(serde_json::to_value(exemption)?);
        }
        println!("{}", serde_json::to_string_pretty(&values)?);
    } else if quiet {
        for record in records {
            println!("{}", record.header.id);
        }
        for exemption in exemptions {
            println!("{}", exemption.header.id);
        }
    } else {
        for record in records {
            println!(
                "{} {:?} {} {} {}",
                record.header.id, record.event, record.target, record.identity, record.summary
            );
        }
        for exemption in exemptions {
            println!(
                "{} TrivialExemption base {} {} {} {}",
                exemption.header.id,
                exemption.base,
                &exemption.commit[..8.min(exemption.commit.len())],
                exemption.identity,
                exemption.summary
            );
        }
    }
    Ok(())
}

/// Quiet output is `<id> <event>`, the event as the record stores it.
pub fn show(id: &str, quiet: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let index = RecordIndex::load_structural(&layout.canonical_dir())?;
    let id = index.resolve(RecordKind::Review, id)?;
    if let Some(exemption) = index.base_exemption(&id) {
        if quiet {
            println!("{} trivial_exemption", exemption.header.id);
        } else {
            println!("{}", serde_json::to_string_pretty(exemption)?);
        }
        return Ok(());
    }
    let record = index.require_review(&id)?;
    if quiet {
        println!("{} {}", record.header.id, event_name(&record.event));
        return Ok(());
    }
    println!("{}", serde_json::to_string_pretty(record)?);
    Ok(())
}

/// The event's stored name, as in the record's `event` field.
pub fn event_name(event: &ReviewEvent) -> &'static str {
    match event {
        ReviewEvent::Review => "review",
        ReviewEvent::Approval => "approval",
        ReviewEvent::Finding { .. } => "finding",
        ReviewEvent::Disposition { .. } => "disposition",
        ReviewEvent::TrivialExemption => "trivial_exemption",
    }
}
