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
fn check_records(repo: &Path, commit: &str) -> Result<()> {
    atelier_app::committed_snapshot::require_committed(repo, commit)
        .map_err(|error| anyhow::anyhow!("land_records_uncommitted: {error:#}\nCommit canonical records and policy with the work they describe, then retry."))
}
fn prime_text() -> Result<String> {
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    let _lock =
        atelier_records::mutation_lock::CanonicalMutationLock::shared(&layout.canonical_dir())?;
    let index = RecordIndex::load(&layout.canonical_dir())?;
    let branch = git::current_branch(repo)?.unwrap_or_else(|| "detached HEAD".into());
    let source = base_policy::working(repo)?;
    let view = atelier_app::work_classification::status_view(&index, Some(&source.policy))?;
    let head = git_freshness::head_commit(repo)?;
    let dirty = git_freshness::working_paths(repo)?;
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
    if let Some(id) = owner(&index, &branch, None)? {
        let record = index.issue_record(&id).context("owner missing")?;
        let scope = review_gate::scope(&index, &id)?;
        lines.push(format!(
            "Owner: {id} [{}] {}",
            record.issue.status, record.issue.title
        ));
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
        for row in view
            .work
            .active
            .iter()
            .chain(&view.work.blocked)
            .filter(|row| scope.contains(&row.id))
            .take(8)
        {
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
        for evidence in evidence_records
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
        {
            let targets = index
                .list_record_links("evidence", &evidence.header.id)?
                .into_iter()
                .flat_map(|link| [link.source_id, link.target_id])
                .filter(|id| scope.contains(id))
                .collect::<Vec<_>>();
            let fresh = targets.iter().all(|id| {
                index.issue_record(id).is_some_and(|issue| {
                    atelier_app::evidence_freshness::evidence_is_fresh(repo, issue, evidence)
                        .unwrap_or(false)
                })
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
        let inspect = if record.issue.issue_type == "epic" {
            format!("atelier work epic {id}")
        } else {
            format!("atelier issue show {id}")
        };
        lines.push(format!("Next: {inspect}; atelier land {id}; use atelier merge {id} --check for completed-owner merge readiness."));
    } else {
        lines.push("Owner: none on this checkout. Select ready work with atelier work ready, then atelier start <id>.".into());
        for row in view.work.active.iter().take(8) {
            lines.push(format!("Active: {} [{}] {}", row.id, row.status, row.title));
        }
    }
    lines.extend(
        git::branch_lines(repo, &layout.canonical_dir(), &index)?
            .into_iter()
            .take(8),
    );
    lines.push("Before handing off: record findings and honest proof, commit records with the code, then run atelier land. In-progress work may remain open.".into());
    Ok(lines.join("\n"))
}
fn land_text(requested: Option<&str>) -> Result<String> {
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    let (head, index) = {
        let _lock =
            atelier_records::mutation_lock::CanonicalMutationLock::shared(&layout.canonical_dir())?;
        let head = git_freshness::head_commit(repo)?;
        check_records(repo, &head)?;
        (head, RecordIndex::load_structural(&layout.canonical_dir())?)
    };
    review_gate::append_only_history(repo)?;
    let branch = git::current_branch(repo)?.unwrap_or_default();
    let initial_paths = git_freshness::working_paths(repo)?;
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
        Some((_, commit)) => base_policy::from_ref(repo, commit)?,
        None => base_policy::working(repo)?,
    };
    index.validate_policy(&source.policy, repo)?;
    let mut lines = source.diagnostics;
    let active = index
        .issues()
        .filter(|issue| {
            source.policy.status_category(&issue.status) == Some("active")
                && issue.issue_type != "mission"
        })
        .collect::<Vec<_>>();
    for issue in active {
        let work = workflow_policy::resolve_work_branch(&source.policy, &index, &issue.id)?;
        let owner = index.require_issue(&work.owner_id)?;
        if owner.workflow_branch.is_none() || git::work_ref(repo, &work.branch)?.is_none() {
            bail!("land_work_untraceable: active work {} has no recorded existing owner branch; run atelier start {} and commit its records", issue.id, issue.id);
        }
    }
    let owned = id
        .as_ref()
        .map(|id| review_gate::scope(&index, id))
        .transpose()?
        .unwrap_or_default();
    let claim_base = base_pin
        .as_ref()
        .map(|(_, pin)| pin.clone())
        .unwrap_or_else(|| git::resolve(repo, "HEAD^").unwrap_or_else(|_| head.clone()));
    let mut current = std::collections::BTreeSet::new();
    let mut historical = std::collections::BTreeSet::new();
    for issue in index.issues().filter(|issue| !owned.contains(&issue.id)) {
        if !source
            .policy
            .issue_status_is_terminal(&issue.issue_type, &issue.status)?
        {
            continue;
        }
        let path = std::path::PathBuf::from(format!(".atelier/issues/{}.md", issue.id));
        let before = git_freshness::blob(repo, &claim_base, &path)
            .ok()
            .and_then(|text| {
                atelier_records::parse_issue_record(&text, path.strip_prefix(".atelier").ok()?).ok()
            });
        if before.as_ref().is_some_and(|before| {
            index.issue_record(&issue.id).is_some_and(|after| {
                atelier_app::evidence_freshness::completion_claim_matches(before, after)
            })
        }) {
            historical.insert(issue.id.clone());
        } else {
            current.insert(issue.id.clone());
        }
    }
    let mut nodes =
        review_gate::evaluate_claims(&index, repo, &source.policy, &claim_base, historical, true)?;
    nodes.extend(review_gate::evaluate_claims(
        &index,
        repo,
        &source.policy,
        &claim_base,
        current,
        false,
    )?);
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
        let base = &base_pin
            .as_ref()
            .context("owner integration base missing")?
            .1;
        nodes.extend(review_gate::evaluate_session(
            &index,
            repo,
            &source.policy,
            &id,
            base,
        )?);
        lines.push(format!("Owner: {id} [{}]. In-progress work can hand off; completed claims satisfy applicable gates.", issue.status));
        if !initial_paths.is_empty() {
            lines.push(format!("Unfinished checkout: {} uncommitted path(s); hand off these changes honestly. Run atelier prime for current findings and proof.", initial_paths.len()));
        }
    } else {
        if branch != source.policy.branch_policy.base_branch {
            bail!("land_work_untraceable: branch {branch} has no owner; run atelier start <id> before claiming work on it");
        }
        lines.push("No work owner on the integration base.".into());
    }
    let failed = nodes
        .iter()
        .filter(|node| !node.passed)
        .map(|node| format!("{} {}: {}", node.target, node.check, node.reason))
        .collect::<Vec<_>>();
    if !failed.is_empty() {
        bail!("land_invariants_unmet:\n{}\nResolve the named checks, commit the records, then rerun atelier land.", failed.join("\n"));
    }
    // Arbitrary policy checks run outside the lock, as in merge. Fence their
    // result against cooperative writers and child commands that changed state.
    let _lock =
        atelier_records::mutation_lock::CanonicalMutationLock::exclusive(&layout.canonical_dir())?;
    check_records(repo, &head)?;
    if base_pin.as_ref().is_some_and(|(reference, commit)| {
        git::resolve(repo, reference).map_or(true, |current| current != *commit)
    }) {
        bail!("land_base_changed: the integration base moved during checks; retry with current policy and proof");
    }
    if git_freshness::head_commit(repo)? != head
        || git::current_branch(repo)?.unwrap_or_default() != branch
        || git_freshness::working_paths(repo)? != initial_paths
    {
        bail!("land_state_changed: checks changed the checkout; inspect git status, record fresh proof, and retry");
    }
    lines.push("Landing checks passed. No commits or records created.".into());
    Ok(lines.join("\n"))
}

pub fn run(landing: bool, requested: Option<&str>, hook: bool, quiet: bool) -> Result<()> {
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
            land_text(requested)
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
