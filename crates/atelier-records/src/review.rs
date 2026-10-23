//! Append-only Markdown review event codec and creation.
use crate::{
    attachment_relationship, canonical_record_path, display_state_path,
    mutation_lock::CanonicalMutationLock, record_kinds::REVIEW_KIND, split_document,
    validate_record_id, RecordStore,
};
use anyhow::{bail, Context, Result};
use atelier_core::{BaseExemption, RecordHeader, Relationships, ReviewEvent, ReviewRecord};
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
    /// The issue an event is about; absent for a base exemption.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    /// The base branch of a base exemption, which has no target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base: Option<String>,
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

/// A file under `reviews/`: an event about an issue, or a base exemption.
#[derive(Debug, Clone, PartialEq)]
pub enum ReviewFile {
    Event(ReviewRecord),
    BaseExemption(BaseExemption),
}

pub fn parse_review_file(text: &str, relative: &Path) -> Result<ReviewFile> {
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
    let header = |relationships| RecordHeader {
        kind: "review".into(),
        id: front.id.clone(),
        title: front.title.clone(),
        status: "recorded".into(),
        labels: Vec::new(),
        relationships,
        created_at: front.created_at,
        updated_at: front.created_at,
    };
    match (front.target.clone(), front.base.clone()) {
        (Some(target), None) => {
            let record = ReviewRecord {
                header: header(relationships(&target, &front.event)),
                target,
                identity: front.identity,
                commit: front.commit,
                event: front.event,
                summary: body.trim().into(),
            };
            validate_review(&record)?;
            Ok(ReviewFile::Event(record))
        }
        (None, Some(base)) => {
            if front.event != ReviewEvent::TrivialExemption {
                bail!(
                    "review {} names a base, which only a trivial exemption may",
                    display_state_path(relative)
                );
            }
            let record = BaseExemption {
                header: header(Relationships::default()),
                base,
                identity: front.identity,
                commit: front.commit,
                summary: body.trim().into(),
            };
            validate_base_exemption(&record)?;
            Ok(ReviewFile::BaseExemption(record))
        }
        _ => bail!(
            "review {} needs exactly one of target and base",
            display_state_path(relative)
        ),
    }
}

/// Parses a review event about an issue; a base exemption is refused.
pub fn parse_review_record(text: &str, relative: &Path) -> Result<ReviewRecord> {
    match parse_review_file(text, relative)? {
        ReviewFile::Event(record) => Ok(record),
        ReviewFile::BaseExemption(_) => bail!(
            "review {} is a base exemption, not an event about an issue",
            display_state_path(relative)
        ),
    }
}

pub fn validate_base_exemption(record: &BaseExemption) -> Result<()> {
    validate_record_id(&record.header.id)?;
    validate_commit(&record.commit)?;
    if record.base.trim().is_empty()
        || record.identity.trim().is_empty()
        || record.summary.trim().is_empty()
        || record.header.title.trim().is_empty()
    {
        bail!("base exemption requires a base, an explicit identity and an explanation");
    }
    if record.header.kind != "review" || record.header.relationships != Relationships::default() {
        bail!("base exemption has no relationships");
    }
    Ok(())
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

pub fn render_base_exemption(record: &BaseExemption) -> Result<String> {
    validate_base_exemption(record)?;
    let front = Front {
        schema: REVIEW_KIND.schema.into(),
        schema_version: REVIEW_KIND.schema_version,
        id: record.header.id.clone(),
        title: record.header.title.clone(),
        target: None,
        base: Some(record.base.clone()),
        identity: record.identity.clone(),
        commit: record.commit.clone(),
        created_at: record.header.created_at,
        event: ReviewEvent::TrivialExemption,
    };
    Ok(format!(
        "---\n{}---\n\n{}\n",
        serde_yaml::to_string(&front)?,
        record.summary.trim()
    ))
}

pub fn render_review_record(record: &ReviewRecord) -> Result<String> {
    validate_review(record)?;
    let front = Front {
        schema: REVIEW_KIND.schema.into(),
        schema_version: REVIEW_KIND.schema_version,
        id: record.header.id.clone(),
        title: record.header.title.clone(),
        target: Some(record.target.clone()),
        base: None,
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

/// A new record's title, built from its kind, subject and short pin, such
/// as `finding on atelier-nqgr @fed2d571`, so the body is stored once
/// (atelier-hulj). Records written before keep the summary's first line.
fn generated_title(event: &str, relation: &str, subject: &str, commit: &str) -> String {
    format!(
        "{event} {relation} {subject} @{}",
        commit.get(..8).unwrap_or(commit)
    )
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
                title: generated_title(event.name(), "on", target, commit),
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
        self.publish_review(&record.header.id, render_review_record(&record)?)?;
        Ok(record)
    }

    /// Creates a trivial exemption of `commit` on the base branch `base`,
    /// with no owner issue (atelier-mbrf).
    pub fn create_base_exemption(
        &self,
        base: &str,
        identity: &str,
        commit: &str,
        summary: &str,
    ) -> Result<BaseExemption> {
        let _lock = CanonicalMutationLock::exclusive(&self.state_dir)?;
        let now = Utc::now();
        let record = BaseExemption {
            header: RecordHeader {
                kind: "review".into(),
                id: self.allocate_record_id()?,
                title: generated_title(ReviewEvent::TrivialExemption.name(), "of", base, commit),
                status: "recorded".into(),
                labels: Vec::new(),
                relationships: Relationships::default(),
                created_at: now,
                updated_at: now,
            },
            base: base.into(),
            identity: identity.into(),
            commit: commit.into(),
            summary: summary.into(),
        };
        self.publish_review(&record.header.id, render_base_exemption(&record)?)?;
        Ok(record)
    }

    fn publish_review(&self, id: &str, text: String) -> Result<()> {
        let relative = canonical_record_path(&REVIEW_KIND, id)?;
        fs::create_dir_all(self.state_dir.join("reviews"))?;
        // The shared writer publishes complete bytes atomically. Exclusive lock
        // and a pre-existing path check prevent cooperative record replacement.
        if self.state_dir.join(&relative).symlink_metadata().is_ok() {
            bail!("review already exists: {}", relative.display());
        }
        self.write_atomic(&relative, text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(subject: &str, event: &str) -> Result<ReviewFile> {
        let commit = "a".repeat(40);
        let text = format!(
            "---\nschema: atelier.review\nschema_version: 1\nid: atelier-r001\ntitle: why\n{subject}identity: Author\ncommit: {commit}\ncreated_at: 2026-10-22T00:00:00Z\nevent:\n  event: {event}\n---\n\nwhy\n"
        );
        parse_review_file(&text, Path::new("reviews/atelier-r001.md"))
    }

    #[test]
    fn a_base_exemption_names_a_base_instead_of_a_target_and_round_trips() {
        let Ok(ReviewFile::BaseExemption(record)) = file("base: main\n", "trivial_exemption")
        else {
            panic!("a base exemption parses");
        };
        assert_eq!(record.base, "main");
        let text = render_base_exemption(&record).unwrap();
        assert!(!text.contains("target:"), "{text}");
        assert_eq!(
            parse_review_file(&text, Path::new("reviews/atelier-r001.md")).unwrap(),
            ReviewFile::BaseExemption(record)
        );
        assert!(matches!(
            file("target: atelier-i001\n", "trivial_exemption"),
            Ok(ReviewFile::Event(_))
        ));
        for (subject, event) in [
            ("base: main\n", "approval"),
            ("target: atelier-i001\nbase: main\n", "trivial_exemption"),
            ("", "trivial_exemption"),
        ] {
            assert!(file(subject, event).is_err(), "{subject}{event}");
        }
        let error = parse_review_record(
            &text_of(file("base: main\n", "trivial_exemption").unwrap()),
            Path::new("reviews/atelier-r001.md"),
        )
        .unwrap_err();
        assert!(error.to_string().contains("is a base exemption"), "{error}");
    }

    fn text_of(file: ReviewFile) -> String {
        match file {
            ReviewFile::BaseExemption(record) => render_base_exemption(&record).unwrap(),
            ReviewFile::Event(record) => render_review_record(&record).unwrap(),
        }
    }
}
