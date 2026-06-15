use super::record_kinds::ISSUE_KIND;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RelationshipTarget {
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AttachmentRelationship {
    pub kind: String,
    pub id: String,
    pub role: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RelatesRelationship {
    pub kind: String,
    pub id: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Relationships {
    pub blocks: Vec<RelationshipTarget>,
    pub children: Vec<RelationshipTarget>,
    pub attachments: Vec<AttachmentRelationship>,
    pub relates: Vec<RelatesRelationship>,
}

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
