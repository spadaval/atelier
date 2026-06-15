use super::record_kinds::ISSUE_KIND;

pub use atelier_core::relationships::{
    sort_relationships, AttachmentRelationship, RelatesRelationship, RelationshipTarget,
    Relationships,
};

pub fn issue_relationship_target(id: &str) -> RelationshipTarget {
    RelationshipTarget {
        kind: ISSUE_KIND.kind.to_string(),
        id: id.to_string(),
    }
}

pub fn issue_relates_relationship(id: &str, relation_type: &str) -> RelatesRelationship {
    RelatesRelationship {
        kind: ISSUE_KIND.kind.to_string(),
        id: id.to_string(),
        relation_type: relation_type.to_string(),
    }
}
