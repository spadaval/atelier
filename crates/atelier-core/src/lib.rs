//! Core Atelier domain vocabulary.
//!
//! This crate is intentionally free of filesystem, SQLite, Clap, and telemetry
//! dependencies. Concrete domain types move here as the migration advances.

/// Stable identifier for a canonical Atelier record.
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RecordId(String);

impl RecordId {
    /// Creates a record identifier after checking the shared non-empty rule.
    pub fn new(value: impl Into<String>) -> Result<Self, RecordIdError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(RecordIdError::Empty);
        }
        Ok(Self(value))
    }

    /// Borrows the identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Record identifier validation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordIdError {
    Empty,
}

impl std::fmt::Display for RecordIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "record id cannot be empty"),
        }
    }
}

impl std::error::Error for RecordIdError {}
