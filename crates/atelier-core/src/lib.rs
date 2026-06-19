//! Core Atelier domain vocabulary.
//!
//! This crate is intentionally free of filesystem, SQLite, Clap, and telemetry
//! dependencies. Concrete domain types move here as the migration advances.

use chrono::{DateTime, Utc};
use std::collections::BTreeMap;
use std::fmt;

pub type IssueId = String;

pub const ISSUE_PRIORITY_LABELS: &[&str] = &["critical", "high", "medium", "low"];

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum IssuePriority {
    P0,
    P1,
    P2,
    P3,
}

impl IssuePriority {
    pub fn canonical_token(self) -> &'static str {
        match self {
            Self::P0 => "P0",
            Self::P1 => "P1",
            Self::P2 => "P2",
            Self::P3 => "P3",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::P0 => "critical",
            Self::P1 => "high",
            Self::P2 => "medium",
            Self::P3 => "low",
        }
    }

    pub fn sort_rank(self) -> u8 {
        match self {
            Self::P0 => 0,
            Self::P1 => 1,
            Self::P2 => 2,
            Self::P3 => 3,
        }
    }

    pub fn score_weight(self) -> i32 {
        match self {
            Self::P0 => 4,
            Self::P1 => 3,
            Self::P2 => 2,
            Self::P3 => 1,
        }
    }

    pub fn from_canonical_token(value: &str) -> Result<Self, ParseIssuePriorityError> {
        match value {
            "P0" => Ok(Self::P0),
            "P1" => Ok(Self::P1),
            "P2" => Ok(Self::P2),
            "P3" => Ok(Self::P3),
            other => Err(ParseIssuePriorityError::new(other)),
        }
    }

    pub fn from_label(value: &str) -> Result<Self, ParseIssuePriorityError> {
        match value {
            "critical" => Ok(Self::P0),
            "high" => Ok(Self::P1),
            "medium" => Ok(Self::P2),
            "low" => Ok(Self::P3),
            other => Err(ParseIssuePriorityError::new(other)),
        }
    }

    pub fn from_cli_input(value: &str) -> Result<Self, ParseIssuePriorityError> {
        Self::from_label(value)
    }

    pub fn from_beads_numeric(value: i64) -> Option<Self> {
        match value {
            0 => Some(Self::P0),
            1 => Some(Self::P1),
            2 => Some(Self::P2),
            3 | 4 => Some(Self::P3),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseIssuePriorityError {
    value: String,
}

impl ParseIssuePriorityError {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl fmt::Display for ParseIssuePriorityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid priority '{}'. Valid values: critical, high, medium, low",
            self.value
        )
    }
}

impl std::error::Error for ParseIssuePriorityError {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Issue {
    pub id: IssueId,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
    #[serde(default)]
    pub fields: BTreeMap<String, serde_json::Value>,
    pub parent_id: Option<IssueId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Comment {
    pub id: i64,
    pub issue_id: IssueId,
    pub content: String,
    pub created_at: DateTime<Utc>,
    #[serde(default = "default_comment_kind")]
    pub kind: String,
}

fn default_comment_kind() -> String {
    "note".to_string()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Session {
    pub id: i64,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub active_issue_id: Option<IssueId>,
    pub handoff_notes: Option<String>,
    pub last_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Relation {
    pub issue_id_1: IssueId,
    pub issue_id_2: IssueId,
    pub relation_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct RelationshipTarget {
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct AttachmentRelationship {
    pub kind: String,
    pub id: String,
    pub role: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct RelatesRelationship {
    pub kind: String,
    pub id: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Relationships {
    pub blocks: Vec<RelationshipTarget>,
    pub children: Vec<RelationshipTarget>,
    pub attachments: Vec<AttachmentRelationship>,
    pub relates: Vec<RelatesRelationship>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecordHeader {
    pub kind: String,
    pub id: String,
    pub title: String,
    pub status: String,
    pub labels: Vec<String>,
    pub relationships: Relationships,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Record {
    Issue(IssueRecord),
    Mission(MissionRecord),
    Evidence(EvidenceRecord),
    Session(SessionRecord),
    Review(ReviewRecord),
}

impl Record {
    pub fn header(&self) -> &RecordHeader {
        match self {
            Record::Issue(record) => &record.header,
            Record::Mission(record) => &record.header,
            Record::Evidence(record) => &record.header,
            Record::Session(record) => &record.header,
            Record::Review(record) => &record.header,
        }
    }

    pub fn header_mut(&mut self) -> &mut RecordHeader {
        match self {
            Record::Issue(record) => &mut record.header,
            Record::Mission(record) => &mut record.header,
            Record::Evidence(record) => &mut record.header,
            Record::Session(record) => &mut record.header,
            Record::Review(record) => &mut record.header,
        }
    }

    pub fn kind(&self) -> &str {
        &self.header().kind
    }

    pub fn id(&self) -> &str {
        &self.header().id
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IssueRecord {
    pub header: RecordHeader,
    pub issue_type: String,
    pub priority: String,
    #[serde(default)]
    pub fields: BTreeMap<String, serde_json::Value>,
    pub closed_at: Option<DateTime<Utc>>,
    pub sections: IssueSections,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReviewRecord {
    pub header: RecordHeader,
    pub mode: String,
    pub issue_id: String,
    pub source_branch: String,
    pub target_branch: String,
    pub events: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MissionRecord {
    pub header: RecordHeader,
    pub sections: MissionSections,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvidenceRecord {
    pub header: RecordHeader,
    pub data: EvidenceRecordData,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SessionRecord {
    pub header: RecordHeader,
    pub data: SessionRecordData,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SessionRecordData {
    pub agent_identity: Option<String>,
    pub role: String,
    pub subskill: Option<String>,
    pub target: Option<SessionTarget>,
    pub session_kind: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SessionTarget {
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IssueSections {
    pub description: String,
    pub outcome: String,
    pub evidence: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IssueSectionName {
    Description,
    Outcome,
    Evidence,
    Notes,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IssueSectionState {
    pub name: IssueSectionName,
    pub required: bool,
    pub present: bool,
    pub empty: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MissionSections {
    pub intent: String,
    pub constraints: String,
    pub risks: String,
    pub validation: String,
    pub terminal_notes: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MissionSectionName {
    Intent,
    Constraints,
    Risks,
    Validation,
    TerminalNotes,
    Notes,
}

impl IssueSections {
    pub const REQUIRED_NAMES: [IssueSectionName; 3] = [
        IssueSectionName::Description,
        IssueSectionName::Outcome,
        IssueSectionName::Evidence,
    ];

    pub const ALL_NAMES: [IssueSectionName; 4] = [
        IssueSectionName::Description,
        IssueSectionName::Outcome,
        IssueSectionName::Evidence,
        IssueSectionName::Notes,
    ];

    pub fn unchecked_from_body(body: Option<&str>) -> Self {
        if let Some(parsed) = body.and_then(parse_issue_sections_lenient) {
            return parsed;
        }
        let description = body
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("No description provided.");
        Self {
            description: description.to_string(),
            outcome: "Outcome was not specified.".to_string(),
            evidence: "Evidence was not specified.".to_string(),
            notes: None,
        }
    }

    pub fn section(&self, name: IssueSectionName) -> Option<&str> {
        match name {
            IssueSectionName::Description => Some(&self.description),
            IssueSectionName::Outcome => Some(&self.outcome),
            IssueSectionName::Evidence => Some(&self.evidence),
            IssueSectionName::Notes => self.notes.as_deref(),
        }
    }

    pub fn section_states(&self) -> Vec<IssueSectionState> {
        Self::ALL_NAMES
            .into_iter()
            .map(|name| {
                let value = self.section(name);
                IssueSectionState {
                    name,
                    required: name.required(),
                    present: value.is_some(),
                    empty: value.map(str::trim).is_none_or(str::is_empty),
                }
            })
            .collect()
    }

    pub fn searchable_text(&self) -> String {
        Self::ALL_NAMES
            .into_iter()
            .filter_map(|name| self.section(name))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn parse_issue_sections_lenient(body: &str) -> Option<IssueSections> {
    let mut current: Option<&str> = None;
    let mut description = String::new();
    let mut outcome = String::new();
    let mut evidence = String::new();
    let mut notes = String::new();

    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            current = match heading.trim() {
                "Description" => Some("description"),
                "Outcome" => Some("outcome"),
                "Evidence" => Some("evidence"),
                "Notes" => Some("notes"),
                _ => return None,
            };
            break;
        }
    }

    let mut current = current?;
    for line in body.lines().skip_while(|line| !line.starts_with("## ")) {
        if let Some(heading) = line.strip_prefix("## ") {
            current = match heading.trim() {
                "Description" => "description",
                "Outcome" => "outcome",
                "Evidence" => "evidence",
                "Notes" => "notes",
                _ => return None,
            };
            continue;
        }
        match current {
            "description" => {
                description.push_str(line);
                description.push('\n');
            }
            "outcome" => {
                outcome.push_str(line);
                outcome.push('\n');
            }
            "evidence" => {
                evidence.push_str(line);
                evidence.push('\n');
            }
            "notes" => {
                notes.push_str(line);
                notes.push('\n');
            }
            _ => {}
        }
    }

    let description = description.trim().to_string();
    let outcome = outcome.trim().to_string();
    let evidence = evidence.trim().to_string();
    if description.is_empty() || outcome.is_empty() || evidence.is_empty() {
        return None;
    }
    Some(IssueSections {
        description,
        outcome,
        evidence,
        notes: (!notes.trim().is_empty()).then(|| notes.trim().to_string()),
    })
}

impl IssueSectionName {
    pub fn title(self) -> &'static str {
        match self {
            IssueSectionName::Description => "Description",
            IssueSectionName::Outcome => "Outcome",
            IssueSectionName::Evidence => "Evidence",
            IssueSectionName::Notes => "Notes",
        }
    }

    pub fn required(self) -> bool {
        matches!(
            self,
            IssueSectionName::Description | IssueSectionName::Outcome | IssueSectionName::Evidence
        )
    }
}

impl MissionSections {
    pub const ALL_NAMES: [MissionSectionName; 6] = [
        MissionSectionName::Intent,
        MissionSectionName::Constraints,
        MissionSectionName::Risks,
        MissionSectionName::Validation,
        MissionSectionName::TerminalNotes,
        MissionSectionName::Notes,
    ];
}

impl MissionSectionName {
    pub fn title(self) -> &'static str {
        match self {
            MissionSectionName::Intent => "Intent",
            MissionSectionName::Constraints => "Constraints",
            MissionSectionName::Risks => "Risks",
            MissionSectionName::Validation => "Validation",
            MissionSectionName::TerminalNotes => "Terminal Notes",
            MissionSectionName::Notes => "Notes",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct EvidenceStreamSummary {
    pub summary: String,
    pub bytes: usize,
    pub truncated: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct EvidenceOutputSummary {
    pub limit_bytes_per_stream: usize,
    pub stdout: EvidenceStreamSummary,
    pub stderr: EvidenceStreamSummary,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct EvidenceTarget {
    pub kind: String,
    pub id: String,
    #[serde(default = "default_evidence_target_role")]
    pub role: String,
}

fn default_evidence_target_role() -> String {
    "validates".to_string()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct EvidenceRecordData {
    #[serde(alias = "kind")]
    pub evidence_type: String,
    pub captured_at: DateTime<Utc>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub producer: Option<String>,
    #[serde(default)]
    pub proof_scope: Option<String>,
    #[serde(default)]
    pub agent_identity: Option<String>,
    #[serde(default)]
    pub independence_level: Option<String>,
    #[serde(default)]
    pub residual_risks: Vec<String>,
    #[serde(default)]
    pub follow_up_ids: Vec<String>,
    #[serde(default)]
    pub exit_code: Option<i32>,
    #[serde(default)]
    pub exit_status: Option<String>,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub spawn_error: Option<String>,
    #[serde(default)]
    pub output: Option<EvidenceOutputSummary>,
    #[serde(default)]
    pub target: Option<EvidenceTarget>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct RecordLink {
    pub source_kind: String,
    pub source_id: String,
    pub target_kind: String,
    pub target_id: String,
    pub relation_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct WorkAssociation {
    pub issue_id: String,
    pub status: String,
    pub branch: Option<String>,
    pub worktree_path: Option<String>,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

/// Stable identifier for a canonical Atelier record.
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RecordId(String);

impl RecordId {
    /// Creates a record identifier after checking the shared non-empty rule.
    pub fn new(value: impl Into<String>) -> Result<Self, RecordIdError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(RecordIdError::Empty);
        }
        Ok(Self(value))
    }

    /// Borrows the identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Record identifier validation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordIdError {
    Empty,
}

impl std::fmt::Display for RecordIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "record id cannot be empty"),
        }
    }
}

impl std::error::Error for RecordIdError {}

/// Typed relationship between canonical records.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RecordRelation {
    pub kind: String,
    pub id: RecordId,
    pub role: String,
}

impl RecordRelation {
    pub fn new(
        kind: impl Into<String>,
        id: RecordId,
        role: impl Into<String>,
    ) -> Result<Self, ValueError> {
        let kind = kind.into();
        let role = role.into();
        validate_non_empty("kind", &kind)?;
        validate_non_empty("role", &role)?;
        Ok(Self { kind, id, role })
    }
}

/// Shared value validation failure for pure domain values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValueError {
    field: &'static str,
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} cannot be empty", self.field)
    }
}

impl std::error::Error for ValueError {}

pub fn validate_non_empty(field: &'static str, value: &str) -> Result<(), ValueError> {
    if value.trim().is_empty() {
        return Err(ValueError { field });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_id_rejects_empty_values() {
        assert_eq!(RecordId::new("  ").unwrap_err(), RecordIdError::Empty);
    }

    #[test]
    fn record_id_preserves_valid_text() {
        let id = RecordId::new("atelier-abcd").unwrap();
        assert_eq!(id.as_str(), "atelier-abcd");
    }

    #[test]
    fn relation_requires_kind_and_role() {
        let id = RecordId::new("atelier-abcd").unwrap();
        assert!(RecordRelation::new("", id.clone(), "blocks").is_err());
        assert!(RecordRelation::new("issue", id, " ").is_err());
    }

    #[test]
    fn issue_priority_maps_tokens_labels_and_ranks() {
        let cases = [
            (IssuePriority::P0, "P0", "critical", 0, 4),
            (IssuePriority::P1, "P1", "high", 1, 3),
            (IssuePriority::P2, "P2", "medium", 2, 2),
            (IssuePriority::P3, "P3", "low", 3, 1),
        ];

        for (priority, token, label, rank, weight) in cases {
            assert_eq!(priority.canonical_token(), token);
            assert_eq!(priority.label(), label);
            assert_eq!(priority.sort_rank(), rank);
            assert_eq!(priority.score_weight(), weight);
            assert_eq!(
                IssuePriority::from_canonical_token(token).unwrap(),
                priority
            );
            assert_eq!(IssuePriority::from_label(label).unwrap(), priority);
            assert_eq!(IssuePriority::from_cli_input(label).unwrap(), priority);
        }
    }

    #[test]
    fn issue_priority_maps_beads_numeric_values() {
        assert_eq!(
            IssuePriority::from_beads_numeric(0),
            Some(IssuePriority::P0)
        );
        assert_eq!(
            IssuePriority::from_beads_numeric(1),
            Some(IssuePriority::P1)
        );
        assert_eq!(
            IssuePriority::from_beads_numeric(2),
            Some(IssuePriority::P2)
        );
        assert_eq!(
            IssuePriority::from_beads_numeric(3),
            Some(IssuePriority::P3)
        );
        assert_eq!(
            IssuePriority::from_beads_numeric(4),
            Some(IssuePriority::P3)
        );
        assert_eq!(IssuePriority::from_beads_numeric(5), None);
    }

    #[test]
    fn issue_priority_rejects_wrong_vocabulary_for_context() {
        let label_error = IssuePriority::from_label("P1").unwrap_err();
        assert_eq!(
            label_error.to_string(),
            "Invalid priority 'P1'. Valid values: critical, high, medium, low"
        );

        let token_error = IssuePriority::from_canonical_token("high").unwrap_err();
        assert_eq!(
            token_error.to_string(),
            "Invalid priority 'high'. Valid values: critical, high, medium, low"
        );
    }
}
