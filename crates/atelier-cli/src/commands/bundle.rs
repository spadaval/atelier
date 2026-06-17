use anyhow::Result;
use std::path::Path;

use atelier_sqlite::Database;

pub fn preview(db: &Database, input: &str) -> Result<()> {
    super::plan::preview_bundle(db, input)
}

pub fn apply(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    input: &str,
    yes: bool,
) -> Result<()> {
    super::plan::apply_bundle(db, state_dir, db_path, input, yes)
}
