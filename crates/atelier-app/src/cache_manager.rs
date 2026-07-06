use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

use crate::storage_layout;
use atelier_sqlite::projection_index;
use atelier_sqlite::{inspect_cache_file, CacheFileState, Database};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CacheUse {
    /// A result that can affect orchestration, validation, mutation, or closeout.
    Decision,
    /// Read-only operator orientation that may use the last good cache while
    /// reporting invalid canonical state explicitly.
    Orientation,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CachePreparation {
    Current,
    RebuiltMissing,
    RebuiltApplicationIdMismatch { found: i32, expected: i32 },
    RebuiltVersionMismatch { found: i32, expected: i32 },
    RebuiltCorrupt,
    RepairedIncrementally,
    RebuiltStale,
    RebuiltAfterIncrementalFailure,
    OrientationDegraded,
}

pub struct CacheAccess {
    layout: storage_layout::StorageLayout,
    db: Database,
    initial_state: CacheFileState,
    preparation: CachePreparation,
}

impl CacheAccess {
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

    pub fn cache_existed(&self) -> bool {
        !matches!(self.initial_state, CacheFileState::Missing)
    }

    pub fn initial_state(&self) -> &CacheFileState {
        &self.initial_state
    }

    pub fn preparation(&self) -> &CachePreparation {
        &self.preparation
    }
}

#[derive(Debug, Clone)]
pub struct CacheManager {
    layout: storage_layout::StorageLayout,
}

impl CacheManager {
    pub fn discover() -> Result<Self> {
        Ok(Self::new(storage_layout::StorageLayout::discover()?))
    }

    pub fn new(layout: storage_layout::StorageLayout) -> Self {
        Self { layout }
    }

    pub fn repo_root(&self) -> &Path {
        self.layout.repo_root()
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

    /// Inspect cache health without creating, migrating, or rebuilding SQLite.
    pub fn inspect_cache(&self) -> CacheFileState {
        inspect_cache_file(&self.db_path())
    }

    /// Open a decision-safe or explicitly degraded cache on demand.
    pub fn get_cache(&self, cache_use: CacheUse) -> Result<CacheAccess> {
        let initial_state = self.inspect_cache();
        let (db, preparation) = match &initial_state {
            CacheFileState::Ready { .. } => (
                Database::open(&self.db_path()).context("Failed to open cache")?,
                CachePreparation::Current,
            ),
            CacheFileState::Missing => {
                let db = self.rebuild_unusable_cache("local cache is missing")?;
                tracing::warn!(
                    "Local cache was missing; rebuilt SQLite cache from {}",
                    self.state_dir().display()
                );
                (db, CachePreparation::RebuiltMissing)
            }
            CacheFileState::ApplicationIdMismatch { found, expected } => {
                let db = self.rebuild_unusable_cache("local cache application identity changed")?;
                tracing::warn!(
                    "Local cache application identity changed from {} to {}; rebuilt SQLite cache from {}",
                    found,
                    expected,
                    self.state_dir().display()
                );
                (
                    db,
                    CachePreparation::RebuiltApplicationIdMismatch {
                        found: *found,
                        expected: *expected,
                    },
                )
            }
            CacheFileState::VersionMismatch { found, expected } => {
                let db = self.rebuild_unusable_cache("local cache schema version changed")?;
                tracing::warn!(
                    "Local cache schema changed from version {} to {}; rebuilt SQLite cache from {}",
                    found,
                    expected,
                    self.state_dir().display()
                );
                (
                    db,
                    CachePreparation::RebuiltVersionMismatch {
                        found: *found,
                        expected: *expected,
                    },
                )
            }
            CacheFileState::Corrupt { detail } => {
                let db = self.rebuild_unusable_cache("local cache is corrupt")?;
                tracing::warn!(
                    "Local cache was corrupt ({detail}); rebuilt SQLite cache from {}",
                    self.state_dir().display()
                );
                (db, CachePreparation::RebuiltCorrupt)
            }
        };

        self.ensure_fresh(db, initial_state, preparation, cache_use)
    }

    /// Open cache state for a health command that intentionally owns freshness
    /// and repair policy. Unlike `get_cache`, this never repairs stale rows.
    pub fn open_cache_for_health(&self) -> Result<CacheAccess> {
        let initial_state = self.inspect_cache();
        match &initial_state {
            CacheFileState::Missing | CacheFileState::Ready { .. } => {
                let db = Database::open(&self.db_path()).context("Failed to open cache")?;
                Ok(CacheAccess {
                    layout: self.layout.clone(),
                    db,
                    initial_state,
                    preparation: CachePreparation::Current,
                })
            }
            CacheFileState::ApplicationIdMismatch { found, expected } => bail!(
                "Local cache application id {found} does not match {expected}; run `atelier check --fix` to rebuild disposable cache state"
            ),
            CacheFileState::VersionMismatch { found, expected } => bail!(
                "Local cache schema version {found} does not match {expected}; run `atelier check --fix` to rebuild disposable cache state"
            ),
            CacheFileState::Corrupt { detail } => bail!(
                "Local cache is corrupt ({detail}); run `atelier check --fix` to rebuild disposable cache state"
            ),
        }
    }

    fn rebuild_unusable_cache(&self, reason: &str) -> Result<Database> {
        let state_dir = self.state_dir();
        crate::rebuild::validate_canonical_state(&state_dir)
            .map_err(|error| cache_validation_error(error, reason))?;
        crate::rebuild::run(&state_dir, &self.db_path()).with_context(|| {
            format!(
                "{reason} and automatic rebuild failed for {}",
                state_dir.display()
            )
        })?;
        Database::open(&self.db_path()).context("Failed to open rebuilt cache")
    }

    fn ensure_fresh(
        &self,
        db: Database,
        initial_state: CacheFileState,
        preparation: CachePreparation,
        cache_use: CacheUse,
    ) -> Result<CacheAccess> {
        let state_dir = self.state_dir();
        if !state_dir.is_dir() {
            return Ok(self.access(db, initial_state, preparation));
        }

        let report = projection_index::check(&db, &state_dir)?;
        if report.is_fresh() {
            return Ok(self.access(db, initial_state, preparation));
        }

        let incremental_error = match crate::rebuild::repair_incremental(&db, &state_dir, &report) {
            Ok(crate::rebuild::IncrementalRepair::Repaired) => {
                tracing::warn!(
                    "Local cache was stale; repaired changed record sources incrementally from {}",
                    state_dir.display()
                );
                return Ok(self.access(db, initial_state, CachePreparation::RepairedIncrementally));
            }
            Ok(crate::rebuild::IncrementalRepair::NeedsFullRebuild) => None,
            Err(error) => Some(error),
        };

        let db_path = self.db_path();
        drop(db);
        match crate::rebuild::run(&state_dir, &db_path).with_context(|| {
            format!(
                "Local cache is stale and automatic rebuild failed for {}\n{}",
                state_dir.display(),
                report.problem_messages().join("\n")
            )
        }) {
            Ok(()) => {
                tracing::warn!(
                    "Local cache was stale; rebuilt SQLite cache from {}",
                    state_dir.display()
                );
                let db = Database::open(&db_path).context("Failed to open rebuilt cache")?;
                let preparation = if incremental_error.is_some() {
                    CachePreparation::RebuiltAfterIncrementalFailure
                } else {
                    CachePreparation::RebuiltStale
                };
                Ok(self.access(db, initial_state, preparation))
            }
            Err(error) if cache_use == CacheUse::Orientation => {
                let db = Database::open(&db_path).context("Failed to reopen existing cache")?;
                Ok(self.degraded_orientation(db, initial_state, &report, &error))
            }
            Err(error) => Err(cache_validation_error(
                error,
                "Canonical tracker records are invalid",
            )),
        }
    }

    fn degraded_orientation(
        &self,
        db: Database,
        initial_state: CacheFileState,
        report: &projection_index::FreshnessReport,
        error: &anyhow::Error,
    ) -> CacheAccess {
        tracing::warn!(
            "Tracker degraded: canonical tracker records are invalid; using the existing local cache for orientation only."
        );
        tracing::warn!("Recovery: 1. run `atelier lint`; 2. fix the named canonical record; 3. run `atelier check --fix`; 4. rerun the blocked command before closing or mutating work.");
        tracing::warn!("Cache freshness: {}", report.problem_messages().join("; "));
        tracing::warn!("Canonical diagnostic: {error:#}");
        self.access(db, initial_state, CachePreparation::OrientationDegraded)
    }

    fn access(
        &self,
        db: Database,
        initial_state: CacheFileState,
        preparation: CachePreparation,
    ) -> CacheAccess {
        CacheAccess {
            layout: self.layout.clone(),
            db,
            initial_state,
            preparation,
        }
    }
}

fn cache_validation_error(error: anyhow::Error, prefix: &str) -> anyhow::Error {
    let detail = format!("{error:#}");
    if looks_like_schema_drift(&detail) {
        error.context(format!(
            "{prefix}: canonical tracker records use a schema this atelier binary does not understand. Rebuild and use `target/debug/atelier` when testing local CLI changes, or update the installed `atelier` binary before continuing."
        ))
    } else {
        error.context(format!(
            "{prefix}; recovery: 1. run `atelier lint`; 2. fix the named canonical record; 3. run `atelier check --fix`; 4. rerun the blocked command."
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

pub fn find_atelier_dir() -> Result<PathBuf> {
    storage_layout::find_atelier_dir()
}

pub fn state_and_db_paths() -> Result<(PathBuf, PathBuf)> {
    Ok(CacheManager::discover()?.state_and_db_paths())
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_core::{EvidenceRecordData, Record};
    use atelier_records::RecordStore;
    use chrono::Utc;
    use rusqlite::Connection;
    use std::fs;
    use tempfile::TempDir;

    fn test_manager() -> (TempDir, CacheManager) {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".atelier")).unwrap();
        let manager = CacheManager::new(storage_layout::StorageLayout::new(dir.path()));
        (dir, manager)
    }

    fn evidence_data() -> EvidenceRecordData {
        EvidenceRecordData {
            evidence_type: "test".to_string(),
            captured_at: Utc::now(),
            command: None,
            path: None,
            uri: None,
            producer: None,
            proof_scope: None,
            agent_identity: None,
            independence_level: None,
            residual_risks: Vec::new(),
            follow_up_ids: Vec::new(),
            exit_code: None,
            exit_status: None,
            success: Some(true),
            spawn_error: None,
            output: None,
            target: None,
        }
    }

    fn create_evidence(manager: &CacheManager) -> String {
        RecordStore::new(manager.state_dir())
            .create_evidence(
                "Cache evidence",
                "pass",
                "Cache evidence summary.",
                evidence_data(),
            )
            .unwrap()
            .header
            .id
    }

    #[test]
    fn discovery_and_health_inspection_do_not_create_cache() {
        let (_dir, manager) = test_manager();

        assert_eq!(manager.inspect_cache(), CacheFileState::Missing);
        assert!(!manager.db_path().exists());
    }

    #[test]
    fn missing_cache_rebuilds_only_when_queried() {
        let (_dir, manager) = test_manager();

        let cache = manager.get_cache(CacheUse::Decision).unwrap();

        assert_eq!(cache.initial_state(), &CacheFileState::Missing);
        assert_eq!(cache.preparation(), &CachePreparation::RebuiltMissing);
        assert!(cache.db_path().exists());
    }

    #[test]
    fn version_mismatch_is_discarded_and_rebuilt() {
        let (_dir, manager) = test_manager();
        manager.get_cache(CacheUse::Decision).unwrap();
        let connection = Connection::open(manager.db_path()).unwrap();
        connection.pragma_update(None, "user_version", 999).unwrap();
        drop(connection);

        let cache = manager.get_cache(CacheUse::Decision).unwrap();

        assert_eq!(
            cache.preparation(),
            &CachePreparation::RebuiltVersionMismatch {
                found: 999,
                expected: atelier_sqlite::CACHE_SCHEMA_VERSION,
            }
        );
        assert_eq!(
            manager.inspect_cache(),
            CacheFileState::Ready {
                version: atelier_sqlite::CACHE_SCHEMA_VERSION
            }
        );
    }

    #[test]
    fn application_id_mismatch_is_discarded_and_rebuilt() {
        let (_dir, manager) = test_manager();
        manager.get_cache(CacheUse::Decision).unwrap();
        let connection = Connection::open(manager.db_path()).unwrap();
        connection
            .pragma_update(None, "application_id", 42)
            .unwrap();
        drop(connection);

        let cache = manager.get_cache(CacheUse::Decision).unwrap();

        assert_eq!(
            cache.preparation(),
            &CachePreparation::RebuiltApplicationIdMismatch {
                found: 42,
                expected: atelier_sqlite::CACHE_APPLICATION_ID,
            }
        );
        assert!(matches!(
            manager.inspect_cache(),
            CacheFileState::Ready { .. }
        ));
    }

    #[test]
    fn corrupt_cache_is_discarded_and_rebuilt() {
        let (_dir, manager) = test_manager();
        fs::create_dir_all(manager.db_path().parent().unwrap()).unwrap();
        fs::write(manager.db_path(), b"not sqlite").unwrap();
        assert!(matches!(
            manager.inspect_cache(),
            CacheFileState::Corrupt { .. }
        ));

        let cache = manager.get_cache(CacheUse::Decision).unwrap();

        assert_eq!(cache.preparation(), &CachePreparation::RebuiltCorrupt);
        assert!(matches!(
            manager.inspect_cache(),
            CacheFileState::Ready { .. }
        ));
    }

    #[test]
    fn bounded_changed_record_is_repaired_incrementally() {
        let (_dir, manager) = test_manager();
        let evidence_id = create_evidence(&manager);
        crate::rebuild::run(&manager.state_dir(), &manager.db_path()).unwrap();
        let store = RecordStore::new(manager.state_dir());
        let mut evidence = store.load_record_by_id("evidence", &evidence_id).unwrap();
        let Record::Evidence(evidence) = &mut evidence else {
            panic!("expected evidence record");
        };
        evidence.summary = "Changed evidence summary.".to_string();
        evidence.header.updated_at = Utc::now();
        store
            .write_record_atomic(&Record::Evidence(evidence.clone()))
            .unwrap();

        let cache = manager.get_cache(CacheUse::Decision).unwrap();

        assert_eq!(
            cache.preparation(),
            &CachePreparation::RepairedIncrementally
        );
    }

    #[test]
    fn missing_source_metadata_falls_back_to_full_rebuild() {
        let (_dir, manager) = test_manager();
        create_evidence(&manager);
        let db = Database::open(&manager.db_path()).unwrap();
        drop(db);
        let raw = Connection::open(manager.db_path()).unwrap();
        raw.execute("DELETE FROM record_source_index", []).unwrap();
        drop(raw);

        let cache = manager.get_cache(CacheUse::Decision).unwrap();

        assert_eq!(cache.preparation(), &CachePreparation::RebuiltStale);
    }

    #[test]
    fn orientation_can_degrade_but_decision_queries_reject_known_stale_rows() {
        let (_dir, manager) = test_manager();
        manager.get_cache(CacheUse::Decision).unwrap();
        fs::create_dir_all(manager.state_dir().join("evidence")).unwrap();
        fs::write(
            manager.state_dir().join("evidence/atelier-bad1.md"),
            "not a canonical evidence record",
        )
        .unwrap();

        let orientation = manager.get_cache(CacheUse::Orientation).unwrap();
        assert_eq!(
            orientation.preparation(),
            &CachePreparation::OrientationDegraded
        );
        drop(orientation);

        let error = match manager.get_cache(CacheUse::Decision) {
            Ok(_) => panic!("decision query must not return known-stale cache rows"),
            Err(error) => error,
        };
        assert!(
            format!("{error:#}").contains("Canonical tracker records are invalid"),
            "unexpected error: {error:#}"
        );
    }

    #[test]
    fn schema_drift_detection_covers_config_and_workflow_errors() {
        assert!(looks_like_schema_drift(
            "project_config_parse_error: .atelier/config.toml: unknown field `admin_token`"
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
