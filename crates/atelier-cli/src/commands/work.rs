use anyhow::{bail, Context, Result};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use atelier_core::Issue;
use atelier_sqlite::Database;

const MISSION_WORKTREE_OWNER_FILE: &str = "mission-worktree-owner";

#[derive(Debug)]
struct WorktreeStatus {
    path: String,
    branch: Option<String>,
    head: Option<String>,
    mission_id: Option<String>,
    dirty: bool,
    dirty_paths: Vec<String>,
    ahead: Option<i64>,
    behind: Option<i64>,
    unpushed_commits: Option<i64>,
    associated_work: Vec<Issue>,
    export_fresh: Option<bool>,
    export_errors: Vec<String>,
}

fn print_started_work(db: &Database, id: &str) -> Result<()> {
    let issue = db.require_issue(id)?;
    let branch = current_branch().ok();
    let path = env::current_dir()?.to_string_lossy().to_string();
    crate::commands::activity_log::record_work_started(id, branch.as_deref(), Some(&path))?;
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
    crate::commands::workflow::transition_issue(&db, state_dir, db_path, id, "start", None)?;
    drop(db);

    let db = Database::open(db_path)?;
    print_started_work(&db, id)
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
        if let Some(mission_id) = status.mission_id.as_deref() {
            println!("Mission:  {mission_id}");
        }
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
            let mission = active_mission_context(db, &work.id)?;
            println!("  {} [{}]", work.id, work.status);
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
                status.branch.as_deref().unwrap_or("(none)")
            );
            println!("    Updated:  {}", work.updated_at.to_rfc3339());
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

fn issue_branch_name(root: &Path, issue: &Issue) -> Result<String> {
    let policy = atelier_app::workflow_policy::load(root)?;
    policy.branch_name_for_owner(
        issue,
        &atelier_app::workflow_policy::BranchOwnerKind::StandaloneIssue,
    )
}

fn epic_branch_name(epic: &Issue) -> Result<String> {
    let root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&root)?;
    policy.branch_name_for_owner(epic, &atelier_app::workflow_policy::BranchOwnerKind::Epic)
}

pub fn worktree_for(db: &Database, id: &str, path: Option<&str>) -> Result<()> {
    let issue = db.require_issue(id)?;
    print_active_mission_context(db, id)?;
    ensure_issue_active_for_worktree(db, id)?;
    let root = repo_root()?;
    let branch = issue_branch_name(&root, &issue)?;
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
    } else if !worktree_has_issue_branch(&worktree_path, &branch)? {
        recover_incomplete_worktree_setup(&root, &worktree_path)?;
    }
    ensure_git_worktree(&worktree_path)?;
    let layout = atelier_app::storage_layout::StorageLayout::new(&worktree_path);
    let state_dir = layout.canonical_dir();
    if state_dir.is_dir() {
        activate_worktree_issue(&worktree_path, id)?;
        std::fs::create_dir_all(layout.target_runtime_dir())
            .context("worktree setup failed while creating runtime directory")?;
        let exe = env::current_exe().context("failed to locate current atelier executable")?;
        let status = Command::new(exe)
            .current_dir(&worktree_path)
            .arg("rebuild")
            .status()
            .context("worktree setup failed while rebuilding runtime projection")?;
        if !status.success() {
            bail!("worktree setup failed while rebuilding runtime projection");
        }
    } else {
        bail!(
            "worktree setup failed because {} does not contain .atelier",
            worktree_path.display()
        );
    }
    validate_worktree_projection(&layout, id)?;
    crate::commands::activity_log::record_work_started(
        id,
        Some(&branch),
        Some(&worktree_path_string),
    )?;
    let _ = issue;
    println!("{}", worktree_path.display());
    Ok(())
}

fn activate_worktree_issue(worktree_path: &Path, id: &str) -> Result<()> {
    let exe = env::current_exe().context("failed to locate current atelier executable")?;
    let output = Command::new(exe)
        .current_dir(worktree_path)
        .args(["issue", "transition", id, "start"])
        .output()
        .context("worktree setup failed while activating issue in worktree")?;
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.contains("Unknown transition 'start'")
        || stderr.contains("available from 'in_progress'")
    {
        return Ok(());
    }
    bail!(
        "worktree setup failed while activating issue in worktree: {}",
        stderr.trim()
    );
}

fn ensure_issue_active_for_worktree(db: &Database, id: &str) -> Result<()> {
    let issue = db.require_issue(id)?;
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    if crate::commands::issue_workflow::issue_status_category(
        workflow_policy.as_ref(),
        &issue.status,
    )
    .as_deref()
        == Some("active")
    {
        return Ok(());
    }
    let root = repo_root()?;
    let layout = atelier_app::storage_layout::StorageLayout::new(&root);
    crate::commands::workflow::transition_issue(
        db,
        &layout.canonical_dir(),
        &layout.runtime_db_path(),
        id,
        "start",
        None,
    )
}

pub fn worktree_for_mission(db: &Database, mission_id: &str, path: Option<&str>) -> Result<()> {
    let mission = db.require_record("mission", mission_id)?;
    let root = repo_root()?;
    let branch = format!("mission/{}", mission.id);
    let worktree_path = path
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join(".atelier-worktrees").join(&mission.id));
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
    ensure_git_worktree(&worktree_path)?;
    let layout = atelier_app::storage_layout::StorageLayout::new(&worktree_path);
    if !layout.canonical_dir().is_dir() {
        bail!(
            "mission worktree setup failed because {} does not contain .atelier",
            worktree_path.display()
        );
    }
    std::fs::create_dir_all(layout.target_runtime_dir())
        .context("mission worktree setup failed while creating runtime directory")?;
    let exe = env::current_exe().context("failed to locate current atelier executable")?;
    let status = Command::new(exe)
        .current_dir(&worktree_path)
        .arg("rebuild")
        .status()
        .context("mission worktree setup failed while rebuilding runtime projection")?;
    if !status.success() {
        bail!("mission worktree setup failed while rebuilding runtime projection");
    }
    ensure_mission_worktree_owner(&layout, &mission.id)?;
    println!("{}", worktree_path.display());
    println!("Mission: {}", mission.id);
    println!("Branch: {branch}");
    Ok(())
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
    let worktree_path = require_mission_worktree_owner(&mission_id, "atelier branch for-epic")?;
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
    println!("Switched to {branch}");
    println!("Epic: {} {}", epic.id, epic.title);
    println!("Mission: {mission_id}");
    println!("Worktree: {}", worktree_path.display());
    Ok(())
}

pub fn branch_status(db: &Database) -> Result<()> {
    let mission_id = current_mission_worktree_owner("atelier branch status")?;
    let worktree_path = env::current_dir().context("failed to read current checkout path")?;
    let current = current_branch().unwrap_or_else(|_| "(detached)".to_string());
    let root = repo_root()?;
    let policy = atelier_app::workflow_policy::load(&root)?;
    println!("Epic Branch Status");
    println!("==================");
    println!("Mission: {mission_id}");
    println!("Worktree: {}", worktree_path.display());
    println!("Current: {current}");
    for issue in db.list_issues(None, None, None)? {
        if issue.issue_type != "epic" {
            continue;
        }
        if !crate::commands::mission::issue_advances_mission(db, &mission_id, &issue.id)? {
            continue;
        }
        let branch = policy
            .branch_name_for_owner(&issue, &atelier_app::workflow_policy::BranchOwnerKind::Epic)?;
        if branch_exists(&branch)? {
            println!("  {branch} - {} [{}]", issue.title, issue.status);
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
    let worktree_path = require_mission_worktree_owner(&mission_id, "atelier branch merge")?;
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
    println!("Merged {branch}");
    println!("Mission: {mission_id}");
    println!("Worktree: {}", worktree_path.display());
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
            "worktree setup failed because {} is not a Git worktree",
            path.display()
        );
    }
    Ok(())
}

fn validate_worktree_projection(
    layout: &atelier_app::storage_layout::StorageLayout,
    id: &str,
) -> Result<()> {
    let worktree_db = Database::open(&layout.runtime_db_path())
        .context("worktree setup failed while opening worktree runtime projection")?;
    worktree_db.require_issue(id).context(
        "worktree setup failed because the issue is missing from the worktree runtime projection",
    )?;
    Ok(())
}

pub fn worktree_merge(db: &Database, id: &str) -> Result<()> {
    let issue = db.require_issue(id)?;
    let root = repo_root()?;
    let branch = issue_branch_name(&root, &issue)?;
    if !branch_exists(&branch)? {
        bail!("No worktree branch found for {id}: {branch}");
    }
    let root = repo_root()?;
    let status = Command::new("git")
        .current_dir(&root)
        .args(["merge", "--no-ff", &branch])
        .status()
        .context("failed to run git merge")?;
    if !status.success() {
        bail!("git merge failed; resolve conflicts with Git, then rerun validation");
    }
    println!("Merged {branch} for {id}");
    Ok(())
}

pub fn worktree_remove(db: &Database, id: &str, force: bool) -> Result<()> {
    db.require_issue(id)?;
    let path = worktree_path_for_issue(db, id)?
        .ok_or_else(|| anyhow::anyhow!("No git worktree found for {id}"))?;
    let path_arg = path.to_string_lossy().to_string();
    let root = repo_root()?;
    let mut command = Command::new("git");
    command.current_dir(&root).args(["worktree", "remove"]);
    if force {
        command.arg("--force");
    }
    let status = command
        .arg(&path)
        .status()
        .context("failed to remove worktree")?;
    if !status.success() {
        bail!("git worktree remove failed; inspect with `git worktree list`");
    }
    println!("Removed worktree {path_arg}");
    Ok(())
}

pub fn worktree_repair(db: &Database, id: &str) -> Result<()> {
    db.require_issue(id)?;
    let Some(path) = expected_or_actual_worktree_path(db, id)? else {
        bail!("No worktree path found for {id}");
    };
    if path.exists() {
        bail!(
            "Worktree path still exists for {id}: {}. Use `atelier worktree remove {id}` or inspect with `atelier worktree status`.",
            path.display()
        );
    }
    println!("Cleared stale worktree path for {id}: {}", path.display());
    Ok(())
}

fn worktree_statuses(db: &Database) -> Result<Vec<WorktreeStatus>> {
    let worktrees = git_worktrees()?;
    let mut statuses = Vec::new();
    for mut worktree in worktrees {
        let path_exists = worktree.path.exists();
        let dirty_paths = if path_exists {
            dirty_paths(&worktree.path)?
        } else {
            vec!["missing git worktree path; inspect `git worktree prune`".to_string()]
        };
        let (ahead, behind) = if path_exists {
            ahead_behind(&worktree.path).unwrap_or((None, None))
        } else {
            (None, None)
        };
        let export_errors = if path_exists {
            export_errors(&worktree.path)
        } else {
            Vec::new()
        };
        let state_dir =
            atelier_app::storage_layout::StorageLayout::new(&worktree.path).canonical_dir();
        let export_fresh = if path_exists && state_dir.is_dir() {
            Some(export_errors.is_empty())
        } else {
            None
        };
        let path_string = worktree.path.to_string_lossy().to_string();
        let associated_work = status_derived_work_for_branch(db, worktree.branch.as_deref())?;
        statuses.push(WorktreeStatus {
            path: path_string,
            branch: worktree.branch.take(),
            head: worktree.head.take(),
            mission_id: worktree_mission_id(db, &worktree.path)?,
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

fn status_derived_work_for_branch(db: &Database, branch: Option<&str>) -> Result<Vec<Issue>> {
    let Some(branch) = branch else {
        return Ok(Vec::new());
    };
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let root = repo_root()?;
    let branch_policy = atelier_app::workflow_policy::load(&root)?;
    let mut associated = Vec::new();
    for issue in db.list_issues(None, None, None)? {
        if crate::commands::issue_workflow::issue_status_category(
            workflow_policy.as_ref(),
            &issue.status,
        )
        .as_deref()
            != Some("active")
        {
            continue;
        }
        if branch_policy
            .resolve_branch_lifecycle(db, &issue.id)?
            .expected_branch
            == branch
        {
            associated.push(issue);
        }
    }
    Ok(associated)
}

fn worktree_has_issue_branch(path: &Path, expected: &str) -> Result<bool> {
    Ok(git_worktrees()?
        .into_iter()
        .any(|worktree| worktree.path == path && worktree.branch.as_deref() == Some(expected)))
}

fn worktree_path_for_issue(db: &Database, id: &str) -> Result<Option<PathBuf>> {
    let issue = db.require_issue(id)?;
    let root = repo_root()?;
    let expected = issue_branch_name(&root, &issue)?;
    Ok(git_worktrees()?
        .into_iter()
        .find(|worktree| worktree.branch.as_deref() == Some(expected.as_str()))
        .map(|worktree| worktree.path))
}

fn expected_or_actual_worktree_path(db: &Database, id: &str) -> Result<Option<PathBuf>> {
    if let Some(path) = worktree_path_for_issue(db, id)? {
        return Ok(Some(path));
    }
    Ok(Some(repo_root()?.join(".atelier-worktrees").join(id)))
}

fn worktree_mission_id(db: &Database, path: &Path) -> Result<Option<String>> {
    let Some(mission_id) = read_mission_worktree_owner(path)? else {
        return Ok(None);
    };
    if db.get_record("mission", &mission_id)?.is_some() {
        return Ok(Some(mission_id));
    }
    Ok(None)
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

fn mission_worktree_owner_path(layout: &atelier_app::storage_layout::StorageLayout) -> PathBuf {
    layout
        .target_runtime_dir()
        .join(MISSION_WORKTREE_OWNER_FILE)
}

fn ensure_mission_worktree_owner(
    layout: &atelier_app::storage_layout::StorageLayout,
    mission_id: &str,
) -> Result<()> {
    let owner_path = mission_worktree_owner_path(layout);
    if let Some(existing) = read_mission_worktree_owner(layout.repo_root())? {
        if existing != mission_id {
            bail!(
                "mission worktree {} is already associated with {}. Choose another path or reuse that mission workspace.",
                layout.repo_root().display(),
                existing
            );
        }
        return Ok(());
    }
    std::fs::write(&owner_path, format!("{mission_id}\n")).with_context(|| {
        format!(
            "failed to record mission worktree ownership at {}",
            owner_path.display()
        )
    })?;
    Ok(())
}

fn read_mission_worktree_owner(path: &Path) -> Result<Option<String>> {
    let owner_path =
        mission_worktree_owner_path(&atelier_app::storage_layout::StorageLayout::new(path));
    if !owner_path.is_file() {
        return Ok(None);
    }
    let owner = std::fs::read_to_string(&owner_path).with_context(|| {
        format!(
            "failed to read mission worktree ownership from {}",
            owner_path.display()
        )
    })?;
    let owner = owner.trim();
    if owner.is_empty() {
        Ok(None)
    } else {
        Ok(Some(owner.to_string()))
    }
}

fn current_mission_worktree_owner(command_name: &str) -> Result<String> {
    let cwd = env::current_dir().context("failed to read current checkout path")?;
    read_mission_worktree_owner(&cwd)?.ok_or_else(|| {
        anyhow::anyhow!(
            "{command_name} must be run inside a mission worktree. Current checkout: {}. Create or switch with `atelier worktree for-mission <mission-id>`.",
            cwd.display()
        )
    })
}

fn require_mission_worktree_owner(
    expected_mission_id: &str,
    command_name: &str,
) -> Result<PathBuf> {
    let cwd = env::current_dir().context("failed to read current checkout path")?;
    let actual_mission_id = read_mission_worktree_owner(&cwd)?.ok_or_else(|| {
        anyhow::anyhow!(
            "{command_name} must be run inside the owning mission worktree for {expected_mission_id}. Current checkout: {}. Create or switch with `atelier worktree for-mission {expected_mission_id}`.",
            cwd.display()
        )
    })?;
    if actual_mission_id != expected_mission_id {
        bail!(
            "{command_name} must be run inside the owning mission worktree for {expected_mission_id}. Current checkout {} belongs to mission {actual_mission_id}. Switch with `atelier worktree for-mission {expected_mission_id}`.",
            cwd.display()
        );
    }
    Ok(cwd)
}

fn owning_mission_id(db: &Database, issue_id: &str) -> Result<String> {
    containing_mission(db, issue_id)?.ok_or_else(|| {
        anyhow::anyhow!(
            "{issue_id} is not linked to an open mission. Link it to a mission before using epic branch commands."
        )
    })
}
