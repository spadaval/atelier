//! Mission operator jobs exposed by the application layer.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MissionJob {
    Create(MissionCreateRequest),
    Show(RecordIdRequest),
    Start(MissionStartRequest),
    Status(MissionStatusRequest),
    Audit(RecordIdRequest),
    Close(MissionCloseRequest),
    List(MissionListRequest),
    Update(MissionUpdateRequest),
    Note(MissionNoteRequest),
    AddWork(MissionIssueRequest),
    Unlink(MissionIssueRequest),
    AddBlocker(MissionIssueRequest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordIdRequest {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionCreateRequest {
    pub title: String,
    pub body: Option<String>,
    pub constraints: Vec<String>,
    pub risks: Vec<String>,
    pub validation: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionStartRequest {
    pub id: String,
    pub switch_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionStatusRequest {
    pub id: Option<String>,
    pub quiet: bool,
    pub closeout: bool,
    pub verbose: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionCloseRequest {
    pub id: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionListRequest {
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionUpdateRequest {
    pub id: String,
    pub title: Option<String>,
    pub status: Option<String>,
    pub body: Option<String>,
    pub constraints: Vec<String>,
    pub risks: Vec<String>,
    pub validation: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionNoteRequest {
    pub id: String,
    pub text: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionIssueRequest {
    pub mission_id: String,
    pub issue_id: String,
}

impl MissionJob {
    pub fn command_group(&self) -> &'static str {
        "mission"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mission_status_request_is_independent_of_cli_parser_shape() {
        let job = MissionJob::Status(MissionStatusRequest {
            id: Some("atelier-v5nb".to_string()),
            quiet: true,
            closeout: false,
            verbose: true,
        });

        assert_eq!(job.command_group(), "mission");
    }
}
