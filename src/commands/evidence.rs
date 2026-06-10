use anyhow::Result;
use serde_json::{json, Value};

use crate::db::Database;
use crate::models::DomainRecord;

const KIND: &str = "evidence";

pub fn add(
    db: &Database,
    evidence_kind: &str,
    result: &str,
    summary: &str,
    path: Option<&str>,
    uri: Option<&str>,
    producer: Option<&str>,
    json_output: bool,
) -> Result<()> {
    let data = json!({
        "kind": evidence_kind,
        "result": result,
        "path": path,
        "uri": uri,
        "producer": producer,
        "captured_at": chrono::Utc::now().to_rfc3339()
    });
    let id = db.create_record(KIND, summary, result, Some(summary), &data.to_string())?;
    let record = db.require_record(KIND, &id)?;
    print_record(&record, json_output)
}

pub fn show(db: &Database, id: &str, json_output: bool) -> Result<()> {
    let record = db.require_record(KIND, id)?;
    print_record(&record, json_output)
}

pub fn list(db: &Database, result: Option<&str>, json_output: bool) -> Result<()> {
    let records = db.list_records(KIND, result)?;
    if json_output {
        let data: Vec<Value> = records.iter().map(record_json).collect::<Result<_>>()?;
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "data": data }))?
        );
        return Ok(());
    }
    if records.is_empty() {
        println!("No evidence.");
        return Ok(());
    }
    for record in records {
        println!("{:<14} {:<10} {}", record.id, record.status, record.title);
    }
    Ok(())
}

fn print_record(record: &DomainRecord, json_output: bool) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(&record_json(record)?)?);
    } else {
        println!("Evidence {}: {}", record.id, record.title);
        println!("Result: {}", record.status);
    }
    Ok(())
}

fn record_json(record: &DomainRecord) -> Result<Value> {
    Ok(json!({
        "id": record.id,
        "kind": record.kind,
        "title": record.title,
        "result": record.status,
        "summary": record.body,
        "data": serde_json::from_str::<Value>(&record.data_json)?,
        "created_at": record.created_at.to_rfc3339(),
        "updated_at": record.updated_at.to_rfc3339()
    }))
}
