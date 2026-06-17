use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use atelier_core::{EvidenceRecordData, Issue, PlanRecord, PlanRecordData, PlanRevision, Record};
use atelier_records::activity::{create_issue_activity, ActivityEventType};
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
    let created = store.create_plan(title, "open", body.unwrap_or(""), data)?;
    refresh_projection(state_dir, db_path)?;
    let db = Database::open(db_path)?;
    print_record(&db, &created)
}

pub fn show(db: &Database, id: &str) -> Result<()> {
    db.require_record(KIND, id)?;
    let record = canonical_plan_record(id)?;
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
    let mut current = match store.load_record_by_id(KIND, id)? {
        Record::Plan(record) => record,
        other => bail!("Expected plan record {id}, found {}", other.kind()),
    };
    let mut data = atelier_records::normalized_plan_data(current.data.clone());
    let next_revision = data.revision + 1;
    data.revision = next_revision;
    data.revisions.push(PlanRevision {
        revision: next_revision,
        reason: reason.unwrap_or("revision").to_string(),
        body: body.to_string(),
    });
    current.body = body.to_string();
    current.data = data;
    current.header.updated_at = chrono::Utc::now();
    store.write_record_atomic(&Record::Plan(current.clone()))?;
    refresh_projection(state_dir, db_path)?;
    let db = Database::open(db_path)?;
    print_record(&db, &current)
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

pub fn preview_bundle(db: &Database, input: &str) -> Result<()> {
    let bundle = load_bundle(input)?;
    validate_bundle(db, &bundle)?;
    print_bundle_summary(preview_bundle_summary(&bundle))
}

pub fn apply_bundle(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    input: &str,
    yes: bool,
) -> Result<()> {
    if !yes {
        bail!("bundle apply requires --yes");
    }
    let bundle = load_bundle(input)?;
    validate_bundle(db, &bundle)?;

    let summary = apply_bundle_file(db, state_dir, &bundle)?;
    refresh_projection(state_dir, db_path)?;

    print_bundle_summary(summary)
}

fn print_record(db: &Database, record: &PlanRecord) -> Result<()> {
    println!(
        "{} [plan] {} - {}",
        record.header.id, record.header.status, record.header.title
    );
    println!(
        "{}",
        "=".repeat(
            record.header.id.len() + record.header.status.len() + record.header.title.len() + 11
        )
    );
    println!("Status:   {}", record.header.status);
    println!("Revision: {}", record.data.revision);
    println!("Created:  {}", record.header.created_at.to_rfc3339());
    println!("Updated:  {}", record.header.updated_at.to_rfc3339());
    let links = db.list_record_links(KIND, &record.header.id)?;
    println!("Links:    {}", links.len());
    print_heading("Body");
    if !record.body.is_empty() {
        println!("{}", record.body);
    } else {
        println!("(none)");
    }
    if !links.is_empty() {
        print_heading("Links");
        for link in links {
            let (kind, id) = if link.source_kind == KIND && link.source_id == record.header.id {
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

fn load_bundle(input: &str) -> Result<BundleFile> {
    let bytes = fs::read(input).with_context(|| format!("Failed to read bundle file {input}"))?;
    serde_json::from_slice(&bytes)
        .with_context(|| format!("Failed to parse bundle {input} as JSON"))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleFile {
    schema: String,
    schema_version: i64,
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(rename = "metadata", default)]
    _metadata: Value,
    #[serde(default)]
    resources: BundleResources,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleResources {
    #[serde(default)]
    issues: Vec<BundleIssue>,
    #[serde(default)]
    missions: Vec<BundleMission>,
    #[serde(default)]
    evidence: Vec<BundleEvidence>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleIssue {
    client_ref: String,
    title: String,
    #[serde(default)]
    operation: Option<String>,
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
    parent: Option<BundleRef>,
    #[serde(default)]
    depends_on: Vec<BundleRef>,
    #[serde(default)]
    blocks: Vec<BundleRef>,
    #[serde(default)]
    notes: Vec<BundleNote>,
    #[serde(default)]
    outcome: Vec<String>,
    #[serde(default)]
    evidence: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleMission {
    client_ref: String,
    title: String,
    #[serde(default)]
    operation: Option<String>,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    work: Vec<BundleRef>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleEvidence {
    client_ref: String,
    title: String,
    #[serde(default)]
    operation: Option<String>,
    evidence_type: String,
    result: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    validates: Vec<BundleRef>,
    #[serde(default)]
    artifact: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleRef {
    #[serde(default)]
    client_ref: Option<String>,
    #[serde(default)]
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleNote {
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

fn validate_bundle(db: &Database, bundle: &BundleFile) -> Result<()> {
    if bundle.schema != "atelier.bundle" {
        bail!("Invalid bundle schema '{}'", bundle.schema);
    }
    if bundle.schema_version != 1 {
        bail!(
            "Unsupported bundle schema_version {}",
            bundle.schema_version
        );
    }
    if bundle.title.trim().is_empty() {
        bail!("Bundle title cannot be empty");
    }

    let mut refs = BTreeSet::new();
    for (client_ref, kind) in bundle.client_refs() {
        validate_client_ref(&client_ref)?;
        if !refs.insert(client_ref.clone()) {
            bail!("Duplicate client_ref '{}'", client_ref);
        }
        if kind == "issue" {
            continue;
        }
        validate_record_kind(kind)?;
    }

    for issue in &bundle.resources.issues {
        validate_operation(issue.operation.as_deref(), &issue.client_ref)?;
        if issue.title.trim().is_empty() {
            bail!("Issue {} title cannot be empty", issue.client_ref);
        }
        validate_issue_type(&issue.issue_type)?;
        validate_priority(&issue.priority)?;
        if let Some(status) = &issue.status {
            validate_bundle_issue_status(status)?;
        }
        validate_refs_exist(db, bundle, issue.parent.iter(), &issue.client_ref)?;
        validate_refs_exist(db, bundle, issue.depends_on.iter(), &issue.client_ref)?;
        validate_refs_exist(db, bundle, issue.blocks.iter(), &issue.client_ref)?;
    }
    for mission in &bundle.resources.missions {
        validate_operation(mission.operation.as_deref(), &mission.client_ref)?;
        if mission.title.trim().is_empty() {
            bail!("Mission {} title cannot be empty", mission.client_ref);
        }
        validate_refs_exist(db, bundle, mission.work.iter(), &mission.client_ref)?;
    }
    for evidence in &bundle.resources.evidence {
        validate_operation(evidence.operation.as_deref(), &evidence.client_ref)?;
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
        validate_refs_exist(db, bundle, evidence.validates.iter(), &evidence.client_ref)?;
    }
    Ok(())
}

fn validate_operation(operation: Option<&str>, client_ref: &str) -> Result<()> {
    match operation.unwrap_or("create") {
        "create" => Ok(()),
        other => {
            bail!("Unsupported operation '{other}' for {client_ref}; v1 bundles are create-only")
        }
    }
}

fn validate_bundle_issue_status(status: &str) -> Result<()> {
    validate_status(status)?;
    let repo_root = atelier_app::storage_layout::find_repo_root()?;
    let policy_path = repo_root.join(atelier_app::workflow_policy::WORKFLOW_POLICY_PATH);
    if !policy_path.exists() {
        return Ok(());
    }
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    if policy.statuses.contains_key(status) {
        Ok(())
    } else {
        bail!(
            "Invalid bundle issue status '{status}'; status is not defined in .atelier/workflow.yaml"
        )
    }
}

fn workflow_initial_issue_status(issue_type: &str) -> Result<String> {
    let repo_root = atelier_app::storage_layout::find_repo_root()?;
    Ok(
        atelier_app::workflow_policy::configured_initial_status(&repo_root, issue_type)?
            .unwrap_or_else(|| "todo".to_string()),
    )
}

fn apply_bundle_file(db: &Database, state_dir: &Path, bundle: &BundleFile) -> Result<Value> {
    let stage_parent = state_dir
        .parent()
        .with_context(|| format!("Cannot determine parent for {}", state_dir.display()))?;
    let stage = create_bundle_stage_dir(stage_parent)?;
    let result = (|| {
        copy_state_tree(state_dir, &stage)?;
        let summary = apply_bundle_to_state(db, &stage, bundle)?;
        install_bundle_stage(&stage, state_dir)?;
        Ok(summary)
    })();
    let _ = fs::remove_dir_all(&stage);
    result
}

fn create_bundle_stage_dir(parent: &Path) -> Result<PathBuf> {
    let base = format!(
        ".atelier-bundle-stage-{}-{}",
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    );
    for suffix in 0..=99 {
        let candidate = if suffix == 0 {
            parent.join(&base)
        } else {
            parent.join(format!("{base}-{suffix:02}"))
        };
        match fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("Failed to create {}", candidate.display()))
            }
        }
    }
    bail!(
        "Failed to allocate bundle staging directory in {}",
        parent.display()
    )
}

fn apply_bundle_to_state(db: &Database, state_dir: &Path, bundle: &BundleFile) -> Result<Value> {
    let mut resolved = BTreeMap::<String, ResolvedRef>::new();
    let mut created = BTreeMap::<String, Vec<Value>>::new();
    let store = RecordStore::new(state_dir);

    for issue in &bundle.resources.issues {
        let description = issue_description(issue);
        let now = chrono::Utc::now();
        let id = store.allocate_issue_id()?;
        let status = match &issue.status {
            Some(status) => status.clone(),
            None => workflow_initial_issue_status(&issue.issue_type)?,
        };
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
            create_issue_activity(
                state_dir,
                &id,
                ActivityEventType::Note,
                &current_actor(),
                chrono::Utc::now(),
                "Added note",
                &body,
            )?;
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

    for mission in &bundle.resources.missions {
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
    for evidence in &bundle.resources.evidence {
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

    for issue in &bundle.resources.issues {
        let source = resolved_ref(db, bundle, &resolved, &BundleRef::client(&issue.client_ref))?;
        if let Some(parent) = &issue.parent {
            let parent = resolved_ref(db, bundle, &resolved, parent)?;
            if parent.kind != "issue" {
                bail!(
                    "Issue parent for {} must resolve to an issue",
                    issue.client_ref
                );
            }
            store.add_issue_child(&parent.id, &source.id)?;
        }
        for blocker in &issue.depends_on {
            let blocker = resolved_ref(db, bundle, &resolved, blocker)?;
            if blocker.kind != "issue" {
                bail!(
                    "depends_on for {} must resolve to an issue",
                    issue.client_ref
                );
            }
            store.add_issue_block(&source.id, &blocker.id)?;
        }
        for blocked in &issue.blocks {
            let blocked = resolved_ref(db, bundle, &resolved, blocked)?;
            if blocked.kind != "issue" {
                bail!("blocks for {} must resolve to an issue", issue.client_ref);
            }
            store.add_issue_block(&blocked.id, &source.id)?;
        }
    }

    let mut relationship_count = 0usize;
    for mission in &bundle.resources.missions {
        let source = resolved_ref(
            db,
            bundle,
            &resolved,
            &BundleRef::client(&mission.client_ref),
        )?;
        for target in &mission.work {
            add_resolved_link(db, &store, bundle, &resolved, &source, target, "advances")?;
            relationship_count += 1;
        }
    }
    for evidence in &bundle.resources.evidence {
        let source = resolved_ref(
            db,
            bundle,
            &resolved,
            &BundleRef::client(&evidence.client_ref),
        )?;
        for target in &evidence.validates {
            add_resolved_link(db, &store, bundle, &resolved, &source, target, "validates")?;
            relationship_count += 1;
        }
    }

    Ok(json!({
        "applied": true,
        "preview": false,
        "title": bundle.title,
        "description": bundle.description,
        "records": created,
        "relationships": relationship_count,
    }))
}

fn copy_state_tree(source: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest).with_context(|| format!("Failed to create {}", dest.display()))?;
    for entry in
        fs::read_dir(source).with_context(|| format!("Failed to read {}", source.display()))?
    {
        let entry = entry?;
        let name = entry.file_name();
        if matches!(
            name.to_str(),
            Some("runtime" | "cache" | "locks" | "diagnostics")
        ) {
            continue;
        }
        let source_path = entry.path();
        let dest_path = dest.join(&name);
        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &dest_path)?;
        } else {
            fs::copy(&source_path, &dest_path).with_context(|| {
                format!(
                    "Failed to copy {} to {}",
                    source_path.display(),
                    dest_path.display()
                )
            })?;
        }
    }
    Ok(())
}

fn copy_dir_recursive(source: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest).with_context(|| format!("Failed to create {}", dest.display()))?;
    for entry in
        fs::read_dir(source).with_context(|| format!("Failed to read {}", source.display()))?
    {
        let entry = entry?;
        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &dest_path)?;
        } else {
            fs::copy(&source_path, &dest_path).with_context(|| {
                format!(
                    "Failed to copy {} to {}",
                    source_path.display(),
                    dest_path.display()
                )
            })?;
        }
    }
    Ok(())
}

fn install_bundle_stage(stage: &Path, state_dir: &Path) -> Result<()> {
    for name in ["issues", "missions", "evidence"] {
        replace_dir_from_stage(&stage.join(name), &state_dir.join(name))?;
    }
    Ok(())
}

fn replace_dir_from_stage(staged: &Path, dest: &Path) -> Result<()> {
    let backup = dest.with_extension("bundle-backup");
    if backup.exists() {
        fs::remove_dir_all(&backup)
            .with_context(|| format!("Failed to remove {}", backup.display()))?;
    }
    if dest.exists() {
        fs::rename(dest, &backup).with_context(|| {
            format!("Failed to move {} to {}", dest.display(), backup.display())
        })?;
    }
    let result = fs::rename(staged, dest)
        .with_context(|| format!("Failed to install staged {}", dest.display()));
    if let Err(err) = result {
        if backup.exists() && !dest.exists() {
            let _ = fs::rename(&backup, dest);
        }
        return Err(err);
    }
    if backup.exists() {
        fs::remove_dir_all(&backup)
            .with_context(|| format!("Failed to remove {}", backup.display()))?;
    }
    Ok(())
}

fn current_actor() -> String {
    std::env::var("ATELIER_AGENT")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "agent".to_string())
}

impl BundleFile {
    fn client_refs(&self) -> Vec<(String, &'static str)> {
        let mut refs = Vec::new();
        refs.extend(
            self.resources
                .issues
                .iter()
                .map(|record| (record.client_ref.clone(), "issue")),
        );
        refs.extend(
            self.resources
                .missions
                .iter()
                .map(|record| (record.client_ref.clone(), "mission")),
        );
        refs.extend(
            self.resources
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

impl BundleRef {
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
    bundle: &BundleFile,
    refs: impl Iterator<Item = &'a BundleRef>,
    owner: &str,
) -> Result<()> {
    for reference in refs {
        validate_ref_exists(db, bundle, reference, owner)?;
    }
    Ok(())
}

fn validate_ref_exists(
    db: &Database,
    bundle: &BundleFile,
    reference: &BundleRef,
    owner: &str,
) -> Result<()> {
    match (&reference.client_ref, &reference.id) {
        (Some(client_ref), None) => {
            if bundle.client_kind(client_ref).is_none() {
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
    for kind in ["mission", "evidence", "workflow_validator"] {
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
    _bundle: &BundleFile,
    resolved: &BTreeMap<String, ResolvedRef>,
    reference: &BundleRef,
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
    let mut record = match kind {
        "mission" => Record::Mission(store.create_mission(
            title,
            status,
            atelier_records::mission_sections_from_inputs(
                title,
                body,
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
        )?),
        "evidence" => Record::Evidence(store.create_evidence(
            title,
            "recorded",
            body.unwrap_or(title),
            serde_json::from_value::<EvidenceRecordData>(data)?,
        )?),
        other => bail!("Unsupported bundle record kind '{other}'"),
    };
    if !labels.is_empty() {
        record.header_mut().labels = labels;
        store.write_record_atomic(&record)?;
    }
    let id = record.header().id.clone();
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
    bundle: &BundleFile,
    resolved: &BTreeMap<String, ResolvedRef>,
    source: &ResolvedRef,
    target: &BundleRef,
    relation_type: &str,
) -> Result<()> {
    let target = resolved_ref(db, bundle, resolved, target)?;
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

fn issue_description(issue: &BundleIssue) -> Option<String> {
    let description = issue
        .description
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("Imported bundle issue.");
    let outcome = if issue.outcome.is_empty() {
        "Outcome was not specified in the bundle.".to_string()
    } else {
        bullet_list(&issue.outcome)
    };
    let evidence = if issue.evidence.is_empty() {
        "Evidence was not specified in the bundle.".to_string()
    } else {
        bullet_list(&issue.evidence)
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

fn preview_bundle_summary(bundle: &BundleFile) -> Value {
    let mut records = BTreeMap::<String, Vec<Value>>::new();
    for (client_ref, kind) in bundle.client_refs() {
        records
            .entry(format!("{kind}s"))
            .or_default()
            .push(json!({ "client_ref": client_ref, "kind": kind }));
    }
    json!({
        "applied": false,
        "preview": true,
        "title": bundle.title,
        "description": bundle.description,
        "records": records,
        "relationships": 0,
    })
}

fn print_bundle_summary(summary: Value) -> Result<()> {
    let preview = summary["preview"].as_bool().unwrap_or(false);
    if preview {
        println!("Bundle preview is valid.");
    } else {
        println!("Bundle applied.");
    }

    println!(
        "Applied:       {}",
        summary["applied"].as_bool().unwrap_or(false)
    );
    println!("Preview:       {preview}");

    if let Some(records) = summary["records"].as_object() {
        println!();
        println!("Records");
        println!("-------");
        for key in ["issues", "missions", "evidence"] {
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
    } else if !preview {
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

fn mission_labels(mission: &BundleMission) -> Vec<String> {
    let mut labels = mission.labels.clone();
    labels.push("mission".to_string());
    sorted(labels)
}

fn created_key(kind: &str) -> &str {
    match kind {
        "evidence" => "evidence",
        "mission" => "missions",
        "issue" => "issues",
        _ => kind,
    }
}

fn canonical_plan_record(id: &str) -> Result<PlanRecord> {
    let Some(state_dir) = find_state_dir_from_cwd()? else {
        bail!("Cannot locate canonical Atelier state directory");
    };
    let store = RecordStore::new(state_dir);
    match store.load_record_by_id(KIND, id)? {
        Record::Plan(record) => Ok(record),
        other => bail!("Expected plan record {id}, found {}", other.kind()),
    }
}

fn find_state_dir_from_cwd() -> Result<Option<PathBuf>> {
    atelier_app::storage_layout::find_canonical_dir_from_cwd()
}

fn default_issue_type() -> String {
    "task".to_string()
}

fn default_priority() -> String {
    "medium".to_string()
}
