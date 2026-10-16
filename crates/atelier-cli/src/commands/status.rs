use anyhow::Result;
use atelier_app::record_index::RecordIndex;

use crate::commands;
use crate::human_output::{
    print_page, render_lines_section, render_metadata, IssueListPanel, IssueListRow, RenderContext,
};
use atelier_app::work_classification::StatusView;

pub fn run(index: &RecordIndex, quiet: bool) -> Result<()> {
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let view = atelier_app::work_classification::status_view(index, workflow_policy.as_ref())?;

    if quiet {
        println!(
            "work={} current_missions={} ready={} human={}",
            if view.work.active.is_empty() {
                "none".to_string()
            } else {
                view.work.active.len().to_string()
            },
            view.current_mission_count,
            view.work.ready.len(),
            view.work.human.len(),
        );
        return Ok(());
    }

    let layout = atelier_app::storage_layout::StorageLayout::discover()?;
    // Local State already reports an unavailable Git checkout.
    let owner = workflow_policy
        .as_ref()
        .and_then(|policy| owner_row(layout.repo_root(), index, policy).ok().flatten());
    print_status_view(&view, owner);
    let lines =
        crate::commands::git::branch_lines(layout.repo_root(), &layout.canonical_dir(), index)
            .unwrap_or_else(|error| vec![format!("Branch orientation unavailable: {error}")]);
    if !lines.is_empty() {
        print_page(
            "Other local work branches",
            [render_lines_section("", lines)],
            RenderContext::for_stdout(),
        );
    }
    Ok(())
}

/// The current branch's owner, with done claims marked as not yet merged.
fn owner_row(
    repo: &std::path::Path,
    index: &RecordIndex,
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
) -> Result<Option<String>> {
    let Some(branch) = commands::git::current_branch(repo)? else {
        return Ok(None);
    };
    let Some(owner) = index.issues().find(|issue| {
        issue
            .workflow_branch
            .as_ref()
            .is_some_and(|owner| owner.work_branch == branch)
    }) else {
        return Ok(None);
    };
    let status = if policy.status_category(&owner.status) == Some("done") {
        commands::git::unmerged_done_label(
            repo,
            &owner.id,
            &owner.status,
            &branch,
            &policy.branch_policy.base_branch,
        )
    } else {
        owner.status.clone()
    };
    Ok(Some(format!("{} [{status}] {}", owner.id, owner.title)))
}

fn print_status_view(view: &StatusView, owner: Option<String>) {
    let current_work = if view.work.active.is_empty() {
        "none".to_string()
    } else {
        format!("{} issue(s)", view.work.active.len())
    };
    let summary_lines = vec![
        format!("Ready work:    {}", view.work.ready.len()),
        format!("Needs a human: {}", view.work.human.len()),
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
            let mut rows = Vec::new();
            if let Some(branch) = state.branch {
                rows.push(("Branch", branch));
            }
            if let Some(owner) = owner {
                rows.push(("Owner", owner));
            }
            if state.dirty_entries.is_empty() {
                rows.push(("Checkout", "clean".to_string()));
            } else {
                rows.push((
                    "Checkout",
                    format!(
                        "dirty ({})",
                        crate::human_output::path_summary(&state.dirty_entries, 3)
                    ),
                ));
            }
            rows
        }
        Err(error) => {
            vec![("Checkout", format!("unavailable - {error}"))]
        }
    };

    let ready = view.work.ready.len();
    let next_actions = if ready > 0 {
        vec![
            format!("  Choose ready work ({ready} ready issue(s) available): atelier work ready"),
            "  Inspect selected work transitions: atelier issue transition <issue-id>".to_string(),
        ]
    } else if !view.work.human.is_empty() {
        vec![format!(
            "  Only work for the human is ready ({} item(s)): atelier work human",
            view.work.human.len()
        )]
    } else if !view.work.blocked.is_empty() {
        vec![
            "  Inspect blocked work (no ready work is available): atelier work blocked".to_string(),
        ]
    } else {
        vec!["  No specific next action is available from checkout state.".to_string()]
    };

    let context = RenderContext::for_stdout();
    print_page(
        "Atelier Status",
        [
            render_lines_section("", summary_lines),
            IssueListPanel::new("Current Work", active_rows)
                .limit(8)
                .render(context),
            render_metadata(Some("Local State"), &local_state),
            render_lines_section("Next Actions", next_actions),
        ],
        context,
    );
}

struct GitState {
    branch: Option<String>,
    dirty_entries: Vec<String>,
}

fn git_state() -> Result<GitState> {
    let output = atelier_app::git_freshness::command(&std::env::current_dir()?)
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
