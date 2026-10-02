use anyhow::Result;
use atelier_app::record_index::RecordIndex;

use std::process::Command;

use crate::commands;
use crate::human_output::{
    IssueListPanel, IssueListRow, LinesPanel, MetadataPanel, Page, RenderContext,
};
use atelier_app::work_classification::StatusView;

pub fn run(index: &RecordIndex, quiet: bool) -> Result<()> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let view = atelier_app::work_classification::status_view(index, workflow_policy.as_ref())?;

    if quiet {
        println!(
            "work={} current_missions={} ready={}",
            if view.work.active.is_empty() {
                "none".to_string()
            } else {
                view.work.active.len().to_string()
            },
            view.current_mission_count,
            view.work.ready.len(),
        );
        return Ok(());
    }

    print_status_view(&view);
    let layout = atelier_app::storage_layout::StorageLayout::discover()?;
    let lines =
        crate::commands::git::branch_lines(layout.repo_root(), &layout.canonical_dir(), index)
            .unwrap_or_else(|error| vec![format!("Branch orientation unavailable: {error}")]);
    if !lines.is_empty() {
        Page::new("Other local work branches")
            .panel(LinesPanel::new("", lines))
            .print(RenderContext::for_stdout());
    }
    Ok(())
}

fn print_status_view(view: &StatusView) {
    let current_work = if view.work.active.is_empty() {
        "none".to_string()
    } else {
        format!("{} issue(s)", view.work.active.len())
    };
    let summary_lines = vec![
        format!("Ready work:    {}", view.work.ready.len()),
        format!("Current work:  {current_work}"),
        format!("Current missions: {}", view.current_mission_count),
    ];

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
            panel
        }
        Err(error) => {
            MetadataPanel::new("Local State").row("Checkout", format!("unavailable - {error}"))
        }
    };

    let ready = view.work.ready.len();
    let next_actions = if ready > 0 {
        vec![
            format!("  Choose ready work ({ready} ready issue(s) available): atelier work ready"),
            "  Inspect selected work transitions: atelier issue transition <issue-id>".to_string(),
        ]
    } else if !view.work.blocked.is_empty() {
        vec![
            "  Inspect blocked work (no ready work is available): atelier work blocked".to_string(),
        ]
    } else {
        vec!["  No specific next action is available from checkout state.".to_string()]
    };

    Page::new("Atelier Status")
        .panel(LinesPanel::new("", summary_lines))
        .panel(IssueListPanel::new("Current Work", active_rows).limit(8))
        .panel(local_state)
        .panel(LinesPanel::new("Next Actions", next_actions))
        .print(RenderContext::for_stdout());
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
