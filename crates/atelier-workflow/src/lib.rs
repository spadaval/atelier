//! Workflow policy boundaries for Atelier.
//!
//! Policy parsing and transition evaluation move here from the root crate during
//! the crate migration.

pub use atelier_core::RecordId;

/// Minimal transition descriptor used by early extraction tests.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TransitionName(String);

impl TransitionName {
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        (!value.trim().is_empty()).then_some(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
