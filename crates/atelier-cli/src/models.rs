pub use atelier_core::{
    EvidenceOutputSummary, EvidenceRecordData, EvidenceStreamSummary, EvidenceTarget, Issue,
    IssueId, RecordLink, Relation, WorkAssociation,
};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use proptest::prelude::*;

    // ==================== Issue Tests ====================

    #[test]
    fn test_issue_serialization_json() {
        let issue = Issue {
            id: "atelier-0001".to_string(),
            title: "Test issue".to_string(),
            description: Some("A description".to_string()),
            status: "todo".to_string(),
            issue_type: "task".to_string(),
            priority: "high".to_string(),
            fields: Default::default(),
            parent_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
        };

        let json = serde_json::to_string(&issue).unwrap();
        let deserialized: Issue = serde_json::from_str(&json).unwrap();

        assert_eq!(issue.id, deserialized.id);
        assert_eq!(issue.title, deserialized.title);
        assert_eq!(issue.description, deserialized.description);
        assert_eq!(issue.status, deserialized.status);
        assert_eq!(issue.priority, deserialized.priority);
        assert_eq!(issue.parent_id, deserialized.parent_id);
    }

    #[test]
    fn test_issue_with_parent() {
        let issue = Issue {
            id: "atelier-0002".to_string(),
            title: "Child issue".to_string(),
            description: None,
            status: "todo".to_string(),
            issue_type: "task".to_string(),
            priority: "medium".to_string(),
            fields: Default::default(),
            parent_id: Some("atelier-0001".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
        };

        let json = serde_json::to_string(&issue).unwrap();
        let deserialized: Issue = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.parent_id, Some("atelier-0001".to_string()));
    }

    #[test]
    fn test_issue_closed_at() {
        let now = Utc::now();
        let issue = Issue {
            id: "atelier-0001".to_string(),
            title: "Closed issue".to_string(),
            description: None,
            status: "done".to_string(),
            issue_type: "task".to_string(),
            priority: "low".to_string(),
            fields: Default::default(),
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: Some(now),
        };

        let json = serde_json::to_string(&issue).unwrap();
        let deserialized: Issue = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.closed_at, Some(now));
    }

    #[test]
    fn test_issue_unicode_fields() {
        let issue = Issue {
            id: "atelier-0001".to_string(),
            title: "测试 🐛 αβγ".to_string(),
            description: Some("Description with émojis 🎉".to_string()),
            status: "todo".to_string(),
            issue_type: "task".to_string(),
            priority: "high".to_string(),
            fields: Default::default(),
            parent_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
        };

        let json = serde_json::to_string(&issue).unwrap();
        let deserialized: Issue = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.title, "测试 🐛 αβγ");
        assert_eq!(
            deserialized.description,
            Some("Description with émojis 🎉".to_string())
        );
    }

    // ==================== Property-Based Tests ====================

    proptest! {
        #[test]
        fn prop_issue_json_roundtrip(
            id in 1i64..10000,
            title in "[a-zA-Z0-9 ]{1,100}",
            status in "open|closed",
            priority in "low|medium|high|critical"
        ) {
            let issue = Issue {
                id: format!("atelier-{id:04}"),
                title: title.clone(),
                description: None,
                status: status.clone(),
                issue_type: "task".to_string(),
                priority: priority.clone(),
                fields: Default::default(),
                parent_id: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                closed_at: None,
            };

            let json = serde_json::to_string(&issue).unwrap();
            let deserialized: Issue = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(deserialized.id, format!("atelier-{id:04}"));
            prop_assert_eq!(deserialized.title, title);
            prop_assert_eq!(deserialized.status, status);
            prop_assert_eq!(deserialized.priority, priority);
        }

        #[test]
        fn prop_issue_with_optional_fields(
            has_desc in proptest::bool::ANY,
            has_parent in proptest::bool::ANY,
            is_closed in proptest::bool::ANY
        ) {
            let now = Utc::now();
            let issue = Issue {
                id: "atelier-0001".to_string(),
                title: "Test".to_string(),
                description: if has_desc { Some("Desc".to_string()) } else { None },
                status: if is_closed { "closed".to_string() } else { "open".to_string() },
                issue_type: "task".to_string(),
                priority: "medium".to_string(),
                fields: Default::default(),
                parent_id: if has_parent { Some("atelier-002r".to_string()) } else { None },
                created_at: now,
                updated_at: now,
                closed_at: if is_closed { Some(now) } else { None },
            };

            let json = serde_json::to_string(&issue).unwrap();
            let deserialized: Issue = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(deserialized.description.is_some(), has_desc);
            prop_assert_eq!(deserialized.parent_id.is_some(), has_parent);
            prop_assert_eq!(deserialized.closed_at.is_some(), is_closed);
        }
    }
}
