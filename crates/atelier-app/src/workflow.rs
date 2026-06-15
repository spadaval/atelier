//! Workflow operator jobs exposed by the application layer.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowJob {
    Check,
}

impl WorkflowJob {
    pub fn command_group(&self) -> &'static str {
        "workflow"
    }
}
