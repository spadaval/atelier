use anyhow::{bail, Context, Result};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::db::Database;
use crate::models::WorkAssociation;

#[derive(Debug)]
struct WorktreeStatus {
    path: String,
    branch: Option<String>,
    head: Option<String>,
    dirty: bool,
    dirty_paths: Vec<String>,
    ahead: Option<i64>,
    behind: Option<i64>,
    unpushed_commits: Option<i64>,
    associated_work: Vec<WorkAssociation>,
    export_fresh: Option<bool>,
    export_errors: Vec<String>,
}

fn start_work_association(db: &Database, id: &str) -> Result<()> {
    let issue = db.require_issue(id)?;
    let branch = current_branch().ok();
    let path = env::current_dir()?.to_string_lossy().to_string();
    db.start_work_association(id, branch.as_deref(), Some(&path))?;
    crate::commands::activity_log::record_work_started(id, branch.as_deref(), Some(&path))?;
    ensure_session_work(db, id)?;
    println!("Started work on {} {}", issue.id, issue.title);
    if let Some(branch) = branch {
        println!("Branch: {branch}");
    }
    println!("Worktree: {path}");
    println!();
    println!("Next Commands");
    println!("-------------");
    println!("  Inspect checkout status: atelier status");
    if let Some(mission_id) = containing_mission(db, id)? {
        println!("  Inspect mission selection and blockers: atelier mission status {mission_id}");
    }
    println!("  Inspect work transitions: atelier issue transition {id} --options");
    Ok(())
}

pub fn start_lifecycle(state_dir: &Path, db_path: &Path, id: &str) -> Result<()> {
    let db = Database::open(db_path)?;
    db.require_issue(id)?;
    print_active_mission_context(&db, id)?;
    ensure_clean_worktree()?;
    if let Some(active) = db.get_active_work_association()? {
        if active.issue_id != id {
            bail!(
                "Worktree already has active issue {}. Use `atelier abandon {} --reason \"...\"` before starting {}.",
                active.issue_id,
                active.issue_id,
                id
            );
        }
        return start_work_association(&db, id);
    }
    crate::commands::workflow::transition_issue(&db, state_dir, db_path, id, "start", None)?;
    drop(db);

    let db = Database::open(db_path)?;
    start_work_association(&db, id)
}

pub fn abandon(db: &Database, id: &str, reason: &str) -> Result<()> {
    let issue = db.require_issue(id)?;
    let abandoned = db.abandon_work_association(id)?;
    if abandoned {
        crate::commands::activity_log::record_work_abandoned(id, reason)?;
        println!("Abandoned work on {} {}", issue.id, issue.title);
        println!("Reason:   {reason}");
    } else {
        println!("No active work association for {}", issue.id);
    }
    Ok(())
}

pub fn repair_active(db: &Database, id: Option<&str>) -> Result<()> {
    let work = match id {
        Some(id) => db
            .get_work_association(id)?
            .filter(|work| work.status == "active"),
        None => db.get_active_work_association()?,
    };
    let Some(work) = work else {
        if let Some(id) = id {
            println!("No active work association for {id}");
        } else {
            println!("No active work association to repair.");
        }
        return Ok(());
    };
    db.require_issue(&work.issue_id)?;
    match work.worktree_path.as_deref() {
        Some(path) if Path::new(path).exists() => {
            bail!(
                "Active work path still exists for {}: {path}. Use `atelier abandon {} --reason \"...\"` to switch away, or inspect with `atelier status`.",
                work.issue_id,
                work.issue_id
            );
        }
        Some(path) => {
            db.remove_work_association(&work.issue_id)?;
            println!(
                "Cleared stale active work association for {}: {path}",
                work.issue_id
            );
        }
        None => {
            db.remove_work_association(&work.issue_id)?;
            println!(
                "Cleared active work association with missing worktree path for {}",
                work.issue_id
            );
        }
    }
    println!("Next Commands");
    println!("-------------");
    println!("  Inspect checkout status: atelier status");
    println!("  Inspect worktrees: atelier worktree status");
    Ok(())
}

pub fn finish_active_association(db: &Database, id: &str) -> Result<bool> {
    let finished = db.finish_work_association(id)?;
    if finished {
        crate::commands::activity_log::record_work_finished(id)?;
    }
    Ok(finished)
}

pub fn worktree_status(db: &Database) -> Result<()> {
    let statuses = worktree_statuses(db)?;
    if statuses.is_empty() {
        print_heading("Worktree Status");
        println!("No Git worktrees found.");
        return Ok(());
    }
    println!("Worktree Status");
    println!("===============");
    println!("{} total", statuses.len());
    for status in statuses {
        let dirty = if status.dirty { "dirty" } else { "clean" };
        let branch = status.branch.as_deref().unwrap_or("(detached)");
        print_heading(&status.path);
        println!("Branch:   {branch}");
        println!("State:    {dirty}");
        println!(
            "Head:     {}",
            status.head.as_deref().unwrap_or("(unknown)")
        );
        println!(
            "Ahead:    {}",
            status
                .ahead
                .map(|value| value.to_string())
                .unwrap_or_else(|| "(unknown)".to_string())
        );
        println!(
            "Behind:   {}",
            status
                .behind
                .map(|value| value.to_string())
                .unwrap_or_else(|| "(unknown)".to_string())
        );
        println!(
            "Unpushed: {}",
            status
                .unpushed_commits
                .map(|value| value.to_string())
                .unwrap_or_else(|| "(unknown)".to_string())
        );
        if !status.dirty_paths.is_empty() {
            print_heading("Dirty Paths");
            for path in status.dirty_paths {
                println!("  {path}");
            }
        }
        print_heading("Associated Work");
        if status.associated_work.is_empty() {
            println!("  (none)");
        }
        for work in status.associated_work {
            let mission = active_mission_context(db, &work.issue_id)?;
            println!("  {} [{}]", work.issue_id, work.status);
            if let Some((mission_id, advances)) = mission {
                println!(
                    "    Mission:  {} ({})",
                    mission_id,
                    if advances {
                        "advances"
                    } else {
                        "outside focus"
                    }
                );
            }
            println!(
                "    Branch:   {}",
                work.branch.as_deref().unwrap_or("(none)")
            );
            println!("    Started:  {}", work.started_at.to_rfc3339());
        }
        if let Some(false) = status.export_fresh {
            print_heading("Export");
            println!("  State: stale");
            if status.export_errors.is_empty() {
                println!("  Errors: (none)");
            } else {
                println!("  Errors:");
                for error in status.export_errors {
                    println!("    {error}");
                }
            }
        } else if let Some(true) = status.export_fresh {
            print_heading("Export");
            println!("  State: current");
        }
    }
    Ok(())
}

fn print_heading(title: &str) {
    println!("{title}");
    println!("{}", "-".repeat(title.len()));
}

fn print_active_mission_context(db: &Database, issue_id: &str) -> Result<()> {
    let Some((mission_id, advances)) = active_mission_context(db, issue_id)? else {
        return Ok(());
    };
    if advances {
        println!("Mission: {mission_id} (active)");
    } else {
        println!(
            "Warning: {issue_id} is outside active mission {mission_id}; non-mission work remains allowed."
        );
    }
    Ok(())
}

fn active_mission_context(db: &Database, issue_id: &str) -> Result<Option<(String, bool)>> {
    let Some(mission) = crate::commands::mission::active_mission(db)? else {
        return Ok(None);
    };
    let advances = crate::commands::mission::issue_advances_mission(db, &mission.id, issue_id)?;
    Ok(Some((mission.id, advances)))
}

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

pub fn worktree_for(db: &Database, id: &str, path: Option<&str>) -> Result<()> {
    let issue = db.require_issue(id)?;
    print_active_mission_context(db, id)?;
    let root = repo_root()?;
    let branch = format!("codex/{}", id);
    let worktree_path = path
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join(".atelier-worktrees").join(id));
    let worktree_path_string = worktree_path.to_string_lossy().to_string();
    if !worktree_path.exists() {
        let output = Command::new("git")
            .current_dir(&root)
            .args(["worktree", "add", "-B", &branch])
            .arg(&worktree_path)
            .arg("HEAD")
            .output()
            .context("failed to run git worktree add")?;
        if !output.status.success() {
            bail!(
                "git worktree add failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
    } else if db
        .get_work_association(id)?
        .and_then(|work| work.worktree_path)
        .as_deref()
        != Some(worktree_path_string.as_str())
    {
        recover_incomplete_worktree_setup(&root, &worktree_path)?;
    }
    ensure_git_worktree(&worktree_path)?;
    let layout = crate::storage_layout::StorageLayout::new(&worktree_path);
    let state_dir = layout.canonical_dir();
    if state_dir.is_dir() {
        std::fs::create_dir_all(layout.target_runtime_dir())
            .context("worktree setup failed while creating runtime directory")?;
        let exe = env::current_exe().context("failed to locate current atelier executable")?;
        let status = Command::new(exe)
            .current_dir(&worktree_path)
            .arg("rebuild")
            .status()
            .context("worktree setup failed while rebuilding runtime projection")?;
        if !status.success() {
            bail!(
                "worktree setup failed while rebuilding runtime projection; active work association was not changed"
            );
        }
    } else {
        bail!(
            "worktree setup failed because {} does not contain .atelier; active work association was not changed",
            worktree_path.display()
        );
    }
    start_worktree_runtime_association(&layout, id, &branch, &worktree_path_string)?;
    db.start_work_association(id, Some(&branch), Some(&worktree_path_string))?;
    crate::commands::activity_log::record_work_started(
        id,
        Some(&branch),
        Some(&worktree_path_string),
    )?;
    let _ = issue;
    println!("{}", worktree_path.display());
    Ok(())
}

fn recover_incomplete_worktree_setup(root: &Path, worktree_path: &Path) -> Result<()> {
    ensure_clean_path(worktree_path)?;
    let output = Command::new("git")
        .current_dir(root)
        .args(["rev-parse", "HEAD"])
        .output()
        .context("failed to read root HEAD for worktree setup retry")?;
    if !output.status.success() {
        bail!("failed to read root HEAD for worktree setup retry");
    }
    let head = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let status = Command::new("git")
        .current_dir(worktree_path)
        .args(["reset", "--hard", &head])
        .status()
        .context("failed to reset incomplete worktree setup")?;
    if !status.success() {
        bail!("failed to reset incomplete worktree setup");
    }
    Ok(())
}

fn ensure_clean_path(path: &Path) -> Result<()> {
    let dirty = dirty_paths(path)?;
    if !dirty.is_empty() {
        bail!(
            "worktree setup retry found uncommitted changes in {}; inspect or clean them before retrying:\n{}",
            path.display(),
            dirty.join("\n")
        );
    }
    Ok(())
}

fn ensure_git_worktree(path: &Path) -> Result<()> {
    let output = Command::new("git")
        .current_dir(path)
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .with_context(|| format!("failed to inspect worktree path {}", path.display()))?;
    if !output.status.success() || String::from_utf8_lossy(&output.stdout).trim() != "true" {
        bail!(
            "worktree setup failed because {} is not a Git worktree; active work association was not changed",
            path.display()
        );
    }
    Ok(())
}

fn start_worktree_runtime_association(
    layout: &crate::storage_layout::StorageLayout,
    id: &str,
    branch: &str,
    worktree_path: &str,
) -> Result<()> {
    let worktree_db = Database::open(&layout.runtime_db_path())
        .context("worktree setup failed while opening worktree runtime projection")?;
    worktree_db.require_issue(id).context(
        "worktree setup failed because the issue is missing from the worktree runtime projection",
    )?;
    ensure_session_work(&worktree_db, id)
        .context("worktree setup failed while associating the worktree session")?;
    worktree_db
        .start_work_association(id, Some(branch), Some(worktree_path))
        .context("worktree setup failed while recording worktree active work association")?;
    Ok(())
}

pub fn worktree_merge(db: &Database, id: &str) -> Result<()> {
    let work = db
        .get_work_association(id)?
        .ok_or_else(|| anyhow::anyhow!("No worktree association for {id}"))?;
    let branch = work
        .branch
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("No branch association for {id}"))?;
    let root = repo_root()?;
    let status = Command::new("git")
        .current_dir(&root)
        .args(["merge", "--no-ff", branch])
        .status()
        .context("failed to run git merge")?;
    if !status.success() {
        bail!("git merge failed; resolve conflicts with Git, then rerun validation");
    }
    println!("Merged {branch} for {id}");
    Ok(())
}

pub fn worktree_remove(db: &Database, id: &str, force: bool) -> Result<()> {
    let work = db
        .get_work_association(id)?
        .ok_or_else(|| anyhow::anyhow!("No worktree association for {id}"))?;
    let path = work
        .worktree_path
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("No worktree path association for {id}"))?;
    let root = repo_root()?;
    let mut command = Command::new("git");
    command.current_dir(&root).args(["worktree", "remove"]);
    if force {
        command.arg("--force");
    }
    let status = command
        .arg(path)
        .status()
        .context("failed to remove worktree")?;
    if !status.success() {
        bail!("git worktree remove failed; inspect with `git worktree list`");
    }
    db.remove_work_association(id)?;
    println!("Removed worktree {path}");
    Ok(())
}

pub fn worktree_repair(db: &Database, id: &str) -> Result<()> {
    let work = db
        .get_work_association(id)?
        .ok_or_else(|| anyhow::anyhow!("No worktree association for {id}"))?;
    let Some(path) = work.worktree_path.as_deref() else {
        bail!("No worktree path association for {id}");
    };
    if Path::new(path).exists() {
        bail!(
            "Worktree path still exists for {id}: {path}. Use `atelier worktree remove {id}` or inspect with `atelier worktree status`."
        );
    }
    db.remove_work_association(id)?;
    println!("Cleared stale worktree association for {id}: {path}");
    Ok(())
}

fn worktree_statuses(db: &Database) -> Result<Vec<WorktreeStatus>> {
    let worktrees = git_worktrees()?;
    let associations = db.list_work_associations()?;
    let mut statuses = Vec::new();
    for mut worktree in worktrees {
        let dirty_paths = dirty_paths(&worktree.path)?;
        let (ahead, behind) = ahead_behind(&worktree.path).unwrap_or((None, None));
        let export_errors = export_errors(&worktree.path);
        let state_dir = crate::storage_layout::StorageLayout::new(&worktree.path).canonical_dir();
        let export_fresh = if state_dir.is_dir() {
            Some(export_errors.is_empty())
        } else {
            None
        };
        let path_string = worktree.path.to_string_lossy().to_string();
        let associated_work = associations
            .iter()
            .filter(|work| work.worktree_path.as_deref() == Some(path_string.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        statuses.push(WorktreeStatus {
            path: path_string,
            branch: worktree.branch.take(),
            head: worktree.head.take(),
            dirty: !dirty_paths.is_empty(),
            dirty_paths,
            ahead,
            behind,
            unpushed_commits: ahead,
            associated_work,
            export_fresh,
            export_errors,
        });
    }
    Ok(statuses)
}

#[derive(Debug)]
struct GitWorktree {
    path: PathBuf,
    head: Option<String>,
    branch: Option<String>,
}

fn git_worktrees() -> Result<Vec<GitWorktree>> {
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .output()
        .context("failed to list git worktrees")?;
    if !output.status.success() {
        bail!("git worktree list failed");
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut worktrees = Vec::new();
    let mut current: Option<GitWorktree> = None;
    for line in text.lines() {
        if line.is_empty() {
            if let Some(worktree) = current.take() {
                worktrees.push(worktree);
            }
            continue;
        }
        if let Some(path) = line.strip_prefix("worktree ") {
            if let Some(worktree) = current.replace(GitWorktree {
                path: PathBuf::from(path),
                head: None,
                branch: None,
            }) {
                worktrees.push(worktree);
            }
        } else if let Some(head) = line.strip_prefix("HEAD ") {
            if let Some(worktree) = current.as_mut() {
                worktree.head = Some(head.to_string());
            }
        } else if let Some(branch) = line.strip_prefix("branch ") {
            if let Some(worktree) = current.as_mut() {
                worktree.branch = Some(branch.trim_start_matches("refs/heads/").to_string());
            }
        }
    }
    if let Some(worktree) = current {
        worktrees.push(worktree);
    }
    Ok(worktrees)
}

fn dirty_paths(path: &Path) -> Result<Vec<String>> {
    let output = Command::new("git")
        .current_dir(path)
        .args(["status", "--porcelain"])
        .output()
        .context("failed to run git status")?;
    if !output.status.success() {
        bail!("git status failed for {}", path.display());
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(git_status_path)
        .filter(|path| !is_workflow_generated_dirty_path(path))
        .collect())
}

fn ahead_behind(path: &Path) -> Result<(Option<i64>, Option<i64>)> {
    let output = Command::new("git")
        .current_dir(path)
        .args(["rev-list", "--left-right", "--count", "HEAD...@{u}"])
        .output()
        .context("failed to calculate ahead/behind")?;
    if !output.status.success() {
        return Ok((None, None));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut parts = text.split_whitespace();
    let ahead = parts.next().and_then(|value| value.parse::<i64>().ok());
    let behind = parts.next().and_then(|value| value.parse::<i64>().ok());
    Ok((ahead, behind))
}

fn export_errors(path: &Path) -> Vec<String> {
    let Ok(exe) = env::current_exe() else {
        return vec!["failed to locate atelier executable".to_string()];
    };
    let Ok(output) = Command::new(exe)
        .current_dir(path)
        .args(["export", "--check"])
        .output()
    else {
        return vec!["failed to run atelier export --check".to_string()];
    };
    if output.status.success() {
        Vec::new()
    } else {
        String::from_utf8_lossy(&output.stderr)
            .lines()
            .map(str::to_string)
            .collect()
    }
}

fn ensure_session_work(db: &Database, id: &str) -> Result<()> {
    let session_id = match db.get_current_session()? {
        Some(session) => session.id,
        None => db.start_session_with_agent(None)?,
    };
    db.set_session_issue(session_id, id)?;
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
            "Worktree has uncommitted changes; commit or stash them before this workflow action:\n{}",
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
