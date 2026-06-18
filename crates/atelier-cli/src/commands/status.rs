use anyhow::Result;

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use crate::commands;
use crate::commands::work_order::WorkOrderRow;
use crate::utils::format_issue_id;
use atelier_app::use_cases as app_use_cases;
use atelier_core::Issue;
use atelier_records::activity::{
    list_all_issue_activities, list_derived_issue_attempts, DerivedIssueAttempt,
    DerivedIssueAttemptState,
};
use atelier_sqlite::{Database, RecordSummary};

pub fn run(db: &Database, state_dir: &Path, quiet: bool) -> Result<()> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let active_issues = current_work_issues(db, workflow_policy.as_ref())?;
    let active_issue_ids = active_issues
        .iter()
        .map(|issue| issue.id.as_str())
        .collect::<BTreeSet<_>>();
    let active_mission = commands::mission::active_mission(db)?;
    let active_sessions = active_session_records(state_dir)?;
    let current_missions = db
        .list_records("mission", None)?
        .into_iter()
        .filter(|mission| mission.status != "closed")
        .collect::<Vec<_>>();
    let ready = db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| !active_issue_ids.contains(issue.id.as_str()))
        .filter_map(
            |issue| match status_issue_state(db, workflow_policy.as_ref(), &issue) {
                Ok("ready") => Some(Ok(issue)),
                Ok(_) => None,
                Err(error) => Some(Err(error)),
            },
        )
        .collect::<Result<Vec<_>>>()?;
    let ready = order_issues_by_work(db, workflow_policy.as_ref(), ready)?;
    let mission_snapshot = active_mission
        .as_ref()
        .map(|mission| mission_snapshot(db, mission, &active_issue_ids))
        .transpose()?;
    let export_stale = atelier_app::export::canonical_stale_entries(db, state_dir)?;
    let tracker_state = if export_stale.is_empty() {
        "current"
    } else {
        "stale"
    };

    if quiet {
        println!(
            "work={} active_mission={} current_missions={} ready={} tracker={}",
            if active_issues.is_empty() {
                "none".to_string()
            } else {
                active_issues.len().to_string()
            },
            active_mission
                .as_ref()
                .map(|mission| mission.id.as_str())
                .unwrap_or("none"),
            current_missions.len(),
            ready.len(),
            tracker_state
        );
        return Ok(());
    }

    println!("Atelier Status");
    println!("==============");
    println!("Tracker:       {tracker_state}");
    println!("Ready work:    {}", ready.len());

    if active_issues.is_empty() {
        println!("Current work:  none");
    } else {
        println!("Current work:  {} issue(s)", active_issues.len());
        for issue in &active_issues {
            let state = status_issue_state(db, workflow_policy.as_ref(), issue)?;
            println!("  {state} {} - {}", issue.id, issue.title);
        }
    }

    match &active_mission {
        Some(mission) => println!("Active mission: {} - {}", mission.id, mission.title),
        None if current_missions.is_empty() => println!("Active mission: none"),
        None => println!("Active mission: none ({} current)", current_missions.len()),
    }
    if active_sessions.is_empty() {
        println!("Active sessions: none");
    } else {
        println!("Active sessions: {}", active_sessions.len());
        for session in &active_sessions {
            println!(
                "  {} {} {} -> issue/{}",
                session.id,
                session.role,
                session.state.as_str(),
                session.issue_id
            );
        }
    }

    if !export_stale.is_empty() {
        println!("Export issues: {}", export_stale.len());
    }

    println!();
    println!("Local State");
    println!("-----------");
    print_git_state();
    println!("Tracker:  {tracker_state}");

    println!();
    println!("Branch Lifecycle");
    println!("----------------");
    print_branch_lifecycle_state(db, &active_issues)?;

    println!();
    println!("Evidence Status");
    println!("---------------");
    print_evidence_status(
        db,
        &active_issues,
        active_mission.as_ref(),
        mission_snapshot.as_ref(),
        &ready,
    )?;

    if let Some((mission, snapshot)) = active_mission.as_ref().zip(mission_snapshot.as_ref()) {
        println!();
        println!("Active Mission");
        println!("--------------");
        println!("{} - {}", mission.id, mission.title);
        println!("Health:   {}", snapshot.health());
        if snapshot.active > 0 {
            println!(
                "Work:     ready {}, active {}, blocked {}, done {}, backlog {}",
                snapshot.ready, snapshot.active, snapshot.blocked, snapshot.done, snapshot.backlog
            );
        } else {
            println!(
                "Work:     ready {}, blocked {}, done {}, backlog {}",
                snapshot.ready, snapshot.blocked, snapshot.done, snapshot.backlog
            );
        }

        println!();
        println!("Ready In Active Mission");
        println!("-----------------------");
        if snapshot.selectable_issues.is_empty() {
            println!("(none)");
        } else {
            for issue in snapshot.selectable_issues.iter().take(5) {
                let state = status_issue_state(db, workflow_policy.as_ref(), issue)?;
                println!(
                    "  {state} {} - {} | no open blockers; {}; {}",
                    issue.id,
                    issue.title,
                    parent_context(issue),
                    proof_context(db, &issue.id)?
                );
            }
        }

        println!();
        println!("Blocked In Active Mission");
        println!("-------------------------");
        if snapshot.blocked_issues.is_empty() {
            println!("(none)");
        } else {
            for issue in snapshot.blocked_issues.iter().take(5) {
                let blockers = open_issue_blockers(db, &issue.id, workflow_policy.as_ref())?;
                println!(
                    "  blocked {} - {} | {} blocker{}; details: atelier issue blocked {}",
                    issue.id,
                    issue.title,
                    blockers.len(),
                    plural_suffix(blockers.len()),
                    issue.id
                );
            }
        }

        println!();
        println!("Immediate Blockers");
        println!("------------------");
        if snapshot.open_blockers.is_empty() {
            println!("(none)");
        } else {
            for blocker_id in snapshot.open_blockers.iter().take(5) {
                let title = db
                    .get_issue(blocker_id)?
                    .map(|issue| issue.title)
                    .unwrap_or_else(|| "(issue missing)".to_string());
                println!("  {blocker_id} - {title}");
            }
        }

        println!();
        println!("Recent Activity");
        println!("---------------");
        let recent = recent_mission_activity(state_dir, &snapshot.issue_ids)?;
        if recent.is_empty() {
            println!("(none)");
        } else {
            for activity in recent {
                println!("{activity}");
            }
        }
    } else {
        println!();
        println!("Recent Activity");
        println!("---------------");
        println!("(no active mission)");
    }

    println!();
    println!("Next Actions");
    println!("------------");
    match active_mission.as_ref().zip(mission_snapshot.as_ref()) {
        Some((mission, snapshot)) => {
            println!(
                "  Inspect active mission health ({}): atelier mission status {}",
                mission.id, mission.id
            );
            println!(
                "  Open active mission record ({}): atelier mission show {}",
                mission.id, mission.id
            );
            if !snapshot.active_issues.is_empty() {
                for issue in snapshot.active_issues.iter().take(3) {
                    println!(
                        "  Inspect current work transitions ({}): atelier issue transition {} --options",
                        issue.id, issue.id
                    );
                }
                if snapshot.active_issues.len() > 3 {
                    println!(
                        "  Inspect remaining current work ({} more issue(s)): atelier status",
                        snapshot.active_issues.len() - 3
                    );
                }
                println!(
                    "  Inspect worktree context if checkout state is unclear: atelier worktree status"
                );
            } else if let Some(issue) = snapshot.selectable_issues.first() {
                println!(
                    "  Start selectable active-mission work ({} selectable issue(s)): atelier start {}",
                    snapshot.selectable_issues.len(),
                    issue.id
                );
            } else if snapshot.blocked > 0 || !snapshot.open_blockers.is_empty() {
                println!(
                    "  Inspect blocked active-mission work (no ready work is available): atelier mission status {}",
                    mission.id
                );
            } else {
                println!(
                    "  Review active mission terminal state (no ready work is available): atelier mission status {}",
                    mission.id
                );
            }
        }
        None => {
            match &active_mission {
                Some(mission) => println!(
                    "  Inspect active mission ({} is active): atelier mission status {}",
                    mission.id, mission.id
                ),
                None if current_missions.is_empty() => {
                    println!("  Inspect mission readiness (no mission is active): atelier mission status")
                }
                None => println!(
            "  Inspect mission choices ({} current mission(s), none active): atelier mission status",
            current_missions.len()
        ),
            }
        }
    }
    if active_mission.is_some() {
        // Active-mission scoped actions above own work selection in focused runs.
    } else if ready.is_empty() {
        println!(
            "  Inspect blocked work (no ready work is available): atelier issue list --blocked"
        );
    } else {
        println!(
            "  Choose ready work ({} ready issue(s) available): atelier issue list --ready",
            ready.len()
        );
        println!("  Start selected work (ready work exists): atelier start <issue-id>");
    }
    if export_stale.is_empty() {
        println!("  Check runtime health (tracker records are current): atelier doctor");
    } else {
        println!(
            "  Repair ignored projection state ({} stale record(s)): atelier doctor --fix",
            export_stale.len()
        );
        println!("  Check tracker records (projection is stale): atelier lint");
    }
    Ok(())
}

fn active_session_records(state_dir: &Path) -> Result<Vec<DerivedIssueAttempt>> {
    let mut sessions = list_derived_issue_attempts(state_dir)?
        .into_iter()
        .filter(|session| session.state == DerivedIssueAttemptState::Active)
        .collect::<Vec<_>>();
    sessions.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(sessions)
}

#[derive(Default)]
struct MissionSnapshot {
    issue_ids: BTreeSet<String>,
    active_issues: Vec<Issue>,
    ready_issues: Vec<Issue>,
    selectable_issues: Vec<Issue>,
    blocked_issues: Vec<Issue>,
    open_blockers: Vec<String>,
    active: usize,
    ready: usize,
    blocked: usize,
    done: usize,
    backlog: usize,
}

impl MissionSnapshot {
    fn health(&self) -> &'static str {
        if !self.open_blockers.is_empty() || self.blocked > 0 {
            "blocked"
        } else if self.active > 0 {
            "active"
        } else if self.ready > 0 {
            "ready"
        } else if self.done > 0 {
            "terminal"
        } else {
            "steady"
        }
    }
}

fn mission_snapshot(
    db: &Database,
    mission: &RecordSummary,
    active_issue_ids: &BTreeSet<&str>,
) -> Result<MissionSnapshot> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let mut snapshot = MissionSnapshot::default();
    snapshot.issue_ids = mission_issue_ids(db, &mission.id)?;

    let mut blocker_ids = mission_direct_blocker_ids(db, &mission.id)?
        .into_iter()
        .collect::<BTreeSet<_>>();
    for issue_id in &snapshot.issue_ids {
        for blocker_id in db.get_blockers(issue_id)? {
            blocker_ids.insert(blocker_id);
        }
    }
    snapshot.open_blockers = blocker_ids
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter(|issue| {
            commands::issue_workflow::issue_blocks_work(workflow_policy.as_ref(), issue)
        })
        .map(|issue| issue.id)
        .collect();
    snapshot.open_blockers.sort();

    for issue_id in &snapshot.issue_ids {
        let Some(issue) = db.get_issue(issue_id)? else {
            continue;
        };
        match issue_bucket(db, &issue, active_issue_ids, workflow_policy.as_ref())? {
            IssueBucket::Active => {
                snapshot.active += 1;
                snapshot.active_issues.push(issue);
            }
            IssueBucket::Ready => {
                snapshot.ready += 1;
                if is_selectable_work(db, &issue)? {
                    snapshot.selectable_issues.push(issue.clone());
                }
                snapshot.ready_issues.push(issue);
            }
            IssueBucket::Blocked => {
                snapshot.blocked += 1;
                snapshot.blocked_issues.push(issue);
            }
            IssueBucket::Done => snapshot.done += 1,
            IssueBucket::Backlog => snapshot.backlog += 1,
        }
    }
    snapshot.active_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.active_issues)?;
    snapshot.ready_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.ready_issues)?;
    snapshot.selectable_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.selectable_issues)?;
    snapshot.blocked_issues =
        order_issues_by_work(db, workflow_policy.as_ref(), snapshot.blocked_issues)?;
    Ok(snapshot)
}

enum IssueBucket {
    Active,
    Ready,
    Blocked,
    Done,
    Backlog,
}

fn issue_bucket(
    db: &Database,
    issue: &Issue,
    active_issue_ids: &BTreeSet<&str>,
    workflow_policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
) -> Result<IssueBucket> {
    if active_issue_ids.contains(issue.id.as_str()) {
        return Ok(IssueBucket::Active);
    }
    if commands::issue_workflow::issue_is_done(workflow_policy, issue) {
        return Ok(IssueBucket::Done);
    }
    if !open_issue_blockers(db, &issue.id, workflow_policy)?.is_empty() {
        return Ok(IssueBucket::Blocked);
    }
    match status_issue_state(db, workflow_policy, issue)? {
        "ready" => Ok(IssueBucket::Ready),
        _ => Ok(IssueBucket::Backlog),
    }
}

pub(crate) fn current_work_issues(
    db: &Database,
    workflow_policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
) -> Result<Vec<Issue>> {
    let issues = db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| {
            commands::issue_workflow::issue_status_category(workflow_policy, &issue.status)
                .as_deref()
                == Some("active")
        })
        .collect::<Vec<_>>();
    order_issues_by_work(db, workflow_policy, issues)
}

fn order_issues_by_work(
    db: &Database,
    workflow_policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
    issues: Vec<Issue>,
) -> Result<Vec<Issue>> {
    let rows = issues
        .iter()
        .map(|issue| work_order_row_for_issue(db, workflow_policy, issue))
        .collect::<Result<Vec<_>>>()?;
    let mut keyed = issues.into_iter().map(Some).collect::<Vec<_>>();
    Ok(commands::work_order::ordered_work_indices(&rows)
        .into_iter()
        .filter_map(|index| keyed[index].take())
        .collect())
}

fn work_order_row_for_issue(
    db: &Database,
    workflow_policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
    issue: &Issue,
) -> Result<WorkOrderRow> {
    Ok(WorkOrderRow {
        id: issue.id.clone(),
        status_category: commands::issue_workflow::issue_status_category(
            workflow_policy,
            &issue.status,
        ),
        priority: issue.priority.clone(),
        updated_at: issue.updated_at,
        open_blockers: open_issue_blockers(db, &issue.id, workflow_policy)?,
    })
}

fn status_issue_state(
    db: &Database,
    workflow_policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
    issue: &Issue,
) -> Result<&'static str> {
    Ok(work_order_row_for_issue(db, workflow_policy, issue)?
        .state()
        .label())
}

fn open_issue_blockers(
    db: &Database,
    issue_id: &str,
    workflow_policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for blocker_id in db.get_blockers(issue_id)? {
        if commands::issue_workflow::issue_blocks_work(
            workflow_policy,
            &db.require_issue(&blocker_id)?,
        ) {
            blockers.push(blocker_id);
        }
    }
    blockers.sort();
    Ok(blockers)
}

fn is_selectable_work(db: &Database, issue: &Issue) -> Result<bool> {
    Ok(issue.issue_type != "epic" || db.get_subissues(&issue.id)?.is_empty())
}

fn parent_context(issue: &Issue) -> String {
    match issue.parent_id.as_deref() {
        Some(parent_id) => format!("parent {parent_id}"),
        None => "mission-linked root".to_string(),
    }
}

fn proof_context(db: &Database, issue_id: &str) -> Result<&'static str> {
    if has_validating_evidence(db, issue_id)? {
        Ok("proof attached")
    } else {
        Ok("proof missing")
    }
}

fn print_evidence_status(
    db: &Database,
    active_issues: &[Issue],
    active_mission: Option<&RecordSummary>,
    mission_snapshot: Option<&MissionSnapshot>,
    ready: &[Issue],
) -> Result<()> {
    let proof_issue_ids = if let Some(snapshot) = mission_snapshot {
        active_issues
            .iter()
            .chain(snapshot.selectable_issues.iter())
            .map(|issue| issue.id.as_str())
            .collect::<BTreeSet<_>>()
    } else {
        active_issues
            .iter()
            .chain(ready.iter())
            .map(|issue| issue.id.as_str())
            .collect::<BTreeSet<_>>()
    };

    if proof_issue_ids.is_empty() {
        if active_mission.is_some() {
            println!("Attached Proof: irrelevant - no current or selectable mission work");
        } else {
            println!("Attached Proof: irrelevant - no current or ready work");
        }
        return Ok(());
    }

    let mut attached = 0usize;
    let mut missing = Vec::new();
    for issue_id in &proof_issue_ids {
        if has_validating_evidence(db, issue_id)? {
            attached += 1;
        } else {
            missing.push((*issue_id).to_string());
        }
    }

    if missing.is_empty() {
        println!("Attached Proof: attached - {attached} issue(s) have validating evidence");
    } else {
        println!(
            "Attached Proof: missing - {} issue(s) without validating evidence; {attached} attached",
            missing.len()
        );
        for issue_id in missing.iter().take(3) {
            println!("  Missing: {issue_id}");
        }
        if missing.len() > 3 {
            println!("  Missing: {} more issue(s)", missing.len() - 3);
        }
        println!("  Next: atelier evidence record --target issue/<id> --kind validation \"...\"");
        println!("  Next: atelier evidence attach <evidence-id> issue <issue-id>");
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

fn has_validating_evidence(db: &Database, issue_id: &str) -> Result<bool> {
    for link in db.list_record_links("issue", issue_id)? {
        if link.relation_type != "validates" {
            continue;
        }
        if link.source_kind == "evidence" || link.target_kind == "evidence" {
            return Ok(true);
        }
    }
    Ok(false)
}

fn mission_issue_ids(db: &Database, mission_id: &str) -> Result<BTreeSet<String>> {
    let mut issue_ids = BTreeSet::new();
    for link in db.list_record_links("mission", mission_id)? {
        let Some((kind, linked_id)) = other_side(&link, "mission", mission_id) else {
            continue;
        };
        if kind == "issue" && link.relation_type == "advances" {
            collect_issue_and_descendants(db, linked_id, &mut issue_ids)?;
        }
    }
    Ok(issue_ids)
}

fn collect_issue_and_descendants(
    db: &Database,
    issue_id: &str,
    issue_ids: &mut BTreeSet<String>,
) -> Result<()> {
    if !issue_ids.insert(issue_id.to_string()) {
        return Ok(());
    }
    for child in db.get_subissues(issue_id)? {
        collect_issue_and_descendants(db, &child.id, issue_ids)?;
    }
    Ok(())
}

fn mission_direct_blocker_ids(db: &Database, mission_id: &str) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for link in db.list_record_links("mission", mission_id)? {
        if link.relation_type != "blocked_by" {
            continue;
        }
        let Some((kind, linked_id)) = other_side(&link, "mission", mission_id) else {
            continue;
        };
        if kind == "issue" {
            blockers.push(linked_id.to_string());
        }
    }
    Ok(blockers)
}

fn other_side<'a>(
    link: &'a atelier_core::RecordLink,
    kind: &str,
    id: &str,
) -> Option<(&'a str, &'a str)> {
    if link.source_kind == kind && link.source_id == id {
        Some((&link.target_kind, &link.target_id))
    } else if link.target_kind == kind && link.target_id == id {
        Some((&link.source_kind, &link.source_id))
    } else {
        None
    }
}

fn recent_mission_activity(state_dir: &Path, issue_ids: &BTreeSet<String>) -> Result<Vec<String>> {
    let mut activities = list_all_issue_activities(state_dir)?
        .into_iter()
        .filter(|activity| issue_ids.contains(&activity.subject_id))
        .collect::<Vec<_>>();
    activities.sort_by(|a, b| b.created_at.cmp(&a.created_at).then(b.id.cmp(&a.id)));
    Ok(activities
        .into_iter()
        .take(3)
        .map(|activity| {
            format!(
                "  {} {}: {}",
                activity.subject_id, activity.event_type, activity.summary
            )
        })
        .collect())
}

fn print_git_state() {
    match git_state() {
        Ok(state) => {
            if let Some(branch) = state.branch {
                println!("Branch:   {branch}");
            }
            if state.dirty_entries.is_empty() {
                println!("Worktree: clean");
            } else {
                println!("Worktree: dirty ({} entries)", state.dirty_entries.len());
                for entry in state.dirty_entries.iter().take(3) {
                    println!("  {entry}");
                }
            }
        }
        Err(error) => println!("Worktree: unavailable - {error}"),
    }
}

fn print_branch_lifecycle_state(db: &Database, active_issues: &[Issue]) -> Result<()> {
    let current_branch = commands::workflow::current_git_branch()?;
    let base_branch = commands::workflow::configured_base_branch()?;
    println!(
        "Current branch: {}",
        current_branch.as_deref().unwrap_or("(detached)")
    );
    println!("Base branch:    {base_branch}");
    match current_branch.as_deref() {
        Some(branch) => match commands::workflow::known_branch_owner(db, branch)? {
            Some(owner) => println!(
                "Branch owner:   {} {} ({})",
                commands::workflow::branch_owner_label(&owner.owner_kind),
                owner.owner_id,
                owner.owner_issue_type
            ),
            None => println!("Branch owner:   (unknown)"),
        },
        None => println!("Branch owner:   (unknown)"),
    }

    if active_issues.is_empty() {
        println!("Active work:    none");
        return Ok(());
    }

    println!("Active work:");
    for issue in active_issues {
        match commands::workflow::branch_lifecycle_context(db, &issue.id) {
            Ok(context) => {
                let state = if context.current_branch.as_deref()
                    == Some(context.resolution.expected_branch.as_str())
                {
                    "ok".to_string()
                } else {
                    format!("mismatch; run `atelier start {}`", issue.id)
                };
                println!(
                    "  {} - owner {} {} ({}) | expected {} | {state}",
                    issue.id,
                    commands::workflow::branch_owner_label(&context.resolution.owner_kind),
                    context.resolution.owner_id,
                    context.resolution.owner_issue_type,
                    context.resolution.expected_branch
                );
            }
            Err(error) => println!("  {} - branch context unavailable: {error}", issue.id),
        }
    }
    Ok(())
}

struct GitState {
    branch: Option<String>,
    dirty_entries: Vec<String>,
}

fn git_state() -> Result<GitState> {
    let output = Command::new("git")
        .args(["status", "--short", "--branch", "--untracked-files=all"])
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        anyhow::bail!(if stderr.is_empty() {
            "git status failed".to_string()
        } else {
            stderr
        });
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut branch = None;
    let mut dirty_entries = Vec::new();
    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            branch = Some(rest.to_string());
        } else if !line.trim().is_empty() {
            dirty_entries.push(line.to_string());
        }
    }
    Ok(GitState {
        branch,
        dirty_entries,
    })
}

pub fn close_all_lifecycle(
    state_dir: &Path,
    db_path: &Path,
    label_filter: Option<&str>,
    priority_filter: Option<&str>,
) -> Result<()> {
    let db = app_use_cases::open_database(db_path)?;
    let issues = db.list_issues(Some("todo"), label_filter, priority_filter)?;
    drop(db);

    if issues.is_empty() {
        println!("No matching todo issues found.");
        return Ok(());
    }

    let mut closed_count = 0;
    for issue in &issues {
        match commands::agent_factory::close_lifecycle(
            state_dir,
            db_path,
            &issue.id,
            "bulk close",
            None,
        ) {
            Ok(()) => closed_count += 1,
            Err(e) => tracing::warn!("Failed to close {}: {}", format_issue_id(&issue.id), e),
        }
    }

    println!("Closed {} issue(s).", closed_count);
    Ok(())
}
