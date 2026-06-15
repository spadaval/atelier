//! Maintenance operator jobs exposed by the application layer.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaintenanceJob {
    Delete(DeleteRecordRequest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteRecordRequest {
    pub target_kind: String,
    pub target_id: String,
    pub force: bool,
}

impl MaintenanceJob {
    pub fn command_group(&self) -> &'static str {
        "maintenance"
    }
}
