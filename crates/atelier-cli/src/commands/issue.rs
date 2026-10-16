use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Local, Utc};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::commands::issue_workflow::{
    format_status_with_category, issue_status_category, issue_status_label,
    load_issue_workflow_policy,
};
use crate::human_output::{
    plural_suffix, render_footer_section, render_issue_inventory, render_issue_inventory_quiet,
    render_lines_section, render_metadata, render_text_section, DisplayRole, FooterAction,
    IssueListPanel, IssueListRow, RenderContext,
};
use atelier_app::issue_inventory::{issue_inventory, IssueInventoryFilters};
use atelier_app::issue_read::ObjectiveReadSummary;
use atelier_app::objective_graph::issue_blocks_work;
use atelier_app::record_index::RecordIndex;
use atelier_app::workflow_policy::WorkflowPolicy;
use atelier_core::{Issue, ISSUE_NOTE_KINDS};
use atelier_records::{validate_issue_type, validate_priority};
use atelier_records::{CanonicalIssueRecord, IssueSections, RecordStore, Relationships};

const OBJECTIVE_ROLLUP_LIMIT: usize = 5;
const MAX_ISSUE_TITLE_LEN: usize = 512;
const MAX_ISSUE_DESCRIPTION_LEN: usize = 64 * 1024;

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

fn label_value(labels: &[String], prefix: &str) -> Option<String> {
    labels
        .iter()
        .find_map(|label| label.strip_prefix(prefix).map(str::to_string))
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
    render_issue_show_human(index, record)
}

fn render_issue_show_human(index: &RecordIndex, record: &CanonicalIssueRecord) -> Result<()> {
    let issue = &record.issue;
    let canonical_id = &issue.id;
    let workflow_policy = load_issue_workflow_policy()?;
    let objective_summary = atelier_app::issue_read::objective_read_summary(
        index,
        canonical_id,
        workflow_policy.as_ref(),
    )?;
    let status_category = issue_status_category(workflow_policy.as_ref(), &issue.status);
    let identity = format!(
        "{} [{}] {} - {}",
        issue.id,
        issue.issue_type,
        format_status_with_category(status_category.as_deref(), &issue.status),
        issue.title
    );
    println!("{identity}");
    println!("{}", "=".repeat(identity.len()));
    println!("Status:   {}", issue.status);
    println!(
        "Category: {}",
        status_category.unwrap_or_else(|| "(unknown)".to_string())
    );
    println!("Type:     {}", issue.issue_type);
    println!("Priority: {}", issue.priority);
    println!("Created:  {}", format_human_datetime(issue.created_at));
    println!("Updated:  {}", format_human_datetime(issue.updated_at));
    if let Some(closed_at) = &issue.closed_at {
        println!("Closed:   {}", format_human_datetime(*closed_at));
    }
    if let Some(owner) = label_value(&record.labels, "owner:") {
        println!("Owner:    {owner}");
    }
    if let Some(assignee) = label_value(&record.labels, "assignee:") {
        println!("Assignee: {assignee}");
    }
    if let Some(size) = label_value(&record.labels, atelier_app::lint::SIZE_LABEL_PREFIX) {
        println!("Size:     {}", size.to_uppercase());
    }
    if !record.labels.is_empty() {
        println!("Labels:   {}", record.labels.join(", "));
    }
    if let Some(path) = canonical_issue_path(canonical_id)? {
        println!("File:     {}", path.display());
    }
    render_parent_context(index, canonical_id)?;
    render_transition_readiness(index, canonical_id)?;

    for name in IssueSections::ALL_NAMES {
        print_text_section(name.title(), record.sections.section(name));
    }

    render_dependency_section(index, "Blocked by", index.get_blockers(canonical_id), true)?;
    if let Some((parent, inherited)) =
        atelier_app::objective_graph::inherited_blockers(index, canonical_id)
    {
        render_dependency_section(
            index,
            &format!("Blocked via parent {parent}"),
            inherited,
            true,
        )?;
    }
    render_dependency_section(index, "Blocking", index.get_blocking(canonical_id), false)?;
    render_issue_link_section(index, canonical_id, &record.relationships)?;
    render_objective_rollup_section(objective_summary.as_ref());
    render_subissue_section(index, canonical_id)?;
    render_impact_section(index, canonical_id)?;
    render_command_footer(canonical_id)?;
    Ok(())
}

fn render_objective_rollup_section(summary: Option<&ObjectiveReadSummary>) {
    let Some(summary) = summary else {
        return;
    };
    let health = summary
        .scope
        .health(summary.relationships.open_blockers.len());
    print_panel(render_metadata(
        Some("Objective Rollup"),
        &[
            ("Health", health.to_string()),
            (
                "Scope",
                format!(
                    "{} scoped issue{}",
                    summary.scope.total(),
                    plural_suffix(summary.scope.total())
                ),
            ),
            (
                "Buckets",
                format!(
                    "active {}, ready {},{} blocked {}, done {}, backlog {}",
                    summary.scope.work.active.len(),
                    summary.scope.work.ready.len(),
                    [
                        ("needs a human", summary.scope.work.human.len()),
                        ("context", summary.scope.work.context.len()),
                    ]
                    .into_iter()
                    .filter(|(_, count)| *count > 0)
                    .map(|(label, count)| format!(" {label} {count},"))
                    .collect::<String>(),
                    summary.scope.work.blocked.len(),
                    summary.scope.work.done.len(),
                    summary.scope.work.backlog.len()
                ),
            ),
            (
                "Relationships",
                format!(
                    "advances {}, open blockers {}, validating evidence {}, other {}",
                    summary.relationships.advances_roots.len(),
                    summary.relationships.open_blockers.len(),
                    summary.evidence.linked_validating_evidence,
                    summary.relationships.other_links
                ),
            ),
            (
                "Evidence Gates",
                format!(
                    "linked validating evidence {}; scoped issues without evidence {}",
                    summary.evidence.linked_validating_evidence,
                    summary.evidence.scoped_issues_without_evidence
                ),
            ),
        ],
    ));

    for (title, rows) in [
        ("Active Work", &summary.scope.work.active),
        ("Ready Work", &summary.scope.work.ready),
        ("Needs a Human", &summary.scope.work.human),
        ("Parents and Missions", &summary.scope.work.context),
        ("Blocked Work", &summary.scope.work.blocked),
        ("Done Work", &summary.scope.work.done),
    ] {
        if !rows.is_empty() {
            print_panel(
                IssueListPanel::from_work_rows(title, rows)
                    .limit(OBJECTIVE_ROLLUP_LIMIT)
                    .render(RenderContext::for_stdout()),
            );
        }
    }

    if !summary.recently_updated.is_empty() {
        print_panel(render_lines_section(
            "Recent Activity Facts",
            summary
                .recently_updated
                .iter()
                .take(OBJECTIVE_ROLLUP_LIMIT)
                .map(|issue| {
                    format!(
                        "  updated {} [{}] {} - {}",
                        issue.id,
                        issue.classification.label(),
                        issue.priority,
                        issue.title
                    )
                }),
        ));
    }
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
            relation.relation_type, other.id, other.status, other.priority, other.title
        ));
    }
    rows.sort();
    print_panel(render_lines_section(
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

fn render_transition_readiness(index: &RecordIndex, canonical_id: &str) -> Result<()> {
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
    lines.push(format!(
        "  options: atelier issue transition {canonical_id}"
    ));
    print_panel(render_lines_section("Transition Readiness", lines));
    Ok(())
}

fn render_parent_context(index: &RecordIndex, canonical_id: &str) -> Result<()> {
    let issue = index.require_issue(canonical_id)?;
    let line = match issue.parent_id {
        Some(parent_id) => {
            let parent = index.require_issue(&parent_id)?;
            format!(
                "Parent: {} [{}] {} - {}",
                parent.id, parent.status, parent.priority, parent.title
            )
        }
        None => "Parent: (none)".to_string(),
    };
    print_panel(render_lines_section("Hierarchy", [line]));
    Ok(())
}

fn print_text_section(title: &str, body: Option<&str>) {
    print_panel(render_text_section(title, body));
}

fn render_dependency_section(
    index: &RecordIndex,
    title: &str,
    ids: Vec<String>,
    blockers: bool,
) -> Result<()> {
    let rows = dependency_rows_for_text(index, ids, blockers)?;
    print_panel(render_lines_section(
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
                issue.id,
                issue_status_label(workflow_policy.as_ref(), &issue.status),
                issue.priority,
                issue.title,
                marker
            ))
        })
        .collect()
}

fn render_subissue_section(index: &RecordIndex, canonical_id: &str) -> Result<()> {
    let subissues = index.get_subissues(canonical_id);
    let workflow_policy = load_issue_workflow_policy()?;
    if subissues.is_empty() {
        print_panel(render_lines_section("Subissues", Vec::<String>::new()));
        return Ok(());
    }

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
    let order = crate::commands::work_order::ordered_work_indices(&classified_subissues);
    let summary = subissue_summary(
        classified_subissues.len(),
        classified_subissues.iter().map(|(subissue, classified)| {
            (
                classified.classification.label(),
                subissue.priority.as_str(),
            )
        }),
    );
    let rows = order
        .into_iter()
        .map(|index| {
            let (subissue, classified) = &classified_subissues[index];
            IssueListRow {
                role: DisplayRole::for_work(classified.classification.label()),
                id: subissue.id.clone(),
                status: Some(classified.classification.label().to_string()),
                priority: subissue.priority.clone(),
                title: subissue.title.clone(),
                blockers: classified.open_blockers.len(),
                depth: 1,
            }
        })
        .collect();
    let mut lines = vec![summary];
    lines.extend(
        IssueListPanel::new("Subissues", rows)
            .render(RenderContext::for_stdout())
            .into_iter()
            .skip(1),
    );
    print_panel(render_lines_section("Subissues", lines));
    Ok(())
}

fn render_impact_section(index: &RecordIndex, canonical_id: &str) -> Result<()> {
    let affected = index.downstream_impact(canonical_id);
    if affected.is_empty() {
        print_panel(render_lines_section(
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
                    issue.id,
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
    print_panel(render_lines_section("Impact", lines));
    Ok(())
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

fn render_command_footer(canonical_id: &str) -> Result<()> {
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
            format!("atelier check {}", canonical_id),
        ),
        FooterAction::new(
            "Add a note",
            format!("atelier issue note {} \"...\"", canonical_id),
        ),
        FooterAction::new(
            "Show transition options",
            format!("atelier issue transition {}", canonical_id),
        ),
        FooterAction::new(
            "Execute a transition",
            format!("atelier issue transition {} <transition>", canonical_id),
        ),
    ]);
    print_panel(render_footer_section("Next Commands", actions));
    Ok(())
}

fn print_panel(lines: Vec<String>) {
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
    let view = issue_inventory(
        index,
        workflow_policy.as_ref(),
        IssueInventoryFilters {
            status: (status != "all").then_some(status),
            category,
            issue_type,
            label,
            priority,
        },
        limit,
    )?;
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

fn format_human_datetime(timestamp: DateTime<Utc>) -> String {
    timestamp
        .with_timezone(&Local)
        .format("%Y-%m-%d %H:%M %Z")
        .to_string()
}

pub struct LifecycleCreateInput<'a> {
    pub title: &'a str,
    pub body: Option<&'a str>,
    pub priority: &'a str,
    pub issue_type: &'a str,
    pub labels: &'a [String],
    pub parent: Option<&'a str>,
    pub status: Option<&'a str>,
    pub strategy: Option<&'a str>,
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
    validate_priority(input.priority)?;
    validate_configured_issue_type(state_dir, input.issue_type)?;
    let parent_id = input
        .parent
        .map(|parent| resolve_id(index, parent))
        .transpose()?;

    let store = atelier_app::project_config::record_store(state_dir)?;
    let now = Utc::now();
    let id = store.allocate_issue_id()?;
    let initial_status = match input.status {
        Some(status) => requested_initial_status(state_dir, input.issue_type, status)?,
        None => lifecycle_initial_status(state_dir, input.issue_type)?,
    };
    let description = input.body.map(str::to_string);
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
            strategy: input
                .strategy
                .map(|path| mission_strategy(state_dir, input.issue_type, path))
                .transpose()?,
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

    if input.quiet {
        println!("{}", record.issue.id);
        return Ok(());
    }
    if let Some(parent_id) = &parent_id {
        println!("Created subissue {} under {}", record.issue.id, parent_id);
    } else {
        let kind = if record.issue.issue_type == "mission" {
            "mission objective"
        } else {
            "issue"
        };
        println!(
            "Created {kind} {} - {}",
            record.issue.id, record.issue.title
        );
        println!("Type:     {}", record.issue.issue_type);
        println!("Priority: {}", record.issue.priority);
    }
    let file_path = canonical_issue_path_from_state(state_dir, &id);
    println!("File: {}", file_path.display());
    print_panel(render_footer_section(
        "Next Commands",
        [
            FooterAction::new("Edit issue Markdown", file_path.display().to_string()),
            FooterAction::new("Validate this issue", format!("atelier check {id}")),
            FooterAction::new("Inspect this issue", format!("atelier issue show {id}")),
            FooterAction::new(
                "Inspect tracked work transitions",
                format!("atelier issue transition {id}"),
            ),
        ],
    ));
    Ok(())
}

fn lifecycle_issue_sections(input: &LifecycleCreateInput<'_>) -> Result<IssueSections> {
    if let Some(body) = input.body.filter(|body| has_level_two_heading(body)) {
        return atelier_records::parse_issue_sections_for_type(
            body,
            Path::new("<issue create --body>"),
            input.issue_type,
        )
        .context("Invalid sectioned --body");
    }
    Ok(IssueSections::for_type(input.issue_type, input.body))
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

/// New work starts in the workflow's initial status, or in a todo-category
/// status one unchecked transition away from it, such as `backlog` via
/// `defer`. A status guarded by checks or required fields (an objective's
/// `ready`) or outside the todo category needs its transition.
fn requested_initial_status(state_dir: &Path, issue_type: &str, status: &str) -> Result<String> {
    let policy = load_repo_policy(state_dir)?;
    let workflow = policy.workflow_for_issue_type(issue_type)?;
    let allowed = std::iter::once(workflow.initial_status.clone())
        .chain(
            workflow
                .transitions
                .values()
                .filter(|transition| {
                    transition.from.contains(&workflow.initial_status)
                        && transition.validators.is_empty()
                        && transition.required_fields.is_empty()
                        && policy.status_category(&transition.to) == Some("todo")
                        && policy
                            .transitions_from_status(issue_type, &transition.to)
                            .is_ok_and(|exits| !exits.is_empty())
                })
                .map(|transition| transition.to.clone()),
        )
        .collect::<std::collections::BTreeSet<_>>();
    if !allowed.contains(status) {
        bail!(
            "issue_create_status_invalid: {issue_type} work cannot be created in status '{status}'; use one of: {}. Other statuses are reached with atelier issue transition.",
            allowed.into_iter().collect::<Vec<_>>().join(", ")
        );
    }
    Ok(status.to_string())
}

/// Only missions link a strategy; its path is the canonical link and must
/// name an existing file.
fn mission_strategy(
    state_dir: &Path,
    issue_type: &str,
    path: &str,
) -> Result<atelier_core::IssueStrategy> {
    if issue_type != "mission" {
        bail!("--strategy applies to missions; {issue_type} records do not link a strategy");
    }
    let strategy = atelier_core::IssueStrategy::new(path)?;
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "cannot determine repository root from {}",
            state_dir.display()
        )
    })?;
    if !repo_root.join(&strategy.path).is_file() {
        bail!("strategy file {path} does not exist; write the strategy first, then link it");
    }
    Ok(strategy)
}

pub struct UpdateInput<'a> {
    pub issue_ref: &'a str,
    pub title: Option<&'a str>,
    pub priority: Option<&'a str>,
    pub issue_type: Option<&'a str>,
    pub body: Option<&'a str>,
    /// `Some(None)` clears a mission's strategy link.
    pub strategy: Option<Option<&'a str>>,
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
    if let Some(body) = input.body {
        if body.len() > MAX_ISSUE_DESCRIPTION_LEN {
            bail!(
                "Description exceeds maximum length of {} bytes",
                MAX_ISSUE_DESCRIPTION_LEN
            );
        }
        if load_repo_policy(state_dir)?.status_category(&record.issue.status) == Some("done") {
            bail!(
                "issue_body_done: {id} is {}; its done claim and proof were judged against the current acceptance, so its body cannot be revised. File follow-up work instead.",
                record.issue.status
            );
        }
        let parsed = atelier_records::parse_issue_sections_for_type(
            body,
            Path::new("<issue update --body>"),
            &record.issue.issue_type,
        )
        .context("Invalid sectioned --body")?;
        if parsed.notes.is_some() || parsed.close_reason.is_some() {
            bail!("--body replaces the typed sections only; add notes with `atelier issue note` and close reasons with `atelier issue transition`");
        }
        record.sections.body = parsed.body;
        changed_fields.push("body");
    }
    match input.strategy {
        Some(Some(path)) => {
            record.issue.strategy =
                Some(mission_strategy(state_dir, &record.issue.issue_type, path)?);
            changed_fields.push("strategy");
        }
        Some(None) => {
            record.issue.strategy = None;
            changed_fields.push("strategy");
        }
        None => {}
    }
    for label in input.labels {
        // A record has one size; a new size label replaces the old one.
        if label.starts_with(atelier_app::lint::SIZE_LABEL_PREFIX) {
            record
                .labels
                .retain(|existing| !existing.starts_with(atelier_app::lint::SIZE_LABEL_PREFIX));
        }
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
        bail!("Nothing to update. Use --title, --priority, --issue-type, --body, --strategy, --no-strategy, --label, --remove-label, or --parent. Use `atelier issue note <id> \"...\"` for notes or `atelier issue transition <id> <transition>` for status changes");
    }
    record.issue.updated_at = now;
    validate_issue_record_against_workflow(state_dir, &record.issue)?;
    if input.body.is_some() {
        // A revised body must pass the same per-record rules as `atelier check`.
        let repo_root = state_dir.parent().ok_or_else(|| {
            anyhow!(
                "cannot determine repository root from {}",
                state_dir.display()
            )
        })?;
        let findings = atelier_app::lint::issue_findings([&record], repo_root, false);
        if !findings.is_empty() {
            bail!(
                "Invalid --body for {id}: {}",
                findings
                    .iter()
                    .map(|finding| finding.message.as_str())
                    .collect::<Vec<_>>()
                    .join("; ")
            );
        }
    }
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
        println!("Parent:   {}", parent);
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
    println!("Added note to issue {}", issue_id);
    Ok(())
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.contains(&value) {
        values.push(value);
    }
}

pub fn lint(state_dir: &Path, issue_ref: Option<&str>) -> Result<()> {
    let view = atelier_app::lint::lint(atelier_app::lint::LintRequest {
        state_dir,
        issue_ref,
    })?;
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
            sections: atelier_records::parse_issue_sections_for_type(
                "## Goal\n\nFixture.\n\n## Done when\n\nFixture.",
                Path::new("<fixture>"),
                "task",
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
                sections: atelier_records::parse_issue_sections_for_type(
                    "## Goal\n\nNotes race.\n\n## Done when\n\nNo note is lost.",
                    Path::new("<fixture>"),
                    "task",
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
            Vec::new(),
        )
        .unwrap();

        let rows = dependency_rows_for_text(&index, index.get_blockers(blocked), true).unwrap();

        assert_eq!(rows.len(), 1);
        assert!(rows[0].contains(blocker));
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
