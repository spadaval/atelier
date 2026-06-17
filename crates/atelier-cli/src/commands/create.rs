use atelier_core::IssuePriority;

/// Built-in issue templates
pub struct Template {
    pub name: &'static str,
    pub priority: &'static str,
    pub label: &'static str,
    pub description_prefix: Option<&'static str>,
}

pub const TEMPLATES: &[Template] = &[
    Template {
        name: "bug",
        priority: "high",
        label: "bug",
        description_prefix: Some("Steps to reproduce:\n1. \n\nExpected: \nActual: "),
    },
    Template {
        name: "feature",
        priority: "medium",
        label: "feature",
        description_prefix: Some("Goal: \n\nAcceptance criteria:\n- "),
    },
    Template {
        name: "refactor",
        priority: "low",
        label: "refactor",
        description_prefix: Some("Current state: \n\nDesired state: \n\nReason: "),
    },
    Template {
        name: "research",
        priority: "low",
        label: "research",
        description_prefix: Some("Question: \n\nContext: \n\nFindings: "),
    },
    Template {
        name: "audit",
        priority: "high",
        label: "audit",
        description_prefix: Some("Scope: \n\nFiles to review: \n\nFindings: \n\nSeverity: "),
    },
    Template {
        name: "continuation",
        priority: "high",
        label: "continuation",
        description_prefix: Some("Previous session: \n\nCompleted: \n\nRemaining: \n\nBlockers: "),
    },
    Template {
        name: "investigation",
        priority: "medium",
        label: "investigation",
        description_prefix: Some(
            "Symptom: \n\nReproduction: \n\nHypotheses: \n\nRoot cause: \n\nFix: ",
        ),
    },
];

pub fn get_template(name: &str) -> Option<&'static Template> {
    TEMPLATES.iter().find(|t| t.name == name)
}

pub fn list_templates() -> Vec<&'static str> {
    TEMPLATES.iter().map(|t| t.name).collect()
}

pub fn validate_priority(priority: &str) -> bool {
    IssuePriority::from_cli_input(priority).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Unit Tests ====================

    #[test]
    fn test_validate_priority_valid() {
        assert!(validate_priority("low"));
        assert!(validate_priority("medium"));
        assert!(validate_priority("high"));
        assert!(validate_priority("critical"));
    }

    #[test]
    fn test_validate_priority_invalid() {
        assert!(!validate_priority(""));
        assert!(!validate_priority("urgent"));
        assert!(!validate_priority("LOW")); // Case sensitive
        assert!(!validate_priority("MEDIUM"));
        assert!(!validate_priority("High"));
        assert!(!validate_priority("CRITICAL"));
        assert!(!validate_priority(" medium"));
        assert!(!validate_priority("medium "));
        assert!(!validate_priority("medium\n"));
    }

    #[test]
    fn test_validate_priority_malicious() {
        // Security: ensure no injection vectors
        assert!(!validate_priority("'; DROP TABLE issues; --"));
        assert!(!validate_priority("high\0medium"));
        assert!(!validate_priority("medium; DELETE FROM issues"));
        assert!(!validate_priority("<script>alert('xss')</script>"));
    }

    #[test]
    fn test_get_template_exists() {
        let bug = get_template("bug");
        assert!(bug.is_some());
        let template = bug.unwrap();
        assert_eq!(template.name, "bug");
        assert_eq!(template.priority, "high");
        assert_eq!(template.label, "bug");
        assert!(template.description_prefix.is_some());
    }

    #[test]
    fn test_get_template_not_found() {
        assert!(get_template("nonexistent").is_none());
        assert!(get_template("").is_none());
        assert!(get_template("Bug").is_none()); // Case sensitive
        assert!(get_template("BUG").is_none());
    }

    #[test]
    fn test_list_templates() {
        let templates = list_templates();
        assert!(templates.contains(&"bug"));
        assert!(templates.contains(&"feature"));
        assert!(templates.contains(&"refactor"));
        assert!(templates.contains(&"research"));
        assert!(templates.contains(&"audit"));
        assert!(templates.contains(&"continuation"));
        assert!(templates.contains(&"investigation"));
        assert_eq!(templates.len(), 7);
    }

    #[test]
    fn test_template_fields() {
        // Verify all templates have required fields
        for template in TEMPLATES {
            assert!(!template.name.is_empty());
            assert!(validate_priority(template.priority));
            assert!(!template.label.is_empty());
        }
    }

    #[test]
    fn test_template_bug_description_prefix() {
        let template = get_template("bug").unwrap();
        let prefix = template.description_prefix.unwrap();
        assert!(prefix.contains("Steps to reproduce"));
        assert!(prefix.contains("Expected"));
        assert!(prefix.contains("Actual"));
    }

    #[test]
    fn test_template_feature_description_prefix() {
        let template = get_template("feature").unwrap();
        let prefix = template.description_prefix.unwrap();
        assert!(prefix.contains("Goal"));
        assert!(prefix.contains("Acceptance criteria"));
    }

    #[test]
    fn test_invalid_priorities_never_validate() {
        for priority in ["urgent", "minor", "blocker", "p0", "mediumish"] {
            assert!(!validate_priority(priority));
        }
    }

    #[test]
    fn test_unknown_template_returns_none() {
        for name in ["unknown", "bugs", "Feature", "roadmap"] {
            assert!(get_template(name).is_none());
        }
    }
}
