//! Issue operator jobs exposed by the application layer.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IssueJob {
    Create(CreateIssueRequest),
    List(ListIssuesRequest),
    Show(ShowIssueRequest),
    Transition(TransitionIssueRequest),
    Update(UpdateIssueRequest),
    Note(NoteIssueRequest),
    Close(CloseIssueRequest),
    Block(BlockIssueRequest),
    Unblock(BlockIssueRequest),
    Blocked(BlockedIssuesRequest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateIssueRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub template: Option<String>,
    pub labels: Vec<String>,
    pub issue_type: Option<String>,
    pub parent: Option<String>,
    pub quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListIssuesRequest {
    pub status: String,
    pub category: Option<String>,
    pub label: Option<String>,
    pub priority: Option<String>,
    pub ready: bool,
    pub blocked: bool,
    pub quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowIssueRequest {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionIssueRequest {
    pub id: String,
    pub transition: Option<String>,
    pub options: bool,
    pub close_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateIssueRequest {
    pub id: String,
    pub title: Option<String>,
    pub priority: Option<String>,
    pub issue_type: Option<String>,
    pub labels: Vec<String>,
    pub remove_labels: Vec<String>,
    pub parent: Option<String>,
    pub no_parent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteIssueRequest {
    pub id: String,
    pub text: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseIssueRequest {
    pub id: String,
    pub to: Option<String>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockIssueRequest {
    pub id: String,
    pub blocker: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockedIssuesRequest {
    pub id: Option<String>,
}

impl IssueJob {
    pub fn command_group(&self) -> &'static str {
        "issue"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_jobs_are_plain_app_requests_not_cli_args() {
        let job = IssueJob::Create(CreateIssueRequest {
            title: "Move command surface".to_string(),
            description: None,
            priority: "high".to_string(),
            template: None,
            labels: vec!["cli".to_string()],
            issue_type: Some("task".to_string()),
            parent: Some("atelier-parent".to_string()),
            quiet: false,
        });

        assert_eq!(job.command_group(), "issue");
        assert!(format!("{job:?}").contains("Move command surface"));
    }
}
