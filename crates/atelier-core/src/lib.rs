//! Core domain contracts for Atelier.

pub const CRATE_NAME: &str = "atelier-core";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoreLayer;

pub fn crate_name() -> &'static str {
    CRATE_NAME
}
