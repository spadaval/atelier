use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Local, Utc};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::commands::issue_workflow::{
    format_status_with_category, issue_blocks_work, issue_status_category, issue_status_label,
    load_issue_workflow_policy, open_blocker_ids_with_policy,
};
use crate::commands::work_order::WorkOrderRow;
use crate::human_output::{
    render_issue_inventory, render_issue_inventory_quiet, DisplayRole, FooterAction, FooterPanel,
    IssueListPanel, IssueListRow, LinesPanel, MetadataPanel, Panel, RenderContext, TextPanel,
};
use crate::utils::format_issue_id;
use atelier_app::issue_inventory::{
    issue_inventory, IssueInventoryFilters, IssueInventoryLimit, IssueInventoryRequest,
    IssueInventorySnapshot, IssueInventoryStatusFilter,
};
use atelier_app::issue_read::{ObjectiveIssueSummary, ObjectiveReadSummary};
use atelier_app::record_index::RecordIndex;
use atelier_app::workflow_policy::WorkflowPolicy;
use atelier_core::{Issue, IssuePriority, ISSUE_NOTE_KINDS};
use atelier_records::validate_issue_type;
use atelier_records::{CanonicalIssueRecord, IssueSections, RecordStore, Relationships};

const OBJECTIVE_ROLLUP_LIMIT: usize = 5;
const MAX_ISSUE_TITLE_LEN: usize = 512;
const MAX_ISSUE_DESCRIPTION_LEN: usize = 64 * 1024;

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

pub fn resolve_id(index: &RecordIndex, issue_ref: &str) -> Result<String> {
    if let Some(id) = index.resolve_issue_ref(issue_ref)? {
        return Ok(id);
    }

    if let Some(actual_kind) = index.record_kind_for_id(issue_ref) {
        bail!(
            "{} is a {} record, not an issue record. Use `{}`.",
            issue_ref,
            actual_kind,
            show_command_for_kind(actual_kind, issue_ref)
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

fn issue_id_for_agent(index: &RecordIndex, issue: &Issue) -> Result<String> {
    let _ = index;
    Ok(format_issue_id(&issue.id))
}

fn label_value(labels: &[String], prefix: &str) -> Option<String> {
    labels
        .iter()
        .find_map(|label| label.strip_prefix(prefix).map(str::to_string))
}

fn dependency_summary(index: &RecordIndex, id: &str) -> Result<DependencySummary> {
    let issue = index
        .get_issue(id)
        .ok_or_else(|| anyhow!("Dependency issue {} was not found", format_issue_id(id)))?;
    Ok(DependencySummary {
        id: issue_id_for_agent(index, &issue)?,
    })
}

fn issue_object_from_record(
    index: &RecordIndex,
    record: &CanonicalIssueRecord,
) -> Result<IssueObject> {
    issue_object_from_parts(
        index,
        record.issue.clone(),
        record.labels.clone(),
        Some(record.sections.clone()),
        record.relationships.clone(),
    )
}

fn issue_object_from_parts(
    index: &RecordIndex,
    issue: Issue,
    labels: Vec<String>,
    sections: Option<IssueSections>,
    relationships: Relationships,
) -> Result<IssueObject> {
    let parent = match &issue.parent_id {
        Some(parent_id) => Some(dependency_summary(index, parent_id)?.id),
        None => None,
    };

    Ok(IssueObject {
        id: issue_id_for_agent(index, &issue)?,
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

fn canonical_issue_path_from_state(state_dir: &Path, issue_id: &str) -> PathBuf {
    state_dir.join("issues").join(format!("{issue_id}.md"))
}

fn canonical_issue_path(issue_id: &str) -> Result<Option<PathBuf>> {
    Ok(find_state_dir_from_cwd()?
        .map(|state_dir| canonical_issue_path_from_state(&state_dir, issue_id)))
}

pub fn show(index: &RecordIndex, issue_ref: &str) -> Result<()> {
    let id = resolve_id(index, issue_ref)?;
    let record = index
        .issue_record(&id)
        .ok_or_else(|| anyhow!("Issue {id} not found"))?;
    let object = issue_object_from_record(index, record)?;
    render_issue_show_human(index, &id, &object)
}

fn render_issue_show_human(
    index: &RecordIndex,
    canonical_id: &str,
    object: &IssueObject,
) -> Result<()> {
    let workflow_policy = load_issue_workflow_policy()?;
    let objective_summary = atelier_app::issue_read::objective_read_summary(
        index,
        canonical_id,
        workflow_policy.as_ref(),
    )?;
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
    render_parent_context(index, canonical_id)?;
    render_transition_readiness(index, canonical_id, object)?;

    if let Some(sections) = &object.sections {
        for name in IssueSections::ALL_NAMES {
            print_text_section(name.title(), sections.section(name));
        }
    } else {
        print_text_section("Description", object.description.as_deref());
    }

    render_dependency_section(index, "Blocked by", index.get_blockers(canonical_id), true)?;
    render_dependency_section(index, "Blocking", index.get_blocking(canonical_id), false)?;
    render_issue_link_section(index, canonical_id, &object.relationships)?;
    render_objective_rollup_section(objective_summary.as_ref());
    render_subissue_section(index, canonical_id)?;
    render_impact_section(index, canonical_id)?;
    render_command_footer(canonical_id, object)?;
    Ok(())
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
        "Active Work",
        &summary.scope.active,
        summary.scope.totals.active,
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
                .take(OBJECTIVE_ROLLUP_LIMIT)
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
            .limit(OBJECTIVE_ROLLUP_LIMIT),
    );
}

fn render_issue_link_section(
    index: &RecordIndex,
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
        let other = index.require_issue(&relation.id)?;
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

pub fn transition_options(index: &RecordIndex, issue_ref: &str, verbose: bool) -> Result<()> {
    let id = resolve_id(index, issue_ref)?;
    let issue = index.require_issue(&id)?;
    let options = crate::commands::workflow::issue_transition_options(index, issue_ref)?;
    crate::commands::workflow::print_issue_transition_options(&issue, &options, verbose);
    Ok(())
}

fn render_transition_readiness(
    index: &RecordIndex,
    canonical_id: &str,
    object: &IssueObject,
) -> Result<()> {
    let mut lines = Vec::new();
    match crate::commands::workflow::issue_transition_options(index, canonical_id) {
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

#[derive(Debug, Clone)]
pub(crate) struct EvidenceGateStatus {
    pub passed: bool,
}

pub(crate) fn issue_evidence_gate_status(
    index: &RecordIndex,
    issue: &Issue,
    sections: Option<&IssueSections>,
) -> Result<EvidenceGateStatus> {
    let _ = sections;
    let layout = atelier_app::storage_layout::StorageLayout::discover()?;
    let gate = atelier_app::evidence_freshness::issue_evidence_gate_status(
        index,
        layout.repo_root(),
        &issue.id,
        1,
        None,
    )?;
    Ok(EvidenceGateStatus {
        passed: gate.passed,
    })
}

fn render_parent_context(index: &RecordIndex, canonical_id: &str) -> Result<()> {
    let issue = index.require_issue(canonical_id)?;
    let line = match issue.parent_id {
        Some(parent_id) => {
            let parent = index.require_issue(&parent_id)?;
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
    index: &RecordIndex,
    title: &str,
    ids: Vec<String>,
    blockers: bool,
) -> Result<()> {
    let rows = dependency_rows_for_text(index, ids, blockers)?;
    print_panel(LinesPanel::new(
        title,
        rows.into_iter().map(|row| format!("  {row}")),
    ));
    Ok(())
}

fn dependency_rows_for_text(
    index: &RecordIndex,
    ids: Vec<String>,
    blockers: bool,
) -> Result<Vec<String>> {
    let workflow_policy = load_issue_workflow_policy()?;
    ids.into_iter()
        .map(|id| {
            let issue = index.require_issue(&id)?;
            let marker = if blockers && issue_blocks_work(workflow_policy.as_ref(), &issue) {
                " (open blocker)"
            } else {
                ""
            };
            Ok(format!(
                "{} [{}] {} - {}{}",
                issue_id_for_agent(index, &issue)?,
                issue_status_label(workflow_policy.as_ref(), &issue.status),
                issue.priority,
                issue.title,
                marker
            ))
        })
        .collect()
}

fn render_subissue_section(index: &RecordIndex, canonical_id: &str) -> Result<()> {
    let mut subissues = index.get_subissues(canonical_id);
    let workflow_policy = load_issue_workflow_policy()?;
    if subissues.is_empty() {
        print_panel(LinesPanel::new("Subissues", Vec::<String>::new()));
        return Ok(());
    }

    subissues = order_issues_by_work(index, workflow_policy.as_ref(), subissues)?;
    let classified_subissues = subissues
        .into_iter()
        .map(|subissue| {
            let classified = atelier_app::work_classification::classify_issue(
                index,
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

fn render_impact_section(index: &RecordIndex, canonical_id: &str) -> Result<()> {
    let affected = index.downstream_impact(canonical_id);
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
                    issue_id_for_agent(index, issue)?,
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
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issues: Vec<Issue>,
) -> Result<Vec<Issue>> {
    let rows = issues
        .iter()
        .map(|issue| work_order_row_for_issue(index, workflow_policy, issue))
        .collect::<Result<Vec<_>>>()?;
    let mut keyed = issues.into_iter().map(Some).collect::<Vec<_>>();
    Ok(crate::commands::work_order::ordered_work_indices(&rows)
        .into_iter()
        .filter_map(|index| keyed[index].take())
        .collect())
}

fn work_order_row_for_issue(
    index: &RecordIndex,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> Result<WorkOrderRow> {
    Ok(WorkOrderRow {
        id: format_issue_id(&issue.id),
        status_category: issue_status_category(workflow_policy, &issue.status),
        priority: issue.priority.clone(),
        updated_at: issue.updated_at,
        open_blockers: open_blocker_ids_with_policy(index, workflow_policy, &issue.id)?,
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

#[allow(clippy::too_many_arguments)]
pub fn list_inventory(
    index: &RecordIndex,
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
        if workflow_policy
            .as_ref()
            .is_some_and(|policy| !policy.statuses.contains_key(status))
        {
            bail!("Invalid issue status '{status}'. Use an exact workflow status or `all`.");
        }
        IssueInventoryStatusFilter::Exact(status.to_string())
    };
    if let Some(category) = category {
        let policy = workflow_policy
            .as_ref()
            .ok_or_else(|| anyhow!("--category requires a workflow policy"))?;
        if !policy
            .statuses
            .values()
            .any(|status| status.category == category)
        {
            bail!("Invalid issue category '{category}'. Use a category from the workflow policy.");
        }
    }
    let limit = IssueInventoryLimit::new(limit)?;
    let issues = index.list_issues(Some("all"), label, priority)?;
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
    index: &RecordIndex,
    state_dir: &Path,
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
    let parent_id = input
        .parent
        .map(|parent| resolve_id(index, parent))
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
            strategy: None,
            workflow_branch: None,
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
        index,
        &record.issue,
        parent_id.as_deref(),
    )?;
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
    if let Some(body) = input
        .body
        .or(input.description)
        .filter(|body| has_level_two_heading(body))
    {
        if !input.constraints.is_empty() || !input.risks.is_empty() || !input.validation.is_empty()
        {
            bail!("sectioned mission content cannot be combined with mission section flags; put the content in --body");
        }
        return atelier_records::parse_issue_sections_for_type(
            body,
            Path::new("<issue create --body>"),
            input.issue_type,
        )
        .context("Invalid sectioned --body");
    }
    let description = lifecycle_issue_description(input);
    Ok(IssueSections::for_type(
        input.issue_type,
        description.as_deref(),
    ))
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

pub fn update_lifecycle(
    index: &RecordIndex,
    state_dir: &Path,
    input: UpdateInput<'_>,
) -> Result<()> {
    let id = resolve_id(index, input.issue_ref)?;
    let previous = index.require_issue(&id)?;
    let requested_parent_id = input
        .parent
        .map(|parent| parent.map(|parent| resolve_id(index, parent)).transpose())
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
        record.sections.change_type(issue_type);
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
        index,
        &record.issue,
        parent_id.as_deref(),
    )?;
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

pub fn lint(state_dir: &Path, issue_ref: Option<&str>) -> Result<()> {
    let outcome = atelier_app::lint::lint(atelier_app::Request {
        input: atelier_app::lint::LintRequest {
            state_dir,
            issue_ref,
        },
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

pub fn validate_priority(priority: &str) -> Result<()> {
    IssuePriority::from_cli_input(priority)
        .map(|_| ())
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_records::issue_relationship_target;
    use tempfile::tempdir;

    fn fixture_record(id: &str, title: &str, status: &str, priority: &str) -> CanonicalIssueRecord {
        let now = Utc::now();
        CanonicalIssueRecord {
            issue: Issue {
                id: id.to_string(),
                title: title.to_string(),
                description: None,
                status: status.to_string(),
                issue_type: "task".to_string(),
                priority: priority.to_string(),
                strategy: None,
                workflow_branch: None,
                parent_id: None,
                created_at: now,
                updated_at: now,
                closed_at: None,
            },
            labels: Vec::new(),
            sections: atelier_records::parse_issue_sections(
                "## Goal\n\nFixture.\n\n## Done when\n\nFixture.",
                Path::new("<fixture>"),
            )
            .unwrap(),
            relationships: Relationships::default(),
        }
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
                    strategy: None,
                    workflow_branch: None,
                    parent_id: None,
                    created_at: now,
                    updated_at: now,
                    closed_at: None,
                },
                labels: Vec::new(),
                sections: atelier_records::parse_issue_sections(
                    "## Goal\n\nNotes race.\n\n## Done when\n\nNo note is lost.",
                    Path::new("<fixture>"),
                )
                .unwrap(),
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
        let blocked = "atelier-blocked";
        let blocker = "atelier-blocker";
        let mut blocker_record = fixture_record(blocker, "Blocking issue", "todo", "high");
        blocker_record
            .relationships
            .blocks
            .push(issue_relationship_target(blocked));
        let index = RecordIndex::from_records(
            vec![
                fixture_record(blocked, "Blocked issue", "todo", "medium"),
                blocker_record,
            ],
            Vec::new(),
        )
        .unwrap();

        let rows = dependency_rows_for_text(&index, index.get_blockers(blocked), true).unwrap();

        assert_eq!(rows.len(), 1);
        assert!(rows[0].contains(&format_issue_id(blocker)));
        assert!(rows[0].contains("[todo] high - Blocking issue"));
        assert!(rows[0].contains("(open blocker)"));
    }

    #[test]
    fn subissue_summary_counts_statuses_and_priorities() {
        let mut parent = fixture_record("atelier-parent", "Parent", "todo", "high");
        parent.issue.issue_type = "epic".to_string();
        for child in ["atelier-child-a", "atelier-child-b"] {
            parent
                .relationships
                .children
                .push(issue_relationship_target(child));
        }
        let child_a = "atelier-child-a";
        let index = RecordIndex::from_records(
            vec![
                parent,
                fixture_record(child_a, "First child", "todo", "high"),
                fixture_record("atelier-child-b", "Second child", "done", "low"),
            ],
            Vec::new(),
        )
        .unwrap();

        let subissues = index.get_subissues("atelier-parent");
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
