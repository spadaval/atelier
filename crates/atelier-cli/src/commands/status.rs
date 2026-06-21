use anyhow::Result;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

use crate::commands;
use crate::utils::format_issue_id;
use atelier_app::use_cases as app_use_cases;
use atelier_core::Issue;
use atelier_records::activity::list_all_issue_activities;
use atelier_sqlite::Database;

pub fn run(db: &Database, state_dir: &Path, quiet: bool) -> Result<()> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let active_issues = current_work_issues(db, workflow_policy.as_ref())?;
    let active_issue_ids = active_issues
        .iter()
        .map(|issue| issue.id.as_str())
        .collect::<BTreeSet<_>>();
    let active_mission = commands::mission::active_mission(db)?;
    let active_role_counts = active_role_counts(&active_issues, workflow_policy.as_ref());
    let current_missions = db
        .list_records("mission", None)?
        .into_iter()
        .filter(|mission| mission.status != "closed")
        .collect::<Vec<_>>();
    let ready = db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| !active_issue_ids.contains(issue.id.as_str()))
        .filter_map(|issue| {
            match commands::objective_status::issue_state(db, workflow_policy.as_ref(), &issue) {
                Ok("ready") => Some(Ok(issue)),
                Ok(_) => None,
                Err(error) => Some(Err(error)),
            }
        })
        .collect::<Result<Vec<_>>>()?;
    let ready =
        commands::objective_status::order_issues_by_work(db, workflow_policy.as_ref(), ready)?;
    let mission_snapshot = active_mission
        .as_ref()
        .map(|mission| {
            commands::objective_status::snapshot_for_mission(db, &mission.id, &active_issue_ids)
        })
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
            let state =
                commands::objective_status::issue_state(db, workflow_policy.as_ref(), issue)?;
            println!(
                "  {state} {} - {} [{}]",
                issue.id,
                issue.title,
                issue_status_role(issue, workflow_policy.as_ref()).unwrap_or("role:unconfigured")
            );
        }
    }

    match &active_mission {
        Some(mission) => println!("Active mission: {} - {}", mission.id, mission.title),
        None if current_missions.is_empty() => println!("Active mission: none"),
        None => println!("Active mission: none ({} current)", current_missions.len()),
    }
    if active_role_counts.is_empty() {
        println!("Active roles:   none");
    } else {
        println!(
            "Active roles:   {}",
            render_role_counts(&active_role_counts)
        );
    }

    if !export_stale.is_empty() {
        println!("Local state issues: {}", export_stale.len());
    }

    println!();
    println!("Local State");
    println!("-----------");
    print_git_state();
    println!("Tracker:  {tracker_state}");

    println!();
    println!("Branch Policy");
    println!("----------------");
    print_branch_lifecycle_state(db, &active_issues)?;

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
                let state =
                    commands::objective_status::issue_state(db, workflow_policy.as_ref(), issue)?;
                println!(
                    "  {state} {} - {} | no open blockers; {}; {}",
                    issue.id,
                    issue.title,
                    commands::objective_status::parent_context(issue),
                    commands::objective_status::proof_context(db, &issue.id)?
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
                let blockers = commands::objective_status::open_issue_blockers(
                    db,
                    &issue.id,
                    workflow_policy.as_ref(),
                )?;
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
                "  Inspect active mission health ({}): atelier issue status {}",
                mission.id, mission.id
            );
            println!(
                "  Open active mission record ({}): atelier issue show {}",
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
                println!("  Inspect checkout context if state is unclear: atelier status");
            } else if let Some(issue) = snapshot.selectable_issues.first() {
                println!(
                    "  Inspect selectable active-mission work transitions ({} selectable issue(s)): atelier issue transition {} --options",
                    snapshot.selectable_issues.len(),
                    issue.id
                );
            } else if snapshot.blocked > 0 || !snapshot.open_blockers.is_empty() {
                println!(
                    "  Inspect blocked active-mission work (no ready work is available): atelier issue status {}",
                    mission.id
                );
            } else {
                println!(
                    "  Review active mission terminal state (no ready work is available): atelier issue status {}",
                    mission.id
                );
            }
        }
        None => match &active_mission {
            Some(mission) => println!(
                "  Inspect active mission ({} is active): atelier issue status {}",
                mission.id, mission.id
            ),
            None if current_missions.is_empty() => {
                println!("  Inspect mission readiness (no mission is active): atelier issue status")
            }
            None => println!(
            "  Inspect mission choices ({} current mission(s), none active): atelier issue status",
            current_missions.len()
        ),
        },
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
        println!(
            "  Inspect selected work transitions (ready work exists): atelier issue transition <issue-id> --options"
        );
    }
    if !export_stale.is_empty() {
        println!(
            "  Repair local Atelier state ({} stale record(s)): atelier doctor --fix",
            export_stale.len()
        );
        println!("  Check committed tracker records after repair: atelier lint");
    }
    Ok(())
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
    commands::objective_status::order_issues_by_work(db, workflow_policy, issues)
}

pub(crate) fn issue_status_role<'a>(
    issue: &'a Issue,
    workflow_policy: Option<&'a atelier_app::workflow_policy::WorkflowPolicy>,
) -> Option<&'a str> {
    workflow_policy.and_then(|policy| policy.status_role(&issue.status))
}

fn active_role_counts(
    issues: &[Issue],
    workflow_policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for issue in issues {
        let role = issue_status_role(issue, workflow_policy)
            .unwrap_or("unconfigured")
            .to_string();
        *counts.entry(role).or_insert(0) += 1;
    }
    counts
}

fn render_role_counts(counts: &BTreeMap<String, usize>) -> String {
    counts
        .iter()
        .map(|(role, count)| format!("{role}={count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn plural_suffix(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
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
                println!("Checkout: clean");
            } else {
                println!("Checkout: dirty ({} entries)", state.dirty_entries.len());
                for entry in state.dirty_entries.iter().take(3) {
                    println!("  {entry}");
                }
            }
        }
        Err(error) => println!("Checkout: unavailable - {error}"),
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
                    format!(
                        "mismatch; inspect `atelier issue transition {} --options` and `atelier status`",
                        issue.id
                    )
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
        let db = match app_use_cases::open_database(db_path) {
            Ok(db) => db,
            Err(e) => {
                tracing::warn!(
                    "Failed to open tracker for {}: {}",
                    format_issue_id(&issue.id),
                    e
                );
                continue;
            }
        };
        match commands::workflow::transition_issue(
            &db,
            state_dir,
            db_path,
            &issue.id,
            "close",
            Some("bulk close"),
        ) {
            Ok(()) => closed_count += 1,
            Err(e) => tracing::warn!("Failed to close {}: {}", format_issue_id(&issue.id), e),
        }
    }

    println!("Closed {} issue(s).", closed_count);
    Ok(())
}
