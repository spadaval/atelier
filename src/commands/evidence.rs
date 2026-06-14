use anyhow::{bail, Result};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::db::Database;
use crate::models::{
    DomainRecord, EvidenceOutputSummary, EvidenceRecordData, EvidenceStreamSummary, EvidenceTarget,
    RecordLink,
};
use crate::record_store::RecordStore;

const KIND: &str = "evidence";
const OUTPUT_SUMMARY_LIMIT_BYTES: usize = 4096;

pub struct CaptureOptions<'a> {
    pub evidence_kind: &'a str,
    pub result: &'a str,
    pub summary: Option<&'a str>,
    pub path: Option<&'a str>,
    pub uri: Option<&'a str>,
    pub producer: Option<&'a str>,
    pub target_kind: Option<&'a str>,
    pub target_id: Option<&'a str>,
    pub role: &'a str,
    pub command: &'a [String],
}

#[derive(Debug, Clone)]
pub struct TargetMetadata<'a> {
    pub kind: &'a str,
    pub id: &'a str,
    pub role: &'a str,
}

#[derive(Debug, Clone)]
struct EvidenceMetadata<'a> {
    proof_scope: &'a str,
    agent_identity: Option<&'a str>,
    independence_level: &'a str,
    residual_risks: Vec<String>,
    follow_up_ids: Vec<String>,
}

impl<'a> EvidenceMetadata<'a> {
    fn from_producer(producer: Option<&'a str>) -> Self {
        Self {
            proof_scope: "scoped to the attached target or summary",
            agent_identity: producer,
            independence_level: "unspecified",
            residual_risks: Vec::new(),
            follow_up_ids: Vec::new(),
        }
    }
}

pub fn add_returning_id(
    state_dir: &Path,
    db_path: &Path,
    evidence_kind: &str,
    result: &str,
    summary: &str,
    path: Option<&str>,
    uri: Option<&str>,
    producer: Option<&str>,
    target: Option<TargetMetadata<'_>>,
) -> Result<String> {
    let metadata = EvidenceMetadata::from_producer(producer);
    let data = EvidenceRecordData {
        evidence_type: evidence_kind.to_string(),
        captured_at: chrono::Utc::now(),
        command: None,
        path: path.map(str::to_string),
        uri: uri.map(str::to_string),
        producer: producer.map(str::to_string),
        proof_scope: Some(metadata.proof_scope.to_string()),
        agent_identity: metadata.agent_identity.map(str::to_string),
        independence_level: Some(metadata.independence_level.to_string()),
        residual_risks: metadata.residual_risks,
        follow_up_ids: metadata.follow_up_ids,
        exit_code: None,
        exit_status: None,
        success: None,
        spawn_error: None,
        output: None,
        target: target.as_ref().map(|target| EvidenceTarget {
            kind: target.kind.to_string(),
            id: target.id.to_string(),
            role: target.role.to_string(),
        }),
    };
    let store = RecordStore::new(state_dir);
    let created = store.create_domain_record(
        KIND,
        summary,
        result,
        Some(summary),
        &serde_json::to_string(&data)?,
    )?;
    let id = created.record.id.clone();
    refresh_projection(state_dir, db_path)?;
    Ok(id)
}

pub fn capture(state_dir: &Path, db_path: &Path, options: CaptureOptions<'_>) -> Result<()> {
    if options.command.is_empty() {
        bail!("evidence capture requires a command after --");
    }

    let target = capture_target(
        db_path,
        options.target_kind,
        options.target_id,
        options.role,
    )?;
    let command_display = format_command(options.command);
    let captured_at = chrono::Utc::now().to_rfc3339();
    let command_output = Command::new(&options.command[0])
        .args(&options.command[1..])
        .output();

    let (exit_code, exit_status, success, stdout_summary, stderr_summary, spawn_error) =
        match command_output {
            Ok(output) => command_result_metadata(&output),
            Err(error) => (
                None,
                "not-started".to_string(),
                false,
                BoundedText::empty(),
                BoundedText::from_text(&error.to_string(), OUTPUT_SUMMARY_LIMIT_BYTES),
                Some(error.to_string()),
            ),
        };

    if options.result == "pass" && !success {
        bail!(
            "cannot record pass evidence for command exit status {}; use --result fail or --result blocked",
            exit_status
        );
    }

    let summary = options
        .summary
        .map(str::to_string)
        .unwrap_or_else(|| command_display.clone());
    let metadata = EvidenceMetadata::from_producer(options.producer);
    let body = command_capture_body(
        &summary,
        &command_display,
        &exit_status,
        &stdout_summary,
        &stderr_summary,
        spawn_error.as_deref(),
    );
    let data = EvidenceRecordData {
        evidence_type: options.evidence_kind.to_string(),
        captured_at: chrono::DateTime::parse_from_rfc3339(&captured_at)?
            .with_timezone(&chrono::Utc),
        command: Some(command_display.clone()),
        path: options.path.map(str::to_string),
        uri: options.uri.map(str::to_string),
        producer: options.producer.map(str::to_string),
        proof_scope: Some(metadata.proof_scope.to_string()),
        agent_identity: metadata.agent_identity.map(str::to_string),
        independence_level: Some(metadata.independence_level.to_string()),
        residual_risks: metadata.residual_risks,
        follow_up_ids: metadata.follow_up_ids,
        exit_code,
        exit_status: Some(exit_status.clone()),
        success: Some(success),
        spawn_error,
        output: Some(EvidenceOutputSummary {
            limit_bytes_per_stream: OUTPUT_SUMMARY_LIMIT_BYTES,
            stdout: stdout_summary.to_stream_summary(),
            stderr: stderr_summary.to_stream_summary(),
        }),
        target: target.as_ref().map(|target| EvidenceTarget {
            kind: target.display_kind.to_string(),
            id: target.id.to_string(),
            role: target.role.to_string(),
        }),
    };

    let store = RecordStore::new(state_dir);
    let created = store.create_domain_record(
        KIND,
        &summary,
        options.result,
        Some(&body),
        &serde_json::to_string(&data)?,
    )?;
    refresh_projection(state_dir, db_path)?;
    if let Some(target) = target {
        attach(
            state_dir,
            db_path,
            &created.record.id,
            &target.display_kind,
            &target.id,
            &target.role,
        )?;
    }
    let db = Database::open(db_path)?;
    let record = db.require_record(KIND, &created.record.id)?;
    print_record(&db, &record)
}

pub fn show(db: &Database, id: &str) -> Result<()> {
    let record = canonical_record_detail(KIND, id)?.unwrap_or(db.require_record(KIND, id)?);
    print_record(db, &record)
}

pub fn attach(
    state_dir: &Path,
    db_path: &Path,
    id: &str,
    target_kind: &str,
    target_id: &str,
    role: &str,
) -> Result<()> {
    let db = Database::open(db_path)?;
    db.require_record(KIND, id)?;
    let target = validate_record_ref(&db, target_kind, target_id, role)?;
    drop(db);
    let store = RecordStore::new(state_dir);
    let inserted =
        store.add_attachment_relationship(KIND, id, &target.canonical_kind, target_id, role)?;
    refresh_projection(state_dir, db_path)?;
    if inserted && target.canonical_kind == "issue" {
        let db = Database::open(db_path)?;
        let evidence = db.require_record(KIND, id)?;
        super::activity_log::record_evidence_attached(target_id, id, Some(&evidence.status))?;
    }
    if inserted {
        println!(
            "Attached evidence {id} to {} {target_id} ({role})",
            target.display_kind
        );
    } else {
        println!(
            "Evidence {id} is already attached to {} {target_id} ({role})",
            target.display_kind
        );
    }
    Ok(())
}

fn validate_record_ref<'a>(
    db: &Database,
    kind: &'a str,
    id: &'a str,
    role: &'a str,
) -> Result<TargetRef<'a>> {
    if kind == "epic" {
        let issue = db.require_issue(id)?;
        if issue.issue_type != "epic" {
            bail!("{id} is not an epic issue");
        }
        return Ok(TargetRef {
            display_kind: kind,
            canonical_kind: "issue",
            id,
            role,
        });
    }

    crate::db::validate_record_kind(kind)?;
    if kind == "issue" {
        db.require_issue(id)?;
    } else {
        db.require_record(kind, id)?;
    }
    Ok(TargetRef {
        display_kind: kind,
        canonical_kind: kind,
        id,
        role,
    })
}

fn refresh_projection(state_dir: &Path, db_path: &Path) -> Result<()> {
    super::projection::refresh_after_canonical_write(state_dir, db_path)
}

pub fn list(db: &Database, result: Option<&str>) -> Result<()> {
    let records = db.list_records(KIND, result)?;
    if records.is_empty() {
        print_heading("Evidence");
        println!("(none)");
        return Ok(());
    }
    print_heading("Evidence");
    println!("{} total", records.len());
    for record in records {
        let data = evidence_record_data(&record)?;
        let kind = data.evidence_type.as_str();
        let command = data.command.as_deref().unwrap_or("(manual)");
        let exit_status = data.exit_status.as_deref().unwrap_or("(none)");
        let targets = format_targets(db, &record.id, &data)?;
        let target = if targets.is_empty() {
            "(none)".to_string()
        } else {
            targets.join(", ")
        };
        println!(
            "  {:<14} {:<13} {:<10} exit={} target={} command={} {}",
            record.id, record.status, kind, exit_status, target, command, record.title
        );
    }
    Ok(())
}

pub fn print_record(db: &Database, record: &DomainRecord) -> Result<()> {
    let data = evidence_record_data(record)?;
    println!(
        "{} [evidence] {} - {}",
        record.id, record.status, record.title
    );
    println!(
        "{}",
        "=".repeat(record.id.len() + record.status.len() + record.title.len() + 15)
    );
    println!("Result:      {}", record.status);
    println!("Kind:        {}", data.evidence_type);
    println!("Captured:    {}", data.captured_at.to_rfc3339());
    if let Some(command) = data.command.as_deref() {
        println!("Command:     {command}");
    }
    if let Some(exit_status) = data.exit_status.as_deref() {
        println!("Exit Status: {exit_status}");
    }
    let targets = format_targets(db, &record.id, &data)?;
    if !targets.is_empty() {
        println!("Target:      {}", targets.join(", "));
    }
    println!(
        "Producer:    {}",
        data.producer
            .as_deref()
            .or(data.agent_identity.as_deref())
            .unwrap_or("(none)")
    );
    println!("Path:        {}", data.path.as_deref().unwrap_or("(none)"));
    println!("URI:         {}", data.uri.as_deref().unwrap_or("(none)"));
    println!("Created:     {}", record.created_at.to_rfc3339());
    println!("Updated:     {}", record.updated_at.to_rfc3339());
    print_heading("Summary");
    if let Some(summary) = &record.body {
        if summary.is_empty() {
            println!("(none)");
        } else {
            println!("{summary}");
        }
    } else {
        println!("(none)");
    }
    print_output_summary(&data)?;
    Ok(())
}

fn print_heading(title: &str) {
    println!("{title}");
    println!("{}", "-".repeat(title.len()));
}

fn evidence_record_data(record: &DomainRecord) -> Result<EvidenceRecordData> {
    let mut data = serde_json::from_str::<EvidenceRecordData>(&record.data_json)?;
    if data.agent_identity.is_none() {
        data.agent_identity = data.producer.clone();
    }
    Ok(data)
}

fn capture_target<'a>(
    db_path: &Path,
    target_kind: Option<&'a str>,
    target_id: Option<&'a str>,
    role: &'a str,
) -> Result<Option<TargetRef<'a>>> {
    match (target_kind, target_id) {
        (Some(kind), Some(id)) => {
            let db = Database::open(db_path)?;
            Ok(Some(validate_record_ref(&db, kind, id, role)?))
        }
        (None, None) => Ok(None),
        _ => bail!("--target-kind and --target-id must be supplied together"),
    }
}

fn command_result_metadata(
    output: &Output,
) -> (
    Option<i32>,
    String,
    bool,
    BoundedText,
    BoundedText,
    Option<String>,
) {
    (
        output.status.code(),
        exit_status_text(output),
        output.status.success(),
        BoundedText::from_bytes(&output.stdout, OUTPUT_SUMMARY_LIMIT_BYTES),
        BoundedText::from_bytes(&output.stderr, OUTPUT_SUMMARY_LIMIT_BYTES),
        None,
    )
}

fn exit_status_text(output: &Output) -> String {
    match output.status.code() {
        Some(code) => code.to_string(),
        None => output.status.to_string(),
    }
}

#[derive(Debug)]
struct BoundedText {
    text: String,
    original_bytes: usize,
    truncated: bool,
}

impl BoundedText {
    fn empty() -> Self {
        Self {
            text: String::new(),
            original_bytes: 0,
            truncated: false,
        }
    }

    fn from_text(text: &str, limit: usize) -> Self {
        Self::from_bytes(text.as_bytes(), limit)
    }

    fn from_bytes(bytes: &[u8], limit: usize) -> Self {
        let original_bytes = bytes.len();
        let truncated = original_bytes > limit;
        let bounded = if truncated { &bytes[..limit] } else { bytes };
        Self {
            text: String::from_utf8_lossy(bounded).to_string(),
            original_bytes,
            truncated,
        }
    }

    fn to_stream_summary(&self) -> EvidenceStreamSummary {
        EvidenceStreamSummary {
            summary: self.text.clone(),
            bytes: self.original_bytes,
            truncated: self.truncated,
        }
    }
}

#[derive(Debug)]
struct TargetRef<'a> {
    display_kind: &'a str,
    canonical_kind: &'a str,
    id: &'a str,
    role: &'a str,
}

fn command_capture_body(
    summary: &str,
    command: &str,
    exit_status: &str,
    stdout: &BoundedText,
    stderr: &BoundedText,
    spawn_error: Option<&str>,
) -> String {
    let mut body = String::new();
    body.push_str(summary);
    body.push_str("\n\nCommand: ");
    body.push_str(command);
    body.push_str("\nExit status: ");
    body.push_str(exit_status);
    if let Some(error) = spawn_error {
        body.push_str("\nSpawn error: ");
        body.push_str(error);
    }
    body.push_str("\n\nStdout summary");
    body.push_str(if stdout.truncated { " (truncated)" } else { "" });
    body.push_str(":\n");
    push_output_block(&mut body, &stdout.text);
    body.push_str("\nStderr summary");
    body.push_str(if stderr.truncated { " (truncated)" } else { "" });
    body.push_str(":\n");
    push_output_block(&mut body, &stderr.text);
    body
}

fn push_output_block(body: &mut String, text: &str) {
    if text.is_empty() {
        body.push_str("(none)\n");
    } else {
        body.push_str(text.trim_end());
        body.push('\n');
    }
}

fn print_output_summary(data: &EvidenceRecordData) -> Result<()> {
    let Some(output) = data.output.as_ref() else {
        return Ok(());
    };
    print_heading("Output Summary");
    print_stream_summary("Stdout", &output.stdout)?;
    print_stream_summary("Stderr", &output.stderr)?;
    Ok(())
}

fn print_stream_summary(name: &str, value: &EvidenceStreamSummary) -> Result<()> {
    let bytes = value.bytes;
    let truncated = value.truncated;
    println!("{name}: {bytes} bytes, truncated: {}", yes_no(truncated));
    if value.summary.is_empty() {
        println!("(none)");
    } else {
        println!("{}", value.summary);
    }
    Ok(())
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn format_targets(
    db: &Database,
    evidence_id: &str,
    data: &EvidenceRecordData,
) -> Result<Vec<String>> {
    let mut targets = BTreeSet::new();
    if let Some(target) = format_data_target(data) {
        targets.insert(target);
    }
    for link in db.list_record_links(KIND, evidence_id)? {
        if let Some((kind, id, role)) = evidence_link_target(&link, evidence_id) {
            let kind = display_target_kind(db, kind, id)?;
            targets.insert(format!("{kind}/{id} ({role})"));
        }
    }
    Ok(targets.into_iter().collect())
}

fn format_data_target(data: &EvidenceRecordData) -> Option<String> {
    let target = data.target.as_ref()?;
    Some(format!(
        "{}/{id} ({})",
        target.kind,
        target.role,
        id = target.id
    ))
}

fn evidence_link_target<'a>(
    link: &'a RecordLink,
    evidence_id: &str,
) -> Option<(&'a str, &'a str, &'a str)> {
    if link.source_kind == KIND && link.source_id == evidence_id {
        Some((&link.target_kind, &link.target_id, &link.relation_type))
    } else if link.target_kind == KIND && link.target_id == evidence_id {
        Some((&link.source_kind, &link.source_id, &link.relation_type))
    } else {
        None
    }
}

fn display_target_kind(db: &Database, kind: &str, id: &str) -> Result<String> {
    if kind == "issue" {
        let issue = db.require_issue(id)?;
        if issue.issue_type == "epic" {
            return Ok("epic".to_string());
        }
    }
    Ok(kind.to_string())
}

fn format_command(args: &[String]) -> String {
    args.iter()
        .map(|arg| quote_command_arg(arg))
        .collect::<Vec<_>>()
        .join(" ")
}

fn quote_command_arg(arg: &str) -> String {
    if !arg.is_empty()
        && arg
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | '/' | ':' | '='))
    {
        arg.to_string()
    } else {
        format!("'{}'", arg.replace('\'', "'\"'\"'"))
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
    crate::storage_layout::find_canonical_dir_from_cwd()
}
