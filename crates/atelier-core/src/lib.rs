//! Core Atelier domain vocabulary.
//!
//! This crate is intentionally free of filesystem, SQLite, Clap, and telemetry
//! dependencies. Concrete domain types move here as the migration advances.

use chrono::{DateTime, Utc};

pub type IssueId = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Issue {
    pub id: IssueId,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct DomainRecord {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub status: String,
    pub body: Option<String>,
    pub data_json: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PlanRevision {
    pub revision: i64,
    pub reason: String,
    pub body: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PlanRecordData {
    pub revision: i64,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub revisions: Vec<PlanRevision>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct MilestoneRecordData {
    #[serde(default)]
    pub desired_state: String,
    #[serde(default)]
    pub scope: Vec<String>,
    #[serde(default)]
    pub validation_criteria: Vec<String>,
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
}
