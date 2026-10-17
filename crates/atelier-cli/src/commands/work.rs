use anyhow::{bail, Result};
use atelier_app::record_index::RecordIndex;
use std::collections::{BTreeMap, BTreeSet};

use atelier_app::issue_read::ObjectiveBucketTotals;
use atelier_app::mission_overview::mission_overview;
use atelier_app::work_classification::{ClassifiedWork, WorkBuckets, WorkClassification, WorkRow};
use atelier_core::Issue;

use crate::commands::git::InFlight;
use crate::human_output::{
    count_noun, last_closed_mission, print_page, render_footer_section, render_labeled_lines,
    render_lines_section, render_metadata, render_row_wrapped, render_unassigned_groups,
    scope_counts, FooterAction, IssueListPanel, IssueListRow, ItemLine, RenderContext, TextStyle,
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

/// Missions `work missions` shows without `--all`.
const MISSIONS_SHOWN: usize = 20;

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
    // `--all` shows the whole overview: done missions, and every mission
    // and unassigned item past the display limits.
    let (mission_limit, unassigned_limit) = if include_done {
        (usize::MAX, usize::MAX)
    } else {
        (MISSIONS_SHOWN, crate::human_output::UNASSIGNED_SHOWN)
    };
    let mut lines = Vec::new();
    for mission in overview.missions.iter().take(mission_limit) {
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
    if overview.missions.len() > mission_limit {
        lines.push(format!(
            "{} omitted; see `atelier work missions --all`",
            count_noun(overview.missions.len() - mission_limit, "more mission")
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
            unassigned_limit,
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

pub fn parent_dashboard(
    index: &RecordIndex,
    parent_ref: &str,
    all: bool,
    quiet: bool,
) -> Result<()> {
    let filter = if all {
        DashboardFilter::All
    } else {
        DashboardFilter::Summary
    };
    scoped_dashboard(index, parent_ref, "parent", filter, quiet)
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
            "{id} is {}, not a mission. Use {}.",
            atelier_app::record_ref::with_article(&issue.issue_type),
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
        Some(proof_gap_count(index, &id, &work.rows)?)
    } else {
        None
    };
    if quiet {
        if filter == DashboardFilter::Summary {
            let counts = &work.totals;
            let mut summary = format!(
                "active={} ready={} human={} context={} blocked={} done={} backlog={}",
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
    let mut metadata = vec![("Scope", scope_counts(work.size, counts))];
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
        // A done parent has no transitions out, and its title says so.
        if category.as_deref() != Some("done") {
            metadata.push((
                "Transition readiness",
                transition_readiness(index, &id, "start")?,
            ));
        }
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
    // Ready work another local branch already started is marked, as in
    // `work ready`, and is not the one the footer offers to start.
    let in_flight = if work.top_ready(&BTreeMap::new()).is_some() {
        in_flight_marks(index).unwrap_or_else(|error| InFlight {
            notices: vec![format!("In-flight marks unavailable: {error:#}")],
            ..InFlight::default()
        })
    } else {
        InFlight::default()
    };
    let context = RenderContext::for_stdout();
    // A mission's success criteria lead its summary, each marked by its
    // proof (atelier-stsa).
    let criteria = if issue.issue_type == "mission"
        && matches!(filter, DashboardFilter::Summary | DashboardFilter::All)
    {
        let repo_root = crate::commands::workflow::repo_root()?;
        let mut read = atelier_app::git_freshness::ReadContext::new(&repo_root);
        let proof = atelier_app::criteria::mission_proof(index, &mut read, &id)?;
        if let Some(counts) = criterion_counts(&proof) {
            metadata.push(("Criteria", counts));
        }
        criteria_panel(context, &proof)
    } else {
        Vec::new()
    };
    let mut panels = vec![render_metadata(None, &metadata), criteria];
    panels.extend(
        work.panels(&issue, filter, &in_flight.marks)
            .into_iter()
            .map(|panel| panel.render(context)),
    );
    if !in_flight.notices.is_empty() {
        panels.push(render_lines_section("", in_flight.notices.clone()));
    }
    panels.push(objective_footer(context, &issue, &work, &in_flight.marks)?);
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
        // The scope in the shared work order, as `status` and `work ready`
        // list it; a parent's own row stays first.
        let own = usize::from(objective.issue_type != "mission");
        let scope = rows.split_off(own);
        let order = crate::commands::work_order::ordered_work_indices(&scope);
        let mut scope = scope.into_iter().map(Some).collect::<Vec<_>>();
        rows.extend(order.into_iter().filter_map(|index| scope[index].take()));
        Ok(Self { rows, size, totals })
    }

    /// The first ready item in work order that no other branch has started.
    fn top_ready(&self, in_flight: &BTreeMap<String, String>) -> Option<&Issue> {
        self.rows
            .iter()
            .find(|(issue, work)| {
                work.classification == WorkClassification::Ready
                    && !in_flight.contains_key(&issue.id)
            })
            .map(|(issue, _)| issue)
    }

    fn panels(
        &self,
        objective: &Issue,
        filter: DashboardFilter,
        in_flight: &BTreeMap<String, String>,
    ) -> Vec<IssueListPanel> {
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
                let panel = IssueListPanel::new(*title, rows);
                let panel = if filter == DashboardFilter::Summary {
                    panel.limit(8, scoped_view_command(objective, *state))
                } else {
                    panel
                };
                let panel = if *state == Some(WorkClassification::Ready) {
                    panel.marks(in_flight)
                } else {
                    panel
                };
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

/// The view of `objective` that lists every item in work state `state`:
/// the mission's filter for that state where it has one, or else the whole
/// scope, which is all a parent view offers.
pub(crate) fn scoped_view_command(objective: &Issue, state: Option<WorkClassification>) -> String {
    let view = if objective.issue_type == "mission" {
        "mission"
    } else {
        "parent"
    };
    let flag = match (view, state) {
        ("mission", Some(WorkClassification::Ready)) => "--ready",
        ("mission", Some(WorkClassification::Blocked)) => "--blocked",
        ("mission", Some(WorkClassification::Active)) => "--active",
        ("mission", Some(WorkClassification::Done)) => "--done",
        _ => "--all",
    };
    format!("atelier work {view} {} {flag}", objective.id)
}

/// Steps from the objective's state: its record, its transitions while it
/// has any, and starting the top ready item no other branch has started. The views of other work states
/// are named where rows are omitted, so the footer does not repeat them.
fn objective_footer(
    context: RenderContext,
    issue: &Issue,
    work: &ScopedWork,
    in_flight: &BTreeMap<String, String>,
) -> Result<Vec<String>> {
    let id = &issue.id;
    let mut actions = vec![FooterAction::new(
        format!("Show {} record", issue.issue_type),
        format!("atelier issue show {id}"),
    )];
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    if crate::commands::issue_workflow::issue_has_transitions(policy.as_ref(), issue) {
        actions.push(FooterAction::new(
            "Inspect transitions",
            format!("atelier issue transition {id}"),
        ));
    }
    if let Some(top) = work.top_ready(in_flight) {
        actions.push(FooterAction::new(
            "Start the top ready item",
            format!("atelier start {}", top.id),
        ));
    }
    Ok(render_footer_section(context, actions))
}

/// Scope items without passing proof; the parent's own row is not its scope,
/// as the scope size leaves it out.
fn proof_gap_count(
    index: &RecordIndex,
    parent_id: &str,
    rows: &[(Issue, ClassifiedWork)],
) -> Result<usize> {
    let layout = atelier_app::storage_layout::StorageLayout::discover()?;
    let mut proofs = atelier_app::evidence_freshness::EvidenceReadContext::new(layout.repo_root());
    let mut count = 0;
    for (issue, _) in rows.iter().filter(|(issue, _)| issue.id != parent_id) {
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
    } else if option.allowed_with_arguments() {
        Ok(format!(
            "{} allowed, needs {}",
            option.name,
            option.missing_arguments.join(" ")
        ))
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
                .blockers_beyond_arguments()
                .first()
                .cloned()
                .unwrap_or_else(|| "blocked".to_string())
        ))
    }
}

/// `<n>: <n> proven, <n> failing, <n> unproven`, zeros left out; `None`
/// when the mission defines no criteria.
fn criterion_counts(proof: &atelier_app::criteria::MissionProof) -> Option<String> {
    use atelier_app::criteria::CriterionMark;
    if proof.proofs.is_empty() {
        return None;
    }
    let count = |mark: CriterionMark| proof.proofs.iter().filter(|p| p.mark == mark).count();
    let parts = [
        CriterionMark::Proven,
        CriterionMark::Failing,
        CriterionMark::Unproven,
    ]
    .into_iter()
    .map(|mark| (count(mark), mark.word()))
    .filter(|(count, _)| *count > 0)
    .map(|(count, word)| format!("{count} {word}"))
    .collect::<Vec<_>>();
    Some(format!("{}: {}", proof.proofs.len(), parts.join(", ")))
}

/// One row per criterion, `<slug> <mark> <claim> | <deciding receipt>`,
/// then what could not be read as a criterion or names one the mission
/// does not define.
fn criteria_panel(
    context: RenderContext,
    proof: &atelier_app::criteria::MissionProof,
) -> Vec<String> {
    use atelier_app::criteria::CriterionMark;
    let mut rows = Vec::new();
    for criterion in &proof.proofs {
        let mark = match criterion.mark {
            CriterionMark::Proven => "✓ proven",
            CriterionMark::Failing => "⚠ failing",
            CriterionMark::Unproven => "unproven",
        };
        let mut row = format!(
            "{} {mark} {}",
            criterion.criterion.slug,
            first_sentence(&criterion.criterion.text)
        );
        if let Some(receipt) = criterion.deciding() {
            row.push_str(&format!(
                " | {} {} at {}",
                receipt.evidence_id,
                receipt.result.as_str(),
                crate::human_output::short_commit(&receipt.commit)
            ));
            match receipt.code_commits_since {
                Some(0) | None => {}
                Some(1) => row.push_str(" | 1 code commit since"),
                Some(count) => row.push_str(&format!(" | {count} code commits since")),
            }
        }
        rows.extend(render_row_wrapped(context, &row));
    }
    for problem in &proof.criteria.parse.problems {
        rows.extend(render_row_wrapped(context, &format!("⚠ {problem}")));
    }
    for (evidence_id, slug) in &proof.orphaned {
        rows.extend(render_row_wrapped(
            context,
            &format!("⚠ {evidence_id} names criterion {slug}, which this mission does not define"),
        ));
    }
    if rows.is_empty() {
        return Vec::new();
    }
    render_lines_section("Success Criteria", rows)
}

/// The text up to its first sentence end, so a row names the claim without
/// the criterion's whole proof plan.
fn first_sentence(text: &str) -> &str {
    match text.find(". ") {
        Some(end) => &text[..=end],
        None => text,
    }
}

/// `work decisions <mission>`: the decisions made across a mission's scope.
/// One-way decision notes lead, then current decision records and
/// reversible notes; open and superseded records and malformed notes follow.
pub fn decisions(index: &RecordIndex, mission_ref: &str, quiet: bool) -> Result<()> {
    let id = crate::commands::issue::resolve_id(index, mission_ref)?;
    let mission = index.require_issue(&id)?;
    if mission.issue_type != "mission" {
        bail!(
            "{id} is {}, not a mission. Its decisions show in `atelier issue show {id} --full`.",
            atelier_app::record_ref::with_article(&mission.issue_type)
        );
    }
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let decisions = atelier_app::decisions::mission_decisions(index, policy.as_ref(), &id)?;
    if quiet {
        // The items holding current decisions, each once.
        let mut ids: Vec<&String> = Vec::new();
        let current = decisions
            .notes_one_way_first()
            .map(|note| &note.issue_id)
            .chain(
                decisions
                    .records
                    .iter()
                    .filter(|record| record.is_current())
                    .map(|record| &record.issue.id),
            );
        for id in current {
            if !ids.contains(&id) {
                println!("{id}");
                ids.push(id);
            }
        }
        return Ok(());
    }
    let context = RenderContext::for_stdout();
    let note_lines = |one_way: bool| {
        decisions
            .notes
            .iter()
            .filter(|note| note.decision.one_way == one_way)
            .flat_map(|note| {
                let decision = &note.decision;
                let mut row = format!(
                    "{} {} {} | because {}",
                    note.issue_id, decision.date, decision.choice, decision.reason
                );
                if !decision.rejected.is_empty() {
                    row.push_str(&format!(" | rejected {}", decision.rejected.join("; ")));
                }
                render_row_wrapped(context, &row)
            })
            .collect::<Vec<_>>()
    };
    let record_lines = |keep: &dyn Fn(&atelier_app::decisions::RecordDecision) -> bool| {
        decisions
            .records
            .iter()
            .filter(|record| keep(record))
            .flat_map(|record| {
                let issue = &record.issue;
                let category = policy
                    .as_ref()
                    .and_then(|policy| policy.status_category(&issue.status));
                let mut line =
                    ItemLine::new(&issue.id, &issue.status, &issue.priority, &issue.title)
                        .state(category, None);
                if let Some(adr) = &issue.adr {
                    line = line.cue(format!("ADR {}", adr.path));
                }
                if !record.supersedes.is_empty() {
                    line = line.cue(format!("supersedes {}", record.supersedes.join(", ")));
                }
                if !record.superseded_by.is_empty() {
                    line = line.cue(format!("superseded by {}", record.superseded_by.join(", ")));
                }
                line.render_wrapped(context, "")
            })
            .collect::<Vec<_>>()
    };
    let section = |title: &str, lines: Vec<String>| {
        if lines.is_empty() {
            Vec::new()
        } else {
            render_lines_section(title, lines)
        }
    };
    let malformed = decisions
        .malformed
        .iter()
        .flat_map(|note| {
            render_row_wrapped(
                context,
                &format!(
                    "⚠ {} decision note {} ({}) {}",
                    note.issue_id, note.number, note.date, note.defect
                ),
            )
        })
        .collect::<Vec<_>>();
    let summary = if decisions.notes.is_empty()
        && decisions.records.is_empty()
        && decisions.malformed.is_empty()
    {
        vec!["No decisions recorded in this mission's scope.".to_string()]
    } else {
        Vec::new()
    };
    let title = ItemLine::new(
        &mission.id,
        &mission.status,
        &mission.priority,
        &mission.title,
    )
    .issue_type(&mission.issue_type)
    .text();
    print_page(
        &format!("Decisions: {title}"),
        [
            summary,
            section("One-way Decisions", note_lines(true)),
            section(
                "Decision Records",
                record_lines(&|record| record.is_current()),
            ),
            section("Reversible Decisions", note_lines(false)),
            section(
                "Open Decision Records",
                record_lines(&|record| !record.decided && !record.is_superseded()),
            ),
            section(
                "Superseded Decisions",
                record_lines(&|record| record.is_superseded()),
            ),
            section("Malformed Decision Notes", malformed),
            render_footer_section(
                context,
                [
                    FooterAction::new(
                        "Record a decision",
                        "atelier issue note <id> --kind decision \"<choice>\" --reason \"<why>\"",
                    ),
                    FooterAction::new("Show the mission", format!("atelier work mission {id}")),
                ],
            ),
        ],
        context,
    );
    Ok(())
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
