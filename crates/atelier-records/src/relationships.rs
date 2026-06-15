use super::record_kinds::ISSUE_KIND;

pub use atelier_core::relationships::{
    sort_relationships, AttachmentRelationship, RelatesRelationship, RelationshipTarget,
    Relationships,
};

pub fn relationship_target(kind: &str, id: &str) -> RelationshipTarget {
    RelationshipTarget {
        kind: kind.to_string(),
        id: id.to_string(),
    }
}

pub fn issue_relationship_target(id: &str) -> RelationshipTarget {
    relationship_target(ISSUE_KIND.kind, id)
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

pub fn issue_relates_relationship(id: &str, relation_type: &str) -> RelatesRelationship {
    relates_relationship(ISSUE_KIND.kind, id, relation_type)
}

pub fn is_attachment_role(relation_type: &str) -> bool {
    matches!(
        relation_type,
        "planned_by" | "validates" | "evidenced_by" | "has_checkpoint"
    )
}

pub fn is_child_relation(relation_type: &str) -> bool {
    matches!(
        relation_type,
        "advances" | "contributes_to" | "implements" | "has_checkpoint"
    )
}

pub fn is_first_class_attachment_kind(kind: &str) -> bool {
    matches!(kind, "plan" | "evidence" | "milestone")
}

pub fn add_relationship_link_for_owner(
    relationships: &mut Relationships,
    owner_kind: &str,
    owner_id: &str,
    source_kind: &str,
    source_id: &str,
    target_kind: &str,
    target_id: &str,
    relation_type: &str,
) {
    if source_kind != owner_kind || source_id != owner_id {
        return;
    }

    if owner_kind == ISSUE_KIND.kind
        && target_kind == ISSUE_KIND.kind
        && is_child_relation(relation_type)
    {
        relationships
            .children
            .push(relationship_target(target_kind, target_id));
    } else if is_first_class_attachment_kind(target_kind) && is_attachment_role(relation_type) {
        relationships.attachments.push(attachment_relationship(
            target_kind,
            target_id,
            relation_type,
        ));
    } else {
        relationships
            .relates
            .push(relates_relationship(target_kind, target_id, relation_type));
    }
}
