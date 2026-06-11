use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::db::{
    validate_issue_type, validate_link_type, validate_priority, validate_status, Database,
};
use crate::models::DomainRecord;

const KIND: &str = "plan";

pub fn create(
    db: &Database,
    title: &str,
    body: Option<&str>,
    reason: Option<&str>,
    json_output: bool,
) -> Result<()> {
    let data = json!({
        "revision": 1,
        "revisions": [{
            "revision": 1,
            "reason": reason.unwrap_or("initial"),
            "body": body.unwrap_or("")
        }]
    });
    let id = db.create_record(KIND, title, "open", body, &data.to_string())?;
    let record = db.require_record(KIND, &id)?;
    print_record(db, &record, json_output)
}

pub fn show(db: &Database, id: &str, json_output: bool) -> Result<()> {
    let record = db.require_record(KIND, id)?;
    print_record(db, &record, json_output)
}

pub fn list(db: &Database, status: Option<&str>, json_output: bool) -> Result<()> {
    let records = db.list_records(KIND, status)?;
    if json_output {
        let data: Vec<Value> = records.iter().map(record_json).collect::<Result<_>>()?;
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "data": data }))?
        );
        return Ok(());
    }
    if records.is_empty() {
        print_heading("Plans");
        println!("(none)");
        return Ok(());
    }
    print_heading("Plans");
    println!("{} total", records.len());
    for record in records {
        println!("  {:<14} {:<10} {}", record.id, record.status, record.title);
    }
    Ok(())
}

pub fn revise(
    db: &Database,
    id: &str,
    body: &str,
    reason: Option<&str>,
    json_output: bool,
) -> Result<()> {
    let current = db.require_record(KIND, id)?;
    let mut data: Value = serde_json::from_str(&current.data_json)?;
    let next_revision = data["revision"].as_i64().unwrap_or(1) + 1;
    data["revision"] = json!(next_revision);
    let revision = json!({
        "revision": next_revision,
        "reason": reason.unwrap_or("revision"),
        "body": body
    });
    data["revisions"]
        .as_array_mut()
        .expect("plan revisions must be an array")
        .push(revision);
    db.update_record(
        KIND,
        id,
        None,
        None,
        Some(body),
        Some(&serde_json::to_string(&data)?),
    )?;
    let record = db.require_record(KIND, id)?;
    print_record(db, &record, json_output)
}

pub fn link(
    db: &Database,
    id: &str,
    target_kind: &str,
    target_id: &str,
    relation_type: &str,
    json_output: bool,
) -> Result<()> {
    db.add_record_link(KIND, id, target_kind, target_id, relation_type)?;
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "source": { "kind": KIND, "id": id },
                "target": { "kind": target_kind, "id": target_id },
                "type": relation_type
            }))?
        );
    } else {
        println!("Linked plan {id} {relation_type} {target_kind} {target_id}");
    }
    Ok(())
}

pub fn apply(
    db: &Database,
    input: &str,
    dry_run_override: bool,
    validate_only_override: bool,
    json_output: bool,
) -> Result<()> {
    let bytes = fs::read(input).with_context(|| format!("Failed to read bulk plan {input}"))?;
    let plan: BulkPlan = serde_json::from_slice(&bytes)
        .with_context(|| format!("Failed to parse bulk plan {input} as JSON"))?;
    let options = plan.effective_options(dry_run_override, validate_only_override);
    validate_bulk_plan(db, &plan)?;

    if options.validate_only {
        print_apply_summary(
            json!({
                "applied": false,
                "dry_run": false,
                "validate_only": true,
                "records": {},
                "links": plan.links.len(),
                "message": "bulk plan is valid"
            }),
            json_output,
        )?;
        return Ok(());
    }

    if options.dry_run {
        let preview = dry_run_preview(&plan);
        print_apply_summary(preview, json_output)?;
        return Ok(());
    }

    let summary = db.transaction(|| apply_bulk_plan(db, &plan))?;
    match options.export.as_str() {
        "auto" => crate::commands::export::run_canonical(db, &find_state_dir()?, false)?,
        "check_only" => crate::commands::export::run_canonical(db, &find_state_dir()?, true)?,
        "skip" => {}
        other => bail!("Invalid export option '{other}'"),
    }

    print_apply_summary(summary, json_output)
}

fn print_record(db: &Database, record: &DomainRecord, json_output: bool) -> Result<()> {
    if json_output {
        let mut data = record_json(record)?;
        data["links"] = serde_json::to_value(db.list_record_links(KIND, &record.id)?)?;
        println!("{}", serde_json::to_string_pretty(&data)?);
        return Ok(());
    }
    println!("{} [plan] {} - {}", record.id, record.status, record.title);
    println!(
        "{}",
        "=".repeat(record.id.len() + record.status.len() + record.title.len() + 11)
    );
    println!("Status:   {}", record.status);
    println!("Revision: {}", data_revision(record).unwrap_or(1));
    println!("Created:  {}", record.created_at.to_rfc3339());
    println!("Updated:  {}", record.updated_at.to_rfc3339());
    let links = db.list_record_links(KIND, &record.id)?;
    println!("Links:    {}", links.len());
    print_heading("Body");
    if let Some(body) = &record.body {
        if !body.is_empty() {
            println!("{body}");
        } else {
            println!("(none)");
        }
    } else {
        println!("(none)");
    }
    if !links.is_empty() {
        print_heading("Links");
        for link in links {
            let (kind, id) = if link.source_kind == KIND && link.source_id == record.id {
                (link.target_kind, link.target_id)
            } else {
                (link.source_kind, link.source_id)
            };
            println!("  {} {} {}", link.relation_type, kind, id);
        }
    }
    Ok(())
}

fn print_heading(title: &str) {
    println!("{title}");
    println!("{}", "-".repeat(title.len()));
}

fn data_revision(record: &DomainRecord) -> Result<i64> {
    let data: Value = serde_json::from_str(&record.data_json)?;
    Ok(data["revision"].as_i64().unwrap_or(1))
}

#[derive(Debug, Deserialize)]
struct BulkPlan {
    schema: String,
    schema_version: i64,
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    apply: BulkApplyOptions,
    #[serde(default)]
    records: BulkRecords,
    #[serde(default)]
    links: Vec<BulkLink>,
}

#[derive(Debug, Deserialize)]
struct BulkApplyOptions {
    #[serde(default)]
    dry_run: bool,
    #[serde(default = "default_on_conflict")]
    on_conflict: String,
    #[serde(default = "default_true")]
    atomic: bool,
    #[serde(default = "default_export")]
    export: String,
    #[serde(default)]
    validate_only: bool,
}

impl Default for BulkApplyOptions {
    fn default() -> Self {
        Self {
            dry_run: false,
            on_conflict: default_on_conflict(),
            atomic: true,
            export: default_export(),
            validate_only: false,
        }
    }
}

#[derive(Debug)]
struct EffectiveApplyOptions {
    dry_run: bool,
    validate_only: bool,
    export: String,
}

#[derive(Debug, Default, Deserialize)]
struct BulkRecords {
    #[serde(default)]
    issues: Vec<BulkIssue>,
    #[serde(default)]
    missions: Vec<BulkMission>,
    #[serde(default)]
    milestones: Vec<BulkMilestone>,
    #[serde(default)]
    plans: Vec<BulkPlanRecord>,
    #[serde(default)]
    evidence: Vec<BulkEvidence>,
}

#[derive(Debug, Deserialize)]
struct BulkIssue {
    client_ref: String,
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default = "default_issue_type")]
    issue_type: String,
    #[serde(default = "default_priority")]
    priority: String,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    parent: Option<BulkRef>,
    #[serde(default)]
    depends_on: Vec<BulkRef>,
    #[serde(default)]
    blocks: Vec<BulkRef>,
    #[serde(default)]
    notes: Vec<BulkNote>,
    #[serde(default)]
    acceptance: Vec<String>,
    #[serde(default)]
    evidence_required: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct BulkMission {
    client_ref: String,
    title: String,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    plans: Vec<BulkRef>,
    #[serde(default)]
    milestones: Vec<BulkRef>,
}

#[derive(Debug, Deserialize)]
struct BulkMilestone {
    client_ref: String,
    title: String,
    #[serde(default)]
    desired_state: String,
    #[serde(default)]
    scope: Vec<String>,
    #[serde(default)]
    validation_criteria: Vec<String>,
    #[serde(default)]
    missions: Vec<BulkRef>,
    #[serde(default)]
    contributing_work: Vec<BulkRef>,
}

#[derive(Debug, Deserialize)]
struct BulkPlanRecord {
    client_ref: String,
    title: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    owner: Option<String>,
    #[serde(default)]
    applies_to: Vec<BulkRef>,
    #[serde(default)]
    supersedes: Vec<BulkRef>,
}

#[derive(Debug, Deserialize)]
struct BulkEvidence {
    client_ref: String,
    title: String,
    evidence_type: String,
    result: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    validates: Vec<BulkRef>,
    #[serde(default)]
    artifact: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct BulkRef {
    #[serde(default)]
    client_ref: Option<String>,
    #[serde(default)]
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BulkNote {
    body: String,
    #[serde(default)]
    author: Option<String>,
    #[serde(default)]
    created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BulkLink {
    source: BulkRef,
    #[serde(rename = "type")]
    relation_type: String,
    target: BulkRef,
}

#[derive(Debug, Clone)]
struct ResolvedRef {
    kind: String,
    id: String,
}

fn validate_bulk_plan(db: &Database, plan: &BulkPlan) -> Result<()> {
    if plan.schema != "atelier.bulk-plan" {
        bail!("Invalid bulk plan schema '{}'", plan.schema);
    }
    if plan.schema_version != 1 {
        bail!(
            "Unsupported bulk plan schema_version {}",
            plan.schema_version
        );
    }
    if plan.title.trim().is_empty() {
        bail!("Bulk plan title cannot be empty");
    }
    if !["fail", "skip_existing", "update_existing"].contains(&plan.apply.on_conflict.as_str()) {
        bail!("Invalid on_conflict option '{}'", plan.apply.on_conflict);
    }
    if !plan.apply.atomic {
        bail!("Non-atomic bulk apply is not supported");
    }
    if !["auto", "skip", "check_only"].contains(&plan.apply.export.as_str()) {
        bail!("Invalid export option '{}'", plan.apply.export);
    }

    let mut refs = BTreeSet::new();
    for (client_ref, kind) in plan.client_refs() {
        validate_client_ref(&client_ref)?;
        if !refs.insert(client_ref.clone()) {
            bail!("Duplicate client_ref '{}'", client_ref);
        }
        if kind == "issue" {
            continue;
        }
        crate::db::validate_record_kind(kind)?;
    }

    for issue in &plan.records.issues {
        if issue.title.trim().is_empty() {
            bail!("Issue {} title cannot be empty", issue.client_ref);
        }
        validate_issue_type(&issue.issue_type)?;
        validate_priority(&issue.priority)?;
        if let Some(status) = &issue.status {
            validate_status(status)?;
        }
        validate_refs_exist(db, plan, issue.parent.iter(), &issue.client_ref)?;
        validate_refs_exist(db, plan, issue.depends_on.iter(), &issue.client_ref)?;
        validate_refs_exist(db, plan, issue.blocks.iter(), &issue.client_ref)?;
    }
    for mission in &plan.records.missions {
        if mission.title.trim().is_empty() {
            bail!("Mission {} title cannot be empty", mission.client_ref);
        }
        validate_refs_exist(db, plan, mission.plans.iter(), &mission.client_ref)?;
        validate_refs_exist(db, plan, mission.milestones.iter(), &mission.client_ref)?;
    }
    for milestone in &plan.records.milestones {
        if milestone.title.trim().is_empty() {
            bail!("Milestone {} title cannot be empty", milestone.client_ref);
        }
        validate_refs_exist(db, plan, milestone.missions.iter(), &milestone.client_ref)?;
        validate_refs_exist(
            db,
            plan,
            milestone.contributing_work.iter(),
            &milestone.client_ref,
        )?;
    }
    for record in &plan.records.plans {
        if record.title.trim().is_empty() {
            bail!("Plan {} title cannot be empty", record.client_ref);
        }
        validate_refs_exist(db, plan, record.applies_to.iter(), &record.client_ref)?;
        validate_refs_exist(db, plan, record.supersedes.iter(), &record.client_ref)?;
    }
    for evidence in &plan.records.evidence {
        if evidence.title.trim().is_empty() {
            bail!("Evidence {} title cannot be empty", evidence.client_ref);
        }
        if ![
            "test",
            "log",
            "screenshot",
            "report",
            "benchmark",
            "external",
        ]
        .contains(&evidence.evidence_type.as_str())
        {
            bail!(
                "Invalid evidence_type '{}' for {}",
                evidence.evidence_type,
                evidence.client_ref
            );
        }
        if !["pass", "fail", "blocked", "informational"].contains(&evidence.result.as_str()) {
            bail!(
                "Invalid evidence result '{}' for {}",
                evidence.result,
                evidence.client_ref
            );
        }
        validate_refs_exist(db, plan, evidence.validates.iter(), &evidence.client_ref)?;
    }
    for link in &plan.links {
        validate_link_type(&link.relation_type)?;
        validate_ref_exists(db, plan, &link.source, "link.source")?;
        validate_ref_exists(db, plan, &link.target, "link.target")?;
    }
    Ok(())
}

fn apply_bulk_plan(db: &Database, plan: &BulkPlan) -> Result<Value> {
    let mut resolved = BTreeMap::<String, ResolvedRef>::new();
    let mut created = BTreeMap::<String, Vec<Value>>::new();

    for issue in &plan.records.issues {
        let description = issue_description(issue);
        let id = db.create_issue_with_type(
            &issue.title,
            description.as_deref(),
            &issue.priority,
            &issue.issue_type,
        )?;
        for label in sorted(issue.labels.clone()) {
            db.add_label(&id, &label)?;
        }
        for note in &issue.notes {
            let body = match &note.author {
                Some(author) if !author.trim().is_empty() => format!("[{author}] {}", note.body),
                _ => note.body.clone(),
            };
            if let Some(created_at) = &note.created_at {
                db.add_comment_at(&id, &body, "note", created_at)?;
            } else {
                db.add_comment(&id, &body, "note")?;
            }
        }
        if issue.status.as_deref() == Some("closed") {
            db.close_issue(&id)?;
        }
        resolved.insert(
            issue.client_ref.clone(),
            ResolvedRef {
                kind: "issue".to_string(),
                id: id.clone(),
            },
        );
        created
            .entry("issues".to_string())
            .or_default()
            .push(json!({
                "client_ref": issue.client_ref,
                "id": id,
            }));
    }

    for mission in &plan.records.missions {
        let data = json!({
            "constraints": [],
            "risks": [],
            "validation": [],
            "milestones": [],
            "plans": [],
            "evidence": [],
            "work": [],
            "labels": sorted(mission.labels.clone()),
        });
        create_bulk_record(
            db,
            &mut resolved,
            &mut created,
            &mission.client_ref,
            "mission",
            &mission.title,
            mission.body.as_deref(),
            data,
        )?;
    }
    for milestone in &plan.records.milestones {
        let data = json!({
            "desired_state": milestone.desired_state,
            "scope": milestone.scope,
            "validation_criteria": milestone.validation_criteria,
        });
        create_bulk_record(
            db,
            &mut resolved,
            &mut created,
            &milestone.client_ref,
            "milestone",
            &milestone.title,
            Some(&milestone.desired_state),
            data,
        )?;
    }
    for record in &plan.records.plans {
        let data = json!({
            "revision": 1,
            "owner": record.owner,
            "revisions": [{
                "revision": 1,
                "reason": "bulk_apply",
                "body": record.body
            }]
        });
        create_bulk_record(
            db,
            &mut resolved,
            &mut created,
            &record.client_ref,
            "plan",
            &record.title,
            Some(&record.body),
            data,
        )?;
    }
    for evidence in &plan.records.evidence {
        let data = json!({
            "evidence_type": evidence.evidence_type,
            "result": evidence.result,
            "artifact": evidence.artifact,
        });
        create_bulk_record(
            db,
            &mut resolved,
            &mut created,
            &evidence.client_ref,
            "evidence",
            &evidence.title,
            Some(&evidence.body),
            data,
        )?;
    }

    for issue in &plan.records.issues {
        let source = resolved_ref(db, plan, &resolved, &BulkRef::client(&issue.client_ref))?;
        if let Some(parent) = &issue.parent {
            let parent = resolved_ref(db, plan, &resolved, parent)?;
            if parent.kind != "issue" {
                bail!(
                    "Issue parent for {} must resolve to an issue",
                    issue.client_ref
                );
            }
            db.update_parent(&source.id, Some(&parent.id))?;
        }
        for blocker in &issue.depends_on {
            let blocker = resolved_ref(db, plan, &resolved, blocker)?;
            if blocker.kind != "issue" {
                bail!(
                    "depends_on for {} must resolve to an issue",
                    issue.client_ref
                );
            }
            db.add_dependency(&source.id, &blocker.id)?;
        }
        for blocked in &issue.blocks {
            let blocked = resolved_ref(db, plan, &resolved, blocked)?;
            if blocked.kind != "issue" {
                bail!("blocks for {} must resolve to an issue", issue.client_ref);
            }
            db.add_dependency(&blocked.id, &source.id)?;
        }
    }

    let mut link_count = 0usize;
    for mission in &plan.records.missions {
        let source = resolved_ref(db, plan, &resolved, &BulkRef::client(&mission.client_ref))?;
        for target in &mission.plans {
            add_resolved_link(db, plan, &resolved, &source, target, "planned_by")?;
            link_count += 1;
        }
        for target in &mission.milestones {
            add_resolved_link(db, plan, &resolved, &source, target, "has_checkpoint")?;
            link_count += 1;
        }
    }
    for milestone in &plan.records.milestones {
        let source = resolved_ref(db, plan, &resolved, &BulkRef::client(&milestone.client_ref))?;
        for target in &milestone.missions {
            add_resolved_link(db, plan, &resolved, &source, target, "advances")?;
            link_count += 1;
        }
        for target in &milestone.contributing_work {
            let contributor = resolved_ref(db, plan, &resolved, target)?;
            db.add_record_link(
                &contributor.kind,
                &contributor.id,
                &source.kind,
                &source.id,
                "contributes_to",
            )?;
            link_count += 1;
        }
    }
    for record in &plan.records.plans {
        let source = resolved_ref(db, plan, &resolved, &BulkRef::client(&record.client_ref))?;
        for target in &record.applies_to {
            add_resolved_link(db, plan, &resolved, &source, target, "planned_by")?;
            link_count += 1;
        }
        for target in &record.supersedes {
            add_resolved_link(db, plan, &resolved, &source, target, "supersedes")?;
            link_count += 1;
        }
    }
    for evidence in &plan.records.evidence {
        let source = resolved_ref(db, plan, &resolved, &BulkRef::client(&evidence.client_ref))?;
        for target in &evidence.validates {
            add_resolved_link(db, plan, &resolved, &source, target, "validates")?;
            link_count += 1;
        }
    }
    for link in &plan.links {
        let source = resolved_ref(db, plan, &resolved, &link.source)?;
        add_resolved_link(
            db,
            plan,
            &resolved,
            &source,
            &link.target,
            &link.relation_type,
        )?;
        link_count += 1;
    }

    Ok(json!({
        "applied": true,
        "dry_run": false,
        "validate_only": false,
        "title": plan.title,
        "description": plan.description,
        "records": created,
        "links": link_count,
    }))
}

fn record_json(record: &DomainRecord) -> Result<Value> {
    Ok(json!({
        "id": record.id,
        "kind": record.kind,
        "title": record.title,
        "status": record.status,
        "body": record.body,
        "data": serde_json::from_str::<Value>(&record.data_json)?,
        "created_at": record.created_at.to_rfc3339(),
        "updated_at": record.updated_at.to_rfc3339()
    }))
}

impl BulkPlan {
    fn effective_options(
        &self,
        dry_run_override: bool,
        validate_only_override: bool,
    ) -> EffectiveApplyOptions {
        EffectiveApplyOptions {
            dry_run: self.apply.dry_run || dry_run_override,
            validate_only: self.apply.validate_only || validate_only_override,
            export: self.apply.export.clone(),
        }
    }

    fn client_refs(&self) -> Vec<(String, &'static str)> {
        let mut refs = Vec::new();
        refs.extend(
            self.records
                .issues
                .iter()
                .map(|record| (record.client_ref.clone(), "issue")),
        );
        refs.extend(
            self.records
                .missions
                .iter()
                .map(|record| (record.client_ref.clone(), "mission")),
        );
        refs.extend(
            self.records
                .milestones
                .iter()
                .map(|record| (record.client_ref.clone(), "milestone")),
        );
        refs.extend(
            self.records
                .plans
                .iter()
                .map(|record| (record.client_ref.clone(), "plan")),
        );
        refs.extend(
            self.records
                .evidence
                .iter()
                .map(|record| (record.client_ref.clone(), "evidence")),
        );
        refs
    }

    fn client_kind(&self, client_ref: &str) -> Option<&'static str> {
        self.client_refs()
            .into_iter()
            .find_map(|(candidate, kind)| (candidate == client_ref).then_some(kind))
    }
}

impl BulkRef {
    fn client(client_ref: &str) -> Self {
        Self {
            client_ref: Some(client_ref.to_string()),
            id: None,
        }
    }
}

fn validate_client_ref(client_ref: &str) -> Result<()> {
    let mut chars = client_ref.chars();
    let Some(first) = chars.next() else {
        bail!("client_ref cannot be empty");
    };
    if !first.is_ascii_alphabetic() {
        bail!(
            "client_ref '{}' must start with an ASCII letter",
            client_ref
        );
    }
    if !chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | ':' | '-')) {
        bail!("client_ref '{}' contains an invalid character", client_ref);
    }
    Ok(())
}

fn validate_refs_exist<'a>(
    db: &Database,
    plan: &BulkPlan,
    refs: impl Iterator<Item = &'a BulkRef>,
    owner: &str,
) -> Result<()> {
    for reference in refs {
        validate_ref_exists(db, plan, reference, owner)?;
    }
    Ok(())
}

fn validate_ref_exists(
    db: &Database,
    plan: &BulkPlan,
    reference: &BulkRef,
    owner: &str,
) -> Result<()> {
    match (&reference.client_ref, &reference.id) {
        (Some(client_ref), None) => {
            if plan.client_kind(client_ref).is_none() {
                bail!(
                    "Reference '{}' for {} does not resolve in this file",
                    client_ref,
                    owner
                );
            }
        }
        (None, Some(id)) => {
            resolve_existing_ref(db, id)
                .with_context(|| format!("Reference id '{}' for {} does not resolve", id, owner))?;
        }
        _ => bail!(
            "Reference for {} must contain exactly one of client_ref or id",
            owner
        ),
    }
    Ok(())
}

fn resolve_existing_ref(db: &Database, id: &str) -> Result<ResolvedRef> {
    if db.get_issue(id)?.is_some() {
        return Ok(ResolvedRef {
            kind: "issue".to_string(),
            id: id.to_string(),
        });
    }
    for kind in [
        "mission",
        "milestone",
        "plan",
        "evidence",
        "workflow_validator",
    ] {
        if db.get_record(kind, id)?.is_some() {
            return Ok(ResolvedRef {
                kind: kind.to_string(),
                id: id.to_string(),
            });
        }
    }
    bail!("Record id '{}' not found", id)
}

fn resolved_ref(
    db: &Database,
    _plan: &BulkPlan,
    resolved: &BTreeMap<String, ResolvedRef>,
    reference: &BulkRef,
) -> Result<ResolvedRef> {
    match (&reference.client_ref, &reference.id) {
        (Some(client_ref), None) => resolved
            .get(client_ref)
            .cloned()
            .with_context(|| format!("Reference '{}' has not been allocated", client_ref)),
        (None, Some(id)) => resolve_existing_ref(db, id),
        _ => bail!("Reference must contain exactly one of client_ref or id"),
    }
}

fn create_bulk_record(
    db: &Database,
    resolved: &mut BTreeMap<String, ResolvedRef>,
    created: &mut BTreeMap<String, Vec<Value>>,
    client_ref: &str,
    kind: &str,
    title: &str,
    body: Option<&str>,
    data: Value,
) -> Result<()> {
    let id = db.create_record(kind, title, "open", body, &serde_json::to_string(&data)?)?;
    resolved.insert(
        client_ref.to_string(),
        ResolvedRef {
            kind: kind.to_string(),
            id: id.clone(),
        },
    );
    created
        .entry(created_key(kind).to_string())
        .or_default()
        .push(json!({ "client_ref": client_ref, "id": id }));
    Ok(())
}

fn add_resolved_link(
    db: &Database,
    plan: &BulkPlan,
    resolved: &BTreeMap<String, ResolvedRef>,
    source: &ResolvedRef,
    target: &BulkRef,
    relation_type: &str,
) -> Result<()> {
    let target = resolved_ref(db, plan, resolved, target)?;
    db.add_record_link(
        &source.kind,
        &source.id,
        &target.kind,
        &target.id,
        relation_type,
    )?;
    Ok(())
}

fn issue_description(issue: &BulkIssue) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(description) = &issue.description {
        if !description.trim().is_empty() {
            parts.push(description.clone());
        }
    }
    if !issue.acceptance.is_empty() {
        parts.push(format!("Acceptance:\n{}", bullet_list(&issue.acceptance)));
    }
    if !issue.evidence_required.is_empty() {
        parts.push(format!(
            "Evidence required:\n{}",
            bullet_list(&issue.evidence_required)
        ));
    }
    (!parts.is_empty()).then(|| parts.join("\n\n"))
}

fn bullet_list(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("- {value}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn dry_run_preview(plan: &BulkPlan) -> Value {
    let mut records = BTreeMap::<String, Vec<Value>>::new();
    for (client_ref, kind) in plan.client_refs() {
        records
            .entry(format!("{kind}s"))
            .or_default()
            .push(json!({ "client_ref": client_ref, "kind": kind }));
    }
    json!({
        "applied": false,
        "dry_run": true,
        "validate_only": false,
        "title": plan.title,
        "description": plan.description,
        "records": records,
        "links": plan.links.len(),
    })
}

fn print_apply_summary(summary: Value, json_output: bool) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(&summary)?);
    } else if summary["validate_only"].as_bool().unwrap_or(false) {
        println!("Bulk plan is valid.");
    } else if summary["dry_run"].as_bool().unwrap_or(false) {
        println!("Bulk plan preview is valid.");
    } else {
        println!("Bulk plan applied.");
    }
    Ok(())
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

fn created_key(kind: &str) -> &str {
    match kind {
        "evidence" => "evidence",
        "mission" => "missions",
        "milestone" => "milestones",
        "plan" => "plans",
        "issue" => "issues",
        _ => kind,
    }
}

fn find_state_dir() -> Result<PathBuf> {
    let root = find_repo_root(&std::env::current_dir()?)?;
    Ok(root.join(".atelier-state"))
}

fn find_repo_root(start: &Path) -> Result<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join(".atelier-state").is_dir() || current.join(".atelier").is_dir() {
            return Ok(current);
        }
        if !current.pop() {
            bail!("Not an Atelier repository (or any parent). Run from a checkout with .atelier-state/.");
        }
    }
}

fn default_on_conflict() -> String {
    "fail".to_string()
}

fn default_true() -> bool {
    true
}

fn default_export() -> String {
    "auto".to_string()
}

fn default_issue_type() -> String {
    "task".to_string()
}

fn default_priority() -> String {
    "medium".to_string()
}
