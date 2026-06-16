use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use atelier_core::{
    DomainRecord, EvidenceRecordData, Issue, MilestoneRecordData, PlanRecordData, PlanRevision,
};
use atelier_records::{
    is_attachment_role, CanonicalIssueRecord, IssueSections, RecordStore, Relationships,
};
use atelier_sqlite::{
    validate_issue_type, validate_priority, validate_record_kind, validate_status, Database,
};

const KIND: &str = "plan";

pub fn create(
    state_dir: &Path,
    db_path: &Path,
    title: &str,
    body: Option<&str>,
    reason: Option<&str>,
) -> Result<()> {
    let data = PlanRecordData {
        revision: 1,
        owner: None,
        revisions: vec![PlanRevision {
            revision: 1,
            reason: reason.unwrap_or("initial").to_string(),
            body: body.unwrap_or("").to_string(),
        }],
    };
    let store = RecordStore::new(state_dir);
    let created =
        store.create_domain_record(KIND, title, "open", body, &serde_json::to_string(&data)?)?;
    refresh_projection(state_dir, db_path)?;
    let db = Database::open(db_path)?;
    let record = db.require_record(KIND, &created.record.id)?;
    print_record(&db, &record)
}

pub fn show(db: &Database, id: &str) -> Result<()> {
    let record = canonical_record_detail(KIND, id)?.unwrap_or(db.require_record(KIND, id)?);
    print_record(&db, &record)
}

pub fn list(db: &Database, status: Option<&str>) -> Result<()> {
    let records = db.list_records(KIND, status)?;
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
    state_dir: &Path,
    db_path: &Path,
    id: &str,
    body: &str,
    reason: Option<&str>,
) -> Result<()> {
    let store = RecordStore::new(state_dir);
    let mut current = store.load_domain_record_by_id(KIND, id)?;
    let mut data = atelier_records::normalized_plan_data(&current.record.data_json)?;
    let next_revision = data.revision + 1;
    data.revision = next_revision;
    data.revisions.push(PlanRevision {
        revision: next_revision,
        reason: reason.unwrap_or("revision").to_string(),
        body: body.to_string(),
    });
    current.record.body = Some(body.to_string());
    current.record.data_json = serde_json::to_string(&data)?;
    current.record.updated_at = chrono::Utc::now();
    store.write_domain_record_atomic(&current)?;
    refresh_projection(state_dir, db_path)?;
    let db = Database::open(db_path)?;
    let record = db.require_record(KIND, id)?;
    print_record(&db, &record)
}

pub fn link(
    state_dir: &Path,
    db_path: &Path,
    id: &str,
    target_kind: &str,
    target_id: &str,
    relation_type: &str,
) -> Result<()> {
    let db = Database::open(db_path)?;
    db.require_record(KIND, id)?;
    validate_record_ref(&db, target_kind, target_id)?;
    drop(db);
    let store = RecordStore::new(state_dir);
    if is_attachment_role(relation_type) {
        store.add_attachment_relationship(KIND, id, target_kind, target_id, relation_type)?;
    } else {
        store.add_relates_relationship(KIND, id, target_kind, target_id, relation_type)?;
    }
    refresh_projection(state_dir, db_path)?;
    println!("Linked plan {id} {relation_type} {target_kind} {target_id}");
    Ok(())
}

fn validate_record_ref(db: &Database, kind: &str, id: &str) -> Result<()> {
    validate_record_kind(kind)?;
    if kind == "issue" {
        db.require_issue(id)?;
    } else {
        db.require_record(kind, id)?;
    }
    Ok(())
}

fn refresh_projection(state_dir: &Path, db_path: &Path) -> Result<()> {
    atelier_app::projection::refresh_after_canonical_write(state_dir, db_path)
}

pub fn apply(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    input: &str,
    dry_run_override: bool,
    validate_only_override: bool,
) -> Result<()> {
    let bytes = fs::read(input).with_context(|| format!("Failed to read bulk plan {input}"))?;
    let plan: BulkPlan = serde_json::from_slice(&bytes)
        .with_context(|| format!("Failed to parse bulk plan {input} as JSON"))?;
    let options = plan.effective_options(dry_run_override, validate_only_override);
    validate_bulk_plan(db, &plan)?;

    if options.validate_only {
        print_apply_summary(json!({
            "applied": false,
            "dry_run": false,
            "validate_only": true,
            "records": {},
            "relationships": 0,
            "message": "bulk plan is valid"
        }))?;
        return Ok(());
    }

    if options.dry_run {
        let preview = dry_run_preview(&plan);
        print_apply_summary(preview)?;
        return Ok(());
    }

    let summary = apply_bulk_plan(db, state_dir, &plan)?;
    match options.export.as_str() {
        "auto" => refresh_projection(state_dir, db_path)?,
        "check_only" => {
            atelier_app::rebuild::validate_canonical_state(state_dir)?;
            refresh_projection(state_dir, db_path)?;
        }
        "skip" => refresh_projection(state_dir, db_path)?,
        other => bail!("Invalid export option '{other}'"),
    }

    print_apply_summary(summary)
}

fn print_record(db: &Database, record: &DomainRecord) -> Result<()> {
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
    Ok(atelier_records::normalized_plan_data(&record.data_json)?.revision)
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
    links: Vec<Value>,
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
    work: Vec<BulkRef>,
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
        validate_record_kind(kind)?;
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
        validate_refs_exist(db, plan, mission.work.iter(), &mission.client_ref)?;
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
    if !plan.links.is_empty() {
        bail!("Top-level bulk-plan links are no longer supported; use domain fields such as issue blocks/depends_on, mission plans/milestones, plan applies_to, or evidence validates");
    }
    Ok(())
}

fn apply_bulk_plan(db: &Database, state_dir: &Path, plan: &BulkPlan) -> Result<Value> {
    let mut resolved = BTreeMap::<String, ResolvedRef>::new();
    let mut created = BTreeMap::<String, Vec<Value>>::new();
    let store = RecordStore::new(state_dir);

    for issue in &plan.records.issues {
        let description = issue_description(issue);
        let now = chrono::Utc::now();
        let id = store.allocate_issue_id()?;
        let status = issue.status.as_deref().unwrap_or("open").to_string();
        let sections = IssueSections::unchecked_from_body(description.as_deref());
        let record = CanonicalIssueRecord {
            issue: Issue {
                id: id.clone(),
                title: issue.title.clone(),
                description,
                status: status.clone(),
                issue_type: issue.issue_type.clone(),
                priority: issue.priority.clone(),
                parent_id: None,
                created_at: now,
                updated_at: now,
                closed_at: (status == "closed").then_some(now),
            },
            labels: sorted(issue.labels.clone()),
            sections,
            relationships: Relationships::default(),
        };
        store.write_issue_atomic(&record)?;
        for note in &issue.notes {
            let body = match &note.author {
                Some(author) if !author.trim().is_empty() => format!("[{author}] {}", note.body),
                _ => note.body.clone(),
            };
            let _ = &note.created_at;
            super::activity_log::record_note(&id, &body)?;
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
        create_bulk_record(
            &store,
            &mut resolved,
            &mut created,
            &mission.client_ref,
            "mission",
            &mission.title,
            mission.body.as_deref(),
            json!({}),
            mission_labels(mission),
        )?;
    }
    for milestone in &plan.records.milestones {
        let data = MilestoneRecordData {
            desired_state: milestone.desired_state.clone(),
            scope: milestone.scope.clone(),
            validation_criteria: milestone.validation_criteria.clone(),
        };
        create_bulk_record(
            &store,
            &mut resolved,
            &mut created,
            &milestone.client_ref,
            "milestone",
            &milestone.title,
            Some(&milestone.desired_state),
            serde_json::to_value(data)?,
            Vec::new(),
        )?;
    }
    for record in &plan.records.plans {
        let data = PlanRecordData {
            revision: 1,
            owner: record.owner.clone(),
            revisions: vec![PlanRevision {
                revision: 1,
                reason: "bulk_apply".to_string(),
                body: record.body.clone(),
            }],
        };
        create_bulk_record(
            &store,
            &mut resolved,
            &mut created,
            &record.client_ref,
            "plan",
            &record.title,
            Some(&record.body),
            serde_json::to_value(data)?,
            Vec::new(),
        )?;
    }
    for evidence in &plan.records.evidence {
        let data = EvidenceRecordData {
            evidence_type: evidence.evidence_type.clone(),
            captured_at: chrono::Utc::now(),
            command: None,
            path: evidence.artifact.clone(),
            uri: None,
            producer: None,
            proof_scope: None,
            agent_identity: None,
            independence_level: None,
            residual_risks: Vec::new(),
            follow_up_ids: Vec::new(),
            exit_code: None,
            exit_status: Some(evidence.result.clone()),
            success: Some(evidence.result == "pass"),
            spawn_error: None,
            output: None,
            target: None,
        };
        create_bulk_record(
            &store,
            &mut resolved,
            &mut created,
            &evidence.client_ref,
            "evidence",
            &evidence.title,
            Some(&evidence.body),
            serde_json::to_value(data)?,
            Vec::new(),
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
            store.add_issue_child(&parent.id, &source.id)?;
        }
        for blocker in &issue.depends_on {
            let blocker = resolved_ref(db, plan, &resolved, blocker)?;
            if blocker.kind != "issue" {
                bail!(
                    "depends_on for {} must resolve to an issue",
                    issue.client_ref
                );
            }
            store.add_issue_block(&source.id, &blocker.id)?;
        }
        for blocked in &issue.blocks {
            let blocked = resolved_ref(db, plan, &resolved, blocked)?;
            if blocked.kind != "issue" {
                bail!("blocks for {} must resolve to an issue", issue.client_ref);
            }
            store.add_issue_block(&blocked.id, &source.id)?;
        }
    }

    let mut relationship_count = 0usize;
    for mission in &plan.records.missions {
        let source = resolved_ref(db, plan, &resolved, &BulkRef::client(&mission.client_ref))?;
        for target in &mission.work {
            add_resolved_link(db, &store, plan, &resolved, &source, target, "advances")?;
            relationship_count += 1;
        }
        for target in &mission.plans {
            add_resolved_link(db, &store, plan, &resolved, &source, target, "planned_by")?;
            relationship_count += 1;
        }
        for target in &mission.milestones {
            add_resolved_link(
                db,
                &store,
                plan,
                &resolved,
                &source,
                target,
                "has_checkpoint",
            )?;
            relationship_count += 1;
        }
    }
    for milestone in &plan.records.milestones {
        let source = resolved_ref(db, plan, &resolved, &BulkRef::client(&milestone.client_ref))?;
        for target in &milestone.missions {
            add_resolved_link(db, &store, plan, &resolved, &source, target, "advances")?;
            relationship_count += 1;
        }
        for target in &milestone.contributing_work {
            let contributor = resolved_ref(db, plan, &resolved, target)?;
            add_relationship(&store, &contributor, &source, "contributes_to")?;
            relationship_count += 1;
        }
    }
    for record in &plan.records.plans {
        let source = resolved_ref(db, plan, &resolved, &BulkRef::client(&record.client_ref))?;
        for target in &record.applies_to {
            add_resolved_link(db, &store, plan, &resolved, &source, target, "planned_by")?;
            relationship_count += 1;
        }
        for target in &record.supersedes {
            add_resolved_link(db, &store, plan, &resolved, &source, target, "supersedes")?;
            relationship_count += 1;
        }
    }
    for evidence in &plan.records.evidence {
        let source = resolved_ref(db, plan, &resolved, &BulkRef::client(&evidence.client_ref))?;
        for target in &evidence.validates {
            add_resolved_link(db, &store, plan, &resolved, &source, target, "validates")?;
            relationship_count += 1;
        }
    }

    Ok(json!({
        "applied": true,
        "dry_run": false,
        "validate_only": false,
        "title": plan.title,
        "description": plan.description,
        "records": created,
        "relationships": relationship_count,
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
    if let Some(issue_id) = db.resolve_issue_ref(id)? {
        return Ok(ResolvedRef {
            kind: "issue".to_string(),
            id: issue_id,
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
    store: &RecordStore,
    resolved: &mut BTreeMap<String, ResolvedRef>,
    created: &mut BTreeMap<String, Vec<Value>>,
    client_ref: &str,
    kind: &str,
    title: &str,
    body: Option<&str>,
    data: Value,
    labels: Vec<String>,
) -> Result<()> {
    let status = if kind == "mission" { "ready" } else { "open" };
    let mut record =
        store.create_domain_record(kind, title, status, body, &serde_json::to_string(&data)?)?;
    if !labels.is_empty() {
        record.labels = labels;
        store.write_domain_record_atomic(&record)?;
    }
    let id = record.record.id;
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
    store: &RecordStore,
    plan: &BulkPlan,
    resolved: &BTreeMap<String, ResolvedRef>,
    source: &ResolvedRef,
    target: &BulkRef,
    relation_type: &str,
) -> Result<()> {
    let target = resolved_ref(db, plan, resolved, target)?;
    add_relationship(store, source, &target, relation_type)?;
    Ok(())
}

fn add_relationship(
    store: &RecordStore,
    source: &ResolvedRef,
    target: &ResolvedRef,
    relation_type: &str,
) -> Result<()> {
    store.add_record_relationship(
        &source.kind,
        &source.id,
        &target.kind,
        &target.id,
        relation_type,
    )?;
    Ok(())
}

fn issue_description(issue: &BulkIssue) -> Option<String> {
    let description = issue
        .description
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("Imported bulk-plan issue.");
    let outcome = if issue.acceptance.is_empty() {
        "Outcome was not specified in the bulk plan.".to_string()
    } else {
        bullet_list(&issue.acceptance)
    };
    let evidence = if issue.evidence_required.is_empty() {
        "Evidence was not specified in the bulk plan.".to_string()
    } else {
        bullet_list(&issue.evidence_required)
    };
    Some(format!(
        "## Description\n\n{description}\n\n## Outcome\n\n{outcome}\n\n## Evidence\n\n{evidence}"
    ))
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
        "relationships": 0,
    })
}

fn print_apply_summary(summary: Value) -> Result<()> {
    let validate_only = summary["validate_only"].as_bool().unwrap_or(false);
    let dry_run = summary["dry_run"].as_bool().unwrap_or(false);
    if validate_only {
        println!("Bulk plan is valid.");
    } else if dry_run {
        println!("Bulk plan preview is valid.");
    } else {
        println!("Bulk plan applied.");
    }

    println!(
        "Applied:       {}",
        summary["applied"].as_bool().unwrap_or(false)
    );
    println!("Dry run:       {dry_run}");
    println!("Validate only: {validate_only}");

    if let Some(records) = summary["records"].as_object() {
        println!();
        println!("Records");
        println!("-------");
        for key in ["issues", "missions", "milestones", "plans", "evidence"] {
            let count = records
                .get(key)
                .and_then(|value| value.as_array())
                .map(|items| items.len())
                .unwrap_or(0);
            println!("  {key}: {count}");
        }
    }

    let relationship_count = summary["relationships"]
        .as_array()
        .map(|relationships| relationships.len())
        .or_else(|| {
            summary["relationships"]
                .as_u64()
                .map(|count| count as usize)
        })
        .unwrap_or(0);
    println!("  relationships: {relationship_count}");

    if let Some(id) = first_created_id(&summary, "missions") {
        println!();
        println!("Next Commands");
        println!("-------------");
        println!("  atelier mission show {id}");
        println!("  atelier lint");
        println!("  atelier doctor");
    } else if !validate_only && !dry_run {
        println!();
        println!("Next Commands");
        println!("-------------");
        println!("  atelier lint");
        println!("  atelier doctor");
    }
    Ok(())
}

fn first_created_id<'a>(summary: &'a Value, kind: &str) -> Option<&'a str> {
    summary["records"][kind]
        .as_array()
        .and_then(|items| items.first())
        .and_then(|item| item["id"].as_str())
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

fn mission_labels(mission: &BulkMission) -> Vec<String> {
    let mut labels = mission.labels.clone();
    labels.push("mission".to_string());
    sorted(labels)
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

fn canonical_record_detail(kind: &str, id: &str) -> Result<Option<DomainRecord>> {
    let Some(state_dir) = find_state_dir_from_cwd()? else {
        return Ok(None);
    };
    let store = RecordStore::new(state_dir);
    Ok(Some(store.load_domain_record_by_id(kind, id)?.record))
}

fn find_state_dir_from_cwd() -> Result<Option<PathBuf>> {
    atelier_app::storage_layout::find_canonical_dir_from_cwd()
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
