//! Markdown review event codec and creation. Commands only add events, apart
//! from setting a finding's disposition in place (ADR 0025); a record can be
//! corrected by hand until it reaches the base, and `review_gate` keeps it as
//! it is there (ADR 0028).
use crate::{
    attachment_relationship, canonical_record_path, display_state_path,
    mutation_lock::CanonicalMutationLock, record_kinds::REVIEW_KIND, split_document,
    validate_record_id, RecordStore,
};
use anyhow::{bail, Context, Result};
use atelier_core::{
    BaseExemption, Disposition, FindingDecision, RecordHeader, Relationships, ReviewEvent,
    ReviewRecord,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// The first schema version, whose findings carried no disposition and whose
/// dispositions were separate `disposition` events.
const SEPARATE_DISPOSITIONS_VERSION: i64 = 1;

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
    event: serde_yaml::Value,
}

/// A separate disposition event, the shape schema version 1 recorded a
/// finding's disposition in. ADR 0025 retired it: Atelier no longer writes
/// one, reads those it finds as folded into their findings
/// ([`fold_retired_dispositions`]), and `atelier migrate dispositions`
/// writes that fold and removes them.
#[derive(Debug, Clone, PartialEq)]
pub struct RetiredDisposition {
    pub id: String,
    pub target: String,
    pub identity: String,
    pub commit: String,
    pub created_at: DateTime<Utc>,
    pub finding: String,
    pub decision: FindingDecision,
    pub supersedes: Option<String>,
    pub resolution_commit: Option<String>,
    pub follow_up: Option<String>,
    pub summary: String,
}

impl RetiredDisposition {
    /// The disposition field this event becomes on its finding.
    pub fn folded(&self) -> Disposition {
        Disposition {
            decision: self.decision,
            identity: self.identity.clone(),
            commit: self.commit.clone(),
            recorded_at: self.created_at,
            resolution_commit: self.resolution_commit.clone(),
            follow_up: self.follow_up.clone(),
            reason: self.summary.clone(),
            migrated_from: Some(self.id.clone()),
        }
    }
}

/// Sets `disposition` on the finding `record`, with what it derives: the
/// update time and the follow-up link. Returns the disposition it replaced.
pub fn set_finding_disposition(
    record: &mut ReviewRecord,
    disposition: Disposition,
) -> Result<Option<Disposition>> {
    let ReviewEvent::Finding {
        disposition: slot, ..
    } = &mut record.event
    else {
        bail!(
            "{} is not a finding; a disposition settles a finding",
            record.header.id
        );
    };
    let previous = slot.replace(disposition);
    record.header.updated_at = record
        .disposition()
        .map_or(record.header.created_at, |set| set.recorded_at);
    record.header.relationships = relationships(&record.target, &record.event);
    Ok(previous)
}

/// Reads separate disposition events as folded into their findings, as
/// `atelier migrate dispositions` would write them (ADR 0025): each finding
/// with events takes its latest one, the event no other supersedes. A
/// finding that already carries a disposition takes events only when they
/// continue from the one it was migrated from. Returns the IDs of the
/// findings that changed, and an error for each event that cannot be
/// folded, with the event's ID.
pub fn fold_retired_dispositions(
    findings: &mut [ReviewRecord],
    events: &[RetiredDisposition],
) -> (Vec<String>, Vec<(String, anyhow::Error)>) {
    let mut by_finding = std::collections::BTreeMap::<&str, Vec<&RetiredDisposition>>::new();
    for event in events {
        by_finding.entry(&event.finding).or_default().push(event);
    }
    let mut folded = Vec::new();
    let mut errors = Vec::new();
    for (finding, chain) in by_finding {
        let fail = |errors: &mut Vec<(String, anyhow::Error)>, message: String| {
            for event in &chain {
                errors.push((event.id.clone(), anyhow::anyhow!("{message}")));
            }
        };
        let Some(record) = findings
            .iter_mut()
            .find(|record| record.header.id == finding)
        else {
            fail(
                &mut errors,
                format!("disposition events of {finding} name a finding that does not exist"),
            );
            continue;
        };
        if !matches!(record.event, ReviewEvent::Finding { .. })
            || chain.iter().any(|event| event.target != record.target)
        {
            fail(
                &mut errors,
                format!(
                    "disposition events of {finding} must reference a finding for the same target"
                ),
            );
            continue;
        }
        let migrated = record
            .disposition()
            .and_then(|current| current.migrated_from.clone());
        if record.disposition().is_some()
            && !chain
                .iter()
                .any(|event| event.supersedes.is_some() && event.supersedes == migrated)
        {
            fail(
                &mut errors,
                format!("finding {finding} carries a disposition that its disposition events do not continue from; set the disposition it should have with `atelier review disposition`"),
            );
            continue;
        }
        let known = |id: &String| {
            chain.iter().any(|event| &event.id == id) || Some(id) == migrated.as_ref()
        };
        if let Some(event) = chain
            .iter()
            .find(|event| event.supersedes.as_ref().is_some_and(|id| !known(id)))
        {
            errors.push((
                event.id.clone(),
                anyhow::anyhow!(
                    "disposition {} supersedes {}, which is not a disposition of finding {finding}",
                    event.id,
                    event.supersedes.as_deref().unwrap_or_default()
                ),
            ));
            continue;
        }
        let latest = chain
            .iter()
            .filter(|event| {
                !chain
                    .iter()
                    .any(|other| other.supersedes.as_ref() == Some(&event.id))
            })
            .collect::<Vec<_>>();
        let roots = chain
            .iter()
            .filter(|event| event.supersedes.is_none())
            .count();
        let [latest] = latest.as_slice() else {
            fail(
                &mut errors,
                format!("finding {finding} has competing or cyclic disposition events"),
            );
            continue;
        };
        if roots > 1 || (migrated.is_none() && roots == 0) {
            fail(
                &mut errors,
                format!("finding {finding} has competing disposition events"),
            );
            continue;
        }
        let disposition = latest.folded();
        if let Err(error) = validate_disposition(&disposition)
            .and_then(|()| set_finding_disposition(record, disposition))
        {
            errors.push((latest.id.clone(), error));
            continue;
        }
        folded.push(finding.to_owned());
    }
    (folded, errors)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RetiredEvent {
    #[allow(dead_code)]
    event: String,
    finding: String,
    decision: FindingDecision,
    supersedes: Option<String>,
    resolution_commit: Option<String>,
    follow_up: Option<String>,
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
    if let Some(issue) = match event {
        ReviewEvent::Finding {
            disposition: Some(disposition),
            ..
        } => disposition.follow_up.as_ref(),
        _ => None,
    } {
        links
            .attachments
            .push(attachment_relationship("issue", issue, "follow_up"));
    }
    links
}

/// A file under `reviews/`: an event about an issue, a base exemption, or a
/// retired disposition event from before ADR 0025.
#[derive(Debug, Clone, PartialEq)]
pub enum ReviewFile {
    Event(ReviewRecord),
    BaseExemption(BaseExemption),
    RetiredDisposition(RetiredDisposition),
}

impl ReviewFile {
    /// The issue the file is about; `None` for a base exemption.
    pub fn target(&self) -> Option<&str> {
        match self {
            ReviewFile::Event(record) => Some(&record.target),
            ReviewFile::RetiredDisposition(record) => Some(&record.target),
            ReviewFile::BaseExemption(_) => None,
        }
    }
}

pub fn parse_review_file(text: &str, relative: &Path) -> Result<ReviewFile> {
    let (front, body) = split_document(text).context("review needs YAML front matter")?;
    // A newer Atelier's review can add fields, so its version is read before
    // the fields this one knows (atelier-u4ql).
    if let Some(found) = serde_yaml::from_str::<serde_yaml::Value>(front)
        .ok()
        .and_then(|value| value.get("schema_version")?.as_i64())
        .filter(|found| *found > REVIEW_KIND.schema_version)
    {
        return Err(atelier_core::NewerSchema {
            path: display_state_path(relative),
            found,
            supported: REVIEW_KIND.schema_version,
        }
        .into());
    }
    let front: Front = serde_yaml::from_str(front)
        .with_context(|| format!("invalid review {}", display_state_path(relative)))?;
    if front.schema != REVIEW_KIND.schema
        || !(SEPARATE_DISPOSITIONS_VERSION..=REVIEW_KIND.schema_version)
            .contains(&front.schema_version)
    {
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
    let separate = front.schema_version == SEPARATE_DISPOSITIONS_VERSION;
    let invalid = || format!("invalid review {}", display_state_path(relative));
    if front.event.get("event").and_then(serde_yaml::Value::as_str) == Some("disposition") {
        let (true, Some(target), None) = (separate, front.target, front.base) else {
            bail!(
                "{}: a disposition is a field of its finding since schema version 2, not an event",
                invalid()
            );
        };
        let event: RetiredEvent = serde_yaml::from_value(front.event).with_context(invalid)?;
        let record = RetiredDisposition {
            id: front.id,
            target,
            identity: front.identity,
            commit: front.commit,
            created_at: front.created_at,
            finding: event.finding,
            decision: event.decision,
            supersedes: event.supersedes,
            resolution_commit: event.resolution_commit,
            follow_up: event.follow_up,
            summary: body.trim().into(),
        };
        validate_retired_disposition(&record)?;
        return Ok(ReviewFile::RetiredDisposition(record));
    }
    let event: ReviewEvent = serde_yaml::from_value(front.event).with_context(invalid)?;
    if separate
        && matches!(
            event,
            ReviewEvent::Finding {
                disposition: Some(_),
                ..
            }
        )
    {
        bail!(
            "{}: a finding carries a disposition only from schema version 2",
            invalid()
        );
    }
    let header = |relationships, updated_at| RecordHeader {
        kind: "review".into(),
        id: front.id.clone(),
        title: front.title.clone(),
        status: "recorded".into(),
        labels: Vec::new(),
        relationships,
        created_at: front.created_at,
        updated_at,
    };
    match (front.target.clone(), front.base.clone()) {
        (Some(target), None) => {
            let updated_at = match &event {
                ReviewEvent::Finding {
                    disposition: Some(disposition),
                    ..
                } => disposition.recorded_at,
                _ => front.created_at,
            };
            let record = ReviewRecord {
                header: header(relationships(&target, &event), updated_at),
                target,
                identity: front.identity,
                commit: front.commit,
                event,
                summary: body.trim().into(),
            };
            validate_review(&record)?;
            Ok(ReviewFile::Event(record))
        }
        (None, Some(base)) => {
            if event != ReviewEvent::TrivialExemption {
                bail!(
                    "review {} names a base, which only a trivial exemption may",
                    display_state_path(relative)
                );
            }
            let record = BaseExemption {
                header: header(Relationships::default(), front.created_at),
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

fn validate_retired_disposition(record: &RetiredDisposition) -> Result<()> {
    for id in [&record.id, &record.target, &record.finding]
        .into_iter()
        .chain(&record.supersedes)
        .chain(&record.follow_up)
    {
        validate_record_id(id)?;
    }
    validate_commit(&record.commit)?;
    if let Some(commit) = &record.resolution_commit {
        validate_commit(commit)?;
    }
    Ok(())
}

/// Parses a review event about an issue; a base exemption or retired
/// disposition is refused.
pub fn parse_review_record(text: &str, relative: &Path) -> Result<ReviewRecord> {
    match parse_review_file(text, relative)? {
        ReviewFile::Event(record) => Ok(record),
        ReviewFile::BaseExemption(_) => bail!(
            "review {} is a base exemption, not an event about an issue",
            display_state_path(relative)
        ),
        ReviewFile::RetiredDisposition(_) => bail!(
            "review {} is a retired disposition event; run `atelier migrate dispositions --apply`",
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
    if let Some(disposition) = record.disposition() {
        validate_disposition(disposition)?;
    }
    Ok(())
}

pub fn validate_disposition(disposition: &Disposition) -> Result<()> {
    validate_commit(&disposition.commit)?;
    if disposition.identity.trim().is_empty() || disposition.reason.trim().is_empty() {
        bail!("a disposition requires an explicit identity and reason");
    }
    if let Some(commit) = &disposition.resolution_commit {
        validate_commit(commit)?;
    }
    if let Some(issue) = &disposition.follow_up {
        validate_record_id(issue)?;
    }
    if let Some(event) = &disposition.migrated_from {
        validate_record_id(event)?;
    }
    if disposition.decision == FindingDecision::Defer && disposition.follow_up.is_none() {
        bail!("DEFER requires a follow-up issue");
    }
    if disposition.decision != FindingDecision::FixNow && disposition.resolution_commit.is_some() {
        bail!("only FIX NOW can carry a resolution commit");
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
        event: serde_yaml::to_value(ReviewEvent::TrivialExemption)?,
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
        event: serde_yaml::to_value(&record.event)?,
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
    /// Creates a new event. Never replaces an existing path.
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

    /// Sets the disposition of finding `id`, replacing any earlier one,
    /// which stays in the Git history of the finding's file (ADR 0025).
    /// Returns the updated finding and the disposition it replaced.
    pub fn set_disposition(
        &self,
        id: &str,
        disposition: Disposition,
    ) -> Result<(ReviewRecord, Option<Disposition>)> {
        let _lock = CanonicalMutationLock::exclusive(&self.state_dir)?;
        let relative = canonical_record_path(&REVIEW_KIND, id)?;
        let mut record = parse_review_record(&self.read_record_file(&relative)?, &relative)?;
        let previous = set_finding_disposition(&mut record, disposition)?;
        self.write_atomic(&relative, render_review_record(&record)?)?;
        Ok((record, previous))
    }

    /// Removes a separate disposition event once its finding carries the
    /// disposition (ADR 0025), noting the path for `--commit-records`.
    pub fn remove_retired_disposition(&self, id: &str) -> Result<()> {
        let _lock = CanonicalMutationLock::exclusive(&self.state_dir)?;
        let relative = canonical_record_path(&REVIEW_KIND, id)?;
        let text = self.read_record_file(&relative)?;
        if !matches!(
            parse_review_file(&text, &relative)?,
            ReviewFile::RetiredDisposition(_)
        ) {
            bail!("{id} is not a separate disposition event");
        }
        self.remove(&relative)
    }

    fn publish_review(&self, id: &str, text: String) -> Result<()> {
        let relative = canonical_record_path(&REVIEW_KIND, id)?;
        // The shared writer publishes complete bytes atomically. Exclusive lock
        // and a pre-existing path check prevent cooperative record replacement.
        if self.exists(&relative) {
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

    /// A newer review is named as one, even with a field this version does
    /// not know; a lower version stays an unsupported schema (atelier-u4ql).
    #[test]
    fn a_newer_review_needs_a_newer_atelier() {
        let commit = "a".repeat(40);
        let text = |version: i64| {
            format!(
                "---\nschema: atelier.review\nschema_version: {version}\nid: atelier-r001\ntitle: why\ntarget: atelier-t001\nidentity: Author\ncommit: {commit}\ncreated_at: 2026-10-22T00:00:00Z\nfuture_field: yes\nevent:\n  event: approval\n---\n\nwhy\n"
            )
        };
        let path = Path::new("reviews/atelier-r001.md");
        let newer = parse_review_file(&text(3), path).unwrap_err();
        assert_eq!(
            newer.downcast_ref::<atelier_core::NewerSchema>(),
            Some(&atelier_core::NewerSchema {
                path: ".atelier/reviews/atelier-r001.md".into(),
                found: 3,
                supported: 2,
            })
        );
        let older = parse_review_file(&text(0), path).unwrap_err();
        assert!(!older.is::<atelier_core::NewerSchema>(), "{older:#}");
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
            ReviewFile::RetiredDisposition(_) => unreachable!("never rendered"),
        }
    }
}
