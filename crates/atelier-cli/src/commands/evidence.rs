use crate::human_output::{print_footer, FooterAction};
use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::human_output;
use atelier_app::record_index::RecordIndex;
use atelier_app::storage_layout::StorageLayout;
use atelier_app::use_cases as app_use_cases;
use atelier_core::{
    EvidenceCriterion, EvidenceOutputSummary, EvidenceRecord, EvidenceRecordData, EvidenceResult,
    EvidenceStreamSummary, EvidenceTarget, RecordLink,
};
use atelier_records::{validate_record_kind, RecordStore};

const KIND: &str = "evidence";
/// A failing capture keeps the first and last bytes of each stream: test
/// runners print the failure at the end.
const OUTPUT_HEAD_BYTES: usize = 1024;
const OUTPUT_TAIL_BYTES: usize = 3072;
const OUTPUT_SUMMARY_LIMIT_BYTES: usize = OUTPUT_HEAD_BYTES + OUTPUT_TAIL_BYTES;
const EVIDENCE_LIST_LIMIT: usize = 20;
const EVIDENCE_LIST_TEXT_LIMIT: usize = 96;

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
    /// Criterion slugs; each takes the captured result.
    pub criteria: &'a [String],
    pub quiet: bool,
}

#[derive(Debug, Clone)]
pub struct TargetMetadata<'a> {
    pub kind: &'a str,
    pub id: &'a str,
    pub role: &'a str,
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

#[allow(clippy::too_many_arguments)]
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
    criteria: &[EvidenceCriterion],
) -> Result<String> {
    if let Some(target) = target.as_ref() {
        validate_record_ref(index, target.kind, target.id, target.role)?;
    }
    let criteria_mission = check_criteria(
        index,
        state_dir,
        target.as_ref().map(|target| (target.kind, target.id)),
        &criteria
            .iter()
            .map(|criterion| criterion.slug.as_str())
            .collect::<Vec<_>>(),
    )?;
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
        agent_identity: producer.map(str::to_string),
        independence_level: None,
        residual_risks: Vec::new(),
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
        criteria: criteria.to_vec(),
        criteria_mission,
    };
    let created = atelier_app::project_config::record_store(state_dir)?
        .create_evidence(summary, "recorded", summary, data)?;
    let id = created.header.id.clone();
    Ok(id)
}

/// `--criterion <slug>=<result>` values for manual proof.
pub fn manual_criteria(values: &[String]) -> Result<Vec<EvidenceCriterion>> {
    values
        .iter()
        .map(|value| {
            value
                .parse::<EvidenceCriterion>()
                .map_err(anyhow::Error::msg)
        })
        .collect()
}

/// `--criterion <slug>` values for command capture, which derives each
/// criterion's result from the exit status.
pub fn captured_criteria(values: &[String]) -> Result<Vec<String>> {
    values
        .iter()
        .map(|value| {
            if value.contains('=') {
                bail!("command capture derives each criterion's result from the exit status; pass `--criterion {}` without a result", value.split('=').next().unwrap_or_default());
            }
            if !atelier_core::is_criterion_slug(value) {
                bail!("`{value}` is not a criterion slug; use lowercase kebab-case, such as `pipe-safe`, or `S<n>`");
            }
            Ok(value.clone())
        })
        .collect()
}

/// A receipt names only criteria its target's single mission defines, and
/// each once; the answer is that mission, stored on the receipt.
fn check_criteria(
    index: &RecordIndex,
    state_dir: &Path,
    target: Option<(&str, &str)>,
    slugs: &[&str],
) -> Result<Option<String>> {
    if slugs.is_empty() {
        return Ok(None);
    }
    if let Some(slug) = slugs
        .iter()
        .enumerate()
        .find_map(|(position, slug)| slugs[..position].contains(slug).then_some(slug))
    {
        bail!("--criterion names {slug} more than once");
    }
    let Some(("issue", target_id)) = target else {
        bail!("--criterion needs an issue --target: the mission whose criteria it names, or an item only that mission holds");
    };
    let repo_root = state_dir
        .parent()
        .context("state directory has no repository parent")?;
    match atelier_app::criteria::receipt_criteria_mission(index, repo_root, target_id, slugs)? {
        Ok(mission) => Ok(Some(mission)),
        Err(problems) => bail!(
            "This receipt {}. Name the criteria as the mission's Success Criteria bullets spell them (`atelier work mission <id>` lists them)",
            problems.join("; ")
        ),
    }
}

pub fn capture(options: CaptureOptions<'_>) -> Result<()> {
    let quiet = options.quiet;
    let created = capture_record(options)?;
    print_new_record(&created, quiet)
}

/// Runs the command and records its receipt, returning the new record.
pub fn capture_record(options: CaptureOptions<'_>) -> Result<EvidenceRecord> {
    if options.command.is_empty() {
        bail!("evidence capture requires a command after --");
    }
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
    let captured_at = chrono::Utc::now();
    let command_output = Command::new(&options.command[0])
        .args(&options.command[1..])
        .output();

    let (exit_code, exit_status, success, stdout_summary, stderr_summary, spawn_error) =
        match command_output {
            Ok(output) => {
                let success = output.status.success();
                (
                    output.status.code(),
                    output
                        .status
                        .code()
                        .map(|code| code.to_string())
                        .unwrap_or_else(|| output.status.to_string()),
                    success,
                    stream_summary(&output.stdout, success),
                    stream_summary(&output.stderr, success),
                    None,
                )
            }
            Err(error) => (
                None,
                "not-started".to_string(),
                false,
                stream_summary(&[], false),
                stream_summary(error.to_string().as_bytes(), false),
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
            "evidence_capture_target_changed: the command replaced the bound target identity {:?} with {:?}; no evidence was written. Next: inspect the child command's effects, then retry against the intended target.",
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
        captured_at,
        command: Some(command_display.clone()),
        path: options.path.map(str::to_string),
        uri: options.uri.map(str::to_string),
        producer: options.producer.map(str::to_string),
        proof_scope: None,
        agent_identity: options.producer.map(str::to_string),
        independence_level: None,
        residual_risks: Vec::new(),
        exit_code,
        exit_status: Some(exit_status.clone()),
        success: Some(success),
        spawn_error,
        output: Some(EvidenceOutputSummary {
            limit_bytes_per_stream: OUTPUT_SUMMARY_LIMIT_BYTES,
            stdout: stdout_summary,
            stderr: stderr_summary,
        }),
        target: target.as_ref().map(|target| EvidenceTarget {
            kind: target.display_kind.to_string(),
            id: target.id.to_string(),
            role: target.role.to_string(),
        }),
        criteria: Vec::new(),
        criteria_mission: None,
    };
    let mut data = data;
    data.criteria = options
        .criteria
        .iter()
        .map(|slug| EvidenceCriterion {
            slug: slug.clone(),
            result: data.result,
        })
        .collect();
    data.criteria_mission = check_criteria(
        &index,
        &state_dir,
        options.target_kind.zip(resolved_target_id.as_deref()),
        &options
            .criteria
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    )
    .context("evidence capture postflight criteria check failed; no evidence was written")?;
    let body = atelier_records::render_evidence_transcript(&summary, &data);

    let created = RecordStore::for_project(&state_dir, &association.project_slug)
        .create_evidence(&summary, "recorded", &body, data)?;
    if let Some(target) = target {
        attach_silently(
            &index,
            &state_dir,
            &created.header.id,
            target.display_kind,
            target.id,
            target.role,
        )?;
    }
    Ok(created)
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
    check_criteria(
        &index,
        &state_dir,
        options.target_kind.zip(resolved_target_id.as_deref()),
        &options
            .criteria
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    )
    .context("the command was not executed")?;
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
            "failed to resolve records directory {}",
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
    let id = index.resolve(atelier_app::record_ref::RecordKind::Evidence, id)?;
    let record = index.require_evidence(&id)?;
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
    RecordStore::new(state_dir).load_evidence_by_id(id)?;
    let target = validate_record_ref(index, target_kind, target_id, role)?;
    let inserted = RecordStore::new(state_dir).add_attachment_relationship(
        KIND,
        id,
        target.canonical_kind,
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
            "Omitted: {} hidden by default limit {}",
            crate::human_output::count_noun(
                records.len() - EVIDENCE_LIST_LIMIT,
                "older evidence record"
            ),
            EVIDENCE_LIST_LIMIT
        );
    }
    print_footer([
        FooterAction::new(
            "Show the newest proof's detail",
            format!("atelier evidence show {}", records[0].header.id),
        ),
        FooterAction::new(
            "Filter by result",
            "atelier evidence list --result <result>",
        ),
        FooterAction::new("List matching IDs", "atelier --quiet evidence list"),
    ]);
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

/// Quiet output is `<id> <result>`.
pub fn print_record(index: &RecordIndex, record: &EvidenceRecord, quiet: bool) -> Result<()> {
    let data = &record.data;
    if quiet {
        println!("{} {}", record.header.id, data.result.as_str());
        return Ok(());
    }
    let targets = format_targets(index, &record.header.id, data)?;
    print_record_with_targets(record, data, targets)
}

/// Prints evidence created in this command, which is absent from the index.
/// Quiet output is the new record's ID.
pub fn print_new_record(record: &EvidenceRecord, quiet: bool) -> Result<()> {
    if quiet {
        println!("{}", record.header.id);
        return Ok(());
    }
    let data = &record.data;
    let targets = format_data_target(data).into_iter().collect();
    print_record_with_targets(record, data, targets)
}

fn print_record_with_targets(
    record: &EvidenceRecord,
    data: &EvidenceRecordData,
    targets: Vec<String>,
) -> Result<()> {
    // The title line names the record kind and status, so no line repeats
    // them.
    println!(
        "{}",
        human_output::heading(&format!(
            "{} evidence {} {}",
            record.header.id, record.header.status, record.header.title
        ))
    );
    let captured = human_output::display_timestamp(data.captured_at);
    for (label, value) in [
        ("Result:", data.result.as_str()),
        ("Commit:", human_output::short_commit(&data.commit)),
        ("Kind:", data.evidence_type.as_str()),
        ("Captured:", captured.as_str()),
    ] {
        println!("{label:<12} {value}");
    }
    let target = (!targets.is_empty()).then(|| targets.join(", "));
    let criteria = (!data.criteria.is_empty()).then(|| {
        data.criteria
            .iter()
            .map(|criterion| format!("{} {}", criterion.slug, criterion.result.as_str()))
            .collect::<Vec<_>>()
            .join(", ")
    });
    for (label, value) in [
        ("Command:", data.command.as_deref()),
        ("Exit Status:", data.exit_status.as_deref()),
        ("Target:", target.as_deref()),
        ("Criteria:", criteria.as_deref()),
    ] {
        if let Some(value) = value {
            println!("{label:<12} {value}");
        }
    }
    let created = human_output::display_timestamp(record.header.created_at);
    let updated = human_output::display_timestamp(record.header.updated_at);
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

/// A passing stream keeps no text, only its size, digest and test-count
/// line; a failing one also keeps its head and tail.
fn stream_summary(bytes: &[u8], passed: bool) -> EvidenceStreamSummary {
    let (summary, truncated) = if passed {
        (String::new(), false)
    } else {
        kept_text(bytes)
    };
    EvidenceStreamSummary {
        summary,
        bytes: bytes.len(),
        truncated,
        sha256: Some(
            Sha256::digest(bytes)
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect(),
        ),
        test_summary: test_summary_line(&String::from_utf8_lossy(bytes)),
    }
}

/// The whole stream when it fits the limit, otherwise its first and last
/// bytes joined by an omitted-bytes marker line. Cuts move off UTF-8
/// continuation bytes so no character is split, and line ends lose trailing
/// whitespace so the record passes `git diff --check`.
fn kept_text(bytes: &[u8]) -> (String, bool) {
    if bytes.len() <= OUTPUT_SUMMARY_LIMIT_BYTES {
        return (trim_line_ends(&String::from_utf8_lossy(bytes)), false);
    }
    let is_continuation = |index: usize| bytes[index] & 0b1100_0000 == 0b1000_0000;
    let mut head_end = OUTPUT_HEAD_BYTES;
    while head_end > OUTPUT_HEAD_BYTES - 3 && is_continuation(head_end) {
        head_end -= 1;
    }
    let mut tail_start = bytes.len() - OUTPUT_TAIL_BYTES;
    while tail_start < bytes.len() - OUTPUT_TAIL_BYTES + 3 && is_continuation(tail_start) {
        tail_start += 1;
    }
    let mut text = String::from_utf8_lossy(&bytes[..head_end]).into_owned();
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str(&format!("… {} bytes omitted …\n", tail_start - head_end));
    text.push_str(&String::from_utf8_lossy(&bytes[tail_start..]));
    (trim_line_ends(&text), true)
}

fn trim_line_ends(text: &str) -> String {
    let mut trimmed = text
        .split('\n')
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n");
    trimmed.truncate(trimmed.trim_end().len());
    trimmed
}

/// The last line a test runner prints with its counts: nextest's
/// `Summary [...]` or cargo test's `test result:`.
fn test_summary_line(text: &str) -> Option<String> {
    text.lines()
        .rev()
        .map(str::trim)
        .find(|line| line.starts_with("Summary [") || line.starts_with("test result:"))
        .map(str::to_string)
}

#[derive(Debug)]
struct TargetRef<'a> {
    display_kind: &'a str,
    canonical_kind: &'a str,
    id: &'a str,
    role: &'a str,
}

fn print_output_summary(data: &EvidenceRecordData) -> Result<()> {
    let Some(output) = data.output.as_ref() else {
        return Ok(());
    };
    print_heading("Output Summary");
    let passed = data.result == EvidenceResult::Pass;
    print_stream_summary("Stdout", &output.stdout, passed)?;
    print_stream_summary("Stderr", &output.stderr, passed)?;
    Ok(())
}

fn print_stream_summary(name: &str, value: &EvidenceStreamSummary, passed: bool) -> Result<()> {
    let bytes = value.bytes;
    let truncated = value.truncated;
    println!("{name}: {bytes} bytes, truncated: {}", yes_no(truncated));
    if let Some(line) = value.test_summary.as_deref() {
        println!("{line}");
    }
    if !value.summary.is_empty() {
        println!("{}", value.summary);
    } else if bytes > 0 && value.sha256.is_some() && passed {
        // A passing capture keeps its digest, not its text.
        println!("(not kept; passing output is summarized)");
    } else if bytes > 0 && value.sha256.is_some() {
        // A failing capture keeps its text without trailing whitespace, so
        // only an all-whitespace stream is left empty.
        println!("(whitespace only)");
    } else {
        println!("(none)");
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

fn display_target_kind(_index: &RecordIndex, kind: &str, _id: &str) -> Result<String> {
    Ok(kind.to_string())
}

pub fn format_command(args: &[String]) -> String {
    args.iter()
        .map(|arg| quote_command_arg(arg))
        .collect::<Vec<_>>()
        .join(" ")
}

/// The arguments a receipt's stored command was written from: the inverse of
/// `format_command`, which leaves safe words bare and single-quotes the rest.
pub fn split_command(command: &str) -> Result<Vec<String>> {
    let mut args = Vec::new();
    let mut current: Option<String> = None;
    let mut quoted = false;
    let mut chars = command.chars();
    while let Some(ch) = chars.next() {
        match (quoted, ch) {
            (true, '\'') => quoted = false,
            (true, ch) => current.get_or_insert_with(String::new).push(ch),
            (false, '\'') => {
                quoted = true;
                current.get_or_insert_with(String::new);
            }
            // A quote inside a quoted word is written as `'"'"'`.
            (false, '"') if chars.as_str().starts_with("'\"") => {
                chars.nth(1);
                current.get_or_insert_with(String::new).push('\'');
            }
            (false, '"' | '\\') => {
                bail!("recorded command {command:?} uses quoting this command did not write")
            }
            (false, ch) if ch.is_whitespace() => args.extend(current.take()),
            (false, ch) => current.get_or_insert_with(String::new).push(ch),
        }
    }
    if quoted {
        bail!("recorded command {command:?} has an unterminated quote");
    }
    args.extend(current);
    Ok(args)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_recorded_command_splits_back_into_the_arguments_it_was_written_from() {
        let args = ["sh", "-c", "echo 'it''s' \"x\" $HOME", "", "plain/arg:1=2"]
            .map(String::from)
            .to_vec();
        assert_eq!(split_command(&format_command(&args)).unwrap(), args);
        assert!(split_command("echo \"unterminated").is_err());
    }

    #[test]
    fn failing_capture_keeps_head_and_tail_on_character_boundaries() {
        for size in [
            0,
            OUTPUT_SUMMARY_LIMIT_BYTES,
            OUTPUT_SUMMARY_LIMIT_BYTES + 1,
        ] {
            let stream = stream_summary(&vec![b'x'; size], false);
            assert_eq!(stream.bytes, size);
            assert_eq!(stream.truncated, size > OUTPUT_SUMMARY_LIMIT_BYTES);
            if size <= OUTPUT_SUMMARY_LIMIT_BYTES {
                assert_eq!(stream.summary, "x".repeat(size));
            }
        }
        assert_eq!(
            stream_summary(&vec![b'x'; OUTPUT_SUMMARY_LIMIT_BYTES + 1], false).summary,
            format!(
                "{}\n… 1 bytes omitted …\n{}",
                "x".repeat(OUTPUT_HEAD_BYTES),
                "x".repeat(OUTPUT_TAIL_BYTES)
            )
        );

        // A four-byte character straddles each cut; neither is split.
        let cat = "🐈".as_bytes();
        let mut bytes = vec![b'h'; OUTPUT_HEAD_BYTES - 1];
        bytes.extend_from_slice(cat);
        bytes.extend(vec![b'm'; 5000]);
        bytes.extend_from_slice(cat);
        bytes.extend(vec![b't'; OUTPUT_TAIL_BYTES - 2]);
        let stream = stream_summary(&bytes, false);
        assert!(stream.truncated);
        assert!(!stream.summary.contains('\u{fffd}'));
        assert_eq!(
            stream.summary,
            format!(
                "{}\n… {} bytes omitted …\n{}",
                "h".repeat(OUTPUT_HEAD_BYTES - 1),
                cat.len() + 5000 + cat.len(),
                "t".repeat(OUTPUT_TAIL_BYTES - 2)
            )
        );
        assert_eq!(stream_summary(b"\xff\n ", false).summary, "\u{fffd}");
    }

    #[test]
    fn failing_capture_keeps_the_final_failure_line() {
        let mut output = String::new();
        for index in 0..600 {
            output.push_str(&format!("PASS test_{index:03}\n"));
        }
        output.push_str("FAIL x\n");
        let stream = stream_summary(output.as_bytes(), false);
        assert!(stream.truncated);
        assert!(stream.summary.starts_with("PASS test_000\n"));
        assert!(stream.summary.ends_with("PASS test_599\nFAIL x"));
        assert!(stream.summary.contains(" bytes omitted …\n"));
    }

    #[test]
    fn passing_capture_keeps_digest_and_test_count_not_text() {
        let output = b"   Compiling x\n     Summary [   1.0s] 3 tests run: 3 passed, 0 skipped\n";
        let stream = stream_summary(output, true);
        assert_eq!(stream.summary, "");
        assert!(!stream.truncated);
        assert_eq!(stream.bytes, output.len());
        assert_eq!(
            stream_summary(b"abc", true).sha256.as_deref(),
            Some("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
        );
        assert_eq!(
            stream.test_summary.as_deref(),
            Some("Summary [   1.0s] 3 tests run: 3 passed, 0 skipped")
        );
        assert_eq!(
            test_summary_line("test result: ok. 1 passed\ntest result: ok. 2 passed\n"),
            Some("test result: ok. 2 passed".to_string())
        );
        assert_eq!(test_summary_line("all good\n"), None);
    }
}
