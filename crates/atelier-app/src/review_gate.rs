//! Review integrity, semantic freshness and derived merge gate graph nodes.
use crate::{git_freshness as git, record_index::RecordIndex, workflow_policy::WorkflowPolicy};
use anyhow::{bail, Context, Result};
use atelier_core::{FindingDecision, ReviewEvent, ReviewRecord};
use atelier_records::{parse_issue_record, split_document};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

#[derive(Debug, Serialize)]
pub struct GateNode {
    pub kind: &'static str,
    pub id: String,
    pub target: String,
    pub check: String,
    pub passed: bool,
    pub reason: String,
    pub supports: Vec<String>,
}
fn gate(owner: &str, check: &str, passed: bool, reason: String, supports: Vec<String>) -> GateNode {
    GateNode {
        kind: "workflow_validator",
        id: format!("{owner}/{check}"),
        target: owner.into(),
        check: check.into(),
        passed,
        reason,
        supports,
    }
}
pub fn scope(index: &RecordIndex, owner: &str) -> Result<BTreeSet<String>> {
    let mut ids = crate::objective_graph::issue_descendant_ids(index, owner)?;
    ids.insert(owner.to_owned());
    Ok(ids)
}

fn entry(repo: &Path, commit: &str, path: &Path) -> Result<Option<(String, String)>> {
    let output = git::git_output(
        repo,
        &[
            "ls-tree",
            "-z",
            commit,
            "--",
            path.to_str().context("record path is not UTF-8")?,
        ],
    )?;
    if output.is_empty() {
        return Ok(None);
    }
    let text = std::str::from_utf8(&output)?;
    let mut fields = text.split_whitespace();
    let mode = fields.next().context("missing Git entry mode")?.to_owned();
    if fields.next() != Some("blob") {
        bail!("expected blob entry for {}", path.display());
    }
    let hash = fields.next().context("missing Git blob ID")?.to_owned();
    Ok(Some((mode, hash)))
}

fn issue_status_only(before: &str, after: &str, path: &Path) -> Result<bool> {
    let relative = path.strip_prefix(".atelier")?;
    let mut left = parse_issue_record(before, relative)?;
    let mut right = parse_issue_record(after, relative)?;
    // Preserve unknown front-matter changes too: only this explicit whitelist
    // can be ignored, even when an issue parser has no use for another key.
    let front = |text: &str| -> Result<serde_yaml::Value> {
        let (yaml, _) = split_document(text).context("missing issue front matter")?;
        let mut value: serde_yaml::Value = serde_yaml::from_str(yaml)?;
        let map = value
            .as_mapping_mut()
            .context("issue front matter is not a mapping")?;
        for key in ["status", "updated_at", "closed_at"] {
            map.remove(serde_yaml::Value::String(key.into()));
        }
        Ok(value)
    };
    if front(before)? != front(after)? {
        return Ok(false);
    }
    left.sections.close_reason = None;
    right.sections.close_reason = None;
    Ok(left.sections == right.sections)
}

fn allowed_edge(repo: &Path, edge: &git::CommitEdge, ids: &BTreeSet<String>) -> Result<bool> {
    for path in &edge.paths {
        let old = entry(repo, &edge.parent, path)?;
        let new = entry(repo, &edge.commit, path)?;
        let regular = |entry: &Option<(String, String)>| {
            entry.as_ref().is_some_and(|(mode, _)| mode == "100644")
        };
        if path.extension().is_some_and(|ext| ext == "md")
            && (path.starts_with(".atelier/reviews") || path.starts_with(".atelier/evidence"))
        {
            if old.is_none() && regular(&new) {
                continue;
            }
            return Ok(false);
        }
        if path.starts_with(".atelier/issues") && regular(&old) && regular(&new) {
            let id = path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            if ids.contains(id)
                && issue_status_only(
                    &git::blob(repo, &edge.parent, path)?,
                    &git::blob(repo, &edge.commit, path)?,
                    path,
                )?
            {
                continue;
            }
        }
        return Ok(false);
    }
    Ok(true)
}

pub fn approval_is_current(repo: &Path, pin: &str, ids: &BTreeSet<String>) -> Result<bool> {
    for edge in git::commit_edges(repo, pin)? {
        if !allowed_edge(repo, &edge, ids)? {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Enforce immutability from branch creation, not merely after approval.
pub fn append_only(repo: &Path, base_commit: &str) -> Result<()> {
    for edge in git::commit_edges(repo, base_commit)? {
        for path in edge
            .paths
            .iter()
            .filter(|path| path.starts_with(".atelier/reviews"))
        {
            if entry(repo, &edge.parent, path)?.is_some()
                || entry(repo, &edge.commit, path)?.is_none_or(|(mode, _)| mode != "100644")
            {
                bail!("review_append_only: {} was rewritten, deleted or changed mode in {}; append a successor event", path.display(), edge.commit);
            }
        }
    }
    Ok(())
}

pub fn author_identities(
    repo: &Path,
    base_commit: &str,
    ids: &BTreeSet<String>,
) -> Result<BTreeSet<String>> {
    let mut authors = BTreeSet::new();
    let mut inspected = BTreeSet::new();
    for edge in git::commit_edges(repo, base_commit)? {
        if !allowed_edge(repo, &edge, ids)? && inspected.insert(edge.commit.clone()) {
            let names = String::from_utf8(git::git_output(
                repo,
                &["show", "-s", "--format=%an%n%ae", &edge.commit],
            )?)?;
            for identity in names.lines() {
                authors.insert(normalize(identity));
            }
            let parts = names.lines().collect::<Vec<_>>();
            if parts.len() == 2 {
                authors.insert(normalize(&format!("{} <{}>", parts[0], parts[1])));
            }
        }
    }
    Ok(authors)
}
fn normalize(identity: &str) -> String {
    identity.trim().to_lowercase()
}
fn independent(record: &ReviewRecord, authors: &BTreeSet<String>) -> bool {
    !authors.contains(&normalize(&record.identity))
}
fn ancestor(repo: &Path, earlier: &str, later: &str) -> bool {
    git::git_output(repo, &["merge-base", "--is-ancestor", earlier, later]).is_ok()
}

pub fn evaluate(
    index: &RecordIndex,
    repo: &Path,
    policy: &WorkflowPolicy,
    owner: &str,
    base_commit: &str,
) -> Result<Vec<GateNode>> {
    let ids = scope(index, owner)?;
    let authors = author_identities(repo, base_commit, &ids)?;
    for record in index
        .reviews()
        .filter(|record| ids.contains(&record.target))
    {
        if !ancestor(repo, &record.commit, "HEAD") {
            bail!(
                "review {} pins a commit outside the work branch",
                record.header.id
            );
        }
        if let ReviewEvent::Disposition {
            finding,
            resolution_commit: Some(resolution),
            ..
        } = &record.event
        {
            let original = index.require_review(finding)?;
            if resolution == &original.commit
                || !ancestor(repo, &original.commit, resolution)
                || !ancestor(repo, resolution, "HEAD")
            {
                bail!(
                    "disposition {} resolution is not a later commit on the work branch",
                    record.header.id
                );
            }
        }
    }
    let terminal = ids
        .iter()
        .filter_map(|id| index.get_issue(id))
        .filter(|issue| {
            !policy
                .issue_status_is_terminal(&issue.issue_type, &issue.status)
                .unwrap_or(false)
        })
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    let mut nodes = vec![gate(
        owner,
        "work.terminal",
        terminal.is_empty(),
        if terminal.is_empty() {
            "owner and all scoped work are terminal".into()
        } else {
            format!("open scoped work: {}", terminal.join(", "))
        },
        Vec::new(),
    )];
    let mut open = BTreeSet::new();
    for id in &ids {
        open.extend(crate::objective_graph::dependency_closure(index, policy, id)?.blocking_ids());
    }
    nodes.push(gate(
        owner,
        "blockers.none_open",
        open.is_empty(),
        if open.is_empty() {
            "no open transitive blockers".into()
        } else {
            format!(
                "open blockers: {}",
                open.into_iter().collect::<Vec<_>>().join(", ")
            )
        },
        Vec::new(),
    ));
    let candidates = index
        .reviews()
        .filter(|record| record.target == owner)
        .filter(|record| {
            matches!(
                record.event,
                ReviewEvent::Approval | ReviewEvent::TrivialExemption
            )
        })
        .filter(|record| {
            (matches!(record.event, ReviewEvent::TrivialExemption) || independent(record, &authors))
                && ancestor(repo, base_commit, &record.commit)
                && approval_is_current(repo, &record.commit, &ids).unwrap_or(false)
        })
        .collect::<Vec<_>>();
    let support = candidates
        .iter()
        .map(|record| record.header.id.clone())
        .collect::<Vec<_>>();
    nodes.push(gate(
        owner,
        "review.current",
        !candidates.is_empty(),
        if candidates.is_empty() {
            format!(
                "no current independent approval or explicit trivial exemption; authors: {}",
                authors.iter().cloned().collect::<Vec<_>>().join(", ")
            )
        } else if candidates
            .iter()
            .any(|record| matches!(record.event, ReviewEvent::TrivialExemption))
        {
            "EXPLICIT TRIVIAL EXEMPTION recorded with explanation".into()
        } else {
            "current independent approval recorded".into()
        },
        support,
    ));
    let mut unresolved = Vec::new();
    let mut dispositions = Vec::new();
    for finding in index.reviews().filter(|record| {
        ids.contains(&record.target) && matches!(record.event, ReviewEvent::Finding { .. })
    }) {
        let ReviewEvent::Finding { .. } = &finding.event else {
            unreachable!()
        };
        let Some(disposition) = index.latest_disposition(&finding.header.id) else {
            unresolved.push(format!("{} has no disposition", finding.header.id));
            continue;
        };
        dispositions.push(disposition.header.id.clone());
        let ReviewEvent::Disposition {
            decision,
            resolution_commit,
            ..
        } = &disposition.event
        else {
            unreachable!()
        };
        let accepted = match decision {
            FindingDecision::FixNow => resolution_commit.as_ref().is_some_and(|resolution| {
                ancestor(repo, resolution, "HEAD")
                    && candidates.iter().any(|approval| {
                        matches!(approval.event, ReviewEvent::Approval)
                            && ancestor(repo, resolution, &approval.commit)
                    })
            }),
            FindingDecision::Defer | FindingDecision::NoAction => {
                independent(disposition, &authors)
            }
        };
        if !accepted {
            let reason = if *decision == FindingDecision::FixNow {
                "unresolved FIX NOW: resolution needs fresh independent approval"
            } else {
                "disposition needs independent non-author acceptance"
            };
            unresolved.push(format!("{}: {reason}", finding.header.id));
        }
    }
    nodes.push(gate(
        owner,
        "findings.disposed",
        unresolved.is_empty(),
        if unresolved.is_empty() {
            "every finding has an accepted disposition".into()
        } else {
            unresolved.join("; ")
        },
        dispositions,
    ));
    for id in &ids {
        if index.require_issue(id)?.status == "superseded" {
            continue;
        }
        if let Some(failure) = crate::evidence_freshness::newest_failure(index, repo, id)? {
            nodes.push(gate(
                id,
                "evidence.no_newer_failure",
                false,
                format!("newer failing evidence {failure}"),
                vec![failure],
            ));
        }
    }
    // Terminal records can also be edited directly; retain the trusted
    // policy's completion checks when evaluating their committed claims.
    for id in &ids {
        let issue = index.require_issue(id)?;
        if !policy.issue_status_is_terminal(&issue.issue_type, &issue.status)? {
            continue;
        }
        for transition in policy
            .workflow_for_issue_type(&issue.issue_type)?
            .transitions
            .values()
            .filter(|transition| transition.to == issue.status)
        {
            let checks = crate::workflow_validation::evaluate_policy_transition(
                crate::workflow_validation::ValidatorRequest {
                    index,
                    repo_root: repo,
                    policy,
                    issue_id: id,
                    validators: &transition.validators,
                },
            )?;
            nodes.extend(
                checks.into_iter().map(|check| {
                    gate(id, &check.validator, check.passed, check.reason, Vec::new())
                }),
            );
        }
    }
    let checks = crate::workflow_validation::evaluate_policy_transition(
        crate::workflow_validation::ValidatorRequest {
            index,
            repo_root: repo,
            policy,
            issue_id: owner,
            validators: &policy.merge_checks,
        },
    )?;
    nodes.extend(checks.into_iter().map(|check| {
        gate(
            owner,
            &check.validator,
            check.passed,
            check.reason,
            Vec::new(),
        )
    }));
    let mut unique: BTreeMap<String, GateNode> = BTreeMap::new();
    for node in nodes {
        if let Some(existing) = unique.get_mut(&node.id) {
            existing.passed &= node.passed;
            if existing.reason != node.reason {
                existing.reason.push_str(&format!("; {}", node.reason));
            }
            existing.supports.extend(node.supports);
            existing.supports.sort();
            existing.supports.dedup();
        } else {
            unique.insert(node.id.clone(), node);
        }
    }
    Ok(unique.into_values().collect())
}
