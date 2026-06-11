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
    print_record(&record)
}

pub fn show(db: &Database, id: &str) -> Result<()> {
    let record = db.require_record(KIND, id)?;
    print_record(&record)
}

pub fn attach(
    db: &Database,
    id: &str,
    target_kind: &str,
    target_id: &str,
    role: &str,
) -> Result<()> {
    db.require_record(KIND, id)?;
    let inserted = db.add_record_link(KIND, id, target_kind, target_id, role)?;
    if inserted && target_kind == "issue" {
        let evidence = db.require_record(KIND, id)?;
        super::activity_log::record_evidence_attached(target_id, id, Some(&evidence.status))?;
    }
    if inserted {
        println!("Attached evidence {id} to {target_kind} {target_id} ({role})");
    } else {
        println!("Evidence {id} is already attached to {target_kind} {target_id} ({role})");
    }
    Ok(())
}

pub fn list(db: &Database, result: Option<&str>) -> Result<()> {
    let records = db.list_records(KIND, result)?;
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

fn print_record(record: &DomainRecord) -> Result<()> {
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
    Ok(())
}

fn print_heading(title: &str) {
    println!("{title}");
    println!("{}", "-".repeat(title.len()));
}

fn evidence_data(record: &DomainRecord) -> Result<Value> {
    Ok(serde_json::from_str::<Value>(&record.data_json)?)
}
