use anyhow::Result;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

use crate::db::Database;
use crate::models::DomainRecord;
use crate::record_store::RecordStore;

const KIND: &str = "evidence";

pub fn add(
    state_dir: &Path,
    db_path: &Path,
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
    let store = RecordStore::new(state_dir);
    let created =
        store.create_domain_record(KIND, summary, result, Some(summary), &data.to_string())?;
    refresh_projection(state_dir, db_path)?;
    let db = Database::open(db_path)?;
    let record = db.require_record(KIND, &created.record.id)?;
    print_record(&record)
}

pub fn show(db: &Database, id: &str) -> Result<()> {
    let record = canonical_record_detail(KIND, id)?.unwrap_or(db.require_record(KIND, id)?);
    print_record(&record)
}

pub fn attach(
    state_dir: &Path,
    db_path: &Path,
    id: &str,
    target_kind: &str,
    target_id: &str,
    role: &str,
) -> Result<()> {
    let db = Database::open(db_path)?;
    db.require_record(KIND, id)?;
    validate_record_ref(&db, target_kind, target_id)?;
    drop(db);
    let store = RecordStore::new(state_dir);
    let inserted = store.add_attachment_relationship(KIND, id, target_kind, target_id, role)?;
    refresh_projection(state_dir, db_path)?;
    if inserted && target_kind == "issue" {
        let db = Database::open(db_path)?;
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

fn validate_record_ref(db: &Database, kind: &str, id: &str) -> Result<()> {
    crate::db::validate_record_kind(kind)?;
    if kind == "issue" {
        db.require_issue(id)?;
    } else {
        db.require_record(kind, id)?;
    }
    Ok(())
}

fn refresh_projection(state_dir: &Path, db_path: &Path) -> Result<()> {
    super::projection::refresh_after_canonical_write(state_dir, db_path)
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
