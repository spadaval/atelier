use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Local, Utc};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::commands::issue_workflow::{
    format_status_with_category, issue_blocks_work, issue_status_category, issue_status_label,
    load_issue_workflow_policy, open_blocker_ids_with_policy,
};
use crate::commands::work_order::{order_work_rows, WorkOrderRow};
use crate::human_output::{
    self, render_issue_inventory, render_issue_inventory_quiet, DisplayRole, FooterAction,
    FooterPanel, IssueListPanel, IssueListRow, LinesPanel, MetadataPanel, Panel, RenderContext,
    StylePolicy, TextPanel,
};
use crate::utils::format_issue_id;
use atelier_app::issue_inventory::{
    issue_inventory, IssueInventoryFilters, IssueInventoryLimit, IssueInventoryRequest,
    IssueInventorySnapshot, IssueInventoryStatusFilter,
};
use atelier_app::issue_read::{ObjectiveIssueSummary, ObjectiveReadSummary};
use atelier_app::workflow_policy::WorkflowPolicy;
use atelier_core::{EvidenceRecord, Issue, IssuePriority, ISSUE_NOTE_KINDS};
use atelier_records::{
    parse_issue_sections, CanonicalIssueRecord, IssueSections, RecordStore, Relationships,
};
use atelier_sqlite::{validate_issue_type, Database};

const MAX_ISSUE_TITLE_LEN: usize = 512;
const MAX_ISSUE_DESCRIPTION_LEN: usize = 64 * 1024;

#[derive(Debug, Clone)]
pub struct IssueSummary {
    pub id: String,
    pub title: String,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
    pub parent: Option<String>,
    pub updated_at: DateTime<Utc>,
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
    updated_at: DateTime<Utc>,
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

impl QueueRow {
    fn work_order_row(&self) -> WorkOrderRow {
        WorkOrderRow {
            id: self.id.clone(),
            status_category: self.status_category.clone(),
            priority: self.priority.clone(),
            updated_at: self.updated_at,
            open_blockers: self.open_blockers.clone(),
        }
    }

    fn state_label(&self) -> &'static str {
        self.work_order_row().state().label()
    }
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

impl IssueStatusFilter {
    fn from_input(
        policy: Option<&WorkflowPolicy>,
        status: &str,
        category: Option<&str>,
    ) -> Result<Self> {
        if let Some(category) = category {
            if status != "todo" && status != "all" {
                bail!("--status and --category cannot be combined");
            }
            return Self::category(policy, category);
        }
        Self::status(policy, status)
    }

    fn status(policy: Option<&WorkflowPolicy>, input: &str) -> Result<Self> {
        if input == "all" {
            return Ok(Self::All);
        }
        if let Some(policy) = policy {
            if policy.statuses.contains_key(input) {
                return Ok(Self::Exact(input.to_string()));
            }
            bail!(
                "Invalid issue status '{input}'. Use an exact workflow status, `all`, or `--category <category>`."
            );
        }
        Ok(Self::Exact(input.to_string()))
    }

    fn category(policy: Option<&WorkflowPolicy>, input: &str) -> Result<Self> {
        let Some(policy) = policy else {
            bail!("--category requires a workflow policy");
        };
        if policy
            .statuses
            .values()
            .any(|status| status.category == input)
        {
            return Ok(Self::Category(input.to_string()));
        }
        bail!("Invalid issue category '{input}'. Use a category from the workflow policy.")
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
                let issue_category = issue_status_category(policy, &issue.status);
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
    issue_status_category(policy, status).is_some()
}

fn print_workflow_read_guidance(context: WorkflowReadContext) {
    if context.missing_policy {
        println!();
        println!(
            "Workflow policy missing: run `atelier check` to inspect tracker setup and restore the committed policy."
        );
    }
    if context.unmigrated_filter {
        println!();
        println!(
            "Issue statuses outside the configured workflow are present; fix the records, then rerun `atelier check`."
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
    pub relationships: Relationships,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
    pub labels: Vec<String>,
    pub parent: Option<String>,
    pub assignee: Option<String>,
    pub owner: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
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
        "mission" => format!("atelier issue show {id}"),
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
    issue_object_from_parts(db, issue, labels, None, Relationships::default())
}

fn issue_object_from_canonical(
    db: &Database,
    cached_issue: Issue,
    record: CanonicalIssueRecord,
) -> Result<IssueObject> {
    let mut issue = record.issue;
    issue.parent_id = cached_issue.parent_id;
    issue.closed_at = cached_issue.closed_at.or(issue.closed_at);
    issue_object_from_parts(
        db,
        issue,
        record.labels,
        Some(record.sections),
        record.relationships,
    )
}

fn issue_object_from_parts(
    db: &Database,
    issue: Issue,
    labels: Vec<String>,
    sections: Option<IssueSections>,
    relationships: Relationships,
) -> Result<IssueObject> {
    let parent = match &issue.parent_id {
        Some(parent_id) => Some(dependency_summary(db, parent_id)?.id),
        None => None,
    };

    Ok(IssueObject {
        id: issue_id_for_agent(db, &issue)?,
        title: issue.title,
        description: issue.description,
        sections,
        relationships,
        status: issue.status,
        issue_type: issue.issue_type,
        priority: issue.priority,
        parent,
        assignee: label_value(&labels, "assignee:"),
        owner: label_value(&labels, "owner:"),
        labels,
        created_at: issue.created_at.to_rfc3339(),
        updated_at: issue.updated_at.to_rfc3339(),
        closed_at: issue.closed_at.map(|dt| dt.to_rfc3339()),
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
        updated_at: issue.updated_at,
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
            Some(format!("Issue record file is malformed: {error:#}")),
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
    let objective_summary =
        objective_read_summary_for_show(db, canonical_id, workflow_policy.as_ref())?;
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
        println!("Fallback: showing the last valid cache-backed view for orientation only.");
        println!("Next: atelier check {}", object.id);
    }

    render_parent_context(db, canonical_id)?;
    render_transition_readiness(db, canonical_id, object)?;

    if let Some(sections) = &object.sections {
        print_text_section("Description", Some(&sections.description));
        print_text_section("Outcome", Some(&sections.outcome));
        print_text_section("Evidence", Some(&sections.evidence));
        print_text_section("Notes", sections.notes.as_deref());
        print_text_section("Close Reason", sections.close_reason.as_deref());
    } else {
        print_text_section("Description", object.description.as_deref());
    }

    render_dependency_section(db, "Blocked by", db.get_blockers(canonical_id)?, true)?;
    render_dependency_section(db, "Blocking", db.get_blocking(canonical_id)?, false)?;
    render_issue_link_section(db, canonical_id, &object.relationships)?;
    render_objective_rollup_section(objective_summary.as_ref());
    render_subissue_section(db, canonical_id)?;
    render_impact_section(db, canonical_id)?;
    render_command_footer(canonical_id, object)?;
    Ok(())
}

fn objective_read_summary_for_show(
    db: &Database,
    canonical_id: &str,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<Option<ObjectiveReadSummary>> {
    let active_issue_ids = active_issue_ids(db)?;
    let active_issue_refs = active_issue_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    atelier_app::issue_read::objective_read_summary(
        db,
        canonical_id,
        workflow_policy,
        &active_issue_refs,
    )
}

fn render_objective_rollup_section(summary: Option<&ObjectiveReadSummary>) {
    let Some(summary) = summary else {
        return;
    };
    let health = summary
        .scope
        .health(summary.relationships.open_blockers.len());
    print_panel(
        MetadataPanel::new("Objective Rollup")
            .row("Health", health.to_string())
            .row(
                "Scope",
                format!(
                    "{} scoped issue{}",
                    summary.scope.total(),
                    plural_suffix(summary.scope.total())
                ),
            )
            .row(
                "Buckets",
                format!(
                    "active {}, ready {}, blocked {}, done {}, backlog {}",
                    summary.scope.totals.active,
                    summary.scope.totals.ready,
                    summary.scope.totals.blocked,
                    summary.scope.totals.done,
                    summary.scope.totals.backlog
                ),
            )
            .row(
                "Relationships",
                format!(
                    "advances {}, open blockers {}, validating evidence {}, other {}",
                    summary.relationships.advances_roots.len(),
                    summary.relationships.open_blockers.len(),
                    summary.evidence.linked_validating_evidence,
                    summary.relationships.other_links
                ),
            )
            .row(
                "Evidence Gates",
                format!(
                    "linked validating evidence {}; scoped issues without evidence {}",
                    summary.evidence.linked_validating_evidence,
                    summary.evidence.scoped_issues_without_evidence
                ),
            ),
    );

    render_objective_issue_rows(
        "Ready Work",
        &summary.scope.ready,
        summary.scope.totals.ready,
    );
    render_objective_issue_rows(
        "Blocked Work",
        &summary.scope.blocked,
        summary.scope.totals.blocked,
    );
    render_objective_issue_rows("Done Work", &summary.scope.done, summary.scope.totals.done);

    if !summary.recent_activity.recently_updated.is_empty() {
        print_panel(LinesPanel::new(
            "Recent Activity Facts",
            summary
                .recent_activity
                .recently_updated
                .iter()
                .map(|issue| {
                    format!(
                        "  updated {} [{}] {} - {}",
                        format_issue_id(&issue.id),
                        issue.bucket.label(),
                        issue.priority,
                        issue.title
                    )
                }),
        ));
    }
}

fn render_objective_issue_rows(title: &str, issues: &[ObjectiveIssueSummary], total_count: usize) {
    if total_count == 0 {
        return;
    }
    let rows = issues
        .iter()
        .map(|issue| IssueListRow {
            role: display_role_for_bucket(issue.bucket.label()),
            id: format_issue_id(&issue.id),
            status: Some(issue.status.clone()),
            priority: issue.priority.clone(),
            title: issue.title.clone(),
            blockers: issue.open_blockers.len(),
            depth: 1,
        })
        .collect::<Vec<_>>();
    print_panel(
        IssueListPanel::new(title, rows)
            .total_count(total_count)
            .limit(issues.len()),
    );
}

fn render_issue_link_section(
    db: &Database,
    issue_id: &str,
    relationships: &Relationships,
) -> Result<()> {
    let mut seen = BTreeSet::new();
    let mut rows = Vec::new();
    for relation in &relationships.relates {
        if relation.kind != "issue" || relation.id == issue_id {
            continue;
        }
        let key = (relation.relation_type.clone(), relation.id.clone());
        if !seen.insert(key) {
            continue;
        }
        let other = db.require_issue(&relation.id)?;
        rows.push(format!(
            "{} {} [{}] {} - {}",
            relation.relation_type,
            format_issue_id(&other.id),
            other.status,
            other.priority,
            other.title
        ));
    }
    rows.sort();
    print_panel(LinesPanel::new(
        "Linked Issues",
        rows.into_iter().map(|row| format!("  {row}")),
    ));
    Ok(())
}

pub fn transition_options(db: &Database, issue_ref: &str, verbose: bool) -> Result<()> {
    let id = resolve_id(db, issue_ref)?;
    let issue = db.require_issue(&id)?;
    let options = crate::commands::workflow::issue_transition_options(db, issue_ref)?;
    crate::commands::workflow::print_issue_transition_options(&issue, &options, verbose);
    Ok(())
}

fn render_transition_readiness(
    db: &Database,
    canonical_id: &str,
    object: &IssueObject,
) -> Result<()> {
    let mut lines = Vec::new();
    match crate::commands::workflow::issue_transition_options(db, canonical_id) {
        Ok(options) => {
            for option in options {
                let state = if option.allowed { "allowed" } else { "blocked" };
                let summary = if option.allowed {
                    format!("to {}", option.to)
                } else {
                    option
                        .validator_results
                        .iter()
                        .find(|result| !result.passed && result.help.is_some())
                        .map(|result| result.reason.clone())
                        .or_else(|| option.blockers.first().cloned())
                        .unwrap_or_else(|| format!("to {}", option.to))
                };
                lines.push(format!("  {}: {} - {}", option.name, state, summary));
                if let Some(help) = option
                    .validator_results
                    .iter()
                    .find(|result| !result.passed && result.help.is_some())
                    .and_then(|result| result.help.as_deref())
                {
                    lines.push(format!("    {help}"));
                } else {
                    lines.push(format!("    {}", option.command));
                }
            }
        }
        Err(error) => {
            lines.push(format!("  options: blocked - {error}"));
        }
    }
    lines.push(format!("  options: atelier issue transition {}", object.id));
    print_panel(LinesPanel::new("Transition Readiness", lines));
    Ok(())
}

fn linked_validating_evidence(db: &Database, issue_id: &str) -> Result<Vec<EvidenceRecord>> {
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
            db.require_record("evidence", &evidence_id)?;
            if let Some(record) = canonical_evidence_record(&evidence_id)? {
                evidence.push(record);
            }
        }
    }
    evidence.sort_by(|a, b| a.header.id.cmp(&b.header.id));
    evidence.dedup_by(|a, b| a.header.id == b.header.id);
    Ok(evidence)
}

#[derive(Debug, Clone)]
pub(crate) struct EvidenceGateStatus {
    pub passed: bool,
}

pub(crate) fn issue_evidence_gate_status(
    db: &Database,
    issue: &Issue,
    sections: Option<&IssueSections>,
) -> Result<EvidenceGateStatus> {
    let evidence = linked_validating_evidence(db, &issue.id)?;
    Ok(issue_evidence_gate_status_from_records(
        issue, sections, &evidence,
    ))
}

fn issue_evidence_gate_status_from_records(
    issue: &Issue,
    sections: Option<&IssueSections>,
    evidence: &[EvidenceRecord],
) -> EvidenceGateStatus {
    if evidence.is_empty() {
        return evidence_gate(
            false,
            "no validating evidence link found",
            Some(evidence_help_hint()),
        );
    }

    let _ = issue;
    let _ = sections;
    let passing = evidence.iter().any(|record| {
        record.data.success.unwrap_or_else(|| {
            !matches!(record.header.status.as_str(), "blocked" | "fail" | "failed")
        })
    });
    if passing {
        evidence_gate(true, "passing validating evidence is linked", None)
    } else {
        evidence_gate(
            false,
            "expected at least 1 passing evidence record",
            Some(evidence_help_hint()),
        )
    }
}

fn canonical_evidence_record(id: &str) -> Result<Option<EvidenceRecord>> {
    let Some(state_dir) = atelier_app::storage_layout::find_canonical_dir_from_cwd()? else {
        return Ok(None);
    };
    let store = RecordStore::new(state_dir);
    Ok(store.load_evidence_by_id(id).ok())
}

pub(crate) fn evidence_help_hint() -> String {
    "record proof with `atelier evidence record --target issue/<id> --kind validation \"...\"` or attach existing proof with `atelier evidence attach <evidence-id> issue <issue-id>`".to_string()
}

fn evidence_gate(
    passed: bool,
    _reason: impl Into<String>,
    _help: Option<String>,
) -> EvidenceGateStatus {
    EvidenceGateStatus { passed }
}

fn render_parent_context(db: &Database, canonical_id: &str) -> Result<()> {
    let issue = db.require_issue(canonical_id)?;
    let line = match issue.parent_id {
        Some(parent_id) => {
            let parent = db.require_issue(&parent_id)?;
            format!(
                "Parent: {} [{}] {} - {}",
                format_issue_id(&parent.id),
                parent.status,
                parent.priority,
                parent.title
            )
        }
        None => "Parent: (none)".to_string(),
    };
    print_panel(LinesPanel::new("Hierarchy", [line]));
    Ok(())
}

fn print_text_section(title: &str, body: Option<&str>) {
    print_panel(TextPanel::new(title, body.map(str::to_string)));
}

fn render_dependency_section(
    db: &Database,
    title: &str,
    ids: Vec<String>,
    blockers: bool,
) -> Result<()> {
    let rows = dependency_rows_for_text(db, ids, blockers)?;
    print_panel(LinesPanel::new(
        title,
        rows.into_iter().map(|row| format!("  {row}")),
    ));
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
    let workflow_policy = load_issue_workflow_policy()?;
    if subissues.is_empty() {
        print_panel(LinesPanel::new("Subissues", Vec::<String>::new()));
        return Ok(());
    }

    subissues = order_issues_by_work(db, workflow_policy.as_ref(), subissues)?;
    let classified_subissues = subissues
        .into_iter()
        .map(|subissue| {
            let classified = atelier_app::read_pipeline::classify_issue(
                db,
                workflow_policy.as_ref(),
                &subissue,
            )?;
            Ok((subissue, classified))
        })
        .collect::<Result<Vec<_>>>()?;
    let summary = subissue_summary(
        classified_subissues.len(),
        classified_subissues.iter().map(|(subissue, classified)| {
            (
                classified.classification.label(),
                subissue.priority.as_str(),
            )
        }),
    );
    let rows = classified_subissues
        .iter()
        .map(|(subissue, classified)| {
            Ok(IssueListRow {
                role: display_role_for_bucket(classified.classification.label()),
                id: format_issue_id(&subissue.id),
                status: Some(classified.classification.label().to_string()),
                priority: subissue.priority.clone(),
                title: subissue.title.clone(),
                blockers: classified.open_blockers.len(),
                depth: 1,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let mut lines = vec![summary];
    lines.extend(
        IssueListPanel::new("Subissues", rows)
            .render(RenderContext::for_stdout())
            .into_iter()
            .skip(2),
    );
    print_panel(LinesPanel::new("Subissues", lines));
    Ok(())
}

fn display_role_for_bucket(label: &str) -> DisplayRole {
    match label {
        "active" => DisplayRole::Executable,
        "ready" => DisplayRole::Selectable,
        "blocked" => DisplayRole::Blocked,
        "blocked through parent" => DisplayRole::BlockedThroughParent,
        _ => DisplayRole::ContextOnly,
    }
}

fn render_impact_section(db: &Database, canonical_id: &str) -> Result<()> {
    let affected = db.downstream_impact(canonical_id)?;
    if affected.is_empty() {
        print_panel(LinesPanel::new(
            "Impact",
            ["No downstream issues found.".to_string()],
        ));
        return Ok(());
    }

    let workflow_policy = load_issue_workflow_policy()?;
    let mut lines = vec![format!(
        "{} downstream issue{} may need review before changing or closing this issue.",
        affected.len(),
        plural_suffix(affected.len())
    )];
    lines.extend(
        affected
            .iter()
            .take(8)
            .map(|issue| {
                Ok(format!(
                    "  {} [{}] {} - {}",
                    issue_id_for_agent(db, issue)?,
                    issue_status_label(workflow_policy.as_ref(), &issue.status),
                    issue.priority,
                    issue.title
                ))
            })
            .collect::<Result<Vec<_>>>()?,
    );
    if affected.len() > 8 {
        lines.push(format!("  ... and {} more", affected.len() - 8));
    }
    print_panel(LinesPanel::new("Impact", lines));
    Ok(())
}

fn order_issues_by_work(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issues: Vec<Issue>,
) -> Result<Vec<Issue>> {
    let rows = issues
        .iter()
        .map(|issue| work_order_row_for_issue(db, workflow_policy, issue))
        .collect::<Result<Vec<_>>>()?;
    let mut keyed = issues.into_iter().map(Some).collect::<Vec<_>>();
    Ok(crate::commands::work_order::ordered_work_indices(&rows)
        .into_iter()
        .filter_map(|index| keyed[index].take())
        .collect())
}

fn work_order_row_for_issue(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> Result<WorkOrderRow> {
    Ok(WorkOrderRow {
        id: format_issue_id(&issue.id),
        status_category: issue_status_category(workflow_policy, &issue.status),
        priority: issue.priority.clone(),
        updated_at: issue.updated_at,
        open_blockers: open_blocker_ids_with_policy(db, workflow_policy, &issue.id)?,
    })
}

fn subissue_summary<'a>(
    total: usize,
    subissues: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> String {
    let mut statuses = BTreeMap::<String, usize>::new();
    let mut priorities = BTreeMap::<String, usize>::new();
    for (status, priority) in subissues {
        *statuses.entry(status.to_string()).or_default() += 1;
        *priorities.entry(priority.to_string()).or_default() += 1;
    }
    format!(
        "{} total | status: {} | priority: {}",
        total,
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

fn priority_rank(priority: &str) -> u8 {
    IssuePriority::from_label(priority)
        .map(|priority| priority.sort_rank())
        .unwrap_or(4)
}

fn render_command_footer(canonical_id: &str, object: &IssueObject) -> Result<()> {
    let mut actions = Vec::new();
    if let Some(path) = canonical_issue_path(canonical_id)? {
        actions.push(FooterAction::new(
            "Edit issue Markdown",
            path.display().to_string(),
        ));
    }
    actions.extend([
        FooterAction::new(
            "Validate this issue",
            format!("atelier check {}", object.id),
        ),
        FooterAction::new(
            "Add a note",
            format!("atelier issue note {} \"...\"", object.id),
        ),
        FooterAction::new(
            "Show transition options",
            format!("atelier issue transition {}", object.id),
        ),
        FooterAction::new(
            "Execute a transition",
            format!("atelier issue transition {} <transition>", object.id),
        ),
    ]);
    print_panel(FooterPanel::new("Next Commands", actions));
    Ok(())
}

fn print_panel(panel: impl Panel) {
    let lines = panel.render(RenderContext::for_stdout());
    if lines.is_empty() {
        return;
    }
    println!();
    println!("{}", lines.join("\n"));
}

fn find_state_dir_from_cwd() -> Result<Option<PathBuf>> {
    atelier_app::storage_layout::find_canonical_dir_from_cwd()
}

pub fn list(
    db: &Database,
    status: Option<&str>,
    category: Option<&str>,
    label: Option<&str>,
    priority: Option<&str>,
    ready: bool,
    quiet: bool,
) -> Result<()> {
    list_with_title(
        db,
        "Issue Queue",
        status,
        category,
        None,
        label,
        priority,
        ready,
        quiet,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn list_inventory(
    db: &Database,
    status: &str,
    category: Option<&str>,
    issue_type: Option<&str>,
    label: Option<&str>,
    priority: Option<&str>,
    limit: usize,
    quiet: bool,
) -> Result<()> {
    let workflow_policy = load_issue_workflow_policy()?;
    if let Some(issue_type) = issue_type {
        let Some(policy) = workflow_policy.as_ref() else {
            bail!("--issue-type requires a workflow policy");
        };
        if !policy.issue_types.contains_key(issue_type) {
            bail!(
                "Invalid issue type '{}'. Use an issue type from .atelier/workflow.yaml: {}",
                issue_type,
                policy
                    .issue_types
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
    let status_filter = if status == "all" {
        IssueInventoryStatusFilter::All
    } else {
        // Reuse workflow parsing for configured-status validation, while keeping
        // category as a separate exact inventory filter.
        IssueStatusFilter::from_input(workflow_policy.as_ref(), status, None)?;
        IssueInventoryStatusFilter::Exact(status.to_string())
    };
    if let Some(category) = category {
        IssueStatusFilter::category(workflow_policy.as_ref(), category)?;
    }
    let limit = IssueInventoryLimit::new(limit)?;
    let issues = db.list_issues(Some("all"), label, priority)?;
    let snapshots = issues.into_iter().map(|issue| IssueInventorySnapshot {
        status_category: workflow_policy
            .as_ref()
            .and_then(|policy| policy.status_category(&issue.status))
            .map(str::to_string),
        id: issue.id.to_string(),
        issue_type: issue.issue_type,
        status: issue.status,
        priority: issue.priority,
        title: issue.title,
        labels: label.into_iter().map(str::to_string).collect(),
    });
    let view = issue_inventory(
        snapshots,
        &IssueInventoryRequest {
            filters: IssueInventoryFilters {
                status: status_filter,
                category: category.map(str::to_string),
                issue_type: issue_type.map(str::to_string),
                label: label.map(str::to_string),
                priority: priority
                    .map(IssuePriority::from_cli_input)
                    .transpose()?
                    .map(|priority| priority.label().to_string()),
            },
            limit,
        },
    );
    let output = if quiet {
        render_issue_inventory_quiet(&view)
    } else {
        render_issue_inventory(&view, RenderContext::for_stdout())
    };
    if !output.is_empty() {
        println!("{output}");
    }
    Ok(())
}

pub(crate) fn list_with_title(
    db: &Database,
    title: &str,
    status: Option<&str>,
    category: Option<&str>,
    issue_type: Option<&str>,
    label: Option<&str>,
    priority: Option<&str>,
    ready: bool,
    quiet: bool,
) -> Result<()> {
    let workflow_policy = load_issue_workflow_policy()?;
    if let Some(issue_type) = issue_type {
        let Some(policy) = workflow_policy.as_ref() else {
            bail!("--issue-type requires a workflow policy");
        };
        if !policy.issue_types.contains_key(issue_type) {
            bail!(
                "Invalid issue type '{}'. Use an issue type from .atelier/workflow.yaml: {}",
                issue_type,
                policy
                    .issue_types
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
    let status_input = status.unwrap_or("todo");
    if ready && ((status_input != "todo" && status_input != "all") || category.is_some()) {
        bail!("--ready uses startable todo-category work; do not combine it with --status or --category");
    }
    let status_filter =
        IssueStatusFilter::from_input(workflow_policy.as_ref(), status_input, category)?;
    let mut read_context = WorkflowReadContext {
        missing_policy: workflow_policy.is_none(),
        unmigrated_filter: false,
    };
    let mut rows = db
        .list_issues(Some("all"), label, priority)?
        .into_iter()
        .filter(|issue| {
            if issue_type.is_some_and(|issue_type| issue.issue_type != issue_type) {
                return false;
            }
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
        render_queue_ids_quiet(order_queue_rows(rows));
    } else {
        render_issue_queue_human(db, title, rows, true)?;
    }
    print_workflow_read_guidance(read_context);
    Ok(())
}

pub(crate) fn list_blocked_with_title(db: &Database, title: &str, quiet: bool) -> Result<()> {
    list_blocked_filtered_with_title(db, title, None, None, None, quiet)
}

fn list_blocked_filtered_with_title(
    db: &Database,
    title: &str,
    issue_type: Option<&str>,
    label: Option<&str>,
    priority: Option<&str>,
    quiet: bool,
) -> Result<()> {
    let workflow_policy = load_issue_workflow_policy()?;
    if let Some(issue_type) = issue_type {
        let Some(policy) = workflow_policy.as_ref() else {
            bail!("--issue-type requires a workflow policy");
        };
        if !policy.issue_types.contains_key(issue_type) {
            bail!(
                "Invalid issue type '{}'. Use an issue type from .atelier/workflow.yaml: {}",
                issue_type,
                policy
                    .issue_types
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
    let rows = db
        .list_issues(Some("all"), label, priority)?
        .into_iter()
        .filter(|issue| issue_type.is_none_or(|issue_type| issue.issue_type == issue_type))
        .map(|issue| {
            let classification =
                atelier_app::read_pipeline::classify_issue(db, workflow_policy.as_ref(), &issue)?
                    .classification;
            Ok((issue, classification))
        })
        .map(|result: Result<_>| {
            result.and_then(|(issue, classification)| {
                issue_summary(db, issue).map(|summary| (summary, classification))
            })
        })
        .map(|result| {
            result.and_then(|(issue, classification)| {
                queue_row(db, workflow_policy.as_ref(), issue).map(|row| (row, classification))
            })
        })
        .filter_map(|result| match result {
            Ok((row, atelier_app::read_pipeline::WorkClassification::Blocked)) => Some(Ok(row)),
            Ok((_row, _)) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>>>()?;
    if rows.is_empty() {
        println!("No issues found.");
    } else if quiet {
        render_queue_ids_quiet(order_queue_rows(rows));
    } else {
        render_issue_queue_human(db, title, rows, true)?;
    }
    Ok(())
}

fn active_issue_ids(db: &Database) -> Result<BTreeSet<String>> {
    let workflow_policy = load_issue_workflow_policy()?;
    let active_issues = crate::commands::status::current_work_issues(db, workflow_policy.as_ref())?;
    Ok(active_issues
        .into_iter()
        .map(|issue| issue.id)
        .collect::<BTreeSet<_>>())
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
    let rows = order_queue_rows(items);

    human_output::print_heading(title);
    println!("{}", queue_summary(&rows));

    let mut groups = queue_groups(db, rows)?;
    groups.sort_by(|a, b| {
        priority_rank(a.priority.as_deref().unwrap_or("low"))
            .cmp(&priority_rank(b.priority.as_deref().unwrap_or("low")))
            .then(a.id.cmp(&b.id))
            .then(a.title.cmp(&b.title))
    });
    let (footer_actions, omitted_footer_actions) = queue_footer_actions(&groups, 5);
    for group in groups {
        print_queue_group(group, show_status);
    }
    if !footer_actions.is_empty() {
        println!();
        println!(
            "{}",
            human_output::render_footer("Next Commands", footer_actions)
        );
        if omitted_footer_actions > 0 {
            println!("  {omitted_footer_actions} more blocker drill-downs omitted");
        }
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
        updated_at: item.updated_at,
    })
}

fn order_queue_rows(rows: Vec<QueueRow>) -> Vec<QueueRow> {
    order_work_rows(rows, QueueRow::work_order_row)
}

fn filter_ready_rows(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    rows: Vec<QueueRow>,
) -> Result<Vec<QueueRow>> {
    let children = children_by_parent(db)?;
    let candidate_ids = rows
        .iter()
        .map(|row| row.id.clone())
        .collect::<BTreeSet<_>>();
    let dependency_ready = match workflow_policy {
        Some(policy) => {
            atelier_app::objective_graph::dependency_readiness_batch(
                db,
                policy,
                candidate_ids.iter().cloned(),
            )?
            .ready_by_issue
        }
        None => BTreeMap::new(),
    };
    rows.into_iter()
        .map(|row| {
            let issue = db.require_issue(&row.id)?;
            if atelier_app::read_pipeline::classify_issue(db, workflow_policy, &issue)?
                .classification
                != atelier_app::read_pipeline::WorkClassification::Ready
            {
                return Ok((row, false));
            }
            if workflow_policy.is_some() && !dependency_ready.get(&row.id).copied().unwrap_or(false)
            {
                return Ok((row, false));
            }
            if has_descendants(&children, &row.id) {
                let external =
                    external_blockers_for_subtree(db, workflow_policy, &children, &row.id)?;
                let is_ready =
                    external.is_empty() && queue_row_can_enter_active(workflow_policy, &row)?;
                Ok((row, is_ready))
            } else {
                let is_ready = row.open_blockers.is_empty()
                    && queue_row_can_enter_active(workflow_policy, &row)?;
                Ok((row, is_ready))
            }
        })
        .filter_map(|result: Result<(QueueRow, bool)>| match result {
            Ok((row, true)) => Some(Ok(row)),
            Ok((_row, false)) => None,
            Err(err) => Some(Err(err)),
        })
        .collect()
}

fn queue_row_can_enter_active(
    workflow_policy: Option<&WorkflowPolicy>,
    row: &QueueRow,
) -> Result<bool> {
    match workflow_policy {
        Some(policy) => policy.can_enter_active(&row.issue_type, &row.status),
        None => Ok(row.status == "todo"),
    }
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
        rows = order_queue_rows(rows);
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
        standalone = order_queue_rows(standalone);
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
        "{} total | Categories: {} | Statuses: {} | Priorities: {} | Blocked: {}",
        rows.len(),
        joined_count_phrases(categories),
        joined_count_phrases(statuses),
        joined_count_phrases(priorities),
        blocked
    )
}

fn joined_count_phrases(counts: BTreeMap<String, usize>) -> String {
    if counts.is_empty() {
        return "none".to_string();
    }
    counts
        .into_iter()
        .map(|(name, count)| format!("{count} {name}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn print_queue_group(group: QueueGroup, show_status: bool) {
    let mut heading = match (&group.id, &group.issue_type, &group.priority) {
        (Some(id), Some(issue_type), Some(priority)) => {
            format!("[{issue_type}] {id} {priority} - {}", group.title)
        }
        _ => group.title,
    };
    if group.id.is_some() && !group.external_blockers.is_empty() {
        let policy = StylePolicy::for_stdout();
        heading.push_str(&format!(
            " ({}; {})",
            DisplayRole::ContextOnly.render(policy),
            DisplayRole::BlockedThroughParent.render(policy)
        ));
    }
    println!("\n{}", human_output::section_heading(&heading));
    if !group.external_blockers.is_empty() {
        println!(
            "  blocked by {} external blocker{}",
            group.external_blockers.len(),
            plural_suffix(group.external_blockers.len())
        );
    }
    if group.rows.is_empty() {
        return;
    }
    let all_blocked = group.rows.iter().all(|row| !row.open_blockers.is_empty());
    let row_limit = if all_blocked { 5 } else { 20 };
    let total_rows = group.rows.len();
    for row in group.rows.into_iter().take(row_limit) {
        let status_text = if show_status {
            format!("{} ", row.state_label())
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
        let blockers = blocker_suffix(&row.id, &row.open_blockers);
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
    if total_rows > row_limit {
        let omitted = total_rows - row_limit;
        let label = if all_blocked { "blocked" } else { "work" };
        println!(
            "  {omitted} more {label} issue{} omitted",
            plural_suffix(omitted)
        );
    }
}

fn blocker_suffix(_issue_id: &str, blockers: &[String]) -> String {
    if blockers.is_empty() {
        String::new()
    } else {
        format!(
            " ({} blocker{})",
            blockers.len(),
            plural_suffix(blockers.len()),
        )
    }
}

fn queue_footer_actions(groups: &[QueueGroup], limit: usize) -> (Vec<FooterAction>, usize) {
    let mut actions = Vec::new();
    for group in groups {
        if let Some(group_id) = &group.id {
            if !group.external_blockers.is_empty() {
                actions.push(FooterAction::new(
                    format!("Inspect blockers for {group_id}"),
                    format!("atelier issue show {group_id}"),
                ));
            }
        }
        for row in &group.rows {
            if !row.open_blockers.is_empty() {
                actions.push(FooterAction::new(
                    format!("Inspect blockers for {}", row.id),
                    format!("atelier issue show {}", row.id),
                ));
            }
        }
    }
    let total = actions.len();
    let (actions, omitted) = human_output::bounded_items(&actions, limit);
    debug_assert_eq!(omitted, total.saturating_sub(actions.len()));
    (actions, omitted)
}

fn plural_suffix(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
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

pub struct LifecycleCreateInput<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub body: Option<&'a str>,
    pub priority: &'a str,
    pub issue_type: &'a str,
    pub labels: &'a [String],
    pub parent: Option<&'a str>,
    pub constraints: Vec<String>,
    pub risks: Vec<String>,
    pub validation: Vec<String>,
    pub quiet: bool,
}

pub fn create_lifecycle(
    state_dir: &Path,
    db_path: &Path,
    input: LifecycleCreateInput<'_>,
) -> Result<()> {
    if input.title.len() > MAX_ISSUE_TITLE_LEN {
        bail!(
            "Title exceeds maximum length of {} characters",
            MAX_ISSUE_TITLE_LEN
        );
    }
    if input.issue_type != "mission"
        && (!input.constraints.is_empty()
            || !input.risks.is_empty()
            || !input.validation.is_empty())
    {
        bail!("mission section flags are not supported for issue records");
    }
    validate_priority(input.priority)?;
    validate_configured_issue_type(state_dir, input.issue_type)?;
    let db = Database::open(db_path)?;
    let parent_id = input
        .parent
        .map(|parent| resolve_id(&db, parent))
        .transpose()?;

    let store = RecordStore::new(state_dir);
    let now = Utc::now();
    let id = store.allocate_issue_id()?;
    let initial_status = lifecycle_initial_status(state_dir, input.issue_type)?;
    let description = lifecycle_issue_description(&input);
    if description
        .as_ref()
        .is_some_and(|description| description.len() > MAX_ISSUE_DESCRIPTION_LEN)
    {
        bail!(
            "Description exceeds maximum length of {} bytes",
            MAX_ISSUE_DESCRIPTION_LEN
        );
    }
    let sections = lifecycle_issue_sections(&input)?;
    let record = CanonicalIssueRecord {
        issue: Issue {
            id: id.clone(),
            title: input.title.to_string(),
            description: description.clone(),
            status: initial_status,
            issue_type: input.issue_type.to_string(),
            priority: input.priority.to_string(),
            fields: Default::default(),
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
        },
        labels: input.labels.to_vec(),
        sections,
        relationships: Relationships::default(),
    };
    atelier_app::workflow_policy::validate_issue_hierarchy(
        &db,
        &record.issue,
        parent_id.as_deref(),
    )?;
    drop(db);
    store.write_issue_atomic(&record)?;
    if let Some(parent_id) = &parent_id {
        store.add_issue_child(parent_id, &id)?;
    }

    let file_path = canonical_issue_path_from_state(state_dir, &id);
    if input.quiet {
        println!("{}", record.issue.id);
    } else if parent_id.is_some() {
        println!(
            "Created subissue {} under {}",
            record.issue.id,
            format_issue_id(parent_id.as_deref().unwrap_or_default())
        );
        println!("File: {}", file_path.display());
        println!();
        println!("Next Commands");
        println!("-------------");
        println!("  Edit issue Markdown: {}", file_path.display());
        println!("  Validate this issue: atelier check {}", record.issue.id);
        println!(
            "  Inspect this issue: atelier issue show {}",
            record.issue.id
        );
        println!(
            "  Inspect tracked work transitions: atelier issue transition {}",
            record.issue.id
        );
    } else {
        if record.issue.issue_type == "mission" {
            println!(
                "Created mission objective {} - {}",
                record.issue.id, record.issue.title
            );
        } else {
            println!("Created issue {} - {}", record.issue.id, record.issue.title);
        }
        println!("Type:     {}", record.issue.issue_type);
        println!("Priority: {}", record.issue.priority);
        println!("File:     {}", file_path.display());
        println!();
        println!("Next Commands");
        println!("-------------");
        println!("  Edit issue Markdown: {}", file_path.display());
        println!("  Validate this issue: atelier check {}", record.issue.id);
        println!(
            "  Inspect this issue: atelier issue show {}",
            record.issue.id
        );
        println!(
            "  Inspect tracked work transitions: atelier issue transition {}",
            record.issue.id
        );
    }
    Ok(())
}

fn lifecycle_issue_description(input: &LifecycleCreateInput<'_>) -> Option<String> {
    if input.issue_type != "mission" {
        return input.description.or(input.body).map(str::to_string);
    }
    let mut lines = Vec::new();
    if let Some(description) = input.description.or(input.body) {
        let description = description.trim();
        if !description.is_empty() {
            lines.push(description.to_string());
        }
    }
    append_named_list(&mut lines, "Constraints", &input.constraints);
    append_named_list(&mut lines, "Risks", &input.risks);
    append_named_list(&mut lines, "Validation", &input.validation);
    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n\n"))
    }
}

fn lifecycle_issue_sections(input: &LifecycleCreateInput<'_>) -> Result<IssueSections> {
    if input.issue_type != "mission" {
        let description = lifecycle_issue_description(input);
        return Ok(IssueSections::unchecked_from_body(description.as_deref()));
    }
    let Some(body) = input.body.filter(|body| has_level_two_heading(body)) else {
        let description = lifecycle_issue_description(input);
        return Ok(IssueSections::unchecked_from_body(description.as_deref()));
    };

    let normalized = if has_issue_section(body, "Description") {
        body.to_string()
    } else {
        format!(
            "## Description\n\nNo description provided.\n\n{}",
            body.trim()
        )
    };
    let mut sections = parse_issue_sections(&normalized, Path::new("<issue create --body>"))
        .context("Invalid sectioned mission --body")?;

    let mut additions = Vec::new();
    append_named_list(&mut additions, "Constraints", &input.constraints);
    append_named_list(&mut additions, "Risks", &input.risks);
    append_named_list(&mut additions, "Validation", &input.validation);
    if !additions.is_empty() {
        sections.description.push_str("\n\n");
        sections.description.push_str(&additions.join("\n\n"));
    }
    Ok(sections)
}

fn has_level_two_heading(body: &str) -> bool {
    body.lines().any(|line| {
        let Some(rest) = line.strip_prefix("##") else {
            return false;
        };
        !rest.starts_with('#')
            && (rest.is_empty() || rest.chars().next().is_some_and(char::is_whitespace))
    })
}

fn has_issue_section(body: &str, name: &str) -> bool {
    body.lines().any(|line| {
        line.strip_prefix("##")
            .filter(|rest| !rest.starts_with('#'))
            .is_some_and(|rest| rest.trim() == name)
    })
}

fn append_named_list(lines: &mut Vec<String>, title: &str, values: &[String]) {
    if values.is_empty() {
        return;
    }
    lines.push(format!(
        "{title}:\n{}",
        values
            .iter()
            .map(|value| format!("- {}", value.trim()))
            .collect::<Vec<_>>()
            .join("\n")
    ));
}

fn validate_configured_issue_type(state_dir: &Path, issue_type: &str) -> Result<()> {
    validate_issue_type(issue_type)?;
    let policy = load_repo_policy(state_dir)?;
    if policy.issue_types.contains_key(issue_type) {
        Ok(())
    } else {
        bail!(
            "workflow policy at {} must declare issue_type '{}' in issue_types and cover it exactly once in workflows.*.applies_to; valid values: {}",
            atelier_app::workflow_policy::WORKFLOW_POLICY_PATH,
            issue_type,
            configured_issue_type_names(&policy).join(", ")
        )
    }
}

fn validate_issue_record_against_workflow(state_dir: &Path, issue: &Issue) -> Result<()> {
    let policy = load_repo_policy(state_dir)?;
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "cannot determine repository root from {}",
            state_dir.display()
        )
    })?;
    let policy_path = repo_root.join(atelier_app::workflow_policy::WORKFLOW_POLICY_PATH);
    atelier_app::workflow_policy::validate_issue_against_policy(&policy, issue, &policy_path)
}

fn load_repo_policy(state_dir: &Path) -> Result<WorkflowPolicy> {
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "cannot determine repository root from {}",
            state_dir.display()
        )
    })?;
    atelier_app::workflow_policy::load(repo_root)
}

fn configured_issue_type_names(policy: &WorkflowPolicy) -> Vec<String> {
    policy.issue_types.keys().cloned().collect()
}

fn lifecycle_initial_status(state_dir: &Path, issue_type: &str) -> Result<String> {
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "cannot determine repository root from {}",
            state_dir.display()
        )
    })?;
    atelier_app::workflow_policy::configured_initial_status(repo_root, issue_type)?.ok_or_else(|| {
        anyhow!(
            "workflow policy file is required at {}; run `atelier check` to inspect setup and restore the committed policy before creating issues",
            atelier_app::workflow_policy::WORKFLOW_POLICY_PATH
        )
    })
}

pub struct UpdateInput<'a> {
    pub issue_ref: &'a str,
    pub title: Option<&'a str>,
    pub priority: Option<&'a str>,
    pub issue_type: Option<&'a str>,
    pub labels: &'a [String],
    pub remove_labels: &'a [String],
    pub parent: Option<Option<&'a str>>,
}

pub fn update_lifecycle(state_dir: &Path, db_path: &Path, input: UpdateInput<'_>) -> Result<()> {
    let db = Database::open(db_path)?;
    let id = resolve_id(&db, input.issue_ref)?;
    let previous = db.require_issue(&id)?;
    let requested_parent_id = input
        .parent
        .map(|parent| parent.map(|parent| resolve_id(&db, parent)).transpose())
        .transpose()?
        .flatten();
    let parent_id = if input.parent.is_some() {
        requested_parent_id
    } else {
        previous.parent_id.clone()
    };

    let mut changed_fields = Vec::new();
    let store = RecordStore::new(state_dir);
    let mut record = store.load_issue_by_id(&id)?;
    let old_parent_for_update = if input.parent.is_some() {
        Some(store.find_issue_parent(&id)?)
    } else {
        None
    };
    let now = Utc::now();

    if let Some(title) = input.title {
        record.issue.title = title.to_string();
        changed_fields.push("title");
    }
    if let Some(priority) = input.priority {
        validate_priority(priority)?;
        record.issue.priority = priority.to_string();
        changed_fields.push("priority");
    }
    if let Some(issue_type) = input.issue_type {
        validate_configured_issue_type(state_dir, issue_type)?;
        record.issue.issue_type = issue_type.to_string();
        changed_fields.push("issue_type");
    }
    for label in input.labels {
        push_unique(&mut record.labels, label.to_string());
        changed_fields.push("labels");
    }
    for label in input.remove_labels {
        record.labels.retain(|existing| existing != label);
        changed_fields.push("labels");
    }
    if input.parent.is_some() {
        changed_fields.push("parent");
    }
    if changed_fields.is_empty() {
        bail!("Nothing to update. Use --title, --priority, --issue-type, --label, --remove-label, or --parent. Use `atelier issue note <id> \"...\"` for notes or `atelier issue transition <id> <transition>` for status changes");
    }
    record.issue.updated_at = now;
    validate_issue_record_against_workflow(state_dir, &record.issue)?;
    atelier_app::workflow_policy::validate_issue_hierarchy(
        &db,
        &record.issue,
        parent_id.as_deref(),
    )?;
    drop(db);
    if let Some(old_parent) = old_parent_for_update {
        if old_parent.as_deref() != parent_id.as_deref() {
            if let Some(old_parent) = old_parent {
                store.remove_issue_child(&old_parent, &id)?;
            }
            if let Some(parent_id) = &parent_id {
                store.add_issue_child(parent_id, &id)?;
            }
        }
    }
    store.write_issue_atomic(&record)?;

    changed_fields.sort_unstable();
    changed_fields.dedup();
    println!(
        "Updated issue {} ({})",
        record.issue.id,
        changed_fields.join(", ")
    );
    println!("Status:   {}", record.issue.status);
    println!("Priority: {}", record.issue.priority);
    println!("Type:     {}", record.issue.issue_type);
    if let Some(assignee) = label_value(&record.labels, "assignee:") {
        println!("Assignee: {assignee}");
    }
    if let Some(parent) = parent_id {
        println!("Parent:   {}", format_issue_id(&parent));
    }
    println!();
    println!("Next Commands");
    println!("-------------");
    println!("  atelier issue show {}", record.issue.id);
    Ok(())
}

/// Append a dated note to the issue record's Notes section.
pub fn append_note(state_dir: &Path, issue_id: &str, kind: &str, text: &str) -> Result<()> {
    if !ISSUE_NOTE_KINDS.contains(&kind) {
        bail!(
            "Invalid note kind '{kind}'. Valid kinds: {}",
            ISSUE_NOTE_KINDS.join(", ")
        );
    }
    if text.trim().is_empty() {
        bail!("Note text for issue {issue_id} must not be empty");
    }
    // Hold the exclusive transaction across read and write so concurrent
    // notes cannot overwrite each other's appended bullets.
    let _transaction = atelier_records::mutation_lock::CanonicalMutationLock::exclusive(state_dir)?;
    let store = RecordStore::new(state_dir);
    let mut record = store.load_issue_by_id(issue_id)?;
    let now = Utc::now();
    record.sections.append_note(now.date_naive(), kind, text);
    record.issue.updated_at = now;
    store.write_issue_atomic(&record)?;
    println!("Added note to issue {}", format_issue_id(issue_id));
    Ok(())
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.contains(&value) {
        values.push(value);
    }
}

pub fn lint(db: &Database, issue_ref: Option<&str>) -> Result<()> {
    let outcome = atelier_app::lint::lint(atelier_app::Request {
        input: atelier_app::lint::LintRequest { db, issue_ref },
    })?;
    let view = outcome.value.data;
    if view.findings.is_empty() {
        println!("Lint passed.");
    } else {
        println!("Lint found {} issue(s):", view.findings.len());
        for finding in &view.findings {
            println!("  {}: {}", finding.id, finding.message);
        }
    }
    if view.findings.is_empty() {
        Ok(())
    } else {
        bail!("Lint failed with {} finding(s)", view.findings.len())
    }
}

pub fn doctor(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    db_path: &Path,
    cache_db_existed: bool,
    fix: bool,
) -> Result<()> {
    let outcome = atelier_app::health::doctor(atelier_app::Request {
        input: atelier_app::health::DoctorRequest {
            db,
            repo_root: repo_root.to_path_buf(),
            state_dir: state_dir.to_path_buf(),
            db_path: db_path.to_path_buf(),
            cache_db_existed,
            fix,
        },
    })?;
    render_doctor(outcome.value.data);
    Ok(())
}

fn render_doctor(view: atelier_app::health::DoctorView) {
    println!("Database: {}", view.db_path.display());
    println!("State: {}", view.state_dir.display());
    if view.fix {
        println!("Repair:");
        println!("  local_cache: repaired");
        println!("  record_files: unchanged");
    }
    println!("Install health:");
    println!("  config: {}", if view.config_ok { "ok" } else { "not ok" });
    println!(
        "  ignored_runtime_paths: {}",
        if view.ignore_rules_current {
            "ok"
        } else {
            "not ok"
        }
    );
    println!("Cache rebuild:");
    println!(
        "  state_dir: {}",
        if view.state_dir_ok { "ok" } else { "not ok" }
    );
    println!(
        "  rebuild_ready: {}",
        if view.rebuild_ready { "ok" } else { "not ok" }
    );
    println!(
        "  cache_fresh: {}",
        if view.cache_fresh { "ok" } else { "not ok" }
    );
    println!(
        "  tables: {}",
        atelier_sqlite::DOMAIN_CACHE_TABLES.join(", ")
    );
    println!("Cache health:");
    println!("  cache_dir: {}", view.cache_dir_status);
    println!(
        "  source_metadata: {}",
        if view.cache_fresh { "ok" } else { "stale" }
    );
    println!("Cache database:");
    println!(
        "  database: {}",
        if view.runtime_db_available {
            "ok"
        } else {
            "missing"
        }
    );
    println!("Legacy health:");
    for (key, value) in view.health {
        println!("{key}: {}", if value { "ok" } else { "not ok" });
    }
}

pub fn rebuild(state_dir: &Path, db_path: &Path) -> Result<()> {
    atelier_app::rebuild::run(state_dir, db_path)?;
    println!("Runtime state rebuilt");
    println!("State:    {}", state_dir.display());
    println!("Database: {}", db_path.display());
    println!();
    println!("Next Commands");
    println!("-------------");
    println!("  atelier doctor");
    println!("  atelier check");
    Ok(())
}

pub fn validate_priority(priority: &str) -> Result<()> {
    IssuePriority::from_cli_input(priority)
        .map(|_| ())
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::test_support::DomainCacheFixture;
    use tempfile::tempdir;

    fn setup_test_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db = Database::open(&dir.path().join("test.db")).unwrap();
        (db, dir)
    }

    #[test]
    fn concurrent_note_appends_keep_every_note() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let now = Utc::now();
        RecordStore::new(&state_dir)
            .write_issue_atomic(&CanonicalIssueRecord {
                issue: Issue {
                    id: "atelier-note".to_string(),
                    title: "Concurrent notes".to_string(),
                    description: None,
                    status: "todo".to_string(),
                    issue_type: "task".to_string(),
                    priority: "medium".to_string(),
                    fields: BTreeMap::new(),
                    parent_id: None,
                    created_at: now,
                    updated_at: now,
                    closed_at: None,
                },
                labels: Vec::new(),
                sections: IssueSections::unchecked_from_body(Some(
                    "## Description\n\nNotes race.\n\n## Outcome\n\nNo note is lost.",
                )),
                relationships: Relationships::default(),
            })
            .unwrap();

        let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
        let writers = (0..8)
            .map(|writer| {
                let state_dir = state_dir.clone();
                let barrier = std::sync::Arc::clone(&barrier);
                std::thread::spawn(move || {
                    barrier.wait();
                    for index in 0..25 {
                        append_note(
                            &state_dir,
                            "atelier-note",
                            "note",
                            &format!("writer {writer} note {index}"),
                        )
                        .unwrap();
                    }
                })
            })
            .collect::<Vec<_>>();
        for writer in writers {
            writer.join().unwrap();
        }

        let notes = RecordStore::new(&state_dir)
            .load_issue_by_id("atelier-note")
            .unwrap()
            .sections
            .notes
            .unwrap();
        let mut kept = notes
            .lines()
            .filter_map(|line| line.split_once("[note] ").map(|(_, text)| text.to_string()))
            .collect::<Vec<_>>();
        kept.sort();
        let mut expected = (0..8)
            .flat_map(|writer| (0..25).map(move |index| format!("writer {writer} note {index}")))
            .collect::<Vec<_>>();
        expected.sort();
        assert_eq!(kept, expected);
    }

    #[test]
    fn dependency_rows_include_context_and_open_blocker_marker() {
        let (db, _dir) = setup_test_db();
        let blocked = db
            .cache_fixture_issue("Blocked issue", None, "medium")
            .unwrap();
        let blocker = db
            .cache_fixture_issue("Blocking issue", None, "high")
            .unwrap();
        db.add_dependency(&blocked, &blocker).unwrap();

        let rows = dependency_rows_for_text(&db, db.get_blockers(&blocked).unwrap(), true).unwrap();

        assert_eq!(rows.len(), 1);
        assert!(rows[0].contains(&format_issue_id(&blocker)));
        assert!(rows[0].contains("[todo] high - Blocking issue"));
        assert!(rows[0].contains("(open blocker)"));
    }

    #[test]
    fn subissue_summary_counts_statuses_and_priorities() {
        let (db, _dir) = setup_test_db();
        let parent = db.cache_fixture_issue("Parent", None, "high").unwrap();
        let child_a = db
            .cache_fixture_subissue(&parent, "First child", None, "high")
            .unwrap();
        let child_b = db
            .cache_fixture_subissue(&parent, "Second child", None, "low")
            .unwrap();
        db.cache_fixture_close(&child_b).unwrap();

        let subissues = db.get_subissues(&parent).unwrap();
        let summary = subissue_summary(
            subissues.len(),
            subissues
                .iter()
                .map(|issue| (issue.status.as_str(), issue.priority.as_str())),
        );

        assert!(summary.contains("2 total"));
        assert!(summary.contains("done=1"));
        assert!(summary.contains("todo=1"));
        assert!(summary.contains("high=1"));
        assert!(summary.contains("low=1"));
        assert!(subissues.iter().any(|issue| issue.id == child_a));
    }
}
