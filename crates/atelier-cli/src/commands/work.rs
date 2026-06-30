use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::path::Path;
use std::process::Command;

use atelier_app::read_pipeline::{WorkBuckets, WorkRow};
use atelier_core::Issue;
use atelier_sqlite::Database;

use crate::human_output::{
    DisplayRole, FooterAction, FooterPanel, IssueListPanel, IssueListRow, LinesPanel,
    MetadataPanel, Page, RenderContext,
};

pub struct QueueOptions<'a> {
    pub status: &'a str,
    pub category: Option<&'a str>,
    pub label: Option<&'a str>,
    pub priority: Option<&'a str>,
    pub ready: bool,
    pub active: bool,
    pub blocked: bool,
    pub backlog: bool,
    pub all: bool,
}

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
        println!("queue");
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
                "mission  Live mission orchestration dashboard",
                "epic     Focused epic execution dashboard",
            ],
        ))
        .panel(FooterPanel::new(
            "Next Commands",
            [
                FooterAction::new("Browse ready work", "atelier work ready"),
                FooterAction::new("Triage blockers", "atelier work blocked"),
                FooterAction::new(
                    "Open mission dashboard",
                    "atelier work mission <mission-id>",
                ),
            ],
        ))
        .print(RenderContext::for_stdout());
    Ok(())
}

pub fn queue(db: &Database, options: QueueOptions<'_>, quiet: bool) -> Result<()> {
    let state_filter_count = [
        options.ready,
        options.active,
        options.blocked,
        options.backlog,
        options.all,
    ]
    .into_iter()
    .filter(|flag| *flag)
    .count();
    if state_filter_count > 1 {
        bail!("choose only one queue state filter");
    }
    if options.ready && (options.status != "todo" || options.category.is_some()) {
        bail!("--ready cannot be combined with --status or --category");
    }
    if options.blocked {
        if options.status != "todo" || options.category.is_some() {
            bail!("--blocked cannot be combined with --status or --category");
        }
        return crate::commands::issue::list_blocked_with_title(db, "Work Queue", quiet);
    }
    let (status, category, ready) = if options.all {
        ("all", None, false)
    } else if options.ready {
        ("todo", None, true)
    } else if options.active {
        ("todo", Some("active"), false)
    } else if options.backlog {
        ("todo", Some("backlog"), false)
    } else {
        (options.status, options.category, false)
    };
    crate::commands::issue::list_with_title(
        db,
        "Work Queue",
        Some(status),
        category,
        None,
        options.label,
        options.priority,
        ready,
        quiet,
    )
}

pub fn list(db: &Database, bucket: &str, quiet: bool) -> Result<()> {
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let buckets = atelier_app::read_pipeline::work_buckets(db, policy.as_ref())?;
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

pub fn missions(db: &Database, quiet: bool) -> Result<()> {
    crate::commands::issue::list_with_title(
        db,
        "Missions",
        Some("all"),
        None,
        Some("mission"),
        None,
        None,
        false,
        quiet,
    )
}

fn print_quiet(bucket: &str, buckets: &WorkBuckets) {
    let rows = match bucket {
        "ready" => &buckets.ready,
        "blocked" => &buckets.blocked,
        "active" => &buckets.active,
        "all" => &buckets.ready,
        _ => &buckets.ready,
    };
    for row in rows {
        println!("{}", row.id);
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
    db: &Database,
    mission_ref: &str,
    options: MissionDashboardOptions,
    quiet: bool,
) -> Result<()> {
    let filter = options.filter()?;
    let mission_id = crate::commands::issue::resolve_id(db, mission_ref)?;
    let mission = db.require_issue(&mission_id)?;
    if mission.issue_type != "mission" {
        bail!(
            "{} is issue_type '{}'; work mission requires issue_type 'mission'",
            mission.id,
            mission.issue_type
        );
    }
    let scoped = mission_scoped_issues(db, &mission.id)?;
    let filtered = filter_dashboard_issues(db, &scoped, filter)?;
    if quiet {
        if filter == DashboardFilter::Summary {
            let counts = dashboard_counts(db, &scoped)?;
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

    let counts = dashboard_counts(db, &scoped)?;
    let panels = if filter == DashboardFilter::Summary {
        dashboard_panels(db, &scoped)?
    } else {
        vec![dashboard_panel_for_filter(db, filter, &filtered)?]
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
    if scoped_work_is_terminal(&counts) {
        metadata = metadata.row("Closeout", transition_readiness(db, &mission.id, "close")?);
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

pub fn epic_dashboard(db: &Database, epic_ref: &str, quiet: bool) -> Result<()> {
    let epic_id = crate::commands::issue::resolve_id(db, epic_ref)?;
    let epic = db.require_issue(&epic_id)?;
    if epic.issue_type != "epic" {
        bail!(
            "{} is issue_type '{}'; work epic requires issue_type 'epic'",
            epic.id,
            epic.issue_type
        );
    }
    let mut scoped = descendant_issues(db, &epic.id)?;
    scoped.insert(0, epic.clone());
    if quiet {
        let counts = dashboard_counts(db, &scoped)?;
        println!(
            "{} active={} ready={} blocked={} done={} backlog={} proof_gaps={}",
            epic.id,
            counts.active,
            counts.ready,
            counts.blocked,
            counts.done,
            counts.backlog,
            proof_gap_count(db, &scoped)?
        );
        return Ok(());
    }

    let counts = dashboard_counts(db, &scoped)?;
    let proof_gaps = proof_gap_count(db, &scoped)?;
    let transition = transition_readiness(db, &epic.id, "start")?;
    let panels = dashboard_panels(db, &scoped)?;
    let mut page = Page::new(format!("Work Epic {} - {}", epic.id, epic.title)).panel(
        MetadataPanel::untitled()
            .row("Status", epic.status.clone())
            .row(
                "Mission",
                owning_mission_id(db, &epic.id).unwrap_or_else(|_| "(none)".to_string()),
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
            FooterAction::new(
                "Open review branch",
                format!("atelier branch for-epic {}", epic.id),
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

fn dashboard_counts(db: &Database, issues: &[Issue]) -> Result<DashboardCounts> {
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let mut counts = DashboardCounts::default();
    for issue in issues {
        match issue_category(db, policy.as_ref(), issue)?.as_str() {
            "active" => counts.active += 1,
            "blocked" => counts.blocked += 1,
            "done" => counts.done += 1,
            "backlog" => counts.backlog += 1,
            _ => counts.ready += 1,
        }
    }
    Ok(counts)
}

fn dashboard_panels(db: &Database, issues: &[Issue]) -> Result<Vec<IssueListPanel>> {
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let mut buckets = BTreeMap::<String, Vec<IssueListRow>>::new();
    for issue in issues {
        let category = issue_category(db, policy.as_ref(), issue)?;
        let role = match category.as_str() {
            "active" => DisplayRole::Executable,
            "blocked" => DisplayRole::Blocked,
            "done" | "backlog" => DisplayRole::ContextOnly,
            _ => DisplayRole::Selectable,
        };
        let open_blockers = crate::commands::issue_workflow::open_blocker_ids_with_policy(
            db,
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
    db: &Database,
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
            let category = issue_category(db, policy.as_ref(), issue)?;
            let role = match category.as_str() {
                "active" => DisplayRole::Executable,
                "blocked" => DisplayRole::Blocked,
                "done" | "backlog" => DisplayRole::ContextOnly,
                _ => DisplayRole::Selectable,
            };
            let open_blockers = crate::commands::issue_workflow::open_blocker_ids_with_policy(
                db,
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
    db: &Database,
    issues: &[Issue],
    filter: DashboardFilter,
) -> Result<Vec<Issue>> {
    if filter == DashboardFilter::Summary || filter == DashboardFilter::All {
        return Ok(issues.to_vec());
    }
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    issues
        .iter()
        .filter_map(|issue| match issue_category(db, policy.as_ref(), issue) {
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
        })
        .collect()
}

fn scoped_work_is_terminal(counts: &DashboardCounts) -> bool {
    counts.active == 0 && counts.ready == 0 && counts.blocked == 0 && counts.backlog == 0
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
    db: &Database,
    policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
    issue: &Issue,
) -> Result<String> {
    let open_blockers =
        crate::commands::issue_workflow::open_blocker_ids_with_policy(db, policy, &issue.id)?;
    if !open_blockers.is_empty() {
        return Ok("blocked".to_string());
    }
    Ok(
        crate::commands::issue_workflow::issue_status_category(policy, &issue.status)
            .unwrap_or_else(|| "todo".to_string()),
    )
}

fn mission_scoped_issues(db: &Database, mission_id: &str) -> Result<Vec<Issue>> {
    let mut scoped = Vec::new();
    for issue in db.list_issues(Some("all"), None, None)? {
        if issue.id == mission_id {
            continue;
        }
        if crate::commands::mission::issue_advances_mission(db, mission_id, &issue.id)? {
            scoped.push(issue);
        }
    }
    scoped.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(scoped)
}

fn descendant_issues(db: &Database, root_id: &str) -> Result<Vec<Issue>> {
    let mut out = Vec::new();
    let mut stack = db
        .get_subissues(root_id)?
        .into_iter()
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    while let Some(id) = stack.pop() {
        let issue = db.require_issue(&id)?;
        stack.extend(db.get_subissues(&id)?.into_iter().map(|child| child.id));
        out.push(issue);
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

fn proof_gap_count(db: &Database, issues: &[Issue]) -> Result<usize> {
    let mut count = 0;
    for issue in issues {
        if !crate::commands::issue::issue_evidence_gate_status(db, issue, None)?.passed {
            count += 1;
        }
    }
    Ok(count)
}

fn transition_readiness(db: &Database, issue_id: &str, transition_name: &str) -> Result<String> {
    let options = match crate::commands::workflow::issue_transition_options(db, issue_id) {
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

fn containing_mission(db: &Database, issue_id: &str) -> Result<Option<String>> {
    for mission in db.list_issues(Some("all"), None, None)? {
        if mission.issue_type != "mission" {
            continue;
        }
        if mission.status == "closed" {
            continue;
        }
        if crate::commands::mission::issue_advances_mission(db, &mission.id, issue_id)? {
            return Ok(Some(mission.id));
        }
    }
    Ok(None)
}

fn epic_branch_name(epic: &Issue) -> Result<String> {
    let root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&root)?;
    policy.branch_name_for_owner(epic, &atelier_app::workflow_policy::BranchOwnerKind::Epic)
}

pub fn branch_for_epic(db: &Database, epic_id: &str) -> Result<()> {
    let epic = db.require_issue(epic_id)?;
    if epic.issue_type != "epic" {
        bail!(
            "{} is issue_type '{}'; epic branch commands require issue_type 'epic'",
            epic.id,
            epic.issue_type
        );
    }
    let mission_id = owning_mission_id(db, &epic.id)?;
    ensure_clean_worktree()?;
    let branch = epic_branch_name(&epic)?;
    if branch_exists(&branch)? {
        git_switch(&branch)?;
    } else {
        let status = Command::new("git")
            .args(["switch", "-c", &branch])
            .status()
            .context("failed to run git switch")?;
        if !status.success() {
            bail!("git switch -c failed for {branch}");
        }
    }
    let checkout_path = env::current_dir().context("failed to read current checkout path")?;
    println!("Switched to {branch}");
    println!("Epic: {} {}", epic.id, epic.title);
    println!("Mission: {mission_id}");
    println!("Checkout: {}", checkout_path.display());
    Ok(())
}

pub fn branch_status(db: &Database) -> Result<()> {
    let checkout_path = env::current_dir().context("failed to read current checkout path")?;
    let current = current_branch().unwrap_or_else(|_| "(detached)".to_string());
    let root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&root)?;
    println!("Epic Branch Status");
    println!("==================");
    println!("Checkout: {}", checkout_path.display());
    println!("Current: {current}");

    let mut printed = BTreeSet::new();
    for mission in db.list_issues(Some("all"), None, None)? {
        if mission.issue_type != "mission" {
            continue;
        }
        if mission.status == "closed" {
            continue;
        }
        for issue in db.list_issues(None, None, None)? {
            if issue.issue_type != "epic" {
                continue;
            }
            if !crate::commands::mission::issue_advances_mission(db, &mission.id, &issue.id)? {
                continue;
            }
            let branch = policy.branch_name_for_owner(
                &issue,
                &atelier_app::workflow_policy::BranchOwnerKind::Epic,
            )?;
            if branch_exists(&branch)? && printed.insert(branch.clone()) {
                println!(
                    "  {branch} - {} [{}] (mission {})",
                    issue.title, issue.status, mission.id
                );
            }
        }
    }
    Ok(())
}

pub fn branch_merge(db: &Database, epic_id: &str) -> Result<()> {
    let epic = db.require_issue(epic_id)?;
    if epic.issue_type != "epic" {
        bail!(
            "{} is issue_type '{}'; epic branch commands require issue_type 'epic'",
            epic.id,
            epic.issue_type
        );
    }
    let mission_id = owning_mission_id(db, &epic.id)?;
    ensure_clean_worktree()?;
    let branch = epic_branch_name(&epic)?;
    if !branch_exists(&branch)? {
        bail!("No epic branch found for {}: {branch}", epic.id);
    }
    let status = Command::new("git")
        .args(["merge", "--no-ff", &branch])
        .status()
        .context("failed to run git merge")?;
    if !status.success() {
        bail!("git merge failed; resolve conflicts with Git, then rerun validation");
    }
    let checkout_path = env::current_dir().context("failed to read current checkout path")?;
    println!("Merged {branch}");
    println!("Mission: {mission_id}");
    println!("Checkout: {}", checkout_path.display());
    Ok(())
}

fn branch_exists(branch: &str) -> Result<bool> {
    let output = Command::new("git")
        .args(["rev-parse", "--verify", "--quiet", branch])
        .output()
        .context("failed to inspect git branch")?;
    Ok(output.status.success())
}

fn git_switch(branch: &str) -> Result<()> {
    let status = Command::new("git")
        .args(["switch", branch])
        .status()
        .context("failed to run git switch")?;
    if !status.success() {
        bail!("git switch failed for {branch}");
    }
    Ok(())
}

fn ensure_clean_worktree() -> Result<()> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .context("failed to run git status")?;
    if !output.status.success() {
        bail!("git status failed");
    }
    let status = String::from_utf8_lossy(&output.stdout);
    let dirty = status
        .lines()
        .filter_map(git_status_path)
        .filter(|path| !is_workflow_generated_dirty_path(path))
        .collect::<Vec<_>>();
    if !dirty.is_empty() {
        bail!(
            "Checkout has uncommitted changes; commit or stash them before this workflow action:\n{}",
            dirty.join("\n")
        );
    }
    Ok(())
}

fn is_workflow_generated_dirty_path(path: &str) -> bool {
    path.starts_with(".atelier/")
}

fn git_status_path(line: &str) -> Option<String> {
    let path = line.get(3..)?.trim();
    let path = path.split(" -> ").last().unwrap_or(path);
    if path.is_empty() {
        None
    } else {
        Some(path.to_string())
    }
}

fn current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .context("failed to read current branch")?;
    if !output.status.success() {
        bail!("git branch --show-current failed");
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn repo_root() -> Result<std::path::PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("failed to locate git repository")?;
    if !output.status.success() {
        bail!("Not in a git repository");
    }
    Ok(Path::new(String::from_utf8_lossy(&output.stdout).trim()).to_path_buf())
}

fn owning_mission_id(db: &Database, issue_id: &str) -> Result<String> {
    containing_mission(db, issue_id)?.ok_or_else(|| {
        anyhow::anyhow!(
            "{issue_id} is not linked to an open mission. Link it to a mission before using epic branch commands."
        )
    })
}
