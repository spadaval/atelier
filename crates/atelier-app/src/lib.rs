//! Application use-case contracts for Atelier.

pub mod evidence;
pub mod graph;
pub mod issue;
pub mod maintenance;
pub mod mission;
pub mod status;
pub mod workflow;

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
