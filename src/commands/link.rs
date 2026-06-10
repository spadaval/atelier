use anyhow::Result;
use serde_json::json;

use crate::db::Database;

pub fn add(
    db: &Database,
    source_kind: &str,
    source_id: &str,
    target_kind: &str,
    target_id: &str,
    relation_type: &str,
    json_output: bool,
) -> Result<()> {
    let inserted = db.add_record_link(
        source_kind,
        source_id,
        target_kind,
        target_id,
        relation_type,
    )?;
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "inserted": inserted,
                "source": { "kind": source_kind, "id": source_id },
                "target": { "kind": target_kind, "id": target_id },
                "type": relation_type
            }))?
        );
    } else if inserted {
        println!("Linked {source_kind} {source_id} {relation_type} {target_kind} {target_id}");
    } else {
        println!("Link already exists");
    }
    Ok(())
}

pub fn remove(
    db: &Database,
    source_kind: &str,
    source_id: &str,
    target_kind: &str,
    target_id: &str,
    relation_type: &str,
    json_output: bool,
) -> Result<()> {
    let removed = db.remove_record_link(
        source_kind,
        source_id,
        target_kind,
        target_id,
        relation_type,
    )?;
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "removed": removed }))?
        );
    } else if removed {
        println!("Removed link");
    } else {
        println!("No such link");
    }
    Ok(())
}

pub fn list(db: &Database, kind: &str, id: &str, json_output: bool) -> Result<()> {
    let links = db.list_record_links(kind, id)?;
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "data": links }))?
        );
        return Ok(());
    }
    if links.is_empty() {
        println!("No links.");
        return Ok(());
    }
    for link in links {
        println!(
            "{} {} --{}--> {} {}",
            link.source_kind, link.source_id, link.relation_type, link.target_kind, link.target_id
        );
    }
    Ok(())
}
