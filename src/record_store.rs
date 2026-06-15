use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::{
    DomainRecord, EvidenceOutputSummary, EvidenceRecordData, Issue, MilestoneRecordData,
    PlanRecordData,
};
use crate::record_id;

mod record_kinds;
mod relationships;

pub use record_kinds::{
    canonical_record_dirs, canonical_record_kind, canonical_record_path, issue_record_path,
    validate_canonical_record_kind, validate_record_kind, RecordKindSpec, FIRST_CLASS_RECORD_KINDS,
    ISSUE_KIND,
};
pub use relationships::{
    issue_relates_relationship, issue_relationship_target, sort_relationships,
    AttachmentRelationship, RelatesRelationship, RelationshipTarget, Relationships,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalIssueRecord {
    pub issue: Issue,
    pub labels: Vec<String>,
    pub sections: IssueSections,
    pub relationships: Relationships,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IssueSections {
    pub description: String,
    pub outcome: String,
    pub evidence: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IssueSectionName {
    Description,
    Outcome,
    Evidence,
    Notes,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IssueSectionState {
    pub name: IssueSectionName,
    pub required: bool,
    pub present: bool,
    pub empty: bool,
}

impl IssueSections {
    pub const REQUIRED_NAMES: [IssueSectionName; 3] = [
        IssueSectionName::Description,
        IssueSectionName::Outcome,
        IssueSectionName::Evidence,
    ];

    pub const ALL_NAMES: [IssueSectionName; 4] = [
        IssueSectionName::Description,
        IssueSectionName::Outcome,
        IssueSectionName::Evidence,
        IssueSectionName::Notes,
    ];

    pub fn unchecked_from_body(body: Option<&str>) -> Self {
        if let Some(body) = body {
            if let Ok(sections) = parse_issue_sections(body, Path::new("<input>")) {
                return sections;
            }
        }

        let description = body
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("No description provided.");
        Self {
            description: description.to_string(),
            outcome: "Outcome was not specified.".to_string(),
            evidence: "Evidence was not specified.".to_string(),
            notes: None,
        }
    }

    pub fn section(&self, name: IssueSectionName) -> Option<&str> {
        match name {
            IssueSectionName::Description => Some(&self.description),
            IssueSectionName::Outcome => Some(&self.outcome),
            IssueSectionName::Evidence => Some(&self.evidence),
            IssueSectionName::Notes => self.notes.as_deref(),
        }
    }

    pub fn section_states(&self) -> Vec<IssueSectionState> {
        Self::ALL_NAMES
            .into_iter()
            .map(|name| {
                let value = self.section(name);
                IssueSectionState {
                    name,
                    required: name.required(),
                    present: value.is_some(),
                    empty: value.map(str::trim).is_none_or(str::is_empty),
                }
            })
            .collect()
    }

    pub fn searchable_text(&self) -> String {
        Self::ALL_NAMES
            .into_iter()
            .filter_map(|name| self.section(name))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl IssueSectionName {
    pub fn title(self) -> &'static str {
        match self {
            IssueSectionName::Description => "Description",
            IssueSectionName::Outcome => "Outcome",
            IssueSectionName::Evidence => "Evidence",
            IssueSectionName::Notes => "Notes",
        }
    }

    pub fn required(self) -> bool {
        matches!(
            self,
            IssueSectionName::Description | IssueSectionName::Outcome | IssueSectionName::Evidence
        )
    }
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

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalDomainRecord {
    pub record: DomainRecord,
    pub labels: Vec<String>,
    pub relationships: Relationships,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MissionSections {
    pub intent: String,
    pub constraints: String,
    pub risks: String,
    pub validation: String,
    pub closeout_notes: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MissionSectionName {
    Intent,
    Constraints,
    Risks,
    Validation,
    CloseoutNotes,
    Notes,
}

impl MissionSections {
    pub const ALL_NAMES: [MissionSectionName; 6] = [
        MissionSectionName::Intent,
        MissionSectionName::Constraints,
        MissionSectionName::Risks,
        MissionSectionName::Validation,
        MissionSectionName::CloseoutNotes,
        MissionSectionName::Notes,
    ];
}

impl MissionSectionName {
    pub fn title(self) -> &'static str {
        match self {
            MissionSectionName::Intent => "Intent",
            MissionSectionName::Constraints => "Constraints",
            MissionSectionName::Risks => "Risks",
            MissionSectionName::Validation => "Validation",
            MissionSectionName::CloseoutNotes => "Closeout Notes",
            MissionSectionName::Notes => "Notes",
        }
    }
}

pub const MISSION_EMPTY_DATA_JSON: &str = "{}";

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
                body: normalized_domain_body(kind, title, body, data_json)?,
                data_json: normalized_domain_data_json(kind, data_json)?,
                created_at: now,
                updated_at: now,
            },
            labels: default_domain_labels(kind),
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

fn render_issue_sections(sections: &IssueSections) -> String {
    let mut body = format!(
        "## Description\n\n{}\n\n## Outcome\n\n{}\n\n## Evidence\n\n{}",
        sections.description.trim(),
        sections.outcome.trim(),
        sections.evidence.trim()
    );
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

pub fn render_domain_record(record: &CanonicalDomainRecord) -> Result<String> {
    let spec = canonical_record_kind(&record.record.kind)?;
    let mut record = record.clone();
    if spec.kind == "mission" {
        record.relationships = normalize_legacy_mission_relationships(record.relationships);
    } else if spec.kind == "evidence" {
        record.relationships = normalize_legacy_evidence_relationships(record.relationships);
    }
    validate_domain_record(&record, Path::new("<record>"), spec)?;
    let mut relationships = record.relationships.clone();
    sort_relationships(&mut relationships);

    if spec.kind == "mission" {
        return render_mission_record(&record, &relationships, spec);
    }
    match spec.kind {
        "evidence" => return render_evidence_record(&record, &relationships, spec),
        "milestone" => return render_milestone_record(&record, &relationships, spec),
        "plan" => return render_plan_record(&record, &relationships, spec),
        _ => {}
    }

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
    reject_legacy_issue_keys(&front_matter, relative)?;
    let relationships = parse_relationships(&front_matter, relative)?;
    let status = require_scalar(&front_matter, "status", relative)?;
    crate::db::validate_status(&status)
        .with_context(|| format!("Invalid status in {}", display_state_path(relative)))?;
    let issue_type = require_scalar(&front_matter, "issue_type", relative)?;
    crate::db::validate_issue_type(&issue_type)
        .with_context(|| format!("Invalid issue_type in {}", display_state_path(relative)))?;
    let updated_at = require_datetime(&front_matter, "updated_at", relative)?;
    let closed_at = optional_datetime(&front_matter, "closed_at", relative)?;
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
    match spec.kind {
        "mission" => return parse_mission_domain_record(front_matter, body, relative, spec, id),
        "evidence" => return parse_evidence_domain_record(front_matter, body, relative, spec, id),
        "milestone" => {
            return parse_milestone_domain_record(front_matter, body, relative, spec, id)
        }
        "plan" => return parse_plan_domain_record(front_matter, body, relative, spec, id),
        _ => {}
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
        labels: optional_string_array(&front_matter, "labels", relative)?.unwrap_or_default(),
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
    if spec.kind == "mission" {
        validate_mission_record(record, relative)?;
        return Ok(());
    }
    match spec.kind {
        "evidence" => {
            validate_evidence_record(record, relative)?;
            return Ok(());
        }
        "milestone" => {
            validate_milestone_record(record, relative)?;
            return Ok(());
        }
        "plan" => {
            validate_plan_record(record, relative)?;
            return Ok(());
        }
        _ => {}
    }
    let _: Value = serde_json::from_str(&record.record.data_json)
        .with_context(|| format!("Invalid data JSON in {}", display_state_path(relative)))?;
    validate_relationships(&record.relationships, relative)?;
    Ok(())
}

fn render_mission_record(
    record: &CanonicalDomainRecord,
    relationships: &Relationships,
    spec: &RecordKindSpec,
) -> Result<String> {
    let sections = mission_sections_from_domain_record(&record.record)?;
    let mut labels = record.labels.clone();
    labels.sort();
    labels.dedup();
    if labels.is_empty() {
        labels = default_domain_labels("mission");
    }

    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.record.created_at.to_rfc3339()),
    )?;
    write_yaml_scalar(&mut output, "id", Some(&record.record.id))?;
    write_yaml_array(&mut output, "labels", &labels)?;
    write_yaml_relationships(&mut output, relationships)?;
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
    output.push_str(&render_mission_sections(&sections));
    output.push('\n');
    Ok(output)
}

fn parse_mission_domain_record(
    front_matter: BTreeMap<String, Value>,
    body: &str,
    relative: &Path,
    spec: &RecordKindSpec,
    id: String,
) -> Result<CanonicalDomainRecord> {
    let relationships = parse_relationships(&front_matter, relative)?;
    let title = require_scalar(&front_matter, "title", relative)?;
    let status = require_scalar(&front_matter, "status", relative)?;
    validate_mission_status(&status, relative)?;
    let created_at = require_datetime(&front_matter, "created_at", relative)?;
    let updated_at = require_datetime(&front_matter, "updated_at", relative)?;

    if front_matter.contains_key("data") {
        let data_json = require_scalar(&front_matter, "data", relative)?;
        let sections = legacy_mission_sections(Some(body), &data_json, &title, relative)?;
        let relationships = normalize_legacy_mission_relationships(relationships);
        return Ok(CanonicalDomainRecord {
            record: DomainRecord {
                id,
                kind: spec.kind.to_string(),
                title,
                status,
                body: Some(render_mission_sections(&sections)),
                data_json: MISSION_EMPTY_DATA_JSON.to_string(),
                created_at,
                updated_at,
            },
            labels: optional_string_array(&front_matter, "labels", relative)?
                .filter(|labels| !labels.is_empty())
                .unwrap_or_else(|| default_domain_labels("mission")),
            relationships,
        });
    }

    reject_unexpected_mission_front_matter(&front_matter, relative)?;
    let labels = string_array(&front_matter, "labels", relative)?;
    validate_sorted_unique("labels", &labels, relative)?;
    let sections = parse_mission_sections(body, relative)?;
    validate_mission_relationships(&relationships, relative)?;

    Ok(CanonicalDomainRecord {
        record: DomainRecord {
            id,
            kind: spec.kind.to_string(),
            title,
            status,
            body: Some(render_mission_sections(&sections)),
            data_json: MISSION_EMPTY_DATA_JSON.to_string(),
            created_at,
            updated_at,
        },
        labels,
        relationships,
    })
}

fn validate_mission_record(record: &CanonicalDomainRecord, relative: &Path) -> Result<()> {
    validate_mission_status(&record.record.status, relative)?;
    let labels = if record.labels.is_empty() {
        default_domain_labels("mission")
    } else {
        record.labels.clone()
    };
    validate_sorted_unique("labels", &labels, relative)?;
    let data: Value = serde_json::from_str(&record.record.data_json)
        .with_context(|| format!("Invalid data JSON in {}", display_state_path(relative)))?;
    if data != Value::Object(Default::default()) {
        bail!(
            "Mission record {} must not serialize mission semantics as data JSON",
            display_state_path(relative)
        );
    }
    mission_sections_from_domain_record(&record.record)?;
    validate_relationships(&record.relationships, relative)?;
    validate_mission_relationships(&record.relationships, relative)?;
    Ok(())
}

fn render_evidence_record(
    record: &CanonicalDomainRecord,
    relationships: &Relationships,
    spec: &RecordKindSpec,
) -> Result<String> {
    let data = normalized_evidence_data(&record.record.data_json)?;

    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.record.created_at.to_rfc3339()),
    )?;
    write_yaml_scalar(&mut output, "id", Some(&record.record.id))?;
    write_yaml_scalar(&mut output, "evidence_type", Some(&data.evidence_type))?;
    write_yaml_scalar(
        &mut output,
        "captured_at",
        Some(&data.captured_at.to_rfc3339()),
    )?;
    write_yaml_scalar(&mut output, "command", data.command.as_deref())?;
    write_yaml_scalar(&mut output, "exit_status", data.exit_status.as_deref())?;
    write_yaml_scalar(&mut output, "path", data.path.as_deref())?;
    write_yaml_scalar(&mut output, "uri", data.uri.as_deref())?;
    write_yaml_scalar(&mut output, "proof_scope", data.proof_scope.as_deref())?;
    write_yaml_scalar(
        &mut output,
        "agent_identity",
        data.agent_identity.as_deref().or(data.producer.as_deref()),
    )?;
    write_yaml_scalar(
        &mut output,
        "independence_level",
        data.independence_level.as_deref(),
    )?;
    write_yaml_array(&mut output, "follow_up_ids", &data.follow_up_ids)?;
    write_yaml_array(&mut output, "residual_risks", &data.residual_risks)?;
    write_evidence_output_summary(&mut output, data.output.as_ref())?;
    write_yaml_relationships(&mut output, relationships)?;
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

fn parse_evidence_domain_record(
    front_matter: BTreeMap<String, Value>,
    body: &str,
    relative: &Path,
    spec: &RecordKindSpec,
    id: String,
) -> Result<CanonicalDomainRecord> {
    let relationships =
        normalize_legacy_evidence_relationships(parse_relationships(&front_matter, relative)?);
    let title = require_scalar(&front_matter, "title", relative)?;
    let status = require_scalar(&front_matter, "status", relative)?;
    let created_at = require_datetime(&front_matter, "created_at", relative)?;
    let updated_at = require_datetime(&front_matter, "updated_at", relative)?;

    let data_json = if front_matter.contains_key("data") {
        let data_json = require_scalar(&front_matter, "data", relative)?;
        serde_json::to_string(&normalized_evidence_data(&data_json)?)?
    } else {
        let data = EvidenceRecordData {
            evidence_type: require_scalar(&front_matter, "evidence_type", relative)?,
            captured_at: require_datetime(&front_matter, "captured_at", relative)?,
            command: optional_scalar(&front_matter, "command")?,
            path: optional_scalar(&front_matter, "path")?,
            uri: optional_scalar(&front_matter, "uri")?,
            producer: None,
            proof_scope: optional_scalar(&front_matter, "proof_scope")?,
            agent_identity: optional_scalar(&front_matter, "agent_identity")?,
            independence_level: optional_scalar(&front_matter, "independence_level")?,
            residual_risks: optional_string_array(&front_matter, "residual_risks", relative)?
                .unwrap_or_default(),
            follow_up_ids: optional_string_array(&front_matter, "follow_up_ids", relative)?
                .unwrap_or_default(),
            exit_code: optional_i32(&front_matter, "exit_code", relative)?,
            exit_status: optional_scalar(&front_matter, "exit_status")?,
            success: optional_bool(&front_matter, "success", relative)?,
            spawn_error: optional_scalar(&front_matter, "spawn_error")?,
            output: optional_yaml_value::<EvidenceOutputSummary>(
                &front_matter,
                "output",
                relative,
            )?,
            target: None,
        };
        serde_json::to_string(&data)?
    };
    let body = if body.is_empty() {
        None
    } else {
        Some(body.to_string())
    };

    Ok(CanonicalDomainRecord {
        record: DomainRecord {
            id,
            kind: spec.kind.to_string(),
            title,
            status,
            body,
            data_json,
            created_at,
            updated_at,
        },
        labels: optional_string_array(&front_matter, "labels", relative)?.unwrap_or_default(),
        relationships,
    })
}

fn validate_evidence_record(record: &CanonicalDomainRecord, relative: &Path) -> Result<()> {
    let data = normalized_evidence_data(&record.record.data_json)
        .with_context(|| format!("Invalid data JSON in {}", display_state_path(relative)))?;
    if data.evidence_type.trim().is_empty() {
        bail!(
            "Evidence record {} must include evidence_type",
            display_state_path(relative)
        );
    }
    validate_relationships(&record.relationships, relative)?;
    Ok(())
}

fn render_milestone_record(
    record: &CanonicalDomainRecord,
    relationships: &Relationships,
    spec: &RecordKindSpec,
) -> Result<String> {
    let data = normalized_milestone_data(&record.record.data_json)?;

    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.record.created_at.to_rfc3339()),
    )?;
    write_yaml_scalar(&mut output, "id", Some(&record.record.id))?;
    write_yaml_scalar(&mut output, "desired_state", Some(&data.desired_state))?;
    write_yaml_array(&mut output, "scope", &data.scope)?;
    write_yaml_array(
        &mut output,
        "validation_criteria",
        &data.validation_criteria,
    )?;
    write_yaml_relationships(&mut output, relationships)?;
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

fn parse_milestone_domain_record(
    front_matter: BTreeMap<String, Value>,
    body: &str,
    relative: &Path,
    spec: &RecordKindSpec,
    id: String,
) -> Result<CanonicalDomainRecord> {
    let relationships = parse_relationships(&front_matter, relative)?;
    let title = require_scalar(&front_matter, "title", relative)?;
    let status = require_scalar(&front_matter, "status", relative)?;
    let created_at = require_datetime(&front_matter, "created_at", relative)?;
    let updated_at = require_datetime(&front_matter, "updated_at", relative)?;
    let data = if front_matter.contains_key("data") {
        normalized_milestone_data(&require_scalar(&front_matter, "data", relative)?)?
    } else {
        MilestoneRecordData {
            desired_state: require_scalar(&front_matter, "desired_state", relative)?,
            scope: optional_string_array(&front_matter, "scope", relative)?.unwrap_or_default(),
            validation_criteria: optional_string_array(
                &front_matter,
                "validation_criteria",
                relative,
            )?
            .unwrap_or_default(),
        }
    };
    let body = if body.is_empty() {
        None
    } else {
        Some(body.to_string())
    };

    Ok(CanonicalDomainRecord {
        record: DomainRecord {
            id,
            kind: spec.kind.to_string(),
            title,
            status,
            body,
            data_json: serde_json::to_string(&data)?,
            created_at,
            updated_at,
        },
        labels: optional_string_array(&front_matter, "labels", relative)?.unwrap_or_default(),
        relationships,
    })
}

fn validate_milestone_record(record: &CanonicalDomainRecord, relative: &Path) -> Result<()> {
    let data = normalized_milestone_data(&record.record.data_json)
        .with_context(|| format!("Invalid milestone data in {}", display_state_path(relative)))?;
    if data.desired_state.trim().is_empty()
        && record
            .record
            .body
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
    {
        bail!(
            "Milestone record {} must include desired_state or body",
            display_state_path(relative)
        );
    }
    validate_relationships(&record.relationships, relative)?;
    Ok(())
}

fn render_plan_record(
    record: &CanonicalDomainRecord,
    relationships: &Relationships,
    spec: &RecordKindSpec,
) -> Result<String> {
    let data = normalized_plan_data(&record.record.data_json)?;

    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.record.created_at.to_rfc3339()),
    )?;
    write_yaml_scalar(&mut output, "id", Some(&record.record.id))?;
    write_yaml_i64(&mut output, "revision", data.revision);
    write_yaml_scalar(&mut output, "owner", data.owner.as_deref())?;
    write_plan_revisions(&mut output, &data.revisions)?;
    write_yaml_relationships(&mut output, relationships)?;
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

fn parse_plan_domain_record(
    front_matter: BTreeMap<String, Value>,
    body: &str,
    relative: &Path,
    spec: &RecordKindSpec,
    id: String,
) -> Result<CanonicalDomainRecord> {
    let relationships = parse_relationships(&front_matter, relative)?;
    let title = require_scalar(&front_matter, "title", relative)?;
    let status = require_scalar(&front_matter, "status", relative)?;
    let created_at = require_datetime(&front_matter, "created_at", relative)?;
    let updated_at = require_datetime(&front_matter, "updated_at", relative)?;
    let mut data = if front_matter.contains_key("data") {
        normalized_plan_data(&require_scalar(&front_matter, "data", relative)?)?
    } else {
        PlanRecordData {
            revision: require_i64(&front_matter, "revision", relative)?,
            owner: optional_scalar(&front_matter, "owner")?,
            revisions: optional_yaml_value(&front_matter, "revisions", relative)?
                .unwrap_or_default(),
        }
    };
    if data.revisions.is_empty() {
        data.revisions.push(crate::models::PlanRevision {
            revision: data.revision,
            reason: "canonical".to_string(),
            body: body.to_string(),
        });
    }
    let body = if body.is_empty() {
        data.revisions
            .iter()
            .find(|revision| revision.revision == data.revision)
            .map(|revision| revision.body.clone())
            .filter(|body| !body.is_empty())
    } else {
        Some(body.to_string())
    };

    Ok(CanonicalDomainRecord {
        record: DomainRecord {
            id,
            kind: spec.kind.to_string(),
            title,
            status,
            body,
            data_json: serde_json::to_string(&data)?,
            created_at,
            updated_at,
        },
        labels: optional_string_array(&front_matter, "labels", relative)?.unwrap_or_default(),
        relationships,
    })
}

fn validate_plan_record(record: &CanonicalDomainRecord, relative: &Path) -> Result<()> {
    let data = normalized_plan_data(&record.record.data_json)
        .with_context(|| format!("Invalid plan data in {}", display_state_path(relative)))?;
    if data.revision < 1 {
        bail!(
            "Plan record {} must have a positive revision",
            display_state_path(relative)
        );
    }
    if data.revisions.is_empty() {
        bail!(
            "Plan record {} must include at least one revision",
            display_state_path(relative)
        );
    }
    if !data
        .revisions
        .iter()
        .any(|revision| revision.revision == data.revision)
    {
        bail!(
            "Plan record {} current revision is missing from revisions",
            display_state_path(relative)
        );
    }
    validate_relationships(&record.relationships, relative)?;
    Ok(())
}

fn validate_mission_status(status: &str, relative: &Path) -> Result<()> {
    match status {
        "draft" | "ready" | "active" | "closed" => Ok(()),
        _ => bail!(
            "Invalid mission status '{}' in {}; valid values: draft, ready, active, closed",
            status,
            display_state_path(relative)
        ),
    }
}

fn validate_mission_relationships(relationships: &Relationships, relative: &Path) -> Result<()> {
    if !relationships.children.is_empty() {
        bail!(
            "Mission relationships.children in {} is reserved and must be empty",
            display_state_path(relative)
        );
    }
    for attachment in &relationships.attachments {
        match (attachment.kind.as_str(), attachment.role.as_str()) {
            ("milestone", "has_checkpoint") | ("plan", "planned_by") => {}
            _ => bail!(
                "Invalid mission attachment {} {} ({}) in {}; use typed mission relationship semantics",
                attachment.kind,
                attachment.id,
                attachment.role,
                display_state_path(relative)
            ),
        }
    }
    for relation in &relationships.relates {
        if relation.kind == "issue" {
            match relation.relation_type.as_str() {
                "advances" | "blocked_by" | "validates" | "related" | "derived_from"
                | "supersedes" | "duplicates" => {}
                _ => {
                    crate::db::validate_relationship_type(&relation.relation_type)?;
                }
            }
        }
    }
    Ok(())
}

fn reject_unexpected_mission_front_matter(
    values: &BTreeMap<String, Value>,
    relative: &Path,
) -> Result<()> {
    let allowed = [
        "created_at",
        "id",
        "labels",
        "relationships",
        "schema",
        "schema_version",
        "status",
        "title",
        "updated_at",
    ];
    let unexpected = values
        .keys()
        .filter(|key| !allowed.contains(&key.as_str()))
        .map(String::as_str)
        .collect::<Vec<_>>();
    if !unexpected.is_empty() {
        bail!(
            "Unsupported mission front matter key(s) {} in {}; mission semantics belong in body sections and relationships",
            unexpected.join(", "),
            display_state_path(relative)
        );
    }
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

fn normalize_legacy_mission_relationships(mut relationships: Relationships) -> Relationships {
    let mut normalized_attachments = Vec::new();
    for attachment in relationships.attachments {
        match (attachment.kind.as_str(), attachment.role.as_str()) {
            ("issue", "advances" | "blocked_by" | "validates") | ("evidence", "validates") => {
                relationships.relates.push(RelatesRelationship {
                    kind: attachment.kind,
                    id: attachment.id,
                    relation_type: attachment.role,
                });
            }
            ("plan", "planned_by") | ("milestone", "has_checkpoint") => {
                normalized_attachments.push(attachment);
            }
            _ => normalized_attachments.push(attachment),
        }
    }
    relationships.attachments = normalized_attachments;
    sort_relationships(&mut relationships);
    relationships
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

fn default_domain_labels(kind: &str) -> Vec<String> {
    match kind {
        "mission" => vec!["mission".to_string()],
        _ => Vec::new(),
    }
}

fn normalized_domain_data_json(kind: &str, data_json: &str) -> Result<String> {
    let _: Value = serde_json::from_str(data_json)?;
    if kind == "mission" {
        Ok(MISSION_EMPTY_DATA_JSON.to_string())
    } else if kind == "evidence" {
        Ok(serde_json::to_string(&normalized_evidence_data(
            data_json,
        )?)?)
    } else if kind == "milestone" {
        Ok(serde_json::to_string(&normalized_milestone_data(
            data_json,
        )?)?)
    } else if kind == "plan" {
        Ok(serde_json::to_string(&normalized_plan_data(data_json)?)?)
    } else {
        Ok(data_json.to_string())
    }
}

fn normalized_domain_body(
    kind: &str,
    title: &str,
    body: Option<&str>,
    data_json: &str,
) -> Result<Option<String>> {
    if kind != "mission" {
        return Ok(body.map(str::to_string));
    }
    let sections = if data_json == MISSION_EMPTY_DATA_JSON {
        if let Some(body) = body {
            parse_mission_sections(body, Path::new("<record>")).unwrap_or_else(|_| {
                mission_sections_from_inputs(title, Some(body), Vec::new(), Vec::new(), Vec::new())
            })
        } else {
            mission_sections_from_inputs(title, body, Vec::new(), Vec::new(), Vec::new())
        }
    } else {
        legacy_mission_sections(body, data_json, title, Path::new("<record>"))?
    };
    Ok(Some(render_mission_sections(&sections)))
}

fn normalized_evidence_data(data_json: &str) -> Result<EvidenceRecordData> {
    let mut data: EvidenceRecordData = serde_json::from_str(data_json)?;
    if data.agent_identity.is_none() {
        data.agent_identity = data.producer.clone();
    }
    data.follow_up_ids.sort();
    Ok(data)
}

fn normalized_milestone_data(data_json: &str) -> Result<MilestoneRecordData> {
    serde_json::from_str(data_json).map_err(Into::into)
}

pub fn normalized_plan_data(data_json: &str) -> Result<PlanRecordData> {
    let mut data: PlanRecordData = serde_json::from_str(data_json)?;
    if data.revision < 1 {
        data.revision = 1;
    }
    data.revisions.sort_by_key(|revision| revision.revision);
    Ok(data)
}

pub fn mission_sections_from_inputs(
    title: &str,
    intent: Option<&str>,
    constraints: Vec<String>,
    risks: Vec<String>,
    validation: Vec<String>,
) -> MissionSections {
    MissionSections {
        intent: intent
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or(title)
            .to_string(),
        constraints: mission_list_section(constraints, "- None."),
        risks: mission_list_section(risks, "- None."),
        validation: mission_list_section(validation, "- Validation was not specified."),
        closeout_notes: None,
        notes: None,
    }
}

pub fn mission_sections_from_domain_record(record: &DomainRecord) -> Result<MissionSections> {
    let relative = canonical_record_kind("mission")
        .and_then(|spec| canonical_record_path(spec, &record.id))
        .unwrap_or_else(|_| PathBuf::from("<record>"));
    if record.data_json == MISSION_EMPTY_DATA_JSON {
        return parse_mission_sections(record.body.as_deref().unwrap_or(""), &relative);
    }
    legacy_mission_sections(
        record.body.as_deref(),
        &record.data_json,
        &record.title,
        &relative,
    )
}

pub fn render_mission_sections(sections: &MissionSections) -> String {
    let mut body = format!(
        "## Intent\n\n{}\n\n## Constraints\n\n{}\n\n## Risks\n\n{}\n\n## Validation\n\n{}",
        sections.intent.trim(),
        sections.constraints.trim(),
        sections.risks.trim(),
        sections.validation.trim()
    );
    if let Some(closeout_notes) = sections
        .closeout_notes
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        body.push_str("\n\n## Closeout Notes\n\n");
        body.push_str(closeout_notes);
    }
    if let Some(notes) = sections
        .notes
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        body.push_str("\n\n## Notes\n\n");
        body.push_str(notes);
    }
    normalize_body(&body)
}

pub fn parse_mission_sections(body: &str, relative: &Path) -> Result<MissionSections> {
    let mut sections = BTreeMap::<String, String>::new();
    let mut current_heading: Option<String> = None;
    let mut current_body = String::new();

    for line in body.lines() {
        if let Some(heading) = issue_level_two_heading(line) {
            if !MissionSections::ALL_NAMES
                .into_iter()
                .any(|name| name.title() == heading)
            {
                bail!(
                    "Unknown mission body section '{}' in {}",
                    heading,
                    display_state_path(relative)
                );
            }
            if let Some(previous) = current_heading.replace(heading.clone()) {
                finish_mission_section(&mut sections, previous, &current_body, relative)?;
                current_body.clear();
            } else if !current_body.trim().is_empty() {
                bail!(
                    "Content before first recognized mission body section in {}",
                    display_state_path(relative)
                );
            } else {
                current_body.clear();
            }
            continue;
        }

        if current_heading.is_none() && !line.trim().is_empty() {
            bail!(
                "Content before first recognized mission body section in {}",
                display_state_path(relative)
            );
        }
        current_body.push_str(line);
        current_body.push('\n');
    }

    if let Some(previous) = current_heading {
        finish_mission_section(&mut sections, previous, &current_body, relative)?;
    } else {
        bail!(
            "Missing required mission body section 'Intent' in {}",
            display_state_path(relative)
        );
    }

    Ok(MissionSections {
        intent: required_mission_section(&sections, MissionSectionName::Intent, relative)?,
        constraints: required_mission_section(
            &sections,
            MissionSectionName::Constraints,
            relative,
        )?,
        risks: required_mission_section(&sections, MissionSectionName::Risks, relative)?,
        validation: required_mission_section(&sections, MissionSectionName::Validation, relative)?,
        closeout_notes: sections
            .get(MissionSectionName::CloseoutNotes.title())
            .cloned(),
        notes: sections.get(MissionSectionName::Notes.title()).cloned(),
    })
}

fn finish_mission_section(
    sections: &mut BTreeMap<String, String>,
    heading: String,
    body: &str,
    relative: &Path,
) -> Result<()> {
    if sections.contains_key(&heading) {
        bail!(
            "Duplicate mission body section '{}' in {}",
            heading,
            display_state_path(relative)
        );
    }
    let content = body.trim().to_string();
    if content.is_empty() {
        bail!(
            "Empty mission body section '{}' in {}",
            heading,
            display_state_path(relative)
        );
    }
    sections.insert(heading, content);
    Ok(())
}

fn required_mission_section(
    sections: &BTreeMap<String, String>,
    name: MissionSectionName,
    relative: &Path,
) -> Result<String> {
    let heading = name.title();
    sections.get(heading).cloned().ok_or_else(|| {
        anyhow!(
            "Missing required mission body section '{}' in {}",
            heading,
            display_state_path(relative)
        )
    })
}

fn legacy_mission_sections(
    body: Option<&str>,
    data_json: &str,
    title: &str,
    relative: &Path,
) -> Result<MissionSections> {
    let data: Value = serde_json::from_str(data_json)
        .with_context(|| format!("Invalid data JSON in {}", display_state_path(relative)))?;
    Ok(MissionSections {
        intent: legacy_mission_intent(body, title),
        constraints: mission_list_section(value_string_array(&data, "constraints"), "- None."),
        risks: mission_list_section(value_string_array(&data, "risks"), "- None."),
        validation: mission_list_section(
            value_string_array(&data, "validation"),
            "- Validation was not specified.",
        ),
        closeout_notes: value_string_array(&data, "closeout_notes")
            .into_iter()
            .next()
            .filter(|value| !value.trim().is_empty()),
        notes: None,
    })
}

fn legacy_mission_intent(body: Option<&str>, title: &str) -> String {
    let Some(body) = body.map(str::trim).filter(|value| !value.is_empty()) else {
        return title.to_string();
    };
    body.lines()
        .map(|line| {
            if let Some(rest) = line.strip_prefix("## ") {
                format!("### {rest}")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn mission_list_section(values: Vec<String>, empty: &str) -> String {
    if values.is_empty() {
        return empty.to_string();
    }
    values
        .into_iter()
        .map(|value| {
            let value = value.trim().to_string();
            if value.starts_with("- ") {
                value
            } else {
                format!("- {value}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn value_string_array(data: &Value, key: &str) -> Vec<String> {
    data.get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(ToOwned::to_owned)
        .collect()
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
            if crate::storage_layout::is_local_atelier_path(&relative) {
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
    let evidence = required_issue_section(
        &sections,
        IssueSectionName::Evidence,
        relative,
        issue_id.as_deref(),
    )?;
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

fn write_yaml_i64(output: &mut String, key: &str, value: i64) {
    output.push_str(key);
    output.push_str(": ");
    output.push_str(&value.to_string());
    output.push('\n');
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

fn write_plan_revisions(
    output: &mut String,
    revisions: &[crate::models::PlanRevision],
) -> Result<()> {
    output.push_str("revisions");
    if revisions.is_empty() {
        output.push_str(": []\n");
        return Ok(());
    }
    output.push_str(":\n");
    for revision in revisions {
        output.push_str("- revision: ");
        output.push_str(&revision.revision.to_string());
        output.push('\n');
        output.push_str("  reason: ");
        output.push_str(&serde_json::to_string(&revision.reason)?);
        output.push('\n');
        output.push_str("  body: ");
        output.push_str(&serde_json::to_string(&revision.body)?);
        output.push('\n');
    }
    Ok(())
}

fn write_evidence_output_summary(
    output: &mut String,
    summary: Option<&EvidenceOutputSummary>,
) -> Result<()> {
    let Some(summary) = summary else {
        output.push_str("output: null\n");
        return Ok(());
    };
    output.push_str("output:\n");
    output.push_str(&format!(
        "  limit_bytes_per_stream: {}\n",
        summary.limit_bytes_per_stream
    ));
    write_evidence_stream_summary(output, "stdout", &summary.stdout)?;
    write_evidence_stream_summary(output, "stderr", &summary.stderr)?;
    Ok(())
}

fn write_evidence_stream_summary(
    output: &mut String,
    key: &str,
    stream: &crate::models::EvidenceStreamSummary,
) -> Result<()> {
    output.push_str("  ");
    output.push_str(key);
    output.push_str(":\n");
    output.push_str(&format!("    bytes: {}\n", stream.bytes));
    output.push_str("    summary: ");
    output.push_str(&serde_json::to_string(&stream.summary)?);
    output.push('\n');
    output.push_str(&format!("    truncated: {}\n", stream.truncated));
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
        let body = "## Description\n\nCanonical body\n\n## Outcome\n\nIssue Markdown round-trips without losing fields.\n\n## Evidence\n\n- `cargo test record_store` passes.";
        CanonicalIssueRecord {
            issue: Issue {
                id: id.to_string(),
                title: "Write RecordStore".to_string(),
                description: None,
                status: "todo".to_string(),
                issue_type: "task".to_string(),
                priority: "high".to_string(),
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

    fn mission_record(id: &str) -> CanonicalDomainRecord {
        let sections = MissionSections {
            intent: "Repair mission records.".to_string(),
            constraints: "- Keep command output readable.".to_string(),
            risks: "- Relationship buckets can drift.".to_string(),
            validation: "- `cargo test record_store` passes.".to_string(),
            closeout_notes: Some("Closed with validation evidence.".to_string()),
            notes: Some("Handoff context.".to_string()),
        };
        CanonicalDomainRecord {
            record: DomainRecord {
                id: id.to_string(),
                kind: "mission".to_string(),
                title: "Typed Mission".to_string(),
                status: "ready".to_string(),
                body: Some(render_mission_sections(&sections)),
                data_json: MISSION_EMPTY_DATA_JSON.to_string(),
                created_at: Utc.with_ymd_and_hms(2026, 6, 10, 12, 0, 0).unwrap(),
                updated_at: Utc.with_ymd_and_hms(2026, 6, 10, 13, 0, 0).unwrap(),
            },
            labels: vec!["mission".to_string()],
            relationships: Relationships {
                blocks: Vec::new(),
                children: Vec::new(),
                attachments: vec![
                    AttachmentRelationship {
                        kind: "milestone".to_string(),
                        id: "atelier-cpnt".to_string(),
                        role: "has_checkpoint".to_string(),
                    },
                    AttachmentRelationship {
                        kind: "plan".to_string(),
                        id: "atelier-plan".to_string(),
                        role: "planned_by".to_string(),
                    },
                ],
                relates: vec![
                    RelatesRelationship {
                        kind: "issue".to_string(),
                        id: "atelier-blok".to_string(),
                        relation_type: "blocked_by".to_string(),
                    },
                    RelatesRelationship {
                        kind: "issue".to_string(),
                        id: "atelier-supp".to_string(),
                        relation_type: "related".to_string(),
                    },
                    RelatesRelationship {
                        kind: "issue".to_string(),
                        id: "atelier-work".to_string(),
                        relation_type: "advances".to_string(),
                    },
                ],
            },
        }
    }

    fn evidence_record(id: &str) -> CanonicalDomainRecord {
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
        CanonicalDomainRecord {
            record: DomainRecord {
                id: id.to_string(),
                kind: "evidence".to_string(),
                title: "RecordStore evidence proof".to_string(),
                status: "pass".to_string(),
                body: Some("RecordStore evidence proof summary.".to_string()),
                data_json: serde_json::to_string(&data).unwrap(),
                created_at: Utc.with_ymd_and_hms(2026, 6, 10, 12, 0, 0).unwrap(),
                updated_at: Utc.with_ymd_and_hms(2026, 6, 10, 13, 0, 0).unwrap(),
            },
            labels: Vec::new(),
            relationships: Relationships {
                blocks: Vec::new(),
                children: Vec::new(),
                attachments: vec![AttachmentRelationship {
                    kind: "issue".to_string(),
                    id: "atelier-proof".to_string(),
                    role: "validates".to_string(),
                }],
                relates: Vec::new(),
            },
        }
    }

    fn milestone_record(id: &str) -> CanonicalDomainRecord {
        let data = MilestoneRecordData {
            desired_state: "Typed milestone contract is in place.".to_string(),
            scope: vec![
                "Canonical front matter".to_string(),
                "Projection metadata".to_string(),
            ],
            validation_criteria: vec!["RecordStore round-trip passes.".to_string()],
        };
        CanonicalDomainRecord {
            record: DomainRecord {
                id: id.to_string(),
                kind: "milestone".to_string(),
                title: "Typed Milestone".to_string(),
                status: "open".to_string(),
                body: Some("Typed milestone contract is in place.".to_string()),
                data_json: serde_json::to_string(&data).unwrap(),
                created_at: Utc.with_ymd_and_hms(2026, 6, 10, 12, 0, 0).unwrap(),
                updated_at: Utc.with_ymd_and_hms(2026, 6, 10, 13, 0, 0).unwrap(),
            },
            labels: Vec::new(),
            relationships: Relationships::default(),
        }
    }

    fn plan_record(id: &str) -> CanonicalDomainRecord {
        let data = PlanRecordData {
            revision: 2,
            owner: Some("planning".to_string()),
            revisions: vec![
                crate::models::PlanRevision {
                    revision: 1,
                    reason: "initial".to_string(),
                    body: "Initial plan.".to_string(),
                },
                crate::models::PlanRevision {
                    revision: 2,
                    reason: "refine".to_string(),
                    body: "Refined plan.".to_string(),
                },
            ],
        };
        CanonicalDomainRecord {
            record: DomainRecord {
                id: id.to_string(),
                kind: "plan".to_string(),
                title: "Typed Plan".to_string(),
                status: "open".to_string(),
                body: Some("Refined plan.".to_string()),
                data_json: serde_json::to_string(&data).unwrap(),
                created_at: Utc.with_ymd_and_hms(2026, 6, 10, 12, 0, 0).unwrap(),
                updated_at: Utc.with_ymd_and_hms(2026, 6, 10, 13, 0, 0).unwrap(),
            },
            labels: Vec::new(),
            relationships: Relationships::default(),
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
        assert!(!text.contains("acceptance:"));
        assert!(!text.contains("evidence_required:"));
        assert!(text.contains("## Outcome\n\nIssue Markdown round-trips without losing fields."));
        assert!(text.contains("## Evidence\n\n- `cargo test record_store` passes."));
        assert!(!text.contains("closed_at:"));
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
    fn mission_record_renders_and_parses_deterministically_without_data_blob() {
        let record = mission_record("atelier-miss");
        let spec = canonical_record_kind("mission").unwrap();
        let path = canonical_record_path(spec, "atelier-miss").unwrap();
        let text = render_domain_record(&record).unwrap();
        let parsed = parse_domain_record(&text, &path, spec).unwrap();

        assert_eq!(parsed, record);
        assert_eq!(render_domain_record(&parsed).unwrap(), text);
        assert!(text.contains("schema: \"atelier.mission\""));
        assert!(text.contains("labels:\n- \"mission\"\n"));
        assert!(!text.contains("\ndata: "));
        assert!(text.contains("## Intent\n\nRepair mission records."));
        assert!(text.contains("## Constraints\n\n- Keep command output readable."));
        assert!(text.contains("## Closeout Notes\n\nClosed with validation evidence."));
        assert!(text.contains(
            "  relates:\n  - kind: \"issue\"\n    id: \"atelier-blok\"\n    type: \"blocked_by\"\n"
        ));
        assert!(text.contains("    type: \"advances\""));
        assert!(text.contains("    type: \"related\""));
    }

    #[test]
    fn evidence_record_renders_and_parses_deterministically_without_data_blob() {
        let record = evidence_record("atelier-evdn");
        let spec = canonical_record_kind("evidence").unwrap();
        let path = canonical_record_path(spec, "atelier-evdn").unwrap();
        let text = render_domain_record(&record).unwrap();
        let parsed = parse_domain_record(&text, &path, spec).unwrap();

        assert_eq!(parsed, record);
        assert_eq!(render_domain_record(&parsed).unwrap(), text);
        assert!(text.contains("schema: \"atelier.evidence\""));
        assert!(!text.contains("\ndata: "));
        assert!(text.contains("evidence_type: \"validation\""));
        assert!(text.contains("captured_at: \"2026-06-10T12:30:00+00:00\""));
        assert!(text.contains("command: \"cargo test record_store\""));
        assert!(text.contains("agent_identity: \"gpt-5.4 implementer\""));
        assert!(text.contains("follow_up_ids:\n- \"atelier-follow\"\n"));
        assert!(text.contains("RecordStore evidence proof summary."));
    }

    #[test]
    fn plan_record_renders_and_parses_deterministically_without_data_blob() {
        let record = plan_record("atelier-plnn");
        let spec = canonical_record_kind("plan").unwrap();
        let path = canonical_record_path(spec, "atelier-plnn").unwrap();
        let text = render_domain_record(&record).unwrap();
        let parsed = parse_domain_record(&text, &path, spec).unwrap();

        assert_eq!(parsed, record);
        assert_eq!(render_domain_record(&parsed).unwrap(), text);
        assert!(text.contains("schema: \"atelier.plan\""));
        assert!(!text.contains("\ndata: "));
        assert!(text.contains("revision: 2"));
        assert!(text.contains("owner: \"planning\""));
        assert!(text.contains("revisions:\n- revision: 1\n  reason: \"initial\""));
        assert!(text.contains("  body: \"Refined plan.\""));
        assert!(text.contains("Refined plan."));
    }

    #[test]
    fn milestone_record_renders_and_parses_deterministically_without_data_blob() {
        let record = milestone_record("atelier-mile");
        let spec = canonical_record_kind("milestone").unwrap();
        let path = canonical_record_path(spec, "atelier-mile").unwrap();
        let text = render_domain_record(&record).unwrap();
        let parsed = parse_domain_record(&text, &path, spec).unwrap();

        assert_eq!(parsed, record);
        assert_eq!(render_domain_record(&parsed).unwrap(), text);
        assert!(text.contains("schema: \"atelier.milestone\""));
        assert!(!text.contains("\ndata: "));
        assert!(text.contains("desired_state: \"Typed milestone contract is in place.\""));
        assert!(text.contains("scope:\n- \"Canonical front matter\""));
        assert!(text.contains("validation_criteria:\n- \"RecordStore round-trip passes.\""));
    }

    #[test]
    fn legacy_plan_and_milestone_data_records_load_into_typed_front_matter() {
        let plan_spec = canonical_record_kind("plan").unwrap();
        let plan_path = canonical_record_path(plan_spec, "atelier-pleg").unwrap();
        let plan_text = r#"---
created_at: "2026-06-10T12:00:00+00:00"
id: "atelier-pleg"
data: "{\"owner\":\"agent\",\"revision\":2,\"revisions\":[{\"body\":\"Initial\",\"reason\":\"initial\",\"revision\":1},{\"body\":\"Updated\",\"reason\":\"revise\",\"revision\":2}]}"
relationships:
  attachments: []
  blocks: []
  children: []
  relates: []
schema: "atelier.plan"
schema_version: 1
status: "open"
title: "Legacy Plan"
updated_at: "2026-06-10T13:00:00+00:00"
---

Updated
"#;
        let plan = parse_domain_record(plan_text, &plan_path, plan_spec).unwrap();
        let rendered_plan = render_domain_record(&plan).unwrap();

        assert!(!rendered_plan.contains("\ndata: "));
        assert!(rendered_plan.contains("revision: 2"));
        assert!(rendered_plan.contains("owner: \"agent\""));
        assert!(rendered_plan.contains("  body: \"Updated\""));

        let milestone_spec = canonical_record_kind("milestone").unwrap();
        let milestone_path = canonical_record_path(milestone_spec, "atelier-mleg").unwrap();
        let milestone_text = r#"---
created_at: "2026-06-10T12:00:00+00:00"
id: "atelier-mleg"
data: "{\"desired_state\":\"Release gate\",\"scope\":[\"CLI\"],\"validation_criteria\":[\"Focused tests pass\"]}"
relationships:
  attachments: []
  blocks: []
  children: []
  relates: []
schema: "atelier.milestone"
schema_version: 1
status: "open"
title: "Legacy Milestone"
updated_at: "2026-06-10T13:00:00+00:00"
---

Release gate
"#;
        let milestone =
            parse_domain_record(milestone_text, &milestone_path, milestone_spec).unwrap();
        let rendered_milestone = render_domain_record(&milestone).unwrap();

        assert!(!rendered_milestone.contains("\ndata: "));
        assert!(rendered_milestone.contains("desired_state: \"Release gate\""));
        assert!(rendered_milestone.contains("scope:\n- \"CLI\""));
        assert!(rendered_milestone.contains("validation_criteria:\n- \"Focused tests pass\""));
    }

    #[test]
    fn legacy_evidence_data_record_loads_into_typed_front_matter() {
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
status: "pass"
title: "Legacy evidence proof"
updated_at: "2026-06-10T13:00:00+00:00"
---

Legacy evidence summary.
"#;
        let parsed = parse_domain_record(text, &path, spec).unwrap();
        let rendered = render_domain_record(&parsed).unwrap();

        assert!(parsed
            .relationships
            .attachments
            .iter()
            .any(|attachment| attachment.kind == "issue"
                && attachment.id == "atelier-proof"
                && attachment.role == "validates"));
        assert!(!rendered.contains("\ndata: "));
        assert!(rendered.contains("evidence_type: \"validation\""));
        assert!(rendered.contains("agent_identity: \"legacy agent\""));
        assert!(!rendered.contains("type: \"validates\""));
        assert!(parse_domain_record(&rendered, &path, spec).is_ok());
    }

    #[test]
    fn mission_render_normalizes_legacy_evidence_attachments() {
        let mut record = mission_record("atelier-miss");
        record
            .relationships
            .attachments
            .push(AttachmentRelationship {
                kind: "evidence".to_string(),
                id: "atelier-prof".to_string(),
                role: "validates".to_string(),
            });

        let text = render_domain_record(&record).unwrap();

        assert!(text.contains(
            "  relates:\n  - kind: \"evidence\"\n    id: \"atelier-prof\"\n    type: \"validates\"\n"
        ));
        assert!(
            !text.contains("kind: \"evidence\"\n    id: \"atelier-prof\"\n    role: \"validates\"")
        );
    }

    #[test]
    fn legacy_mission_data_record_loads_into_typed_sections_and_relationships() {
        let spec = canonical_record_kind("mission").unwrap();
        let path = canonical_record_path(spec, "atelier-legd").unwrap();
        let text = r#"---
created_at: "2026-06-10T12:00:00+00:00"
id: "atelier-legd"
data: "{\"constraints\":[\"Keep scope focused.\"],\"risks\":[\"Old buckets drift.\"],\"validation\":[\"Run focused tests.\"],\"work\":[]}"
relationships:
  attachments:
  - kind: "issue"
    id: "atelier-work"
    role: "advances"
  - kind: "issue"
    id: "atelier-vlid"
    role: "validates"
  - kind: "evidence"
    id: "atelier-proof"
    role: "validates"
  - kind: "plan"
    id: "atelier-plan"
    role: "planned_by"
  blocks: []
  children: []
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Legacy Mission"
updated_at: "2026-06-10T13:00:00+00:00"
---

Legacy intent.

## Problem

Legacy missions used free-form body headings.
"#;
        let parsed = parse_domain_record(text, &path, spec).unwrap();
        let rendered = render_domain_record(&parsed).unwrap();

        assert_eq!(parsed.record.data_json, MISSION_EMPTY_DATA_JSON);
        assert!(parsed
            .record
            .body
            .as_deref()
            .unwrap()
            .contains("## Constraints\n\n- Keep scope focused."));
        assert!(parsed.relationships.attachments.iter().any(|attachment| {
            attachment.kind == "plan"
                && attachment.id == "atelier-plan"
                && attachment.role == "planned_by"
        }));
        assert!(parsed.relationships.relates.iter().any(|relation| {
            relation.kind == "issue"
                && relation.id == "atelier-work"
                && relation.relation_type == "advances"
        }));
        assert!(parsed.relationships.relates.iter().any(|relation| {
            relation.kind == "issue"
                && relation.id == "atelier-vlid"
                && relation.relation_type == "validates"
        }));
        assert!(parsed.relationships.relates.iter().any(|relation| {
            relation.kind == "evidence"
                && relation.id == "atelier-proof"
                && relation.relation_type == "validates"
        }));
        assert!(!rendered.contains("\ndata: "));
        assert!(rendered.contains("## Intent\n\nLegacy intent."));
        assert!(rendered.contains("### Problem\n\nLegacy missions used free-form body headings."));
        assert!(parse_domain_record(&rendered, &path, spec).is_ok());
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
            3
        );
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
