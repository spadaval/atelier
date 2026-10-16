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
};
pub use atelier_core::{
    EvidenceRecord, EvidenceResult, IssueBody, IssueFieldError, IssueSectionName,
    IssueSectionState, IssueSections, IssueStrategy, RecordHeader, WorkflowBranch,
};

pub mod mutation_lock;
pub mod review;

mod record_id;
mod record_kinds;
mod relationships;

pub use record_id::{derive_project_slug, validate_project_slug, validate_record_id};

pub use record_kinds::{
    canonical_record_dirs, canonical_record_kind, canonical_record_path, issue_record_path,
    validate_record_kind, RecordKindSpec, CANONICAL_RECORD_KINDS, ISSUE_KIND, REVIEW_KIND,
};
pub use relationships::{
    attachment_relationship, issue_relates_relationship, issue_relationship_target,
    relates_relationship, relationship_target, sort_relationships, AttachmentRelationship,
    RelatesRelationship, RelationshipTarget, Relationships,
};

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

pub const MAX_LABEL_LEN: usize = 128;
pub fn validate_status(status: &str) -> Result<()> {
    validate_token(status, "status", "Status")
}

fn validate_token(value: &str, field: &str, label: &str) -> Result<()> {
    let mut chars = value.chars();
    if matches!(chars.next(), Some(first) if first.is_ascii_lowercase())
        && chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        Ok(())
    } else {
        bail!("Invalid {field} '{value}'. {label} values must match ^[a-z][a-z0-9_]*$")
    }
}

pub fn validate_priority(priority: &str) -> Result<()> {
    IssuePriority::from_cli_input(priority)
        .map(|_| ())
        .map_err(Into::into)
}

pub fn validate_issue_type(issue_type: &str) -> Result<()> {
    validate_token(issue_type, "issue_type", "Issue type")
}

pub fn validate_relation_type(relation_type: &str) -> Result<()> {
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
    validate_relation_type(relation_type)
}

pub fn validate_relationship_type(relation_type: &str) -> Result<()> {
    validate_relation_type(relation_type)
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

pub struct RecordStore {
    state_dir: PathBuf,
    project_slug: Option<String>,
}

impl RecordStore {
    /// A store for reading and updating existing records.
    pub fn new(state_dir: impl Into<PathBuf>) -> Self {
        Self {
            state_dir: state_dir.into(),
            project_slug: None,
        }
    }

    /// A store that can also create records, minting IDs from the project's
    /// configured slug.
    pub fn for_project(state_dir: impl Into<PathBuf>, project_slug: impl Into<String>) -> Self {
        Self {
            state_dir: state_dir.into(),
            project_slug: Some(project_slug.into()),
        }
    }

    pub fn discover_issue_paths(&self) -> Result<Vec<PathBuf>> {
        self.discover_record_paths(ISSUE_KIND.kind)
    }

    /// Discover record files for one concrete kind using the shared directory
    /// and extension contract.
    pub fn discover_record_paths(&self, kind: &str) -> Result<Vec<PathBuf>> {
        let spec = canonical_record_kind(kind)?;
        let record_dir = self.state_dir.join(spec.canonical_dir);
        if !record_dir.exists() {
            return Ok(Vec::new());
        }

        let mut records = Vec::new();
        collect_record_paths(
            &self.state_dir,
            &record_dir,
            spec.extension,
            spec.kind,
            &mut records,
        )?;
        records.sort();
        Ok(records)
    }

    pub fn load_issue(&self, relative: &Path) -> Result<CanonicalIssueRecord> {
        let text = self.read_record_file(relative)?;
        parse_issue_record(&text, relative)
    }

    pub fn load_issue_by_id(&self, id: &str) -> Result<CanonicalIssueRecord> {
        record_id::validate_record_id(id)?;
        let spec = canonical_record_kind(ISSUE_KIND.kind)?;
        self.load_issue(&canonical_record_path(spec, id)?)
    }

    pub fn load_evidence_by_id(&self, id: &str) -> Result<EvidenceRecord> {
        record_id::validate_record_id(id)?;
        let spec = canonical_record_kind("evidence")?;
        let relative = canonical_record_path(spec, id)?;
        let text = self.read_record_file(&relative)?;
        parse_evidence_record_file(&text, &relative)
    }

    pub fn load_issues(&self) -> Result<Vec<CanonicalIssueRecord>> {
        let mut records = Vec::new();
        let mut ids = BTreeSet::new();
        for relative in self.discover_issue_paths()? {
            let record = self.load_issue(&relative)?;
            if !ids.insert(record.issue.id.clone()) {
                bail!("Duplicate issue ID in record files: {}", record.issue.id);
            }
            records.push(record);
        }
        records.sort_by(|a, b| a.issue.id.cmp(&b.issue.id));
        Ok(records)
    }

    pub fn allocate_issue_id(&self) -> Result<String> {
        let slug = self
            .project_slug
            .as_deref()
            .ok_or_else(|| anyhow!("record ID allocation requires the configured project slug"))?;
        record_id::allocate_issue_id(slug, |candidate| self.canonical_id_exists(candidate))
    }

    pub fn allocate_record_id(&self) -> Result<String> {
        self.allocate_issue_id()
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
        let _transaction = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
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
        let _transaction = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
        let now = Utc::now();
        let record = EvidenceRecord {
            header: RecordHeader {
                kind: "evidence".to_string(),
                id: self.allocate_record_id()?,
                title: title.to_string(),
                status: status.to_string(),
                labels: Vec::new(),
                relationships: Relationships::default(),
                created_at: now,
                updated_at: now,
            },
            data,
            summary: summary.to_string(),
        };
        self.write_evidence_atomic(&record)?;
        Ok(record)
    }

    pub fn write_evidence_atomic(&self, record: &EvidenceRecord) -> Result<()> {
        let _transaction = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
        let spec = canonical_record_kind("evidence")?;
        let relative = canonical_record_path(spec, &record.header.id)?;
        self.write_atomic(&relative, render_evidence_record_file(record)?)
    }

    pub fn add_attachment_relationship(
        &self,
        source_kind: &str,
        source_id: &str,
        target_kind: &str,
        target_id: &str,
        role: &str,
    ) -> Result<bool> {
        let _transaction = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
        validate_link_type(role)?;
        self.mutate_relationships(source_kind, source_id, |relationships| {
            insert_unique(
                &mut relationships.attachments,
                attachment_relationship(target_kind, target_id, role),
            )
        })
    }

    pub fn add_issue_child(&self, parent_id: &str, child_id: &str) -> Result<bool> {
        let _transaction = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
        if parent_id == child_id {
            bail!("An issue cannot be its own parent");
        }
        self.load_issue_by_id(child_id)?;
        let target = issue_relationship_target(child_id);
        let mut parent = self.load_issue_by_id(parent_id)?;
        if !insert_unique(&mut parent.relationships.children, target) {
            return Ok(false);
        }
        parent.issue.updated_at = Utc::now();
        self.write_issue_atomic(&parent)?;
        Ok(true)
    }

    pub fn remove_issue_child(&self, parent_id: &str, child_id: &str) -> Result<bool> {
        let _transaction = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
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

    pub fn add_record_relationship(
        &self,
        source_kind: &str,
        source_id: &str,
        target_kind: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<bool> {
        let _transaction = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
        validate_record_kind(target_kind)?;
        record_id::validate_record_id(target_id)?;
        if is_attachment_role(relation_type) {
            validate_link_type(relation_type)?;
        } else {
            validate_relationship_type(relation_type)?;
        }

        self.mutate_relationships(source_kind, source_id, |relationships| {
            add_relationship_to_bucket(relationships, target_kind, target_id, relation_type)
        })
    }

    // Each caller holds the exclusive transaction before validating/loading.
    fn mutate_relationships(
        &self,
        source_kind: &str,
        source_id: &str,
        mutate: impl FnOnce(&mut Relationships) -> bool,
    ) -> Result<bool> {
        record_id::validate_record_id(source_id)?;
        let spec = canonical_record_kind(source_kind)?;
        let relative = canonical_record_path(spec, source_id)?;
        let text = self.read_record_file(&relative)?;
        match spec.kind {
            "issue" => {
                let mut record = parse_issue_record(&text, &relative)?;
                if !mutate(&mut record.relationships) {
                    return Ok(false);
                }
                record.issue.updated_at = Utc::now();
                self.write_issue_atomic(&record)?;
            }
            "evidence" => {
                let mut record = parse_evidence_record_file(&text, &relative)?;
                if !mutate(&mut record.header.relationships) {
                    return Ok(false);
                }
                record.header.updated_at = Utc::now();
                self.write_evidence_atomic(&record)?;
            }
            "review" => {
                let mut record = review::parse_review_record(&text, &relative)?;
                if mutate(&mut record.header.relationships) {
                    bail!("review records are append-only; create a successor event");
                }
                return Ok(false);
            }
            _ => bail!(
                "Record kind '{}' is not a first-class record-file type",
                spec.kind
            ),
        }
        Ok(true)
    }

    pub fn add_issue_block(&self, blocked_id: &str, blocker_id: &str) -> Result<bool> {
        let _transaction = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
        if blocked_id == blocker_id {
            bail!("An issue cannot block itself");
        }
        self.load_issue_by_id(blocked_id)?;
        if self.would_create_block_cycle(blocked_id, blocker_id)? {
            bail!("Adding this dependency would create a circular dependency chain");
        }

        let target = issue_relationship_target(blocked_id);
        let mut blocker = self.load_issue_by_id(blocker_id)?;
        if !insert_unique(&mut blocker.relationships.blocks, target) {
            return Ok(false);
        }
        self.write_issue_atomic(&blocker)?;
        Ok(true)
    }

    pub fn remove_issue_block(&self, blocked_id: &str, blocker_id: &str) -> Result<bool> {
        let _transaction = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
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
        let _transaction = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
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
        let _transaction = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
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

    fn write_atomic(&self, relative: &Path, contents: String) -> Result<()> {
        let _lock = mutation_lock::CanonicalMutationLock::exclusive(&self.state_dir)?;
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

    fn read_record_file(&self, relative: &Path) -> Result<String> {
        let bytes = fs::read(self.state_dir.join(relative))
            .with_context(|| format!("Missing record file {}", display_state_path(relative)))?;
        String::from_utf8(bytes)
            .with_context(|| format!("Record file {} is not UTF-8", display_state_path(relative)))
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
    if let Some(strategy) = &record.issue.strategy {
        write_yaml_value(&mut output, "strategy", &serde_json::to_value(strategy)?)?;
    }
    if let Some(branch) = &record.issue.workflow_branch {
        write_yaml_value(
            &mut output,
            "workflow_branch",
            &serde_json::to_value(branch)?,
        )?;
    }
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
    let body = render_issue_sections(&record.sections);
    let path = issue_record_path(&record.issue.id);
    parse_issue_sections_for_type(&body, &path, &record.issue.issue_type).with_context(|| {
        format!(
            "Refusing to write issue {} because its rendered body would not parse",
            record.issue.id
        )
    })?;
    output.push_str(&body);
    output.push('\n');
    Ok(output)
}

pub fn render_issue_sections(sections: &IssueSections) -> String {
    normalize_body(
        &IssueSections::ALL_NAMES
            .into_iter()
            .filter_map(|name| {
                sections
                    .section(name)
                    .map(str::trim)
                    .filter(|text| !text.is_empty())
                    .map(|text| format!("## {}\n\n{text}", name.title()))
            })
            .collect::<Vec<_>>()
            .join("\n\n"),
    )
}

pub fn parse_issue_record(text: &str, relative: &Path) -> Result<CanonicalIssueRecord> {
    let (front_matter, body) = split_front_matter(text, relative)?;

    let id = validate_record_front_matter(&front_matter, relative, &ISSUE_KIND)?;

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
    if front_matter.contains_key("fields") {
        bail!("generic issue fields are obsolete in {}; use typed strategy and workflow_branch front matter", display_state_path(relative));
    }
    let strategy = optional_yaml_value::<IssueStrategy>(&front_matter, "strategy", relative)?;
    if let Some(strategy) = &strategy {
        IssueStrategy::new(&strategy.path)?;
    }
    let workflow_branch =
        optional_yaml_value::<WorkflowBranch>(&front_matter, "workflow_branch", relative)?;
    let sections = parse_issue_sections_for_type(body, relative, &issue_type)?;

    let issue = Issue {
        id,
        title: require_scalar(&front_matter, "title", relative)?,
        description: None,
        status: status.clone(),
        issue_type,
        priority: db_priority(&require_scalar(&front_matter, "priority", relative)?)
            .with_context(|| format!("Invalid priority in {}", display_state_path(relative)))?,
        strategy,
        workflow_branch,
        parent_id: None,
        created_at: require_datetime(&front_matter, "created_at", relative)?,
        updated_at,
        closed_at: closed_at.or((status == "closed").then_some(updated_at)),
    };

    Ok(CanonicalIssueRecord {
        issue,
        labels: string_array(&front_matter, "labels", relative)?,
        sections,
        relationships,
    })
}

fn validate_record_front_matter(
    front_matter: &BTreeMap<String, Value>,
    relative: &Path,
    spec: &RecordKindSpec,
) -> Result<String> {
    let schema = require_scalar(front_matter, "schema", relative)?;
    if schema != spec.schema {
        bail!(
            "Unsupported schema '{}' in {}; expected {}",
            schema,
            display_state_path(relative),
            spec.schema
        );
    }
    let schema_version = require_i64(front_matter, "schema_version", relative)?;
    if schema_version != spec.schema_version {
        bail!(
            "Unsupported schema_version {} in {}; expected {}",
            schema_version,
            display_state_path(relative),
            spec.schema_version
        );
    }

    let id = require_scalar(front_matter, "id", relative)?;
    record_id::validate_record_id(&id).with_context(|| {
        format!(
            "Invalid {} id {} in {}",
            spec.kind,
            id,
            display_state_path(relative)
        )
    })?;
    let expected = canonical_record_path(spec, &id)?;
    let kind_label = if spec.kind == "issue" {
        "Issue"
    } else {
        spec.kind
    };
    if relative != expected {
        bail!(
            "{} id {} in {} does not match record-file path {}",
            kind_label,
            id,
            display_state_path(relative),
            display_state_path(&expected)
        );
    }
    Ok(id)
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
    if !record.sections.matches_type(&record.issue.issue_type) {
        bail!(
            "body shape does not match issue_type {}",
            record.issue.issue_type
        );
    }
    for state in record.sections.section_states() {
        if state.required && state.empty {
            bail!("required section {} is empty", state.name.title());
        }
    }
    for name in IssueSections::ALL_NAMES {
        if let Some(text) = record.sections.section(name) {
            validate_issue_section_text(&record.issue.id, name, text)?;
        }
    }
    Ok(())
}

/// Reject issue section text that would open another `##` section once the
/// record is rendered, so every written issue record parses back unchanged.
pub fn validate_issue_section_text(
    issue_id: &str,
    name: IssueSectionName,
    text: &str,
) -> Result<()> {
    if normalize_body(text)
        .lines()
        .any(|line| issue_level_two_heading(line).is_some())
    {
        bail!(
            "issue {issue_id} section '{}' must not contain a `##` Markdown heading line; it would start another record section",
            name.title()
        );
    }
    Ok(())
}

/// Render a concrete evidence record through its record-file codec.
pub fn render_evidence_record_file(record: &EvidenceRecord) -> Result<String> {
    if record.header.kind != "evidence" {
        bail!("Evidence record kind must be evidence");
    }
    let relative = Path::new("<record>");
    record_id::validate_record_id(&record.header.id).with_context(|| {
        format!(
            "Invalid evidence id '{}' in {}",
            record.header.id,
            display_state_path(relative)
        )
    })?;
    validate_evidence_record(record, relative)?;
    let mut relationships = record.header.relationships.clone();
    sort_relationships(&mut relationships);
    let spec = &record_kinds::EVIDENCE_KIND;
    let data = &record.data;
    let body = render_evidence_body(Some(&record.summary), data);

    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.header.created_at.to_rfc3339()),
    )?;
    write_yaml_scalar(&mut output, "id", Some(&record.header.id))?;
    write_yaml_scalar(&mut output, "evidence_type", Some(&data.evidence_type))?;
    write_yaml_scalar(&mut output, "commit", Some(&data.commit))?;
    write_yaml_scalar(&mut output, "result", Some(data.result.as_str()))?;
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
    write_evidence_target(&mut output, data.target.as_ref())?;
    write_yaml_array_if_not_empty(&mut output, "follow_up_ids", &data.follow_up_ids)?;
    write_yaml_array_if_not_empty(&mut output, "residual_risks", &data.residual_risks)?;
    write_yaml_relationships(&mut output, &relationships)?;
    write_yaml_scalar(&mut output, "schema", Some(spec.schema))?;
    output.push_str(&format!("schema_version: {}\n", spec.schema_version));
    write_yaml_scalar(&mut output, "status", Some(&record.header.status))?;
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

/// Parse a concrete evidence record without exposing mixed-kind dispatch to
/// callers that already know the domain type.
pub fn parse_evidence_record_file(text: &str, relative: &Path) -> Result<EvidenceRecord> {
    let spec = canonical_record_kind("evidence")?;
    let (front_matter, body) = split_front_matter(text, relative)?;
    let id = validate_record_front_matter(&front_matter, relative, spec)?;
    reject_forbidden_data_front_matter(&front_matter, relative)?;
    let relationships = parse_relationships(&front_matter, relative)?;
    let title = require_scalar(&front_matter, "title", relative)?;
    let status = require_scalar(&front_matter, "status", relative)?;
    let created_at = require_datetime(&front_matter, "created_at", relative)?;
    let updated_at = require_datetime(&front_matter, "updated_at", relative)?;

    let mut data = EvidenceRecordData {
        commit: require_scalar(&front_matter, "commit", relative)?,
        result: require_scalar(&front_matter, "result", relative)?
            .parse::<EvidenceResult>()
            .map_err(anyhow::Error::msg)?,
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

    let record = EvidenceRecord {
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
    };
    validate_evidence_record(&record, relative)?;
    Ok(record)
}

fn validate_evidence_record(record: &EvidenceRecord, relative: &Path) -> Result<()> {
    let data = &record.data;
    if data.evidence_type.trim().is_empty() {
        bail!(
            "Evidence record {} must include evidence_type",
            display_state_path(relative)
        );
    }
    if !matches!(data.commit.len(), 40 | 64) || !data.commit.bytes().all(|b| b.is_ascii_hexdigit())
    {
        bail!(
            "evidence commit must be a full Git object ID in {}",
            display_state_path(relative)
        );
    }
    if data
        .success
        .is_some_and(|success| success != (data.result == EvidenceResult::Pass))
    {
        bail!(
            "evidence result conflicts with captured command success in {}",
            display_state_path(relative)
        );
    }
    if data.command.is_some() {
        if let Some(status) = &data.exit_status {
            let expected = if status == "not-started" {
                EvidenceResult::Blocked
            } else if status == "0" {
                EvidenceResult::Pass
            } else {
                EvidenceResult::Fail
            };
            if data.result != expected {
                bail!(
                    "explicit evidence result conflicts with captured command exit status in {}",
                    display_state_path(relative)
                );
            }
        }
    }
    validate_relationships(&record.header.relationships, relative)?;
    Ok(())
}

fn add_relationship_to_bucket(
    relationships: &mut Relationships,
    target_kind: &str,
    target_id: &str,
    relation_type: &str,
) -> bool {
    if is_attachment_role(relation_type) {
        insert_unique(
            &mut relationships.attachments,
            attachment_relationship(target_kind, target_id, relation_type),
        )
    } else {
        insert_unique(
            &mut relationships.relates,
            relates_relationship(target_kind, target_id, relation_type),
        )
    }
}

fn insert_unique<T: PartialEq>(values: &mut Vec<T>, value: T) -> bool {
    if values.contains(&value) {
        return false;
    }
    values.push(value);
    true
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
            collect_record_paths(root, &path, extension, kind_name, records)?;
        } else if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .context("Failed to relativize record-file path")?
                .to_path_buf();
            if is_local_atelier_path(&relative) {
                continue;
            }
            if relative.extension().and_then(|ext| ext.to_str()) != Some(extension) {
                bail!(
                    "Unsupported {} record file {}; expected .{} record",
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

/// Whether a path under the state directory is machine-local (runtime state,
/// identity, local config, temporary or lock files) rather than a record.
pub fn is_local_atelier_path(relative_path: &Path) -> bool {
    let local_file = relative_path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".tmp") || name.ends_with(".lock"));
    local_file
        || relative_path.starts_with("runtime")
        || relative_path == Path::new("config.toml")
        || relative_path == Path::new("agent.json")
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
    parse_datetime(&value, key, relative)
}

fn parse_datetime(value: &str, key: &str, relative: &Path) -> Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
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
    let Some(value) = optional_value(values, key) else {
        return Ok(None);
    };
    let Some(value) = value.as_str() else {
        bail!(
            "Front matter key '{}' in {} must be a string",
            key,
            display_state_path(relative)
        );
    };
    parse_datetime(value, key, relative).map(Some)
}

fn optional_value<'a>(values: &'a BTreeMap<String, Value>, key: &str) -> Option<&'a Value> {
    values.get(key).filter(|value| !value.is_null())
}

fn optional_scalar(values: &BTreeMap<String, Value>, key: &str) -> Result<Option<String>> {
    let Some(value) = optional_value(values, key) else {
        return Ok(None);
    };
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
    let Some(value) = optional_value(values, key) else {
        return Ok(None);
    };
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
    let Some(value) = optional_value(values, key) else {
        return Ok(None);
    };
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
    let values = values.get(key).and_then(Value::as_array).ok_or_else(|| {
        anyhow!(
            "Missing array front matter key '{}' in {}",
            key,
            display_state_path(relative)
        )
    })?;
    string_array_values(values, key, relative)
}

fn string_array_values(values: &[Value], key: &str, relative: &Path) -> Result<Vec<String>> {
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
    string_array_values(values, key, relative).map(Some)
}

fn optional_yaml_value<T: serde::de::DeserializeOwned>(
    values: &BTreeMap<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Option<T>> {
    let Some(value) = optional_value(values, key) else {
        return Ok(None);
    };
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

pub fn parse_issue_sections_for_type(
    body: &str,
    relative: &Path,
    issue_type: &str,
) -> Result<IssueSections> {
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
                        &if heading == "Epics" {
                            format!("Unknown issue body section '{heading}'; missions no longer have an Epics section (atelier-9vq3). Move its content into Success criteria or Notes")
                        } else if matches!(heading.as_str(), "Proof" | "Delivers") {
                            format!("Unknown issue body section '{heading}'; it belonged to the retired epic type, which is now a parent issue (atelier-9vq3). Run `atelier migrate epics` to convert epic records")
                        } else {
                            format!("Unknown issue body section '{heading}'")
                        }
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
    }
    use IssueSectionName::*;
    let required = |name| required_issue_section(&sections, name, relative, issue_id.as_deref());
    let optional = |name: IssueSectionName| sections.get(name.title()).cloned();
    let (body, allowed) = match issue_type {
        "mission" => (
            IssueBody::Mission {
                outcome: required(Outcome)?,
                success_criteria: optional(SuccessCriteria),
                background: optional(Background),
            },
            vec![Outcome, SuccessCriteria, Background],
        ),
        "decision"
            if sections.contains_key(Goal.title()) && !sections.contains_key(Question.title()) =>
        {
            bail!(
                "{} is a decision with the work-item shape (Goal, Done when); decision records use Question, Options and Decision (atelier-ox63). Convert it (Goal to Question, the decided outcome to Decision), or rename a custom decision type in the workflow policy",
                display_state_path(relative)
            );
        }
        "decision" => (
            IssueBody::Decision {
                question: required(Question)?,
                options: optional(Options),
                decision: optional(Decision),
                context: optional(Context),
                background: optional(Background),
            },
            vec![Question, Options, Decision, Context, Background],
        ),
        _ => {
            if issue_type == "epic" || sections.contains_key(Outcome.title()) {
                if !sections.contains_key(Goal.title()) {
                    bail!(
                        "{} has the retired epic shape (Outcome, Proof, Delivers); epics are now parent issues with Goal and Done when (atelier-9vq3). Run `atelier migrate epics` to convert them",
                        display_state_path(relative)
                    );
                }
            }
            (
                IssueBody::Issue {
                    goal: required(Goal)?,
                    done_when: required(DoneWhen)?,
                    scope: optional(Scope),
                    context: optional(Context),
                    unknowns: optional(Unknowns),
                    design: optional(Design),
                    background: optional(Background),
                },
                vec![Goal, DoneWhen, Scope, Context, Unknowns, Design, Background],
            )
        }
    };
    for heading in sections.keys() {
        if !allowed
            .iter()
            .chain([Notes, CloseReason].iter())
            .any(|name| name.title() == heading)
        {
            bail!(
                "section '{heading}' is not valid for {issue_type} in {}",
                display_state_path(relative)
            );
        }
    }
    Ok(IssueSections {
        body,
        notes: optional(Notes),
        close_reason: optional(CloseReason),
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

fn relationship_objects<'a>(
    values: &'a serde_json::Map<String, Value>,
    key: &'a str,
    relative: &'a Path,
) -> Result<impl Iterator<Item = Result<&'a serde_json::Map<String, Value>>> + 'a> {
    let values = values.get(key).and_then(Value::as_array).ok_or_else(|| {
        anyhow!(
            "Missing relationships.{} array in {}",
            key,
            display_state_path(relative)
        )
    })?;
    Ok(values.iter().map(move |value| {
        value.as_object().ok_or_else(|| {
            anyhow!(
                "relationships.{} in {} must contain objects",
                key,
                display_state_path(relative)
            )
        })
    }))
}

fn relationship_targets(
    values: &serde_json::Map<String, Value>,
    key: &str,
    relative: &Path,
) -> Result<Vec<RelationshipTarget>> {
    let mut targets = Vec::new();
    let mut seen = BTreeSet::new();
    for object in relationship_objects(values, key, relative)? {
        let object = object?;
        let kind = required_relationship_string(object, key, "kind", relative)?;
        validate_record_kind(&kind)?;
        let id = required_relationship_string(object, key, "id", relative)?;
        record_id::validate_record_id(&id).with_context(|| {
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
    let mut attachments = Vec::new();
    let mut seen = BTreeSet::new();
    for object in relationship_objects(values, key, relative)? {
        let object = object?;
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
    let mut relates = Vec::new();
    let mut seen = BTreeSet::new();
    for object in relationship_objects(values, key, relative)? {
        let object = object?;
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
    output.push_str(key);
    output.push_str(": ");
    match value {
        Some(value) => output.push_str(&serde_json::to_string(value)?),
        None => output.push_str("null"),
    }
    output.push('\n');
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
    output.push_str(":\n");
    let rendered = serde_yaml::to_string(value)?;
    for line in rendered.trim().lines() {
        output.push_str("  ");
        output.push_str(line);
        output.push('\n');
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

fn write_yaml_array_if_not_empty(output: &mut String, key: &str, values: &[String]) -> Result<()> {
    if !values.is_empty() {
        write_yaml_array(output, key, values)?;
    }
    Ok(())
}

fn write_evidence_target(output: &mut String, target: Option<&EvidenceTarget>) -> Result<()> {
    if let Some(target) = target {
        output.push_str("target:\n");
        for (key, value) in [
            ("kind", &target.kind),
            ("id", &target.id),
            ("role", &target.role),
        ] {
            output.push_str("  ");
            write_yaml_scalar(output, key, Some(value))?;
        }
    }
    Ok(())
}

fn render_evidence_body(body: Option<&str>, data: &EvidenceRecordData) -> String {
    let normalized = normalize_body(body.unwrap_or(""));
    if data.output.is_none() || evidence_body_has_transcript_sections(&normalized) {
        return normalized;
    }

    let summary = if normalized.is_empty() {
        &data.evidence_type
    } else {
        &normalized
    };
    render_evidence_transcript(summary, data)
        .trim_end()
        .to_string()
}

/// Format typed command/output metadata, retaining its final transcript newline.
pub fn render_evidence_transcript(summary: &str, data: &EvidenceRecordData) -> String {
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
    output
}

fn evidence_body_has_transcript_sections(body: &str) -> bool {
    body.lines()
        .any(|line| matches!(line.trim(), "## Command" | "## Stdout" | "## Stderr"))
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
        .unwrap_or(summary.len());
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
    write_relationship_entries(
        output,
        "blocks",
        relationships
            .blocks
            .iter()
            .map(|value| [("kind", value.kind.as_str()), ("id", value.id.as_str())]),
    )?;
    write_relationship_entries(
        output,
        "children",
        relationships
            .children
            .iter()
            .map(|value| [("kind", value.kind.as_str()), ("id", value.id.as_str())]),
    )?;
    write_relationship_entries(
        output,
        "attachments",
        relationships.attachments.iter().map(|value| {
            [
                ("kind", value.kind.as_str()),
                ("id", value.id.as_str()),
                ("role", value.role.as_str()),
            ]
        }),
    )?;
    write_relationship_entries(
        output,
        "relates",
        relationships.relates.iter().map(|value| {
            [
                ("kind", value.kind.as_str()),
                ("id", value.id.as_str()),
                ("type", value.relation_type.as_str()),
            ]
        }),
    )?;
    Ok(())
}

fn write_relationship_entries<'a, const N: usize>(
    output: &mut String,
    key: &str,
    entries: impl IntoIterator<Item = [(&'static str, &'a str); N]>,
) -> Result<()> {
    let mut entries = entries.into_iter().peekable();
    output.push_str("  ");
    output.push_str(key);
    if entries.peek().is_none() {
        output.push_str(": []\n");
        return Ok(());
    }
    output.push_str(":\n");
    for entry in entries {
        for (position, (key, value)) in entry.into_iter().enumerate() {
            let indent = if position == 0 { "  - " } else { "    " };
            output.push_str(&format!(
                "{indent}{key}: {}\n",
                serde_json::to_string(value)?
            ));
        }
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
        let body = "## Goal\n\nCanonical body\n\n## Done when\n\nIssue Markdown round-trips without losing fields.\n\n## Context\n\n- `cargo test record_store` passes.";
        CanonicalIssueRecord {
            issue: Issue {
                id: id.to_string(),
                title: "Write RecordStore".to_string(),
                description: None,
                status: "todo".to_string(),
                issue_type: "task".to_string(),
                priority: "high".to_string(),
                strategy: None,
                workflow_branch: None,
                parent_id: None,
                created_at: Utc.with_ymd_and_hms(2026, 6, 10, 12, 0, 0).unwrap(),
                updated_at: Utc.with_ymd_and_hms(2026, 6, 10, 13, 0, 0).unwrap(),
                closed_at: None,
            },
            labels: vec!["record-store".to_string(), "storage".to_string()],
            sections: parse_issue_sections_for_type(body, &issue_record_path(id), "task").unwrap(),
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

    fn evidence_record(id: &str) -> EvidenceRecord {
        let data = EvidenceRecordData {
            commit: "a".repeat(40),
            result: atelier_core::EvidenceResult::Pass,
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
        EvidenceRecord {
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
schema_version: 2
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
                ("issue", "atelier.issue", 2, "issues"),
                ("review", "atelier.review", 1, "reviews"),
                ("evidence", "atelier.evidence", 2, "evidence"),
            ]
        );
        for (kind, _, _, _) in canonical_contracts {
            assert!(canonical_record_kind(kind).is_ok());
            let spec = canonical_record_kind(kind).unwrap();
            assert_eq!(
                canonical_record_path(spec, "atelier-abcd").unwrap(),
                PathBuf::from(spec.canonical_dir).join("atelier-abcd.md")
            );
        }
    }

    #[test]
    fn workflow_validator_is_only_a_gate_kind_and_cannot_be_a_record_or_link_kind() {
        assert!(validate_record_kind("workflow_validator").is_err());
        let mut record = issue_record("atelier-abcd");
        record.relationships.relates.push(relates_relationship(
            "workflow_validator",
            "atelier-node",
            "related",
        ));
        assert!(render_issue_record(&record)
            .unwrap_err()
            .to_string()
            .contains("Invalid record kind 'workflow_validator'"));
        let text = render_issue_record(&issue_record("atelier-abcd"))
            .unwrap()
            .replace("kind: \"issue\"", "kind: \"workflow_validator\"");
        assert!(
            parse_issue_record(&text, &issue_record_path("atelier-abcd"))
                .unwrap_err()
                .to_string()
                .contains("Invalid record kind 'workflow_validator'")
        );
        assert!(canonical_record_kind("workflow_validator").is_err());
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
        assert!(text.contains("schema_version: 2"));
        assert!(text.contains("priority: \"P1\""));
        assert!(text.contains("relationships:\n"));
        assert!(text.contains("  blocks:\n  - kind: \"issue\"\n    id: \"atelier-bbbb\"\n"));
        assert!(text.contains("  children:\n  - kind: \"issue\"\n    id: \"atelier-aaaa\"\n"));
        assert!(text.contains(
            "  relates:\n  - kind: \"issue\"\n    id: \"atelier-cccc\"\n    type: \"related\"\n"
        ));
        assert!(!text.contains("acceptance:"));
        assert!(!text.contains("evidence_required:"));
        assert!(text.contains("## Done when\n\nIssue Markdown round-trips without losing fields."));
        assert!(text.contains("## Context\n\n- `cargo test record_store` passes."));
        assert!(!text.contains("closed_at:"));
    }

    #[test]
    fn issue_record_round_trips_dated_notes_and_close_reason() {
        let mut record = issue_record("atelier-note");
        let date = chrono::NaiveDate::from_ymd_opt(2026, 9, 30).unwrap();
        record
            .sections
            .append_note(date, "handoff", "First line\n## not a heading");
        record.sections.append_note(date, "note", "Second note");
        record.sections.close_reason = Some("Shipped with proof".to_string());

        let text = render_issue_record(&record).unwrap();
        let parsed = parse_issue_record(&text, &issue_record_path("atelier-note")).unwrap();

        assert_eq!(parsed, record);
        assert!(text.contains(
            "## Notes\n\n- 2026-09-30 [handoff] First line\n  ## not a heading\n- 2026-09-30 [note] Second note"
        ));
        assert!(text.contains("## Close Reason\n\nShipped with proof"));
        assert_eq!(
            parsed.sections.section(IssueSectionName::CloseReason),
            Some("Shipped with proof")
        );
    }

    #[test]
    fn issue_record_write_rejects_section_text_that_opens_another_section() {
        let dir = tempdir().unwrap();
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
        let mut record = issue_record("atelier-smug");
        record.sections.close_reason = Some("done\n## Context\nsmuggled".to_string());

        let error = store.write_issue_atomic(&record).unwrap_err().to_string();

        assert!(error.contains("atelier-smug"), "{error}");
        assert!(error.contains("'Close Reason'"), "{error}");
        assert!(!store
            .state_dir
            .join(issue_record_path("atelier-smug"))
            .exists());

        record.sections.close_reason = Some("done\r## Notes".to_string());
        assert!(store.write_issue_atomic(&record).is_err());

        record.sections.close_reason = Some("done\n### Detail is fine".to_string());
        store.write_issue_atomic(&record).unwrap();
    }

    #[test]
    fn record_store_concrete_services_round_trip_and_discover_domain_files() {
        let dir = tempdir().unwrap();
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
        let issue = issue_record("atelier-iss1");
        let evidence = evidence_record("atelier-evd1");

        store.write_issue_atomic(&issue).unwrap();
        store.write_evidence_atomic(&evidence).unwrap();

        assert_eq!(store.load_issue_by_id("atelier-iss1").unwrap(), issue);
        assert_eq!(store.load_evidence_by_id("atelier-evd1").unwrap(), evidence);
        assert_eq!(
            store.discover_record_paths("issue").unwrap(),
            vec![PathBuf::from("issues/atelier-iss1.md")]
        );
        assert_eq!(
            store.discover_record_paths("evidence").unwrap(),
            vec![PathBuf::from("evidence/atelier-evd1.md")]
        );
    }

    #[test]
    fn generic_issue_relationship_mutation_preserves_typed_fields_and_is_idempotent() {
        let dir = tempdir().unwrap();
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
        let mut issue = issue_record("atelier-miss");
        issue.issue.issue_type = "mission".into();
        issue.issue.status = "draft".into();
        issue.issue.strategy = Some(IssueStrategy::new("docs/strategies/mission.md").unwrap());
        issue.issue.workflow_branch = Some(WorkflowBranch {
            work_branch: "mission/example".into(),
            branch_base: "master".into(),
        });
        issue.labels = vec!["kept".into()];
        issue.sections = IssueSections::for_type("mission", Some("Keep this outcome."));
        issue.sections.append_note(
            chrono::NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
            "progress",
            "Keep this note.",
        );
        store.write_issue_atomic(&issue).unwrap();
        assert!(store
            .add_record_relationship(
                "issue",
                "atelier-miss",
                "evidence",
                "atelier-proof",
                "validates"
            )
            .unwrap());
        let changed = store.load_issue_by_id("atelier-miss").unwrap();
        assert_eq!(changed.issue.strategy, issue.issue.strategy);
        assert_eq!(changed.issue.workflow_branch, issue.issue.workflow_branch);
        assert_eq!(changed.issue.issue_type, issue.issue.issue_type);
        assert_eq!(changed.sections, issue.sections);
        assert_eq!(changed.labels, issue.labels);
        assert_eq!(
            changed.relationships.attachments,
            vec![attachment_relationship(
                "evidence",
                "atelier-proof",
                "validates"
            )]
        );
        let once = fs::read(store.state_dir.join(issue_record_path("atelier-miss"))).unwrap();
        assert!(!store
            .add_record_relationship(
                "issue",
                "atelier-miss",
                "evidence",
                "atelier-proof",
                "validates"
            )
            .unwrap());
        assert_eq!(
            fs::read(store.state_dir.join(issue_record_path("atelier-miss"))).unwrap(),
            once
        );
    }

    #[test]
    fn mission_record_round_trips_typed_strategy_reference() {
        let mut record = issue_record("atelier-strg");
        record.issue.issue_type = "mission".to_string();
        record.sections = IssueSections::for_type("mission", Some("Mission outcome."));
        record.issue.status = "draft".to_string();
        record.issue.strategy =
            Some(IssueStrategy::new("docs/strategies/adaptive-mission.md").unwrap());

        let text = render_issue_record(&record).unwrap();
        let parsed = parse_issue_record(&text, &issue_record_path("atelier-strg")).unwrap();

        assert!(text.contains("strategy:\n  path: docs/strategies/adaptive-mission.md\n"));
        assert!(!text.contains("fields:\n"));
        assert_eq!(
            parsed.issue.strategy().unwrap(),
            record.issue.strategy().unwrap()
        );
        assert_eq!(render_issue_record(&parsed).unwrap(), text);
    }

    #[test]
    fn root_exports_expose_supported_record_boundaries() {
        let record = CanonicalIssueRecord {
            ..issue_record("atelier-mods")
        };
        let text = render_issue_record(&record).unwrap();
        assert!(split_document(&text).is_some());
        assert_eq!(IssueSectionName::Context.title(), "Context");
        let _store = RecordStore::for_project(tempdir().unwrap().path(), "atelier");
        validate_priority("high").unwrap();
        let _: Option<EvidenceOutputSummary> = None;
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
        let store = RecordStore::for_project(dir.path(), "atelier");
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
        let store = RecordStore::for_project(dir.path(), "atelier");
        let mut issue = issue_record("atelier-iss1");
        issue.relationships = Relationships::default();
        let mut evidence = evidence_record("atelier-evd1");
        evidence.header.relationships = Relationships::default();
        store.write_issue_atomic(&issue).unwrap();
        store.write_evidence_atomic(&evidence).unwrap();

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
    fn relationship_mutation_preserves_review_noops_and_append_only_refusal() {
        let dir = tempdir().unwrap();
        let store = RecordStore::for_project(dir.path(), "atelier");
        let record = store
            .create_review(
                "atelier-owner",
                "Reviewer",
                &"a".repeat(40),
                atelier_core::ReviewEvent::Review,
                "Independent review",
            )
            .unwrap();
        let path = dir
            .path()
            .join(canonical_record_path(&REVIEW_KIND, &record.header.id).unwrap());
        let original = fs::read(&path).unwrap();
        assert!(!store
            .add_attachment_relationship(
                "review",
                &record.header.id,
                "issue",
                "atelier-owner",
                "reviews",
            )
            .unwrap());
        assert_eq!(fs::read(&path).unwrap(), original);
        let error = store
            .add_attachment_relationship(
                "review",
                &record.header.id,
                "issue",
                "atelier-owner",
                "custom",
            )
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "review records are append-only; create a successor event"
        );
        assert_eq!(fs::read(&path).unwrap(), original);

        fs::write(&path, "invalid review").unwrap();
        let error = store
            .add_attachment_relationship(
                "review",
                &record.header.id,
                "issue",
                "atelier-owner",
                "reviews",
            )
            .unwrap_err();
        assert_eq!(error.to_string(), "review needs YAML front matter");
    }

    #[test]
    fn relationship_mutation_keeps_source_id_kind_and_read_error_priority() {
        let dir = tempdir().unwrap();
        let store = RecordStore::for_project(dir.path(), "atelier");
        let error = store
            .add_attachment_relationship("unknown", "../bad", "issue", "atelier-owner", "validates")
            .unwrap_err();
        assert_eq!(error.to_string(), "expected <project-slug>-<random-base36>");
        let error = store
            .add_attachment_relationship(
                "unknown",
                "atelier-ok",
                "issue",
                "atelier-owner",
                "validates",
            )
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "Record kind 'unknown' is not a first-class record-file type"
        );
        let error = store
            .add_attachment_relationship(
                "evidence",
                "atelier-ok",
                "issue",
                "atelier-owner",
                "validates",
            )
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "Missing record file .atelier/evidence/atelier-ok.md"
        );
        assert!(!dir.path().join("evidence").exists());
    }

    #[test]
    fn typed_evidence_rendering_checks_kind_and_id_before_data() {
        let mut record = evidence_record("atelier-proof");
        record.header.kind = "issue".into();
        record.header.id = "../bad".into();
        record.data.commit = "short".into();
        assert_eq!(
            render_evidence_record_file(&record)
                .unwrap_err()
                .to_string(),
            "Evidence record kind must be evidence"
        );
        record.header.kind = "evidence".into();
        assert_eq!(
            render_evidence_record_file(&record)
                .unwrap_err()
                .to_string(),
            "Invalid evidence id '../bad' in .atelier/<record>"
        );
        record.header.id = "atelier-proof".into();
        assert_eq!(
            render_evidence_record_file(&record)
                .unwrap_err()
                .to_string(),
            "evidence commit must be a full Git object ID in .atelier/<record>"
        );
    }

    #[test]
    fn evidence_record_renders_and_parses_deterministically_without_data_blob() {
        let record = evidence_record("atelier-evdn");
        let spec = canonical_record_kind("evidence").unwrap();
        let path = canonical_record_path(spec, "atelier-evdn").unwrap();
        let text = render_evidence_record_file(&record).unwrap();
        let parsed = parse_evidence_record_file(&text, &path).unwrap();

        assert_eq!(parsed, record);
        assert_eq!(render_evidence_record_file(&parsed).unwrap(), text);
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
schema_version: 2
status: "recorded"
title: "Legacy evidence proof"
updated_at: "2026-06-10T13:00:00+00:00"
---

Legacy evidence summary.
"#;
        let error = parse_evidence_record_file(text, &path).unwrap_err();
        assert!(error.to_string().contains("Forbidden data front matter"));
    }

    #[test]
    fn evidence_transcript_keeps_capture_and_codec_whitespace_boundaries() {
        let mut record = evidence_record("atelier-evdn");
        record.data.command = Some("printf 'héllo' \t".into());
        record.data.exit_status = Some("0".into());
        record.data.spawn_error = Some("fixture error".into());
        record.data.residual_risks = Vec::new();
        record.data.output = Some(EvidenceOutputSummary {
            limit_bytes_per_stream: 4096,
            stdout: atelier_core::EvidenceStreamSummary {
                bytes: 11,
                truncated: true,
                summary: "héllo\n \t".into(),
            },
            stderr: empty_evidence_stream(),
        });
        let summary = " \r\n## Command\n\n```text\nraw\n```\n ";
        let transcript = render_evidence_transcript(summary, &record.data);
        let expected = format!(
            "## Summary\n\n{summary}\n\n## Command\n\n```console\nprintf 'héllo'\n```\n\nExit status: 0\nSpawn error: fixture error\n\n## Stdout\n\nBytes: 11\nTruncated: yes\n\n```text\nhéllo\n```\n\n## Stderr\n\nBytes: 0\nTruncated: no\n\n```text\n```\n"
        );
        assert_eq!(transcript, expected);
        assert_eq!(
            render_evidence_body(Some(&transcript), &record.data),
            normalize_body(&expected)
        );
        assert_eq!(
            render_evidence_body(Some("  plain\r\nsummary  "), &record.data),
            render_evidence_transcript("  plain\nsummary  ", &record.data).trim_end()
        );
        assert_eq!(
            render_evidence_body(Some(""), &record.data),
            render_evidence_transcript(&record.data.evidence_type, &record.data).trim_end()
        );
        record.summary = transcript;
        let rendered = render_evidence_record_file(&record).unwrap();
        assert!(rendered.ends_with("```\n\n"));
        record.data.output = None;
        assert_eq!(
            render_evidence_body(Some(" raw\r\nbody \r"), &record.data),
            " raw\nbody \n"
        );
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
            .contains("does not match record-file path .atelier/issues/atelier-abcd.md"));
    }

    #[test]
    fn issue_parser_contract_accepts_sectioned_body_without_legacy_arrays() {
        let body = "## Goal\n\nCanonical problem statement.\n\n## Done when\n\nThe desired finished world is observable.\n\n## Context\n\n- `atelier check atelier-abcd` passes.\n\n## Notes\n\nSequencing context.";
        let text = sectioned_issue_text("atelier-abcd", body);
        let parsed = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap();

        assert_eq!(parsed.issue.description, None);
        assert_eq!(
            parsed.sections.section(IssueSectionName::Goal).unwrap(),
            "Canonical problem statement.".to_string()
        );
        assert_eq!(
            parsed.sections.section(IssueSectionName::DoneWhen).unwrap(),
            "The desired finished world is observable.".to_string()
        );
        assert_eq!(
            parsed.sections.section(IssueSectionName::Context).unwrap(),
            "- `atelier check atelier-abcd` passes.".to_string()
        );
        assert_eq!(
            parsed.sections.notes.as_deref(),
            Some("Sequencing context.")
        );
    }

    #[test]
    fn issue_sections_report_shared_presence_state_and_search_text() {
        let body = "## Goal\n\nCanonical problem statement.\n\n## Done when\n\nThe desired finished world is observable.\n\n## Context\n\n- `atelier check atelier-abcd` passes.";
        let sections =
            parse_issue_sections_for_type(body, &issue_record_path("atelier-abcd"), "task")
                .unwrap();
        let states = sections.section_states();

        assert_eq!(
            states
                .iter()
                .filter(|state| state.required && state.present && !state.empty)
                .count(),
            2
        );
        assert!(states.iter().any(|state| {
            state.name == IssueSectionName::Context
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
            .section(IssueSectionName::DoneWhen)
            .unwrap()
            .contains("The desired finished world is observable."));
    }

    #[test]
    fn issue_parser_contract_rejects_legacy_acceptance_and_evidence_front_matter() {
        let body = "## Goal\n\nCanonical problem statement.\n\n## Done when\n\nThe desired finished world is observable.\n\n## Context\n\n- `atelier check atelier-abcd` passes.";
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
        let body = "## Goal\n\nCanonical problem statement.\n\n## Done when\n\nThe desired finished world is observable.\n\n## Context\n\n- `atelier check atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body)
            .replace("priority: \"P1\"", "priority: \"high\"");

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error.to_string().contains("Invalid priority"));
    }

    #[test]
    fn issue_parser_contract_accepts_missing_evidence_section() {
        let body = "## Goal\n\nCanonical problem statement.\n\n## Done when\n\nThe desired finished world is observable.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let parsed = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap();

        assert_eq!(
            parsed.sections.section(IssueSectionName::DoneWhen).unwrap(),
            "The desired finished world is observable."
        );
        assert_eq!(parsed.sections.section(IssueSectionName::Context), None);
        assert!(!render_issue_record(&parsed).unwrap().contains("## Context"));
    }

    #[test]
    fn issue_parser_contract_rejects_missing_required_sections() {
        let body = "## Goal\n\nCanonical problem statement.\n\n## Context\n\n- `atelier check atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Missing required issue body section 'Done when'"));
    }

    #[test]
    fn issue_parser_contract_rejects_duplicate_recognized_headings() {
        let body = "## Goal\n\nCanonical problem statement.\n\n## Done when\n\nThe desired finished world is observable.\n\n## Context\n\n- `atelier check atelier-abcd` passes.\n\n## Done when\n\nA second outcome.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Duplicate issue body section 'Done when'"));
    }

    #[test]
    fn issue_parser_contract_rejects_content_before_first_recognized_heading() {
        let body = "Preamble is not part of the issue contract.\n\n## Goal\n\nCanonical problem statement.\n\n## Done when\n\nThe desired finished world is observable.\n\n## Context\n\n- `atelier check atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Content before first recognized issue body section"));
    }

    #[test]
    fn issue_parser_contract_rejects_unknown_top_level_sections() {
        let body = "## Goal\n\nCanonical problem statement.\n\n## Done when\n\nThe desired finished world is observable.\n\n## Acceptance\n\nLegacy section name.\n\n## Context\n\n- `atelier check atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Unknown issue body section 'Acceptance'"));
    }

    #[test]
    fn issue_parser_contract_rejects_empty_present_sections() {
        let body = "## Goal\n\nCanonical problem statement.\n\n## Done when\n\n\n## Context\n\n- `atelier check atelier-abcd` passes.";
        let text = sectioned_issue_text("atelier-abcd", body);

        let error = parse_issue_record(&text, &issue_record_path("atelier-abcd")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Empty issue body section 'Done when'"));
    }

    #[test]
    fn record_store_discovers_and_rejects_noncanonical_issue_paths() {
        let dir = tempdir().unwrap();
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
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
            .contains("does not match record-file path .atelier/issues/atelier-abcd.md"));
    }

    #[test]
    fn record_store_allocates_ids_across_canonical_dirs() {
        let dir = tempdir().unwrap();
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
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
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
        let mut record = issue_record("atelier-abcd");
        record.labels.clear();
        store.write_issue_atomic(&record).unwrap();

        record.labels.push("graph".into());
        store.write_issue_atomic(&record).unwrap();
        assert_eq!(
            store.load_issue_by_id("atelier-abcd").unwrap().labels,
            vec!["graph".to_string()]
        );
        record.labels.clear();
        store.write_issue_atomic(&record).unwrap();
        assert!(store
            .load_issue_by_id("atelier-abcd")
            .unwrap()
            .labels
            .is_empty());
    }

    #[test]
    fn record_store_block_unblock_mutates_blocker_relationships() {
        let dir = tempdir().unwrap();
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
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
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
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
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
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
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
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
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
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
        let store = RecordStore::for_project(dir.path().join(".atelier"), "atelier");
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
        assert!(canonical_record_kind("session").is_err());
    }
    #[test]
    fn version_two_rejects_old_versions_generic_fields_and_strategy_revisions() {
        let record = issue_record("atelier-vers");
        let text = render_issue_record(&record).unwrap();
        let path = issue_record_path("atelier-vers");
        let old = parse_issue_record(
            &text.replace("schema_version: 2", "schema_version: 1"),
            &path,
        )
        .unwrap_err();
        assert!(old.to_string().contains("schema_version"), "{old}");
        let fields = parse_issue_record(&text.replace("priority:", "fields: {}\npriority:"), &path)
            .unwrap_err();
        assert!(
            fields
                .to_string()
                .contains("generic issue fields are obsolete"),
            "{fields}"
        );
        let strategy = parse_issue_record(
            &text.replace(
                "priority:",
                "strategy: {path: docs/strategy.md, revision: 2}\npriority:",
            ),
            &path,
        )
        .unwrap_err();
        assert!(format!("{strategy:#}").contains("revision"), "{strategy:#}");
        let evidence = render_evidence_record_file(&evidence_record("atelier-evers")).unwrap();
        let path = Path::new("evidence/atelier-evers.md");
        assert!(parse_evidence_record_file(
            &evidence.replace("schema_version: 2", "schema_version: 1"),
            path
        )
        .is_err());
        assert!(
            parse_evidence_record_file(&evidence.replace("result: \"pass\"\n", ""), path).is_err()
        );
        assert!(
            parse_evidence_record_file(&evidence.replace(&"a".repeat(40), "abcd"), path).is_err()
        );
    }

    #[test]
    fn work_and_mission_bodies_round_trip_their_own_shapes() {
        for (kind, body) in [
            ("task", "## Goal\n\nBounded change.\n\n## Done when\n\nFocused test passes.\n\n## Scope\n\nsrc/lib.rs\n\n## Context\n\nADR 0019."),
            ("feature", "## Goal\n\nObservable increment.\n\n## Done when\n\nEvery child is done.\n\n## Unknowns\n\nOne design question.\n\n## Design\n\nADR 0021."),
            ("mission", "## Outcome\n\nMission outcome.\n\n## Success criteria\n\nS1: scenario passes."),
        ] {
            let mut record = issue_record("atelier-body");
            record.issue.issue_type = kind.to_string();
            record.sections = parse_issue_sections_for_type(body, Path::new("<body>"), kind).unwrap();
            let text = render_issue_record(&record).unwrap();
            let parsed = parse_issue_record(&text, &issue_record_path("atelier-body")).unwrap();
            assert_eq!(parsed.sections, record.sections);
            let other_kind = if kind == "mission" { "task" } else { "mission" };
            assert!(parse_issue_sections_for_type(body, Path::new("<body>"), other_kind).is_err());
        }
        let custom = parse_issue_sections_for_type(
            "## Goal\n\nDecide.\n\n## Done when\n\nDecided.",
            Path::new("<body>"),
            "decision",
        )
        .unwrap_err()
        .to_string();
        assert!(custom.contains("decision records use Question"), "{custom}");
        let retired = "## Outcome\n\nOld epic.\n\n## Proof\n\nA scenario.";
        for kind in ["epic", "task"] {
            let error = parse_issue_sections_for_type(retired, Path::new("<body>"), kind)
                .unwrap_err()
                .to_string();
            assert!(error.contains("atelier migrate epics"), "{error}");
        }
    }
}
