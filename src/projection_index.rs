use anyhow::Result;
use std::path::Path;

use crate::db::Database;

pub use atelier_sqlite::{FreshnessReport, SourceEntry};

impl Database {
    pub(crate) fn init_projection_index_schema(&self) -> Result<()> {
        atelier_sqlite::ProjectionIndex::init_schema(&self.conn)
    }

    pub fn replace_projection_sources(&self, entries: &[SourceEntry]) -> Result<()> {
        atelier_sqlite::ProjectionIndex::new(&self.conn).replace_sources(entries)
    }

    pub fn projection_sources(&self) -> Result<Vec<SourceEntry>> {
        atelier_sqlite::ProjectionIndex::new(&self.conn).sources()
    }
}

pub fn refresh(db: &Database, state_dir: &Path) -> Result<()> {
    atelier_sqlite::ProjectionIndex::new(&db.conn).refresh_sources(state_dir)
}

pub fn check(db: &Database, state_dir: &Path) -> Result<FreshnessReport> {
    atelier_sqlite::ProjectionIndex::new(&db.conn).check_freshness(state_dir)
}
