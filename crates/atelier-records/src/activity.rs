use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, SecondsFormat, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::mission_plan_review::{MissionGraphRevision, MissionPlanReviewEvent};

const ACTIVITY_SCHEMA: &str = "atelier.activity";
const ACTIVITY_SCHEMA_VERSION: i64 = 1;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IssueActivity {
    pub id: String,
    pub subject_kind: String,
    pub subject_id: String,
    pub event_type: ActivityEventType,
    pub actor: String,
    pub created_at: DateTime<Utc>,
    pub summary: String,
    pub pr_attribution: Option<ActivityPrAttribution>,
    pub workflow_transition: Option<WorkflowTransitionActivity>,
    pub mission_plan_review: Option<MissionPlanReviewEvent>,
    pub body: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityPrAttribution {
    pub action: String,
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_author: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionPlanStartAuthorization {
    pub graph_revision: MissionGraphRevision,
    pub approval_activity_id: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowTransitionActivity {
    pub transition: String,
    pub from: String,
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mission_plan_start: Option<MissionPlanStartAuthorization>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ActivityEventType {
    Comment,
    Note,
    Handoff,
    Plan,
    CloseReason,
    StatusChanged,
    FieldChanged,
    WorkStarted,
    WorkFinished,
    WorkAbandoned,
    EvidenceAttached,
    TransitionApplied,
    TransitionBlocked,
    MissionPlanReview,
}

impl ActivityEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Comment => "comment",
            Self::Note => "note",
            Self::Handoff => "handoff",
            Self::Plan => "plan",
            Self::CloseReason => "close_reason",
            Self::StatusChanged => "status_changed",
            Self::FieldChanged => "field_changed",
            Self::WorkStarted => "work_started",
            Self::WorkFinished => "work_finished",
            Self::WorkAbandoned => "work_abandoned",
            Self::EvidenceAttached => "evidence_attached",
            Self::TransitionApplied => "transition_applied",
            Self::TransitionBlocked => "transition_blocked",
            Self::MissionPlanReview => "mission_plan_review",
        }
    }
}

impl fmt::Display for ActivityEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ActivityEventType {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "comment" => Ok(Self::Comment),
            "note" => Ok(Self::Note),
            "handoff" => Ok(Self::Handoff),
            "plan" => Ok(Self::Plan),
            "close_reason" => Ok(Self::CloseReason),
            "status_changed" => Ok(Self::StatusChanged),
            "field_changed" => Ok(Self::FieldChanged),
            "work_started" => Ok(Self::WorkStarted),
            "work_finished" => Ok(Self::WorkFinished),
            "work_abandoned" => Ok(Self::WorkAbandoned),
            "evidence_attached" => Ok(Self::EvidenceAttached),
            "transition_applied" => Ok(Self::TransitionApplied),
            "transition_blocked" => Ok(Self::TransitionBlocked),
            "mission_plan_review" => Ok(Self::MissionPlanReview),
            other => bail!("Unsupported activity event_type '{}'", other),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActivityFrontMatter {
    schema: String,
    schema_version: i64,
    id: String,
    subject_kind: String,
    subject_id: String,
    event_type: String,
    actor: String,
    created_at: DateTime<Utc>,
    summary: String,
    #[serde(default)]
    pr_attribution: Option<ActivityPrAttribution>,
    #[serde(default)]
    workflow_transition: Option<WorkflowTransitionActivity>,
    #[serde(default)]
    mission_plan_review: Option<MissionPlanReviewEvent>,
}

#[cfg(test)]
pub fn issue_activity_path(issue_id: &str, activity_id: &str) -> PathBuf {
    record_activity_path("issue", issue_id, activity_id)
}

pub fn record_activity_dir(subject_kind: &str, subject_id: &str) -> PathBuf {
    PathBuf::from(record_root(subject_kind)).join(format!("{subject_id}.activity"))
}

pub fn record_activity_path(subject_kind: &str, subject_id: &str, activity_id: &str) -> PathBuf {
    record_activity_dir(subject_kind, subject_id).join(format!("{activity_id}.md"))
}

pub fn list_issue_activities(state_dir: &Path, issue_id: &str) -> Result<Vec<IssueActivity>> {
    list_record_activities(state_dir, "issue", issue_id)
}

pub fn list_record_activities(
    state_dir: &Path,
    subject_kind: &str,
    subject_id: &str,
) -> Result<Vec<IssueActivity>> {
    let dir = state_dir.join(record_activity_dir(subject_kind, subject_id));
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut activities = Vec::new();
    for entry in fs::read_dir(&dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let file_name = path
            .file_name()
            .ok_or_else(|| anyhow!("Invalid activity path {}", path.display()))?;
        let relative = record_activity_dir(subject_kind, subject_id).join(file_name);
        activities.push(IssueActivity::load(state_dir, &relative)?);
    }
    activities.sort_by(|a, b| a.created_at.cmp(&b.created_at).then(a.id.cmp(&b.id)));
    Ok(activities)
}

pub fn list_all_issue_activities(state_dir: &Path) -> Result<Vec<IssueActivity>> {
    list_all_record_activities(state_dir, "issue")
}

pub fn list_all_record_activities(
    state_dir: &Path,
    subject_kind: &str,
) -> Result<Vec<IssueActivity>> {
    let record_dir = state_dir.join(record_root(subject_kind));
    if !record_dir.exists() {
        return Ok(Vec::new());
    }
    let mut activities = Vec::new();
    for entry in fs::read_dir(&record_dir)
        .with_context(|| format!("Failed to read {}", record_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(subject_id) = name.strip_suffix(".activity") else {
            continue;
        };
        activities.extend(list_record_activities(state_dir, subject_kind, subject_id)?);
    }
    activities.sort_by(|a, b| a.created_at.cmp(&b.created_at).then(a.id.cmp(&b.id)));
    Ok(activities)
}

pub fn timestamp_activity_id(created_at: DateTime<Utc>) -> String {
    format!(
        "{}{:06}Z",
        created_at.format("%Y%m%dT%H%M%S"),
        created_at.nanosecond() / 1_000
    )
}

/// Require the precision that canonical activity Markdown can preserve.
///
/// Activity timestamps render with exactly six fractional digits. Rejecting
/// finer precision avoids silently changing timestamps, IDs, or receipt hashes
/// when records are rendered and loaded again.
pub fn validate_activity_timestamp(created_at: DateTime<Utc>) -> Result<()> {
    if !created_at.nanosecond().is_multiple_of(1_000) {
        bail!(
            "activity created_at must use microsecond precision; sub-microsecond timestamps cannot round-trip through canonical activity records"
        );
    }
    Ok(())
}

fn canonical_activity_timestamp(created_at: DateTime<Utc>) -> DateTime<Utc> {
    created_at
        .with_nanosecond((created_at.nanosecond() / 1_000) * 1_000)
        .expect("truncated nanoseconds remain in the valid timestamp range")
}

pub fn allocate_activity_id(
    state_dir: &Path,
    subject_kind: &str,
    subject_id: &str,
    created_at: DateTime<Utc>,
) -> Result<String> {
    let base = timestamp_activity_id(created_at);
    for suffix in 0..=99 {
        let candidate = if suffix == 0 {
            base.clone()
        } else {
            format!("{base}-{suffix:02}")
        };
        if !state_dir
            .join(record_activity_path(subject_kind, subject_id, &candidate))
            .exists()
        {
            return Ok(candidate);
        }
    }
    bail!("No available activity id for {subject_kind} {subject_id} at {base}")
}

#[cfg(test)]
pub fn write_issue_activity(state_dir: &Path, activity: &IssueActivity) -> Result<PathBuf> {
    write_record_activity(state_dir, activity)
}

pub fn write_record_activity(state_dir: &Path, activity: &IssueActivity) -> Result<PathBuf> {
    let _lock = crate::mutation_lock::CanonicalMutationLock::shared(state_dir)?;
    let rendered = activity.to_markdown()?;
    let relative = record_activity_path(&activity.subject_kind, &activity.subject_id, &activity.id);
    let path = state_dir.join(&relative);
    let parent = path.parent().ok_or_else(|| {
        anyhow!(
            "Cannot determine parent for {}",
            display_state_path(&relative)
        )
    })?;
    fs::create_dir_all(parent).with_context(|| format!("Failed to create {}", parent.display()))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .with_context(|| format!("Refusing to overwrite {}", display_state_path(&relative)))?;
    file.write_all(rendered.as_bytes())
        .with_context(|| format!("Failed to write {}", display_state_path(&relative)))?;
    Ok(relative)
}

pub fn create_issue_activity(
    state_dir: &Path,
    subject_id: &str,
    event_type: ActivityEventType,
    actor: &str,
    created_at: DateTime<Utc>,
    summary: &str,
    body: &str,
) -> Result<IssueActivity> {
    create_issue_activity_with_metadata(
        state_dir, subject_id, event_type, actor, created_at, summary, None, body,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn create_issue_activity_with_metadata(
    state_dir: &Path,
    subject_id: &str,
    event_type: ActivityEventType,
    actor: &str,
    created_at: DateTime<Utc>,
    summary: &str,
    pr_attribution: Option<ActivityPrAttribution>,
    body: &str,
) -> Result<IssueActivity> {
    create_record_activity_with_metadata(
        state_dir,
        "issue",
        subject_id,
        event_type,
        actor,
        created_at,
        summary,
        pr_attribution,
        body,
    )
}

/// Append one typed mission-plan review event to a mission's activity stream.
pub fn create_mission_plan_review_activity(
    state_dir: &Path,
    mission_id: &str,
    actor: &str,
    created_at: DateTime<Utc>,
    summary: &str,
    event: MissionPlanReviewEvent,
    body: &str,
) -> Result<IssueActivity> {
    let _transaction = crate::mutation_lock::CanonicalMutationLock::shared(state_dir)?;
    let created_at = canonical_activity_timestamp(created_at);
    let id = allocate_activity_id(state_dir, "issue", mission_id, created_at)?;
    let activity = IssueActivity {
        id,
        subject_kind: "issue".to_string(),
        subject_id: mission_id.to_string(),
        event_type: ActivityEventType::MissionPlanReview,
        actor: actor.to_string(),
        created_at,
        summary: summary.to_string(),
        pr_attribution: None,
        workflow_transition: None,
        mission_plan_review: Some(event),
        body: normalize_body(body),
    };
    write_record_activity(state_dir, &activity)?;
    Ok(activity)
}

pub fn create_record_activity(
    state_dir: &Path,
    subject_kind: &str,
    subject_id: &str,
    event_type: ActivityEventType,
    actor: &str,
    created_at: DateTime<Utc>,
    summary: &str,
    body: &str,
) -> Result<IssueActivity> {
    create_record_activity_with_metadata(
        state_dir,
        subject_kind,
        subject_id,
        event_type,
        actor,
        created_at,
        summary,
        None,
        body,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn create_record_activity_with_metadata(
    state_dir: &Path,
    subject_kind: &str,
    subject_id: &str,
    event_type: ActivityEventType,
    actor: &str,
    created_at: DateTime<Utc>,
    summary: &str,
    pr_attribution: Option<ActivityPrAttribution>,
    body: &str,
) -> Result<IssueActivity> {
    let _transaction = crate::mutation_lock::CanonicalMutationLock::shared(state_dir)?;
    let created_at = canonical_activity_timestamp(created_at);
    let id = allocate_activity_id(state_dir, subject_kind, subject_id, created_at)?;
    let activity = IssueActivity {
        id,
        subject_kind: subject_kind.to_string(),
        subject_id: subject_id.to_string(),
        event_type,
        actor: actor.to_string(),
        created_at,
        summary: summary.to_string(),
        pr_attribution,
        workflow_transition: None,
        mission_plan_review: None,
        body: normalize_body(body),
    };
    write_record_activity(state_dir, &activity)?;
    Ok(activity)
}

impl IssueActivity {
    pub fn from_markdown(text: &str, relative: &Path) -> Result<Self> {
        let (front_matter, body) = split_front_matter(text, relative)?;
        let front: ActivityFrontMatter = serde_yaml::from_str(front_matter).with_context(|| {
            format!(
                "Invalid activity front matter in {}",
                display_state_path(relative)
            )
        })?;

        if front.schema != ACTIVITY_SCHEMA {
            bail!(
                "Unsupported schema '{}' in {}; expected {}",
                front.schema,
                display_state_path(relative),
                ACTIVITY_SCHEMA
            );
        }
        if front.schema_version != ACTIVITY_SCHEMA_VERSION {
            bail!(
                "Unsupported schema_version {} in {}; expected {}",
                front.schema_version,
                display_state_path(relative),
                ACTIVITY_SCHEMA_VERSION
            );
        }
        validate_subject_kind(&front.subject_kind, relative)?;
        let expected = record_activity_path(&front.subject_kind, &front.subject_id, &front.id);
        if relative != expected {
            bail!(
                "Activity id {} for {} {} in {} does not match record-file path {}",
                front.id,
                front.subject_kind,
                front.subject_id,
                display_state_path(relative),
                display_state_path(&expected)
            );
        }
        let event_type = ActivityEventType::from_str(&front.event_type).with_context(|| {
            format!(
                "Invalid event_type '{}' in {}",
                front.event_type,
                display_state_path(relative)
            )
        })?;
        validate_workflow_transition_metadata(
            event_type,
            front.workflow_transition.as_ref(),
            relative,
        )?;
        validate_plan_review_metadata(event_type, front.mission_plan_review.as_ref(), relative)?;
        validate_activity_timestamp(front.created_at).with_context(|| {
            format!(
                "Invalid activity timestamp in {}",
                display_state_path(relative)
            )
        })?;

        Ok(Self {
            id: front.id,
            subject_kind: front.subject_kind,
            subject_id: front.subject_id,
            event_type,
            actor: front.actor,
            created_at: front.created_at,
            summary: front.summary,
            pr_attribution: front.pr_attribution,
            workflow_transition: front.workflow_transition,
            mission_plan_review: front.mission_plan_review,
            body: body.to_string(),
        })
    }

    pub fn load(state_dir: &Path, relative: &Path) -> Result<Self> {
        let bytes = fs::read(state_dir.join(relative))
            .with_context(|| format!("Missing activity file {}", display_state_path(relative)))?;
        let text = String::from_utf8(bytes).with_context(|| {
            format!(
                "Activity file {} is not UTF-8",
                display_state_path(relative)
            )
        })?;
        Self::from_markdown(&text, relative)
    }

    pub fn to_markdown(&self) -> Result<String> {
        validate_activity_timestamp(self.created_at)?;
        let mut output = String::new();
        output.push_str("---\n");
        write_yaml_scalar(&mut output, "schema", ACTIVITY_SCHEMA)?;
        output.push_str("schema_version: 1\n");
        write_yaml_scalar(&mut output, "id", &self.id)?;
        validate_subject_kind(&self.subject_kind, Path::new("<generated>"))?;
        write_yaml_scalar(&mut output, "subject_kind", &self.subject_kind)?;
        write_yaml_scalar(&mut output, "subject_id", &self.subject_id)?;
        write_yaml_scalar(&mut output, "event_type", self.event_type.as_str())?;
        write_yaml_scalar(&mut output, "actor", &self.actor)?;
        write_yaml_scalar(
            &mut output,
            "created_at",
            &self.created_at.to_rfc3339_opts(SecondsFormat::Micros, true),
        )?;
        write_yaml_scalar(&mut output, "summary", &self.summary)?;
        write_yaml_struct_if_some(&mut output, "pr_attribution", self.pr_attribution.as_ref())?;
        validate_workflow_transition_metadata(
            self.event_type,
            self.workflow_transition.as_ref(),
            Path::new("<generated>"),
        )?;
        write_yaml_struct_if_some(
            &mut output,
            "workflow_transition",
            self.workflow_transition.as_ref(),
        )?;
        validate_plan_review_metadata(
            self.event_type,
            self.mission_plan_review.as_ref(),
            Path::new("<generated>"),
        )?;
        write_yaml_struct_if_some(
            &mut output,
            "mission_plan_review",
            self.mission_plan_review.as_ref(),
        )?;
        output.push_str("---\n\n");
        output.push_str(&normalize_body(&self.body));
        output.push('\n');
        Ok(output)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_workflow_transition_activity(
    state_dir: &Path,
    issue_id: &str,
    actor: &str,
    created_at: DateTime<Utc>,
    summary: &str,
    workflow_transition: WorkflowTransitionActivity,
    body: &str,
) -> Result<IssueActivity> {
    let _transaction = crate::mutation_lock::CanonicalMutationLock::shared(state_dir)?;
    let created_at = canonical_activity_timestamp(created_at);
    let id = allocate_activity_id(state_dir, "issue", issue_id, created_at)?;
    let activity = IssueActivity {
        id,
        subject_kind: "issue".to_string(),
        subject_id: issue_id.to_string(),
        event_type: ActivityEventType::TransitionApplied,
        actor: actor.to_string(),
        created_at,
        summary: summary.to_string(),
        pr_attribution: None,
        workflow_transition: Some(workflow_transition),
        mission_plan_review: None,
        body: normalize_body(body),
    };
    write_record_activity(state_dir, &activity)?;
    Ok(activity)
}

fn validate_plan_review_metadata(
    event_type: ActivityEventType,
    event: Option<&MissionPlanReviewEvent>,
    relative: &Path,
) -> Result<()> {
    match (event_type, event) {
        (ActivityEventType::MissionPlanReview, Some(event)) => {
            event.validate().with_context(|| {
                format!(
                    "Invalid mission-plan review event in {}",
                    display_state_path(relative)
                )
            })
        }
        (ActivityEventType::MissionPlanReview, None) => bail!(
            "Mission-plan review activity in {} is missing mission_plan_review metadata",
            display_state_path(relative)
        ),
        (_, Some(_)) => bail!(
            "Activity in {} has mission_plan_review metadata but event_type is '{}'",
            display_state_path(relative),
            event_type
        ),
        (_, None) => Ok(()),
    }
}

fn validate_workflow_transition_metadata(
    event_type: ActivityEventType,
    transition: Option<&WorkflowTransitionActivity>,
    relative: &Path,
) -> Result<()> {
    let Some(transition) = transition else {
        return Ok(());
    };
    if event_type != ActivityEventType::TransitionApplied {
        bail!(
            "Activity in {} has workflow_transition metadata but event_type is '{}'",
            display_state_path(relative),
            event_type
        );
    }
    for (field, value) in [
        ("transition", transition.transition.as_str()),
        ("from", transition.from.as_str()),
        ("to", transition.to.as_str()),
    ] {
        if value.trim().is_empty() {
            bail!(
                "workflow_transition.{field} in {} must not be empty",
                display_state_path(relative)
            );
        }
    }
    if let Some(authorization) = &transition.mission_plan_start {
        if transition.transition != "start"
            || transition.from != "ready"
            || transition.to != "in_progress"
        {
            bail!(
                "mission_plan_start in {} is valid only for start (ready -> in_progress)",
                display_state_path(relative)
            );
        }
        authorization.graph_revision.validate()?;
        if authorization.approval_activity_id.trim().is_empty() {
            bail!(
                "mission_plan_start.approval_activity_id in {} must not be empty",
                display_state_path(relative)
            );
        }
    }
    Ok(())
}

fn write_yaml_struct_if_some<T: Serialize>(
    output: &mut String,
    key: &str,
    value: Option<&T>,
) -> Result<()> {
    let Some(value) = value else {
        return Ok(());
    };
    let rendered = serde_yaml::to_string(value)?;
    output.push_str(key);
    output.push_str(":\n");
    for line in rendered.trim_end().lines() {
        output.push_str("  ");
        output.push_str(line);
        output.push('\n');
    }
    Ok(())
}

fn split_front_matter<'a>(text: &'a str, relative: &Path) -> Result<(&'a str, &'a str)> {
    let rest = text.strip_prefix("---\n").ok_or_else(|| {
        anyhow!(
            "Missing YAML front matter in {}",
            display_state_path(relative)
        )
    })?;
    let (front, body) = rest.split_once("\n---\n").ok_or_else(|| {
        anyhow!(
            "Unterminated YAML front matter in {}",
            display_state_path(relative)
        )
    })?;
    let body = body.strip_prefix('\n').unwrap_or(body);
    let body = body.strip_suffix('\n').unwrap_or(body);
    Ok((front, body))
}

fn write_yaml_scalar(output: &mut String, key: &str, value: &str) -> Result<()> {
    output.push_str(key);
    output.push_str(": ");
    output.push_str(&serde_json::to_string(value)?);
    output.push('\n');
    Ok(())
}

fn normalize_body(body: &str) -> String {
    body.replace("\r\n", "\n").replace('\r', "\n")
}

fn record_root(subject_kind: &str) -> &'static str {
    match subject_kind {
        "issue" => "issues",
        _ => "records",
    }
}

fn validate_subject_kind(subject_kind: &str, relative: &Path) -> Result<()> {
    if subject_kind != "issue" {
        bail!(
            "Unsupported subject_kind '{}' in {}; expected issue",
            subject_kind,
            display_state_path(relative)
        );
    }
    Ok(())
}

fn display_state_path(relative_path: &Path) -> String {
    format!(
        ".atelier/{}",
        relative_path.to_string_lossy().replace('\\', "/")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use tempfile::tempdir;

    fn at() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 6, 10, 18, 19, 20)
            .unwrap()
            .with_nanosecond(123_456_000)
            .unwrap()
    }

    fn activity() -> IssueActivity {
        IssueActivity {
            id: "20260610T181920123456Z".to_string(),
            subject_kind: "issue".to_string(),
            subject_id: "atelier-qxvj".to_string(),
            event_type: ActivityEventType::Handoff,
            actor: "agent@example.com".to_string(),
            created_at: at(),
            summary: "Implemented activity sidecars".to_string(),
            pr_attribution: None,
            workflow_transition: None,
            mission_plan_review: None,
            body: "Line one\n\nLine two".to_string(),
        }
    }

    #[test]
    fn timestamp_activity_id_uses_utc_microseconds() {
        assert_eq!(timestamp_activity_id(at()), "20260610T181920123456Z");
    }

    #[test]
    fn rejects_sub_microsecond_timestamps_before_emission_or_load() {
        let sub_microsecond = DateTime::parse_from_rfc3339("2026-06-10T18:19:20.123456789Z")
            .unwrap()
            .with_timezone(&Utc);
        let mut invalid = activity();
        invalid.created_at = sub_microsecond;

        let directory = tempdir().unwrap();
        let error = write_issue_activity(directory.path(), &invalid)
            .unwrap_err()
            .to_string();
        assert!(error.contains("must use microsecond precision"), "{error}");
        assert!(!directory
            .path()
            .join(issue_activity_path(&invalid.subject_id, &invalid.id))
            .exists());

        let created = create_issue_activity(
            directory.path(),
            "atelier-qxvj",
            ActivityEventType::Note,
            "agent@example.com",
            sub_microsecond,
            "Canonicalized producer timestamp",
            "Round-trippable body",
        )
        .unwrap();
        assert_eq!(created.created_at.nanosecond(), 123_456_000);
        assert_eq!(created.id, "20260610T181920123456Z");
        assert_eq!(
            IssueActivity::load(
                directory.path(),
                &issue_activity_path(&created.subject_id, &created.id)
            )
            .unwrap(),
            created
        );

        let rendered = activity().to_markdown().unwrap().replace(
            "2026-06-10T18:19:20.123456Z",
            "2026-06-10T18:19:20.123456789Z",
        );
        let error = IssueActivity::from_markdown(
            &rendered,
            &issue_activity_path(&invalid.subject_id, &invalid.id),
        )
        .unwrap_err();
        let error = format!("{error:#}");
        assert!(error.contains("must use microsecond precision"), "{error}");
    }

    #[test]
    fn issue_activity_sidecar_path_is_canonical() {
        assert_eq!(
            issue_activity_path("atelier-qxvj", "20260610T181920123456Z"),
            PathBuf::from("issues/atelier-qxvj.activity/20260610T181920123456Z.md")
        );
    }

    #[test]
    fn front_matter_and_body_round_trip() {
        let activity = activity();
        let rendered = activity.to_markdown().unwrap();

        assert!(rendered.contains("schema: \"atelier.activity\""));
        assert!(rendered.contains("schema_version: 1"));
        assert!(rendered.contains("subject_kind: \"issue\""));
        assert!(rendered.ends_with("Line one\n\nLine two\n"));

        let parsed = IssueActivity::from_markdown(
            &rendered,
            &issue_activity_path(&activity.subject_id, &activity.id),
        )
        .unwrap();
        assert_eq!(parsed, activity);
        assert_eq!(parsed.to_markdown().unwrap(), rendered);
    }

    #[test]
    fn mission_start_authorization_round_trips_as_typed_transition_metadata() {
        let authorization = MissionPlanStartAuthorization {
            graph_revision: MissionGraphRevision(format!(
                "mission-graph-v2:sha256:{}",
                "a".repeat(64)
            )),
            approval_activity_id: "20260610T181919123456Z".to_string(),
        };
        let transition = WorkflowTransitionActivity {
            transition: "start".to_string(),
            from: "ready".to_string(),
            to: "in_progress".to_string(),
            mission_plan_start: Some(authorization),
        };
        let mut activity = activity();
        activity.event_type = ActivityEventType::TransitionApplied;
        activity.workflow_transition = Some(transition);

        let rendered = activity.to_markdown().unwrap();
        assert!(rendered.contains("workflow_transition:"));
        assert!(rendered.contains("mission_plan_start:"));
        assert!(rendered.contains("approval_activity_id:"));
        let parsed = IssueActivity::from_markdown(
            &rendered,
            &issue_activity_path(&activity.subject_id, &activity.id),
        )
        .unwrap();
        assert_eq!(parsed, activity);
    }

    #[test]
    fn mission_start_authorization_is_rejected_outside_exact_start_transition() {
        let mut activity = activity();
        activity.event_type = ActivityEventType::TransitionApplied;
        activity.workflow_transition = Some(WorkflowTransitionActivity {
            transition: "ready".to_string(),
            from: "plan_review".to_string(),
            to: "ready".to_string(),
            mission_plan_start: Some(MissionPlanStartAuthorization {
                graph_revision: MissionGraphRevision(format!(
                    "mission-graph-v2:sha256:{}",
                    "b".repeat(64)
                )),
                approval_activity_id: "20260610T181919123456Z".to_string(),
            }),
        });

        let error = activity.to_markdown().unwrap_err().to_string();
        assert!(
            error.contains("valid only for start (ready -> in_progress)"),
            "{error}"
        );
    }

    #[test]
    fn rejects_invalid_schema_version_subject_and_event_type() {
        let rendered = activity().to_markdown().unwrap();
        let relative = issue_activity_path("atelier-qxvj", "20260610T181920123456Z");

        let error = IssueActivity::from_markdown(
            &rendered.replace("schema_version: 1", "schema_version: 2"),
            &relative,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("Unsupported schema_version 2"));

        let error = IssueActivity::from_markdown(
            &rendered.replace("subject_kind: \"issue\"", "subject_kind: \"plan\""),
            &relative,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("Unsupported subject_kind 'plan'"));

        let error = IssueActivity::from_markdown(
            &rendered.replace("event_type: \"handoff\"", "event_type: \"unknown\""),
            &relative,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("Invalid event_type 'unknown'"));

        let error = IssueActivity::from_markdown(
            &rendered.replace("event_type: \"handoff\"", "event_type: \"decision\""),
            &relative,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("Invalid event_type 'decision'"));
    }

    #[test]
    fn rejects_invalid_schema() {
        let rendered = activity().to_markdown().unwrap();
        let relative = issue_activity_path("atelier-qxvj", "20260610T181920123456Z");
        let error = IssueActivity::from_markdown(
            &rendered.replace("schema: \"atelier.activity\"", "schema: \"atelier.issue\""),
            &relative,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("Unsupported schema 'atelier.issue'"));
    }

    #[test]
    fn allocation_adds_deterministic_suffixes_for_same_timestamp_collisions() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path();
        let issue_id = "atelier-qxvj";
        let base = timestamp_activity_id(at());
        let first_path = state_dir.join(issue_activity_path(issue_id, &base));
        fs::create_dir_all(first_path.parent().unwrap()).unwrap();
        fs::write(&first_path, "existing").unwrap();

        assert_eq!(
            allocate_activity_id(state_dir, "issue", issue_id, at()).unwrap(),
            format!("{base}-01")
        );
        fs::write(
            state_dir.join(issue_activity_path(issue_id, &format!("{base}-01"))),
            "existing",
        )
        .unwrap();
        assert_eq!(
            allocate_activity_id(state_dir, "issue", issue_id, at()).unwrap(),
            format!("{base}-02")
        );
    }

    #[test]
    fn write_refuses_to_overwrite_existing_activity() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path();
        let activity = activity();
        let relative = write_issue_activity(state_dir, &activity).unwrap();

        assert_eq!(
            relative,
            issue_activity_path(&activity.subject_id, &activity.id)
        );
        let error = write_issue_activity(state_dir, &activity)
            .unwrap_err()
            .to_string();
        assert!(error.contains("Refusing to overwrite"));
        assert_eq!(IssueActivity::load(state_dir, &relative).unwrap(), activity);
    }

    #[test]
    fn create_allocates_collision_suffix_and_does_not_overwrite() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path();
        let issue_id = "atelier-qxvj";
        let existing = activity();
        write_issue_activity(state_dir, &existing).unwrap();

        let created = create_issue_activity(
            state_dir,
            issue_id,
            ActivityEventType::Note,
            "agent@example.com",
            at(),
            "Follow-up",
            "Second body",
        )
        .unwrap();

        assert_eq!(created.id, "20260610T181920123456Z-01");
        assert_eq!(
            fs::read_to_string(state_dir.join(issue_activity_path(issue_id, &existing.id)))
                .unwrap(),
            existing.to_markdown().unwrap()
        );
        assert_eq!(
            IssueActivity::load(state_dir, &issue_activity_path(issue_id, &created.id)).unwrap(),
            created
        );
    }

    #[test]
    fn pr_metadata_round_trip() {
        let mut activity = activity();
        activity.event_type = ActivityEventType::WorkStarted;
        activity.pr_attribution = Some(ActivityPrAttribution {
            action: "comment".to_string(),
            role: "reviewer".to_string(),
            pull_request: Some("forgejo/example#42".to_string()),
            remote_author: Some("reviewer-user".to_string()),
        });

        let rendered = activity.to_markdown().unwrap();

        assert!(!rendered.contains("attempt:\n"));
        assert!(rendered.contains("pr_attribution:\n"));
        assert!(rendered.contains("  action: comment"));
        assert!(rendered.contains("  role: reviewer"));

        let parsed = IssueActivity::from_markdown(
            &rendered,
            &issue_activity_path(&activity.subject_id, &activity.id),
        )
        .unwrap();
        assert_eq!(parsed, activity);
    }

    #[test]
    fn lists_issue_activities_in_oldest_first_order() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path();
        let issue_id = "atelier-qxvj";
        let first = activity();
        write_issue_activity(state_dir, &first).unwrap();
        let second = create_issue_activity(
            state_dir,
            issue_id,
            ActivityEventType::Comment,
            "agent@example.com",
            at() + chrono::Duration::seconds(1),
            "Second",
            "Second body",
        )
        .unwrap();

        let listed = list_issue_activities(state_dir, issue_id).unwrap();

        assert_eq!(listed, vec![first, second]);
        assert!(list_issue_activities(state_dir, "atelier-missing")
            .unwrap()
            .is_empty());
    }
}
