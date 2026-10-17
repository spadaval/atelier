use anyhow::{bail, Result};
use atelier_app::record_index::RecordIndex;
use std::collections::BTreeSet;

use atelier_app::issue_read::ObjectiveBucketTotals;
use atelier_app::mission_overview::mission_overview;
use atelier_app::work_classification::{ClassifiedWork, WorkBuckets, WorkClassification, WorkRow};
use atelier_core::Issue;

use crate::commands::git::InFlight;
use crate::human_output::{
    count_noun, last_closed_mission, print_page, render_footer_section, render_labeled_lines,
    render_lines_section, render_metadata, render_unassigned_groups, scope_counts, FooterAction,
    IssueListPanel, IssueListRow, ItemLine, RenderContext, TextStyle,
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
    /// The one filter chosen; clap rejects more than one.
    fn filter(self) -> DashboardFilter {
        if self.ready {
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
        }
    }
}

pub fn dashboards(index: &RecordIndex, quiet: bool) -> Result<()> {
    if quiet {
        println!("ready");
        println!("human");
        println!("blocked");
        println!("active");
        println!("all");
        println!("missions");
        println!("mission");
        println!("parent");
        return Ok(());
    }

    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let overview = mission_overview(index, policy.as_ref(), false)?;
    let mut next = vec![
        FooterAction::new("Browse ready work", "atelier work ready"),
        FooterAction::new("Triage blockers", "atelier work blocked"),
        FooterAction::new("Compare missions", "atelier work missions"),
    ];
    // The mission `work missions` lists first; with none open there is no
    // dashboard to point at.
    if let Some(top) = overview.missions.first() {
        next.push(FooterAction::new(
            "Open the top mission's dashboard",
            format!("atelier work mission {}", top.issue.id),
        ));
    }
    print_page(
        "Atelier Work",
        [
            render_lines_section(
                "Dashboards",
                [
                    "ready    Small top-level executable work picker",
                    "human    Ready work only the human can do (needs-human)",
                    "blocked  Repo-wide blocker triage",
                    "missions Flat mission summaries with complete scope counts",
                    "mission  Live mission orchestration dashboard",
                    "parent   A parent issue and its children",
                ],
            ),
            render_footer_section(RenderContext::for_stdout(), next),
        ],
        RenderContext::for_stdout(),
    );
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

    let groups = match bucket {
        "ready" => vec![
            ("Ready Work", &buckets.ready),
            ("Needs a Human", &buckets.human),
            ("Parents and Missions (context)", &buckets.context),
        ],
        "human" => vec![("Needs a Human", &buckets.human)],
        "blocked" => vec![("Blocked Work", &buckets.blocked)],
        "active" => vec![("Active Work", &buckets.active)],
        "all" => vec![
            ("Active Work", &buckets.active),
            ("Ready Work", &buckets.ready),
            ("Needs a Human", &buckets.human),
            ("Parents and Missions (context)", &buckets.context),
            ("Blocked Work", &buckets.blocked),
            ("Backlog Work", &buckets.backlog),
        ],
        other => bail!("unknown work bucket '{other}'"),
    };
    // Ready work another local branch already started is marked with its
    // state there; the summaries are read once for the whole page.
    let in_flight = if matches!(bucket, "ready" | "all") {
        in_flight_marks(index).unwrap_or_else(|error| InFlight {
            notices: vec![format!("In-flight marks unavailable: {error:#}")],
            ..InFlight::default()
        })
    } else {
        InFlight::default()
    };
    let context = RenderContext::for_stdout();
    let mut panels = groups
        .into_iter()
        .map(|(title, rows)| {
            let panel = IssueListPanel::from_work_rows(title, rows);
            if title == "Ready Work" {
                panel.marks(&in_flight.marks)
            } else {
                panel
            }
            .render(context)
        })
        .collect::<Vec<_>>();
    if !in_flight.notices.is_empty() {
        panels.push(render_lines_section("", in_flight.notices));
    }
    print_page("Atelier Work", panels, context);
    Ok(())
}

fn in_flight_marks(index: &RecordIndex) -> Result<InFlight> {
    let layout = atelier_app::storage_layout::StorageLayout::discover()?;
    Ok(
        crate::commands::git::branch_summaries(layout.repo_root(), &layout.canonical_dir(), index)?
            .in_flight(),
    )
}

fn bucket_needs_execution_gates(bucket: &str) -> bool {
    matches!(bucket, "ready" | "human" | "all")
}

fn apply_ready_execution_gates(
    index: &RecordIndex,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
    buckets: &mut WorkBuckets,
) -> Result<()> {
    let candidate_ids = buckets
        .ready
        .iter()
        .chain(&buckets.human)
        .map(|row| row.id.clone())
        .collect::<BTreeSet<_>>();
    let dependency = atelier_app::objective_graph::dependency_readiness_batch(
        index,
        policy,
        candidate_ids.iter().cloned(),
    )?;
    for bucket in [&mut buckets.ready, &mut buckets.human] {
        let mut kept = Vec::new();
        for mut row in std::mem::take(bucket) {
            let dependency_ready = dependency
                .ready_by_issue
                .get(&row.id)
                .copied()
                .unwrap_or(false);
            if dependency_ready {
                kept.push(row);
            } else {
                row.open_blockers.push("readiness gate".to_string());
                buckets.blocked.push(row);
            }
        }
        *bucket = kept;
    }
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
            ItemLine::new(
                &mission.issue.id,
                &mission.issue.status,
                &mission.issue.priority,
                &mission.issue.title,
            )
            .state(Some(&mission.status_category), None)
            .render_wrapped(context, ""),
        );
        let open_blockers = mission.open_blockers.len();
        for (label, value) in [
            (
                "Scope",
                Some(scope_counts(mission.scope_ids.len(), &mission.totals)),
            ),
            (
                "Open blockers",
                (open_blockers > 0).then(|| open_blockers.to_string()),
            ),
            (
                "More info",
                Some(format!("atelier work mission {}", mission.issue.id)),
            ),
        ]
        .into_iter()
        .filter_map(|(label, value)| Some((label, value?)))
        {
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
        match &overview.last_closed_mission {
            Some(mission) => lines.extend(render_labeled_lines(
                context,
                "",
                "No open missions; last closed",
                &last_closed_mission(mission),
                TextStyle::Secondary,
            )),
            None => lines.push("No missions match the current overview.".to_string()),
        }
    }
    if overview.missions.len() > 20 {
        lines.push(format!(
            "{} omitted",
            count_noun(overview.missions.len() - 20, "more mission")
        ));
    }
    if overview.unassigned_nonterminal > 0 {
        lines.push(String::new());
        lines.extend(render_labeled_lines(
            context,
            "",
            "Unassigned",
            &count_noun(overview.unassigned_nonterminal, "open issue"),
            TextStyle::Secondary,
        ));
        lines.extend(render_unassigned_groups(
            &overview.unassigned,
            // Branch summaries are status's; this view stays branch-local.
            &Default::default(),
            "  ",
            usize::MAX,
            context,
        ));
    }
    print_page(
        "Mission Overview",
        [
            render_lines_section("", lines),
            render_footer_section(
                context,
                [FooterAction::new(
                    "Browse mission records",
                    "atelier issue list --issue-type mission",
                )],
            ),
        ],
        context,
    );
    Ok(())
}

fn print_quiet(bucket: &str, buckets: &WorkBuckets) {
    let groups: &[&[WorkRow]] = match bucket {
        "ready" => &[&buckets.ready],
        "human" => &[&buckets.human],
        "blocked" => &[&buckets.blocked],
        "active" => &[&buckets.active],
        "all" => &[
            &buckets.active,
            &buckets.ready,
            &buckets.human,
            &buckets.context,
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

pub fn mission_dashboard(
    index: &RecordIndex,
    mission_ref: &str,
    options: MissionDashboardOptions,
    quiet: bool,
) -> Result<()> {
    scoped_dashboard(index, mission_ref, "mission", options.filter(), quiet)
}

pub fn parent_dashboard(index: &RecordIndex, parent_ref: &str, quiet: bool) -> Result<()> {
    scoped_dashboard(index, parent_ref, "parent", DashboardFilter::Summary, quiet)
}

/// The dashboards that do show an issue a scoped view refused: its own
/// parent view, its parent's, and its mission's, or else `issue show`.
fn related_dashboards(index: &RecordIndex, issue: &Issue, has_children: bool) -> Result<String> {
    let mut commands = Vec::new();
    if has_children {
        commands.push(format!("`atelier work parent {}`", issue.id));
    }
    if let Some(parent) = &issue.parent_id {
        commands.push(format!("`atelier work parent {parent}`"));
    }
    if let Some(mission) = atelier_app::objective_graph::containing_mission(index, &issue.id)? {
        commands.push(format!("`atelier work mission {mission}`"));
    }
    if commands.is_empty() {
        commands.push(format!("`atelier issue show {}`", issue.id));
    }
    Ok(commands.join(" or "))
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
    let has_children = !index.get_subissues(&id).is_empty();
    match issue_type {
        "mission" if issue.issue_type != "mission" => bail!(
            "{id} is a {}, not a mission. Use {}.",
            issue.issue_type,
            related_dashboards(index, &issue, has_children)?
        ),
        "parent" if issue.issue_type == "mission" => {
            bail!("{id} is a mission, not a parent. Use `atelier work mission {id}`.")
        }
        "parent" if !has_children => bail!(
            "{id} has no children, so it is not a parent. Use {}.",
            related_dashboards(index, &issue, false)?
        ),
        _ => {}
    }
    let work = ScopedWork::new(index, &issue)?;
    let proof_gaps = if issue_type == "parent" {
        Some(proof_gap_count(index, &work.rows)?)
    } else {
        None
    };
    if quiet {
        if filter == DashboardFilter::Summary {
            let counts = &work.totals;
            let mut summary = format!(
                "{id} active={} ready={} human={} context={} blocked={} done={} backlog={}",
                counts.active,
                counts.ready,
                counts.human,
                counts.context,
                counts.blocked,
                counts.done,
                counts.backlog
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
    let mut metadata = vec![("Work", scope_counts(work.size, counts))];
    // The title is the item line, so no metadata repeats its status.
    let category = policy_category(&issue.status)?;
    let title = if let Some(proof_gaps) = proof_gaps {
        metadata.push((
            "Mission",
            atelier_app::objective_graph::containing_mission(index, &id)?
                .unwrap_or_else(|| "(none)".to_string()),
        ));
        if proof_gaps > 0 {
            metadata.push(("Proof gaps", proof_gaps.to_string()));
        }
        metadata.push((
            "Transition readiness",
            transition_readiness(index, &id, "start")?,
        ));
        ItemLine::new(&id, &issue.status, &issue.priority, &issue.title)
            .issue_type(&issue.issue_type)
            .state(category.as_deref(), None)
            .text()
    } else {
        if let Some(strategy) = issue.strategy()? {
            metadata.push(("Strategy", strategy.path));
        }
        ItemLine::new(
            &id,
            &issue.status,
            &issue.priority,
            strip_mission_title_prefix(&issue.title),
        )
        .issue_type("mission")
        .state(category.as_deref(), None)
        .text()
    };
    let context = RenderContext::for_stdout();
    let mut panels = vec![render_metadata(None, &metadata)];
    panels.extend(
        work.panels(filter)
            .into_iter()
            .map(|panel| panel.render(context)),
    );
    panels.push(objective_footer(context, &issue, counts));
    print_page(&title, panels, context);
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

/// An objective's work: its rows, which for a parent include the parent
/// itself, and the counts of its scope, which never do, so `work parent` and
/// `issue show` give one parent the same size.
struct ScopedWork {
    rows: Vec<(Issue, ClassifiedWork)>,
    size: usize,
    totals: ObjectiveBucketTotals,
}

impl ScopedWork {
    fn new(index: &RecordIndex, objective: &Issue) -> Result<Self> {
        let mut issues = atelier_app::objective_graph::issue_descendant_ids(index, &objective.id)?
            .into_iter()
            .map(|id| index.require_issue(&id))
            .collect::<Result<Vec<_>>>()?;
        if objective.issue_type != "mission" {
            issues.insert(0, objective.clone());
        }
        let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
        let mut rows = Vec::new();
        let mut totals = ObjectiveBucketTotals::default();
        let mut size = 0;
        for issue in issues {
            let classified =
                atelier_app::work_classification::classify_issue(index, policy.as_ref(), &issue)?;
            if issue.id == objective.id {
                rows.push((issue, classified));
                continue;
            }
            size += 1;
            match classified.classification {
                WorkClassification::Active => totals.active += 1,
                WorkClassification::Ready => totals.ready += 1,
                WorkClassification::NeedsHuman => totals.human += 1,
                WorkClassification::Context => totals.context += 1,
                WorkClassification::Blocked => totals.blocked += 1,
                WorkClassification::Done => totals.done += 1,
                WorkClassification::Backlog => totals.backlog += 1,
            }
            rows.push((issue, classified));
        }
        Ok(Self { rows, size, totals })
    }

    fn panels(&self, filter: DashboardFilter) -> Vec<IssueListPanel> {
        let groups: &[(Option<WorkClassification>, &str)] = match filter {
            DashboardFilter::Summary => &[
                (Some(WorkClassification::Active), "Active Work"),
                (Some(WorkClassification::Ready), "Ready Work"),
                (Some(WorkClassification::NeedsHuman), "Needs a Human"),
                (Some(WorkClassification::Context), "Parents and Missions"),
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
                        id: issue.id.clone(),
                        status: issue.status.clone(),
                        status_category: work.status_category.clone(),
                        classification: work.classification,
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
                let panel = IssueListPanel::new(*title, rows).limit(
                    if filter == DashboardFilter::Summary {
                        8
                    } else {
                        usize::MAX
                    },
                );
                // Only the unfiltered list mixes work states.
                Some(if state.is_none() {
                    panel.row_states()
                } else {
                    panel
                })
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

fn objective_footer(
    context: RenderContext,
    issue: &Issue,
    counts: &ObjectiveBucketTotals,
) -> Vec<String> {
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
    render_footer_section(context, actions)
}

fn proof_gap_count(index: &RecordIndex, rows: &[(Issue, ClassifiedWork)]) -> Result<usize> {
    let layout = atelier_app::storage_layout::StorageLayout::discover()?;
    let mut proofs = atelier_app::evidence_freshness::EvidenceReadContext::new(layout.repo_root());
    let mut count = 0;
    for (issue, _) in rows {
        if !proofs.issue_gate(index, &issue.id, 1, None)?.passed {
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
                return Ok("closed".to_string());
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

/// The status's category under the working policy, when there is one.
fn policy_category(status: &str) -> Result<Option<String>> {
    Ok(
        crate::commands::issue_workflow::load_issue_workflow_policy()?
            .and_then(|policy| policy.status_category(status).map(str::to_owned)),
    )
}
