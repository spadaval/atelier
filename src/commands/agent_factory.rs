use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, Local, Utc};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::activity::{list_issue_activities, ActivityEventType};
use crate::db::Database;
use crate::models::{Comment, DomainRecord, Issue};
use crate::record_store::{
    issue_record_path, issue_section_diagnostic, CanonicalIssueRecord, IssueSectionName,
    IssueSections, RecordStore, RelationshipTarget, Relationships,
};
use crate::utils::format_issue_id;
use crate::workflow_policy::WorkflowPolicy;

#[derive(Debug, Clone)]
pub struct IssueSummary {
    pub id: String,
    pub title: String,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
    pub parent: Option<String>,
}

#[derive(Debug, Clone)]
struct QueueRow {
    id: String,
    title: String,
    status: String,
    status_category: Option<String>,
    issue_type: String,
    priority: String,
    parent: Option<String>,
    open_blockers: Vec<String>,
    depth: usize,
}

#[derive(Debug, Clone)]
struct QueueGroup {
    id: Option<String>,
    title: String,
    issue_type: Option<String>,
    priority: Option<String>,
    external_blockers: Vec<String>,
    rows: Vec<QueueRow>,
}

struct DependencyListRow {
    blocked_id: String,
    blocked_title: String,
    blocked_status: String,
    blocked_priority: String,
    blocker_id: String,
    blocker_title: String,
    blocker_status: String,
    blocker_priority: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IssueStartReadiness {
    Ready,
    Blocked,
    NotReady,
}

#[derive(Debug, Clone)]
enum IssueStatusFilter {
    All,
    Exact(String),
    Category(String),
}

#[derive(Debug, Clone, Copy, Default)]
struct WorkflowReadContext {
    missing_policy: bool,
    unmigrated_filter: bool,
}

pub(crate) fn load_issue_workflow_policy() -> Result<Option<WorkflowPolicy>> {
    let repo_root = crate::storage_layout::find_repo_root()?;
    let policy_path = repo_root.join(crate::workflow_policy::WORKFLOW_POLICY_PATH);
    if !policy_path.exists() {
        return Ok(None);
    }
    crate::workflow_policy::load(&repo_root).map(Some)
}

impl IssueStatusFilter {
    fn from_input(policy: Option<&WorkflowPolicy>, input: &str) -> Self {
        if input == "all" {
            return Self::All;
        }
        let category = operator_category_filter(input);
        if policy.is_none() {
            return category
                .map(Self::Category)
                .unwrap_or_else(|| Self::Exact(input.to_string()));
        }
        if let Some(category) = category {
            return Self::Category(category);
        }
        if let Some(policy) = policy {
            if policy.statuses.contains_key(input) {
                return Self::Exact(input.to_string());
            }
            if policy
                .statuses
                .values()
                .any(|status| status.category == input)
            {
                return Self::Category(input.to_string());
            }
        }
        Self::Exact(input.to_string())
    }

    fn matches(
        &self,
        policy: Option<&WorkflowPolicy>,
        issue: &Issue,
        context: &mut WorkflowReadContext,
    ) -> bool {
        match self {
            Self::All => true,
            Self::Exact(status) => {
                if policy.is_some()
                    && !policy
                        .and_then(|policy| policy.status_category(status))
                        .is_some()
                    && issue.status == *status
                {
                    context.unmigrated_filter = true;
                }
                issue.status == *status
            }
            Self::Category(category) => {
                let issue_category = workflow_category(policy, &issue.status);
                if policy.is_some()
                    && issue_category.is_some()
                    && !policy_status_known(policy, &issue.status)
                {
                    context.unmigrated_filter = true;
                }
                issue_category.as_deref() == Some(category.as_str())
            }
        }
    }
}

fn policy_status_known(policy: Option<&WorkflowPolicy>, status: &str) -> bool {
    policy
        .and_then(|policy| policy.status_category(status))
        .is_some()
}

fn operator_category_filter(input: &str) -> Option<String> {
    match input {
        "todo" => Some("todo".to_string()),
        "in_progress" => Some("active".to_string()),
        "active" => Some("active".to_string()),
        "blocked" => Some("blocked".to_string()),
        "review" => Some("review".to_string()),
        "validation" => Some("validation".to_string()),
        "done" => Some("done".to_string()),
        _ => None,
    }
}

pub(crate) fn issue_status_category(
    policy: Option<&WorkflowPolicy>,
    status: &str,
) -> Option<String> {
    policy
        .and_then(|policy| policy.status_category(status))
        .map(operator_category_label)
}

fn workflow_category(policy: Option<&WorkflowPolicy>, status: &str) -> Option<String> {
    policy
        .and_then(|policy| policy.status_category(status))
        .map(str::to_string)
}

fn operator_category_label(category: &str) -> String {
    match category {
        "active" => "in_progress".to_string(),
        other => other.to_string(),
    }
}

pub(crate) fn issue_status_label(policy: Option<&WorkflowPolicy>, status: &str) -> String {
    format_status_with_category(issue_status_category(policy, status).as_deref(), status)
}

fn format_status_with_category(category: Option<&str>, status: &str) -> String {
    match category {
        Some(category) => format!("{category}/{status}"),
        None => format!("unknown/{status}"),
    }
}

pub(crate) fn issue_is_done(policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    workflow_category(policy, &issue.status).as_deref() == Some("done")
}

pub(crate) fn issue_blocks_work(policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    !issue_is_done(policy, issue)
}

pub(crate) fn issue_start_readiness(
    db: &Database,
    policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> Result<IssueStartReadiness> {
    if policy.is_none() {
        return if workflow_category(None, &issue.status).as_deref() == Some("todo") {
            if open_blocker_ids_with_policy(db, None, &issue.id)?.is_empty() {
                Ok(IssueStartReadiness::Ready)
            } else {
                Ok(IssueStartReadiness::Blocked)
            }
        } else {
            Ok(IssueStartReadiness::NotReady)
        };
    }
    let Some(policy) = policy else {
        unreachable!("handled missing policy above")
    };
    let options = match crate::commands::workflow::issue_transition_options(db, &issue.id) {
        Ok(options) => options,
        Err(_) => return Ok(IssueStartReadiness::NotReady),
    };
    let mut has_start_target = false;
    let mut blocked = false;
    for option in options {
        if workflow_category(Some(policy), &option.to).as_deref() != Some("active") {
            continue;
        }
        has_start_target = true;
        if option.allowed {
            return Ok(IssueStartReadiness::Ready);
        }
        blocked = true;
    }
    if blocked {
        Ok(IssueStartReadiness::Blocked)
    } else if has_start_target {
        Ok(IssueStartReadiness::NotReady)
    } else {
        Ok(IssueStartReadiness::NotReady)
    }
}

fn print_workflow_read_guidance(context: WorkflowReadContext) {
    if context.missing_policy {
        println!();
        println!(
            "Workflow policy missing: run `atelier workflow init`, then `atelier workflow check`."
        );
    }
    if context.unmigrated_filter {
        println!();
        println!(
            "Issue statuses outside the configured workflow are present; fix the records, then run `atelier workflow check`."
        );
    }
}

#[derive(Debug, Clone)]
pub struct DependencySummary {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct IssueObject {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub sections: Option<IssueSections>,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
    pub labels: Vec<String>,
    pub parent: Option<String>,
    pub notes: Vec<NoteObject>,
    pub assignee: Option<String>,
    pub owner: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub close_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NoteObject {
    pub kind: String,
    pub created_at: String,
    pub body: String,
}

pub fn resolve_id(db: &Database, issue_ref: &str) -> Result<String> {
    if let Some(id) = db.resolve_issue_ref(issue_ref)? {
        return Ok(id);
    }

    if let Some(actual_kind) = db.record_kind_for_id(issue_ref)? {
        bail!(
            "{} is a {} record, not an issue record. Use `{}`.",
            issue_ref,
            actual_kind,
            show_command_for_kind(&actual_kind, issue_ref)
        );
    }

    Err(anyhow!("Issue {issue_ref} was not found"))
}

fn show_command_for_kind(kind: &str, id: &str) -> String {
    match kind {
        "mission" => format!("atelier mission show {id}"),
        "plan" => format!("atelier plan show {id}"),
        "evidence" => format!("atelier evidence show {id}"),
        _ => format!("atelier {kind} show {id}"),
    }
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

fn note_object(comment: Comment) -> NoteObject {
    NoteObject {
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

fn activity_note_objects(issue_id: &str) -> Result<Vec<NoteObject>> {
    let Some(state_dir) = find_state_dir_from_cwd()? else {
        return Ok(Vec::new());
    };
    Ok(list_issue_activities(&state_dir, issue_id)?
        .into_iter()
        .filter(|activity| {
            matches!(
                activity.event_type,
                ActivityEventType::Comment
                    | ActivityEventType::Note
                    | ActivityEventType::Handoff
                    | ActivityEventType::Plan
            )
        })
        .map(|activity| NoteObject {
            kind: activity.event_type.to_string(),
            created_at: activity.created_at.to_rfc3339(),
            body: activity.body,
        })
        .collect())
}

fn activity_close_reason(issue_id: &str) -> Result<Option<String>> {
    let Some(state_dir) = find_state_dir_from_cwd()? else {
        return Ok(None);
    };
    Ok(list_issue_activities(&state_dir, issue_id)?
        .into_iter()
        .rev()
        .find(|activity| activity.event_type == ActivityEventType::CloseReason)
        .map(|activity| activity.body))
}

fn activity_field_new_value(issue_id: &str, field: &str) -> Result<Option<String>> {
    let Some(state_dir) = find_state_dir_from_cwd()? else {
        return Ok(None);
    };
    for activity in list_issue_activities(&state_dir, issue_id)?
        .into_iter()
        .rev()
        .filter(|activity| activity.event_type == ActivityEventType::FieldChanged)
    {
        if !activity
            .body
            .lines()
            .any(|line| scalar_line_value(line, "field").as_deref() == Some(field))
        {
            continue;
        }
        if let Some(value) = activity
            .body
            .lines()
            .find_map(|line| scalar_line_value(line, "new"))
        {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

fn scalar_line_value(line: &str, key: &str) -> Option<String> {
    let value = line.strip_prefix(&format!("{key}: "))?.trim();
    if value == "null" {
        return None;
    }
    serde_json::from_str::<String>(value).ok()
}

fn dependency_summary(db: &Database, id: &str) -> Result<DependencySummary> {
    let issue = db
        .get_issue(id)?
        .ok_or_else(|| anyhow!("Dependency issue {} was not found", format_issue_id(id)))?;
    Ok(DependencySummary {
        id: issue_id_for_agent(db, &issue)?,
    })
}

pub fn issue_object(db: &Database, issue: Issue) -> Result<IssueObject> {
    let labels = db.get_labels(&issue.id)?;
    issue_object_from_parts(db, issue, labels, None)
}

fn issue_object_from_canonical(
    db: &Database,
    projection_issue: Issue,
    record: CanonicalIssueRecord,
) -> Result<IssueObject> {
    let mut issue = record.issue;
    issue.parent_id = projection_issue.parent_id;
    issue.closed_at = projection_issue.closed_at.or(issue.closed_at);
    issue_object_from_parts(db, issue, record.labels, Some(record.sections))
}

fn issue_object_from_parts(
    db: &Database,
    issue: Issue,
    labels: Vec<String>,
    sections: Option<IssueSections>,
) -> Result<IssueObject> {
    let parent = match &issue.parent_id {
        Some(parent_id) => Some(dependency_summary(db, parent_id)?.id),
        None => None,
    };

    let raw_comments = db.get_comments(&issue.id)?;
    let imported_owner = comment_metadata_value(&raw_comments, "owner");
    let imported_assignee = comment_metadata_value(&raw_comments, "assignee");
    let close_reason = comment_metadata_value(&raw_comments, "Close reason")
        .or_else(|| label_value(&labels, "close-reason:"))
        .or(activity_close_reason(&issue.id)?);
    let comments = if raw_comments.is_empty() {
        activity_note_objects(&issue.id)?
    } else {
        raw_comments.into_iter().map(note_object).collect()
    };

    Ok(IssueObject {
        id: issue_id_for_agent(db, &issue)?,
        title: issue.title,
        description: issue.description,
        sections,
        status: issue.status,
        issue_type: issue.issue_type,
        priority: issue.priority,
        parent,
        notes: comments,
        assignee: label_value(&labels, "assignee:")
            .or(imported_assignee)
            .or(activity_field_new_value(&issue.id, "assignee")?),
        owner: label_value(&labels, "owner:").or(imported_owner),
        labels,
        created_at: issue.created_at.to_rfc3339(),
        updated_at: issue.updated_at.to_rfc3339(),
        closed_at: issue.closed_at.map(|dt| dt.to_rfc3339()),
        close_reason,
    })
}

fn canonical_issue_detail(issue_id: &str) -> Result<Option<CanonicalIssueRecord>> {
    let Some(state_dir) = find_state_dir_from_cwd()? else {
        return Ok(None);
    };
    let store = RecordStore::new(state_dir);
    Ok(Some(store.load_issue_by_id(issue_id)?))
}

fn canonical_issue_path_from_state(state_dir: &Path, issue_id: &str) -> PathBuf {
    state_dir.join("issues").join(format!("{issue_id}.md"))
}

fn canonical_issue_path(issue_id: &str) -> Result<Option<PathBuf>> {
    Ok(find_state_dir_from_cwd()?
        .map(|state_dir| canonical_issue_path_from_state(&state_dir, issue_id)))
}

fn issue_summary(db: &Database, issue: Issue) -> Result<IssueSummary> {
    Ok(IssueSummary {
        id: issue_id_for_agent(db, &issue)?,
        title: issue.title,
        status: issue.status,
        issue_type: issue.issue_type,
        priority: issue.priority,
        parent: issue
            .parent_id
            .map(|id| dependency_summary(db, &id).map(|summary| summary.id))
            .transpose()?,
    })
}

pub fn show(db: &Database, issue_ref: &str) -> Result<()> {
    let id = resolve_id(db, issue_ref)?;
    let issue = db.require_issue(&id)?;
    let (object, degraded) = match canonical_issue_detail(&id) {
        Ok(Some(record)) => (issue_object_from_canonical(db, issue, record)?, None),
        Ok(None) => (issue_object(db, issue)?, None),
        Err(error) => (
            issue_object(db, issue)?,
            Some(format!("Canonical issue record is malformed: {error:#}")),
        ),
    };
    render_issue_show_human(db, &id, &object, degraded.as_deref())
}

fn render_issue_show_human(
    db: &Database,
    canonical_id: &str,
    object: &IssueObject,
    degraded: Option<&str>,
) -> Result<()> {
    let workflow_policy = load_issue_workflow_policy()?;
    let status_category = issue_status_category(workflow_policy.as_ref(), &object.status);
    let identity = format!(
        "{} [{}] {} - {}",
        object.id,
        object.issue_type,
        format_status_with_category(status_category.as_deref(), &object.status),
        object.title
    );
    println!("{identity}");
    println!("{}", "=".repeat(identity.len()));
    println!("Status:   {}", object.status);
    println!(
        "Category: {}",
        status_category.unwrap_or_else(|| "(unknown)".to_string())
    );
    println!("Type:     {}", object.issue_type);
    println!("Priority: {}", object.priority);
    println!(
        "Created:  {}",
        format_human_datetime_str(&object.created_at)
    );
    println!(
        "Updated:  {}",
        format_human_datetime_str(&object.updated_at)
    );
    if let Some(closed_at) = &object.closed_at {
        println!("Closed:   {}", format_human_datetime_str(closed_at));
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
    if let Some(path) = canonical_issue_path(canonical_id)? {
        println!("File:     {}", path.display());
    }
    if let Some(degraded) = degraded {
        println!();
        println!("Tracker Degraded");
        println!("----------------");
        println!("{degraded}");
        println!("Fallback: showing the last valid local projection for orientation only.");
        println!("Next: atelier lint {}", object.id);
    }

    render_parent_context(db, canonical_id)?;
    render_transition_readiness(db, canonical_id, object)?;

    if let Some(sections) = &object.sections {
        print_text_section("Description", Some(&sections.description));
        print_text_section("Outcome", Some(&sections.outcome));
        print_text_section("Evidence", Some(&sections.evidence));
        print_text_section("Notes", sections.notes.as_deref());
    } else {
        print_text_section("Description", object.description.as_deref());
    }
    print_text_section("Close Reason", object.close_reason.as_deref());

    render_dependency_section(db, "Blocked by", db.get_blockers(canonical_id)?, true)?;
    render_dependency_section(db, "Blocking", db.get_blocking(canonical_id)?, false)?;
    render_subissue_section(db, canonical_id)?;
    render_recent_activity_section(canonical_id, object)?;
    render_command_footer(canonical_id, object)?;
    Ok(())
}

pub fn transition_options(db: &Database, issue_ref: &str) -> Result<()> {
    let id = resolve_id(db, issue_ref)?;
    let issue = db.require_issue(&id)?;
    let options = crate::commands::workflow::issue_transition_options(db, issue_ref)?;
    crate::commands::workflow::print_issue_transition_options(&issue, &options);
    Ok(())
}

fn render_transition_readiness(
    db: &Database,
    canonical_id: &str,
    object: &IssueObject,
) -> Result<()> {
    println!("\nTransition Readiness");
    println!("--------------------");
    match crate::commands::workflow::issue_transition_options(db, canonical_id) {
        Ok(options) => {
            for option in options {
                let state = if option.allowed { "allowed" } else { "blocked" };
                let summary = if option.allowed {
                    format!("to {}", option.to)
                } else {
                    option
                        .blockers
                        .first()
                        .cloned()
                        .unwrap_or_else(|| format!("to {}", option.to))
                };
                println!("  {}: {} - {}", option.name, state, summary);
                println!("    {}", option.command);
            }
        }
        Err(error) => {
            println!("  options: blocked - {error}");
        }
    }
    println!(
        "  options: atelier issue transition {} --options",
        object.id
    );
    Ok(())
}

fn linked_validating_evidence(db: &Database, issue_id: &str) -> Result<Vec<DomainRecord>> {
    let mut evidence = Vec::new();
    for link in db.list_record_links("issue", issue_id)? {
        if link.relation_type != "validates" {
            continue;
        }
        let evidence_id = if link.source_kind == "evidence" {
            Some(link.source_id)
        } else if link.target_kind == "evidence" {
            Some(link.target_id)
        } else {
            None
        };
        if let Some(evidence_id) = evidence_id {
            evidence.push(db.require_record("evidence", &evidence_id)?);
        }
    }
    evidence.sort_by(|a, b| a.id.cmp(&b.id));
    evidence.dedup_by(|a, b| a.id == b.id);
    Ok(evidence)
}

#[derive(Debug, Clone)]
pub(crate) struct EvidenceGateStatus {
    pub passed: bool,
    pub reason: String,
}

pub(crate) fn issue_evidence_gate_status(
    db: &Database,
    issue: &Issue,
) -> Result<EvidenceGateStatus> {
    let evidence = linked_validating_evidence(db, &issue.id)?;
    Ok(issue_evidence_gate_status_from_records(issue, &evidence))
}

fn issue_evidence_gate_status_from_records(
    issue: &Issue,
    evidence: &[DomainRecord],
) -> EvidenceGateStatus {
    if evidence.is_empty() {
        return evidence_gate(false, "no validating evidence link found");
    }

    let passing = evidence
        .iter()
        .filter(|record| record.status == "pass")
        .collect::<Vec<_>>();
    if passing.is_empty() {
        let statuses = evidence
            .iter()
            .map(|record| format!("{} [{}]", record.id, record.status))
            .collect::<Vec<_>>()
            .join(", ");
        return evidence_gate(
            false,
            format!("linked validating evidence is not passing: {statuses}"),
        );
    }

    if issue_requires_line_by_line_proof(issue) {
        if passing
            .iter()
            .any(|record| evidence_record_demonstrates_closeout_proof(record))
        {
            return evidence_gate(true, "line-by-line or contract-audit evidence is linked");
        }
        return evidence_gate(
            false,
            "linked passing evidence does not demonstrate line-by-line or contract-audit proof required for validation and parent closeout work",
        );
    }

    evidence_gate(true, "validating evidence is linked")
}

fn evidence_gate(passed: bool, reason: impl Into<String>) -> EvidenceGateStatus {
    EvidenceGateStatus {
        passed,
        reason: reason.into(),
    }
}

fn issue_requires_line_by_line_proof(issue: &Issue) -> bool {
    matches!(
        issue.issue_type.as_str(),
        "validation" | "epic" | "closeout" | "audit" | "review"
    )
}

fn evidence_record_demonstrates_closeout_proof(record: &DomainRecord) -> bool {
    let mut text = String::new();
    text.push_str(&record.title);
    text.push('\n');
    if let Some(body) = &record.body {
        text.push_str(body);
        text.push('\n');
    }
    text.push_str(&record.data_json);
    let text = text.to_ascii_lowercase();

    let closeout_terms = [
        "line-by-line",
        "line by line",
        "contract audit",
        "contract-audit",
        "closeout audit",
        "mission audit",
        "outcome audit",
        "outcome line",
        "linked epic outcome",
        "epic outcome",
    ];
    let classification_terms = [
        "classified",
        "classification",
        "pass/fail",
        "pass, fail",
        "blocked",
        "deferred",
        "not-applicable",
        "not applicable",
        "maps",
        "mapped",
    ];

    closeout_terms.iter().any(|term| text.contains(term))
        && classification_terms.iter().any(|term| text.contains(term))
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
    let workflow_policy = load_issue_workflow_policy()?;
    ids.into_iter()
        .map(|id| {
            let issue = db.require_issue(&id)?;
            let marker = if blockers && issue_blocks_work(workflow_policy.as_ref(), &issue) {
                " (open blocker)"
            } else {
                ""
            };
            Ok(format!(
                "{} [{}] {} - {}{}",
                issue_id_for_agent(db, &issue)?,
                issue_status_label(workflow_policy.as_ref(), &issue.status),
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
        "todo" => 0,
        "in_progress" => 1,
        "blocked" => 2,
        "review" => 3,
        "validation" => 4,
        "done" => 5,
        "archived" => 6,
        _ => 7,
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

fn render_command_footer(canonical_id: &str, object: &IssueObject) -> Result<()> {
    println!("\nNext Commands");
    println!("-------------");
    if let Some(path) = canonical_issue_path(canonical_id)? {
        println!("  Edit issue Markdown: {}", path.display());
    }
    println!("  Validate this issue: atelier lint {}", object.id);
    println!("  Add a note: atelier note add issue {} \"...\"", object.id);
    println!(
        "  Show full activity: atelier history --issue {}",
        object.id
    );
    println!(
        "  Show transition options: atelier issue transition {} --options",
        object.id
    );
    println!(
        "  Execute a transition: atelier issue transition {} <transition>",
        object.id
    );
    Ok(())
}

fn recent_activity_lines(canonical_id: &str, object: &IssueObject) -> Result<Vec<String>> {
    if let Some(state_dir) = find_state_dir_from_cwd()? {
        let activities = list_issue_activities(&state_dir, canonical_id)?;
        if !activities.is_empty() {
            return Ok(activities
                .iter()
                .rev()
                .take(8)
                .map(|activity| {
                    let body = human_activity_body(&activity.body).replace('\n', "\n  ");
                    let timestamp = format_human_datetime(activity.created_at);
                    if body.trim().is_empty() {
                        format!(
                            "[{}] {}: {}",
                            timestamp, activity.event_type, activity.summary
                        )
                    } else {
                        format!(
                            "[{}] {}: {}\n  {}",
                            timestamp, activity.event_type, activity.summary, body
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
        .take(8)
        .map(|note| {
            format!(
                "[{}] {}: {}",
                format_human_datetime_str(&note.created_at),
                note.kind,
                note.body.replace('\n', "\n  ")
            )
        })
        .collect())
}

fn find_state_dir_from_cwd() -> Result<Option<PathBuf>> {
    crate::storage_layout::find_canonical_dir_from_cwd()
}

pub fn list(
    db: &Database,
    status: Option<&str>,
    label: Option<&str>,
    priority: Option<&str>,
    ready: bool,
    quiet: bool,
) -> Result<()> {
    let workflow_policy = load_issue_workflow_policy()?;
    let status_input = status.unwrap_or("todo");
    if ready && status_input != "todo" {
        bail!("--ready uses startable todo-category work; do not combine it with --status");
    }
    let status_filter = IssueStatusFilter::from_input(workflow_policy.as_ref(), status_input);
    let mut read_context = WorkflowReadContext {
        missing_policy: workflow_policy.is_none(),
        unmigrated_filter: false,
    };
    let mut rows = db
        .list_issues(Some("all"), label, priority)?
        .into_iter()
        .filter(|issue| {
            ready || status_filter.matches(workflow_policy.as_ref(), issue, &mut read_context)
        })
        .map(|issue| issue_summary(db, issue))
        .map(|summary| summary.and_then(|issue| queue_row(db, workflow_policy.as_ref(), issue)))
        .collect::<Result<Vec<_>>>()?;
    if ready {
        rows = filter_ready_rows(db, workflow_policy.as_ref(), rows)?;
    }
    if rows.is_empty() {
        println!("No issues found.");
    } else if quiet {
        render_queue_ids_quiet(rows);
    } else {
        render_issue_queue_human(db, "Issue Queue", rows, true)?;
    }
    print_workflow_read_guidance(read_context);
    Ok(())
}

pub fn search(db: &Database, query: &str, quiet: bool) -> Result<()> {
    let lowercase = query.to_lowercase();
    let mut items = Vec::new();
    for issue in search_candidate_issues(db, &lowercase)? {
        items.push(issue_summary(db, issue)?);
    }
    if items.is_empty() {
        println!("No issues found matching '{query}'.");
        Ok(())
    } else if quiet {
        render_issue_ids_quiet(items);
        Ok(())
    } else {
        let rows = items
            .into_iter()
            .map(|item| {
                let workflow_policy = load_issue_workflow_policy()?;
                queue_row(db, workflow_policy.as_ref(), item)
            })
            .collect::<Result<Vec<_>>>()?;
        render_issue_queue_human(db, &format!("Search Results: {query}"), rows, true)
    }
}

fn search_candidate_issues(db: &Database, lowercase_query: &str) -> Result<Vec<Issue>> {
    let projection_issues = db.list_issues(Some("all"), None, None)?;
    let Some(state_dir) = find_state_dir_from_cwd()? else {
        let matched = projection_issues
            .into_iter()
            .filter(|issue| projection_issue_matches(issue, lowercase_query))
            .collect::<Vec<_>>();
        return Ok(matched);
    };

    let store = RecordStore::new(&state_dir);
    let mut canonical = store
        .load_issues()?
        .into_iter()
        .map(|record| (record.issue.id.clone(), record))
        .collect::<BTreeMap<_, _>>();

    let mut matched = Vec::new();
    for projection_issue in projection_issues {
        let Some(record) = canonical.remove(&projection_issue.id) else {
            bail!(
                "Projection issue {} has no canonical Markdown record",
                projection_issue.id
            );
        };
        let activity_matches = list_issue_activities(&state_dir, &projection_issue.id)?
            .into_iter()
            .any(|activity| {
                activity.summary.to_lowercase().contains(lowercase_query)
                    || activity.body.to_lowercase().contains(lowercase_query)
            });
        if canonical_issue_matches(&record, lowercase_query) || activity_matches {
            let mut issue = record.issue;
            issue.parent_id = projection_issue.parent_id;
            issue.closed_at = projection_issue.closed_at.or(issue.closed_at);
            matched.push(issue);
        }
    }
    Ok(matched)
}

fn projection_issue_matches(issue: &Issue, lowercase_query: &str) -> bool {
    let haystack = format!(
        "{}\n{}",
        issue.title,
        issue.description.as_deref().unwrap_or_default()
    )
    .to_lowercase();
    haystack.contains(lowercase_query)
}

fn canonical_issue_matches(record: &CanonicalIssueRecord, lowercase_query: &str) -> bool {
    let haystack = format!(
        "{}\n{}",
        record.issue.title,
        record.sections.searchable_text()
    )
    .to_lowercase();
    haystack.contains(lowercase_query)
}

fn render_issue_ids_quiet(items: Vec<IssueSummary>) {
    for item in items {
        println!("{}", item.id);
    }
}

fn render_queue_ids_quiet(items: Vec<QueueRow>) {
    for item in items {
        println!("{}", item.id);
    }
}

fn render_issue_queue_human(
    db: &Database,
    title: &str,
    items: Vec<QueueRow>,
    show_status: bool,
) -> Result<()> {
    let mut rows = items;
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

    let mut groups = queue_groups(db, rows)?;
    groups.sort_by(|a, b| {
        priority_rank(a.priority.as_deref().unwrap_or("low"))
            .cmp(&priority_rank(b.priority.as_deref().unwrap_or("low")))
            .then(a.id.cmp(&b.id))
            .then(a.title.cmp(&b.title))
    });
    for group in groups {
        print_queue_group(group, show_status);
    }

    Ok(())
}

fn queue_row(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    item: IssueSummary,
) -> Result<QueueRow> {
    let open_blockers = open_blocker_ids_with_policy(db, workflow_policy, &item.id)?;
    let depth = ancestry_depth(db, &item.id)?;
    let status_category = issue_status_category(workflow_policy, &item.status);
    Ok(QueueRow {
        id: item.id,
        title: item.title,
        status: item.status,
        status_category,
        issue_type: item.issue_type,
        priority: item.priority,
        parent: item.parent,
        open_blockers,
        depth,
    })
}

fn open_blocker_ids_with_policy(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issue_id: &str,
) -> Result<Vec<String>> {
    let mut blockers = db
        .get_blockers(issue_id)?
        .into_iter()
        .filter_map(|id| db.require_issue(&id).ok())
        .filter(|issue| issue_blocks_work(workflow_policy, issue))
        .map(|issue| format_issue_id(&issue.id))
        .collect::<Vec<_>>();
    blockers.sort();
    Ok(blockers)
}

fn filter_ready_rows(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    rows: Vec<QueueRow>,
) -> Result<Vec<QueueRow>> {
    let children = children_by_parent(db)?;
    rows.into_iter()
        .map(|row| {
            if has_descendants(&children, &row.id) {
                let external =
                    external_blockers_for_subtree(db, workflow_policy, &children, &row.id)?;
                let readiness =
                    issue_start_readiness(db, workflow_policy, &db.require_issue(&row.id)?)?;
                Ok((
                    row,
                    external.is_empty() && readiness == IssueStartReadiness::Ready,
                ))
            } else {
                let readiness =
                    issue_start_readiness(db, workflow_policy, &db.require_issue(&row.id)?)?;
                Ok((
                    row.clone(),
                    row.open_blockers.is_empty() && readiness == IssueStartReadiness::Ready,
                ))
            }
        })
        .filter_map(|result: Result<(QueueRow, bool)>| match result {
            Ok((row, true)) => Some(Ok(row)),
            Ok((_row, false)) => None,
            Err(err) => Some(Err(err)),
        })
        .collect()
}

fn queue_groups(db: &Database, rows: Vec<QueueRow>) -> Result<Vec<QueueGroup>> {
    let row_ids = rows
        .iter()
        .map(|row| row.id.clone())
        .collect::<BTreeSet<_>>();
    let children = children_by_parent(db)?;
    let mut grouped = BTreeMap::<String, Vec<QueueRow>>::new();
    let mut standalone = Vec::new();

    for row in rows {
        if let Some(group_id) = row_root_parent(db, &row.id)? {
            grouped.entry(group_id).or_default().push(row);
        } else if has_descendants(&children, &row.id) {
            grouped.entry(row.id.clone()).or_default().push(row);
        } else {
            standalone.push(row);
        }
    }

    let mut groups = Vec::new();
    for (group_id, mut rows) in grouped {
        rows.sort_by(|a, b| {
            ancestry_depth(db, &a.id)
                .unwrap_or(0)
                .cmp(&ancestry_depth(db, &b.id).unwrap_or(0))
                .then(priority_rank(&a.priority).cmp(&priority_rank(&b.priority)))
                .then(a.id.cmp(&b.id))
        });
        let issue = db.require_issue(&group_id)?;
        let workflow_policy = load_issue_workflow_policy()?;
        let external_blockers =
            external_blockers_for_subtree(db, workflow_policy.as_ref(), &children, &group_id)?;
        let include_header_row = row_ids.contains(&group_id);
        if include_header_row {
            rows.retain(|row| row.id != group_id);
        }
        groups.push(QueueGroup {
            id: Some(format_issue_id(&issue.id)),
            title: issue.title,
            issue_type: Some(issue.issue_type),
            priority: Some(issue.priority),
            external_blockers,
            rows,
        });
    }

    if !standalone.is_empty() {
        standalone.sort_by(|a, b| {
            status_rank(&a.status)
                .cmp(&status_rank(&b.status))
                .then(priority_rank(&a.priority).cmp(&priority_rank(&b.priority)))
                .then(a.id.cmp(&b.id))
        });
        groups.push(QueueGroup {
            id: None,
            title: "Standalone".to_string(),
            issue_type: None,
            priority: None,
            external_blockers: Vec::new(),
            rows: standalone,
        });
    }

    Ok(groups)
}

fn children_by_parent(db: &Database) -> Result<BTreeMap<String, Vec<String>>> {
    let mut children = BTreeMap::<String, Vec<String>>::new();
    for issue in db.list_issues(Some("all"), None, None)? {
        if let Some(parent_id) = issue.parent_id {
            children.entry(parent_id).or_default().push(issue.id);
        }
    }
    Ok(children)
}

fn has_descendants(children: &BTreeMap<String, Vec<String>>, issue_id: &str) -> bool {
    children
        .get(issue_id)
        .map(|children| !children.is_empty())
        .unwrap_or(false)
}

fn row_root_parent(db: &Database, issue_id: &str) -> Result<Option<String>> {
    let mut current = db.require_issue(issue_id)?;
    let mut root = None;
    while let Some(parent_id) = current.parent_id.clone() {
        root = Some(parent_id.clone());
        current = db.require_issue(&parent_id)?;
    }
    Ok(root)
}

fn ancestry_depth(db: &Database, issue_id: &str) -> Result<usize> {
    let mut current = db.require_issue(issue_id)?;
    let mut depth = 0;
    while let Some(parent_id) = current.parent_id.clone() {
        depth += 1;
        current = db.require_issue(&parent_id)?;
    }
    Ok(depth)
}

fn subtree_ids(children: &BTreeMap<String, Vec<String>>, root_id: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    let mut stack = vec![root_id.to_string()];
    while let Some(id) = stack.pop() {
        if ids.insert(id.clone()) {
            if let Some(child_ids) = children.get(&id) {
                stack.extend(child_ids.iter().cloned());
            }
        }
    }
    ids
}

fn external_blockers_for_subtree(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    children: &BTreeMap<String, Vec<String>>,
    root_id: &str,
) -> Result<Vec<String>> {
    let subtree = subtree_ids(children, root_id);
    let mut blockers = BTreeSet::<String>::new();
    for issue_id in &subtree {
        for blocker_id in db.get_blockers(issue_id)? {
            let blocker = db.require_issue(&blocker_id)?;
            if issue_blocks_work(workflow_policy, &blocker) && !subtree.contains(&blocker_id) {
                blockers.insert(format_issue_id(&blocker_id));
            }
        }
    }
    Ok(blockers.into_iter().collect())
}

fn queue_summary(rows: &[QueueRow]) -> String {
    let mut categories = BTreeMap::<String, usize>::new();
    let mut statuses = BTreeMap::<String, usize>::new();
    let mut priorities = BTreeMap::<String, usize>::new();
    let mut blocked = 0;
    for row in rows {
        *categories
            .entry(
                row.status_category
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string()),
            )
            .or_default() += 1;
        *statuses.entry(row.status.clone()).or_default() += 1;
        *priorities.entry(row.priority.clone()).or_default() += 1;
        if !row.open_blockers.is_empty() {
            blocked += 1;
        }
    }
    format!(
        "{} total | Category: {} | Status: {} | Priority: {} | Blocked: {}",
        rows.len(),
        joined_counts(categories),
        joined_counts(statuses),
        joined_counts(priorities),
        blocked
    )
}

fn print_queue_group(group: QueueGroup, show_status: bool) {
    let heading = match (&group.id, &group.issue_type, &group.priority) {
        (Some(id), Some(issue_type), Some(priority)) => {
            format!("[{issue_type}] {id} {priority} - {}", group.title)
        }
        _ => group.title,
    };
    println!("\n{heading}");
    println!("{}", "-".repeat(heading.len()));
    if !group.external_blockers.is_empty() {
        println!("  blocked by {}", compact_id_list(&group.external_blockers));
    }
    if group.rows.is_empty() {
        return;
    }
    for row in group.rows {
        let status_text = if show_status {
            format!(
                "{} ",
                format_status_with_category(row.status_category.as_deref(), &row.status)
            )
        } else {
            String::new()
        };
        let parent = if group.id.is_none() {
            row.parent
                .as_deref()
                .map(|parent| format!(" parent:{parent}"))
                .unwrap_or_default()
        } else {
            String::new()
        };
        let blockers = blocker_suffix(&row.open_blockers);
        let indent = "  ".repeat(row.depth.max(1));
        println!(
            "  {}[{}] {}{} - {}{}",
            format!("{indent}{status_text}"),
            row.issue_type,
            row.id,
            parent,
            row.title,
            blockers
        );
    }
}

fn blocker_suffix(blockers: &[String]) -> String {
    if blockers.is_empty() {
        String::new()
    } else {
        format!(" - blocked by {}", compact_id_list(blockers))
    }
}

fn compact_id_list(ids: &[String]) -> String {
    const LIMIT: usize = 3;
    if ids.len() <= LIMIT {
        ids.join(", ")
    } else {
        format!("{}, +{} more", ids[..LIMIT].join(", "), ids.len() - LIMIT)
    }
}

fn format_human_datetime_str(timestamp: &str) -> String {
    DateTime::parse_from_rfc3339(timestamp)
        .map(|dt| format_human_datetime(dt.with_timezone(&Utc)))
        .unwrap_or_else(|_| timestamp.to_string())
}

fn format_human_datetime(timestamp: DateTime<Utc>) -> String {
    timestamp
        .with_timezone(&Local)
        .format("%Y-%m-%d %H:%M %Z")
        .to_string()
}

fn human_activity_body(body: &str) -> String {
    let mut field = None;
    let mut old = None;
    let mut new = None;
    let mut all_structured = true;
    for line in body.lines().filter(|line| !line.trim().is_empty()) {
        if let Some(value) = scalar_line_value(line, "field") {
            field = Some(value);
        } else if let Some(value) = scalar_line_value(line, "old") {
            old = Some(value);
        } else if let Some(value) = scalar_line_value(line, "new") {
            new = Some(value);
        } else {
            all_structured = false;
        }
    }
    if all_structured {
        if let Some(field) = field {
            let old = old.unwrap_or_else(|| "(none)".to_string());
            let new = new.unwrap_or_else(|| "(none)".to_string());
            return format!("Changed {field}: {old} -> {new}");
        }
    }
    body.to_string()
}

pub struct CreateInput<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub priority: &'a str,
    pub issue_type: Option<&'a str>,
    pub labels: &'a [String],
    pub parent: Option<&'a str>,
}

pub struct LifecycleCreateInput<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub priority: &'a str,
    pub issue_type: &'a str,
    pub labels: &'a [String],
    pub parent: Option<&'a str>,
    pub work: bool,
    pub quiet: bool,
    pub atelier_dir: Option<&'a Path>,
}

pub fn create_lifecycle(
    state_dir: &Path,
    db_path: &Path,
    input: LifecycleCreateInput<'_>,
) -> Result<()> {
    validate_priority(input.priority)?;
    crate::db::validate_issue_type(input.issue_type)?;
    let db = Database::open(db_path)?;
    let parent_id = input
        .parent
        .map(|parent| resolve_id(&db, parent))
        .transpose()?;
    let session = if input.work {
        db.get_current_session().ok().flatten()
    } else {
        None
    };
    if input.work {
        if let (Some(dir), Some(_)) = (input.atelier_dir, session.as_ref()) {
            // The issue does not exist yet, so the lock can only be enforced after
            // projection refresh. Keep the existing warning behavior otherwise.
            let _ = dir;
        }
    }
    drop(db);

    let store = RecordStore::new(state_dir);
    let now = Utc::now();
    let id = store.allocate_issue_id()?;
    let initial_status = lifecycle_initial_status(state_dir, input.issue_type)?;
    let record = CanonicalIssueRecord {
        issue: Issue {
            id: id.clone(),
            title: input.title.to_string(),
            description: input.description.map(str::to_string),
            status: initial_status,
            issue_type: input.issue_type.to_string(),
            priority: input.priority.to_string(),
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
        },
        labels: input.labels.to_vec(),
        sections: IssueSections::unchecked_from_body(input.description),
        relationships: Relationships::default(),
    };
    store.write_issue_atomic(&record)?;
    if let Some(parent_id) = &parent_id {
        let mut parent = store.load_issue_by_id(parent_id)?;
        add_child_relationship(&mut parent, &id);
        parent.issue.updated_at = now;
        store.write_issue_atomic(&parent)?;
    }

    super::projection::refresh_after_canonical_write(state_dir, db_path)?;
    let refreshed = Database::open(db_path)?;
    if input.work {
        if let Some(session) = &session {
            if let Some(dir) = input.atelier_dir {
                crate::lock_check::enforce_lock(dir, &id, &refreshed)?;
            }
            refreshed.set_session_issue(session.id, &id)?;
        } else if !input.quiet {
            tracing::warn!("--work specified but no active session");
        }
    }
    let issue = refreshed.require_issue(&id)?;
    let object = issue_object(&refreshed, issue)?;
    let file_path = canonical_issue_path_from_state(state_dir, &id);
    if input.quiet {
        println!("{}", object.id);
    } else if parent_id.is_some() {
        println!(
            "Created subissue {} under {}",
            object.id,
            format_issue_id(parent_id.as_deref().unwrap_or_default())
        );
        println!("File: {}", file_path.display());
        if input.work && session.is_some() {
            println!("Now working on: {} {}", object.id, object.title);
        }
        println!();
        println!("Next Commands");
        println!("-------------");
        println!("  Edit issue Markdown: {}", file_path.display());
        println!("  Validate this issue: atelier lint {}", object.id);
        println!("  Inspect this issue: atelier issue show {}", object.id);
        println!("  Start tracked work: atelier start {}", object.id);
    } else {
        println!("Created issue {} - {}", object.id, object.title);
        println!("Type:     {}", object.issue_type);
        println!("Priority: {}", object.priority);
        println!("File:     {}", file_path.display());
        if input.work && session.is_some() {
            println!("Now working on: {} {}", object.id, object.title);
        }
        println!();
        println!("Next Commands");
        println!("-------------");
        println!("  Edit issue Markdown: {}", file_path.display());
        println!("  Validate this issue: atelier lint {}", object.id);
        println!("  Inspect this issue: atelier issue show {}", object.id);
        println!("  Start tracked work: atelier start {}", object.id);
    }
    Ok(())
}

pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {
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
    apply_workflow_initial_status(db, &id, issue_type)?;
    for label in input.labels {
        db.add_label(&id, label)?;
    }
    let issue = db.require_issue(&id)?;
    let object = issue_object(db, issue)?;
    println!("Created issue {}: {}", object.id, object.title);
    println!("Type:     {}", object.issue_type);
    println!("Priority: {}", object.priority);
    println!();
    println!("Next Commands");
    println!("-------------");
    println!("  atelier issue show {}", object.id);
    println!("  atelier start {}", object.id);
    Ok(())
}

fn lifecycle_initial_status(state_dir: &Path, issue_type: &str) -> Result<String> {
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "cannot determine repository root from {}",
            state_dir.display()
        )
    })?;
    crate::workflow_policy::configured_initial_status(repo_root, issue_type)?.ok_or_else(|| {
        anyhow!(
            "workflow policy file is required at {}; run `atelier workflow init` before creating issues",
            crate::workflow_policy::WORKFLOW_POLICY_PATH
        )
    })
}

fn apply_workflow_initial_status(db: &Database, id: &str, issue_type: &str) -> Result<()> {
    let repo_root = crate::storage_layout::find_repo_root()?;
    let status = crate::workflow_policy::configured_initial_status(&repo_root, issue_type)?
        .ok_or_else(|| {
            anyhow!(
                "workflow policy file is required at {}; run `atelier workflow init` before creating issues",
                crate::workflow_policy::WORKFLOW_POLICY_PATH
            )
        })?;
    db.update_issue_status(id, &status)?;
    Ok(())
}

pub struct UpdateInput<'a> {
    pub issue_ref: &'a str,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub priority: Option<&'a str>,
    pub issue_type: Option<&'a str>,
    pub labels: &'a [String],
    pub remove_labels: &'a [String],
    pub parent: Option<Option<&'a str>>,
    pub claim: bool,
    pub append_notes: Option<&'a str>,
}

pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {
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

    if let Some(issue_type) = input.issue_type {
        crate::db::validate_issue_type(issue_type)?;
        if db.update_issue_type(&id, issue_type)? {
            changed_fields.push("issue_type");
            crate::commands::activity_log::record_field_changed(
                &id,
                "issue_type",
                Some(&previous.issue_type),
                Some(issue_type),
            )?;
        }
    }

    for label in input.labels {
        db.add_label(&id, label)?;
        changed_fields.push("labels");
        crate::commands::activity_log::record_field_changed(&id, "labels", None, Some(label))?;
    }
    for label in input.remove_labels {
        db.remove_label(&id, label)?;
        changed_fields.push("labels");
        crate::commands::activity_log::record_field_changed(&id, "labels", Some(label), None)?;
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
        bail!("Nothing to update. Use --title, --description, --priority, --issue-type, --label, --remove-label, --parent, --claim, or --append-notes. Use `atelier issue transition <id> <transition>` for status changes");
    }
    changed_fields.sort_unstable();
    changed_fields.dedup();

    let issue = db.require_issue(&id)?;
    let object = issue_object(db, issue)?;
    println!(
        "Updated issue {} ({})",
        object.id,
        changed_fields.join(", ")
    );
    println!("Status:   {}", object.status);
    println!("Priority: {}", object.priority);
    println!("Type:     {}", object.issue_type);
    if let Some(assignee) = &object.assignee {
        println!("Assignee: {assignee}");
    }
    if let Some(parent) = &object.parent {
        println!("Parent:   {parent}");
    }
    println!();
    println!("Next Commands");
    println!("-------------");
    println!("  atelier issue show {}", object.id);
    println!("  atelier issue transition {} --options", object.id);
    Ok(())
}

pub fn update_lifecycle(state_dir: &Path, db_path: &Path, input: UpdateInput<'_>) -> Result<()> {
    let db = Database::open(db_path)?;
    let id = resolve_id(&db, input.issue_ref)?;
    let previous = db.require_issue(&id)?;
    let previous_assignee = label_value(&db.get_labels(&id)?, "assignee:");
    let parent_id = input
        .parent
        .map(|parent| parent.map(|parent| resolve_id(&db, parent)).transpose())
        .transpose()?
        .flatten();
    drop(db);

    let mut changed_fields = Vec::new();
    let store = RecordStore::new(state_dir);
    let mut record = store.load_issue_by_id(&id)?;
    let now = Utc::now();

    if let Some(title) = input.title {
        record.issue.title = title.to_string();
        changed_fields.push("title");
        crate::commands::activity_log::record_field_changed(
            &id,
            "title",
            Some(&previous.title),
            Some(title),
        )?;
    }
    if let Some(description) = input.description {
        record.issue.description = Some(description.to_string());
        changed_fields.push("description");
        crate::commands::activity_log::record_field_changed(
            &id,
            "description",
            previous.description.as_deref(),
            Some(description),
        )?;
    }
    if let Some(priority) = input.priority {
        validate_priority(priority)?;
        record.issue.priority = priority.to_string();
        changed_fields.push("priority");
        crate::commands::activity_log::record_field_changed(
            &id,
            "priority",
            Some(&previous.priority),
            Some(priority),
        )?;
    }
    if let Some(issue_type) = input.issue_type {
        crate::db::validate_issue_type(issue_type)?;
        record.issue.issue_type = issue_type.to_string();
        changed_fields.push("issue_type");
        crate::commands::activity_log::record_field_changed(
            &id,
            "issue_type",
            Some(&previous.issue_type),
            Some(issue_type),
        )?;
    }
    for label in input.labels {
        push_unique(&mut record.labels, label.to_string());
        changed_fields.push("labels");
        crate::commands::activity_log::record_field_changed(&id, "labels", None, Some(label))?;
    }
    for label in input.remove_labels {
        record.labels.retain(|existing| existing != label);
        changed_fields.push("labels");
        crate::commands::activity_log::record_field_changed(&id, "labels", Some(label), None)?;
    }
    if input.parent.is_some() {
        let old_parent = parent_record_containing_child(&store, &id)?;
        if old_parent.as_deref() != parent_id.as_deref() {
            if let Some(old_parent) = old_parent {
                let mut parent = store.load_issue_by_id(&old_parent)?;
                remove_child_relationship(&mut parent, &id);
                parent.issue.updated_at = now;
                store.write_issue_atomic(&parent)?;
            }
            if let Some(parent_id) = &parent_id {
                let mut parent = store.load_issue_by_id(parent_id)?;
                add_child_relationship(&mut parent, &id);
                parent.issue.updated_at = now;
                store.write_issue_atomic(&parent)?;
            }
        }
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
            record
                .labels
                .retain(|label| label != &format!("assignee:{previous_assignee}"));
        }
        push_unique(&mut record.labels, format!("assignee:{assignee}"));
        changed_fields.push("assignee");
        crate::commands::activity_log::record_field_changed(
            &id,
            "assignee",
            previous_assignee.as_deref(),
            Some(&assignee),
        )?;
        crate::commands::activity_log::record_note(&id, &format!("Claimed by {assignee}"))?;
    }
    if let Some(note) = input.append_notes {
        changed_fields.push("notes");
        crate::commands::activity_log::record_note(&id, note)?;
    }
    if changed_fields.is_empty() {
        bail!("Nothing to update. Use --title, --description, --priority, --issue-type, --label, --remove-label, --parent, --claim, or --append-notes. Use `atelier issue transition <id> <transition>` for status changes");
    }
    record.issue.updated_at = now;
    store.write_issue_atomic(&record)?;

    super::projection::refresh_after_canonical_write(state_dir, db_path)?;
    let db = Database::open(db_path)?;
    changed_fields.sort_unstable();
    changed_fields.dedup();
    let issue = db.require_issue(&id)?;
    let object = issue_object(&db, issue)?;
    println!(
        "Updated issue {} ({})",
        object.id,
        changed_fields.join(", ")
    );
    println!("Status:   {}", object.status);
    println!("Priority: {}", object.priority);
    println!("Type:     {}", object.issue_type);
    if let Some(assignee) = &object.assignee {
        println!("Assignee: {assignee}");
    }
    if let Some(parent) = &object.parent {
        println!("Parent:   {parent}");
    }
    println!();
    println!("Next Commands");
    println!("-------------");
    println!("  atelier issue show {}", object.id);
    Ok(())
}

pub fn close_lifecycle(
    state_dir: &Path,
    db_path: &Path,
    issue_ref: &str,
    reason: &str,
    to_status: Option<&str>,
) -> Result<()> {
    let db = Database::open(db_path)?;
    crate::commands::workflow::close_issue(&db, state_dir, db_path, issue_ref, to_status, reason)
}

pub fn delete_lifecycle(state_dir: &Path, db_path: &Path, issue_ref: &str) -> Result<String> {
    let db = Database::open(db_path)?;
    let id = resolve_id(&db, issue_ref)?;
    db.require_issue(&id)?;
    let descendants = descendant_issue_ids(&db, &id)?;
    drop(db);

    let store = RecordStore::new(state_dir);
    for parent_id in parent_records_containing_any_child(&store, &descendants)? {
        if descendants.contains(&parent_id) {
            continue;
        }
        let mut parent = store.load_issue_by_id(&parent_id)?;
        parent
            .relationships
            .children
            .retain(|child| !descendants.contains(&child.id));
        parent.issue.updated_at = Utc::now();
        store.write_issue_atomic(&parent)?;
    }
    for issue_id in &descendants {
        store.delete_issue_atomic(issue_id)?;
    }
    super::projection::refresh_after_canonical_write(state_dir, db_path)?;
    Ok(id)
}

fn descendant_issue_ids(db: &Database, root: &str) -> Result<Vec<String>> {
    let mut ids = vec![root.to_string()];
    let mut index = 0;
    while index < ids.len() {
        let current = ids[index].clone();
        for child in db.get_subissues(&current)? {
            ids.push(child.id);
        }
        index += 1;
    }
    Ok(ids)
}

fn parent_record_containing_child(store: &RecordStore, child_id: &str) -> Result<Option<String>> {
    Ok(store
        .load_issues()?
        .into_iter()
        .find(|record| {
            record
                .relationships
                .children
                .iter()
                .any(|child| child.kind == "issue" && child.id == child_id)
        })
        .map(|record| record.issue.id))
}

fn parent_records_containing_any_child(
    store: &RecordStore,
    child_ids: &[String],
) -> Result<Vec<String>> {
    let child_ids = child_ids.iter().collect::<BTreeSet<_>>();
    Ok(store
        .load_issues()?
        .into_iter()
        .filter(|record| {
            record
                .relationships
                .children
                .iter()
                .any(|child| child.kind == "issue" && child_ids.contains(&child.id))
        })
        .map(|record| record.issue.id)
        .collect())
}

fn add_child_relationship(record: &mut CanonicalIssueRecord, child_id: &str) {
    let child = RelationshipTarget {
        kind: "issue".to_string(),
        id: child_id.to_string(),
    };
    if !record.relationships.children.contains(&child) {
        record.relationships.children.push(child);
    }
}

fn remove_child_relationship(record: &mut CanonicalIssueRecord, child_id: &str) {
    record
        .relationships
        .children
        .retain(|child| !(child.kind == "issue" && child.id == child_id));
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.contains(&value) {
        values.push(value);
    }
}

pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
    let blocked_id = resolve_id(db, blocked_ref)?;
    let blocker_id = resolve_id(db, blocker_ref)?;
    let changed = db.add_dependency(&blocked_id, &blocker_id)?;
    dep_result(db, "dep.add", "add", &blocked_id, &blocker_id, changed)
}

pub fn dep_add_canonical(
    db: &Database,
    store: &RecordStore,
    blocked_ref: &str,
    blocker_ref: &str,
) -> Result<()> {
    let blocked_id = resolve_id(db, blocked_ref)?;
    let blocker_id = resolve_id(db, blocker_ref)?;
    let changed = store.add_issue_block(&blocked_id, &blocker_id)?;
    dep_result(db, "dep.add", "add", &blocked_id, &blocker_id, changed)
}

pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
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
    )
}

pub fn dep_remove_canonical(
    db: &Database,
    store: &RecordStore,
    blocked_ref: &str,
    blocker_ref: &str,
) -> Result<()> {
    let blocked_id = resolve_id(db, blocked_ref)?;
    let blocker_id = resolve_id(db, blocker_ref)?;
    let changed = store.remove_issue_block(&blocked_id, &blocker_id)?;
    dep_result(
        db,
        "dep.remove",
        "remove",
        &blocked_id,
        &blocker_id,
        changed,
    )
}

fn dep_result(
    db: &Database,
    command: &str,
    action: &str,
    blocked_id: &str,
    blocker_id: &str,
    changed: bool,
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
    let _ = command;
    let blocked = data["blocked"].as_str().unwrap_or_default();
    let blocker = data["blocker"].as_str().unwrap_or_default();
    let state = data["state"].as_str().unwrap_or_default();
    match action {
        "remove" => println!("{blocked} is no longer blocked by {blocker} ({state})"),
        _ => println!("{blocked} is blocked by {blocker} ({state})"),
    }
    Ok(())
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

pub fn dep_list(db: &Database, issue_ref: Option<&str>) -> Result<()> {
    let mut rows = Vec::new();
    let issues = if let Some(issue_ref) = issue_ref {
        let id = resolve_id(db, issue_ref)?;
        vec![db.require_issue(&id)?]
    } else {
        db.list_issues(Some("all"), None, None)?
    };
    for issue in issues {
        for blocker_id in db.get_blockers(&issue.id)? {
            let blocker = db.require_issue(&blocker_id)?;
            rows.push(DependencyListRow {
                blocked_id: issue_id_for_agent(db, &issue)?,
                blocked_title: issue.title.clone(),
                blocked_status: issue.status.clone(),
                blocked_priority: issue.priority.clone(),
                blocker_id: issue_id_for_agent(db, &blocker)?,
                blocker_title: blocker.title.clone(),
                blocker_status: blocker.status.clone(),
                blocker_priority: blocker.priority.clone(),
            });
        }
    }
    if rows.is_empty() {
        println!("No dependencies found.");
        Ok(())
    } else {
        println!("Dependencies");
        println!("============");
        println!("{} total", rows.len());
        rows.sort_by(|a, b| {
            status_rank(&a.blocked_status)
                .cmp(&status_rank(&b.blocked_status))
                .then(priority_rank(&a.blocked_priority).cmp(&priority_rank(&b.blocked_priority)))
                .then(a.blocked_id.cmp(&b.blocked_id))
                .then(a.blocker_id.cmp(&b.blocker_id))
        });
        for row in rows {
            println!(
                "  {} [{}] {} - {}",
                row.blocked_id, row.blocked_status, row.blocked_priority, row.blocked_title
            );
            println!(
                "    blocked by {} [{}] {} - {}",
                row.blocker_id, row.blocker_status, row.blocker_priority, row.blocker_title
            );
        }
        Ok(())
    }
}

const EVIDENCE_PROOF_TARGET_HINT: &str = "command, transcript, evidence record, test, \
review artifact, file change, or manual check";

const CONCRETE_EVIDENCE_MARKERS: &[&str] = &[
    "command",
    "transcript",
    "evidence record",
    "evidence id",
    "test",
    "tests",
    "nextest",
    "lint",
    "doctor",
    "export",
    "review artifact",
    "review",
    "artifact",
    "file change",
    "file diff",
    "manual check",
    "manual validation",
    "screenshot",
    "stdout",
    "stderr",
    "command output",
    "help text",
    "atelier ",
    "`atelier ",
    "cargo ",
    "git diff",
    "target/debug/atelier",
    ".rs",
    ".md",
    ".toml",
    ".json",
    ".yaml",
    ".yml",
];

const VAGUE_EVIDENCE_MARKERS: &[&str] = &[
    "not specified",
    "to be determined",
    "tbd",
    "todo",
    "none yet",
    "will be added",
    "add later",
    "later",
];

fn issue_requires_concrete_evidence(issue: &Issue) -> bool {
    !matches!(issue.status.as_str(), "done" | "archived") && issue.issue_type != "epic"
}

fn evidence_entries(evidence: &str) -> Vec<String> {
    if evidence
        .lines()
        .any(|line| strip_markdown_list_marker(line.trim()).is_some())
    {
        let mut entries = Vec::new();
        let mut current = String::new();
        for line in evidence.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(item) = strip_markdown_list_marker(trimmed) {
                if !current.trim().is_empty() {
                    entries.push(current.trim().to_string());
                }
                current = item.trim().to_string();
            } else if current.trim().is_empty() {
                current = trimmed.to_string();
            } else {
                current.push(' ');
                current.push_str(trimmed);
            }
        }
        if !current.trim().is_empty() {
            entries.push(current.trim().to_string());
        }
        entries
    } else {
        evidence
            .split("\n\n")
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
            .map(str::to_string)
            .collect()
    }
}

fn strip_markdown_list_marker(line: &str) -> Option<&str> {
    for prefix in ["- ", "* ", "+ "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some(rest);
        }
    }

    let (digits, rest) = line.split_once('.')?;
    if !digits.is_empty()
        && digits.chars().all(|character| character.is_ascii_digit())
        && rest.starts_with(' ')
    {
        Some(rest.trim_start())
    } else {
        None
    }
}

fn evidence_entry_names_observable_target(entry: &str) -> bool {
    let lower = entry.to_lowercase();
    if VAGUE_EVIDENCE_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
    {
        return false;
    }
    CONCRETE_EVIDENCE_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
}

pub fn lint(db: &Database, issue_ref: Option<&str>) -> Result<()> {
    let issues = if let Some(issue_ref) = issue_ref {
        let id = resolve_id(db, issue_ref)?;
        vec![db.require_issue(&id)?]
    } else {
        db.list_issues(Some("all"), None, None)?
    };
    let canonical_state_dir = find_state_dir_from_cwd()?;
    let (canonical_issues, canonical_findings) = if let Some(state_dir) = &canonical_state_dir {
        let store = RecordStore::new(&state_dir);
        let mut records = BTreeMap::new();
        let mut findings = Vec::new();
        let paths = if let Some(issue_ref) = issue_ref {
            let id = resolve_id(db, issue_ref)?;
            vec![issue_record_path(&id)]
        } else {
            match store.discover_issue_paths() {
                Ok(paths) => paths,
                Err(error) => {
                    findings.push(json!({
                        "id": "(canonical)",
                        "code": "invalid_canonical_state",
                        "path": state_dir.display().to_string(),
                        "message": format!("Canonical tracker Markdown is invalid: {error:#}")
                    }));
                    Vec::new()
                }
            }
        };
        for relative in paths {
            match store.load_issue(&relative) {
                Ok(record) => {
                    records.insert(record.issue.id.clone(), record);
                }
                Err(error) => {
                    findings.push(json!({
                        "id": issue_ref
                            .map(|_| relative.file_stem()
                                .and_then(|stem| stem.to_str())
                                .unwrap_or("(unknown)")
                                .to_string())
                            .unwrap_or_else(|| relative.file_stem()
                                .and_then(|stem| stem.to_str())
                                .unwrap_or("(unknown)")
                                .to_string()),
                        "code": "invalid_canonical_issue",
                        "path": state_dir.join(&relative).display().to_string(),
                        "message": format!("Canonical tracker Markdown is invalid: {error:#}")
                    }));
                }
            }
        }
        if findings.is_empty() {
            if let Err(error) = super::rebuild::validate_canonical_state(state_dir) {
                findings.push(json!({
                    "id": "(canonical)",
                    "code": "invalid_canonical_state",
                    "path": state_dir.display().to_string(),
                    "message": format!("Canonical tracker Markdown is invalid: {error:#}")
                }));
            }
        }
        (records, findings)
    } else {
        (BTreeMap::new(), Vec::new())
    };
    let mut findings = canonical_findings;
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
        if let Some(record) = canonical_issues.get(&issue.id) {
            for state in record.sections.section_states() {
                if state.required && (!state.present || state.empty) {
                    findings.push(json!({
                        "id": issue_id_for_agent(db, &issue)?,
                        "code": "invalid_issue_section",
                        "section": state.name.title(),
                        "path": canonical_state_dir
                            .as_ref()
                            .map(|state_dir| {
                                canonical_issue_path_from_state(state_dir, &issue.id)
                                    .display()
                                    .to_string()
                            })
                            .unwrap_or_default(),
                        "message": format!(
                            "Issue section {} must be present and non-empty",
                            state.name.title()
                        )
                    }));
                }
            }
            if issue_requires_concrete_evidence(&issue) {
                for (index, entry) in evidence_entries(&record.sections.evidence)
                    .iter()
                    .enumerate()
                {
                    if !evidence_entry_names_observable_target(entry) {
                        let relative = issue_record_path(&issue.id);
                        findings.push(json!({
                            "id": issue_id_for_agent(db, &issue)?,
                            "code": "vague_evidence",
                            "section": IssueSectionName::Evidence.title(),
                            "path": canonical_state_dir
                                .as_ref()
                                .map(|state_dir| {
                                    canonical_issue_path_from_state(state_dir, &issue.id)
                                        .display()
                                        .to_string()
                                })
                                .unwrap_or_default(),
                            "message": issue_section_diagnostic(
                                Some(&issue.id),
                                IssueSectionName::Evidence.title(),
                                &relative,
                                &format!(
                                    "Issue section Evidence entry {} must name an observable proof target ({})",
                                    index + 1,
                                    EVIDENCE_PROOF_TARGET_HINT
                                )
                            )
                        }));
                    }
                }
            }
        }
    }
    if findings.is_empty() {
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

pub fn doctor(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    runtime_db_existed: bool,
) -> Result<()> {
    let layout = crate::storage_layout::StorageLayout::new(repo_root);
    let atelier_dir = layout.atelier_dir();
    let config_path = layout.config_path();
    let cache_dir = layout.cache_dir();
    let db_path = layout.runtime_db_path();
    let rebuild_ready = super::rebuild::validate_canonical_state(state_dir).is_ok();
    let projection_fresh = crate::projection_index::check(db, state_dir)
        .map(|report| report.is_fresh())
        .unwrap_or(false);
    let runtime_tables_available = db.runtime_state_tables_available().unwrap_or(false);
    let ignore_rules_current = runtime_gitignore_entries_present(repo_root);
    let diagnostics = if crate::telemetry::diagnostics_enabled() {
        "enabled"
    } else {
        "disabled"
    };
    let mut health = BTreeMap::new();
    health.insert("config", config_path.exists());
    health.insert("database", runtime_db_existed);
    health.insert("ignore_rules", ignore_rules_current);
    health.insert("projection_fresh", projection_fresh);
    health.insert("rebuild_ready", rebuild_ready);
    health.insert("runtime_state", atelier_dir.is_dir());
    health.insert("runtime_tables", runtime_tables_available);
    println!("Database: {}", db_path.display());
    println!("State: {}", state_dir.display());
    println!("Install health:");
    println!(
        "  config: {}",
        if config_path.exists() { "ok" } else { "not ok" }
    );
    println!(
        "  ignored_runtime_paths: {}",
        if ignore_rules_current { "ok" } else { "not ok" }
    );
    println!("Projection rebuild:");
    println!(
        "  state_dir: {}",
        if state_dir.is_dir() { "ok" } else { "not ok" }
    );
    println!(
        "  rebuild_ready: {}",
        if rebuild_ready { "ok" } else { "not ok" }
    );
    println!(
        "  projection_fresh: {}",
        if projection_fresh { "ok" } else { "not ok" }
    );
    println!(
        "  tables: {}",
        crate::db::CANONICAL_PROJECTION_TABLES.join(", ")
    );
    println!("Cache health:");
    println!("  cache_dir: {}", optional_dir_status(&cache_dir));
    println!(
        "  projection_metadata: {}",
        if projection_fresh { "ok" } else { "stale" }
    );
    println!("Runtime state:");
    println!(
        "  directory: {}",
        if atelier_dir.is_dir() { "ok" } else { "not ok" }
    );
    println!(
        "  database: {}",
        if runtime_db_existed {
            "ok"
        } else {
            "missing (runtime projection artifact)"
        }
    );
    println!(
        "  local_tables: {}",
        if runtime_tables_available {
            "ok"
        } else {
            "not ok"
        }
    );
    println!("  diagnostics: {diagnostics}");
    println!("Compatibility:");
    println!("  tables: {}", crate::db::COMPATIBILITY_TABLES.join(", "));
    println!("Legacy health:");
    for (key, value) in health {
        println!("{key}: {}", if value { "ok" } else { "not ok" });
    }
    Ok(())
}

fn runtime_gitignore_entries_present(repo_root: &Path) -> bool {
    let Ok(gitignore) = std::fs::read_to_string(repo_root.join(".gitignore")) else {
        return false;
    };
    crate::commands::init::ROOT_GITIGNORE_ENTRIES
        .iter()
        .all(|entry| gitignore.lines().any(|line| line.trim() == *entry))
}

fn optional_dir_status(path: &Path) -> &'static str {
    if path.is_dir() {
        "ok"
    } else {
        "missing (optional)"
    }
}

pub fn export_canonical(db: &Database, state_dir: &Path, check: bool) -> Result<()> {
    if check {
        let stale = super::export::canonical_stale_entries(db, state_dir)?;
        if stale.is_empty() {
            println!("Canonical export is current");
            println!("State: {}", state_dir.display());
            Ok(())
        } else {
            bail!("Canonical export is stale:\n{}", stale.join("\n"))
        }
    } else {
        super::export::run_canonical(db, state_dir, false)?;
        println!("Canonical export written");
        println!("State: {}", state_dir.display());
        println!();
        println!("Next Commands");
        println!("-------------");
        println!("  atelier export --check");
        Ok(())
    }
}

pub fn rebuild(state_dir: &Path, db_path: &Path) -> Result<()> {
    super::rebuild::run(state_dir, db_path)?;
    println!("Runtime state rebuilt");
    println!("State:    {}", state_dir.display());
    println!("Database: {}", db_path.display());
    println!();
    println!("Next Commands");
    println!("-------------");
    println!("  atelier doctor");
    println!("  atelier export --check");
    Ok(())
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
        assert!(rows[0].contains("[todo/todo] high - Blocking issue"));
        assert!(rows[0].contains("(open blocker)"));
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
        assert!(summary.contains("done=1"));
        assert!(summary.contains("todo=1"));
        assert!(summary.contains("high=1"));
        assert!(summary.contains("low=1"));
        assert!(subissues.iter().any(|issue| issue.id == child_a));
    }
}
