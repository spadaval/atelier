use anyhow::{anyhow, bail, Result};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::activity::list_issue_activities;
use crate::db::Database;
use crate::models::{Comment, Issue};
use crate::utils::format_issue_id;

#[derive(Debug, Clone, Serialize)]
pub struct IssueSummary {
    pub id: String,
    pub title: String,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
    pub labels: Vec<String>,
    pub parent: Option<String>,
}

struct QueueRow {
    id: String,
    title: String,
    status: String,
    issue_type: String,
    priority: String,
    parent: Option<String>,
    open_blockers: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DependencySummary {
    pub id: String,
    pub title: String,
    pub status: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IssueObject {
    pub id: String,
    pub canonical_id: String,
    pub title: String,
    pub description: Option<String>,
    pub acceptance_criteria: Option<String>,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
    pub labels: Vec<String>,
    pub parent: Option<String>,
    pub dependencies: Vec<DependencySummary>,
    pub dependents: Vec<DependencySummary>,
    pub notes: Vec<NoteObject>,
    pub assignee: Option<String>,
    pub owner: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub started_at: Option<String>,
    pub closed_at: Option<String>,
    pub close_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NoteObject {
    pub id: String,
    pub author: Option<String>,
    pub kind: String,
    pub created_at: String,
    pub body: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    NotFound,
    InvalidInput,
    InvalidDependency,
    Blocked,
    StaleExport,
    SchemaMismatch,
    DirtyTracker,
    StorageError,
}

impl ErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorCode::NotFound => "not_found",
            ErrorCode::InvalidInput => "invalid_input",
            ErrorCode::InvalidDependency => "invalid_dependency",
            ErrorCode::Blocked => "blocked",
            ErrorCode::StaleExport => "stale_export",
            ErrorCode::SchemaMismatch => "schema_mismatch",
            ErrorCode::DirtyTracker => "dirty_tracker",
            ErrorCode::StorageError => "storage_error",
        }
    }
}

pub fn print_success(command: &str, data: Value) -> Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "ok": true,
            "command": command,
            "data": data,
            "warnings": []
        }))?
    );
    Ok(())
}

pub fn print_error(command: &str, code: ErrorCode, message: &str, details: Value) -> Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "ok": false,
            "command": command,
            "error": {
                "code": code.as_str(),
                "message": message,
                "details": details
            }
        }))?
    );
    Ok(())
}

pub fn resolve_id(db: &Database, issue_ref: &str) -> Result<String> {
    db.resolve_issue_ref(issue_ref)?
        .ok_or_else(|| anyhow!("Issue {issue_ref} was not found"))
}

fn issue_id_for_agent(db: &Database, issue: &Issue) -> Result<String> {
    let _ = db;
    Ok(format_issue_id(&issue.id))
}

fn label_value(labels: &[String], prefix: &str) -> Option<String> {
    labels
        .iter()
        .find_map(|label| label.strip_prefix(prefix).map(str::to_string))
}

fn split_acceptance(description: Option<&str>) -> (Option<String>, Option<String>) {
    let Some(description) = description else {
        return (None, None);
    };
    let marker = "## Acceptance Criteria";
    if let Some(idx) = description.find(marker) {
        let before = description[..idx].trim();
        let after = description[idx + marker.len()..].trim();
        let acceptance = after
            .split("\n## ")
            .next()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        let body = if before.is_empty() {
            None
        } else {
            Some(before.to_string())
        };
        (body, acceptance)
    } else {
        (Some(description.to_string()), None)
    }
}

fn note_object(comment: Comment) -> NoteObject {
    let author = comment
        .content
        .strip_prefix("Author: ")
        .and_then(|rest| rest.lines().next())
        .map(str::to_string);
    NoteObject {
        id: comment.id.to_string(),
        author,
        kind: comment.kind,
        created_at: comment.created_at.to_rfc3339(),
        body: comment.content,
    }
}

fn comment_metadata_value(comments: &[Comment], key: &str) -> Option<String> {
    let prefix = format!("{key}:");
    comments.iter().find_map(|comment| {
        comment.content.lines().find_map(|line| {
            line.strip_prefix(&prefix)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string)
        })
    })
}

fn dependency_summary(db: &Database, id: &str) -> Result<DependencySummary> {
    let issue = db
        .get_issue(id)?
        .ok_or_else(|| anyhow!("Dependency issue {} was not found", format_issue_id(id)))?;
    Ok(DependencySummary {
        id: issue_id_for_agent(db, &issue)?,
        title: issue.title,
        status: issue.status,
        priority: issue.priority,
    })
}

pub fn issue_object(db: &Database, issue: Issue) -> Result<IssueObject> {
    let labels = db.get_labels(&issue.id)?;
    let (description, acceptance_criteria) = split_acceptance(issue.description.as_deref());
    let parent = match &issue.parent_id {
        Some(parent_id) => Some(dependency_summary(db, parent_id)?.id),
        None => None,
    };

    let mut dependencies = db
        .get_blockers(&issue.id)?
        .into_iter()
        .map(|id| dependency_summary(db, &id))
        .collect::<Result<Vec<_>>>()?;
    dependencies.sort_by(|a, b| a.id.cmp(&b.id));

    let mut dependents = db
        .get_blocking(&issue.id)?
        .into_iter()
        .map(|id| dependency_summary(db, &id))
        .collect::<Result<Vec<_>>>()?;
    dependents.sort_by(|a, b| a.id.cmp(&b.id));

    let raw_comments = db.get_comments(&issue.id)?;
    let imported_owner = comment_metadata_value(&raw_comments, "owner");
    let imported_assignee = comment_metadata_value(&raw_comments, "assignee");
    let close_reason = comment_metadata_value(&raw_comments, "Close reason")
        .or_else(|| label_value(&labels, "close-reason:"));
    let comments = raw_comments.into_iter().map(note_object).collect();

    Ok(IssueObject {
        id: issue_id_for_agent(db, &issue)?,
        canonical_id: issue.id.clone(),
        title: issue.title,
        description,
        acceptance_criteria,
        status: issue.status,
        issue_type: issue.issue_type,
        priority: issue.priority,
        parent,
        dependencies,
        dependents,
        notes: comments,
        assignee: label_value(&labels, "assignee:").or(imported_assignee),
        owner: label_value(&labels, "owner:").or(imported_owner),
        labels,
        created_at: issue.created_at.to_rfc3339(),
        updated_at: issue.updated_at.to_rfc3339(),
        started_at: None,
        closed_at: issue.closed_at.map(|dt| dt.to_rfc3339()),
        close_reason,
    })
}

fn issue_summary(db: &Database, issue: Issue) -> Result<IssueSummary> {
    let labels = db.get_labels(&issue.id)?;
    Ok(IssueSummary {
        id: issue_id_for_agent(db, &issue)?,
        title: issue.title,
        status: issue.status,
        issue_type: issue.issue_type,
        priority: issue.priority,
        labels,
        parent: issue
            .parent_id
            .map(|id| dependency_summary(db, &id).map(|summary| summary.id))
            .transpose()?,
    })
}

pub fn show(db: &Database, issue_ref: &str, json_output: bool) -> Result<()> {
    let id = resolve_id(db, issue_ref)?;
    let issue = db.require_issue(&id)?;
    let object = issue_object(db, issue)?;
    if json_output {
        print_success("issue.show", serde_json::to_value(object)?)
    } else {
        render_issue_show_human(db, &id, &object)
    }
}

fn render_issue_show_human(db: &Database, canonical_id: &str, object: &IssueObject) -> Result<()> {
    let identity = format!(
        "{} [{}] {} - {}",
        object.id, object.issue_type, object.status, object.title
    );
    println!("{identity}");
    println!("{}", "=".repeat(identity.len()));
    println!("Status:   {}", object.status);
    println!("Type:     {}", object.issue_type);
    println!("Priority: {}", object.priority);
    println!("Created:  {}", object.created_at);
    println!("Updated:  {}", object.updated_at);
    if let Some(closed_at) = &object.closed_at {
        println!("Closed:   {closed_at}");
    }
    if let Some(owner) = &object.owner {
        println!("Owner:    {owner}");
    }
    if let Some(assignee) = &object.assignee {
        println!("Assignee: {assignee}");
    }
    if !object.labels.is_empty() {
        println!("Labels:   {}", object.labels.join(", "));
    }

    render_parent_context(db, canonical_id)?;

    print_text_section("Description", object.description.as_deref());
    print_text_section("Acceptance Criteria", object.acceptance_criteria.as_deref());
    print_text_section("Close Reason", object.close_reason.as_deref());

    render_dependency_section(db, "Blocked by", db.get_blockers(canonical_id)?, true)?;
    render_dependency_section(db, "Blocking", db.get_blocking(canonical_id)?, false)?;
    render_subissue_section(db, canonical_id)?;
    render_recent_activity_section(canonical_id, object)?;
    render_command_footer(object);
    Ok(())
}

fn render_parent_context(db: &Database, canonical_id: &str) -> Result<()> {
    let issue = db.require_issue(canonical_id)?;
    println!("\nHierarchy");
    println!("---------");
    match issue.parent_id {
        Some(parent_id) => {
            let parent = db.require_issue(&parent_id)?;
            println!(
                "Parent: {} [{}] {} - {}",
                format_issue_id(&parent.id),
                parent.status,
                parent.priority,
                parent.title
            );
        }
        None => println!("Parent: (none)"),
    }
    Ok(())
}

fn print_text_section(title: &str, body: Option<&str>) {
    if let Some(body) = body.map(str::trim).filter(|body| !body.is_empty()) {
        println!("\n{title}");
        println!("{}", "-".repeat(title.len()));
        println!("{body}");
    }
}

fn render_dependency_section(
    db: &Database,
    title: &str,
    ids: Vec<String>,
    blockers: bool,
) -> Result<()> {
    println!("\n{title}");
    println!("{}", "-".repeat(title.len()));
    let rows = dependency_rows_for_text(db, ids, blockers)?;
    if rows.is_empty() {
        println!("(none)");
    } else {
        for row in rows {
            println!("  {row}");
        }
    }
    Ok(())
}

fn dependency_rows_for_text(
    db: &Database,
    ids: Vec<String>,
    blockers: bool,
) -> Result<Vec<String>> {
    ids.into_iter()
        .map(|id| {
            let issue = db.require_issue(&id)?;
            let marker = if blockers && issue.status == "open" {
                " OPEN BLOCKER"
            } else {
                ""
            };
            Ok(format!(
                "{} [{}] {} - {}{}",
                issue_id_for_agent(db, &issue)?,
                issue.status,
                issue.priority,
                issue.title,
                marker
            ))
        })
        .collect()
}

fn render_subissue_section(db: &Database, canonical_id: &str) -> Result<()> {
    let mut subissues = db.get_subissues(canonical_id)?;
    println!("\nSubissues");
    println!("---------");
    if subissues.is_empty() {
        println!("(none)");
        return Ok(());
    }

    println!("{}", subissue_summary(&subissues));
    subissues.sort_by(|a, b| {
        status_rank(&a.status)
            .cmp(&status_rank(&b.status))
            .then(priority_rank(&a.priority).cmp(&priority_rank(&b.priority)))
            .then(a.id.cmp(&b.id))
            .then(a.title.cmp(&b.title))
    });
    for subissue in subissues {
        println!(
            "  {} [{}] {} - {}",
            format_issue_id(&subissue.id),
            subissue.status,
            subissue.priority,
            subissue.title
        );
    }
    Ok(())
}

fn subissue_summary(subissues: &[Issue]) -> String {
    let mut statuses = BTreeMap::<String, usize>::new();
    let mut priorities = BTreeMap::<String, usize>::new();
    for subissue in subissues {
        *statuses.entry(subissue.status.clone()).or_default() += 1;
        *priorities.entry(subissue.priority.clone()).or_default() += 1;
    }
    format!(
        "{} total | status: {} | priority: {}",
        subissues.len(),
        joined_counts(statuses),
        joined_counts(priorities)
    )
}

fn joined_counts(counts: BTreeMap<String, usize>) -> String {
    counts
        .into_iter()
        .map(|(name, count)| format!("{name}={count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn status_rank(status: &str) -> u8 {
    match status {
        "open" => 0,
        "in_progress" => 1,
        "blocked" => 2,
        "closed" => 3,
        _ => 4,
    }
}

fn priority_rank(priority: &str) -> u8 {
    match priority {
        "critical" => 0,
        "high" => 1,
        "medium" => 2,
        "low" => 3,
        _ => 4,
    }
}

fn render_recent_activity_section(canonical_id: &str, object: &IssueObject) -> Result<()> {
    println!("\nRecent Activity");
    println!("---------------");
    let activity = recent_activity_lines(canonical_id, object)?;
    if activity.is_empty() {
        println!("(none)");
        return Ok(());
    }
    for line in activity {
        println!("  {line}");
    }
    Ok(())
}

fn render_command_footer(object: &IssueObject) {
    println!("\nNext Commands");
    println!("-------------");
    println!("  atelier issue comment {} \"...\"", object.id);
    if object.status == "closed" {
        println!("  atelier issue reopen {}", object.id);
    } else {
        println!("  atelier work start {}", object.id);
        println!("  atelier issue close {} --reason \"...\"", object.id);
    }
}

fn recent_activity_lines(canonical_id: &str, object: &IssueObject) -> Result<Vec<String>> {
    if let Some(state_dir) = find_state_dir_from_cwd()? {
        let activities = list_issue_activities(&state_dir, canonical_id)?;
        if !activities.is_empty() {
            return Ok(activities
                .iter()
                .rev()
                .take(5)
                .map(|activity| {
                    let body = activity.body.replace('\n', "\n  ");
                    if body.trim().is_empty() {
                        format!(
                            "[{}] {}: {}",
                            activity.created_at.to_rfc3339(),
                            activity.event_type,
                            activity.summary
                        )
                    } else {
                        format!(
                            "[{}] {}: {}\n  {}",
                            activity.created_at.to_rfc3339(),
                            activity.event_type,
                            activity.summary,
                            body
                        )
                    }
                })
                .collect());
        }
    }

    Ok(object
        .notes
        .iter()
        .rev()
        .take(5)
        .map(|note| {
            format!(
                "[{}] {}: {}",
                note.created_at,
                note.kind,
                note.body.replace('\n', "\n  ")
            )
        })
        .collect())
}

fn find_state_dir_from_cwd() -> Result<Option<PathBuf>> {
    let mut current = std::env::current_dir()?;
    loop {
        let state_dir = current.join(".atelier-state");
        if state_dir.is_dir() {
            return Ok(Some(state_dir));
        }
        if current.join(".atelier").is_dir() {
            return Ok(None);
        }
        if !current.pop() {
            return Ok(None);
        }
    }
}

pub fn list(
    db: &Database,
    status: Option<&str>,
    label: Option<&str>,
    priority: Option<&str>,
    json_output: bool,
    quiet: bool,
) -> Result<()> {
    let items = db
        .list_issues(status, label, priority)?
        .into_iter()
        .map(|issue| issue_summary(db, issue))
        .collect::<Result<Vec<_>>>()?;
    if json_output {
        print_success(
            "issue.list",
            json!({
                "items": items,
                "count": items.len(),
                "filters": {
                    "status": status,
                    "label": label,
                    "priority": priority
                }
            }),
        )
    } else if items.is_empty() {
        println!("No issues found.");
        Ok(())
    } else if quiet {
        render_issue_ids_quiet(items);
        Ok(())
    } else {
        render_issue_queue_human(db, "Issue Queue", items, true)
    }
}

pub fn ready(db: &Database, json_output: bool, quiet: bool) -> Result<()> {
    let items = db
        .list_ready_issues()?
        .into_iter()
        .map(|issue| issue_summary(db, issue))
        .collect::<Result<Vec<_>>>()?;
    if json_output {
        print_success(
            "issue.ready",
            json!({ "items": items, "count": items.len() }),
        )
    } else if items.is_empty() {
        let blocked_count = db.list_blocked_issues()?.len();
        println!("No issues ready to work on ({} blocked).", blocked_count);
        Ok(())
    } else if quiet {
        render_issue_ids_quiet(items);
        Ok(())
    } else {
        render_issue_queue_human(db, "Ready Issues", items, false)
    }
}

pub fn search(db: &Database, query: &str, json_output: bool, quiet: bool) -> Result<()> {
    let lowercase = query.to_lowercase();
    let mut items = Vec::new();
    for issue in db.list_issues(Some("all"), None, None)? {
        let haystack = format!(
            "{}\n{}",
            issue.title,
            issue.description.as_deref().unwrap_or_default()
        )
        .to_lowercase();
        if haystack.contains(&lowercase) {
            items.push(issue_summary(db, issue)?);
        }
    }
    if json_output {
        print_success(
            "issue.search",
            json!({ "query": query, "items": items, "count": items.len() }),
        )
    } else if items.is_empty() {
        println!("No issues found matching '{query}'.");
        Ok(())
    } else if quiet {
        render_issue_ids_quiet(items);
        Ok(())
    } else {
        render_issue_queue_human(db, &format!("Search Results: {query}"), items, true)
    }
}

fn render_issue_ids_quiet(items: Vec<IssueSummary>) {
    for item in items {
        println!("{}", item.id);
    }
}

fn render_issue_queue_human(
    db: &Database,
    title: &str,
    items: Vec<IssueSummary>,
    show_status: bool,
) -> Result<()> {
    let mut rows = items
        .into_iter()
        .map(|item| queue_row(db, item))
        .collect::<Result<Vec<_>>>()?;
    rows.sort_by(|a, b| {
        status_rank(&a.status)
            .cmp(&status_rank(&b.status))
            .then(priority_rank(&a.priority).cmp(&priority_rank(&b.priority)))
            .then(a.issue_type.cmp(&b.issue_type))
            .then(a.parent.cmp(&b.parent))
            .then(a.id.cmp(&b.id))
    });

    println!("{title}");
    println!("{}", "=".repeat(title.len()));
    println!("{}", queue_summary(&rows));

    let mut grouped = BTreeMap::<(String, String), Vec<QueueRow>>::new();
    for row in rows {
        let status_group = if show_status {
            row.status.clone()
        } else {
            "ready".to_string()
        };
        grouped
            .entry((status_group, row.priority.clone()))
            .or_default()
            .push(row);
    }

    let group_order = ["open", "ready", "in_progress", "blocked", "closed"];
    let priority_order = ["critical", "high", "medium", "low"];
    for status in group_order {
        for priority in priority_order {
            if let Some(rows) = grouped.remove(&(status.to_string(), priority.to_string())) {
                print_queue_group(status, priority, rows, show_status);
            }
        }
    }
    for ((status, priority), rows) in grouped {
        print_queue_group(&status, &priority, rows, show_status);
    }

    Ok(())
}

fn queue_row(db: &Database, item: IssueSummary) -> Result<QueueRow> {
    let open_blockers = db
        .get_blockers(&item.id)?
        .into_iter()
        .filter_map(|id| db.require_issue(&id).ok())
        .filter(|issue| issue.status == "open")
        .count();
    Ok(QueueRow {
        id: item.id,
        title: item.title,
        status: item.status,
        issue_type: item.issue_type,
        priority: item.priority,
        parent: item.parent,
        open_blockers,
    })
}

fn queue_summary(rows: &[QueueRow]) -> String {
    let mut statuses = BTreeMap::<String, usize>::new();
    let mut priorities = BTreeMap::<String, usize>::new();
    let mut blocked = 0;
    for row in rows {
        *statuses.entry(row.status.clone()).or_default() += 1;
        *priorities.entry(row.priority.clone()).or_default() += 1;
        if row.open_blockers > 0 {
            blocked += 1;
        }
    }
    format!(
        "{} total | status: {} | priority: {} | blocked={}",
        rows.len(),
        joined_counts(statuses),
        joined_counts(priorities),
        blocked
    )
}

fn print_queue_group(status: &str, priority: &str, rows: Vec<QueueRow>, show_status: bool) {
    let heading = format!("{status} {priority}");
    println!("\n{heading}");
    println!("{}", "-".repeat(heading.len()));
    for row in rows {
        let status_text = if show_status {
            format!("[{}] ", row.status)
        } else {
            String::new()
        };
        let parent = row
            .parent
            .as_deref()
            .map(|parent| format!(" parent={parent}"))
            .unwrap_or_default();
        let blockers = if row.open_blockers > 0 {
            format!(" blocked_by={}", row.open_blockers)
        } else {
            String::new()
        };
        println!(
            "  {}{}{} {} - {}{}",
            status_text, row.id, parent, row.issue_type, row.title, blockers
        );
    }
}

pub struct CreateInput<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub priority: &'a str,
    pub issue_type: Option<&'a str>,
    pub labels: &'a [String],
    pub parent: Option<&'a str>,
}

pub fn create(db: &Database, input: CreateInput<'_>, json_output: bool) -> Result<()> {
    validate_priority(input.priority)?;
    let issue_type = input.issue_type.unwrap_or("task");
    crate::db::validate_issue_type(issue_type)?;
    let parent_id = input
        .parent
        .map(|parent| resolve_id(db, parent))
        .transpose()?;
    let id = match parent_id {
        Some(parent_id) => db.create_subissue_with_type(
            &parent_id,
            input.title,
            input.description,
            input.priority,
            issue_type,
        )?,
        None => {
            db.create_issue_with_type(input.title, input.description, input.priority, issue_type)?
        }
    };
    for label in input.labels {
        db.add_label(&id, label)?;
    }
    let issue = db.require_issue(&id)?;
    let object = issue_object(db, issue)?;
    if json_output {
        print_success("issue.create", serde_json::to_value(object)?)
    } else {
        println!("Created issue {}: {}", object.id, object.title);
        Ok(())
    }
}

pub struct UpdateInput<'a> {
    pub issue_ref: &'a str,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub priority: Option<&'a str>,
    pub status: Option<&'a str>,
    pub labels: &'a [String],
    pub parent: Option<Option<&'a str>>,
    pub claim: bool,
    pub append_notes: Option<&'a str>,
}

pub fn update(db: &Database, input: UpdateInput<'_>, json_output: bool) -> Result<()> {
    let id = resolve_id(db, input.issue_ref)?;
    let previous = db.require_issue(&id)?;
    let previous_assignee = label_value(&db.get_labels(&id)?, "assignee:");
    let mut changed_fields = Vec::new();

    if input.title.is_some() || input.description.is_some() || input.priority.is_some() {
        if let Some(priority) = input.priority {
            validate_priority(priority)?;
        }
        if db.update_issue(&id, input.title, input.description, input.priority)? {
            if input.title.is_some() {
                changed_fields.push("title");
                crate::commands::activity_log::record_field_changed(
                    &id,
                    "title",
                    Some(&previous.title),
                    input.title,
                )?;
            }
            if input.description.is_some() {
                changed_fields.push("description");
                crate::commands::activity_log::record_field_changed(
                    &id,
                    "description",
                    previous.description.as_deref(),
                    input.description,
                )?;
            }
            if input.priority.is_some() {
                changed_fields.push("priority");
                crate::commands::activity_log::record_field_changed(
                    &id,
                    "priority",
                    Some(&previous.priority),
                    input.priority,
                )?;
            }
        }
    }

    if let Some(status) = input.status {
        match status {
            "open" => {
                db.reopen_issue(&id)?;
                changed_fields.push("status");
                crate::commands::activity_log::record_status_changed(
                    &id,
                    &previous.status,
                    "open",
                )?;
            }
            "closed" => {
                db.close_issue(&id)?;
                changed_fields.push("status");
                crate::commands::activity_log::record_status_changed(
                    &id,
                    &previous.status,
                    "closed",
                )?;
            }
            "in_progress" => {
                db.add_label(&id, "status:in_progress")?;
                changed_fields.push("status");
                crate::commands::activity_log::record_status_changed(
                    &id,
                    &previous.status,
                    "in_progress",
                )?;
            }
            other => bail!("Invalid status '{other}'. Valid values: open, in_progress, closed"),
        }
    }

    for label in input.labels {
        db.add_label(&id, label)?;
        changed_fields.push("labels");
        crate::commands::activity_log::record_field_changed(&id, "labels", None, Some(label))?;
    }

    if let Some(parent) = input.parent {
        let parent_id = parent.map(|parent| resolve_id(db, parent)).transpose()?;
        db.update_parent(&id, parent_id.as_deref())?;
        changed_fields.push("parent");
        crate::commands::activity_log::record_field_changed(
            &id,
            "parent",
            previous.parent_id.as_deref(),
            parent_id.as_deref(),
        )?;
    }

    if input.claim {
        let assignee = current_actor();
        if let Some(previous_assignee) = &previous_assignee {
            db.remove_label(&id, &format!("assignee:{previous_assignee}"))?;
        }
        db.add_label(&id, &format!("assignee:{assignee}"))?;
        db.add_comment(&id, &format!("Claimed by {assignee}"), "handoff")?;
        changed_fields.push("assignee");
        crate::commands::activity_log::record_field_changed(
            &id,
            "assignee",
            previous_assignee.as_deref(),
            Some(&assignee),
        )?;
    }

    if let Some(note) = input.append_notes {
        db.add_comment(&id, note, "handoff")?;
        changed_fields.push("notes");
        crate::commands::activity_log::record_note(&id, note)?;
    }

    if changed_fields.is_empty() {
        bail!("Nothing to update. Use --title, --description, --priority, --status, --label, --parent, --claim, or --append-notes");
    }
    changed_fields.sort_unstable();
    changed_fields.dedup();

    let issue = db.require_issue(&id)?;
    let object = issue_object(db, issue)?;
    if json_output {
        print_success(
            "issue.update",
            json!({
                "issue": object,
                "previous_assignee": previous_assignee,
                "assignee": object.assignee,
                "changed": previous.updated_at != db.require_issue(&id)?.updated_at || !changed_fields.is_empty(),
                "changed_fields": changed_fields
            }),
        )
    } else {
        println!(
            "Updated issue {} ({})",
            object.id,
            changed_fields.join(", ")
        );
        Ok(())
    }
}

pub fn close(
    db: &Database,
    issue_ref: &str,
    reason: Option<&str>,
    json_output: bool,
) -> Result<()> {
    let id = resolve_id(db, issue_ref)?;
    let open_blockers = db
        .get_blockers(&id)?
        .into_iter()
        .filter_map(|blocker_id| db.get_issue(&blocker_id).ok().flatten())
        .filter(|issue| issue.status == "open")
        .collect::<Vec<_>>();
    if !open_blockers.is_empty() {
        bail!(
            "Issue {} cannot be closed because it has open blockers: {}",
            issue_ref,
            open_blockers
                .iter()
                .map(|issue| format_issue_id(&issue.id))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let previous = db.require_issue(&id)?;
    db.close_issue(&id)?;
    crate::commands::activity_log::record_status_changed(&id, &previous.status, "closed")?;
    if let Some(reason) = reason {
        db.add_comment(&id, &format!("Close reason: {reason}"), "resolution")?;
        crate::commands::activity_log::record_close_reason(&id, reason)?;
    }
    let issue = db.require_issue(&id)?;
    let object = issue_object(db, issue)?;
    if json_output {
        print_success("issue.close", serde_json::to_value(object)?)
    } else {
        println!(
            "Closed issue {}{}",
            object.id,
            reason.map(|r| format!(": {r}")).unwrap_or_default()
        );
        Ok(())
    }
}

pub fn reopen(db: &Database, issue_ref: &str, json_output: bool) -> Result<()> {
    let id = resolve_id(db, issue_ref)?;
    let previous = db.require_issue(&id)?;
    db.reopen_issue(&id)?;
    crate::commands::activity_log::record_status_changed(&id, &previous.status, "open")?;
    let object = issue_object(db, db.require_issue(&id)?)?;
    if json_output {
        print_success("issue.reopen", serde_json::to_value(object)?)
    } else {
        println!("Reopened issue {}", object.id);
        Ok(())
    }
}

pub fn dep_add(
    db: &Database,
    blocked_ref: &str,
    blocker_ref: &str,
    json_output: bool,
) -> Result<()> {
    let blocked_id = resolve_id(db, blocked_ref)?;
    let blocker_id = resolve_id(db, blocker_ref)?;
    let changed = db.add_dependency(&blocked_id, &blocker_id)?;
    dep_result(
        db,
        "dep.add",
        "add",
        &blocked_id,
        &blocker_id,
        changed,
        json_output,
    )
}

pub fn dep_remove(
    db: &Database,
    blocked_ref: &str,
    blocker_ref: &str,
    json_output: bool,
) -> Result<()> {
    let blocked_id = resolve_id(db, blocked_ref)?;
    let blocker_id = resolve_id(db, blocker_ref)?;
    let changed = db.remove_dependency(&blocked_id, &blocker_id)?;
    dep_result(
        db,
        "dep.remove",
        "remove",
        &blocked_id,
        &blocker_id,
        changed,
        json_output,
    )
}

fn dep_result(
    db: &Database,
    command: &str,
    action: &str,
    blocked_id: &str,
    blocker_id: &str,
    changed: bool,
    json_output: bool,
) -> Result<()> {
    let blocked = db.require_issue(blocked_id)?;
    let blocker = db.require_issue(blocker_id)?;
    let data = json!({
        "source": issue_id_for_agent(db, &blocker)?,
        "target": issue_id_for_agent(db, &blocked)?,
        "blocked": issue_id_for_agent(db, &blocked)?,
        "blocker": issue_id_for_agent(db, &blocker)?,
        "type": "blocks",
        "action": action,
        "state": dependency_state(action, changed),
        "changed": changed
    });
    if json_output {
        print_success(command, data)
    } else {
        let blocked = data["blocked"].as_str().unwrap_or_default();
        let blocker = data["blocker"].as_str().unwrap_or_default();
        let state = data["state"].as_str().unwrap_or_default();
        match action {
            "remove" => println!("{blocked} is no longer blocked by {blocker} ({state})"),
            _ => println!("{blocked} is blocked by {blocker} ({state})"),
        }
        Ok(())
    }
}

fn dependency_state(action: &str, changed: bool) -> &'static str {
    match (action, changed) {
        ("add", true) => "added",
        ("add", false) => "already-present",
        ("remove", true) => "removed",
        ("remove", false) => "already-absent",
        _ => "unchanged",
    }
}

pub fn dep_list(db: &Database, issue_ref: Option<&str>, json_output: bool) -> Result<()> {
    let mut edges = Vec::new();
    let issues = if let Some(issue_ref) = issue_ref {
        let id = resolve_id(db, issue_ref)?;
        vec![db.require_issue(&id)?]
    } else {
        db.list_issues(Some("all"), None, None)?
    };
    for issue in issues {
        for blocker_id in db.get_blockers(&issue.id)? {
            let blocker = db.require_issue(&blocker_id)?;
            edges.push(json!({
                "source": issue_id_for_agent(db, &blocker)?,
                "target": issue_id_for_agent(db, &issue)?,
                "blocked": issue_id_for_agent(db, &issue)?,
                "blocker": issue_id_for_agent(db, &blocker)?,
                "type": "blocks"
            }));
        }
    }
    if json_output {
        print_success("dep.list", json!({ "items": edges, "count": edges.len() }))
    } else if edges.is_empty() {
        println!("No dependencies found.");
        Ok(())
    } else {
        for edge in edges {
            println!("{} is blocked by {}", edge["blocked"], edge["blocker"]);
        }
        Ok(())
    }
}

pub fn lint(db: &Database, issue_ref: Option<&str>, json_output: bool) -> Result<()> {
    let issues = if let Some(issue_ref) = issue_ref {
        let id = resolve_id(db, issue_ref)?;
        vec![db.require_issue(&id)?]
    } else {
        db.list_issues(Some("all"), None, None)?
    };
    let mut findings = Vec::new();
    for issue in issues {
        if issue.title.trim().is_empty() {
            findings.push(json!({
                "id": issue_id_for_agent(db, &issue)?,
                "code": "missing_title",
                "message": "Issue title must not be empty"
            }));
        }
        if !crate::db::VALID_ISSUE_TYPES.contains(&issue.issue_type.as_str()) {
            findings.push(json!({
                "id": issue_id_for_agent(db, &issue)?,
                "code": "invalid_issue_type",
                "message": format!("Issue type '{}' is not valid", issue.issue_type)
            }));
        }
        for blocker_id in db.get_blockers(&issue.id)? {
            if db.get_issue(&blocker_id)?.is_none() {
                findings.push(json!({
                    "id": issue_id_for_agent(db, &issue)?,
                    "code": "missing_blocker",
                    "message": format!("Dependency references missing issue {}", format_issue_id(&blocker_id))
                }));
            }
        }
    }
    let data = json!({
        "checked": if issue_ref.is_some() { 1 } else { db.list_issues(Some("all"), None, None)?.len() },
        "findings": findings,
        "finding_count": findings.len()
    });
    if json_output {
        print_success("lint", data)?;
    } else if findings.is_empty() {
        println!("Lint passed.");
    } else {
        println!("Lint found {} issue(s):", findings.len());
        for finding in &findings {
            println!("  {}: {}", finding["id"], finding["message"]);
        }
    }
    if findings.is_empty() {
        Ok(())
    } else {
        bail!("Lint failed with {} finding(s)", findings.len())
    }
}

pub fn doctor(db: &Database, repo_root: &Path, state_dir: &Path, json_output: bool) -> Result<()> {
    let db_path = repo_root.join(".atelier").join("state.db");
    let export_fresh = super::export::canonical_stale_entries(db, state_dir)
        .map(|stale| stale.is_empty())
        .unwrap_or(false);
    let rebuild_ready = super::rebuild::validate_canonical_state(state_dir).is_ok();
    let projection_fresh = crate::projection_index::check(db, state_dir)
        .map(|report| report.is_fresh())
        .unwrap_or(false);
    let mut health = BTreeMap::new();
    health.insert("database", db_path.exists());
    health.insert("projection", state_dir.is_dir());
    health.insert("export_fresh", export_fresh);
    health.insert("projection_fresh", projection_fresh);
    health.insert("rebuild_ready", rebuild_ready);
    if json_output {
        print_success(
            "doctor",
            json!({
                "database_path": db_path,
                "state_path": state_dir,
                "schema_version": 1,
                "health": health
            }),
        )
    } else {
        println!("Database: {}", db_path.display());
        println!("State: {}", state_dir.display());
        for (key, value) in health {
            println!("{key}: {}", if value { "ok" } else { "not ok" });
        }
        Ok(())
    }
}

pub fn export_canonical(
    db: &Database,
    state_dir: &Path,
    check: bool,
    json_output: bool,
) -> Result<()> {
    if check {
        let stale = super::export::canonical_stale_entries(db, state_dir)?;
        if stale.is_empty() {
            if json_output {
                print_success(
                    "export.check",
                    json!({ "fresh": true, "state_path": state_dir, "stale": [] }),
                )
            } else {
                eprintln!("Canonical export is current");
                Ok(())
            }
        } else if json_output {
            print_error(
                "export.check",
                ErrorCode::StaleExport,
                "Canonical export is stale; run `atelier export` before handoff",
                json!({ "state_path": state_dir, "stale": stale }),
            )?;
            std::process::exit(1);
        } else {
            bail!("Canonical export is stale:\n{}", stale.join("\n"))
        }
    } else {
        super::export::run_canonical(db, state_dir, false)?;
        if json_output {
            print_success(
                "export",
                json!({ "state_path": state_dir, "written": true }),
            )
        } else {
            Ok(())
        }
    }
}

pub fn rebuild(state_dir: &Path, db_path: &Path, json_output: bool) -> Result<()> {
    super::rebuild::run(state_dir, db_path)?;
    if json_output {
        print_success(
            "rebuild",
            json!({ "state_path": state_dir, "database_path": db_path, "rebuilt": true }),
        )
    } else {
        Ok(())
    }
}

pub fn classify_error(error: &anyhow::Error) -> ErrorCode {
    let message = error.to_string();
    if message.contains("not found") || message.contains("was not found") {
        ErrorCode::NotFound
    } else if message.contains("block") && message.contains("open blockers") {
        ErrorCode::Blocked
    } else if message.contains("dependency")
        || message.contains("block itself")
        || message.contains("circular")
    {
        ErrorCode::InvalidDependency
    } else if message.contains("stale") {
        ErrorCode::StaleExport
    } else if message.contains("schema") || message.contains("manifest") {
        ErrorCode::SchemaMismatch
    } else if message.contains("dirty") {
        ErrorCode::DirtyTracker
    } else if message.contains("Invalid") || message.contains("Nothing to update") {
        ErrorCode::InvalidInput
    } else {
        ErrorCode::StorageError
    }
}

pub fn validate_priority(priority: &str) -> Result<()> {
    if crate::db::VALID_PRIORITIES.contains(&priority) {
        Ok(())
    } else {
        bail!(
            "Invalid priority '{}'. Valid values: {}",
            priority,
            crate::db::VALID_PRIORITIES.join(", ")
        )
    }
}

fn current_actor() -> String {
    std::env::var("ATELIER_AGENT")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "agent".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn setup_test_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db = Database::open(&dir.path().join("test.db")).unwrap();
        (db, dir)
    }

    #[test]
    fn dependency_rows_include_context_and_open_blocker_marker() {
        let (db, _dir) = setup_test_db();
        let blocked = db.create_issue("Blocked issue", None, "medium").unwrap();
        let blocker = db.create_issue("Blocking issue", None, "high").unwrap();
        db.add_dependency(&blocked, &blocker).unwrap();

        let rows = dependency_rows_for_text(&db, db.get_blockers(&blocked).unwrap(), true).unwrap();

        assert_eq!(rows.len(), 1);
        assert!(rows[0].contains(&format_issue_id(&blocker)));
        assert!(rows[0].contains("[open] high - Blocking issue"));
        assert!(rows[0].contains("OPEN BLOCKER"));
    }

    #[test]
    fn subissue_summary_counts_statuses_and_priorities() {
        let (db, _dir) = setup_test_db();
        let parent = db.create_issue("Parent", None, "high").unwrap();
        let child_a = db
            .create_subissue(&parent, "First child", None, "high")
            .unwrap();
        let child_b = db
            .create_subissue(&parent, "Second child", None, "low")
            .unwrap();
        db.close_issue(&child_b).unwrap();

        let subissues = db.get_subissues(&parent).unwrap();
        let summary = subissue_summary(&subissues);

        assert!(summary.contains("2 total"));
        assert!(summary.contains("closed=1"));
        assert!(summary.contains("open=1"));
        assert!(summary.contains("high=1"));
        assert!(summary.contains("low=1"));
        assert!(subissues.iter().any(|issue| issue.id == child_a));
    }
}
