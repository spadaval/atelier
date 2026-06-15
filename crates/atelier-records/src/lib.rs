//! Canonical Markdown record contracts for Atelier.

pub mod activity;
pub mod record_kinds;
pub mod relationships;
pub mod store;

pub const CRATE_NAME: &str = "atelier-records";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordsLayer;

pub fn dependency_crate_names() -> [&'static str; 2] {
    [atelier_core::crate_name(), atelier_workflow::CRATE_NAME]
}

pub use activity::*;
pub use record_kinds::*;
pub use relationships::*;
pub use store::*;
