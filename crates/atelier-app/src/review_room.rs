use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, bail, Context, Result};
use atelier_core::{Record, RecordHeader, Relationships, ReviewRecord};
use atelier_records::RecordStore;
use atelier_sqlite::Database;
use chrono::Utc;
use serde_json::{json, Value};

use crate::project_config::{ProjectConfig, ReviewConfig};
use crate::workflow_policy::{self, REVIEW_FIELD};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomOpenRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub db_path: &'a Path,
    pub issue_ref: Option<&'a str>,
    pub role: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub source_branch: &'a str,
    pub target_branch: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomOpenOutcome {
    pub issue_id: String,
    pub owner_id: String,
    pub review_id: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomStatusRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub issue_ref: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomStatusOutcome {
    pub issue_id: String,
    pub review_id: String,
    pub status: String,
    pub approvals: usize,
    pub unresolved_blocking: usize,
    pub unresolved_nonblocking: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomShowOutcome {
    pub status: RoomStatusOutcome,
    pub title: String,
    pub source_branch: String,
    pub target_branch: String,
    pub events: Vec<RoomEventView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomEventView {
    pub id: String,
    pub kind: String,
    pub actor: Option<String>,
    pub body: Option<String>,
    pub severity: Option<String>,
    pub finding: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomCommentRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub db_path: &'a Path,
    pub issue_ref: Option<&'a str>,
    pub role: &'a str,
    pub body: &'a str,
    pub finding: bool,
    pub severity: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomEventOutcome {
    pub issue_id: String,
    pub review_id: String,
    pub event_id: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomDecisionRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub db_path: &'a Path,
    pub issue_ref: Option<&'a str>,
    pub role: &'a str,
    pub body: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomResolveRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub db_path: &'a Path,
    pub issue_ref: Option<&'a str>,
    pub finding: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomMergeRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub db_path: &'a Path,
    pub issue_ref: Option<&'a str>,
    pub role: &'a str,
}

pub fn ensure_room_mode(repo_root: &Path) -> Result<()> {
    let config_path = repo_root.join(".atelier/config.toml");
    match ProjectConfig::load(repo_root)?.review {
        ReviewConfig::Room => Ok(()),
        ReviewConfig::Provider(_) => bail!(
            "review_mode_invalid: {} uses review.mode = \"provider\"; native room commands require review.mode = \"room\"",
            config_path.display()
        ),
    }
}

pub fn open(db: &Database, request: RoomOpenRequest<'_>) -> Result<RoomOpenOutcome> {
    ensure_room_mode(request.repo_root)?;
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let policy = workflow_policy::load(request.repo_root)?;
    let resolution = workflow_policy::resolve_branch_lifecycle(&policy, db, &issue_id)?;
    if request.source_branch != resolution.expected_branch
        || request.target_branch != resolution.base_branch
    {
        bail!(
            "review_room_branch_mismatch: requested room branches are {} -> {}, but issue {} expects {} -> {}; rerun `atelier review open --issue {} --source-branch {} --target-branch {}`",
            request.source_branch,
            request.target_branch,
            resolution.owner_id,
            resolution.expected_branch,
            resolution.base_branch,
            resolution.owner_id,
            resolution.expected_branch,
            resolution.base_branch
        );
    }
    if workflow_policy::effective_review_field(db, &issue_id)?.is_some() {
        bail!(
            "review_room_active: issue {} already has a linked review artifact; inspect `atelier review status --issue {}` before opening another review",
            resolution.owner_id,
            resolution.owner_id
        );
    }

    let store = RecordStore::new(request.state_dir);
    let now = Utc::now();
    let review_id = store.allocate_record_id()?;
    let record = ReviewRecord {
        header: RecordHeader {
            kind: "review".to_string(),
            id: review_id.clone(),
            title: request.title.to_string(),
            status: "open".to_string(),
            labels: vec!["review".to_string()],
            relationships: Relationships::default(),
            created_at: now,
            updated_at: now,
        },
        mode: "room".to_string(),
        issue_id: resolution.owner_id.clone(),
        source_branch: request.source_branch.to_string(),
        target_branch: request.target_branch.to_string(),
        events: vec![event(
            1,
            "opened",
            request.role,
            Some(request.body),
            None,
            None,
        )],
    };
    store.write_record_atomic(&Record::Review(record))?;

    let mut owner = store.load_issue_by_id(&resolution.owner_id)?;
    owner.issue.fields.insert(
        REVIEW_FIELD.to_string(),
        json!({
            "kind": "room",
            "id": review_id,
        }),
    );
    owner.issue.updated_at = Utc::now();
    store.write_issue_atomic(&owner)?;
    crate::projection::refresh_after_canonical_write(request.state_dir, request.db_path)?;

    Ok(RoomOpenOutcome {
        issue_id,
        owner_id: resolution.owner_id,
        review_id,
        status: "open".to_string(),
    })
}

pub fn status(db: &Database, request: RoomStatusRequest<'_>) -> Result<RoomStatusOutcome> {
    ensure_room_mode(request.repo_root)?;
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let record = linked_room(db, request.state_dir, &issue_id)?;
    Ok(status_for_record(issue_id, &record))
}

pub fn show(db: &Database, request: RoomStatusRequest<'_>) -> Result<RoomShowOutcome> {
    ensure_room_mode(request.repo_root)?;
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let record = linked_room(db, request.state_dir, &issue_id)?;
    let status = status_for_record(issue_id, &record);
    let events = record.events.iter().map(event_view).collect();
    Ok(RoomShowOutcome {
        status,
        title: record.header.title,
        source_branch: record.source_branch,
        target_branch: record.target_branch,
        events,
    })
}

pub fn comments(db: &Database, request: RoomStatusRequest<'_>) -> Result<Vec<RoomEventView>> {
    ensure_room_mode(request.repo_root)?;
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let record = linked_room(db, request.state_dir, &issue_id)?;
    let resolved = resolved_findings(&record);
    Ok(record
        .events
        .iter()
        .filter(|event| {
            let kind = event.get("kind").and_then(Value::as_str);
            kind == Some("comment")
                || kind == Some("finding")
                    && event
                        .get("id")
                        .and_then(Value::as_str)
                        .is_some_and(|id| !resolved.contains(id))
        })
        .map(event_view)
        .collect())
}

pub fn comment(db: &Database, request: RoomCommentRequest<'_>) -> Result<RoomEventOutcome> {
    ensure_room_mode(request.repo_root)?;
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let mut record = linked_room(db, request.state_dir, &issue_id)?;
    if record.header.status == "merged" {
        bail!(
            "review_room_closed: review room {} is already merged",
            record.header.id
        );
    }
    let severity = if request.finding {
        Some(validate_severity(request.severity.unwrap_or("blocking"))?)
    } else if request.severity.is_some() {
        bail!("review_room_invalid: --severity requires --finding");
    } else {
        None
    };
    let kind = if request.finding {
        "finding"
    } else {
        "comment"
    };
    let next = next_event_number(&record);
    let event_id = event_id(next);
    record.events.push(event(
        next,
        kind,
        request.role,
        Some(request.body),
        severity,
        None,
    ));
    write_room(request.state_dir, request.db_path, record)?;
    Ok(RoomEventOutcome {
        issue_id: issue_id.clone(),
        review_id: linked_room_id(db, &issue_id)?,
        event_id,
        status: "open".to_string(),
    })
}

pub fn approve(db: &Database, request: RoomDecisionRequest<'_>) -> Result<RoomEventOutcome> {
    append_decision(db, request, "approval")
}

pub fn request_changes(
    db: &Database,
    request: RoomDecisionRequest<'_>,
) -> Result<RoomEventOutcome> {
    append_decision(db, request, "changes_requested")
}

pub fn resolve(db: &Database, request: RoomResolveRequest<'_>) -> Result<RoomEventOutcome> {
    ensure_room_mode(request.repo_root)?;
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let mut record = linked_room(db, request.state_dir, &issue_id)?;
    if record.header.status == "merged" {
        bail!(
            "review_room_closed: review room {} is already merged",
            record.header.id
        );
    }
    let unresolved = unresolved_findings(&record);
    if !unresolved
        .iter()
        .any(|finding| finding.id == request.finding)
    {
        bail!(
            "review_room_finding_missing: unresolved finding {} was not found in room {}",
            request.finding,
            record.header.id
        );
    }
    let next = next_event_number(&record);
    let event_id = event_id(next);
    record.events.push(event(
        next,
        "resolved",
        &current_actor(),
        None,
        None,
        Some(request.finding),
    ));
    write_room(request.state_dir, request.db_path, record)?;
    Ok(RoomEventOutcome {
        issue_id: issue_id.clone(),
        review_id: linked_room_id(db, &issue_id)?,
        event_id,
        status: "open".to_string(),
    })
}

pub fn merge(db: &Database, request: RoomMergeRequest<'_>) -> Result<RoomEventOutcome> {
    ensure_room_mode(request.repo_root)?;
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let mut record = linked_room(db, request.state_dir, &issue_id)?;
    if record.header.status == "merged" {
        return Ok(RoomEventOutcome {
            issue_id,
            review_id: record.header.id,
            event_id: "merged".to_string(),
            status: "merged".to_string(),
        });
    }
    let summary = summarize(&record);
    if summary.current_approvals == 0 {
        bail!(
            "review_room_not_approved: room {} needs a current approval after the latest request-changes event",
            record.header.id
        );
    }
    if summary.unresolved_blocking > 0 {
        bail!(
            "review_room_blocking_findings: room {} has {} unresolved blocking finding(s)",
            record.header.id,
            summary.unresolved_blocking
        );
    }
    let next = next_event_number(&record);
    let event_id = event_id(next);
    record
        .events
        .push(event(next, "merged", request.role, None, None, None));
    record.header.status = "merged".to_string();
    record.header.updated_at = Utc::now();
    let review_id = record.header.id.clone();
    write_room(request.state_dir, request.db_path, record)?;
    Ok(RoomEventOutcome {
        issue_id,
        review_id,
        event_id,
        status: "merged".to_string(),
    })
}

fn append_decision(
    db: &Database,
    request: RoomDecisionRequest<'_>,
    kind: &str,
) -> Result<RoomEventOutcome> {
    ensure_room_mode(request.repo_root)?;
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let mut record = linked_room(db, request.state_dir, &issue_id)?;
    if record.header.status == "merged" {
        bail!(
            "review_room_closed: review room {} is already merged",
            record.header.id
        );
    }
    let next = next_event_number(&record);
    let event_id = event_id(next);
    record.events.push(event(
        next,
        kind,
        request.role,
        Some(request.body),
        None,
        None,
    ));
    write_room(request.state_dir, request.db_path, record)?;
    Ok(RoomEventOutcome {
        issue_id: issue_id.clone(),
        review_id: linked_room_id(db, &issue_id)?,
        event_id,
        status: "open".to_string(),
    })
}

fn write_room(state_dir: &Path, db_path: &Path, record: ReviewRecord) -> Result<()> {
    let mut record = record;
    record.header.updated_at = Utc::now();
    RecordStore::new(state_dir).write_record_atomic(&Record::Review(record))?;
    crate::projection::refresh_after_canonical_write(state_dir, db_path)
}

fn linked_room(db: &Database, state_dir: &Path, issue_id: &str) -> Result<ReviewRecord> {
    let review_id = linked_room_id(db, issue_id)?;
    match RecordStore::new(state_dir).load_record_by_id("review", &review_id)? {
        Record::Review(record) => Ok(record),
        other => bail!(
            "review_room_invalid: expected review record, found {}",
            other.kind()
        ),
    }
}

fn linked_room_id(db: &Database, issue_id: &str) -> Result<String> {
    let field = workflow_policy::effective_review_field(db, issue_id)?.ok_or_else(|| {
        anyhow!(
            "review_room_missing: issue {} has no linked review room; run `atelier review open --issue {}` first",
            issue_id,
            issue_id
        )
    })?;
    field
        .as_object()
        .filter(|object| object.get("kind").and_then(Value::as_str) == Some("room"))
        .and_then(|object| object.get("id"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| anyhow!("review_mode_invalid: linked review field is not a native room"))
}

fn status_for_record(issue_id: String, record: &ReviewRecord) -> RoomStatusOutcome {
    let summary = summarize(record);
    RoomStatusOutcome {
        issue_id,
        review_id: record.header.id.clone(),
        status: record.header.status.clone(),
        approvals: summary.current_approvals,
        unresolved_blocking: summary.unresolved_blocking,
        unresolved_nonblocking: summary.unresolved_nonblocking,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Finding {
    id: String,
    severity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Summary {
    current_approvals: usize,
    unresolved_blocking: usize,
    unresolved_nonblocking: usize,
}

fn summarize(record: &ReviewRecord) -> Summary {
    let latest_change_request = record
        .events
        .iter()
        .enumerate()
        .filter_map(|(index, event)| {
            (event.get("kind").and_then(Value::as_str) == Some("changes_requested"))
                .then_some(index + 1)
        })
        .max()
        .unwrap_or(0);
    let current_approvals = record
        .events
        .iter()
        .enumerate()
        .filter(|(index, event)| {
            index + 1 > latest_change_request
                && event.get("kind").and_then(Value::as_str) == Some("approval")
        })
        .count();
    let unresolved = unresolved_findings(record);
    Summary {
        current_approvals,
        unresolved_blocking: unresolved
            .iter()
            .filter(|finding| finding.severity == "blocking")
            .count(),
        unresolved_nonblocking: unresolved
            .iter()
            .filter(|finding| finding.severity == "nonblocking")
            .count(),
    }
}

fn unresolved_findings(record: &ReviewRecord) -> Vec<Finding> {
    let resolved = resolved_findings(record);
    record
        .events
        .iter()
        .filter(|event| event.get("kind").and_then(Value::as_str) == Some("finding"))
        .filter_map(|event| {
            let id = event.get("id").and_then(Value::as_str)?;
            if resolved.contains(id) {
                return None;
            }
            Some(Finding {
                id: id.to_string(),
                severity: event
                    .get("severity")
                    .and_then(Value::as_str)
                    .unwrap_or("blocking")
                    .to_string(),
            })
        })
        .collect()
}

fn resolved_findings(record: &ReviewRecord) -> BTreeSet<String> {
    record
        .events
        .iter()
        .filter(|event| event.get("kind").and_then(Value::as_str) == Some("resolved"))
        .filter_map(|event| event.get("finding").and_then(Value::as_str))
        .map(str::to_string)
        .collect()
}

fn event_view(event: &Value) -> RoomEventView {
    RoomEventView {
        id: event
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        kind: event
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        actor: event
            .get("actor")
            .and_then(Value::as_str)
            .map(str::to_string),
        body: event
            .get("body")
            .and_then(Value::as_str)
            .map(str::to_string),
        severity: event
            .get("severity")
            .and_then(Value::as_str)
            .map(str::to_string),
        finding: event
            .get("finding")
            .and_then(Value::as_str)
            .map(str::to_string),
    }
}

fn event(
    number: usize,
    kind: &str,
    actor: &str,
    body: Option<&str>,
    severity: Option<&str>,
    finding: Option<&str>,
) -> Value {
    let mut value = json!({
        "id": event_id(number),
        "kind": kind,
        "actor": actor,
        "created_at": Utc::now().to_rfc3339(),
    });
    if let Some(body) = body.filter(|body| !body.trim().is_empty()) {
        value["body"] = json!(body);
    }
    if let Some(severity) = severity {
        value["severity"] = json!(severity);
    }
    if let Some(finding) = finding {
        value["finding"] = json!(finding);
    }
    value
}

fn event_id(number: usize) -> String {
    format!("evt-{number:04}")
}

fn next_event_number(record: &ReviewRecord) -> usize {
    record.events.len() + 1
}

fn validate_severity(value: &str) -> Result<&str> {
    match value {
        "blocking" | "nonblocking" => Ok(value),
        other => bail!(
            "review_room_invalid_severity: expected blocking or nonblocking, got '{}'",
            other
        ),
    }
}

fn infer_issue_id(db: &Database, repo_root: &Path, issue_ref: Option<&str>) -> Result<String> {
    if let Some(issue_ref) = issue_ref {
        return db
            .resolve_issue_ref(issue_ref)?
            .ok_or_else(|| anyhow!("Issue {issue_ref} was not found"));
    }
    if let Some(issue_id) = issue_from_current_owner_branch(db, repo_root)? {
        return Ok(issue_id);
    }
    let active = db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| {
            issue.status == "in_progress"
                || issue.status == "review"
                || issue.status == "validation"
        })
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    match active.as_slice() {
        [one] => Ok(one.clone()),
        [] => bail!("review_target_missing: pass --issue <id> or run from an owner branch"),
        _ => bail!(
            "review_target_ambiguous: multiple active issues found ({}); pass --issue <id>",
            active.join(", ")
        ),
    }
}

fn issue_from_current_owner_branch(db: &Database, repo_root: &Path) -> Result<Option<String>> {
    let branch = current_branch(repo_root)?;
    let policy = workflow_policy::load(repo_root)?;
    let mut owners = BTreeSet::new();
    for issue in db.list_issues(Some("all"), None, None)? {
        if let Ok(resolution) = workflow_policy::resolve_branch_lifecycle(&policy, db, &issue.id) {
            if resolution.expected_branch == branch {
                owners.insert(resolution.owner_id);
            }
        }
    }
    match owners.len() {
        0 => Ok(None),
        1 => Ok(owners.into_iter().next()),
        _ => bail!(
            "review_target_ambiguous: current owner branch {} matches multiple owners ({}); pass --issue <id>",
            branch,
            owners.into_iter().collect::<Vec<_>>().join(", ")
        ),
    }
}

fn current_branch(repo_root: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["branch", "--show-current"])
        .output()
        .context("failed to inspect current git branch")?;
    if !output.status.success() {
        bail!("failed to inspect current git branch");
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn current_actor() -> String {
    std::env::var("ATELIER_AGENT")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "agent".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_core::{Issue, IssueSections};
    use tempfile::tempdir;

    fn setup_repo() -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".atelier")).unwrap();
        std::fs::write(
            dir.path().join(".atelier/config.toml"),
            r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier"

[paths]
state_root = ".atelier"
runtime_dir = ".atelier/.runtime"
runtime_database = "atelier.db"
cache_dir = ".atelier/.cache"

[review]
mode = "room"
"#,
        )
        .unwrap();
        std::fs::write(
            dir.path().join(".atelier/workflow.yaml"),
            workflow_policy::STARTER_POLICY_YAML,
        )
        .unwrap();
        Command::new("git")
            .arg("init")
            .current_dir(dir.path())
            .output()
            .unwrap();
        let db = Database::open(&dir.path().join("runtime/atelier.db")).unwrap();
        insert_issue(
            &db,
            &dir.path().join(".atelier"),
            "atelier-issue",
            "feature",
            "todo",
            None,
        );
        (dir, db)
    }

    fn insert_issue(
        db: &Database,
        state_dir: &Path,
        id: &str,
        issue_type: &str,
        status: &str,
        parent_id: Option<&str>,
    ) {
        let now = Utc::now();
        db.insert_issue_rebuild(&Issue {
            id: id.to_string(),
            title: id.to_string(),
            description: Some("body".to_string()),
            status: status.to_string(),
            issue_type: issue_type.to_string(),
            priority: "medium".to_string(),
            fields: Default::default(),
            parent_id: parent_id.map(str::to_string),
            created_at: now,
            updated_at: now,
            closed_at: None,
        })
        .unwrap();
        let record = atelier_records::CanonicalIssueRecord {
            issue: db.require_issue(id).unwrap(),
            labels: Vec::new(),
            sections: IssueSections::unchecked_from_body(Some(
                "## Description\n\nbody\n\n## Outcome\n\nworks\n\n## Evidence\n\nproof",
            )),
            relationships: Relationships::default(),
        };
        RecordStore::new(state_dir)
            .write_issue_atomic(&record)
            .unwrap();
    }

    #[test]
    fn room_merge_requires_current_approval_and_resolved_blocking_findings() {
        let (dir, db) = setup_repo();
        let state_dir = dir.path().join(".atelier");
        let db_path = dir.path().join("runtime/atelier.db");
        let open = open(
            &db,
            RoomOpenRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                role: "worker",
                title: "Review atelier-issue",
                body: "",
                source_branch: "codex/atelier-issue",
                target_branch: "main",
            },
        )
        .unwrap();
        assert_eq!(open.status, "open");
        let db = Database::open(&db_path).unwrap();
        let finding = comment(
            &db,
            RoomCommentRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                role: "reviewer",
                body: "Fix this",
                finding: true,
                severity: Some("blocking"),
            },
        )
        .unwrap();
        approve(
            &db,
            RoomDecisionRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                role: "reviewer",
                body: "looks good",
            },
        )
        .unwrap();
        let error = merge(
            &db,
            RoomMergeRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                role: "manager",
            },
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("review_room_blocking_findings"));
        resolve(
            &db,
            RoomResolveRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                finding: &finding.event_id,
            },
        )
        .unwrap();
        request_changes(
            &db,
            RoomDecisionRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                role: "reviewer",
                body: "new changes needed",
            },
        )
        .unwrap();
        let error = merge(
            &db,
            RoomMergeRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                role: "manager",
            },
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("review_room_not_approved"));
        approve(
            &db,
            RoomDecisionRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                role: "validator",
                body: "current approval",
            },
        )
        .unwrap();
        let merged = merge(
            &db,
            RoomMergeRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                role: "manager",
            },
        )
        .unwrap();
        assert_eq!(merged.status, "merged");
    }
}
