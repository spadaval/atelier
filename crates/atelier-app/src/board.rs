//! A repository's board (atelier-ahpf): where work stands, what is done and
//! what is next, as the site shows it. The board itself is read from one
//! commit of the base branch; the work in flight on each owned work branch is
//! read from that branch's own records at its tip, as `atelier status` reads
//! its branch section.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use ts_rs::TS;

use atelier_core::{Issue, ReviewEvent};

use crate::branch_work::{branch_merge, branch_owner_ids};
use crate::git_freshness::{self, ReadContext};
use crate::issue_read::ObjectiveBucketTotals;
use crate::mission_overview::mission_overview;
use crate::record_index::RecordIndex;
use crate::review_gate::{self, ApprovalState};
use crate::work_classification::{
    apply_ready_gates, classify_issue, work_buckets, ClassifiedWork, WorkRow,
};
use crate::workflow_policy::WorkflowPolicy;

/// How many days back the board's recently done work reaches.
pub const RECENT_DAYS: u32 = 14;

/// One item as a board lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct BoardRow {
    pub id: String,
    pub title: String,
    pub issue_type: String,
    pub status: String,
    pub status_category: Option<String>,
    pub priority: String,
    /// Where the work stands, as the CLI words it: `active`, `ready`,
    /// `needs human`, `context`, `blocked`, `backlog` or `done`.
    pub state: String,
    pub labels: Vec<String>,
    /// Blockers that are not done; `readiness gate` when a dependency is not
    /// ready, as `atelier work ready` marks it.
    pub open_blockers: Vec<String>,
    pub closed_at: Option<DateTime<Utc>>,
    /// The work branch the record names, once started.
    pub work_branch: Option<String>,
}

impl BoardRow {
    fn new(index: &RecordIndex, issue: &Issue, classified: &ClassifiedWork) -> Self {
        Self {
            id: issue.id.clone(),
            title: issue.title.clone(),
            issue_type: issue.issue_type.clone(),
            status: issue.status.clone(),
            status_category: classified.status_category.clone(),
            priority: issue.priority.clone(),
            state: classified.classification.label().to_owned(),
            labels: index
                .issue_record(&issue.id)
                .map(|record| record.labels.clone())
                .unwrap_or_default(),
            open_blockers: classified.open_blockers.clone(),
            closed_at: issue.closed_at,
            work_branch: issue
                .workflow_branch
                .as_ref()
                .map(|branch| branch.work_branch.clone()),
        }
    }

    fn from_work_row(index: &RecordIndex, row: &WorkRow) -> Self {
        let record = index.issue_record(&row.id);
        Self {
            id: row.id.clone(),
            title: row.title.clone(),
            issue_type: record
                .map(|record| record.issue.issue_type.clone())
                .unwrap_or_default(),
            status: row.status.clone(),
            status_category: row.status_category.clone(),
            priority: row.priority.clone(),
            state: row.state_label().to_owned(),
            labels: record
                .map(|record| record.labels.clone())
                .unwrap_or_default(),
            open_blockers: row.open_blockers.clone(),
            closed_at: record.and_then(|record| record.issue.closed_at),
            work_branch: record
                .and_then(|record| record.issue.workflow_branch.as_ref())
                .map(|branch| branch.work_branch.clone()),
        }
    }
}

/// An open mission and how far it has got.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct BoardMission {
    pub id: String,
    pub title: String,
    pub status: String,
    pub status_category: String,
    pub priority: String,
    /// The items in its scope by where they stand, as `atelier work
    /// missions` counts them.
    pub totals: ObjectiveBucketTotals,
    /// How many items its scope holds, and how many of those are done.
    pub planned: usize,
    pub done: usize,
    /// How many success criteria it has, and how many the newest receipt in
    /// the base's history proves, as its debrief marks them.
    pub criteria: usize,
    pub proven: usize,
    /// Why its criteria could not be read; the counts are 0 then.
    pub criteria_error: Option<String>,
}

/// Open work under one heading: a mission's scope, or, for work in no open
/// mission, one label or no label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct WorkGroup {
    /// The mission whose scope this is.
    pub mission: Option<String>,
    /// The label, for work in no open mission; `None` with no mission is
    /// work without a label.
    pub label: Option<String>,
    pub title: String,
    /// In the order `atelier work` lists work.
    pub work: Vec<BoardRow>,
}

/// What a base commit says about a repository's work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct Board {
    /// The day recently done work counts back from, `YYYY-MM-DD` in UTC.
    pub today: String,
    pub recent_days: u32,
    /// Open missions, as `atelier work missions` orders them.
    pub missions: Vec<BoardMission>,
    /// Open work by mission, then work in no open mission by label, as
    /// `atelier work missions` groups it.
    pub groups: Vec<WorkGroup>,
    /// Work the base records in an active status, as `atelier status`
    /// lists it under Current Work.
    pub active: Vec<BoardRow>,
    /// Ready work, in the order `atelier work ready` lists it.
    pub ready: Vec<BoardRow>,
    /// Ready work only the human can do, as `atelier work ready` lists it.
    pub human: Vec<BoardRow>,
    /// Work closed within `recent_days` of `today`, newest first.
    pub recently_done: Vec<BoardRow>,
}

/// The board of the records in `index`, read at the commit `read` pins, with
/// recently done work counted back from `today`.
pub fn board(
    index: &RecordIndex,
    policy: &WorkflowPolicy,
    read: &mut ReadContext<'_>,
    today: NaiveDate,
) -> Result<Board> {
    let overview = mission_overview(index, Some(policy), false)?;
    let classified = index
        .issues()
        .map(|issue| {
            Ok((
                issue.id.clone(),
                classify_issue(index, Some(policy), issue)?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    let ordered = |ids: &mut dyn Iterator<Item = &String>| -> Vec<BoardRow> {
        let rows = ids
            .filter_map(|id| index.issue_record(id))
            .map(|record| (record.issue.clone(), classified[&record.issue.id].clone()))
            .collect::<Vec<_>>();
        crate::work_order::ordered_work_indices(&rows)
            .into_iter()
            .map(|at| BoardRow::new(index, &rows[at].0, &rows[at].1))
            .collect()
    };
    let open = |id: &&String| {
        index
            .issue_record(id)
            .is_some_and(|record| record.issue.issue_type != "mission")
            && classified[*id].classification
                != crate::work_classification::WorkClassification::Done
    };

    let mut missions = Vec::new();
    let mut groups = Vec::new();
    for mission in &overview.missions {
        let (criteria, proven, criteria_error) =
            match crate::criteria::mission_proof(index, read, &mission.issue.id) {
                Ok(proof) => (
                    proof.proofs.len(),
                    proof
                        .proofs
                        .iter()
                        .filter(|proof| proof.mark == crate::criteria::CriterionMark::Proven)
                        .count(),
                    None,
                ),
                Err(error) => (0, 0, Some(format!("{error:#}"))),
            };
        missions.push(BoardMission {
            id: mission.issue.id.clone(),
            title: mission.issue.title.clone(),
            status: mission.issue.status.clone(),
            status_category: mission.status_category.clone(),
            priority: mission.issue.priority.clone(),
            totals: mission.totals.clone(),
            planned: mission.scope_ids.len(),
            done: mission.totals.done,
            criteria,
            proven,
            criteria_error,
        });
        groups.push(WorkGroup {
            mission: Some(mission.issue.id.clone()),
            label: None,
            title: mission.issue.title.clone(),
            work: ordered(&mut mission.scope_ids.iter().filter(open)),
        });
    }
    for group in &overview.unassigned {
        groups.push(WorkGroup {
            mission: None,
            label: group.label.clone(),
            title: group.label.clone().unwrap_or_else(|| "no label".to_owned()),
            work: ordered(&mut group.work.iter().map(|(issue, _)| &issue.id)),
        });
    }

    let mut buckets = work_buckets(index, Some(policy))?;
    apply_ready_gates(index, policy, &mut buckets)?;
    let rows = |rows: &[WorkRow]| {
        rows.iter()
            .map(|row| BoardRow::from_work_row(index, row))
            .collect::<Vec<_>>()
    };
    let since = today - chrono::Duration::days(i64::from(RECENT_DAYS));
    let mut recently_done = buckets
        .done
        .iter()
        .filter_map(|row| index.issue_record(&row.id))
        .map(|record| &record.issue)
        .filter(|issue| issue.closed_at.unwrap_or(issue.updated_at).date_naive() > since)
        .collect::<Vec<_>>();
    recently_done.sort_by(|left, right| {
        right
            .closed_at
            .unwrap_or(right.updated_at)
            .cmp(&left.closed_at.unwrap_or(left.updated_at))
            .then_with(|| left.id.cmp(&right.id))
    });
    Ok(Board {
        today: today.format("%Y-%m-%d").to_string(),
        recent_days: RECENT_DAYS,
        missions,
        groups,
        active: rows(&buckets.active),
        ready: rows(&buckets.ready),
        human: rows(&buckets.human),
        recently_done: recently_done
            .into_iter()
            .map(|issue| BoardRow::new(index, issue, &classified[&issue.id]))
            .collect(),
    })
}

/// Where an owner's review stands on its work branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    /// No review event targets the owner's scope.
    Unreviewed,
    /// Reviewed, but with no independent approval or trivial exemption.
    NotApproved,
    /// `merge`'s `review.current` would pass, as `atelier status` marks
    /// `✓ approved`.
    Approved,
    /// Later commits changed what the approval covers: `⚠ approval stale`.
    ApprovalStale,
}

/// Work in flight on one work branch, read from the branch's own records at
/// `tip`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct BranchWork {
    pub branch: String,
    pub tip: String,
    /// The owner as the branch records it.
    pub item: BoardRow,
    pub review: ReviewState,
    /// Findings in the owner's scope no decision or later approval settles,
    /// as `findings.disposed` judges them.
    pub open_findings: Vec<String>,
    /// Base commits the branch does not have.
    pub behind: usize,
}

/// One work branch's owners and where each stands, or `None` when the branch
/// holds no tracker, no issue owns it, the base already holds it, or every
/// owner is done on the base, as `atelier status` leaves such branches out
/// of its owned rows. `base` is the base's records at `base_commit`.
pub fn branch_work(
    common_dir: &Path,
    base: &RecordIndex,
    base_policy: &WorkflowPolicy,
    base_commit: &str,
    branch: &str,
    tip: &str,
) -> Result<Option<Vec<BranchWork>>> {
    let listed = git_freshness::git_output(
        common_dir,
        &[
            "ls-tree",
            tip,
            "--",
            crate::workflow_policy::WORKFLOW_POLICY_PATH,
        ],
    )?;
    if listed.is_empty() {
        return Ok(None);
    }
    let merged = git_freshness::git_output(
        common_dir,
        &["merge-base", "--is-ancestor", tip, base_commit],
    )
    .is_ok();
    if merged {
        return Ok(None);
    }
    let mut owners = branch_owner_ids(common_dir, base, branch, tip)?;
    // An owner done on the base no longer owns the branch (atelier-4nsl).
    owners.retain(|id| {
        !base
            .issue_record(id)
            .is_some_and(|record| base_policy.status_category(&record.issue.status) == Some("done"))
    });
    if owners.is_empty() {
        return Ok(None);
    }
    let records =
        crate::commit_records::load(&mut ReadContext::at_commit(common_dir, tip.to_owned()))?;
    let merge = branch_merge(common_dir, tip, base_commit)?;
    let mut read = ReadContext::from_snapshot(common_dir, tip.to_owned(), &[]);
    let index = &records.index;
    let mut rows = Vec::new();
    for id in owners {
        let owner = index.require_issue(&id)?;
        let classified = classify_issue(index, Some(&records.policy), &owner)?;
        let scope = review_gate::scope(index, &id)?;
        let reviewed = index.reviews().any(|review| scope.contains(&review.target));
        let review = match review_gate::approval_state(index, &mut read, &id, &merge.fork_point)? {
            ApprovalState::Current => ReviewState::Approved,
            ApprovalState::Stale => ReviewState::ApprovalStale,
            ApprovalState::Missing if reviewed => ReviewState::NotApproved,
            ApprovalState::Missing => ReviewState::Unreviewed,
        };
        rows.push(BranchWork {
            branch: branch.to_owned(),
            tip: tip.to_owned(),
            item: BoardRow::new(index, &owner, &classified),
            review,
            open_findings: open_findings(index, &mut read, &id, &scope),
            behind: merge.behind,
        });
    }
    Ok(Some(rows))
}

/// The findings on `scope` that `findings.disposed` would not count as
/// settled: a defer or no_action recommendation with no decision, or a
/// fix_now finding no later approval resolves.
fn open_findings(
    index: &RecordIndex,
    read: &mut ReadContext<'_>,
    owner: &str,
    scope: &BTreeSet<String>,
) -> Vec<String> {
    let mut open = index
        .reviews()
        .filter(|review| {
            scope.contains(&review.target) && matches!(review.event, ReviewEvent::Finding { .. })
        })
        .filter(|finding| match review_gate::settlement(finding) {
            review_gate::Settlement::Decided => false,
            review_gate::Settlement::Fix => {
                review_gate::fix_approval(index, read, owner, finding).is_none()
            }
            review_gate::Settlement::Undecided => true,
        })
        .map(|finding| finding.header.id.clone())
        .collect::<Vec<_>>();
    open.sort();
    open
}

/// One row of the board's work in progress: an owner on its work branch, or
/// work the base records as active with no branch read for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct InProgress {
    pub item: BoardRow,
    /// The branch the work is on, as its owner's branch records it or, for
    /// active work on the base, as the base records it.
    pub branch: Option<String>,
    /// How the branch stands; `None` for active work no branch was read for.
    pub on_branch: Option<BranchState>,
}

/// An owner's review and position on its branch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct BranchState {
    pub tip: String,
    pub review: ReviewState,
    pub open_findings: Vec<String>,
    pub behind: usize,
}

/// The board's work in progress: every owner of a work branch, in branch
/// order, then active work on the base that no branch read owns.
pub fn in_progress(board: &Board, branches: &[BranchWork]) -> Vec<InProgress> {
    let mut rows = branches
        .iter()
        .map(|work| InProgress {
            item: work.item.clone(),
            branch: Some(work.branch.clone()),
            on_branch: Some(BranchState {
                tip: work.tip.clone(),
                review: work.review,
                open_findings: work.open_findings.clone(),
                behind: work.behind,
            }),
        })
        .collect::<Vec<_>>();
    let owned = branches
        .iter()
        .map(|work| work.item.id.as_str())
        .collect::<BTreeSet<_>>();
    rows.extend(
        board
            .active
            .iter()
            .filter(|row| !owned.contains(row.id.as_str()))
            .map(|row| InProgress {
                item: row.clone(),
                branch: row.work_branch.clone(),
                on_branch: None,
            }),
    );
    rows
}
