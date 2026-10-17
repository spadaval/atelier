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

/// Quiet output is the event IDs, one per line.
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
    if json {
        println!("{}", serde_json::to_string_pretty(&records)?);
    } else if quiet {
        for record in records {
            println!("{}", record.header.id);
        }
    } else {
        for record in records {
            println!(
                "{} {:?} {} {} {}",
                record.header.id, record.event, record.target, record.identity, record.summary
            );
        }
    }
    Ok(())
}

/// Quiet output is `<id> <event>`, the event as the record stores it.
pub fn show(id: &str, quiet: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let index = RecordIndex::load_structural(&layout.canonical_dir())?;
    let record = index.require_review(&index.resolve(RecordKind::Review, id)?)?;
    if quiet {
        println!("{} {}", record.header.id, event_name(&record.event));
        return Ok(());
    }
    println!("{}", serde_json::to_string_pretty(record)?);
    Ok(())
}

/// The event's stored name, as in the record's `event` field.
fn event_name(event: &ReviewEvent) -> &'static str {
    match event {
        ReviewEvent::Review => "review",
        ReviewEvent::Approval => "approval",
        ReviewEvent::Finding { .. } => "finding",
        ReviewEvent::Disposition { .. } => "disposition",
        ReviewEvent::TrivialExemption => "trivial_exemption",
    }
}
