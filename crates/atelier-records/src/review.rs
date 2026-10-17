//! Append-only Markdown review event codec and creation.
use crate::{
    attachment_relationship, canonical_record_path, display_state_path,
    mutation_lock::CanonicalMutationLock, record_kinds::REVIEW_KIND, split_document,
    validate_record_id, RecordStore,
};
use anyhow::{bail, Context, Result};
use atelier_core::{RecordHeader, Relationships, ReviewEvent, ReviewRecord};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Front {
    schema: String,
    schema_version: i64,
    id: String,
    title: String,
    target: String,
    identity: String,
    commit: String,
    created_at: DateTime<Utc>,
    event: ReviewEvent,
}

pub fn validate_commit(commit: &str) -> Result<()> {
    if !matches!(commit.len(), 40 | 64) || !commit.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        bail!("review commit must be a full Git commit ID");
    }
    Ok(())
}

fn relationships(target: &str, event: &ReviewEvent) -> Relationships {
    let mut links = Relationships::default();
    links
        .attachments
        .push(attachment_relationship("issue", target, "reviews"));
    if let ReviewEvent::Disposition {
        finding,
        supersedes,
        follow_up,
        ..
    } = event
    {
        links
            .attachments
            .push(attachment_relationship("review", finding, "disposes"));
        if let Some(previous) = supersedes {
            links
                .attachments
                .push(attachment_relationship("review", previous, "supersedes"));
        }
        if let Some(issue) = follow_up {
            links
                .attachments
                .push(attachment_relationship("issue", issue, "follow_up"));
        }
    }
    links
}

pub fn parse_review_record(text: &str, relative: &Path) -> Result<ReviewRecord> {
    let (front, body) = split_document(text).context("review needs YAML front matter")?;
    let front: Front = serde_yaml::from_str(front)
        .with_context(|| format!("invalid review {}", display_state_path(relative)))?;
    if front.schema != REVIEW_KIND.schema || front.schema_version != REVIEW_KIND.schema_version {
        bail!(
            "unsupported review schema in {}",
            display_state_path(relative)
        );
    }
    if canonical_record_path(&REVIEW_KIND, &front.id)? != relative {
        bail!(
            "review id does not match file path {}",
            display_state_path(relative)
        );
    }
    let record = ReviewRecord {
        header: RecordHeader {
            kind: "review".into(),
            id: front.id,
            title: front.title,
            status: "recorded".into(),
            labels: Vec::new(),
            relationships: relationships(&front.target, &front.event),
            created_at: front.created_at,
            updated_at: front.created_at,
        },
        target: front.target,
        identity: front.identity,
        commit: front.commit,
        event: front.event,
        summary: body.trim().into(),
    };
    validate_review(&record)?;
    Ok(record)
}

pub fn validate_review(record: &ReviewRecord) -> Result<()> {
    validate_record_id(&record.header.id)?;
    validate_record_id(&record.target)?;
    validate_commit(&record.commit)?;
    if record.identity.trim().is_empty()
        || record.summary.trim().is_empty()
        || record.header.title.trim().is_empty()
    {
        bail!("review requires an explicit identity and explanation");
    }
    if record.header.kind != "review"
        || record.header.relationships != relationships(&record.target, &record.event)
    {
        bail!("review relationships must match its typed event references");
    }
    if let ReviewEvent::Disposition {
        finding,
        supersedes,
        resolution_commit,
        follow_up,
        decision,
    } = &record.event
    {
        validate_record_id(finding)?;
        if let Some(previous) = supersedes {
            validate_record_id(previous)?;
        }
        if let Some(commit) = resolution_commit {
            validate_commit(commit)?;
        }
        if let Some(issue) = follow_up {
            validate_record_id(issue)?;
        }
        if *decision == atelier_core::FindingDecision::Defer && follow_up.is_none() {
            bail!("DEFER requires a follow-up issue");
        }
        if *decision != atelier_core::FindingDecision::FixNow && resolution_commit.is_some() {
            bail!("only FIX NOW can carry a resolution commit");
        }
    }
    Ok(())
}

pub fn render_review_record(record: &ReviewRecord) -> Result<String> {
    validate_review(record)?;
    let front = Front {
        schema: REVIEW_KIND.schema.into(),
        schema_version: REVIEW_KIND.schema_version,
        id: record.header.id.clone(),
        title: record.header.title.clone(),
        target: record.target.clone(),
        identity: record.identity.clone(),
        commit: record.commit.clone(),
        created_at: record.header.created_at,
        event: record.event.clone(),
    };
    Ok(format!(
        "---\n{}---\n\n{}\n",
        serde_yaml::to_string(&front)?,
        record.summary.trim()
    ))
}

impl RecordStore {
    /// Creates a new immutable event. Never replaces an existing path.
    pub fn create_review(
        &self,
        target: &str,
        identity: &str,
        commit: &str,
        event: ReviewEvent,
        summary: &str,
    ) -> Result<ReviewRecord> {
        let _lock = CanonicalMutationLock::exclusive(&self.state_dir)?;
        let now = Utc::now();
        let record = ReviewRecord {
            header: RecordHeader {
                kind: "review".into(),
                id: self.allocate_record_id()?,
                title: summary.lines().next().unwrap_or("").into(),
                status: "recorded".into(),
                labels: Vec::new(),
                relationships: relationships(target, &event),
                created_at: now,
                updated_at: now,
            },
            target: target.into(),
            identity: identity.into(),
            commit: commit.into(),
            event,
            summary: summary.into(),
        };
        let relative = canonical_record_path(&REVIEW_KIND, &record.header.id)?;
        let text = render_review_record(&record)?;
        let directory = self.state_dir.join("reviews");
        fs::create_dir_all(&directory)?;
        // The shared writer publishes complete bytes atomically. Exclusive lock
        // and a pre-existing path check prevent cooperative record replacement.
        if self.state_dir.join(&relative).symlink_metadata().is_ok() {
            bail!("review already exists: {}", relative.display());
        }
        self.write_atomic(&relative, text)?;
        Ok(record)
    }
}
