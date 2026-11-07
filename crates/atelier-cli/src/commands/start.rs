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
//! The branch is chosen by where `start` runs (atelier-md0v): an owner with
//! a recorded branch, in this checkout's records or in a local branch's own,
//! switches to it; on the base, a new branch named with the owner's bare ID
//! is created; on a branch another item owns, `start` refuses and names that
//! item; on any other branch, `start` takes it. A detached HEAD has no branch
//! to take or create from. Branch names carry no issue type, and the base is
//! always the policy's `base_branch`.

use anyhow::{bail, Context, Result};
use atelier_app::record_index::RecordIndex;
use atelier_app::record_ref::RecordKind;
use chrono::Utc;
use std::path::Path;

use crate::commands::{bundle, git};
use crate::commands::{lease, workflow};
use atelier_app::workflow_policy::WORKFLOW_BRANCH_FIELD;
use atelier_records::CanonicalIssueRecord;

const START_TRANSITION: &str = "start";

/// Quiet output is the branch name alone; notes about human work go to stderr.
pub fn run(
    index: &RecordIndex,
    state_dir: &Path,
    issue_ref: &str,
    take_over: bool,
    quiet: bool,
) -> Result<()> {
    let repo_root = workflow::repo_root()?;
    let issue_id = crate::commands::issue::resolve_id(index, issue_ref)?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    // Work whose workflow has no `start` transition is done on the base and
    // takes no branch (atelier-8bh4).
    if let Some(issue) = index.get_issue(&issue_id) {
        workflow::refuse_done_on_base(&policy, &issue, "start")?;
    }
    let policy_base = policy.branch_policy.base_branch.clone();
    let derived = atelier_app::workflow_policy::resolve_work_branch(&policy, index, &issue_id)?;
    let current = git::current_branch(&repo_root)?;
    // A local branch whose own record names it the owner's work branch,
    // when this checkout's records do not yet, as on the base before the
    // work merges.
    let owned_on = if derived.recorded {
        None
    } else {
        git::owning_branches(&repo_root, &derived.owner_id)?
            .into_iter()
            .next()
    };
    let branch = match (current.as_deref(), owned_on) {
        // A recorded branch is where the work is.
        _ if derived.recorded => derived.branch.clone(),
        (_, Some(owned)) => owned,
        // On the base, or detached, the owner gets a branch named with its
        // bare ID.
        (None, None) => derived.branch.clone(),
        (Some(name), None) if name == policy_base => derived.branch.clone(),
        // Another item's branch is not this item's to take; this item's
        // parent owning it would have recorded it above.
        (Some(name), None) => {
            if let Some(other) = crate::commands::session::branch_owners(&repo_root, index, name)
                .into_iter()
                .find(|id| *id != derived.owner_id)
            {
                bail!(
                    "start_branch_owned: '{name}' is the work branch of {other}, so {issue_id} cannot start on it. Next: git switch {policy_base} && atelier start {issue_id}"
                );
            }
            name.to_owned()
        }
    };

    let switch_needed = current.as_deref() != Some(branch.as_str());
    let existing = git::work_ref(&repo_root, &branch)?;
    let branch_exists = existing.is_some();
    if switch_needed {
        // An untracked file outside the tracker is no commit's, and a switch
        // carries it along, so it holds nothing up unless the switch would
        // overwrite it; an untracked record would join the started branch's
        // records, so it refuses with uncommitted changes (atelier-l457).
        let untracked = atelier_app::git_freshness::untracked_paths(&repo_root)?;
        let dirty = atelier_app::git_freshness::working_paths(&repo_root)?
            .into_iter()
            .filter(|path| path.starts_with(".atelier") || !untracked.contains(path))
            .collect::<Vec<_>>();
        if !dirty.is_empty() {
            let stash = if dirty.iter().any(|path| untracked.contains(path)) {
                "git stash push --include-untracked"
            } else {
                "git stash push"
            };
            bail!(
                "start_checkout_dirty: cannot switch to branch '{branch}' to start {issue_id}; the checkout has uncommitted changes:\n{}\nCommit them, or set them aside with `{stash}`, then rerun `atelier start {issue_id}`.",
                dirty.iter().map(|path| path.display().to_string()).collect::<Vec<_>>().join("\n")
            );
        }
        // A new branch starts at HEAD, so only switching to an existing one
        // can write over an untracked file.
        if let Some((_, commit)) = &existing {
            let overwritten = git::untracked_overwritten(&repo_root, &[commit])?;
            if !overwritten.is_empty() {
                bail!(
                    "start_untracked_overwritten: switching to branch '{branch}' to start {issue_id} would overwrite untracked paths it holds:\n{}\nMove them out of the way, then rerun `atelier start {issue_id}`.",
                    overwritten.iter().map(|path| path.display().to_string()).collect::<Vec<_>>().join("\n")
                );
            }
        }
        if !branch_exists && current.is_none() {
            bail!(
                "start_detached_head: cannot create branch '{branch}' for {issue_id} from a detached HEAD. Next: git switch {policy_base} && atelier start {issue_id}"
            );
        }
    }

    // The records `start` checks and changes: the target branch's when it
    // switches to an existing branch, otherwise this checkout's (a new
    // branch starts from the clean current HEAD).
    let pinned = if switch_needed {
        existing.as_ref().map(|(_, commit)| commit.clone())
    } else {
        None
    };
    let snapshot = match &existing {
        Some((_, commit)) if switch_needed => {
            Some(git::Snapshot::at(&repo_root, state_dir, commit)?)
        }
        _ => None,
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
    // Another worktree's live lease on the owner refuses the start unless the
    // caller takes it over; a stale one is replaced with a note.
    let mut lease_notes = Vec::new();
    match lease::held_by_other(&repo_root, &work_branch.owner_id)? {
        Some(lease::Held::Live(held)) if !take_over => bail!(
            "start_item_held: {} is held by another session: {}. Work something else, or rerun `atelier start {issue_id} --take-over` to take it over.",
            work_branch.owner_id,
            lease::describe(&held)
        ),
        Some(lease::Held::Live(held)) => lease_notes.push(format!(
            "Took over {} from {}.",
            work_branch.owner_id,
            lease::describe(&held)
        )),
        Some(lease::Held::Stale(held, why)) => lease_notes.push(format!(
            "Replaced a stale lease on {}: {}; {why}.",
            work_branch.owner_id,
            lease::describe(&held)
        )),
        None => {}
    }
    if work_branch.recorded && work_branch.branch != branch {
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

    // Prepare every record change before acting. A recorded branch is never
    // replaced. The base is the policy's; `branch_base` is still written,
    // and ignored when read, so an older Atelier can parse the record
    // (atelier-md0v).
    let base = view_policy.branch_policy.base_branch.clone();
    let mut owner = view_index
        .issue_record(&work_branch.owner_id)
        .cloned()
        .ok_or_else(|| view_index.not_found(RecordKind::Issue, &work_branch.owner_id))?;
    let record_branch = !work_branch.recorded;
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
    // The records start writes pass the rules `atelier check` applies,
    // checked against the records they join (atelier-ilba).
    let changes = writes
        .iter()
        .map(|record| -> Result<_> {
            Ok(atelier_records::guard::Change {
                path: atelier_records::issue_record_path(&record.issue.id),
                text: Some(atelier_records::render_issue_record(record)?),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    atelier_app::record_guard::check_changes(&view_root.join(".atelier"), &changes)?;
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
        let store = atelier_records::RecordStore::staging(stage.path());
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
    // A branch reviewed on the light path follows the issue path once an
    // issue owns it, where its records about the branch count for nothing
    // (atelier-4nsl).
    let light_records = view_index
        .branch_reviews()
        .filter(|record| record.target == branch)
        .count();
    drop(snapshot);
    let previous_head = atelier_app::git_freshness::head_commit(&repo_root)?;

    // Act.
    if switch_needed {
        let args: Vec<&str> = match &existing {
            Some((reference, _)) if reference.starts_with("refs/remotes/") => {
                vec!["switch", "-c", &branch, "--track", reference]
            }
            Some(_) => vec!["switch", &branch],
            None => vec!["switch", "-c", &branch],
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

    // The lease is advisory and the start has already succeeded: a failed
    // write is a warning, not a failed start (atelier-x5j3).
    if let Err(error) = lease::claim(&repo_root, &work_branch.owner_id, &branch) {
        lease_notes.push(format!(
            "Warning: started {}, but could not write its session lease ({error:#}); other sessions will not see this item as held.",
            work_branch.owner_id
        ));
    }
    let mut notes = start_notes(
        &view_policy,
        &issue_id,
        human_work,
        &work_branch.owner_id,
        &human_children,
    );
    notes.extend(lease_notes);
    if record_branch && light_records > 0 {
        notes.push(format!(
            "{branch} holds {} about the branch itself, which stop counting now that {} owns it; get {} reviewed and approved before `atelier merge`.",
            atelier_core::count_noun(light_records, "light-path review record"),
            work_branch.owner_id,
            work_branch.owner_id
        ));
    }
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
    policy: &atelier_app::workflow_policy::WorkflowPolicy,
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
            policy,
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
