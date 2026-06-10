use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use rusqlite::params;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use crate::db::Database;
use crate::record_store;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceEntry {
    pub path: String,
    pub size_bytes: i64,
    pub modified_micros: Option<i64>,
    pub sha256: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FreshnessProblem {
    MissingMetadata { path: String },
    MissingSource { path: String },
    ChangedSource { path: String },
    UnindexedSource { path: String },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FreshnessReport {
    pub checked: bool,
    pub source_count: usize,
    pub problems: Vec<FreshnessProblem>,
}

impl FreshnessReport {
    pub fn is_fresh(&self) -> bool {
        self.problems.is_empty()
    }

    pub fn problem_messages(&self) -> Vec<String> {
        self.problems
            .iter()
            .map(|problem| match problem {
                FreshnessProblem::MissingMetadata { path } => {
                    format!("missing projection metadata for {path}")
                }
                FreshnessProblem::MissingSource { path } => {
                    format!("indexed source is missing: {path}")
                }
                FreshnessProblem::ChangedSource { path } => {
                    format!("indexed source changed: {path}")
                }
                FreshnessProblem::UnindexedSource { path } => {
                    format!("canonical source is not indexed: {path}")
                }
            })
            .collect()
    }
}

impl Database {
    pub(crate) fn init_projection_index_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS projection_index_sources (
                path TEXT PRIMARY KEY,
                size_bytes INTEGER NOT NULL,
                modified_micros INTEGER,
                sha256 TEXT NOT NULL,
                indexed_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_projection_index_sources_hash
                ON projection_index_sources(sha256);
            "#,
        )?;
        Ok(())
    }

    pub fn replace_projection_sources(&self, entries: &[SourceEntry]) -> Result<()> {
        let indexed_at = Utc::now().to_rfc3339();
        self.conn
            .execute("DELETE FROM projection_index_sources", [])?;
        for entry in entries {
            self.conn.execute(
                "INSERT INTO projection_index_sources
                 (path, size_bytes, modified_micros, sha256, indexed_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    entry.path,
                    entry.size_bytes,
                    entry.modified_micros,
                    entry.sha256,
                    indexed_at
                ],
            )?;
        }
        Ok(())
    }

    pub fn projection_sources(&self) -> Result<Vec<SourceEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT path, size_bytes, modified_micros, sha256
             FROM projection_index_sources
             ORDER BY path",
        )?;
        let entries = stmt
            .query_map([], |row| {
                Ok(SourceEntry {
                    path: row.get(0)?,
                    size_bytes: row.get(1)?,
                    modified_micros: row.get(2)?,
                    sha256: row.get(3)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(entries)
    }
}

pub fn refresh(db: &Database, state_dir: &Path) -> Result<()> {
    let snapshot = snapshot_sources(state_dir)?;
    db.replace_projection_sources(&snapshot)
}

pub fn check(db: &Database, state_dir: &Path) -> Result<FreshnessReport> {
    if !state_dir.exists() {
        return Ok(FreshnessReport {
            checked: false,
            source_count: 0,
            problems: Vec::new(),
        });
    }

    let current = snapshot_sources(state_dir)?;
    let stored = db.projection_sources()?;
    let current_by_path = current
        .iter()
        .map(|entry| (entry.path.clone(), entry))
        .collect::<BTreeMap<_, _>>();
    let stored_by_path = stored
        .iter()
        .map(|entry| (entry.path.clone(), entry))
        .collect::<BTreeMap<_, _>>();

    let mut problems = Vec::new();
    if stored.is_empty() && !current.is_empty() {
        for entry in &current {
            problems.push(FreshnessProblem::MissingMetadata {
                path: entry.path.clone(),
            });
        }
    } else {
        for stored_entry in &stored {
            match current_by_path.get(&stored_entry.path) {
                Some(current_entry) if current_entry.sha256 == stored_entry.sha256 => {}
                Some(_) => problems.push(FreshnessProblem::ChangedSource {
                    path: stored_entry.path.clone(),
                }),
                None => problems.push(FreshnessProblem::MissingSource {
                    path: stored_entry.path.clone(),
                }),
            }
        }
        for current_entry in &current {
            if !stored_by_path.contains_key(&current_entry.path) {
                problems.push(FreshnessProblem::UnindexedSource {
                    path: current_entry.path.clone(),
                });
            }
        }
    }

    Ok(FreshnessReport {
        checked: true,
        source_count: current.len(),
        problems,
    })
}

pub fn ensure_fresh(db: &Database, state_dir: &Path) -> Result<()> {
    let report = check(db, state_dir)?;
    if report.is_fresh() {
        return Ok(());
    }

    bail!(
        "Projection index is stale; run `atelier rebuild` before querying.\n{}",
        report.problem_messages().join("\n")
    )
}

fn snapshot_sources(state_dir: &Path) -> Result<Vec<SourceEntry>> {
    if !state_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    for dir in record_store::canonical_record_dirs() {
        let source_dir = state_dir.join(dir);
        if source_dir.exists() {
            collect_source_files(state_dir, &source_dir, &mut entries)?;
        }
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(entries)
}

fn collect_source_files(root: &Path, dir: &Path, entries: &mut Vec<SourceEntry>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(".activity"))
            {
                continue;
            }
            collect_source_files(root, &path, entries)?;
        } else if path.is_file() {
            entries.push(source_entry(root, &path)?);
        }
    }
    Ok(())
}

fn source_entry(root: &Path, path: &Path) -> Result<SourceEntry> {
    let relative = path
        .strip_prefix(root)
        .with_context(|| format!("Failed to relativize {}", path.display()))?;
    let relative = canonical_relative_path(relative)?;
    let metadata =
        fs::metadata(path).with_context(|| format!("Failed to stat {}", path.display()))?;
    let bytes = fs::read(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(SourceEntry {
        path: relative,
        size_bytes: metadata
            .len()
            .try_into()
            .context("source file is too large")?,
        modified_micros: metadata
            .modified()
            .ok()
            .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
            .map(|duration| {
                (duration.as_secs() as i64)
                    .saturating_mul(1_000_000)
                    .saturating_add(i64::from(duration.subsec_micros()))
            }),
        sha256: format!("{:x}", hasher.finalize()),
    })
}

fn canonical_relative_path(path: &Path) -> Result<String> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::Normal(part) => parts.push(
                part.to_str()
                    .ok_or_else(|| {
                        anyhow!("canonical state path is not UTF-8: {}", path.display())
                    })?
                    .to_string(),
            ),
            _ => bail!("canonical state path is not relative: {}", path.display()),
        }
    }
    Ok(parts.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn freshness_reports_changed_missing_and_unindexed_sources() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier-state");
        let issues = state_dir.join("issues");
        fs::create_dir_all(&issues).unwrap();
        fs::write(issues.join("atelier-aaaa.md"), "one").unwrap();
        fs::write(issues.join("atelier-bbbb.md"), "two").unwrap();
        fs::create_dir_all(dir.path().join(".atelier")).unwrap();
        let db = Database::open(&dir.path().join(".atelier/state.db")).unwrap();

        refresh(&db, &state_dir).unwrap();
        assert!(check(&db, &state_dir).unwrap().is_fresh());

        fs::write(issues.join("atelier-aaaa.md"), "changed").unwrap();
        fs::remove_file(issues.join("atelier-bbbb.md")).unwrap();
        fs::write(issues.join("atelier-cccc.md"), "new").unwrap();

        let report = check(&db, &state_dir).unwrap();
        assert_eq!(report.source_count, 2);
        assert!(report.problems.contains(&FreshnessProblem::ChangedSource {
            path: "issues/atelier-aaaa.md".to_string()
        }));
        assert!(report.problems.contains(&FreshnessProblem::MissingSource {
            path: "issues/atelier-bbbb.md".to_string()
        }));
        assert!(report
            .problems
            .contains(&FreshnessProblem::UnindexedSource {
                path: "issues/atelier-cccc.md".to_string()
            }));
    }

    #[test]
    fn freshness_reports_missing_metadata_when_state_exists() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier-state");
        fs::create_dir_all(state_dir.join("issues")).unwrap();
        fs::write(state_dir.join("issues/atelier-aaaa.md"), "one").unwrap();
        fs::create_dir_all(dir.path().join(".atelier")).unwrap();
        let db = Database::open(&dir.path().join(".atelier/state.db")).unwrap();

        let report = check(&db, &state_dir).unwrap();

        assert_eq!(
            report.problems,
            vec![FreshnessProblem::MissingMetadata {
                path: "issues/atelier-aaaa.md".to_string()
            }]
        );
    }
}
