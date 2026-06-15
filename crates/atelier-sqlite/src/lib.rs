//! SQLite projection and runtime contracts for Atelier.

pub const CRATE_NAME: &str = "atelier-sqlite";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SqliteLayer;

pub fn dependency_crate_names() -> [&'static str; 3] {
    [
        atelier_core::crate_name(),
        atelier_records::CRATE_NAME,
        atelier_workflow::CRATE_NAME,
    ]
}
