use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::db::Database;
use crate::models::Issue;

#[derive(Debug, Deserialize)]
struct Manifest {
    schema: String,
    schema_version: i64,
    format_version: i64,
    records: Vec<ManifestRecord>,
}

#[derive(Debug, Deserialize)]
struct ManifestRecord {
    path: String,
    kind: String,
    id: Option<String>,
    schema: String,
    schema_version: i64,
    role: String,
    sha256: String,
}

#[derive(Debug)]
struct CanonicalIssue {
    issue: Issue,
    labels: Vec<String>,
    blocks: Vec<i64>,
    depends_on: Vec<i64>,
}

#[derive(Debug)]
struct RebuildProjection {
    issues: Vec<CanonicalIssue>,
    relations: Vec<(i64, i64, String)>,
}

pub fn run(state_dir: &Path, db_path: &Path) -> Result<()> {
    let rebuild = load_projection(state_dir)?;
    write_rebuilt_database(db_path, &rebuild)?;
    eprintln!("Rebuilt {} from {}", db_path.display(), state_dir.display());
    Ok(())
}

fn load_projection(state_dir: &Path) -> Result<RebuildProjection> {
    let manifest_path = state_dir.join("manifest.json");
    let manifest_bytes = fs::read(&manifest_path)
        .with_context(|| format!("Missing canonical manifest {}", manifest_path.display()))?;
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes)
        .with_context(|| format!("Invalid JSON in {}", manifest_path.display()))?;
    validate_manifest(&manifest)?;

    let records = manifest_records_by_path(&manifest)?;
    let mut issues = Vec::new();
    let mut issue_ids = BTreeSet::new();

    for record in records.values().filter(|record| record.kind == "issue") {
        validate_record_hash(state_dir, record)?;
        let issue = load_issue_record(state_dir, record)?;
        if !issue_ids.insert(issue.issue.id) {
            bail!(
                "Duplicate issue ID in canonical projection: {}",
                record.id()
            );
        }
        issues.push(issue);
    }

    for issue in &issues {
        if let Some(parent_id) = issue.issue.parent_id {
            ensure_issue_exists(parent_id, &issue_ids, "parent", issue.issue.id)?;
        }
        for blocked_id in &issue.blocks {
            ensure_issue_exists(*blocked_id, &issue_ids, "blocks", issue.issue.id)?;
        }
        for blocker_id in &issue.depends_on {
            ensure_issue_exists(*blocker_id, &issue_ids, "depends_on", issue.issue.id)?;
        }
    }

    let graph_record = records
        .values()
        .find(|record| record.kind == "graph" && record.role == "canonical")
        .ok_or_else(|| anyhow!("manifest is missing canonical graph.json record"))?;
    validate_record_hash(state_dir, graph_record)?;
    let relations = load_graph_relations(state_dir, graph_record, &issue_ids)?;

    issues.sort_by_key(|issue| issue.issue.id);
    Ok(RebuildProjection { issues, relations })
}

fn validate_manifest(manifest: &Manifest) -> Result<()> {
    if manifest.schema != "atelier.manifest" {
        bail!(
            "Unsupported manifest schema '{}'; expected atelier.manifest",
            manifest.schema
        );
    }
    if manifest.schema_version != 1 {
        bail!(
            "Unsupported manifest schema_version {}; expected 1",
            manifest.schema_version
        );
    }
    if manifest.format_version != 1 {
        bail!(
            "Unsupported canonical export format_version {}; expected 1",
            manifest.format_version
        );
    }
    Ok(())
}

fn manifest_records_by_path(manifest: &Manifest) -> Result<BTreeMap<PathBuf, &ManifestRecord>> {
    let mut records = BTreeMap::new();
    for record in &manifest.records {
        let relative = state_relative_path(&record.path)?;
        if records.insert(relative.clone(), record).is_some() {
            bail!(
                "Duplicate manifest record path: {}",
                display_state_path(&relative)
            );
        }
        validate_manifest_record(record, &relative)?;
    }
    Ok(records)
}

fn validate_manifest_record(record: &ManifestRecord, relative: &Path) -> Result<()> {
    if record.schema_version != 1 {
        bail!(
            "Unsupported schema_version {} for {}; expected 1",
            record.schema_version,
            display_state_path(relative)
        );
    }
    match (
        record.kind.as_str(),
        record.schema.as_str(),
        record.role.as_str(),
    ) {
        ("issue", "atelier.issue", "canonical") => {
            let id = record.id();
            let expected = PathBuf::from("issues").join(format!("{id}.md"));
            if relative != expected {
                bail!(
                    "Manifest path {} does not match issue id {}",
                    display_state_path(relative),
                    id
                );
            }
        }
        ("graph", "atelier.graph", "canonical") if relative == Path::new("graph.json") => {}
        ("mission-control", "atelier.mission-control", "derived") => {}
        _ if record.role == "derived" => {}
        _ => bail!(
            "Unsupported manifest record {} with kind '{}', schema '{}', role '{}'",
            display_state_path(relative),
            record.kind,
            record.schema,
            record.role
        ),
    }

    if record.role == "canonical" && record.sha256.len() != 64 {
        bail!(
            "Invalid sha256 for {}; expected 64 lowercase hex characters",
            display_state_path(relative)
        );
    }
    Ok(())
}

fn validate_record_hash(state_dir: &Path, record: &ManifestRecord) -> Result<Vec<u8>> {
    let relative = state_relative_path(&record.path)?;
    let path = state_dir.join(&relative);
    let bytes = fs::read(&path)
        .with_context(|| format!("Missing projection file {}", display_state_path(&relative)))?;
    let actual = sha256_hex(&bytes);
    if actual != record.sha256 {
        bail!(
            "Content hash mismatch for {}: expected {}, got {}",
            display_state_path(&relative),
            record.sha256,
            actual
        );
    }
    Ok(bytes)
}

fn load_issue_record(state_dir: &Path, record: &ManifestRecord) -> Result<CanonicalIssue> {
    let relative = state_relative_path(&record.path)?;
    let bytes = validate_record_hash(state_dir, record)?;
    let text = String::from_utf8(bytes).with_context(|| {
        format!(
            "Projection file {} is not UTF-8",
            display_state_path(&relative)
        )
    })?;
    let (front_matter, body) = split_front_matter(&text, &relative)?;

    require_scalar(&front_matter, "schema", &relative).and_then(|schema| {
        if schema == "atelier.issue" {
            Ok(())
        } else {
            bail!(
                "Unsupported schema '{}' in {}; expected atelier.issue",
                schema,
                display_state_path(&relative)
            )
        }
    })?;
    let schema_version = require_i64(&front_matter, "schema_version", &relative)?;
    if schema_version != 1 {
        bail!(
            "Unsupported schema_version {} in {}; expected 1",
            schema_version,
            display_state_path(&relative)
        );
    }

    let canonical_id = require_scalar(&front_matter, "id", &relative)?;
    if Some(canonical_id.as_str()) != record.id.as_deref() {
        bail!(
            "Issue id {} in {} does not match manifest id {:?}",
            canonical_id,
            display_state_path(&relative),
            record.id
        );
    }
    let id = parse_issue_id(&canonical_id).with_context(|| {
        format!(
            "Invalid issue id {} in {}",
            canonical_id,
            display_state_path(&relative)
        )
    })?;

    let parent_id = optional_issue_id(&front_matter, "parent", &relative)?;
    let blocks = issue_id_array(&front_matter, "blocks", &relative)?;
    let depends_on = issue_id_array(&front_matter, "depends_on", &relative)?;
    let status = require_scalar(&front_matter, "status", &relative)?;
    let updated_at = require_datetime(&front_matter, "updated_at", &relative)?;
    let description = if body.is_empty() {
        None
    } else {
        Some(body.to_string())
    };

    Ok(CanonicalIssue {
        issue: Issue {
            id,
            title: require_scalar(&front_matter, "title", &relative)?,
            description,
            status: status.clone(),
            priority: db_priority(&require_scalar(&front_matter, "priority", &relative)?)
                .with_context(|| {
                    format!("Invalid priority in {}", display_state_path(&relative))
                })?,
            parent_id,
            created_at: require_datetime(&front_matter, "created_at", &relative)?,
            updated_at,
            closed_at: (status == "closed").then_some(updated_at),
        },
        labels: string_array(&front_matter, "labels", &relative)?,
        blocks,
        depends_on,
    })
}

fn load_graph_relations(
    state_dir: &Path,
    record: &ManifestRecord,
    issue_ids: &BTreeSet<i64>,
) -> Result<Vec<(i64, i64, String)>> {
    let relative = state_relative_path(&record.path)?;
    let bytes = validate_record_hash(state_dir, record)?;
    let graph: Value = serde_json::from_slice(&bytes)
        .with_context(|| format!("Invalid JSON in {}", display_state_path(&relative)))?;
    if graph.get("schema").and_then(Value::as_str) != Some("atelier.graph") {
        bail!(
            "Unsupported graph schema in {}; expected atelier.graph",
            display_state_path(&relative)
        );
    }
    if graph.get("schema_version").and_then(Value::as_i64) != Some(1) {
        bail!(
            "Unsupported graph schema_version in {}; expected 1",
            display_state_path(&relative)
        );
    }

    for node in graph
        .get("nodes")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("graph nodes must be an array"))?
    {
        let kind = node.get("kind").and_then(Value::as_str).unwrap_or("");
        let id = node.get("id").and_then(Value::as_str).unwrap_or("");
        if kind != "issue" {
            bail!(
                "Unsupported graph node kind '{}' in {}",
                kind,
                display_state_path(&relative)
            );
        }
        let issue_id = parse_issue_id(id).with_context(|| {
            format!(
                "Invalid graph node id '{}' in {}",
                id,
                display_state_path(&relative)
            )
        })?;
        if !issue_ids.contains(&issue_id) {
            bail!("Graph node {} references a missing issue record", id);
        }
    }

    let mut relations = Vec::new();
    for edge in graph
        .get("edges")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("graph edges must be an array"))?
    {
        for field in ["source_kind", "target_kind"] {
            if edge.get(field).and_then(Value::as_str) != Some("issue") {
                bail!(
                    "Graph edge has unsupported {} in {}",
                    field,
                    display_state_path(&relative)
                );
            }
        }
        let source = graph_edge_issue_id(edge, "source_id", issue_ids, &relative)?;
        let target = graph_edge_issue_id(edge, "target_id", issue_ids, &relative)?;
        let relation_type = edge.get("type").and_then(Value::as_str).ok_or_else(|| {
            anyhow!(
                "Graph edge is missing type in {}",
                display_state_path(&relative)
            )
        })?;
        let metadata_source = edge
            .get("metadata")
            .and_then(|metadata| metadata.get("source"))
            .and_then(Value::as_str);
        if metadata_source == Some("relation") {
            relations.push((source, target, relation_type.to_string()));
        }
    }

    Ok(relations)
}

fn graph_edge_issue_id(
    edge: &Value,
    field: &str,
    issue_ids: &BTreeSet<i64>,
    relative: &Path,
) -> Result<i64> {
    let id = edge.get(field).and_then(Value::as_str).unwrap_or("");
    let issue_id = parse_issue_id(id).with_context(|| {
        format!(
            "Invalid graph edge {} '{}' in {}",
            field,
            id,
            display_state_path(relative)
        )
    })?;
    if !issue_ids.contains(&issue_id) {
        bail!("Graph edge {} references a missing issue record", id);
    }
    Ok(issue_id)
}

fn write_rebuilt_database(db_path: &Path, rebuild: &RebuildProjection) -> Result<()> {
    let parent = db_path.parent().ok_or_else(|| {
        anyhow!(
            "Cannot determine parent directory for {}",
            db_path.display()
        )
    })?;
    fs::create_dir_all(parent).with_context(|| format!("Failed to create {}", parent.display()))?;

    let tmp_path = db_path.with_extension("rebuild-tmp");
    if tmp_path.exists() {
        fs::remove_file(&tmp_path)
            .with_context(|| format!("Failed to remove stale {}", tmp_path.display()))?;
    }

    {
        let db = Database::open(&tmp_path)?;
        db.transaction(|| {
            for issue in &rebuild.issues {
                let mut row = issue.issue.clone();
                row.parent_id = None;
                db.insert_issue_rebuild(&row)?;
            }
            for issue in &rebuild.issues {
                if issue.issue.parent_id.is_some() {
                    db.update_parent_import(
                        issue.issue.id,
                        issue.issue.parent_id,
                        &issue.issue.updated_at,
                    )?;
                }
            }
            for issue in &rebuild.issues {
                for label in &issue.labels {
                    db.add_label(issue.issue.id, label)?;
                }
                for blocked_id in &issue.blocks {
                    db.add_dependency(*blocked_id, issue.issue.id)?;
                }
                for blocker_id in &issue.depends_on {
                    db.add_dependency(issue.issue.id, *blocker_id)?;
                }
            }
            for (source, target, relation_type) in &rebuild.relations {
                db.add_typed_relation(*source, *target, relation_type)?;
            }
            Ok(())
        })?;
    }

    if db_path.exists() {
        fs::remove_file(db_path)
            .with_context(|| format!("Failed to replace {}", db_path.display()))?;
    }
    fs::rename(&tmp_path, db_path).with_context(|| {
        format!(
            "Failed to move rebuilt database from {} to {}",
            tmp_path.display(),
            db_path.display()
        )
    })?;
    Ok(())
}

fn split_front_matter<'a>(
    text: &'a str,
    relative: &Path,
) -> Result<(BTreeMap<String, Value>, &'a str)> {
    let rest = text.strip_prefix("---\n").ok_or_else(|| {
        anyhow!(
            "Missing YAML front matter in {}",
            display_state_path(relative)
        )
    })?;
    let (front, body) = rest.split_once("\n---\n").ok_or_else(|| {
        anyhow!(
            "Unterminated YAML front matter in {}",
            display_state_path(relative)
        )
    })?;
    let body = body.strip_prefix('\n').unwrap_or(body);
    let body = body.strip_suffix('\n').unwrap_or(body);
    Ok((parse_front_matter(front, relative)?, body))
}

fn parse_front_matter(front: &str, relative: &Path) -> Result<BTreeMap<String, Value>> {
    let mut values = BTreeMap::new();
    let mut lines = front.lines().peekable();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }
        let (key, value) = line.split_once(':').ok_or_else(|| {
            anyhow!(
                "Invalid front matter line in {}: {}",
                display_state_path(relative),
                line
            )
        })?;
        let key = key.to_string();
        let value = value.trim_start();
        if value.is_empty() {
            let mut array = Vec::new();
            while let Some(next) = lines.peek() {
                if !next.starts_with("- ") {
                    break;
                }
                let item = lines.next().unwrap().trim_start_matches("- ");
                array.push(parse_yaml_value(item, relative)?);
            }
            values.insert(key, Value::Array(array));
        } else {
            values.insert(key, parse_yaml_value(value, relative)?);
        }
    }
    Ok(values)
}

fn parse_yaml_value(value: &str, relative: &Path) -> Result<Value> {
    match value {
        "null" => Ok(Value::Null),
        "[]" => Ok(Value::Array(Vec::new())),
        value if value.starts_with('"') => serde_json::from_str(value).with_context(|| {
            format!(
                "Invalid quoted front matter value '{}' in {}",
                value,
                display_state_path(relative)
            )
        }),
        value => value
            .parse::<i64>()
            .map(|number| Value::Number(number.into()))
            .with_context(|| {
                format!(
                    "Unsupported front matter value '{}' in {}",
                    value,
                    display_state_path(relative)
                )
            }),
    }
}

fn require_scalar(values: &BTreeMap<String, Value>, key: &str, relative: &Path) -> Result<String> {
    values
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| {
            anyhow!(
                "Missing string front matter key '{}' in {}",
                key,
                display_state_path(relative)
            )
        })
}

fn require_i64(values: &BTreeMap<String, Value>, key: &str, relative: &Path) -> Result<i64> {
    values.get(key).and_then(Value::as_i64).ok_or_else(|| {
        anyhow!(
            "Missing integer front matter key '{}' in {}",
            key,
            display_state_path(relative)
        )
    })
}

fn require_datetime(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<DateTime<Utc>> {
    let value = require_scalar(values, key, relative)?;
    DateTime::parse_from_rfc3339(&value)
        .map(|dt| dt.with_timezone(&Utc))
        .with_context(|| {
            format!(
                "Invalid timestamp '{}' for key '{}' in {}",
                value,
                key,
                display_state_path(relative)
            )
        })
}

fn string_array(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Vec<String>> {
    let values = values.get(key).and_then(Value::as_array).ok_or_else(|| {
        anyhow!(
            "Missing array front matter key '{}' in {}",
            key,
            display_state_path(relative)
        )
    })?;
    values
        .iter()
        .map(|value| {
            value.as_str().map(str::to_string).ok_or_else(|| {
                anyhow!(
                    "Array '{}' in {} must contain only strings",
                    key,
                    display_state_path(relative)
                )
            })
        })
        .collect()
}

fn issue_id_array(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Vec<i64>> {
    string_array(values, key, relative)?
        .into_iter()
        .map(|value| {
            parse_issue_id(&value).with_context(|| {
                format!(
                    "Invalid issue id '{}' in key '{}' of {}",
                    value,
                    key,
                    display_state_path(relative)
                )
            })
        })
        .collect()
}

fn optional_issue_id(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Option<i64>> {
    match values.get(key) {
        Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => parse_issue_id(value).map(Some).with_context(|| {
            format!(
                "Invalid issue id '{}' in key '{}' of {}",
                value,
                key,
                display_state_path(relative)
            )
        }),
        _ => bail!(
            "Key '{}' in {} must be an issue id string or null",
            key,
            display_state_path(relative)
        ),
    }
}

fn ensure_issue_exists(
    id: i64,
    issue_ids: &BTreeSet<i64>,
    relation: &str,
    source_id: i64,
) -> Result<()> {
    if issue_ids.contains(&id) {
        Ok(())
    } else {
        bail!(
            "Issue ISS-{source_id:04} has {} reference to missing issue ISS-{id:04}",
            relation
        )
    }
}

fn parse_issue_id(id: &str) -> Result<i64> {
    let suffix = id
        .strip_prefix("ISS-")
        .ok_or_else(|| anyhow!("expected ISS-<number>"))?;
    if suffix.is_empty() || !suffix.chars().all(|c| c.is_ascii_digit()) {
        bail!("expected ISS-<number>");
    }
    let number = suffix.parse::<i64>()?;
    if number <= 0 {
        bail!("issue id suffix must be positive");
    }
    Ok(number)
}

fn db_priority(priority: &str) -> Result<String> {
    match priority {
        "P0" => Ok("critical".to_string()),
        "P1" => Ok("high".to_string()),
        "P2" => Ok("medium".to_string()),
        "P3" => Ok("low".to_string()),
        "critical" | "high" | "medium" | "low" => Ok(priority.to_string()),
        other => bail!("unsupported canonical priority '{}'", other),
    }
}

fn state_relative_path(path: &str) -> Result<PathBuf> {
    let relative = path
        .strip_prefix(".atelier-state/")
        .ok_or_else(|| anyhow!("Manifest path '{}' must start with .atelier-state/", path))?;
    let path = PathBuf::from(relative);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        bail!(
            "Manifest path '{}' must stay within .atelier-state/",
            path.display()
        );
    }
    Ok(path)
}

fn display_state_path(relative_path: &Path) -> String {
    format!(
        ".atelier-state/{}",
        relative_path.to_string_lossy().replace('\\', "/")
    )
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

trait ManifestRecordExt {
    fn id(&self) -> String;
}

impl ManifestRecordExt for ManifestRecord {
    fn id(&self) -> String {
        self.id.clone().unwrap_or_else(|| "<none>".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    use crate::commands::export;

    fn setup_test_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::open(&db_path).unwrap();
        (db, dir)
    }

    #[test]
    fn rebuild_round_trips_canonical_issue_state() {
        let (db, dir) = setup_test_db();
        let parent = db
            .create_issue("Parent", Some("Parent body"), "high")
            .unwrap();
        let child = db
            .create_subissue(parent, "Child", Some("Child body"), "low")
            .unwrap();
        db.add_label(child, "alpha").unwrap();
        db.add_label(child, "zeta").unwrap();
        db.add_dependency(child, parent).unwrap();
        db.add_typed_relation(parent, child, "derived").unwrap();

        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let rebuilt_path = dir.path().join(".atelier/state.db");
        run(&state_dir, &rebuilt_path).unwrap();
        let rebuilt = Database::open(&rebuilt_path).unwrap();

        let rebuilt_parent = rebuilt.get_issue(parent).unwrap().unwrap();
        let rebuilt_child = rebuilt.get_issue(child).unwrap().unwrap();
        assert_eq!(rebuilt_parent.title, "Parent");
        assert_eq!(rebuilt_child.title, "Child");
        assert_eq!(rebuilt_child.parent_id, Some(parent));
        assert_eq!(rebuilt_child.priority, "low");
        assert_eq!(rebuilt.get_labels(child).unwrap(), vec!["alpha", "zeta"]);
        assert_eq!(rebuilt.get_blockers(child).unwrap(), vec![parent]);
        assert_eq!(rebuilt.get_blocking(parent).unwrap(), vec![child]);

        let rebuilt_state_dir = dir.path().join(".rebuilt-state");
        export::run_canonical(&rebuilt, &rebuilt_state_dir, false).unwrap();
        assert_eq!(
            fs::read_to_string(state_dir.join("issues/ISS-0002.md")).unwrap(),
            fs::read_to_string(rebuilt_state_dir.join("issues/ISS-0002.md")).unwrap()
        );
        assert_eq!(
            fs::read_to_string(state_dir.join("graph.json")).unwrap(),
            fs::read_to_string(rebuilt_state_dir.join("graph.json")).unwrap()
        );
    }

    #[test]
    fn rebuild_allows_parent_records_after_children() {
        let (db, dir) = setup_test_db();
        let child = db.create_issue("Child", Some("Child body"), "low").unwrap();
        let parent = db
            .create_issue("Parent", Some("Parent body"), "high")
            .unwrap();
        db.update_parent(child, Some(parent)).unwrap();

        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let rebuilt_path = dir.path().join(".atelier/state.db");
        run(&state_dir, &rebuilt_path).unwrap();
        let rebuilt = Database::open(&rebuilt_path).unwrap();

        let rebuilt_child = rebuilt.get_issue(child).unwrap().unwrap();
        assert_eq!(rebuilt_child.parent_id, Some(parent));
    }

    #[test]
    fn rebuild_reports_missing_manifest_record_file() {
        let (db, dir) = setup_test_db();
        db.create_issue("Missing", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();
        fs::remove_file(state_dir.join("issues/ISS-0001.md")).unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/state.db")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Missing projection file .atelier-state/issues/ISS-0001.md"));
    }

    #[test]
    fn rebuild_reports_hash_mismatch() {
        let (db, dir) = setup_test_db();
        db.create_issue("Corrupt", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();
        fs::write(
            state_dir.join("issues/ISS-0001.md"),
            "not the bytes in manifest\n",
        )
        .unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/state.db")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Content hash mismatch for .atelier-state/issues/ISS-0001.md"));
    }

    #[test]
    fn rebuild_rejects_future_manifest_format() {
        let (db, dir) = setup_test_db();
        db.create_issue("Future", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();
        let manifest_path = state_dir.join("manifest.json");
        let manifest = fs::read_to_string(&manifest_path)
            .unwrap()
            .replace("\"format_version\": 1", "\"format_version\": 99");
        fs::write(manifest_path, manifest).unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/state.db")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Unsupported canonical export format_version 99"));
    }
}
