//! Application use-case contracts for Atelier.

pub mod status;

pub const CRATE_NAME: &str = "atelier-app";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppLayer;

pub fn dependency_crate_names() -> [&'static str; 4] {
    [
        atelier_core::crate_name(),
        atelier_records::CRATE_NAME,
        atelier_sqlite::CRATE_NAME,
        atelier_workflow::CRATE_NAME,
    ]
}
