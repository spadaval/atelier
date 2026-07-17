//! One-shot migration from direct mission readiness to independent plan review.

use anyhow::{anyhow, bail, Context, Result};
use atelier_records::activity::{
    record_activity_path, timestamp_activity_id, ActivityEventType, IssueActivity,
};
use atelier_records::mission_plan_review::{
    mission_graph_revision, LegacyGrandfatherEligibility, MissionPlanReviewCutoverManifest,
    MissionPlanReviewEvent, LEGACY_GRANDFATHER_MIGRATION_ACTOR, LEGACY_GRANDFATHER_MIGRATION_ID,
    LEGACY_GRANDFATHER_STATUS, MISSION_PLAN_REVIEW_CUTOVER_MANIFEST_PATH,
    MISSION_PLAN_REVIEW_CUTOVER_SCHEMA, MISSION_PLAN_REVIEW_CUTOVER_SCHEMA_VERSION,
};
use atelier_records::{issue_record_path, render_issue_record, RecordStore};
use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

const RECOVERY: &str = "Mission-plan cutover is not complete. Retry `atelier migrate-mission-plan-review` first so any durable recovery journal can converge to an exact pre- or post-cutover state. If recovery reports malformed or unowned canonical bytes, repair the named .atelier file, run `atelier check --fix`, then retry the migration.";
const JOURNAL_DIR: &str = "runtime/mission-plan-review-cutover-journal";
const JOURNAL_SCHEMA: &str = "atelier.mission-plan-review-cutover-journal";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MigrationReport {
    pub mission_count: usize,
    pub terminal_count: usize,
    pub draft_count: usize,
    pub plan_review_count: usize,
    pub active_count: usize,
    pub already_applied: bool,
}

#[derive(Debug)]
struct PlannedWrite {
    path: PathBuf,
    contents: Vec<u8>,
    must_not_exist: bool,
    original: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CutoverJournal {
    schema: String,
    schema_version: u32,
    entries: Vec<CutoverJournalEntry>,
    original_snapshot: Vec<CanonicalSnapshotEntry>,
    post_snapshot: Vec<CanonicalSnapshotEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CutoverJournalEntry {
    relative_path: String,
    original_existed: bool,
    workflow_activation: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSnapshotEntry {
    relative_path: String,
    path_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sha256: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct CanonicalSnapshot {
    files: BTreeMap<PathBuf, Vec<u8>>,
    directories: BTreeSet<PathBuf>,
}

#[cfg(test)]
#[derive(Debug, Default)]
struct MigrationFault {
    interrupt_after_boundary: Option<usize>,
    concurrent_edit_before_apply: Option<(PathBuf, Vec<u8>)>,
}

pub fn run(repo_root: &Path) -> Result<MigrationReport> {
    let now = Utc::now();
    let cutover_at = now
        .with_nanosecond((now.nanosecond() / 1_000) * 1_000)
        .expect("truncated nanoseconds remain valid");
    migrate_at(repo_root, cutover_at, None)
}

fn migrate_at(
    repo_root: &Path,
    cutover_at: DateTime<Utc>,
    #[cfg(test)] fault: Option<MigrationFault>,
    #[cfg(not(test))] _fault: Option<()>,
) -> Result<MigrationReport> {
    let state_dir = repo_root.join(".atelier");
    let _lock = atelier_records::mutation_lock::CanonicalMutationLock::exclusive(&state_dir)?;
    recover_cutover_journal(repo_root).context(RECOVERY)?;
    let policy = crate::workflow_policy::load(repo_root)?;
    let mission_workflow = policy.workflow_for_issue_type("mission")?;
    let terminal = mission_workflow
        .done_statuses
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let already_applied =
        crate::workflow_policy::enforces_independent_mission_plan_review(&policy)?;
    let store = RecordStore::new(&state_dir);
    let mut missions = store
        .load_issues()?
        .into_iter()
        .filter(|record| record.issue.issue_type == "mission")
        .collect::<Vec<_>>();
    missions.sort_by(|left, right| left.issue.id.cmp(&right.issue.id));

    let mut report = MigrationReport {
        mission_count: missions.len(),
        terminal_count: 0,
        draft_count: 0,
        plan_review_count: 0,
        active_count: 0,
        already_applied,
    };
    for mission in &missions {
        match mission.issue.status.as_str() {
            "draft" => report.draft_count += 1,
            "ready" if !already_applied => report.plan_review_count += 1,
            "plan_review" if already_applied => report.plan_review_count += 1,
            "ready" if already_applied => {}
            LEGACY_GRANDFATHER_STATUS => report.active_count += 1,
            status if terminal.contains(status) => report.terminal_count += 1,
            status => bail!(
                "Mission {} has unknown legacy status '{}'; {RECOVERY}",
                mission.issue.id,
                status
            ),
        }
    }

    if already_applied {
        validate_staged(repo_root, &[]).context(RECOVERY)?;
        return Ok(report);
    }

    let manifest_path = state_dir.join(MISSION_PLAN_REVIEW_CUTOVER_MANIFEST_PATH);
    if manifest_path.exists() {
        bail!(
            "Refusing migration because {} already exists under a legacy workflow; {RECOVERY}",
            manifest_path.display()
        );
    }

    // Re-inventory immediately before emission. A changed candidate set aborts
    // rather than minting eligibility from a stale audit.
    let inventory_ids = RecordStore::new(&state_dir)
        .load_issues()?
        .into_iter()
        .filter(|record| record.issue.issue_type == "mission")
        .map(|record| (record.issue.id, record.issue.status))
        .collect::<Vec<_>>();
    let audited_ids = missions
        .iter()
        .map(|record| (record.issue.id.clone(), record.issue.status.clone()))
        .collect::<Vec<_>>();
    if inventory_ids != audited_ids {
        bail!("Mission inventory changed during cutover preparation; {RECOVERY}");
    }

    let mut writes = Vec::new();
    for mission in &missions {
        if mission.issue.status == "ready" {
            let mut migrated = mission.clone();
            migrated.issue.status = "plan_review".to_string();
            writes.push(PlannedWrite {
                path: state_dir.join(issue_record_path(&mission.issue.id)),
                contents: render_issue_record(&migrated)?.into_bytes(),
                must_not_exist: false,
                original: None,
            });
        }
    }

    let receipt_id = timestamp_activity_id(cutover_at);
    let mut eligible = Vec::new();
    for mission in missions
        .iter()
        .filter(|record| record.issue.status == LEGACY_GRANDFATHER_STATUS)
    {
        let graph_revision = mission_graph_revision(&state_dir, &mission.issue.id)?;
        eligible.push(LegacyGrandfatherEligibility {
            mission_id: mission.issue.id.clone(),
            graph_revision,
            legacy_status: LEGACY_GRANDFATHER_STATUS.to_string(),
            receipt_activity_id: receipt_id.clone(),
        });
    }
    if !eligible.is_empty() {
        let manifest = MissionPlanReviewCutoverManifest {
            schema: MISSION_PLAN_REVIEW_CUTOVER_SCHEMA.to_string(),
            schema_version: MISSION_PLAN_REVIEW_CUTOVER_SCHEMA_VERSION,
            migration_id: LEGACY_GRANDFATHER_MIGRATION_ID.to_string(),
            cutover_at,
            eligible_missions: eligible.clone(),
        };
        manifest.validate()?;
        writes.push(PlannedWrite {
            path: manifest_path,
            contents: serde_yaml::to_string(&manifest)?.into_bytes(),
            must_not_exist: true,
            original: None,
        });
        for entry in &eligible {
            let activity = IssueActivity {
                id: receipt_id.clone(),
                subject_kind: "issue".to_string(),
                subject_id: entry.mission_id.clone(),
                event_type: ActivityEventType::MissionPlanReview,
                actor: LEGACY_GRANDFATHER_MIGRATION_ACTOR.to_string(),
                created_at: cutover_at,
                summary: "Grandfather active mission at independent plan-review cutover"
                    .to_string(),
                pr_attribution: None,
                workflow_transition: None,
                mission_plan_review: Some(MissionPlanReviewEvent::LegacyGrandfather {
                    graph_revision: entry.graph_revision.clone(),
                    legacy_status: entry.legacy_status.clone(),
                    migration_id: LEGACY_GRANDFATHER_MIGRATION_ID.to_string(),
                    cutover_receipt: manifest.receipt_for(entry)?,
                }),
                body: String::new(),
            };
            writes.push(PlannedWrite {
                path: state_dir.join(record_activity_path(
                    "issue",
                    &entry.mission_id,
                    &receipt_id,
                )),
                contents: format!("{}\n", activity.to_markdown()?.trim_end()).into_bytes(),
                must_not_exist: true,
                original: None,
            });
        }
    }

    let workflow_path = state_dir.join("workflow.yaml");
    let workflow_text = fs::read_to_string(&workflow_path)?;
    writes.push(PlannedWrite {
        path: workflow_path,
        contents: render_target_workflow(&workflow_text)?.into_bytes(),
        must_not_exist: false,
        original: None,
    });

    capture_originals(&mut writes).context(RECOVERY)?;
    let canonical_snapshot = canonical_snapshot(&state_dir)?;
    validate_staged(repo_root, &writes).context(RECOVERY)?;
    #[cfg(test)]
    if let Some((path, contents)) = fault
        .as_ref()
        .and_then(|fault| fault.concurrent_edit_before_apply.as_ref())
    {
        fs::write(path, contents)?;
    }
    apply_journaled(
        repo_root,
        &writes,
        &canonical_snapshot,
        #[cfg(test)]
        fault
            .as_ref()
            .and_then(|fault| fault.interrupt_after_boundary),
        #[cfg(not(test))]
        None,
    )
    .context(RECOVERY)?;
    Ok(report)
}

fn render_target_workflow(text: &str) -> Result<String> {
    let mut root: Value = serde_yaml::from_str(text).context("Invalid legacy workflow YAML")?;
    let mapping = root
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("Workflow root must be a mapping"))?;
    let statuses = mapping_value_mut(mapping, "statuses")?
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("Workflow statuses must be a mapping"))?;
    let plan_review_status: Value = serde_yaml::from_str("{ category: active, role: reviewer }")?;
    let plan_review_key = Value::String("plan_review".to_string());
    match statuses.get(&plan_review_key) {
        Some(existing) if existing != &plan_review_status => {
            bail!("Existing plan_review status conflicts with the cutover contract")
        }
        Some(_) => {}
        None => {
            statuses.insert(plan_review_key, plan_review_status);
        }
    }
    let workflows = mapping_value_mut(mapping, "workflows")?
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("Workflow workflows must be a mapping"))?;
    let mission = mapping_value_mut(workflows, "mission")?
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("Mission workflow must be a mapping"))?;
    let transitions = mapping_value_mut(mission, "transitions")?
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("Mission transitions must be a mapping"))?;
    if transitions.contains_key(Value::String("request_plan_review".to_string())) {
        bail!("Legacy mission workflow already defines request_plan_review");
    }
    let request_plan_review: Value = serde_yaml::from_str(
        r#"
from: [draft]
to: plan_review
description: "Submit the exact current mission graph for independent plan review."
validators: [issue.sections_parseable]
"#,
    )?;
    let preserved = std::mem::take(transitions);
    let mut target = Mapping::new();
    for (key, mut transition) in preserved {
        let name = key.as_str();
        if name == Some("ready") {
            target.insert(
                Value::String("request_plan_review".to_string()),
                request_plan_review.clone(),
            );
            update_ready_transition(&mut transition)?;
        } else if name == Some("start") {
            update_start_transition(&mut transition)?;
        }
        target.insert(key, transition);
    }
    if !target.contains_key(Value::String("request_plan_review".to_string())) {
        bail!("Legacy mission workflow is missing ready transition");
    }
    *transitions = target;
    let mut output = serde_yaml::to_string(&root)?;
    if !output.ends_with('\n') {
        output.push('\n');
    }
    Ok(output)
}

fn update_ready_transition(value: &mut Value) -> Result<()> {
    let transition = value
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("Mission ready transition must be a mapping"))?;
    transition.insert(
        Value::String("from".to_string()),
        serde_yaml::from_str("[plan_review]")?,
    );
    transition.insert(
        Value::String("to".to_string()),
        Value::String("ready".to_string()),
    );
    transition.insert(
        Value::String("description".to_string()),
        Value::String(
            "Make an independently approved exact mission graph ready for execution.".to_string(),
        ),
    );
    update_validators(
        transition,
        &["issue.sections_parseable"],
        &[
            "plan_review.current_approval",
            "blockers.transitive_none_open",
        ],
    )
}

fn update_start_transition(value: &mut Value) -> Result<()> {
    let transition = value
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("Mission start transition must be a mapping"))?;
    transition.insert(
        Value::String("from".to_string()),
        serde_yaml::from_str("[ready]")?,
    );
    transition.insert(
        Value::String("to".to_string()),
        Value::String("in_progress".to_string()),
    );
    transition.insert(
        Value::String("description".to_string()),
        Value::String("Start coordinated mission work.".to_string()),
    );
    update_validators(
        transition,
        &["baseline.default_checks"],
        &[
            "plan_review.current_approval",
            "blockers.transitive_none_open",
            "git.worktree_clean",
        ],
    )?;
    ensure_sequence_entry(transition, "actions", "git.prepare_branch")
}

fn update_validators(transition: &mut Mapping, removed: &[&str], required: &[&str]) -> Result<()> {
    let validators = transition
        .entry(Value::String("validators".to_string()))
        .or_insert_with(|| Value::Sequence(Vec::new()))
        .as_sequence_mut()
        .ok_or_else(|| anyhow!("Mission transition validators must be a sequence"))?;
    validators.retain(|validator| {
        validator_builtin(validator)
            .is_none_or(|builtin| !removed.contains(&builtin) && !required.contains(&builtin))
    });
    for builtin in required.iter().rev() {
        validators.insert(0, Value::String((*builtin).to_string()));
    }
    Ok(())
}

fn validator_builtin(value: &Value) -> Option<&str> {
    if let Some(value) = value.as_str() {
        return Some(value);
    }
    let mapping = value.as_mapping()?;
    if mapping.len() != 1 {
        return None;
    }
    mapping.keys().next()?.as_str()
}

fn ensure_sequence_entry(mapping: &mut Mapping, field: &str, required: &str) -> Result<()> {
    let sequence = mapping
        .entry(Value::String(field.to_string()))
        .or_insert_with(|| Value::Sequence(Vec::new()))
        .as_sequence_mut()
        .ok_or_else(|| anyhow!("Mission transition {field} must be a sequence"))?;
    if !sequence
        .iter()
        .any(|value| value.as_str() == Some(required))
    {
        sequence.push(Value::String(required.to_string()));
    }
    Ok(())
}

fn mapping_value_mut<'a>(mapping: &'a mut Mapping, key: &str) -> Result<&'a mut Value> {
    mapping
        .get_mut(Value::String(key.to_string()))
        .ok_or_else(|| anyhow!("Workflow is missing {key}"))
}

fn validate_staged(repo_root: &Path, writes: &[PlannedWrite]) -> Result<()> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let stage_root = repo_root.join(format!(
        ".atelier/runtime/mission-plan-review-stage-{}-{nonce}",
        process::id()
    ));
    let stage_state = stage_root.join(".atelier");
    copy_tree(&repo_root.join(".atelier"), &stage_state)?;
    let validation = (|| -> Result<()> {
        for write in writes {
            let relative = write.path.strip_prefix(repo_root)?;
            let target = stage_root.join(relative);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(target, &write.contents)?;
        }
        let db_path = stage_root.join("cache.sqlite3");
        crate::rebuild::run(&stage_state, &db_path)?;
        let db = atelier_sqlite::Database::open(&db_path)?;
        crate::workflow_policy::check(&db, &stage_root)?;
        Ok(())
    })();
    let cleanup = fs::remove_dir_all(&stage_root);
    validation.and(cleanup.context("Failed to remove migration staging directory"))
}

fn copy_tree(source: &Path, target: &Path) -> Result<()> {
    fn copy_canonical(root: &Path, source: &Path, target: &Path) -> Result<()> {
        fs::create_dir_all(target)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let source_path = entry.path();
            let relative = source_path.strip_prefix(root)?;
            if crate::storage_layout::is_local_atelier_path(relative) {
                continue;
            }
            let target_path = target.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_canonical(root, &source_path, &target_path)?;
            } else if entry.file_type()?.is_file() {
                fs::copy(source_path, target_path)?;
            }
        }
        Ok(())
    }
    copy_canonical(source, source, target)
}

fn capture_originals(writes: &mut [PlannedWrite]) -> Result<()> {
    for write in writes {
        let original = fs::read(&write.path).ok();
        if write.must_not_exist && original.is_some() {
            bail!(
                "Refusing to overwrite cutover file {}",
                write.path.display()
            );
        }
        if !write.must_not_exist && original.is_none() {
            bail!("Cutover source disappeared: {}", write.path.display());
        }
        write.original = original;
    }
    Ok(())
}

fn canonical_snapshot(state_dir: &Path) -> Result<CanonicalSnapshot> {
    fn collect(state_dir: &Path, dir: &Path, snapshot: &mut CanonicalSnapshot) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(state_dir)?.to_path_buf();
            if crate::storage_layout::is_local_atelier_path(&relative) {
                continue;
            }
            let file_type = entry.file_type()?;
            if file_type.is_symlink() {
                bail!(
                    "Canonical cutover snapshots reject symlinks at {}",
                    relative.display()
                );
            }
            if file_type.is_dir() {
                snapshot.directories.insert(relative);
                collect(state_dir, &path, snapshot)?;
            } else if file_type.is_file() {
                snapshot.files.insert(relative, fs::read(path)?);
            } else {
                bail!(
                    "Canonical cutover snapshots reject special path {}",
                    relative.display()
                );
            }
        }
        Ok(())
    }

    let mut snapshot = CanonicalSnapshot::default();
    collect(state_dir, state_dir, &mut snapshot)?;
    Ok(snapshot)
}

fn snapshot_manifest(snapshot: &CanonicalSnapshot) -> Vec<CanonicalSnapshotEntry> {
    let mut entries = snapshot
        .directories
        .iter()
        .map(|path| CanonicalSnapshotEntry {
            relative_path: path.to_string_lossy().into_owned(),
            path_type: "directory".to_string(),
            sha256: None,
        })
        .chain(
            snapshot
                .files
                .iter()
                .map(|(path, contents)| CanonicalSnapshotEntry {
                    relative_path: path.to_string_lossy().into_owned(),
                    path_type: "file".to_string(),
                    sha256: Some(lower_hex(&Sha256::digest(contents))),
                }),
        )
        .collect::<Vec<_>>();
    entries.sort();
    entries
}

fn lower_hex(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut encoded, "{byte:02x}").expect("writing to a String cannot fail");
    }
    encoded
}

fn expected_snapshot(
    state_dir: &Path,
    original: &CanonicalSnapshot,
    writes: &[PlannedWrite],
    workflow_activated: bool,
) -> Result<CanonicalSnapshot> {
    let mut expected = original.clone();
    for write in writes {
        let relative = write.path.strip_prefix(state_dir)?.to_path_buf();
        if relative == Path::new("workflow.yaml") && !workflow_activated {
            continue;
        }
        if let Some(parent) = relative.parent() {
            let mut current = PathBuf::new();
            for component in parent.components() {
                current.push(component);
                expected.directories.insert(current.clone());
            }
        }
        expected.files.insert(relative, write.contents.clone());
    }
    Ok(expected)
}

fn apply_journaled(
    repo_root: &Path,
    writes: &[PlannedWrite],
    original_snapshot: &CanonicalSnapshot,
    interrupt_after_boundary: Option<usize>,
) -> Result<()> {
    let state_dir = repo_root.join(".atelier");
    let post_snapshot = expected_snapshot(&state_dir, original_snapshot, writes, true)?;
    prepare_cutover_journal(&state_dir, writes, original_snapshot, &post_snapshot)?;
    if interrupt_after_boundary == Some(0) {
        bail!("injected abrupt migration interruption after durable journal");
    }

    let workflow_index = writes
        .iter()
        .position(|write| write.path == state_dir.join("workflow.yaml"))
        .ok_or_else(|| anyhow!("Cutover plan is missing workflow activation"))?;
    let result = (|| -> Result<()> {
        let mut boundary = 0;
        for (index, write) in writes.iter().enumerate() {
            if index == workflow_index {
                continue;
            }
            compare_original(write)?;
            atomic_replace(&write.path, &write.contents)?;
            boundary += 1;
            if interrupt_after_boundary == Some(boundary) {
                bail!("injected abrupt migration interruption at write boundary {boundary}");
            }
        }

        let expected = expected_snapshot(&state_dir, original_snapshot, writes, false)?;
        let actual = canonical_snapshot(&state_dir)?;
        if actual != expected {
            bail!(
                "Canonical tracker changed after staged validation; refusing workflow activation"
            );
        }
        let workflow = &writes[workflow_index];
        compare_original(workflow)?;
        atomic_replace(&workflow.path, &workflow.contents)?;
        boundary += 1;
        if interrupt_after_boundary == Some(boundary) {
            bail!("injected abrupt migration interruption at workflow activation boundary");
        }
        let expected = expected_snapshot(&state_dir, original_snapshot, writes, true)?;
        if canonical_snapshot(&state_dir)? != expected {
            bail!("Canonical tracker changed during workflow activation");
        }
        Ok(())
    })();

    if let Err(error) = result {
        if interrupt_after_boundary.is_some() {
            // Simulate process loss: leave the durable journal and partial
            // writes for the next invocation's recovery path.
            return Err(error);
        }
        recover_cutover_journal(repo_root)?;
        return Err(error);
    }
    remove_cutover_journal(&state_dir)?;
    Ok(())
}

fn compare_original(write: &PlannedWrite) -> Result<()> {
    let current = fs::read(&write.path).ok();
    if current != write.original {
        bail!(
            "Concurrent canonical mutation detected at {}; refusing cutover",
            write.path.display()
        );
    }
    Ok(())
}

fn prepare_cutover_journal(
    state_dir: &Path,
    writes: &[PlannedWrite],
    original_snapshot: &CanonicalSnapshot,
    post_snapshot: &CanonicalSnapshot,
) -> Result<()> {
    let final_dir = state_dir.join(JOURNAL_DIR);
    if final_dir.exists() {
        bail!(
            "Cutover recovery journal already exists at {}",
            final_dir.display()
        );
    }
    let temp_dir = state_dir.join(format!("{JOURNAL_DIR}.{}.tmp", process::id()));
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;
    let mut entries = Vec::new();
    for (index, write) in writes.iter().enumerate() {
        compare_original(write)?;
        let target_path = temp_dir.join(format!("{index:04}.target"));
        write_synced(&target_path, &write.contents)?;
        if let Some(original) = &write.original {
            write_synced(&temp_dir.join(format!("{index:04}.original")), original)?;
        }
        entries.push(CutoverJournalEntry {
            relative_path: write
                .path
                .strip_prefix(state_dir)?
                .to_string_lossy()
                .into_owned(),
            original_existed: write.original.is_some(),
            workflow_activation: write.path == state_dir.join("workflow.yaml"),
        });
    }
    let journal = CutoverJournal {
        schema: JOURNAL_SCHEMA.to_string(),
        schema_version: 1,
        entries,
        original_snapshot: snapshot_manifest(original_snapshot),
        post_snapshot: snapshot_manifest(post_snapshot),
    };
    write_synced(
        &temp_dir.join("journal.yaml"),
        serde_yaml::to_string(&journal)?.as_bytes(),
    )?;
    sync_dir(&temp_dir)?;
    fs::rename(&temp_dir, &final_dir)?;
    sync_dir(final_dir.parent().expect("journal has parent"))?;
    Ok(())
}

fn recover_cutover_journal(repo_root: &Path) -> Result<()> {
    let state_dir = repo_root.join(".atelier");
    let journal_dir = state_dir.join(JOURNAL_DIR);
    if !journal_dir.exists() {
        return Ok(());
    }
    let journal: CutoverJournal =
        serde_yaml::from_str(&fs::read_to_string(journal_dir.join("journal.yaml"))?)?;
    if journal.schema != JOURNAL_SCHEMA || journal.schema_version != 1 || journal.entries.is_empty()
    {
        bail!("Invalid mission-plan cutover recovery journal");
    }
    validate_snapshot_manifest(&journal.original_snapshot)?;
    validate_snapshot_manifest(&journal.post_snapshot)?;
    let mut loaded = Vec::new();
    for (index, entry) in journal.entries.iter().enumerate() {
        let relative = PathBuf::from(&entry.relative_path);
        if relative.is_absolute()
            || relative
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            bail!("Invalid cutover journal path {}", entry.relative_path);
        }
        let original = if entry.original_existed {
            Some(fs::read(journal_dir.join(format!("{index:04}.original")))?)
        } else {
            None
        };
        let target = fs::read(journal_dir.join(format!("{index:04}.target")))?;
        loaded.push((
            state_dir.join(relative),
            original,
            target,
            entry.workflow_activation,
        ));
    }

    let live_snapshot = canonical_snapshot(&state_dir)?;
    let exact_post = snapshot_manifest(&live_snapshot) == journal.post_snapshot;
    if exact_post {
        remove_cutover_journal(&state_dir)?;
        return Ok(());
    }

    let original_entries = journal
        .original_snapshot
        .iter()
        .map(|entry| (PathBuf::from(&entry.relative_path), entry))
        .collect::<BTreeMap<_, _>>();
    let post_entries = journal
        .post_snapshot
        .iter()
        .map(|entry| (PathBuf::from(&entry.relative_path), entry))
        .collect::<BTreeMap<_, _>>();
    let live_manifest = snapshot_manifest(&live_snapshot);
    let live_entries = live_manifest
        .iter()
        .map(|entry| (PathBuf::from(&entry.relative_path), entry))
        .collect::<BTreeMap<_, _>>();
    let all_paths = original_entries
        .keys()
        .chain(post_entries.keys())
        .chain(live_entries.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for path in all_paths {
        let original = original_entries.get(&path).copied();
        let post = post_entries.get(&path).copied();
        if original != post {
            // Planned files and their created parent directories may be in
            // either journal-owned state until recovery chooses pre or post.
            continue;
        }
        let live = live_entries.get(&path).copied();
        if live == original {
            continue;
        }
        let drift = match (original, live) {
            (Some(_), None) => "deletion",
            (None, Some(_)) => "addition",
            _ => "concurrent bytes or path type",
        };
        bail!(
            "Cutover recovery found unowned {drift} at {}; manual recovery required",
            path.display()
        );
    }

    // Deactivate the target policy first, then restore prerequisites. Every
    // current byte sequence must be journal-owned; external drift is never
    // overwritten by recovery.
    loaded.sort_by_key(|(_, _, _, workflow)| !*workflow);
    for (path, original, target, _) in &loaded {
        let current = fs::read(path).ok();
        if current.as_ref() != original.as_ref() && current.as_ref() != Some(target) {
            bail!(
                "Cutover recovery found unowned concurrent bytes at {}; manual recovery required",
                path.display()
            );
        }
        match original {
            Some(bytes) => atomic_replace(path, bytes)?,
            None if path.exists() => {
                fs::remove_file(path)?;
                sync_dir(path.parent().expect("cutover path has parent"))?;
            }
            None => {}
        }
    }
    let mut created_directories = journal
        .post_snapshot
        .iter()
        .filter(|entry| {
            entry.path_type == "directory"
                && !original_entries.contains_key(Path::new(&entry.relative_path))
        })
        .map(|entry| PathBuf::from(&entry.relative_path))
        .collect::<Vec<_>>();
    created_directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    for relative in created_directories {
        let path = state_dir.join(relative);
        if path.is_dir() && fs::read_dir(&path)?.next().is_none() {
            fs::remove_dir(&path)?;
            sync_dir(path.parent().expect("cutover directory has parent"))?;
        }
    }
    if snapshot_manifest(&canonical_snapshot(&state_dir)?) != journal.original_snapshot {
        bail!("Cutover recovery did not restore the exact original canonical snapshot");
    }
    remove_cutover_journal(&state_dir)
}

fn validate_snapshot_manifest(entries: &[CanonicalSnapshotEntry]) -> Result<()> {
    if entries
        .windows(2)
        .any(|pair| pair[0].relative_path >= pair[1].relative_path)
    {
        bail!("Cutover snapshot manifest paths must be sorted and unique");
    }
    for entry in entries {
        let path = Path::new(&entry.relative_path);
        if path.is_absolute()
            || path
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
            || match (entry.path_type.as_str(), entry.sha256.as_deref()) {
                ("directory", None) => false,
                ("file", Some(digest)) => {
                    digest.len() != 64
                        || !digest
                            .bytes()
                            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                }
                _ => true,
            }
        {
            bail!(
                "Invalid cutover canonical snapshot entry {}",
                entry.relative_path
            );
        }
    }
    Ok(())
}

fn remove_cutover_journal(state_dir: &Path) -> Result<()> {
    let journal_dir = state_dir.join(JOURNAL_DIR);
    if journal_dir.exists() {
        let cleanup = state_dir.join(format!("{JOURNAL_DIR}.{}.complete", process::id()));
        if cleanup.exists() {
            fs::remove_dir_all(&cleanup)?;
        }
        fs::rename(&journal_dir, &cleanup)?;
        sync_dir(journal_dir.parent().expect("journal has parent"))?;
        fs::remove_dir_all(cleanup)?;
    }
    Ok(())
}

fn atomic_replace(path: &Path, contents: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("Path has no parent: {}", path.display()))?;
    fs::create_dir_all(parent)?;
    let temp = path.with_extension(format!("migration-{}-tmp", process::id()));
    write_synced(&temp, contents)?;
    fs::rename(&temp, path)?;
    sync_dir(parent)
}

fn write_synced(path: &Path, contents: &[u8]) -> Result<()> {
    fs::write(path, contents)?;
    File::open(path)?.sync_all()?;
    Ok(())
}

fn sync_dir(path: &Path) -> Result<()> {
    File::open(path)?.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_core::Issue;
    use atelier_records::{CanonicalIssueRecord, IssueSections, Relationships};
    use chrono::TimeZone;
    use std::collections::BTreeMap;
    use tempfile::TempDir;

    fn at() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 7, 16, 12, 34, 56)
            .single()
            .unwrap()
            .with_nanosecond(123_456_000)
            .unwrap()
    }

    fn fixture(statuses: &[(&str, &str)]) -> TempDir {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".atelier/issues")).unwrap();
        fs::write(dir.path().join(".atelier/workflow.yaml"), legacy_workflow()).unwrap();
        let store = RecordStore::new(dir.path().join(".atelier"));
        for (id, status) in statuses {
            let closed_at = (*status == "closed").then_some(at());
            store
                .write_issue_atomic(&CanonicalIssueRecord {
                    issue: Issue {
                        id: (*id).to_string(),
                        title: format!("Mission {id}"),
                        description: Some("Material plan".to_string()),
                        status: (*status).to_string(),
                        issue_type: "mission".to_string(),
                        priority: "medium".to_string(),
                        fields: BTreeMap::new(),
                        parent_id: None,
                        created_at: at(),
                        updated_at: at(),
                        closed_at,
                    },
                    labels: vec!["medium".to_string()],
                    sections: IssueSections {
                        description: "Material plan".to_string(),
                        outcome: "Completed outcome".to_string(),
                        evidence: String::new(),
                        notes: None,
                    },
                    relationships: Relationships::default(),
                })
                .unwrap();
        }
        dir
    }

    fn legacy_workflow() -> String {
        let mut root: Value =
            serde_yaml::from_str(crate::workflow_policy::STARTER_POLICY_YAML).unwrap();
        let statuses = root["statuses"].as_mapping_mut().unwrap();
        statuses.remove(Value::String("plan_review".to_string()));
        let transitions = root["workflows"]["mission"]["transitions"]
            .as_mapping_mut()
            .unwrap();
        transitions.remove(Value::String("request_plan_review".to_string()));
        transitions
            .get_mut(Value::String("ready".to_string()))
            .unwrap()["from"] = serde_yaml::from_str("[draft]").unwrap();
        transitions
            .get_mut(Value::String("ready".to_string()))
            .unwrap()["validators"] =
            serde_yaml::from_str("[issue.sections_parseable, blockers.transitive_none_open]")
                .unwrap();
        transitions
            .get_mut(Value::String("start".to_string()))
            .unwrap()["validators"] =
            serde_yaml::from_str("[baseline.default_checks, blockers.transitive_none_open]")
                .unwrap();
        serde_yaml::to_string(&root).unwrap()
    }

    fn bytes(path: &Path) -> Vec<u8> {
        fs::read(path).unwrap()
    }

    #[test]
    fn target_workflow_preserves_custom_policy_and_installs_review_gate() {
        let input = r#"
schema: atelier.workflow
schema_version: 3
branch_policy: { base_branch: custom, merge_strategy: squash }
statuses:
  draft: { category: todo }
  ready: { category: todo }
  in_progress: { category: active, role: worker }
  closed: { category: done }
  superseded: { category: done }
workflows:
  mission:
    applies_to: [mission]
    initial_status: draft
    done_statuses: [closed, superseded]
    transitions:
      archive_draft:
        from: [draft]
        to: superseded
        validators: [custom.archive]
        actions: [custom.archive_action]
      ready:
        from: [draft]
        to: ready
        validators: [issue.sections_parseable, custom.ready, blockers.transitive_none_open]
        actions: [custom.ready_action]
      start:
        from: [ready]
        to: in_progress
        validators: [baseline.default_checks, custom.start, blockers.transitive_none_open]
        actions: [custom.start_action, git.prepare_branch]
      request_publish:
        from: [in_progress]
        to: closed
        validators: [custom.publish]
        actions: [custom.publish_action]
"#;
        let before: Value = serde_yaml::from_str(input).unwrap();
        let rendered = render_target_workflow(input).unwrap();
        let value: Value = serde_yaml::from_str(&rendered).unwrap();
        assert_eq!(value["branch_policy"]["base_branch"], "custom");
        assert_eq!(
            value["workflows"]["mission"]["transitions"]["archive_draft"],
            before["workflows"]["mission"]["transitions"]["archive_draft"]
        );
        assert_eq!(
            serde_yaml::to_string(&value["workflows"]["mission"]["transitions"]["archive_draft"])
                .unwrap(),
            serde_yaml::to_string(&before["workflows"]["mission"]["transitions"]["archive_draft"])
                .unwrap()
        );
        assert_eq!(
            value["workflows"]["mission"]["transitions"]["request_publish"],
            before["workflows"]["mission"]["transitions"]["request_publish"]
        );
        let mut statuses_after = value["statuses"].as_mapping().unwrap().clone();
        statuses_after.remove(Value::String("plan_review".to_string()));
        assert_eq!(statuses_after, *before["statuses"].as_mapping().unwrap());
        assert_eq!(
            value["workflows"]["mission"]["transitions"]["ready"]["from"][0],
            "plan_review"
        );
        assert!(
            value["workflows"]["mission"]["transitions"]["ready"]["validators"]
                .as_sequence()
                .unwrap()
                .contains(&Value::String("custom.ready".to_string()))
        );
        assert_eq!(
            value["workflows"]["mission"]["transitions"]["ready"]["actions"],
            before["workflows"]["mission"]["transitions"]["ready"]["actions"]
        );
        assert_eq!(
            value["workflows"]["mission"]["transitions"]["start"]["validators"][0],
            "plan_review.current_approval"
        );
        assert!(
            value["workflows"]["mission"]["transitions"]["start"]["validators"]
                .as_sequence()
                .unwrap()
                .contains(&Value::String("custom.start".to_string()))
        );
        assert_eq!(
            value["workflows"]["mission"]["transitions"]["start"]["actions"],
            before["workflows"]["mission"]["transitions"]["start"]["actions"]
        );
        let transition_order = value["workflows"]["mission"]["transitions"]
            .as_mapping()
            .unwrap()
            .keys()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>();
        assert_eq!(
            transition_order,
            vec![
                "archive_draft",
                "request_plan_review",
                "ready",
                "start",
                "request_publish"
            ]
        );
    }

    #[test]
    fn migrates_each_legacy_class_and_is_idempotent() {
        let dir = fixture(&[
            ("atelier-draft", "draft"),
            ("atelier-ready", "ready"),
            ("atelier-active", "in_progress"),
            ("atelier-done", "closed"),
        ]);
        let state = dir.path().join(".atelier");
        let draft_path = state.join(issue_record_path("atelier-draft"));
        let done_path = state.join(issue_record_path("atelier-done"));
        let draft_before = bytes(&draft_path);
        let done_before = bytes(&done_path);
        let report = migrate_at(dir.path(), at(), None).unwrap();
        assert_eq!(
            (
                report.mission_count,
                report.terminal_count,
                report.draft_count,
                report.plan_review_count,
                report.active_count
            ),
            (4, 1, 1, 1, 1)
        );
        assert_eq!(bytes(&draft_path), draft_before);
        assert_eq!(bytes(&done_path), done_before);
        assert_eq!(
            RecordStore::new(&state)
                .load_issue_by_id("atelier-ready")
                .unwrap()
                .issue
                .status,
            "plan_review"
        );
        let manifest =
            atelier_records::mission_plan_review::load_mission_plan_review_cutover_manifest(&state)
                .unwrap()
                .unwrap();
        assert_eq!(manifest.eligible_missions.len(), 1);
        assert_eq!(manifest.eligible_missions[0].mission_id, "atelier-active");
        let activity = state.join(record_activity_path(
            "issue",
            "atelier-active",
            &timestamp_activity_id(at()),
        ));
        assert!(activity.exists());
        assert!(bytes(&activity)
            .windows(LEGACY_GRANDFATHER_MIGRATION_ACTOR.len())
            .any(|window| window == LEGACY_GRANDFATHER_MIGRATION_ACTOR.as_bytes()));
        let snapshot = [
            draft_path,
            done_path,
            state.join("issues/atelier-ready.md"),
            state.join(MISSION_PLAN_REVIEW_CUTOVER_MANIFEST_PATH),
            activity,
        ]
        .map(|path| (path.clone(), bytes(&path)));
        let repeated = migrate_at(dir.path(), at(), None).unwrap();
        assert!(repeated.already_applied);
        for (path, expected) in snapshot {
            assert_eq!(bytes(&path), expected);
        }
        let fresh = atelier_records::mission_plan_review::mission_plan_review_state(
            &state,
            "atelier-active",
        )
        .unwrap();
        assert_eq!(
            fresh.freshness,
            atelier_records::mission_plan_review::MissionPlanReviewFreshness::FreshGrandfather
        );
        let mut edited = RecordStore::new(&state)
            .load_issue_by_id("atelier-active")
            .unwrap();
        edited.sections.description.push_str(" with material edit");
        RecordStore::new(&state)
            .write_issue_atomic(&edited)
            .unwrap();
        let stale = atelier_records::mission_plan_review::mission_plan_review_state(
            &state,
            "atelier-active",
        )
        .unwrap();
        assert_eq!(
            stale.freshness,
            atelier_records::mission_plan_review::MissionPlanReviewFreshness::Stale
        );
    }

    #[test]
    fn no_active_mission_omits_empty_manifest() {
        let dir = fixture(&[
            ("atelier-draft", "draft"),
            ("atelier-ready", "ready"),
            ("atelier-done", "closed"),
        ]);
        migrate_at(dir.path(), at(), None).unwrap();
        assert!(!dir
            .path()
            .join(".atelier")
            .join(MISSION_PLAN_REVIEW_CUTOVER_MANIFEST_PATH)
            .exists());
    }

    #[test]
    fn collision_and_unknown_status_fail_without_changes() {
        let dir = fixture(&[("atelier-active", "in_progress")]);
        let state = dir.path().join(".atelier");
        let collision = state.join(record_activity_path(
            "issue",
            "atelier-active",
            &timestamp_activity_id(at()),
        ));
        fs::create_dir_all(collision.parent().unwrap()).unwrap();
        fs::write(&collision, "collision\n").unwrap();
        let workflow_before = bytes(&state.join("workflow.yaml"));
        let error = migrate_at(dir.path(), at(), None).unwrap_err().to_string();
        assert!(error.contains("Mission-plan cutover is not complete"));
        assert_eq!(bytes(&state.join("workflow.yaml")), workflow_before);
        assert_eq!(bytes(&collision), b"collision\n");

        let dir = fixture(&[("atelier-bad", "blocked")]);
        let workflow_before = bytes(&dir.path().join(".atelier/workflow.yaml"));
        assert!(migrate_at(dir.path(), at(), None)
            .unwrap_err()
            .to_string()
            .contains("unknown legacy status"));
        assert_eq!(
            bytes(&dir.path().join(".atelier/workflow.yaml")),
            workflow_before
        );
    }

    #[test]
    fn interruption_at_every_write_boundary_recovers_to_exact_pre_or_post_state() {
        for boundary in 0..=4 {
            let dir = fixture(&[
                ("atelier-ready", "ready"),
                ("atelier-active", "in_progress"),
                ("atelier-done", "closed"),
            ]);
            let state = dir.path().join(".atelier");
            let pre = canonical_snapshot(&state).unwrap();
            let fault = MigrationFault {
                interrupt_after_boundary: Some(boundary),
                ..MigrationFault::default()
            };
            assert!(migrate_at(dir.path(), at(), Some(fault)).is_err());
            assert!(state.join(JOURNAL_DIR).exists());

            recover_cutover_journal(dir.path()).unwrap();
            if boundary < 4 {
                assert_eq!(canonical_snapshot(&state).unwrap(), pre);
            } else {
                let policy = crate::workflow_policy::load(dir.path()).unwrap();
                assert!(
                    crate::workflow_policy::enforces_independent_mission_plan_review(&policy)
                        .unwrap()
                );
            }
            assert!(!state.join(JOURNAL_DIR).exists());

            migrate_at(dir.path(), at(), None).unwrap();
            let post = canonical_snapshot(&state).unwrap();
            let activity_dir = state.join("issues/atelier-active.activity");
            assert_eq!(
                fs::read_dir(activity_dir)
                    .unwrap()
                    .filter_map(|entry| entry.ok())
                    .count(),
                1
            );
            assert!(migrate_at(dir.path(), at(), None).unwrap().already_applied);
            assert_eq!(canonical_snapshot(&state).unwrap(), post);
        }
    }

    #[test]
    fn recovery_refuses_unrelated_add_edit_and_delete_after_every_boundary() {
        for boundary in 0..=4 {
            for drift in ["add", "edit", "delete"] {
                let dir = fixture(&[
                    ("atelier-ready", "ready"),
                    ("atelier-active", "in_progress"),
                    ("atelier-done", "closed"),
                ]);
                let state = dir.path().join(".atelier");
                let pre = canonical_snapshot(&state).unwrap();
                let fault = MigrationFault {
                    interrupt_after_boundary: Some(boundary),
                    ..MigrationFault::default()
                };
                assert!(migrate_at(dir.path(), at(), Some(fault)).is_err());

                let done_path = state.join(issue_record_path("atelier-done"));
                let done_original = fs::read(&done_path).unwrap();
                let added_path = state.join("issues/unowned-canonical-addition.md");
                match drift {
                    "add" => fs::write(&added_path, b"unowned addition\n").unwrap(),
                    "edit" => fs::write(&done_path, b"unowned edit\n").unwrap(),
                    "delete" => fs::remove_file(&done_path).unwrap(),
                    _ => unreachable!(),
                }
                let error = recover_cutover_journal(dir.path()).unwrap_err().to_string();
                assert!(
                    error.contains("unowned"),
                    "{drift} boundary {boundary}: {error}"
                );
                assert!(state.join(JOURNAL_DIR).exists());
                match drift {
                    "add" => assert_eq!(fs::read(&added_path).unwrap(), b"unowned addition\n"),
                    "edit" => assert_eq!(fs::read(&done_path).unwrap(), b"unowned edit\n"),
                    "delete" => assert!(!done_path.exists()),
                    _ => unreachable!(),
                }

                // Documented operator repair restores only the named unowned
                // path. Journal-owned paths then recover deterministically.
                match drift {
                    "add" => fs::remove_file(added_path).unwrap(),
                    "edit" | "delete" => fs::write(done_path, done_original).unwrap(),
                    _ => unreachable!(),
                }
                recover_cutover_journal(dir.path()).unwrap();
                if boundary < 4 {
                    assert_eq!(canonical_snapshot(&state).unwrap(), pre);
                } else {
                    let policy = crate::workflow_policy::load(dir.path()).unwrap();
                    assert!(
                        crate::workflow_policy::enforces_independent_mission_plan_review(&policy)
                            .unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn writer_transaction_started_before_cutover_cannot_overwrite_post_cutover_state() {
        use std::sync::{mpsc, Arc};
        use std::thread;
        use std::time::Duration;

        let dir = fixture(&[
            ("atelier-ready", "ready"),
            ("atelier-active", "in_progress"),
        ]);
        let repo_root = Arc::new(dir.path().to_path_buf());
        let state = repo_root.join(".atelier");
        let (read_tx, read_rx) = mpsc::channel();
        let (write_tx, write_rx) = mpsc::channel();
        let writer_root = Arc::clone(&repo_root);
        let writer = thread::spawn(move || {
            let writer_state = writer_root.join(".atelier");
            let _transaction =
                atelier_records::mutation_lock::CanonicalMutationLock::shared(&writer_state)
                    .unwrap();
            let mut record = RecordStore::new(&writer_state)
                .load_issue_by_id("atelier-ready")
                .unwrap();
            record.issue.title = "Writer-owned pre-cutover edit".to_string();
            read_tx.send(()).unwrap();
            write_rx.recv().unwrap();
            RecordStore::new(&writer_state)
                .write_issue_atomic(&record)
                .unwrap();
        });
        read_rx.recv().unwrap();

        let (migration_tx, migration_rx) = mpsc::channel();
        let migration_root = Arc::clone(&repo_root);
        let migration = thread::spawn(move || {
            let result = migrate_at(&migration_root, at(), None);
            migration_tx.send(result).unwrap();
        });
        assert!(migration_rx
            .recv_timeout(Duration::from_millis(100))
            .is_err());
        write_tx.send(()).unwrap();
        writer.join().unwrap();
        migration_rx
            .recv_timeout(Duration::from_secs(5))
            .unwrap()
            .unwrap();
        migration.join().unwrap();

        let migrated = RecordStore::new(&state)
            .load_issue_by_id("atelier-ready")
            .unwrap();
        assert_eq!(migrated.issue.status, "plan_review");
        assert_eq!(migrated.issue.title, "Writer-owned pre-cutover edit");
    }

    #[test]
    fn concurrent_mutation_is_rejected_without_overwrite_or_activation() {
        let dir = fixture(&[
            ("atelier-ready", "ready"),
            ("atelier-active", "in_progress"),
        ]);
        let state = dir.path().join(".atelier");
        let ready_path = state.join(issue_record_path("atelier-ready"));
        let workflow_before = bytes(&state.join("workflow.yaml"));
        let mut concurrent = RecordStore::new(&state)
            .load_issue_by_id("atelier-ready")
            .unwrap();
        concurrent.issue.title = "Concurrent valid edit".to_string();
        let concurrent_bytes = render_issue_record(&concurrent).unwrap().into_bytes();
        let fault = MigrationFault {
            concurrent_edit_before_apply: Some((ready_path.clone(), concurrent_bytes.clone())),
            ..MigrationFault::default()
        };
        let error = migrate_at(dir.path(), at(), Some(fault))
            .unwrap_err()
            .to_string();
        assert!(error.contains("Mission-plan cutover is not complete"));
        assert_eq!(bytes(&ready_path), concurrent_bytes);
        assert_eq!(bytes(&state.join("workflow.yaml")), workflow_before);
        assert!(!state.join(JOURNAL_DIR).exists());
        assert!(!state
            .join(MISSION_PLAN_REVIEW_CUTOVER_MANIFEST_PATH)
            .exists());
    }
}
