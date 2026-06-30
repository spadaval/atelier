use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::storage_layout;
use atelier_sqlite::projection_index;
use atelier_sqlite::Database;

pub fn find_atelier_dir() -> Result<PathBuf> {
    storage_layout::find_atelier_dir()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandStorageAccess {
    /// Read/query commands that depend on a fresh SQLite projection.
    ProjectionQuery,
    /// Orientation reads that can fall back to the existing projection when
    /// canonical records are malformed, as long as the degraded state is named.
    DegradedProjectionQuery,
    /// Commands that write canonical Markdown and then refresh the projection.
    CanonicalMutation,
    /// Commands that intentionally inspect the existing projection and own any
    /// command-specific freshness or degraded-state behavior.
    ExistingProjection,
    /// Diagnostics, export, rebuild, and repair flows that own freshness policy.
    HealthRepair,
}

impl CommandStorageAccess {
    pub fn requires_fresh_projection(self) -> bool {
        matches!(
            self,
            CommandStorageAccess::ProjectionQuery
                | CommandStorageAccess::DegradedProjectionQuery
                | CommandStorageAccess::CanonicalMutation
        )
    }

    pub fn allows_degraded_projection(self) -> bool {
        matches!(self, CommandStorageAccess::DegradedProjectionQuery)
    }
}

pub struct CommandStorage {
    layout: storage_layout::StorageLayout,
    db: Database,
    pub projection_db_existed: bool,
}

impl CommandStorage {
    pub fn db(&self) -> &Database {
        &self.db
    }

    pub fn into_db(self) -> Database {
        self.db
    }

    pub fn state_dir(&self) -> PathBuf {
        self.layout.canonical_dir()
    }

    pub fn db_path(&self) -> PathBuf {
        self.layout.runtime_db_path()
    }

    pub fn state_and_db_paths(&self) -> (PathBuf, PathBuf) {
        (self.state_dir(), self.db_path())
    }

    pub fn repo_root(&self) -> &Path {
        self.layout.repo_root()
    }
}

pub fn command_storage(mode: CommandStorageAccess) -> Result<CommandStorage> {
    let layout = storage_layout::StorageLayout::discover()?;
    let projection_db_existed = layout.runtime_db_path().exists();
    let db = Database::open(&layout.runtime_db_path()).context("Failed to open database")?;
    let db = if mode.requires_fresh_projection() {
        ensure_fresh_projection_db(
            db,
            &layout,
            projection_db_existed,
            mode.allows_degraded_projection(),
        )?
    } else {
        db
    };
    Ok(CommandStorage {
        layout,
        db,
        projection_db_existed,
    })
}

fn ensure_fresh_projection_db(
    db: Database,
    layout: &storage_layout::StorageLayout,
    projection_db_existed: bool,
    allow_degraded_projection: bool,
) -> Result<Database> {
    let state_dir = layout.canonical_dir();
    if state_dir.is_dir() {
        if !projection_db_existed {
            crate::rebuild::validate_canonical_state(&state_dir).map_err(|error| {
                projection_validation_error(error, "Runtime projection database is missing")
            })?;
            let db_path = layout.runtime_db_path();
            drop(db);
            crate::rebuild::run(&state_dir, &db_path).with_context(|| {
                format!(
                    "Runtime projection database is missing and automatic rebuild failed for {}",
                    state_dir.display()
                )
            })?;
            tracing::warn!(
                "Runtime projection database was missing; rebuilt local SQLite projection from {}",
                state_dir.display()
            );
            return Database::open(&db_path).context("Failed to open database");
        }

        let report = projection_index::check(&db, &state_dir)?;
        if !report.is_fresh() {
            match crate::rebuild::repair_incremental(&db, &state_dir, &report) {
                Ok(crate::rebuild::IncrementalRepair::Repaired) => {
                    tracing::warn!(
                        "Projection index was stale; repaired local SQLite projection incrementally from {}",
                        state_dir.display()
                    );
                    return Ok(db);
                }
                Ok(crate::rebuild::IncrementalRepair::NeedsFullRebuild) => {}
                Err(error) if allow_degraded_projection => {
                    tracing::warn!(
                        "Tracker degraded: canonical tracker Markdown is invalid; using existing local projection for orientation only."
                    );
                    tracing::warn!("Recovery: 1. run `atelier lint`; 2. fix the named canonical Markdown record; 3. run `atelier doctor --fix`; 4. rerun the blocked command before closing or mutating work.");
                    tracing::warn!(
                        "Projection freshness: {}",
                        report.problem_messages().join("; ")
                    );
                    tracing::warn!("Canonical diagnostic: {error:#}");
                    return Ok(db);
                }
                Err(error) => {
                    return Err(projection_validation_error(
                        error,
                        "Canonical tracker Markdown is invalid",
                    ));
                }
            }
            let db_path = layout.runtime_db_path();
            drop(db);
            if let Err(error) = crate::rebuild::run(&state_dir, &db_path).with_context(|| {
                format!(
                    "Projection index is stale and automatic rebuild failed for {}\n{}",
                    state_dir.display(),
                    report.problem_messages().join("\n")
                )
            }) {
                if allow_degraded_projection {
                    tracing::warn!(
                        "Tracker degraded: canonical tracker Markdown is invalid; using existing local projection for orientation only."
                    );
                    tracing::warn!("Recovery: 1. run `atelier lint`; 2. fix the named canonical Markdown record; 3. run `atelier doctor --fix`; 4. rerun the blocked command before closing or mutating work.");
                    tracing::warn!(
                        "Projection freshness: {}",
                        report.problem_messages().join("; ")
                    );
                    tracing::warn!("Canonical diagnostic: {error:#}");
                    return Database::open(&db_path).context("Failed to open database");
                }
                return Err(projection_validation_error(
                    error,
                    "Canonical tracker Markdown is invalid",
                ));
            }
            tracing::warn!(
                "Projection index was stale; rebuilt local SQLite projection from {}",
                state_dir.display()
            );
            return Database::open(&db_path).context("Failed to open database");
        }
    }
    Ok(db)
}

fn projection_validation_error(error: anyhow::Error, prefix: &str) -> anyhow::Error {
    let detail = format!("{error:#}");
    if looks_like_schema_drift(&detail) {
        error.context(format!(
            "{prefix}: canonical tracker records use a schema this atelier binary does not understand. \
             Rebuild and use `target/debug/atelier` when testing local CLI changes, or update the installed `atelier` binary before continuing."
        ))
    } else {
        error.context(format!(
            "{prefix}; recovery: 1. run `atelier lint`; 2. fix the named canonical Markdown record; 3. run `atelier doctor --fix`; 4. rerun the blocked command."
        ))
    }
}

fn looks_like_schema_drift(detail: &str) -> bool {
    detail.contains("Unsupported schema")
        || detail.contains("Unsupported schema_version")
        || detail.contains("project_config_parse_error")
        || detail.contains("workflow_config_invalid")
        || detail.contains("unknown field")
}

pub fn state_and_db_paths() -> Result<(PathBuf, PathBuf)> {
    Ok(command_storage(CommandStorageAccess::CanonicalMutation)?.state_and_db_paths())
}

pub fn existing_projection_db() -> Result<Database> {
    Ok(command_storage(CommandStorageAccess::ExistingProjection)?.into_db())
}

pub fn projection_query_db() -> Result<Database> {
    Ok(command_storage(CommandStorageAccess::ProjectionQuery)?.into_db())
}

pub fn degraded_projection_query_db() -> Result<Database> {
    Ok(command_storage(CommandStorageAccess::DegradedProjectionQuery)?.into_db())
}

pub fn lint_db() -> Result<Database> {
    let layout = storage_layout::StorageLayout::discover()?;
    if layout.runtime_db_path().exists() {
        Ok(command_storage(CommandStorageAccess::ExistingProjection)?.into_db())
    } else {
        projection_query_db()
    }
}

pub fn canonical_mutation_db() -> Result<Database> {
    Ok(command_storage(CommandStorageAccess::CanonicalMutation)?.into_db())
}

#[cfg(test)]
mod tests {
    use super::{looks_like_schema_drift, CommandStorageAccess};

    #[test]
    fn access_modes_declare_projection_freshness_policy() {
        assert!(CommandStorageAccess::ProjectionQuery.requires_fresh_projection());
        assert!(CommandStorageAccess::DegradedProjectionQuery.requires_fresh_projection());
        assert!(CommandStorageAccess::CanonicalMutation.requires_fresh_projection());
        assert!(!CommandStorageAccess::ExistingProjection.requires_fresh_projection());
        assert!(!CommandStorageAccess::HealthRepair.requires_fresh_projection());
        assert!(!CommandStorageAccess::ProjectionQuery.allows_degraded_projection());
        assert!(CommandStorageAccess::DegradedProjectionQuery.allows_degraded_projection());
        assert!(!CommandStorageAccess::CanonicalMutation.allows_degraded_projection());
    }

    #[test]
    fn schema_drift_detection_covers_config_and_workflow_errors() {
        assert!(looks_like_schema_drift(
            "project_config_parse_error: .atelier/config.toml: unknown field `admin_token_env`"
        ));
        assert!(looks_like_schema_drift(
            "workflow_config_invalid_validator: unsupported built-in validator"
        ));
        assert!(looks_like_schema_drift("Unsupported schema_version 99"));
        assert!(!looks_like_schema_drift(
            "canonical issue record is missing required section"
        ));
    }
}
