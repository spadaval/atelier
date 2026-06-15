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

/// Typed relationship between canonical records.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RecordRelation {
    pub kind: String,
    pub id: RecordId,
    pub role: String,
}

impl RecordRelation {
    pub fn new(
        kind: impl Into<String>,
        id: RecordId,
        role: impl Into<String>,
    ) -> Result<Self, ValueError> {
        let kind = kind.into();
        let role = role.into();
        validate_non_empty("kind", &kind)?;
        validate_non_empty("role", &role)?;
        Ok(Self { kind, id, role })
    }
}

/// Shared value validation failure for pure domain values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValueError {
    field: &'static str,
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} cannot be empty", self.field)
    }
}

impl std::error::Error for ValueError {}

pub fn validate_non_empty(field: &'static str, value: &str) -> Result<(), ValueError> {
    if value.trim().is_empty() {
        return Err(ValueError { field });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_id_rejects_empty_values() {
        assert_eq!(RecordId::new("  ").unwrap_err(), RecordIdError::Empty);
    }

    #[test]
    fn record_id_preserves_valid_text() {
        let id = RecordId::new("atelier-abcd").unwrap();
        assert_eq!(id.as_str(), "atelier-abcd");
    }

    #[test]
    fn relation_requires_kind_and_role() {
        let id = RecordId::new("atelier-abcd").unwrap();
        assert!(RecordRelation::new("", id.clone(), "blocks").is_err());
        assert!(RecordRelation::new("issue", id, " ").is_err());
    }
}
