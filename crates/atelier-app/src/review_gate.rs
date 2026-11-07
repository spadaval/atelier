//! Review integrity, semantic freshness and derived merge gate graph nodes.
use crate::{
    git_freshness as git,
    path_log::{Listing, PathLog},
    record_index::RecordIndex,
    workflow_policy::WorkflowPolicy,
};
use anyhow::{bail, Context, Result};
use atelier_core::{FindingDecision, ReviewEvent, ReviewRecord};
use atelier_records::review::{parse_review_file, parse_review_record, ReviewFile};
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
    /// What a failed check waits on, one entry per record, so a view can
    /// name each with its fix; `--json` leaves it out.
    #[serde(skip)]
    pub gaps: Vec<Gap>,
    /// The command that fixes a failed check, when the check names one.
    #[serde(skip)]
    pub fix: Option<String>,
}
/// One thing a failed gate waits on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gap {
    /// Open work in the owner's scope.
    Open(String),
    /// An open blocker.
    Blocker(String),
    /// A finding with no disposition.
    Undisposed(String),
    /// A finding whose fix_now disposition names no resolution commit.
    Unresolved(String),
    /// A resolved fix_now finding with no independent approval at or after
    /// its resolution.
    AwaitingApproval(String),
    /// A finding whose defer or no_action disposition no non-author set.
    Unaccepted(String),
    /// A finding whose disposition could not be traced to the commit that
    /// set it, and why.
    Untraced { finding: String, error: String },
    /// Failing evidence newer than the claim's passing proof.
    FailingEvidence(String),
    /// A `shell.check` command that failed, with the last line it printed.
    FailedCommand { command: String, output: String },
    /// A done claim, made at `commit`, outside every current approval.
    UnreviewedClaim { claim: String, commit: String },
}
pub fn gate(
    owner: &str,
    check: &str,
    passed: bool,
    reason: String,
    supports: Vec<String>,
) -> GateNode {
    GateNode {
        kind: "workflow_validator",
        id: format!("{owner}/{check}"),
        target: owner.into(),
        check: check.into(),
        passed,
        reason,
        supports,
        gaps: Vec::new(),
        fix: None,
    }
}
impl GateNode {
    pub fn with_gaps(mut self, gaps: Vec<Gap>) -> Self {
        self.gaps = gaps;
        self
    }
}
/// A validator's result as a gate node. Its hint's command, quoted in
/// backticks, is the node's fix; failed blocker checks name each open
/// blocker, and failed shell checks their command.
fn validator_gate(
    index: &RecordIndex,
    policy: &WorkflowPolicy,
    target: &str,
    check: crate::workflow_validation::ValidatorResult,
    definition: Option<&crate::workflow_policy::ValidatorDefinition>,
) -> Result<GateNode> {
    let gaps = if check.passed {
        Vec::new()
    } else {
        match check.validator.as_str() {
            "blockers.none_open" => index
                .get_blockers(target)
                .into_iter()
                .filter(|id| {
                    index
                        .get_issue(id)
                        .is_some_and(|issue| policy.status_category(&issue.status) != Some("done"))
                })
                .map(Gap::Blocker)
                .collect(),
            "blockers.transitive_none_open" => {
                crate::objective_graph::dependency_closure(index, policy, target)?
                    .blocking_ids()
                    .into_iter()
                    .map(Gap::Blocker)
                    .collect()
            }
            "work.terminal" => crate::objective_graph::issue_descendant_ids(index, target)?
                .into_iter()
                .filter(|id| {
                    index
                        .get_issue(id)
                        .is_some_and(|issue| policy.status_category(&issue.status) != Some("done"))
                })
                .map(Gap::Open)
                .collect(),
            "shell.check" => match definition.and_then(|definition| definition.params.as_ref()) {
                Some(crate::workflow_policy::ValidatorParams::ShellCheck { command }) => {
                    vec![Gap::FailedCommand {
                        command: command.clone(),
                        output: check
                            .reason
                            .lines()
                            .skip(1)
                            .filter(|line| !line.trim().is_empty())
                            .last()
                            .unwrap_or_default()
                            .trim()
                            .to_owned(),
                    }]
                }
                _ => Vec::new(),
            },
            _ => Vec::new(),
        }
    };
    let fix = check
        .help
        .as_deref()
        .and_then(|help| help.split('`').nth(1))
        .map(str::to_owned);
    let mut node = gate(
        target,
        &check.validator,
        check.passed,
        check.reason,
        Vec::new(),
    )
    .with_gaps(gaps);
    node.fix = fix;
    Ok(node)
}
/// The authors to show, one per person: a `name <email>` identity stands for
/// its name and email alone (atelier-9jr8).
fn shown_authors(authors: &BTreeSet<String>) -> Vec<String> {
    let combined = authors
        .iter()
        .filter_map(|author| {
            let (name, email) = author.split_once(" <")?;
            Some((name.to_owned(), email.trim_end_matches('>').to_owned()))
        })
        .collect::<Vec<_>>();
    authors
        .iter()
        .filter(|author| {
            author.contains(" <")
                || !combined
                    .iter()
                    .any(|(name, email)| *author == name || *author == email)
        })
        .cloned()
        .collect()
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

/// Whether `edge` changes only what an approval of `ids` pinned at `pin`
/// keeps current (see `approval_is_current_to`).
fn allowed_edge(
    read: &mut git::ReadContext<'_>,
    edge: &git::CommitEdge,
    ids: &BTreeSet<String>,
    pin: &str,
) -> Result<bool> {
    for path in &edge.paths {
        let old = read.entry(&edge.parent, path)?;
        let new = read.entry(&edge.commit, path)?;
        let regular = |entry: &Option<(String, String)>| {
            entry.as_ref().is_some_and(|(mode, _)| mode == "100644")
        };
        if path.extension().is_some_and(|ext| ext == "md") && path.starts_with(".atelier/reviews") {
            if review_change_allowed(read, edge, path, &old, &new)? {
                continue;
            }
            return Ok(false);
        }
        if path.extension().is_some_and(|ext| ext == "md") && path.starts_with(".atelier/evidence")
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
        // An issue outside the scope that is not in the approval's pinned
        // tree is planning, not reviewed work, in every later commit: whether
        // it was created on the branch, such as the follow-up a deferred
        // finding needs (atelier-cafy, atelier-iaas), or on the base, before
        // or after the approval, and merged in (atelier-g0mh, atelier-kotb).
        // Such records may be committed to the
        // base without review anyway. Its own done claim is judged by
        // `claims.reviewed` and `land`, not by the approval.
        if issue_id.is_some_and(|id| !ids.contains(id))
            && (old.is_none() || regular(&old))
            && regular(&new)
            && read.entry(pin, path)?.is_none()
        {
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

/// Whether `edge`'s change to the review record at `path` leaves every
/// event as it was: it adds a new event, sets or changes a finding's
/// disposition and nothing else (ADR 0025), or removes a separate
/// disposition event the way `review disposition` and
/// `atelier migrate dispositions` do, in a commit whose finding carries its
/// own disposition and keeps none of its events. A separate event is never
/// added back once removed: either would bring back a decision that was
/// replaced, which the old append-only events refused (atelier-pfbp).
/// Approval freshness and the append-only check share this one rule.
fn review_change_allowed(
    read: &mut git::ReadContext<'_>,
    edge: &git::CommitEdge,
    path: &Path,
    old: &Option<(String, String)>,
    new: &Option<(String, String)>,
) -> Result<bool> {
    let regular =
        |entry: &Option<(String, String)>| entry.as_ref().is_some_and(|(mode, _)| mode == "100644");
    let relative = path.strip_prefix(".atelier")?;
    Ok(match (old, new) {
        (None, _) => regular(new) && !retired_event_restored(read, edge, path, relative)?,
        (Some(_), None) => {
            regular(old)
                && match parse_review_file(&read.blob(&edge.parent, path)?, relative) {
                    Ok(ReviewFile::RetiredDisposition(event)) => {
                        folded_by(read, &edge.commit, &event.finding)?
                    }
                    _ => false,
                }
        }
        (Some(_), Some(_)) => {
            regular(old)
                && regular(new)
                && disposition_only(
                    &read.blob(&edge.parent, path)?,
                    &read.blob(&edge.commit, path)?,
                    relative,
                )
        }
    })
}

/// Whether `commit` holds `finding` with its own disposition field and none
/// of its separate disposition events, as the fold writes it.
fn folded_by(read: &mut git::ReadContext<'_>, commit: &str, finding: &str) -> Result<bool> {
    let path = std::path::PathBuf::from(format!(".atelier/reviews/{finding}.md"));
    if read.entry(commit, &path)?.is_none() {
        return Ok(false);
    }
    let carries = parse_review_record(&read.blob(commit, &path)?, path.strip_prefix(".atelier")?)
        .is_ok_and(|record| record.disposition().is_some());
    Ok(carries && !read.retired_findings_at(commit)?.contains(finding))
}

/// Whether `edge` adds back a separate disposition event its first parent's
/// history already held.
fn retired_event_restored(
    read: &mut git::ReadContext<'_>,
    edge: &git::CommitEdge,
    path: &Path,
    relative: &Path,
) -> Result<bool> {
    let parent = read.commit_id(&edge.parent)?;
    let earlier = read.path_log(&PathLog {
        start: parent,
        paths: vec![path.display().to_string()],
        listing: Listing::Changes,
        reverse: false,
    })?;
    Ok(!earlier.is_empty()
        && matches!(
            parse_review_file(&read.blob(&edge.commit, path)?, relative),
            Ok(ReviewFile::RetiredDisposition(_))
        ))
}

/// Whether two versions of a review record are the same finding, differing
/// at most in its disposition and what that derives: its links and update
/// time. The schema version may differ, since a finding written before
/// ADR 0025 is rewritten in the new version when it is first disposed.
pub(crate) fn disposition_only(before: &str, after: &str, relative: &Path) -> bool {
    let (Ok(mut before), Ok(mut after)) = (
        parse_review_record(before, relative),
        parse_review_record(after, relative),
    ) else {
        return false;
    };
    for record in [&mut before, &mut after] {
        let ReviewEvent::Finding { disposition, .. } = &mut record.event else {
            return false;
        };
        *disposition = None;
        record.header.updated_at = record.header.created_at;
        record.header.relationships = Default::default();
    }
    before == after
}

pub fn approval_is_current(
    read: &mut git::ReadContext<'_>,
    pin: &str,
    ids: &BTreeSet<String>,
    base: &str,
) -> Result<bool> {
    approval_is_current_to(read, pin, ids, "HEAD", base)
}
/// The one rule for what an approval or trivial exemption covers
/// (atelier-n642). Pinned at `pin`, it stays current at `end` when every
/// commit after the pin only adds review or evidence records, files or edits
/// issues outside the approved scope `ids` that are new since the pin (a
/// follow-up and its links, atelier-iaas), changes a record in `ids` by status,
/// timestamps and Close Reason alone, or cleanly merges commits already on
/// `base`. So a close commit that changes only the scope's own records keeps
/// it current, and one that also edits code or another field does not.
/// `merge`'s `review.current`, the session gates and `land`'s check of done
/// claims on the base all ask this; `end` must contain `pin`.
pub fn approval_is_current_to(
    read: &mut git::ReadContext<'_>,
    pin: &str,
    ids: &BTreeSet<String>,
    end: &str,
    base: &str,
) -> Result<bool> {
    for (edge, merge) in work_edges(read, pin, end, base)? {
        if !edge_allowed(read, &edge, merge.as_ref(), ids, pin)? {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Whether one edge after `pin` keeps an approval of `ids` current: the one
/// edge check `approval_is_current_to` and `approval_stale_edges` share.
fn edge_allowed(
    read: &mut git::ReadContext<'_>,
    edge: &git::CommitEdge,
    merge: Option<&git::MergeResolution>,
    ids: &BTreeSet<String>,
    pin: &str,
) -> Result<bool> {
    match merge {
        Some(merge) => allowed_base_merge(read, edge, merge, ids, pin),
        None => allowed_edge(read, edge, ids, pin),
    }
}

/// A commit after an approval's pin that makes it stale, with the paths a
/// new review has to look at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaleEdge {
    pub commit: String,
    /// The commit's subject line.
    pub subject: String,
    /// Set for a merge: whether every commit it brings in is on the base.
    pub merge: Option<BaseMerge>,
    /// What a new review has to read: for a merge, the paths it resolved or
    /// edited by hand and the in-scope records it changed, or all it changed
    /// when it did neither; for any other commit, what it changed.
    pub paths: Vec<std::path::PathBuf>,
    /// The paths a merge resolved or edited beyond Git's own merge.
    pub by_hand: Vec<std::path::PathBuf>,
}
/// Whether a merge brings in only commits already on the base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseMerge {
    Base,
    Other,
}

/// Every commit of the branch (not of the base) after `pin` that makes the approval stale at `end`, oldest
/// first, each with the paths a new review has to look at; empty while the
/// approval is current. It applies the rule of [`approval_is_current_to`] to
/// each edge, so what `merge --sync` reports as needing review is exactly
/// what makes the approval stale, but unlike that check it computes the
/// paths. For a merge they are those it resolved by hand or edited beyond
/// Git's own merge, plus in-scope records it changed; for any other commit,
/// what it changed.
pub fn approval_stale_edges(
    read: &mut git::ReadContext<'_>,
    pin: &str,
    ids: &BTreeSet<String>,
    end: &str,
    base: &str,
) -> Result<Vec<StaleEdge>> {
    let mut stale = Vec::new();
    let edges = work_edges(read, pin, end, base)?;
    // A commit the base already holds is reviewed there; when a merge brings
    // it in unaccepted, the merge is what to report, even when the merge
    // itself changes nothing against the branch, such as a conflict
    // resolved to the branch's side.
    let mut unaccepted_base = HashSet::new();
    for (edge, merge) in &edges {
        if ancestor(read, &edge.commit, base)
            && !edge_allowed(read, edge, merge.as_ref(), ids, pin)?
        {
            unaccepted_base.insert(edge.commit.clone());
        }
    }
    // Edges come newest first.
    for (edge, merge) in edges.into_iter().rev() {
        if ancestor(read, &edge.commit, base) {
            continue;
        }
        let brings_unaccepted = !edge.sides.is_empty()
            && !unaccepted_base.is_empty()
            && read
                .only_in(&edge.sides, &edge.parent)?
                .iter()
                .any(|commit| unaccepted_base.contains(commit));
        if !brings_unaccepted && edge_allowed(read, &edge, merge.as_ref(), ids, pin)? {
            continue;
        }
        let mut by_hand = Vec::new();
        let paths = if edge.sides.is_empty() {
            edge.paths
        } else {
            by_hand = git::hand_resolved_paths(read.repo, &edge.commit)?;
            let mut paths = by_hand.clone();
            paths.extend(
                edge.paths
                    .iter()
                    .filter(|path| in_scope_issue(path, ids))
                    .cloned(),
            );
            paths.sort();
            paths.dedup();
            if paths.is_empty() {
                edge.paths
            } else {
                paths
            }
        };
        let subject = String::from_utf8(git::git_output(
            read.repo,
            &["log", "-1", "--format=%s", &edge.commit],
        )?)?
        .trim()
        .to_owned();
        let merge = if edge.sides.is_empty() {
            None
        } else if edge.sides.iter().all(|side| ancestor(read, side, base)) {
            Some(BaseMerge::Base)
        } else {
            Some(BaseMerge::Other)
        };
        stale.push(StaleEdge {
            commit: edge.commit,
            subject,
            merge,
            paths,
            by_hand,
        });
    }
    Ok(stale)
}

fn in_scope_issue(path: &Path, ids: &BTreeSet<String>) -> bool {
    path.parent() == Some(Path::new(".atelier/issues"))
        && path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .is_some_and(|id| ids.contains(id))
}

/// Whether `record`, an approval or trivial exemption on a work branch that
/// left the base at `fork_point`, is current at the branch's HEAD as
/// `merge`'s `review.current` judges it. An approval from before the base
/// was last merged in still counts; one from before the branch left the base
/// does not.
pub fn current_on_branch(
    read: &mut git::ReadContext<'_>,
    record: &ReviewRecord,
    ids: &BTreeSet<String>,
    fork_point: &str,
) -> bool {
    (ancestor(read, fork_point, &record.commit) || !ancestor(read, &record.commit, fork_point))
        && approval_is_current(read, &record.commit, ids, fork_point).unwrap_or(false)
}

/// Where a work branch owner's review stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalState {
    /// `merge`'s `review.current` would pass.
    Current,
    /// An independent approval or trivial exemption exists, but later
    /// commits changed what it covers.
    Stale,
    /// No independent approval or trivial exemption of the owner.
    Missing,
}

/// The owner's approval state on a work branch, judged as `merge`'s
/// `review.current` judges it: `index` holds the branch's records and `read`
/// is pinned at its tip, and `fork_point` is where it left the base. Status
/// reads it for other branches without checking them out (atelier-tciw).
pub fn approval_state(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    owner: &str,
    fork_point: &str,
) -> Result<ApprovalState> {
    Ok(approval_standing(index, read, owner, fork_point)?.state)
}

/// Why an approval or trivial exemption does not make `review.current` pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotCurrent {
    /// Its identity matches a Git author of the branch's work.
    Author,
    /// It predates the point where the branch left the base, which already
    /// holds the work since.
    PredatesFork,
    /// Commits after its pin changed what it covers, oldest first.
    Stale(Vec<StaleEdge>),
}

/// Where the owner's review stands, with the record it rests on and, when
/// that record does not count, why (atelier-cisa).
#[derive(Debug)]
pub struct Standing<'i> {
    pub state: ApprovalState,
    /// The current approval or exemption; else the newest stale one; else,
    /// while the state is missing, the newest approval by an author.
    pub record: Option<&'i ReviewRecord>,
    /// Why `record` does not count; `None` while it is current.
    pub not_current: Option<NotCurrent>,
    /// The branch's authors, normalized, as independence is judged.
    pub authors: BTreeSet<String>,
}
impl Standing<'_> {
    /// The authors to show, one per person.
    pub fn shown_authors(&self) -> Vec<String> {
        shown_authors(&self.authors)
    }
}

/// [`approval_state`] with the record it rests on and why it does not count
/// (atelier-uv5d, atelier-cisa): the one judgement `review.current`, `merge
/// --sync` and `review list` explain.
pub fn approval_standing<'i>(
    index: &'i RecordIndex,
    read: &mut git::ReadContext<'_>,
    owner: &str,
    fork_point: &str,
) -> Result<Standing<'i>> {
    let ids = scope(index, owner)?;
    let authors = author_identities(read, fork_point, &ids)?;
    standing(index, read, owner, &ids, authors, fork_point)
}

fn standing<'i>(
    index: &'i RecordIndex,
    read: &mut git::ReadContext<'_>,
    owner: &str,
    ids: &BTreeSet<String>,
    authors: BTreeSet<String>,
    fork_point: &str,
) -> Result<Standing<'i>> {
    let newer = |record: &ReviewRecord, than: Option<&ReviewRecord>| {
        than.is_none_or(|newest| {
            (record.header.created_at, &record.header.id)
                > (newest.header.created_at, &newest.header.id)
        })
    };
    let mut stale: Option<&ReviewRecord> = None;
    let mut by_author: Option<&ReviewRecord> = None;
    for record in index.reviews().filter(|record| record.target == owner) {
        let accepted_identity = match record.event {
            ReviewEvent::TrivialExemption => true,
            ReviewEvent::Approval => independent(record, &authors),
            _ => continue,
        };
        if !accepted_identity {
            if newer(record, by_author) {
                by_author = Some(record);
            }
            continue;
        }
        if current_on_branch(read, record, ids, fork_point) {
            return Ok(Standing {
                state: ApprovalState::Current,
                record: Some(record),
                not_current: None,
                authors,
            });
        }
        if newer(record, stale) {
            stale = Some(record);
        }
    }
    Ok(match (stale, by_author) {
        (Some(record), _) => {
            let edges = approval_stale_edges(read, &record.commit, ids, "HEAD", fork_point)?;
            Standing {
                state: ApprovalState::Stale,
                record: Some(record),
                not_current: Some(if edges.is_empty() {
                    NotCurrent::PredatesFork
                } else {
                    NotCurrent::Stale(edges)
                }),
                authors,
            }
        }
        (None, record) => Standing {
            state: ApprovalState::Missing,
            record,
            not_current: record.map(|_| NotCurrent::Author),
            authors,
        },
    })
}

/// `review.current`'s reason when it fails: the approval it considered and
/// why it does not count, or that there is none.
fn not_current_reason(standing: &Standing<'_>, ids: &BTreeSet<String>) -> String {
    const LISTED: usize = 3;
    let authors = standing.shown_authors();
    let (Some(record), Some(why)) = (standing.record, &standing.not_current) else {
        return if authors.is_empty() {
            "no independent approval or trivial exemption".to_owned()
        } else {
            format!(
                "no independent approval or trivial exemption; authors: {}",
                authors.join(", ")
            )
        };
    };
    let name = format!(
        "{} {} @{}",
        if record.event == ReviewEvent::TrivialExemption {
            "trivial exemption"
        } else {
            "approval"
        },
        record.header.id,
        short(&record.commit)
    );
    let listed = |paths: &[std::path::PathBuf]| {
        let mut shown = paths
            .iter()
            .take(LISTED)
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>();
        if paths.len() > LISTED {
            shown.push(format!("{} more", paths.len() - LISTED));
        }
        match shown.as_slice() {
            [] => "nothing".to_owned(),
            [one] => one.clone(),
            [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
        }
    };
    match why {
        NotCurrent::Author => format!(
            "{name} by {} is not independent: the identity matches a Git author of the branch (authors: {})",
            record.identity,
            authors.join(", ")
        ),
        NotCurrent::PredatesFork => format!(
            "{name} is stale: it predates the point where the branch left the base, which already holds the work since"
        ),
        NotCurrent::Stale(edges) => {
            let first = &edges[0];
            let cause = match first.merge {
                Some(kind) => {
                    let records = first
                        .paths
                        .iter()
                        .filter(|path| in_scope_issue(path, ids))
                        .cloned()
                        .collect::<Vec<_>>();
                    let by_hand = first
                        .by_hand
                        .iter()
                        .filter(|path| !records.contains(path))
                        .cloned()
                        .collect::<Vec<_>>();
                    let mut parts = Vec::new();
                    if !records.is_empty() {
                        parts.push(format!(
                            "changed in-scope {} {} beyond status",
                            if records.len() == 1 { "record" } else { "records" },
                            listed(&records)
                        ));
                    }
                    if !by_hand.is_empty() {
                        parts.push(format!("resolved or edited {} by hand", listed(&by_hand)));
                    }
                    if parts.is_empty() {
                        parts.push(format!("changed {}", listed(&first.paths)));
                    }
                    format!(
                        "{} {} {}",
                        match kind {
                            BaseMerge::Base => "base merge",
                            BaseMerge::Other => "merge",
                        },
                        short(&first.commit),
                        parts.join(" and ")
                    )
                }
                None => format!(
                    "{} \"{}\" changed {}",
                    short(&first.commit),
                    first.subject,
                    listed(&first.paths)
                ),
            };
            let later = match edges.len() - 1 {
                0 => String::new(),
                1 => " (and 1 later commit)".to_owned(),
                n => format!(" (and {n} later commits)"),
            };
            format!("{name} is stale: {cause}{later}")
        }
    }
}

fn short(commit: &str) -> &str {
    &commit[..commit.len().min(8)]
}

/// Whether an approval or trivial exemption is one `merge` would accept,
/// judged from history as the base's claim gates judge it (atelier-n90q): an
/// approval's identity is independent of the branch's own authors up to the
/// event, leaving out the commits of every branch merged in (an exemption is
/// explicit and names itself), and every finding on the
/// scope pinned at or before it has an accepted disposition: a FIX NOW whose
/// resolution commit is at or before the approval's pin, or a DEFER or NO
/// ACTION accepted by a non-author. Only that resolution commit is checked
/// against the pin; the disposition itself may be recorded later, as
/// `merge`'s findings.disposed allows. `land` on the base counts only these
/// approvals as covering commits.
pub fn approval_accepted(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    record: &ReviewRecord,
) -> Result<bool> {
    let independent = match record.event {
        ReviewEvent::TrivialExemption => true,
        ReviewEvent::Approval => original_independent(index, read, record).unwrap_or(false),
        _ => false,
    };
    if !independent {
        return Ok(false);
    }
    let issue = index.require_issue(&record.target)?;
    let ids = scope(index, issue.parent_id.as_deref().unwrap_or(&record.target))?;
    for finding in index.reviews().filter(|finding| {
        ids.contains(&finding.target) && matches!(finding.event, ReviewEvent::Finding { .. })
    }) {
        if !ancestor(read, &finding.commit, &record.commit) {
            continue;
        }
        let accepted = match finding.disposition() {
            Some(disposition) if disposition.decision == FindingDecision::FixNow => disposition
                .resolution_commit
                .as_ref()
                .is_some_and(|resolution| ancestor(read, resolution, &record.commit)),
            Some(_) => disposition_independent(index, read, finding).unwrap_or(false),
            None => false,
        };
        if !accepted {
            return Ok(false);
        }
    }
    Ok(true)
}
/// A merge of base content into the work branch, checked against the branch
/// side. Paths Git merged on its own came from the base, except that an
/// in-scope issue record may still change only in status, so base planning
/// cannot rewrite the approved scope. A record conflict resolved by hand must
/// be a three-way merge of its sides, so it cannot drop a blocker or a child
/// either side added; in-scope ones are status-only too (atelier-n7g1,
/// atelier-1isk).
fn allowed_base_merge(
    read: &mut git::ReadContext<'_>,
    edge: &git::CommitEdge,
    merge: &git::MergeResolution,
    ids: &BTreeSet<String>,
    pin: &str,
) -> Result<bool> {
    // A conflict resolved to the branch side's version changes nothing
    // against it, yet may drop the base's side, so every conflict is checked.
    let paths = edge
        .paths
        .iter()
        .chain(&merge.conflicted)
        .collect::<BTreeSet<_>>();
    for path in paths {
        let issue = path.parent() == Some(Path::new(".atelier/issues"));
        let in_scope_issue = issue
            && path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .is_some_and(|id| ids.contains(id));
        let conflicted = merge.conflicted.contains(path);
        if !in_scope_issue && !conflicted {
            continue;
        }
        if conflicted && issue {
            let (Some(base), [side]) = (&merge.merge_base, edge.sides.as_slice()) else {
                return Ok(false);
            };
            if !record_three_way(read, path, base, &edge.parent, side, &edge.commit)? {
                return Ok(false);
            }
            if !in_scope_issue {
                continue;
            }
        }
        let single = git::CommitEdge {
            paths: vec![path.clone()],
            ..edge.clone()
        };
        if !allowed_edge(read, &single, ids, pin)? {
            return Ok(false);
        }
    }
    Ok(true)
}
/// Whether `resolved` is a three-way merge of an issue record's `ours` and
/// `theirs` from `base`: each front-matter field and the body take the side
/// that changed them, lists merge as sets of whole entries (each side's
/// additions kept and removals respected), and a value both sides changed
/// differently is a real conflict that needs review. Timestamps may take
/// either side.
fn record_three_way(
    read: &mut git::ReadContext<'_>,
    path: &Path,
    base: &str,
    ours: &str,
    theirs: &str,
    resolved: &str,
) -> Result<bool> {
    use serde_yaml::Value;
    let mut parts = |commit: &str| -> Result<(Value, Value)> {
        if read.entry(commit, path)?.is_none() {
            return Ok((Value::Null, Value::Null));
        }
        let text = read.blob(commit, path)?;
        Ok(match split_document(&text) {
            Some((front, body)) => (
                serde_yaml::from_str(front).unwrap_or(Value::String(front.to_owned())),
                Value::String(body.to_owned()),
            ),
            None => (Value::Null, Value::String(text)),
        })
    };
    let (base, ours, theirs, resolved) =
        (parts(base)?, parts(ours)?, parts(theirs)?, parts(resolved)?);
    Ok(merged(&base.0, &ours.0, &theirs.0, &resolved.0, "")
        && merged(&base.1, &ours.1, &theirs.1, &resolved.1, ""))
}
fn merged(
    base: &serde_yaml::Value,
    ours: &serde_yaml::Value,
    theirs: &serde_yaml::Value,
    resolved: &serde_yaml::Value,
    key: &str,
) -> bool {
    use serde_yaml::Value;
    if matches!(key, "updated_at" | "closed_at") {
        return resolved == ours || resolved == theirs;
    }
    if ours == theirs {
        return resolved == ours;
    }
    if ours == base {
        return resolved == theirs;
    }
    if theirs == base {
        return resolved == ours;
    }
    match (ours, theirs, resolved) {
        (Value::Mapping(ours), Value::Mapping(theirs), Value::Mapping(resolved)) => {
            let empty = serde_yaml::Mapping::new();
            let base = base.as_mapping().unwrap_or(&empty);
            let null = Value::Null;
            base.keys()
                .chain(ours.keys())
                .chain(theirs.keys())
                .chain(resolved.keys())
                .all(|field| {
                    merged(
                        base.get(field).unwrap_or(&null),
                        ours.get(field).unwrap_or(&null),
                        theirs.get(field).unwrap_or(&null),
                        resolved.get(field).unwrap_or(&null),
                        field.as_str().unwrap_or(""),
                    )
                })
        }
        (Value::Sequence(ours), Value::Sequence(theirs), Value::Sequence(resolved)) => {
            let empty = Vec::new();
            let base = base.as_sequence().unwrap_or(&empty);
            let expected = ours
                .iter()
                .filter(|entry| !base.contains(entry) || theirs.contains(entry))
                .chain(
                    theirs
                        .iter()
                        .filter(|entry| !base.contains(entry) && !ours.contains(entry)),
                )
                .collect::<Vec<_>>();
            // The same entries, each as many times, so a duplicate cannot
            // make room for one nobody wrote (atelier-wgil).
            resolved.len() == expected.len()
                && resolved.iter().all(|entry| {
                    resolved.iter().filter(|other| *other == entry).count()
                        == expected.iter().filter(|other| **other == entry).count()
                })
        }
        _ => false,
    }
}
/// The merges in `edges` (from `pin`) that bring in commits already on
/// `base`, each with the commits only its other sides hold. Those commits
/// were reviewed on their own branches, so they are neither work the
/// approval covers nor work its reviewer authored (atelier-n7g1).
fn base_merges<'e>(
    read: &mut git::ReadContext<'_>,
    pin: &str,
    edges: &'e [git::CommitEdge],
    base: &str,
) -> Result<Vec<(&'e git::CommitEdge, HashSet<String>)>> {
    let mut merges = Vec::new();
    for edge in edges.iter().filter(|edge| !edge.sides.is_empty()) {
        if ancestor(read, pin, &edge.parent)
            && edge.sides.iter().all(|side| ancestor(read, side, base))
        {
            merges.push((edge, read.only_in(&edge.sides, &edge.parent)?));
        }
    }
    Ok(merges)
}
/// The edges from `pin` to `end` that can change approved work, each with
/// its merge resolution when it merges commits already on `base`. Those base
/// commits are left out; the merge itself is checked with
/// `allowed_base_merge`.
fn work_edges(
    read: &mut git::ReadContext<'_>,
    pin: &str,
    end: &str,
    base: &str,
) -> Result<Vec<(git::CommitEdge, Option<git::MergeResolution>)>> {
    let edges = read.edges(pin, end)?;
    let mut merged_base = HashSet::new();
    let mut merges = BTreeMap::new();
    for (edge, only_in_sides) in base_merges(read, pin, &edges, base)? {
        let Some(merge) = read.merge_resolution(&edge.commit)? else {
            continue;
        };
        merged_base.extend(only_in_sides);
        merges.insert(edge.commit.clone(), merge);
    }
    Ok(edges
        .iter()
        .filter(|edge| !merged_base.contains(&edge.commit))
        .map(|edge| (edge.clone(), merges.get(&edge.commit).cloned()))
        .collect())
}

/// Enforce immutability from branch creation, not merely after approval.
///
/// A later commit cannot undo a rewrite, since every commit in the range is
/// checked, so the recovery is a rebase that leaves out the commit that made
/// it, named by its ID so it holds wherever HEAD has moved since, as other
/// fixes commit (atelier-v4rd, atelier-61il). At the tip it only drops it,
/// which brings the record back; below, it rewrites the commits after it.
pub fn append_only(read: &mut git::ReadContext<'_>, base_commit: &str) -> Result<()> {
    let head = read.commit_id("HEAD")?;
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
            if !review_change_allowed(read, edge, path, &before, &after)? {
                let commit = &edge.commit;
                if let Some(side) = merged_migration(read, edge, path, &after)? {
                    let rewrites = if *commit == head {
                        ""
                    } else {
                        ", and rewrites the commits after it: get them reviewed again, since records pinned to them no longer count"
                    };
                    bail!("review_append_only: merge {commit} removed {}, a separate disposition event the branch still continues: the base it merged ran `atelier migrate dispositions`, and the branch holds a later event an older Atelier recorded. Leave the merge out and merge again, migrating before you commit, which folds the branch's events in{}{rewrites}. Next: git rebase --autostash --onto {commit}~1 {commit} && git merge --no-commit {side} && atelier migrate dispositions --apply && git add .atelier && git commit --no-edit", path.display(), merged_work_dropped(read, edge)?);
                }
                let effect = if *commit == head {
                    "which brings the record back (only a finding's disposition may change; record a new event for anything else)"
                } else {
                    "which rewrites the commits after it: get them reviewed again"
                };
                bail!("review_append_only: {} was rewritten beyond a finding's disposition, deleted or changed mode in {commit}; a later commit cannot undo it, so leave that commit out of the branch, {effect}{}. Next: git rebase --autostash --onto {commit}~1 {commit}", path.display(), merged_work_dropped(read, edge)?);
            }
        }
    }
    Ok(())
}

/// The other parent of a merge `edge` that removed the separate disposition
/// event at `path` while the branch still holds a later one, as merging a
/// migrated base into a branch an older Atelier recorded on does
/// (atelier-j5oq); `None` for any other refused change.
fn merged_migration(
    read: &mut git::ReadContext<'_>,
    edge: &git::CommitEdge,
    path: &Path,
    after: &Option<(String, String)>,
) -> Result<Option<String>> {
    let [side] = edge.sides.as_slice() else {
        return Ok(None);
    };
    if after.is_some() {
        return Ok(None);
    }
    let removed = matches!(
        parse_review_file(
            &read.blob(&edge.parent, path)?,
            path.strip_prefix(".atelier")?
        ),
        Ok(ReviewFile::RetiredDisposition(_))
    );
    Ok(removed.then(|| side.clone()))
}

/// What leaving out `edge`'s commit also drops when it is a merge: the
/// rebase replays only the commits after it onto its first parent, so the
/// work it merged in goes with it (atelier-zegg). Empty for any other commit.
fn merged_work_dropped(read: &git::ReadContext<'_>, edge: &git::CommitEdge) -> Result<String> {
    if edge.sides.is_empty() {
        return Ok(String::new());
    }
    let merged = String::from_utf8(git::git_output(
        read.repo,
        &[
            "rev-list",
            "--count",
            "--no-merges",
            &format!("{}~1..{}", edge.commit, edge.commit),
        ],
    )?)?;
    let merged = merged.trim().parse::<usize>().unwrap_or(0);
    Ok(format!(
        "; it is a merge, so this also drops the {merged} commit{} it merged in: merge or redo any of that work that should stay",
        if merged == 1 { "" } else { "s" }
    ))
}

/// Historical acceptance is valid only while review events remain
/// append-only, apart from findings' dispositions.
/// The check starts before the first commit to add a review event, which
/// may be a merge that resolved one into being (atelier-as7i).
pub fn append_only_history(read: &mut git::ReadContext<'_>) -> Result<()> {
    let head = read.pinned("HEAD")?;
    let history = read.path_log(&PathLog {
        start: head,
        paths: vec![".atelier/reviews/".to_owned()],
        listing: Listing::Additions,
        reverse: true,
    })?;
    let Some(first) = history.first() else {
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

/// The authors of the work on a branch since `base_commit`, against whom
/// review independence is judged by `review.current`.
pub fn author_identities(
    read: &mut git::ReadContext<'_>,
    base_commit: &str,
    ids: &BTreeSet<String>,
) -> Result<BTreeSet<String>> {
    author_identities_to(read, base_commit, ids, "HEAD", base_commit)
}
/// The author set review independence is judged against (atelier-0cj6):
/// the authors of the commits from `start` to `end` that change more than an
/// approval keeps current, leaving out the commits merges bring in from
/// `base`. A reviewer who authored work on the base that the branch merged in
/// is still independent of the branch. `merge`'s `review.current` passes the
/// branch's fork point with the base as `base`, so it counts the authors of a
/// merged side branch not on the base; `land` judges from history and passes the tip, so it leaves
/// out every merged side branch. `merge` can therefore refuse an approval
/// `land` accepts, never the reverse (atelier-p5qp).
fn author_identities_to(
    read: &mut git::ReadContext<'_>,
    start: &str,
    ids: &BTreeSet<String>,
    end: &str,
    base: &str,
) -> Result<BTreeSet<String>> {
    let edges = read.edges(start, end)?;
    let merged_base = base_merges(read, start, &edges, base)?
        .into_iter()
        .flat_map(|(_, only_in_sides)| only_in_sides)
        .collect::<HashSet<_>>();
    let mut authors = BTreeSet::new();
    let mut inspected = BTreeSet::new();
    for edge in edges
        .iter()
        .filter(|edge| !merged_base.contains(&edge.commit))
    {
        if !allowed_edge(read, edge, ids, start)? && inspected.insert(edge.commit.clone()) {
            add_author(read, &edge.commit, &mut authors)?;
        }
    }
    Ok(authors)
}
fn normalize(identity: &str) -> String {
    identity.trim().to_lowercase()
}
/// Whether `record`'s identity is none of `authors`, compared as
/// `review.current` compares them: trimmed and case-insensitive.
pub fn independent(record: &ReviewRecord, authors: &BTreeSet<String>) -> bool {
    identity_independent(&record.identity, authors)
}
/// Whether `identity`, such as a disposition's, is none of `authors`.
pub fn identity_independent(identity: &str, authors: &BTreeSet<String>) -> bool {
    !authors.contains(&normalize(identity))
}
fn ancestor(read: &mut git::ReadContext<'_>, earlier: &str, later: &str) -> bool {
    read.ancestor(earlier, later).unwrap_or(false)
}

/// Merge gates; the policy's `merge_checks` run in `checks_root`, the
/// checkout of the tree that merging would put on the base.
pub fn evaluate(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    owner: &str,
    base_commit: &str,
    checks_root: &Path,
) -> Result<Vec<GateNode>> {
    evaluate_inner(index, read, policy, owner, base_commit, Some(checks_root))
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
    let repo = read.repo;
    evaluate_inner(
        index,
        read,
        policy,
        owner,
        base_commit,
        complete.then_some(repo),
    )
}

fn evaluate_inner(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    owner: &str,
    base_commit: &str,
    checks_root: Option<&Path>,
) -> Result<Vec<GateNode>> {
    evaluate_scope(
        index,
        read,
        policy,
        owner,
        base_commit,
        match checks_root {
            Some(root) => EvaluationKind::Merge(root),
            None => EvaluationKind::Session,
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
enum EvaluationKind<'p> {
    Merge(&'p Path),
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
    kind: EvaluationKind<'_>,
    ids: BTreeSet<String>,
) -> Result<Vec<GateNode>> {
    let complete = matches!(kind, EvaluationKind::Merge(_));
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
                "review {} pins a commit outside the work branch, usually because the branch was rebased or reset after the review. To recover: 1. reset the branch to the tip it had before (find it with `git reflog`); 2. cherry-pick the code and planning commits made since, leaving out the review and evidence records they add, which pin rewritten commits (for a commit that mixes both, run `git cherry-pick -n <commit>`, then `git rm -f` each file that `git diff --cached --name-only --diff-filter=A -- .atelier/reviews .atelier/evidence` lists, then commit); 3. merge the base in with `git merge`; 4. get the branch reviewed again",
                record.header.id
            );
        }
        let Some(disposition) = record.disposition() else {
            continue;
        };
        if !ancestor(read, &disposition.commit, "HEAD") {
            bail!(
                "the disposition of finding {} pins a commit outside the work branch, usually because the branch was rebased or reset after it was recorded; reset the branch to its earlier tip (`git reflog`) or record the disposition again",
                record.header.id
            );
        }
        if let Some(resolution) = &disposition.resolution_commit {
            if resolution == &record.commit
                || !ancestor(read, &record.commit, resolution)
                || !ancestor(read, resolution, "HEAD")
            {
                bail!(
                    "finding {} resolution is not a later commit on the work branch",
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
        nodes.push(
            gate(
                owner,
                "work.terminal",
                terminal.is_empty(),
                if terminal.is_empty() {
                    "owner and all scoped work are closed".into()
                } else {
                    format!("open scoped work: {}", terminal.join(", "))
                },
                Vec::new(),
            )
            .with_gaps(terminal.iter().cloned().map(Gap::Open).collect()),
        );
        let mut open = BTreeSet::new();
        // A superseded child's blockers do not hold its parent (atelier-lq42).
        for id in ids.iter().filter(|id| {
            index
                .get_issue(id)
                .is_none_or(|issue| issue.status != "superseded")
        }) {
            open.extend(
                crate::objective_graph::dependency_closure(index, policy, id)?.blocking_ids(),
            );
        }
        nodes.push(
            gate(
                owner,
                "blockers.none_open",
                open.is_empty(),
                if open.is_empty() {
                    "no open transitive blockers".into()
                } else {
                    format!(
                        "open blockers: {}",
                        open.iter().cloned().collect::<Vec<_>>().join(", ")
                    )
                },
                Vec::new(),
            )
            .with_gaps(open.into_iter().map(Gap::Blocker).collect()),
        );
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
                    || if global {
                        approval_is_current(read, &record.commit, &ids, base_commit)
                            .unwrap_or(false)
                    } else {
                        current_on_branch(read, record, &ids, base_commit)
                    })
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
            match candidates
                .iter()
                .find(|record| matches!(record.event, ReviewEvent::Approval))
                .or(candidates.first())
            {
                // Only the owner's approvals count here, as `standing` judges
                // them, so it names the one considered and why it does not.
                None => not_current_reason(
                    &standing(index, read, owner, &ids, authors.clone(), base_commit)?,
                    &ids,
                ),
                Some(record) if matches!(record.event, ReviewEvent::Approval) => {
                    format!("current independent approval {}", record.header.id)
                }
                Some(record) => format!("trivial exemption {}", record.header.id),
            },
            support,
        ));
    }
    let mut unresolved = Vec::new();
    let mut gaps = Vec::new();
    let mut disposed = Vec::new();
    let mut approval_valid = BTreeMap::new();
    for finding in index.reviews().filter(|record| {
        applicable.contains(&record.target) && matches!(record.event, ReviewEvent::Finding { .. })
    }) {
        let ReviewEvent::Finding { .. } = &finding.event else {
            unreachable!()
        };
        let Some(disposition) = finding.disposition() else {
            unresolved.push(format!("{} has no disposition", finding.header.id));
            gaps.push(Gap::Undisposed(finding.header.id.clone()));
            continue;
        };
        disposed.push(finding.header.id.clone());
        let (decision, resolution_commit) = (disposition.decision, &disposition.resolution_commit);
        // Why a defer/no_action disposition's original acceptance could not
        // be checked; it then counts as not accepted (atelier-as7i).
        let mut untraced = None;
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
                                || disposition_retained(read, base_commit, finding)
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
                    disposition_independent(index, read, finding).unwrap_or_else(|error| {
                        untraced = Some(format!("{error:#}"));
                        false
                    })
                } else {
                    identity_independent(&disposition.identity, &authors)
                }
            }
        };
        if !accepted {
            let finding_id = finding.header.id.clone();
            let (reason, gap) = match (decision, resolution_commit, untraced) {
                (FindingDecision::FixNow, Some(_), _) => (
                    "fix_now resolution needs a fresh independent approval".to_owned(),
                    Gap::AwaitingApproval(finding_id),
                ),
                (FindingDecision::FixNow, None, _) => (
                    "fix_now disposition names no resolution commit".to_owned(),
                    Gap::Unresolved(finding_id),
                ),
                (_, _, Some(error)) => (
                    format!(
                        "its disposition could not be traced to the commit that set it: {error}"
                    ),
                    Gap::Untraced {
                        finding: finding_id,
                        error,
                    },
                ),
                _ => (
                    "disposition needs independent non-author acceptance".to_owned(),
                    Gap::Unaccepted(finding_id),
                ),
            };
            unresolved.push(format!("{}: {reason}", finding.header.id));
            gaps.push(gap);
        }
    }
    nodes.push(
        gate(
            owner,
            "findings.disposed",
            unresolved.is_empty(),
            if unresolved.is_empty() {
                "every finding has an accepted disposition".into()
            } else {
                unresolved.join("; ")
            },
            disposed,
        )
        .with_gaps(gaps),
    );
    nodes.extend(completion_claims(
        index,
        read,
        policy,
        base_commit,
        &claimed,
        historical,
    )?);
    let checks = if let EvaluationKind::Merge(checks_root) = kind {
        crate::workflow_validation::evaluate_policy_transition(
            crate::workflow_validation::ValidatorRequest {
                index,
                repo_root: checks_root,
                policy,
                issue_id: owner,
                validators: &policy.merge_checks,
            },
        )?
    } else {
        Vec::new()
    };
    // The results come in the order of the policy's merge_checks.
    for (check, definition) in checks.into_iter().zip(&policy.merge_checks) {
        nodes.push(validator_gate(
            index,
            policy,
            owner,
            check,
            Some(definition),
        )?);
    }
    let mut unique: BTreeMap<String, GateNode> = BTreeMap::new();
    for node in nodes {
        if let Some(existing) = unique.get_mut(&node.id) {
            // A check that ran twice reports why it failed, once; when every
            // run passed, the first reason says so.
            match (existing.passed, node.passed) {
                (true, false) => existing.reason = node.reason,
                (false, false) if existing.reason != node.reason => {
                    existing.reason.push_str(&format!("; {}", node.reason))
                }
                _ => {}
            }
            existing.passed &= node.passed;
            for gap in node.gaps {
                if !existing.gaps.contains(&gap) {
                    existing.gaps.push(gap);
                }
            }
            existing.fix = existing.fix.take().or(node.fix);
            existing.supports.extend(node.supports);
            existing.supports.sort();
            existing.supports.dedup();
        } else {
            unique.insert(node.id.clone(), node);
        }
    }
    Ok(unique.into_values().collect())
}

/// The fields the base's policy required on transitions into the issue's
/// status when its current done claim was made. The policy is the base's at
/// the point where the claim's commit and `base_commit` last met, never the
/// branch's own, so a branch cannot weaken the policy around a claim; for a
/// claim `base_commit` already holds, that is the commit that made it.
/// `None` for an uncommitted claim, or one whose earlier policy cannot be
/// read: it answers to the current policy.
fn fields_required_when_claimed(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    base_commit: &str,
    id: &str,
) -> Result<Option<BTreeSet<String>>> {
    let current = index
        .issue_record(id)
        .context("closed issue record missing")?;
    let head = read.pinned("HEAD")?;
    let path = std::path::Path::new(".atelier").join(atelier_records::issue_record_path(id));
    let path_text = path.to_string_lossy().into_owned();
    let commits = read.history(&["log", "--format=%H", &head, "--", &path_text])?;
    // The oldest commit, walking back, whose record still makes this claim.
    let mut introduced = None;
    for commit in commits.lines() {
        match read.issue_at(commit, id) {
            Ok(before)
                if crate::evidence_freshness::completion_claim_matches(&before, current, index) =>
            {
                introduced = Some(commit.to_string());
            }
            _ => break,
        }
    }
    let Some(commit) = introduced else {
        return Ok(None);
    };
    let Ok(base) = git::git_output(read.repo, &["merge-base", &commit, base_commit]) else {
        return Ok(None);
    };
    let base = String::from_utf8(base)?.trim().to_string();
    let Ok(then) = crate::base_policy::from_captured(read, &base) else {
        return Ok(None);
    };
    let issue = &current.issue;
    let Ok(workflow) = then.policy.workflow_for_issue_type(&issue.issue_type) else {
        return Ok(None);
    };
    Ok(Some(
        workflow
            .transitions
            .values()
            .filter(|transition| transition.to == issue.status)
            .flat_map(|transition| transition.required_fields.iter().cloned())
            .collect(),
    ))
}

/// Validate committed terminal claims without imposing session choreography.
fn completion_claims(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    policy: &WorkflowPolicy,
    base_commit: &str,
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
            nodes.push(
                gate(
                    id,
                    "blockers.transitive_none_open",
                    blockers.is_empty(),
                    if blockers.is_empty() {
                        "no open completion blockers".to_owned()
                    } else {
                        format!("open completion blockers: {}", blockers.join(", "))
                    },
                    Vec::new(),
                )
                .with_gaps(blockers.into_iter().map(Gap::Blocker).collect()),
            );
        }
    }
    for id in ids {
        let issue = index.require_issue(id)?;
        if !policy.issue_status_is_terminal(&issue.issue_type, &issue.status)?
            || issue.status == "superseded"
        {
            continue;
        }
        if let Some(failure) = crate::evidence_freshness::newest_failure(index, id)? {
            nodes.push(
                gate(
                    id,
                    "evidence.no_newer_failure",
                    false,
                    format!("newer failing evidence {failure}"),
                    vec![failure.clone()],
                )
                .with_gaps(vec![Gap::FailingEvidence(failure)]),
            );
        }
    }
    // Terminal records can also be edited directly; retain the trusted
    // policy's completion checks when evaluating their committed claims.
    let mut then_required = BTreeMap::<String, Option<BTreeSet<String>>>::new();
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
            let report = atelier_core::CloseReport::parse(
                index
                    .issue_record(id)
                    .context("closed issue record missing")?
                    .sections
                    .close_reason
                    .as_deref()
                    .unwrap_or_default(),
            );
            for field in &transition.required_fields {
                let present = match field.as_str() {
                    "close_reason" => !report.outcome.is_empty(),
                    // A report part the policy came to require after a claim
                    // was made does not unsettle it (atelier-sfsq): neither a
                    // claim the base already holds nor a child closed on its
                    // parent's branch before the base's policy changed. A
                    // held claim is judged by the policy of the commit that
                    // made it, not skipped, so a claim one commit back on a
                    // base with no upstream still needs its parts
                    // (atelier-uxrs).
                    field if atelier_core::ReportPart::from_field(field).is_some() => {
                        let present = atelier_core::ReportPart::from_field(field)
                            .and_then(|part| report.part(part))
                            .is_some();
                        if !present {
                            if !then_required.contains_key(id) {
                                let fields =
                                    fields_required_when_claimed(index, read, base_commit, id)?;
                                then_required.insert(id.clone(), fields);
                            }
                            if then_required[id]
                                .as_ref()
                                .is_some_and(|fields| !fields.contains(field))
                            {
                                continue;
                            }
                        }
                        present
                    }
                    other => bail!("unsupported required field '{other}'"),
                };
                nodes.push(gate(
                    id,
                    &format!("required_field.{field}"),
                    present,
                    if present {
                        format!("committed claim includes required field {field}")
                    } else if let Some(part) = atelier_core::ReportPart::from_field(field) {
                        format!(
                            "committed claim is missing required field {field}; add a `### {}` subsection to {id}'s Close Reason on its work branch",
                            part.title()
                        )
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
            for (check, definition) in checks.into_iter().zip(
                transition
                    .validators
                    .iter()
                    .filter(|definition| !historical || definition.builtin != "git.worktree_clean"),
            ) {
                nodes.push(validator_gate(index, policy, id, check, Some(definition))?);
            }
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
/// The query for the commit that introduced a review event: the commits
/// whose diff from their first parent adds the event's file, oldest first.
///
/// A merge that adds the file when neither parent has it, such as a
/// conflict resolution, is listed (atelier-as7i). So is a clean merge of a
/// branch that added it: asking for merge diffs turns history
/// simplification off, so Git lists the branch's commit and the merge, and
/// does not simplify the merge away. Callers take the first commit listed
/// as the introducer. Git orders the walk by committer date, so while dates
/// increase along history the branch's own commit, an ancestor of the
/// merge, comes first after `--reverse`.
fn introduced_query(head: &str, id: &str) -> PathLog {
    PathLog {
        start: head.to_owned(),
        paths: vec![format!(".atelier/reviews/{id}.md")],
        listing: Listing::Additions,
        reverse: true,
    }
}
/// The queries for where a reviewed scope starts, the first commit adding
/// one of its issue records, and for its owner's record history, both up to
/// the introducing `event` and oldest first.
fn scope_queries(index: &RecordIndex, target: &str, event: &str) -> Result<(PathLog, PathLog)> {
    let issue = index.require_issue(target)?;
    let owner = issue.parent_id.as_deref().unwrap_or(target);
    let ids = scope(index, owner)?;
    let history = PathLog {
        start: event.to_owned(),
        paths: ids
            .iter()
            .map(|id| format!(".atelier/issues/{id}.md"))
            .collect(),
        listing: Listing::Additions,
        reverse: true,
    };
    let updates = PathLog {
        start: event.to_owned(),
        paths: vec![format!(".atelier/issues/{owner}.md")],
        listing: Listing::Changes,
        reverse: true,
    };
    Ok((history, updates))
}
/// Whether an approval's identity is independent of the work's authors at
/// the commit that introduced it, which must still hold it unchanged.
fn original_independent(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    record: &ReviewRecord,
) -> Result<bool> {
    let event_path = format!(".atelier/reviews/{}.md", record.header.id);
    let head = read.pinned("HEAD")?;
    let introduced = read.path_log(&introduced_query(&head, &record.header.id))?;
    let event = introduced.first().context("review event history missing")?;
    let path = Path::new(&event_path);
    if read.entry(event, path)? != read.entry("HEAD", path)? {
        bail!(
            "review_append_only: {} differs from its original event",
            record.header.id
        );
    }
    independent_at(index, read, &record.target, &record.identity, event)
}
/// Whether a finding's disposition was set by an identity independent of
/// the work's authors, judged at the commit that set it rather than at any
/// commit the disposition names (ADR 0025): the oldest commit since which
/// the finding's file has held this disposition, or, for one
/// `atelier migrate dispositions` folded in, the commit that added the
/// separate event it came from, whose fields it must match.
fn disposition_independent(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    finding: &ReviewRecord,
) -> Result<bool> {
    let disposition = finding
        .disposition()
        .with_context(|| format!("finding {} has no disposition", finding.header.id))?;
    let head = read.pinned("HEAD")?;
    if let Some(original) = &disposition.migrated_from {
        // Read from a separate event still in the tree, as an unmigrated
        // repository holds it: it was set where that event was added.
        let path = std::path::PathBuf::from(format!(".atelier/reviews/{original}.md"));
        if read.entry(&head, &path)?.is_some() {
            let event = migrated_event_commit(read, &head, finding, original)?;
            return independent_at(index, read, &finding.target, &disposition.identity, &event);
        }
    }
    let mut set_at = disposition_set_at(read, &head, finding)?;
    // Set by the commit that removed the event it names, the migration: it
    // was set where that event was added. Once the file has held another
    // disposition, the later setting counts, even of the same bytes.
    if let Some(original) = &disposition.migrated_from {
        let path = std::path::PathBuf::from(format!(".atelier/reviews/{original}.md"));
        let parent = read.first_parent(&set_at)?;
        let removed_here = match &parent {
            Some(parent) => {
                read.entry(parent, &path)?.is_some() && read.entry(&set_at, &path)?.is_none()
            }
            None => false,
        };
        if removed_here {
            set_at = migrated_event_commit(read, &head, finding, original)?;
        }
    }
    independent_at(index, read, &finding.target, &disposition.identity, &set_at)
}
/// The commit that added `original`, the separate disposition event the
/// migration folded into `finding`, whose fields the disposition must match.
fn migrated_event_commit(
    read: &mut git::ReadContext<'_>,
    head: &str,
    finding: &ReviewRecord,
    original: &str,
) -> Result<String> {
    let introduced = read.path_log(&introduced_query(head, original))?;
    let event = introduced
        .first()
        .with_context(|| format!("disposition event {original} is not in history"))?
        .clone();
    let path = std::path::PathBuf::from(format!(".atelier/reviews/{original}.md"));
    let folded =
        match parse_review_file(&read.blob(&event, &path)?, path.strip_prefix(".atelier")?)? {
            ReviewFile::RetiredDisposition(old) if old.finding == finding.header.id => old.folded(),
            _ => bail!(
                "{original} is not a disposition event of finding {}",
                finding.header.id
            ),
        };
    if Some(&folded) != finding.disposition() {
        bail!(
            "review_append_only: the disposition of finding {} differs from {original}, the event it was migrated from",
            finding.header.id
        );
    }
    Ok(event)
}
/// The oldest commit, walking back from `head`, since which `finding`'s
/// file has held its current disposition.
fn disposition_set_at(
    read: &mut git::ReadContext<'_>,
    head: &str,
    finding: &ReviewRecord,
) -> Result<String> {
    let path = std::path::PathBuf::from(format!(".atelier/reviews/{}.md", finding.header.id));
    let relative = path.strip_prefix(".atelier")?.to_owned();
    let changes = read.path_log(&PathLog {
        start: head.to_owned(),
        paths: vec![path.display().to_string()],
        listing: Listing::Changes,
        reverse: false,
    })?;
    let mut set_at = None;
    for commit in changes.iter() {
        let held = read
            .blob(commit, &path)
            .ok()
            .and_then(|text| parse_review_record(&text, &relative).ok());
        if held.as_ref().and_then(ReviewRecord::disposition) != finding.disposition() {
            break;
        }
        set_at = Some(commit.clone());
    }
    set_at.with_context(|| format!("finding {} history missing", finding.header.id))
}
/// Whether `identity` is independent of the authors of `target`'s work from
/// where its branch started up to `event`.
fn independent_at(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    target: &str,
    identity: &str,
    event: &str,
) -> Result<bool> {
    let issue = index.require_issue(target)?;
    let ids = scope(index, issue.parent_id.as_deref().unwrap_or(target))?;
    let head = read.pinned("HEAD")?;
    let (history, updates) = scope_queries(index, target, event)?;
    let history = read.path_log(&history)?;
    let start = history.first().context("review predates its issue scope")?;
    let owner = issue.parent_id.as_deref().unwrap_or(target);
    let updates = read.path_log(&updates)?;
    let branch_start = updates.iter().find(|commit| {
        read.issue_at(commit, owner)
            .is_ok_and(|record| record.issue.workflow_branch.is_some())
    });
    // History records no base for the branch, so `head` stands for it: the
    // commits of every branch merged into this one are left out, including
    // side branches never on the base, which `review.current` would count
    // (atelier-p5qp).
    let authors = if let Some(origin) = branch_start {
        match read.first_parent(origin)? {
            Some(parent) => author_identities_to(read, &parent, &ids, event, &head)?,
            None => {
                let mut authors = author_identities_to(read, origin, &ids, event, &head)?;
                add_author(read, origin, &mut authors)?;
                authors
            }
        }
    } else {
        let mut authors = author_identities_to(read, start, &ids, event, &head)?;
        add_author(read, start, &mut authors)?;
        authors
    };
    Ok(identity_independent(identity, &authors))
}

fn retained(read: &mut git::ReadContext<'_>, base: &str, record: &ReviewRecord) -> bool {
    let path = std::path::PathBuf::from(format!(".atelier/reviews/{}.md", record.header.id));
    let old = read.entry(base, &path).ok().flatten();
    old.is_some() && old == read.entry("HEAD", &path).ok().flatten()
}

/// Whether `base` already held `finding`'s disposition: in the finding's
/// file, or, for one `atelier migrate dispositions` folded in, as the
/// separate event it came from.
fn disposition_retained(
    read: &mut git::ReadContext<'_>,
    base: &str,
    finding: &ReviewRecord,
) -> bool {
    let Some(disposition) = finding.disposition() else {
        return false;
    };
    let at_base = |read: &mut git::ReadContext<'_>, id: &str| {
        let path = std::path::PathBuf::from(format!(".atelier/reviews/{id}.md"));
        read.entry(base, &path).ok().flatten()?;
        parse_review_file(
            &read.blob(base, &path).ok()?,
            path.strip_prefix(".atelier").ok()?,
        )
        .ok()
    };
    let held = match at_base(read, &finding.header.id) {
        Some(ReviewFile::Event(record)) => record.disposition().cloned(),
        _ => None,
    };
    held.as_ref() == Some(disposition)
        || disposition.migrated_from.as_deref().is_some_and(|event| {
            matches!(at_base(read, event), Some(ReviewFile::RetiredDisposition(old)) if old.folded() == *disposition)
        })
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
    let end = read.claim_endpoint(index, target, &approval.commit)?;
    approval_is_current_to(read, &approval.commit, &ids, &end, base)
}

fn retired_review_cluster(
    read: &mut git::ReadContext<'_>,
    edge: &git::CommitEdge,
    path: &Path,
) -> Result<bool> {
    // A base exemption has no target to retire with, so it is never removed.
    let Some(target) = parse_review_file(
        &read.blob(&edge.parent, path)?,
        path.strip_prefix(".atelier")?,
    )?
    .target()
    .map(str::to_owned) else {
        return Ok(false);
    };
    let issue_path = std::path::PathBuf::from(format!(".atelier/issues/{target}.md"));
    if read.entry(&edge.commit, &issue_path)?.is_some() {
        return Ok(false);
    }
    let issue = read.issue_at(&edge.parent, &target)?;
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
        let other = parse_review_file(
            &read.blob(&edge.parent, other_path)?,
            other_path.strip_prefix(".atelier")?,
        )?;
        if other.target() == Some(target.as_str())
            && read.entry(&edge.commit, other_path)?.is_some()
        {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_resolutions_keep_both_sides_and_add_nothing_even_with_duplicates() {
        let list = |items: &[&str]| {
            serde_yaml::Value::Sequence(items.iter().map(|item| (*item).into()).collect())
        };
        let (base, ours, theirs) = (
            list(&["dup", "dup"]),
            list(&["dup", "dup", "branch"]),
            list(&["dup", "dup", "base"]),
        );
        assert!(merged(
            &base,
            &ours,
            &theirs,
            &list(&["dup", "dup", "branch", "base"]),
            ""
        ));
        assert!(!merged(
            &base,
            &ours,
            &theirs,
            &list(&["dup", "branch", "base", "smuggled"]),
            ""
        ));
        assert!(!merged(
            &base,
            &ours,
            &theirs,
            &list(&["dup", "dup", "branch"]),
            ""
        ));
        assert!(!merged(
            &base,
            &ours,
            &theirs,
            &list(&["dup", "branch", "base", "base"]),
            ""
        ));
    }

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
