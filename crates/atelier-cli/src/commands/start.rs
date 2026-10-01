//! `atelier start <id>`: check out the branch that owns an issue's work,
//! record that branch on its owner, and apply the issue's `start` transition
//! when the workflow offers one from the current status.
//!
//! Every check runs, and every record to be written is loaded, before any Git
//! operation or write. When `start` switches to an existing branch, it checks
//! and changes the records as they are on that branch, not on the current one.
//! Atelier never commits here; the record changes are left for the agent to
//! commit with the work they describe.

use anyhow::{bail, Context, Result};
use atelier_app::record_index::RecordIndex;
use chrono::Utc;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::commands::workflow;
use atelier_app::use_cases as app_use_cases;
use atelier_app::workflow_policy::WORKFLOW_BRANCH_FIELD;
use atelier_records::CanonicalIssueRecord;

const START_TRANSITION: &str = "start";

pub fn run(index: &RecordIndex, state_dir: &Path, issue_ref: &str) -> Result<()> {
    let issue_id = crate::commands::issue::resolve_id(index, issue_ref)?;
    let repo_root = workflow::repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let branch =
        atelier_app::workflow_policy::resolve_work_branch(&policy, index, &issue_id)?.branch;

    let current = current_branch(&repo_root)?;
    let switch_needed = current.as_deref() != Some(branch.as_str());
    let branch_exists = branch_exists(&repo_root, &branch)?;
    if switch_needed {
        let dirty = dirty_entries(&repo_root)?;
        if !dirty.is_empty() {
            bail!(
                "start_checkout_dirty: cannot switch to branch '{branch}' to start {issue_id}; the checkout has uncommitted changes:\n{}\nCommit or stash them, then rerun `atelier start {issue_id}`.",
                dirty.join("\n")
            );
        }
        if !branch_exists && current.is_none() {
            bail!(
                "start_detached_head: cannot create branch '{branch}' for {issue_id} from a detached HEAD; check out the branch to start from, then rerun `atelier start {issue_id}`."
            );
        }
    }

    // The records `start` checks and changes: the target branch's when it
    // switches to an existing branch, otherwise this checkout's (a new branch
    // starts from the clean current HEAD).
    let snapshot = if switch_needed && branch_exists {
        Some(BranchSnapshot::export(&repo_root, state_dir, &branch)?)
    } else {
        None
    };
    let (view_index, view_root) = match &snapshot {
        Some(snapshot) => (&snapshot.index, snapshot.root()),
        None => (index, repo_root.as_path()),
    };
    let Some(issue) = view_index.get_issue(&issue_id) else {
        bail!(
            "start_branch_missing_record: branch '{branch}' does not contain issue {issue_id}; commit the record on the branch that owns this work, then rerun `atelier start {issue_id}`."
        );
    };
    let source = atelier_app::workflow_policy::action_policy(&repo_root, view_index, &issue_id)?;
    for diagnostic in source.diagnostics {
        eprintln!("{diagnostic}");
    }
    let view_policy = source.policy;
    view_index.validate_policy(&view_policy, view_root)?;
    let work_branch =
        atelier_app::workflow_policy::resolve_work_branch(&view_policy, view_index, &issue_id)?;
    if work_branch.branch != branch {
        bail!(
            "start_branch_mismatch: on branch '{branch}', {issue_id} is owned by branch '{}'; repair the owner's {WORKFLOW_BRANCH_FIELD} field, then rerun `atelier start {issue_id}`.",
            work_branch.branch
        );
    }
    if view_policy.status_category(&issue.status) == Some("done") {
        bail!(
            "start_issue_done: {issue_id} has done status '{}'{}; there is no work to start.",
            issue.status,
            if snapshot.is_some() {
                format!(" on branch '{branch}'")
            } else {
                String::new()
            }
        );
    }
    let transition = view_policy
        .workflow_for_issue_type(&issue.issue_type)?
        .transitions
        .get(START_TRANSITION)
        .filter(|transition| transition.from.contains(&issue.status));
    if let Some(transition) = transition {
        workflow::ensure_transition_allowed(
            view_index,
            &view_policy,
            view_root,
            &issue,
            START_TRANSITION,
            transition,
            None,
        )?;
    }

    // Prepare every record change before acting. A recorded base is never
    // replaced; a missing one is the branch `start` creates from, or the
    // configured base branch for a branch created by other means.
    let base =
        work_branch
            .recorded_base
            .clone()
            .unwrap_or_else(|| match (&current, branch_exists) {
                (Some(current), false) => current.clone(),
                _ => view_policy.branch_policy.base_branch.clone(),
            });
    let mut owner = view_index
        .issue_record(&work_branch.owner_id)
        .cloned()
        .with_context(|| format!("Issue {} not found", work_branch.owner_id))?;
    let record_branch = work_branch.recorded_base.is_none();
    if record_branch {
        owner.issue.workflow_branch = Some(atelier_core::WorkflowBranch {
            work_branch: branch.clone(),
            branch_base: base.clone(),
        });
        owner.issue.updated_at = Utc::now();
    }
    let mut writes: Vec<CanonicalIssueRecord> = Vec::new();
    if let Some(transition) = transition {
        let mut record = if owner.issue.id == issue_id {
            owner.clone()
        } else {
            view_index
                .issue_record(&issue_id)
                .cloned()
                .with_context(|| format!("Issue {issue_id} not found"))?
        };
        workflow::apply_transition(&view_policy, &mut record, transition, None);
        if record.issue.id != owner.issue.id && record_branch {
            writes.push(owner);
        }
        writes.push(record);
    } else if record_branch {
        writes.push(owner);
    }
    drop(snapshot);

    // Act.
    if switch_needed {
        let args: &[&str] = if branch_exists {
            &["switch", &branch]
        } else {
            &["switch", "-c", &branch]
        };
        println!("git {}", args.join(" "));
        git_stdout(&repo_root, args)?;
    } else {
        println!("Already on branch {branch}; no git operations.");
    }
    for record in &writes {
        app_use_cases::write_canonical_issue(state_dir, record)?;
    }

    if record_branch {
        println!(
            "Recorded {WORKFLOW_BRANCH_FIELD} on {}: {branch} (base {base})",
            work_branch.owner_id
        );
    }
    match transition {
        Some(transition) => workflow::print_applied_transition(
            START_TRANSITION,
            &issue_id,
            &issue.status,
            &transition.to,
        ),
        None => println!(
            "Status:   {} (no start transition from this status)",
            issue.status
        ),
    }
    println!("Branch:   {branch} (owner {})", work_branch.owner_id);
    println!("Base:     {base}");
    println!("Commit the record changes with the work they describe.");
    Ok(())
}

/// A read-only copy of another branch's tracker records, loaded into an
/// index so `start` can check the records it is about to change before
/// switching.
struct BranchSnapshot {
    dir: tempfile::TempDir,
    index: RecordIndex,
}

impl BranchSnapshot {
    fn export(repo_root: &Path, state_dir: &Path, branch: &str) -> Result<Self> {
        let state_relative = state_dir.strip_prefix(repo_root).with_context(|| {
            format!(
                "tracker state {} is outside repository {}",
                state_dir.display(),
                repo_root.display()
            )
        })?;
        let dir = tempfile::tempdir().context("failed to create branch snapshot directory")?;
        // Fully qualified: Git resolves a bare name to a same-named tag first.
        let branch_ref = format!("refs/heads/{branch}");
        let mut archive = Command::new("git")
            .current_dir(repo_root)
            .args(["archive", "--format=tar", &branch_ref, "--"])
            .arg(state_relative)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("failed to run git archive")?;
        let tar = Command::new("tar")
            .current_dir(dir.path())
            .args(["-xf", "-"])
            .stdin(archive.stdout.take().context("git archive has no stdout")?)
            .output()
            .context("failed to run tar")?;
        let archive = archive.wait_with_output()?;
        if !archive.status.success() || !tar.status.success() {
            bail!(
                "start_branch_snapshot: cannot read tracker records from branch '{branch}': {}{}",
                String::from_utf8_lossy(&archive.stderr).trim(),
                String::from_utf8_lossy(&tar.stderr).trim()
            );
        }
        let index =
            RecordIndex::load_structural(&dir.path().join(state_relative)).with_context(|| {
                format!("start_branch_snapshot: tracker records on branch '{branch}' are invalid")
            })?;
        Ok(Self { dir, index })
    }

    fn root(&self) -> &Path {
        self.dir.path()
    }
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
