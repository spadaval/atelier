use anyhow::Result;

use std::path::Path;

use crate::utils::format_issue_id;
use crate::{commands, db::Database};

pub fn run(db: &Database, state_dir: &Path, quiet: bool) -> Result<()> {
    let active_work = db.get_active_work_association()?;
    let active_mission = commands::mission::active_mission(db)?;
    let open_missions = db.list_records("mission", Some("open"))?;
    let ready = db.list_ready_issues()?;
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

    match active_work {
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
    println!("Next Actions");
    println!("------------");
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
    if ready.is_empty() {
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
