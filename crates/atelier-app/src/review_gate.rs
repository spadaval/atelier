//! Review integrity, semantic freshness and derived merge gate graph nodes.
use crate::{git_freshness as git, record_index::RecordIndex, workflow_policy::WorkflowPolicy};
use anyhow::{bail, Context, Result};
use atelier_core::{FindingDecision, ReviewEvent, ReviewRecord};
use atelier_records::{parse_issue_record, split_document};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
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

fn issue_status_only(before: &str, after: &str, path: &Path) -> Result<bool> {
    let relative = path.strip_prefix(".atelier")?;
    if parse_issue_record(before, relative).is_err() {
        // An exact `atelier migrate epics` rewrite of an approved epic changes
        // its shape, not its content, so it does not author the record.
        if let Ok(record) = parse_issue_record(after, relative) {
            if let Ok(Some(migrated)) = crate::epic_migration::migrate_epic_text(
                before,
                &record.issue.issue_type,
                &record.issue.status,
            ) {
                if parse_issue_record(&migrated.text, relative).is_ok() {
                    return issue_status_only(&migrated.text, after, path);
                }
            }
        }
    }
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
    match (
        parse_issue_record(before, relative),
        parse_issue_record(after, relative),
    ) {
        (Ok(mut left), Ok(mut right)) => {
            left.sections.close_reason = None;
            right.sections.close_reason = None;
            Ok(left.sections == right.sections)
        }
        // History can hold records in a retired shape, such as pre-migration
        // epics. Compare their bodies as text, ignoring only Close Reason.
        _ => Ok(body_without_close_reason(before) == body_without_close_reason(after)),
    }
}

fn body_without_close_reason(text: &str) -> Option<String> {
    let (_, body) = split_document(text)?;
    let mut kept = String::new();
    let mut skipping = false;
    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            skipping = heading.trim() == "Close Reason";
        }
        if !skipping {
            kept.push_str(line.trim_end());
            kept.push('\n');
        }
    }
    Some(kept.trim_end().to_owned())
}

fn allowed_edge(
    read: &mut git::ReadContext<'_>,
    edge: &git::CommitEdge,
    ids: &BTreeSet<String>,
) -> Result<bool> {
    for path in &edge.paths {
        let old = read.entry(&edge.parent, path)?;
        let new = read.entry(&edge.commit, path)?;
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
        let issue_id = (path.parent() == Some(Path::new(".atelier/issues"))
            && path.extension().is_some_and(|ext| ext == "md"))
        .then(|| path.file_stem().and_then(|name| name.to_str()))
        .flatten();
        // A new issue outside the scope, such as the follow-up a deferred
        // finding needs, is planning, not reviewed work (atelier-cafy).
        if issue_id.is_some_and(|id| !ids.contains(id)) && old.is_none() && regular(&new) {
            continue;
        }
        if path.starts_with(".atelier/issues") && regular(&old) && regular(&new) {
            let id = path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            let (Some((_, before)), Some((_, after))) = (&old, &new) else {
                unreachable!("regular entries exist")
            };
            if ids.contains(id)
                && read.status_only(before, after, path, |read| {
                    issue_status_only(
                        &read.blob(&edge.parent, path)?,
                        &read.blob(&edge.commit, path)?,
                        path,
                    )
                })?
            {
                continue;
            }
        }
        return Ok(false);
    }
    Ok(true)
}

pub fn approval_is_current(
    read: &mut git::ReadContext<'_>,
    pin: &str,
    ids: &BTreeSet<String>,
    base: &str,
) -> Result<bool> {
    approval_is_current_to(read, pin, ids, "HEAD", base)
}
fn approval_is_current_to(
    read: &mut git::ReadContext<'_>,
    pin: &str,
    ids: &BTreeSet<String>,
    end: &str,
    base: &str,
) -> Result<bool> {
    for edge in work_edges(read, pin, end, base)? {
        if !allowed_edge(read, &edge, ids)? {
            return Ok(false);
        }
    }
    Ok(true)
}
/// The edges from `pin` to `end` that can change approved work. A clean merge
/// of commits already on `base` brings in work reviewed on its own branch, so
/// the merge and the base commits it brings in are left out (atelier-n7g1).
fn work_edges(
    read: &mut git::ReadContext<'_>,
    pin: &str,
    end: &str,
    base: &str,
) -> Result<Vec<git::CommitEdge>> {
    let edges = read.edges(pin, end)?;
    let mut merged_base = HashSet::new();
    for edge in edges.iter().filter(|edge| !edge.sides.is_empty()) {
        if ancestor(read, pin, &edge.parent)
            && edge.sides.iter().all(|side| ancestor(read, side, base))
            && read.clean_merge(&edge.commit)?
        {
            merged_base.insert(edge.commit.clone());
            merged_base.extend(read.only_in(&edge.sides, &edge.parent)?);
        }
    }
    Ok(edges
        .iter()
        .filter(|edge| !merged_base.contains(&edge.commit))
        .cloned()
        .collect())
}

/// Enforce immutability from branch creation, not merely after approval.
pub fn append_only(read: &mut git::ReadContext<'_>, base_commit: &str) -> Result<()> {
    for edge in read.edges(base_commit, "HEAD")?.iter() {
        for path in edge
            .paths
            .iter()
            .filter(|path| path.starts_with(".atelier/reviews"))
        {
            let before = read.entry(&edge.parent, path)?;
            let after = read.entry(&edge.commit, path)?;
            if before.is_some() && after.is_none() && retired_review_cluster(read, edge, path)? {
                continue;
            }
            if before.is_some() || after.is_none_or(|(mode, _)| mode != "100644") {
                bail!("review_append_only: {} was rewritten, deleted or changed mode in {}; append a successor event", path.display(), edge.commit);
            }
        }
    }
    Ok(())
}

/// Historical acceptance is valid only while review events remain append-only.
pub fn append_only_history(read: &mut git::ReadContext<'_>) -> Result<()> {
    let head = read.pinned("HEAD")?;
    let history = read.history(&[
        "log",
        "--root",
        "--reverse",
        "--diff-filter=A",
        "--format=%H",
        &head,
        "--",
        ".atelier/reviews/",
    ])?;
    let Some(first) = history.lines().next() else {
        return Ok(());
    };
    let pin = match read.first_parent(first)? {
        Some(parent) => parent,
        None => {
            let tree = git::git_output(
                read.repo,
                &["ls-tree", "-rz", first, "--", ".atelier/reviews/"],
            )?;
            if tree
                .split(|byte| *byte == 0)
                .filter(|entry| !entry.is_empty())
                .any(|entry| !entry.starts_with(b"100644 "))
            {
                bail!("review_append_only: initial review events must be regular non-executable files");
            }
            first.to_owned()
        }
    };
    append_only(read, &pin)
}

pub fn author_identities(
    read: &mut git::ReadContext<'_>,
    base_commit: &str,
    ids: &BTreeSet<String>,
) -> Result<BTreeSet<String>> {
    author_identities_to(read, base_commit, ids, "HEAD")
}
fn author_identities_to(
    read: &mut git::ReadContext<'_>,
    base_commit: &str,
    ids: &BTreeSet<String>,
    end: &str,
) -> Result<BTreeSet<String>> {
    let mut authors = BTreeSet::new();
    let mut inspected = BTreeSet::new();
    for edge in read.edges(base_commit, end)?.iter() {
        if !allowed_edge(read, edge, ids)? && inspected.insert(edge.commit.clone()) {
            add_author(read, &edge.commit, &mut authors)?;
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
fn ancestor(read: &mut git::ReadContext<'_>, earlier: &str, later: &str) -> bool {
    read.ancestor(earlier, later).unwrap_or(false)
}

pub fn evaluate(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    owner: &str,
    base_commit: &str,
) -> Result<Vec<GateNode>> {
    evaluate_inner(index, read, policy, owner, base_commit, true)
}

/// Active handoffs check committed completion claims, not merge readiness.
pub fn evaluate_session(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    owner: &str,
    base_commit: &str,
) -> Result<Vec<GateNode>> {
    let issue = index.require_issue(owner)?;
    let complete = policy.issue_status_is_terminal(&issue.issue_type, &issue.status)?;
    evaluate_inner(index, read, policy, owner, base_commit, complete)
}

fn evaluate_inner(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    owner: &str,
    base_commit: &str,
    complete: bool,
) -> Result<Vec<GateNode>> {
    evaluate_scope(
        index,
        read,
        policy,
        owner,
        base_commit,
        if complete {
            EvaluationKind::Merge
        } else {
            EvaluationKind::Session
        },
        scope(index, owner)?,
    )
}
/// Session-wide completion claims; retained claims keep accepted historical proof.
pub fn evaluate_claims(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    base_commit: &str,
    ids: BTreeSet<String>,
    historical: bool,
) -> Result<Vec<GateNode>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    evaluate_scope(
        index,
        read,
        policy,
        "session",
        base_commit,
        if historical {
            EvaluationKind::Historical
        } else {
            EvaluationKind::CurrentClaims
        },
        ids,
    )
}
enum EvaluationKind {
    Merge,
    Session,
    CurrentClaims,
    Historical,
}
fn evaluate_scope(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    owner: &str,
    base_commit: &str,
    kind: EvaluationKind,
    ids: BTreeSet<String>,
) -> Result<Vec<GateNode>> {
    let complete = matches!(kind, EvaluationKind::Merge);
    let historical = matches!(kind, EvaluationKind::Historical);
    let global = matches!(
        kind,
        EvaluationKind::CurrentClaims | EvaluationKind::Historical
    );
    let claimed = ids
        .iter()
        .filter_map(|id| {
            let issue = index.get_issue(id)?;
            policy
                .issue_status_is_terminal(&issue.issue_type, &issue.status)
                .unwrap_or(false)
                .then(|| id.clone())
        })
        .collect::<BTreeSet<_>>();
    let applicable = if complete { &ids } else { &claimed };
    let mut approval_targets = BTreeSet::from([owner.to_owned()]);
    if !complete {
        approval_targets.extend(claimed.iter().cloned());
        approval_targets.extend(
            claimed
                .iter()
                .filter_map(|id| index.get_issue(id)?.parent_id),
        );
    }
    let authors = if global {
        BTreeSet::new()
    } else {
        author_identities(read, base_commit, &ids)?
    };
    for record in index
        .reviews()
        .filter(|record| ids.contains(&record.target))
    {
        if !ancestor(read, &record.commit, "HEAD") {
            bail!(
                "review {} pins a commit outside the work branch, usually because the branch was rebased or reset after the review. Reset the branch to the tip it had before (find it with `git reflog`), then merge the base in with `git merge` instead",
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
                || !ancestor(read, &original.commit, resolution)
                || !ancestor(read, resolution, "HEAD")
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
    let mut nodes = Vec::new();
    if complete {
        nodes.push(gate(
            owner,
            "work.terminal",
            terminal.is_empty(),
            if terminal.is_empty() {
                "owner and all scoped work are terminal".into()
            } else {
                format!("open scoped work: {}", terminal.join(", "))
            },
            Vec::new(),
        ));
        let mut open = BTreeSet::new();
        for id in &ids {
            open.extend(
                crate::objective_graph::dependency_closure(index, policy, id)?.blocking_ids(),
            );
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
    }
    if global {
        prefetch_original_independent(
            index,
            read,
            index.reviews().filter(|record| {
                (approval_targets.contains(&record.target)
                    && matches!(record.event, ReviewEvent::Approval))
                    || (matches!(
                        record.event,
                        ReviewEvent::Disposition {
                            decision: FindingDecision::Defer | FindingDecision::NoAction,
                            ..
                        }
                    ) && applicable.contains(&record.target))
            }),
        )?;
    }
    let candidates = index
        .reviews()
        .filter(|record| approval_targets.contains(&record.target))
        .filter(|record| {
            matches!(
                record.event,
                ReviewEvent::Approval | ReviewEvent::TrivialExemption
            )
        })
        .filter(|record| {
            let accepted_identity = matches!(record.event, ReviewEvent::TrivialExemption)
                || if global {
                    original_independent(index, read, record).unwrap_or(false)
                } else {
                    independent(record, &authors)
                };
            // An approval from before the base was last merged in still
            // counts; one from before the branch left the base does not.
            accepted_identity
                && ((historical && retained(read, base_commit, record))
                    || ((global
                        || ancestor(read, base_commit, &record.commit)
                        || !ancestor(read, &record.commit, base_commit))
                        && approval_is_current(read, &record.commit, &ids, base_commit)
                            .unwrap_or(false)))
        })
        .collect::<Vec<_>>();
    let support = candidates
        .iter()
        .map(|record| record.header.id.clone())
        .collect::<Vec<_>>();
    if complete {
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
    }
    let mut unresolved = Vec::new();
    let mut dispositions = Vec::new();
    let mut approval_valid = BTreeMap::new();
    for finding in index.reviews().filter(|record| {
        applicable.contains(&record.target) && matches!(record.event, ReviewEvent::Finding { .. })
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
                ancestor(read, resolution, "HEAD")
                    && candidates.iter().any(|approval| {
                        matches!(approval.event, ReviewEvent::Approval)
                            && (approval.target == owner
                                || approval.target == finding.target
                                || index.get_issue(&finding.target).is_some_and(|issue| {
                                    issue.parent_id.as_deref() == Some(&approval.target)
                                }))
                            && ancestor(read, resolution, &approval.commit)
                            && (!historical
                                || retained(read, base_commit, disposition)
                                || !retained(read, base_commit, approval))
                            && (!historical
                                || *approval_valid
                                    .entry((approval.header.id.clone(), finding.target.clone()))
                                    .or_insert_with(|| {
                                        original_approval_valid(
                                            index,
                                            read,
                                            approval,
                                            &finding.target,
                                            base_commit,
                                        )
                                        .unwrap_or(false)
                                    }))
                    })
            }),
            FindingDecision::Defer | FindingDecision::NoAction => {
                if global {
                    original_independent(index, read, disposition)?
                } else {
                    independent(disposition, &authors)
                }
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
    nodes.extend(completion_claims(
        index, read, policy, &claimed, historical,
    )?);
    let checks = if complete {
        crate::workflow_validation::evaluate_policy_transition(
            crate::workflow_validation::ValidatorRequest {
                index,
                repo_root: read.repo,
                policy,
                issue_id: owner,
                validators: &policy.merge_checks,
            },
        )?
    } else {
        Vec::new()
    };
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

/// Validate committed terminal claims without imposing session choreography.
fn completion_claims(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    ids: &BTreeSet<String>,
    historical: bool,
) -> Result<Vec<GateNode>> {
    let mut nodes = Vec::new();
    for id in ids {
        let issue = index.require_issue(id)?;
        // Superseded work will not happen, so open blockers, including those
        // inherited from a parent, do not hold its retirement; the supersede
        // transition checks none either (atelier-lq42).
        if policy.issue_status_is_terminal(&issue.issue_type, &issue.status)?
            && issue.status != "superseded"
        {
            let blockers =
                crate::objective_graph::dependency_closure(index, policy, id)?.blocking_ids();
            nodes.push(gate(
                id,
                "blockers.transitive_none_open",
                blockers.is_empty(),
                format!("completion blockers: {}", blockers.join(", ")),
                Vec::new(),
            ));
        }
    }
    for id in ids {
        let issue = index.require_issue(id)?;
        if !policy.issue_status_is_terminal(&issue.issue_type, &issue.status)?
            || issue.status == "superseded"
        {
            continue;
        }
        if let Some(failure) = read.newest_failure(index, id, !historical)? {
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
    for id in ids {
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
            for field in &transition.required_fields {
                let present = match field.as_str() {
                    "close_reason" => index
                        .issue_record(id)
                        .context("terminal issue record missing")?
                        .sections
                        .close_reason
                        .as_deref()
                        .is_some_and(|value| !value.trim().is_empty()),
                    other => bail!("unsupported required field '{other}'"),
                };
                nodes.push(gate(
                    id,
                    &format!("required_field.{field}"),
                    present,
                    if present {
                        format!("committed claim includes required field {field}")
                    } else {
                        format!("committed claim is missing required field {field}")
                    },
                    Vec::new(),
                ));
            }
            let checks = crate::workflow_validation::evaluate_with_context(
                crate::workflow_validation::ValidatorRequest {
                    index,
                    repo_root: read.repo,
                    policy,
                    issue_id: id,
                    validators: &transition.validators,
                },
                !historical,
                read,
            )?;
            nodes.extend(
                checks.into_iter().map(|check| {
                    gate(id, &check.validator, check.passed, check.reason, Vec::new())
                }),
            );
        }
    }
    Ok(nodes)
}

fn add_author(
    read: &mut git::ReadContext<'_>,
    commit: &str,
    authors: &mut BTreeSet<String>,
) -> Result<()> {
    let names = read.author_names(commit)?;
    for identity in names.lines() {
        authors.insert(normalize(identity));
    }
    let parts = names.lines().collect::<Vec<_>>();
    if parts.len() == 2 {
        authors.insert(normalize(&format!("{} <{}>", parts[0], parts[1])));
    }
    Ok(())
}
/// The `git log` that finds the commit introducing a review event.
fn introduced_query(head: &str, record: &ReviewRecord) -> Vec<String> {
    [
        "log",
        "--root",
        "--reverse",
        "--diff-filter=A",
        "--format=%H",
        head,
        "--",
        &format!(".atelier/reviews/{}.md", record.header.id),
    ]
    .map(str::to_owned)
    .to_vec()
}
/// The `git log` queries for where a reviewed scope starts and for its
/// owner's record history, both up to the introducing `event`.
fn scope_queries(
    index: &RecordIndex,
    record: &ReviewRecord,
    event: &str,
) -> Result<(Vec<String>, Vec<String>)> {
    let issue = index.require_issue(&record.target)?;
    let owner = issue.parent_id.as_deref().unwrap_or(&record.target);
    let ids = scope(index, owner)?;
    let history = [
        "log",
        "--root",
        "--reverse",
        "--diff-filter=A",
        "--format=%H",
        event,
        "--",
    ]
    .map(str::to_owned)
    .into_iter()
    .chain(ids.iter().map(|id| format!(".atelier/issues/{id}.md")))
    .collect();
    let updates = [
        "log",
        "--root",
        "--reverse",
        "--format=%H",
        event,
        "--",
        &format!(".atelier/issues/{owner}.md"),
    ]
    .map(str::to_owned)
    .to_vec();
    Ok((history, updates))
}
fn as_args(query: &[String]) -> Vec<&str> {
    query.iter().map(String::as_str).collect()
}
/// Run the history queries of `original_independent` for many records at
/// once; each is an independent `git log` walk (atelier-0349).
fn prefetch_original_independent<'r>(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    records: impl IntoIterator<Item = &'r ReviewRecord>,
) -> Result<()> {
    let head = read.pinned("HEAD")?;
    let records = records.into_iter().collect::<Vec<_>>();
    read.prefetch_histories(
        records
            .iter()
            .map(|record| introduced_query(&head, record))
            .collect(),
    );
    let mut queries = Vec::new();
    for record in records {
        let Ok(introduced) = read.history(&as_args(&introduced_query(&head, record))) else {
            continue;
        };
        if let Some(event) = introduced.lines().next() {
            if let Ok((history, updates)) = scope_queries(index, record, event) {
                queries.extend([history, updates]);
            }
        }
    }
    read.prefetch_histories(queries);
    Ok(())
}
fn original_independent(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    record: &ReviewRecord,
) -> Result<bool> {
    let issue = index.require_issue(&record.target)?;
    let ids = scope(index, issue.parent_id.as_deref().unwrap_or(&record.target))?;
    let event_path = format!(".atelier/reviews/{}.md", record.header.id);
    let head = read.pinned("HEAD")?;
    let introduced = read.history(&as_args(&introduced_query(&head, record)))?;
    let event = introduced
        .lines()
        .next()
        .context("review event history missing")?;
    let path = Path::new(&event_path);
    if read.entry(event, path)? != read.entry("HEAD", path)? {
        bail!(
            "review_append_only: {} differs from its original event",
            record.header.id
        );
    }
    let (history, updates) = scope_queries(index, record, event)?;
    let history = read.history(&as_args(&history))?;
    let start = history
        .lines()
        .next()
        .context("review predates its issue scope")?;
    let owner = issue.parent_id.as_deref().unwrap_or(&record.target);
    let updates = read.history(&as_args(&updates))?;
    let branch_start = updates.lines().find(|commit| {
        read.issue_at(commit, owner)
            .is_ok_and(|record| record.issue.workflow_branch.is_some())
    });
    let authors = if let Some(origin) = branch_start {
        match read.first_parent(origin)? {
            Some(parent) => author_identities_to(read, &parent, &ids, event)?,
            None => {
                let mut authors = author_identities_to(read, origin, &ids, event)?;
                add_author(read, origin, &mut authors)?;
                authors
            }
        }
    } else {
        let mut authors = author_identities_to(read, start, &ids, event)?;
        add_author(read, start, &mut authors)?;
        authors
    };
    Ok(independent(record, &authors))
}

fn retained(read: &mut git::ReadContext<'_>, base: &str, record: &ReviewRecord) -> bool {
    let path = std::path::PathBuf::from(format!(".atelier/reviews/{}.md", record.header.id));
    let old = read.entry(base, &path).ok().flatten();
    old.is_some() && old == read.entry("HEAD", &path).ok().flatten()
}

fn original_approval_valid(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    approval: &ReviewRecord,
    target: &str,
    base: &str,
) -> Result<bool> {
    let issue = index.require_issue(target)?;
    let ids = scope(index, issue.parent_id.as_deref().unwrap_or(target))?;
    let end = read.claim_endpoint(target, &approval.commit)?;
    approval_is_current_to(read, &approval.commit, &ids, &end, base)
}

fn retired_review_cluster(
    read: &mut git::ReadContext<'_>,
    edge: &git::CommitEdge,
    path: &Path,
) -> Result<bool> {
    let record = atelier_records::review::parse_review_record(
        &read.blob(&edge.parent, path)?,
        path.strip_prefix(".atelier")?,
    )?;
    let issue_path = std::path::PathBuf::from(format!(".atelier/issues/{}.md", record.target));
    if read.entry(&edge.commit, &issue_path)?.is_some() {
        return Ok(false);
    }
    let issue = read.issue_at(&edge.parent, &record.target)?;
    let policy = crate::base_policy::from_captured(read, &edge.parent)?.policy;
    if !policy.issue_status_is_terminal(&issue.issue.issue_type, &issue.issue.status)? {
        return Ok(false);
    }
    let paths = String::from_utf8(git::git_output(
        read.repo,
        &[
            "ls-tree",
            "-r",
            "--name-only",
            &edge.parent,
            "--",
            ".atelier/reviews/",
        ],
    )?)?;
    for name in paths.lines() {
        let other_path = Path::new(name);
        if other_path
            .extension()
            .is_none_or(|extension| extension != "md")
        {
            continue;
        }
        let other = atelier_records::review::parse_review_record(
            &read.blob(&edge.parent, other_path)?,
            other_path.strip_prefix(".atelier")?,
        )?;
        if other.target == record.target && read.entry(&edge.commit, other_path)?.is_some() {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_epic_migration_is_status_only_but_content_edits_are_not() {
        let (epic, migrated) = crate::evidence_freshness::tests::approved_epic();
        let path = Path::new(".atelier/issues/atelier-e001.md");
        assert!(issue_status_only(&epic, &migrated, path).unwrap());
        let edited = migrated.replace("Checked.", "Checked differently.");
        assert!(!issue_status_only(&epic, &edited, path).unwrap());
        // `migrate epics --to` chooses the type, so any declared type counts.
        let retyped = migrated.replace("issue_type: \"feature\"", "issue_type: \"bug\"");
        assert!(issue_status_only(&epic, &retyped, path).unwrap());
    }
}
