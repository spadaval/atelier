use atelier_core::{
    base36_padded, legacy_issue_id, sort_relationships, validate_record_id, AttachmentRelationship,
    EvidenceTarget, RelatesRelationship, RelationshipTarget, Relationships,
};

#[test]
fn record_ids_are_project_scoped_and_base36() {
    assert_eq!(base36_padded(0, 4), "0000");
    assert_eq!(base36_padded(35, 4), "000z");
    assert_eq!(base36_padded(36, 4), "0010");
    assert_eq!(legacy_issue_id(1), "atelier-0001");

    assert!(validate_record_id("atelier-z1p8").is_ok());
    assert!(validate_record_id("atelier-0010").is_ok());
    assert!(validate_record_id("Atelier-0010").is_err());
    assert!(validate_record_id("atelier-00_1").is_err());
    assert!(validate_record_id("atelier").is_err());
}

#[test]
fn relationship_sets_are_sorted_and_deduplicated_by_value() {
    let mut relationships = Relationships {
        blocks: vec![
            RelationshipTarget {
                kind: "issue".to_string(),
                id: "atelier-0002".to_string(),
            },
            RelationshipTarget {
                kind: "issue".to_string(),
                id: "atelier-0001".to_string(),
            },
            RelationshipTarget {
                kind: "issue".to_string(),
                id: "atelier-0002".to_string(),
            },
        ],
        children: vec![
            RelationshipTarget {
                kind: "issue".to_string(),
                id: "atelier-0004".to_string(),
            },
            RelationshipTarget {
                kind: "issue".to_string(),
                id: "atelier-0004".to_string(),
            },
        ],
        attachments: vec![
            AttachmentRelationship {
                kind: "evidence".to_string(),
                id: "atelier-0003".to_string(),
                role: "validates".to_string(),
            },
            AttachmentRelationship {
                kind: "evidence".to_string(),
                id: "atelier-0003".to_string(),
                role: "validates".to_string(),
            },
        ],
        relates: vec![
            RelatesRelationship {
                kind: "mission".to_string(),
                id: "atelier-0005".to_string(),
                relation_type: "advances".to_string(),
            },
            RelatesRelationship {
                kind: "mission".to_string(),
                id: "atelier-0005".to_string(),
                relation_type: "advances".to_string(),
            },
        ],
    };

    sort_relationships(&mut relationships);

    assert_eq!(
        relationships.blocks,
        vec![
            RelationshipTarget {
                kind: "issue".to_string(),
                id: "atelier-0001".to_string(),
            },
            RelationshipTarget {
                kind: "issue".to_string(),
                id: "atelier-0002".to_string(),
            },
        ]
    );
    assert_eq!(relationships.children.len(), 1);
    assert_eq!(relationships.attachments.len(), 1);
    assert_eq!(relationships.relates.len(), 1);
}

#[test]
fn evidence_target_defaults_to_validation_role_at_domain_boundary() {
    let target: EvidenceTarget =
        serde_json::from_str(r#"{"kind":"issue","id":"atelier-0001"}"#).unwrap();

    assert_eq!(target.kind, "issue");
    assert_eq!(target.id, "atelier-0001");
    assert_eq!(target.role, "validates");
}
