//! Read-only landing and Claude Code hook input; no durable session state.
use crate::commands::git;
use anyhow::{bail, Context, Result};
use atelier_app::{
    base_policy, git_freshness, record_index::RecordIndex, review_gate,
    storage_layout::StorageLayout, workflow_policy,
};
use serde_json::{json, Value};
use std::{io::Read, path::Path};

fn owner(index: &RecordIndex, branch: &str, requested: Option<&str>) -> Result<Option<String>> {
    if let Some(reference) = requested {
        let id = index.resolve(atelier_app::record_ref::RecordKind::Issue, reference)?;
        let issue = index.require_issue(&id)?;
        return Ok(Some(issue.parent_id.clone().unwrap_or(id)));
    }
    let owners = branch_owners(index, branch);
    match owners.as_slice() {
        [] => Ok(None),
        [id] => Ok(Some(id.clone())),
        _ => bail!(
            "land_owner_ambiguous: {branch} has several owners: {}; pass the intended issue ID",
            owners.join(", ")
        ),
    }
}
/// The issues whose recorded work branch is `branch`; more than one is a
/// recording error that each caller reports in its own terms.
pub(crate) fn branch_owners(index: &RecordIndex, branch: &str) -> Vec<String> {
    index
        .issues()
        .filter(|issue| {
            issue
                .workflow_branch
                .as_ref()
                .is_some_and(|owner| owner.work_branch == branch)
        })
        .map(|issue| issue.id.clone())
        .collect()
}
fn check_records(
    repo: &Path,
    snapshot: &atelier_app::committed_snapshot::CommittedSnapshot,
) -> Result<Vec<std::path::PathBuf>> {
    snapshot
        .verified_working_paths(repo)
        .map_err(records_uncommitted)
}
/// Land names its own check for records that differ from HEAD: one code,
/// every record, and one recovery step.
fn records_uncommitted(error: anyhow::Error) -> anyhow::Error {
    match error.downcast_ref::<atelier_app::committed_snapshot::RecordsUncommitted>() {
        Some(uncommitted) => anyhow::anyhow!(
            "land_records_uncommitted: {}; {}. Next: {}",
            uncommitted.summary(),
            uncommitted.advice(),
            uncommitted.commit_command()
        ),
        None => error,
    }
}
fn check_traceability(
    repo: &Path,
    index: &RecordIndex,
    policy: &atelier_workflow::WorkflowPolicy,
) -> Result<()> {
    let active = index
        .issues()
        .filter(|issue| {
            policy.status_category(&issue.status) == Some("active") && issue.issue_type != "mission"
        })
        .collect::<Vec<_>>();
    for issue in active {
        let work = workflow_policy::resolve_work_branch(policy, index, &issue.id)?;
        let owner = index.require_issue(&work.owner_id)?;
        if owner.workflow_branch.is_none() || git::work_ref(repo, &work.branch)?.is_none() {
            // A local branch that owns it is where it lands (atelier-9a6e).
            if let Some(owned) = git::owning_branches(repo, &work.owner_id)?.first() {
                bail!("land_work_untraceable: active work {} is owned on {owned}, which this checkout's records do not name. Next: git switch {owned} && atelier land", issue.id);
            }
            bail!("land_work_untraceable: active work {} has no recorded existing owner branch. Next: atelier start {}", issue.id, issue.id);
        }
    }
    Ok(())
}
/// Tracker files (anything under `.atelier/`) a branch changes relative to
/// the base. Every local and remote-tracking copy of the base counts, so a
/// stale local base cannot attribute upstream changes to the branch.
/// `None` when the base has no Atelier policy yet, so the branch installs it.
fn tracker_changes_since(repo: &Path, base: &str) -> Result<Option<Vec<std::path::PathBuf>>> {
    let refs = git::text(
        repo,
        &[
            "for-each-ref",
            "--format=%(refname)",
            &format!("refs/heads/{base}"),
            "refs/remotes/",
        ],
    )?;
    let refs = refs
        .lines()
        .filter(|reference| {
            reference.strip_prefix("refs/heads/") == Some(base)
                || reference
                    .strip_prefix("refs/remotes/")
                    .and_then(|rest| rest.split_once('/'))
                    .is_some_and(|(_, name)| name == base)
        })
        .collect::<Vec<_>>();
    if refs.is_empty() {
        bail!("land_work_untraceable: base branch {base} not found, so this unowned branch's tracker changes cannot be checked; fetch {base}, or give the work an owner branch with `atelier start` and its item's ID");
    }
    // No copy of the base has a workflow policy yet, so there are no gates on
    // the base to land through; the caller says whether this is an install.
    if refs.iter().all(|reference| {
        git_freshness::git_output(
            repo,
            &[
                "ls-tree",
                reference,
                "--",
                workflow_policy::WORKFLOW_POLICY_PATH,
            ],
        )
        .is_ok_and(|listed| listed.is_empty())
    }) {
        return Ok(None);
    }
    let mut changed: Option<Vec<std::path::PathBuf>> = None;
    for reference in refs {
        let Ok(fork) = git::text(repo, &["merge-base", reference, "HEAD"]) else {
            continue;
        };
        let paths = git_freshness::changed_paths(repo, &fork, "HEAD")?
            .into_iter()
            .filter(|path| path.starts_with(".atelier"))
            .collect::<Vec<_>>();
        changed = Some(match changed {
            Some(previous) => previous
                .into_iter()
                .filter(|path| paths.contains(path))
                .collect(),
            None => paths,
        });
    }
    changed.map(Some).with_context(|| {
        format!("land_work_untraceable: this unowned branch shares no history with {base}, so its tracker changes cannot be checked; start the work from {base} with `atelier start` and its item's ID")
    })
}
/// Names the commits the local base holds beyond its upstream that the
/// current owner did not make, since pushing the base publishes them
/// (atelier-mvzi). A commit is the owner's when its message names the owner or
/// an issue in the owner's scope (`owned`), as the records start, close and
/// merge commits for owned work do; with no owner, none are.
fn foreign_base_commits_note(
    repo: &Path,
    base: &str,
    owned: &std::collections::BTreeSet<String>,
) -> Result<Option<String>> {
    let Ok(upstream) = git::resolve(repo, &format!("{base}@{{upstream}}")) else {
        return Ok(None);
    };
    let Ok(local) = git::resolve(repo, &format!("refs/heads/{base}")) else {
        return Ok(None);
    };
    let range = format!("{upstream}..{local}");
    let log = git::text(repo, &["log", "--format=%h%x1f%an%x1f%s%x1f%B%x1e", &range])?;
    let foreign = log
        .split('\x1e')
        .filter_map(|entry| {
            let mut parts = entry.trim().splitn(4, '\x1f');
            let (short, author, subject, message) =
                (parts.next()?, parts.next()?, parts.next()?, parts.next()?);
            (!short.is_empty() && !owned.iter().any(|id| message.contains(id.as_str())))
                .then(|| format!("{short} {subject} ({author})"))
        })
        .collect::<Vec<_>>();
    if foreign.is_empty() {
        return Ok(None);
    }
    let listed = crate::human_output::first_five(foreign.iter().cloned(), "; ");
    Ok(Some(format!(
        "Warning: {base} is ahead of its upstream with {} that {}: {listed}. Pushing {base} publishes them; confirm they are meant to go out.",
        crate::human_output::count_noun(foreign.len(), "commit"),
        if owned.is_empty() { "no owner made" } else { "this owner did not make" },
    )))
}
fn land_text(requested: Option<&str>, install: bool, deep: bool) -> Result<String> {
    land_checked(requested, install, deep).map(|(text, _)| text)
}
/// Whether `code` is one of land's own checks or the record fence it runs.
fn is_land_code(code: &str) -> bool {
    code.starts_with("land_") || code == "review_append_only"
}
/// The landing report, and the commit every check ran against. A failed
/// precondition is one more failed check, and the gates still run against
/// what can be read, such as HEAD's committed records when the checkout's
/// differ; one that keeps the gates from running is reported with the
/// preconditions before it.
fn land_checked(requested: Option<&str>, install: bool, deep: bool) -> Result<(String, String)> {
    let mut preconditions = crate::gate_report::Preconditions::default();
    land_gates(requested, install, deep, &mut preconditions).map_err(|error| {
        let skipped = if preconditions.owner.is_some() {
            "the owner's review, findings, proof, blocker and open-work gates and the done claims"
        } else {
            "the base commit and done-claim checks"
        };
        preconditions.refusal(error, "land", skipped, is_land_code)
    })
}
fn land_gates(
    requested: Option<&str>,
    install: bool,
    deep: bool,
    preconditions: &mut crate::gate_report::Preconditions,
) -> Result<(String, String)> {
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    if let Some((operation, abort)) = git::operation_in_progress(repo)? {
        return Err(git::operation_refusal(
            &format!("land_{operation}_in_progress"),
            operation,
            abort,
        ));
    }
    let (head, branch, snapshot, initial_paths, index, uncommitted) = {
        let _lock =
            atelier_records::mutation_lock::CanonicalMutationLock::shared(&layout.canonical_dir())?;
        let head = git_freshness::head_commit(repo)?;
        let branch = git::checkout_branch(repo)?.unwrap_or_default();
        let snapshot = atelier_app::committed_snapshot::CommittedSnapshot::new(&head)?;
        match snapshot.read_record_files(repo, false) {
            Ok((files, paths)) => {
                git::require_committed_config(repo, &head, "land")?;
                let index = files.into_index()?;
                (head, branch, snapshot, paths, index, None)
            }
            // The gates read HEAD's committed records instead, so every
            // other failure is reported with this one.
            Err(error) => {
                let uncommitted =
                    error.downcast::<atelier_app::committed_snapshot::RecordsUncommitted>()?;
                let (dir, index) =
                    git::Snapshot::at(repo, &layout.canonical_dir(), &head)?.into_parts();
                let paths = snapshot.working_paths(repo)?;
                (
                    head,
                    branch,
                    snapshot,
                    paths,
                    index,
                    Some((uncommitted, dir)),
                )
            }
        }
    };
    // Where the records the gates read are: the checkout, or HEAD's when
    // the checkout's differ.
    let records_root = uncommitted.as_ref().map_or(repo, |(_, dir)| dir.path());
    let mut read = git_freshness::ReadContext::from_snapshot(repo, head.clone(), &initial_paths);
    // The candidate policy is the committed one; it is the working one when
    // the records are committed.
    let candidate = match &uncommitted {
        None => atelier_workflow::load(repo),
        Some(_) => git_freshness::blob(
            repo,
            &head,
            Path::new(workflow_policy::WORKFLOW_POLICY_PATH),
        )
        .and_then(|text| {
            atelier_workflow::parse_policy_text(&text, workflow_policy::WORKFLOW_POLICY_PATH)
        }),
    };
    let candidate_invalid = match candidate {
        Ok(_) => None,
        Err(error) => Some((
            crate::gate_report::policy_failure(&error).with_context(|| {
                format!("candidate workflow policy is invalid: {error:#}. Next: fix .atelier/workflow.yaml and commit it")
            })?,
            atelier_workflow::needs_newer_atelier(&error),
        )),
    };
    let record_failures = |preconditions: &mut crate::gate_report::Preconditions| {
        if let Some((uncommitted, _)) = &uncommitted {
            preconditions.fail(
                "land_records_uncommitted",
                format!("{}; {}", uncommitted.summary(), uncommitted.advice()),
                Some(uncommitted.commit_command()),
            );
        }
        if let Some(((code, cause), newer)) = &candidate_invalid {
            let (cause, fix) = crate::gate_report::policy_fix(cause, *newer);
            preconditions.fail(code, cause, Some(fix));
        }
    };
    let id = match owner(&index, &branch, requested) {
        Ok(id) => id,
        Err(error) => {
            record_failures(preconditions);
            return Err(error);
        }
    };
    preconditions.owner = id.clone();
    record_failures(preconditions);
    // The owner's branch is where its gates are read; from anywhere else
    // they cannot be, so land says where to run instead.
    let here = if branch.is_empty() {
        "detached HEAD".to_owned()
    } else {
        branch.clone()
    };
    let ownership = match &id {
        Some(id) => {
            let issue = index.require_issue(id)?;
            if issue.issue_type == "mission" {
                bail!("land_mission: a mission, and missions have no branch; land one of its items. Next: atelier work mission {id}");
            }
            match &issue.workflow_branch {
                None => match git::owning_branches(repo, id)?.first() {
                    Some(owned) => bail!(
                        "land_branch_mismatch: owned on {owned}, not on {here}. Next: git switch {owned} && atelier land"
                    ),
                    None => bail!("land_work_untraceable: {id} has no work branch. Next: atelier start {id}"),
                },
                Some(ownership)
                    if ownership.work_branch != branch || ownership.branch_base == branch =>
                {
                    let work = &ownership.work_branch;
                    if git::resolve(repo, &format!("refs/heads/{work}")).is_ok() {
                        bail!("land_branch_mismatch: owned on {work}, not on {here}. Next: git switch {work} && atelier land");
                    }
                    if base_policy::working(repo)
                        .ok()
                        .and_then(|source| {
                            source
                                .policy
                                .issue_status_is_terminal(&issue.issue_type, &issue.status)
                                .ok()
                        })
                        .unwrap_or(false)
                    {
                        bail!("land_branch_mismatch: done, and its work branch {work} is gone, so there is nothing of it to land here; land this checkout as it is. Next: atelier land");
                    }
                    bail!("land_branch_mismatch: owned on {work}, which is not a local branch. Next: git fetch && git switch {work} && atelier land");
                }
                Some(ownership) => Some(ownership.clone()),
            }
        }
        None => None,
    };
    // The owner's branch is checked above, so its recorded base is the pin.
    let base_pin = base_pin(repo, &index, id.as_deref())?;
    let source = match (&base_pin, &uncommitted) {
        (Some((_, commit)), _) => base_policy::from_captured(&mut read, commit)?,
        // The working policy is not committed, so HEAD's is the one to use.
        (None, Some(_)) => base_policy::from_captured(&mut read, &head)?,
        (None, None) => base_policy::working(repo)?,
    };
    let (branch_base, behind) = branch_base(repo, base_pin.as_ref(), id.as_deref(), &head)?;
    let behind_note = behind.map(|(base, count)| format!(
        "Base {base} has {} this branch does not. Merge them in with `git merge {base}` and record fresh proof before atelier merge; a clean merge keeps an approval unless it changes an in-scope issue record beyond its status, and then review.current names the merge and the record.",
        crate::human_output::count_noun(count, "commit")
    ));
    index.validate_policy(&source.policy, records_root)?;
    // The policy that does not parse is reported once, as its failed check;
    // the fallback's own notes about it would repeat it. A base policy's
    // notes are about another commit, so they stay.
    let mut lines = source
        .diagnostics
        .iter()
        .filter(|line| {
            candidate_invalid.is_none() || base_pin.is_some() || !line.starts_with("Invalid ")
        })
        .cloned()
        .collect::<Vec<_>>();
    check_traceability(repo, &index, &source.policy)?;
    let owned = id
        .as_ref()
        .map(|id| review_gate::scope(&index, id))
        .transpose()?
        .unwrap_or_default();
    // Every change that reaches the base has its own claims and review
    // records checked, so history up to a trusted point holds by induction:
    // land checks only what changed since then, and atelier doctor checks
    // everything (atelier-37uq).
    let base_name = &source.policy.branch_policy.base_branch;
    if let Some(note) = foreign_base_commits_note(repo, base_name, &owned)? {
        lines.push(note);
    }
    // Trust stops at the base's upstream, not the local base, which can hold
    // commits that no land has checked; without an upstream, nothing is
    // trusted. A work branch trusts no further than its branch base.
    let trusted = git::resolve(repo, &format!("{base_name}@{{upstream}}"))
        .ok()
        .filter(|_| !deep)
        .and_then(|tip| {
            let since = branch_base.as_deref().unwrap_or(&head);
            git::text(repo, &["merge-base", &tip, since]).ok()
        });
    match &trusted {
        Some(point) => review_gate::append_only(&mut read, point)?,
        None => review_gate::append_only_history(&mut read)?,
    }
    let touched = match &trusted {
        Some(point) => touched_claims(repo, &mut read, &index, point, &head)?,
        None => None,
    };
    // A policy change can invalidate claims that held under the old policy,
    // so every claim is checked again. The gates come from the base's policy,
    // which can have changed since the branch left it.
    let policy_unchanged = |point: &str| {
        let at = |commit: &str| {
            git_freshness::blob(
                repo,
                commit,
                Path::new(workflow_policy::WORKFLOW_POLICY_PATH),
            )
            .ok()
        };
        let trusted_policy = at(point);
        std::iter::once(head.clone())
            .chain(
                [
                    format!("refs/heads/{base_name}"),
                    format!("{base_name}@{{upstream}}"),
                ]
                .iter()
                .filter_map(|reference| git::resolve(repo, reference).ok()),
            )
            .all(|commit| at(&commit) == trusted_policy)
    };
    let claims_trusted = trusted.as_deref().filter(|point| {
        touched.is_some()
            && policy_unchanged(point)
            && lint_trusts(repo, &source.policy, point, &head)
    });
    let mut nodes = claim_nodes(
        repo,
        &mut read,
        &index,
        &source.policy,
        &owned,
        branch_base.as_deref(),
        &head,
        claims_trusted,
        &touched.unwrap_or_default(),
    )?;
    if install && (id.is_some() || branch == source.policy.branch_policy.base_branch) {
        bail!("land_install_invalid: --install is for an unowned branch that installs Atelier, not for an owner's branch or the base. Next: atelier land");
    }
    // What land found beyond its checks, after the passes, and the step a
    // passing land leads to.
    let mut situation = Vec::new();
    let mut next = None;
    if let (Some(id), Some(ownership)) = (&id, &ownership) {
        let id = id.clone();
        let issue = index.require_issue(&id)?;
        let done = source
            .policy
            .issue_status_is_terminal(&issue.issue_type, &issue.status)?;
        if done && !initial_paths.is_empty() {
            preconditions.fail(
                "land_completion_dirty",
                format!(
                    "done, but the checkout has {}: {}",
                    crate::human_output::count_noun(initial_paths.len(), "uncommitted path"),
                    crate::human_output::first_five(
                        initial_paths.iter().map(|path| path.display().to_string()),
                        ", "
                    )
                ),
                Some("commit them with `git add -A && git commit -m \"…\"`, or set them aside with `git stash push --include-untracked`".to_owned()),
            );
        }
        if source.policy.branch_policy.base_branch != ownership.branch_base {
            // The record names the base; recording the policy's base in it
            // is the fix, committed alone (atelier-9a6e). The new line is
            // the one the record would render, passed to perl as an
            // argument, so any legal branch name survives the shell and the
            // pattern (atelier-hen4).
            let (recorded, trusted) = (
                &ownership.branch_base,
                &source.policy.branch_policy.base_branch,
            );
            let path = format!(".atelier/issues/{id}.md");
            let line = index
                .issue_record(&id)
                .and_then(|record| {
                    let mut record = record.clone();
                    record.issue.workflow_branch.as_mut()?.branch_base = trusted.clone();
                    atelier_records::render_issue_record(&record).ok()
                })
                .and_then(|text| {
                    text.lines()
                        .find(|line| line.starts_with("  branch_base: "))
                        .map(str::to_owned)
                });
            let quote = |text: &str| format!("'{}'", text.replace('\'', "'\\''"));
            preconditions.fail(
                "land_base_mismatch",
                format!("recorded base {recorded} disagrees with trusted policy base {trusted}"),
                line.map(|line| {
                    format!(
                        "perl -pi -e 'BEGIN {{ $line = shift }} $done ||= s/^  branch_base: .*\\n/$line\\n/' {} {path} && git commit -m \"…\" -- {path}",
                        quote(&line)
                    )
                }),
            );
        }
        let base = branch_base
            .as_ref()
            .context("owner integration base missing")?;
        if let Some(note) = &behind_note {
            lines.push(note.clone());
        }
        nodes.extend(review_gate::evaluate_session(
            &index,
            &mut read,
            &source.policy,
            &id,
            base,
        )?);
        if done {
            next = Some(format!("Next: atelier merge {id}"));
        } else if !initial_paths.is_empty() {
            situation.push(format!(
                "Unfinished checkout: {}: {}. See `atelier status` before handing off.",
                crate::human_output::count_noun(initial_paths.len(), "uncommitted path"),
                crate::human_output::first_five(
                    initial_paths.iter().map(|path| path.display().to_string()),
                    ", "
                )
            ));
        }
    } else if branch == source.policy.branch_policy.base_branch {
        situation.push(git::check_base_commits(
            repo,
            &index,
            &source.policy,
            "HEAD",
        )?);
    } else {
        let base = &source.policy.branch_policy.base_branch;
        let label = if branch.is_empty() {
            "detached HEAD".to_owned()
        } else {
            format!("branch {branch}")
        };
        // Whether a branch installs Atelier is the caller's statement, not
        // something land infers from the branch's own, editable policy
        // (atelier-bibd).
        match tracker_changes_since(repo, base)? {
            None if install => situation.push(format!(
                "Installing Atelier: {base} has no workflow policy yet, so atelier merge cannot gate {label}. Land it through the repository's usual review as one merge commit (merge the pull request on the host and pull, or `git merge --no-ff`), then start further work on an item with `atelier start`."
            )),
            None => bail!(
                "land_install_unconfirmed: {base} has no workflow policy yet, so atelier merge cannot gate {label}. If this branch installs Atelier, rerun `atelier land --install`; otherwise give the work an owner branch with `atelier start` and its item's ID."
            ),
            Some(_) if install => bail!(
                "land_install_invalid: {base} already has a workflow policy, so {label} does not install Atelier; give the work an owner branch with `atelier start` and its item's ID"
            ),
            Some(changed) if !changed.is_empty() => {
                let listed = crate::human_output::first_five(
                    changed.iter().map(|path| path.display().to_string()),
                    ", ",
                );
                // The item whose record the branch changes is the one to own it.
                let item = changed.iter().find_map(|path| {
                    let id = path.strip_prefix(".atelier/issues").ok()?.file_stem()?.to_str()?;
                    index
                        .get_issue(id)
                        .filter(|issue| issue.issue_type != "mission")
                        .map(|_| id.to_owned())
                });
                // An item a local branch owns lands there.
                let owned = match &item {
                    Some(item) => git::owning_branches(repo, item)?
                        .into_iter()
                        .find(|owner| *owner != branch)
                        .map(|owner| (item.clone(), owner)),
                    None => None,
                };
                if let Some((item, owner)) = owned {
                    bail!(
                        "land_work_untraceable: {label} has no owner but changes Atelier tracker files since {base}: {listed}; {item} is owned on {owner}. Next: land it there with `git switch {owner} && atelier land`, or move the tracker changes onto {owner}"
                    );
                }
                match item {
                    Some(item) => bail!(
                        "land_work_untraceable: {label} has no owner but changes Atelier tracker files since {base}: {listed}. Next: give the work an owner branch with `atelier start {item}`, or move the tracker changes onto one"
                    ),
                    None => bail!(
                        "land_work_untraceable: {label} has no owner but changes Atelier tracker files since {base}: {listed}; give the work an owner branch with `atelier start` and its item's ID, or move the tracker changes onto one"
                    ),
                }
            }
            Some(_) => situation.push(format!("No work owner on {label}; it changes no Atelier tracker files since {base}, so there is nothing to land.")),
        }
    }
    nodes.splice(0..0, std::mem::take(&mut preconditions.failed));
    preconditions.gates_ran = true;
    let differing = uncommitted
        .as_ref()
        .map(|(records, _)| crate::gate_report::Uncommitted::new(records, repo, &index));
    let mut report = crate::gate_report::Gates {
        index: &index,
        policy: &source.policy,
        owner: id.as_deref(),
        uncommitted: differing.as_ref(),
    }
    .report(&nodes);
    if uncommitted.is_none() {
        report.passes.insert(0, "✓ records committed".to_owned());
    }
    if !report.failed.is_empty() {
        // Uncommitted code makes passing proof stale; name it (atelier-749l),
        // unless the dirty completion already does.
        let stale_proof = nodes
            .iter()
            .any(|node| !node.passed && node.stale_from_checkout);
        let dirty_named = nodes
            .iter()
            .any(|node| node.check == "land_completion_dirty");
        let dirty_note = (stale_proof && !dirty_named && !initial_paths.is_empty()).then(|| {
            format!(
                "Uncommitted changes make passing proof stale: {}. Commit or remove them, or rerun the check after committing.",
                crate::human_output::first_five(
                    initial_paths.iter().map(|path| path.display().to_string()),
                    ", "
                )
            )
        });
        let ran_on = match (&id, &ownership) {
            (Some(id), Some(_)) => crate::gate_report::RanOn::Owner(id.clone()),
            (None, _) if branch == source.policy.branch_policy.base_branch => {
                crate::gate_report::RanOn::Base
            }
            _ => crate::gate_report::RanOn::Other,
        };
        return Err(crate::gate_report::GatesRefused {
            report,
            ran_on,
            notes: lines
                .into_iter()
                .filter(|line| line.starts_with("Invalid ") || line.starts_with("Using last valid"))
                .chain(dirty_note)
                .chain(behind_note)
                .collect(),
        }
        .into());
    }
    // Arbitrary policy checks run outside the lock, as in merge. Fence their
    // result against cooperative writers and child commands that changed state.
    let _lock =
        atelier_records::mutation_lock::CanonicalMutationLock::exclusive(&layout.canonical_dir())?;
    let final_paths = check_records(repo, &snapshot)?;
    check_traceability(repo, &index, &source.policy)?;
    if base_pin.as_ref().is_some_and(|(reference, commit)| {
        git::resolve(repo, reference).map_or(true, |current| current != *commit)
    }) {
        bail!("land_base_changed: the integration base moved during checks; retry with current policy and proof");
    }
    if git_freshness::head_commit(repo)? != head
        || git::checkout_branch(repo)?.unwrap_or_default() != branch
        || final_paths != initial_paths
    {
        bail!("land_state_changed: checks changed the checkout; inspect git status, record fresh proof, and retry");
    }
    lines.extend(report.passes);
    lines.extend(situation);
    lines.extend(next);
    Ok((lines.join("\n"), head))
}

/// A hook payload the adapter cannot use, such as input that is not JSON,
/// another event's payload or a missing or unusable `cwd`: the hook is
/// misconfigured, which is no landing failure (atelier-ft88).
#[derive(Debug)]
pub(crate) struct Misconfigured(String);
impl std::fmt::Display for Misconfigured {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Atelier hook misconfigured: {}; check .claude/settings.json",
            self.0
        )
    }
}
impl std::error::Error for Misconfigured {}

/// Reads a Claude Code hook payload for `event` from stdin and enters its
/// `cwd`. Returns whether a Stop hook is already active; every failure is
/// [`Misconfigured`].
pub(crate) fn read_hook_payload(event: &str) -> Result<bool> {
    let read = || -> Result<bool> {
        let mut bytes = String::new();
        std::io::stdin().read_to_string(&mut bytes)?;
        let payload: Value = serde_json::from_str(&bytes)?;
        if payload["hook_event_name"] != event {
            bail!("expected {event} hook payload");
        }
        let cwd = payload["cwd"]
            .as_str()
            .context("hook payload requires cwd")?;
        std::env::set_current_dir(cwd).with_context(|| format!("hook payload cwd {cwd}"))?;
        Ok(payload["stop_hook_active"].as_bool().unwrap_or(false))
    };
    read().map_err(|error| Misconfigured(format!("{error:#}")).into())
}

/// The error `land` has already reported with its verdict line; `main` exits
/// non-zero without printing it again.
#[derive(Debug)]
pub struct Reported;
impl std::fmt::Display for Reported {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("land refused")
    }
}
impl std::error::Error for Reported {}

/// What a refused land reports, and its verdict: failed gates as their own
/// lines, each with its fix, then the passes and notes; a refusal by a check
/// that stops land before its gates as an error with its one code.
fn refusal(error: &anyhow::Error) -> (Vec<String>, String) {
    let (lines, failed) = match error.downcast_ref::<crate::gate_report::GatesRefused>() {
        Some(refused) => (
            refused
                .report
                .failures
                .iter()
                .chain(&refused.report.passes)
                .chain(&refused.notes)
                .cloned()
                .collect(),
            refused.report.failed.clone(),
        ),
        // The reinstall is the one fix; the reads that met it add nothing.
        None => match newer_schema(error) {
            Some(newer) => (vec![format!("Error: {newer}")], Vec::new()),
            None => (
                vec![format!("Error: {error:?}")],
                refusal_code(error).into_iter().collect(),
            ),
        },
    };
    let verdict = if failed.is_empty() {
        "land: refused".to_owned()
    } else {
        crate::gate_report::verdict("land", &failed)
    };
    (lines, verdict)
}

/// The record a newer Atelier wrote, when that is why `error` happened.
pub fn newer_schema(error: &anyhow::Error) -> Option<&atelier_core::NewerSchema> {
    error
        .chain()
        .find_map(|cause| cause.downcast_ref::<atelier_core::NewerSchema>())
}

/// The leading `land_*` check code of a refusal, if it has one.
fn refusal_code(error: &anyhow::Error) -> Option<String> {
    let message = error.to_string();
    let (code, _) = message.split_once(':')?;
    (is_land_code(code) && code.chars().all(|c| c.is_ascii_lowercase() || c == '_'))
        .then(|| code.to_owned())
}

/// Refuses pushing the base branch itself when it has an upstream configured
/// that does not resolve to a remote-tracking ref (deleted remote branch, or a
/// URL or path remote): the base-commit review check skips such a base, so the
/// push could publish unreviewed base commits. A work branch publishes no base
/// ref, and an unresolved base upstream makes land check more for it, so it is
/// not refused. A base with no upstream is left to that check's own note.
fn refuse_unresolved_base_upstream(repo: &Path) -> Result<()> {
    let base = atelier_workflow::load(repo)?.branch_policy.base_branch;
    if git::checkout_branch(repo)?.as_deref() != Some(base.as_str()) {
        return Ok(());
    }
    let configured = ["remote", "merge"]
        .iter()
        .all(|key| git::text(repo, &["config", "--get", &format!("branch.{base}.{key}")]).is_ok());
    if configured && git::resolve(repo, &format!("{base}@{{upstream}}")).is_err() {
        bail!("land_push_base_upstream_unresolved: {base} has an upstream that does not resolve to a remote-tracking branch (its remote branch is gone, or its remote is a URL or path), so unreviewed {base} commits cannot be checked before publishing; run `git fetch --prune` and `git branch -u <remote>/{base} {base}`, then rerun `atelier land --push`");
    }
    Ok(())
}

/// Pushes the commit land checked to the current branch's own upstream
/// branch, never another ref. Returns the destination, `<remote>/<branch>`.
fn push_checked_branch(repo: &Path, checked: &str) -> Result<String> {
    let branch = git::checkout_branch(repo)?.context(
        "land_push_detached: a detached HEAD has no branch to push; switch to the owner's branch",
    )?;
    let config = |key: &str| {
        git::text(
            repo,
            &["config", "--get", &format!("branch.{branch}.{key}")],
        )
        .ok()
    };
    let (Some(remote), Some(merge)) = (config("remote"), config("merge")) else {
        bail!("land_push_no_upstream: {branch} has no upstream; set one with `git push -u <remote> {branch}`, then rerun `atelier land --push`");
    };
    if remote == "." || merge != format!("refs/heads/{branch}") {
        bail!("land_push_upstream_mismatch: {branch} tracks {remote} {merge}, not a remote branch of its own name, so pushing could publish this work to another branch; run `git push -u <remote> {branch}` to set its own upstream, then rerun `atelier land --push`");
    }
    if git::resolve(repo, "HEAD")? != checked {
        bail!("land_state_changed: HEAD moved after the checks; nothing pushed; rerun `atelier land --push`");
    }
    let output = atelier_app::git_freshness::command(repo)
        .args(["push", &remote, &format!("{checked}:{merge}")])
        .output()?;
    if !output.status.success() {
        bail!(
            "land_push_failed: git push to {remote} refused {branch}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(format!("{remote}/{branch}"))
}

pub fn land(
    requested: Option<&str>,
    hook: bool,
    install: bool,
    push: bool,
    quiet: bool,
) -> Result<()> {
    if !hook {
        let nothing_pushed = if push { ", nothing pushed" } else { "" };
        // The checks' report prints before any push, so a warning is read
        // before the commits go out.
        let pushed = land_checked(requested, install, false).and_then(|(output, checked)| {
            // Refuse before the report, so no pass line precedes the refusal.
            if push {
                refuse_unresolved_base_upstream(StorageLayout::discover()?.repo_root())?;
            }
            if !quiet {
                println!("{output}");
            }
            if !push {
                return Ok(None);
            }
            let layout = StorageLayout::discover()?;
            push_checked_branch(layout.repo_root(), &checked).map(Some)
        });
        match pushed {
            Ok(destination) => {
                let verdict = match &destination {
                    Some(to) => format!("land: passed, pushed to {to}"),
                    None => "land: passed".to_owned(),
                };
                println!(
                    "{}",
                    if quiet {
                        verdict.trim_start_matches("land: ")
                    } else {
                        &verdict
                    }
                );
                return Ok(());
            }
            Err(error) => {
                let (lines, verdict) = refusal(&error);
                eprintln!("{}\n{verdict}{nothing_pushed}", lines.join("\n"));
                return Err(Reported.into());
            }
        }
    }
    let output = match read_hook_payload("Stop") {
        // Not a landing failure, so it does not hold the turn.
        Err(error) => json!({"systemMessage": error.to_string()}),
        Ok(repeated) => match land_text(requested, install, false) {
            Ok(_) => json!({}),
            Err(error) => stop_hook_refusal(&error, repeated),
        },
    };
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}

/// The Stop hook's answer to a refused land: the refusal as `land` prints
/// it, as the reason that blocks the turn. Two refusals leave the agent
/// nothing to do now, so they are reported without blocking: uncommitted
/// records alone on the base, and on the owner's own work branch a review
/// still pending on the owner, including a fresh approval for resolved
/// fix_now findings. A repeated stop is not blocked again, so the
/// hook cannot loop. `land` itself still refuses in every case.
fn stop_hook_refusal(error: &anyhow::Error, repeated: bool) -> Value {
    let (lines, verdict) = refusal(error);
    let report = format!("{}\n{verdict}", lines.join("\n"));
    if let Some(lead) = error
        .downcast_ref::<crate::gate_report::GatesRefused>()
        .and_then(nothing_to_do_now)
    {
        return json!({"systemMessage": format!("{lead}\n{report}")});
    }
    if repeated {
        json!({"systemMessage": format!("Atelier landing still fails. Stopping to avoid a hook loop; unfinished work must be handed off honestly.\n{report}")})
    } else {
        json!({"decision": "block", "reason": report})
    }
}

/// Why a refusal need not block a turn, read from its failed checks by
/// code and where land ran; `None` for every other refusal.
fn nothing_to_do_now(refused: &crate::gate_report::GatesRefused) -> Option<String> {
    use crate::gate_report::RanOn;
    let checks = &refused.report.checks;
    if checks.is_empty() {
        return None;
    }
    match &refused.ran_on {
        // On the base, records are committed when the work is handed off,
        // not at every turn (atelier-mp6v).
        RanOn::Base
            if checks
                .iter()
                .all(|check| check.check == "land_records_uncommitted") =>
        {
            Some("Atelier: the base has uncommitted records; land refuses until they are committed, but they do not block a turn.".to_owned())
        }
        // Only the owner's review is missing: its approval, or a fresh one
        // for fix_now findings a later commit already resolved. Any other
        // finding state, or any other failure, blocks (atelier-i80r).
        RanOn::Owner(owner)
            if checks.iter().all(|check| {
                check.record == *owner
                    && match check.check.as_str() {
                        "review.current" | "claims.reviewed" => true,
                        "findings.disposed" => {
                            !check.gaps.is_empty()
                                && check.gaps.iter().all(|gap| {
                                    matches!(gap, atelier_app::review_gate::Gap::AwaitingApproval(_))
                                })
                        }
                        _ => false,
                    }
            }) =>
        {
            Some(format!("Atelier: {owner} awaits an independent review; land refuses until it is approved, but waiting does not block a turn."))
        }
        _ => None,
    }
}

/// Lint reads every record, so under a policy that gates on it any record
/// change since `point` can affect every claim.
fn lint_trusts(
    repo: &Path,
    policy: &atelier_workflow::WorkflowPolicy,
    point: &str,
    head: &str,
) -> bool {
    !policy
        .workflows
        .values()
        .flat_map(|workflow| workflow.transitions.values())
        .flat_map(|transition| &transition.validators)
        .any(|validator| validator.builtin == "lint.none_blocking")
        || git::text(
            repo,
            &["diff", "--name-only", point, head, "--", ".atelier"],
        )
        .is_ok_and(|changed| changed.trim().is_empty())
}

/// Gate nodes for the done claims outside the owner's scope that `commit`,
/// the merge commit or the fast-forwarded branch head, can affect, judged in its records `index`. `base_head`
/// passed these checks when it reached the base, so it is the trusted point,
/// as for land; a policy change or lint gate checks every claim.
pub(crate) fn merge_claim_nodes(
    repo: &Path,
    read: &mut git_freshness::ReadContext<'_>,
    index: &RecordIndex,
    policy: &atelier_workflow::WorkflowPolicy,
    owner: &str,
    base_head: &str,
    commit: &str,
) -> Result<Vec<review_gate::GateNode>> {
    let owned = review_gate::scope(index, owner)?;
    let touched = touched_claims(repo, read, index, base_head, commit)?;
    let policy_at = |at: &str| {
        git_freshness::blob(repo, at, Path::new(workflow_policy::WORKFLOW_POLICY_PATH)).ok()
    };
    let trusted = (touched.is_some()
        && policy_at(base_head) == policy_at(commit)
        && lint_trusts(repo, policy, base_head, commit))
    .then_some(base_head);
    claim_nodes(
        repo,
        read,
        index,
        policy,
        &owned,
        Some(base_head),
        commit,
        trusted,
        &touched.unwrap_or_default(),
    )
}

/// The owner's recorded base branch, as a ref and the commit it names.
fn base_pin(
    repo: &Path,
    index: &RecordIndex,
    owner: Option<&str>,
) -> Result<Option<(String, String)>> {
    let Some(owner) = owner else {
        return Ok(None);
    };
    index
        .require_issue(owner)?
        .workflow_branch
        .as_ref()
        .map(|branch| {
            let reference = format!("refs/heads/{}", branch.branch_base);
            Ok((reference.clone(), git::resolve(repo, &reference)?))
        })
        .transpose()
}

/// A base the work lacks commits from: its name and how many.
type Behind = Option<(String, usize)>;

/// The commit land checks an owner's work against: its base's tip, or
/// where the work left a base that has moved on, with the base's name and
/// how many commits the work lacks. A base that has moved on is not the
/// branch's failure: check the work against where it left the base, and
/// say to merge the base in before integrating (atelier-qz9x).
fn branch_base(
    repo: &Path,
    base_pin: Option<&(String, String)>,
    owner: Option<&str>,
    head: &str,
) -> Result<(Option<String>, Behind)> {
    Ok(match (base_pin, owner) {
        (Some((reference, tip)), Some(id))
            if git_freshness::git_output(repo, &["merge-base", "--is-ancestor", tip, head])
                .is_err() =>
        {
            let name = reference.trim_start_matches("refs/heads/");
            let Ok(fork) = git::text(repo, &["merge-base", tip, head]) else {
                bail!(
                    "land_base_unrelated: {name} shares no history with this checkout, so the work cannot be checked against it; switch to the owner's work branch, or start the work again from {name} with `atelier start {id}`"
                );
            };
            let count = git::text(repo, &["rev-list", "--count", &format!("{head}..{tip}")])?
                .trim()
                .parse::<usize>()
                .with_context(|| format!("git rev-list --count {head}..{tip} printed no count"))?;
            (Some(fork), Some((name.to_owned(), count)))
        }
        (Some((_, tip)), _) => (Some(tip.clone()), None),
        (None, _) => (None, None),
    })
}

/// The claims land would judge from this checkout, for views that mark
/// proof as land does: the commits a done claim must already hold to be
/// judged as of its close, and the owner's scope, which its own gates judge.
pub(crate) fn checkout_claim_context(
    repo: &Path,
    index: &RecordIndex,
) -> Result<atelier_app::evidence_freshness::ClaimContext> {
    let head = git_freshness::head_commit(repo)?;
    let branch = git::checkout_branch(repo)?.unwrap_or_default();
    let id = owner(index, &branch, None)?;
    let base_pin = base_pin(repo, index, id.as_deref())?;
    let (branch_base, _) = branch_base(repo, base_pin.as_ref(), id.as_deref(), &head)?;
    let (_, mut parents) = claim_parents(repo, branch_base.as_deref(), &head)?;
    if branch_base.is_none() {
        // Without an owner, land refuses tracker changes the base does not
        // hold (land_work_untraceable) before it judges any claim, so only a
        // parent the base already contains holds a claim, as on the base
        // itself; a detached checkout of unmerged work is judged at HEAD.
        let base = workflow_policy::load(repo)?.branch_policy.base_branch;
        if let Ok(tip) = git::resolve(repo, &format!("refs/heads/{base}")) {
            parents.retain(|parent| {
                git_freshness::git_output(repo, &["merge-base", "--is-ancestor", parent, &tip])
                    .is_ok()
            });
        }
    }
    Ok(atelier_app::evidence_freshness::ClaimContext {
        parents,
        owned: id
            .map(|id| review_gate::scope(index, &id))
            .transpose()?
            .unwrap_or_default(),
    })
}

/// The commit a claim made now is checked against, and the commits a done
/// claim outside the owner's scope must already hold to keep its accepted
/// proof: the branch base when there is one, else every parent of `head`.
/// Without a branch base, a claim is retained when any parent of HEAD
/// already has it: a merge tip, such as a branch that merged the base in
/// last, brings claims in through its other parents (atelier-kqt9).
fn claim_parents(
    repo: &Path,
    branch_base: Option<&str>,
    head: &str,
) -> Result<(String, Vec<String>)> {
    let claim_base = branch_base
        .map(str::to_owned)
        .unwrap_or_else(|| git::resolve(repo, "HEAD^").unwrap_or_else(|_| head.to_owned()));
    let parents = if branch_base.is_some() {
        vec![claim_base.clone()]
    } else {
        let parents = git::text(repo, &["rev-list", "--parents", "-n", "1", head])?
            .split_whitespace()
            .skip(1)
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if parents.is_empty() {
            vec![claim_base.clone()]
        } else {
            parents
        }
    };
    Ok((claim_base, parents))
}

/// Gate nodes for the done claims outside the owner's scope. Claims unchanged
/// since `trusted` were checked when they reached it and are skipped; with no
/// trusted point every claim is checked (atelier-37uq).
#[allow(clippy::too_many_arguments)]
fn claim_nodes(
    repo: &Path,
    read: &mut git_freshness::ReadContext<'_>,
    index: &RecordIndex,
    policy: &atelier_workflow::WorkflowPolicy,
    owned: &std::collections::BTreeSet<String>,
    branch_base: Option<&str>,
    head: &str,
    trusted: Option<&str>,
    touched: &std::collections::BTreeSet<String>,
) -> Result<Vec<review_gate::GateNode>> {
    let (claim_base, claim_parents) = claim_parents(repo, branch_base, head)?;
    let mut current = std::collections::BTreeSet::new();
    let mut historical =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    for issue in index.issues().filter(|issue| !owned.contains(&issue.id)) {
        if !policy.issue_status_is_terminal(&issue.issue_type, &issue.status)? {
            continue;
        }
        let retained_at = read.claim_retained_at(index, &claim_parents, &issue.id);
        // Nothing that bears on it changed since the trusted point.
        let unchanged_since_trusted = trusted.is_some() && !touched.contains(&issue.id);
        match retained_at {
            Some(_) if unchanged_since_trusted => {}
            Some(parent) => {
                historical
                    .entry(parent.clone())
                    .or_default()
                    .insert(issue.id.clone());
            }
            None => {
                current.insert(issue.id.clone());
            }
        }
    }
    let mut nodes = Vec::new();
    for (parent, ids) in historical {
        nodes.extend(review_gate::evaluate_claims(
            index, read, policy, &parent, ids, true,
        )?);
    }
    nodes.extend(review_gate::evaluate_claims(
        index,
        read,
        policy,
        &claim_base,
        current,
        false,
    )?);
    Ok(nodes)
}

/// Issues whose claims a change since `point` can affect, though their own
/// records may not have changed: every changed issue record and the issues
/// it links as blocked, parent, child or advanced, and the targets of every changed
/// review or evidence record, each as it was at `point` and as it is now,
/// then every issue those reach through the same links. An `advances` link
/// touches only the mission at one end and stops there: a mission's claim
/// depends on the work that advances it, but no claim depends on the
/// mission's, so a change to one piece of its work does not re-check the
/// rest (atelier-ss25).
/// `None` when a record at `point` cannot be read, so every claim is checked.
fn touched_claims(
    repo: &Path,
    read: &mut git_freshness::ReadContext<'_>,
    index: &RecordIndex,
    point: &str,
    head: &str,
) -> Result<Option<std::collections::BTreeSet<String>>> {
    let changed = git::text(
        repo,
        &[
            "diff",
            "--no-renames",
            "--name-only",
            point,
            head,
            "--",
            ".atelier/issues",
            ".atelier/reviews",
            ".atelier/evidence",
        ],
    )?;
    let existed = git::text(
        repo,
        &[
            "ls-tree",
            "-r",
            "--name-only",
            point,
            "--",
            ".atelier/issues",
            ".atelier/reviews",
            ".atelier/evidence",
        ],
    )?
    .lines()
    .map(str::to_owned)
    .collect::<std::collections::BTreeSet<_>>();
    // The issues `record` links whose claims its change can affect, through
    // links the walk continues along and `advances` links it stops at. A
    // record stores only the links it declares (atelier-ihss).
    fn linked(
        record: &atelier_records::CanonicalIssueRecord,
        touched: &mut std::collections::BTreeSet<String>,
        stops: &mut std::collections::BTreeSet<String>,
    ) {
        touched.insert(record.issue.id.to_string());
        touched.extend(record.issue.parent_id.iter().map(ToString::to_string));
        touched.extend(
            record
                .relationships
                .blocks
                .iter()
                .chain(&record.relationships.children)
                .map(|link| link.id.clone()),
        );
        for link in record
            .relationships
            .relates
            .iter()
            .filter(|link| link.kind == "issue")
        {
            match link.relation_type.as_str() {
                atelier_records::BLOCKED_BY => {
                    touched.insert(link.id.clone());
                }
                "advances" => {
                    stops.insert(link.id.clone());
                }
                _ => {}
            }
        }
    }
    fn attached(
        evidence: &atelier_records::EvidenceRecord,
        touched: &mut std::collections::BTreeSet<String>,
    ) {
        touched.extend(
            evidence
                .header
                .relationships
                .attachments
                .iter()
                .filter(|link| link.kind == "issue")
                .map(|link| link.id.clone()),
        );
    }
    let mut touched = std::collections::BTreeSet::new();
    let mut stops = std::collections::BTreeSet::new();
    for path in changed.lines() {
        let Some(id) = Path::new(path).file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        let relative = Path::new(path)
            .strip_prefix(".atelier")
            .unwrap_or(Path::new(path));
        let before = existed.contains(path);
        if path.starts_with(".atelier/issues/") {
            touched.insert(id.to_owned());
            if before {
                let Ok(record) = read.issue_at(point, id) else {
                    return Ok(None);
                };
                linked(&record, &mut touched, &mut stops);
            }
            if let Some(record) = index.issue_record(id) {
                linked(record, &mut touched, &mut stops);
            }
        } else if path.starts_with(".atelier/reviews/") {
            if before {
                let Ok(record) = git_freshness::blob(repo, point, Path::new(path))
                    .and_then(|text| atelier_records::review::parse_review_file(&text, relative))
                else {
                    return Ok(None);
                };
                // A base exemption is about no issue.
                if let atelier_records::review::ReviewFile::Event(record) = record {
                    touched.insert(record.target);
                }
            }
            if let Ok(review) = index.require_review(id) {
                touched.insert(review.target.clone());
            }
        } else {
            if before {
                let Ok(record) = git_freshness::blob(repo, point, Path::new(path))
                    .and_then(|text| atelier_records::parse_evidence_record_file(&text, relative))
                else {
                    return Ok(None);
                };
                attached(&record, &mut touched);
            }
            if let Some(evidence) = index.evidence(id) {
                attached(evidence, &mut touched);
            }
        }
    }
    // The blocker and open-work checks follow whole chains: blockers of
    // blockers, blockers a parent passes to its children and children of
    // children, in either direction, so a record also reaches the records
    // that declare a link to it. A claim is checked when anything along its
    // chain changed. An `advances` link, declared on either side, touches
    // the mission at its other end and goes no further.
    let mut declared_by = std::collections::BTreeMap::<String, Vec<String>>::new();
    let mut advanced_by = std::collections::BTreeMap::<String, Vec<String>>::new();
    for record in index.issue_records() {
        for link in record
            .relationships
            .relates
            .iter()
            .filter(|link| link.kind == "issue")
        {
            let into = match link.relation_type.as_str() {
                atelier_records::BLOCKED_BY => &mut declared_by,
                "advances" => &mut advanced_by,
                _ => continue,
            };
            into.entry(link.id.clone())
                .or_default()
                .push(record.issue.id.to_string());
        }
    }
    let mut queue = touched.iter().cloned().collect::<Vec<_>>();
    while let Some(id) = queue.pop() {
        let mut next = declared_by
            .get(&id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect();
        if let Some(record) = index.issue_record(&id) {
            linked(record, &mut next, &mut stops);
        }
        stops.extend(advanced_by.get(&id).into_iter().flatten().cloned());
        queue.extend(next.into_iter().filter(|id| touched.insert(id.clone())));
    }
    touched.extend(stops.into_iter().filter(|id| {
        index
            .issue_record(id)
            .is_some_and(|record| record.issue.issue_type == "mission")
    }));
    Ok(Some(touched))
}

/// The full verification land skips: every retained claim and the whole
/// review history, for history that reached the base without the gates
/// (atelier-37uq).
pub fn doctor(quiet: bool) -> Result<()> {
    let output = land_text(None, false, true)?;
    if quiet {
        println!("passed");
    } else {
        println!("{output}");
        println!("Doctor: every done claim and the whole review history were checked.");
    }
    Ok(())
}
