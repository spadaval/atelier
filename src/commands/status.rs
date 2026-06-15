use anyhow::Result;

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use crate::activity::list_all_issue_activities;
use crate::models::{DomainRecord, Issue, RecordLink};
use crate::utils::format_issue_id;
use crate::{commands, db::Database};
use atelier_app::status::{
    self as app_status, GitStatusView, IssueStartReadiness, IssueSummary, MissionSummary,
    ResultView, StatusPorts, TrackerState,
};

pub fn run(db: &Database, state_dir: &Path, quiet: bool) -> Result<()> {
    let ports = StatusCommandPorts { db, state_dir };
    let view = app_status::load_status(&ports)?;

    if quiet {
        println!("{}", view.quiet_line());
        return Ok(());
    }

    println!("Atelier Status");
    println!("==============");
    println!("Tracker:       {}", view.tracker_state.label());
    println!("Ready work:    {}", view.ready_work_count);
    print_current_work_summary(&view.current_work);

    match &view.active_mission {
        Some(mission) => println!("Active mission: {} - {}", mission.id, mission.title),
        None if view.current_mission_count == 0 => println!("Active mission: none"),
        None => println!(
            "Active mission: none ({} current)",
            view.current_mission_count
        ),
    }

    if view.tracker_state.stale_count() > 0 {
        println!("Export issues: {}", view.tracker_state.stale_count());
    }

    println!();
    println!("Local State");
    println!("-----------");
    print_git_state(&view.git);
    println!("Tracker:  {}", view.tracker_state.label());

    if let Some(snapshot) = view.active_mission_snapshot.as_ref() {
        println!();
        println!("Active Mission");
        println!("--------------");
        println!("{} - {}", snapshot.mission.id, snapshot.mission.title);
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
            for selectable in snapshot.selectable_issues.iter().take(5) {
                println!(
                    "  {} - {} | ready: no open blockers; {}; {}",
                    selectable.issue.id,
                    selectable.issue.title,
                    selectable.parent_context,
                    selectable.proof_context
                );
            }
        }

        println!();
        println!("Immediate Blockers");
        println!("------------------");
        if snapshot.open_blockers.is_empty() {
            println!("(none)");
        } else {
            for blocker in snapshot.open_blockers.iter().take(5) {
                println!("  {} - {}", blocker.id, blocker.title);
            }
        }

        println!();
        println!("Recent Activity");
        println!("---------------");
        if snapshot.recent_activity.is_empty() {
            println!("(none)");
        } else {
            for activity in &snapshot.recent_activity {
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
    for action in view.next_actions() {
        println!("  {action}");
    }
    Ok(())
}

struct StatusCommandPorts<'a> {
    db: &'a Database,
    state_dir: &'a Path,
}

impl StatusPorts for StatusCommandPorts<'_> {
    fn current_work_issues(&self) -> Result<Vec<IssueSummary>> {
        current_work_issues(self.db)
    }

    fn all_issues(&self) -> Result<Vec<IssueSummary>> {
        Ok(self
            .db
            .list_issues(Some("all"), None, None)?
            .into_iter()
            .map(issue_summary)
            .collect())
    }

    fn issue(&self, issue_id: &str) -> Result<Option<IssueSummary>> {
        Ok(self.db.get_issue(issue_id)?.map(issue_summary))
    }

    fn blockers(&self, issue_id: &str) -> Result<Vec<String>> {
        self.db.get_blockers(issue_id)
    }

    fn subissues(&self, issue_id: &str) -> Result<Vec<IssueSummary>> {
        Ok(self
            .db
            .get_subissues(issue_id)?
            .into_iter()
            .map(issue_summary)
            .collect())
    }

    fn active_mission(&self) -> Result<Option<MissionSummary>> {
        Ok(commands::mission::active_mission(self.db)?.map(mission_summary))
    }

    fn current_missions(&self) -> Result<Vec<MissionSummary>> {
        Ok(self
            .db
            .list_records("mission", None)?
            .into_iter()
            .filter(|mission| mission.status != "closed")
            .map(mission_summary)
            .collect())
    }

    fn mission_issue_ids(&self, mission_id: &str) -> Result<BTreeSet<String>> {
        mission_issue_ids(self.db, mission_id)
    }

    fn mission_direct_blocker_ids(&self, mission_id: &str) -> Result<Vec<String>> {
        mission_direct_blocker_ids(self.db, mission_id)
    }

    fn has_validating_evidence(&self, issue_id: &str) -> Result<bool> {
        has_validating_evidence(self.db, issue_id)
    }

    fn issue_start_readiness(&self, issue: &IssueSummary) -> Result<IssueStartReadiness> {
        let Some(db_issue) = self.db.get_issue(&issue.id)? else {
            return Ok(IssueStartReadiness::NotReady);
        };
        let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
        Ok(
            match commands::issue_workflow::issue_start_readiness(
                self.db,
                workflow_policy.as_ref(),
                &db_issue,
            )? {
                commands::issue_workflow::IssueStartReadiness::Ready => IssueStartReadiness::Ready,
                commands::issue_workflow::IssueStartReadiness::Blocked => {
                    IssueStartReadiness::Blocked
                }
                commands::issue_workflow::IssueStartReadiness::NotReady => {
                    IssueStartReadiness::NotReady
                }
            },
        )
    }

    fn issue_status_category(&self, status: &str) -> Option<String> {
        let workflow_policy = commands::issue_workflow::load_issue_workflow_policy().ok()?;
        commands::issue_workflow::issue_status_category(workflow_policy.as_ref(), status)
    }

    fn tracker_state(&self) -> Result<TrackerState> {
        let export_stale = commands::export::canonical_stale_entries(self.db, self.state_dir)?;
        Ok(if export_stale.is_empty() {
            TrackerState::Current
        } else {
            TrackerState::Stale {
                issue_count: export_stale.len(),
            }
        })
    }

    fn recent_mission_activity(&self, issue_ids: &BTreeSet<String>) -> Result<Vec<String>> {
        recent_mission_activity(self.state_dir, issue_ids)
    }

    fn git_status(&self) -> Result<GitStatusView> {
        let state = git_state()?;
        Ok(GitStatusView {
            branch: state.branch,
            dirty_entries: state.dirty_entries,
        })
    }
}

pub(crate) fn current_work_issues(db: &Database) -> Result<Vec<IssueSummary>> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let mut issues = db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| is_current_work_issue(issue, workflow_policy.as_ref()))
        .map(issue_summary)
        .collect::<Vec<_>>();
    issues.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(issues)
}

pub(crate) fn format_current_work_line(issue: &IssueSummary) -> String {
    app_status::format_current_work_line(issue)
}

fn is_current_work_issue(
    issue: &Issue,
    workflow_policy: Option<&atelier_workflow::WorkflowPolicy>,
) -> bool {
    commands::issue_workflow::issue_status_category(workflow_policy, &issue.status).as_deref()
        == Some("active")
        || issue.status == "in_progress"
}

fn print_current_work_summary(current_work: &[IssueSummary]) {
    match current_work.len() {
        0 => println!("Current work:  none"),
        1 => {
            println!("Current work:  1 issue");
            println!("  {}", format_current_work_line(&current_work[0]));
        }
        count => {
            println!("Current work:  {count} issues");
            for issue in current_work {
                println!("  {}", format_current_work_line(issue));
            }
        }
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

fn other_side<'a>(link: &'a RecordLink, kind: &str, id: &str) -> Option<(&'a str, &'a str)> {
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

fn print_git_state(git: &ResultView<GitStatusView>) {
    match git {
        ResultView::Available(state) => {
            if let Some(branch) = state.branch.as_ref() {
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
        ResultView::Unavailable(error) => println!("Worktree: unavailable - {error}"),
    }
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

fn issue_summary(issue: Issue) -> IssueSummary {
    IssueSummary {
        id: issue.id,
        title: issue.title,
        status: issue.status,
        issue_type: issue.issue_type,
        parent_id: issue.parent_id,
    }
}

fn mission_summary(record: DomainRecord) -> MissionSummary {
    MissionSummary {
        id: record.id,
        title: record.title,
    }
}

pub fn close_all_lifecycle(
    state_dir: &Path,
    db_path: &Path,
    label_filter: Option<&str>,
    priority_filter: Option<&str>,
) -> Result<()> {
    let db = Database::open(db_path)?;
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
