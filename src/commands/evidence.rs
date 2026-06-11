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
        print_heading("Evidence");
        println!("(none)");
        return Ok(());
    }
    print_heading("Evidence");
    println!("{} total", records.len());
    for record in records {
        let data = evidence_data(&record)?;
        let kind = data["kind"].as_str().unwrap_or("unknown");
        println!(
            "  {:<14} {:<13} {:<10} {}",
            record.id, record.status, kind, record.title
        );
    }
    Ok(())
}

fn print_record(record: &DomainRecord, json_output: bool) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(&record_json(record)?)?);
    } else {
        let data = evidence_data(record)?;
        println!(
            "{} [evidence] {} - {}",
            record.id, record.status, record.title
        );
        println!(
            "{}",
            "=".repeat(record.id.len() + record.status.len() + record.title.len() + 15)
        );
        println!("Result:      {}", record.status);
        println!(
            "Kind:        {}",
            data["kind"].as_str().unwrap_or("unknown")
        );
        println!(
            "Captured:    {}",
            data["captured_at"].as_str().unwrap_or("(unknown)")
        );
        println!(
            "Producer:    {}",
            data["producer"].as_str().unwrap_or("(none)")
        );
        println!("Path:        {}", data["path"].as_str().unwrap_or("(none)"));
        println!("URI:         {}", data["uri"].as_str().unwrap_or("(none)"));
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
    }
    Ok(())
}

fn print_heading(title: &str) {
    println!("{title}");
    println!("{}", "-".repeat(title.len()));
}

fn evidence_data(record: &DomainRecord) -> Result<Value> {
    Ok(serde_json::from_str::<Value>(&record.data_json)?)
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
