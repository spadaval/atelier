use anyhow::{bail, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::db::Database;
use crate::record_id;
use crate::utils::format_issue_id;
use atelier_core::Issue;

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
    id_mapping: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
struct LossyField {
    source_id: String,
    field: String,
    handling: String,
}

pub fn run_beads_jsonl(db: &Database, input_path: &Path, state_dir: &Path) -> Result<()> {
    let report = import_beads_jsonl(db, input_path)?;
    super::export::run_canonical(db, state_dir, false)?;

    println!("Imported Beads backup from {}", input_path.display());
    println!("  source records: {}", report.source_records);
    println!("  imported issues: {}", report.imported_issues);
    println!(
        "  parent-child relationships: {}",
        report.parent_child_links
    );
    println!("  blocking relationships: {}", report.blocking_links);
    println!("  skipped records: {}", report.skipped_records);
    println!("  lossy/deferred fields: {}", report.lossy_fields.len());
    println!("  canonical state: {}", state_dir.display());
    if !report.lossy_fields.is_empty() {
        println!("\nLossy/deferred field report:");
        for field in &report.lossy_fields {
            println!("  {} {}: {}", field.source_id, field.field, field.handling);
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
            if db.get_issue(&id)?.is_some() {
                bail!(
                    "Import target {} for Beads ID {} already exists",
                    format_issue_id(&id),
                    record.id
                );
            }

            let issue = imported_issue(record, id.clone(), &mut lossy_fields)?;
            db.insert_issue_import(&issue)?;
            for label in record.labels.as_deref().unwrap_or_default() {
                db.add_label(&id, label)?;
            }
            add_preservation_comments(db, record, &id, &mut lossy_fields)?;
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
                        db.update_parent_import(&issue_id, Some(&depends_on_id), &updated_at)?;
                        parent_edges.insert((issue_id.clone(), depends_on_id.clone()));
                    }
                    "blocks" => {
                        db.add_dependency(&issue_id, &depends_on_id)?;
                        block_edges.insert((issue_id.clone(), depends_on_id.clone()));
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

fn deterministic_id_mapping(records: &[BeadsIssue]) -> Result<BTreeMap<String, String>> {
    let mut ids = BTreeSet::new();
    for record in records {
        if !ids.insert(record.id.clone()) {
            bail!("Duplicate Beads issue ID {}", record.id);
        }
    }
    Ok(ids
        .into_iter()
        .enumerate()
        .map(|(index, id)| (id, record_id::legacy_issue_id(index as i64 + 1)))
        .collect())
}

fn mapped_id(mapping: &BTreeMap<String, String>, source_id: &str) -> Result<String> {
    mapping
        .get(source_id)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Dependency references missing Beads ID {}", source_id))
}

fn imported_issue(
    record: &BeadsIssue,
    id: String,
    lossy_fields: &mut Vec<LossyField>,
) -> Result<Issue> {
    let status = match record.status.as_str() {
        "open" => "todo",
        "closed" => "done",
        "archived" => "archived",
        "in_progress" => {
            lossy_fields.push(lossy(
                &record.id,
                "status",
                "mapped in_progress to todo; assignee/start metadata reported as lossy fields",
            ));
            "todo"
        }
        other => {
            lossy_fields.push(lossy(
                &record.id,
                "status",
                format!("mapped unsupported status '{other}' to todo"),
            ));
            "todo"
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
    let description = record
        .description
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("Imported Beads issue did not include a description.");
    let outcome = record
        .acceptance_criteria
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("Imported Beads issue did not include acceptance criteria.");
    format!(
        "## Description\n\n{description}\n\n## Outcome\n\n{outcome}\n\n## Evidence\n\n- `atelier import-beads <path>` imports this record and `atelier export --check` validates canonical Markdown."
    )
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
    id: &str,
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
    id_mapping: &BTreeMap<String, String>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_records::{parse_issue_sections, IssueSectionName};
    use tempfile::tempdir;

    fn setup_test_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::open(&db_path).unwrap();
        (db, dir)
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
        assert_eq!(
            report.id_mapping["atelier-z1p"],
            record_id::legacy_issue_id(1)
        );
        assert_eq!(
            report.id_mapping["atelier-z1p.2"],
            record_id::legacy_issue_id(2)
        );
        assert_eq!(
            report.id_mapping["atelier-z1p.4"],
            record_id::legacy_issue_id(3)
        );

        let imported = db.list_issues(Some("all"), None, None).unwrap();
        assert_eq!(imported.len(), 3);
        assert_eq!(
            db.get_issue(record_id::legacy_issue_id(2))
                .unwrap()
                .unwrap()
                .parent_id,
            Some(record_id::legacy_issue_id(1))
        );
        assert_eq!(
            db.get_issue(record_id::legacy_issue_id(3))
                .unwrap()
                .unwrap()
                .parent_id,
            Some(record_id::legacy_issue_id(1))
        );
        assert_eq!(
            db.get_blockers(record_id::legacy_issue_id(3)).unwrap(),
            vec![record_id::legacy_issue_id(2)]
        );
        assert_eq!(
            db.get_blocking(record_id::legacy_issue_id(2)).unwrap(),
            vec![record_id::legacy_issue_id(3)]
        );

        assert_eq!(
            db.get_issue(record_id::legacy_issue_id(1))
                .unwrap()
                .unwrap()
                .issue_type,
            "epic"
        );
        assert_eq!(
            db.get_issue(record_id::legacy_issue_id(2))
                .unwrap()
                .unwrap()
                .issue_type,
            "feature"
        );
        assert_eq!(
            db.get_issue(record_id::legacy_issue_id(3))
                .unwrap()
                .unwrap()
                .issue_type,
            "task"
        );
        let labels = db.get_labels(record_id::legacy_issue_id(3)).unwrap();
        assert!(!labels.contains(&"task".to_string()));
        assert!(!labels.iter().any(|label| label.starts_with("beads:")));
        assert!(!db
            .get_issue(record_id::legacy_issue_id(3))
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

        super::super::agent_factory::show(&db, &record_id::legacy_issue_id(2)).unwrap();
        assert!(db
            .update_issue(
                record_id::legacy_issue_id(2),
                Some("Imported record updated"),
                None,
                Some("critical")
            )
            .unwrap());
        assert!(db.close_issue(record_id::legacy_issue_id(2)).unwrap());

        let issue = db
            .get_issue(record_id::legacy_issue_id(2))
            .unwrap()
            .unwrap();
        assert_eq!(issue.title, "Imported record updated");
        assert_eq!(issue.priority, "critical");
        assert_eq!(issue.status, "done");
    }

    #[test]
    fn test_imported_beads_description_uses_current_issue_sections() {
        let record = BeadsIssue {
            record_type: "issue".to_string(),
            id: "atelier-source".to_string(),
            title: "Imported source".to_string(),
            description: Some("Source description.".to_string()),
            acceptance_criteria: Some("Source acceptance criteria.".to_string()),
            status: "open".to_string(),
            priority: 1,
            issue_type: Some("task".to_string()),
            owner: None,
            assignee: None,
            created_at: None,
            created_by: None,
            updated_at: None,
            started_at: None,
            closed_at: None,
            close_reason: None,
            notes: None,
            labels: None,
            dependencies: None,
            dependency_count: None,
            dependent_count: None,
            comment_count: None,
            extra: BTreeMap::new(),
        };

        let body = imported_description(&record);
        let sections = parse_issue_sections(&body, Path::new("imported.md")).unwrap();

        assert_eq!(
            sections.section(IssueSectionName::Description),
            Some("Source description.")
        );
        assert_eq!(
            sections.section(IssueSectionName::Outcome),
            Some("Source acceptance criteria.")
        );
        assert!(sections
            .section(IssueSectionName::Evidence)
            .unwrap()
            .contains("atelier import-beads <path>"));
        assert!(!body.contains("Acceptance Criteria"));
    }

    #[test]
    fn test_import_beads_writes_canonical_state() {
        let (db, dir) = setup_test_db();
        let import_path = dir.path().join("issues.manual.jsonl");
        let state_dir = dir.path().join(".atelier");
        fs::write(
            &import_path,
            include_str!("../../tests/fixtures/beads/issues.manual.jsonl"),
        )
        .unwrap();

        run_beads_jsonl(&db, &import_path, &state_dir).unwrap();

        assert!(!state_dir.join("manifest.json").exists());
        assert!(!state_dir.join("graph.json").exists());
        assert!(state_dir
            .join("issues")
            .join(format!("{}.md", record_id::legacy_issue_id(1)))
            .exists());
    }
}
