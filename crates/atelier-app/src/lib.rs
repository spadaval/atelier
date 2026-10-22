//! Application use-case boundary.
//!
//! Command orchestration uses typed request and view APIs
//! that do not write directly to stdout or stderr.

pub mod base_policy;
pub mod committed_snapshot;
pub mod criteria;
pub mod decisions;
pub mod epic_migration;
pub mod evidence_freshness;
pub mod git_freshness;
pub mod init;
pub mod issue_inventory;
pub mod issue_read;
pub mod lint;
pub mod mission_overview;
pub mod mission_readiness;
pub mod mission_review;
pub mod objective_graph;
pub(crate) mod path_log;
pub mod project_config;
pub mod record_history;
pub mod record_index;
pub mod record_ref;
pub mod review_gate;
pub mod storage_layout;
pub mod use_cases;
pub mod work_classification;
pub mod workflow_policy;
pub mod workflow_validation;

#[cfg(test)]
pub(crate) mod test_records;
