use anyhow::{bail, Result};
use chrono::{DateTime, Local, Utc};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::commands::agent_factory::{classify_requirement_coverage, ProofCoverageStatus};
use crate::db::Database;
use crate::models::{DomainRecord, Issue, RecordLink};
use crate::record_store::{self, RecordStore, MISSION_EMPTY_DATA_JSON};

const KIND: &str = "mission";
pub fn create(
    state_dir: &Path,
    db_path: &Path,
    title: &str,
    body: Option<&str>,
    constraints: Vec<String>,
    risks: Vec<String>,
    validation: Vec<String>,
) -> Result<()> {
    let sections =
        record_store::mission_sections_from_inputs(title, body, constraints, risks, validation);
    let mission_body = record_store::render_mission_sections(&sections);
    let store = RecordStore::new(state_dir);
    let created = store.create_domain_record(
        KIND,
        title,
        "ready",
        Some(&mission_body),
        MISSION_EMPTY_DATA_JSON,
    )?;
    refresh_projection(state_dir, db_path)?;
    let db = Database::open(db_path)?;
    let record = db.require_record(KIND, &created.record.id)?;
    print_record(&record)
}

pub fn show(db: &Database, id: &str) -> Result<()> {
    view(db, id)
}

pub fn start(state_dir: &Path, db_path: &Path, id: &str, switch_active: bool) -> Result<()> {
    let db = Database::open(db_path)?;
    let mission = db.require_record(KIND, id)?;
    let current_missions = current_mission_records(&db)?;
    let active = current_missions
        .iter()
        .filter(|record| is_active_mission(record))
        .collect::<Vec<_>>();
    let other_active = active
        .iter()
        .find(|record| record.id != mission.id)
        .map(|record| record.id.clone());
    if let Some(other_active) = other_active {
        if !switch_active {
            bail!(
                "Mission {} is already active. Use `atelier mission start {} --switch` to change focus.",
                other_active,
                mission.id
            );
        }
    }
    drop(db);

    let store = RecordStore::new(state_dir);
    let mut changed = false;
    for record in current_missions {
        let mut canonical = store.load_domain_record_by_id(KIND, &record.id)?;
        let should_be_active = canonical.record.id == mission.id;
        if set_mission_active_state(&mut canonical.record, should_be_active)? {
            canonical.record.updated_at = Utc::now();
            store.write_domain_record_atomic(&canonical)?;
            changed = true;
        }
    }
    if changed {
        refresh_projection(state_dir, db_path)?;
    }
    println!("Active mission: {} - {}", mission.id, mission.title);
    println!("Next Commands");
    println!("-------------");
    println!("  atelier mission status {}", mission.id);
    println!("  atelier issue list --ready");
    Ok(())
}

pub fn status(
    db: &Database,
    state_dir: &Path,
    id: Option<&str>,
    quiet: bool,
    closeout: bool,
    verbose: bool,
) -> Result<()> {
    if closeout {
        let Some(id) = id else {
            bail!("mission status --closeout requires a mission id");
        };
        return audit(db, state_dir, id, quiet);
    }
    match id {
        Some(id) => status_one(db, state_dir, id, quiet, verbose),
        None => match active_mission(db)? {
            Some(record) => status_one(db, state_dir, &record.id, quiet, verbose),
            None => status_dashboard(db, state_dir, quiet),
        },
    }
}

fn status_dashboard(db: &Database, state_dir: &Path, quiet: bool) -> Result<()> {
    let records = current_mission_records(db)?;
    let mut rows = records
        .into_iter()
        .map(|record| {
            Ok(MissionListRow {
                summary: mission_list_summary(db, &record.id)?,
                record,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    rows.sort_by(compare_mission_list_rows);
    let tracker = tracker_health(db, state_dir);

    if quiet {
        println!(
            "missions={} blocked={} closeout_needed={} tracker={}",
            rows.len(),
            rows.iter()
                .filter(|row| row.summary.open_blockers > 0 || row.summary.total_work().blocked > 0)
                .count(),
            rows.iter()
                .filter(|row| row.summary.closeout_needed())
                .count(),
            tracker.status_token()
        );
        for row in &rows {
            println!(
                "{} ready={} blocked={} done={} backlog={}",
                row.record.id,
                row.summary.total_work().ready,
                row.summary.total_work().blocked,
                row.summary.total_work().done,
                row.summary.total_work().backlog
            );
        }
        return Ok(());
    }

    println!("Mission Status");
    println!("==============");
    println!(
        "{} | tracker {}",
        mission_list_summary_line(&rows),
        tracker.status_text()
    );
    if rows.is_empty() {
        println!("(none)");
    } else {
        for row in &rows {
            let work = row.summary.total_work();
            let health = mission_health(&row.summary);
            println!(
                "  {} [{}] {} - {} | {} | evidence gaps {} | closeout {}",
                row.record.id,
                health,
                mission_focus_label(&row.record),
                row.record.title,
                work.to_compact_text(),
                row.summary.evidence_gap_count(),
                if row.summary.closeout_needed() {
                    "needed"
                } else {
                    "not ready"
                }
            );
        }
    }
    print_mission_heading("Next Commands");
    if let Some(row) = rows.first() {
        println!("  atelier mission status {}", row.record.id);
    }
    println!("  atelier mission list");
    println!("  atelier issue list --ready");
    println!("  atelier doctor");
    Ok(())
}

fn status_one(db: &Database, state_dir: &Path, id: &str, quiet: bool, verbose: bool) -> Result<()> {
    let mission = canonical_record_detail(KIND, id)?.unwrap_or(db.require_record(KIND, id)?);
    let summary = mission_list_summary(db, &mission.id)?;
    let tracker = tracker_health(db, state_dir);
    let active_work = active_work_for_mission(db, &mission.id)?;
    let closeout = mission_closeout_status(db, state_dir, &mission, &summary)?;
    let validator_failures = closeout.validator_failure_count();

    if quiet {
        let work = summary.total_work();
        println!(
            "{} health={} ready={} blocked={} done={} backlog={} blockers={} evidence_gaps={} validator_failures={} tracker={} closeout_needed={}",
            mission.id,
            mission_health_for(&mission, &summary),
            work.ready,
            work.blocked,
            work.done,
            work.backlog,
            summary.open_blockers,
            summary.evidence_gap_count(),
            validator_failures,
            tracker.status_token(),
            if mission_lifecycle_status(&mission) == "closed" {
                "complete"
            } else if closeout.ready() {
                "yes"
            } else {
                "no"
            }
        );
        return Ok(());
    }

    let identity = format!(
        "Mission Status {} [{}] - {}",
        mission.id,
        mission_focus_label(&mission),
        mission.title
    );
    println!("{identity}");
    println!("{}", "=".repeat(identity.len()));
    println!("Health:   {}", mission_health_for(&mission, &summary));
    println!("Tracker:  {}", tracker.status_text());
    println!(
        "Closeout: {}",
        if mission_lifecycle_status(&mission) == "closed" {
            "complete"
        } else if closeout.ready() {
            "ready"
        } else {
            "blocked"
        }
    );

    print_mission_heading("Work");
    println!("Total: {}", summary.total_work().to_compact_text());
    if summary.epics.is_empty() {
        println!("Epics: none");
    } else {
        for epic in &summary.epics {
            println!(
                "  [epic] {} [{}] {} - {} | {}",
                epic.issue.id,
                epic.issue.status,
                epic.issue.priority,
                epic.issue.title,
                epic.work.to_compact_text()
            );
        }
    }
    if !summary.other_work.is_empty() {
        println!("Other: {}", summary.other_work.to_compact_text());
    }

    print_mission_heading("Selectable Work");
    if summary.selectable_work.is_empty() {
        println!("(none)");
    } else {
        for issue in summary.selectable_work.iter().take(5) {
            println!(
                "  {} - {} | ready: no open blockers; {}; {}",
                issue.id,
                issue.title,
                parent_context(issue),
                proof_context(db, &issue.id)?
            );
        }
    }

    print_mission_heading("Blocked Work");
    if summary.blocked_work.is_empty() {
        println!("(none)");
    } else {
        for blocked in summary.blocked_work.iter().take(5) {
            println!(
                "  {} - {} | blocked by {}; {}; {}",
                blocked.issue.id,
                blocked.issue.title,
                compact_strings(&blocked.blockers),
                parent_context(&blocked.issue),
                proof_context(db, &blocked.issue.id)?
            );
        }
    }

    print_mission_heading("Blockers");
    if summary.open_blockers == 0 && summary.total_work().blocked == 0 {
        println!("(none)");
    } else {
        println!(
            "Mission blockers: {}",
            count_label(summary.open_blockers, "open")
        );
        println!(
            "Blocked work: {}",
            count_label(summary.total_work().blocked, "blocked")
        );
    }

    print_mission_heading("Evidence");
    if summary.evidence_count == 0 {
        println!("Gap: no evidence records are linked to this mission.");
    } else {
        println!("Linked evidence: {}", summary.evidence_count);
    }

    print_mission_heading("Reliability");
    print_reliability_summary(db, state_dir, &mission, &summary, &tracker, &closeout)?;

    print_mission_heading("Closeout Gates");
    if mission_lifecycle_status(&mission) == "closed" {
        println!("Mission is closed.");
    } else {
        closeout.print_human();
    }

    let show_advanced_validator_detail = verbose
        || closeout
            .validator_results
            .iter()
            .any(|result| !result.passed && result.validator == "ignored_tests_reviewed");
    if show_advanced_validator_detail {
        print_mission_heading("Advanced Validator Detail");
        if validator_failures == 0 {
            println!("All advanced closeout validators passed.");
        } else {
            println!(
                "{} advanced closeout validator failure detected.",
                validator_failures
            );
            for result in closeout
                .validator_results
                .iter()
                .filter(|result| !result.passed)
            {
                println!("  fail  {} - {}", result.validator, result.reason);
            }
        }
    }

    print_mission_heading("Active Work");
    if active_work.is_empty() {
        println!("(none)");
    } else {
        for work in active_work {
            println!(
                "  {} [{}] branch={} worktree={}",
                work.issue_id,
                work.status,
                work.branch.as_deref().unwrap_or("(none)"),
                work.worktree_path.as_deref().unwrap_or("(none)")
            );
        }
    }

    print_status_next_commands(&mission, &summary, &closeout);
    Ok(())
}

fn print_status_next_commands(
    mission: &DomainRecord,
    summary: &MissionListSummary,
    closeout: &MissionCloseoutStatus,
) {
    print_mission_heading("Next Commands");
    let lifecycle = mission_lifecycle_status(mission);
    println!(
        "  Inspect mission record (durable intent and linked work): atelier mission show {}",
        mission.id
    );
    match lifecycle.as_str() {
        "closed" => {
            println!(
                "  Inspect closeout audit history: atelier mission status --closeout {}",
                mission.id
            );
            println!(
                "  Inspect mission history: atelier history --mission {}",
                mission.id
            );
            return;
        }
        "draft" => {
            println!(
                "  Shape mission work or move to ready when gates permit: atelier mission update {} --status ready",
                mission.id
            );
        }
        _ => {
            println!(
                "  Refresh mission status (current blockers and closeout gates): atelier mission status {}",
                mission.id
            );
        }
    }
    if closeout.ready() {
        println!(
            "  Close mission (all closeout gates pass): atelier mission update {} --status closed",
            mission.id
        );
    } else {
        println!(
            "  Inspect closeout audit (mission validation and linked epic outcomes): atelier mission status --closeout {}",
            mission.id
        );
        if summary.total_work().blocked > 0 || summary.open_blockers > 0 {
            println!("  Resolve open blockers before assigning more implementation work");
        } else if let Some(issue) = summary.selectable_work.first() {
            println!(
                "  Start selectable mission work ({} selectable issue(s)): atelier start {}",
                summary.selectable_work.len(),
                issue.id
            );
        }
        if summary.evidence_gap_count() > 0 {
            println!(
                "  Record validation proof ({} evidence gap(s)): atelier evidence record --target issue/<id> --kind validation --result pass \"...\"",
                summary.evidence_gap_count()
            );
        }
    }
    println!("  Check runtime health (tracker and projection state): atelier doctor");
}

pub fn view(db: &Database, id: &str) -> Result<()> {
    let mission = canonical_record_detail(KIND, id)?.unwrap_or(db.require_record(KIND, id)?);
    let links = db.list_record_links(KIND, id)?;
    let mut plans = Vec::new();
    let mut evidence = Vec::new();
    let mut milestones = Vec::new();
    let mut mission_blockers = Vec::new();
    let mut supporting = Vec::new();
    let mut seen_records = BTreeSet::new();
    let mut seen_mission_blockers = BTreeSet::new();
    let mut seen_work = BTreeSet::new();
    let mut work = BTreeMap::from([
        ("done".to_string(), Vec::<Value>::new()),
        ("ready".to_string(), Vec::<Value>::new()),
        ("blocked".to_string(), Vec::<Value>::new()),
        ("backlog".to_string(), Vec::<Value>::new()),
    ]);

    for link in &links {
        let Some((kind, linked_id)) = other_side(link, KIND, id) else {
            continue;
        };
        match kind {
            "plan"
                if link.relation_type == "planned_by"
                    && seen_records.insert((kind.to_string(), linked_id.to_string())) =>
            {
                plans.push(record_summary(db, kind, linked_id)?)
            }
            "evidence"
                if link.relation_type == "validates"
                    && seen_records.insert((kind.to_string(), linked_id.to_string())) =>
            {
                evidence.push(record_summary(db, kind, linked_id)?)
            }
            "milestone"
                if link.relation_type == "has_checkpoint"
                    && seen_records.insert((kind.to_string(), linked_id.to_string())) =>
            {
                milestones.push(record_summary(db, kind, linked_id)?)
            }
            "issue" => {
                if link.relation_type == "blocked_by" {
                    if seen_mission_blockers.insert(linked_id.to_string()) {
                        let issue = db.require_issue(linked_id)?;
                        mission_blockers.push(issue_json_with_relation(
                            db,
                            &issue,
                            &link.relation_type,
                        )?);
                    }
                    continue;
                }
                if link.relation_type != "advances" {
                    if seen_records.insert((kind.to_string(), linked_id.to_string())) {
                        supporting.push(linked_record_summary(
                            db,
                            kind,
                            linked_id,
                            &link.relation_type,
                        )?);
                    }
                    continue;
                }
                if !seen_work.insert(linked_id.to_string()) {
                    continue;
                }
                let issue = db.require_issue(linked_id)?;
                let bucket = issue_bucket(db, &issue)?;
                work.get_mut(bucket)
                    .expect("known work bucket")
                    .push(issue_json_with_relation(db, &issue, &link.relation_type)?);
            }
            _ => {
                if seen_records.insert((kind.to_string(), linked_id.to_string())) {
                    supporting.push(linked_record_summary(
                        db,
                        kind,
                        linked_id,
                        &link.relation_type,
                    )?);
                }
            }
        }
    }

    render_mission_show_human(
        &mission,
        &plans,
        &milestones,
        &evidence,
        &work,
        &mission_blockers,
        &supporting,
    )?;
    Ok(())
}

fn canonical_record_detail(kind: &str, id: &str) -> Result<Option<DomainRecord>> {
    let Some(state_dir) = find_state_dir_from_cwd()? else {
        return Ok(None);
    };
    let store = RecordStore::new(state_dir);
    Ok(Some(store.load_domain_record_by_id(kind, id)?.record))
}

pub fn active_mission(db: &Database) -> Result<Option<DomainRecord>> {
    let active = current_mission_records(db)?
        .into_iter()
        .filter(is_active_mission)
        .collect::<Vec<_>>();
    if active.len() > 1 {
        bail!(
            "Multiple active missions found: {}. Run `atelier lint` and switch one mission focus.",
            active
                .iter()
                .map(|record| record.id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(active.into_iter().next())
}

fn current_mission_records(db: &Database) -> Result<Vec<DomainRecord>> {
    mission_records_for_filter(db, Some("current"))
}

fn mission_records_for_filter(db: &Database, status: Option<&str>) -> Result<Vec<DomainRecord>> {
    let records = db.list_records(KIND, None)?;
    Ok(match status {
        None | Some("all") => records,
        Some("current") => records
            .into_iter()
            .filter(|record| mission_lifecycle_status(record) != "closed")
            .collect(),
        Some(status) => {
            let status = normalize_mission_status(status)?;
            records
                .into_iter()
                .filter(|record| mission_lifecycle_status(record) == status)
                .collect()
        }
    })
}

fn normalize_mission_status(status: &str) -> Result<&str> {
    match status {
        "draft" | "ready" | "active" | "closed" => Ok(status),
        _ => bail!(
            "Invalid mission status '{}'. Must be one of: draft, ready, active, closed",
            status
        ),
    }
}

fn mission_lifecycle_status(record: &DomainRecord) -> String {
    record.status.clone()
}

pub fn issue_advances_mission(db: &Database, mission_id: &str, issue_id: &str) -> Result<bool> {
    Ok(mission_issue_ids(db, mission_id)?.contains(issue_id))
}

fn find_state_dir_from_cwd() -> Result<Option<PathBuf>> {
    crate::storage_layout::find_canonical_dir_from_cwd()
}

pub fn list(db: &Database, status: Option<&str>) -> Result<()> {
    let status_filter = match status {
        Some("all") => None,
        Some(status) => Some(status),
        None => Some("current"),
    };
    let records = mission_records_for_filter(db, status_filter)?;
    let mut rows = records
        .into_iter()
        .map(|record| {
            Ok(MissionListRow {
                summary: mission_list_summary(db, &record.id)?,
                record,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    rows.sort_by(compare_mission_list_rows);
    render_mission_list_human(&rows)
}

pub fn update(
    state_dir: &Path,
    db_path: &Path,
    id: &str,
    title: Option<&str>,
    status: Option<&str>,
    body: Option<&str>,
    constraints: Vec<String>,
    risks: Vec<String>,
    validation: Vec<String>,
) -> Result<()> {
    if title.is_none()
        && status.is_none()
        && body.is_none()
        && constraints.is_empty()
        && risks.is_empty()
        && validation.is_empty()
    {
        bail!("Nothing to update");
    }
    let store = RecordStore::new(state_dir);
    let mut current = store.load_domain_record_by_id(KIND, id)?;
    let mut sections = record_store::mission_sections_from_domain_record(&current.record)?;
    replace_section_list(&mut sections.constraints, constraints);
    replace_section_list(&mut sections.risks, risks);
    replace_section_list(&mut sections.validation, validation);
    if let Some(title) = title {
        current.record.title = title.to_string();
    }
    if let Some(status) = status {
        let status = normalize_mission_status(status)?;
        if status == "closed" && current.record.status != "closed" {
            let db = Database::open(db_path)?;
            enforce_closeout(&db, state_dir, id)?;
        }
        current.record.status = status.to_string();
    }
    if let Some(body) = body {
        sections.intent = body.trim().to_string();
    }
    current.record.body = Some(record_store::render_mission_sections(&sections));
    current.record.data_json = MISSION_EMPTY_DATA_JSON.to_string();
    current.record.updated_at = Utc::now();
    store.write_domain_record_atomic(&current)?;
    refresh_projection(state_dir, db_path)?;
    let db = Database::open(db_path)?;
    let record = db.require_record(KIND, id)?;
    print_record(&record)
}

pub fn add_work(state_dir: &Path, db_path: &Path, id: &str, issue_id: &str) -> Result<()> {
    let db = Database::open(db_path)?;
    db.require_record(KIND, id)?;
    let issue = db.require_issue(issue_id)?;
    drop(db);
    let store = RecordStore::new(state_dir);
    let inserted = store.add_relates_relationship(KIND, id, "issue", &issue.id, "advances")?;
    refresh_projection(state_dir, db_path)?;
    if inserted {
        println!("Added work {} to mission {}", issue.id, id);
    } else {
        println!("Work {} is already on mission {}", issue.id, id);
    }
    Ok(())
}

pub fn audit(db: &Database, state_dir: &Path, id: &str, quiet: bool) -> Result<()> {
    let mission = db.require_record(KIND, id)?;
    let audit = mission_contract_audit(db, state_dir, &mission)?;
    if quiet {
        println!(
            "{} audit={} items={} failures={}",
            mission.id,
            if audit.passed() { "pass" } else { "fail" },
            audit.items.len(),
            audit.failure_count()
        );
    } else {
        audit.print_human(&mission);
    }
    if audit.passed() {
        Ok(())
    } else {
        bail!(
            "mission contract audit failed; resolve failing items or attach proof before closing {}",
            mission.id
        )
    }
}

struct MissionCloseoutStatus {
    mission_id: String,
    open_work: Vec<String>,
    open_blockers: Vec<String>,
    evidence_missing: bool,
    contract_audit: MissionContractAudit,
    validator_results: Vec<crate::commands::workflow::ValidatorResult>,
}

#[derive(Debug, Clone)]
struct MissionContractAudit {
    items: Vec<MissionContractAuditItem>,
}

#[derive(Debug, Clone)]
struct MissionContractAuditItem {
    group: String,
    target: String,
    text: String,
    coverage_status: ProofCoverageStatus,
    reason: String,
}

impl MissionContractAudit {
    pub(crate) fn passed(&self) -> bool {
        self.items
            .iter()
            .all(|item| item.coverage_status.satisfies_closeout())
    }

    pub(crate) fn failure_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| !item.coverage_status.satisfies_closeout())
            .count()
    }

    pub(crate) fn pass_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.coverage_status.satisfies_closeout())
            .count()
    }

    fn coverage_count(&self, status: ProofCoverageStatus) -> usize {
        self.items
            .iter()
            .filter(|item| item.coverage_status == status)
            .count()
    }

    fn print_human(&self, mission: &DomainRecord) {
        let status = if self.passed() { "pass" } else { "fail" };
        let identity = format!(
            "Mission Contract Audit {} [{}] - {}",
            mission.id, status, mission.title
        );
        println!("{identity}");
        println!("{}", "=".repeat(identity.len()));
        println!(
            "Summary: {} pass, {} fail, {} total",
            self.pass_count(),
            self.failure_count(),
            self.items.len()
        );
        println!(
            "Coverage: covered {}, missing {}, failed {}, blocked {}, deferred {}, not-applicable {}",
            self.coverage_count(ProofCoverageStatus::Covered),
            self.coverage_count(ProofCoverageStatus::Missing),
            self.coverage_count(ProofCoverageStatus::Failed),
            self.coverage_count(ProofCoverageStatus::Blocked),
            self.coverage_count(ProofCoverageStatus::Deferred),
            self.coverage_count(ProofCoverageStatus::NotApplicable),
        );
        if self.items.is_empty() {
            println!("No authored mission validation or linked epic outcome items.");
        }

        let groups = self
            .items
            .iter()
            .map(|item| item.group.as_str())
            .collect::<BTreeSet<_>>();
        for group in groups {
            println!();
            println!("{group}");
            println!("{}", "-".repeat(group.len()));
            for item in self.items.iter().filter(|item| item.group == group) {
                println!(
                    "  [{}] {} - {}",
                    item.coverage_status.label(),
                    item.target,
                    item.text
                );
                println!("    {}", item.reason);
            }
        }

        println!();
        println!("Next Commands");
        println!("-------------");
        if self.passed() {
            println!(
                "  Close mission when other gates pass: atelier mission update {} --status closed",
                mission.id
            );
        } else {
            println!(
                "  Inspect closeout gates: atelier mission status {}",
                mission.id
            );
            println!(
                "  Attach missing proof: atelier evidence attach <evidence-id> mission {}",
                mission.id
            );
        }
    }
}

impl MissionCloseoutStatus {
    fn ready(&self) -> bool {
        self.open_work.is_empty()
            && self.open_blockers.is_empty()
            && !self.evidence_missing
            && self.contract_audit.passed()
            && self.validator_results.iter().all(|result| result.passed)
    }

    fn validator_failure_count(&self) -> usize {
        self.validator_results
            .iter()
            .filter(|result| !result.passed)
            .count()
    }

    fn blocking_messages(&self) -> Vec<String> {
        let mut messages = Vec::new();
        if !self.open_work.is_empty() {
            messages.push(format!(
                "open mission work: {}; close or defer linked work before mission closeout",
                compact_strings(&self.open_work)
            ));
        }
        if !self.open_blockers.is_empty() {
            messages.push(format!(
                "open blockers: {}; close or remove blocker links before mission closeout",
                compact_strings(&self.open_blockers)
            ));
        }
        if self.evidence_missing {
            messages.push(
                "missing mission proof: attach validation evidence to the mission".to_string(),
            );
        }
        if !self.contract_audit.passed() {
            messages.push(format!(
                "contract audit failed: {} unresolved item(s); run `atelier mission audit {}`",
                self.contract_audit.failure_count(),
                self.mission_id
            ));
        }
        for result in self
            .validator_results
            .iter()
            .filter(|result| !result.passed)
        {
            if let Some(message) = closeout_validator_blocking_message(result, &self.mission_id) {
                messages.push(message);
            }
        }
        messages
    }

    fn print_human(&self) {
        if self.ready() {
            println!("All required closeout gates pass.");
            return;
        }
        if self.open_work.is_empty() {
            println!("Work: closed");
        } else {
            println!("Work: open - {}", compact_strings(&self.open_work));
            println!("  Next: atelier issue close <issue-id> --reason \"...\"");
        }
        if self.open_blockers.is_empty() {
            println!("Blockers: clear");
        } else {
            println!("Blockers: open - {}", compact_strings(&self.open_blockers));
            println!("  Next: close or unblock the blocker issues.");
        }
        if self.evidence_missing {
            println!("Mission Proof: missing");
            println!(
                "  Next: atelier evidence record --target issue/<id> --kind validation --result pass \"...\""
            );
            println!("  Next: atelier evidence attach <evidence-id> mission <mission-id>");
        } else {
            println!("Mission Proof: attached");
        }
        if self.contract_audit.passed() {
            println!("Contract Audit: pass");
        } else {
            println!(
                "Contract Audit: fail - {} unresolved item(s)",
                self.contract_audit.failure_count()
            );
            println!("  Next: atelier mission audit {}", self.mission_id);
        }
        for result in &self.validator_results {
            if let Some(line) = closeout_validator_status_line(result, &self.mission_id) {
                println!("{}", line.summary);
                if let Some(next) = line.next {
                    println!("  Next: {next}");
                }
            }
        }
    }
}

#[derive(Default)]
struct IssueSectionGapSummary {
    malformed: Vec<String>,
    missing_outcome: Vec<String>,
    missing_evidence: Vec<String>,
}

fn print_reliability_summary(
    db: &Database,
    state_dir: &Path,
    mission: &DomainRecord,
    summary: &MissionListSummary,
    tracker: &TrackerHealth,
    closeout: &MissionCloseoutStatus,
) -> Result<()> {
    let section_gaps = mission_issue_section_gaps(db, state_dir, &mission.id)?;
    let issue_proof_gaps = mission_issue_proof_gaps(db, &mission.id)?;

    if tracker.stale_entries.is_empty() {
        println!("Projection Freshness: current");
    } else {
        println!(
            "Projection Freshness: stale - {}",
            compact_strings(&tracker.stale_entries)
        );
        println!("  Next: atelier doctor");
    }

    if let Some(result) = closeout_validator_result(closeout, "issue_sections_parseable") {
        if result.passed && section_gaps.malformed.is_empty() {
            println!("Malformed Work: none");
        } else {
            let reason = if section_gaps.malformed.is_empty() {
                result.reason.clone()
            } else {
                compact_strings(&section_gaps.malformed)
            };
            println!("Malformed Work: found - {reason}");
            println!("  Next: atelier lint");
        }
    }

    print_section_gap_signal("Missing Outcome Sections", &section_gaps.missing_outcome);
    print_section_gap_signal("Missing Evidence Sections", &section_gaps.missing_evidence);
    print_graph_hygiene_signal(summary);

    let mut proof_parts = Vec::new();
    if summary.evidence_count == 0 {
        proof_parts.push("mission proof is not attached".to_string());
    }
    if !issue_proof_gaps.is_empty() {
        proof_parts.push(format!(
            "issue proof gaps: {}",
            compact_strings(&issue_proof_gaps)
        ));
    }
    if proof_parts.is_empty() {
        println!("Attached Proof: complete");
    } else {
        println!("Attached Proof: missing - {}", proof_parts.join("; "));
        println!(
            "  Next: atelier evidence record --target issue/<id> --kind validation --result pass \"...\""
        );
        println!("  Next: atelier evidence attach <evidence-id> issue <issue-id>");
    }

    print_reliability_validator_signal(
        closeout,
        "command_surface_current",
        "Docs/Help Drift",
        "clear",
        "detected",
        "update docs, help text, or command-surface tests",
    );
    print_reliability_validator_signal(
        closeout,
        "ignored_tests_reviewed",
        "Ignored Test Review",
        "current",
        "needed",
        "assign owners or remove stale ignored tests",
    );

    if closeout.open_blockers.is_empty() {
        println!("Open Blockers: none");
    } else {
        println!(
            "Open Blockers: {} open - {}",
            closeout.open_blockers.len(),
            compact_strings(&closeout.open_blockers)
        );
        println!("  Next: close or unblock listed blockers");
    }

    println!("Drill-downs:");
    println!("  atelier mission audit {}", mission.id);
    println!("  atelier lint");
    println!("  atelier doctor");
    Ok(())
}

fn print_graph_hygiene_signal(summary: &MissionListSummary) {
    if summary.duplicate_reachability.is_empty() {
        println!("Graph Hygiene: clear");
        return;
    }

    let details = summary
        .duplicate_reachability
        .iter()
        .map(format_duplicate_reachability)
        .collect::<Vec<_>>();
    println!(
        "Graph Hygiene: warning - duplicate reachability for {} issue(s): {}",
        summary.duplicate_reachability.len(),
        compact_strings(&details)
    );
    println!(
        "  Totals count each unique issue once. Keep mission links on root issues or epics and let child issues flow through hierarchy."
    );
}

fn print_section_gap_signal(label: &str, ids: &[String]) {
    if ids.is_empty() {
        println!("{label}: none");
    } else {
        println!("{label}: {} issue(s) - {}", ids.len(), compact_strings(ids));
        println!("  Next: atelier lint");
    }
}

fn print_reliability_validator_signal(
    closeout: &MissionCloseoutStatus,
    validator: &str,
    label: &str,
    pass_text: &str,
    fail_text: &str,
    next: &str,
) {
    let Some(result) = closeout_validator_result(closeout, validator) else {
        return;
    };
    if result.passed {
        println!("{label}: {pass_text}");
    } else {
        println!("{label}: {fail_text} - {}", result.reason);
        println!("  Next: {next}");
    }
}

fn closeout_validator_result<'a>(
    closeout: &'a MissionCloseoutStatus,
    validator: &str,
) -> Option<&'a crate::commands::workflow::ValidatorResult> {
    closeout
        .validator_results
        .iter()
        .find(|result| result.validator == validator)
}

fn mission_issue_section_gaps(
    db: &Database,
    state_dir: &Path,
    mission_id: &str,
) -> Result<IssueSectionGapSummary> {
    let store = RecordStore::new(state_dir);
    let mut gaps = IssueSectionGapSummary::default();
    for issue_id in mission_issue_ids(db, mission_id)? {
        match store.load_issue_by_id(&issue_id) {
            Ok(record) => {
                for state in record.sections.section_states() {
                    if !state.required || (state.present && !state.empty) {
                        continue;
                    }
                    if state.name == record_store::IssueSectionName::Outcome {
                        gaps.missing_outcome.push(issue_id.clone());
                    } else if state.name == record_store::IssueSectionName::Evidence {
                        gaps.missing_evidence.push(issue_id.clone());
                    }
                }
            }
            Err(error) => {
                let diagnostic = error.to_string();
                if diagnostic.contains("section 'Outcome'")
                    || diagnostic.contains("section Outcome")
                    || diagnostic.contains("section `Outcome`")
                {
                    gaps.missing_outcome.push(issue_id.clone());
                }
                if diagnostic.contains("section 'Evidence'")
                    || diagnostic.contains("section Evidence")
                    || diagnostic.contains("section `Evidence`")
                {
                    gaps.missing_evidence.push(issue_id.clone());
                }
                gaps.malformed.push(format!("{issue_id}: {diagnostic}"));
            }
        }
    }
    gaps.malformed.sort();
    gaps.missing_outcome.sort();
    gaps.missing_evidence.sort();
    Ok(gaps)
}

fn mission_issue_proof_gaps(db: &Database, mission_id: &str) -> Result<Vec<String>> {
    let mut gaps = Vec::new();
    for issue_id in mission_issue_ids(db, mission_id)? {
        if validating_evidence_ids(db, "issue", &issue_id)?.is_empty() {
            gaps.push(issue_id);
        }
    }
    gaps.sort();
    Ok(gaps)
}

struct CloseoutStatusLine {
    summary: String,
    next: Option<String>,
}

fn closeout_validator_status_line(
    result: &crate::commands::workflow::ValidatorResult,
    mission_id: &str,
) -> Option<CloseoutStatusLine> {
    let (label, pass_text, fail_text, next) = closeout_validator_user_text(&result.validator)?;
    if result.passed {
        Some(CloseoutStatusLine {
            summary: format!("{label}: {pass_text}"),
            next: None,
        })
    } else {
        let next = next.replace("{mission}", mission_id);
        Some(CloseoutStatusLine {
            summary: format!("{label}: {fail_text} - {}", result.reason),
            next: Some(next),
        })
    }
}

fn closeout_validator_blocking_message(
    result: &crate::commands::workflow::ValidatorResult,
    mission_id: &str,
) -> Option<String> {
    let (label, _pass_text, fail_text, next) = closeout_validator_user_text(&result.validator)?;
    let next = next.replace("{mission}", mission_id);
    Some(format!(
        "{}: {} - {}; next: {}",
        label.to_ascii_lowercase(),
        fail_text,
        result.reason,
        next
    ))
}

fn closeout_validator_user_text(
    validator: &str,
) -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    match validator {
        "durable_state_current" => Some((
            "Tracker State",
            "current",
            "stale",
            "atelier export --check",
        )),
        "issue_sections_parseable" => Some((
            "Linked Issue Records",
            "parseable",
            "malformed",
            "atelier lint",
        )),
        "no_blocking_lints" => Some(("Blocking Lints", "clear", "failing", "atelier lint")),
        "command_surface_current" => Some((
            "Docs/Help Drift",
            "clear",
            "detected",
            "update docs, help text, or command-surface tests",
        )),
        "ignored_tests_reviewed" => Some((
            "Ignored Test Review",
            "current",
            "needed",
            "assign owners or remove stale ignored tests",
        )),
        "validation_criteria_satisfied" => Some((
            "Validation Criteria",
            "satisfied",
            "incomplete",
            "atelier mission audit {mission}",
        )),
        "git_worktree_clean" => Some((
            "Worktree",
            "clean",
            "dirty",
            "commit or remove untracked worktree changes",
        )),
        "no_open_work" | "no_open_blockers" | "evidence_attached" => None,
        _ => Some((
            "Additional Closeout Check",
            "passed",
            "failed",
            "atelier mission status {mission}",
        )),
    }
}

fn mission_contract_audit(
    db: &Database,
    state_dir: &Path,
    mission: &DomainRecord,
) -> Result<MissionContractAudit> {
    let mut items = Vec::new();
    let sections = record_store::mission_sections_from_domain_record(mission)?;
    let mission_evidence = validating_evidence_records(db, KIND, &mission.id)?;
    let mission_work = mission_issue_ids(db, &mission.id)?;
    let open_work = open_mission_work(db, &mission.id)?;
    let open_blockers = open_mission_blockers(db, &mission.id)?;
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;

    for item in contract_items_from_text(&sections.validation) {
        let (coverage_status, reason) = if mission_work.is_empty() {
            (
                ProofCoverageStatus::Missing,
                "No linked mission work exists for this validation expectation.".to_string(),
            )
        } else if !open_work.is_empty() {
            (
                ProofCoverageStatus::Missing,
                format!("Open linked work remains: {}", compact_strings(&open_work)),
            )
        } else if !open_blockers.is_empty() {
            (
                ProofCoverageStatus::Blocked,
                format!("Open blockers remain: {}", compact_strings(&open_blockers)),
            )
        } else {
            coverage_reason(
                classify_requirement_coverage(&item, &mission_evidence),
                "No matching validation evidence is attached to the mission.",
            )
        };
        items.push(MissionContractAuditItem {
            group: "Mission Validation".to_string(),
            target: mission.id.clone(),
            text: item,
            coverage_status,
            reason,
        });
    }

    let store = RecordStore::new(state_dir);
    for epic in mission_linked_epics(db, &mission.id)? {
        match store.load_issue_by_id(&epic.id) {
            Ok(record) => {
                let evidence = validating_evidence_records(db, "issue", &epic.id)?;
                for item in contract_items_from_text(&record.sections.outcome) {
                    let (coverage_status, reason) =
                        if !crate::commands::issue_workflow::issue_is_done(
                            workflow_policy.as_ref(),
                            &epic,
                        ) {
                            (
                                ProofCoverageStatus::Missing,
                                format!(
                                    "Linked epic is still {}.",
                                    crate::commands::issue_workflow::issue_status_label(
                                        workflow_policy.as_ref(),
                                        &epic.status,
                                    )
                                ),
                            )
                        } else {
                            coverage_reason(
                                classify_requirement_coverage(&item, &evidence),
                                "No matching validation evidence is attached to this linked epic.",
                            )
                        };
                    items.push(MissionContractAuditItem {
                        group: "Linked Epic Outcomes".to_string(),
                        target: epic.id.clone(),
                        text: item,
                        coverage_status,
                        reason,
                    });
                }
            }
            Err(error) => items.push(MissionContractAuditItem {
                group: "Linked Epic Outcomes".to_string(),
                target: epic.id.clone(),
                text: epic.title.clone(),
                coverage_status: ProofCoverageStatus::Failed,
                reason: format!("Linked epic record is malformed: {error}"),
            }),
        }
    }

    Ok(MissionContractAudit { items })
}

pub(crate) fn mission_contract_audit_gate(
    db: &Database,
    state_dir: &Path,
    mission_id: &str,
) -> Result<(bool, String)> {
    let mission = db.require_record(KIND, mission_id)?;
    let audit = mission_contract_audit(db, state_dir, &mission)?;
    if audit.passed() {
        Ok((
            true,
            format!(
                "mission contract audit passed: {} pass, 0 fail",
                audit.pass_count()
            ),
        ))
    } else {
        Ok((
            false,
            format!(
                "mission contract audit failed: {} unresolved item(s); run `atelier mission audit {mission_id}`",
                audit.failure_count()
            ),
        ))
    }
}

fn mission_linked_epics(db: &Database, mission_id: &str) -> Result<Vec<Issue>> {
    let mut epics = mission_issue_ids(db, mission_id)?
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter(|issue| issue.issue_type == "epic")
        .collect::<Vec<_>>();
    epics.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(epics)
}

fn coverage_reason(
    coverage: crate::commands::agent_factory::ProofCoverage,
    missing_reason: &str,
) -> (ProofCoverageStatus, String) {
    let status = coverage.status;
    let evidence = if coverage.evidence_refs.is_empty() {
        String::new()
    } else {
        format!(": {}", compact_strings(&coverage.evidence_refs))
    };
    let reason = match coverage.status {
        ProofCoverageStatus::Covered => format!("Covered by evidence{evidence}"),
        ProofCoverageStatus::Missing => missing_reason.to_string(),
        ProofCoverageStatus::Failed => format!("Matching evidence failed{evidence}"),
        ProofCoverageStatus::Blocked => format!("Matching evidence is blocked{evidence}"),
        ProofCoverageStatus::Deferred => format!("Matching evidence is deferred{evidence}"),
        ProofCoverageStatus::NotApplicable => {
            format!("Marked not-applicable by evidence{evidence}")
        }
    };
    (status, reason)
}

fn validating_evidence_records(
    db: &Database,
    target_kind: &str,
    target_id: &str,
) -> Result<Vec<DomainRecord>> {
    let mut records = db
        .list_record_links(target_kind, target_id)?
        .into_iter()
        .filter(|link| link.relation_type == "validates")
        .filter_map(|link| {
            if link.source_kind == "evidence" {
                Some(link.source_id)
            } else if link.target_kind == "evidence" {
                Some(link.target_id)
            } else {
                None
            }
        })
        .map(|id| db.require_record("evidence", &id))
        .collect::<Result<Vec<_>>>()?;
    records.sort_by(|a, b| a.id.cmp(&b.id));
    records.dedup_by(|a, b| a.id == b.id);
    Ok(records)
}

fn validating_evidence_ids(
    db: &Database,
    target_kind: &str,
    target_id: &str,
) -> Result<Vec<String>> {
    Ok(validating_evidence_records(db, target_kind, target_id)?
        .into_iter()
        .map(|record| record.id)
        .collect())
}

fn contract_items_from_text(text: &str) -> Vec<String> {
    let mut items = text
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("- ")
                .or_else(|| trimmed.strip_prefix("* "))
                .map(str::trim)
                .filter(|item| !is_placeholder_contract_item(item))
                .map(ToOwned::to_owned)
        })
        .collect::<Vec<_>>();
    if items.is_empty() {
        let paragraph = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        if !paragraph.is_empty() && !is_placeholder_contract_item(&paragraph) {
            items.push(paragraph);
        }
    }
    items
}

fn is_placeholder_contract_item(item: &str) -> bool {
    let normalized = item
        .trim()
        .trim_start_matches("- ")
        .trim_start_matches("* ")
        .trim()
        .trim_end_matches('.')
        .to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "none" | "validation was not specified" | "outcome was not specified"
    )
}

fn compact_strings(values: &[String]) -> String {
    const LIMIT: usize = 8;
    if values.len() <= LIMIT {
        values.join(", ")
    } else {
        format!(
            "{}, ... and {} more",
            values[..LIMIT].join(", "),
            values.len() - LIMIT
        )
    }
}

fn enforce_closeout(db: &Database, state_dir: &Path, mission_id: &str) -> Result<()> {
    let mission = db.require_record(KIND, mission_id)?;
    let summary = mission_list_summary(db, mission_id)?;
    let closeout = mission_closeout_status(db, state_dir, &mission, &summary)?;
    if closeout.ready() {
        return Ok(());
    }
    println!("Mission closeout blocked: {mission_id}");
    println!("Closeout blockers");
    println!("-----------------");
    for message in closeout.blocking_messages() {
        println!("  - {message}");
    }
    bail!("mission closeout blocked; run `atelier mission status {mission_id}` for next commands")
}

fn mission_closeout_status(
    db: &Database,
    state_dir: &Path,
    mission: &DomainRecord,
    summary: &MissionListSummary,
) -> Result<MissionCloseoutStatus> {
    let open_work = open_mission_work(db, &mission.id)?;
    let open_blockers = open_mission_blockers(db, &mission.id)?;
    let contract_audit = mission_contract_audit(db, state_dir, mission)?;
    let validator_results =
        match crate::commands::workflow::evaluate(db, KIND, &mission.id, "close", Vec::new()) {
            Ok(results) => results,
            Err(error) => vec![crate::commands::workflow::ValidatorResult {
                target_kind: KIND.to_string(),
                target_id: mission.id.clone(),
                transition: "close".to_string(),
                validator: "workflow_policy".to_string(),
                passed: false,
                reason: format!("{error:#}; run `atelier lint` for workflow/config diagnostics"),
                elapsed_ms: 0,
            }],
        };
    Ok(MissionCloseoutStatus {
        mission_id: mission.id.clone(),
        open_work,
        open_blockers,
        evidence_missing: summary.evidence_count == 0,
        contract_audit,
        validator_results,
    })
}

fn open_mission_work(db: &Database, mission_id: &str) -> Result<Vec<String>> {
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let mut open = mission_issue_ids(db, mission_id)?
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter(|issue| {
            crate::commands::issue_workflow::issue_blocks_work(workflow_policy.as_ref(), issue)
        })
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    open.sort();
    Ok(open)
}

fn open_mission_blockers(db: &Database, mission_id: &str) -> Result<Vec<String>> {
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let mut blocker_ids = BTreeSet::new();
    for blocker in mission_direct_blocker_ids(db, mission_id)? {
        blocker_ids.insert(blocker);
    }
    for issue_id in mission_issue_ids(db, mission_id)? {
        for blocker in db.get_blockers(&issue_id)? {
            blocker_ids.insert(blocker);
        }
    }
    let mut open = blocker_ids
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter(|issue| {
            crate::commands::issue_workflow::issue_blocks_work(workflow_policy.as_ref(), issue)
        })
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    open.sort();
    Ok(open)
}

fn mission_direct_blocker_ids(db: &Database, mission_id: &str) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for link in db.list_record_links(KIND, mission_id)? {
        if link.relation_type != "blocked_by" {
            continue;
        }
        let Some((kind, linked_id)) = other_side(&link, KIND, mission_id) else {
            continue;
        };
        if kind == "issue" {
            blockers.push(linked_id.to_string());
        }
    }
    Ok(blockers)
}

pub fn add_blocker(state_dir: &Path, db_path: &Path, id: &str, issue_id: &str) -> Result<()> {
    let db = Database::open(db_path)?;
    db.require_record(KIND, id)?;
    let issue = db.require_issue(issue_id)?;
    drop(db);
    let store = RecordStore::new(state_dir);
    let inserted = store.add_relates_relationship(KIND, id, "issue", &issue.id, "blocked_by")?;
    refresh_projection(state_dir, db_path)?;
    if inserted {
        println!("Added blocker {} to mission {}", issue.id, id);
    } else {
        println!("Blocker {} is already on mission {}", issue.id, id);
    }
    Ok(())
}

fn refresh_projection(state_dir: &Path, db_path: &Path) -> Result<()> {
    super::projection::refresh_after_canonical_write(state_dir, db_path)
}

fn print_record(record: &DomainRecord) -> Result<()> {
    println!("Mission {}: {}", record.id, record.title);
    println!("Status: {}", record.status);
    if let Some(body) = &record.body {
        if !body.is_empty() {
            println!("\n{}", body);
        }
    }
    Ok(())
}

struct MissionListRow {
    record: DomainRecord,
    summary: MissionListSummary,
}

#[derive(Default)]
struct MissionListSummary {
    work: WorkCounts,
    other_work: WorkCounts,
    epics: Vec<MissionListEpic>,
    selectable_work: Vec<Issue>,
    blocked_work: Vec<BlockedMissionWork>,
    open_blockers: usize,
    evidence_count: usize,
    duplicate_reachability: Vec<DuplicateReachability>,
}

struct MissionListEpic {
    issue: Issue,
    work: WorkCounts,
}

struct BlockedMissionWork {
    issue: Issue,
    blockers: Vec<String>,
}

struct DuplicateReachability {
    issue_id: String,
    roots: Vec<String>,
}

#[derive(Clone, Copy, Default)]
struct WorkCounts {
    ready: usize,
    blocked: usize,
    done: usize,
    backlog: usize,
}

fn mission_list_summary(db: &Database, mission_id: &str) -> Result<MissionListSummary> {
    let mut summary = MissionListSummary::default();
    let mut seen_blockers = BTreeSet::new();
    let mut seen_work = BTreeSet::new();
    let mut linked_work = Vec::new();

    for link in db.list_record_links(KIND, mission_id)? {
        let Some((kind, linked_id)) = other_side(&link, KIND, mission_id) else {
            continue;
        };
        match kind {
            "issue" if link.relation_type == "blocked_by" => {
                if seen_blockers.insert(linked_id.to_string()) {
                    let issue = db.require_issue(linked_id)?;
                    let workflow_policy =
                        crate::commands::issue_workflow::load_issue_workflow_policy()?;
                    if crate::commands::issue_workflow::issue_blocks_work(
                        workflow_policy.as_ref(),
                        &issue,
                    ) {
                        summary.open_blockers += 1;
                    }
                }
            }
            "issue" if link.relation_type == "advances" => {
                if !seen_work.insert(linked_id.to_string()) {
                    continue;
                }
                let issue = db.require_issue(linked_id)?;
                linked_work.push(issue);
            }
            "evidence" if link.relation_type == "validates" => {
                summary.evidence_count += 1;
            }
            _ => {}
        }
    }

    let linked_epic_ids = linked_work
        .iter()
        .filter(|issue| issue.issue_type == "epic")
        .map(|issue| issue.id.clone())
        .collect::<BTreeSet<_>>();
    let mission_issue_ids = mission_issue_ids(db, mission_id)?;

    for issue_id in &mission_issue_ids {
        summary
            .work
            .add_bucket(issue_bucket(db, &db.require_issue(issue_id)?)?);
    }
    summary.duplicate_reachability = duplicate_reachability(db, &mission_issue_ids, &seen_work)?;

    for issue in linked_work {
        if issue.issue_type == "epic" {
            summary.epics.push(MissionListEpic {
                work: epic_work_counts(db, &issue.id)?,
                issue,
            });
        } else if !has_ancestor_in_set(db, &issue, &linked_epic_ids)? {
            summary.other_work.add_bucket(issue_bucket(db, &issue)?);
        }
    }

    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    for issue_id in mission_issue_ids {
        let issue = db.require_issue(&issue_id)?;
        if !is_selectable_work(db, &issue)? {
            continue;
        }
        let blockers = open_blockers(db, &issue.id)?;
        if !blockers.is_empty() {
            summary
                .blocked_work
                .push(BlockedMissionWork { issue, blockers });
            continue;
        }
        if matches!(
            crate::commands::issue_workflow::issue_start_readiness(
                db,
                workflow_policy.as_ref(),
                &issue
            )?,
            crate::commands::issue_workflow::IssueStartReadiness::Ready
        ) {
            summary.selectable_work.push(issue);
        }
    }

    summary.epics.sort_by(compare_mission_list_epics);
    summary.selectable_work.sort_by(|a, b| a.id.cmp(&b.id));
    summary
        .blocked_work
        .sort_by(|a, b| a.issue.id.cmp(&b.issue.id));
    Ok(summary)
}

impl MissionListSummary {
    fn total_work(&self) -> WorkCounts {
        self.work
    }

    fn evidence_gap_count(&self) -> usize {
        usize::from(self.evidence_count == 0)
    }

    fn closeout_needed(&self) -> bool {
        let work = self.total_work();
        work.done > 0
            && work.ready == 0
            && work.blocked == 0
            && work.backlog == 0
            && self.open_blockers == 0
            && self.evidence_count > 0
    }
}

fn render_mission_list_human(rows: &[MissionListRow]) -> Result<()> {
    println!("Missions");
    println!("========");
    println!("{}", mission_list_summary_line(rows));

    if rows.is_empty() {
        println!("(none)");
        print_mission_list_next_commands(None);
        return Ok(());
    }

    print_mission_list_group(
        "Active",
        rows.iter()
            .filter(|row| mission_lifecycle_status(&row.record) == "active"),
    );

    print_mission_list_group(
        "Ready",
        rows.iter()
            .filter(|row| mission_lifecycle_status(&row.record) == "ready"),
    );

    print_mission_list_group(
        "Draft",
        rows.iter()
            .filter(|row| mission_lifecycle_status(&row.record) == "draft"),
    );

    let other_statuses = rows
        .iter()
        .filter(|row| {
            !matches!(
                mission_lifecycle_status(&row.record).as_str(),
                "active" | "ready" | "draft" | "closed"
            )
        })
        .map(|row| mission_lifecycle_status(&row.record))
        .collect::<BTreeSet<_>>();
    for status in other_statuses.iter() {
        print_mission_list_group(
            &status_heading(status),
            rows.iter()
                .filter(|row| mission_lifecycle_status(&row.record) == *status),
        );
    }

    print_mission_list_group(
        "Closed",
        rows.iter()
            .filter(|row| mission_lifecycle_status(&row.record) == "closed"),
    );

    let first_actionable = rows
        .iter()
        .find(|row| mission_lifecycle_status(&row.record) != "closed")
        .or_else(|| rows.first());
    print_mission_list_next_commands(first_actionable);
    Ok(())
}

fn mission_list_summary_line(rows: &[MissionListRow]) -> String {
    let mut statuses = BTreeMap::<String, usize>::new();
    let mut blocked_missions = 0;
    for row in rows {
        *statuses
            .entry(mission_lifecycle_status(&row.record))
            .or_default() += 1;
        if row.summary.open_blockers > 0 || row.summary.total_work().blocked > 0 {
            blocked_missions += 1;
        }
    }
    let status_text = mission_status_summary_text(rows.len(), statuses);
    let blocked_text = count_label(blocked_missions, "blocked");
    format!("{status_text} | {blocked_text}")
}

fn print_mission_list_group<'a>(title: &str, rows: impl Iterator<Item = &'a MissionListRow>) {
    let rows = rows.collect::<Vec<_>>();
    if rows.is_empty() {
        return;
    }
    println!("\n{title}");
    println!("{}", "-".repeat(title.len()));
    for row in rows {
        println!(
            "  {} [{}] - {}",
            row.record.id,
            mission_lifecycle_status(&row.record),
            row.record.title
        );
        if mission_lifecycle_status(&row.record) != "closed" {
            print_mission_list_open_work(row);
        }
    }
}

fn print_mission_list_next_commands(first_actionable: Option<&MissionListRow>) {
    print_mission_heading("Next Commands");
    if let Some(row) = first_actionable {
        println!("  atelier mission status {}", row.record.id);
        println!("  atelier mission show {}", row.record.id);
    }
    println!("  atelier mission status");
    println!("  atelier mission create \"...\"");
}

fn compare_mission_list_rows(a: &MissionListRow, b: &MissionListRow) -> std::cmp::Ordering {
    mission_status_rank(&mission_lifecycle_status(&a.record))
        .cmp(&mission_status_rank(&mission_lifecycle_status(&b.record)))
        .then_with(|| {
            if mission_lifecycle_status(&a.record) != "ready"
                && mission_lifecycle_status(&b.record) != "ready"
            {
                mission_lifecycle_status(&a.record).cmp(&mission_lifecycle_status(&b.record))
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .then_with(|| b.record.updated_at.cmp(&a.record.updated_at))
        .then_with(|| a.record.id.cmp(&b.record.id))
}

fn mission_status_rank(status: &str) -> u8 {
    match status {
        "active" => 0,
        "ready" => 1,
        "draft" => 2,
        "closed" => 4,
        _ => 3,
    }
}

fn status_heading(status: &str) -> String {
    status
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn mission_status_summary_text(total: usize, statuses: BTreeMap<String, usize>) -> String {
    if statuses.len() == 1 {
        let (status, count) = statuses
            .into_iter()
            .next()
            .unwrap_or_else(|| ("mission".to_string(), total));
        format!("{count} {status} {}", plural_noun(count, "mission"))
    } else if statuses.is_empty() {
        "0 missions".to_string()
    } else {
        format!("{total} missions | {}", joined_plain_counts(statuses))
    }
}

fn joined_plain_counts(counts: BTreeMap<String, usize>) -> String {
    counts
        .into_iter()
        .map(|(status, count)| format!("{count} {status}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn print_mission_list_open_work(row: &MissionListRow) {
    if row.summary.epics.is_empty() {
        println!("    No linked epics.");
    } else {
        for epic in &row.summary.epics {
            println!(
                "    [epic] {} [{}] {} - {} | {}",
                epic.issue.id,
                epic.issue.status,
                epic.issue.priority,
                epic.issue.title,
                epic.work.to_inline_text()
            );
        }
    }
    if !row.summary.other_work.is_empty() {
        println!(
            "    Other linked work: {}",
            row.summary.other_work.to_compact_text()
        );
    }
    if row.summary.open_blockers > 0 {
        println!(
            "    Mission blockers: {}",
            count_label(row.summary.open_blockers, "open")
        );
    }
    if row.summary.evidence_gap_count() > 0 {
        println!("    Evidence gaps: {}", row.summary.evidence_gap_count());
    }
    if row.summary.closeout_needed() {
        println!("    Closeout: needed");
    }
}

fn epic_work_counts(db: &Database, epic_id: &str) -> Result<WorkCounts> {
    let mut counts = WorkCounts::default();
    let mut seen = BTreeSet::new();
    collect_descendant_work_counts(db, epic_id, &mut seen, &mut counts)?;
    Ok(counts)
}

fn collect_descendant_work_counts(
    db: &Database,
    parent_id: &str,
    seen: &mut BTreeSet<String>,
    counts: &mut WorkCounts,
) -> Result<()> {
    for child in db.get_subissues(parent_id)? {
        if !seen.insert(child.id.clone()) {
            continue;
        }
        counts.add_bucket(issue_bucket(db, &child)?);
        collect_descendant_work_counts(db, &child.id, seen, counts)?;
    }
    Ok(())
}

fn duplicate_reachability(
    db: &Database,
    mission_issue_ids: &BTreeSet<String>,
    linked_root_ids: &BTreeSet<String>,
) -> Result<Vec<DuplicateReachability>> {
    let mut duplicates = Vec::new();
    for issue_id in mission_issue_ids {
        if let Some(roots) = duplicate_reachability_roots(db, issue_id, linked_root_ids)? {
            duplicates.push(DuplicateReachability {
                issue_id: issue_id.clone(),
                roots,
            });
        }
    }
    duplicates.sort_by(|a, b| a.issue_id.cmp(&b.issue_id));
    Ok(duplicates)
}

fn duplicate_reachability_roots(
    db: &Database,
    issue_id: &str,
    linked_root_ids: &BTreeSet<String>,
) -> Result<Option<Vec<String>>> {
    let mut roots = Vec::new();
    let mut current_id = Some(issue_id.to_string());
    while let Some(id) = current_id {
        if linked_root_ids.contains(&id) {
            roots.push(id.clone());
        }
        current_id = db.require_issue(&id)?.parent_id;
    }
    if roots.len() > 1 {
        Ok(Some(roots))
    } else {
        Ok(None)
    }
}

fn format_duplicate_reachability(duplicate: &DuplicateReachability) -> String {
    let mut qualifiers = duplicate
        .roots
        .iter()
        .map(|root| {
            if root == &duplicate.issue_id {
                "direct".to_string()
            } else {
                root.clone()
            }
        })
        .collect::<Vec<_>>();
    qualifiers.sort();
    format!("{} ({})", duplicate.issue_id, qualifiers.join(" + "))
}

fn compare_mission_list_epics(a: &MissionListEpic, b: &MissionListEpic) -> std::cmp::Ordering {
    mission_status_rank(&a.issue.status)
        .cmp(&mission_status_rank(&b.issue.status))
        .then_with(|| a.issue.id.cmp(&b.issue.id))
}

fn has_ancestor_in_set(
    db: &Database,
    issue: &Issue,
    ancestor_ids: &BTreeSet<String>,
) -> Result<bool> {
    let mut parent_id = issue.parent_id.clone();
    while let Some(id) = parent_id {
        if ancestor_ids.contains(&id) {
            return Ok(true);
        }
        parent_id = db.require_issue(&id)?.parent_id;
    }
    Ok(false)
}

impl WorkCounts {
    fn add_bucket(&mut self, bucket: &str) {
        match bucket {
            "ready" => self.ready += 1,
            "blocked" => self.blocked += 1,
            "done" => self.done += 1,
            _ => self.backlog += 1,
        }
    }

    fn is_empty(self) -> bool {
        self.ready == 0 && self.blocked == 0 && self.done == 0 && self.backlog == 0
    }

    fn to_inline_text(self) -> String {
        format!(
            "ready {}, blocked {}, done {}",
            self.ready, self.blocked, self.done
        )
    }

    fn to_compact_text(self) -> String {
        let mut parts = Vec::new();
        if self.ready > 0 {
            parts.push(count_label(self.ready, "ready"));
        }
        if self.blocked > 0 {
            parts.push(count_label(self.blocked, "blocked"));
        }
        if self.done > 0 {
            parts.push(count_label(self.done, "done"));
        }
        if self.backlog > 0 {
            parts.push(count_label(self.backlog, "backlog"));
        }
        if parts.is_empty() {
            "none".to_string()
        } else {
            parts.join(", ")
        }
    }
}

struct TrackerHealth {
    stale_entries: Vec<String>,
}

impl TrackerHealth {
    fn status_token(&self) -> &'static str {
        if self.stale_entries.is_empty() {
            "ok"
        } else {
            "stale"
        }
    }

    fn status_text(&self) -> String {
        if self.stale_entries.is_empty() {
            "ok".to_string()
        } else {
            format!("stale ({} findings)", self.stale_entries.len())
        }
    }
}

fn tracker_health(db: &Database, state_dir: &Path) -> TrackerHealth {
    let stale_entries = super::export::canonical_stale_entries(db, state_dir)
        .unwrap_or_else(|error| vec![format!("tracker health check failed: {error:#}")]);
    TrackerHealth { stale_entries }
}

fn mission_health(summary: &MissionListSummary) -> &'static str {
    let work = summary.total_work();
    if summary.open_blockers > 0 || work.blocked > 0 {
        "blocked"
    } else if summary.closeout_needed() {
        "closeout"
    } else if work.ready > 0 {
        "ready"
    } else if summary.evidence_gap_count() > 0 {
        "needs-evidence"
    } else {
        "steady"
    }
}

fn mission_health_for(mission: &DomainRecord, summary: &MissionListSummary) -> &'static str {
    if mission_lifecycle_status(mission) == "closed" {
        "closed"
    } else {
        mission_health(summary)
    }
}

fn active_work_for_mission(
    db: &Database,
    mission_id: &str,
) -> Result<Vec<crate::models::WorkAssociation>> {
    let issue_ids = mission_issue_ids(db, mission_id)?;
    Ok(db
        .list_work_associations()?
        .into_iter()
        .filter(|work| work.status == "active" && issue_ids.contains(&work.issue_id))
        .collect())
}

fn mission_issue_ids(db: &Database, mission_id: &str) -> Result<BTreeSet<String>> {
    let mut issue_ids = BTreeSet::new();
    for link in db.list_record_links(KIND, mission_id)? {
        let Some((kind, linked_id)) = other_side(&link, KIND, mission_id) else {
            continue;
        };
        if kind == "issue" && link.relation_type == "advances" {
            collect_issue_and_descendants(db, linked_id, &mut issue_ids)?;
        }
    }
    Ok(issue_ids)
}

fn is_active_mission(record: &DomainRecord) -> bool {
    record.status == "active"
}

fn set_mission_active_state(record: &mut DomainRecord, active: bool) -> Result<bool> {
    let target_status = if active { "active" } else { "ready" };
    if is_active_mission(record) == active && record.status == target_status {
        return Ok(false);
    }
    record.status = target_status.to_string();
    Ok(true)
}

fn mission_focus_label(record: &DomainRecord) -> String {
    mission_lifecycle_status(record)
}

fn collect_issue_and_descendants(
    db: &Database,
    issue_id: &str,
    issue_ids: &mut BTreeSet<String>,
) -> Result<()> {
    if !issue_ids.insert(issue_id.to_string()) {
        return Ok(());
    }
    for child in db.get_subissues(issue_id)? {
        collect_issue_and_descendants(db, &child.id, issue_ids)?;
    }
    Ok(())
}

fn count_label(count: usize, label: &str) -> String {
    format!("{count} {label}")
}

fn plural_noun(count: usize, noun: &str) -> String {
    if count == 1 {
        noun.to_string()
    } else {
        format!("{noun}s")
    }
}

fn render_mission_show_human(
    mission: &DomainRecord,
    plans: &[Value],
    milestones: &[Value],
    evidence: &[Value],
    work: &BTreeMap<String, Vec<Value>>,
    mission_blockers: &[Value],
    supporting: &[Value],
) -> Result<()> {
    let sections = record_store::mission_sections_from_domain_record(mission)?;
    let status = mission_lifecycle_status(mission);
    let identity = format!("Mission {} [{}] - {}", mission.id, status, mission.title);
    println!("{identity}");
    println!("{}", "=".repeat(identity.len()));
    println!("Status:   {}", status);
    println!("Created:  {}", format_human_datetime(mission.created_at));
    println!("Updated:  {}", format_human_datetime(mission.updated_at));

    print_mission_section("Intent", &sections.intent);
    print_mission_section("Constraints", &sections.constraints);
    print_mission_section("Risks", &sections.risks);
    print_mission_section("Validation", &sections.validation);
    if let Some(closeout_notes) = sections.closeout_notes.as_deref() {
        print_mission_section("Closeout Notes", closeout_notes);
    }
    if let Some(notes) = sections.notes.as_deref() {
        print_mission_section("Notes", notes);
    }

    print_mission_heading("Progress");
    println!(
        "Records: plans={} milestones={} evidence={}",
        plans.len(),
        milestones.len(),
        evidence.len()
    );
    println!(
        "Work: ready={} blocked={} done={} backlog={}",
        work_bucket_len(work, "ready"),
        work_bucket_len(work, "blocked"),
        work_bucket_len(work, "done"),
        work_bucket_len(work, "backlog")
    );
    println!("Mission Blockers: {}", mission_blockers.len());

    print_record_group("Plans", plans);
    print_record_group("Milestones", milestones);
    print_record_group("Evidence", evidence);
    print_mission_blockers(mission_blockers);
    print_work_groups(work);
    print_record_group("Supporting Records", supporting);
    print_evidence_gaps(evidence);
    print_mission_next_commands(mission);
    Ok(())
}

fn print_mission_section(title: &str, value: &str) {
    print_mission_heading(title);
    if value.trim().is_empty() {
        println!("(none)");
        return;
    }
    println!("{}", value.trim());
}

fn print_mission_heading(title: &str) {
    println!("\n{title}");
    println!("{}", "-".repeat(title.len()));
}

fn print_record_group(title: &str, records: &[Value]) {
    print_mission_heading(title);
    if records.is_empty() {
        println!("(none)");
        return;
    }
    for record in records {
        println!("  {}", record_row(record));
    }
}

fn print_mission_blockers(blockers: &[Value]) {
    print_mission_heading("Mission Blockers");
    if blockers.is_empty() {
        println!("(none)");
        return;
    }
    for blocker in blockers {
        let marker = if blocker["status_category"].as_str() != Some("done") {
            " (open blocker)"
        } else {
            ""
        };
        println!("  {}{}", issue_row(blocker), marker);
    }
}

fn print_work_groups(work: &BTreeMap<String, Vec<Value>>) {
    print_mission_heading("Linked Work");
    let groups = [
        ("Ready", "ready"),
        ("Blocked", "blocked"),
        ("Done", "done"),
        ("Backlog", "backlog"),
    ];
    if groups
        .iter()
        .all(|(_, bucket)| work_bucket_len(work, bucket) == 0)
    {
        println!("(none)");
        return;
    }
    for (label, bucket) in groups {
        let Some(items) = work.get(bucket) else {
            continue;
        };
        if items.is_empty() {
            continue;
        }
        println!("{label} ({})", items.len());
        for item in items {
            println!("  {}", issue_row(item));
        }
    }
}

fn print_evidence_gaps(evidence: &[Value]) {
    print_mission_heading("Evidence Gaps");
    if evidence.is_empty() {
        println!("  No evidence records are linked to this mission.");
    } else {
        println!("(none)");
    }
}

fn print_mission_next_commands(mission: &DomainRecord) {
    print_mission_heading("Next Commands");
    println!("  atelier mission status {}", mission.id);
    println!("  atelier mission show {}", mission.id);
    println!("  atelier history --mission {}", mission.id);
    if mission_lifecycle_status(mission) == "closed" {
        println!("  atelier mission update {} --status ready", mission.id);
    } else {
        println!("  atelier mission add-work {} <issue-id>", mission.id);
        println!("  atelier mission status {}", mission.id);
    }
}

fn work_bucket_len(work: &BTreeMap<String, Vec<Value>>, bucket: &str) -> usize {
    work.get(bucket).map_or(0, Vec::len)
}

fn record_row(record: &Value) -> String {
    let relation_type = value_str(record, "relation_type");
    let relation = if relation_type == "(unknown)" || relation_type.is_empty() {
        String::new()
    } else {
        format!(" ({})", readable_relation(relation_type))
    };
    format!(
        "{} [{}] - {}",
        value_str(record, "id"),
        value_str(record, "status"),
        value_str(record, "title")
    ) + &relation
}

fn issue_row(issue: &Value) -> String {
    let open_blockers = issue["open_blockers"]
        .as_array()
        .map_or(0, |blockers| blockers.len());
    let relation_type = value_str(issue, "relation_type");
    let relation = if relation_type == "(unknown)" || relation_type.is_empty() {
        String::new()
    } else {
        format!(" ({})", readable_relation(relation_type))
    };
    let blocker_suffix = if open_blockers == 1 {
        " - 1 open blocker".to_string()
    } else if open_blockers > 1 {
        format!(" - {open_blockers} open blockers")
    } else {
        String::new()
    };
    format!(
        "{} [{}] {} {} - {}{}{}",
        value_str(issue, "id"),
        value_status_label(issue),
        value_str(issue, "priority"),
        value_str(issue, "issue_type"),
        value_str(issue, "title"),
        relation,
        blocker_suffix
    )
}

fn readable_relation(relation_type: &str) -> String {
    relation_type.replace('_', " ")
}

fn format_human_datetime(timestamp: DateTime<Utc>) -> String {
    timestamp
        .with_timezone(&Local)
        .format("%Y-%m-%d %H:%M %Z")
        .to_string()
}

fn value_str<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or("(unknown)")
}

fn value_status_label(value: &Value) -> String {
    let status = value_str(value, "status");
    match value["status_category"].as_str() {
        Some(category) => format!("{category}/{status}"),
        None => status.to_string(),
    }
}

fn other_side<'a>(link: &'a RecordLink, kind: &str, id: &str) -> Option<(&'a str, &'a str)> {
    if link.source_kind == kind && link.source_id == id {
        Some((&link.target_kind, &link.target_id))
    } else if link.target_kind == kind && link.target_id == id {
        Some((&link.source_kind, &link.source_id))
    } else {
        None
    }
}

fn record_summary(db: &Database, kind: &str, id: &str) -> Result<Value> {
    let record = db.require_record(kind, id)?;
    Ok(json!({
        "id": record.id,
        "kind": record.kind,
        "title": record.title,
        "status": record.status,
    }))
}

fn linked_record_summary(
    db: &Database,
    kind: &str,
    id: &str,
    relation_type: &str,
) -> Result<Value> {
    if kind == "issue" {
        let issue = db.require_issue(id)?;
        return Ok(json!({
            "id": issue.id,
            "kind": "issue",
            "title": issue.title,
            "status": issue.status,
            "relation_type": relation_type,
        }));
    }
    let mut summary = record_summary(db, kind, id)?;
    summary["relation_type"] = Value::String(relation_type.to_string());
    Ok(summary)
}

fn issue_json_with_relation(db: &Database, issue: &Issue, relation_type: &str) -> Result<Value> {
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    Ok(json!({
        "id": issue.id,
        "title": issue.title,
        "status": issue.status,
        "status_category": crate::commands::issue_workflow::issue_status_category(
            workflow_policy.as_ref(),
            &issue.status,
        ),
        "priority": issue.priority,
        "issue_type": issue.issue_type,
        "relation_type": relation_type,
        "open_blockers": open_blockers(db, &issue.id)?,
    }))
}

fn issue_bucket(db: &Database, issue: &Issue) -> Result<&'static str> {
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    if crate::commands::issue_workflow::issue_is_done(workflow_policy.as_ref(), issue) {
        return Ok("done");
    }
    if !open_blockers(db, &issue.id)?.is_empty() {
        return Ok("blocked");
    }
    match crate::commands::issue_workflow::issue_start_readiness(
        db,
        workflow_policy.as_ref(),
        issue,
    )? {
        crate::commands::issue_workflow::IssueStartReadiness::Ready => return Ok("ready"),
        crate::commands::issue_workflow::IssueStartReadiness::Blocked => return Ok("blocked"),
        crate::commands::issue_workflow::IssueStartReadiness::NotReady => {}
    }
    Ok("backlog")
}

fn open_blockers(db: &Database, issue_id: &str) -> Result<Vec<String>> {
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let mut blockers = Vec::new();
    for blocker_id in db.get_blockers(issue_id)? {
        if crate::commands::issue_workflow::issue_blocks_work(
            workflow_policy.as_ref(),
            &db.require_issue(&blocker_id)?,
        ) {
            blockers.push(blocker_id);
        }
    }
    blockers.sort();
    Ok(blockers)
}

fn is_selectable_work(db: &Database, issue: &Issue) -> Result<bool> {
    Ok(issue.issue_type != "epic" || db.get_subissues(&issue.id)?.is_empty())
}

fn parent_context(issue: &Issue) -> String {
    match issue.parent_id.as_deref() {
        Some(parent_id) => format!("parent {parent_id}"),
        None => "mission-linked root".to_string(),
    }
}

fn proof_context(db: &Database, issue_id: &str) -> Result<&'static str> {
    if has_validating_evidence(db, issue_id)? {
        Ok("proof attached")
    } else {
        Ok("proof missing")
    }
}

fn has_validating_evidence(db: &Database, issue_id: &str) -> Result<bool> {
    for link in db.list_record_links("issue", issue_id)? {
        if link.relation_type != "validates" {
            continue;
        }
        if link.source_kind == "evidence" || link.target_kind == "evidence" {
            return Ok(true);
        }
    }
    Ok(false)
}

fn replace_section_list(section: &mut String, values: Vec<String>) {
    if !values.is_empty() {
        *section = values
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
            .join("\n");
    }
}
