use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::{DomainRecord, Issue};
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
pub struct RelationshipTarget {
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AttachmentRelationship {
    pub kind: String,
    pub id: String,
    pub role: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RelatesRelationship {
    pub kind: String,
    pub id: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Relationships {
    pub blocks: Vec<RelationshipTarget>,
    pub children: Vec<RelationshipTarget>,
    pub attachments: Vec<AttachmentRelationship>,
    pub relates: Vec<RelatesRelationship>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalIssueRecord {
    pub issue: Issue,
    pub labels: Vec<String>,
    pub acceptance: Vec<String>,
    pub evidence_required: Vec<String>,
    pub relationships: Relationships,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalDomainRecord {
    pub record: DomainRecord,
    pub relationships: Relationships,
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

    pub fn load_issue_by_id(&self, id: &str) -> Result<CanonicalIssueRecord> {
        record_id::validate_record_id(id)?;
        self.load_issue(&issue_record_path(id))
    }

    pub fn load_domain_record_by_id(&self, kind: &str, id: &str) -> Result<CanonicalDomainRecord> {
        record_id::validate_record_id(id)?;
        let spec = canonical_record_kind(kind)?;
        self.load_domain_record(&canonical_record_path(spec, id)?, spec)
    }

    pub fn load_domain_record(
        &self,
        relative: &Path,
        spec: &RecordKindSpec,
    ) -> Result<CanonicalDomainRecord> {
        let bytes = fs::read(self.state_dir.join(relative))
            .with_context(|| format!("Missing projection file {}", display_state_path(relative)))?;
        let text = String::from_utf8(bytes).with_context(|| {
            format!(
                "Projection file {} is not UTF-8",
                display_state_path(relative)
            )
        })?;
        parse_domain_record(&text, relative, spec)
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

    pub fn allocate_domain_record_id(&self) -> Result<String> {
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
        self.write_atomic(&relative, render_issue_record(record)?)
    }

    pub fn create_domain_record(
        &self,
        kind: &str,
        title: &str,
        status: &str,
        body: Option<&str>,
        data_json: &str,
    ) -> Result<CanonicalDomainRecord> {
        let _: Value = serde_json::from_str(data_json)?;
        let now = Utc::now();
        let record = CanonicalDomainRecord {
            record: DomainRecord {
                id: self.allocate_domain_record_id()?,
                kind: kind.to_string(),
                title: title.to_string(),
                status: status.to_string(),
                body: body.map(str::to_string),
                data_json: data_json.to_string(),
                created_at: now,
                updated_at: now,
            },
            relationships: Relationships::default(),
        };
        self.write_domain_record_atomic(&record)?;
        Ok(record)
    }

    pub fn write_domain_record_atomic(&self, record: &CanonicalDomainRecord) -> Result<()> {
        let spec = canonical_record_kind(&record.record.kind)?;
        validate_domain_record(record, Path::new("<record>"), spec)?;
        let relative = canonical_record_path(spec, &record.record.id)?;
        self.write_atomic(&relative, render_domain_record(record)?)
    }

    pub fn add_attachment_relationship(
        &self,
        source_kind: &str,
        source_id: &str,
        target_kind: &str,
        target_id: &str,
        role: &str,
    ) -> Result<bool> {
        crate::db::validate_link_type(role)?;
        let mut record = self.load_domain_record_by_id(source_kind, source_id)?;
        let attachment = AttachmentRelationship {
            kind: target_kind.to_string(),
            id: target_id.to_string(),
            role: role.to_string(),
        };
        if record.relationships.attachments.contains(&attachment) {
            return Ok(false);
        }
        record.relationships.attachments.push(attachment);
        record.record.updated_at = Utc::now();
        self.write_domain_record_atomic(&record)?;
        Ok(true)
    }

    pub fn add_relates_relationship(
        &self,
        source_kind: &str,
        source_id: &str,
        target_kind: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<bool> {
        crate::db::validate_relationship_type(relation_type)?;
        let mut record = self.load_domain_record_by_id(source_kind, source_id)?;
        let relation = RelatesRelationship {
            kind: target_kind.to_string(),
            id: target_id.to_string(),
            relation_type: relation_type.to_string(),
        };
        if record.relationships.relates.contains(&relation) {
            return Ok(false);
        }
        record.relationships.relates.push(relation);
        record.record.updated_at = Utc::now();
        self.write_domain_record_atomic(&record)?;
        Ok(true)
    }

    pub fn add_issue_label(&self, issue_id: &str, label: &str) -> Result<bool> {
        if label.len() > crate::db::MAX_LABEL_LEN {
            bail!(
                "Label exceeds maximum length of {} characters",
                crate::db::MAX_LABEL_LEN
            );
        }
        let mut record = self.load_issue_by_id(issue_id)?;
        if record.labels.iter().any(|existing| existing == label) {
            return Ok(false);
        }
        record.labels.push(label.to_string());
        self.write_issue_atomic(&record)?;
        Ok(true)
    }

    pub fn remove_issue_label(&self, issue_id: &str, label: &str) -> Result<bool> {
        let mut record = self.load_issue_by_id(issue_id)?;
        let original_len = record.labels.len();
        record.labels.retain(|existing| existing != label);
        if record.labels.len() == original_len {
            return Ok(false);
        }
        self.write_issue_atomic(&record)?;
        Ok(true)
    }

    pub fn add_issue_block(&self, blocked_id: &str, blocker_id: &str) -> Result<bool> {
        if blocked_id == blocker_id {
            bail!("An issue cannot block itself");
        }
        self.load_issue_by_id(blocked_id)?;
        if self.would_create_block_cycle(blocked_id, blocker_id)? {
            bail!("Adding this dependency would create a circular dependency chain");
        }

        let target = issue_relationship_target(blocked_id);
        let mut blocker = self.load_issue_by_id(blocker_id)?;
        if blocker.relationships.blocks.contains(&target) {
            return Ok(false);
        }
        blocker.relationships.blocks.push(target);
        self.write_issue_atomic(&blocker)?;
        Ok(true)
    }

    pub fn remove_issue_block(&self, blocked_id: &str, blocker_id: &str) -> Result<bool> {
        self.load_issue_by_id(blocked_id)?;
        let target = issue_relationship_target(blocked_id);
        let mut blocker = self.load_issue_by_id(blocker_id)?;
        let original_len = blocker.relationships.blocks.len();
        blocker
            .relationships
            .blocks
            .retain(|existing| existing != &target);
        if blocker.relationships.blocks.len() == original_len {
            return Ok(false);
        }
        self.write_issue_atomic(&blocker)?;
        Ok(true)
    }

    pub fn add_issue_relation(
        &self,
        issue_id: &str,
        related_id: &str,
        relation_type: &str,
    ) -> Result<bool> {
        crate::db::validate_relation_type(relation_type)?;
        if issue_id == related_id {
            bail!("Cannot relate an issue to itself");
        }
        let mut issue = self.load_issue_by_id(issue_id)?;
        let mut related = self.load_issue_by_id(related_id)?;
        let issue_target = issue_relates_relationship(issue_id, relation_type);
        let related_target = issue_relates_relationship(related_id, relation_type);
        let issue_changed = !issue.relationships.relates.contains(&related_target);
        let related_changed = !related.relationships.relates.contains(&issue_target);
        if !issue_changed && !related_changed {
            return Ok(false);
        }
        if issue_changed {
            issue.relationships.relates.push(related_target);
            self.write_issue_atomic(&issue)?;
        }
        if related_changed {
            related.relationships.relates.push(issue_target);
            self.write_issue_atomic(&related)?;
        }
        Ok(true)
    }

    pub fn remove_issue_relation(
        &self,
        issue_id: &str,
        related_id: &str,
        relation_type: &str,
    ) -> Result<bool> {
        crate::db::validate_relation_type(relation_type)?;
        let mut issue = self.load_issue_by_id(issue_id)?;
        let mut related = self.load_issue_by_id(related_id)?;
        let issue_target = issue_relates_relationship(issue_id, relation_type);
        let related_target = issue_relates_relationship(related_id, relation_type);
        let issue_original_len = issue.relationships.relates.len();
        let related_original_len = related.relationships.relates.len();
        issue
            .relationships
            .relates
            .retain(|existing| existing != &related_target);
        related
            .relationships
            .relates
            .retain(|existing| existing != &issue_target);
        let issue_changed = issue.relationships.relates.len() != issue_original_len;
        let related_changed = related.relationships.relates.len() != related_original_len;
        if !issue_changed && !related_changed {
            return Ok(false);
        }
        if issue_changed {
            self.write_issue_atomic(&issue)?;
        }
        if related_changed {
            self.write_issue_atomic(&related)?;
        }
        Ok(true)
    }

    pub fn delete_issue_atomic(&self, id: &str) -> Result<()> {
        record_id::validate_record_id(id)?;
        self.delete_atomic(&issue_record_path(id))
    }

    pub fn delete_domain_record_atomic(&self, kind: &str, id: &str) -> Result<()> {
        record_id::validate_record_id(id)?;
        let spec = canonical_record_kind(kind)?;
        self.delete_atomic(&canonical_record_path(spec, id)?)
    }

    fn write_atomic(&self, relative: &Path, contents: String) -> Result<()> {
        let path = self.state_dir.join(relative);
        let parent = path
            .parent()
            .ok_or_else(|| anyhow!("Record path has no parent: {}", path.display()))?;
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
        let tmp_path = unique_temp_path(&path, "tmp")?;
        fs::write(&tmp_path, contents)
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

    fn delete_atomic(&self, relative: &Path) -> Result<()> {
        let path = self.state_dir.join(relative);
        if path.exists() {
            fs::remove_file(&path)
                .with_context(|| format!("Failed to remove {}", path.display()))?;
        }
        Ok(())
    }

    fn would_create_block_cycle(&self, blocked_id: &str, blocker_id: &str) -> Result<bool> {
        let issues = self.load_issues()?;
        let mut blocking_by_blocker: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for issue in issues {
            for blocked in issue.relationships.blocks {
                if blocked.kind == ISSUE_KIND.kind {
                    blocking_by_blocker
                        .entry(issue.issue.id.clone())
                        .or_default()
                        .push(blocked.id);
                }
            }
        }

        let mut visited = BTreeSet::new();
        let mut stack = vec![blocked_id.to_string()];
        while let Some(current) = stack.pop() {
            if current == blocker_id {
                return Ok(true);
            }
            if visited.insert(current.clone()) {
                if let Some(blocking) = blocking_by_blocker.get(&current) {
                    stack.extend(blocking.iter().cloned());
                }
            }
        }
        Ok(false)
    }
}

fn unique_temp_path(path: &Path, suffix: &str) -> Result<PathBuf> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("Record path has no file name: {}", path.display()))?;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("System clock is before UNIX epoch")?
        .as_nanos();
    Ok(path.with_file_name(format!(
        ".{file_name}.{}.{}.{}",
        process::id(),
        nanos,
        suffix
    )))
}

fn issue_relationship_target(id: &str) -> RelationshipTarget {
    RelationshipTarget {
        kind: ISSUE_KIND.kind.to_string(),
        id: id.to_string(),
    }
}

fn issue_relates_relationship(id: &str, relation_type: &str) -> RelatesRelationship {
    RelatesRelationship {
        kind: ISSUE_KIND.kind.to_string(),
        id: id.to_string(),
        relation_type: relation_type.to_string(),
    }
}

pub fn render_issue_record(record: &CanonicalIssueRecord) -> Result<String> {
    validate_issue_record(record, Path::new("<record>"))?;
    let mut labels = record.labels.clone();
    let mut relationships = record.relationships.clone();
    labels.sort();
    sort_relationships(&mut relationships);

    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_array(&mut output, "acceptance", &record.acceptance)?;
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.issue.created_at.to_rfc3339()),
    )?;
    write_yaml_array(&mut output, "evidence_required", &record.evidence_required)?;
    write_yaml_scalar(&mut output, "id", Some(&record.issue.id))?;
    write_yaml_scalar(&mut output, "issue_type", Some(&record.issue.issue_type))?;
    write_yaml_array(&mut output, "labels", &labels)?;
    write_yaml_scalar(
        &mut output,
        "priority",
        Some(&canonical_priority(&record.issue.priority)),
    )?;
    write_yaml_relationships(&mut output, &relationships)?;
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

pub fn render_domain_record(record: &CanonicalDomainRecord) -> Result<String> {
    let spec = canonical_record_kind(&record.record.kind)?;
    validate_domain_record(record, Path::new("<record>"), spec)?;
    let mut relationships = record.relationships.clone();
    sort_relationships(&mut relationships);

    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.record.created_at.to_rfc3339()),
    )?;
    write_yaml_scalar(&mut output, "id", Some(&record.record.id))?;
    write_json_scalar(&mut output, "data", &record.record.data_json)?;
    write_yaml_relationships(&mut output, &relationships)?;
    write_yaml_scalar(&mut output, "schema", Some(spec.schema))?;
    output.push_str(&format!("schema_version: {}\n", spec.schema_version));
    write_yaml_scalar(&mut output, "status", Some(&record.record.status))?;
    write_yaml_scalar(&mut output, "title", Some(&record.record.title))?;
    write_yaml_scalar(
        &mut output,
        "updated_at",
        Some(&record.record.updated_at.to_rfc3339()),
    )?;
    output.push_str("---\n\n");
    output.push_str(&normalize_body(record.record.body.as_deref().unwrap_or("")));
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

    reject_legacy_relationship_keys(&front_matter, relative)?;
    let relationships = parse_relationships(&front_matter, relative)?;
    let acceptance = string_array(&front_matter, "acceptance", relative)?;
    let evidence_required = string_array(&front_matter, "evidence_required", relative)?;
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
            parent_id: None,
            created_at: require_datetime(&front_matter, "created_at", relative)?,
            updated_at,
            closed_at: (status == "closed").then_some(updated_at),
        },
        labels: string_array(&front_matter, "labels", relative)?,
        acceptance,
        evidence_required,
        relationships,
    })
}

pub fn parse_domain_record(
    text: &str,
    relative: &Path,
    spec: &RecordKindSpec,
) -> Result<CanonicalDomainRecord> {
    let (front_matter, body) = split_front_matter(text, relative)?;

    let schema = require_scalar(&front_matter, "schema", relative)?;
    if schema != spec.schema {
        bail!(
            "Unsupported schema '{}' in {}; expected {}",
            schema,
            display_state_path(relative),
            spec.schema
        );
    }
    let schema_version = require_i64(&front_matter, "schema_version", relative)?;
    if schema_version != spec.schema_version {
        bail!(
            "Unsupported schema_version {} in {}; expected {}",
            schema_version,
            display_state_path(relative),
            spec.schema_version
        );
    }

    let id = require_scalar(&front_matter, "id", relative)?;
    record_id::validate_record_id(&id).with_context(|| {
        format!(
            "Invalid {} id {} in {}",
            spec.kind,
            id,
            display_state_path(relative)
        )
    })?;
    let expected = canonical_record_path(spec, &id)?;
    if relative != expected {
        bail!(
            "{} id {} in {} does not match canonical path {}",
            spec.kind,
            id,
            display_state_path(relative),
            display_state_path(&expected)
        );
    }
    let data_json = require_scalar(&front_matter, "data", relative)?;
    let _: Value = serde_json::from_str(&data_json)
        .with_context(|| format!("Invalid data JSON in {}", display_state_path(relative)))?;
    let relationships = parse_relationships(&front_matter, relative)?;
    let body = if body.is_empty() {
        None
    } else {
        Some(body.to_string())
    };
    Ok(CanonicalDomainRecord {
        record: DomainRecord {
            id,
            kind: spec.kind.to_string(),
            title: require_scalar(&front_matter, "title", relative)?,
            status: require_scalar(&front_matter, "status", relative)?,
            body,
            data_json,
            created_at: require_datetime(&front_matter, "created_at", relative)?,
            updated_at: require_datetime(&front_matter, "updated_at", relative)?,
        },
        relationships,
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
    validate_relationships(&record.relationships, relative)?;
    Ok(())
}

fn validate_domain_record(
    record: &CanonicalDomainRecord,
    relative: &Path,
    spec: &RecordKindSpec,
) -> Result<()> {
    if record.record.kind != spec.kind {
        bail!(
            "Record kind '{}' does not match expected kind '{}' in {}",
            record.record.kind,
            spec.kind,
            display_state_path(relative)
        );
    }
    record_id::validate_record_id(&record.record.id).with_context(|| {
        format!(
            "Invalid {} id '{}' in {}",
            spec.kind,
            record.record.id,
            display_state_path(relative)
        )
    })?;
    let _: Value = serde_json::from_str(&record.record.data_json)
        .with_context(|| format!("Invalid data JSON in {}", display_state_path(relative)))?;
    validate_relationships(&record.relationships, relative)?;
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
            if is_transient_record_path(&relative) {
                continue;
            }
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

fn is_transient_record_path(relative: &Path) -> bool {
    relative
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.') && name.ends_with(".tmp"))
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
    serde_yaml::from_str(front).with_context(|| {
        format!(
            "Invalid YAML front matter in {}",
            display_state_path(relative)
        )
    })
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

fn reject_legacy_relationship_keys(
    values: &BTreeMap<String, Value>,
    relative: &Path,
) -> Result<()> {
    for key in ["blocks", "depends_on", "parent", "links"] {
        if values.contains_key(key) {
            bail!(
                "Legacy relationship front matter key '{}' in {}; use relationships",
                key,
                display_state_path(relative)
            );
        }
    }
    Ok(())
}

pub fn parse_relationships(
    values: &BTreeMap<String, Value>,
    relative: &Path,
) -> Result<Relationships> {
    let object = values
        .get("relationships")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            anyhow!(
                "Missing object front matter key 'relationships' in {}",
                display_state_path(relative)
            )
        })?;
    let mut relationships = Relationships {
        blocks: relationship_targets(object, "blocks", relative)?,
        children: relationship_targets(object, "children", relative)?,
        attachments: attachment_relationships(object, "attachments", relative)?,
        relates: relates_relationships(object, "relates", relative)?,
    };
    sort_relationships(&mut relationships);
    validate_relationships(&relationships, relative)?;
    Ok(relationships)
}

fn relationship_targets(
    values: &serde_json::Map<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Vec<RelationshipTarget>> {
    let values = values.get(key).and_then(Value::as_array).ok_or_else(|| {
        anyhow!(
            "Missing relationships.{} array in {}",
            key,
            display_state_path(relative)
        )
    })?;

    let mut targets = Vec::new();
    let mut seen = BTreeSet::new();
    for value in values {
        let object = value.as_object().ok_or_else(|| {
            anyhow!(
                "relationships.{} in {} must contain objects",
                key,
                display_state_path(relative)
            )
        })?;
        let kind = object.get("kind").and_then(Value::as_str).ok_or_else(|| {
            anyhow!(
                "relationships.{} entry in {} is missing kind",
                key,
                display_state_path(relative)
            )
        })?;
        crate::db::validate_record_kind(kind)?;
        let id = object.get("id").and_then(Value::as_str).ok_or_else(|| {
            anyhow!(
                "relationships.{} entry in {} is missing id",
                key,
                display_state_path(relative)
            )
        })?;
        record_id::validate_record_id(id).with_context(|| {
            format!(
                "Invalid relationships.{} id '{}' in {}",
                key,
                id,
                display_state_path(relative)
            )
        })?;
        let target = RelationshipTarget {
            kind: kind.to_string(),
            id: id.to_string(),
        };
        if !seen.insert(target.clone()) {
            bail!(
                "Duplicate relationships.{} target {} {} in {}",
                key,
                kind,
                id,
                display_state_path(relative)
            );
        }
        targets.push(target);
    }
    Ok(targets)
}

fn attachment_relationships(
    values: &serde_json::Map<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Vec<AttachmentRelationship>> {
    let values = values.get(key).and_then(Value::as_array).ok_or_else(|| {
        anyhow!(
            "Missing relationships.{} array in {}",
            key,
            display_state_path(relative)
        )
    })?;
    let mut attachments = Vec::new();
    let mut seen = BTreeSet::new();
    for value in values {
        let object = value.as_object().ok_or_else(|| {
            anyhow!(
                "relationships.{} in {} must contain objects",
                key,
                display_state_path(relative)
            )
        })?;
        let attachment = AttachmentRelationship {
            kind: required_relationship_string(object, key, "kind", relative)?,
            id: required_relationship_string(object, key, "id", relative)?,
            role: required_relationship_string(object, key, "role", relative)?,
        };
        crate::db::validate_record_kind(&attachment.kind)?;
        crate::db::validate_link_type(&attachment.role)?;
        record_id::validate_record_id(&attachment.id)?;
        if !seen.insert(attachment.clone()) {
            bail!(
                "Duplicate relationships.{} target {} {} ({}) in {}",
                key,
                attachment.kind,
                attachment.id,
                attachment.role,
                display_state_path(relative)
            );
        }
        attachments.push(attachment);
    }
    Ok(attachments)
}

fn relates_relationships(
    values: &serde_json::Map<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Vec<RelatesRelationship>> {
    let values = values.get(key).and_then(Value::as_array).ok_or_else(|| {
        anyhow!(
            "Missing relationships.{} array in {}",
            key,
            display_state_path(relative)
        )
    })?;
    let mut relates = Vec::new();
    let mut seen = BTreeSet::new();
    for value in values {
        let object = value.as_object().ok_or_else(|| {
            anyhow!(
                "relationships.{} in {} must contain objects",
                key,
                display_state_path(relative)
            )
        })?;
        let relation = RelatesRelationship {
            kind: required_relationship_string(object, key, "kind", relative)?,
            id: required_relationship_string(object, key, "id", relative)?,
            relation_type: required_relationship_string(object, key, "type", relative)?,
        };
        crate::db::validate_record_kind(&relation.kind)?;
        crate::db::validate_relationship_type(&relation.relation_type)?;
        record_id::validate_record_id(&relation.id)?;
        if !seen.insert(relation.clone()) {
            bail!(
                "Duplicate relationships.{} target {} {} ({}) in {}",
                key,
                relation.kind,
                relation.id,
                relation.relation_type,
                display_state_path(relative)
            );
        }
        relates.push(relation);
    }
    Ok(relates)
}

fn required_relationship_string(
    object: &serde_json::Map<String, Value>,
    bucket: &str,
    key: &str,
    relative: &Path,
) -> Result<String> {
    object
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| {
            anyhow!(
                "relationships.{} entry in {} is missing {}",
                bucket,
                display_state_path(relative),
                key
            )
        })
}

fn validate_relationships(relationships: &Relationships, relative: &Path) -> Result<()> {
    for target in relationships
        .blocks
        .iter()
        .chain(relationships.children.iter())
    {
        crate::db::validate_record_kind(&target.kind)?;
        record_id::validate_record_id(&target.id)?;
    }
    for attachment in &relationships.attachments {
        crate::db::validate_record_kind(&attachment.kind)?;
        crate::db::validate_link_type(&attachment.role)?;
        record_id::validate_record_id(&attachment.id)?;
    }
    for relation in &relationships.relates {
        crate::db::validate_record_kind(&relation.kind)?;
        crate::db::validate_relationship_type(&relation.relation_type)?;
        record_id::validate_record_id(&relation.id)?;
    }
    let _ = relative;
    Ok(())
}

pub fn sort_relationships(relationships: &mut Relationships) {
    relationships.blocks.sort();
    relationships.blocks.dedup();
    relationships.children.sort();
    relationships.children.dedup();
    relationships.attachments.sort();
    relationships.attachments.dedup();
    relationships.relates.sort();
    relationships.relates.dedup();
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

fn write_json_scalar(output: &mut String, key: &str, value: &str) -> Result<()> {
    let _: Value = serde_json::from_str(value)?;
    output.push_str(key);
    output.push_str(": ");
    output.push_str(&serde_json::to_string(value)?);
    output.push('\n');
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

pub fn write_yaml_relationships(output: &mut String, relationships: &Relationships) -> Result<()> {
    output.push_str("relationships:\n");
    write_relationship_targets(output, "blocks", &relationships.blocks)?;
    write_relationship_targets(output, "children", &relationships.children)?;
    write_attachment_relationships(output, &relationships.attachments)?;
    write_relates_relationships(output, &relationships.relates)?;
    Ok(())
}

fn write_relationship_targets(
    output: &mut String,
    key: &str,
    values: &[RelationshipTarget],
) -> Result<()> {
    output.push_str("  ");
    output.push_str(key);
    if values.is_empty() {
        output.push_str(": []\n");
        return Ok(());
    }
    output.push_str(":\n");
    for value in values {
        output.push_str("  - kind: ");
        output.push_str(&serde_json::to_string(&value.kind)?);
        output.push('\n');
        output.push_str("    id: ");
        output.push_str(&serde_json::to_string(&value.id)?);
        output.push('\n');
    }
    Ok(())
}

fn write_attachment_relationships(
    output: &mut String,
    values: &[AttachmentRelationship],
) -> Result<()> {
    output.push_str("  attachments");
    if values.is_empty() {
        output.push_str(": []\n");
        return Ok(());
    }
    output.push_str(":\n");
    for value in values {
        output.push_str("  - kind: ");
        output.push_str(&serde_json::to_string(&value.kind)?);
        output.push('\n');
        output.push_str("    id: ");
        output.push_str(&serde_json::to_string(&value.id)?);
        output.push('\n');
        output.push_str("    role: ");
        output.push_str(&serde_json::to_string(&value.role)?);
        output.push('\n');
    }
    Ok(())
}

fn write_relates_relationships(output: &mut String, values: &[RelatesRelationship]) -> Result<()> {
    output.push_str("  relates");
    if values.is_empty() {
        output.push_str(": []\n");
        return Ok(());
    }
    output.push_str(":\n");
    for value in values {
        output.push_str("  - kind: ");
        output.push_str(&serde_json::to_string(&value.kind)?);
        output.push('\n');
        output.push_str("    id: ");
        output.push_str(&serde_json::to_string(&value.id)?);
        output.push('\n');
        output.push_str("    type: ");
        output.push_str(&serde_json::to_string(&value.relation_type)?);
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
        ".atelier/{}",
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
            acceptance: vec![
                "Issue Markdown round-trips without losing fields".to_string(),
                "Atomic writes do not expose partial state".to_string(),
            ],
            evidence_required: vec!["cargo test record_store".to_string()],
            relationships: Relationships {
                blocks: vec![RelationshipTarget {
                    kind: "issue".to_string(),
                    id: "atelier-bbbb".to_string(),
                }],
                children: vec![RelationshipTarget {
                    kind: "issue".to_string(),
                    id: "atelier-aaaa".to_string(),
                }],
                attachments: Vec::new(),
                relates: vec![RelatesRelationship {
                    kind: "issue".to_string(),
                    id: "atelier-cccc".to_string(),
                    relation_type: "related".to_string(),
                }],
            },
        }
    }

    fn sectioned_issue_text(id: &str, body: &str) -> String {
        format!(
            r#"---
created_at: "2026-06-10T12:00:00+00:00"
id: "{id}"
issue_type: "task"
labels:
  - "record-store"
priority: "P1"
relationships:
  attachments: []
  blocks: []
  children: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Write RecordStore"
updated_at: "2026-06-10T13:00:00+00:00"
---

{body}
"#
        )
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
        assert!(text.contains("relationships:\n"));
        assert!(text.contains("  blocks:\n  - kind: \"issue\"\n    id: \"atelier-bbbb\"\n"));
        assert!(text.contains("  children:\n  - kind: \"issue\"\n    id: \"atelier-aaaa\"\n"));
        assert!(text.contains(
            "  relates:\n  - kind: \"issue\"\n    id: \"atelier-cccc\"\n    type: \"related\"\n"
        ));
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
            .contains("does not match canonical path .atelier/issues/atelier-abcd.md"));
    }

    #[test]
    #[ignore = "contract test for the upcoming issue body section parser"]
    fn issue_parser_contract_accepts_sectioned_body_without_legacy_arrays() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.\n\n## Notes\n\nSequencing context.";
        let text = sectioned_issue_text("atelier-abcd", body);
        let parsed = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap();

        assert_eq!(parsed.acceptance, Vec::<String>::new());
        assert_eq!(parsed.evidence_required, Vec::<String>::new());
        assert_eq!(parsed.issue.description.as_deref(), Some(body));
    }

    #[test]
    #[ignore = "contract test for the upcoming issue body section parser"]
    fn issue_parser_contract_rejects_legacy_acceptance_and_evidence_front_matter() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body).replace(
            "created_at:",
            "acceptance:\n  - \"legacy acceptance\"\nevidence_required:\n  - \"legacy proof\"\ncreated_at:",
        );

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        let message = error.to_string();
        assert!(message.contains("acceptance"));
        assert!(message.contains("evidence_required"));
    }

    #[test]
    #[ignore = "contract test for the upcoming issue body section parser"]
    fn issue_parser_contract_rejects_missing_required_sections() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Missing required issue body section 'Outcome'"));
    }

    #[test]
    #[ignore = "contract test for the upcoming issue body section parser"]
    fn issue_parser_contract_rejects_duplicate_recognized_headings() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.\n\n## Outcome\n\nA second outcome.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Duplicate issue body section 'Outcome'"));
    }

    #[test]
    #[ignore = "contract test for the upcoming issue body section parser"]
    fn issue_parser_contract_rejects_content_before_first_recognized_heading() {
        let body = "Preamble is not part of the issue contract.\n\n## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Content before first recognized issue body section"));
    }

    #[test]
    #[ignore = "contract test for the upcoming issue body section parser"]
    fn issue_parser_contract_rejects_unknown_top_level_sections() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.\n\n## Acceptance\n\nLegacy section name.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Unknown issue body section 'Acceptance'"));
    }

    #[test]
    fn record_store_discovers_and_rejects_noncanonical_issue_paths() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier"));
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
            .contains("does not match canonical path .atelier/issues/atelier-abcd.md"));
    }

    #[test]
    fn record_store_allocates_ids_across_canonical_dirs() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier"));
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
    fn record_store_label_unlabel_mutates_issue_front_matter() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier"));
        let mut record = issue_record("atelier-abcd");
        record.labels.clear();
        store.write_issue_atomic(&record).unwrap();

        assert!(store.add_issue_label("atelier-abcd", "graph").unwrap());
        assert!(!store.add_issue_label("atelier-abcd", "graph").unwrap());
        assert_eq!(
            store.load_issue_by_id("atelier-abcd").unwrap().labels,
            vec!["graph".to_string()]
        );
        assert!(store.remove_issue_label("atelier-abcd", "graph").unwrap());
        assert!(!store.remove_issue_label("atelier-abcd", "graph").unwrap());
        assert!(store
            .load_issue_by_id("atelier-abcd")
            .unwrap()
            .labels
            .is_empty());
    }

    #[test]
    fn record_store_block_unblock_mutates_blocker_relationships() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier"));
        let mut blocked = issue_record("atelier-abcd");
        blocked.relationships = Relationships::default();
        let mut blocker = issue_record("atelier-efgh");
        blocker.relationships = Relationships::default();
        store.write_issue_atomic(&blocked).unwrap();
        store.write_issue_atomic(&blocker).unwrap();

        assert!(store
            .add_issue_block("atelier-abcd", "atelier-efgh")
            .unwrap());
        assert!(!store
            .add_issue_block("atelier-abcd", "atelier-efgh")
            .unwrap());
        assert_eq!(
            store
                .load_issue_by_id("atelier-efgh")
                .unwrap()
                .relationships
                .blocks,
            vec![RelationshipTarget {
                kind: "issue".to_string(),
                id: "atelier-abcd".to_string(),
            }]
        );
        assert!(store
            .remove_issue_block("atelier-abcd", "atelier-efgh")
            .unwrap());
        assert!(!store
            .remove_issue_block("atelier-abcd", "atelier-efgh")
            .unwrap());
        assert!(store
            .load_issue_by_id("atelier-efgh")
            .unwrap()
            .relationships
            .blocks
            .is_empty());
    }

    #[test]
    fn record_store_block_rejects_cycles_and_self_blocks() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier"));
        for id in ["atelier-abcd", "atelier-efgh", "atelier-ijkl"] {
            let mut record = issue_record(id);
            record.relationships = Relationships::default();
            store.write_issue_atomic(&record).unwrap();
        }

        store
            .add_issue_block("atelier-efgh", "atelier-abcd")
            .unwrap();
        store
            .add_issue_block("atelier-ijkl", "atelier-efgh")
            .unwrap();

        let cycle = store
            .add_issue_block("atelier-abcd", "atelier-ijkl")
            .unwrap_err();
        assert!(cycle.to_string().contains("circular dependency"));
        let self_block = store
            .add_issue_block("atelier-abcd", "atelier-abcd")
            .unwrap_err();
        assert!(self_block.to_string().contains("cannot block itself"));
    }

    #[test]
    fn record_store_relate_unrelate_mutates_both_issue_records() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier"));
        let mut first = issue_record("atelier-abcd");
        first.relationships = Relationships::default();
        let mut second = issue_record("atelier-efgh");
        second.relationships = Relationships::default();
        store.write_issue_atomic(&first).unwrap();
        store.write_issue_atomic(&second).unwrap();

        assert!(store
            .add_issue_relation("atelier-abcd", "atelier-efgh", "derived")
            .unwrap());
        assert!(!store
            .add_issue_relation("atelier-abcd", "atelier-efgh", "derived")
            .unwrap());
        assert_eq!(
            store
                .load_issue_by_id("atelier-abcd")
                .unwrap()
                .relationships
                .relates,
            vec![RelatesRelationship {
                kind: "issue".to_string(),
                id: "atelier-efgh".to_string(),
                relation_type: "derived".to_string(),
            }]
        );
        assert_eq!(
            store
                .load_issue_by_id("atelier-efgh")
                .unwrap()
                .relationships
                .relates,
            vec![RelatesRelationship {
                kind: "issue".to_string(),
                id: "atelier-abcd".to_string(),
                relation_type: "derived".to_string(),
            }]
        );

        assert!(store
            .remove_issue_relation("atelier-abcd", "atelier-efgh", "derived")
            .unwrap());
        assert!(!store
            .remove_issue_relation("atelier-abcd", "atelier-efgh", "derived")
            .unwrap());
        assert!(store
            .load_issue_by_id("atelier-abcd")
            .unwrap()
            .relationships
            .relates
            .is_empty());
        assert!(store
            .load_issue_by_id("atelier-efgh")
            .unwrap()
            .relationships
            .relates
            .is_empty());
    }

    #[test]
    fn write_issue_atomic_ignores_stale_fixed_temp_artifact() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier"));
        let record = issue_record("atelier-abcd");
        store.write_issue_atomic(&record).unwrap();
        let path = store.state_dir.join(issue_record_path("atelier-abcd"));
        let tmp_path = path.with_extension("md.tmp");
        fs::create_dir_all(&tmp_path).unwrap();

        store.write_issue_atomic(&record).unwrap();
        assert!(path.exists());
        assert!(tmp_path.is_dir());
        assert_eq!(
            store.discover_issue_paths().unwrap(),
            vec![issue_record_path("atelier-abcd")]
        );
    }

    #[test]
    fn write_issue_atomic_rejects_path_traversal_ids_before_writing() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier"));
        let record = issue_record("../escaped");

        let error = store.write_issue_atomic(&record).unwrap_err();

        assert!(error.to_string().contains("Invalid issue id"));
        assert!(!dir.path().join(".atelier").join("escaped.md").exists());
        assert!(!dir.path().join("escaped.md").exists());
    }
}
