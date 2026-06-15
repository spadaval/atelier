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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorting_relationships_deduplicates_each_value_set() {
        let mut relationships = Relationships {
            blocks: vec![
                RelationshipTarget {
                    kind: "issue".to_string(),
                    id: "atelier-0002".to_string(),
                },
                RelationshipTarget {
                    kind: "issue".to_string(),
                    id: "atelier-0002".to_string(),
                },
            ],
            ..Relationships::default()
        };

        sort_relationships(&mut relationships);

        assert_eq!(relationships.blocks.len(), 1);
    }
}
