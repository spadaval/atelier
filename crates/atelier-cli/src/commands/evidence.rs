use anyhow::{bail, Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::human_output;
use atelier_app::record_index::RecordIndex;
use atelier_app::storage_layout::StorageLayout;
use atelier_app::use_cases as app_use_cases;
use atelier_core::{
    EvidenceOutputSummary, EvidenceRecord, EvidenceRecordData, EvidenceResult,
    EvidenceStreamSummary, EvidenceTarget, RecordLink,
};
use atelier_records::{validate_record_kind, RecordStore};

const KIND: &str = "evidence";
const OUTPUT_SUMMARY_LIMIT_BYTES: usize = 4096;
const EVIDENCE_LIST_LIMIT: usize = 20;
const EVIDENCE_LIST_TEXT_LIMIT: usize = 96;
const ACCEPTED_EVIDENCE_RELATION_ROLES: &[&str] = &["validates"];

pub struct CaptureOptions<'a> {
    pub evidence_kind: &'a str,
    pub summary: Option<&'a str>,
    pub path: Option<&'a str>,
    pub uri: Option<&'a str>,
    pub producer: Option<&'a str>,
    pub target_kind: Option<&'a str>,
    pub target_id: Option<&'a str>,
    pub role: &'a str,
    pub command: &'a [String],
    pub quiet: bool,
}

#[derive(Debug, Clone)]
pub struct TargetMetadata<'a> {
    pub kind: &'a str,
    pub id: &'a str,
    pub role: &'a str,
}

#[derive(Debug, Clone)]
struct EvidenceMetadata<'a> {
    agent_identity: Option<&'a str>,
    residual_risks: Vec<String>,
    follow_up_ids: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CaptureAssociation {
    repo_root: PathBuf,
    state_dir: PathBuf,
    git_dir: Option<PathBuf>,
    project_slug: String,
    #[cfg(unix)]
    repo_device: u64,
    #[cfg(unix)]
    repo_inode: u64,
    #[cfg(unix)]
    state_device: u64,
    #[cfg(unix)]
    state_inode: u64,
}

#[derive(Debug)]
struct CaptureBinding {
    association: CaptureAssociation,
    target_identity: Option<CaptureTargetIdentity>,
    commit: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CaptureTargetIdentity {
    canonical_kind: String,
    id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    schema: String,
    schema_version: i64,
    record_type: String,
}

impl<'a> EvidenceMetadata<'a> {
    fn from_producer(producer: Option<&'a str>) -> Self {
        Self {
            agent_identity: producer,
            residual_risks: Vec::new(),
            follow_up_ids: Vec::new(),
        }
    }
}

pub fn add_returning_id(
    index: &RecordIndex,
    state_dir: &Path,
    evidence_kind: &str,
    summary: &str,
    result: EvidenceResult,
    path: Option<&str>,
    uri: Option<&str>,
    producer: Option<&str>,
    target: Option<TargetMetadata<'_>>,
) -> Result<String> {
    if let Some(target) = target.as_ref() {
        validate_record_ref(index, target.kind, target.id, target.role)?;
    }
    let metadata = EvidenceMetadata::from_producer(producer);
    let data = EvidenceRecordData {
        commit: atelier_app::git_freshness::head_commit(
            state_dir
                .parent()
                .context("state directory has no repository parent")?,
        )?,
        result,
        evidence_type: evidence_kind.to_string(),
        captured_at: chrono::Utc::now(),
        command: None,
        path: path.map(str::to_string),
        uri: uri.map(str::to_string),
        producer: producer.map(str::to_string),
        proof_scope: None,
        agent_identity: metadata.agent_identity.map(str::to_string),
        independence_level: None,
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
    let created =
        RecordStore::new(state_dir).create_evidence(summary, "recorded", summary, data)?;
    let id = created.header.id.clone();
    Ok(id)
}

pub fn capture(options: CaptureOptions<'_>) -> Result<()> {
    if options.command.is_empty() {
        bail!("evidence capture requires a command after --");
    }
    validate_evidence_relation_role(options.role)?;
    if matches!(
        (options.target_kind, options.target_id),
        (Some(_), None) | (None, Some(_))
    ) {
        bail!("--target-kind and --target-id must be supplied together");
    }

    // Bind and validate the exact repository/target before executing anything.
    // This transaction is deliberately released before the arbitrary child.
    let binding = capture_preflight(&options)?;
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

    // Never hold the canonical association lock while waiting for an arbitrary
    // child: the child may be a normal Atelier writer or the exclusive
    // migration. Once it exits, acquire one transaction and reload/revalidate
    // current repository and target state before allocating or appending proof.
    let layout = StorageLayout::discover().context(
        "evidence capture postflight could not rediscover the original repository; no evidence was written",
    )?;
    let state_dir = layout.canonical_dir();
    // No child remains to wait on. Hold the exclusive association transaction
    // across final identity validation and every canonical append so no normal
    // writer can replace the target between the check and attachment.
    let _transaction =
        atelier_records::mutation_lock::CanonicalMutationLock::exclusive(&state_dir)?;
    let association = capture_association(&layout)?;
    if atelier_app::git_freshness::head_commit(layout.repo_root())? != binding.commit {
        bail!("evidence capture command changed HEAD; no evidence was written. Commit the intended changes and rerun the check.");
    }
    if association != binding.association {
        bail!(
            "evidence_capture_association_changed: the command completed, but the repository or .atelier association changed before proof could be recorded; no evidence was written. Return to {} and inspect the child command's effects before retrying evidence capture.",
            binding.association.repo_root.display()
        );
    }
    let index = RecordIndex::load(&state_dir).context("evidence capture postflight could not load records; no evidence was written. Run `atelier check` and repair the record files before retrying")?;
    let resolved_target_id = match (options.target_kind, options.target_id) {
        (Some(kind), Some(id)) => Some(
            app_use_cases::resolve_evidence_target_ref(&index, kind, id).context(
                "evidence capture postflight target validation failed after the command completed; no evidence was written",
            )?,
        ),
        (None, None) => None,
        _ => bail!("--target-kind and --target-id must be supplied together"),
    };
    let (target, target_identity) = capture_target_and_identity(
        &index,
        options.target_kind,
        resolved_target_id.as_deref(),
        options.role,
    )
    .context(
        "evidence capture postflight target validation failed after the command completed; no evidence was written",
    )?;
    if target_identity != binding.target_identity {
        bail!(
            "evidence_capture_target_changed: the command replaced the bound target identity {:?} with {:?}; no evidence was written. Inspect the child command's effects and retry against the intended canonical target.",
            binding.target_identity,
            target_identity
        );
    }

    if atelier_app::git_freshness::working_paths(layout.repo_root())?
        .iter()
        .any(|path| !atelier_app::git_freshness::is_record_path(path))
    {
        bail!("evidence capture command changed code or policy; no evidence was written. Commit the relevant changes and rerun the check.");
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
        commit: binding.commit.clone(),
        result: if spawn_error.is_some() {
            EvidenceResult::Blocked
        } else if success {
            EvidenceResult::Pass
        } else {
            EvidenceResult::Fail
        },
        evidence_type: options.evidence_kind.to_string(),
        captured_at: chrono::DateTime::parse_from_rfc3339(&captured_at)?
            .with_timezone(&chrono::Utc),
        command: Some(command_display.clone()),
        path: options.path.map(str::to_string),
        uri: options.uri.map(str::to_string),
        producer: options.producer.map(str::to_string),
        proof_scope: None,
        agent_identity: metadata.agent_identity.map(str::to_string),
        independence_level: None,
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

    let created =
        RecordStore::new(&state_dir).create_evidence(&summary, "recorded", &body, data)?;
    if let Some(target) = target {
        attach_silently(
            &index,
            &state_dir,
            &created.header.id,
            &target.display_kind,
            &target.id,
            &target.role,
        )?;
    }
    print_new_record(&created, options.quiet)
}

fn capture_preflight(options: &CaptureOptions<'_>) -> Result<CaptureBinding> {
    let layout = StorageLayout::discover().context(
        "evidence capture preflight could not discover an Atelier repository; the command was not executed",
    )?;
    let state_dir = layout.canonical_dir();
    let _transaction = atelier_records::mutation_lock::CanonicalMutationLock::shared(&state_dir)?;
    let association = capture_association(&layout)?;
    let index = RecordIndex::load(&state_dir).context("evidence capture preflight could not load records; the command was not executed. Run `atelier check` and repair the record files before retrying")?;
    let resolved_target_id = match (options.target_kind, options.target_id) {
        (Some(kind), Some(id)) => Some(
            app_use_cases::resolve_evidence_target_ref(&index, kind, id).context(
                "evidence capture preflight target validation failed; the command was not executed",
            )?,
        ),
        (None, None) => None,
        _ => bail!("--target-kind and --target-id must be supplied together"),
    };
    let (_, target_identity) = capture_target_and_identity(
        &index,
        options.target_kind,
        resolved_target_id.as_deref(),
        options.role,
    )
    .context("evidence capture preflight target validation failed; the command was not executed")?;
    let dirty = atelier_app::git_freshness::working_paths(layout.repo_root())?;
    if dirty
        .iter()
        .any(|path| !atelier_app::git_freshness::is_record_path(path))
    {
        bail!("evidence capture requires committed code and policy; commit the relevant changes before running the check");
    }
    let commit = atelier_app::git_freshness::head_commit(layout.repo_root())?;
    Ok(CaptureBinding {
        commit,
        association,
        target_identity,
    })
}

fn capture_target_and_identity<'a>(
    index: &RecordIndex,
    target_kind: Option<&'a str>,
    target_id: Option<&'a str>,
    role: &'a str,
) -> Result<(Option<TargetRef<'a>>, Option<CaptureTargetIdentity>)> {
    let target = match (target_kind, target_id) {
        (Some(kind), Some(id)) => Some(validate_record_ref(index, kind, id, role)?),
        (None, None) => None,
        _ => bail!("--target-kind and --target-id must be supplied together"),
    };
    let identity = match target.as_ref() {
        Some(target) if target.canonical_kind == "issue" => {
            let issue = index.require_issue(target.id)?;
            let spec = atelier_records::canonical_record_kind("issue")?;
            Some(CaptureTargetIdentity {
                canonical_kind: "issue".to_string(),
                id: issue.id,
                created_at: issue.created_at,
                schema: spec.schema.to_string(),
                schema_version: spec.schema_version,
                record_type: issue.issue_type,
            })
        }
        Some(target) => {
            let header = &index.require_evidence(target.id)?.header;
            let spec = atelier_records::canonical_record_kind(target.canonical_kind)?;
            Some(CaptureTargetIdentity {
                canonical_kind: header.kind.clone(),
                id: header.id.clone(),
                created_at: header.created_at,
                schema: spec.schema.to_string(),
                schema_version: spec.schema_version,
                record_type: header.kind.clone(),
            })
        }
        None => None,
    };
    Ok((target, identity))
}

fn capture_association(layout: &StorageLayout) -> Result<CaptureAssociation> {
    let repo_root = fs::canonicalize(layout.repo_root()).with_context(|| {
        format!(
            "failed to resolve evidence repository root {}",
            layout.repo_root().display()
        )
    })?;
    let state_dir = fs::canonicalize(layout.canonical_dir()).with_context(|| {
        format!(
            "failed to resolve evidence canonical state {}",
            layout.canonical_dir().display()
        )
    })?;
    let project_slug = atelier_app::project_config::ProjectConfig::load(&repo_root)?.project_slug;
    let git_dir = git_dir_identity(&repo_root)?;
    #[cfg(unix)]
    {
        // Root identities catch wholesale workspace/.atelier replacement while
        // allowing legitimate atomic swaps of canonical subtrees such as
        // issues/ and evidence/ during import or bundle application.
        use std::os::unix::fs::MetadataExt;
        let repo_metadata = fs::metadata(&repo_root)?;
        let state_metadata = fs::metadata(&state_dir)?;
        Ok(CaptureAssociation {
            repo_root,
            state_dir,
            git_dir,
            project_slug,
            repo_device: repo_metadata.dev(),
            repo_inode: repo_metadata.ino(),
            state_device: state_metadata.dev(),
            state_inode: state_metadata.ino(),
        })
    }
    #[cfg(not(unix))]
    {
        Ok(CaptureAssociation {
            repo_root,
            state_dir,
            git_dir,
            project_slug,
        })
    }
}

fn git_dir_identity(repo_root: &Path) -> Result<Option<PathBuf>> {
    let dot_git = repo_root.join(".git");
    if !dot_git.exists() {
        return Ok(None);
    }
    if dot_git.is_dir() {
        return Ok(Some(fs::canonicalize(dot_git)?));
    }
    let pointer = fs::read_to_string(&dot_git)
        .with_context(|| format!("failed to read Git worktree pointer {}", dot_git.display()))?;
    let git_dir = pointer
        .trim()
        .strip_prefix("gitdir:")
        .map(str::trim)
        .context("invalid Git worktree pointer while binding evidence repository")?;
    let git_dir = Path::new(git_dir);
    let git_dir = if git_dir.is_absolute() {
        git_dir.to_path_buf()
    } else {
        repo_root.join(git_dir)
    };
    Ok(Some(fs::canonicalize(git_dir)?))
}

pub fn show(index: &RecordIndex, id: &str, quiet: bool) -> Result<()> {
    let record = index.require_evidence(id)?;
    print_record(index, record, quiet)
}

pub fn attach(
    index: &RecordIndex,
    state_dir: &Path,
    id: &str,
    target_kind: &str,
    target_id: &str,
    role: &str,
    quiet: bool,
) -> Result<()> {
    attach_impl(
        index,
        state_dir,
        id,
        target_kind,
        target_id,
        role,
        Some(quiet),
    )
}

/// Attaches evidence created in this command (and therefore absent from
/// `index`) without printing.
pub fn attach_silently(
    index: &RecordIndex,
    state_dir: &Path,
    id: &str,
    target_kind: &str,
    target_id: &str,
    role: &str,
) -> Result<()> {
    attach_impl(index, state_dir, id, target_kind, target_id, role, None)
}

fn attach_impl(
    index: &RecordIndex,
    state_dir: &Path,
    id: &str,
    target_kind: &str,
    target_id: &str,
    role: &str,
    quiet: Option<bool>,
) -> Result<()> {
    validate_evidence_relation_role(role)?;
    RecordStore::new(state_dir).load_evidence_by_id(id)?;
    let target = validate_record_ref(index, target_kind, target_id, role)?;
    let inserted = RecordStore::new(state_dir).add_attachment_relationship(
        KIND,
        id,
        &target.canonical_kind,
        target_id,
        role,
    )?;
    let Some(quiet) = quiet else {
        return Ok(());
    };
    if quiet {
        println!("{id}");
    } else if inserted {
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
    index: &RecordIndex,
    kind: &'a str,
    id: &'a str,
    role: &'a str,
) -> Result<TargetRef<'a>> {
    validate_evidence_relation_role(role)?;
    if kind == "epic" {
        let issue = index.require_issue(id)?;
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

    validate_record_kind(kind)?;
    if kind == "issue" {
        index.require_issue(id)?;
    } else {
        index.require_evidence(id)?;
    }
    Ok(TargetRef {
        display_kind: kind,
        canonical_kind: kind,
        id,
        role,
    })
}

pub fn validate_evidence_relation_role(role: &str) -> Result<()> {
    if ACCEPTED_EVIDENCE_RELATION_ROLES.contains(&role) {
        return Ok(());
    }
    bail!(
        "Invalid evidence relation role '{role}'. Accepted evidence relation vocabulary: {}. Evidence kinds such as validation belong in --kind, not --role. Normal flow: record proof with `atelier evidence record --target issue/<id> --kind validation \"summary\"`; reuse existing proof with `atelier evidence attach <evidence-id> issue <issue-id>`.",
        ACCEPTED_EVIDENCE_RELATION_ROLES.join(", ")
    )
}

pub fn list(index: &RecordIndex, result: Option<EvidenceResult>, quiet: bool) -> Result<()> {
    let records = index
        .evidence_records()
        .into_iter()
        .filter(|record| result.is_none_or(|result| record.data.result == result))
        .collect::<Vec<_>>();
    if quiet {
        for record in records {
            println!("{}", record.header.id);
        }
        return Ok(());
    }
    if records.is_empty() {
        print_heading("Evidence");
        println!("(none)");
        return Ok(());
    }
    print_heading("Evidence");
    println!("{} total", records.len());
    println!(
        "Showing: {} of {}",
        records.len().min(EVIDENCE_LIST_LIMIT),
        records.len()
    );
    for record in records.iter().take(EVIDENCE_LIST_LIMIT) {
        let data = &record.data;
        let kind = data.evidence_type.as_str();
        let command = evidence_list_command(data.command.as_deref());
        let title = evidence_list_title(&record.header.title, data.command.as_deref());
        let exit_status = data.exit_status.as_deref().unwrap_or("(none)");
        let targets = format_targets(index, &record.header.id, data)?;
        let target = if targets.is_empty() {
            "(none)".to_string()
        } else {
            targets.join(", ")
        };
        println!(
            "  {:<14} {:<13} {:<10} exit {} target {} command {} - {}",
            record.header.id,
            data.result.as_str(),
            kind,
            exit_status,
            target,
            command,
            title
        );
    }
    if records.len() > EVIDENCE_LIST_LIMIT {
        println!(
            "Omitted: {} older evidence record(s) hidden by default limit {}",
            records.len() - EVIDENCE_LIST_LIMIT,
            EVIDENCE_LIST_LIMIT
        );
    }
    println!();
    println!("Next Commands");
    println!("-------------");
    println!("  Show proof detail: atelier evidence show <evidence-id>");
    println!("  Filter by result: atelier evidence list --result <result>");
    println!("  List matching IDs: atelier --quiet evidence list");
    Ok(())
}

fn evidence_list_command(command: Option<&str>) -> String {
    let Some(command) = command else {
        return "(manual)".to_string();
    };
    let parts = command.split_whitespace().collect::<Vec<_>>();
    let raw_summary = parts.iter().take(3).copied().collect::<Vec<_>>().join(" ");
    let truncated = parts.len() > 3 || raw_summary.chars().count() > EVIDENCE_LIST_TEXT_LIMIT;
    let summary = bounded_list_text(&raw_summary);
    if truncated {
        format!("{summary} ...")
    } else {
        summary
    }
}

fn evidence_list_title(title: &str, command: Option<&str>) -> String {
    if command == Some(title) {
        "(command-backed proof)".to_string()
    } else {
        bounded_list_text(title)
    }
}

fn bounded_list_text(value: &str) -> String {
    let normalized = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.chars().count() <= EVIDENCE_LIST_TEXT_LIMIT {
        return normalized;
    }
    let mut output = normalized
        .chars()
        .take(EVIDENCE_LIST_TEXT_LIMIT - 3)
        .collect::<String>();
    output.push_str("...");
    output
}

pub fn print_record(index: &RecordIndex, record: &EvidenceRecord, quiet: bool) -> Result<()> {
    let data = &record.data;
    let targets = format_targets(index, &record.header.id, data)?;
    print_record_with_targets(record, data, targets, quiet)
}

/// Prints evidence created in this command, which is absent from the index.
pub fn print_new_record(record: &EvidenceRecord, quiet: bool) -> Result<()> {
    let data = &record.data;
    let targets = format_data_target(data).into_iter().collect();
    print_record_with_targets(record, data, targets, quiet)
}

fn print_record_with_targets(
    record: &EvidenceRecord,
    data: &EvidenceRecordData,
    targets: Vec<String>,
    quiet: bool,
) -> Result<()> {
    if quiet {
        let exit_status = data.exit_status.as_deref().unwrap_or("(none)");
        let targets = if targets.is_empty() {
            "(none)".to_string()
        } else {
            targets.join(",").replace(' ', "")
        };
        println!(
            "{} {} {} exit={exit_status} target={targets}",
            record.header.id, record.header.status, data.evidence_type
        );
        return Ok(());
    }
    let identity = format!(
        "{} [evidence] {} - {}",
        record.header.id, record.header.status, record.header.title
    );
    println!("{identity}\n{}", "=".repeat(identity.len()));
    let captured = data.captured_at.to_rfc3339();
    for (label, value) in [
        ("Status:", record.header.status.as_str()),
        ("Result:", data.result.as_str()),
        ("Commit:", data.commit.as_str()),
        ("Kind:", data.evidence_type.as_str()),
        ("Captured:", captured.as_str()),
    ] {
        println!("{label:<12} {value}");
    }
    let target = (!targets.is_empty()).then(|| targets.join(", "));
    for (label, value) in [
        ("Command:", data.command.as_deref()),
        ("Exit Status:", data.exit_status.as_deref()),
        ("Target:", target.as_deref()),
    ] {
        if let Some(value) = value {
            println!("{label:<12} {value}");
        }
    }
    let created = record.header.created_at.to_rfc3339();
    let updated = record.header.updated_at.to_rfc3339();
    for (label, value) in [
        (
            "Producer:",
            data.producer
                .as_deref()
                .or(data.agent_identity.as_deref())
                .unwrap_or("(none)"),
        ),
        ("Path:", data.path.as_deref().unwrap_or("(none)")),
        ("URI:", data.uri.as_deref().unwrap_or("(none)")),
        ("Created:", created.as_str()),
        ("Updated:", updated.as_str()),
    ] {
        println!("{label:<12} {value}");
    }
    print_heading("Summary");
    if record.summary.is_empty() {
        println!("(none)");
    } else {
        println!("{}", record.summary);
    }
    print_output_summary(data)?;
    Ok(())
}

fn print_heading(title: &str) {
    human_output::print_section_heading(title);
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
    body.push_str("## Summary\n\n");
    body.push_str(summary);
    body.push_str("\n\n## Command\n\n```console\n");
    body.push_str(command);
    body.push_str("\n```\n\nExit status: ");
    body.push_str(exit_status);
    if let Some(error) = spawn_error {
        body.push_str("\nSpawn error: ");
        body.push_str(error);
    }
    body.push_str("\n\n## Stdout\n\n");
    push_output_block(&mut body, stdout);
    body.push_str("\n## Stderr\n\n");
    push_output_block(&mut body, stderr);
    body
}

fn push_output_block(body: &mut String, text: &BoundedText) {
    body.push_str(&format!("Bytes: {}\n", text.original_bytes));
    body.push_str(&format!(
        "Truncated: {}\n\n",
        if text.truncated { "yes" } else { "no" }
    ));
    body.push_str("```text\n");
    if text.text.is_empty() {
        body.push_str("```\n");
    } else {
        body.push_str(text.text.trim_end());
        body.push_str("\n```\n");
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
    index: &RecordIndex,
    evidence_id: &str,
    data: &EvidenceRecordData,
) -> Result<Vec<String>> {
    let mut targets = BTreeSet::new();
    if let Some(target) = format_data_target(data) {
        targets.insert(target);
    }
    for link in index.list_record_links(KIND, evidence_id)? {
        if let Some((kind, id, role)) = evidence_link_target(&link, evidence_id) {
            let kind = display_target_kind(index, kind, id)?;
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

fn display_target_kind(index: &RecordIndex, kind: &str, id: &str) -> Result<String> {
    if kind == "issue" {
        let issue = index.require_issue(id)?;
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
