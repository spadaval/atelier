//! Markdown review event codec and creation. Commands only add events, apart
//! from setting a finding's disposition in place (ADR 0025); a record can be
//! corrected by hand until it reaches the base, and `review_gate` keeps it as
//! it is there (ADR 0028). An event is about an issue, or, on the light path,
//! about a branch no issue owns (ADR 0028 decision 6, atelier-4nsl).
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

/// The schema version an event about a branch is written at, the first that
/// can name a branch (atelier-4nsl). An older Atelier reads it as a record a
/// newer Atelier wrote, not as an invalid one. Events about an issue and base
/// exemptions are still written at `REVIEW_KIND.schema_version`, so a binary
/// that predates the light path keeps reading every ordinary review.
pub const BRANCH_TARGET_VERSION: i64 = 3;

/// The schema version a finding that recommends or decides `wont_fix` is
/// written at, about an issue or a branch, the first that can hold it
/// (atelier-stnv, atelier-xlxn). An older Atelier reads it as a record a
/// newer Atelier wrote, not as an invalid one; every other record keeps the
/// version it had, so an older binary keeps reading it.
pub const WONT_FIX_VERSION: i64 = 4;

/// The newest schema version this Atelier reads.
const SUPPORTED_VERSION: i64 = WONT_FIX_VERSION;

/// Whether an event recommends or decides `wont_fix`.
fn holds_wont_fix(event: &ReviewEvent) -> bool {
    let ReviewEvent::Finding {
        recommendation,
        disposition,
    } = event
    else {
        return false;
    };
    *recommendation == FindingDecision::WontFix
        || disposition
            .as_ref()
            .is_some_and(|disposition| disposition.decision == FindingDecision::WontFix)
}

/// The version an event is written at: `subject_version`, or
/// [`WONT_FIX_VERSION`] when it holds `wont_fix`.
fn written_version(event: &ReviewEvent, subject_version: i64) -> i64 {
    if holds_wont_fix(event) {
        WONT_FIX_VERSION
    } else {
        subject_version
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Front {
    schema: String,
    schema_version: i64,
    id: String,
    title: String,
    /// The issue an event is about; absent for a base exemption and for an
    /// event about a branch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    /// The base branch of a base exemption, which has no target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base: Option<String>,
    /// The branch an event on the light path is about, which no issue owns;
    /// from schema version 3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    branch: Option<String>,
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
    // An event about an issue links it as the issue it reviews; one about a
    // branch links none, and keeps linking none.
    let issue_target = record
        .header
        .relationships
        .attachments
        .iter()
        .any(|link| link.kind == "issue" && link.role == "reviews");
    record.header.relationships = relationships(
        issue_target.then_some(record.target.as_str()),
        &record.event,
    );
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

/// The links an event derives: the issue it is about, which an event about a
/// branch has none of, and a deferred finding's follow-up.
fn relationships(issue: Option<&str>, event: &ReviewEvent) -> Relationships {
    let mut links = Relationships::default();
    if let Some(target) = issue {
        links
            .attachments
            .push(attachment_relationship("issue", target, "reviews"));
    }
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

/// A file under `reviews/`: an event about an issue, an event about a
/// branch no issue owns (the light path), a base exemption, or a retired
/// disposition event from before ADR 0025.
#[derive(Debug, Clone, PartialEq)]
pub enum ReviewFile {
    Event(ReviewRecord),
    /// An event whose `target` is a branch name, not an issue
    /// (atelier-4nsl). It never counts for an issue.
    BranchEvent(ReviewRecord),
    BaseExemption(BaseExemption),
    RetiredDisposition(RetiredDisposition),
}

impl ReviewFile {
    /// The issue the file is about; `None` for a base exemption and an
    /// event about a branch.
    pub fn target(&self) -> Option<&str> {
        match self {
            ReviewFile::Event(record) => Some(&record.target),
            ReviewFile::RetiredDisposition(record) => Some(&record.target),
            ReviewFile::BaseExemption(_) | ReviewFile::BranchEvent(_) => None,
        }
    }
}

pub fn parse_review_file(text: &str, relative: &Path) -> Result<ReviewFile> {
    parse_review_file_supporting(text, relative, SUPPORTED_VERSION)
}

/// [`parse_review_file`] by a reader whose newest schema version is
/// `supported`, as an older Atelier reads a newer record.
fn parse_review_file_supporting(text: &str, relative: &Path, supported: i64) -> Result<ReviewFile> {
    let (front, body) = split_document(text).context("review needs YAML front matter")?;
    // A newer Atelier's review can add fields, so its version is read before
    // the fields this one knows (atelier-u4ql).
    if let Some(found) = serde_yaml::from_str::<serde_yaml::Value>(front)
        .ok()
        .and_then(|value| value.get("schema_version")?.as_i64())
        .filter(|found| *found > supported)
    {
        return Err(atelier_core::NewerSchema {
            path: display_state_path(relative),
            found,
            supported,
        }
        .into());
    }
    let front: Front = serde_yaml::from_str(front)
        .with_context(|| format!("invalid review {}", display_state_path(relative)))?;
    if front.schema != REVIEW_KIND.schema
        || !(SEPARATE_DISPOSITIONS_VERSION..=supported).contains(&front.schema_version)
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
        let (true, Some(target), None, None) = (separate, front.target, front.base, front.branch)
        else {
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
    if holds_wont_fix(&event) && front.schema_version < WONT_FIX_VERSION {
        bail!(
            "{}: a finding holds wont_fix only from schema version {WONT_FIX_VERSION}",
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
    let updated_at = match &event {
        ReviewEvent::Finding {
            disposition: Some(disposition),
            ..
        } => disposition.recorded_at,
        _ => front.created_at,
    };
    match (
        front.target.clone(),
        front.base.clone(),
        front.branch.clone(),
    ) {
        (Some(target), None, None) => {
            let record = ReviewRecord {
                header: header(relationships(Some(&target), &event), updated_at),
                target,
                identity: front.identity,
                commit: front.commit,
                event,
                summary: body.trim().into(),
            };
            validate_review(&record)?;
            Ok(ReviewFile::Event(record))
        }
        (None, None, Some(branch)) => {
            if front.schema_version < BRANCH_TARGET_VERSION {
                bail!(
                    "{}: an event names a branch only from schema version {BRANCH_TARGET_VERSION}",
                    invalid()
                );
            }
            let record = ReviewRecord {
                header: header(relationships(None, &event), updated_at),
                target: branch,
                identity: front.identity,
                commit: front.commit,
                event,
                summary: body.trim().into(),
            };
            validate_branch_review(&record)?;
            Ok(ReviewFile::BranchEvent(record))
        }
        (None, Some(base), None) => {
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
            "review {} needs exactly one of target, base and branch",
            display_state_path(relative)
        ),
    }
}

/// Parses an event about an issue or about a branch; a base exemption or
/// retired disposition is refused. A finding's disposition is set the same
/// way on either.
pub fn parse_review_event(text: &str, relative: &Path) -> Result<ReviewRecord> {
    match parse_review_file(text, relative)? {
        ReviewFile::BranchEvent(record) => Ok(record),
        _ => parse_review_record(text, relative),
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
        ReviewFile::BranchEvent(record) => bail!(
            "review {} is about branch {}, not an issue",
            display_state_path(relative),
            record.target
        ),
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
        || record.header.relationships != relationships(Some(&record.target), &record.event)
    {
        bail!("review relationships must match its typed event references");
    }
    if let Some(disposition) = record.disposition() {
        validate_disposition(disposition)?;
    }
    Ok(())
}

/// Validates an event about a branch: `target` holds the branch name. It
/// links no issue, and there is no trivial exemption of a branch: an
/// approval is the light path's only cover (atelier-4nsl).
pub fn validate_branch_review(record: &ReviewRecord) -> Result<()> {
    validate_record_id(&record.header.id)?;
    validate_branch_name(&record.target)?;
    validate_commit(&record.commit)?;
    if record.event == ReviewEvent::TrivialExemption {
        bail!("a branch has no trivial exemption; an approval covers a branch no issue owns");
    }
    if record.identity.trim().is_empty()
        || record.summary.trim().is_empty()
        || record.header.title.trim().is_empty()
    {
        bail!("review requires an explicit identity and explanation");
    }
    if record.header.kind != "review"
        || record.header.relationships != relationships(None, &record.event)
    {
        bail!("review relationships must match its typed event references");
    }
    if let Some(disposition) = record.disposition() {
        validate_disposition(disposition)?;
    }
    Ok(())
}

/// A branch name as Git allows one for a local branch, as far as a review
/// record needs: one line, with no space, control character or `..`, and
/// none of the characters Git refuses in a ref.
pub fn validate_branch_name(name: &str) -> Result<()> {
    let refused = name.is_empty()
        || name.starts_with(['-', '/', '.'])
        || name.ends_with(['/', '.'])
        || name.ends_with(".lock")
        || name.contains("..")
        || name.contains("//")
        || name.contains("@{")
        || name
            .chars()
            .any(|c| c.is_whitespace() || c.is_control() || "~^:?*[\\".contains(c));
    if refused {
        bail!("{name:?} is not a branch name");
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
        branch: None,
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
        schema_version: written_version(&record.event, REVIEW_KIND.schema_version),
        id: record.header.id.clone(),
        title: record.header.title.clone(),
        target: Some(record.target.clone()),
        base: None,
        branch: None,
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

/// Renders an event about a branch, whose `target` is the branch name, at
/// [`BRANCH_TARGET_VERSION`], or [`WONT_FIX_VERSION`] when it holds
/// `wont_fix`.
pub fn render_branch_review_record(record: &ReviewRecord) -> Result<String> {
    validate_branch_review(record)?;
    let front = Front {
        schema: REVIEW_KIND.schema.into(),
        schema_version: written_version(&record.event, BRANCH_TARGET_VERSION),
        id: record.header.id.clone(),
        title: record.header.title.clone(),
        target: None,
        base: None,
        branch: Some(record.target.clone()),
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
                relationships: relationships(Some(target), &event),
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

    /// Creates a new event about `branch`, a branch no issue owns, on the
    /// light path (atelier-4nsl). Never replaces an existing path.
    pub fn create_branch_review(
        &self,
        branch: &str,
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
                title: generated_title(event.name(), "on branch", branch, commit),
                status: "recorded".into(),
                labels: Vec::new(),
                relationships: relationships(None, &event),
                created_at: now,
                updated_at: now,
            },
            target: branch.into(),
            identity: identity.into(),
            commit: commit.into(),
            event,
            summary: summary.into(),
        };
        self.publish_review(&record.header.id, render_branch_review_record(&record)?)?;
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
        let text = self.read_record_file(&relative)?;
        // A finding about a branch stays about that branch.
        let branch = matches!(
            parse_review_file(&text, &relative)?,
            ReviewFile::BranchEvent(_)
        );
        let mut record = parse_review_event(&text, &relative)?;
        let previous = set_finding_disposition(&mut record, disposition)?;
        let rendered = if branch {
            render_branch_review_record(&record)?
        } else {
            render_review_record(&record)?
        };
        self.write_atomic(&relative, rendered)?;
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
        let newer = parse_review_file(&text(5), path).unwrap_err();
        assert_eq!(
            newer.downcast_ref::<atelier_core::NewerSchema>(),
            Some(&atelier_core::NewerSchema {
                path: ".atelier/reviews/atelier-r001.md".into(),
                found: 5,
                supported: 4,
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
            ReviewFile::BranchEvent(record) => render_branch_review_record(&record).unwrap(),
            ReviewFile::RetiredDisposition(_) => unreachable!("never rendered"),
        }
    }

    fn event_file(version: i64, subject: &str, event: &str) -> String {
        let commit = "a".repeat(40);
        format!(
            "---\nschema: atelier.review\nschema_version: {version}\nid: atelier-r001\ntitle: why\n{subject}identity: Reviewer\ncommit: {commit}\ncreated_at: 2026-11-07T00:00:00Z\nevent:\n{event}---\n\nwhy\n"
        )
    }

    /// An event about a branch is written at schema version 3, names the
    /// branch and links no issue; an event about an issue is still written
    /// at version 2, so a binary that predates the light path keeps reading
    /// it (atelier-4nsl).
    #[test]
    fn an_event_about_a_branch_is_written_at_the_newer_version_and_round_trips() {
        let path = Path::new("reviews/atelier-r001.md");
        let ReviewFile::BranchEvent(record) = parse_review_file(
            &event_file(3, "branch: release\n", "  event: approval\n"),
            path,
        )
        .unwrap() else {
            panic!("an event about a branch parses as one");
        };
        assert_eq!(record.target, "release");
        assert!(record.header.relationships.attachments.is_empty());
        let text = render_branch_review_record(&record).unwrap();
        assert!(text.contains("schema_version: 3\n"), "{text}");
        assert!(
            text.contains("branch: release\n") && !text.contains("target:"),
            "{text}"
        );
        assert_eq!(
            parse_review_file(&text, path).unwrap(),
            ReviewFile::BranchEvent(record.clone())
        );
        // Never read as an event about an issue.
        assert!(parse_review_record(&text, path).is_err());
        assert_eq!(parse_review_event(&text, path).unwrap(), record);
        let ReviewFile::Event(issue) = parse_review_file(
            &event_file(2, "target: atelier-i001\n", "  event: approval\n"),
            path,
        )
        .unwrap() else {
            panic!("an event about an issue parses as one");
        };
        assert!(render_review_record(&issue)
            .unwrap()
            .contains("schema_version: 2\n"));
        // A deferred finding about a branch links its follow-up and keeps
        // linking no target when its disposition changes.
        let mut finding = record.clone();
        finding.event = ReviewEvent::Finding {
            recommendation: FindingDecision::Defer,
            disposition: None,
        };
        finding.header.relationships = relationships(None, &finding.event);
        set_finding_disposition(
            &mut finding,
            Disposition {
                decision: FindingDecision::Defer,
                identity: "Lead".into(),
                commit: "b".repeat(40),
                recorded_at: Utc::now(),
                resolution_commit: None,
                follow_up: Some("atelier-f001".into()),
                reason: "later".into(),
                migrated_from: None,
            },
        )
        .unwrap();
        let links = &finding.header.relationships.attachments;
        assert_eq!(links.len(), 1, "{links:?}");
        assert_eq!(
            (links[0].id.as_str(), links[0].role.as_str()),
            ("atelier-f001", "follow_up")
        );
        render_branch_review_record(&finding).unwrap();
    }

    fn wont_fix_disposition() -> Disposition {
        Disposition {
            decision: FindingDecision::WontFix,
            identity: "Lead".into(),
            commit: "b".repeat(40),
            recorded_at: "2026-11-08T00:00:00Z".parse().unwrap(),
            resolution_commit: None,
            follow_up: None,
            reason: "not worth it".into(),
            migrated_from: None,
        }
    }

    /// A finding that recommends or decides wont_fix is written at schema
    /// version 4, about an issue or a branch, so an older Atelier reports it
    /// as needing a newer one rather than as invalid; a finding without it
    /// keeps version 2 (or 3 about a branch), and both round-trip
    /// (atelier-xlxn).
    #[test]
    fn a_wont_fix_finding_is_written_at_the_newer_version_and_round_trips() {
        let path = Path::new("reviews/atelier-r001.md");
        let finding = |subject: &str, recommendation: &str| {
            event_file(
                2,
                subject,
                &format!("  event: finding\n  recommendation: {recommendation}\n"),
            )
        };
        let ReviewFile::Event(plain) =
            parse_review_file(&finding("target: atelier-i001\n", "fix_now"), path).unwrap()
        else {
            panic!("a finding about an issue parses as one");
        };
        let text = render_review_record(&plain).unwrap();
        assert!(text.contains("schema_version: 2\n"), "{text}");
        assert_eq!(
            parse_review_file(&text, path).unwrap(),
            ReviewFile::Event(plain.clone())
        );

        let mut recommended = plain.clone();
        recommended.event = ReviewEvent::Finding {
            recommendation: FindingDecision::WontFix,
            disposition: None,
        };
        let mut decided = plain.clone();
        set_finding_disposition(&mut decided, wont_fix_disposition()).unwrap();
        for record in [recommended, decided] {
            let text = render_review_record(&record).unwrap();
            assert!(text.contains("schema_version: 4\n"), "{text}");
            assert_eq!(
                parse_review_file(&text, path).unwrap(),
                ReviewFile::Event(record.clone())
            );
            let older = parse_review_file_supporting(&text, path, 3).unwrap_err();
            assert!(older.is::<atelier_core::NewerSchema>(), "{older:#}");
        }

        let ReviewFile::BranchEvent(mut branch) = parse_review_file(
            &event_file(
                3,
                "branch: release\n",
                "  event: finding\n  recommendation: fix_now\n",
            ),
            path,
        )
        .unwrap() else {
            panic!("a finding about a branch parses as one");
        };
        assert!(render_branch_review_record(&branch)
            .unwrap()
            .contains("schema_version: 3\n"));
        set_finding_disposition(&mut branch, wont_fix_disposition()).unwrap();
        let text = render_branch_review_record(&branch).unwrap();
        assert!(text.contains("schema_version: 4\n"), "{text}");
        assert_eq!(
            parse_review_file(&text, path).unwrap(),
            ReviewFile::BranchEvent(branch)
        );

        // A wont_fix at an older version is refused, as no Atelier wrote it.
        assert!(parse_review_file(&finding("target: atelier-i001\n", "wont_fix"), path).is_err());
    }

    /// A reader that predates the light path, whose newest version is 2,
    /// reads an event about a branch as a record a newer Atelier wrote, not
    /// as an invalid one; this Atelier refuses a branch at an older version,
    /// a trivial exemption of a branch, and a record with two subjects.
    #[test]
    fn an_older_reader_needs_a_newer_atelier_for_an_event_about_a_branch() {
        let path = Path::new("reviews/atelier-r001.md");
        let text = event_file(3, "branch: release\n", "  event: approval\n");
        let older = parse_review_file_supporting(&text, path, 2).unwrap_err();
        let newer = older
            .downcast_ref::<atelier_core::NewerSchema>()
            .expect("needs a newer Atelier");
        assert_eq!((newer.found, newer.supported), (3, 2));
        assert!(newer.to_string().contains("newer"), "{newer}");
        for (version, subject, event) in [
            (2, "branch: release\n", "  event: approval\n"),
            (3, "branch: release\n", "  event: trivial_exemption\n"),
            (
                3,
                "branch: release\ntarget: atelier-i001\n",
                "  event: approval\n",
            ),
            (3, "branch: release\nbase: main\n", "  event: approval\n"),
            (3, "branch: two words\n", "  event: approval\n"),
        ] {
            let text = event_file(version, subject, event);
            assert!(parse_review_file(&text, path).is_err(), "{text}");
        }
    }
}
