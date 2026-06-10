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
    let record = db.require_record(KIND, id)?;
    print_record(db, &record, json_output)
}

pub fn view(db: &Database, id: &str, json_output: bool) -> Result<()> {
    let mission = db.require_record(KIND, id)?;
    let links = db.list_record_links(KIND, id)?;
    let mut plans = Vec::new();
    let mut evidence = Vec::new();
    let mut milestones = Vec::new();
    let mut seen_records = BTreeSet::new();
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
                if !seen_work.insert(linked_id.to_string()) {
                    continue;
                }
                let issue = db.require_issue(linked_id)?;
                let bucket = issue_bucket(db, &issue)?;
                work.get_mut(bucket)
                    .expect("known work bucket")
                    .push(json!({
                        "id": issue.id,
                        "title": issue.title,
                        "status": issue.status,
                        "priority": issue.priority,
                        "issue_type": issue.issue_type,
                        "relation_type": link.relation_type,
                        "open_blockers": open_blockers(db, &issue.id)?,
                    }));
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

    println!("Mission {}: {}", mission.id, mission.title);
    println!("Status: {}", mission.status);
    println!(
        "Plans: {}  Milestones: {}  Evidence: {}",
        plans.len(),
        milestones.len(),
        evidence.len()
    );
    println!(
        "Work: {} ready, {} blocked, {} done, {} backlog",
        work["ready"].len(),
        work["blocked"].len(),
        work["done"].len(),
        work["backlog"].len()
    );
    if evidence.is_empty() {
        println!("Evidence gap: no evidence records are linked to this mission.");
    }
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
