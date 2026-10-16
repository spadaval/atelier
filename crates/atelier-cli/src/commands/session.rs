//! Fresh session context and read-only landing; no durable session state.
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
    let owners = index
        .issues()
        .filter(|issue| {
            issue
                .workflow_branch
                .as_ref()
                .is_some_and(|owner| owner.work_branch == branch)
        })
        .map(|issue| issue.id.clone())
        .collect::<Vec<_>>();
    match owners.as_slice() {
        [] => Ok(None),
        [id] => Ok(Some(id.clone())),
        _ => bail!(
            "land_owner_ambiguous: {branch} has several owners: {}; pass the intended issue ID",
            owners.join(", ")
        ),
    }
}
fn check_records(
    repo: &Path,
    snapshot: &atelier_app::committed_snapshot::CommittedSnapshot,
) -> Result<Vec<std::path::PathBuf>> {
    snapshot.verified_working_paths(repo)
        .map_err(|error| anyhow::anyhow!("land_records_uncommitted: {error:#}\nCommit canonical records and policy with the work they describe, then retry."))
}
fn prime_text() -> Result<String> {
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    let _lock =
        atelier_records::mutation_lock::CanonicalMutationLock::shared(&layout.canonical_dir())?;
    let index = RecordIndex::load_structural(&layout.canonical_dir())?;
    let branch = git::current_branch(repo)?.unwrap_or_else(|| "detached HEAD".into());
    let source = base_policy::working(repo)?;
    index.validate_policy(&source.policy, repo)?;
    let owner = owner(&index, &branch, None)?
        .map(|id| review_gate::scope(&index, &id).map(|scope| (id, scope)))
        .transpose()?;
    let work = atelier_app::work_classification::work_buckets_for(
        &index,
        Some(&source.policy),
        index.issues().filter(|issue| match &owner {
            Some((_, scope)) => scope.contains(&issue.id),
            None => source.policy.status_category(&issue.status) == Some("active"),
        }),
    )?;
    let head = git_freshness::head_commit(repo)?;
    let dirty = git_freshness::working_paths(repo)?;
    let mut proof = git_freshness::ReadContext::from_snapshot(repo, head.clone(), &dirty);
    let mut lines = vec![
        format!("Atelier session: {}", repo.display()),
        format!("Branch: {branch} @ {}", &head[..8]),
        format!(
            "Checkout: {}",
            if dirty.is_empty() { "clean" } else { "uncommitted changes" }
        ),
        "Guidance: read AGENTS.md, the repository strategy, and .agents/skills/agent-factory/SKILL.md. Agent Factory owns process; commit records with code. Atelier never creates commits.".into(),
    ];
    lines.extend(source.diagnostics);
    if let Some((id, scope)) = owner {
        let record = index.issue_record(&id).context("owner missing")?;
        let status = if source.policy.status_category(&record.issue.status) == Some("done") {
            git::unmerged_done_label(
                repo,
                &id,
                &record.issue.status,
                &branch,
                &source.policy.branch_policy.base_branch,
            )
        } else {
            record.issue.status.clone()
        };
        lines.push(format!("Owner: {id} [{status}] {}", record.issue.title));
        if let Some(ownership) = &record.issue.workflow_branch {
            lines.push(format!("Integration base: {}", ownership.branch_base));
        }
        let content = atelier_records::render_issue_record(record)?;
        let (_, body) =
            atelier_records::split_document(&content).context("owner has no record body")?;
        lines.push(crate::utils::truncate(body, 3000));
        for mission in index.issues().filter(|issue| issue.issue_type == "mission") {
            if atelier_app::objective_graph::mission_issue_ids(&index, &mission.id)?.contains(&id) {
                lines.push(format!(
                    "Mission: {} [{}] {}",
                    mission.id, mission.status, mission.title
                ));
                if let Some(strategy) = &mission.strategy {
                    lines.push(format!("Strategy: {}", strategy.path));
                }
            }
        }
        let human_children =
            atelier_app::work_classification::open_human_children(&index, &source.policy, &id);
        if !human_children.is_empty() {
            lines.push(format!(
                "Needs a human: {}",
                atelier_app::work_classification::human_children_note(&id, &human_children)
            ));
        }
        for row in work.active.iter().chain(&work.blocked).take(8) {
            lines.push(format!(
                "Work: {} [{}] {} — blockers: {}",
                row.id,
                row.status,
                row.title,
                row.open_blockers.join(", ")
            ));
        }
        let mut reviews = index
            .reviews()
            .filter(|review| scope.contains(&review.target))
            .collect::<Vec<_>>();
        reviews.sort_by_key(|review| {
            std::cmp::Reverse((review.header.created_at, review.header.id.clone()))
        });
        for review in reviews.into_iter().take(8) {
            lines.push(format!(
                "Review: {} for {} — {:?}: {}",
                review.header.id, review.target, review.event, review.summary
            ));
        }
        let mut evidence_records = index.evidence_records();
        evidence_records.sort_by_key(|record| {
            std::cmp::Reverse((record.data.captured_at, record.header.id.clone()))
        });
        let evidence_records = evidence_records
            .into_iter()
            .filter(|evidence| {
                index
                    .list_record_links("evidence", &evidence.header.id)
                    .is_ok_and(|links| {
                        links.iter().any(|link| {
                            scope.contains(&link.source_id) || scope.contains(&link.target_id)
                        })
                    })
            })
            .take(8)
            .collect::<Vec<_>>();
        for evidence in evidence_records {
            let targets = index
                .list_record_links("evidence", &evidence.header.id)?
                .into_iter()
                .flat_map(|link| [link.source_id, link.target_id])
                .filter(|id| scope.contains(id))
                .collect::<Vec<_>>();
            let fresh = targets.iter().all(|id| {
                index
                    .issue_record(id)
                    .is_some_and(|issue| proof.evidence_is_fresh(issue, evidence).unwrap_or(false))
            });
            lines.push(format!(
                "Evidence: {} — recorded {:?} @ {} ({})",
                evidence.header.id,
                evidence.data.result,
                &evidence.data.commit[..8.min(evidence.data.commit.len())],
                if fresh {
                    "fresh"
                } else {
                    "stale against current work"
                }
            ));
        }
        let inspect = if !index.get_subissues(&id).is_empty() {
            format!("atelier work parent {id}")
        } else {
            format!("atelier issue show {id}")
        };
        lines.push(format!("Next: {inspect}; atelier land {id}; use atelier merge {id} --check for completed-owner merge readiness."));
    } else {
        lines.push("Owner: none on this checkout. Select ready work with atelier work ready, then atelier start <id>.".into());
        for row in work.active.iter().take(8) {
            lines.push(format!("Active: {} [{}] {}", row.id, row.status, row.title));
        }
    }
    let waiting = atelier_app::work_classification::work_buckets(&index, Some(&source.policy))?
        .human
        .len();
    if waiting > 0 {
        lines.push(format!("Needs a human: {waiting} ready item(s) labelled needs-human wait for the human; agents should not start them. List them with atelier work human."));
    }
    lines.extend(
        git::branch_lines_for(
            repo,
            &layout.canonical_dir(),
            &index,
            &branch,
            &source.policy.branch_policy.base_branch,
        )?
        .into_iter()
        .take(8),
    );
    lines.push("Before handing off: record findings and honest proof, commit records with the code, then run atelier land. In-progress work may remain open.".into());
    Ok(lines.join("\n"))
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
fn land_text(requested: Option<&str>, install: bool) -> Result<String> {
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
    review_gate::append_only_history(&mut read)?;
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
    let claim_base = branch_base
        .clone()
        .unwrap_or_else(|| git::resolve(repo, "HEAD^").unwrap_or_else(|_| head.clone()));
    // Without a branch base, a claim is retained when any parent of HEAD
    // already has it: a merge tip, such as a branch that merged the base in
    // last, brings claims in through its other parents (atelier-kqt9).
    let claim_parents = if branch_base.is_some() {
        vec![claim_base.clone()]
    } else {
        let parents = git::text(repo, &["rev-list", "--parents", "-n", "1", &head])?
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
        if !source
            .policy
            .issue_status_is_terminal(&issue.issue_type, &issue.status)?
        {
            continue;
        }
        let retained_at = claim_parents.iter().find(|parent| {
            read.issue_at(parent, &issue.id).ok().is_some_and(|before| {
                index.issue_record(&issue.id).is_some_and(|after| {
                    atelier_app::evidence_freshness::completion_claim_matches(&before, after)
                })
            })
        });
        match retained_at {
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
            &index,
            &mut read,
            &source.policy,
            &parent,
            ids,
            true,
        )?);
    }
    nodes.extend(review_gate::evaluate_claims(
        &index,
        &mut read,
        &source.policy,
        &claim_base,
        current,
        false,
    )?);
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
        lines.push(format!("Owner: {id} [{}]. In-progress work can hand off; completed claims satisfy applicable gates.", issue.status));
        if !initial_paths.is_empty() {
            lines.push(format!("Unfinished checkout: {} uncommitted path(s); hand off these changes honestly. Run atelier prime for current findings and proof.", initial_paths.len()));
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

pub fn run(
    landing: bool,
    requested: Option<&str>,
    hook: bool,
    install: bool,
    quiet: bool,
) -> Result<()> {
    let mut repeated = false;
    let input = if hook {
        let mut bytes = String::new();
        std::io::stdin().read_to_string(&mut bytes)?;
        let parsed = (|| -> Result<()> {
            let payload: Value = serde_json::from_str(&bytes)?;
            let expected = if landing { "Stop" } else { "SessionStart" };
            if payload["hook_event_name"] != expected {
                bail!("expected {expected} hook payload");
            }
            repeated = payload["stop_hook_active"].as_bool().unwrap_or(false);
            std::env::set_current_dir(
                payload["cwd"]
                    .as_str()
                    .context("hook payload requires cwd")?,
            )?;
            Ok(())
        })();
        parsed
    } else {
        Ok(())
    };
    let result = input.and_then(|()| {
        if landing {
            land_text(requested, install)
        } else {
            prime_text()
        }
    });
    if hook {
        let output = match (landing, result) {
            (false, result) => {
                json!({"hookSpecificOutput": {"hookEventName": "SessionStart", "additionalContext": result.unwrap_or_else(|error| format!("Atelier context unavailable: {error:#}. Run atelier check and repair the named records."))}})
            }
            (true, Ok(_)) => json!({}),
            (true, Err(error)) if repeated => {
                json!({"systemMessage": format!("Atelier landing still fails: {error:#}. Stopping to avoid a hook loop; unfinished work must be handed off honestly.")})
            }
            (true, Err(error)) => {
                json!({"decision": "block", "reason": format!("Atelier landing failed: {error:#}. Fix the named invariant or hand off unfinished work honestly; Atelier never creates commits.")})
            }
        };
        println!("{}", serde_json::to_string(&output)?);
    } else {
        let output = result?;
        if quiet && landing {
            println!("passed");
        } else {
            println!("{output}");
        }
    }
    Ok(())
}
