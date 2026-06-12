use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, SecondsFormat, Timelike, Utc};
use serde::Deserialize;
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
    pub subject_id: String,
    pub event_type: ActivityEventType,
    pub actor: String,
    pub created_at: DateTime<Utc>,
    pub summary: String,
    pub body: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ActivityEventType {
    Comment,
    Note,
    Handoff,
    Decision,
    Plan,
    CloseReason,
    StatusChanged,
    FieldChanged,
    WorkStarted,
    WorkFinished,
    EvidenceAttached,
}

impl ActivityEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Comment => "comment",
            Self::Note => "note",
            Self::Handoff => "handoff",
            Self::Decision => "decision",
            Self::Plan => "plan",
            Self::CloseReason => "close_reason",
            Self::StatusChanged => "status_changed",
            Self::FieldChanged => "field_changed",
            Self::WorkStarted => "work_started",
            Self::WorkFinished => "work_finished",
            Self::EvidenceAttached => "evidence_attached",
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
            "decision" => Ok(Self::Decision),
            "plan" => Ok(Self::Plan),
            "close_reason" => Ok(Self::CloseReason),
            "status_changed" => Ok(Self::StatusChanged),
            "field_changed" => Ok(Self::FieldChanged),
            "work_started" => Ok(Self::WorkStarted),
            "work_finished" => Ok(Self::WorkFinished),
            "evidence_attached" => Ok(Self::EvidenceAttached),
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
}

pub fn issue_activity_dir(issue_id: &str) -> PathBuf {
    PathBuf::from("issues").join(format!("{issue_id}.activity"))
}

pub fn issue_activity_path(issue_id: &str, activity_id: &str) -> PathBuf {
    issue_activity_dir(issue_id).join(format!("{activity_id}.md"))
}

pub fn list_issue_activities(state_dir: &Path, issue_id: &str) -> Result<Vec<IssueActivity>> {
    let dir = state_dir.join(issue_activity_dir(issue_id));
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
        let relative = issue_activity_dir(issue_id).join(file_name);
        activities.push(IssueActivity::load(state_dir, &relative)?);
    }
    activities.sort_by(|a, b| a.created_at.cmp(&b.created_at).then(a.id.cmp(&b.id)));
    Ok(activities)
}

pub fn list_all_issue_activities(state_dir: &Path) -> Result<Vec<IssueActivity>> {
    let issue_dir = state_dir.join("issues");
    if !issue_dir.exists() {
        return Ok(Vec::new());
    }
    let mut activities = Vec::new();
    for entry in fs::read_dir(&issue_dir)
        .with_context(|| format!("Failed to read {}", issue_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(issue_id) = name.strip_suffix(".activity") else {
            continue;
        };
        activities.extend(list_issue_activities(state_dir, issue_id)?);
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
            .join(issue_activity_path(subject_id, &candidate))
            .exists()
        {
            return Ok(candidate);
        }
    }
    bail!("No available activity id for issue {subject_id} at {base}")
}

pub fn write_issue_activity(state_dir: &Path, activity: &IssueActivity) -> Result<PathBuf> {
    let relative = issue_activity_path(&activity.subject_id, &activity.id);
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
    let id = allocate_activity_id(state_dir, subject_id, created_at)?;
    let activity = IssueActivity {
        id,
        subject_id: subject_id.to_string(),
        event_type,
        actor: actor.to_string(),
        created_at,
        summary: summary.to_string(),
        body: normalize_body(body),
    };
    write_issue_activity(state_dir, &activity)?;
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
        if front.subject_kind != "issue" {
            bail!(
                "Unsupported subject_kind '{}' in {}; expected issue",
                front.subject_kind,
                display_state_path(relative)
            );
        }
        let expected = issue_activity_path(&front.subject_id, &front.id);
        if relative != expected {
            bail!(
                "Activity id {} for issue {} in {} does not match canonical path {}",
                front.id,
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
            subject_id: front.subject_id,
            event_type,
            actor: front.actor,
            created_at: front.created_at,
            summary: front.summary,
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
        write_yaml_scalar(&mut output, "subject_kind", "issue")?;
        write_yaml_scalar(&mut output, "subject_id", &self.subject_id)?;
        write_yaml_scalar(&mut output, "event_type", self.event_type.as_str())?;
        write_yaml_scalar(&mut output, "actor", &self.actor)?;
        write_yaml_scalar(
            &mut output,
            "created_at",
            &self.created_at.to_rfc3339_opts(SecondsFormat::Micros, true),
        )?;
        write_yaml_scalar(&mut output, "summary", &self.summary)?;
        output.push_str("---\n\n");
        output.push_str(&normalize_body(&self.body));
        output.push('\n');
        Ok(output)
    }
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
            subject_id: "atelier-qxvj".to_string(),
            event_type: ActivityEventType::Handoff,
            actor: "agent@example.com".to_string(),
            created_at: at(),
            summary: "Implemented activity sidecars".to_string(),
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
            allocate_activity_id(state_dir, issue_id, at()).unwrap(),
            format!("{base}-01")
        );
        fs::write(
            state_dir.join(issue_activity_path(issue_id, &format!("{base}-01"))),
            "existing",
        )
        .unwrap();
        assert_eq!(
            allocate_activity_id(state_dir, issue_id, at()).unwrap(),
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
