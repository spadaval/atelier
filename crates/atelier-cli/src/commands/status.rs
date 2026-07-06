use anyhow::Result;

use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use crate::commands;
use crate::human_output::{
    IssueListPanel, IssueListRow, LinesPanel, MetadataPanel, Page, RenderContext,
};
use crate::utils::format_issue_id;
use atelier_app::read_pipeline::{StatusNextAction, StatusView};
use atelier_app::use_cases as app_use_cases;
use atelier_core::Issue;
use atelier_sqlite::Database;

pub fn run(db: &Database, state_dir: &Path, quiet: bool) -> Result<()> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let view = atelier_app::read_pipeline::status_view(db, state_dir, workflow_policy.as_ref())?;

    if quiet {
        println!(
            "work={} current_missions={} ready={} tracker={}",
            if view.work.active.is_empty() {
                "none".to_string()
            } else {
                view.work.active.len().to_string()
            },
            view.current_missions.len(),
            view.work.ready.len(),
            view.tracker_state
        );
        return Ok(());
    }

    print_status_view(db, &view)
}

fn print_status_view(_db: &Database, view: &StatusView) -> Result<()> {
    let current_work = if view.work.active.is_empty() {
        "none".to_string()
    } else {
        format!("{} issue(s)", view.work.active.len())
    };
    let active_roles = if view.active_role_counts.is_empty() {
        "none".to_string()
    } else {
        render_role_counts(&view.active_role_counts)
    };
    let mut summary_lines = vec![
        format!("Tracker:       {}", view.tracker_state),
        format!("Ready work:    {}", view.work.ready.len()),
        format!("Current work:  {current_work}"),
        format!("Current missions: {}", view.current_missions.len()),
        format!("Active roles:   {active_roles}"),
    ];
    if view.stale_records > 0 {
        summary_lines.push(format!("Local state issues: {}", view.stale_records));
    }

    let active_rows = view
        .work
        .active
        .iter()
        .map(|issue| IssueListRow {
            role: crate::human_output::DisplayRole::Executable,
            id: issue.id.clone(),
            status: issue.status_category.clone(),
            priority: issue.priority.clone(),
            title: issue.title.clone(),
            blockers: 0,
            depth: 1,
        })
        .collect::<Vec<_>>();

    let git = git_state();
    let local_state = match git {
        Ok(state) => {
            let mut panel = MetadataPanel::new("Local State");
            if let Some(branch) = state.branch {
                panel = panel.row("Branch", branch);
            }
            if state.dirty_entries.is_empty() {
                panel = panel.row("Checkout", "clean");
            } else {
                panel = panel.row(
                    "Checkout",
                    format!(
                        "dirty ({})",
                        crate::human_output::path_summary(&state.dirty_entries, 3)
                    ),
                );
            }
            panel.row("Tracker", view.tracker_state.clone())
        }
        Err(error) => MetadataPanel::new("Local State")
            .row("Checkout", format!("unavailable - {error}"))
            .row("Tracker", view.tracker_state.clone()),
    };

    let next_actions = match view.next_action {
        StatusNextAction::InspectReadyWork { count } => {
            vec![
                format!(
                    "  Choose ready work ({count} ready issue(s) available): atelier work ready"
                ),
                "  Inspect selected work transitions: atelier issue transition <issue-id>"
                    .to_string(),
            ]
        }
        StatusNextAction::InspectBlockedWork => {
            vec![
                "  Inspect blocked work (no ready work is available): atelier work blocked"
                    .to_string(),
            ]
        }
        StatusNextAction::InspectHealth { stale_records } => {
            vec![
                format!(
                    "  Repair local Atelier state ({stale_records} stale record(s)): atelier check --fix"
                ),
                "  Check committed tracker records after repair: atelier check".to_string(),
            ]
        }
        StatusNextAction::NoSpecificAction => {
            vec!["  No specific next action is available from checkout state.".to_string()]
        }
    };

    Page::new("Atelier Status")
        .panel(LinesPanel::new("", summary_lines))
        .panel(IssueListPanel::new("Current Work", active_rows).limit(8))
        .panel(local_state)
        .panel(LinesPanel::new("Next Actions", next_actions))
        .print(RenderContext::for_stdout());
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

fn render_role_counts(counts: &BTreeMap<String, usize>) -> String {
    counts
        .iter()
        .map(|(role, count)| format!("{role}={count}"))
        .collect::<Vec<_>>()
        .join(", ")
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
