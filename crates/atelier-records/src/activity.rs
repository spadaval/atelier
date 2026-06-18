use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, SecondsFormat, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

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
    pub attempt: Option<ActivityAttemptMetadata>,
    pub pr_attribution: Option<ActivityPrAttribution>,
    pub body: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityAttemptMetadata {
    pub role: ActivityAttemptRole,
    pub serial: u32,
    pub lifecycle: ActivityAttemptLifecycle,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subskill: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityAttemptRole {
    Worker,
    Reviewer,
    Validator,
}

impl ActivityAttemptRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Worker => "worker",
            Self::Reviewer => "reviewer",
            Self::Validator => "validator",
        }
    }
}

impl fmt::Display for ActivityAttemptRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityAttemptLifecycle {
    Started,
    Updated,
    Finished,
    Abandoned,
}

impl ActivityAttemptLifecycle {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Updated => "updated",
            Self::Finished => "finished",
            Self::Abandoned => "abandoned",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityPrAttribution {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_author: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DerivedIssueAttempt {
    pub id: String,
    pub issue_id: String,
    pub role: ActivityAttemptRole,
    pub serial: u32,
    pub state: DerivedIssueAttemptState,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub actor: String,
    pub agent: Option<String>,
    pub subskill: Option<String>,
    pub activity_ids: Vec<String>,
    pub activities: Vec<DerivedIssueAttemptActivity>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DerivedIssueAttemptActivity {
    pub id: String,
    pub event_type: ActivityEventType,
    pub lifecycle: ActivityAttemptLifecycle,
    pub created_at: DateTime<Utc>,
    pub summary: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DerivedIssueAttemptState {
    Active,
    Finished,
    Abandoned,
}

impl DerivedIssueAttemptState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Finished => "finished",
            Self::Abandoned => "abandoned",
        }
    }
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
    attempt: Option<ActivityAttemptMetadata>,
    #[serde(default)]
    pr_attribution: Option<ActivityPrAttribution>,
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

pub fn list_mission_activities(state_dir: &Path, mission_id: &str) -> Result<Vec<IssueActivity>> {
    list_record_activities(state_dir, "mission", mission_id)
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

pub fn list_all_mission_activities(state_dir: &Path) -> Result<Vec<IssueActivity>> {
    list_all_record_activities(state_dir, "mission")
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
    file.write_all(activity.to_markdown()?.as_bytes())
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
        state_dir, subject_id, event_type, actor, created_at, summary, None, None, body,
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
    attempt: Option<ActivityAttemptMetadata>,
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
        attempt,
        pr_attribution,
        body,
    )
}

pub fn create_mission_activity(
    state_dir: &Path,
    subject_id: &str,
    event_type: ActivityEventType,
    actor: &str,
    created_at: DateTime<Utc>,
    summary: &str,
    body: &str,
) -> Result<IssueActivity> {
    create_record_activity_with_metadata(
        state_dir, "mission", subject_id, event_type, actor, created_at, summary, None, None, body,
    )
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
    attempt: Option<ActivityAttemptMetadata>,
    pr_attribution: Option<ActivityPrAttribution>,
    body: &str,
) -> Result<IssueActivity> {
    let id = allocate_activity_id(state_dir, subject_kind, subject_id, created_at)?;
    let activity = IssueActivity {
        id,
        subject_kind: subject_kind.to_string(),
        subject_id: subject_id.to_string(),
        event_type,
        actor: actor.to_string(),
        created_at,
        summary: summary.to_string(),
        attempt,
        pr_attribution,
        body: normalize_body(body),
    };
    write_record_activity(state_dir, &activity)?;
    Ok(activity)
}

pub fn list_derived_issue_attempts(state_dir: &Path) -> Result<Vec<DerivedIssueAttempt>> {
    derive_issue_attempts(list_all_issue_activities(state_dir)?)
}

pub fn derive_issue_attempts(
    activities: impl IntoIterator<Item = IssueActivity>,
) -> Result<Vec<DerivedIssueAttempt>> {
    let mut activities = activities.into_iter().collect::<Vec<_>>();
    activities.sort_by(|a, b| a.created_at.cmp(&b.created_at).then(a.id.cmp(&b.id)));
    let mut attempts = BTreeMap::<(String, ActivityAttemptRole, u32), DerivedIssueAttempt>::new();
    for activity in activities {
        let Some(attempt) = activity.attempt.clone() else {
            continue;
        };
        let key = (activity.subject_id.clone(), attempt.role, attempt.serial);
        let id = derived_attempt_id(&activity.subject_id, attempt.role, attempt.serial);
        let entry = attempts.entry(key).or_insert_with(|| DerivedIssueAttempt {
            id,
            issue_id: activity.subject_id.clone(),
            role: attempt.role,
            serial: attempt.serial,
            state: DerivedIssueAttemptState::Active,
            started_at: activity.created_at,
            updated_at: activity.created_at,
            ended_at: None,
            actor: activity.actor.clone(),
            agent: attempt.agent.clone(),
            subskill: attempt.subskill.clone(),
            activity_ids: Vec::new(),
            activities: Vec::new(),
        });
        if activity.created_at < entry.started_at {
            entry.started_at = activity.created_at;
        }
        if activity.created_at >= entry.updated_at {
            entry.updated_at = activity.created_at;
            if entry.agent.is_none() {
                entry.agent = attempt.agent.clone();
            }
            if entry.subskill.is_none() {
                entry.subskill = attempt.subskill.clone();
            }
        }
        let lifecycle = attempt.lifecycle;
        match lifecycle {
            ActivityAttemptLifecycle::Started | ActivityAttemptLifecycle::Updated => {}
            ActivityAttemptLifecycle::Finished => {
                entry.state = DerivedIssueAttemptState::Finished;
                entry.ended_at = Some(activity.created_at);
            }
            ActivityAttemptLifecycle::Abandoned => {
                entry.state = DerivedIssueAttemptState::Abandoned;
                entry.ended_at = Some(activity.created_at);
            }
        }
        entry.activity_ids.push(activity.id.clone());
        entry.activities.push(DerivedIssueAttemptActivity {
            id: activity.id,
            event_type: activity.event_type,
            lifecycle,
            created_at: activity.created_at,
            summary: activity.summary,
        });
    }
    Ok(attempts.into_values().collect())
}

pub fn derived_attempt_id(issue_id: &str, role: ActivityAttemptRole, serial: u32) -> String {
    format!("{issue_id}/{role}/{serial}")
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
                "Activity id {} for {} {} in {} does not match canonical path {}",
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

        Ok(Self {
            id: front.id,
            subject_kind: front.subject_kind,
            subject_id: front.subject_id,
            event_type,
            actor: front.actor,
            created_at: front.created_at,
            summary: front.summary,
            attempt: front.attempt,
            pr_attribution: front.pr_attribution,
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
        write_yaml_struct_if_some(&mut output, "attempt", self.attempt.as_ref())?;
        write_yaml_struct_if_some(&mut output, "pr_attribution", self.pr_attribution.as_ref())?;
        output.push_str("---\n\n");
        output.push_str(&normalize_body(&self.body));
        output.push('\n');
        Ok(output)
    }
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
        "mission" => "missions",
        _ => "records",
    }
}

fn validate_subject_kind(subject_kind: &str, relative: &Path) -> Result<()> {
    if !matches!(subject_kind, "issue" | "mission") {
        bail!(
            "Unsupported subject_kind '{}' in {}; expected issue or mission",
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
            attempt: None,
            pr_attribution: None,
            body: "Line one\n\nLine two".to_string(),
        }
    }

    #[test]
    fn timestamp_activity_id_uses_utc_microseconds() {
        assert_eq!(timestamp_activity_id(at()), "20260610T181920123456Z");
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
    fn attempt_and_pr_metadata_round_trip() {
        let mut activity = activity();
        activity.event_type = ActivityEventType::WorkStarted;
        activity.attempt = Some(ActivityAttemptMetadata {
            role: ActivityAttemptRole::Worker,
            serial: 1,
            lifecycle: ActivityAttemptLifecycle::Started,
            agent: Some("codex".to_string()),
            subskill: Some("implement".to_string()),
        });
        activity.pr_attribution = Some(ActivityPrAttribution {
            action: "comment".to_string(),
            pull_request: Some("forgejo/example#42".to_string()),
            remote_author: Some("reviewer-user".to_string()),
        });

        let rendered = activity.to_markdown().unwrap();

        assert!(rendered.contains("attempt:\n"));
        assert!(rendered.contains("  role: worker"));
        assert!(rendered.contains("  serial: 1"));
        assert!(rendered.contains("  lifecycle: started"));
        assert!(rendered.contains("pr_attribution:\n"));
        assert!(rendered.contains("  action: comment"));

        let parsed = IssueActivity::from_markdown(
            &rendered,
            &issue_activity_path(&activity.subject_id, &activity.id),
        )
        .unwrap();
        assert_eq!(parsed, activity);
    }

    #[test]
    fn issue_attempts_are_derived_by_issue_role_and_serial() {
        let first_started = IssueActivity {
            id: "20260610T181920123456Z".to_string(),
            subject_kind: "issue".to_string(),
            subject_id: "atelier-one".to_string(),
            event_type: ActivityEventType::WorkStarted,
            actor: "worker-a".to_string(),
            created_at: at(),
            summary: "Started work".to_string(),
            attempt: Some(ActivityAttemptMetadata {
                role: ActivityAttemptRole::Worker,
                serial: 1,
                lifecycle: ActivityAttemptLifecycle::Started,
                agent: Some("codex".to_string()),
                subskill: Some("implement".to_string()),
            }),
            pr_attribution: None,
            body: String::new(),
        };
        let first_finished = IssueActivity {
            id: "20260610T181921123456Z".to_string(),
            created_at: at() + chrono::Duration::seconds(1),
            summary: "Finished work".to_string(),
            attempt: Some(ActivityAttemptMetadata {
                lifecycle: ActivityAttemptLifecycle::Finished,
                ..first_started.attempt.clone().unwrap()
            }),
            ..first_started.clone()
        };
        let reviewer_started = IssueActivity {
            id: "20260610T181922123456Z".to_string(),
            created_at: at() + chrono::Duration::seconds(2),
            actor: "reviewer-a".to_string(),
            summary: "Started review".to_string(),
            attempt: Some(ActivityAttemptMetadata {
                role: ActivityAttemptRole::Reviewer,
                serial: 1,
                lifecycle: ActivityAttemptLifecycle::Started,
                agent: None,
                subskill: Some("review".to_string()),
            }),
            ..first_started.clone()
        };
        let second_worker_started = IssueActivity {
            id: "20260610T181923123456Z".to_string(),
            created_at: at() + chrono::Duration::seconds(3),
            summary: "Started follow-up work".to_string(),
            attempt: Some(ActivityAttemptMetadata {
                role: ActivityAttemptRole::Worker,
                serial: 2,
                lifecycle: ActivityAttemptLifecycle::Started,
                agent: None,
                subskill: Some("implement".to_string()),
            }),
            ..first_started.clone()
        };

        let attempts = derive_issue_attempts([
            reviewer_started,
            first_finished,
            second_worker_started,
            first_started,
        ])
        .unwrap();

        assert_eq!(attempts.len(), 3);
        assert_eq!(attempts[0].id, "atelier-one/worker/1");
        assert_eq!(attempts[0].state, DerivedIssueAttemptState::Finished);
        assert_eq!(attempts[0].activity_ids.len(), 2);
        assert_eq!(attempts[1].id, "atelier-one/worker/2");
        assert_eq!(attempts[1].state, DerivedIssueAttemptState::Active);
        assert_eq!(attempts[2].id, "atelier-one/reviewer/1");
        assert_eq!(attempts[2].state, DerivedIssueAttemptState::Active);
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
