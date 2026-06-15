//! Canonical Markdown record contracts for Atelier.

pub const CRATE_NAME: &str = "atelier-records";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordsLayer;

pub fn dependency_crate_names() -> [&'static str; 2] {
    [atelier_core::crate_name(), atelier_workflow::CRATE_NAME]
}
