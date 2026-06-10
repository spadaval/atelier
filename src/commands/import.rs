use anyhow::{bail, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::Path;

use super::export::{ExportData, ExportedIssue};
use crate::db::Database;
use crate::models::Issue;
use crate::utils::format_issue_id;

pub fn run_json(db: &Database, input_path: &Path) -> Result<()> {
    let content = fs::read_to_string(input_path).context("Failed to read import file")?;

    let data: ExportData = serde_json::from_str(&content).context("Failed to parse JSON")?;

    println!(
        "Importing {} issues from {}",
        data.issues.len(),
        input_path.display()
    );

    // Wrap entire import in a transaction for atomicity
    // If any part fails, all changes are rolled back
    let count = db.transaction(|| {
        // Map old IDs to new IDs for parent relationships
        let mut id_map: HashMap<i64, i64> = HashMap::new();

        // First pass: create all issues without parent relationships
        for issue in &data.issues {
            let new_id = import_issue(db, issue, None)?;
            id_map.insert(issue.id, new_id);
        }

        // Second pass: update parent relationships
        for issue in &data.issues {
            if let Some(old_parent_id) = issue.parent_id {
                if let Some(&new_parent_id) = id_map.get(&old_parent_id) {
                    if let Some(&new_id) = id_map.get(&issue.id) {
                        // Update parent_id for this issue
                        db.update_parent(new_id, Some(new_parent_id))?;
                    }
                }
            }
        }

        Ok(data.issues.len())
    })?;

    println!("Successfully imported {} issues", count);
    Ok(())
}

#[derive(Debug, Deserialize)]
struct BeadsIssue {
    #[serde(rename = "_type")]
    record_type: String,
    id: String,
    title: String,
    description: Option<String>,
    acceptance_criteria: Option<String>,
    status: String,
    priority: i64,
    issue_type: Option<String>,
    owner: Option<String>,
    assignee: Option<String>,
    created_at: Option<String>,
    created_by: Option<String>,
    updated_at: Option<String>,
    started_at: Option<String>,
    closed_at: Option<String>,
    close_reason: Option<String>,
    notes: Option<String>,
    labels: Option<Vec<String>>,
    dependencies: Option<Vec<BeadsDependency>>,
    dependency_count: Option<i64>,
    dependent_count: Option<i64>,
    comment_count: Option<i64>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct BeadsDependency {
    issue_id: String,
    depends_on_id: String,
    #[serde(rename = "type")]
    dependency_type: String,
    created_at: Option<String>,
    created_by: Option<String>,
    metadata: Option<String>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BeadsImportReport {
    source_path: String,
    source_records: usize,
    imported_issues: usize,
    parent_child_links: usize,
    blocking_links: usize,
    skipped_records: usize,
    lossy_fields: Vec<LossyField>,
    id_mapping: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, Serialize)]
struct LossyField {
    source_id: String,
    field: String,
    handling: String,
}

pub fn run_beads_jsonl(
    db: &Database,
    input_path: &Path,
    state_dir: &Path,
    json: bool,
) -> Result<()> {
    let report = import_beads_jsonl(db, input_path)?;
    super::export::run_canonical(db, state_dir, false)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("Imported Beads backup from {}", input_path.display());
        println!("  source records: {}", report.source_records);
        println!("  imported issues: {}", report.imported_issues);
        println!("  parent-child links: {}", report.parent_child_links);
        println!("  blocking links: {}", report.blocking_links);
        println!("  skipped records: {}", report.skipped_records);
        println!("  lossy/deferred fields: {}", report.lossy_fields.len());
        println!("  canonical state: {}", state_dir.display());
        if !report.lossy_fields.is_empty() {
            println!("\nLossy/deferred field report:");
            for field in &report.lossy_fields {
                println!("  {} {}: {}", field.source_id, field.field, field.handling);
            }
        }
    }

    Ok(())
}

fn import_beads_jsonl(db: &Database, input_path: &Path) -> Result<BeadsImportReport> {
    let content = fs::read_to_string(input_path).context("Failed to read Beads import file")?;
    let mut source_records = Vec::new();
    let mut skipped_records = 0;

    for (line_number, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let record: BeadsIssue = serde_json::from_str(line)
            .with_context(|| format!("Failed to parse Beads JSONL line {}", line_number + 1))?;
        if record.record_type == "issue" {
            source_records.push(record);
        } else {
            skipped_records += 1;
        }
    }

    let id_mapping = deterministic_id_mapping(&source_records)?;
    let mut parent_edges = BTreeSet::new();
    let mut block_edges = BTreeSet::new();
    let mut lossy_fields = Vec::new();

    db.transaction(|| {
        for record in &source_records {
            let id = mapped_id(&id_mapping, &record.id)?;
            if db.get_issue(id)?.is_some() {
                bail!(
                    "Import target {} for Beads ID {} already exists",
                    format_issue_id(id),
                    record.id
                );
            }

            let issue = imported_issue(record, id, &mut lossy_fields)?;
            db.insert_issue_import(&issue)?;
            for label in record.labels.as_deref().unwrap_or_default() {
                db.add_label(id, label)?;
            }
            add_preservation_comments(db, record, id, &mut lossy_fields)?;
            report_extra_fields(record, &mut lossy_fields);
        }

        for record in &source_records {
            let issue_id = mapped_id(&id_mapping, &record.id)?;
            for dependency in record.dependencies.as_deref().unwrap_or_default() {
                validate_dependency_record(record, dependency, &id_mapping, &mut lossy_fields)?;
                let depends_on_id = mapped_id(&id_mapping, &dependency.depends_on_id)?;
                match dependency.dependency_type.as_str() {
                    "parent-child" => {
                        let updated_at = parse_optional_datetime(record.updated_at.as_deref())?
                            .unwrap_or_else(Utc::now);
                        db.update_parent_import(issue_id, Some(depends_on_id), &updated_at)?;
                        parent_edges.insert((issue_id, depends_on_id));
                    }
                    "blocks" => {
                        db.add_dependency(issue_id, depends_on_id)?;
                        block_edges.insert((issue_id, depends_on_id));
                    }
                    other => lossy_fields.push(lossy(
                        &record.id,
                        "dependencies.type",
                        format!("unsupported dependency type '{other}' was not imported"),
                    )),
                }
            }
        }

        Ok(())
    })?;

    Ok(BeadsImportReport {
        source_path: input_path.display().to_string(),
        source_records: source_records.len(),
        imported_issues: source_records.len(),
        parent_child_links: parent_edges.len(),
        blocking_links: block_edges.len(),
        skipped_records,
        lossy_fields,
        id_mapping,
    })
}

fn deterministic_id_mapping(records: &[BeadsIssue]) -> Result<BTreeMap<String, i64>> {
    let mut ids = BTreeSet::new();
    for record in records {
        if !ids.insert(record.id.clone()) {
            bail!("Duplicate Beads issue ID {}", record.id);
        }
    }
    Ok(ids
        .into_iter()
        .enumerate()
        .map(|(index, id)| (id, index as i64 + 1))
        .collect())
}

fn mapped_id(mapping: &BTreeMap<String, i64>, source_id: &str) -> Result<i64> {
    mapping
        .get(source_id)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("Dependency references missing Beads ID {}", source_id))
}

fn imported_issue(
    record: &BeadsIssue,
    id: i64,
    lossy_fields: &mut Vec<LossyField>,
) -> Result<Issue> {
    let status = match record.status.as_str() {
        "open" => "open",
        "closed" => "closed",
        "archived" => "archived",
        "in_progress" => {
            lossy_fields.push(lossy(
                &record.id,
                "status",
                "mapped in_progress to open; assignee/start metadata reported as lossy fields",
            ));
            "open"
        }
        other => {
            lossy_fields.push(lossy(
                &record.id,
                "status",
                format!("mapped unsupported status '{other}' to open"),
            ));
            "open"
        }
    }
    .to_string();

    let updated_at = parse_optional_datetime(record.updated_at.as_deref())?
        .or(parse_optional_datetime(record.created_at.as_deref())?)
        .unwrap_or_else(Utc::now);
    let closed_at = parse_optional_datetime(record.closed_at.as_deref())?;
    let created_at = parse_optional_datetime(record.created_at.as_deref())?.unwrap_or(updated_at);

    Ok(Issue {
        id,
        title: record.title.clone(),
        description: Some(imported_description(record)),
        status,
        issue_type: record
            .issue_type
            .clone()
            .unwrap_or_else(|| "task".to_string()),
        priority: import_priority(record.priority, &record.id, lossy_fields),
        parent_id: None,
        created_at,
        updated_at,
        closed_at,
    })
}

fn imported_description(record: &BeadsIssue) -> String {
    let mut sections = Vec::new();
    if let Some(description) = record.description.as_deref() {
        if !description.trim().is_empty() {
            sections.push(description.trim().to_string());
        }
    }
    if let Some(acceptance) = record.acceptance_criteria.as_deref() {
        if !acceptance.trim().is_empty() {
            sections.push(format!("## Acceptance Criteria\n\n{}", acceptance.trim()));
        }
    }
    sections.join("\n\n")
}

fn import_priority(priority: i64, source_id: &str, lossy_fields: &mut Vec<LossyField>) -> String {
    match priority {
        0 => "critical",
        1 => "high",
        2 => "medium",
        3 | 4 => "low",
        other => {
            lossy_fields.push(lossy(
                source_id,
                "priority",
                format!("mapped unsupported numeric priority {other} to medium"),
            ));
            "medium"
        }
    }
    .to_string()
}

fn add_preservation_comments(
    db: &Database,
    record: &BeadsIssue,
    id: i64,
    lossy_fields: &mut Vec<LossyField>,
) -> Result<()> {
    let created_at = record
        .updated_at
        .as_deref()
        .or(record.created_at.as_deref())
        .unwrap_or("1970-01-01T00:00:00Z");
    if let Some(notes) = record.notes.as_deref() {
        if !notes.trim().is_empty() {
            db.add_comment_at(id, notes.trim(), "note", created_at)?;
        }
    }
    if let Some(reason) = record.close_reason.as_deref() {
        if !reason.trim().is_empty() {
            db.add_comment_at(id, reason.trim(), "close-reason", created_at)?;
        }
    }

    for (field, value) in [
        ("owner", record.owner.as_deref()),
        ("assignee", record.assignee.as_deref()),
        ("created_by", record.created_by.as_deref()),
        ("started_at", record.started_at.as_deref()),
        ("close_reason", record.close_reason.as_deref()),
    ] {
        if value.is_some() {
            lossy_fields.push(lossy(
                &record.id,
                field,
                "reported in import summary; not written to Atelier issue state",
            ));
        }
    }

    for (field, value) in [
        ("dependency_count", record.dependency_count),
        ("dependent_count", record.dependent_count),
        ("comment_count", record.comment_count),
    ] {
        if value.is_some() {
            lossy_fields.push(lossy(
                &record.id,
                field,
                "source aggregate count is recomputed by Atelier and not imported",
            ));
        }
    }
    Ok(())
}

fn validate_dependency_record(
    record: &BeadsIssue,
    dependency: &BeadsDependency,
    id_mapping: &BTreeMap<String, i64>,
    lossy_fields: &mut Vec<LossyField>,
) -> Result<()> {
    if dependency.issue_id != record.id {
        bail!(
            "Dependency on {} is attached to source record {}",
            dependency.issue_id,
            record.id
        );
    }
    mapped_id(id_mapping, &dependency.depends_on_id)?;
    for (field, value) in [
        ("dependencies.created_at", dependency.created_at.as_deref()),
        ("dependencies.created_by", dependency.created_by.as_deref()),
        ("dependencies.metadata", dependency.metadata.as_deref()),
    ] {
        if value.is_some() {
            lossy_fields.push(lossy(
                &record.id,
                field,
                "dependency edge metadata is not represented in current Atelier dependency schema",
            ));
        }
    }
    for key in dependency.extra.keys() {
        lossy_fields.push(lossy(
            &record.id,
            format!("dependencies.{key}"),
            "unknown dependency field was not imported",
        ));
    }
    Ok(())
}

fn report_extra_fields(record: &BeadsIssue, lossy_fields: &mut Vec<LossyField>) {
    for key in record.extra.keys() {
        lossy_fields.push(lossy(
            &record.id,
            key,
            "unknown source field was not imported",
        ));
    }
}

fn parse_optional_datetime(value: Option<&str>) -> Result<Option<DateTime<Utc>>> {
    value
        .filter(|value| !value.trim().is_empty())
        .map(|value| {
            DateTime::parse_from_rfc3339(value)
                .map(|dt| dt.with_timezone(&Utc))
                .with_context(|| format!("Invalid timestamp '{value}' in Beads import"))
        })
        .transpose()
}

fn lossy(source_id: &str, field: impl Into<String>, handling: impl Into<String>) -> LossyField {
    LossyField {
        source_id: source_id.to_string(),
        field: field.into(),
        handling: handling.into(),
    }
}

fn import_issue(db: &Database, issue: &ExportedIssue, parent_id: Option<i64>) -> Result<i64> {
    let id = if let Some(pid) = parent_id {
        db.create_subissue_with_type(
            pid,
            &issue.title,
            issue.description.as_deref(),
            &issue.priority,
            &issue.issue_type,
        )?
    } else {
        db.create_issue_with_type(
            &issue.title,
            issue.description.as_deref(),
            &issue.priority,
            &issue.issue_type,
        )?
    };

    // Add labels
    for label in &issue.labels {
        db.add_label(id, label)?;
    }

    // Add comments
    for comment in &issue.comments {
        db.add_comment(id, &comment.content, &comment.kind)?;
    }

    // Close if needed
    if issue.status == "closed" {
        db.close_issue(id)?;
    }

    println!(
        "  Imported: {} -> {} {}",
        format_issue_id(issue.id),
        format_issue_id(id),
        issue.title
    );
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::super::export::{ExportData, ExportedIssue};
    use super::*;
    use proptest::prelude::*;
    use tempfile::tempdir;

    fn setup_test_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::open(&db_path).unwrap();
        (db, dir)
    }

    fn create_test_export(issues: Vec<ExportedIssue>) -> String {
        let data = ExportData {
            version: 1,
            exported_at: "2024-01-01T00:00:00Z".to_string(),
            issues,
        };
        serde_json::to_string_pretty(&data).unwrap()
    }

    fn make_issue(id: i64, title: &str, parent_id: Option<i64>, status: &str) -> ExportedIssue {
        ExportedIssue {
            id,
            title: title.to_string(),
            description: None,
            status: status.to_string(),
            issue_type: "task".to_string(),
            priority: "medium".to_string(),
            parent_id,
            labels: vec![],
            comments: vec![],
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            closed_at: None,
        }
    }

    #[test]
    fn test_import_single_issue() {
        let (db, dir) = setup_test_db();
        let json = create_test_export(vec![make_issue(1, "Test issue", None, "open")]);
        let import_path = dir.path().join("import.json");
        fs::write(&import_path, json).unwrap();
        let result = run_json(&db, &import_path);
        assert!(result.is_ok());
        let issues = db.list_issues(Some("all"), None, None).unwrap();
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_import_multiple_issues() {
        let (db, dir) = setup_test_db();
        let json = create_test_export(vec![
            make_issue(1, "Issue 1", None, "open"),
            make_issue(2, "Issue 2", None, "open"),
        ]);
        let import_path = dir.path().join("import.json");
        fs::write(&import_path, json).unwrap();
        run_json(&db, &import_path).unwrap();
        let issues = db.list_issues(Some("all"), None, None).unwrap();
        assert_eq!(issues.len(), 2);
    }

    #[test]
    fn test_import_closed_issue() {
        let (db, dir) = setup_test_db();
        let json = create_test_export(vec![make_issue(1, "Closed", None, "closed")]);
        let import_path = dir.path().join("import.json");
        fs::write(&import_path, json).unwrap();
        run_json(&db, &import_path).unwrap();
        let issues = db.list_issues(Some("closed"), None, None).unwrap();
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_import_with_labels() {
        let (db, dir) = setup_test_db();
        let mut issue = make_issue(1, "Labeled", None, "open");
        issue.labels = vec!["bug".to_string()];
        let json = create_test_export(vec![issue]);
        let import_path = dir.path().join("import.json");
        fs::write(&import_path, json).unwrap();
        run_json(&db, &import_path).unwrap();
        let issues = db.list_issues(Some("all"), None, None).unwrap();
        let labels = db.get_labels(issues[0].id).unwrap();
        assert!(labels.contains(&"bug".to_string()));
        assert_eq!(issues[0].issue_type, "task");
    }

    #[test]
    fn test_import_preserves_issue_type_separate_from_labels() {
        let (db, dir) = setup_test_db();
        let mut issue = make_issue(1, "Typed", None, "open");
        issue.issue_type = "validation".to_string();
        issue.labels = vec!["epic".to_string()];
        let json = create_test_export(vec![issue]);
        let import_path = dir.path().join("import.json");
        fs::write(&import_path, json).unwrap();

        run_json(&db, &import_path).unwrap();

        let issue = db.get_issue(1).unwrap().unwrap();
        assert_eq!(issue.issue_type, "validation");
        assert_eq!(db.get_labels(issue.id).unwrap(), vec!["epic".to_string()]);
    }

    #[test]
    fn test_import_invalid_json() {
        let (db, dir) = setup_test_db();
        let import_path = dir.path().join("invalid.json");
        fs::write(&import_path, "not valid json").unwrap();
        let result = run_json(&db, &import_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_import_missing_file() {
        let (db, dir) = setup_test_db();
        let import_path = dir.path().join("nonexistent.json");
        let result = run_json(&db, &import_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_import_empty_issues() {
        let (db, dir) = setup_test_db();
        let json = create_test_export(vec![]);
        let import_path = dir.path().join("import.json");
        fs::write(&import_path, json).unwrap();
        let result = run_json(&db, &import_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_import_beads_fixture_preserves_counts_and_links() {
        let (db, dir) = setup_test_db();
        let import_path = dir.path().join("issues.manual.jsonl");
        fs::write(
            &import_path,
            include_str!("../../tests/fixtures/beads/issues.manual.jsonl"),
        )
        .unwrap();

        let report = import_beads_jsonl(&db, &import_path).unwrap();

        assert_eq!(report.source_records, 3);
        assert_eq!(report.imported_issues, 3);
        assert_eq!(report.parent_child_links, 2);
        assert_eq!(report.blocking_links, 1);
        assert_eq!(report.id_mapping["atelier-z1p"], 1);
        assert_eq!(report.id_mapping["atelier-z1p.2"], 2);
        assert_eq!(report.id_mapping["atelier-z1p.4"], 3);

        let imported = db.list_issues(Some("all"), None, None).unwrap();
        assert_eq!(imported.len(), 3);
        assert_eq!(db.get_issue(2).unwrap().unwrap().parent_id, Some(1));
        assert_eq!(db.get_issue(3).unwrap().unwrap().parent_id, Some(1));
        assert_eq!(db.get_blockers(3).unwrap(), vec![2]);
        assert_eq!(db.get_blocking(2).unwrap(), vec![3]);

        assert_eq!(db.get_issue(1).unwrap().unwrap().issue_type, "epic");
        assert_eq!(db.get_issue(2).unwrap().unwrap().issue_type, "feature");
        assert_eq!(db.get_issue(3).unwrap().unwrap().issue_type, "task");
        let labels = db.get_labels(3).unwrap();
        assert!(!labels.contains(&"task".to_string()));
        assert!(!labels.iter().any(|label| label.starts_with("beads:")));
        assert!(!db
            .get_issue(3)
            .unwrap()
            .unwrap()
            .description
            .unwrap()
            .contains("Beads Source"));
    }

    #[test]
    fn test_imported_beads_records_can_be_shown_updated_and_closed() {
        let (db, dir) = setup_test_db();
        let import_path = dir.path().join("issues.manual.jsonl");
        fs::write(
            &import_path,
            include_str!("../../tests/fixtures/beads/issues.manual.jsonl"),
        )
        .unwrap();
        import_beads_jsonl(&db, &import_path).unwrap();

        super::super::show::run(&db, 2).unwrap();
        assert!(db
            .update_issue(2, Some("Imported record updated"), None, Some("critical"))
            .unwrap());
        assert!(db.close_issue(2).unwrap());

        let issue = db.get_issue(2).unwrap().unwrap();
        assert_eq!(issue.title, "Imported record updated");
        assert_eq!(issue.priority, "critical");
        assert_eq!(issue.status, "closed");
    }

    #[test]
    fn test_import_beads_writes_canonical_state() {
        let (db, dir) = setup_test_db();
        let import_path = dir.path().join("issues.manual.jsonl");
        let state_dir = dir.path().join(".atelier-state");
        fs::write(
            &import_path,
            include_str!("../../tests/fixtures/beads/issues.manual.jsonl"),
        )
        .unwrap();

        run_beads_jsonl(&db, &import_path, &state_dir, true).unwrap();

        assert!(!state_dir.join("manifest.json").exists());
        assert!(!state_dir.join("graph.json").exists());
        assert!(state_dir.join("issues").join("ISS-0001.md").exists());
    }

    proptest! {
        #[test]
        fn prop_import_never_panics(title in "[a-zA-Z0-9 ]{1,50}") {
            let (db, dir) = setup_test_db();
            let json = create_test_export(vec![make_issue(1, &title, None, "open")]);
            let import_path = dir.path().join("import.json");
            fs::write(&import_path, json).unwrap();
            let result = run_json(&db, &import_path);
            prop_assert!(result.is_ok());
        }
    }
}
