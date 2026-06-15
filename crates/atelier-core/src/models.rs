use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type IssueId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relation {
    pub issue_id_1: IssueId,
    pub issue_id_2: IssueId,
    pub relation_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanRevision {
    pub revision: i64,
    pub reason: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanRecordData {
    pub revision: i64,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub revisions: Vec<PlanRevision>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MilestoneRecordData {
    #[serde(default)]
    pub desired_state: String,
    #[serde(default)]
    pub scope: Vec<String>,
    #[serde(default)]
    pub validation_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidenceStreamSummary {
    pub summary: String,
    pub bytes: usize,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidenceOutputSummary {
    pub limit_bytes_per_stream: usize,
    pub stdout: EvidenceStreamSummary,
    pub stderr: EvidenceStreamSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidenceTarget {
    pub kind: String,
    pub id: String,
    #[serde(default = "default_evidence_target_role")]
    pub role: String,
}

fn default_evidence_target_role() -> String {
    "validates".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordLink {
    pub source_kind: String,
    pub source_id: String,
    pub target_kind: String,
    pub target_id: String,
    pub relation_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkAssociation {
    pub issue_id: String,
    pub status: String,
    pub branch: Option<String>,
    pub worktree_path: Option<String>,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_serialization_preserves_domain_values() {
        let now = Utc::now();
        let issue = Issue {
            id: "atelier-0001".to_string(),
            title: "Test issue".to_string(),
            description: Some("A description".to_string()),
            status: "todo".to_string(),
            issue_type: "task".to_string(),
            priority: "high".to_string(),
            parent_id: Some("atelier-root".to_string()),
            created_at: now,
            updated_at: now,
            closed_at: None,
        };

        let json = serde_json::to_string(&issue).unwrap();
        let deserialized: Issue = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, issue);
    }

    #[test]
    fn evidence_target_defaults_to_validates_role() {
        let target: EvidenceTarget =
            serde_json::from_str(r#"{"kind":"issue","id":"atelier-0001"}"#).unwrap();

        assert_eq!(target.role, "validates");
    }
}
