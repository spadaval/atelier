//! Workflow policy contracts for Atelier.

pub const CRATE_NAME: &str = "atelier-workflow";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkflowLayer;

pub fn core_crate_name() -> &'static str {
    atelier_core::crate_name()
}
