use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use atelier_core::{
    EvidenceOutputSummary, EvidenceRecordData, EvidenceTarget, Issue, IssuePriority,
    ISSUE_PRIORITY_LABELS,
};
pub use atelier_core::{
    EvidenceRecord, IssueRecord, IssueSectionName, IssueSectionState, IssueSections, Record,
    RecordHeader, ReviewRecord,
};

pub mod activity;
pub mod document;
pub mod evidence;
pub mod issue;
pub mod store;
pub mod validation;

mod record_id;
mod record_kinds;
mod relationships;

pub use record_kinds::{
    canonical_record_dirs, canonical_record_kind, canonical_record_path, issue_record_path,
    validate_canonical_record_kind, validate_record_kind, RecordKindSpec, CANONICAL_RECORD_KINDS,
    FIRST_CLASS_RECORD_KINDS, ISSUE_KIND,
};
pub use relationships::{
    attachment_relationship, issue_relates_relationship, issue_relationship_target,
    relates_relationship, relationship_target, sort_relationships, AttachmentRelationship,
    RelatesRelationship, RelationshipTarget, Relationships,
};

/// Canonical record kind vocabulary shared by parser and application code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordKind {
    Evidence,
    Issue,
    Review,
}

impl RecordKind {
    pub fn directory(self) -> &'static str {
        match self {
            Self::Evidence => "evidence",
            Self::Issue => "issues",
            Self::Review => "reviews",
        }
    }
}

/// Deterministic front matter/body rendering used by record-store extraction.
pub fn render_document(front_matter: &serde_yaml::Value, body: &str) -> Result<String> {
    let mut rendered = String::from("---\n");
    rendered.push_str(&serde_yaml::to_string(front_matter)?);
    rendered.push_str("---\n\n");
    rendered.push_str(body.trim_end());
    rendered.push('\n');
    Ok(rendered)
}

/// Split a canonical Markdown document into YAML front matter and body text.
pub fn split_document(input: &str) -> Option<(&str, &str)> {
    let rest = input.strip_prefix("---\n")?;
    let (front_matter, body) = rest.split_once("\n---\n")?;
    Some((front_matter, body.trim_start_matches('\n')))
}

pub const WELL_KNOWN_RELATION_TYPES: &[&str] = &["related", "assumption", "falsifies", "derived"];

pub const WELL_KNOWN_LINK_TYPES: &[&str] = &[
    "advances",
    "blocked_by",
    "contributes_to",
    "validates",
    "evidenced_by",
    "implements",
    "part_of",
    "supersedes",
    "derived_from",
    "duplicates",
    "related",
];

pub fn validate_issue_relation_type(relation_type: &str) -> Result<()> {
    validate_relation_type(relation_type)?;
    if matches!(relation_type, "validates" | "evidenced_by") {
        bail!(
            "Issue-to-issue relationship '{}' is not supported; use 'advances' for mission scope or attach evidence with role 'validates'",
            relation_type
        );
    }
    Ok(())
}

pub const VALID_PRIORITIES: &[&str] = ISSUE_PRIORITY_LABELS;
pub const VALID_ISSUE_TYPES: &[&str] = &[
    "bug",
    "epic",
    "feature",
    "mission",
    "spike",
    "task",
    "validation",
];
pub const MAX_LABEL_LEN: usize = 128;
pub fn validate_status(status: &str) -> Result<()> {
    let mut chars = status.chars();
    if matches!(chars.next(), Some(first) if first.is_ascii_lowercase())
        && chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        Ok(())
    } else {
        bail!(
            "Invalid status '{}'. Status values must match ^[a-z][a-z0-9_]*$",
            status,
        )
    }
}

pub fn validate_priority(priority: &str) -> Result<()> {
    IssuePriority::from_cli_input(priority)
        .map(|_| ())
        .map_err(Into::into)
}

pub fn validate_issue_type(issue_type: &str) -> Result<()> {
    let mut chars = issue_type.chars();
    if matches!(chars.next(), Some(first) if first.is_ascii_lowercase())
        && chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        Ok(())
    } else {
        bail!(
            "Invalid issue_type '{}'. Issue type values must match ^[a-z][a-z0-9_]*$",
            issue_type,
        )
    }
}

pub fn validate_relation_type(relation_type: &str) -> Result<()> {
    if relation_type.is_empty() {
        bail!("Relation type cannot be empty");
    }
    validate_relation_type_syntax(relation_type)
}

fn validate_relation_type_syntax(relation_type: &str) -> Result<()> {
    let mut chars = relation_type.chars();
    let Some(first) = chars.next() else {
        bail!("Relation type cannot be empty");
    };
    if !first.is_ascii_lowercase() {
        bail!(
            "Invalid relation type '{}'. Values must start with a lowercase ASCII letter",
            relation_type
        );
    }
    if !chars
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '_' | '-' | '.'))
    {
        bail!(
            "Invalid relation type '{}'. Values may contain only lowercase ASCII letters, digits, '_', '-', or '.'",
            relation_type
        );
    }
    Ok(())
}

pub fn validate_link_type(relation_type: &str) -> Result<()> {
    if relation_type.is_empty() {
        bail!("Link type cannot be empty");
    }
    if WELL_KNOWN_LINK_TYPES.contains(&relation_type) {
        Ok(())
    } else {
        validate_relation_type_syntax(relation_type)
    }
}

pub fn validate_relationship_type(relation_type: &str) -> Result<()> {
    if WELL_KNOWN_LINK_TYPES.contains(&relation_type) {
        Ok(())
    } else {
        validate_relation_type(relation_type)
    }
}

pub fn is_attachment_role(relation_type: &str) -> bool {
    matches!(relation_type, "validates" | "evidenced_by")
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalIssueRecord {
    pub issue: Issue,
    pub labels: Vec<String>,
    pub sections: IssueSections,
    pub relationships: Relationships,
}

pub fn issue_section_diagnostic(
    issue_id: Option<&str>,
    section: &str,
    relative: &Path,
    detail: &str,
) -> String {
    let issue_id = issue_id
        .map(str::to_string)
        .or_else(|| issue_id_from_record_path(relative))
        .unwrap_or_else(|| "(unknown)".to_string());
    format!(
        "{detail} for issue {issue_id}, section {section}, path {}",
        display_state_path(relative)
    )
}

impl CanonicalIssueRecord {
    pub fn into_record(self) -> Record {
        Record::Issue(IssueRecord {
            header: RecordHeader {
                kind: ISSUE_KIND.kind.to_string(),
                id: self.issue.id,
                title: self.issue.title,
                status: self.issue.status,
                labels: self.labels,
                relationships: self.relationships,
                created_at: self.issue.created_at,
                updated_at: self.issue.updated_at,
            },
            issue_type: self.issue.issue_type,
            priority: self.issue.priority,
            fields: self.issue.fields,
            closed_at: self.issue.closed_at,
            sections: self.sections,
        })
    }
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
        let spec = canonical_record_kind(ISSUE_KIND.kind)?;
        self.load_issue(&canonical_record_path(spec, id)?)
    }

    pub fn load_record_by_id(&self, kind: &str, id: &str) -> Result<Record> {
        record_id::validate_record_id(id)?;
        let spec = canonical_record_kind(kind)?;
        self.load_record_at(&canonical_record_path(spec, id)?, spec)
    }

    pub fn load_record_at(&self, relative: &Path, spec: &RecordKindSpec) -> Result<Record> {
        let bytes = fs::read(self.state_dir.join(relative))
            .with_context(|| format!("Missing projection file {}", display_state_path(relative)))?;
        let text = String::from_utf8(bytes).with_context(|| {
            format!(
                "Projection file {} is not UTF-8",
                display_state_path(relative)
            )
        })?;
        parse_record(&text, relative, spec)
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

    pub fn allocate_record_id(&self) -> Result<String> {
        record_id::allocate_issue_id(|candidate| self.canonical_id_exists(candidate))
    }

    pub fn canonical_id_exists(&self, id: &str) -> Result<bool> {
        for spec in CANONICAL_RECORD_KINDS {
            let relative = canonical_record_path(spec, id)?;
            if self.state_dir.join(relative).exists() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn write_issue_atomic(&self, record: &CanonicalIssueRecord) -> Result<()> {
        validate_issue_record(record, Path::new("<record>"))?;
        let spec = canonical_record_kind(ISSUE_KIND.kind)?;
        let relative = canonical_record_path(spec, &record.issue.id)?;
        self.write_atomic(&relative, render_issue_record(record)?)
    }

    pub fn create_evidence(
        &self,
        title: &str,
        status: &str,
        summary: &str,
        data: EvidenceRecordData,
    ) -> Result<EvidenceRecord> {
        let now = Utc::now();
        let record = EvidenceRecord {
            header: RecordHeader {
                kind: "evidence".to_string(),
                id: self.allocate_record_id()?,
                title: title.to_string(),
                status: status.to_string(),
                labels: default_record_labels("evidence"),
                relationships: Relationships::default(),
                created_at: now,
                updated_at: now,
            },
            data,
            summary: summary.to_string(),
        };
        self.write_record_atomic(&Record::Evidence(record.clone()))?;
        Ok(record)
    }

    pub fn write_record_atomic(&self, record: &Record) -> Result<()> {
        let header = record.header();
        let spec = canonical_record_kind(&header.kind)?;
        let relative = canonical_record_path(spec, &header.id)?;
        self.write_atomic(&relative, render_record(record)?)
    }

    pub fn add_attachment_relationship(
        &self,
        source_kind: &str,
        source_id: &str,
        target_kind: &str,
        target_id: &str,
        role: &str,
    ) -> Result<bool> {
        validate_link_type(role)?;
        let mut record = self.load_record_by_id(source_kind, source_id)?;
        let attachment = attachment_relationship(target_kind, target_id, role);
        if record
            .header()
            .relationships
            .attachments
            .contains(&attachment)
        {
            return Ok(false);
        }
        record
            .header_mut()
            .relationships
            .attachments
            .push(attachment);
        record.header_mut().updated_at = Utc::now();
        self.write_record_atomic(&record)?;
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
        validate_relationship_type(relation_type)?;
        let mut record = self.load_record_by_id(source_kind, source_id)?;
        let relation = relates_relationship(target_kind, target_id, relation_type);
        if record.header().relationships.relates.contains(&relation) {
            return Ok(false);
        }
        record.header_mut().relationships.relates.push(relation);
        record.header_mut().updated_at = Utc::now();
        self.write_record_atomic(&record)?;
        Ok(true)
    }

    pub fn remove_relates_relationship(
        &self,
        source_kind: &str,
        source_id: &str,
        target_kind: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<bool> {
        validate_relationship_type(relation_type)?;
        let mut record = self.load_record_by_id(source_kind, source_id)?;
        let original_len = record.header().relationships.relates.len();
        record
            .header_mut()
            .relationships
            .relates
            .retain(|existing| {
                existing.kind != target_kind
                    || existing.id != target_id
                    || existing.relation_type != relation_type
            });
        if record.header().relationships.relates.len() == original_len {
            return Ok(false);
        }
        record.header_mut().updated_at = Utc::now();
        self.write_record_atomic(&record)?;
        Ok(true)
    }

    pub fn add_issue_label(&self, issue_id: &str, label: &str) -> Result<bool> {
        if label.len() > MAX_LABEL_LEN {
            bail!(
                "Label exceeds maximum length of {} characters",
                MAX_LABEL_LEN
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

    pub fn add_issue_child(&self, parent_id: &str, child_id: &str) -> Result<bool> {
        if parent_id == child_id {
            bail!("An issue cannot be its own parent");
        }
        self.load_issue_by_id(child_id)?;
        let target = issue_relationship_target(child_id);
        let mut parent = self.load_issue_by_id(parent_id)?;
        if parent.relationships.children.contains(&target) {
            return Ok(false);
        }
        parent.relationships.children.push(target);
        parent.issue.updated_at = Utc::now();
        self.write_issue_atomic(&parent)?;
        Ok(true)
    }

    pub fn remove_issue_child(&self, parent_id: &str, child_id: &str) -> Result<bool> {
        let target = issue_relationship_target(child_id);
        let mut parent = self.load_issue_by_id(parent_id)?;
        let original_len = parent.relationships.children.len();
        parent
            .relationships
            .children
            .retain(|existing| existing != &target);
        if parent.relationships.children.len() == original_len {
            return Ok(false);
        }
        parent.issue.updated_at = Utc::now();
        self.write_issue_atomic(&parent)?;
        Ok(true)
    }

    pub fn find_issue_parent(&self, child_id: &str) -> Result<Option<String>> {
        Ok(self
            .load_issues()?
            .into_iter()
            .find(|record| {
                record
                    .relationships
                    .children
                    .iter()
                    .any(|child| child.kind == ISSUE_KIND.kind && child.id == child_id)
            })
            .map(|record| record.issue.id))
    }

    pub fn remove_issue_children_from_external_parents(
        &self,
        child_ids: &[String],
    ) -> Result<Vec<String>> {
        let child_ids = child_ids.iter().collect::<BTreeSet<_>>();
        let mut changed_parents = Vec::new();
        for mut parent in self.load_issues()? {
            if child_ids.contains(&parent.issue.id) {
                continue;
            }
            let original_len = parent.relationships.children.len();
            parent
                .relationships
                .children
                .retain(|child| !(child.kind == ISSUE_KIND.kind && child_ids.contains(&child.id)));
            if parent.relationships.children.len() != original_len {
                parent.issue.updated_at = Utc::now();
                changed_parents.push(parent.issue.id.clone());
                self.write_issue_atomic(&parent)?;
            }
        }
        Ok(changed_parents)
    }

    pub fn add_record_relationship(
        &self,
        source_kind: &str,
        source_id: &str,
        target_kind: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<bool> {
        validate_record_kind(target_kind)?;
        record_id::validate_record_id(target_id)?;
        if is_attachment_role(relation_type) {
            validate_link_type(relation_type)?;
        } else {
            validate_relationship_type(relation_type)?;
        }

        if source_kind == ISSUE_KIND.kind {
            let mut record = self.load_issue_by_id(source_id)?;
            let changed = add_relationship_to_bucket(
                &mut record.relationships,
                target_kind,
                target_id,
                relation_type,
            );
            if changed {
                record.issue.updated_at = Utc::now();
                self.write_issue_atomic(&record)?;
            }
            Ok(changed)
        } else {
            let mut record = self.load_record_by_id(source_kind, source_id)?;
            let changed = add_relationship_to_bucket(
                &mut record.header_mut().relationships,
                target_kind,
                target_id,
                relation_type,
            );
            if changed {
                record.header_mut().updated_at = Utc::now();
                self.write_record_atomic(&record)?;
            }
            Ok(changed)
        }
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
        validate_issue_relation_type(relation_type)?;
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
        validate_relation_type(relation_type)?;
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

pub fn render_issue_record(record: &CanonicalIssueRecord) -> Result<String> {
    validate_issue_record(record, Path::new("<record>"))?;
    let mut labels = record.labels.clone();
    let mut relationships = record.relationships.clone();
    labels.sort();
    sort_relationships(&mut relationships);

    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.issue.created_at.to_rfc3339()),
    )?;
    write_yaml_scalar(&mut output, "id", Some(&record.issue.id))?;
    write_yaml_scalar(&mut output, "issue_type", Some(&record.issue.issue_type))?;
    write_yaml_array(&mut output, "labels", &labels)?;
    let mut fields = record.issue.fields.clone();
    if let Some(review) = fields.remove("review") {
        write_yaml_value(&mut output, "review", &review)?;
    }
    write_yaml_map_if_not_empty(&mut output, "fields", &fields)?;
    write_yaml_scalar(
        &mut output,
        "priority",
        Some(&canonical_priority(&record.issue.priority)),
    )?;
    write_yaml_relationships(&mut output, &relationships)?;
    write_yaml_scalar(&mut output, "schema", Some(ISSUE_KIND.schema))?;
    output.push_str(&format!("schema_version: {}\n", ISSUE_KIND.schema_version));
    if let Some(closed_at) = record.issue.closed_at.as_ref() {
        write_yaml_scalar(&mut output, "closed_at", Some(&closed_at.to_rfc3339()))?;
    }
    write_yaml_scalar(&mut output, "status", Some(&record.issue.status))?;
    write_yaml_scalar(&mut output, "title", Some(&record.issue.title))?;
    write_yaml_scalar(
        &mut output,
        "updated_at",
        Some(&record.issue.updated_at.to_rfc3339()),
    )?;
    output.push_str("---\n\n");
    output.push_str(&render_issue_sections(&record.sections));
    output.push('\n');
    Ok(output)
}

fn canonical_issue_record_from_record(record: &Record) -> Result<CanonicalIssueRecord> {
    let Record::Issue(issue) = record else {
        bail!("Issue record kind must use Record::Issue");
    };
    Ok(CanonicalIssueRecord {
        issue: Issue {
            id: issue.header.id.clone(),
            title: issue.header.title.clone(),
            description: None,
            status: issue.header.status.clone(),
            issue_type: issue.issue_type.clone(),
            priority: issue.priority.clone(),
            fields: issue.fields.clone(),
            parent_id: None,
            created_at: issue.header.created_at,
            updated_at: issue.header.updated_at,
            closed_at: issue.closed_at,
        },
        labels: issue.header.labels.clone(),
        sections: issue.sections.clone(),
        relationships: issue.header.relationships.clone(),
    })
}

fn render_issue_sections(sections: &IssueSections) -> String {
    let mut body = format!(
        "## Description\n\n{}\n\n## Outcome\n\n{}",
        sections.description.trim(),
        sections.outcome.trim()
    );
    if let Some(evidence) = sections.section(IssueSectionName::Evidence) {
        body.push_str("\n\n## Evidence\n\n");
        body.push_str(evidence.trim());
    }
    if let Some(notes) = sections
        .notes
        .as_deref()
        .map(str::trim)
        .filter(|notes| !notes.is_empty())
    {
        body.push_str("\n\n## Notes\n\n");
        body.push_str(notes);
    }
    normalize_body(&body)
}

pub fn render_record(record: &Record) -> Result<String> {
    let spec = canonical_record_kind(&record.header().kind)?;
    if spec.kind == ISSUE_KIND.kind {
        return render_issue_record(&canonical_issue_record_from_record(record)?);
    }
    let mut record = record.clone();
    if spec.kind == "evidence" {
        record.header_mut().relationships =
            normalize_legacy_evidence_relationships(record.header().relationships.clone());
    }
    validate_record(&record, Path::new("<record>"), spec)?;
    let mut relationships = record.header().relationships.clone();
    sort_relationships(&mut relationships);

    match spec.kind {
        "evidence" => return render_evidence_record(&record, &relationships, spec),
        "review" => return render_review_record(&record, &relationships, spec),
        _ => {}
    }
    bail!(
        "Unsupported canonical record kind '{}' for typed rendering",
        spec.kind
    )
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
    reject_legacy_issue_keys(&front_matter, relative)?;
    let relationships = parse_relationships(&front_matter, relative)?;
    let status = require_scalar(&front_matter, "status", relative)?;
    validate_status(&status)
        .with_context(|| format!("Invalid status in {}", display_state_path(relative)))?;
    let issue_type = require_scalar(&front_matter, "issue_type", relative)?;
    validate_issue_type(&issue_type)
        .with_context(|| format!("Invalid issue_type in {}", display_state_path(relative)))?;
    let updated_at = require_datetime(&front_matter, "updated_at", relative)?;
    let closed_at = optional_datetime(&front_matter, "closed_at", relative)?;
    let mut fields = optional_object(&front_matter, "fields", relative)?;
    if front_matter.contains_key("pull_request") {
        bail!(
            "Legacy pull_request field in {}; use structured review field",
            display_state_path(relative)
        );
    }
    if let Some(review) = front_matter.get("review") {
        fields.insert("review".to_string(), review.clone());
    }
    let sections = parse_issue_sections(body, relative)?;

    Ok(CanonicalIssueRecord {
        issue: Issue {
            id,
            title: require_scalar(&front_matter, "title", relative)?,
            description: None,
            status: status.clone(),
            issue_type,
            priority: db_priority(&require_scalar(&front_matter, "priority", relative)?)
                .with_context(|| format!("Invalid priority in {}", display_state_path(relative)))?,
            fields,
            parent_id: None,
            created_at: require_datetime(&front_matter, "created_at", relative)?,
            updated_at,
            closed_at: closed_at.or((status == "closed").then_some(updated_at)),
        },
        labels: string_array(&front_matter, "labels", relative)?,
        sections,
        relationships,
    })
}

pub fn parse_record(text: &str, relative: &Path, spec: &RecordKindSpec) -> Result<Record> {
    if spec.kind == ISSUE_KIND.kind {
        return Ok(parse_issue_record(text, relative)?.into_record());
    }
    if spec.kind == "review" {
        return parse_review_record(text, relative, spec);
    }

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
    reject_forbidden_data_front_matter(&front_matter, relative)?;
    match spec.kind {
        "evidence" => return parse_evidence_record(front_matter, body, relative, spec, id),
        _ => {}
    }
    bail!(
        "Unsupported canonical record kind '{}' in {}",
        spec.kind,
        display_state_path(relative)
    )
}

fn reject_forbidden_data_front_matter(
    front_matter: &BTreeMap<String, Value>,
    relative: &Path,
) -> Result<()> {
    if front_matter.contains_key("data") {
        bail!(
            "Forbidden data front matter in {}; use typed front matter and body sections instead",
            display_state_path(relative)
        );
    }
    Ok(())
}

fn validate_issue_record(record: &CanonicalIssueRecord, relative: &Path) -> Result<()> {
    record_id::validate_record_id(&record.issue.id).with_context(|| {
        format!(
            "Invalid issue id '{}' in {}",
            record.issue.id,
            display_state_path(relative)
        )
    })?;
    validate_status(&record.issue.status)
        .with_context(|| format!("Invalid status in {}", display_state_path(relative)))?;
    validate_issue_type(&record.issue.issue_type)
        .with_context(|| format!("Invalid issue_type in {}", display_state_path(relative)))?;
    validate_priority(&record.issue.priority)
        .with_context(|| format!("Invalid priority in {}", display_state_path(relative)))?;
    validate_relationships(&record.relationships, relative)?;
    Ok(())
}

fn validate_record(record: &Record, relative: &Path, spec: &RecordKindSpec) -> Result<()> {
    if record.header().kind != spec.kind {
        bail!(
            "Record kind '{}' does not match expected kind '{}' in {}",
            record.header().kind,
            spec.kind,
            display_state_path(relative)
        );
    }
    record_id::validate_record_id(&record.header().id).with_context(|| {
        format!(
            "Invalid {} id '{}' in {}",
            spec.kind,
            record.header().id,
            display_state_path(relative)
        )
    })?;
    match spec.kind {
        "evidence" => {
            validate_evidence_record(record, relative)?;
            return Ok(());
        }
        "review" => {
            validate_review_record(record, relative)?;
            return Ok(());
        }
        _ => {}
    }
    validate_relationships(&record.header().relationships, relative)?;
    Ok(())
}

fn render_review_record(
    record: &Record,
    relationships: &Relationships,
    spec: &RecordKindSpec,
) -> Result<String> {
    let Record::Review(record) = record else {
        bail!("Expected review record");
    };
    let mut labels = record.header.labels.clone();
    labels.sort();
    labels.dedup();
    let mut output = String::new();
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.header.created_at.to_rfc3339()),
    )?;
    write_yaml_value(&mut output, "events", &Value::Array(record.events.clone()))?;
    write_yaml_scalar(&mut output, "id", Some(&record.header.id))?;
    write_yaml_scalar(&mut output, "issue", Some(&record.issue_id))?;
    write_yaml_array(&mut output, "labels", &labels)?;
    write_yaml_scalar(&mut output, "mode", Some(&record.mode))?;
    write_yaml_relationships(&mut output, relationships)?;
    write_yaml_scalar(&mut output, "schema", Some(spec.schema))?;
    output.push_str(&format!("schema_version: {}\n", spec.schema_version));
    write_yaml_scalar(&mut output, "source_branch", Some(&record.source_branch))?;
    write_yaml_scalar(&mut output, "status", Some(&record.header.status))?;
    write_yaml_scalar(&mut output, "target_branch", Some(&record.target_branch))?;
    write_yaml_scalar(&mut output, "title", Some(&record.header.title))?;
    write_yaml_scalar(
        &mut output,
        "updated_at",
        Some(&record.header.updated_at.to_rfc3339()),
    )?;
    Ok(output)
}

fn parse_review_record(text: &str, relative: &Path, spec: &RecordKindSpec) -> Result<Record> {
    let front_matter = parse_yaml_document(text, relative)?;
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
            "Invalid review id {} in {}",
            id,
            display_state_path(relative)
        )
    })?;
    let expected = canonical_record_path(spec, &id)?;
    if relative != expected {
        bail!(
            "review id {} in {} does not match canonical path {}",
            id,
            display_state_path(relative),
            display_state_path(&expected)
        );
    }
    reject_forbidden_data_front_matter(&front_matter, relative)?;
    let relationships = parse_relationships(&front_matter, relative)?;
    let mode = require_scalar(&front_matter, "mode", relative)?;
    if mode != "room" {
        bail!(
            "Review record {} mode must be 'room', got '{}'",
            display_state_path(relative),
            mode
        );
    }
    let events = front_matter
        .get("events")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let labels = string_array(&front_matter, "labels", relative)?;
    validate_sorted_unique("labels", &labels, relative)?;
    let status = require_scalar(&front_matter, "status", relative)?;
    validate_status(&status)
        .with_context(|| format!("Invalid review status in {}", display_state_path(relative)))?;
    let issue_id = require_scalar(&front_matter, "issue", relative)?;
    record_id::validate_record_id(&issue_id).with_context(|| {
        format!(
            "Invalid review issue id {} in {}",
            issue_id,
            display_state_path(relative)
        )
    })?;
    Ok(Record::Review(ReviewRecord {
        header: RecordHeader {
            id,
            kind: spec.kind.to_string(),
            title: require_scalar(&front_matter, "title", relative)?,
            status,
            labels,
            relationships,
            created_at: require_datetime(&front_matter, "created_at", relative)?,
            updated_at: require_datetime(&front_matter, "updated_at", relative)?,
        },
        mode,
        issue_id,
        source_branch: require_scalar(&front_matter, "source_branch", relative)?,
        target_branch: require_scalar(&front_matter, "target_branch", relative)?,
        events,
    }))
}

fn validate_review_record(record: &Record, relative: &Path) -> Result<()> {
    let Record::Review(record) = record else {
        bail!("Expected review record in {}", display_state_path(relative));
    };
    if record.mode != "room" {
        bail!(
            "Review record mode must be 'room' in {}",
            display_state_path(relative)
        );
    }
    record_id::validate_record_id(&record.issue_id).with_context(|| {
        format!(
            "Invalid review issue id '{}' in {}",
            record.issue_id,
            display_state_path(relative)
        )
    })?;
    validate_status(&record.header.status)
        .with_context(|| format!("Invalid review status in {}", display_state_path(relative)))?;
    validate_relationships(&record.header.relationships, relative)?;
    Ok(())
}

fn render_evidence_record(
    record: &Record,
    relationships: &Relationships,
    spec: &RecordKindSpec,
) -> Result<String> {
    let Record::Evidence(record) = record else {
        bail!("Expected evidence record");
    };
    let data = normalized_evidence_data(record.data.clone());
    let body = render_evidence_body(Some(&record.summary), &data);

    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.header.created_at.to_rfc3339()),
    )?;
    write_yaml_scalar(&mut output, "id", Some(&record.header.id))?;
    write_yaml_scalar(&mut output, "evidence_type", Some(&data.evidence_type))?;
    write_yaml_scalar(
        &mut output,
        "captured_at",
        Some(&data.captured_at.to_rfc3339()),
    )?;
    write_yaml_scalar_if_some(&mut output, "command", data.command.as_deref())?;
    write_yaml_scalar_if_some(&mut output, "exit_status", data.exit_status.as_deref())?;
    write_yaml_scalar_if_some(&mut output, "path", data.path.as_deref())?;
    write_yaml_scalar_if_some(&mut output, "uri", data.uri.as_deref())?;
    write_yaml_scalar_if_some(&mut output, "proof_scope", data.proof_scope.as_deref())?;
    write_yaml_scalar_if_some(
        &mut output,
        "agent_identity",
        data.agent_identity.as_deref().or(data.producer.as_deref()),
    )?;
    write_yaml_scalar_if_some(
        &mut output,
        "independence_level",
        data.independence_level.as_deref(),
    )?;
    write_evidence_target_if_some(&mut output, data.target.as_ref())?;
    write_yaml_array_if_not_empty(&mut output, "follow_up_ids", &data.follow_up_ids)?;
    write_yaml_array_if_not_empty(&mut output, "residual_risks", &data.residual_risks)?;
    write_yaml_relationships(&mut output, relationships)?;
    write_yaml_scalar(&mut output, "schema", Some(spec.schema))?;
    output.push_str(&format!("schema_version: {}\n", spec.schema_version));
    write_yaml_scalar(
        &mut output,
        "status",
        Some(normalized_evidence_status(&record.header.status)),
    )?;
    write_yaml_scalar(&mut output, "title", Some(&record.header.title))?;
    write_yaml_scalar(
        &mut output,
        "updated_at",
        Some(&record.header.updated_at.to_rfc3339()),
    )?;
    output.push_str("---\n\n");
    output.push_str(&body);
    output.push('\n');
    Ok(output)
}

fn parse_evidence_record(
    front_matter: BTreeMap<String, Value>,
    body: &str,
    relative: &Path,
    spec: &RecordKindSpec,
    id: String,
) -> Result<Record> {
    let relationships =
        normalize_legacy_evidence_relationships(parse_relationships(&front_matter, relative)?);
    let title = require_scalar(&front_matter, "title", relative)?;
    let status =
        normalized_evidence_status(&require_scalar(&front_matter, "status", relative)?).to_string();
    let created_at = require_datetime(&front_matter, "created_at", relative)?;
    let updated_at = require_datetime(&front_matter, "updated_at", relative)?;

    let mut data = EvidenceRecordData {
        evidence_type: require_scalar(&front_matter, "evidence_type", relative)?,
        captured_at: require_datetime(&front_matter, "captured_at", relative)?,
        command: optional_scalar(&front_matter, "command")?,
        path: optional_scalar(&front_matter, "path")?,
        uri: optional_scalar(&front_matter, "uri")?,
        producer: None,
        proof_scope: optional_scalar(&front_matter, "proof_scope")?,
        agent_identity: optional_scalar(&front_matter, "agent_identity")?,
        independence_level: optional_scalar(&front_matter, "independence_level")?,
        target: optional_yaml_value::<EvidenceTarget>(&front_matter, "target", relative)?,
        residual_risks: optional_string_array(&front_matter, "residual_risks", relative)?
            .unwrap_or_default(),
        follow_up_ids: optional_string_array(&front_matter, "follow_up_ids", relative)?
            .unwrap_or_default(),
        exit_code: optional_i32(&front_matter, "exit_code", relative)?,
        exit_status: optional_scalar(&front_matter, "exit_status")?,
        success: optional_bool(&front_matter, "success", relative)?,
        spawn_error: optional_scalar(&front_matter, "spawn_error")?,
        output: optional_yaml_value::<EvidenceOutputSummary>(&front_matter, "output", relative)?,
    };
    apply_evidence_body_sections(&mut data, body);

    Ok(Record::Evidence(EvidenceRecord {
        header: RecordHeader {
            id,
            kind: spec.kind.to_string(),
            title,
            status,
            labels: optional_string_array(&front_matter, "labels", relative)?.unwrap_or_default(),
            relationships,
            created_at,
            updated_at,
        },
        data,
        summary: body.to_string(),
    }))
}

fn validate_evidence_record(record: &Record, relative: &Path) -> Result<()> {
    let Record::Evidence(record) = record else {
        bail!(
            "Expected evidence record in {}",
            display_state_path(relative)
        );
    };
    let data = normalized_evidence_data(record.data.clone());
    if data.evidence_type.trim().is_empty() {
        bail!(
            "Evidence record {} must include evidence_type",
            display_state_path(relative)
        );
    }
    validate_relationships(&record.header.relationships, relative)?;
    Ok(())
}

fn validate_sorted_unique(key: &str, values: &[String], relative: &Path) -> Result<()> {
    let mut sorted = values.to_vec();
    sorted.sort();
    sorted.dedup();
    if sorted != values {
        bail!(
            "{} in {} must be sorted and unique",
            key,
            display_state_path(relative)
        );
    }
    Ok(())
}

fn normalize_legacy_evidence_relationships(mut relationships: Relationships) -> Relationships {
    let mut normalized_attachments = relationships.attachments.clone();
    for relation in &relationships.relates {
        if relation.relation_type == "validates" {
            normalized_attachments.push(AttachmentRelationship {
                kind: relation.kind.clone(),
                id: relation.id.clone(),
                role: "validates".to_string(),
            });
        }
    }
    relationships.attachments = normalized_attachments;
    relationships
        .relates
        .retain(|relation| relation.relation_type != "validates");
    sort_relationships(&mut relationships);
    relationships
}

fn default_record_labels(kind: &str) -> Vec<String> {
    let _ = kind;
    Vec::new()
}

fn normalized_evidence_data(mut data: EvidenceRecordData) -> EvidenceRecordData {
    if data.agent_identity.is_none() {
        data.agent_identity = data.producer.clone();
    }
    if matches!(
        data.proof_scope.as_deref(),
        Some("scoped to the attached target or summary")
    ) {
        data.proof_scope = None;
    }
    if matches!(data.independence_level.as_deref(), Some("unspecified")) {
        data.independence_level = None;
    }
    data.follow_up_ids.sort();
    data
}

fn normalized_evidence_status(status: &str) -> &str {
    match status {
        "pass" | "fail" | "blocked" | "deferred" => "recorded",
        other => other,
    }
}

fn add_relationship_to_bucket(
    relationships: &mut Relationships,
    target_kind: &str,
    target_id: &str,
    relation_type: &str,
) -> bool {
    if is_attachment_role(relation_type) {
        let attachment = attachment_relationship(target_kind, target_id, relation_type);
        if relationships.attachments.contains(&attachment) {
            return false;
        }
        relationships.attachments.push(attachment);
    } else {
        let relation = relates_relationship(target_kind, target_id, relation_type);
        if relationships.relates.contains(&relation) {
            return false;
        }
        relationships.relates.push(relation);
    }
    true
}

fn collect_issue_record_paths(root: &Path, dir: &Path, records: &mut Vec<PathBuf>) -> Result<()> {
    collect_record_paths(root, dir, "md", "issue", records)
}

fn collect_record_paths(
    root: &Path,
    dir: &Path,
    extension: &str,
    kind_name: &str,
    records: &mut Vec<PathBuf>,
) -> Result<()> {
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
            collect_record_paths(root, &path, extension, kind_name, records)?;
        } else if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .context("Failed to relativize canonical record path")?
                .to_path_buf();
            if is_local_atelier_path(&relative) {
                continue;
            }
            if relative.extension().and_then(|ext| ext.to_str()) != Some(extension) {
                bail!(
                    "Unsupported canonical {} file {}; expected .{} record",
                    kind_name,
                    display_state_path(&relative),
                    extension
                );
            }
            records.push(relative);
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
        || file_name.ends_with(".journal")
        || file_name.ends_with("-journal")
        || file_name.ends_with("-wal")
        || file_name.ends_with("-shm")
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

fn parse_yaml_document(text: &str, relative: &Path) -> Result<BTreeMap<String, Value>> {
    serde_yaml::from_str(text)
        .with_context(|| format!("Invalid YAML document in {}", display_state_path(relative)))
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

fn optional_object(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<BTreeMap<String, Value>> {
    let Some(value) = values.get(key) else {
        return Ok(BTreeMap::new());
    };
    let Some(object) = value.as_object() else {
        bail!(
            "Front matter key '{}' must be a mapping in {}",
            key,
            display_state_path(relative)
        );
    };
    Ok(object
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect())
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

fn optional_datetime(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Option<DateTime<Utc>>> {
    let Some(value) = values.get(key) else {
        return Ok(None);
    };
    if value.is_null() {
        return Ok(None);
    }
    let Some(value) = value.as_str() else {
        bail!(
            "Front matter key '{}' in {} must be a string",
            key,
            display_state_path(relative)
        );
    };
    DateTime::parse_from_rfc3339(value)
        .map(|dt| Some(dt.with_timezone(&Utc)))
        .with_context(|| {
            format!(
                "Invalid datetime '{}' in key '{}' of {}",
                value,
                key,
                display_state_path(relative)
            )
        })
}

fn optional_scalar(values: &BTreeMap<String, Value>, key: &str) -> Result<Option<String>> {
    let Some(value) = values.get(key) else {
        return Ok(None);
    };
    if value.is_null() {
        return Ok(None);
    }
    value
        .as_str()
        .map(|value| Some(value.to_string()))
        .ok_or_else(|| anyhow!("Front matter key '{}' must be a string", key))
}

fn optional_i32(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Option<i32>> {
    let Some(value) = values.get(key) else {
        return Ok(None);
    };
    if value.is_null() {
        return Ok(None);
    }
    let value = value.as_i64().ok_or_else(|| {
        anyhow!(
            "Front matter key '{}' in {} must be an integer",
            key,
            display_state_path(relative)
        )
    })?;
    Ok(Some(i32::try_from(value).with_context(|| {
        format!(
            "Front matter key '{}' in {} exceeds i32 range",
            key,
            display_state_path(relative)
        )
    })?))
}

fn optional_bool(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Option<bool>> {
    let Some(value) = values.get(key) else {
        return Ok(None);
    };
    if value.is_null() {
        return Ok(None);
    }
    value.as_bool().map(Some).ok_or_else(|| {
        anyhow!(
            "Front matter key '{}' in {} must be a boolean",
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

fn optional_string_array(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Option<Vec<String>>> {
    let Some(values) = values.get(key) else {
        return Ok(None);
    };
    let values = values.as_array().ok_or_else(|| {
        anyhow!(
            "Front matter key '{}' in {} must be an array",
            key,
            display_state_path(relative)
        )
    })?;
    values
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
        .collect::<Result<Vec<_>>>()
        .map(Some)
}

fn optional_yaml_value<T: serde::de::DeserializeOwned>(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Option<T>> {
    let Some(value) = values.get(key) else {
        return Ok(None);
    };
    if value.is_null() {
        return Ok(None);
    }
    serde_json::from_value(value.clone())
        .with_context(|| {
            format!(
                "Front matter key '{}' in {} has invalid nested YAML",
                key,
                display_state_path(relative)
            )
        })
        .map(Some)
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

fn reject_legacy_issue_keys(values: &BTreeMap<String, Value>, relative: &Path) -> Result<()> {
    let keys = ["acceptance", "evidence_required"]
        .into_iter()
        .filter(|key| values.contains_key(*key))
        .collect::<Vec<_>>();
    if !keys.is_empty() {
        bail!(
            "Legacy issue front matter key(s) {} in {}; use issue body sections",
            keys.join(", "),
            display_state_path(relative)
        );
    }
    Ok(())
}

pub fn parse_issue_sections(body: &str, relative: &Path) -> Result<IssueSections> {
    let mut sections = BTreeMap::<String, String>::new();
    let mut current_heading: Option<String> = None;
    let mut current_body = String::new();
    let issue_id = issue_id_from_record_path(relative);

    for line in body.lines() {
        if let Some(heading) = issue_level_two_heading(line) {
            if !IssueSections::ALL_NAMES
                .into_iter()
                .any(|name| name.title() == heading)
            {
                bail!(
                    "{}",
                    issue_section_diagnostic(
                        issue_id.as_deref(),
                        &heading,
                        relative,
                        &format!("Unknown issue body section '{heading}'")
                    )
                );
            }
            if let Some(previous) = current_heading.replace(heading.clone()) {
                finish_issue_section(
                    &mut sections,
                    previous,
                    &current_body,
                    relative,
                    issue_id.as_deref(),
                )?;
                current_body.clear();
            } else if !current_body.trim().is_empty() {
                bail!(
                    "Content before first recognized issue body section in {}",
                    display_state_path(relative)
                );
            } else {
                current_body.clear();
            }
            continue;
        }

        if current_heading.is_none() && !line.trim().is_empty() {
            bail!(
                "Content before first recognized issue body section in {}",
                display_state_path(relative)
            );
        }
        current_body.push_str(line);
        current_body.push('\n');
    }

    if let Some(previous) = current_heading {
        finish_issue_section(
            &mut sections,
            previous,
            &current_body,
            relative,
            issue_id.as_deref(),
        )?;
    } else {
        bail!(
            "{}",
            issue_section_diagnostic(
                issue_id.as_deref(),
                IssueSectionName::Description.title(),
                relative,
                "Missing required issue body section 'Description'"
            )
        );
    }

    let description = required_issue_section(
        &sections,
        IssueSectionName::Description,
        relative,
        issue_id.as_deref(),
    )?;
    let outcome = required_issue_section(
        &sections,
        IssueSectionName::Outcome,
        relative,
        issue_id.as_deref(),
    )?;
    let evidence = sections
        .get(IssueSectionName::Evidence.title())
        .cloned()
        .unwrap_or_default();
    let notes = sections.get(IssueSectionName::Notes.title()).cloned();

    Ok(IssueSections {
        description,
        outcome,
        evidence,
        notes,
    })
}

fn issue_level_two_heading(line: &str) -> Option<String> {
    let rest = line.strip_prefix("##")?;
    if rest.starts_with('#') {
        return None;
    }
    if !rest.is_empty() && !rest.chars().next().is_some_and(char::is_whitespace) {
        return None;
    }
    Some(rest.trim().to_string())
}

fn finish_issue_section(
    sections: &mut BTreeMap<String, String>,
    heading: String,
    body: &str,
    relative: &Path,
    issue_id: Option<&str>,
) -> Result<()> {
    if sections.contains_key(&heading) {
        bail!(
            "{}",
            issue_section_diagnostic(
                issue_id,
                &heading,
                relative,
                &format!("Duplicate issue body section '{heading}'")
            )
        );
    }
    let content = body.trim().to_string();
    if content.is_empty() {
        bail!(
            "{}",
            issue_section_diagnostic(
                issue_id,
                &heading,
                relative,
                &format!("Empty issue body section '{heading}'")
            )
        );
    }
    sections.insert(heading, content);
    Ok(())
}

fn required_issue_section(
    sections: &BTreeMap<String, String>,
    name: IssueSectionName,
    relative: &Path,
    issue_id: Option<&str>,
) -> Result<String> {
    let heading = name.title();
    sections.get(heading).cloned().ok_or_else(|| {
        anyhow!(
            "{}",
            issue_section_diagnostic(
                issue_id,
                heading,
                relative,
                &format!("Missing required issue body section '{heading}'")
            )
        )
    })
}

fn issue_id_from_record_path(relative: &Path) -> Option<String> {
    let file_name = relative.file_stem()?.to_str()?;
    record_id::validate_record_id(file_name).ok()?;
    Some(file_name.to_string())
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
        validate_record_kind(kind)?;
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
        validate_record_kind(&attachment.kind)?;
        validate_link_type(&attachment.role)?;
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
        validate_record_kind(&relation.kind)?;
        validate_relationship_type(&relation.relation_type)?;
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
        validate_record_kind(&target.kind)?;
        record_id::validate_record_id(&target.id)?;
    }
    for attachment in &relationships.attachments {
        validate_record_kind(&attachment.kind)?;
        validate_link_type(&attachment.role)?;
        record_id::validate_record_id(&attachment.id)?;
    }
    for relation in &relationships.relates {
        validate_record_kind(&relation.kind)?;
        validate_relationship_type(&relation.relation_type)?;
        record_id::validate_record_id(&relation.id)?;
    }
    let _ = relative;
    Ok(())
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

fn write_yaml_scalar_if_some(output: &mut String, key: &str, value: Option<&str>) -> Result<()> {
    if let Some(value) = value {
        write_yaml_scalar(output, key, Some(value))?;
    }
    Ok(())
}

fn write_yaml_value(output: &mut String, key: &str, value: &Value) -> Result<()> {
    output.push_str(key);
    let rendered = serde_yaml::to_string(value)?;
    let rendered = rendered.trim();
    if value.is_array() || value.is_object() {
        output.push_str(":\n");
        for line in rendered.lines() {
            output.push_str("  ");
            output.push_str(line);
            output.push('\n');
        }
        return Ok(());
    }
    output.push_str(": ");
    output.push_str(rendered);
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

fn write_yaml_array_if_not_empty(output: &mut String, key: &str, values: &[String]) -> Result<()> {
    if !values.is_empty() {
        write_yaml_array(output, key, values)?;
    }
    Ok(())
}

fn write_yaml_map_if_not_empty(
    output: &mut String,
    key: &str,
    values: &BTreeMap<String, Value>,
) -> Result<()> {
    if values.is_empty() {
        return Ok(());
    }
    output.push_str(key);
    output.push_str(":\n");
    let rendered = serde_yaml::to_string(values)?;
    for line in rendered.lines() {
        if line.trim().is_empty() {
            continue;
        }
        output.push_str("  ");
        output.push_str(line);
        output.push('\n');
    }
    Ok(())
}

fn write_evidence_target(output: &mut String, target: &EvidenceTarget) -> Result<()> {
    output.push_str("target:\n");
    output.push_str("  kind: ");
    output.push_str(&serde_json::to_string(&target.kind)?);
    output.push('\n');
    output.push_str("  id: ");
    output.push_str(&serde_json::to_string(&target.id)?);
    output.push('\n');
    output.push_str("  role: ");
    output.push_str(&serde_json::to_string(&target.role)?);
    output.push('\n');
    Ok(())
}

fn write_evidence_target_if_some(
    output: &mut String,
    target: Option<&EvidenceTarget>,
) -> Result<()> {
    if let Some(target) = target {
        write_evidence_target(output, target)?;
    }
    Ok(())
}

fn render_evidence_body(body: Option<&str>, data: &EvidenceRecordData) -> String {
    let normalized = normalize_body(body.unwrap_or(""));
    if data.output.is_none() || evidence_body_has_transcript_sections(&normalized) {
        return normalized;
    }

    let summary = evidence_summary_from_legacy_body(&normalized).unwrap_or(&data.evidence_type);
    let mut output = String::new();
    output.push_str("## Summary\n\n");
    output.push_str(summary);
    output.push_str("\n\n");
    if let Some(command) = data.command.as_deref() {
        output.push_str("## Command\n\n");
        output.push_str("```console\n");
        output.push_str(command.trim_end());
        output.push_str("\n```\n");
        if let Some(exit_status) = data.exit_status.as_deref() {
            output.push_str("\nExit status: ");
            output.push_str(exit_status);
            output.push('\n');
        }
        if let Some(error) = data.spawn_error.as_deref() {
            output.push_str("Spawn error: ");
            output.push_str(error);
            output.push('\n');
        }
        output.push('\n');
    }
    if let Some(summary) = data.output.as_ref() {
        push_evidence_stream_section(&mut output, "Stdout", &summary.stdout);
        output.push('\n');
        push_evidence_stream_section(&mut output, "Stderr", &summary.stderr);
    }
    if !data.residual_risks.is_empty() {
        output.push_str("\n## Residual Risks\n\n");
        for risk in &data.residual_risks {
            output.push_str("- ");
            output.push_str(risk);
            output.push('\n');
        }
    }
    output.trim_end().to_string()
}

fn evidence_body_has_transcript_sections(body: &str) -> bool {
    body.lines()
        .any(|line| matches!(line.trim(), "## Command" | "## Stdout" | "## Stderr"))
}

fn evidence_summary_from_legacy_body(body: &str) -> Option<&str> {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut end = trimmed.len();
    for marker in [
        "\n\nCommand:",
        "\nCommand:",
        "\nExit status:",
        "\n\nStdout summary",
        "\nStdout summary",
        "\n\nStderr summary",
        "\nStderr summary",
    ] {
        if let Some(index) = trimmed.find(marker) {
            end = end.min(index);
        }
    }
    Some(trimmed[..end].trim())
}

fn push_evidence_stream_section(
    output: &mut String,
    title: &str,
    stream: &atelier_core::EvidenceStreamSummary,
) {
    output.push_str("## ");
    output.push_str(title);
    output.push_str("\n\n");
    output.push_str(&format!("Bytes: {}\n", stream.bytes));
    output.push_str(&format!(
        "Truncated: {}\n\n",
        if stream.truncated { "yes" } else { "no" }
    ));
    output.push_str("```text\n");
    if !stream.summary.is_empty() {
        output.push_str(stream.summary.trim_end());
        output.push('\n');
    }
    output.push_str("```\n");
}

fn apply_evidence_body_sections(data: &mut EvidenceRecordData, body: &str) {
    let sections = markdown_sections(body);
    if data.command.is_none() {
        data.command = sections
            .iter()
            .find(|(title, _)| title == "Command")
            .and_then(|(_, content)| {
                first_fenced_block(content).or_else(|| first_non_empty_line(content))
            })
            .map(str::to_string);
    }
    if data.exit_status.is_none() {
        if let Some((_, command)) = sections.iter().find(|(title, _)| title == "Command") {
            data.exit_status = command
                .lines()
                .find_map(|line| line.trim().strip_prefix("Exit status:").map(str::trim))
                .filter(|value| !value.is_empty())
                .map(str::to_string);
        }
    }
    if data.output.is_none() {
        let stdout = sections
            .iter()
            .find(|(title, _)| title == "Stdout")
            .map(|(_, content)| evidence_stream_from_section(content))
            .unwrap_or_else(empty_evidence_stream);
        let stderr = sections
            .iter()
            .find(|(title, _)| title == "Stderr")
            .map(|(_, content)| evidence_stream_from_section(content))
            .unwrap_or_else(empty_evidence_stream);
        if stdout.bytes > 0
            || !stdout.summary.is_empty()
            || stderr.bytes > 0
            || !stderr.summary.is_empty()
        {
            data.output = Some(EvidenceOutputSummary {
                limit_bytes_per_stream: 4096,
                stdout,
                stderr,
            });
        }
    }
}

fn markdown_sections(body: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current_title: Option<String> = None;
    let mut current_body = String::new();
    for line in body.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            if let Some(title) = current_title.replace(title.trim().to_string()) {
                sections.push((title, current_body.trim().to_string()));
                current_body.clear();
            }
        } else if current_title.is_some() {
            current_body.push_str(line);
            current_body.push('\n');
        }
    }
    if let Some(title) = current_title {
        sections.push((title, current_body.trim().to_string()));
    }
    sections
}

fn first_fenced_block(content: &str) -> Option<&str> {
    let mut fence_start = None;
    let mut line_start = 0;
    for line in content.split_inclusive('\n') {
        let line_end = line_start + line.len();
        let line_text = line.trim_end_matches('\n');
        if line_text.trim_start().starts_with("```") {
            if let Some(start) = fence_start {
                return Some(content[start..line_start].trim_end());
            }
            fence_start = Some(line_end);
        }
        line_start = line_end;
    }
    None
}

fn first_non_empty_line(content: &str) -> Option<&str> {
    content.lines().map(str::trim).find(|line| !line.is_empty())
}

fn evidence_stream_from_section(content: &str) -> atelier_core::EvidenceStreamSummary {
    let summary = first_fenced_block(content).unwrap_or("").to_string();
    let bytes = content
        .lines()
        .find_map(|line| line.trim().strip_prefix("Bytes:"))
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or_else(|| summary.len());
    let truncated = content
        .lines()
        .find_map(|line| line.trim().strip_prefix("Truncated:"))
        .map(|value| matches!(value.trim(), "yes" | "true"))
        .unwrap_or(false);
    atelier_core::EvidenceStreamSummary {
        summary,
        bytes,
        truncated,
    }
}

fn empty_evidence_stream() -> atelier_core::EvidenceStreamSummary {
    atelier_core::EvidenceStreamSummary {
        summary: String::new(),
        bytes: 0,
        truncated: false,
    }
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
    IssuePriority::from_label(priority)
        .map(|priority| priority.canonical_token().to_string())
        .unwrap_or_else(|_| priority.to_string())
}

fn db_priority(priority: &str) -> Result<String> {
    IssuePriority::from_canonical_token(priority)
        .map(|priority| priority.label().to_string())
        .map_err(Into::into)
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
        let body = "## Description\n\nCanonical body\n\n## Outcome\n\nIssue Markdown round-trips without losing fields.\n\n## Evidence\n\n- `cargo test record_store` passes.";
        CanonicalIssueRecord {
            issue: Issue {
                id: id.to_string(),
                title: "Write RecordStore".to_string(),
                description: None,
                status: "todo".to_string(),
                issue_type: "task".to_string(),
                priority: "high".to_string(),
                fields: BTreeMap::new(),
                parent_id: None,
                created_at: Utc.with_ymd_and_hms(2026, 6, 10, 12, 0, 0).unwrap(),
                updated_at: Utc.with_ymd_and_hms(2026, 6, 10, 13, 0, 0).unwrap(),
                closed_at: None,
            },
            labels: vec!["record-store".to_string(), "storage".to_string()],
            sections: parse_issue_sections(body, &issue_record_path(id)).unwrap(),
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

    fn record_header(
        kind: &str,
        id: &str,
        title: &str,
        status: &str,
        labels: Vec<String>,
        relationships: Relationships,
    ) -> RecordHeader {
        RecordHeader {
            id: id.to_string(),
            kind: kind.to_string(),
            title: title.to_string(),
            status: status.to_string(),
            labels,
            relationships,
            created_at: Utc.with_ymd_and_hms(2026, 6, 10, 12, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 6, 10, 13, 0, 0).unwrap(),
        }
    }

    fn evidence_record(id: &str) -> Record {
        let data = EvidenceRecordData {
            evidence_type: "validation".to_string(),
            captured_at: Utc.with_ymd_and_hms(2026, 6, 10, 12, 30, 0).unwrap(),
            command: Some("cargo test record_store".to_string()),
            path: Some("docs/architecture/markdown-first-record-store.md".to_string()),
            uri: None,
            producer: None,
            proof_scope: Some("Outcome: canonical evidence front matter is readable".to_string()),
            agent_identity: Some("gpt-5.4 implementer".to_string()),
            independence_level: Some("implementer".to_string()),
            residual_risks: vec!["Need focused export/rebuild verification.".to_string()],
            follow_up_ids: vec!["atelier-follow".to_string()],
            exit_status: Some("0".to_string()),
            exit_code: None,
            success: None,
            spawn_error: None,
            output: None,
            target: None,
        };
        Record::Evidence(EvidenceRecord {
            header: record_header(
                "evidence",
                id,
                "RecordStore evidence proof",
                "recorded",
                Vec::new(),
                Relationships {
                    blocks: Vec::new(),
                    children: Vec::new(),
                    attachments: vec![AttachmentRelationship {
                        kind: "issue".to_string(),
                        id: "atelier-proof".to_string(),
                        role: "validates".to_string(),
                    }],
                    relates: Vec::new(),
                },
            ),
            data,
            summary: "RecordStore evidence proof summary.".to_string(),
        })
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
status: "todo"
title: "Write RecordStore"
updated_at: "2026-06-10T13:00:00+00:00"
---

{body}
"#
        )
    }

    #[test]
    fn registered_first_class_record_kinds_have_canonical_contracts() {
        let canonical_contracts = CANONICAL_RECORD_KINDS
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
            canonical_contracts,
            vec![
                ("issue", "atelier.issue", 1, Some("issues")),
                ("evidence", "atelier.evidence", 1, Some("evidence")),
                ("review", "atelier.review", 1, Some("reviews")),
            ]
        );
        for (kind, _, _, _) in canonical_contracts {
            assert!(validate_canonical_record_kind(kind).is_ok());
            let spec = canonical_record_kind(kind).unwrap();
            let extension = if kind == "review" { "yaml" } else { "md" };
            assert_eq!(
                canonical_record_path(spec, "atelier-abcd").unwrap(),
                PathBuf::from(spec.canonical_dir.unwrap())
                    .join(format!("atelier-abcd.{extension}"))
            );
        }

        let generic_contracts = FIRST_CLASS_RECORD_KINDS
            .iter()
            .map(|spec| spec.kind)
            .collect::<Vec<_>>();
        assert_eq!(generic_contracts, vec!["evidence", "review"]);
    }

    #[test]
    fn plan_and_milestone_record_kinds_are_deferred() {
        for kind in ["plan", "milestone"] {
            assert!(validate_record_kind(kind).is_err());
            assert!(validate_canonical_record_kind(kind).is_err());
            assert!(canonical_record_kind(kind).is_err());
        }
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
        assert!(text.contains("priority: \"P1\""));
        assert!(text.contains("relationships:\n"));
        assert!(text.contains("  blocks:\n  - kind: \"issue\"\n    id: \"atelier-bbbb\"\n"));
        assert!(text.contains("  children:\n  - kind: \"issue\"\n    id: \"atelier-aaaa\"\n"));
        assert!(text.contains(
            "  relates:\n  - kind: \"issue\"\n    id: \"atelier-cccc\"\n    type: \"related\"\n"
        ));
        assert!(!text.contains("acceptance:"));
        assert!(!text.contains("evidence_required:"));
        assert!(text.contains("## Outcome\n\nIssue Markdown round-trips without losing fields."));
        assert!(text.contains("## Evidence\n\n- `cargo test record_store` passes."));
        assert!(!text.contains("closed_at:"));
    }

    #[test]
    fn issue_record_round_trips_review_link() {
        let mut record = issue_record("atelier-flds");
        record.issue.fields.insert(
            "review".to_string(),
            serde_json::json!({"kind": "room", "id": "atelier-rvw1"}),
        );

        let text = render_issue_record(&record).unwrap();
        let parsed = parse_issue_record(&text, &issue_record_path("atelier-flds")).unwrap();

        assert_eq!(parsed.issue.fields, record.issue.fields);
        assert_eq!(render_issue_record(&parsed).unwrap(), text);
        assert!(text.contains("review:\n"));
        assert!(text.contains("kind: room"));
        assert!(text.contains("id: atelier-rvw1"));
        assert!(!text.contains("fields:\n"));
    }

    #[test]
    fn issue_record_rejects_legacy_pull_request_field() {
        let text = sectioned_issue_text(
            "atelier-flds",
            "## Description\n\nBody\n\n## Outcome\n\nDone\n\n## Evidence\n\nProof",
        )
        .replace("priority: \"P1\"\n", "priority: \"P1\"\npull_request: 42\n");

        let error = parse_issue_record(&text, &issue_record_path("atelier-flds"))
            .unwrap_err()
            .to_string();

        assert!(error.contains("Legacy pull_request field"));
    }

    #[test]
    fn review_room_record_renders_and_parses_yaml() {
        let record = Record::Review(ReviewRecord {
            header: record_header(
                "review",
                "atelier-rvw1",
                "Review room",
                "open",
                vec!["review".to_string()],
                Relationships::default(),
            ),
            mode: "room".to_string(),
            issue_id: "atelier-epic".to_string(),
            source_branch: "epic/atelier-epic".to_string(),
            target_branch: "master".to_string(),
            events: vec![serde_json::json!({
                "id": "evt-0001",
                "kind": "comment",
                "actor": "reviewer",
                "body": "Looks good"
            })],
        });
        let spec = canonical_record_kind("review").unwrap();
        let text = render_record(&record).unwrap();
        let parsed = parse_record(
            &text,
            &canonical_record_path(spec, "atelier-rvw1").unwrap(),
            spec,
        )
        .unwrap();

        assert_eq!(parsed, record);
        assert_eq!(render_record(&parsed).unwrap(), text);
        assert!(text.contains("schema: \"atelier.review\""));
        assert!(text.contains("events:\n"));
        assert!(!text.starts_with("---"));
    }

    #[test]
    fn issue_record_uses_generic_canonical_record_dispatch() {
        let record = issue_record("atelier-genr").into_record();
        let text = render_record(&record).unwrap();
        let spec = canonical_record_kind("issue").unwrap();
        let parsed = parse_record(
            &text,
            &canonical_record_path(spec, "atelier-genr").unwrap(),
            spec,
        )
        .unwrap();

        assert_eq!(parsed, record);
        assert_eq!(
            canonical_record_path(spec, "atelier-genr").unwrap(),
            issue_record_path("atelier-genr")
        );
    }

    #[test]
    fn ownership_modules_expose_supported_record_boundaries() {
        let record = issue::CanonicalIssueRecord {
            ..issue_record("atelier-mods")
        };
        let text = issue::render_issue_record(&record).unwrap();
        assert!(document::split_document(&text).is_some());
        assert_eq!(issue::IssueSectionName::Evidence.title(), "Evidence");
        let _store = store::RecordStore::new(tempdir().unwrap().path());
        validation::validate_priority("high").unwrap();
        let _: Option<evidence::EvidenceOutputSummary> = None;
    }

    #[test]
    fn issue_record_round_trips_explicit_closed_at_for_done_status() {
        let mut record = issue_record("atelier-done");
        let closed_at = Utc.with_ymd_and_hms(2026, 6, 10, 14, 0, 0).unwrap();
        record.issue.status = "done".to_string();
        record.issue.closed_at = Some(closed_at);

        let text = render_issue_record(&record).unwrap();
        let parsed = parse_issue_record(&text, &issue_record_path("atelier-done")).unwrap();

        assert!(text.contains("closed_at: \"2026-06-10T14:00:00+00:00\""));
        assert_eq!(parsed.issue.status, "done");
        assert_eq!(parsed.issue.closed_at, Some(closed_at));
    }

    #[test]
    fn record_store_mutates_issue_child_relationships_in_canonical_markdown() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path());
        let mut parent = issue_record("atelier-par1");
        parent.relationships = Relationships::default();
        let mut child = issue_record("atelier-chd1");
        child.relationships = Relationships::default();
        store.write_issue_atomic(&parent).unwrap();
        store.write_issue_atomic(&child).unwrap();

        assert!(store
            .add_issue_child("atelier-par1", "atelier-chd1")
            .unwrap());
        assert_eq!(
            store.find_issue_parent("atelier-chd1").unwrap().as_deref(),
            Some("atelier-par1")
        );
        let parent_text = fs::read_to_string(dir.path().join("issues/atelier-par1.md")).unwrap();
        assert!(parent_text.contains("children:\n  - kind: \"issue\"\n    id: \"atelier-chd1\""));

        assert!(store
            .remove_issue_child("atelier-par1", "atelier-chd1")
            .unwrap());
        assert_eq!(store.find_issue_parent("atelier-chd1").unwrap(), None);
    }

    #[test]
    fn record_store_mutates_generic_issue_and_domain_relationships() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path());
        let mut issue = issue_record("atelier-iss1");
        issue.relationships = Relationships::default();
        let mut evidence = evidence_record("atelier-evd1");
        evidence.header_mut().relationships = Relationships::default();
        store.write_issue_atomic(&issue).unwrap();
        store.write_record_atomic(&evidence).unwrap();

        assert!(store
            .add_record_relationship(
                "issue",
                "atelier-iss1",
                "evidence",
                "atelier-evd1",
                "validates",
            )
            .unwrap());

        let issue_text = fs::read_to_string(dir.path().join("issues/atelier-iss1.md")).unwrap();
        assert!(issue_text.contains(
            "attachments:\n  - kind: \"evidence\"\n    id: \"atelier-evd1\"\n    role: \"validates\""
        ));
    }

    #[test]
    fn evidence_record_renders_and_parses_deterministically_without_data_blob() {
        let record = evidence_record("atelier-evdn");
        let spec = canonical_record_kind("evidence").unwrap();
        let path = canonical_record_path(spec, "atelier-evdn").unwrap();
        let text = render_record(&record).unwrap();
        let parsed = parse_record(&text, &path, spec).unwrap();

        assert_eq!(parsed, record);
        assert_eq!(render_record(&parsed).unwrap(), text);
        assert!(text.contains("schema: \"atelier.evidence\""));
        assert!(!text.contains("\ndata: "));
        assert!(text.contains("evidence_type: \"validation\""));
        assert!(text.contains("captured_at: \"2026-06-10T12:30:00+00:00\""));
        assert!(text.contains("command: \"cargo test record_store\""));
        assert!(text.contains("agent_identity: \"gpt-5.4 implementer\""));
        assert!(text.contains("follow_up_ids:\n- \"atelier-follow\"\n"));
        assert!(text.contains("RecordStore evidence proof summary."));
        assert!(!text.contains("output:"));
        assert!(!text.contains(&format!("{}: null", "target")));
    }

    #[test]
    fn evidence_record_rejects_data_front_matter() {
        let spec = canonical_record_kind("evidence").unwrap();
        let path = canonical_record_path(spec, "atelier-eleg").unwrap();
        let text = r#"---
created_at: "2026-06-10T12:00:00+00:00"
id: "atelier-eleg"
data: "{\"captured_at\":\"2026-06-10T12:30:00Z\",\"command\":\"cargo test record_store\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stdout\":{\"bytes\":25,\"summary\":\"record_store tests passed\\n\",\"truncated\":false},\"stderr\":{\"bytes\":0,\"summary\":\"\",\"truncated\":false}},\"path\":\"docs/architecture/markdown-first-record-store.md\",\"producer\":\"legacy agent\",\"proof_scope\":\"Outcome: readable evidence metadata\",\"result\":\"pass\",\"success\":true,\"target\":{\"id\":\"atelier-proof\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  attachments: []
  blocks: []
  children: []
  relates:
  - kind: "issue"
    id: "atelier-proof"
    type: "validates"
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Legacy evidence proof"
updated_at: "2026-06-10T13:00:00+00:00"
---

Legacy evidence summary.
"#;
        let error = parse_record(text, &path, spec).unwrap_err();
        assert!(error.to_string().contains("Forbidden data front matter"));
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
    fn issue_parser_contract_accepts_sectioned_body_without_legacy_arrays() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.\n\n## Notes\n\nSequencing context.";
        let text = sectioned_issue_text("atelier-abcd", body);
        let parsed = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap();

        assert_eq!(parsed.issue.description, None);
        assert_eq!(
            parsed.sections.description,
            "Canonical problem statement.".to_string()
        );
        assert_eq!(
            parsed.sections.outcome,
            "The desired finished world is observable.".to_string()
        );
        assert_eq!(
            parsed.sections.evidence,
            "- `atelier lint atelier-abcd` passes.".to_string()
        );
        assert_eq!(
            parsed.sections.notes.as_deref(),
            Some("Sequencing context.")
        );
    }

    #[test]
    fn issue_sections_report_shared_presence_state_and_search_text() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.";
        let sections = parse_issue_sections(body, &issue_record_path("atelier-abcd")).unwrap();
        let states = sections.section_states();

        assert_eq!(
            states
                .iter()
                .filter(|state| state.required && state.present && !state.empty)
                .count(),
            2
        );
        assert!(states.iter().any(|state| {
            state.name == IssueSectionName::Evidence
                && !state.required
                && state.present
                && !state.empty
        }));
        assert!(states.iter().any(|state| {
            state.name == IssueSectionName::Notes
                && !state.required
                && !state.present
                && state.empty
        }));
        assert!(sections
            .searchable_text()
            .contains("The desired finished world is observable."));
    }

    #[test]
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
    fn issue_parser_contract_rejects_label_priority_in_canonical_markdown() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body)
            .replace("priority: \"P1\"", "priority: \"high\"");

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error.to_string().contains("Invalid priority"));
    }

    #[test]
    fn issue_parser_contract_accepts_missing_evidence_section() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let parsed = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap();

        assert_eq!(
            parsed.sections.outcome,
            "The desired finished world is observable."
        );
        assert_eq!(parsed.sections.section(IssueSectionName::Evidence), None);
        assert!(!render_issue_record(&parsed)
            .unwrap()
            .contains("## Evidence"));
    }

    #[test]
    fn issue_parser_contract_rejects_missing_required_sections() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Missing required issue body section 'Outcome'"));
    }

    #[test]
    fn issue_parser_contract_rejects_duplicate_recognized_headings() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.\n\n## Outcome\n\nA second outcome.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Duplicate issue body section 'Outcome'"));
    }

    #[test]
    fn issue_parser_contract_rejects_content_before_first_recognized_heading() {
        let body = "Preamble is not part of the issue contract.\n\n## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Content before first recognized issue body section"));
    }

    #[test]
    fn issue_parser_contract_rejects_unknown_top_level_sections() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Outcome\n\nThe desired finished world is observable.\n\n## Acceptance\n\nLegacy section name.\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Unknown issue body section 'Acceptance'"));
    }

    #[test]
    fn issue_parser_contract_rejects_empty_present_sections() {
        let body = "## Description\n\nCanonical problem statement.\n\n## Outcome\n\n\n## Evidence\n\n- `atelier lint atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Empty issue body section 'Outcome'"));
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
        fs::create_dir_all(store.state_dir.join("evidence")).unwrap();
        fs::write(
            store
                .state_dir
                .join("evidence")
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
    fn record_store_rejects_issue_to_issue_validates_relation() {
        let dir = tempdir().unwrap();
        let store = RecordStore::new(dir.path().join(".atelier"));
        let mut first = issue_record("atelier-abcd");
        first.relationships = Relationships::default();
        let mut second = issue_record("atelier-efgh");
        second.relationships = Relationships::default();
        store.write_issue_atomic(&first).unwrap();
        store.write_issue_atomic(&second).unwrap();

        let error = store
            .add_issue_relation("atelier-abcd", "atelier-efgh", "validates")
            .unwrap_err()
            .to_string();

        assert!(error.contains("Issue-to-issue relationship 'validates' is not supported"));
        assert!(error.contains("attach evidence"));
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

    #[test]
    fn canonical_record_kinds_exclude_sessions() {
        let kinds = CANONICAL_RECORD_KINDS
            .iter()
            .map(|spec| spec.kind)
            .collect::<Vec<_>>();

        assert!(!kinds.contains(&"session"));
        assert!(canonical_record_kind("session").is_err());
        assert!(validate_canonical_record_kind("session").is_err());
    }
}
