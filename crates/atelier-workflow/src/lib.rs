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

/// Minimal workflow status category vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusCategory {
    Todo,
    Active,
    Done,
    Blocked,
}

pub fn status_category(status: &str) -> StatusCategory {
    match status {
        "done" | "archived" => StatusCategory::Done,
        "blocked" => StatusCategory::Blocked,
        "in_progress" | "review" | "validation" => StatusCategory::Active,
        _ => StatusCategory::Todo,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_name_rejects_empty_values() {
        assert_eq!(TransitionName::new(" "), None);
    }

    #[test]
    fn transition_name_keeps_text() {
        assert_eq!(TransitionName::new("start").unwrap().as_str(), "start");
    }

    #[test]
    fn status_categories_match_workflow_groups() {
        assert_eq!(status_category("todo"), StatusCategory::Todo);
        assert_eq!(status_category("in_progress"), StatusCategory::Active);
        assert_eq!(status_category("validation"), StatusCategory::Active);
        assert_eq!(status_category("blocked"), StatusCategory::Blocked);
        assert_eq!(status_category("done"), StatusCategory::Done);
    }
}
