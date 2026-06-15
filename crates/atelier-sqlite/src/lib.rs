//! SQLite projection and runtime-state boundary.
//!
//! Rebuild freshness, query indexes, graph/search/readiness queries, and local
//! runtime recovery move here during the migration.

pub use atelier_core::RecordId;

/// Marker for the rebuildable projection database API.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProjectionIndex;
