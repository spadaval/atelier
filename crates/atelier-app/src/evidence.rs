//! Evidence operator jobs exposed by the application layer.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceJob {
    Record(RecordEvidenceRequest),
    Show { id: String },
    Attach(AttachEvidenceRequest),
    List { result: Option<String> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordEvidenceRequest {
    pub target: Option<String>,
    pub role: String,
    pub kind: String,
    pub result: String,
    pub summary_text: Option<String>,
    pub command: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachEvidenceRequest {
    pub evidence_id: String,
    pub target_kind: String,
    pub target_id: String,
    pub role: String,
}

impl EvidenceJob {
    pub fn command_group(&self) -> &'static str {
        "evidence"
    }
}
