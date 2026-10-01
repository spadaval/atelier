use anyhow::{bail, Result};
use atelier_app::record_index::RecordIndex;
use std::collections::{BTreeMap, BTreeSet};

use atelier_app::mission_overview::mission_overview;
use atelier_app::work_classification::{WorkBuckets, WorkClassification, WorkRow};
use atelier_core::Issue;

use crate::human_output::{
    plural_suffix, render_labeled_lines, DisplayRole, FooterAction, FooterPanel, IssueHeadingRow,
    IssueListPanel, IssueListRow, LinesPanel, MetadataPanel, Page, RenderContext, TextStyle,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct MissionDashboardOptions {
    pub ready: bool,
    pub blocked: bool,
    pub active: bool,
    pub done: bool,
    pub all: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum DashboardFilter {
    Summary,
    Ready,
    Blocked,
    Active,
    Done,
    All,
}

impl MissionDashboardOptions {
    fn filter(self) -> Result<DashboardFilter> {
        let count = [self.ready, self.blocked, self.active, self.done, self.all]
            .into_iter()
            .filter(|flag| *flag)
            .count();
        if count > 1 {
            bail!("choose only one mission work filter");
        }
        Ok(if self.ready {
            DashboardFilter::Ready
        } else if self.blocked {
            DashboardFilter::Blocked
        } else if self.active {
            DashboardFilter::Active
        } else if self.done {
            DashboardFilter::Done
        } else if self.all {
            DashboardFilter::All
        } else {
            DashboardFilter::Summary
        })
    }
}

pub fn dashboards(quiet: bool) -> Result<()> {
    if quiet {
        println!("ready");
        println!("blocked");
        println!("active");
        println!("all");
        println!("missions");
        println!("mission");
        println!("epic");
        return Ok(());
    }

    Page::new("Atelier Work")
        .panel(LinesPanel::new(
            "Dashboards",
            [
                "ready    Small top-level executable work picker",
                "blocked  Repo-wide blocker triage",
                "missions Flat mission summaries with complete scope counts",
                "mission  Live mission orchestration dashboard",
                "epic     Focused epic execution dashboard",
            ],
        ))
        .panel(FooterPanel::new(
            "Next Commands",
            [
                FooterAction::new("Browse ready work", "atelier work ready"),
                FooterAction::new("Triage blockers", "atelier work blocked"),
                FooterAction::new("Compare missions", "atelier work missions"),
                FooterAction::new(
                    "Open mission dashboard",
                    "atelier work mission <mission-id>",
                ),
            ],
        ))
        .print(RenderContext::for_stdout());
    Ok(())
}

pub fn list(index: &RecordIndex, bucket: &str, quiet: bool) -> Result<()> {
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let mut buckets = atelier_app::work_classification::work_buckets(index, policy.as_ref())?;
    if bucket_needs_execution_gates(bucket) {
        if let Some(policy) = policy.as_ref() {
            apply_ready_execution_gates(index, policy, &mut buckets)?;
        }
    }
    if quiet {
        print_quiet(bucket, &buckets);
        return Ok(());
    }

    let mut page = Page::new("Atelier Work");
    match bucket {
        "ready" => {
            page = page.panel(work_bucket_panel("Ready Work", &buckets.ready));
        }
        "blocked" => {
            page = page.panel(work_bucket_panel("Blocked Work", &buckets.blocked));
        }
        "active" => {
            page = page.panel(work_bucket_panel("Active Work", &buckets.active));
        }
        "all" => {
            page = page
                .panel(work_bucket_panel("Active Work", &buckets.active))
                .panel(work_bucket_panel("Ready Work", &buckets.ready))
                .panel(work_bucket_panel("Blocked Work", &buckets.blocked))
                .panel(work_bucket_panel("Backlog Work", &buckets.backlog));
        }
        other => bail!("unknown work bucket '{other}'"),
    }
    page.print(RenderContext::for_stdout());
    Ok(())
}

fn bucket_needs_execution_gates(bucket: &str) -> bool {
    matches!(bucket, "ready" | "all")
}

fn apply_ready_execution_gates(
    index: &RecordIndex,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    buckets: &mut WorkBuckets,
) -> Result<()> {
    let candidate_ids = buckets
        .ready
        .iter()
        .map(|row| row.id.clone())
        .collect::<BTreeSet<_>>();
    let dependency = atelier_app::objective_graph::dependency_readiness_batch(
        index,
        policy,
        candidate_ids.iter().cloned(),
    )?;
    let mut ready = Vec::new();
    for mut row in std::mem::take(&mut buckets.ready) {
        let dependency_ready = dependency
            .ready_by_issue
            .get(&row.id)
            .copied()
            .unwrap_or(false);
        if dependency_ready {
            ready.push(row);
        } else {
            row.open_blockers.push("readiness gate".to_string());
            buckets.blocked.push(row);
        }
    }
    buckets.ready = ready;
    buckets
        .blocked
        .sort_by(|left, right| left.id.cmp(&right.id));
    Ok(())
}

pub fn missions(index: &RecordIndex, include_done: bool, quiet: bool) -> Result<()> {
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let overview = mission_overview(index, policy.as_ref(), include_done)?;
    if quiet {
        for mission in overview.missions {
            println!("{}", mission.issue.id);
        }
        return Ok(());
    }

    let context = RenderContext::for_stdout();
    let mut lines = Vec::new();
    for mission in overview.missions.iter().take(20) {
        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines.extend(
            IssueHeadingRow {
                indent: "",
                id: &mission.issue.id,
                issue_type: None,
                status_category: &mission.status_category,
                priority: &mission.issue.priority,
                title: &mission.issue.title,
            }
            .render(context),
        );
        let counts = &mission.totals;
        for (label, value) in [
            ("Status", mission.issue.status.clone()),
            (
                "Scope",
                format!(
                    "{} issues: {} active, {} ready, {} blocked, {} done, {} backlog",
                    mission.scope_ids.len(),
                    counts.active,
                    counts.ready,
                    counts.blocked,
                    counts.done,
                    counts.backlog
                ),
            ),
            ("Open blockers", mission.open_blockers.len().to_string()),
            (
                "Drill down",
                format!("atelier work mission {}", mission.issue.id),
            ),
        ] {
            lines.extend(render_labeled_lines(
                context,
                "  ",
                label,
                &value,
                TextStyle::Secondary,
            ));
        }
    }
    if overview.missions.is_empty() {
        lines.push("No missions match the current overview.".to_string());
    }
    if overview.missions.len() > 20 {
        lines.push(format!(
            "{} more missions omitted",
            overview.missions.len() - 20
        ));
    }
    if overview.unassigned_nonterminal > 0 || overview.linked_only_to_hidden_done_missions > 0 {
        lines.push(String::new());
        lines.extend(render_labeled_lines(
            context,
            "",
            "Unassigned",
            &format!("{} nonterminal issues", overview.unassigned_nonterminal),
            TextStyle::Secondary,
        ));
        lines.extend(render_labeled_lines(
            context,
            "",
            "Linked only to done missions",
            &format!(
                "{} nonterminal issues",
                overview.linked_only_to_hidden_done_missions
            ),
            TextStyle::Secondary,
        ));
    }
    lines.extend(render_labeled_lines(
        context,
        "",
        "Browse mission records",
        "atelier issue list --issue-type mission",
        TextStyle::Secondary,
    ));
    Page::new("Mission Overview")
        .panel(LinesPanel::new("", lines))
        .print(context);
    Ok(())
}

fn print_quiet(bucket: &str, buckets: &WorkBuckets) {
    let groups: &[&[WorkRow]] = match bucket {
        "ready" => &[&buckets.ready],
        "blocked" => &[&buckets.blocked],
        "active" => &[&buckets.active],
        "all" => &[
            &buckets.active,
            &buckets.ready,
            &buckets.blocked,
            &buckets.backlog,
        ],
        _ => &[&buckets.ready],
    };
    for rows in groups {
        for row in *rows {
            println!("{}", row.id);
        }
    }
}

fn work_bucket_panel(title: &str, rows: &[WorkRow]) -> IssueListPanel {
    IssueListPanel::new(
        title,
        rows.iter()
            .map(|row| IssueListRow {
                role: display_role_for_state(row.state_label()),
                id: row.id.clone(),
                status: Some(row.status.clone()),
                priority: row.priority.clone(),
                title: row.title.clone(),
                blockers: row.open_blockers.len(),
                depth: 1,
            })
            .collect(),
    )
    .total_count(rows.len())
    .limit(20)
}

fn display_role_for_state(state: &str) -> DisplayRole {
    match state {
        "active" => DisplayRole::Executable,
        "ready" => DisplayRole::Selectable,
        "blocked" => DisplayRole::Blocked,
        _ => DisplayRole::ContextOnly,
    }
}

pub fn mission_dashboard(
    index: &RecordIndex,
    mission_ref: &str,
    options: MissionDashboardOptions,
    quiet: bool,
) -> Result<()> {
    let filter = options.filter()?;
    let mission_id = crate::commands::issue::resolve_id(index, mission_ref)?;
    let mission = index.require_issue(&mission_id)?;
    if mission.issue_type != "mission" {
        bail!(
            "{} is issue_type '{}'; work mission requires issue_type 'mission'",
            mission.id,
            mission.issue_type
        );
    }
    let scoped = mission_scoped_issues(index, &mission.id)?;
    let filtered = filter_dashboard_issues(index, &scoped, filter)?;
    if quiet {
        if filter == DashboardFilter::Summary {
            let counts = dashboard_counts(index, &scoped)?;
            println!(
                "{} active={} ready={} blocked={} done={} backlog={}",
                mission.id,
                counts.active,
                counts.ready,
                counts.blocked,
                counts.done,
                counts.backlog
            );
        } else {
            for issue in filtered {
                println!("{}", issue.id);
            }
        }
        return Ok(());
    }

    let counts = dashboard_counts(index, &scoped)?;
    let panels = if filter == DashboardFilter::Summary {
        dashboard_panels(index, &scoped)?
    } else {
        vec![dashboard_panel_for_filter(index, filter, &filtered)?]
    };
    let title = strip_mission_title_prefix(&mission.title);
    let mut metadata = MetadataPanel::untitled()
        .row("Status", mission.status.clone())
        .row(
            "Work",
            format!(
                "active {}, ready {}, blocked {}, done {}, backlog {}",
                counts.active, counts.ready, counts.blocked, counts.done, counts.backlog
            ),
        );
    if let Some(strategy) = mission.strategy()? {
        metadata = metadata.row("Strategy", strategy.path);
    }
    let mut page = Page::new(format!(
        "{} [mission] {} - {}",
        mission.id, mission.status, title
    ))
    .panel(metadata);
    for panel in panels {
        page = page.panel(panel);
    }
    page.panel(mission_footer(&mission.id, &counts))
        .print(RenderContext::for_stdout());
    Ok(())
}

pub fn epic_dashboard(index: &RecordIndex, epic_ref: &str, quiet: bool) -> Result<()> {
    let epic_id = crate::commands::issue::resolve_id(index, epic_ref)?;
    let epic = index.require_issue(&epic_id)?;
    if epic.issue_type != "epic" {
        bail!(
            "{} is issue_type '{}'; work epic requires issue_type 'epic'",
            epic.id,
            epic.issue_type
        );
    }
    let mut scoped = descendant_issues(index, &epic.id)?;
    scoped.insert(0, epic.clone());
    if quiet {
        let counts = dashboard_counts(index, &scoped)?;
        println!(
            "{} active={} ready={} blocked={} done={} backlog={} proof_gaps={}",
            epic.id,
            counts.active,
            counts.ready,
            counts.blocked,
            counts.done,
            counts.backlog,
            proof_gap_count(index, &scoped)?
        );
        return Ok(());
    }

    let counts = dashboard_counts(index, &scoped)?;
    let proof_gaps = proof_gap_count(index, &scoped)?;
    let transition = transition_readiness(index, &epic.id, "start")?;
    let panels = dashboard_panels(index, &scoped)?;
    let mut page = Page::new(format!("Work Epic {} - {}", epic.id, epic.title)).panel(
        MetadataPanel::untitled()
            .row("Status", epic.status.clone())
            .row(
                "Mission",
                containing_mission(index, &epic.id)?.unwrap_or_else(|| "(none)".to_string()),
            )
            .row(
                "Work",
                format!(
                    "active {}, ready {}, blocked {}, done {}, backlog {}",
                    counts.active, counts.ready, counts.blocked, counts.done, counts.backlog
                ),
            )
            .row("Proof gaps", proof_gaps.to_string())
            .row("Transition readiness", transition),
    );
    for panel in panels {
        page = page.panel(panel);
    }
    page.panel(FooterPanel::new(
        "Next Commands",
        [
            FooterAction::new(
                "Show epic record",
                format!("atelier issue show {}", epic.id),
            ),
            FooterAction::new(
                "Inspect transitions",
                format!("atelier issue transition {}", epic.id),
            ),
        ],
    ))
    .print(RenderContext::for_stdout());
    Ok(())
}

#[derive(Default)]
struct DashboardCounts {
    active: usize,
    ready: usize,
    blocked: usize,
    done: usize,
    backlog: usize,
}

fn dashboard_counts(index: &RecordIndex, issues: &[Issue]) -> Result<DashboardCounts> {
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let mut counts = DashboardCounts::default();
    for issue in issues {
        match issue_category(index, policy.as_ref(), issue)?.as_str() {
            "active" => counts.active += 1,
            "blocked" => counts.blocked += 1,
            "done" => counts.done += 1,
            "backlog" => counts.backlog += 1,
            _ => counts.ready += 1,
        }
    }
    Ok(counts)
}

fn dashboard_panels(index: &RecordIndex, issues: &[Issue]) -> Result<Vec<IssueListPanel>> {
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let mut buckets = BTreeMap::<String, Vec<IssueListRow>>::new();
    for issue in issues {
        let category = issue_category(index, policy.as_ref(), issue)?;
        let role = match category.as_str() {
            "active" => DisplayRole::Executable,
            "blocked" => DisplayRole::Blocked,
            "done" | "backlog" => DisplayRole::ContextOnly,
            _ => DisplayRole::Selectable,
        };
        let open_blockers = crate::commands::issue_workflow::open_blocker_ids_with_policy(
            index,
            policy.as_ref(),
            &issue.id,
        )?;
        buckets
            .entry(category)
            .or_default()
            .push(dashboard_row(issue, role, open_blockers));
    }
    let mut panels = Vec::new();
    for (category, title) in [
        ("active", "Active Work"),
        ("todo", "Ready Work"),
        ("blocked", "Blocked Work"),
        ("backlog", "Backlog Work"),
        ("done", "Done Work"),
    ] {
        let rows = buckets.remove(category).unwrap_or_default();
        if rows.is_empty() {
            continue;
        }
        panels.push(IssueListPanel::new(title, rows).limit(8));
    }
    Ok(panels)
}

fn dashboard_panel_for_filter(
    index: &RecordIndex,
    filter: DashboardFilter,
    issues: &[Issue],
) -> Result<IssueListPanel> {
    let title = match filter {
        DashboardFilter::Ready => "Ready Work",
        DashboardFilter::Blocked => "Blocked Work",
        DashboardFilter::Active => "Active Work",
        DashboardFilter::Done => "Done Work",
        DashboardFilter::All | DashboardFilter::Summary => "Scoped Work",
    };
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let rows = issues
        .iter()
        .map(|issue| {
            let category = issue_category(index, policy.as_ref(), issue)?;
            let role = match category.as_str() {
                "active" => DisplayRole::Executable,
                "blocked" => DisplayRole::Blocked,
                "done" | "backlog" => DisplayRole::ContextOnly,
                _ => DisplayRole::Selectable,
            };
            let open_blockers = crate::commands::issue_workflow::open_blocker_ids_with_policy(
                index,
                policy.as_ref(),
                &issue.id,
            )?;
            Ok(dashboard_row(issue, role, open_blockers))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(IssueListPanel::new(title, rows).limit(usize::MAX))
}

fn dashboard_row(issue: &Issue, role: DisplayRole, open_blockers: Vec<String>) -> IssueListRow {
    let title = if open_blockers.is_empty() {
        issue.title.clone()
    } else {
        format!("{} | blockers: {}", issue.title, open_blockers.join(", "))
    };
    IssueListRow {
        role,
        id: issue.id.clone(),
        status: Some(issue.status.clone()),
        priority: issue.priority.clone(),
        title,
        blockers: open_blockers.len(),
        depth: 1,
    }
}

fn filter_dashboard_issues(
    index: &RecordIndex,
    issues: &[Issue],
    filter: DashboardFilter,
) -> Result<Vec<Issue>> {
    if filter == DashboardFilter::Summary || filter == DashboardFilter::All {
        return Ok(issues.to_vec());
    }
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    issues
        .iter()
        .filter_map(
            |issue| match issue_category(index, policy.as_ref(), issue) {
                Ok(category)
                    if matches!(
                        (filter, category.as_str()),
                        (DashboardFilter::Ready, "todo")
                            | (DashboardFilter::Blocked, "blocked")
                            | (DashboardFilter::Active, "active")
                            | (DashboardFilter::Done, "done")
                    ) =>
                {
                    Some(Ok(issue.clone()))
                }
                Ok(_) => None,
                Err(error) => Some(Err(error)),
            },
        )
        .collect()
}

fn strip_mission_title_prefix(title: &str) -> &str {
    title
        .strip_prefix("Mission:")
        .map(str::trim)
        .unwrap_or(title)
}

fn mission_footer(mission_id: &str, counts: &DashboardCounts) -> FooterPanel {
    let mut actions = vec![
        FooterAction::new(
            "Show mission record",
            format!("atelier issue show {mission_id}"),
        ),
        FooterAction::new(
            "Inspect transitions",
            format!("atelier issue transition {mission_id}"),
        ),
    ];
    if counts.ready > 0 {
        actions.push(FooterAction::new(
            "List mission ready work",
            format!("atelier work mission {mission_id} --ready"),
        ));
    }
    if counts.blocked > 0 {
        actions.push(FooterAction::new(
            "List mission blockers",
            format!("atelier work mission {mission_id} --blocked"),
        ));
    }
    FooterPanel::new("Next Commands", actions)
}

fn issue_category(
    index: &RecordIndex,
    policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
    issue: &Issue,
) -> Result<String> {
    let classified = atelier_app::work_classification::classify_issue(index, policy, issue)?;
    Ok(match classified.classification {
        WorkClassification::Ready => "todo",
        other => other.label(),
    }
    .to_string())
}

fn mission_scoped_issues(index: &RecordIndex, mission_id: &str) -> Result<Vec<Issue>> {
    let mut scoped = atelier_app::objective_graph::mission_issue_ids(index, mission_id)?
        .into_iter()
        .filter_map(|id| index.get_issue(&id))
        .collect::<Vec<_>>();
    scoped.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(scoped)
}

fn descendant_issues(index: &RecordIndex, root_id: &str) -> Result<Vec<Issue>> {
    atelier_app::objective_graph::issue_descendant_ids(index, root_id)?
        .into_iter()
        .map(|id| index.require_issue(&id))
        .collect()
}

fn proof_gap_count(index: &RecordIndex, issues: &[Issue]) -> Result<usize> {
    let mut count = 0;
    for issue in issues {
        if !crate::commands::issue::issue_evidence_gate_status(index, issue, None)?.passed {
            count += 1;
        }
    }
    Ok(count)
}

fn transition_readiness(
    index: &RecordIndex,
    issue_id: &str,
    transition_name: &str,
) -> Result<String> {
    let options = match crate::commands::workflow::issue_transition_options(index, issue_id) {
        Ok(options) => options,
        Err(error) => {
            let message = error.to_string();
            if message.contains("has no configured transitions") {
                return Ok("terminal".to_string());
            }
            return Ok(format!("not available: {message}"));
        }
    };
    let Some(option) = options
        .into_iter()
        .find(|option| option.name == transition_name)
    else {
        return Ok("not available".to_string());
    };
    if option.allowed {
        Ok(format!("{} allowed", option.name))
    } else if let Some(result) = option
        .validator_results
        .iter()
        .find(|result| !result.passed && result.help.is_some())
    {
        Ok(format!(
            "{} blocked: {}; {}",
            option.name,
            result.reason,
            result.help.as_deref().unwrap_or_default()
        ))
    } else {
        Ok(format!(
            "{} blocked: {}",
            option.name,
            option
                .blockers
                .first()
                .cloned()
                .unwrap_or_else(|| "blocked".to_string())
        ))
    }
}

fn containing_mission(index: &RecordIndex, issue_id: &str) -> Result<Option<String>> {
    for mission in index.list_issues(Some("all"), None, None)? {
        if mission.issue_type == "mission"
            && atelier_app::objective_graph::mission_issue_ids(index, &mission.id)?
                .contains(issue_id)
        {
            return Ok(Some(mission.id));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_views_rendering_ready_work_pay_execution_gate_cost() {
        assert!(bucket_needs_execution_gates("ready"));
        assert!(bucket_needs_execution_gates("all"));
        for bucket in ["blocked", "active", "backlog"] {
            assert!(!bucket_needs_execution_gates(bucket), "{bucket}");
        }
    }
}
