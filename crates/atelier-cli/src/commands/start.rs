//! `atelier start <id>`: check out the branch that owns an issue's work,
//! record that branch on its owner, and apply the issue's `start` transition
//! when the workflow offers one from the current status.
//!
//! Every check runs before any Git operation or record write. Atelier never
//! commits here; the record changes are left for the agent to commit with the
//! work they describe.

use anyhow::{bail, Context, Result};
use chrono::Utc;
use std::path::Path;
use std::process::Command;

use crate::commands::workflow;
use atelier_app::use_cases as app_use_cases;
use atelier_app::workflow_policy::{TransitionDefinition, WORKFLOW_BRANCH_FIELD};
use atelier_sqlite::Database;

const START_TRANSITION: &str = "start";

pub fn run(db: &Database, state_dir: &Path, issue_ref: &str) -> Result<()> {
    let issue_id = crate::commands::issue::resolve_id(db, issue_ref)?;
    let repo_root = workflow::repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let issue = db.require_issue(&issue_id)?;
    let work_branch = atelier_app::workflow_policy::resolve_work_branch(&policy, db, &issue.id)?;
    let branch = work_branch.branch.as_str();

    let transition = policy
        .workflow_for_issue_type(&issue.issue_type)?
        .transitions
        .get(START_TRANSITION)
        .filter(|transition| transition.from.contains(&issue.status));
    if let Some(transition) = transition {
        workflow::ensure_transition_allowed(
            db,
            &policy,
            &repo_root,
            &issue,
            START_TRANSITION,
            transition,
            None,
        )?;
    }

    let current = current_branch(&repo_root)?;
    let switch_needed = current.as_deref() != Some(branch);
    let branch_exists = branch_exists(&repo_root, branch)?;
    if switch_needed {
        let dirty = dirty_entries(&repo_root)?;
        if !dirty.is_empty() {
            bail!(
                "start_checkout_dirty: cannot switch to branch '{branch}' to start {}; the checkout has uncommitted changes:\n{}\nCommit or stash them, then rerun `atelier start {}`.",
                issue.id,
                dirty.join("\n"),
                issue.id
            );
        }
        if branch_exists {
            for record_id in [issue.id.as_str(), work_branch.owner_id.as_str()] {
                ensure_branch_has_record(&repo_root, state_dir, branch, record_id)?;
            }
        } else if current.is_none() {
            bail!(
                "start_detached_head: cannot create branch '{branch}' for {} from a detached HEAD; check out the branch to start from, then rerun `atelier start {}`.",
                issue.id,
                issue.id
            );
        }
    }
    let base = work_branch
        .recorded_base
        .clone()
        .or_else(|| (!branch_exists).then(|| current.clone()).flatten())
        .unwrap_or_else(|| policy.branch_policy.base_branch.clone());

    if switch_needed {
        let args: &[&str] = if branch_exists {
            &["switch", branch]
        } else {
            &["switch", "-c", branch]
        };
        println!("git {}", args.join(" "));
        git(&repo_root, args)?;
    } else {
        println!("Already on branch {branch}; no git operations.");
    }

    // Records are read again from the checked-out branch.
    let mut owner = app_use_cases::load_canonical_issue(state_dir, &work_branch.owner_id)?;
    let field = serde_json::json!({ "work_branch": branch, "branch_base": base });
    let field_changed = owner.issue.fields.get(WORKFLOW_BRANCH_FIELD) != Some(&field);
    if field_changed {
        owner
            .issue
            .fields
            .insert(WORKFLOW_BRANCH_FIELD.to_string(), field);
        owner.issue.updated_at = Utc::now();
    }
    let mut record = if owner.issue.id == issue.id {
        owner
    } else {
        if field_changed {
            app_use_cases::write_canonical_issue(state_dir, &owner)?;
        }
        app_use_cases::load_canonical_issue(state_dir, &issue.id)?
    };
    if field_changed {
        println!(
            "Recorded {WORKFLOW_BRANCH_FIELD} on {}: {branch} (base {base})",
            work_branch.owner_id
        );
    }

    match transition {
        Some(transition) if transition.from.contains(&record.issue.status) => {
            let from = record.issue.status.clone();
            workflow::apply_transition_record(&policy, state_dir, &mut record, transition, None)?;
            workflow::print_applied_transition(START_TRANSITION, &issue.id, &from, &transition.to);
        }
        _ => {
            if field_changed && record.issue.id == work_branch.owner_id {
                app_use_cases::write_canonical_issue(state_dir, &record)?;
            }
            print_status_unchanged(transition, &record.issue.status);
        }
    }
    println!("Branch:   {branch} (owner {})", work_branch.owner_id);
    println!("Base:     {base}");
    println!("Commit the record changes with the work they describe.");
    Ok(())
}

fn print_status_unchanged(transition: Option<&TransitionDefinition>, status: &str) {
    match transition {
        Some(_) => {
            println!("Status:   {status} (start transition no longer applies on this branch)")
        }
        None => println!("Status:   {status} (no start transition from this status)"),
    }
}

fn ensure_branch_has_record(
    repo_root: &Path,
    state_dir: &Path,
    branch: &str,
    record_id: &str,
) -> Result<()> {
    let record_path = state_dir
        .strip_prefix(repo_root)
        .unwrap_or(state_dir)
        .join("issues")
        .join(format!("{record_id}.md"));
    let record_path = record_path.to_string_lossy();
    let status = Command::new("git")
        .current_dir(repo_root)
        .args(["cat-file", "-e", &format!("{branch}:{record_path}")])
        .status()
        .context("failed to run git cat-file")?;
    if !status.success() {
        bail!(
            "start_branch_missing_record: branch '{branch}' does not contain {record_path}; commit the record on the branch that owns this work, then rerun `atelier start {record_id}`."
        );
    }
    Ok(())
}

fn current_branch(repo_root: &Path) -> Result<Option<String>> {
    let branch = git_stdout(repo_root, &["branch", "--show-current"])?;
    Ok(Some(branch).filter(|branch| !branch.is_empty()))
}

fn branch_exists(repo_root: &Path, branch: &str) -> Result<bool> {
    Ok(Command::new("git")
        .current_dir(repo_root)
        .args([
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ])
        .output()
        .context("failed to run git rev-parse")?
        .status
        .success())
}

fn dirty_entries(repo_root: &Path) -> Result<Vec<String>> {
    Ok(git_stdout(repo_root, &["status", "--porcelain"])?
        .lines()
        .map(str::to_string)
        .collect())
}

fn git_stdout(repo_root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(args)
        .output()
        .with_context(|| format!("failed to run git {}", args.join(" ")))?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn git(repo_root: &Path, args: &[&str]) -> Result<()> {
    git_stdout(repo_root, args).map(|_| ())
}
