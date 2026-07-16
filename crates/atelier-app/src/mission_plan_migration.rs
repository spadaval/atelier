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
use serde_yaml::{Mapping, Value};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

const RECOVERY: &str = "No canonical cutover was applied. Repair the reported .atelier file, run `atelier check --fix`, then retry `atelier migrate-mission-plan-review`.";

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
    fail_after_write: Option<usize>,
) -> Result<MigrationReport> {
    let state_dir = repo_root.join(".atelier");
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
            });
        }
    }

    let workflow_path = state_dir.join("workflow.yaml");
    let workflow_text = fs::read_to_string(&workflow_path)?;
    writes.push(PlannedWrite {
        path: workflow_path,
        contents: render_target_workflow(&workflow_text)?.into_bytes(),
        must_not_exist: false,
    });

    validate_staged(repo_root, &writes).context(RECOVERY)?;
    apply_with_rollback(&writes, fail_after_write).context(RECOVERY)?;
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
    statuses.insert(
        Value::String("plan_review".to_string()),
        serde_yaml::from_str("{ category: active, role: reviewer }")?,
    );
    let workflows = mapping_value_mut(mapping, "workflows")?
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("Workflow workflows must be a mapping"))?;
    let mission = mapping_value_mut(workflows, "mission")?
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("Mission workflow must be a mapping"))?;
    let transitions = mapping_value_mut(mission, "transitions")?
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("Mission transitions must be a mapping"))?;
    let publish = transitions
        .remove(Value::String("request_publish".to_string()))
        .ok_or_else(|| anyhow!("Legacy mission workflow is missing request_publish"))?;
    let target: Mapping = serde_yaml::from_str::<Value>(
        r#"
request_plan_review:
  from: [draft]
  to: plan_review
  description: "Submit the exact current mission graph for independent plan review."
  validators: [issue.sections_parseable]
ready:
  from: [plan_review]
  to: ready
  description: "Make an independently approved exact mission graph ready for execution."
  validators: [plan_review.current_approval, blockers.transitive_none_open]
start:
  from: [ready]
  to: in_progress
  description: "Start coordinated mission work."
  validators: [plan_review.current_approval, blockers.transitive_none_open, git.worktree_clean]
  actions: [git.prepare_branch]
"#,
    )?
    .as_mapping()
    .cloned()
    .unwrap();
    *transitions = target;
    transitions.insert(Value::String("request_publish".to_string()), publish);
    let mut output = serde_yaml::to_string(&root)?;
    if !output.ends_with('\n') {
        output.push('\n');
    }
    Ok(output)
}

fn mapping_value_mut<'a>(mapping: &'a mut Mapping, key: &str) -> Result<&'a mut Value> {
    mapping
        .get_mut(Value::String(key.to_string()))
        .ok_or_else(|| anyhow!("Workflow is missing {key}"))
}

fn validate_staged(repo_root: &Path, writes: &[PlannedWrite]) -> Result<()> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let stage_root = repo_root.join(format!(
        ".atelier-migration-stage-{}-{nonce}",
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
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&source_path, &target_path)?;
        } else if entry.file_type()?.is_file() {
            fs::copy(source_path, target_path)?;
        }
    }
    Ok(())
}

fn apply_with_rollback(writes: &[PlannedWrite], fail_after_write: Option<usize>) -> Result<()> {
    let originals = writes
        .iter()
        .map(|write| {
            if write.must_not_exist && write.path.exists() {
                bail!(
                    "Refusing to overwrite cutover file {}",
                    write.path.display()
                );
            }
            Ok((write.path.clone(), fs::read(&write.path).ok()))
        })
        .collect::<Result<Vec<_>>>()?;
    let result = (|| -> Result<()> {
        for (index, write) in writes.iter().enumerate() {
            if fail_after_write == Some(index) {
                bail!("injected migration apply failure");
            }
            if let Some(parent) = write.path.parent() {
                fs::create_dir_all(parent)?;
            }
            let temp = write
                .path
                .with_extension(format!("migration-{}-tmp", process::id()));
            fs::write(&temp, &write.contents)?;
            fs::rename(&temp, &write.path)?;
        }
        Ok(())
    })();
    if let Err(error) = result {
        for (path, original) in originals.iter().rev() {
            match original {
                Some(bytes) => fs::write(path, bytes)?,
                None if path.exists() => fs::remove_file(path)?,
                None => {}
            }
        }
        return Err(error);
    }
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
        let rendered = render_target_workflow(
            r#"
schema: atelier.workflow
schema_version: 3
branch_policy: { base_branch: custom, merge_strategy: squash }
statuses:
  draft: { category: todo }
  ready: { category: todo }
  in_progress: { category: active, role: worker }
  closed: { category: done }
workflows:
  mission:
    applies_to: [mission]
    initial_status: draft
    done_statuses: [closed]
    transitions:
      ready: { from: [draft], to: ready }
      start: { from: [ready], to: in_progress }
      request_publish: { from: [in_progress], to: closed }
"#,
        )
        .unwrap();
        let value: Value = serde_yaml::from_str(&rendered).unwrap();
        assert_eq!(value["branch_policy"]["base_branch"], "custom");
        assert_eq!(
            value["workflows"]["mission"]["transitions"]["ready"]["from"][0],
            "plan_review"
        );
        assert_eq!(
            value["workflows"]["mission"]["transitions"]["start"]["validators"][0],
            "plan_review.current_approval"
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
        assert!(error.contains("No canonical cutover was applied"));
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
    fn apply_failure_rolls_back_every_canonical_write() {
        let dir = fixture(&[
            ("atelier-ready", "ready"),
            ("atelier-active", "in_progress"),
            ("atelier-done", "closed"),
        ]);
        let state = dir.path().join(".atelier");
        let ready = state.join(issue_record_path("atelier-ready"));
        let workflow = state.join("workflow.yaml");
        let before = (bytes(&ready), bytes(&workflow));
        assert!(migrate_at(dir.path(), at(), Some(2)).is_err());
        assert_eq!((bytes(&ready), bytes(&workflow)), before);
        assert!(!state
            .join(MISSION_PLAN_REVIEW_CUTOVER_MANIFEST_PATH)
            .exists());
        assert!(!state
            .join(record_activity_path(
                "issue",
                "atelier-active",
                &timestamp_activity_id(at())
            ))
            .exists());
    }
}
