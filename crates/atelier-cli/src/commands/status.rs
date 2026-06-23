use anyhow::Result;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

use crate::commands;
use crate::utils::format_issue_id;
use atelier_app::use_cases as app_use_cases;
use atelier_core::Issue;
use atelier_sqlite::{Database, RecordSummary};

pub fn run(db: &Database, state_dir: &Path, quiet: bool) -> Result<()> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let active_issues = current_work_issues(db, workflow_policy.as_ref())?;
    let active_issue_ids = active_issues
        .iter()
        .map(|issue| issue.id.as_str())
        .collect::<BTreeSet<_>>();
    let active_role_counts = active_role_counts(&active_issues, workflow_policy.as_ref());
    let current_missions = db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| issue.issue_type == "mission")
        .map(mission_summary_from_issue)
        .filter(|mission| mission.status != "closed")
        .filter(|mission| mission.status != "superseded")
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
    let export_stale = atelier_app::export::canonical_stale_entries(db, state_dir)?;
    let tracker_state = if export_stale.is_empty() {
        "current"
    } else {
        "stale"
    };

    if quiet {
        println!(
            "work={} current_missions={} ready={} tracker={}",
            if active_issues.is_empty() {
                "none".to_string()
            } else {
                active_issues.len().to_string()
            },
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

    println!("Current missions: {}", current_missions.len());
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
    println!("Evidence Status");
    println!("---------------");
    print_evidence_status(db, &active_issues, None, None, &ready)?;

    println!();
    println!("Recent Activity");
    println!("---------------");
    println!("(no active mission focus)");

    println!();
    println!("Next Actions");
    println!("------------");
    if current_missions.is_empty() {
        println!("  Inspect objective readiness: atelier issue status <id>");
    } else {
        println!(
            "  Inspect mission choices ({} current mission(s), none active): atelier issue table --kind mission",
            current_missions.len()
        );
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

fn mission_summary_from_issue(issue: Issue) -> RecordSummary {
    let id = issue.id.clone();
    RecordSummary {
        kind: "issue".to_string(),
        id: id.clone(),
        title: issue.title,
        status: issue.status,
        created_at: issue.created_at,
        updated_at: issue.updated_at,
        source_path: format!("issues/{id}.md"),
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

fn print_evidence_status(
    db: &Database,
    active_issues: &[Issue],
    active_mission: Option<&RecordSummary>,
    mission_snapshot: Option<&commands::objective_status::ObjectiveStatusSnapshot>,
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
        if commands::objective_status::has_validating_evidence(db, issue_id)? {
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
