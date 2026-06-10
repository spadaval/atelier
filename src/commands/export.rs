use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::db::Database;
use crate::models::{DomainRecord, Issue};

#[derive(Serialize, Deserialize)]
pub struct ExportedIssue {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    #[serde(default = "default_issue_type")]
    pub issue_type: String,
    pub priority: String,
    pub parent_id: Option<String>,
    pub labels: Vec<String>,
    pub comments: Vec<ExportedComment>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ExportedComment {
    pub content: String,
    pub created_at: String,
    #[serde(default = "default_comment_kind")]
    pub kind: String,
}

fn default_comment_kind() -> String {
    "note".to_string()
}

fn default_issue_type() -> String {
    "task".to_string()
}

#[derive(Serialize, Deserialize)]
pub struct ExportData {
    pub version: i32,
    pub exported_at: String,
    pub issues: Vec<ExportedIssue>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ProjectionFile {
    path: PathBuf,
    bytes: Vec<u8>,
}

fn export_issue(db: &Database, issue: &Issue) -> Result<ExportedIssue> {
    let labels = db.get_labels(&issue.id)?;
    let comments = db.get_comments(&issue.id)?;

    Ok(ExportedIssue {
        id: issue.id.clone(),
        title: issue.title.clone(),
        description: issue.description.clone(),
        status: issue.status.clone(),
        issue_type: issue.issue_type.clone(),
        priority: issue.priority.clone(),
        parent_id: issue.parent_id.clone(),
        labels,
        comments: comments
            .into_iter()
            .map(|c| ExportedComment {
                content: c.content,
                created_at: c.created_at.to_rfc3339(),
                kind: c.kind,
            })
            .collect(),
        created_at: issue.created_at.to_rfc3339(),
        updated_at: issue.updated_at.to_rfc3339(),
        closed_at: issue.closed_at.map(|dt| dt.to_rfc3339()),
    })
}

pub fn run_json(db: &Database, output_path: Option<&str>) -> Result<()> {
    let issues = db.list_issues(Some("all"), None, None)?;

    let exported: Vec<ExportedIssue> = issues
        .iter()
        .map(|i| export_issue(db, i))
        .collect::<Result<Vec<_>>>()?;

    let data = ExportData {
        version: 1,
        exported_at: chrono::Utc::now().to_rfc3339(),
        issues: exported,
    };

    let json = serde_json::to_string_pretty(&data)?;

    match output_path {
        Some(path) => {
            fs::write(path, json).context("Failed to write export file")?;
            eprintln!("Exported {} issues to {}", data.issues.len(), path);
        }
        None => {
            let mut stdout = io::stdout().lock();
            writeln!(stdout, "{}", json)?;
        }
    }
    Ok(())
}

pub fn run_canonical(db: &Database, state_dir: &Path, check: bool) -> Result<()> {
    let files = build_canonical_projection(db, state_dir)?;

    if check {
        let stale = stale_projection_entries(state_dir, &files)?;
        if stale.is_empty() {
            eprintln!("Canonical export is current");
            return Ok(());
        }

        bail!("Canonical export is stale:\n{}", stale.join("\n"));
    }

    write_canonical_projection(state_dir, &files)?;
    eprintln!(
        "Exported canonical state to {}",
        state_dir.to_string_lossy()
    );
    Ok(())
}

pub fn canonical_stale_entries(db: &Database, state_dir: &Path) -> Result<Vec<String>> {
    let files = build_canonical_projection(db, state_dir)?;
    stale_projection_entries(state_dir, &files)
}

fn build_canonical_projection(db: &Database, state_dir: &Path) -> Result<Vec<ProjectionFile>> {
    let mut issues = db.list_issues(Some("all"), None, None)?;
    issues.sort_by(|a, b| a.id.cmp(&b.id));

    let mut files = Vec::new();
    for issue in &issues {
        files.push(ProjectionFile {
            path: issue_record_path(&issue.id),
            bytes: render_issue_record(db, issue)?.into_bytes(),
        });
    }
    for kind in ["mission", "milestone", "plan", "evidence"] {
        for record in db.list_records(kind, None)? {
            files.push(ProjectionFile {
                path: record_path(kind, &record.id),
                bytes: render_domain_record(db, &record)?.into_bytes(),
            });
        }
    }
    preserve_existing_activity_files(state_dir, &mut files)?;

    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(files)
}

fn preserve_existing_activity_files(
    state_dir: &Path,
    files: &mut Vec<ProjectionFile>,
) -> Result<()> {
    if !state_dir.exists() {
        return Ok(());
    }
    for relative in canonical_files_under(state_dir)? {
        if !is_issue_activity_path(&relative) {
            continue;
        }
        files.push(ProjectionFile {
            bytes: fs::read(state_dir.join(&relative)).with_context(|| {
                format!(
                    "Failed to read canonical activity {}",
                    display_state_path(&relative)
                )
            })?,
            path: relative,
        });
    }
    Ok(())
}

fn is_issue_activity_path(relative: &Path) -> bool {
    let mut components = relative.components();
    let Some(std::path::Component::Normal(root)) = components.next() else {
        return false;
    };
    if root != "issues" {
        return false;
    }
    let Some(std::path::Component::Normal(dir)) = components.next() else {
        return false;
    };
    if !dir.to_string_lossy().ends_with(".activity") {
        return false;
    }
    let Some(std::path::Component::Normal(file)) = components.next() else {
        return false;
    };
    components.next().is_none() && file.to_string_lossy().ends_with(".md")
}

fn write_canonical_projection(state_dir: &Path, files: &[ProjectionFile]) -> Result<()> {
    fs::create_dir_all(state_dir).context("Failed to create canonical export directory")?;

    let expected: BTreeSet<PathBuf> = files.iter().map(|file| file.path.clone()).collect();
    remove_stale_canonical_files(state_dir, &expected)?;

    for file in files {
        let path = state_dir.join(&file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create canonical export subdirectory")?;
        }
        fs::write(&path, &file.bytes)
            .with_context(|| format!("Failed to write {}", path.display()))?;
    }

    Ok(())
}

fn stale_projection_entries(state_dir: &Path, files: &[ProjectionFile]) -> Result<Vec<String>> {
    let mut stale = Vec::new();
    let expected: BTreeMap<PathBuf, &[u8]> = files
        .iter()
        .map(|file| (file.path.clone(), file.bytes.as_slice()))
        .collect();

    if state_dir.exists() {
        if let Err(error) = crate::commands::rebuild::validate_canonical_state(state_dir) {
            stale.push(format!("invalid: {error:#}"));
        }
    }

    for (relative_path, expected_bytes) in &expected {
        let actual_path = state_dir.join(relative_path);
        match fs::read(&actual_path) {
            Ok(actual_bytes) if actual_bytes == *expected_bytes => {}
            Ok(_) => stale.push(format!("changed: {}", display_state_path(relative_path))),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                stale.push(format!("missing: {}", display_state_path(relative_path)));
            }
            Err(error) => {
                return Err(error).with_context(|| {
                    format!("Failed to read canonical export {}", actual_path.display())
                })
            }
        }
    }

    if state_dir.exists() {
        for relative_path in canonical_files_under(state_dir)? {
            if !expected.contains_key(&relative_path) {
                stale.push(format!("untracked: {}", display_state_path(&relative_path)));
            }
        }
    }

    stale.sort();
    Ok(stale)
}

fn remove_stale_canonical_files(state_dir: &Path, expected: &BTreeSet<PathBuf>) -> Result<()> {
    if !state_dir.exists() {
        return Ok(());
    }

    for relative_path in canonical_files_under(state_dir)? {
        if !expected.contains(&relative_path) {
            let path = state_dir.join(relative_path);
            fs::remove_file(&path)
                .with_context(|| format!("Failed to remove stale projection {}", path.display()))?;
        }
    }

    Ok(())
}

fn canonical_files_under(state_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_canonical_files(state_dir, state_dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_canonical_files(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_canonical_files(root, &path, files)?;
        } else if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .context("Failed to relativize canonical export path")?;
            files.push(relative.to_path_buf());
        }
    }
    Ok(())
}

fn render_issue_record(db: &Database, issue: &Issue) -> Result<String> {
    let labels = db.get_labels(&issue.id)?;
    let mut blocks = issue_ids(db.get_blocking(&issue.id)?);
    let mut depends_on = issue_ids(db.get_blockers(&issue.id)?);
    let mut links = issue_links(db, &issue.id)?;
    blocks.sort();
    depends_on.sort();
    links.sort();

    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_array(&mut output, "acceptance", &[])?;
    write_yaml_array(&mut output, "blocks", &blocks)?;
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&issue.created_at.to_rfc3339()),
    )?;
    write_yaml_array(&mut output, "depends_on", &depends_on)?;
    write_yaml_array(&mut output, "evidence_required", &[])?;
    write_yaml_scalar(&mut output, "id", Some(&issue.id))?;
    write_yaml_scalar(&mut output, "issue_type", Some(&issue.issue_type))?;
    write_yaml_array(&mut output, "labels", &labels)?;
    write_yaml_links(&mut output, "links", &links)?;
    let parent = issue.parent_id.clone();
    write_yaml_scalar(&mut output, "parent", parent.as_deref())?;
    write_yaml_scalar(
        &mut output,
        "priority",
        Some(&canonical_priority(&issue.priority)),
    )?;
    write_yaml_scalar(&mut output, "schema", Some("atelier.issue"))?;
    output.push_str("schema_version: 1\n");
    write_yaml_scalar(&mut output, "status", Some(&issue.status))?;
    write_yaml_scalar(&mut output, "title", Some(&issue.title))?;
    write_yaml_scalar(
        &mut output,
        "updated_at",
        Some(&issue.updated_at.to_rfc3339()),
    )?;
    output.push_str("---\n\n");
    output.push_str(&normalize_body(issue.description.as_deref().unwrap_or("")));
    output.push('\n');
    Ok(output)
}

fn render_domain_record(db: &Database, record: &DomainRecord) -> Result<String> {
    let mut links = db
        .list_record_links(&record.kind, &record.id)?
        .into_iter()
        .filter(|link| link.source_kind == record.kind && link.source_id == record.id)
        .collect::<Vec<_>>();
    links.sort_by(|a, b| {
        (
            &a.target_kind,
            &a.target_id,
            &a.relation_type,
            &a.created_at,
        )
            .cmp(&(
                &b.target_kind,
                &b.target_id,
                &b.relation_type,
                &b.created_at,
            ))
    });
    let mut output = String::new();
    output.push_str("---\n");
    write_yaml_scalar(
        &mut output,
        "created_at",
        Some(&record.created_at.to_rfc3339()),
    )?;
    write_yaml_scalar(&mut output, "id", Some(&record.id))?;
    write_json_scalar(&mut output, "data", &record.data_json)?;
    write_record_links(&mut output, "links", &links)?;
    write_yaml_scalar(
        &mut output,
        "schema",
        Some(&format!("atelier.{}", record.kind)),
    )?;
    output.push_str("schema_version: 1\n");
    write_yaml_scalar(&mut output, "status", Some(&record.status))?;
    write_yaml_scalar(&mut output, "title", Some(&record.title))?;
    write_yaml_scalar(
        &mut output,
        "updated_at",
        Some(&record.updated_at.to_rfc3339()),
    )?;
    output.push_str("---\n\n");
    output.push_str(&normalize_body(record.body.as_deref().unwrap_or("")));
    output.push('\n');
    Ok(output)
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct IssueLink {
    relation_type: String,
    target_kind: String,
    target_id: String,
}

fn issue_links(db: &Database, issue_id: &str) -> Result<Vec<IssueLink>> {
    let mut links = Vec::new();
    for relation in db.get_typed_relations(issue_id)? {
        if relation.issue_id_1 == issue_id {
            links.push(IssueLink {
                relation_type: relation.relation_type,
                target_kind: "issue".to_string(),
                target_id: relation.issue_id_2,
            });
        }
    }
    Ok(links)
}

fn issue_ids(ids: Vec<String>) -> Vec<String> {
    ids
}

fn issue_record_path(id: &str) -> PathBuf {
    PathBuf::from("issues").join(format!("{}.md", id))
}

fn record_path(kind: &str, id: &str) -> PathBuf {
    let dir = match kind {
        "mission" => "missions",
        "milestone" => "milestones",
        "plan" => "plans",
        "evidence" => "evidence",
        _ => kind,
    };
    PathBuf::from(dir).join(format!("{}.md", id))
}

fn display_state_path(relative_path: &Path) -> String {
    format!(
        ".atelier-state/{}",
        relative_path.to_string_lossy().replace('\\', "/")
    )
}

fn canonical_priority(priority: &str) -> String {
    match priority {
        "critical" => "P0".to_string(),
        "high" => "P1".to_string(),
        "medium" => "P2".to_string(),
        "low" => "P3".to_string(),
        other => other.to_string(),
    }
}

fn normalize_body(body: &str) -> String {
    body.replace("\r\n", "\n").replace('\r', "\n")
}

fn write_yaml_scalar(output: &mut String, key: &str, value: Option<&str>) -> Result<()> {
    match value {
        Some(value) => {
            output.push_str(key);
            output.push_str(": ");
            output.push_str(&serde_json::to_string(value)?);
            output.push('\n');
        }
        None => {
            output.push_str(key);
            output.push_str(": null\n");
        }
    }
    Ok(())
}

fn write_yaml_array(output: &mut String, key: &str, values: &[String]) -> Result<()> {
    output.push_str(key);
    if values.is_empty() {
        output.push_str(": []\n");
        return Ok(());
    }
    output.push_str(":\n");
    for value in values {
        output.push_str("- ");
        output.push_str(&serde_json::to_string(value)?);
        output.push('\n');
    }
    Ok(())
}

fn write_yaml_links(output: &mut String, key: &str, links: &[IssueLink]) -> Result<()> {
    output.push_str(key);
    if links.is_empty() {
        output.push_str(": []\n");
        return Ok(());
    }
    output.push_str(":\n");
    for link in links {
        output.push_str("- target_id: ");
        output.push_str(&serde_json::to_string(&link.target_id)?);
        output.push('\n');
        output.push_str("  target_kind: ");
        output.push_str(&serde_json::to_string(&link.target_kind)?);
        output.push('\n');
        output.push_str("  type: ");
        output.push_str(&serde_json::to_string(&link.relation_type)?);
        output.push('\n');
    }
    Ok(())
}

fn write_record_links(
    output: &mut String,
    key: &str,
    links: &[crate::models::RecordLink],
) -> Result<()> {
    output.push_str(key);
    if links.is_empty() {
        output.push_str(": []\n");
        return Ok(());
    }
    output.push_str(":\n");
    for link in links {
        output.push_str("- target_id: ");
        output.push_str(&serde_json::to_string(&link.target_id)?);
        output.push('\n');
        output.push_str("  target_kind: ");
        output.push_str(&serde_json::to_string(&link.target_kind)?);
        output.push('\n');
        output.push_str("  type: ");
        output.push_str(&serde_json::to_string(&link.relation_type)?);
        output.push('\n');
    }
    Ok(())
}

fn write_json_scalar(output: &mut String, key: &str, value: &str) -> Result<()> {
    let _: serde_json::Value = serde_json::from_str(value)?;
    output.push_str(key);
    output.push_str(": ");
    output.push_str(&serde_json::to_string(value)?);
    output.push('\n');
    Ok(())
}

pub fn run_markdown(db: &Database, output_path: Option<&str>) -> Result<()> {
    let issues = db.list_issues(Some("all"), None, None)?;
    let mut md = String::new();

    md.push_str("# Atelier Issues Export\n\n");
    md.push_str(&format!(
        "Exported: {}\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    // Group by status
    let open: Vec<_> = issues.iter().filter(|i| i.status == "open").collect();
    let closed: Vec<_> = issues.iter().filter(|i| i.status == "closed").collect();
    let archived: Vec<_> = issues.iter().filter(|i| i.status == "archived").collect();

    if !open.is_empty() {
        md.push_str("## Open Issues\n\n");
        for issue in &open {
            write_issue_md(&mut md, db, issue)?;
        }
    }

    if !closed.is_empty() {
        md.push_str("## Closed Issues\n\n");
        for issue in &closed {
            write_issue_md(&mut md, db, issue)?;
        }
    }

    if !archived.is_empty() {
        md.push_str("## Archived Issues\n\n");
        for issue in &archived {
            write_issue_md(&mut md, db, issue)?;
        }
    }

    match output_path {
        Some(path) => {
            fs::write(path, md).context("Failed to write export file")?;
            eprintln!("Exported {} issues to {}", issues.len(), path);
        }
        None => {
            let mut stdout = io::stdout().lock();
            writeln!(stdout, "{}", md)?;
        }
    }
    Ok(())
}

fn write_issue_md(md: &mut String, db: &Database, issue: &Issue) -> Result<()> {
    let checkbox = if issue.status == "closed" {
        "[x]"
    } else {
        "[ ]"
    };

    md.push_str(&format!(
        "### {} {}: {}\n\n",
        checkbox, issue.id, issue.title
    ));
    md.push_str(&format!("- **Priority:** {}\n", issue.priority));
    md.push_str(&format!("- **Status:** {}\n", issue.status));

    if let Some(parent_id) = &issue.parent_id {
        md.push_str(&format!("- **Parent:** {}\n", parent_id));
    }

    let labels = db.get_labels(&issue.id)?;
    if !labels.is_empty() {
        md.push_str(&format!("- **Labels:** {}\n", labels.join(", ")));
    }

    md.push_str(&format!(
        "- **Created:** {}\n",
        issue.created_at.format("%Y-%m-%d")
    ));

    if let Some(ref desc) = issue.description {
        if !desc.is_empty() {
            md.push_str(&format!("\n{}\n", desc));
        }
    }

    let comments = db.get_comments(&issue.id)?;
    if !comments.is_empty() {
        md.push_str("\n**Comments:**\n");
        for comment in comments {
            md.push_str(&format!(
                "- [{}] {}\n",
                comment.created_at.format("%Y-%m-%d %H:%M"),
                comment.content
            ));
        }
    }

    md.push_str("\n---\n\n");
    Ok(())
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
    fn test_export_issue_basic() {
        let (db, _dir) = setup_test_db();
        let id = db.create_issue("Test issue", None, "medium").unwrap();
        let issue = db.get_issue(&id).unwrap().unwrap();
        let exported = export_issue(&db, &issue).unwrap();
        assert_eq!(exported.id, id);
        assert_eq!(exported.title, "Test issue");
        assert_eq!(exported.priority, "medium");
        assert_eq!(exported.status, "open");
    }

    #[test]
    fn test_export_issue_with_labels() {
        let (db, _dir) = setup_test_db();
        let id = db.create_issue("Test issue", None, "medium").unwrap();
        db.add_label(&id, "bug").unwrap();
        db.add_label(&id, "urgent").unwrap();
        let issue = db.get_issue(&id).unwrap().unwrap();
        let exported = export_issue(&db, &issue).unwrap();
        assert_eq!(exported.labels.len(), 2);
    }

    #[test]
    fn test_export_issue_with_comments() {
        let (db, _dir) = setup_test_db();
        let id = db.create_issue("Test issue", None, "medium").unwrap();
        db.add_comment(&id, "First comment", "note").unwrap();
        db.add_comment(&id, "Second comment", "note").unwrap();
        let issue = db.get_issue(&id).unwrap().unwrap();
        let exported = export_issue(&db, &issue).unwrap();
        assert_eq!(exported.comments.len(), 2);
    }

    #[test]
    fn test_export_closed_issue() {
        let (db, _dir) = setup_test_db();
        let id = db.create_issue("Test issue", None, "medium").unwrap();
        db.close_issue(&id).unwrap();
        let issue = db.get_issue(&id).unwrap().unwrap();
        let exported = export_issue(&db, &issue).unwrap();
        assert_eq!(exported.status, "closed");
        assert!(exported.closed_at.is_some());
    }

    #[test]
    fn test_run_json_to_file() {
        let (db, dir) = setup_test_db();
        db.create_issue("Issue 1", None, "high").unwrap();
        db.create_issue("Issue 2", Some("Description"), "low")
            .unwrap();
        let output_path = dir.path().join("export.json");
        let result = run_json(&db, Some(output_path.to_str().unwrap()));
        assert!(result.is_ok());
        let content = fs::read_to_string(&output_path).unwrap();
        let data: ExportData = serde_json::from_str(&content).unwrap();
        assert_eq!(data.version, 1);
        assert_eq!(data.issues.len(), 2);
    }

    #[test]
    fn test_run_json_empty_database() {
        let (db, dir) = setup_test_db();
        let output_path = dir.path().join("export.json");
        let result = run_json(&db, Some(output_path.to_str().unwrap()));
        assert!(result.is_ok());
        let content = fs::read_to_string(&output_path).unwrap();
        let data: ExportData = serde_json::from_str(&content).unwrap();
        assert_eq!(data.issues.len(), 0);
    }

    #[test]
    fn test_run_markdown_to_file() {
        let (db, dir) = setup_test_db();
        db.create_issue("Issue 1", None, "high").unwrap();
        let output_path = dir.path().join("export.md");
        let result = run_markdown(&db, Some(output_path.to_str().unwrap()));
        assert!(result.is_ok());
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("# Atelier Issues Export"));
    }

    #[test]
    fn test_markdown_groups_by_status() {
        let (db, dir) = setup_test_db();
        db.create_issue("Open issue", None, "medium").unwrap();
        let closed_id = db.create_issue("Closed issue", None, "medium").unwrap();
        db.close_issue(&closed_id).unwrap();
        let output_path = dir.path().join("export.md");
        run_markdown(&db, Some(output_path.to_str().unwrap())).unwrap();
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("## Open Issues"));
        assert!(content.contains("## Closed Issues"));
    }

    #[test]
    fn test_export_unicode_content() {
        let (db, dir) = setup_test_db();
        let id = db
            .create_issue("Test 🐛", Some("Description αβγ"), "medium")
            .unwrap();
        db.add_label(&id, "バグ").unwrap();
        let output_path = dir.path().join("export.json");
        run_json(&db, Some(output_path.to_str().unwrap())).unwrap();
        let content = fs::read_to_string(&output_path).unwrap();
        let data: ExportData = serde_json::from_str(&content).unwrap();
        assert_eq!(data.issues[0].title, "Test 🐛");
    }

    #[test]
    fn test_export_data_roundtrip() {
        let data = ExportData {
            version: 1,
            exported_at: "2024-01-01T00:00:00Z".to_string(),
            issues: vec![ExportedIssue {
                id: "atelier-0001".to_string(),
                title: "Test".to_string(),
                description: Some("Desc".to_string()),
                status: "open".to_string(),
                issue_type: "task".to_string(),
                priority: "medium".to_string(),
                parent_id: None,
                labels: vec!["bug".to_string()],
                comments: vec![ExportedComment {
                    content: "Comment".to_string(),
                    created_at: "2024-01-01T00:00:00Z".to_string(),
                    kind: "note".to_string(),
                }],
                created_at: "2024-01-01T00:00:00Z".to_string(),
                updated_at: "2024-01-01T00:00:00Z".to_string(),
                closed_at: None,
            }],
        };
        let json = serde_json::to_string(&data).unwrap();
        let parsed: ExportData = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.version, data.version);
        assert_eq!(parsed.issues.len(), 1);
    }

    #[test]
    fn test_canonical_noop_export_is_deterministic() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier-state");

        run_canonical(&db, &state_dir, false).unwrap();
        let first_files = canonical_files_under(&state_dir).unwrap();

        run_canonical(&db, &state_dir, false).unwrap();
        let second_files = canonical_files_under(&state_dir).unwrap();

        assert_eq!(first_files, second_files);
        assert!(run_canonical(&db, &state_dir, true).is_ok());
        assert!(!state_dir.join("manifest.json").exists());
        assert!(!state_dir.join("graph.json").exists());
    }

    #[test]
    fn test_canonical_export_preserves_issue_activity_sidecars() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier-state");
        let id = db.create_issue("Activity", None, "medium").unwrap();
        run_canonical(&db, &state_dir, false).unwrap();
        let activity_path = state_dir
            .join("issues")
            .join(format!("{id}.activity"))
            .join("20260610T181920123456Z.md");
        fs::create_dir_all(activity_path.parent().unwrap()).unwrap();
        fs::write(
            &activity_path,
            format!(
                "---\nschema: \"atelier.activity\"\nschema_version: 1\nid: \"20260610T181920123456Z\"\nsubject_kind: \"issue\"\nsubject_id: \"{id}\"\nevent_type: \"comment\"\nactor: \"tester\"\ncreated_at: \"2026-06-10T18:19:20.123456Z\"\nsummary: \"Activity\"\n---\n\nBody\n"
            ),
        )
        .unwrap();

        run_canonical(&db, &state_dir, false).unwrap();

        assert!(activity_path.exists());
        assert!(run_canonical(&db, &state_dir, true).is_ok());
    }

    #[test]
    fn test_canonical_changed_record_export_rewrites_issue() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier-state");
        let id = db
            .create_issue("Original title", Some("Original body"), "high")
            .unwrap();

        run_canonical(&db, &state_dir, false).unwrap();
        let issue_path = state_dir.join(issue_record_path(&id));
        let first_issue = fs::read_to_string(&issue_path).unwrap();
        assert!(first_issue.contains("title: \"Original title\""));
        assert!(first_issue.ends_with("Original body\n"));

        db.update_issue(&id, Some("Changed title"), Some("Changed body"), None)
            .unwrap();
        run_canonical(&db, &state_dir, false).unwrap();
        let second_issue = fs::read_to_string(&issue_path).unwrap();

        assert_ne!(first_issue, second_issue);
        assert!(second_issue.contains("title: \"Changed title\""));
        assert!(second_issue.ends_with("Changed body\n"));
    }

    #[test]
    fn test_canonical_issue_type_is_explicit_not_label_derived() {
        let (db, _dir) = setup_test_db();
        let id = db
            .create_issue_with_type("Validate taxonomy", None, "medium", "validation")
            .unwrap();
        db.add_label(&id, "epic").unwrap();
        let issue = db.get_issue(&id).unwrap().unwrap();

        let exported = export_issue(&db, &issue).unwrap();
        let issue_text = render_issue_record(&db, &issue).unwrap();

        assert_eq!(exported.issue_type, "validation");
        assert!(issue_text.contains("issue_type: \"validation\"\n"));
        assert!(issue_text.contains("labels:\n- \"epic\"\n"));
    }

    #[test]
    fn test_canonical_check_fails_when_projection_is_stale() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier-state");
        let id = db.create_issue("Original title", None, "medium").unwrap();
        run_canonical(&db, &state_dir, false).unwrap();

        db.update_issue(&id, Some("Changed title"), None, None)
            .unwrap();
        let error = run_canonical(&db, &state_dir, true).unwrap_err();

        assert!(error.to_string().contains("Canonical export is stale"));
        assert!(error
            .to_string()
            .contains(&format!("changed: .atelier-state/issues/{id}.md")));
    }

    #[test]
    fn test_canonical_export_removes_stale_record_file() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier-state");
        let id = db.create_issue("Temporary", None, "medium").unwrap();
        run_canonical(&db, &state_dir, false).unwrap();
        let issue_path = state_dir.join(issue_record_path(&id));
        assert!(issue_path.exists());
        fs::write(state_dir.join("manifest.json"), "{}\n").unwrap();
        fs::write(state_dir.join("graph.json"), "{}\n").unwrap();

        db.delete_issue(&id).unwrap();
        run_canonical(&db, &state_dir, false).unwrap();

        assert!(!issue_path.exists());
        assert!(!state_dir.join("manifest.json").exists());
        assert!(!state_dir.join("graph.json").exists());
        assert!(run_canonical(&db, &state_dir, true).is_ok());
    }

    #[test]
    fn test_canonical_check_reports_invalid_duplicate_id() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Original", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier-state");
        run_canonical(&db, &state_dir, false).unwrap();
        let copy_path = state_dir.join(issue_record_path("atelier-zzzz"));
        fs::copy(state_dir.join(issue_record_path(&id)), copy_path).unwrap();

        let error = run_canonical(&db, &state_dir, true).unwrap_err();
        assert!(error.to_string().contains("invalid:"));
        assert!(error.to_string().contains("does not match canonical path"));
    }

    #[test]
    fn test_canonical_check_reports_dangling_link() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Source", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier-state");
        run_canonical(&db, &state_dir, false).unwrap();
        db.add_typed_relation(
            &id,
            db.create_issue("Target", None, "medium").unwrap(),
            "related",
        )
        .unwrap();
        let missing_id = "atelier-zzzz";
        let path = state_dir.join(issue_record_path(&id));
        let text = fs::read_to_string(&path).unwrap().replace(
            "links: []",
            &format!(
                "links:\n- target_id: \"{missing_id}\"\n  target_kind: \"issue\"\n  type: \"related\""
            ),
        );
        fs::write(path, text).unwrap();

        let error = run_canonical(&db, &state_dir, true).unwrap_err();
        assert!(error.to_string().contains(&format!(
            "Issue {id} has related reference to missing issue {missing_id}"
        )));
    }

    #[test]
    fn test_canonical_markdown_serialization_stability() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier-state");
        let parent = db
            .create_issue("Parent", Some("Parent body\r\nline 2"), "high")
            .unwrap();
        let child = db
            .create_subissue(&parent, "Child", Some("Child body"), "low")
            .unwrap();
        db.add_label(&child, "zeta").unwrap();
        db.add_label(&child, "alpha").unwrap();
        db.add_dependency(&child, &parent).unwrap();
        db.add_typed_relation(&parent, &child, "derived").unwrap();

        let first = build_canonical_projection(&db, &state_dir).unwrap();
        let second = build_canonical_projection(&db, &state_dir).unwrap();
        let issue = first
            .iter()
            .find(|file| file.path == issue_record_path(&child))
            .unwrap();
        let parent_issue = first
            .iter()
            .find(|file| file.path == issue_record_path(&parent))
            .unwrap();

        assert_eq!(first, second);
        let issue_text = String::from_utf8(issue.bytes.clone()).unwrap();
        assert!(issue_text.contains("labels:\n- \"alpha\"\n- \"zeta\"\n"));
        assert!(issue_text.contains(&format!("parent: \"{parent}\"")));
        assert!(issue_text.ends_with("Child body\n"));

        let link_text = String::from_utf8(parent_issue.bytes.clone()).unwrap()
            + &String::from_utf8(issue.bytes.clone()).unwrap();
        assert!(
            link_text.contains(&format!(
                "links:\n- target_id: \"{child}\"\n  target_kind: \"issue\"\n  type: \"derived\"\n"
            )) || link_text.contains(&format!(
            "links:\n- target_id: \"{parent}\"\n  target_kind: \"issue\"\n  type: \"derived\"\n"
        ))
        );
    }

    #[test]
    fn test_export_json_file_is_valid() {
        let (db, dir) = setup_test_db();
        db.create_issue("Exported issue", None, "medium").unwrap();
        let output_path = dir.path().join("export.json");
        run_json(&db, Some(output_path.to_str().unwrap())).unwrap();
        let content = fs::read_to_string(&output_path).unwrap();
        let result: Result<ExportData, _> = serde_json::from_str(&content);
        assert!(result.is_ok());
    }
}
