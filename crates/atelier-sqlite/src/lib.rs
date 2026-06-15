//! SQLite projection and runtime-state boundary.
//!
//! Rebuild freshness, query indexes, graph/search/readiness queries, and local
//! runtime recovery move here during the migration.

pub use atelier_core::RecordId;

/// Marker for the rebuildable projection database API.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProjectionIndex;

/// Table ownership in the single rebuildable runtime database.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TableOwner {
    Projection,
    Runtime,
}

pub const PROJECTION_TABLES: &[&str] = &[
    "issues",
    "labels",
    "dependencies",
    "relations",
    "records",
    "record_links",
    "projection_index_sources",
];

pub const RUNTIME_TABLES: &[&str] = &["runtime_metadata"];

pub fn table_owner(table: &str) -> Option<TableOwner> {
    if PROJECTION_TABLES.contains(&table) {
        Some(TableOwner::Projection)
    } else if RUNTIME_TABLES.contains(&table) {
        Some(TableOwner::Runtime)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_tables_have_explicit_ownership() {
        assert_eq!(table_owner("issues"), Some(TableOwner::Projection));
        assert_eq!(table_owner("runtime_metadata"), Some(TableOwner::Runtime));
    }

    #[test]
    fn removed_tables_are_not_part_of_target_schema() {
        assert_eq!(table_owner("sessions"), None);
        assert_eq!(table_owner("work_associations"), None);
        assert_eq!(table_owner("claims"), None);
    }
}
