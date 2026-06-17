//! Issue record ownership.
//!
//! Issue records have issue-specific body sections and lifecycle fields, while
//! still participating in the shared canonical record-kind registry.

pub use crate::{issue_record_path, parse_issue_record, render_issue_record, CanonicalIssueRecord};
pub use atelier_core::{IssueRecord, IssueSectionName, IssueSectionState, IssueSections};
