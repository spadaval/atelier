use anyhow::{bail, Result};
use atelier_app::record_index::RecordIndex;
use std::collections::BTreeSet;

use atelier_app::issue_read::ObjectiveBucketTotals;
use atelier_app::mission_overview::mission_overview;
use atelier_app::work_classification::{ClassifiedWork, WorkBuckets, WorkClassification, WorkRow};
use atelier_core::Issue;

use crate::human_output::{
    render_labeled_lines, DisplayRole, FooterAction, FooterPanel, IssueHeadingRow, IssueListPanel,
    IssueListRow, LinesPanel, MetadataPanel, Page, RenderContext, TextStyle,
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
    scoped_dashboard(index, mission_ref, "mission", options.filter()?, quiet)
}

pub fn epic_dashboard(index: &RecordIndex, epic_ref: &str, quiet: bool) -> Result<()> {
    scoped_dashboard(index, epic_ref, "epic", DashboardFilter::Summary, quiet)
}

fn scoped_dashboard(
    index: &RecordIndex,
    issue_ref: &str,
    issue_type: &str,
    filter: DashboardFilter,
    quiet: bool,
) -> Result<()> {
    let id = crate::commands::issue::resolve_id(index, issue_ref)?;
    let issue = index.require_issue(&id)?;
    if issue.issue_type != issue_type {
        bail!(
            "{} is issue_type '{}'; work {issue_type} requires issue_type '{issue_type}'",
            issue.id,
            issue.issue_type
        );
    }
    let work = ScopedWork::new(index, &issue)?;
    let proof_gaps = if issue_type == "epic" {
        Some(proof_gap_count(index, &work.rows)?)
    } else {
        None
    };
    if quiet {
        if filter == DashboardFilter::Summary {
            let counts = &work.totals;
            let mut summary = format!(
                "{id} active={} ready={} blocked={} done={} backlog={}",
                counts.active, counts.ready, counts.blocked, counts.done, counts.backlog
            );
            if let Some(gaps) = proof_gaps {
                summary.push_str(&format!(" proof_gaps={gaps}"));
            }
            println!("{summary}");
        } else {
            for (issue, classified) in &work.rows {
                if filter
                    .classification()
                    .is_none_or(|state| state == classified.classification)
                {
                    println!("{}", issue.id);
                }
            }
        }
        return Ok(());
    }
    let counts = &work.totals;
    let mut metadata = MetadataPanel::untitled()
        .row("Status", issue.status.clone())
        .row(
            "Work",
            format!(
                "active {}, ready {}, blocked {}, done {}, backlog {}",
                counts.active, counts.ready, counts.blocked, counts.done, counts.backlog
            ),
        );
    let title = if let Some(proof_gaps) = proof_gaps {
        metadata = metadata
            .row(
                "Mission",
                containing_mission(index, &id)?.unwrap_or_else(|| "(none)".to_string()),
            )
            .row("Proof gaps", proof_gaps.to_string())
            .row(
                "Transition readiness",
                transition_readiness(index, &id, "start")?,
            );
        format!("Work Epic {id} - {}", issue.title)
    } else {
        if let Some(strategy) = issue.strategy()? {
            metadata = metadata.row("Strategy", strategy.path);
        }
        format!(
            "{id} [mission] {} - {}",
            issue.status,
            strip_mission_title_prefix(&issue.title)
        )
    };
    let mut page = Page::new(title).panel(metadata);
    for panel in work.panels(filter) {
        page = page.panel(panel);
    }
    page.panel(objective_footer(&issue, counts))
        .print(RenderContext::for_stdout());
    Ok(())
}

impl DashboardFilter {
    fn classification(self) -> Option<WorkClassification> {
        match self {
            Self::Ready => Some(WorkClassification::Ready),
            Self::Blocked => Some(WorkClassification::Blocked),
            Self::Active => Some(WorkClassification::Active),
            Self::Done => Some(WorkClassification::Done),
            Self::Summary | Self::All => None,
        }
    }
}

struct ScopedWork {
    rows: Vec<(Issue, ClassifiedWork)>,
    totals: ObjectiveBucketTotals,
}

impl ScopedWork {
    fn new(index: &RecordIndex, objective: &Issue) -> Result<Self> {
        let mut issues = atelier_app::objective_graph::issue_descendant_ids(index, &objective.id)?
            .into_iter()
            .map(|id| index.require_issue(&id))
            .collect::<Result<Vec<_>>>()?;
        if objective.issue_type == "epic" {
            issues.insert(0, objective.clone());
        }
        let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
        let mut rows = Vec::new();
        let mut totals = ObjectiveBucketTotals::default();
        for issue in issues {
            let classified =
                atelier_app::work_classification::classify_issue(index, policy.as_ref(), &issue)?;
            match classified.classification {
                WorkClassification::Active => totals.active += 1,
                WorkClassification::Ready => totals.ready += 1,
                WorkClassification::Blocked => totals.blocked += 1,
                WorkClassification::Done => totals.done += 1,
                WorkClassification::Backlog => totals.backlog += 1,
            }
            rows.push((issue, classified));
        }
        Ok(Self { rows, totals })
    }

    fn panels(&self, filter: DashboardFilter) -> Vec<IssueListPanel> {
        let groups: &[(Option<WorkClassification>, &str)] = match filter {
            DashboardFilter::Summary => &[
                (Some(WorkClassification::Active), "Active Work"),
                (Some(WorkClassification::Ready), "Ready Work"),
                (Some(WorkClassification::Blocked), "Blocked Work"),
                (Some(WorkClassification::Backlog), "Backlog Work"),
                (Some(WorkClassification::Done), "Done Work"),
            ],
            DashboardFilter::Ready => &[(filter.classification(), "Ready Work")],
            DashboardFilter::Blocked => &[(filter.classification(), "Blocked Work")],
            DashboardFilter::Active => &[(filter.classification(), "Active Work")],
            DashboardFilter::Done => &[(filter.classification(), "Done Work")],
            DashboardFilter::All => &[(None, "Scoped Work")],
        };
        groups
            .iter()
            .filter_map(|(state, title)| {
                let rows = self
                    .rows
                    .iter()
                    .filter(|(_, work)| state.is_none_or(|state| state == work.classification))
                    .map(|(issue, work)| IssueListRow {
                        role: display_role_for_state(work.classification.label()),
                        id: issue.id.clone(),
                        status: Some(issue.status.clone()),
                        priority: issue.priority.clone(),
                        title: if work.open_blockers.is_empty() {
                            issue.title.clone()
                        } else {
                            format!(
                                "{} | blockers: {}",
                                issue.title,
                                work.open_blockers.join(", ")
                            )
                        },
                        blockers: work.open_blockers.len(),
                        depth: 1,
                    })
                    .collect::<Vec<_>>();
                if rows.is_empty() && filter == DashboardFilter::Summary {
                    return None;
                }
                Some(IssueListPanel::new(*title, rows).limit(
                    if filter == DashboardFilter::Summary {
                        8
                    } else {
                        usize::MAX
                    },
                ))
            })
            .collect()
    }
}

fn strip_mission_title_prefix(title: &str) -> &str {
    title
        .strip_prefix("Mission:")
        .map(str::trim)
        .unwrap_or(title)
}

fn objective_footer(issue: &Issue, counts: &ObjectiveBucketTotals) -> FooterPanel {
    let mission_id = &issue.id;
    let mut actions = vec![
        FooterAction::new(
            format!("Show {} record", issue.issue_type),
            format!("atelier issue show {mission_id}"),
        ),
        FooterAction::new(
            "Inspect transitions",
            format!("atelier issue transition {mission_id}"),
        ),
    ];
    if issue.issue_type == "mission" && counts.ready > 0 {
        actions.push(FooterAction::new(
            "List mission ready work",
            format!("atelier work mission {mission_id} --ready"),
        ));
    }
    if issue.issue_type == "mission" && counts.blocked > 0 {
        actions.push(FooterAction::new(
            "List mission blockers",
            format!("atelier work mission {mission_id} --blocked"),
        ));
    }
    FooterPanel::new("Next Commands", actions)
}

fn proof_gap_count(index: &RecordIndex, rows: &[(Issue, ClassifiedWork)]) -> Result<usize> {
    let mut count = 0;
    for (issue, _) in rows {
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
