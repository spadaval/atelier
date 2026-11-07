//! Review integrity, approval freshness and derived merge gate graph nodes.
use crate::{
    git_freshness as git,
    path_log::{Listing, PathLog},
    record_index::RecordIndex,
    workflow_policy::WorkflowPolicy,
};
use anyhow::{bail, Context, Result};
use atelier_core::{FindingDecision, ReviewEvent, ReviewRecord};
use atelier_records::review::{parse_review_file, parse_review_record, ReviewFile};
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
    /// A finding recommended defer or no_action with no recorded decision.
    Undisposed(String),
    /// A fix_now finding with no current approval after it: of a later
    /// commit and recorded later, or, for an older Atelier's fix_now
    /// disposition naming a resolution commit, at or after that commit.
    AwaitingApproval(String),
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
pub(crate) fn validator_gate(
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
pub fn scope(index: &RecordIndex, owner: &str) -> Result<BTreeSet<String>> {
    let mut ids = crate::objective_graph::issue_descendant_ids(index, owner)?;
    ids.insert(owner.to_owned());
    Ok(ids)
}

/// The open work in `owner`'s scope that merging marks done, owner first:
/// each item of the `work` workflow not yet in a done status, so a
/// superseded child and an item an older Atelier closed are left as they are
/// (atelier-jnrh). `review request` asks each for ticked Done when lines and
/// a Report.
pub fn open_work(index: &RecordIndex, policy: &WorkflowPolicy, owner: &str) -> Result<Vec<String>> {
    let mut open = Vec::new();
    let ids = scope(index, owner)?;
    for id in std::iter::once(owner).chain(ids.iter().map(String::as_str).filter(|id| *id != owner))
    {
        let issue = index.require_issue(id)?;
        if policy.merge_done_status(&issue.issue_type).is_some()
            && !policy.issue_status_is_terminal(&issue.issue_type, &issue.status)?
        {
            open.push(id.to_owned());
        }
    }
    Ok(open)
}

/// Whether `edge`'s change to the review record at `path` leaves every
/// event as it was: it adds a new event, sets or changes a finding's
/// disposition and nothing else (ADR 0025), or removes a separate
/// disposition event the way `review disposition` and
/// `atelier migrate dispositions` do, in a commit whose finding carries its
/// own disposition and keeps none of its events. A separate event is never
/// added back once removed: either would bring back a decision that was
/// replaced, which the old append-only events refused (atelier-pfbp).
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
/// ADR 0025 is rewritten in the new version when it is first disposed. A
/// finding about an issue stays about it, and one about a branch stays about
/// that branch (atelier-4nsl).
pub(crate) fn disposition_only(before: &str, after: &str, relative: &Path) -> bool {
    let (Ok(mut before), Ok(mut after)) = (
        parse_review_file(before, relative),
        parse_review_file(after, relative),
    ) else {
        return false;
    };
    let (mut before, mut after) = match (&mut before, &mut after) {
        (ReviewFile::Event(before), ReviewFile::Event(after))
        | (ReviewFile::BranchEvent(before), ReviewFile::BranchEvent(after)) => {
            (before.clone(), after.clone())
        }
        _ => return false,
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

pub fn approval_is_current(read: &mut git::ReadContext<'_>, pin: &str, base: &str) -> Result<bool> {
    approval_is_current_to(read, pin, "HEAD", base)
}
/// The one rule for what an approval or trivial exemption covers
/// (atelier-n642, atelier-3jdm): the code. Pinned at `pin`, it stays current
/// at `end` while no commit after the pin changes code, which is everything
/// outside the tracker records (`git::is_record_path`), so workflow policy
/// and config count. Record edits never make it stale, whatever record they
/// touch. A merge that brings in only commits already on `base` keeps it
/// current when it matches Git's own merge of its parents everywhere outside
/// the records: those commits were reviewed on their own branches. Each
/// commit is checked, so changing code and then restoring it does not
/// recover an approval. `merge`'s `review.current`, the session gates and
/// `land`'s check of done claims on the base all ask this; `end` must
/// contain `pin`.
pub fn approval_is_current_to(
    read: &mut git::ReadContext<'_>,
    pin: &str,
    end: &str,
    base: &str,
) -> Result<bool> {
    Ok(work_edges(read, pin, end, base)?
        .iter()
        .all(|(edge, clean_base_merge)| edge_allowed(edge, *clean_base_merge)))
}

/// Whether one edge after a pin keeps an approval current: it is a clean
/// merge of the base, or it changes no code. The one edge check
/// `approval_is_current_to` and `approval_stale_edges` share.
fn edge_allowed(edge: &git::CommitEdge, clean_base_merge: bool) -> bool {
    clean_base_merge || !changes_code(edge)
}

/// Whether `edge` changes anything outside the tracker records.
fn changes_code(edge: &git::CommitEdge) -> bool {
    edge.paths.iter().any(|path| !git::is_record_path(path))
}

/// The code among `paths`: those outside the tracker records.
fn code_paths(paths: &[std::path::PathBuf]) -> Vec<std::path::PathBuf> {
    paths
        .iter()
        .filter(|path| !git::is_record_path(path))
        .cloned()
        .collect()
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
    /// What a new review has to read, which is code only: for a merge, the
    /// code it resolved or edited by hand, or all the code it changed when it
    /// did neither; for any other commit, the code it changed.
    pub paths: Vec<std::path::PathBuf>,
    /// The code a merge resolved or edited beyond Git's own merge.
    pub by_hand: Vec<std::path::PathBuf>,
}
/// Whether a merge brings in only commits already on the base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseMerge {
    Base,
    Other,
}

/// Every commit of the branch (not of the base) after `pin` that makes the
/// approval stale at `end`, oldest first, each with the code a new review has
/// to look at; empty while the approval is current. It applies the rule of
/// [`approval_is_current_to`] to each edge, so what `merge --sync` reports as
/// needing review is exactly what makes the approval stale, but unlike that
/// check it computes the paths. For a merge they are the code it resolved by
/// hand or edited beyond Git's own merge; for any other commit, the code it
/// changed.
pub fn approval_stale_edges(
    read: &mut git::ReadContext<'_>,
    pin: &str,
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
    for (edge, clean_base_merge) in &edges {
        if ancestor(read, &edge.commit, base) && !edge_allowed(edge, *clean_base_merge) {
            unaccepted_base.insert(edge.commit.clone());
        }
    }
    // Edges come newest first.
    for (edge, clean_base_merge) in edges.into_iter().rev() {
        if ancestor(read, &edge.commit, base) {
            continue;
        }
        let brings_unaccepted = !edge.sides.is_empty()
            && !unaccepted_base.is_empty()
            && read
                .only_in(&edge.sides, &edge.parent)?
                .iter()
                .any(|commit| unaccepted_base.contains(commit));
        if !brings_unaccepted && edge_allowed(&edge, clean_base_merge) {
            continue;
        }
        let (paths, by_hand) = if edge.sides.is_empty() {
            (code_paths(&edge.paths), Vec::new())
        } else {
            let by_hand = code_paths(&read.hand_resolved_paths(&edge.commit)?.unwrap_or_default());
            if by_hand.is_empty() {
                (code_paths(&edge.paths), by_hand)
            } else {
                (by_hand.clone(), by_hand)
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

/// Whether `record`, an approval or trivial exemption on a work branch that
/// left the base at `fork_point`, is current at the branch's HEAD as
/// `merge`'s `review.current` judges it. An approval from before the base
/// was last merged in still counts; one from before the branch left the base
/// does not.
pub fn current_on_branch(
    read: &mut git::ReadContext<'_>,
    record: &ReviewRecord,
    fork_point: &str,
) -> bool {
    (ancestor(read, fork_point, &record.commit) || !ancestor(read, &record.commit, fork_point))
        && approval_is_current(read, &record.commit, fork_point).unwrap_or(false)
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
    /// The current approval or exemption, else the newest stale one.
    pub record: Option<&'i ReviewRecord>,
    /// Why `record` does not count; `None` while it is current.
    pub not_current: Option<NotCurrent>,
}

/// [`approval_state`] with the record it rests on and why it does not count
/// (atelier-uv5d, atelier-cisa): the one judgement `review.current`, `merge
/// --sync` and `review list` explain. A reviewer's identity is recorded but
/// not compared with the work's Git authors (atelier-md0v).
pub fn approval_standing<'i>(
    index: &'i RecordIndex,
    read: &mut git::ReadContext<'_>,
    owner: &str,
    fork_point: &str,
) -> Result<Standing<'i>> {
    standing_among(
        index.reviews().filter(|record| {
            record.target == owner
                && matches!(
                    record.event,
                    ReviewEvent::Approval | ReviewEvent::TrivialExemption
                )
        }),
        read,
        fork_point,
    )
}

/// The standing [`approval_standing`] judges among `candidates`, the
/// approvals and trivial exemptions that could cover the work: the current
/// one, else the newest stale one. The light path judges a branch's
/// approvals with it too (atelier-4nsl).
pub(crate) fn standing_among<'i>(
    candidates: impl Iterator<Item = &'i ReviewRecord>,
    read: &mut git::ReadContext<'_>,
    fork_point: &str,
) -> Result<Standing<'i>> {
    let newer = |record: &ReviewRecord, than: Option<&ReviewRecord>| {
        than.is_none_or(|newest| {
            (record.header.created_at, &record.header.id)
                > (newest.header.created_at, &newest.header.id)
        })
    };
    let mut stale: Option<&ReviewRecord> = None;
    for record in candidates {
        if current_on_branch(read, record, fork_point) {
            return Ok(Standing {
                state: ApprovalState::Current,
                record: Some(record),
                not_current: None,
            });
        }
        if newer(record, stale) {
            stale = Some(record);
        }
    }
    Ok(match stale {
        Some(record) => {
            let edges = approval_stale_edges(read, &record.commit, "HEAD", fork_point)?;
            Standing {
                state: ApprovalState::Stale,
                record: Some(record),
                not_current: Some(if edges.is_empty() {
                    NotCurrent::PredatesFork
                } else {
                    NotCurrent::Stale(edges)
                }),
            }
        }
        None => Standing {
            state: ApprovalState::Missing,
            record: None,
            not_current: None,
        },
    })
}

/// `review.current`'s reason when it fails: the approval it considered and
/// why it does not count, or that there is none.
pub(crate) fn not_current_reason(standing: &Standing<'_>) -> String {
    const LISTED: usize = 3;
    let (Some(record), Some(why)) = (standing.record, &standing.not_current) else {
        return "no independent approval or trivial exemption".to_owned();
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
        NotCurrent::PredatesFork => format!(
            "{name} is stale: it predates the point where the branch left the base, which already holds the work since"
        ),
        NotCurrent::Stale(edges) => {
            let first = &edges[0];
            let cause = match first.merge {
                Some(kind) => format!(
                    "{} {} {}",
                    match kind {
                        BaseMerge::Base => "base merge",
                        BaseMerge::Other => "merge",
                    },
                    short(&first.commit),
                    if first.by_hand.is_empty() {
                        format!("changed {}", listed(&first.paths))
                    } else {
                        format!("resolved or edited {} by hand", listed(&first.by_hand))
                    }
                ),
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
/// judged from history as the base's claim gates judge it (atelier-n90q): it
/// is as first committed, and every finding on the scope pinned at or before
/// it is settled by the rules `findings.disposed` applies (atelier-md0v). A
/// defer or no_action decision settles a finding; a fix_now finding is
/// settled by this approval when it reviewed a later commit holding the
/// finding's record and was recorded after it, or, for an older Atelier's
/// fix_now disposition, when its resolution commit is at or before the pin.
/// The reviewer's identity is
/// recorded but not compared with the work's Git authors. `land` on the base
/// counts only these approvals as covering commits.
pub fn approval_accepted(
    index: &RecordIndex,
    read: &mut git::ReadContext<'_>,
    record: &ReviewRecord,
) -> Result<bool> {
    let intact = match record.event {
        ReviewEvent::TrivialExemption => true,
        ReviewEvent::Approval => original_intact(read, record).unwrap_or(false),
        _ => false,
    };
    if !intact {
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
        let settled = match settlement(finding) {
            Settlement::Decided => true,
            Settlement::Fix => fix_resolved(read, finding, record),
            Settlement::Undecided => false,
        };
        if !settled {
            return Ok(false);
        }
    }
    Ok(true)
}

/// What settles a finding (atelier-md0v).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Settlement {
    /// A recorded defer or no_action decision.
    Decided,
    /// An approval after the fix: a finding recommended fix_now with no
    /// decision, or one whose decision is fix_now.
    Fix,
    /// Nothing yet: a finding recommended defer or no_action needs a
    /// recorded decision, even if it was fixed instead.
    Undecided,
}

pub(crate) fn settlement(finding: &ReviewRecord) -> Settlement {
    match (&finding.event, finding.disposition()) {
        (_, Some(disposition)) if disposition.decision == FindingDecision::FixNow => {
            Settlement::Fix
        }
        (_, Some(_)) => Settlement::Decided,
        (
            ReviewEvent::Finding {
                recommendation: FindingDecision::FixNow,
                ..
            },
            None,
        ) => Settlement::Fix,
        _ => Settlement::Undecided,
    }
}

/// The approval that resolves the fix_now `finding` by [`fix_resolved`]'s
/// rule, an approval of its owner, its item or its item's parent, for views
/// that say where a finding stands; whether that approval is still current
/// is `review.current`'s question. `None` for a finding that is not fix_now,
/// or that no approval since resolves.
pub fn fix_approval<'i>(
    index: &'i RecordIndex,
    read: &mut git::ReadContext<'_>,
    owner: &str,
    finding: &ReviewRecord,
) -> Option<&'i ReviewRecord> {
    if settlement(finding) != Settlement::Fix {
        return None;
    }
    let parent = index
        .get_issue(&finding.target)
        .and_then(|issue| issue.parent_id);
    index
        .reviews()
        .filter(|approval| {
            approval.event == ReviewEvent::Approval
                && (approval.target == owner
                    || approval.target == finding.target
                    || parent.as_deref() == Some(approval.target.as_str()))
        })
        .find(|approval| fix_resolved(read, finding, approval))
}

/// Whether `approval` resolves the fix_now `finding`: it reviewed a later
/// commit than the finding, one that holds the finding's record, and was
/// recorded after it, so an approval that never saw the finding cannot
/// resolve it, as a parallel reviewer's approval of a head that held no
/// such record would (finding atelier-6xiw). An older Atelier's fix_now
/// disposition naming a resolution commit is resolved, as it was, by an
/// approval at or after that commit.
pub(crate) fn fix_resolved(
    read: &mut git::ReadContext<'_>,
    finding: &ReviewRecord,
    approval: &ReviewRecord,
) -> bool {
    match finding
        .disposition()
        .and_then(|disposition| disposition.resolution_commit.as_ref())
    {
        Some(resolution) => ancestor(read, resolution, &approval.commit),
        None => {
            let path =
                std::path::PathBuf::from(format!(".atelier/reviews/{}.md", finding.header.id));
            approval.event == ReviewEvent::Approval
                && approval.commit != finding.commit
                && approval.header.created_at > finding.header.created_at
                && ancestor(read, &finding.commit, &approval.commit)
                && read
                    .entry(&approval.commit, &path)
                    .is_ok_and(|entry| entry.is_some())
        }
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
/// whether it is a clean merge of commits already on `base`: one that matches
/// Git's own merge of its parents everywhere outside the records. The base
/// commits a clean merge brings in are left out; a merge that resolved or
/// edited code by hand is checked like any other commit, and so are the base
/// commits it brings in.
fn work_edges(
    read: &mut git::ReadContext<'_>,
    pin: &str,
    end: &str,
    base: &str,
) -> Result<Vec<(git::CommitEdge, bool)>> {
    let edges = read.edges(pin, end)?;
    let mut merged_base = HashSet::new();
    let mut clean = HashSet::new();
    for (edge, only_in_sides) in base_merges(read, pin, &edges, base)? {
        let by_hand = read.hand_resolved_paths(&edge.commit)?;
        if !by_hand.is_some_and(|paths| paths.iter().all(|path| git::is_record_path(path))) {
            continue;
        }
        merged_base.extend(only_in_sides);
        clean.insert(edge.commit.clone());
    }
    Ok(edges
        .iter()
        .filter(|edge| !merged_base.contains(&edge.commit))
        .map(|edge| (edge.clone(), clean.contains(&edge.commit)))
        .collect())
}

/// How a review record stood when a change to it joined the base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Held {
    /// The base held it.
    OnBase,
    /// It was committed before the change and is final once committed: an
    /// approval or trivial exemption, or any record when the base takes
    /// each commit by itself or cannot be read.
    Committed,
}

/// Whether a review file is an approval or a trivial exemption, either
/// kind, which stays as first committed wherever it is (atelier-ilba): a
/// hand edit of its commit, target or identity would move it onto work its
/// reviewer never saw. A file that does not parse is neither.
fn final_once_committed(text: &str, relative: &Path) -> bool {
    match parse_review_file(text, relative) {
        Ok(ReviewFile::Event(record) | ReviewFile::BranchEvent(record)) => matches!(
            record.event,
            ReviewEvent::Approval | ReviewEvent::TrivialExemption
        ),
        Ok(ReviewFile::BaseExemption(_)) => true,
        _ => false,
    }
}

/// Whether `commit` holds an approval or a trivial exemption at `path`, as
/// [`final_once_committed`] reads it.
pub fn approval_or_exemption_at(repo: &Path, commit: &str, path: &Path) -> bool {
    let Ok(relative) = path.strip_prefix(".atelier") else {
        return false;
    };
    git::blob(repo, commit, path).is_ok_and(|text| final_once_committed(&text, relative))
}

/// Where review records stop being correctable: the policy's base branch
/// (atelier-ilba). Until a finding or another review record reaches the
/// base, any agent on the branch may edit or delete it; once it is there,
/// it stays as it is apart from a finding's disposition. Approvals and
/// trivial exemptions stay as first committed wherever they are. Each change is judged against the base as
/// it stood where the change joined it, so a correction made on a branch is
/// not refused after it merges.
pub struct ReviewBase {
    name: String,
    /// The base branch's tip, when it resolves.
    tip: Option<String>,
    /// The base's first-parent line from its tip, each commit with its
    /// distance from the tip, read when first needed.
    line: Option<std::collections::HashMap<String, usize>>,
}

impl ReviewBase {
    pub fn new(repo: &Path, branch: &atelier_workflow::BranchLifecycleConfig) -> Self {
        let tip = git::git_output(
            repo,
            &[
                "rev-parse",
                "--verify",
                "--quiet",
                &format!("refs/heads/{}^{{commit}}", branch.base_branch),
            ],
        )
        .ok()
        .map(|out| String::from_utf8_lossy(&out).trim().to_owned())
        .filter(|tip| !tip.is_empty());
        Self {
            name: branch.base_branch.clone(),
            tip,
            line: None,
        }
    }

    /// The base branch's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// How `path` stood on the base when `commit`'s change joined it, or,
    /// for a commit not on the base yet, stands on it now; `None` when it
    /// was not there:
    ///
    /// - A commit a merge brought into the base joined it at that merge,
    ///   which held the base as the merge's first parent.
    /// - A commit on the base's first-parent line joined it by itself, at
    ///   its own first parent.
    /// - A commit not on the base yet is judged against the base's tip.
    ///
    /// `atelier merge` always integrates with a merge commit, so this holds
    /// under every policy (atelier-jnrh); a commit an earlier fast-forward
    /// put on the base's first-parent line was judged at its own first
    /// parent then and still is. Without a base branch to read, every commit
    /// is judged at its own first parent: a review record is final once
    /// committed.
    pub fn held(
        &mut self,
        read: &mut git::ReadContext<'_>,
        commit: &str,
        path: &Path,
    ) -> Result<Option<Held>> {
        let (before, held) = match self.tip.clone() {
            Some(tip) => {
                if read.ancestor(commit, &tip)? {
                    let joined = self.joined_at(read, commit, &tip)?;
                    (read.first_parent(&joined)?, Held::OnBase)
                } else {
                    (Some(tip), Held::OnBase)
                }
            }
            _ => (read.first_parent(commit)?, Held::Committed),
        };
        Ok(match before {
            Some(before) if read.entry(&before, path)?.is_some() => Some(held),
            _ => None,
        })
    }

    /// Whether a review record is final once committed, as [`Self::held`]
    /// judges every commit without a base branch to read.
    pub fn final_once_committed(&self) -> bool {
        self.tip.is_none()
    }

    /// Whether a change to `path` in the checkout, once committed, changes a
    /// record [`Self::held`] would find on the base: whether the base's tip
    /// holds it, or HEAD when a review record is final once committed.
    pub fn holds_uncommitted(&self, repo: &Path, path: &Path) -> bool {
        let at = match &self.tip {
            Some(tip) if !self.final_once_committed() => tip.as_str(),
            _ => "HEAD",
        };
        git::git_output(
            repo,
            &["cat-file", "-e", &format!("{at}:{}", path.display())],
        )
        .is_ok()
    }

    /// The commit on the base's first-parent line that brought `commit` in:
    /// `commit` itself when it is on that line, else the oldest one there
    /// that descends from it, the merge that took it in.
    fn joined_at(
        &mut self,
        read: &mut git::ReadContext<'_>,
        commit: &str,
        tip: &str,
    ) -> Result<String> {
        let commit = read.commit_id(commit)?;
        if self.line.is_none() {
            let listed = git::git_output(read.repo, &["rev-list", "--first-parent", tip])?;
            self.line = Some(
                String::from_utf8(listed)?
                    .lines()
                    .enumerate()
                    .map(|(distance, commit)| (commit.to_owned(), distance))
                    .collect(),
            );
        }
        let line = self.line.as_ref().expect("first-parent line read");
        if line.contains_key(&commit) {
            return Ok(commit);
        }
        let descendants = String::from_utf8(git::git_output(
            read.repo,
            &["rev-list", "--ancestry-path", &format!("{commit}..{tip}")],
        )?)?;
        descendants
            .lines()
            .filter_map(|descendant| line.get(descendant).map(|distance| (distance, descendant)))
            .max_by_key(|(distance, _)| **distance)
            .map(|(_, merge)| merge.to_owned())
            .with_context(|| format!("no commit on the base's first-parent line holds {commit}"))
    }
}

/// Keeps review records on the base as they are, apart from a finding's
/// disposition, and approvals and trivial exemptions as first committed,
/// and lets any other review record be corrected or deleted (atelier-ilba). Every commit from `base_commit` to HEAD is checked, each
/// against the base as it stood where the commit joined it ([`ReviewBase`]).
///
/// A later commit cannot undo a rewrite, since every commit in the range is
/// checked, so the recovery is a rebase that leaves out the commit that made
/// it, named by its ID so it holds wherever HEAD has moved since, as other
/// fixes commit (atelier-v4rd, atelier-61il). At the tip it only drops it,
/// which brings the record back; below, it rewrites the commits after it.
pub fn append_only(
    read: &mut git::ReadContext<'_>,
    base_commit: &str,
    branch: &atelier_workflow::BranchLifecycleConfig,
) -> Result<()> {
    let head = read.commit_id("HEAD")?;
    let mut base = ReviewBase::new(read.repo, branch);
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
                // A record not yet on the base may change in any way that
                // leaves it a regular file, or go, unless it is an approval
                // or exemption. Adding one is judged as before.
                let regular = after.as_ref().is_none_or(|(mode, _)| mode == "100644");
                let relative = path.strip_prefix(".atelier")?;
                let held = match before {
                    Some(_) if final_once_committed(&read.blob(&edge.parent, path)?, relative) => {
                        Some(Held::Committed)
                    }
                    Some(_) => base.held(read, &edge.commit, path)?,
                    None => Some(Held::Committed),
                };
                if regular && held.is_none() {
                    continue;
                }
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
                let change = match held {
                    Some(Held::OnBase) => format!(
                        "was on {} already when {commit} rewrote it beyond a finding's disposition, deleted it or changed its mode; a review record stays as it is once it reaches {}, and a later commit cannot undo that",
                        base.name(),
                        base.name()
                    ),
                    _ => format!("was rewritten beyond a finding's disposition, deleted or changed mode in {commit}; a later commit cannot undo it"),
                };
                bail!("review_append_only: {} {change}, so leave that commit out of the branch, {effect}{}. Next: git rebase --autostash --onto {commit}~1 {commit}", path.display(), merged_work_dropped(read, edge)?);
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

/// Historical acceptance is valid only while review events on the base
/// remain append-only, apart from findings' dispositions ([`append_only`]).
/// The check starts before the first commit to add a review event, which
/// may be a merge that resolved one into being (atelier-as7i).
pub fn append_only_history(
    read: &mut git::ReadContext<'_>,
    branch: &atelier_workflow::BranchLifecycleConfig,
) -> Result<()> {
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
    append_only(read, &pin, branch)
}

pub(crate) fn ancestor(read: &mut git::ReadContext<'_>, earlier: &str, later: &str) -> bool {
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
    let mut nodes = Vec::new();
    if complete {
        // Merge marks the scope's open work done, so its gates read the
        // records with those marks and refuse only an open blocker
        // (atelier-jnrh).
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
            // From history, an approval counts only as first committed. Its
            // identity is recorded but not compared with the work's Git
            // authors (atelier-md0v).
            let intact = matches!(record.event, ReviewEvent::TrivialExemption)
                || !global
                || original_intact(read, record).unwrap_or(false);
            // An approval from before the base was last merged in still
            // counts; one from before the branch left the base does not.
            intact
                && ((historical && retained(read, base_commit, record))
                    || if global {
                        approval_is_current(read, &record.commit, base_commit).unwrap_or(false)
                    } else {
                        current_on_branch(read, record, base_commit)
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
                None => not_current_reason(&approval_standing(index, read, owner, base_commit)?),
                Some(record) if matches!(record.event, ReviewEvent::Approval) => {
                    format!("current independent approval {}", record.header.id)
                }
                Some(record) => format!("trivial exemption {}", record.header.id),
            },
            support,
        ));
    }
    // A defer or no_action decision settles a finding; a fix_now finding
    // is settled by a current approval after its fix (atelier-md0v).
    let mut unresolved = Vec::new();
    let mut gaps = Vec::new();
    let mut disposed = Vec::new();
    let mut approval_valid = BTreeMap::new();
    for finding in index.reviews().filter(|record| {
        applicable.contains(&record.target) && matches!(record.event, ReviewEvent::Finding { .. })
    }) {
        let finding_id = finding.header.id.clone();
        let settled = match settlement(finding) {
            Settlement::Decided => true,
            Settlement::Undecided => {
                unresolved.push(format!(
                    "{finding_id} needs a decision: a finding recommended defer or no_action records one"
                ));
                gaps.push(Gap::Undisposed(finding_id));
                continue;
            }
            Settlement::Fix => {
                let resolution = finding
                    .disposition()
                    .and_then(|disposition| disposition.resolution_commit.as_ref());
                // Before the base held it, a finding (or its old fix_now
                // disposition) was not yet seen by an approval the base
                // already held.
                let finding_retained = match resolution {
                    Some(_) => disposition_retained(read, base_commit, finding),
                    None => retained(read, base_commit, finding),
                };
                resolution.is_none_or(|resolution| ancestor(read, resolution, "HEAD"))
                    && candidates.iter().any(|approval| {
                        matches!(approval.event, ReviewEvent::Approval)
                            && (approval.target == owner
                                || approval.target == finding.target
                                || index.get_issue(&finding.target).is_some_and(|issue| {
                                    issue.parent_id.as_deref() == Some(&approval.target)
                                }))
                            && fix_resolved(read, finding, approval)
                            && (!historical
                                || finding_retained
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
            }
        };
        if settled {
            disposed.push(finding_id);
        } else {
            unresolved.push(format!(
                "{finding_id}: fix_now needs a current approval after its fix"
            ));
            gaps.push(Gap::AwaitingApproval(finding_id));
        }
    }
    nodes.push(
        gate(
            owner,
            "findings.disposed",
            unresolved.is_empty(),
            if unresolved.is_empty() {
                "every finding is decided, or fixed and approved since".into()
            } else {
                unresolved.join("; ")
            },
            disposed,
        )
        .with_gaps(gaps),
    );
    nodes.extend(completion_claims(
        index, read, policy, &claimed, historical,
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
            let report = index
                .issue_record(id)
                .context("closed issue record missing")?
                .sections
                .report
                .as_deref()
                .unwrap_or_default()
                .trim()
                .to_owned();
            for field in &transition.required_fields {
                // `close_reason` is the only required field: a non-empty
                // Report (atelier-jnrh).
                let present = match field.as_str() {
                    "close_reason" => !report.is_empty(),
                    other => bail!("unsupported required field '{other}'"),
                };
                nodes.push(gate(
                    id,
                    &format!("required_field.{field}"),
                    present,
                    if present {
                        format!("committed claim includes required field {field}")
                    } else {
                        format!("committed claim is missing required field {field}, a Report")
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
/// Whether an approval is as first committed: the commit that introduced
/// it, which must still hold it unchanged, as `land` and `doctor` read
/// history. Its identity is recorded but not compared with the work's Git
/// authors (atelier-md0v).
pub(crate) fn original_intact(
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
    Ok(true)
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
    let end = read.claim_endpoint(index, target, &approval.commit)?;
    approval_is_current_to(read, &approval.commit, &end, base)
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
