use anyhow::{bail, Result};
use chrono::{DateTime, Local, Utc};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::db::Database;
use crate::models::{DomainRecord, Issue, RecordLink};
use crate::record_store::RecordStore;

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
    let data = json!({
        "constraints": constraints,
        "risks": risks,
        "validation": validation,
        "milestones": [],
        "plans": [],
        "evidence": [],
        "work": []
    });
    let store = RecordStore::new(state_dir);
    let created = store.create_domain_record(KIND, title, "open", body, &data.to_string())?;
    refresh_projection(state_dir, db_path)?;
    let db = Database::open(db_path)?;
    let record = db.require_record(KIND, &created.record.id)?;
    print_record(&record)
}

pub fn show(db: &Database, id: &str) -> Result<()> {
    view(db, id)
}

pub fn view(db: &Database, id: &str) -> Result<()> {
    let mission = canonical_record_detail(KIND, id)?.unwrap_or(db.require_record(KIND, id)?);
    let links = db.list_record_links(KIND, id)?;
    let mut plans = Vec::new();
    let mut evidence = Vec::new();
    let mut milestones = Vec::new();
    let mut mission_blockers = Vec::new();
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
            "plan" if seen_records.insert((kind.to_string(), linked_id.to_string())) => {
                plans.push(record_summary(db, kind, linked_id)?)
            }
            "evidence" if seen_records.insert((kind.to_string(), linked_id.to_string())) => {
                evidence.push(record_summary(db, kind, linked_id)?)
            }
            "milestone" if seen_records.insert((kind.to_string(), linked_id.to_string())) => {
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
                if !seen_work.insert(linked_id.to_string()) {
                    continue;
                }
                let issue = db.require_issue(linked_id)?;
                let bucket = issue_bucket(db, &issue)?;
                work.get_mut(bucket)
                    .expect("known work bucket")
                    .push(issue_json_with_relation(db, &issue, &link.relation_type)?);
            }
            _ => {}
        }
    }

    render_mission_show_human(
        &mission,
        &plans,
        &milestones,
        &evidence,
        &work,
        &mission_blockers,
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

fn find_state_dir_from_cwd() -> Result<Option<PathBuf>> {
    let mut current = std::env::current_dir()?;
    loop {
        let state_dir = current.join(".atelier-state");
        if state_dir.is_dir() {
            return Ok(Some(state_dir));
        }
        if current.join(".atelier").is_dir() {
            return Ok(None);
        }
        if !current.pop() {
            return Ok(None);
        }
    }
}

pub fn list(db: &Database, status: Option<&str>) -> Result<()> {
    let status_filter = match status {
        Some("all") => None,
        Some(status) => Some(status),
        None => Some("open"),
    };
    let records = db.list_records(KIND, status_filter)?;
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
    let mut data: Value = serde_json::from_str(&current.record.data_json)?;
    replace_array(&mut data, "constraints", constraints);
    replace_array(&mut data, "risks", risks);
    replace_array(&mut data, "validation", validation);
    if let Some(title) = title {
        current.record.title = title.to_string();
    }
    if let Some(status) = status {
        current.record.status = status.to_string();
    }
    if let Some(body) = body {
        current.record.body = Some(body.to_string());
    }
    current.record.data_json = serde_json::to_string(&data)?;
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
    let inserted = store.add_attachment_relationship(KIND, id, "issue", &issue.id, "advances")?;
    refresh_projection(state_dir, db_path)?;
    if inserted {
        println!("Added work {} to mission {}", issue.id, id);
    } else {
        println!("Work {} is already on mission {}", issue.id, id);
    }
    Ok(())
}

pub fn add_blocker(state_dir: &Path, db_path: &Path, id: &str, issue_id: &str) -> Result<()> {
    let db = Database::open(db_path)?;
    db.require_record(KIND, id)?;
    let issue = db.require_issue(issue_id)?;
    drop(db);
    let store = RecordStore::new(state_dir);
    let inserted = store.add_attachment_relationship(KIND, id, "issue", &issue.id, "blocked_by")?;
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
    open_blockers: usize,
}

struct MissionListEpic {
    issue: Issue,
    work: WorkCounts,
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
                    if issue.status == "open" {
                        summary.open_blockers += 1;
                    }
                }
            }
            "issue" => {
                if !seen_work.insert(linked_id.to_string()) {
                    continue;
                }
                let issue = db.require_issue(linked_id)?;
                linked_work.push(issue);
            }
            _ => {}
        }
    }

    let linked_epic_ids = linked_work
        .iter()
        .filter(|issue| issue.issue_type == "epic")
        .map(|issue| issue.id.clone())
        .collect::<BTreeSet<_>>();

    for issue in linked_work {
        summary.work.add_bucket(issue_bucket(db, &issue)?);
        if issue.issue_type == "epic" {
            summary.epics.push(MissionListEpic {
                work: epic_work_counts(db, &issue.id)?,
                issue,
            });
        } else if !has_ancestor_in_set(db, &issue, &linked_epic_ids)? {
            summary.other_work.add_bucket(issue_bucket(db, &issue)?);
        }
    }

    summary.epics.sort_by(compare_mission_list_epics);
    Ok(summary)
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
        "Open",
        rows.iter().filter(|row| row.record.status == "open"),
    );

    let other_statuses = rows
        .iter()
        .filter(|row| row.record.status != "open" && row.record.status != "closed")
        .map(|row| row.record.status.as_str())
        .collect::<BTreeSet<_>>();
    for status in other_statuses.iter() {
        print_mission_list_group(
            &status_heading(status),
            rows.iter().filter(|row| row.record.status == *status),
        );
    }

    print_mission_list_group(
        "Closed",
        rows.iter().filter(|row| row.record.status == "closed"),
    );

    let first_actionable = rows
        .iter()
        .find(|row| row.record.status != "closed")
        .or_else(|| rows.first());
    print_mission_list_next_commands(first_actionable);
    Ok(())
}

fn mission_list_summary_line(rows: &[MissionListRow]) -> String {
    let mut statuses = BTreeMap::<String, usize>::new();
    let mut blocked_missions = 0;
    for row in rows {
        *statuses.entry(row.record.status.clone()).or_default() += 1;
        if row.summary.open_blockers > 0 || row.summary.work.blocked > 0 {
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
            row.record.id, row.record.status, row.record.title
        );
        if row.record.status == "open" {
            print_mission_list_open_work(row);
        }
    }
}

fn print_mission_list_next_commands(first_actionable: Option<&MissionListRow>) {
    print_mission_heading("Next Commands");
    if let Some(row) = first_actionable {
        println!("  atelier mission show {}", row.record.id);
    }
    println!("  atelier mission create \"...\"");
}

fn compare_mission_list_rows(a: &MissionListRow, b: &MissionListRow) -> std::cmp::Ordering {
    mission_status_rank(&a.record.status)
        .cmp(&mission_status_rank(&b.record.status))
        .then_with(|| {
            if a.record.status != "open" && b.record.status != "open" {
                a.record.status.cmp(&b.record.status)
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .then_with(|| b.record.updated_at.cmp(&a.record.updated_at))
        .then_with(|| a.record.id.cmp(&b.record.id))
}

fn mission_status_rank(status: &str) -> u8 {
    match status {
        "open" => 0,
        "closed" => 2,
        _ => 1,
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
) -> Result<()> {
    let data: Value = serde_json::from_str(&mission.data_json)?;
    let identity = format!(
        "Mission {} [{}] - {}",
        mission.id, mission.status, mission.title
    );
    println!("{identity}");
    println!("{}", "=".repeat(identity.len()));
    println!("Status:   {}", mission.status);
    println!("Created:  {}", format_human_datetime(mission.created_at));
    println!("Updated:  {}", format_human_datetime(mission.updated_at));

    print_mission_text_section("Body", mission.body.as_deref());
    print_mission_list_section("Constraints", string_array(&data, "constraints"));
    print_mission_list_section("Risks", string_array(&data, "risks"));
    print_mission_list_section("Validation", string_array(&data, "validation"));

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
    print_evidence_gaps(evidence);
    print_mission_next_commands(mission);
    Ok(())
}

fn print_mission_text_section(title: &str, body: Option<&str>) {
    if let Some(body) = body.map(str::trim).filter(|body| !body.is_empty()) {
        print_mission_heading(title);
        println!("{body}");
    }
}

fn print_mission_list_section(title: &str, values: Vec<String>) {
    print_mission_heading(title);
    if values.is_empty() {
        println!("(none)");
        return;
    }
    for value in values {
        println!("  {value}");
    }
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
        let marker = if blocker["status"].as_str() == Some("open") {
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
    println!("  atelier mission show {}", mission.id);
    if mission.status == "closed" {
        println!("  atelier mission update {} --status open", mission.id);
    } else {
        println!("  atelier mission add-work {} <issue-id>", mission.id);
        println!("  atelier workflow validate mission {}", mission.id);
    }
}

fn string_array(data: &Value, key: &str) -> Vec<String> {
    data.get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(ToOwned::to_owned)
        .collect()
}

fn work_bucket_len(work: &BTreeMap<String, Vec<Value>>, bucket: &str) -> usize {
    work.get(bucket).map_or(0, Vec::len)
}

fn record_row(record: &Value) -> String {
    format!(
        "{} [{}] - {}",
        value_str(record, "id"),
        value_str(record, "status"),
        value_str(record, "title")
    )
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
        value_str(issue, "status"),
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

fn issue_json_with_relation(db: &Database, issue: &Issue, relation_type: &str) -> Result<Value> {
    Ok(json!({
        "id": issue.id,
        "title": issue.title,
        "status": issue.status,
        "priority": issue.priority,
        "issue_type": issue.issue_type,
        "relation_type": relation_type,
        "open_blockers": open_blockers(db, &issue.id)?,
    }))
}

fn issue_bucket(db: &Database, issue: &Issue) -> Result<&'static str> {
    if issue.status == "closed" {
        return Ok("done");
    }
    if !open_blockers(db, &issue.id)?.is_empty() {
        return Ok("blocked");
    }
    if issue.status == "open" {
        return Ok("ready");
    }
    Ok("backlog")
}

fn open_blockers(db: &Database, issue_id: &str) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for blocker_id in db.get_blockers(issue_id)? {
        if db.require_issue(&blocker_id)?.status == "open" {
            blockers.push(blocker_id);
        }
    }
    blockers.sort();
    Ok(blockers)
}

fn replace_array(data: &mut Value, key: &str, values: Vec<String>) {
    if !values.is_empty() {
        data[key] = Value::Array(values.into_iter().map(Value::String).collect());
    }
}
