//! CLI shell contracts for Atelier.

pub const CRATE_NAME: &str = "atelier-cli";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliLayer;

pub fn app_crate_name() -> &'static str {
    atelier_app::CRATE_NAME
}
