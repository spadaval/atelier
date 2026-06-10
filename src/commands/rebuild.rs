use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::db::{validate_relation_type, Database};
use crate::models::{DomainRecord, Issue};
use crate::record_id;

#[derive(Debug)]
struct CanonicalIssue {
    issue: Issue,
    labels: Vec<String>,
    blocks: Vec<String>,
    depends_on: Vec<String>,
    relations: Vec<(String, String, String)>,
}

#[derive(Debug)]
struct RebuildProjection {
    issues: Vec<CanonicalIssue>,
    records: Vec<CanonicalRecord>,
    relations: Vec<(String, String, String)>,
    record_links: Vec<(String, String, String, String, String)>,
}

#[derive(Debug)]
struct CanonicalRecord {
    record: DomainRecord,
    links: Vec<(String, String, String, String, String)>,
}

pub fn run(state_dir: &Path, db_path: &Path) -> Result<()> {
    let rebuild = load_projection(state_dir)?;
    write_rebuilt_database(db_path, &rebuild)?;
    eprintln!("Rebuilt {} from {}", db_path.display(), state_dir.display());
    Ok(())
}

pub(crate) fn validate_canonical_state(state_dir: &Path) -> Result<()> {
    load_projection(state_dir).map(|_| ())
}

fn load_projection(state_dir: &Path) -> Result<RebuildProjection> {
    let mut issues = Vec::new();
    let mut records = Vec::new();
    let mut issue_ids = BTreeSet::new();
    let mut record_refs = BTreeSet::new();
    let mut canonical_paths = BTreeSet::new();

    for relative in discover_issue_record_paths(state_dir)? {
        canonical_paths.insert(relative.clone());
        let issue = load_issue_record(state_dir, &relative)?;
        if !issue_ids.insert(issue.issue.id.clone()) {
            bail!(
                "Duplicate issue ID in canonical projection: {}",
                issue.issue.id
            );
        }
        issues.push(issue);
    }
    for kind in ["mission", "milestone", "plan", "evidence"] {
        for relative in discover_record_paths(state_dir, kind)? {
            canonical_paths.insert(relative.clone());
            let record = load_domain_record(state_dir, &relative, kind)?;
            if !record_refs.insert((record.record.kind.clone(), record.record.id.clone())) {
                bail!(
                    "Duplicate {} ID in canonical projection: {}",
                    record.record.kind,
                    record.record.id
                );
            }
            records.push(record);
        }
    }
    ensure_no_unsupported_canonical_files(state_dir, &canonical_paths)?;

    let mut relations = Vec::new();
    let mut relation_keys = BTreeSet::new();
    for issue in &issues {
        if let Some(parent_id) = &issue.issue.parent_id {
            ensure_issue_exists(parent_id, &issue_ids, "parent", &issue.issue.id)?;
        }
        for blocked_id in &issue.blocks {
            ensure_issue_exists(blocked_id, &issue_ids, "blocks", &issue.issue.id)?;
        }
        for blocker_id in &issue.depends_on {
            ensure_issue_exists(blocker_id, &issue_ids, "depends_on", &issue.issue.id)?;
        }

        for relation in &issue.relations {
            ensure_issue_exists(&relation.1, &issue_ids, &relation.2, &issue.issue.id)?;
            let key = (relation.0.clone(), relation.1.clone(), relation.2.clone());
            if !relation_keys.insert(key.clone()) {
                bail!("Duplicate typed link {} -> {} ({})", key.0, key.1, key.2);
            }
            relations.push(key);
        }
    }

    let mut record_links = Vec::new();
    let mut record_link_keys = BTreeSet::new();
    for record in &records {
        for link in &record.links {
            let target_exists = if link.2 == "issue" {
                issue_ids.contains(&link.3)
            } else {
                record_refs.contains(&(link.2.clone(), link.3.clone()))
            };
            if !target_exists {
                bail!(
                    "Unknown {} link target {} from {} {}",
                    link.2,
                    link.3,
                    record.record.kind,
                    record.record.id
                );
            }
            if !record_link_keys.insert(link.clone()) {
                bail!(
                    "Duplicate typed link {} {} -> {} {} ({})",
                    link.0,
                    link.1,
                    link.2,
                    link.3,
                    link.4
                );
            }
            record_links.push(link.clone());
        }
    }

    issues.sort_by(|a, b| a.issue.id.cmp(&b.issue.id));
    records.sort_by(|a, b| (&a.record.kind, &a.record.id).cmp(&(&b.record.kind, &b.record.id)));
    Ok(RebuildProjection {
        issues,
        records,
        relations,
        record_links,
    })
}

fn discover_issue_record_paths(state_dir: &Path) -> Result<Vec<PathBuf>> {
    let issue_dir = state_dir.join("issues");
    if !issue_dir.exists() {
        return Ok(Vec::new());
    }

    let mut records = Vec::new();
    collect_issue_record_paths(state_dir, &issue_dir, &mut records)?;
    records.sort();
    Ok(records)
}

fn collect_issue_record_paths(root: &Path, dir: &Path, records: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_issue_record_paths(root, &path, records)?;
        } else if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .context("Failed to relativize canonical issue path")?
                .to_path_buf();
            if relative.extension().and_then(|ext| ext.to_str()) != Some("md") {
                bail!(
                    "Unsupported canonical issue file {}; expected Markdown .md record",
                    display_state_path(&relative)
                );
            }
            records.push(relative);
        }
    }
    Ok(())
}

fn discover_record_paths(state_dir: &Path, kind: &str) -> Result<Vec<PathBuf>> {
    let dir_name = record_dir(kind);
    let record_dir = state_dir.join(dir_name);
    if !record_dir.exists() {
        return Ok(Vec::new());
    }
    let mut records = Vec::new();
    collect_issue_record_paths(state_dir, &record_dir, &mut records)?;
    records.sort();
    Ok(records)
}

fn ensure_no_unsupported_canonical_files(
    state_dir: &Path,
    expected: &BTreeSet<PathBuf>,
) -> Result<()> {
    if !state_dir.exists() {
        return Ok(());
    }
    for relative in canonical_files_under(state_dir)? {
        if relative == Path::new("manifest.json") || relative == Path::new("graph.json") {
            continue;
        }
        if (relative.starts_with("issues")
            || relative.starts_with("missions")
            || relative.starts_with("milestones")
            || relative.starts_with("plans")
            || relative.starts_with("evidence"))
            && expected.contains(&relative)
        {
            continue;
        }
        if relative == Path::new("mission-control.json") {
            continue;
        }
        bail!(
            "Unsupported canonical projection file {}",
            display_state_path(&relative)
        );
    }
    Ok(())
}

fn load_domain_record(state_dir: &Path, relative: &Path, kind: &str) -> Result<CanonicalRecord> {
    let bytes = fs::read(state_dir.join(relative))
        .with_context(|| format!("Missing projection file {}", display_state_path(relative)))?;
    let text = String::from_utf8(bytes).with_context(|| {
        format!(
            "Projection file {} is not UTF-8",
            display_state_path(relative)
        )
    })?;
    let (front_matter, body) = split_front_matter(&text, relative)?;

    let expected_schema = format!("atelier.{kind}");
    let schema = require_scalar(&front_matter, "schema", relative)?;
    if schema != expected_schema {
        bail!(
            "Unsupported schema '{}' in {}; expected {}",
            schema,
            display_state_path(relative),
            expected_schema
        );
    }
    let schema_version = require_i64(&front_matter, "schema_version", relative)?;
    if schema_version != 1 {
        bail!(
            "Unsupported schema_version {} in {}; expected 1",
            schema_version,
            display_state_path(relative)
        );
    }

    let id = require_scalar(&front_matter, "id", relative)?;
    record_id::validate_record_id(&id).with_context(|| {
        format!(
            "Invalid {} id {} in {}",
            kind,
            id,
            display_state_path(relative)
        )
    })?;
    let expected = record_path(kind, &id);
    if relative != expected {
        bail!(
            "{} id {} in {} does not match canonical path {}",
            kind,
            id,
            display_state_path(relative),
            display_state_path(&expected)
        );
    }
    let data_json = require_scalar(&front_matter, "data", relative)?;
    let _: Value = serde_json::from_str(&data_json)
        .with_context(|| format!("Invalid data JSON in {}", display_state_path(relative)))?;
    let links = record_links(&front_matter, "links", kind, &id, relative)?;
    let body = if body.is_empty() {
        None
    } else {
        Some(body.to_string())
    };
    Ok(CanonicalRecord {
        record: DomainRecord {
            id,
            kind: kind.to_string(),
            title: require_scalar(&front_matter, "title", relative)?,
            status: require_scalar(&front_matter, "status", relative)?,
            body,
            data_json,
            created_at: require_datetime(&front_matter, "created_at", relative)?,
            updated_at: require_datetime(&front_matter, "updated_at", relative)?,
        },
        links,
    })
}

fn canonical_files_under(state_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_canonical_files(state_dir, state_dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_canonical_files(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_canonical_files(root, &path, files)?;
        } else if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .context("Failed to relativize canonical projection path")?;
            files.push(relative.to_path_buf());
        }
    }
    Ok(())
}

fn load_issue_record(state_dir: &Path, relative: &Path) -> Result<CanonicalIssue> {
    let bytes = fs::read(state_dir.join(relative))
        .with_context(|| format!("Missing projection file {}", display_state_path(relative)))?;
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
    record_id::validate_record_id(&canonical_id).with_context(|| {
        format!(
            "Invalid issue id {} in {}",
            canonical_id,
            display_state_path(&relative)
        )
    })?;
    let id = canonical_id.clone();
    let expected = issue_record_path(&id);
    if relative != expected {
        bail!(
            "Issue id {} in {} does not match canonical path {}",
            canonical_id,
            display_state_path(relative),
            display_state_path(&expected)
        );
    }

    let parent_id = optional_issue_id(&front_matter, "parent", &relative)?;
    let blocks = issue_id_array(&front_matter, "blocks", &relative)?;
    let depends_on = issue_id_array(&front_matter, "depends_on", &relative)?;
    let relations = typed_links(&front_matter, "links", &id, &relative)?;
    let status = require_scalar(&front_matter, "status", &relative)?;
    let issue_type = require_scalar(&front_matter, "issue_type", &relative)?;
    crate::db::validate_issue_type(&issue_type)
        .with_context(|| format!("Invalid issue_type in {}", display_state_path(&relative)))?;
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
            issue_type,
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
        relations,
    })
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
                        &issue.issue.id,
                        issue.issue.parent_id.as_deref(),
                        &issue.issue.updated_at,
                    )?;
                }
            }
            for issue in &rebuild.issues {
                for label in &issue.labels {
                    db.add_label(&issue.issue.id, label)?;
                }
                for blocked_id in &issue.blocks {
                    db.add_dependency(blocked_id, &issue.issue.id)?;
                }
                for blocker_id in &issue.depends_on {
                    db.add_dependency(&issue.issue.id, blocker_id)?;
                }
            }
            for (source, target, relation_type) in &rebuild.relations {
                db.add_typed_relation(&source, &target, relation_type)?;
            }
            for record in &rebuild.records {
                db.insert_record_rebuild(&record.record)?;
            }
            for (source_kind, source_id, target_kind, target_id, relation_type) in
                &rebuild.record_links
            {
                db.add_record_link(
                    source_kind,
                    source_id,
                    target_kind,
                    target_id,
                    relation_type,
                )?;
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
                if item.contains(": ") {
                    let mut object = serde_json::Map::new();
                    parse_yaml_object_field(item, &mut object, relative)?;
                    while let Some(next) = lines.peek() {
                        if !next.starts_with("  ") {
                            break;
                        }
                        let nested = lines.next().unwrap().trim_start();
                        parse_yaml_object_field(nested, &mut object, relative)?;
                    }
                    array.push(Value::Object(object));
                } else {
                    array.push(parse_yaml_value(item, relative)?);
                }
            }
            values.insert(key, Value::Array(array));
        } else {
            values.insert(key, parse_yaml_value(value, relative)?);
        }
    }
    Ok(values)
}

fn parse_yaml_object_field(
    field: &str,
    object: &mut serde_json::Map<String, Value>,
    relative: &Path,
) -> Result<()> {
    let (key, value) = field.split_once(": ").ok_or_else(|| {
        anyhow!(
            "Invalid object front matter field in {}: {}",
            display_state_path(relative),
            field
        )
    })?;
    if object
        .insert(key.to_string(), parse_yaml_value(value, relative)?)
        .is_some()
    {
        bail!(
            "Duplicate object front matter key '{}' in {}",
            key,
            display_state_path(relative)
        );
    }
    Ok(())
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
) -> Result<Vec<String>> {
    string_array(values, key, relative)?
        .into_iter()
        .map(|value| {
            record_id::validate_record_id(&value).with_context(|| {
                format!(
                    "Invalid issue id '{}' in key '{}' of {}",
                    value,
                    key,
                    display_state_path(relative)
                )
            })?;
            Ok(value)
        })
        .collect()
}

fn typed_links(
    values: &BTreeMap<String, Value>,
    key: &str,
    source_id: &str,
    relative: &Path,
) -> Result<Vec<(String, String, String)>> {
    let values = values.get(key).and_then(Value::as_array).ok_or_else(|| {
        anyhow!(
            "Missing array front matter key '{}' in {}",
            key,
            display_state_path(relative)
        )
    })?;

    let mut links = Vec::new();
    let mut seen = BTreeSet::new();
    for value in values {
        let object = value.as_object().ok_or_else(|| {
            anyhow!(
                "Array '{}' in {} must contain link objects",
                key,
                display_state_path(relative)
            )
        })?;
        let target_kind = object
            .get("target_kind")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                anyhow!(
                    "Link in {} is missing target_kind",
                    display_state_path(relative)
                )
            })?;
        if target_kind != "issue" {
            bail!(
                "Link in {} has unsupported target_kind '{}'; expected issue",
                display_state_path(relative),
                target_kind
            );
        }
        let target = object
            .get("target_id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                anyhow!(
                    "Link in {} is missing target_id",
                    display_state_path(relative)
                )
            })?;
        record_id::validate_record_id(target).with_context(|| {
            format!(
                "Invalid link target_id '{}' in {}",
                target,
                display_state_path(relative)
            )
        })?;
        let target_id = target.to_string();
        let relation_type = object
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Link in {} is missing type", display_state_path(relative)))?;
        validate_relation_type(relation_type).with_context(|| {
            format!(
                "Invalid link relation type '{}' in {}",
                relation_type,
                display_state_path(relative)
            )
        })?;
        let link = (
            source_id.to_string(),
            target_id.clone(),
            relation_type.to_string(),
        );
        if !seen.insert(link.clone()) {
            bail!(
                "Duplicate link {} -> {} ({}) in {}",
                source_id,
                target_id,
                relation_type,
                display_state_path(relative)
            );
        }
        links.push(link);
    }
    Ok(links)
}

fn record_links(
    values: &BTreeMap<String, Value>,
    key: &str,
    source_kind: &str,
    source_id: &str,
    relative: &Path,
) -> Result<Vec<(String, String, String, String, String)>> {
    let values = values.get(key).and_then(Value::as_array).ok_or_else(|| {
        anyhow!(
            "Missing array front matter key '{}' in {}",
            key,
            display_state_path(relative)
        )
    })?;

    let mut links = Vec::new();
    let mut seen = BTreeSet::new();
    for value in values {
        let object = value.as_object().ok_or_else(|| {
            anyhow!(
                "Array '{}' in {} must contain link objects",
                key,
                display_state_path(relative)
            )
        })?;
        let target_kind = object
            .get("target_kind")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                anyhow!(
                    "Link in {} is missing target_kind",
                    display_state_path(relative)
                )
            })?;
        crate::db::validate_record_kind(target_kind)?;
        let target_id = object
            .get("target_id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                anyhow!(
                    "Link in {} is missing target_id",
                    display_state_path(relative)
                )
            })?;
        record_id::validate_record_id(target_id)?;
        let relation_type = object
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Link in {} is missing type", display_state_path(relative)))?;
        crate::db::validate_link_type(relation_type)?;
        let link = (
            source_kind.to_string(),
            source_id.to_string(),
            target_kind.to_string(),
            target_id.to_string(),
            relation_type.to_string(),
        );
        if !seen.insert(link.clone()) {
            bail!(
                "Duplicate link {} {} -> {} {} ({}) in {}",
                source_kind,
                source_id,
                target_kind,
                target_id,
                relation_type,
                display_state_path(relative)
            );
        }
        links.push(link);
    }
    Ok(links)
}

fn record_dir(kind: &str) -> &str {
    match kind {
        "mission" => "missions",
        "milestone" => "milestones",
        "plan" => "plans",
        "evidence" => "evidence",
        _ => kind,
    }
}

fn record_path(kind: &str, id: &str) -> PathBuf {
    PathBuf::from(record_dir(kind)).join(format!("{id}.md"))
}

fn optional_issue_id(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Option<String>> {
    match values.get(key) {
        Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => {
            record_id::validate_record_id(value).with_context(|| {
                format!(
                    "Invalid issue id '{}' in key '{}' of {}",
                    value,
                    key,
                    display_state_path(relative)
                )
            })?;
            Ok(Some(value.clone()))
        }
        _ => bail!(
            "Key '{}' in {} must be an issue id string or null",
            key,
            display_state_path(relative)
        ),
    }
}

fn ensure_issue_exists(
    id: &str,
    issue_ids: &BTreeSet<String>,
    relation: &str,
    source_id: &str,
) -> Result<()> {
    if issue_ids.contains(id) {
        Ok(())
    } else {
        bail!(
            "Issue {source_id} has {} reference to missing issue {id}",
            relation
        )
    }
}

fn issue_record_path(id: &str) -> PathBuf {
    PathBuf::from("issues").join(format!("{id}.md"))
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

fn display_state_path(relative_path: &Path) -> String {
    format!(
        ".atelier-state/{}",
        relative_path.to_string_lossy().replace('\\', "/")
    )
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
            .create_subissue(&parent, "Child", Some("Child body"), "low")
            .unwrap();
        db.add_label(&child, "alpha").unwrap();
        db.add_label(&child, "zeta").unwrap();
        db.add_dependency(&child, &parent).unwrap();
        db.add_typed_relation(&parent, &child, "derived").unwrap();

        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let rebuilt_path = dir.path().join(".atelier/state.db");
        run(&state_dir, &rebuilt_path).unwrap();
        let rebuilt = Database::open(&rebuilt_path).unwrap();

        let rebuilt_parent = rebuilt.get_issue(&parent).unwrap().unwrap();
        let rebuilt_child = rebuilt.get_issue(&child).unwrap().unwrap();
        assert_eq!(rebuilt_parent.title, "Parent");
        assert_eq!(rebuilt_child.title, "Child");
        assert_eq!(rebuilt_child.parent_id, Some(parent.clone()));
        assert_eq!(rebuilt_child.priority, "low");
        assert_eq!(rebuilt.get_labels(&child).unwrap(), vec!["alpha", "zeta"]);
        assert_eq!(rebuilt.get_blockers(&child).unwrap(), vec![parent.clone()]);
        assert_eq!(rebuilt.get_blocking(&parent).unwrap(), vec![child.clone()]);

        let rebuilt_state_dir = dir.path().join(".rebuilt-state");
        export::run_canonical(&rebuilt, &rebuilt_state_dir, false).unwrap();
        assert_eq!(
            fs::read_to_string(state_dir.join(issue_record_path(&child))).unwrap(),
            fs::read_to_string(rebuilt_state_dir.join(issue_record_path(&child))).unwrap()
        );
        assert!(!rebuilt_state_dir.join("graph.json").exists());
        assert_eq!(rebuilt.get_typed_relations(&parent).unwrap().len(), 1);
    }

    #[test]
    fn rebuild_allows_parent_records_after_children() {
        let (db, dir) = setup_test_db();
        let child = db.create_issue("Child", Some("Child body"), "low").unwrap();
        let parent = db
            .create_issue("Parent", Some("Parent body"), "high")
            .unwrap();
        db.update_parent(&child, Some(&parent)).unwrap();

        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let rebuilt_path = dir.path().join(".atelier/state.db");
        run(&state_dir, &rebuilt_path).unwrap();
        let rebuilt = Database::open(&rebuilt_path).unwrap();

        let rebuilt_child = rebuilt.get_issue(&child).unwrap().unwrap();
        assert_eq!(rebuilt_child.parent_id, Some(parent.clone()));
    }

    #[test]
    fn rebuild_succeeds_without_manifest_or_graph() {
        let (db, dir) = setup_test_db();
        db.create_issue("Standalone", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();

        assert!(!state_dir.join("manifest.json").exists());
        assert!(!state_dir.join("graph.json").exists());
        run(&state_dir, &dir.path().join(".atelier/state.db")).unwrap();
    }

    #[test]
    fn rebuild_reports_path_id_mismatch() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Mismatch", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();
        let wrong_id = "atelier-zzzz";
        fs::rename(
            state_dir.join(issue_record_path(&id)),
            state_dir.join(issue_record_path(wrong_id)),
        )
        .unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/state.db")).unwrap_err();
        assert!(error.to_string().contains(&format!(
            "does not match canonical path .atelier-state/issues/{id}.md"
        )));
    }

    #[test]
    fn rebuild_reports_malformed_front_matter() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Malformed", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();
        fs::write(state_dir.join(issue_record_path(&id)), "not front matter\n").unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/state.db")).unwrap_err();
        assert!(error.to_string().contains(&format!(
            "Missing YAML front matter in .atelier-state/issues/{id}.md"
        )));
    }

    #[test]
    fn rebuild_reports_schema_mismatch() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Wrong schema", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();
        let path = state_dir.join(issue_record_path(&id));
        let text = fs::read_to_string(&path)
            .unwrap()
            .replace("schema: \"atelier.issue\"", "schema: \"atelier.graph\"");
        fs::write(path, text).unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/state.db")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Unsupported schema 'atelier.graph'"));
    }

    #[test]
    fn rebuild_reports_dangling_dependency_and_duplicate_link() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Source", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let missing_id = "atelier-zzzz";
        let path = state_dir.join(issue_record_path(&id));
        let text = fs::read_to_string(&path).unwrap().replace(
            "depends_on: []",
            &format!("depends_on:\n- \"{missing_id}\""),
        );
        fs::write(&path, text).unwrap();
        let error = run(&state_dir, &dir.path().join(".atelier/state.db")).unwrap_err();
        assert!(error.to_string().contains(&format!(
            "Issue {id} has depends_on reference to missing issue {missing_id}"
        )));

        let text = fs::read_to_string(&path)
            .unwrap()
            .replace(&format!("depends_on:\n- \"{missing_id}\""), "depends_on: []")
            .replace(
                "links: []",
                &format!(
                    "links:\n- target_id: \"{id}\"\n  target_kind: \"issue\"\n  type: \"related\"\n- target_id: \"{id}\"\n  target_kind: \"issue\"\n  type: \"related\""
                ),
            );
        fs::write(&path, text).unwrap();
        let error = run(&state_dir, &dir.path().join(".atelier/state.db")).unwrap_err();
        assert!(error
            .to_string()
            .contains(&format!("Duplicate link {id} -> {id} (related)")));
    }

    #[test]
    fn rebuild_reports_invalid_relation_type() {
        let (db, dir) = setup_test_db();
        let first = db.create_issue("First", None, "medium").unwrap();
        let second = db.create_issue("Second", None, "medium").unwrap();
        db.add_typed_relation(&first, &second, "related").unwrap();
        let state_dir = dir.path().join(".atelier-state");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let path = [first.as_str(), second.as_str()]
            .into_iter()
            .map(|id| state_dir.join(issue_record_path(id)))
            .find(|path| {
                fs::read_to_string(path)
                    .map(|text| text.contains("type: \"related\""))
                    .unwrap_or(false)
            })
            .unwrap();
        let text = fs::read_to_string(&path)
            .unwrap()
            .replace("type: \"related\"", "type: \"\"");
        fs::write(path, text).unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/state.db")).unwrap_err();
        assert!(error.to_string().contains("Invalid link relation type ''"));
    }
}
