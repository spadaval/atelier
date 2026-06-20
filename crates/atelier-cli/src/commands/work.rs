use anyhow::{bail, Context, Result};
use std::collections::BTreeSet;
use std::env;
use std::path::Path;
use std::process::Command;

use atelier_core::Issue;
use atelier_sqlite::Database;

fn containing_mission(db: &Database, issue_id: &str) -> Result<Option<String>> {
    for mission in db.list_records("mission", None)? {
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
    for mission in db.list_records("mission", None)? {
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
