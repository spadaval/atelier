use anyhow::{bail, Result};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

use crate::db::Database;
use crate::models::{DomainRecord, Issue, RecordLink};

const KIND: &str = "mission";

pub fn create(
    db: &Database,
    title: &str,
    body: Option<&str>,
    constraints: Vec<String>,
    risks: Vec<String>,
    validation: Vec<String>,
    json_output: bool,
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
    let id = db.create_record(KIND, title, "open", body, &data.to_string())?;
    let record = db.require_record(KIND, &id)?;
    print_record(db, &record, json_output)
}

pub fn show(db: &Database, id: &str, json_output: bool) -> Result<()> {
    view(db, id, json_output)
}

pub fn view(db: &Database, id: &str, json_output: bool) -> Result<()> {
    let mission = db.require_record(KIND, id)?;
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

    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "mission": record_json(&mission)?,
                "plans": plans,
                "milestones": milestones,
                "evidence": evidence,
                "work": work,
                "mission_blockers": mission_blockers,
                "evidence_gaps": if evidence.is_empty() {
                    vec!["No evidence records are linked to this mission.".to_string()]
                } else {
                    Vec::<String>::new()
                },
                "links": links,
            }))?
        );
        return Ok(());
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
        println!("No missions.");
        return Ok(());
    }
    for record in records {
        println!("{:<14} {:<10} {}", record.id, record.status, record.title);
    }
    Ok(())
}

pub fn update(
    db: &Database,
    id: &str,
    title: Option<&str>,
    status: Option<&str>,
    body: Option<&str>,
    constraints: Vec<String>,
    risks: Vec<String>,
    validation: Vec<String>,
    json_output: bool,
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
    let current = db.require_record(KIND, id)?;
    let mut data: Value = serde_json::from_str(&current.data_json)?;
    replace_array(&mut data, "constraints", constraints);
    replace_array(&mut data, "risks", risks);
    replace_array(&mut data, "validation", validation);
    db.update_record(
        KIND,
        id,
        title,
        status,
        body,
        Some(&serde_json::to_string(&data)?),
    )?;
    let record = db.require_record(KIND, id)?;
    print_record(db, &record, json_output)
}

fn print_record(db: &Database, record: &DomainRecord, json_output: bool) -> Result<()> {
    if json_output {
        let mut data = record_json(record)?;
        data["links"] = serde_json::to_value(db.list_record_links(KIND, &record.id)?)?;
        println!("{}", serde_json::to_string_pretty(&data)?);
        return Ok(());
    }
    println!("Mission {}: {}", record.id, record.title);
    println!("Status: {}", record.status);
    if let Some(body) = &record.body {
        if !body.is_empty() {
            println!("\n{}", body);
        }
    }
    Ok(())
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
    println!("Created:  {}", mission.created_at.to_rfc3339());
    println!("Updated:  {}", mission.updated_at.to_rfc3339());

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
    println!("Mission blockers={}", mission_blockers.len());

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
            " OPEN BLOCKER"
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
        println!(
            "  atelier link add mission {} issue <issue-id> --type advances",
            mission.id
        );
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
    let blocker_suffix = if open_blockers > 0 {
        format!(" open_blockers={open_blockers}")
    } else {
        String::new()
    };
    format!(
        "{} [{}] {} {} - {} relation={}{}",
        value_str(issue, "id"),
        value_str(issue, "status"),
        value_str(issue, "priority"),
        value_str(issue, "issue_type"),
        value_str(issue, "title"),
        relation_type,
        blocker_suffix
    )
}

fn value_str<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or("(unknown)")
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
