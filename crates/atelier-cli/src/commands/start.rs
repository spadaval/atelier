//! `atelier start <id>`: check out the branch that owns an issue's work,
//! record that branch on its owner, and apply the issue's `start` transition
//! when the workflow offers one from the current status.
//!
//! Every check runs, and every record to be written is loaded, before any Git
//! operation or write. When `start` switches to an existing branch, it checks
//! and changes the records as they are on that branch, not on the current one.
//! Atelier never commits here; the record changes are left for the agent to
//! commit with the work they describe.
//!
//! A new owner branch is created from the current branch, or from the base
//! named by `--base` whatever is checked out, including a detached HEAD. The
//! recorded base must be the policy's base branch, the one `land` trusts.

use anyhow::{bail, Context, Result};
use atelier_app::record_index::RecordIndex;
use atelier_app::record_ref::RecordKind;
use chrono::Utc;
use std::path::Path;

use crate::commands::workflow;
use crate::commands::{bundle, git};
use atelier_app::workflow_policy::WORKFLOW_BRANCH_FIELD;
use atelier_records::CanonicalIssueRecord;

const START_TRANSITION: &str = "start";

/// Quiet output is the branch name alone; notes about human work go to stderr.
pub fn run(
    index: &RecordIndex,
    state_dir: &Path,
    issue_ref: &str,
    base_arg: Option<&str>,
    quiet: bool,
) -> Result<()> {
    let issue_id = crate::commands::issue::resolve_id(index, issue_ref)?;
    let repo_root = workflow::repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    let branch =
        atelier_app::workflow_policy::resolve_work_branch(&policy, index, &issue_id)?.branch;

    let current = git::current_branch(&repo_root)?;
    let switch_needed = current.as_deref() != Some(branch.as_str());
    let existing = git::work_ref(&repo_root, &branch)?;
    let branch_exists = existing.is_some();
    if switch_needed {
        let dirty = atelier_app::git_freshness::working_paths(&repo_root)?;
        if !dirty.is_empty() {
            bail!(
                "start_checkout_dirty: cannot switch to branch '{branch}' to start {issue_id}; the checkout has uncommitted changes:\n{}\nCommit or stash them, then rerun `atelier start {issue_id}`.",
                dirty.iter().map(|path| path.display().to_string()).collect::<Vec<_>>().join("\n")
            );
        }
        if !branch_exists && current.is_none() && base_arg.is_none() {
            bail!(
                "start_detached_head: cannot create branch '{branch}' for {issue_id} from a detached HEAD; pass the base to create it from with `atelier start {issue_id} --base <branch>`, or check out the branch to start from."
            );
        }
    }

    // `--base` names a local branch to create the owner branch from.
    let explicit = base_arg
        .filter(|_| !branch_exists)
        .map(|name| -> Result<(String, String)> {
            let commit = git::resolve(&repo_root, &format!("refs/heads/{name}")).map_err(|_| {
                anyhow::anyhow!("start_base_missing: there is no local branch '{name}' to start {issue_id} from; create or fetch it, then rerun `atelier start {issue_id} --base {name}`.")
            })?;
            Ok((name.to_owned(), commit))
        })
        .transpose()?;
    // The records `start` checks and changes: the target branch's when it
    // switches to an existing branch, the explicit base's when a new branch
    // is created from it, otherwise this checkout's (a new branch starts from
    // the clean current HEAD).
    let pinned = if switch_needed {
        existing
            .as_ref()
            .map(|(_, commit)| commit.clone())
            .or_else(|| explicit.as_ref().map(|(_, commit)| commit.clone()))
    } else {
        None
    };
    let snapshot = pinned
        .as_ref()
        .map(|commit| git::Snapshot::at(&repo_root, state_dir, commit))
        .transpose()?;
    let (view_index, view_root) = match &snapshot {
        Some(snapshot) => (&snapshot.index, snapshot.root()),
        None => (index, repo_root.as_path()),
    };
    let Some(issue) = view_index.get_issue(&issue_id) else {
        if let Some((name, _)) = &explicit {
            bail!(
                "start_branch_missing_record: base '{name}' does not contain issue {issue_id}; commit the record on {name}, then rerun `atelier start {issue_id} --base {name}`."
            );
        }
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
    if let Some(name) = base_arg {
        let policy_base = &view_policy.branch_policy.base_branch;
        let recorded = work_branch.recorded_base.as_ref().unwrap_or(policy_base);
        if name != policy_base || name != recorded {
            bail!(
                "start_base_mismatch: --base {name} is not the base land trusts for {issue_id}: the policy base is {policy_base}{}; rerun `atelier start {issue_id} --base {policy_base}`.",
                if recorded == policy_base { String::new() } else { format!(" and the recorded base is {recorded}") }
            );
        }
    }
    if work_branch.recorded_base.is_none() && base_arg.is_none() && !branch_exists {
        if let Some(wrong) = current.as_deref().filter(|name| {
            *name != branch
                && name
                    .rsplit_once('/')
                    .is_some_and(|(_, tail)| tail == work_branch.owner_id)
        }) {
            let owner_type = &view_index.require_issue(&work_branch.owner_id)?.issue_type;
            let policy_base = &view_policy.branch_policy.base_branch;
            bail!(
                "start_branch_prefix_mismatch: you are on '{wrong}', but {} is a {owner_type}, so its branch is '{branch}'; start would record '{wrong}' as its base. Rerun `atelier start {issue_id} --base {policy_base}` (or switch to {policy_base} first) and start creates '{branch}'.",
                work_branch.owner_id
            );
        }
    }
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
            quiet,
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
                (_, false) if explicit.is_some() => explicit.as_ref().unwrap().0.clone(),
                (Some(current), false) => current.clone(),
                _ => view_policy.branch_policy.base_branch.clone(),
            });
    let mut owner = view_index
        .issue_record(&work_branch.owner_id)
        .cloned()
        .ok_or_else(|| view_index.not_found(RecordKind::Issue, &work_branch.owner_id))?;
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
                .ok_or_else(|| view_index.not_found(RecordKind::Issue, &issue_id))?
        };
        workflow::apply_transition(&view_policy, &mut record, transition, None);
        if record.issue.id != owner.issue.id && record_branch {
            writes.push(owner);
        }
        writes.push(record);
    } else if record_branch {
        writes.push(owner);
    }
    // Stage the entire issue directory, so owner + child install as one
    // rollback-protected transaction under the command's exclusive lock.
    let staged = if writes.is_empty() {
        None
    } else {
        let source = view_root.join(".atelier/issues");
        let fingerprint = crate::commands::bulk_canonical::canonical_tree_fingerprint(&source)?;
        let runtime = state_dir.join("runtime");
        std::fs::create_dir_all(&runtime)?;
        let stage = tempfile::tempdir_in(&runtime)?;
        bundle::copy_dir_recursive(&source, &stage.path().join("issues"))?;
        let store = atelier_records::RecordStore::new(stage.path());
        for record in &writes {
            store.write_issue_atomic(record)?;
        }
        Some((stage, fingerprint))
    };
    // Work started while the local base holds unreviewed commits would carry
    // them in, and a review of `base...branch` would never see them. The
    // base's own records say which of its commits came through review.
    // A base without an upstream has nothing to check, and a base without
    // the workflow policy predates the gates (Atelier adopted on a branch).
    let policy_base = &view_policy.branch_policy.base_branch;
    let checkable = git::resolve(&repo_root, &format!("refs/heads/{policy_base}"))
        .ok()
        .filter(|_| git::resolve(&repo_root, &format!("{policy_base}@{{upstream}}")).is_ok())
        .filter(|commit| {
            atelier_app::git_freshness::git_output(
                &repo_root,
                &[
                    "ls-tree",
                    commit,
                    "--",
                    atelier_app::workflow_policy::WORKFLOW_POLICY_PATH,
                ],
            )
            .is_ok_and(|listed| !listed.is_empty())
        });
    if let Some(base_commit) = checkable {
        let base_records = git::Snapshot::at(&repo_root, state_dir, &base_commit)?;
        git::check_base_commits(&repo_root, &base_records.index, &view_policy, &base_commit)?;
    }
    let human_work = view_index
        .issue_record(&issue_id)
        .is_some_and(|record| atelier_app::work_classification::needs_human(&record.labels));
    let human_children = atelier_app::work_classification::open_human_children(
        &view_index,
        &view_policy,
        &work_branch.owner_id,
    );
    drop(snapshot);
    let previous_head = atelier_app::git_freshness::head_commit(&repo_root)?;

    // Act.
    if switch_needed {
        let args: Vec<&str> = match &existing {
            Some((reference, _)) if reference.starts_with("refs/remotes/") => {
                vec!["switch", "-c", &branch, "--track", reference]
            }
            Some(_) => vec!["switch", &branch],
            None => match &explicit {
                Some((_, commit)) => vec!["switch", "-c", &branch, commit],
                None => vec!["switch", "-c", &branch],
            },
        };
        if !quiet {
            println!("git {}", args.join(" "));
        }
        checkout(&repo_root, &args)?;
    } else if !quiet {
        println!("Already on branch {branch}; no git operations.");
    }
    let install = (|| -> Result<()> {
        let expected = pinned.as_ref().unwrap_or(&previous_head);
        if atelier_app::git_freshness::head_commit(&repo_root)? != *expected
            || git::current_branch(&repo_root)?.as_deref() != Some(&branch)
        {
            bail!("start_branch_drift: branch changed after validation; no records installed");
        }
        if let Some((stage, fingerprint)) = &staged {
            crate::commands::bulk_canonical::ensure_unchanged(
                &state_dir.join("issues"),
                fingerprint,
            )?;
            bundle::install_directories(stage.path(), state_dir, &["issues"], &mut |_| Ok(()))?;
        }
        Ok(())
    })();
    if let Err(error) = install {
        if switch_needed {
            let restore = match &current {
                Some(previous) => checkout(&repo_root, &["switch", previous]),
                None => checkout(&repo_root, &["switch", "--detach", &previous_head]),
            };
            restore.context(
                "start rollback: record install failed; could not restore previous checkout",
            )?;
        }
        return Err(error).context("start record transaction failed; prior records restored. Inspect git status before retrying; a newly created local branch may remain.");
    }

    let notes = start_notes(
        &issue_id,
        human_work,
        &work_branch.owner_id,
        &human_children,
    );
    if quiet {
        println!("{branch}");
        for note in notes {
            eprintln!("{note}");
        }
        return Ok(());
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
    for note in notes {
        println!("Note:     {note}");
    }
    println!("Commit the record changes with the work they describe.");
    Ok(())
}

/// Notes that `start` is starting work for the human.
fn start_notes(
    issue_id: &str,
    human_work: bool,
    owner_id: &str,
    human_children: &[String],
) -> Vec<String> {
    let mut notes = Vec::new();
    if human_work {
        notes.push(format!(
            "{issue_id} is labelled needs-human; it is work for the human, not an agent."
        ));
    }
    if !human_children.is_empty() {
        notes.push(atelier_app::work_classification::human_children_note(
            owner_id,
            human_children,
        ));
    }
    notes
}

fn checkout(repo: &Path, args: &[&str]) -> Result<String> {
    // Git hooks may create commits or mutate records, outside start's contract.
    let args = ["-c", "core.hooksPath=/dev/null"]
        .into_iter()
        .chain(args.iter().copied())
        .collect::<Vec<_>>();
    git::text(repo, &args)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paired_owner_child_install_failure_restores_all_previous_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let state = dir.path().join(".atelier");
        let staged = dir.path().join("stage");
        for root in [&state, &staged] {
            std::fs::create_dir_all(root.join("issues")).unwrap();
        }
        for name in ["owner.md", "child.md"] {
            std::fs::write(state.join("issues").join(name), "prior record").unwrap();
            std::fs::write(staged.join("issues").join(name), "new record").unwrap();
        }
        let result = bundle::install_directories(&staged, &state, &["issues"], &mut |_| {
            bail!("installation failure")
        });
        assert!(result.is_err());
        for name in ["owner.md", "child.md"] {
            assert_eq!(
                std::fs::read_to_string(state.join("issues").join(name)).unwrap(),
                "prior record"
            );
        }
    }
}
