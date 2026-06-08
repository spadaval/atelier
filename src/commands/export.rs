use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::db::Database;
use crate::models::Issue;

#[derive(Serialize, Deserialize)]
pub struct ExportedIssue {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub parent_id: Option<i64>,
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

#[derive(Serialize, Deserialize)]
pub struct ExportData {
    pub version: i32,
    pub exported_at: String,
    pub issues: Vec<ExportedIssue>,
}

#[derive(Debug, Clone)]
struct ProjectionFile {
    path: PathBuf,
    bytes: Vec<u8>,
}

fn export_issue(db: &Database, issue: &Issue) -> Result<ExportedIssue> {
    let labels = db.get_labels(issue.id)?;
    let comments = db.get_comments(issue.id)?;

    Ok(ExportedIssue {
        id: issue.id,
        title: issue.title.clone(),
        description: issue.description.clone(),
        status: issue.status.clone(),
        priority: issue.priority.clone(),
        parent_id: issue.parent_id,
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
    let files = build_canonical_projection(db)?;

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

fn build_canonical_projection(db: &Database) -> Result<Vec<ProjectionFile>> {
    let mut issues = db.list_issues(Some("all"), None, None)?;
    issues.sort_by_key(|issue| issue.id);

    let mut files = Vec::new();
    for issue in &issues {
        files.push(ProjectionFile {
            path: issue_record_path(issue.id),
            bytes: render_issue_record(db, issue)?.into_bytes(),
        });
    }

    files.push(ProjectionFile {
        path: PathBuf::from("graph.json"),
        bytes: render_graph(db, &issues)?.into_bytes(),
    });

    let manifest = render_manifest(&files)?.into_bytes();
    files.push(ProjectionFile {
        path: PathBuf::from("manifest.json"),
        bytes: manifest,
    });
    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(files)
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
    let labels = db.get_labels(issue.id)?;
    let mut blocks = issue_ids(db.get_blocking(issue.id)?);
    let mut depends_on = issue_ids(db.get_blockers(issue.id)?);
    blocks.sort();
    depends_on.sort();

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
    write_yaml_scalar(&mut output, "id", Some(&issue_record_id(issue.id)))?;
    write_yaml_scalar(&mut output, "issue_type", Some("task"))?;
    write_yaml_array(&mut output, "labels", &labels)?;
    write_yaml_array(&mut output, "links", &[])?;
    let parent = issue.parent_id.map(issue_record_id);
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

fn render_graph(db: &Database, issues: &[Issue]) -> Result<String> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for issue in issues {
        let source_id = issue_record_id(issue.id);
        nodes.push(json_object([
            ("id", serde_json::Value::String(source_id.clone())),
            ("kind", serde_json::Value::String("issue".to_string())),
            (
                "schema",
                serde_json::Value::String("atelier.issue".to_string()),
            ),
            ("schema_version", serde_json::Value::Number(1.into())),
        ]));

        if let Some(parent_id) = issue.parent_id {
            edges.push(graph_edge(
                &source_id,
                "parent",
                &issue_record_id(parent_id),
                "front-matter",
            ));
        }

        for blocked_id in db.get_blocking(issue.id)? {
            edges.push(graph_edge(
                &source_id,
                "blocks",
                &issue_record_id(blocked_id),
                "front-matter",
            ));
        }

        for relation in db.get_typed_relations(issue.id)? {
            if relation.issue_id_1 == issue.id {
                edges.push(graph_edge(
                    &source_id,
                    &relation.relation_type,
                    &issue_record_id(relation.issue_id_2),
                    "relation",
                ));
            }
        }
    }

    nodes.sort_by_key(value_sort_key);
    edges.sort_by_key(value_sort_key);

    let graph = json_object([
        ("edges", serde_json::Value::Array(edges)),
        ("nodes", serde_json::Value::Array(nodes)),
        (
            "schema",
            serde_json::Value::String("atelier.graph".to_string()),
        ),
        ("schema_version", serde_json::Value::Number(1.into())),
    ]);
    let mut json = serde_json::to_string_pretty(&graph)?;
    json.push('\n');
    Ok(json)
}

fn graph_edge(
    source_id: &str,
    relation_type: &str,
    target_id: &str,
    metadata_source: &str,
) -> serde_json::Value {
    json_object([
        (
            "metadata",
            json_object([(
                "source",
                serde_json::Value::String(metadata_source.to_string()),
            )]),
        ),
        (
            "source_id",
            serde_json::Value::String(source_id.to_string()),
        ),
        (
            "source_kind",
            serde_json::Value::String("issue".to_string()),
        ),
        (
            "target_id",
            serde_json::Value::String(target_id.to_string()),
        ),
        (
            "target_kind",
            serde_json::Value::String("issue".to_string()),
        ),
        ("type", serde_json::Value::String(relation_type.to_string())),
    ])
}

fn render_manifest(files: &[ProjectionFile]) -> Result<String> {
    let mut records = Vec::new();
    for file in files
        .iter()
        .filter(|file| file.path != Path::new("manifest.json"))
    {
        let state_path = display_state_path(&file.path);
        let (kind, id, schema) = if file.path == Path::new("graph.json") {
            ("graph", serde_json::Value::Null, "atelier.graph")
        } else {
            let id = file
                .path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .ok_or_else(|| {
                    anyhow::anyhow!("Invalid projection path {}", file.path.display())
                })?;
            (
                "issue",
                serde_json::Value::String(id.to_string()),
                "atelier.issue",
            )
        };

        records.push(json_object([
            ("id", id),
            ("kind", serde_json::Value::String(kind.to_string())),
            ("path", serde_json::Value::String(state_path)),
            ("role", serde_json::Value::String("canonical".to_string())),
            ("schema", serde_json::Value::String(schema.to_string())),
            ("schema_version", serde_json::Value::Number(1.into())),
            ("sha256", serde_json::Value::String(sha256_hex(&file.bytes))),
        ]));
    }
    records.sort_by_key(value_sort_key);

    let manifest = json_object([
        ("format_version", serde_json::Value::Number(1.into())),
        ("generated_at", serde_json::Value::Null),
        (
            "generator",
            json_object([
                ("name", serde_json::Value::String("atelier".to_string())),
                (
                    "version",
                    serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()),
                ),
            ]),
        ),
        ("records", serde_json::Value::Array(records)),
        (
            "schema",
            serde_json::Value::String("atelier.manifest".to_string()),
        ),
        ("schema_version", serde_json::Value::Number(1.into())),
    ]);

    let mut json = serde_json::to_string_pretty(&manifest)?;
    json.push('\n');
    Ok(json)
}

fn json_object<const N: usize>(entries: [(&str, serde_json::Value); N]) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (key, value) in entries {
        map.insert(key.to_string(), value);
    }
    serde_json::Value::Object(map)
}

fn value_sort_key(value: &serde_json::Value) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn issue_ids(ids: Vec<i64>) -> Vec<String> {
    ids.into_iter().map(issue_record_id).collect()
}

fn issue_record_id(id: i64) -> String {
    format!("ISS-{id:04}")
}

fn issue_record_path(id: i64) -> PathBuf {
    PathBuf::from("issues").join(format!("{}.md", issue_record_id(id)))
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
        "### {} #{}: {}\n\n",
        checkbox, issue.id, issue.title
    ));
    md.push_str(&format!("- **Priority:** {}\n", issue.priority));
    md.push_str(&format!("- **Status:** {}\n", issue.status));

    if let Some(parent_id) = issue.parent_id {
        md.push_str(&format!("- **Parent:** #{}\n", parent_id));
    }

    let labels = db.get_labels(issue.id)?;
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

    let comments = db.get_comments(issue.id)?;
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
    use proptest::prelude::*;
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
        let issue = db.get_issue(id).unwrap().unwrap();
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
        db.add_label(id, "bug").unwrap();
        db.add_label(id, "urgent").unwrap();
        let issue = db.get_issue(id).unwrap().unwrap();
        let exported = export_issue(&db, &issue).unwrap();
        assert_eq!(exported.labels.len(), 2);
    }

    #[test]
    fn test_export_issue_with_comments() {
        let (db, _dir) = setup_test_db();
        let id = db.create_issue("Test issue", None, "medium").unwrap();
        db.add_comment(id, "First comment", "note").unwrap();
        db.add_comment(id, "Second comment", "note").unwrap();
        let issue = db.get_issue(id).unwrap().unwrap();
        let exported = export_issue(&db, &issue).unwrap();
        assert_eq!(exported.comments.len(), 2);
    }

    #[test]
    fn test_export_closed_issue() {
        let (db, _dir) = setup_test_db();
        let id = db.create_issue("Test issue", None, "medium").unwrap();
        db.close_issue(id).unwrap();
        let issue = db.get_issue(id).unwrap().unwrap();
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
        db.close_issue(closed_id).unwrap();
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
        db.add_label(id, "バグ").unwrap();
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
                id: 1,
                title: "Test".to_string(),
                description: Some("Desc".to_string()),
                status: "open".to_string(),
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
        let first_manifest = fs::read_to_string(state_dir.join("manifest.json")).unwrap();
        let first_graph = fs::read_to_string(state_dir.join("graph.json")).unwrap();

        run_canonical(&db, &state_dir, false).unwrap();
        let second_manifest = fs::read_to_string(state_dir.join("manifest.json")).unwrap();
        let second_graph = fs::read_to_string(state_dir.join("graph.json")).unwrap();

        assert_eq!(first_manifest, second_manifest);
        assert_eq!(first_graph, second_graph);
        assert!(run_canonical(&db, &state_dir, true).is_ok());
        assert!(!state_dir.join("issues").join("ISS-0001.md").exists());
    }

    #[test]
    fn test_canonical_changed_record_export_rewrites_issue() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier-state");
        let id = db
            .create_issue("Original title", Some("Original body"), "high")
            .unwrap();

        run_canonical(&db, &state_dir, false).unwrap();
        let issue_path = state_dir.join("issues").join("ISS-0001.md");
        let first_issue = fs::read_to_string(&issue_path).unwrap();
        assert!(first_issue.contains("title: \"Original title\""));
        assert!(first_issue.ends_with("Original body\n"));

        db.update_issue(id, Some("Changed title"), Some("Changed body"), None)
            .unwrap();
        run_canonical(&db, &state_dir, false).unwrap();
        let second_issue = fs::read_to_string(&issue_path).unwrap();

        assert_ne!(first_issue, second_issue);
        assert!(second_issue.contains("title: \"Changed title\""));
        assert!(second_issue.ends_with("Changed body\n"));
    }

    #[test]
    fn test_canonical_check_fails_when_projection_is_stale() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier-state");
        let id = db.create_issue("Original title", None, "medium").unwrap();
        run_canonical(&db, &state_dir, false).unwrap();

        db.update_issue(id, Some("Changed title"), None, None)
            .unwrap();
        let error = run_canonical(&db, &state_dir, true).unwrap_err();

        assert!(error.to_string().contains("Canonical export is stale"));
        assert!(error
            .to_string()
            .contains("changed: .atelier-state/issues/ISS-0001.md"));
    }

    #[test]
    fn test_canonical_export_removes_stale_record_file() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier-state");
        let id = db.create_issue("Temporary", None, "medium").unwrap();
        run_canonical(&db, &state_dir, false).unwrap();
        let issue_path = state_dir.join("issues").join("ISS-0001.md");
        assert!(issue_path.exists());

        db.delete_issue(id).unwrap();
        run_canonical(&db, &state_dir, false).unwrap();

        assert!(!issue_path.exists());
        assert!(run_canonical(&db, &state_dir, true).is_ok());
    }

    #[test]
    fn test_canonical_json_and_markdown_serialization_stability() {
        let (db, _dir) = setup_test_db();
        let parent = db
            .create_issue("Parent", Some("Parent body\r\nline 2"), "high")
            .unwrap();
        let child = db
            .create_subissue(parent, "Child", Some("Child body"), "low")
            .unwrap();
        db.add_label(child, "zeta").unwrap();
        db.add_label(child, "alpha").unwrap();
        db.add_dependency(child, parent).unwrap();

        let first = build_canonical_projection(&db).unwrap();
        let second = build_canonical_projection(&db).unwrap();
        let first_manifest = first
            .iter()
            .find(|file| file.path == Path::new("manifest.json"))
            .unwrap();
        let second_manifest = second
            .iter()
            .find(|file| file.path == Path::new("manifest.json"))
            .unwrap();
        let issue = first
            .iter()
            .find(|file| file.path == Path::new("issues/ISS-0002.md"))
            .unwrap();
        let graph = first
            .iter()
            .find(|file| file.path == Path::new("graph.json"))
            .unwrap();

        assert_eq!(first_manifest.bytes, second_manifest.bytes);
        let issue_text = String::from_utf8(issue.bytes.clone()).unwrap();
        assert!(issue_text.contains("labels:\n- \"alpha\"\n- \"zeta\"\n"));
        assert!(issue_text.contains("parent: \"ISS-0001\""));
        assert!(issue_text.ends_with("Child body\n"));

        let graph_json: serde_json::Value = serde_json::from_slice(&graph.bytes).unwrap();
        assert_eq!(graph_json["schema"], "atelier.graph");
        assert!(graph_json["edges"].as_array().unwrap().iter().any(|edge| {
            edge["source_id"] == "ISS-0001"
                && edge["target_id"] == "ISS-0002"
                && edge["type"] == "blocks"
        }));
    }

    proptest! {
        #[test]
        fn prop_export_never_panics(title in "[a-zA-Z0-9 ]{1,50}") {
            let (db, dir) = setup_test_db();
            db.create_issue(&title, None, "medium").unwrap();
            let output_path = dir.path().join("export.json");
            let result = run_json(&db, Some(output_path.to_str().unwrap()));
            prop_assert!(result.is_ok());
        }

        #[test]
        fn prop_json_is_valid(title in "[a-zA-Z0-9 ]{1,30}") {
            let (db, dir) = setup_test_db();
            db.create_issue(&title, None, "medium").unwrap();
            let output_path = dir.path().join("export.json");
            run_json(&db, Some(output_path.to_str().unwrap())).unwrap();
            let content = fs::read_to_string(&output_path).unwrap();
            let result: Result<ExportData, _> = serde_json::from_str(&content);
            prop_assert!(result.is_ok());
        }
    }
}
