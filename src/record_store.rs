use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::Issue;
use crate::record_id;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RecordKindSpec {
    pub kind: &'static str,
    pub schema: &'static str,
    pub schema_version: i64,
    pub canonical_dir: Option<&'static str>,
}

pub const ISSUE_KIND: RecordKindSpec = RecordKindSpec {
    kind: "issue",
    schema: "atelier.issue",
    schema_version: 1,
    canonical_dir: Some("issues"),
};

pub const FIRST_CLASS_RECORD_KINDS: &[RecordKindSpec] = &[
    RecordKindSpec {
        kind: "mission",
        schema: "atelier.mission",
        schema_version: 1,
        canonical_dir: Some("missions"),
    },
    RecordKindSpec {
        kind: "milestone",
        schema: "atelier.milestone",
        schema_version: 1,
        canonical_dir: Some("milestones"),
    },
    RecordKindSpec {
        kind: "plan",
        schema: "atelier.plan",
        schema_version: 1,
        canonical_dir: Some("plans"),
    },
    RecordKindSpec {
        kind: "evidence",
        schema: "atelier.evidence",
        schema_version: 1,
        canonical_dir: Some("evidence"),
    },
];

pub const NON_CANONICAL_RECORD_KINDS: &[RecordKindSpec] = &[RecordKindSpec {
    kind: "workflow_validator",
    schema: "atelier.workflow_validator",
    schema_version: 1,
    canonical_dir: None,
}];

pub fn record_kind(kind: &str) -> Option<&'static RecordKindSpec> {
    std::iter::once(&ISSUE_KIND)
        .chain(FIRST_CLASS_RECORD_KINDS.iter())
        .chain(NON_CANONICAL_RECORD_KINDS.iter())
        .find(|spec| spec.kind == kind)
}

pub fn canonical_record_kind(kind: &str) -> Result<&'static RecordKindSpec> {
    let Some(spec) = FIRST_CLASS_RECORD_KINDS
        .iter()
        .find(|spec| spec.kind == kind && spec.canonical_dir.is_some())
    else {
        bail!(
            "Record kind '{}' is not a canonical first-class record",
            kind
        );
    };
    Ok(spec)
}

pub fn validate_canonical_record_kind(kind: &str) -> Result<()> {
    canonical_record_kind(kind).map(|_| ())
}

pub fn validate_record_kind(kind: &str) -> Result<()> {
    if record_kind(kind).is_some() {
        Ok(())
    } else {
        bail!(
            "Invalid record kind '{}'. Valid values: {}",
            kind,
            all_record_kind_names().join(", ")
        )
    }
}

pub fn canonical_record_path(spec: &RecordKindSpec, id: &str) -> Result<PathBuf> {
    let Some(dir) = spec.canonical_dir else {
        bail!("Record kind '{}' has no canonical directory", spec.kind);
    };
    Ok(PathBuf::from(dir).join(format!("{id}.md")))
}

pub fn issue_record_path(id: &str) -> PathBuf {
    PathBuf::from(ISSUE_KIND.canonical_dir.expect("issue has canonical dir"))
        .join(format!("{id}.md"))
}

pub fn canonical_record_dirs() -> Vec<&'static str> {
    std::iter::once(ISSUE_KIND.canonical_dir.expect("issue has canonical dir"))
        .chain(
            FIRST_CLASS_RECORD_KINDS
                .iter()
                .filter_map(|spec| spec.canonical_dir),
        )
        .collect()
}

fn all_record_kind_names() -> Vec<&'static str> {
    std::iter::once(ISSUE_KIND.kind)
        .chain(FIRST_CLASS_RECORD_KINDS.iter().map(|spec| spec.kind))
        .chain(NON_CANONICAL_RECORD_KINDS.iter().map(|spec| spec.kind))
        .collect()
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IssueLink {
    pub relation_type: String,
    pub target_kind: String,
    pub target_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalIssueRecord {
    pub issue: Issue,
    pub labels: Vec<String>,
    pub blocks: Vec<String>,
    pub depends_on: Vec<String>,
    pub acceptance: Vec<String>,
    pub evidence_required: Vec<String>,
    pub links: Vec<IssueLink>,
}

pub struct RecordStore {
    state_dir: PathBuf,
}

impl RecordStore {
    pub fn new(state_dir: impl Into<PathBuf>) -> Self {
        Self {
            state_dir: state_dir.into(),
        }
    }

    pub fn discover_issue_paths(&self) -> Result<Vec<PathBuf>> {
        let issue_dir = self.state_dir.join("issues");
        if !issue_dir.exists() {
            return Ok(Vec::new());
        }

        let mut records = Vec::new();
        collect_issue_record_paths(&self.state_dir, &issue_dir, &mut records)?;
        records.sort();
        Ok(records)
    }

    pub fn load_issue(&self, relative: &Path) -> Result<CanonicalIssueRecord> {
        let bytes = fs::read(self.state_dir.join(relative))
            .with_context(|| format!("Missing projection file {}", display_state_path(relative)))?;
        let text = String::from_utf8(bytes).with_context(|| {
            format!(
                "Projection file {} is not UTF-8",
                display_state_path(relative)
            )
        })?;
        parse_issue_record(&text, relative)
    }

    pub fn load_issues(&self) -> Result<Vec<CanonicalIssueRecord>> {
        let mut records = Vec::new();
        let mut ids = BTreeSet::new();
        for relative in self.discover_issue_paths()? {
            let record = self.load_issue(&relative)?;
            if !ids.insert(record.issue.id.clone()) {
                bail!(
                    "Duplicate issue ID in canonical records: {}",
                    record.issue.id
                );
            }
            records.push(record);
        }
        records.sort_by(|a, b| a.issue.id.cmp(&b.issue.id));
        Ok(records)
    }

    pub fn allocate_issue_id(&self) -> Result<String> {
        record_id::allocate_issue_id(|candidate| self.canonical_id_exists(candidate))
    }

    pub fn canonical_id_exists(&self, id: &str) -> Result<bool> {
        for relative in canonical_record_dirs()
            .into_iter()
            .map(PathBuf::from)
            .map(|dir| dir.join(format!("{id}.md")))
        {
            if self.state_dir.join(relative).exists() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn write_issue_atomic(&self, record: &CanonicalIssueRecord) -> Result<()> {
        validate_issue_record(record, Path::new("<record>"))?;
        let relative = issue_record_path(&record.issue.id);
        let path = self.state_dir.join(&relative);
        let parent = path
            .parent()
            .ok_or_else(|| anyhow!("Issue path has no parent: {}", path.display()))?;
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
        let tmp_path = path.with_extension("md.tmp");
        fs::write(&tmp_path, render_issue_record(record)?)
            .with_context(|| format!("Failed to write {}", tmp_path.display()))?;
        fs::rename(&tmp_path, &path).with_context(|| {
            let _ = fs::remove_file(&tmp_path);
            format!(
                "Failed to atomically replace {} with {}",
                path.display(),
                tmp_path.display()
            )
        })?;
        Ok(())
    }
}

pub fn render_issue_record(record: &CanonicalIssueRecord) -> Result<String> {
    validate_issue_record(record, Path::new("<record>"))?;
    let mut labels = record.labels.clone();
    let mut blocks = record.blocks.clone();
    let mut depends_on = record.depends_on.clone();
    let mut links = record.links.clone();
    labels.sort();
    blocks.sort();
    depends_on.sort();
    links.sort();

    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_array(&mut output, "acceptance", &record.acceptance)?;
    write_yaml_array(&mut output, "blocks", &blocks)?;
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.issue.created_at.to_rfc3339()),
    )?;
    write_yaml_array(&mut output, "depends_on", &depends_on)?;
    write_yaml_array(&mut output, "evidence_required", &record.evidence_required)?;
    write_yaml_scalar(&mut output, "id", Some(&record.issue.id))?;
    write_yaml_scalar(&mut output, "issue_type", Some(&record.issue.issue_type))?;
    write_yaml_array(&mut output, "labels", &labels)?;
    write_yaml_links(&mut output, "links", &links)?;
    write_yaml_scalar(&mut output, "parent", record.issue.parent_id.as_deref())?;
    write_yaml_scalar(
        &mut output,
        "priority",
        Some(&canonical_priority(&record.issue.priority)),
    )?;
    write_yaml_scalar(&mut output, "schema", Some(ISSUE_KIND.schema))?;
    output.push_str(&format!("schema_version: {}\n", ISSUE_KIND.schema_version));
    write_yaml_scalar(&mut output, "status", Some(&record.issue.status))?;
    write_yaml_scalar(&mut output, "title", Some(&record.issue.title))?;
    write_yaml_scalar(
        &mut output,
        "updated_at",
        Some(&record.issue.updated_at.to_rfc3339()),
    )?;
    output.push_str("---\n\n");
    output.push_str(&normalize_body(
        record.issue.description.as_deref().unwrap_or(""),
    ));
    output.push('\n');
    Ok(output)
}

pub fn parse_issue_record(text: &str, relative: &Path) -> Result<CanonicalIssueRecord> {
    let (front_matter, body) = split_front_matter(text, relative)?;

    require_scalar(&front_matter, "schema", relative).and_then(|schema| {
        if schema == ISSUE_KIND.schema {
            Ok(())
        } else {
            bail!(
                "Unsupported schema '{}' in {}; expected {}",
                schema,
                display_state_path(relative),
                ISSUE_KIND.schema
            )
        }
    })?;
    let schema_version = require_i64(&front_matter, "schema_version", relative)?;
    if schema_version != ISSUE_KIND.schema_version {
        bail!(
            "Unsupported schema_version {} in {}; expected {}",
            schema_version,
            display_state_path(relative),
            ISSUE_KIND.schema_version
        );
    }

    let id = require_scalar(&front_matter, "id", relative)?;
    record_id::validate_record_id(&id).with_context(|| {
        format!(
            "Invalid issue id {} in {}",
            id,
            display_state_path(relative)
        )
    })?;
    let expected = issue_record_path(&id);
    if relative != expected {
        bail!(
            "Issue id {} in {} does not match canonical path {}",
            id,
            display_state_path(relative),
            display_state_path(&expected)
        );
    }

    let parent_id = optional_issue_id(&front_matter, "parent", relative)?;
    let blocks = issue_id_array(&front_matter, "blocks", relative)?;
    let depends_on = issue_id_array(&front_matter, "depends_on", relative)?;
    let acceptance = string_array(&front_matter, "acceptance", relative)?;
    let evidence_required = string_array(&front_matter, "evidence_required", relative)?;
    let links = typed_links(&front_matter, "links", relative)?;
    let status = require_scalar(&front_matter, "status", relative)?;
    crate::db::validate_status(&status)
        .with_context(|| format!("Invalid status in {}", display_state_path(relative)))?;
    let issue_type = require_scalar(&front_matter, "issue_type", relative)?;
    crate::db::validate_issue_type(&issue_type)
        .with_context(|| format!("Invalid issue_type in {}", display_state_path(relative)))?;
    let updated_at = require_datetime(&front_matter, "updated_at", relative)?;
    let description = if body.is_empty() {
        None
    } else {
        Some(body.to_string())
    };

    Ok(CanonicalIssueRecord {
        issue: Issue {
            id,
            title: require_scalar(&front_matter, "title", relative)?,
            description,
            status: status.clone(),
            issue_type,
            priority: db_priority(&require_scalar(&front_matter, "priority", relative)?)
                .with_context(|| format!("Invalid priority in {}", display_state_path(relative)))?,
            parent_id,
            created_at: require_datetime(&front_matter, "created_at", relative)?,
            updated_at,
            closed_at: (status == "closed").then_some(updated_at),
        },
        labels: string_array(&front_matter, "labels", relative)?,
        blocks,
        depends_on,
        acceptance,
        evidence_required,
        links,
    })
}

fn validate_issue_record(record: &CanonicalIssueRecord, relative: &Path) -> Result<()> {
    record_id::validate_record_id(&record.issue.id).with_context(|| {
        format!(
            "Invalid issue id '{}' in {}",
            record.issue.id,
            display_state_path(relative)
        )
    })?;
    crate::db::validate_status(&record.issue.status)
        .with_context(|| format!("Invalid status in {}", display_state_path(relative)))?;
    crate::db::validate_issue_type(&record.issue.issue_type)
        .with_context(|| format!("Invalid issue_type in {}", display_state_path(relative)))?;
    crate::db::validate_priority(&record.issue.priority)
        .with_context(|| format!("Invalid priority in {}", display_state_path(relative)))?;
    if let Some(parent_id) = record.issue.parent_id.as_deref() {
        record_id::validate_record_id(parent_id).with_context(|| {
            format!(
                "Invalid issue id '{}' in key 'parent' of {}",
                parent_id,
                display_state_path(relative)
            )
        })?;
    }
    validate_issue_ids(&record.blocks, "blocks", relative)?;
    validate_issue_ids(&record.depends_on, "depends_on", relative)?;
    for link in &record.links {
        if link.target_kind != "issue" {
            bail!(
                "Link in {} has unsupported target_kind '{}'; expected issue",
                display_state_path(relative),
                link.target_kind
            );
        }
        record_id::validate_record_id(&link.target_id).with_context(|| {
            format!(
                "Invalid link target_id '{}' in {}",
                link.target_id,
                display_state_path(relative)
            )
        })?;
        crate::db::validate_relation_type(&link.relation_type).with_context(|| {
            format!(
                "Invalid link relation type '{}' in {}",
                link.relation_type,
                display_state_path(relative)
            )
        })?;
    }
    Ok(())
}

fn validate_issue_ids(values: &[String], key: &str, relative: &Path) -> Result<()> {
    for value in values {
        record_id::validate_record_id(value).with_context(|| {
            format!(
                "Invalid issue id '{}' in key '{}' of {}",
                value,
                key,
                display_state_path(relative)
            )
        })?;
    }
    Ok(())
}

fn collect_issue_record_paths(root: &Path, dir: &Path, records: &mut Vec<PathBuf>) -> Result<()> {
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
                "Invalid datetime '{}' in key '{}' of {}",
                value,
                key,
                display_state_path(relative)
            )
        })
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
            "Front matter key '{}' in {} must be string or null",
            key,
            display_state_path(relative)
        ),
    }
}

fn string_array(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Vec<String>> {
    values
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| {
            anyhow!(
                "Missing array front matter key '{}' in {}",
                key,
                display_state_path(relative)
            )
        })?
        .iter()
        .map(|value| {
            value.as_str().map(str::to_string).ok_or_else(|| {
                anyhow!(
                    "Array '{}' in {} must contain strings",
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
    relative: &Path,
) -> Result<Vec<IssueLink>> {
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
        let target_id = object
            .get("target_id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                anyhow!(
                    "Link in {} is missing target_id",
                    display_state_path(relative)
                )
            })?;
        record_id::validate_record_id(target_id).with_context(|| {
            format!(
                "Invalid link target_id '{}' in {}",
                target_id,
                display_state_path(relative)
            )
        })?;
        let relation_type = object
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Link in {} is missing type", display_state_path(relative)))?;
        crate::db::validate_relation_type(relation_type).with_context(|| {
            format!(
                "Invalid link relation type '{}' in {}",
                relation_type,
                display_state_path(relative)
            )
        })?;
        let link = IssueLink {
            relation_type: relation_type.to_string(),
            target_kind: target_kind.to_string(),
            target_id: target_id.to_string(),
        };
        if !seen.insert(link.clone()) {
            bail!(
                "Duplicate link to {} ({}) in {}",
                target_id,
                relation_type,
                display_state_path(relative)
            );
        }
        links.push(link);
    }
    Ok(links)
}

fn write_yaml_scalar(output: &mut String, key: &str, value: Option<&str>) -> Result<()> {
    match value {
        Some(value) => {
            output.push_str(key);
            output.push_str(": ");
            output.push_str(&serde_json::to_string(value)?);
            output.push('\n');
        }
        None => {
            output.push_str(key);
            output.push_str(": null\n");
        }
    }
    Ok(())
}

fn write_yaml_array(output: &mut String, key: &str, values: &[String]) -> Result<()> {
    output.push_str(key);
    if values.is_empty() {
        output.push_str(": []\n");
        return Ok(());
    }
    output.push_str(":\n");
    for value in values {
        output.push_str("- ");
        output.push_str(&serde_json::to_string(value)?);
        output.push('\n');
    }
    Ok(())
}

fn write_yaml_links(output: &mut String, key: &str, links: &[IssueLink]) -> Result<()> {
    output.push_str(key);
    if links.is_empty() {
        output.push_str(": []\n");
        return Ok(());
    }
    output.push_str(":\n");
    for link in links {
        output.push_str("- target_id: ");
        output.push_str(&serde_json::to_string(&link.target_id)?);
        output.push('\n');
        output.push_str("  target_kind: ");
        output.push_str(&serde_json::to_string(&link.target_kind)?);
        output.push('\n');
        output.push_str("  type: ");
        output.push_str(&serde_json::to_string(&link.relation_type)?);
        output.push('\n');
    }
    Ok(())
}

fn canonical_priority(priority: &str) -> String {
    match priority {
        "critical" => "P0".to_string(),
        "high" => "P1".to_string(),
        "medium" => "P2".to_string(),
        "low" => "P3".to_string(),
        other => other.to_string(),
    }
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

fn normalize_body(body: &str) -> String {
    body.replace("\r\n", "\n").replace('\r', "\n")
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
    use chrono::TimeZone;
    use tempfile::tempdir;

    fn issue_record(id: &str) -> CanonicalIssueRecord {
        CanonicalIssueRecord {
            issue: Issue {
                id: id.to_string(),
                title: "Write RecordStore".to_string(),
                description: Some("Canonical body".to_string()),
                status: "open".to_string(),
                issue_type: "task".to_string(),
                priority: "high".to_string(),
                parent_id: None,
                created_at: Utc.with_ymd_and_hms(2026, 6, 10, 12, 0, 0).unwrap(),
                updated_at: Utc.with_ymd_and_hms(2026, 6, 10, 13, 0, 0).unwrap(),
                closed_at: None,
            },
            labels: vec!["record-store".to_string(), "storage".to_string()],
            blocks: vec!["atelier-bbbb".to_string()],
            depends_on: vec!["atelier-aaaa".to_string()],
            acceptance: vec![
                "Issue Markdown round-trips without losing fields".to_string(),
                "Atomic writes do not expose partial state".to_string(),
            ],
            evidence_required: vec!["cargo test record_store".to_string()],
            links: vec![IssueLink {
                relation_type: "related".to_string(),
                target_kind: "issue".to_string(),
                target_id: "atelier-cccc".to_string(),
            }],
        }
    }

    #[test]
    fn registered_first_class_record_kinds_have_canonical_contracts() {
        let contracts = FIRST_CLASS_RECORD_KINDS
            .iter()
            .map(|spec| {
                (
                    spec.kind,
                    spec.schema,
                    spec.schema_version,
                    spec.canonical_dir,
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            contracts,
            vec![
                ("mission", "atelier.mission", 1, Some("missions")),
                ("milestone", "atelier.milestone", 1, Some("milestones")),
                ("plan", "atelier.plan", 1, Some("plans")),
                ("evidence", "atelier.evidence", 1, Some("evidence")),
            ]
        );
    }

    #[test]
    fn workflow_validator_kind_is_registered_but_not_canonical_yet() {
        validate_record_kind("workflow_validator").unwrap();
        assert!(validate_canonical_record_kind("workflow_validator").is_err());
        assert!(canonical_record_kind("workflow_validator").is_err());
    }

    #[test]
    fn issue_record_renders_and_parses_deterministically() {
        let record = issue_record("atelier-abcd");
        let text = render_issue_record(&record).unwrap();
        let parsed = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap();

        assert_eq!(parsed, record);
        assert_eq!(render_issue_record(&parsed).unwrap(), text);
        assert!(text.contains("schema: \"atelier.issue\""));
        assert!(text.contains("schema_version: 1"));
        assert!(text.contains("- \"Issue Markdown round-trips without losing fields\""));
        assert!(text.contains("- \"cargo test record_store\""));
    }

    #[test]
    fn issue_parser_reports_malformed_front_matter() {
        let error = parse_issue_record("not front matter\n", &issue_record_path("atelier-abcd"))
            .unwrap_err();
        assert!(error.to_string().contains("Missing YAML front matter"));
    }

    #[test]
    fn issue_parser_reports_schema_and_path_mismatch() {
        let text = render_issue_record(&issue_record("atelier-abcd")).unwrap();
        let schema_error = parse_issue_record(
            &text.replace("schema: \"atelier.issue\"", "schema: \"atelier.graph\""),
            &issue_record_path("atelier-abcd"),
        )
        .unwrap_err();
        assert!(schema_error
            .to_string()
            .contains("Unsupported schema 'atelier.graph'"));

        let path_error = parse_issue_record(&text, &issue_record_path("atelier-wxyz")).unwrap_err();
        assert!(path_error
            .to_string()
            .contains("does not match canonical path .atelier-state/issues/atelier-abcd.md"));
    }

    #[test]
    fn record_store_discovers_and_rejects_noncanonical_issue_paths() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier-state"));
        let record = issue_record("atelier-abcd");
        store.write_issue_atomic(&record).unwrap();
        let duplicate_path = store
            .state_dir
            .join("issues")
            .join("nested")
            .join("atelier-abcd.md");
        fs::create_dir_all(duplicate_path.parent().unwrap()).unwrap();
        fs::write(duplicate_path, render_issue_record(&record).unwrap()).unwrap();

        let error = store.load_issues().unwrap_err();
        assert!(error
            .to_string()
            .contains("does not match canonical path .atelier-state/issues/atelier-abcd.md"));
    }

    #[test]
    fn record_store_allocates_ids_across_canonical_dirs() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier-state"));
        let candidate = "atelier-0001";
        fs::create_dir_all(store.state_dir.join("missions")).unwrap();
        fs::write(
            store
                .state_dir
                .join("missions")
                .join(format!("{candidate}.md")),
            "",
        )
        .unwrap();

        assert!(store.canonical_id_exists(candidate).unwrap());
        let allocated = store.allocate_issue_id().unwrap();
        assert_ne!(allocated, candidate);
    }

    #[test]
    fn write_issue_atomic_preserves_existing_file_when_temp_write_fails() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier-state"));
        let record = issue_record("atelier-abcd");
        store.write_issue_atomic(&record).unwrap();
        let path = store.state_dir.join(issue_record_path("atelier-abcd"));
        let original = fs::read_to_string(&path).unwrap();
        let tmp_path = path.with_extension("md.tmp");
        fs::create_dir_all(&tmp_path).unwrap();

        let error = store.write_issue_atomic(&record).unwrap_err();
        assert!(error.to_string().contains("Failed to write"));
        assert_eq!(fs::read_to_string(&path).unwrap(), original);
    }

    #[test]
    fn write_issue_atomic_rejects_path_traversal_ids_before_writing() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier-state"));
        let record = issue_record("../escaped");

        let error = store.write_issue_atomic(&record).unwrap_err();

        assert!(error.to_string().contains("Invalid issue id"));
        assert!(!dir
            .path()
            .join(".atelier-state")
            .join("escaped.md")
            .exists());
        assert!(!dir.path().join("escaped.md").exists());
    }
}
