use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
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

#[derive(Debug, Deserialize)]
struct WorkflowConfig {
    #[serde(default)]
    hooks: BTreeMap<String, WorkflowHook>,
}

#[derive(Debug, Deserialize)]
struct WorkflowHook {
    event: String,
    command: HookCommand,
}

#[derive(Debug, Deserialize)]
struct HookCommand {
    argv: Vec<String>,
    #[serde(default)]
    env: BTreeMap<String, String>,
}

pub fn start(db: &Database, id: &str) -> Result<()> {
    let issue = db.require_issue(id)?;
    ensure_clean_worktree()?;
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
    Ok(())
}

pub fn finish(db: &Database, id: &str) -> Result<()> {
    let issue = db.require_issue(id)?;
    ensure_clean_worktree()?;
    let export_stale =
        crate::commands::export::canonical_stale_entries(db, &repo_root()?.join(".atelier-state"))?;
    if !export_stale.is_empty() {
        bail!("Canonical export is stale:\n{}", export_stale.join("\n"));
    }
    let finished = db.finish_work_association(id)?;
    if finished {
        crate::commands::activity_log::record_work_finished(id)?;
    }
    if finished {
        println!("Finished work on {} {}", issue.id, issue.title);
    } else {
        println!("No active work association for {}", issue.id);
    }
    Ok(())
}

pub fn status(db: &Database) -> Result<()> {
    let active = db.get_active_work_association()?;
    match active {
        Some(work) => {
            let title = db
                .get_issue(&work.issue_id)?
                .map(|issue| issue.title)
                .unwrap_or_else(|| "(issue missing)".to_string());
            println!("Work Status");
            println!("===========");
            println!("Active:   yes");
            println!("Issue:    {} - {}", work.issue_id, title);
            println!("Status:   {}", work.status);
            println!("Branch:   {}", work.branch.as_deref().unwrap_or("(none)"));
            println!(
                "Worktree: {}",
                work.worktree_path.as_deref().unwrap_or("(none)")
            );
            println!("Started:  {}", work.started_at.to_rfc3339());
            println!(
                "Finished: {}",
                work.finished_at
                    .map(|finished_at| finished_at.to_rfc3339())
                    .unwrap_or_else(|| "(none)".to_string())
            );
        }
        None => {
            println!("Work Status");
            println!("===========");
            println!("Active: no");
            print_heading("Next Commands");
            println!("  atelier issue ready");
            println!("  atelier work start <issue-id>");
        }
    }
    Ok(())
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
            println!("  {} [{}]", work.issue_id, work.status);
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

pub fn worktree_for(db: &Database, id: &str, path: Option<&str>) -> Result<()> {
    let issue = db.require_issue(id)?;
    let root = repo_root()?;
    let branch = format!("codex/{}", id);
    let worktree_path = path
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join(".atelier-worktrees").join(id));
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
    }
    let state_dir = worktree_path.join(".atelier-state");
    if state_dir.is_dir() {
        let exe = env::current_exe().context("failed to locate current atelier executable")?;
        let _ = Command::new(exe)
            .current_dir(&worktree_path)
            .arg("rebuild")
            .status();
    }
    run_worktree_setup_hooks(&root, &worktree_path, id, &branch)?;
    db.start_work_association(id, Some(&branch), Some(&worktree_path.to_string_lossy()))?;
    crate::commands::activity_log::record_work_started(
        id,
        Some(&branch),
        Some(&worktree_path.to_string_lossy()),
    )?;
    let _ = issue;
    println!("{}", worktree_path.display());
    Ok(())
}

fn run_worktree_setup_hooks(
    root: &Path,
    worktree_path: &Path,
    issue_id: &str,
    branch: &str,
) -> Result<()> {
    let config_path = root.join("atelier.workflow.yaml");
    if !config_path.is_file() {
        return Ok(());
    }
    let text = std::fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read {}", config_path.display()))?;
    let config: WorkflowConfig = serde_yaml::from_str(&text)
        .with_context(|| format!("failed to parse {}", config_path.display()))?;
    for (id, hook) in config.hooks {
        if hook.event != "worktree_setup" {
            continue;
        }
        if hook.command.argv.is_empty() {
            bail!("worktree_setup hook {id} has empty command argv");
        }
        let mut command = Command::new(&hook.command.argv[0]);
        command
            .current_dir(worktree_path)
            .args(&hook.command.argv[1..])
            .env("ATELIER_WORKTREE_PATH", worktree_path)
            .env("ATELIER_WORK_ISSUE_ID", issue_id)
            .env("ATELIER_WORK_BRANCH", branch);
        for (key, value) in hook.command.env {
            command.env(key, value);
        }
        let output = command
            .output()
            .with_context(|| format!("failed to run worktree_setup hook {id}"))?;
        if !output.status.success() {
            bail!("worktree_setup hook {id} failed");
        }
    }
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

fn worktree_statuses(db: &Database) -> Result<Vec<WorktreeStatus>> {
    let worktrees = git_worktrees()?;
    let associations = db.list_work_associations()?;
    let mut statuses = Vec::new();
    for mut worktree in worktrees {
        let dirty_paths = dirty_paths(&worktree.path)?;
        let (ahead, behind) = ahead_behind(&worktree.path).unwrap_or((None, None));
        let export_errors = export_errors(&worktree.path);
        let export_fresh = if worktree.path.join(".atelier-state").is_dir() {
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
        || (path.starts_with(".atelier-state/issues/")
            && (path.ends_with(".activity/")
                || (path.contains(".activity/") && path.ends_with(".md"))))
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
