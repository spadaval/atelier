//! Core domain contracts for Atelier.

pub mod models;
pub mod record_id;
pub mod relationships;

pub const CRATE_NAME: &str = "atelier-core";

pub use models::*;
pub use record_id::{
    base36_padded, legacy_issue_id, validate_record_id, DEFAULT_PROJECT_SLUG, DEFAULT_SUFFIX_LEN,
};
pub use relationships::*;

pub fn crate_name() -> &'static str {
    CRATE_NAME
}
