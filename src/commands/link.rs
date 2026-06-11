use anyhow::Result;
use serde_json::json;
use std::collections::BTreeMap;

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
    if inserted {
        record_evidence_activity(db, source_kind, source_id, target_kind, target_id)?;
    }
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

fn record_evidence_activity(
    db: &Database,
    source_kind: &str,
    source_id: &str,
    target_kind: &str,
    target_id: &str,
) -> Result<()> {
    match (source_kind, target_kind) {
        ("evidence", "issue") => {
            let evidence = db.require_record("evidence", source_id)?;
            super::activity_log::record_evidence_attached(
                target_id,
                source_id,
                Some(&evidence.status),
            )
        }
        ("issue", "evidence") => {
            let evidence = db.require_record("evidence", target_id)?;
            super::activity_log::record_evidence_attached(
                source_id,
                target_id,
                Some(&evidence.status),
            )
        }
        _ => Ok(()),
    }
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
        println!(
            "Removed link: {source_kind} {source_id} {relation_type} {target_kind} {target_id}"
        );
        println!();
        println!("Next Commands");
        println!("-------------");
        println!("  atelier link list {source_kind} {source_id}");
    } else {
        println!(
            "No such link: {source_kind} {source_id} {relation_type} {target_kind} {target_id}"
        );
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
    println!("Links for {kind} {id}");
    println!("{}", "=".repeat(format!("Links for {kind} {id}").len()));
    let mut grouped = BTreeMap::<String, Vec<_>>::new();
    for link in links {
        grouped
            .entry(link.relation_type.clone())
            .or_default()
            .push(link);
    }
    for (relation, links) in grouped {
        let heading = relation.replace('_', " ");
        println!("\n{heading}");
        println!("{}", "-".repeat(heading.len()));
        for link in links {
            println!(
                "  {} {} -> {} {}",
                link.source_kind, link.source_id, link.target_kind, link.target_id
            );
        }
    }
    Ok(())
}
