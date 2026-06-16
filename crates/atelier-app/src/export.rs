use anyhow::{bail, Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::storage_layout;
use atelier_core::{DomainRecord, Issue, RecordLink};
use atelier_records as record_store;
use atelier_records::{
    attachment_relationship, relates_relationship, relationship_target, CanonicalIssueRecord,
    IssueSections, Relationships, FIRST_CLASS_RECORD_KINDS,
};
use atelier_sqlite::projection_index;
use atelier_sqlite::Database;

use crate::{Outcome, Request, ViewModel};

#[derive(Debug, Clone, Eq, PartialEq)]
struct ProjectionFile {
    path: PathBuf,
    bytes: Vec<u8>,
}

pub struct CanonicalExportRequest<'a> {
    pub db: &'a Database,
    pub state_dir: PathBuf,
    pub check: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CanonicalExportView {
    pub state_dir: PathBuf,
    pub check: bool,
    pub stale_entries: Vec<String>,
    pub wrote: bool,
}

pub fn canonical_export(
    request: Request<CanonicalExportRequest<'_>>,
) -> Result<Outcome<ViewModel<CanonicalExportView>>> {
    let input = request.input;
    if input.check {
        let stale_entries = canonical_stale_entries(input.db, &input.state_dir)?;
        if stale_entries.is_empty() {
            return Ok(Outcome {
                value: ViewModel {
                    data: CanonicalExportView {
                        state_dir: input.state_dir,
                        check: true,
                        stale_entries,
                        wrote: false,
                    },
                },
            });
        }

        bail!("Canonical export is stale:\n{}", stale_entries.join("\n"));
    }

    let rewrite_risks = canonical_rewrite_risk_entries(input.db, &input.state_dir)?;
    if !rewrite_risks.is_empty() {
        bail!(
            "Refusing to write canonical tracker records from the local projection:\n{}\nrecovery: 1. run `atelier lint`; 2. fix any named canonical Markdown records; 3. run `atelier doctor --fix` for ignored local projection/runtime state; 4. rerun the blocked command",
            rewrite_risks.join("\n")
        );
    }

    write_canonical_from_db(input.db, &input.state_dir)?;
    Ok(Outcome {
        value: ViewModel {
            data: CanonicalExportView {
                state_dir: input.state_dir,
                check: false,
                stale_entries: Vec::new(),
                wrote: true,
            },
        },
    })
}

pub fn run_canonical(db: &Database, state_dir: &Path, check: bool) -> Result<()> {
    if check {
        let stale = canonical_stale_entries(db, state_dir)?;
        if stale.is_empty() {
            tracing::info!("Canonical export is current");
            return Ok(());
        }

        bail!("Canonical export is stale:\n{}", stale.join("\n"));
    }

    write_canonical_from_db(db, state_dir)?;
    tracing::info!(
        "Exported canonical state to {}",
        state_dir.to_string_lossy()
    );
    Ok(())
}

pub fn write_canonical_from_db(db: &Database, state_dir: &Path) -> Result<()> {
    let files = build_canonical_projection(db, state_dir)?;
    write_canonical_projection(state_dir, &files)?;
    projection_index::refresh(db, state_dir)
}

pub fn canonical_stale_entries(db: &Database, state_dir: &Path) -> Result<Vec<String>> {
    canonical_check_entries(db, state_dir)
}

fn canonical_rewrite_risk_entries(db: &Database, state_dir: &Path) -> Result<Vec<String>> {
    if !state_dir.exists() {
        return Ok(Vec::new());
    }

    let mut risks = canonical_check_entries(db, state_dir)?;
    if !risks.is_empty() {
        return Ok(risks);
    }

    let projected = build_canonical_projection(db, state_dir)?;
    let expected: BTreeSet<PathBuf> = projected.iter().map(|file| file.path.clone()).collect();
    for file in projected {
        let path = state_dir.join(&file.path);
        match fs::read(&path) {
            Ok(existing) if existing == file.bytes => {}
            Ok(_) => risks.push(format!(
                "would rewrite tracked canonical record from local projection: {}",
                display_state_path(&file.path)
            )),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => risks.push(format!(
                "would create tracked canonical record from local projection: {}",
                display_state_path(&file.path)
            )),
            Err(error) => {
                return Err(error).with_context(|| format!("Failed to read {}", path.display()));
            }
        }
    }

    for relative_path in canonical_files_under(state_dir)? {
        if !is_renderer_owned_path(&relative_path) {
            continue;
        }
        if !expected.contains(&relative_path) {
            risks.push(format!(
                "would remove tracked canonical record from local projection: {}",
                display_state_path(&relative_path)
            ));
        }
    }

    risks.sort();
    Ok(risks)
}

fn is_renderer_owned_path(relative: &Path) -> bool {
    if relative == Path::new("manifest.json") || relative == Path::new("graph.json") {
        return true;
    }
    if is_activity_path(relative) {
        return true;
    }
    relative.components().next().is_some_and(|component| {
        let name = component.as_os_str().to_string_lossy();
        atelier_records::canonical_record_dirs()
            .iter()
            .any(|directory| name == *directory)
    })
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
    for spec in FIRST_CLASS_RECORD_KINDS {
        for record in db.list_records(spec.kind, None)? {
            files.push(ProjectionFile {
                path: record_store::canonical_record_path(spec, &record.id)?,
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
        if !is_activity_path(&relative) {
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

fn is_activity_path(relative: &Path) -> bool {
    let mut components = relative.components();
    let Some(std::path::Component::Normal(root)) = components.next() else {
        return false;
    };
    if root != "issues" && root != "missions" {
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

fn canonical_check_entries(db: &Database, state_dir: &Path) -> Result<Vec<String>> {
    let mut stale = Vec::new();

    if !state_dir.exists() {
        if has_project_records(db)? {
            stale.push(format!("missing: {}", state_dir.display()));
        }
        return Ok(stale);
    }

    if let Err(error) = crate::rebuild::validate_canonical_state(state_dir) {
        stale.push(format!(
            "invalid: canonical tracker Markdown is invalid while running a deterministic export diagnostic: {error:#}\nrecovery: 1. run `atelier lint`; 2. fix the named canonical Markdown record; 3. run `atelier doctor --fix`; 4. rerun the blocked command"
        ));
        return Ok(stale);
    }

    let freshness = projection_index::check(db, state_dir)?;
    stale.extend(
        freshness
            .problem_messages()
            .into_iter()
            .map(|message| format!("projection: {message}")),
    );

    stale.sort();
    Ok(stale)
}

fn has_project_records(db: &Database) -> Result<bool> {
    if !db.list_issues(Some("all"), None, None)?.is_empty() {
        return Ok(true);
    }
    for spec in FIRST_CLASS_RECORD_KINDS {
        if !db.list_records(spec.kind, None)?.is_empty() {
            return Ok(true);
        }
    }
    Ok(false)
}

fn remove_stale_canonical_files(state_dir: &Path, expected: &BTreeSet<PathBuf>) -> Result<()> {
    if !state_dir.exists() {
        return Ok(());
    }

    for relative_path in canonical_files_under(state_dir)? {
        if !is_renderer_owned_path(&relative_path) {
            continue;
        }
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
        let relative = path
            .strip_prefix(root)
            .context("Failed to relativize canonical export path")?;
        if storage_layout::is_local_atelier_path(relative) {
            continue;
        }
        if path.is_dir() {
            collect_canonical_files(root, &path, files)?;
        } else if path.is_file() {
            files.push(relative.to_path_buf());
        }
    }
    Ok(())
}

fn render_issue_record(db: &Database, issue: &Issue) -> Result<String> {
    let labels = db.get_labels(&issue.id)?;
    let mut relationships = issue_relationships(db, issue)?;
    record_store::sort_relationships(&mut relationships);

    record_store::render_issue_record(&CanonicalIssueRecord {
        issue: issue.clone(),
        labels,
        sections: IssueSections::unchecked_from_body(issue.description.as_deref()),
        relationships,
    })
}

fn render_domain_record(db: &Database, record: &DomainRecord) -> Result<String> {
    let mut relationships = domain_relationships(db, record)?;
    record_store::sort_relationships(&mut relationships);
    record_store::render_domain_record(&record_store::CanonicalDomainRecord {
        record: record.clone(),
        labels: domain_labels(record),
        relationships,
    })
}

fn issue_relationships(db: &Database, issue: &Issue) -> Result<Relationships> {
    let mut relationships = Relationships::default();
    for id in db.get_blocking(&issue.id)? {
        relationships.blocks.push(relationship_target("issue", &id));
    }
    for child in db.get_subissues(&issue.id)? {
        relationships
            .children
            .push(relationship_target("issue", &child.id));
    }
    for relation in db.get_typed_relations(&issue.id)? {
        if relation.issue_id_1 == issue.id {
            relationships.relates.push(relates_relationship(
                "issue",
                &relation.issue_id_2,
                &relation.relation_type,
            ));
        }
    }
    for link in db.list_record_links("issue", &issue.id)? {
        classify_record_link_for_owner(&mut relationships, &link, "issue", &issue.id);
    }
    Ok(relationships)
}

fn domain_relationships(db: &Database, record: &DomainRecord) -> Result<Relationships> {
    let mut relationships = Relationships::default();
    for link in db.list_record_links(&record.kind, &record.id)? {
        classify_record_link_for_owner(&mut relationships, &link, &record.kind, &record.id);
    }
    Ok(relationships)
}

fn classify_record_link_for_owner(
    relationships: &mut Relationships,
    link: &RecordLink,
    owner_kind: &str,
    owner_id: &str,
) {
    if link.source_kind == owner_kind && link.source_id == owner_id {
        if owner_kind == "issue"
            && link.target_kind == "issue"
            && is_child_relation(&link.relation_type)
        {
            relationships
                .children
                .push(relationship_target(&link.target_kind, &link.target_id));
        } else if is_attachment_kind(&link.target_kind) && is_attachment_role(&link.relation_type) {
            relationships.attachments.push(attachment_relationship(
                &link.target_kind,
                &link.target_id,
                &link.relation_type,
            ));
        } else {
            relationships.relates.push(relates_relationship(
                &link.target_kind,
                &link.target_id,
                &link.relation_type,
            ));
        }
    }
}

fn is_child_relation(relation_type: &str) -> bool {
    matches!(
        relation_type,
        "advances" | "contributes_to" | "implements" | "has_checkpoint"
    )
}

fn is_attachment_kind(kind: &str) -> bool {
    matches!(kind, "plan" | "evidence" | "milestone")
}

fn is_attachment_role(relation_type: &str) -> bool {
    matches!(
        relation_type,
        "planned_by" | "validates" | "evidenced_by" | "has_checkpoint"
    )
}

fn issue_record_path(id: &str) -> PathBuf {
    record_store::issue_record_path(id)
}

fn domain_labels(record: &DomainRecord) -> Vec<String> {
    match record.kind.as_str() {
        "mission" => vec!["mission".to_string()],
        _ => Vec::new(),
    }
}

fn display_state_path(relative_path: &Path) -> String {
    format!(
        ".atelier/{}",
        relative_path.to_string_lossy().replace('\\', "/")
    )
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
    fn test_canonical_noop_export_is_deterministic() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier");

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
        let state_dir = dir.path().join(".atelier");
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
        let state_dir = dir.path().join(".atelier");
        let id = db
            .create_issue("Original title", Some("Original body"), "high")
            .unwrap();

        run_canonical(&db, &state_dir, false).unwrap();
        let issue_path = state_dir.join(issue_record_path(&id));
        let first_issue = fs::read_to_string(&issue_path).unwrap();
        assert!(first_issue.contains("title: \"Original title\""));
        assert!(first_issue.contains("## Description\n\nOriginal body"));
        assert!(first_issue.contains("## Outcome\n\nOutcome was not specified."));
        assert!(first_issue.contains("## Evidence\n\nEvidence was not specified."));

        db.update_issue(&id, Some("Changed title"), Some("Changed body"), None)
            .unwrap();
        run_canonical(&db, &state_dir, false).unwrap();
        let second_issue = fs::read_to_string(&issue_path).unwrap();

        assert_ne!(first_issue, second_issue);
        assert!(second_issue.contains("title: \"Changed title\""));
        assert!(second_issue.contains("## Description\n\nChanged body"));
        assert!(second_issue.contains("## Outcome\n\nOutcome was not specified."));
        assert!(second_issue.contains("## Evidence\n\nEvidence was not specified."));
    }

    #[test]
    fn test_canonical_issue_type_is_explicit_not_label_derived() {
        let (db, _dir) = setup_test_db();
        let id = db
            .create_issue_with_type("Validate taxonomy", None, "medium", "validation")
            .unwrap();
        db.add_label(&id, "epic").unwrap();
        let issue = db.get_issue(&id).unwrap().unwrap();

        let issue_text = render_issue_record(&db, &issue).unwrap();

        assert!(issue_text.contains("issue_type: \"validation\"\n"));
        assert!(issue_text.contains("labels:\n- \"epic\"\n"));
    }

    #[test]
    fn test_canonical_check_ignores_sqlite_only_canonical_drift() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier");
        let id = db.create_issue("Original title", None, "medium").unwrap();
        run_canonical(&db, &state_dir, false).unwrap();

        db.update_issue(&id, Some("Changed title"), None, None)
            .unwrap();

        assert!(run_canonical(&db, &state_dir, true).is_ok());
        let issue_text = fs::read_to_string(state_dir.join(issue_record_path(&id))).unwrap();
        assert!(issue_text.contains("title: \"Original title\""));
    }

    #[test]
    fn test_canonical_check_reports_stale_projection_metadata() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier");
        let id = db.create_issue("Original title", None, "medium").unwrap();
        run_canonical(&db, &state_dir, false).unwrap();

        let issue_path = state_dir.join(issue_record_path(&id));
        let markdown = fs::read_to_string(&issue_path).unwrap();
        fs::write(
            &issue_path,
            markdown.replace("Original title", "Markdown-first title"),
        )
        .unwrap();

        let error = run_canonical(&db, &state_dir, true).unwrap_err();

        assert!(error.to_string().contains("Canonical export is stale"));
        assert!(error.to_string().contains(&format!(
            "projection: indexed source changed: issues/{id}.md"
        )));
    }

    #[test]
    fn test_canonical_export_removes_stale_record_file() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier");
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
        let state_dir = dir.path().join(".atelier");
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
        let state_dir = dir.path().join(".atelier");
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
            "  relates: []",
            &format!(
                "  relates:\n  - kind: \"issue\"\n    id: \"{missing_id}\"\n    type: \"related\""
            ),
        );
        fs::write(path, text).unwrap();

        let error = run_canonical(&db, &state_dir, true).unwrap_err();
        assert!(error.to_string().contains(&format!(
            "{id} has related reference to missing issue {missing_id}"
        )));
    }

    #[test]
    fn test_canonical_markdown_serialization_stability() {
        let (db, dir) = setup_test_db();
        let state_dir = dir.path().join(".atelier");
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
        assert!(issue_text.contains("## Description\n\nChild body"));
        assert!(issue_text.contains("## Outcome\n\nOutcome was not specified."));
        assert!(issue_text.contains("## Evidence\n\nEvidence was not specified."));

        let parent_text = String::from_utf8(parent_issue.bytes.clone()).unwrap();
        assert!(parent_text.contains(&format!(
            "  blocks:\n  - kind: \"issue\"\n    id: \"{child}\"\n"
        )));
        assert!(parent_text.contains(&format!(
            "  children:\n  - kind: \"issue\"\n    id: \"{child}\"\n"
        )));
        let combined_text = parent_text + &String::from_utf8(issue.bytes.clone()).unwrap();
        assert!(
            combined_text.contains(&format!(
                "  relates:\n  - kind: \"issue\"\n    id: \"{child}\"\n    type: \"derived\"\n"
            )) || combined_text.contains(&format!(
                "  relates:\n  - kind: \"issue\"\n    id: \"{parent}\"\n    type: \"derived\"\n"
            ))
        );
    }
}
