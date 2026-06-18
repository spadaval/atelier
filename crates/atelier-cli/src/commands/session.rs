use anyhow::{bail, Result};
use atelier_core::{Record, SessionRecord, SessionTarget};
use atelier_records::RecordStore;
use atelier_sqlite::Database;
use chrono::Utc;
use std::path::Path;

const VALID_ROLES: &[&str] = &["worker", "reviewer", "manager", "admin"];

pub fn begin(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    role: &str,
    issue: Option<&str>,
    mission: Option<&str>,
    subskill: Option<&str>,
    agent: Option<&str>,
    kind: &str,
) -> Result<()> {
    validate_role(role)?;
    let target = session_target(db, issue, mission)?;
    let title = target
        .as_ref()
        .map(|target| format!("{role} session for {} {}", target.kind, target.id))
        .unwrap_or_else(|| format!("{role} session"));
    let store = RecordStore::new(state_dir);
    let record = store.create_session(
        &title,
        role,
        agent.map(str::to_string),
        subskill.map(str::to_string),
        target,
        kind,
    )?;
    atelier_app::projection::refresh_after_canonical_write(state_dir, db_path)?;
    print_record(&record);
    Ok(())
}

pub fn show(state_dir: &Path, id: &str) -> Result<()> {
    let record = load_session(state_dir, id)?;
    print_record(&record);
    Ok(())
}

pub fn list(state_dir: &Path, active: bool) -> Result<()> {
    let store = RecordStore::new(state_dir);
    let records = store
        .load_sessions()?
        .into_iter()
        .filter(|record| !active || record.header.status == "active")
        .collect::<Vec<_>>();
    println!("Sessions");
    println!("--------");
    if records.is_empty() {
        println!("(none)");
        return Ok(());
    }
    for record in records {
        let target = format_target(record.data.target.as_ref());
        println!(
            "  {:<14} {:<8} {:<9} {:<10} {}",
            record.header.id,
            record.header.status,
            record.data.role,
            record.data.session_kind,
            target
        );
    }
    Ok(())
}

pub fn end(state_dir: &Path, db_path: &Path, id: &str, reason: &str) -> Result<()> {
    if reason.trim().is_empty() {
        bail!("session end requires --reason \"...\"");
    }
    let store = RecordStore::new(state_dir);
    let mut record = load_session(state_dir, id)?;
    if record.header.status == "ended" {
        bail!("Session {id} is already ended");
    }
    let now = Utc::now();
    record.header.status = "ended".to_string();
    record.header.updated_at = now;
    record.data.ended_at = Some(now);
    store.write_record_atomic(&Record::Session(record.clone()))?;
    atelier_app::projection::refresh_after_canonical_write(state_dir, db_path)?;
    println!("Ended session {} - {}", record.header.id, reason.trim());
    Ok(())
}

fn load_session(state_dir: &Path, id: &str) -> Result<SessionRecord> {
    match RecordStore::new(state_dir).load_record_by_id("session", id)? {
        Record::Session(record) => Ok(record),
        other => bail!("Expected session record {id}, found {}", other.kind()),
    }
}

fn session_target(
    db: &Database,
    issue: Option<&str>,
    mission: Option<&str>,
) -> Result<Option<SessionTarget>> {
    match (issue, mission) {
        (Some(_), Some(_)) => bail!("Use either --issue or --mission, not both"),
        (Some(id), None) => {
            db.require_issue(id)?;
            Ok(Some(SessionTarget {
                kind: "issue".to_string(),
                id: id.to_string(),
            }))
        }
        (None, Some(id)) => {
            db.require_record("mission", id)?;
            Ok(Some(SessionTarget {
                kind: "mission".to_string(),
                id: id.to_string(),
            }))
        }
        (None, None) => Ok(None),
    }
}

fn validate_role(role: &str) -> Result<()> {
    if VALID_ROLES.contains(&role) {
        Ok(())
    } else {
        bail!(
            "Invalid session role '{}'. Valid roles: {}",
            role,
            VALID_ROLES.join(", ")
        )
    }
}

fn print_record(record: &SessionRecord) {
    println!(
        "{} [session] {} - {}",
        record.header.id, record.header.status, record.header.title
    );
    println!("Role:        {}", record.data.role);
    println!("Kind:        {}", record.data.session_kind);
    println!(
        "Target:      {}",
        format_target(record.data.target.as_ref())
    );
    println!(
        "Agent:       {}",
        record.data.agent_identity.as_deref().unwrap_or("(none)")
    );
    println!(
        "Subskill:    {}",
        record.data.subskill.as_deref().unwrap_or("(none)")
    );
    println!("Started:     {}", record.data.started_at.to_rfc3339());
    println!(
        "Ended:       {}",
        record
            .data
            .ended_at
            .map(|ended| ended.to_rfc3339())
            .unwrap_or_else(|| "(active)".to_string())
    );
}

pub fn format_target(target: Option<&SessionTarget>) -> String {
    target
        .map(|target| format!("{}/{}", target.kind, target.id))
        .unwrap_or_else(|| "(none)".to_string())
}
