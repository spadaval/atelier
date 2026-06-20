use anyhow::Result;
use std::path::Path;

use crate::utils::format_issue_id;
use atelier_app::use_cases as app_use_cases;
use atelier_core::Issue;
use atelier_records::RecordStore;
use atelier_sqlite::{validate_relation_type, Database};

const BLOCKED_BY_ROLE: &str = "blocked_by";

#[cfg(test)]
pub fn add_typed(
    db: &Database,
    issue_id: &str,
    related_id: &str,
    relation_type: &str,
) -> Result<()> {
    validate_relation_type(relation_type)?;
    db.require_issue(issue_id)?;
    db.require_issue(related_id)?;

    if db.add_typed_relation(&issue_id, &related_id, relation_type)? {
        println!(
            "Linked {} ↔ {} ({})",
            format_issue_id(issue_id),
            format_issue_id(related_id),
            relation_type
        );
    } else {
        println!(
            "Issues {} and {} already have a '{}' relation",
            format_issue_id(issue_id),
            format_issue_id(related_id),
            relation_type
        );
    }

    Ok(())
}

pub fn add_typed_canonical(
    db: &Database,
    store: &RecordStore,
    issue_id: &str,
    related_id: &str,
    relation_type: &str,
) -> Result<()> {
    validate_relation_type(relation_type)?;
    db.require_issue(issue_id)?;
    db.require_issue(related_id)?;

    if store.add_issue_relation(issue_id, related_id, relation_type)? {
        println!(
            "Linked {} ↔ {} ({})",
            format_issue_id(issue_id),
            format_issue_id(related_id),
            relation_type
        );
    } else {
        println!(
            "Issues {} and {} already have a '{}' relation",
            format_issue_id(issue_id),
            format_issue_id(related_id),
            relation_type
        );
    }

    Ok(())
}

pub fn link_issue(
    state_dir: &Path,
    db_path: &Path,
    issue_ref: &str,
    target_ref: &str,
    role: &str,
) -> Result<()> {
    validate_relation_type(role)?;
    let db = Database::open(db_path)?;
    let issue_id = crate::commands::agent_factory::resolve_id(&db, issue_ref)?;
    let target_id = crate::commands::agent_factory::resolve_id(&db, target_ref)?;
    let store = RecordStore::new(state_dir);
    let changed = if role == BLOCKED_BY_ROLE {
        store.add_issue_block(&issue_id, &target_id)?
    } else {
        store.add_issue_relation(&issue_id, &target_id, role)?
    };
    drop(db);
    app_use_cases::refresh_after_canonical_write(state_dir, db_path)?;
    if changed {
        println!("Linked {issue_id} -> {target_id} ({role})");
    } else {
        println!("Link {issue_id} -> {target_id} ({role}) already exists");
    }
    print_link_next_commands(&issue_id, &target_id);
    Ok(())
}

#[cfg(test)]
pub fn remove_typed(
    db: &Database,
    issue_id: &str,
    related_id: &str,
    relation_type: &str,
) -> Result<()> {
    validate_relation_type(relation_type)?;

    if db.remove_typed_relation(&issue_id, &related_id, relation_type)? {
        println!(
            "Unlinked {} ↔ {} ({})",
            format_issue_id(issue_id),
            format_issue_id(related_id),
            relation_type
        );
    } else {
        println!(
            "No '{}' relation found between {} and {}",
            relation_type,
            format_issue_id(issue_id),
            format_issue_id(related_id)
        );
    }

    Ok(())
}

pub fn remove_typed_canonical(
    db: &Database,
    store: &RecordStore,
    issue_id: &str,
    related_id: &str,
    relation_type: &str,
) -> Result<()> {
    validate_relation_type(relation_type)?;
    db.require_issue(issue_id)?;
    db.require_issue(related_id)?;

    if store.remove_issue_relation(issue_id, related_id, relation_type)? {
        println!(
            "Unlinked {} ↔ {} ({})",
            format_issue_id(issue_id),
            format_issue_id(related_id),
            relation_type
        );
    } else {
        println!(
            "No '{}' relation found between {} and {}",
            relation_type,
            format_issue_id(issue_id),
            format_issue_id(related_id)
        );
    }

    Ok(())
}

pub fn unlink_issue(
    state_dir: &Path,
    db_path: &Path,
    issue_ref: &str,
    target_ref: &str,
    role: &str,
) -> Result<()> {
    validate_relation_type(role)?;
    let db = Database::open(db_path)?;
    let issue_id = crate::commands::agent_factory::resolve_id(&db, issue_ref)?;
    let target_id = crate::commands::agent_factory::resolve_id(&db, target_ref)?;
    let store = RecordStore::new(state_dir);
    let changed = if role == BLOCKED_BY_ROLE {
        store.remove_issue_block(&issue_id, &target_id)?
    } else {
        store.remove_issue_relation(&issue_id, &target_id, role)?
    };
    drop(db);
    app_use_cases::refresh_after_canonical_write(state_dir, db_path)?;
    if changed {
        println!("Unlinked {issue_id} -> {target_id} ({role})");
    } else {
        println!("No link {issue_id} -> {target_id} ({role}) exists");
    }
    print_link_next_commands(&issue_id, &target_id);
    Ok(())
}

fn print_link_next_commands(issue_id: &str, target_id: &str) {
    println!("Next Commands");
    println!("-------------");
    println!("  atelier issue show {issue_id}");
    println!("  atelier issue status {issue_id}");
    println!("  atelier issue show {target_id}");
}

pub fn list(db: &Database, issue_id: &str) -> Result<()> {
    db.require_issue(issue_id)?;

    let relations = db.get_typed_relations(issue_id)?;

    if relations.is_empty() {
        println!("No related issues for {}", format_issue_id(issue_id));
        return Ok(());
    }

    println!("Relations for {}:", format_issue_id(issue_id));

    // Group by relation type for cleaner display
    let mut by_type: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for rel in &relations {
        let other_id = if rel.issue_id_1 == issue_id {
            rel.issue_id_2.clone()
        } else {
            rel.issue_id_1.clone()
        };
        by_type
            .entry(rel.relation_type.clone())
            .or_default()
            .push(other_id);
    }

    for (rel_type, ids) in &by_type {
        println!("\n  [{}]:", rel_type);
        for id in ids {
            if let Some(issue) = db.get_issue(&id)? {
                let status_marker = if matches!(issue.status.as_str(), "done" | "archived") {
                    "✓"
                } else {
                    " "
                };
                println!(
                    "    {:<5} [{}] {:8} {}",
                    format_issue_id(id),
                    status_marker,
                    issue.priority,
                    issue.title
                );
            }
        }
    }

    Ok(())
}

pub fn impact(db: &Database, kind: &str, id: &str) -> Result<()> {
    match kind {
        "issue" => issue_impact(db, id),
        "mission" => mission_impact(db, id),
        _ => {
            anyhow::bail!("`atelier graph impact` supports mission and issue records; got '{kind}'")
        }
    }
}

fn issue_impact(db: &Database, issue_id: &str) -> Result<()> {
    db.require_issue(issue_id)?;

    let affected = db.downstream_impact(issue_id)?;

    if affected.is_empty() {
        println!(
            "No downstream issues found for {}",
            format_issue_id(issue_id)
        );
        return Ok(());
    }

    println!(
        "{} has downstream impact on {} issue(s):\n",
        format_issue_id(issue_id),
        affected.len()
    );

    for issue in &affected {
        print_impact_issue(issue);
    }

    println!(
        "\nThese issues are linked through hierarchy or impact-bearing relations from {}.",
        format_issue_id(issue_id)
    );
    println!("Review each issue before changing, closing, or invalidating the source.");

    Ok(())
}

fn mission_impact(db: &Database, mission_id: &str) -> Result<()> {
    let mission = db.require_record("mission", mission_id)?;
    let affected = mission_downstream_issues(db, mission_id)?;

    if affected.is_empty() {
        println!(
            "No downstream records found for mission {}",
            format_issue_id(mission_id)
        );
        return Ok(());
    }

    println!(
        "Mission {} [{}] {} has downstream impact on {} record(s):\n",
        format_issue_id(&mission.id),
        mission.status,
        mission.title,
        affected.len()
    );

    for issue in &affected {
        print_impact_issue(issue);
    }

    println!(
        "\nThese records are linked through mission work, hierarchy, or impact-bearing relations from mission {}.",
        format_issue_id(mission_id)
    );
    println!("Review each issue before changing, closing, or invalidating the mission.");

    Ok(())
}

fn mission_downstream_issues(db: &Database, mission_id: &str) -> Result<Vec<Issue>> {
    let mut seen = std::collections::BTreeSet::new();
    let mut affected = Vec::new();

    for issue in mission_linked_issues(db, mission_id)? {
        if seen.insert(issue.id.clone()) {
            affected.push(issue.clone());
        }
        for downstream in db.downstream_impact(&issue.id)? {
            if seen.insert(downstream.id.clone()) {
                affected.push(downstream);
            }
        }
    }

    Ok(affected)
}

fn mission_linked_issues(db: &Database, mission_id: &str) -> Result<Vec<Issue>> {
    let mut issues = Vec::new();
    for link in db.list_record_links("mission", mission_id)? {
        if link.relation_type != "advances" {
            continue;
        }
        let issue_id = if link.source_kind == "issue" {
            Some(link.source_id)
        } else if link.target_kind == "issue" {
            Some(link.target_id)
        } else {
            None
        };
        if let Some(issue_id) = issue_id {
            issues.push(db.require_issue(&issue_id)?);
        }
    }
    issues.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(issues)
}

fn print_impact_issue(issue: &Issue) {
    let status_marker = if matches!(issue.status.as_str(), "done" | "archived") {
        "✓"
    } else {
        " "
    };
    let parent_note = if let Some(pid) = &issue.parent_id {
        format!(" (child of {})", format_issue_id(pid))
    } else {
        String::new()
    };
    println!(
        "  {:<5} [{}] {:8} {}{}",
        format_issue_id(&issue.id),
        status_marker,
        issue.priority,
        issue.title,
        parent_note
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn setup_test_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::open(&db_path).unwrap();
        (db, dir)
    }

    #[test]
    fn test_add_relation() {
        let (db, _dir) = setup_test_db();
        let id1 = db.create_issue("Issue 1", None, "medium").unwrap();
        let id2 = db.create_issue("Issue 2", None, "medium").unwrap();

        let result = add_typed(&db, &id1, &id2, "related");
        assert!(result.is_ok());

        let related = db.get_related_issues(id1).unwrap();
        assert_eq!(related.len(), 1);
        assert_eq!(related[0].id, id2);
    }

    #[test]
    fn test_add_typed_relation() {
        let (db, _dir) = setup_test_db();
        let id1 = db.create_issue("Assumption A", None, "medium").unwrap();
        let id2 = db.create_issue("Assumption B", None, "medium").unwrap();

        let result = add_typed(&db, &id1, &id2, "assumption");
        assert!(result.is_ok());

        let by_type = db.get_issues_by_relation_type(&id1, "assumption").unwrap();
        assert_eq!(by_type.len(), 1);
        assert_eq!(by_type[0].id, id2);

        // Should not appear under "related"
        let by_related = db.get_issues_by_relation_type(&id1, "related").unwrap();
        assert_eq!(by_related.len(), 0);
    }

    #[test]
    fn test_multiple_relation_types_between_same_issues() {
        let (db, _dir) = setup_test_db();
        let id1 = db.create_issue("Issue 1", None, "medium").unwrap();
        let id2 = db.create_issue("Issue 2", None, "medium").unwrap();

        add_typed(&db, &id1, &id2, "related").unwrap();
        add_typed(&db, &id1, &id2, "assumption").unwrap();

        let relations = db.get_typed_relations(id1).unwrap();
        assert_eq!(relations.len(), 2);
    }

    #[test]
    fn test_custom_relation_type() {
        let (db, _dir) = setup_test_db();
        let id1 = db.create_issue("Issue 1", None, "medium").unwrap();
        let id2 = db.create_issue("Issue 2", None, "medium").unwrap();

        // Unknown types are accepted with a warning
        let result = add_typed(&db, &id1, &id2, "caused-by");
        assert!(result.is_ok());

        let relations = db.get_typed_relations(id1).unwrap();
        assert_eq!(relations.len(), 1);
        assert_eq!(relations[0].relation_type, "caused-by");
    }

    #[test]
    fn test_empty_relation_type_rejected() {
        let (db, _dir) = setup_test_db();
        let id1 = db.create_issue("Issue 1", None, "medium").unwrap();
        let id2 = db.create_issue("Issue 2", None, "medium").unwrap();

        let result = add_typed(&db, &id1, &id2, "");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_add_relation_bidirectional() {
        let (db, _dir) = setup_test_db();
        let id1 = db.create_issue("Issue 1", None, "medium").unwrap();
        let id2 = db.create_issue("Issue 2", None, "medium").unwrap();

        add_typed(&db, &id1, &id2, "related").unwrap();

        let related1 = db.get_related_issues(id1).unwrap();
        let related2 = db.get_related_issues(id2).unwrap();
        assert_eq!(related1.len(), 1);
        assert_eq!(related2.len(), 1);
    }

    #[test]
    fn test_add_relation_nonexistent_issue() {
        let (db, _dir) = setup_test_db();
        let id = db.create_issue("Issue 1", None, "medium").unwrap();

        let result = add_typed(&db, &id, "atelier-missing", "related");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_add_duplicate_relation() {
        let (db, _dir) = setup_test_db();
        let id1 = db.create_issue("Issue 1", None, "medium").unwrap();
        let id2 = db.create_issue("Issue 2", None, "medium").unwrap();

        add_typed(&db, &id1, &id2, "related").unwrap();
        let result = add_typed(&db, &id1, &id2, "related");
        assert!(result.is_ok());

        let related = db.get_related_issues(id1).unwrap();
        assert_eq!(related.len(), 1);
    }

    #[test]
    fn test_remove_relation() {
        let (db, _dir) = setup_test_db();
        let id1 = db.create_issue("Issue 1", None, "medium").unwrap();
        let id2 = db.create_issue("Issue 2", None, "medium").unwrap();

        add_typed(&db, &id1, &id2, "related").unwrap();
        let result = remove_typed(&db, &id1, &id2, "related");
        assert!(result.is_ok());

        let related = db.get_related_issues(id1).unwrap();
        assert_eq!(related.len(), 0);
    }

    #[test]
    fn test_remove_typed_relation() {
        let (db, _dir) = setup_test_db();
        let id1 = db.create_issue("Issue 1", None, "medium").unwrap();
        let id2 = db.create_issue("Issue 2", None, "medium").unwrap();

        add_typed(&db, &id1, &id2, "assumption").unwrap();
        add_typed(&db, &id1, &id2, "related").unwrap();

        remove_typed(&db, &id1, &id2, "assumption").unwrap();

        // "related" should still exist
        let relations = db.get_typed_relations(id1).unwrap();
        assert_eq!(relations.len(), 1);
        assert_eq!(relations[0].relation_type, "related");
    }

    #[test]
    fn test_remove_nonexistent_relation() {
        let (db, _dir) = setup_test_db();
        let id1 = db.create_issue("Issue 1", None, "medium").unwrap();
        let id2 = db.create_issue("Issue 2", None, "medium").unwrap();

        let result = remove_typed(&db, &id1, &id2, "related");
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_relations() {
        let (db, _dir) = setup_test_db();
        let id1 = db.create_issue("Issue 1", None, "medium").unwrap();
        let id2 = db.create_issue("Issue 2", None, "medium").unwrap();
        let id3 = db.create_issue("Issue 3", None, "medium").unwrap();

        add_typed(&db, &id1, &id2, "related").unwrap();
        add_typed(&db, &id1, &id3, "related").unwrap();

        let result = list(&db, &id1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_relations_nonexistent() {
        let (db, _dir) = setup_test_db();

        let result = list(&db, "atelier-missing");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_no_relations() {
        let (db, _dir) = setup_test_db();
        let id = db.create_issue("Lonely issue", None, "medium").unwrap();

        let result = list(&db, &id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_downstream_impact_parent_child() {
        let (db, _dir) = setup_test_db();
        let root = db.create_issue("Root assumption", None, "high").unwrap();
        let child1 = db.create_subissue(&root, "Why 1", None, "medium").unwrap();
        let child2 = db.create_subissue(&root, "Why 2", None, "medium").unwrap();
        let grandchild = db
            .create_subissue(&child1, "Why 1.1", None, "medium")
            .unwrap();

        let affected = db.downstream_impact(root).unwrap();
        let affected_ids: Vec<String> = affected.iter().map(|i| i.id.clone()).collect();

        assert!(affected_ids.contains(&child1));
        assert!(affected_ids.contains(&child2));
        assert!(affected_ids.contains(&grandchild));
        assert_eq!(affected.len(), 3);
    }

    #[test]
    fn test_downstream_impact_derived_relations() {
        let (db, _dir) = setup_test_db();
        let assumption = db.create_issue("Core assumption", None, "high").unwrap();
        let conclusion = db
            .create_issue("Conclusion built on assumption", None, "medium")
            .unwrap();

        db.add_typed_relation(&assumption, &conclusion, "derived")
            .unwrap();

        let affected = db.downstream_impact(assumption).unwrap();
        assert_eq!(affected.len(), 1);
        assert_eq!(affected[0].id, conclusion);
    }

    #[test]
    fn test_downstream_impact_named_impact_relations() {
        let (db, _dir) = setup_test_db();
        let source = db.create_issue("Source", None, "high").unwrap();
        let caused = db.create_issue("Caused work", None, "medium").unwrap();
        let falsified = db.create_issue("Falsified work", None, "medium").unwrap();

        db.add_typed_relation(&source, &caused, "caused-by")
            .unwrap();
        db.add_typed_relation(&source, &falsified, "falsifies")
            .unwrap();

        let affected = db.downstream_impact(source).unwrap();
        let affected_ids: Vec<String> = affected.iter().map(|i| i.id.clone()).collect();

        assert!(affected_ids.contains(&caused));
        assert!(affected_ids.contains(&falsified));
    }

    #[test]
    fn test_downstream_impact_assumption_one_hop() {
        let (db, _dir) = setup_test_db();
        let a1 = db.create_issue("Assumption A", None, "high").unwrap();
        let a2 = db
            .create_issue("Assumption B (shared)", None, "medium")
            .unwrap();
        let a3 = db
            .create_issue("Assumption C (shared with B)", None, "medium")
            .unwrap();

        db.add_typed_relation(&a1, &a2, "assumption").unwrap();
        db.add_typed_relation(&a2, &a3, "assumption").unwrap();

        // Impact from a1 should flag a2 one hop, but not a3 two hops away.
        let affected = db.downstream_impact(a1).unwrap();
        let affected_ids: Vec<String> = affected.iter().map(|i| i.id.clone()).collect();

        assert!(affected_ids.contains(&a2));
        assert!(!affected_ids.contains(&a3));
    }
}
