//! Core Atelier domain vocabulary.
//!
//! This crate is intentionally free of filesystem and Clap
//! dependencies. Concrete domain types move here as the migration advances.

use chrono::{DateTime, NaiveDate, Utc};
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Component, Path};

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

pub const ISSUE_STRATEGY_FIELD: &str = "strategy";

/// A mission's governing strategy document at the revision used to commission it.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IssueStrategy {
    pub path: String,
    pub revision: u64,
}

impl IssueStrategy {
    pub fn new(path: impl Into<String>, revision: u64) -> Result<Self, IssueFieldError> {
        let path = path.into();
        let parsed = Path::new(&path);
        let path_is_repo_relative = !path.trim().is_empty()
            && !parsed.is_absolute()
            && parsed
                .components()
                .all(|component| matches!(component, Component::Normal(_) | Component::CurDir));
        if !path_is_repo_relative {
            return Err(IssueFieldError::InvalidStrategy(
                "path must be a non-empty repository-relative path without parent traversal"
                    .to_string(),
            ));
        }
        if revision == 0 {
            return Err(IssueFieldError::InvalidStrategy(
                "revision must be a positive integer".to_string(),
            ));
        }
        Ok(Self { path, revision })
    }

    pub fn from_value(value: &serde_json::Value) -> Result<Self, IssueFieldError> {
        let object = value.as_object().ok_or_else(|| {
            IssueFieldError::InvalidStrategy("strategy must be an object".to_string())
        })?;
        if object.len() != 2 || !object.contains_key("path") || !object.contains_key("revision") {
            return Err(IssueFieldError::InvalidStrategy(
                "strategy must contain exactly path and revision".to_string(),
            ));
        }
        let path = object
            .get("path")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| IssueFieldError::InvalidStrategy("path must be a string".to_string()))?;
        let revision = object
            .get("revision")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| {
                IssueFieldError::InvalidStrategy("revision must be a positive integer".to_string())
            })?;
        Self::new(path, revision)
    }

    pub fn to_value(&self) -> serde_json::Value {
        serde_json::json!({ "path": self.path, "revision": self.revision })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IssueFieldError {
    InvalidStrategy(String),
}

impl fmt::Display for IssueFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStrategy(detail) => write!(f, "invalid issue strategy field: {detail}"),
        }
    }
}

impl std::error::Error for IssueFieldError {}

impl Issue {
    pub fn strategy(&self) -> Result<Option<IssueStrategy>, IssueFieldError> {
        self.fields
            .get(ISSUE_STRATEGY_FIELD)
            .map(IssueStrategy::from_value)
            .transpose()
    }

    pub fn set_strategy(&mut self, strategy: IssueStrategy) {
        self.fields
            .insert(ISSUE_STRATEGY_FIELD.to_string(), strategy.to_value());
    }
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
    Evidence(EvidenceRecord),
}

impl Record {
    pub fn header(&self) -> &RecordHeader {
        match self {
            Record::Issue(record) => &record.header,
            Record::Evidence(record) => &record.header,
        }
    }

    pub fn header_mut(&mut self) -> &mut RecordHeader {
        match self {
            Record::Issue(record) => &mut record.header,
            Record::Evidence(record) => &mut record.header,
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
pub struct EvidenceRecord {
    pub header: RecordHeader,
    pub data: EvidenceRecordData,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IssueSections {
    pub description: String,
    pub outcome: String,
    pub evidence: String,
    pub notes: Option<String>,
    pub close_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IssueSectionName {
    Description,
    Outcome,
    Evidence,
    Notes,
    CloseReason,
}

/// Note kinds accepted by `IssueSections::append_note`.
pub const ISSUE_NOTE_KINDS: &[&str] = &[
    "note",
    "plan",
    "observation",
    "blocker",
    "resolution",
    "result",
    "handoff",
    "human",
];

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IssueSectionState {
    pub name: IssueSectionName,
    pub required: bool,
    pub present: bool,
    pub empty: bool,
}

impl IssueSections {
    pub const REQUIRED_NAMES: [IssueSectionName; 2] =
        [IssueSectionName::Description, IssueSectionName::Outcome];

    pub const ALL_NAMES: [IssueSectionName; 5] = [
        IssueSectionName::Description,
        IssueSectionName::Outcome,
        IssueSectionName::Evidence,
        IssueSectionName::Notes,
        IssueSectionName::CloseReason,
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
            evidence: String::new(),
            notes: None,
            close_reason: None,
        }
    }

    /// Append one dated `- YYYY-MM-DD [kind] text` bullet to the Notes
    /// section, creating the section when absent. Continuation lines are
    /// indented so note text cannot open a new record section.
    pub fn append_note(&mut self, date: NaiveDate, kind: &str, text: &str) {
        let mut lines = text.trim().lines();
        let mut bullet = format!(
            "- {} [{kind}] {}",
            date.format("%Y-%m-%d"),
            lines.next().unwrap_or_default().trim_end()
        );
        for line in lines {
            bullet.push('\n');
            if !line.trim().is_empty() {
                bullet.push_str("  ");
                bullet.push_str(line.trim_end());
            }
        }
        self.notes = Some(match self.notes.as_deref().map(str::trim) {
            Some(existing) if !existing.is_empty() => format!("{existing}\n{bullet}"),
            _ => bullet,
        });
    }

    pub fn section(&self, name: IssueSectionName) -> Option<&str> {
        match name {
            IssueSectionName::Description => Some(&self.description),
            IssueSectionName::Outcome => Some(&self.outcome),
            IssueSectionName::Evidence => {
                (!self.evidence.trim().is_empty()).then_some(&self.evidence)
            }
            IssueSectionName::Notes => self.notes.as_deref(),
            IssueSectionName::CloseReason => self.close_reason.as_deref(),
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
    let mut close_reason = String::new();

    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            current = match heading.trim() {
                "Description" => Some("description"),
                "Outcome" => Some("outcome"),
                "Evidence" => Some("evidence"),
                "Notes" => Some("notes"),
                "Close Reason" => Some("close_reason"),
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
                "Close Reason" => "close_reason",
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
            "close_reason" => {
                close_reason.push_str(line);
                close_reason.push('\n');
            }
            _ => {}
        }
    }

    let description = description.trim().to_string();
    let outcome = outcome.trim().to_string();
    let evidence = evidence.trim().to_string();
    if description.is_empty() || outcome.is_empty() {
        return None;
    }
    Some(IssueSections {
        description,
        outcome,
        evidence,
        notes: (!notes.trim().is_empty()).then(|| notes.trim().to_string()),
        close_reason: (!close_reason.trim().is_empty()).then(|| close_reason.trim().to_string()),
    })
}

impl IssueSectionName {
    pub fn title(self) -> &'static str {
        match self {
            IssueSectionName::Description => "Description",
            IssueSectionName::Outcome => "Outcome",
            IssueSectionName::Evidence => "Evidence",
            IssueSectionName::Notes => "Notes",
            IssueSectionName::CloseReason => "Close Reason",
        }
    }

    pub fn required(self) -> bool {
        matches!(
            self,
            IssueSectionName::Description | IssueSectionName::Outcome
        )
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
