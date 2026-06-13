use anyhow::Result;

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use crate::activity::list_all_issue_activities;
use crate::models::{DomainRecord, Issue};
use crate::utils::format_issue_id;
use crate::{commands, db::Database};

pub fn run(db: &Database, state_dir: &Path, quiet: bool) -> Result<()> {
    let active_work = db.get_active_work_association()?;
    let active_issue_id = active_work.as_ref().map(|work| work.issue_id.as_str());
    let active_mission = commands::mission::active_mission(db)?;
    let open_missions = db.list_records("mission", Some("open"))?;
    let ready = db
        .list_ready_issues()?
        .into_iter()
        .filter(|issue| Some(issue.id.as_str()) != active_issue_id)
        .collect::<Vec<_>>();
    let mission_snapshot = active_mission
        .as_ref()
        .map(|mission| mission_snapshot(db, mission, active_issue_id))
        .transpose()?;
    let export_stale = commands::export::canonical_stale_entries(db, state_dir)?;
    let tracker_state = if export_stale.is_empty() {
        "current"
    } else {
        "stale"
    };

    if quiet {
        println!(
            "work={} active_mission={} open_missions={} ready={} tracker={}",
            if active_work.is_some() {
                "active"
            } else {
                "none"
            },
            active_mission
                .as_ref()
                .map(|mission| mission.id.as_str())
                .unwrap_or("none"),
            open_missions.len(),
            ready.len(),
            tracker_state
        );
        return Ok(());
    }

    println!("Atelier Status");
    println!("==============");
    println!("Tracker:       {tracker_state}");
    println!("Ready work:    {}", ready.len());

    match &active_work {
        Some(work) => {
            let title = db
                .get_issue(&work.issue_id)?
                .map(|issue| issue.title)
                .unwrap_or_else(|| "(issue missing)".to_string());
            println!("Active work:   {} - {}", work.issue_id, title);
            println!(
                "Work branch:   {}",
                work.branch.as_deref().unwrap_or("(none)")
            );
            println!(
                "Worktree:      {}",
                work.worktree_path.as_deref().unwrap_or("(none)")
            );
        }
        None => println!("Active work:   none"),
    }

    match &active_mission {
        Some(mission) => println!("Active mission: {} - {}", mission.id, mission.title),
        None if open_missions.is_empty() => println!("Active mission: none"),
        None => println!("Active mission: none ({} open)", open_missions.len()),
    }

    if !export_stale.is_empty() {
        println!("Export issues: {}", export_stale.len());
    }

    println!();
    println!("Local State");
    println!("-----------");
    print_git_state();
    println!("Tracker:  {tracker_state}");

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
        if snapshot.ready_issues.is_empty() {
            println!("(none)");
        } else {
            for issue in snapshot.ready_issues.iter().take(5) {
                println!("  {} - {}", issue.id, issue.title);
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
            if let Some(issue) = snapshot.active_issue.as_ref() {
                println!(
                    "  Finish active work ({}): atelier finish {}",
                    issue.id, issue.id
                );
            } else if let Some(issue) = snapshot.ready_issues.first() {
                println!(
                    "  Start ready active-mission work ({} ready issue(s)): atelier start {}",
                    snapshot.ready_issues.len(),
                    issue.id
                );
            } else if snapshot.blocked > 0 || !snapshot.open_blockers.is_empty() {
                println!(
                    "  Inspect blocked active-mission work (no ready work is available): atelier mission status {}",
                    mission.id
                );
            } else {
                println!(
                    "  Review active mission closeout (no ready work is available): atelier mission status {}",
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
                None if open_missions.is_empty() => {
                    println!("  Inspect mission readiness (no mission is active): atelier mission status")
                }
                None => println!(
            "  Inspect mission choices ({} open mission(s), none active): atelier mission status",
            open_missions.len()
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
        println!("  Check runtime health (tracker export is current): atelier doctor");
    } else {
        println!(
            "  Refresh canonical export ({} stale record(s)): atelier export",
            export_stale.len()
        );
        println!("  Check tracker records (export is stale): atelier lint");
    }
    Ok(())
}

#[derive(Default)]
struct MissionSnapshot {
    issue_ids: BTreeSet<String>,
    active_issue: Option<Issue>,
    ready_issues: Vec<Issue>,
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
            "closeout"
        } else {
            "steady"
        }
    }
}

fn mission_snapshot(
    db: &Database,
    mission: &DomainRecord,
    active_issue_id: Option<&str>,
) -> Result<MissionSnapshot> {
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
        .filter(|issue| issue.status == "open")
        .map(|issue| issue.id)
        .collect();
    snapshot.open_blockers.sort();

    for issue_id in &snapshot.issue_ids {
        let Some(issue) = db.get_issue(issue_id)? else {
            continue;
        };
        match issue_bucket(db, &issue, active_issue_id)? {
            IssueBucket::Active => {
                snapshot.active += 1;
                snapshot.active_issue = Some(issue);
            }
            IssueBucket::Ready => {
                snapshot.ready += 1;
                snapshot.ready_issues.push(issue);
            }
            IssueBucket::Blocked => snapshot.blocked += 1,
            IssueBucket::Done => snapshot.done += 1,
            IssueBucket::Backlog => snapshot.backlog += 1,
        }
    }
    snapshot.ready_issues.sort_by(|a, b| a.id.cmp(&b.id));
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
    active_issue_id: Option<&str>,
) -> Result<IssueBucket> {
    if issue.status == "closed" {
        return Ok(IssueBucket::Done);
    }
    if issue.status != "open" {
        return Ok(IssueBucket::Backlog);
    }
    if Some(issue.id.as_str()) == active_issue_id {
        return Ok(IssueBucket::Active);
    }
    if open_issue_blockers(db, &issue.id)?.is_empty() {
        Ok(IssueBucket::Ready)
    } else {
        Ok(IssueBucket::Blocked)
    }
}

fn open_issue_blockers(db: &Database, issue_id: &str) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for blocker_id in db.get_blockers(issue_id)? {
        if db.require_issue(&blocker_id)?.status == "open" {
            blockers.push(blocker_id);
        }
    }
    blockers.sort();
    Ok(blockers)
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
    link: &'a crate::models::RecordLink,
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

pub fn close_all(
    db: &Database,
    label_filter: Option<&str>,
    priority_filter: Option<&str>,
) -> Result<()> {
    let issues = db.list_issues(Some("open"), label_filter, priority_filter)?;

    if issues.is_empty() {
        println!("No matching open issues found.");
        return Ok(());
    }

    let mut closed_count = 0;
    for issue in &issues {
        match commands::agent_factory::close(db, &issue.id.to_string(), None) {
            Ok(()) => closed_count += 1,
            Err(e) => tracing::warn!("Failed to close {}: {}", format_issue_id(&issue.id), e),
        }
    }

    println!("Closed {} issue(s).", closed_count);
    Ok(())
}

pub fn close_all_lifecycle(
    state_dir: &Path,
    db_path: &Path,
    label_filter: Option<&str>,
    priority_filter: Option<&str>,
) -> Result<()> {
    let db = Database::open(db_path)?;
    let issues = db.list_issues(Some("open"), label_filter, priority_filter)?;
    drop(db);

    if issues.is_empty() {
        println!("No matching open issues found.");
        return Ok(());
    }

    let mut closed_count = 0;
    for issue in &issues {
        match commands::agent_factory::close_lifecycle(state_dir, db_path, &issue.id, None) {
            Ok(()) => closed_count += 1,
            Err(e) => tracing::warn!("Failed to close {}: {}", format_issue_id(&issue.id), e),
        }
    }

    println!("Closed {} issue(s).", closed_count);
    Ok(())
}
