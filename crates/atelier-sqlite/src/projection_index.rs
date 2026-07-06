use anyhow::{anyhow, bail, Context, Result};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use atelier_records as record_store;

use crate::{Database, RecordSourceCacheRow};

const MAX_PROBLEM_SAMPLES: usize = 5;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceEntry {
    pub path: String,
    pub kind: String,
    pub id: String,
    pub size_bytes: i64,
    pub modified_micros: Option<i64>,
    pub sha256: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SourceStat {
    path: String,
    kind: String,
    id: String,
    size_bytes: i64,
    modified_micros: Option<i64>,
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
        let mut messages = Vec::new();
        let mut missing_metadata = Vec::new();
        let mut missing_sources = Vec::new();
        let mut changed_sources = Vec::new();
        let mut unindexed_sources = Vec::new();

        for problem in &self.problems {
            match problem {
                FreshnessProblem::MissingMetadata { path } => missing_metadata.push(path.as_str()),
                FreshnessProblem::MissingSource { path } => missing_sources.push(path.as_str()),
                FreshnessProblem::ChangedSource { path } => changed_sources.push(path.as_str()),
                FreshnessProblem::UnindexedSource { path } => unindexed_sources.push(path.as_str()),
            }
        }

        push_missing_metadata_message(&mut messages, &missing_metadata);
        push_path_group_message(
            &mut messages,
            &missing_sources,
            "indexed source is missing",
            "indexed sources are missing",
        );
        push_path_group_message(
            &mut messages,
            &changed_sources,
            "indexed source changed",
            "indexed sources changed",
        );
        push_path_group_message(
            &mut messages,
            &unindexed_sources,
            "record-file source is not cached",
            "record-file sources are not cached",
        );
        if !messages.is_empty() {
            messages.push(
                "recovery: 1. run `atelier check`; 2. fix any named record files; 3. run `atelier check --fix` to repair local runtime/cache state; 4. rerun the blocked command"
                    .to_string(),
            );
        }
        messages
    }
}

fn push_missing_metadata_message(messages: &mut Vec<String>, paths: &[&str]) {
    match paths {
        [] => {}
        [path] => messages.push(format!(
            "domain-cache metadata is missing for record-file source: {path}"
        )),
        _ => messages.push(format!(
            "domain-cache metadata is missing for {} record-file sources (showing first {}): {}",
            paths.len(),
            paths.len().min(MAX_PROBLEM_SAMPLES),
            paths
                .iter()
                .take(MAX_PROBLEM_SAMPLES)
                .copied()
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn push_path_group_message(
    messages: &mut Vec<String>,
    paths: &[&str],
    singular: &str,
    plural: &str,
) {
    match paths {
        [] => {}
        [path] => messages.push(format!("{singular}: {path}")),
        _ => messages.push(format!(
            "{} {} (showing first {}): {}",
            paths.len(),
            plural,
            paths.len().min(MAX_PROBLEM_SAMPLES),
            paths
                .iter()
                .take(MAX_PROBLEM_SAMPLES)
                .copied()
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

pub fn refresh(db: &Database, state_dir: &Path) -> Result<()> {
    let snapshot = snapshot_sources(state_dir)?;
    let stored = db.record_source_cache_rows()?;
    if snapshot.len() != stored.len() {
        bail!("record-file set does not match domain-cache source metadata");
    }
    for entry in snapshot {
        db.refresh_record_source_metadata(&RecordSourceCacheRow {
            path: entry.path,
            record_kind: entry.kind,
            record_id: entry.id,
            size_bytes: entry.size_bytes,
            modified_micros: entry.modified_micros,
            content_hash: Some(entry.sha256),
            indexed_at: chrono::Utc::now(),
        })?;
    }
    Ok(())
}

pub fn source_entry_for_path(state_dir: &Path, relative: &str) -> Result<SourceEntry> {
    let path = state_dir.join(relative);
    source_entry(state_dir, &path)
}

pub fn check(db: &Database, state_dir: &Path) -> Result<FreshnessReport> {
    if !state_dir.exists() {
        return Ok(FreshnessReport {
            checked: false,
            source_count: 0,
            problems: Vec::new(),
        });
    }

    let current = snapshot_source_stats(state_dir)?;
    let stored = db.record_source_cache_rows()?;
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
                Some(current_entry)
                    if current_entry.size_bytes == stored_entry.size_bytes
                        && current_entry.modified_micros == stored_entry.modified_micros => {}
                Some(current_entry) => {
                    let hashed = source_entry(state_dir, &state_dir.join(&current_entry.path))?;
                    if stored_entry.content_hash.as_deref() == Some(hashed.sha256.as_str()) {
                        db.refresh_record_source_metadata(&RecordSourceCacheRow {
                            path: hashed.path,
                            record_kind: hashed.kind,
                            record_id: hashed.id,
                            size_bytes: hashed.size_bytes,
                            modified_micros: hashed.modified_micros,
                            content_hash: Some(hashed.sha256),
                            indexed_at: chrono::Utc::now(),
                        })?;
                    } else {
                        problems.push(FreshnessProblem::ChangedSource {
                            path: stored_entry.path.clone(),
                        });
                    }
                }
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

fn snapshot_source_stats(state_dir: &Path) -> Result<Vec<SourceStat>> {
    if !state_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    for dir in record_store::canonical_record_dirs() {
        let source_dir = state_dir.join(dir);
        if source_dir.exists() {
            collect_source_stats(state_dir, &source_dir, &mut entries)?;
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
            let relative = path
                .strip_prefix(root)
                .context("Failed to relativize source file")?;
            if is_local_atelier_path(relative) {
                continue;
            }
            entries.push(source_entry(root, &path)?);
        }
    }
    Ok(())
}

fn collect_source_stats(root: &Path, dir: &Path, entries: &mut Vec<SourceStat>) -> Result<()> {
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
            collect_source_stats(root, &path, entries)?;
        } else if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .context("Failed to relativize source file")?;
            if is_local_atelier_path(relative) {
                continue;
            }
            entries.push(source_stat(root, &path)?);
        }
    }
    Ok(())
}

fn is_local_atelier_path(relative_path: &Path) -> bool {
    if is_local_artifact_path(relative_path) {
        return true;
    }

    let Some(first) = relative_path.components().next() else {
        return false;
    };
    let first = first.as_os_str();
    first == ".cache"
        || first == "runtime"
        || first == "cache"
        || first == "rules"
        || first == "rules.local"
        || relative_path == Path::new("config.toml")
        || relative_path == Path::new("state.db")
        || relative_path == Path::new("agent.json")
}

fn is_local_artifact_path(relative_path: &Path) -> bool {
    let Some(file_name) = relative_path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    file_name.starts_with(".state.db.")
        && (file_name.ends_with(".rebuild-tmp")
            || file_name.ends_with(".rebuild-tmp-shm")
            || file_name.ends_with(".rebuild-tmp-wal")
            || file_name.ends_with(".rebuild-tmp-journal"))
        || file_name.ends_with(".tmp")
        || file_name.ends_with(".lock")
        || file_name.ends_with(".md-journal")
}

fn source_entry(root: &Path, path: &Path) -> Result<SourceEntry> {
    let relative = path
        .strip_prefix(root)
        .with_context(|| format!("Failed to relativize {}", path.display()))?;
    let relative = canonical_relative_path(relative)?;
    let stat = source_stat(root, path)?;
    let bytes = fs::read(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(SourceEntry {
        path: relative,
        kind: stat.kind,
        id: stat.id,
        size_bytes: stat.size_bytes,
        modified_micros: stat.modified_micros,
        sha256: format!("{:x}", hasher.finalize()),
    })
}

fn source_stat(root: &Path, path: &Path) -> Result<SourceStat> {
    let relative = path
        .strip_prefix(root)
        .with_context(|| format!("Failed to relativize {}", path.display()))?;
    let relative = canonical_relative_path(relative)?;
    let (kind, id) = source_identity(&relative)?;
    let metadata =
        fs::metadata(path).with_context(|| format!("Failed to stat {}", path.display()))?;
    Ok(SourceStat {
        path: relative,
        kind,
        id,
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
    })
}

fn source_identity(relative: &str) -> Result<(String, String)> {
    let path = Path::new(relative);
    let dir = path
        .components()
        .next()
        .and_then(|component| component.as_os_str().to_str())
        .ok_or_else(|| anyhow!("record-file source path has no directory: {relative}"))?;
    let kind = record_store::CANONICAL_RECORD_KINDS
        .iter()
        .find(|spec| spec.canonical_dir == Some(dir))
        .map(|spec| spec.kind.to_string())
        .ok_or_else(|| anyhow!("record-file source path has unknown directory: {relative}"))?;
    let id = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| anyhow!("record-file source path has no record id: {relative}"))?
        .to_string();
    Ok((kind, id))
}

fn canonical_relative_path(path: &Path) -> Result<String> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::Normal(part) => parts.push(
                part.to_str()
                    .ok_or_else(|| anyhow!("record-file path is not UTF-8: {}", path.display()))?
                    .to_string(),
            ),
            _ => bail!("record-file path is not relative: {}", path.display()),
        }
    }
    Ok(parts.join("/"))
}
