use super::record_kinds::ISSUE_KIND;
pub use atelier_core::{
    AttachmentRelationship, RelatesRelationship, RelationshipTarget, Relationships,
};

/// The typed link a blocked issue declares to its blocker. A blocker that
/// declares the edge stores it in `blocks` instead; both mean the same edge.
pub const BLOCKED_BY: &str = "blocked_by";

/// The typed link new work declares to the record it surfaced from: the item
/// being worked, a review finding, or an evidence receipt.
pub const DISCOVERED_FROM: &str = "discovered_from";

pub fn issue_relationship_target(id: &str) -> RelationshipTarget {
    relationship_target(ISSUE_KIND.kind, id)
}

pub fn relationship_target(kind: &str, id: &str) -> RelationshipTarget {
    RelationshipTarget {
        kind: kind.to_string(),
        id: id.to_string(),
    }
}

pub fn issue_relates_relationship(id: &str, relation_type: &str) -> RelatesRelationship {
    relates_relationship(ISSUE_KIND.kind, id, relation_type)
}

pub fn attachment_relationship(kind: &str, id: &str, role: &str) -> AttachmentRelationship {
    AttachmentRelationship {
        kind: kind.to_string(),
        id: id.to_string(),
        role: role.to_string(),
    }
}

pub fn relates_relationship(kind: &str, id: &str, relation_type: &str) -> RelatesRelationship {
    RelatesRelationship {
        kind: kind.to_string(),
        id: id.to_string(),
        relation_type: relation_type.to_string(),
    }
}

pub fn sort_relationships(relationships: &mut Relationships) {
    relationships.blocks.sort();
    relationships.blocks.dedup();
    relationships.children.sort();
    relationships.children.dedup();
    relationships.attachments.sort();
    relationships.attachments.dedup();
    relationships.relates.sort();
    relationships.relates.dedup();
}
