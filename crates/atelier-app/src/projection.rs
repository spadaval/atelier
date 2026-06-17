use anyhow::{Context, Result};
use std::path::Path;

/// Refresh the SQLite projection after a successful canonical RecordStore write.
///
/// Callers must drop any open `Database` handle for `db_path` before invoking
/// this helper because the refresh attaches and replaces that SQLite file.
pub fn refresh_after_canonical_write(state_dir: &Path, db_path: &Path) -> Result<()> {
    super::rebuild::validate_canonical_state(state_dir).with_context(|| {
        format!(
            "Canonical write succeeded but {} is not rebuild-ready; \
             fix canonical Markdown before refreshing the projection.",
            state_dir.display()
        )
    })?;
    super::rebuild::refresh_projection(state_dir, db_path).with_context(|| {
        format!(
            "Canonical write succeeded but projection refresh failed for {}",
            state_dir.display()
        )
    })
}
