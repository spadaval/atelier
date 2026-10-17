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
        let id = index
            .resolve_issue_ref(reference)?
            .context("landing issue not found")?;
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
    snapshot.verified_working_paths(repo)
        .map_err(|error| anyhow::anyhow!("land_records_uncommitted: {error:#}\nCommit canonical records and policy with the work they describe, then retry."))
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
            bail!("land_work_untraceable: active work {} has no recorded existing owner branch; run atelier start {} and commit its records", issue.id, issue.id);
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
        bail!("land_work_untraceable: base branch {base} not found, so this unowned branch's tracker changes cannot be checked; fetch {base} or run atelier start <id>");
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
        format!("land_work_untraceable: this unowned branch shares no history with {base}, so its tracker changes cannot be checked; run atelier start <id> from {base}")
    })
}
fn land_text(requested: Option<&str>, install: bool, deep: bool) -> Result<String> {
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    let (head, branch, snapshot, initial_paths, index) = {
        let _lock =
            atelier_records::mutation_lock::CanonicalMutationLock::shared(&layout.canonical_dir())?;
        let head = git_freshness::head_commit(repo)?;
        let branch = git::checkout_branch(repo)?.unwrap_or_default();
        let snapshot = atelier_app::committed_snapshot::CommittedSnapshot::new(&head)?;
        let (files, paths) = snapshot.read_record_files(repo, false).map_err(|error| {
            anyhow::anyhow!("land_records_uncommitted: {error:#}\nCommit canonical records and policy with the work they describe, then retry.")
        })?;
        git::require_committed_config(repo, &head, "land")?;
        (head, branch, snapshot, paths, files.into_index()?)
    };
    let mut read = git_freshness::ReadContext::from_snapshot(repo, head.clone(), &initial_paths);
    atelier_workflow::load(repo)
        .context("candidate workflow policy is invalid; repair it before landing")?;
    let id = owner(&index, &branch, requested)?;
    let base_pin = id
        .as_ref()
        .map(|id| -> Result<Option<(String, String)>> {
            index
                .require_issue(id)?
                .workflow_branch
                .as_ref()
                .map(|owner| {
                    let reference = format!("refs/heads/{}", owner.branch_base);
                    Ok((reference.clone(), git::resolve(repo, &reference)?))
                })
                .transpose()
        })
        .transpose()?
        .flatten();
    let source = match &base_pin {
        Some((_, commit)) => base_policy::from_captured(&mut read, commit)?,
        None => base_policy::working(repo)?,
    };
    // A base that has moved on is not the branch's failure: check the work
    // against where it left the base, and say to merge the base in before
    // integrating (atelier-qz9x).
    let mut behind = None;
    let branch_base = match &base_pin {
        Some((reference, tip))
            if git_freshness::git_output(repo, &["merge-base", "--is-ancestor", tip, &head])
                .is_err() =>
        {
            let name = reference.trim_start_matches("refs/heads/");
            let Ok(fork) = git::text(repo, &["merge-base", tip, &head]) else {
                bail!(
                    "land_base_unrelated: {name} shares no history with this checkout, so the work cannot be checked against it; switch to the owner's work branch, or start the work again from {name} with atelier start {}",
                    id.as_deref().unwrap_or("<id>")
                );
            };
            let count = git::text(repo, &["rev-list", "--count", &format!("{head}..{tip}")])?;
            behind = Some((name.to_owned(), count));
            Some(fork)
        }
        Some((_, tip)) => Some(tip.clone()),
        None => None,
    };
    let behind_note = behind.map(|(base, count)| format!(
        "Base {base} has {count} commit(s) this branch does not. Merge them in with `git merge {base}` and record fresh proof before atelier merge; a clean merge keeps an approval."
    ));
    index.validate_policy(&source.policy, repo)?;
    let mut lines = source.diagnostics;
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
    // Lint reads every record, so under a policy that gates on it any record
    // change can affect every claim.
    let lints = source
        .policy
        .workflows
        .values()
        .flat_map(|workflow| workflow.transitions.values())
        .flat_map(|transition| &transition.validators)
        .any(|validator| validator.builtin == "lint.none_blocking");
    let records_unchanged = |point: &str| {
        git::text(
            repo,
            &["diff", "--name-only", point, &head, "--", ".atelier"],
        )
        .is_ok_and(|changed| changed.trim().is_empty())
    };
    let claims_trusted = trusted.as_deref().filter(|point| {
        touched.is_some() && policy_unchanged(point) && (!lints || records_unchanged(point))
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
        bail!("land_install_invalid: --install is for an unowned branch that installs Atelier, not for an owner's branch or the base; rerun atelier land without --install");
    }
    if let Some(id) = id {
        let issue = index.require_issue(&id)?;
        let ownership = issue.workflow_branch.as_ref().context("land_work_untraceable: owner has no committed branch metadata; run atelier start and commit the record")?;
        if ownership.work_branch != branch || ownership.branch_base == branch {
            bail!(
                "land_branch_mismatch: owner {id} requires {} with distinct base {}",
                ownership.work_branch,
                ownership.branch_base
            );
        }
        if source
            .policy
            .issue_status_is_terminal(&issue.issue_type, &issue.status)?
            && !initial_paths.is_empty()
        {
            bail!("land_completion_dirty: a completed owner requires a clean checkout for current proof/review; commit or hand off the work as active");
        }
        if source.policy.branch_policy.base_branch != ownership.branch_base {
            bail!(
                "land_base_mismatch: recorded base {} disagrees with trusted policy base {}",
                ownership.branch_base,
                source.policy.branch_policy.base_branch
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
        lines.push(format!("Owner: {id} {}. In-progress work can hand off; completed claims satisfy applicable gates.", issue.status));
        if !initial_paths.is_empty() {
            lines.push(format!("Unfinished checkout: {} uncommitted path(s); hand off these changes honestly. Run atelier status for current findings and proof.", initial_paths.len()));
        }
    } else if branch == source.policy.branch_policy.base_branch {
        lines.push("No work owner on the integration base.".into());
        lines.push(git::check_base_commits(
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
            None if install => lines.push(format!(
                "Installing Atelier: {base} has no workflow policy yet, so atelier merge cannot gate {label}. Land it through the repository's usual review as one merge commit (merge the pull request on the host and pull, or git merge --no-ff), then start further work with atelier start <id>."
            )),
            None => bail!(
                "land_install_unconfirmed: {base} has no workflow policy yet, so atelier merge cannot gate {label}. If this branch installs Atelier, rerun atelier land --install; otherwise run atelier start <id> to give the work an owner branch."
            ),
            Some(_) if install => bail!(
                "land_install_invalid: {base} already has a workflow policy, so {label} does not install Atelier; run atelier start <id> to give the work an owner branch"
            ),
            Some(changed) if !changed.is_empty() => {
                const SHOWN: usize = 5;
                let mut listed = changed
                    .iter()
                    .take(SHOWN)
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                if changed.len() > SHOWN {
                    listed.push_str(&format!(" and {} more", changed.len() - SHOWN));
                }
                bail!(
                    "land_work_untraceable: {label} has no owner but changes Atelier tracker files since {base}: {listed}; run atelier start <id> to give the work an owner branch, or move the tracker changes onto one"
                );
            }
            Some(_) => lines.push(format!("No work owner on {label}; it changes no Atelier tracker files since {base}, so there is nothing to land.")),
        }
    }
    let failed = nodes
        .iter()
        .filter(|node| !node.passed)
        .map(|node| format!("{} {}: {}", node.target, node.check, node.reason))
        .collect::<Vec<_>>();
    if !failed.is_empty() {
        // Uncommitted code makes passing proof stale; name it (atelier-749l).
        let stale_proof = nodes
            .iter()
            .any(|node| !node.passed && node.stale_from_checkout);
        let dirty_note = (stale_proof && !initial_paths.is_empty()).then(|| {
            const SHOWN: usize = 5;
            let mut listed = initial_paths
                .iter()
                .take(SHOWN)
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            if initial_paths.len() > SHOWN {
                listed.push_str(&format!(" and {} more", initial_paths.len() - SHOWN));
            }
            format!("Uncommitted changes make passing proof stale: {listed}. Commit or remove them, or rerun the check after committing.")
        });
        bail!(
            "land_invariants_unmet:\n{}\nResolve the named checks, commit the records, then rerun atelier land.{}",
            failed.join("\n"),
            [dirty_note, behind_note]
                .into_iter()
                .flatten()
                .map(|note| format!("\n{note}"))
                .collect::<String>()
        );
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
    lines.push("Landing checks passed. No commits or records created.".into());
    Ok(lines.join("\n"))
}

/// Reads a Claude Code hook payload for `event` from stdin and enters its
/// `cwd`. Returns whether a Stop hook is already active.
pub(crate) fn read_hook_payload(event: &str) -> Result<bool> {
    let mut bytes = String::new();
    std::io::stdin().read_to_string(&mut bytes)?;
    let payload: Value = serde_json::from_str(&bytes)?;
    if payload["hook_event_name"] != event {
        bail!("expected {event} hook payload");
    }
    std::env::set_current_dir(
        payload["cwd"]
            .as_str()
            .context("hook payload requires cwd")?,
    )?;
    Ok(payload["stop_hook_active"].as_bool().unwrap_or(false))
}

pub fn land(requested: Option<&str>, hook: bool, install: bool, quiet: bool) -> Result<()> {
    if !hook {
        let output = land_text(requested, install, false)?;
        println!("{}", if quiet { "passed" } else { &output });
        return Ok(());
    }
    let mut repeated = false;
    let result = read_hook_payload("Stop").and_then(|active| {
        repeated = active;
        land_text(requested, install, false)
    });
    let output = match result {
        Ok(_) => json!({}),
        Err(error) if repeated => {
            json!({"systemMessage": format!("Atelier landing still fails: {error:#}. Stopping to avoid a hook loop; unfinished work must be handed off honestly.")})
        }
        Err(error) => {
            json!({"decision": "block", "reason": format!("Atelier landing failed: {error:#}. Fix the named invariant or hand off unfinished work honestly; Atelier never creates commits.")})
        }
    };
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
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
    let claim_base = branch_base
        .map(str::to_owned)
        .unwrap_or_else(|| git::resolve(repo, "HEAD^").unwrap_or_else(|_| head.to_owned()));
    // Without a branch base, a claim is retained when any parent of HEAD
    // already has it: a merge tip, such as a branch that merged the base in
    // last, brings claims in through its other parents (atelier-kqt9).
    let claim_parents = if branch_base.is_some() {
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
    let mut current = std::collections::BTreeSet::new();
    let mut historical =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    for issue in index.issues().filter(|issue| !owned.contains(&issue.id)) {
        if !policy.issue_status_is_terminal(&issue.issue_type, &issue.status)? {
            continue;
        }
        let retained_at = claim_parents.iter().find(|parent| {
            read.issue_at(parent, &issue.id).ok().is_some_and(|before| {
                index.issue_record(&issue.id).is_some_and(|after| {
                    atelier_app::evidence_freshness::completion_claim_matches(&before, after)
                })
            })
        });
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
/// then every issue those reach through the same links.
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
    fn linked(
        record: &atelier_records::CanonicalIssueRecord,
        touched: &mut std::collections::BTreeSet<String>,
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
        touched.extend(
            record
                .relationships
                .relates
                .iter()
                .filter(|link| link.kind == "issue" && link.relation_type == "advances")
                .map(|link| link.id.clone()),
        );
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
                linked(&record, &mut touched);
            }
            if let Some(record) = index.issue_record(id) {
                linked(record, &mut touched);
            }
        } else if path.starts_with(".atelier/reviews/") {
            if before {
                let Ok(record) = git_freshness::blob(repo, point, Path::new(path))
                    .and_then(|text| atelier_records::review::parse_review_record(&text, relative))
                else {
                    return Ok(None);
                };
                touched.insert(record.target);
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
    // blockers, blockers a parent passes to its children, children of
    // children, and work that advances a mission, in either direction. A
    // claim is checked when anything along its chain changed.
    let mut advanced_by = std::collections::BTreeMap::<String, Vec<String>>::new();
    for record in index.issue_records() {
        for link in &record.relationships.relates {
            if link.kind == "issue" && link.relation_type == "advances" {
                advanced_by
                    .entry(link.id.clone())
                    .or_default()
                    .push(record.issue.id.to_string());
            }
        }
    }
    let mut queue = touched.iter().cloned().collect::<Vec<_>>();
    while let Some(id) = queue.pop() {
        let mut next = advanced_by
            .get(&id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect();
        if let Some(record) = index.issue_record(&id) {
            linked(record, &mut next);
        }
        queue.extend(next.into_iter().filter(|id| touched.insert(id.clone())));
    }
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
