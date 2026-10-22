//! Mission reviews (atelier-rx22, atelier-acrt): one view model per review,
//! built from records and Git history and rendered by the CLI as dense
//! Markdown and as a static HTML page, so the two forms cannot disagree.
//!
//! The debrief reads as an after-action report (atelier-8bil): the problem,
//! what the mission set out to do, what happened in time order, then where
//! it stands: results, how it was validated, plan against actual, what needs
//! the human, what is left and what was found along the way.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use atelier_core::{
    CloseReport, EvidenceCriterion, EvidenceResult, FindingDecision, Issue, IssueSectionName,
    ReviewEvent,
};
use chrono::{DateTime, Utc};

use crate::criteria::{mission_proof_until, CriterionMark, MissionProof};
use crate::decisions::{mission_decisions, Decisions, MalformedDecision, NoteDecision};
use crate::git_freshness::ReadContext;
use crate::objective_graph::{mission_issue_ids, mission_objective_kind};
use crate::record_history::{mission_timeline, MissionTimeline};
use crate::record_index::RecordIndex;
use crate::workflow_policy::WorkflowPolicy;

/// What a reference to an issue shows: its chip and the card behind it.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IssueRef {
    pub id: String,
    pub title: String,
    pub issue_type: String,
    pub status: String,
    pub status_category: Option<String>,
    /// The first sentence of its Goal, Question or Outcome.
    pub summary: String,
    /// The first sentence of its Close Reason, once closed.
    pub outcome: Option<String>,
    pub parent: Option<String>,
}

/// A review event or evidence receipt that record text mentions by ID.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RecordRef {
    /// `finding`, `approval`, `disposition`, `review`, `trivial` or `receipt`.
    pub kind: String,
    pub summary: String,
}

/// Something only the human can settle.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Ask {
    /// An open decision record in scope.
    Decision(String),
    /// Open work labelled `needs-human`.
    NeedsHuman(String),
    /// A criterion whose newest proof fails.
    Failing(String),
    /// Criteria without passing proof.
    Unproven(Vec<String>),
}

/// A fact about one item that departs from the plan, beside its report's
/// own Deviations.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Change {
    /// A done claim later withdrawn.
    Reopened(DateTime<Utc>),
    /// The item's newest receipt does not pass.
    NonPassEvidence {
        evidence: String,
        result: EvidenceResult,
        summary: String,
    },
    /// Review was waived as trivial.
    TrivialExemption { summary: String },
}

/// How review of one item went.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ReviewSummary {
    /// Any review event targets the item.
    pub recorded: bool,
    pub approved: bool,
    /// Who approved it, oldest approval first.
    pub approvers: Vec<String>,
    pub trivial: bool,
    pub fixed: usize,
    pub deferred: usize,
    pub no_action: usize,
    /// Findings with no disposition yet.
    pub undisposed: usize,
}

impl ReviewSummary {
    pub fn findings(&self) -> usize {
        self.fixed + self.deferred + self.no_action + self.undisposed
    }

    fn absorb(&mut self, other: &ReviewSummary) {
        self.recorded |= other.recorded;
        self.approved |= other.approved;
        self.trivial |= other.trivial;
        for approver in &other.approvers {
            if !self.approvers.contains(approver) {
                self.approvers.push(approver.clone());
            }
        }
        self.fixed += other.fixed;
        self.deferred += other.deferred;
        self.no_action += other.no_action;
        self.undisposed += other.undisposed;
    }
}

/// One piece of delivered work, with its children.
#[derive(Debug, Clone)]
pub struct WorkEntry {
    pub id: String,
    /// The parent, when the parent itself is not done.
    pub parent: Option<String>,
    pub closed_at: Option<DateTime<Utc>>,
    /// The Close Reason read as the implementer report.
    pub report: Option<CloseReport>,
    /// The criteria it delivers.
    pub criteria: Vec<String>,
    pub children: Vec<String>,
    pub added_after_start: bool,
    pub changes: Vec<Change>,
    /// Decision notes on the item and its children, one-way first.
    pub decisions: Vec<NoteDecision>,
    /// Decision notes on the item and its children that `check` refuses.
    pub malformed: Vec<MalformedDecision>,
    pub review: ReviewSummary,
    /// Follow-ups its review deferred findings to.
    pub follow_ups: Vec<String>,
}

/// Work in scope that is not done, or that was dropped.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenEntry {
    pub id: String,
    pub parent: Option<String>,
    /// The item (or the mission) whose review filed it, or that it was
    /// discovered from.
    pub from: Option<String>,
    /// Whether `from` comes from a `discovered_from` link instead of the
    /// disposition that deferred a finding to it.
    pub discovered: bool,
    /// When it joined the mission: its link after the start, or its record's
    /// creation for a follow-up outside the scope.
    pub filed_at: Option<DateTime<Utc>>,
    pub added_after_start: bool,
    pub changes: Vec<Change>,
    /// Why a dropped item was dropped.
    pub reason: Option<String>,
}

/// A review event on the mission itself, such as a plan approval.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MissionEvent {
    pub id: String,
    pub event: &'static str,
    pub identity: String,
    pub commit: String,
    pub created_at: DateTime<Utc>,
    pub summary: String,
}

/// A receipt that judged mission criteria.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValidationReceipt {
    pub id: String,
    /// The mission, or the item it was recorded on.
    pub target: String,
    pub result: EvidenceResult,
    pub criteria: Vec<EvidenceCriterion>,
    pub summary: String,
}

/// The receipts one producer recorded against one commit: one validation
/// of the mission.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValidationRun {
    pub identity: String,
    pub at: DateTime<Utc>,
    pub commit: String,
    /// Code commits since the run's commit, where one of its receipts still
    /// decides a mark.
    pub code_commits_since: Option<usize>,
    pub receipts: Vec<ValidationReceipt>,
    /// Work delivered after the run, when code has changed since.
    pub later_work: Vec<String>,
}

/// How the mission was checked: review of each item, the runs that judged
/// its criteria, and the criteria still waiting for proof.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Validation {
    /// Delivered items with an approval, items waived as trivial, items
    /// reviewed without an approval, and items with no review at all.
    pub approved: usize,
    pub trivial: usize,
    pub not_approved: Vec<String>,
    pub unreviewed: Vec<String>,
    /// Findings on the delivered items, by their latest disposition.
    pub findings: ReviewSummary,
    pub runs: Vec<ValidationRun>,
    /// Criteria not proven, each with the open item that will prove it.
    pub awaiting: Vec<(String, CriterionMark, Option<String>)>,
}

/// One moment in what the mission did.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Event {
    /// The mission left `draft`.
    Started,
    /// A work entry closed.
    Delivered(String),
    /// Work joined the mission's scope after the start.
    Added(String),
    /// Work found along the way was filed, other than by a delivered item's
    /// review.
    Filed(String),
    /// Work was dropped.
    Dropped(String),
    /// A validation run, by index into `Validation::runs`.
    Validated(usize),
    /// A review event on the mission, by index into `mission_events`.
    MissionReview(usize),
    /// A base commit exempted from review as trivial while the mission ran,
    /// by index into `base_exemptions`.
    BaseExemption(usize),
    /// The mission made its done claim.
    Closed,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimedEvent {
    pub at: Option<DateTime<Utc>>,
    pub event: Event,
}

#[derive(Debug, Clone)]
pub struct Debrief {
    pub mission: Issue,
    pub status_category: Option<String>,
    pub outcome: String,
    /// The strategy's Why section, or the mission's Background.
    pub why: Option<String>,
    pub strategy: Option<String>,
    /// The commit the review was generated from; `None` without history.
    pub generated_from: Option<String>,
    pub timeline: MissionTimeline,
    pub criteria: MissionProof,
    /// For each criterion, the done items that deliver it.
    pub delivered_by: BTreeMap<String, Vec<String>>,
    pub asks: Vec<Ask>,
    /// Items linked to the mission by its start and not filed by review,
    /// each followed by its children.
    pub planned: Vec<(String, Option<String>)>,
    /// Done items linked to the mission, in closing order.
    pub work: Vec<WorkEntry>,
    /// Planned work, and added work someone has taken on, not done and not
    /// asked of the human.
    pub left: Vec<OpenEntry>,
    /// Open work filed by review, or added after the start and parked in
    /// backlog, not asked of the human. Without a recorded source
    /// (atelier-tq1j), parked work added after the start counts as found.
    pub found: Vec<OpenEntry>,
    /// Superseded items, with their reason.
    pub dropped: Vec<OpenEntry>,
    /// Items that joined the scope after the start and were not found
    /// along the way, in joining order.
    pub added: Vec<String>,
    pub events: Vec<TimedEvent>,
    pub validation: Validation,
    pub decisions: Decisions,
    pub mission_events: Vec<MissionEvent>,
    /// Base commits exempted from review as trivial, with no owner issue,
    /// from the mission's start to its done claim (atelier-mbrf). They name
    /// no mission, so every mission running at the time shows them.
    pub base_exemptions: Vec<MissionEvent>,
    pub refs: BTreeMap<String, IssueRef>,
    /// Reviews and receipts on the mission and its scope, for mentions.
    pub record_refs: BTreeMap<String, RecordRef>,
    /// Planned items done and in total, superseded ones left out.
    pub planned_done: usize,
    pub planned_total: usize,
}

impl Debrief {
    pub fn title_of<'a>(&'a self, id: &'a str) -> &'a str {
        self.refs.get(id).map_or(id, |issue| issue.title.as_str())
    }

    pub fn entry(&self, id: &str) -> Option<&WorkEntry> {
        self.work.iter().find(|entry| entry.id == id)
    }

    /// The work entry an item is shown in: its own, or its parent's.
    pub fn entry_holding(&self, id: &str) -> Option<&WorkEntry> {
        self.work
            .iter()
            .find(|entry| entry.id == id || entry.children.iter().any(|child| child == id))
    }

    pub fn proven(&self) -> usize {
        self.criteria
            .proofs
            .iter()
            .filter(|proof| proof.mark == CriterionMark::Proven)
            .count()
    }
}

pub fn debrief(
    index: &RecordIndex,
    policy: Option<&WorkflowPolicy>,
    read: &mut ReadContext<'_>,
    mission_id: &str,
) -> Result<Debrief> {
    mission_objective_kind(index, mission_id)?;
    let record = index
        .issue_record(mission_id)
        .ok_or_else(|| index.not_found(crate::record_ref::RecordKind::Issue, mission_id))?;
    let mission = record.issue.clone();
    let category = |issue: &Issue| {
        policy
            .and_then(|policy| policy.status_category(&issue.status))
            .map(str::to_string)
    };
    let done =
        |issue: &Issue| category(issue).as_deref() == Some("done") && issue.status != "superseded";
    let scope = mission_issue_ids(index, mission_id)?;
    let tree = scope_tree(index, &scope);
    let linked = tree
        .iter()
        .filter(|(_, parent)| parent.is_none())
        .map(|(id, _)| id.clone())
        .collect::<BTreeSet<_>>();
    let children = tree
        .iter()
        .filter_map(|(id, parent)| Some((id.clone(), parent.clone()?)))
        .collect::<Vec<_>>();
    let timeline = mission_timeline(read, policy, mission_id, &scope, &linked, &children)?;
    let generated_from = read.pinned("HEAD").ok();
    let criteria = mission_proof_until(
        index,
        read,
        mission_id,
        timeline
            .closed
            .as_ref()
            .map(|moment| moment.commit.as_str()),
    )?;
    let added_at = timeline
        .added_after_start
        .iter()
        .map(|(id, moment)| (id.clone(), moment.time))
        .collect::<BTreeMap<_, _>>();

    // Reviews of each scoped item: findings, their latest dispositions, and
    // where deferred findings went.
    let mut reviews: BTreeMap<String, ReviewSummary> = BTreeMap::new();
    let mut follow_up_source: BTreeMap<String, String> = BTreeMap::new();
    let mut follow_ups_of: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut trivial: BTreeMap<String, String> = BTreeMap::new();
    let mut findings: BTreeMap<String, String> = BTreeMap::new();
    // Each finding's latest disposition, with where it was deferred to.
    #[allow(clippy::type_complexity)]
    let mut dispositions: BTreeMap<
        String,
        (DateTime<Utc>, FindingDecision, Option<String>, String),
    > = BTreeMap::new();
    let mut ordered_reviews = index
        .reviews()
        .filter(|review| scope.contains(&review.target) || review.target == mission_id)
        .collect::<Vec<_>>();
    ordered_reviews.sort_by(|a, b| {
        (a.header.created_at, &a.header.id).cmp(&(b.header.created_at, &b.header.id))
    });
    for review in ordered_reviews {
        let summary = reviews.entry(review.target.clone()).or_default();
        summary.recorded = true;
        match &review.event {
            ReviewEvent::Approval => {
                summary.approved = true;
                if !summary.approvers.contains(&review.identity) {
                    summary.approvers.push(review.identity.clone());
                }
            }
            ReviewEvent::TrivialExemption => {
                summary.trivial = true;
                trivial.insert(review.target.clone(), review.summary.clone());
            }
            ReviewEvent::Finding { .. } => {
                findings.insert(review.header.id.clone(), review.target.clone());
            }
            ReviewEvent::Disposition {
                finding,
                decision,
                follow_up,
                ..
            } => {
                let newer = dispositions
                    .get(finding)
                    .is_none_or(|(at, ..)| review.header.created_at >= *at);
                if newer {
                    dispositions.insert(
                        finding.clone(),
                        (
                            review.header.created_at,
                            *decision,
                            follow_up.clone(),
                            review.target.clone(),
                        ),
                    );
                }
            }
            ReviewEvent::Review => {}
        }
    }
    for (_, decision, follow_up, target) in dispositions.values() {
        if let (FindingDecision::Defer, Some(follow_up)) = (decision, follow_up) {
            follow_up_source
                .entry(follow_up.clone())
                .or_insert_with(|| target.clone());
            let list = follow_ups_of.entry(target.clone()).or_default();
            if !list.contains(follow_up) {
                list.push(follow_up.clone());
            }
        }
    }
    // Work that declares it was discovered from an item in scope (directly,
    // or through one of its findings or receipts) is found along the way
    // whether or not it advances the mission. A deferral names the same
    // source, which `check` enforces, so it wins when both are present.
    let mut discovered_ids: BTreeSet<String> = BTreeSet::new();
    let source_scope = scope
        .iter()
        .map(String::as_str)
        .chain(std::iter::once(mission_id))
        .collect::<BTreeSet<_>>();
    for record in index.issue_records() {
        let new = &record.issue.id;
        if follow_up_source.contains_key(new) || new == mission_id {
            continue;
        }
        let source = record
            .relationships
            .relates
            .iter()
            .filter(|relation| relation.relation_type == atelier_records::DISCOVERED_FROM)
            .filter_map(|relation| index.discovery_source(&relation.kind, &relation.id))
            .find(|source| source_scope.contains(source.as_str()));
        if let Some(source) = source {
            follow_up_source.insert(new.clone(), source.clone());
            discovered_ids.insert(new.clone());
            let list = follow_ups_of.entry(source).or_default();
            if !list.contains(new) {
                list.push(new.clone());
            }
        }
    }
    for (finding, target) in &findings {
        let summary = reviews.entry(target.clone()).or_default();
        match dispositions.get(finding).map(|(_, decision, ..)| *decision) {
            Some(FindingDecision::FixNow) => summary.fixed += 1,
            Some(FindingDecision::Defer) => summary.deferred += 1,
            Some(FindingDecision::NoAction) => summary.no_action += 1,
            None => summary.undisposed += 1,
        }
    }

    // Criteria and the done items that deliver them: an item whose Goal's
    // first sentence names the criterion outside parentheses (where other
    // work is mentioned in passing), or that the criterion's text names, or
    // that holds the receipt deciding it.
    let mut delivered_by: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for proof in &criteria.proofs {
        let slug = &proof.criterion.slug;
        let mut by = Vec::new();
        for (id, _) in &tree {
            let Some(item) = index.issue_record(id) else {
                continue;
            };
            if !done(&item.issue) || item.issue.issue_type == "decision" {
                continue;
            }
            let goal = item
                .sections
                .section(IssueSectionName::Goal)
                .map(|text| outside_parentheses(&first_sentence(text)))
                .is_some_and(|text| names_criterion(&text, slug));
            let named = names_word(&proof.criterion.text, id);
            let proved = proof.deciding().is_some_and(|receipt| {
                &receipt.target_id == id && receipt.result == EvidenceResult::Pass
            });
            if goal || named || proved {
                by.push(id.clone());
            }
        }
        // A parent and its child both naming a criterion credit the child.
        let parents = by
            .iter()
            .filter_map(|id| index.issue_record(id)?.issue.parent_id.clone())
            .collect::<BTreeSet<_>>();
        by.retain(|id| !parents.contains(id));
        delivered_by.insert(slug.clone(), by);
    }

    let decisions = mission_decisions(index, policy, mission_id)?;
    let changes_of = |id: &str| -> Result<Vec<Change>> {
        let mut changes = Vec::new();
        if let Some((_, moment)) = timeline
            .reopened
            .iter()
            .find(|(reopened, _)| reopened == id)
        {
            changes.push(Change::Reopened(moment.time));
        }
        if let Some((evidence, result, summary)) = newest_receipt(index, id)? {
            if result != EvidenceResult::Pass {
                changes.push(Change::NonPassEvidence {
                    evidence,
                    result,
                    summary,
                });
            }
        }
        if let Some(summary) = trivial.get(id) {
            changes.push(Change::TrivialExemption {
                summary: summary.clone(),
            });
        }
        Ok(changes)
    };
    let open_entry = |id: &str, parent: Option<String>| -> Result<OpenEntry> {
        Ok(OpenEntry {
            id: id.to_string(),
            parent,
            from: follow_up_source.get(id).cloned(),
            discovered: discovered_ids.contains(id),
            filed_at: added_at.get(id).copied(),
            added_after_start: added_at.contains_key(id),
            changes: changes_of(id)?,
            reason: None,
        })
    };
    let mut work = Vec::new();
    let mut open = Vec::new();
    let mut dropped = Vec::new();
    let work_entry = |id: &str, parent: Option<String>, kids: Vec<String>| -> Result<WorkEntry> {
        let item = index
            .issue_record(id)
            .ok_or_else(|| index.not_found(crate::record_ref::RecordKind::Issue, id))?;
        let mut changes = Vec::new();
        let mut notes = Vec::new();
        let mut review = ReviewSummary::default();
        let mut follow_ups = Vec::new();
        let members = std::iter::once(id.to_string())
            .chain(kids.iter().cloned())
            .collect::<Vec<_>>();
        for member in &members {
            changes.extend(changes_of(member)?);
            if let Some(record) = index.issue_record(member) {
                notes.extend(record.sections.decision_notes().into_iter().flatten().map(
                    |decision| NoteDecision {
                        issue_id: member.clone(),
                        decision,
                    },
                ));
            }
            if let Some(summary) = reviews.get(member) {
                review.absorb(summary);
            }
            for follow_up in follow_ups_of.get(member).into_iter().flatten() {
                if !follow_ups.contains(follow_up) {
                    follow_ups.push(follow_up.clone());
                }
            }
        }
        notes.sort_by_key(|note| !note.decision.one_way);
        let criteria_delivered = delivered_by
            .iter()
            .filter(|(_, by)| by.iter().any(|by| members.contains(by)))
            .map(|(slug, _)| slug.clone())
            .collect::<Vec<_>>();
        Ok(WorkEntry {
            id: id.to_string(),
            parent,
            closed_at: item.issue.closed_at,
            report: item
                .sections
                .close_reason
                .as_deref()
                .map(CloseReport::parse),
            criteria: criteria_delivered,
            children: kids,
            added_after_start: added_at.contains_key(id),
            changes,
            decisions: notes,
            malformed: decisions
                .malformed
                .iter()
                .filter(|note| members.contains(&note.issue_id))
                .cloned()
                .collect(),
            review,
            follow_ups,
        })
    };
    let dropped_entry = |id: &str, parent: Option<String>| -> Result<OpenEntry> {
        let mut entry = open_entry(id, parent)?;
        entry.reason = Some(close_outcome(index, id));
        Ok(entry)
    };
    for id in &linked {
        let Some(item) = index.issue_record(id) else {
            continue;
        };
        let kids = index
            .child_ids(id)
            .iter()
            .filter(|child| scope.contains(*child))
            .cloned()
            .collect::<Vec<_>>();
        if done(&item.issue) {
            work.push(work_entry(id, None, kids)?);
            continue;
        }
        if item.issue.status == "superseded" {
            dropped.push(dropped_entry(id, None)?);
        } else {
            open.push(open_entry(id, None)?);
        }
        // Children of a parent that is not done stand on their own.
        for kid in &kids {
            let Some(child) = index.issue_record(kid) else {
                continue;
            };
            if done(&child.issue) {
                work.push(work_entry(kid, Some(id.clone()), Vec::new())?);
            } else if child.issue.status == "superseded" {
                dropped.push(dropped_entry(kid, Some(id.clone()))?);
            } else {
                open.push(open_entry(kid, Some(id.clone()))?);
            }
        }
    }
    work.sort_by(|a, b| {
        (a.closed_at.is_none(), a.closed_at, &a.id).cmp(&(
            b.closed_at.is_none(),
            b.closed_at,
            &b.id,
        ))
    });
    // Follow-ups review filed outside the mission's scope still belong here.
    for (follow_up, source) in &follow_up_source {
        let Some(record) = index.issue_record(follow_up) else {
            continue;
        };
        if !scope.contains(follow_up) && !done(&record.issue) && record.issue.status != "superseded"
        {
            let mut entry = open_entry(follow_up, None)?;
            entry.from = Some(source.clone());
            entry.filed_at = Some(record.issue.created_at);
            open.push(entry);
        }
    }
    // Planned work: linked by the start and not filed by review.
    let is_planned = |id: &str| !follow_up_source.contains_key(id) && !added_at.contains_key(id);
    // In filing order, which is closer to the plan's than ID order; children
    // follow their parent.
    let created = |id: &str| index.issue_record(id).map(|record| record.issue.created_at);
    let mut planned_tops = linked
        .iter()
        .filter(|id| is_planned(id))
        .cloned()
        .collect::<Vec<_>>();
    planned_tops.sort_by_key(|id| (created(id), id.clone()));
    let planned = planned_tops
        .iter()
        .flat_map(|top| {
            tree.iter()
                .filter(move |(id, parent)| id == top || parent.as_ref() == Some(top))
                .filter(|(id, _)| is_planned(id))
                .cloned()
        })
        .collect::<Vec<_>>();
    let planned_records = linked
        .iter()
        .filter(|id| is_planned(id))
        .filter_map(|id| index.issue_record(id))
        .filter(|record| record.issue.status != "superseded")
        .collect::<Vec<_>>();
    let planned_done = planned_records
        .iter()
        .filter(|record| done(&record.issue))
        .count();

    let mut asks = Vec::new();
    for (id, _) in &tree {
        let Some(item) = index.issue_record(id) else {
            continue;
        };
        if category(&item.issue).as_deref() == Some("done") {
            continue;
        }
        if item.issue.issue_type == "decision" {
            asks.push(Ask::Decision(id.clone()));
        } else if item.labels.iter().any(|label| label == "needs-human") {
            asks.push(Ask::NeedsHuman(id.clone()));
        }
    }
    for proof in &criteria.proofs {
        if proof.mark == CriterionMark::Failing {
            asks.push(Ask::Failing(proof.criterion.slug.clone()));
        }
    }
    // A criterion an asked-for item already names needs no second ask.
    let asked = asks
        .iter()
        .filter_map(|ask| match ask {
            Ask::Decision(id) | Ask::NeedsHuman(id) => Some(id.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let goal_of = |id: &str| {
        index
            .issue_record(id)
            .and_then(|record| record.sections.section(IssueSectionName::Goal))
            .map(|goal| outside_parentheses(&first_sentence(goal)))
    };
    let asker_of = |slug: &str| {
        asked
            .iter()
            .find(|id| goal_of(id).is_some_and(|goal| names_criterion(&goal, slug)))
            .cloned()
    };
    let unproven = criteria
        .proofs
        .iter()
        .filter(|proof| proof.mark == CriterionMark::Unproven)
        .map(|proof| proof.criterion.slug.clone())
        .filter(|slug| asker_of(slug).is_none())
        .collect::<Vec<_>>();
    if !unproven.is_empty() {
        asks.push(Ask::Unproven(unproven));
    }

    // Open work splits into what is left of the plan and what was found along
    // the way; work asked of the human is listed only as an ask.
    // Work added after the start counts as found only while it is parked:
    // added work someone has taken on is scope.
    let parked = |id: &str| {
        index.issue_record(id).is_some_and(|record| {
            crate::work_classification::classify_issue(index, policy, &record.issue).is_ok_and(
                |work| {
                    work.classification == crate::work_classification::WorkClassification::Backlog
                },
            )
        })
    };
    let mut left = Vec::new();
    let mut found = Vec::new();
    for entry in open {
        if asked.contains(&entry.id) {
            continue;
        }
        if entry.from.is_some() || (entry.added_after_start && parked(&entry.id)) {
            found.push(entry);
        } else {
            left.push(entry);
        }
    }
    found.sort_by(|a, b| (a.filed_at, &a.id).cmp(&(b.filed_at, &b.id)));
    let found_ids = found
        .iter()
        .map(|entry| entry.id.clone())
        .collect::<BTreeSet<_>>();
    let added = timeline
        .added_after_start
        .iter()
        .map(|(id, _)| id.clone())
        .filter(|id| !found_ids.contains(id) && !follow_up_source.contains_key(id))
        .collect::<Vec<_>>();

    let mission_events = mission_events(index, mission_id);
    let base_exemptions = base_exemptions(index, &timeline);
    let validation = validation(index, &criteria, &work, &asker_of);
    let events = events(
        &timeline,
        &work,
        &added,
        &added_at,
        &found,
        &dropped,
        &validation,
        (&mission_events, &base_exemptions),
        index,
    );

    // Record text can mention any issue, so every issue can be named.
    let refs = index
        .issue_records()
        .filter_map(|record| {
            let id = record.issue.id.clone();
            Some((id.clone(), issue_ref(index, policy, &id)?))
        })
        .collect();
    let mut reviewed = scope.clone();
    reviewed.insert(mission_id.to_string());
    let mut record_refs = BTreeMap::new();
    for review in index
        .reviews()
        .filter(|review| reviewed.contains(&review.target))
    {
        record_refs.insert(
            review.header.id.clone(),
            RecordRef {
                kind: event_name(&review.event).to_string(),
                summary: review
                    .summary
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" "),
            },
        );
    }
    for exemption in &base_exemptions {
        record_refs.insert(
            exemption.id.clone(),
            RecordRef {
                kind: "trivial".into(),
                summary: exemption
                    .summary
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" "),
            },
        );
    }
    for record in index.evidence_records() {
        if record
            .data
            .target
            .as_ref()
            .is_some_and(|target| reviewed.contains(&target.id))
        {
            record_refs.insert(
                record.header.id.clone(),
                RecordRef {
                    kind: "receipt".into(),
                    summary: format!(
                        "{}: {}",
                        record.data.result.as_str(),
                        first_sentence(&record.header.title)
                    ),
                },
            );
        }
    }
    let why = mission
        .strategy
        .as_ref()
        .and_then(|strategy| std::fs::read_to_string(read.repo.join(&strategy.path)).ok())
        .and_then(|text| atelier_core::markdown_section(&text, "why"))
        .or_else(|| {
            record
                .sections
                .section(IssueSectionName::Background)
                .map(str::to_string)
        })
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty());

    Ok(Debrief {
        status_category: category(&mission),
        outcome: record
            .sections
            .section(IssueSectionName::Outcome)
            .unwrap_or_default()
            .to_string(),
        why,
        strategy: mission
            .strategy
            .as_ref()
            .map(|strategy| strategy.path.clone()),
        generated_from,
        timeline,
        criteria,
        delivered_by,
        asks,
        planned,
        work,
        left,
        found,
        dropped,
        added,
        events,
        validation,
        decisions,
        mission_events,
        base_exemptions,
        refs,
        record_refs,
        planned_done,
        planned_total: planned_records.len(),
        mission,
    })
}

/// How the mission was checked, from its items' reviews and the receipts
/// that judge its criteria.
fn validation(
    index: &RecordIndex,
    criteria: &MissionProof,
    work: &[WorkEntry],
    asker_of: &dyn Fn(&str) -> Option<String>,
) -> Validation {
    let mut validation = Validation::default();
    let mission_id = criteria.criteria.mission_id.as_str();
    for entry in work {
        let review = &entry.review;
        validation.findings.fixed += review.fixed;
        validation.findings.deferred += review.deferred;
        validation.findings.no_action += review.no_action;
        validation.findings.undisposed += review.undisposed;
        if review.approved {
            validation.approved += 1;
        } else if review.trivial {
            validation.trivial += 1;
        } else if index
            .issue_record(&entry.id)
            .is_none_or(|record| record.issue.issue_type == "decision")
        {
            // A decision record is decided, not code-reviewed.
        } else if review.recorded {
            validation.not_approved.push(entry.id.clone());
        } else {
            validation.unreviewed.push(entry.id.clone());
        }
    }

    // Receipts naming this mission's criteria, grouped by who recorded them
    // against which commit.
    let mut runs: BTreeMap<(String, String), ValidationRun> = BTreeMap::new();
    for record in index.evidence_records() {
        if record.data.criteria.is_empty()
            || record.data.criteria_mission.as_deref() != Some(mission_id)
        {
            continue;
        }
        let identity = record
            .data
            .agent_identity
            .clone()
            .or_else(|| record.data.producer.clone())
            .unwrap_or_else(|| "unnamed producer".into());
        let run = runs
            .entry((identity.clone(), record.data.commit.clone()))
            .or_insert_with(|| ValidationRun {
                identity,
                at: record.data.captured_at,
                commit: record.data.commit.clone(),
                code_commits_since: None,
                receipts: Vec::new(),
                later_work: Vec::new(),
            });
        run.at = run.at.min(record.data.captured_at);
        let since = criteria
            .proofs
            .iter()
            .filter_map(|proof| proof.deciding())
            .find(|receipt| receipt.evidence_id == record.header.id)
            .and_then(|receipt| receipt.code_commits_since);
        if since.is_some() {
            run.code_commits_since = since;
        }
        run.receipts.push(ValidationReceipt {
            id: record.header.id.clone(),
            target: record
                .data
                .target
                .as_ref()
                .map(|target| target.id.clone())
                .unwrap_or_else(|| mission_id.to_string()),
            result: record.data.result,
            criteria: record.data.criteria.clone(),
            summary: record.header.title.clone(),
        });
    }
    let order = |slug: &str| {
        criteria
            .proofs
            .iter()
            .position(|proof| proof.criterion.slug == slug)
            .unwrap_or(usize::MAX)
    };
    let mut runs = runs.into_values().collect::<Vec<_>>();
    for run in &mut runs {
        run.receipts.sort_by_key(|receipt| {
            (
                receipt
                    .criteria
                    .iter()
                    .map(|criterion| order(&criterion.slug))
                    .min(),
                receipt.id.clone(),
            )
        });
        if run.code_commits_since.is_some_and(|count| count > 0) {
            run.later_work = work
                .iter()
                .filter(|entry| entry.closed_at.is_some_and(|closed| closed > run.at))
                .map(|entry| entry.id.clone())
                .collect();
        }
    }
    runs.sort_by(|a, b| (a.at, &a.identity).cmp(&(b.at, &b.identity)));
    validation.runs = runs;
    validation.awaiting = criteria
        .proofs
        .iter()
        .filter(|proof| proof.mark != CriterionMark::Proven)
        .map(|proof| {
            (
                proof.criterion.slug.clone(),
                proof.mark,
                asker_of(&proof.criterion.slug),
            )
        })
        .collect();
    validation
}

/// What the mission did, oldest first. Undated moments come last.
#[allow(clippy::too_many_arguments)]
fn events(
    timeline: &MissionTimeline,
    work: &[WorkEntry],
    added: &[String],
    added_at: &BTreeMap<String, DateTime<Utc>>,
    found: &[OpenEntry],
    dropped: &[OpenEntry],
    validation: &Validation,
    (mission_events, base_exemptions): (&[MissionEvent], &[MissionEvent]),
    index: &RecordIndex,
) -> Vec<TimedEvent> {
    let mut events = Vec::new();
    if let Some(started) = &timeline.started {
        events.push(TimedEvent {
            at: Some(started.time),
            event: Event::Started,
        });
    }
    for entry in work {
        events.push(TimedEvent {
            at: entry.closed_at,
            event: Event::Delivered(entry.id.clone()),
        });
    }
    for id in added {
        events.push(TimedEvent {
            at: added_at.get(id).copied(),
            event: Event::Added(id.clone()),
        });
    }
    // Follow-ups a delivered item's review filed are told in its entry.
    let delivered = work
        .iter()
        .flat_map(|entry| std::iter::once(&entry.id).chain(&entry.children))
        .collect::<BTreeSet<_>>();
    for entry in found {
        if entry
            .from
            .as_ref()
            .is_none_or(|from| !delivered.contains(from))
        {
            events.push(TimedEvent {
                at: entry.filed_at,
                event: Event::Filed(entry.id.clone()),
            });
        }
    }
    for entry in dropped {
        let at = index
            .issue_record(&entry.id)
            .map(|record| record.issue.closed_at.unwrap_or(record.issue.updated_at));
        events.push(TimedEvent {
            at,
            event: Event::Dropped(entry.id.clone()),
        });
    }
    for (number, run) in validation.runs.iter().enumerate() {
        events.push(TimedEvent {
            at: Some(run.at),
            event: Event::Validated(number),
        });
    }
    for (number, event) in mission_events.iter().enumerate() {
        events.push(TimedEvent {
            at: Some(event.created_at),
            event: Event::MissionReview(number),
        });
    }
    for (number, exemption) in base_exemptions.iter().enumerate() {
        events.push(TimedEvent {
            at: Some(exemption.created_at),
            event: Event::BaseExemption(number),
        });
    }
    if let Some(closed) = &timeline.closed {
        events.push(TimedEvent {
            at: Some(closed.time),
            event: Event::Closed,
        });
    }
    // A stable sort keeps the push order for equal times: the start before
    // what follows it, the close after what precedes it.
    events.sort_by_key(|event| (event.at.is_none(), event.at));
    events
}

/// Whether `text` names `word` as a whole word: not inside a longer
/// identifier, so `S1` does not match `S10` or `atelier-S1x`.
fn names_word(text: &str, word: &str) -> bool {
    let part = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';
    text.match_indices(word).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + word.len()..].chars().next();
        before.is_none_or(|c| !part(c)) && after.is_none_or(|c| !part(c))
    })
}

/// Whether `text` names the criterion: `S<n>` and hyphenated slugs as whole
/// words, a one-word slug such as `fast` only as code, since as a plain
/// word it is ordinary English.
fn names_criterion(text: &str, slug: &str) -> bool {
    let distinctive = slug.contains('-') || slug.chars().any(|c| c.is_ascii_digit());
    if distinctive {
        names_word(text, slug)
    } else {
        text.contains(&format!("`{slug}`"))
    }
}

/// The text with every parenthesised aside removed.
fn outside_parentheses(text: &str) -> String {
    let mut depth = 0usize;
    text.chars()
        .filter(|c| match c {
            '(' => {
                depth += 1;
                false
            }
            ')' => {
                depth = depth.saturating_sub(1);
                false
            }
            _ => depth == 0,
        })
        .collect()
}

fn close_outcome(index: &RecordIndex, id: &str) -> String {
    index
        .issue_record(id)
        .and_then(|record| record.sections.close_reason.as_deref())
        .map(|text| CloseReport::parse(text).outcome)
        .unwrap_or_default()
}

fn issue_ref(index: &RecordIndex, policy: Option<&WorkflowPolicy>, id: &str) -> Option<IssueRef> {
    let record = index.issue_record(id)?;
    let issue = &record.issue;
    let lead = [
        IssueSectionName::Goal,
        IssueSectionName::Question,
        IssueSectionName::Outcome,
    ]
    .iter()
    .find_map(|name| record.sections.section(*name))
    .unwrap_or_default();
    Some(IssueRef {
        id: id.to_string(),
        title: issue.title.clone(),
        issue_type: issue.issue_type.clone(),
        status: issue.status.clone(),
        status_category: policy
            .and_then(|policy| policy.status_category(&issue.status))
            .map(str::to_string),
        summary: first_sentence(lead),
        outcome: Some(first_sentence(&close_outcome(index, id))).filter(|text| !text.is_empty()),
        parent: issue.parent_id.clone(),
    })
}

/// The text's first sentence on one line, at most 240 characters.
pub fn first_sentence(text: &str) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let end = flat.find(". ").map_or(flat.len(), |at| at + 1);
    let sentence = &flat[..end];
    if sentence.chars().count() > 240 {
        format!("{}…", sentence.chars().take(239).collect::<String>())
    } else {
        sentence.to_string()
    }
}

/// The newest receipt attached to the item: its ID, result and summary.
fn newest_receipt(
    index: &RecordIndex,
    issue_id: &str,
) -> Result<Option<(String, EvidenceResult, String)>> {
    let mut newest: Option<&atelier_core::EvidenceRecord> = None;
    for link in index.list_record_links("issue", issue_id)? {
        if link.relation_type != "validates" {
            continue;
        }
        let evidence_id = if link.source_kind == "evidence" {
            link.source_id
        } else if link.target_kind == "evidence" {
            link.target_id
        } else {
            continue;
        };
        let record = index.require_evidence(&evidence_id)?;
        if newest.is_none_or(|known| {
            (record.data.captured_at, &record.header.id)
                > (known.data.captured_at, &known.header.id)
        }) {
            newest = Some(record);
        }
    }
    Ok(newest.map(|record| {
        (
            record.header.id.clone(),
            record.data.result,
            record.header.title.clone(),
        )
    }))
}

fn event_name(event: &ReviewEvent) -> &'static str {
    match event {
        ReviewEvent::Review => "review",
        ReviewEvent::Approval => "approval",
        ReviewEvent::Finding { .. } => "finding",
        ReviewEvent::Disposition { .. } => "disposition",
        ReviewEvent::TrivialExemption => "trivial",
    }
}

/// Review events whose target is the mission itself, oldest first.
pub fn mission_events(index: &RecordIndex, mission_id: &str) -> Vec<MissionEvent> {
    let mut events = index
        .reviews()
        .filter(|review| review.target == mission_id)
        .map(|review| MissionEvent {
            id: review.header.id.clone(),
            event: event_name(&review.event),
            identity: review.identity.clone(),
            commit: review.commit.clone(),
            created_at: review.header.created_at,
            summary: review.summary.clone(),
        })
        .collect::<Vec<_>>();
    events.sort_by(|a, b| (a.created_at, &a.id).cmp(&(b.created_at, &b.id)));
    events
}

/// Base exemptions recorded from the mission's start to its done claim,
/// oldest first; none before it starts.
fn base_exemptions(index: &RecordIndex, timeline: &MissionTimeline) -> Vec<MissionEvent> {
    let Some(started) = &timeline.started else {
        return Vec::new();
    };
    let mut events = index
        .base_exemptions()
        .filter(|exemption| {
            exemption.header.created_at >= started.time
                && timeline
                    .closed
                    .as_ref()
                    .is_none_or(|closed| exemption.header.created_at <= closed.time)
        })
        .map(|exemption| MissionEvent {
            id: exemption.header.id.clone(),
            event: "trivial",
            identity: exemption.identity.clone(),
            commit: exemption.commit.clone(),
            created_at: exemption.header.created_at,
            summary: exemption.summary.clone(),
        })
        .collect::<Vec<_>>();
    events.sort_by(|a, b| (a.created_at, &a.id).cmp(&(b.created_at, &b.id)));
    events
}

/// One scoped item as the brief shows it (atelier-acrt).
#[derive(Debug, Clone)]
pub struct BriefItem {
    pub issue: Issue,
    pub status_category: Option<String>,
    pub parent: Option<String>,
    /// The `size-*` label's size, upper-cased, such as `M`.
    pub size: Option<String>,
    /// Blockers that are not done.
    pub open_blockers: Vec<String>,
    pub unknowns: Option<String>,
    /// Labelled `needs-human`.
    pub needs_human: bool,
}

/// A mission's plan, for review before work starts.
#[derive(Debug, Clone)]
pub struct Brief {
    pub mission: Issue,
    pub status_category: Option<String>,
    pub outcome: String,
    pub strategy: Option<String>,
    pub generated_from: Option<String>,
    pub criteria: crate::criteria::MissionCriteria,
    pub items: Vec<BriefItem>,
    /// Open decision records and needs-human items in scope.
    pub human_decisions: Vec<Issue>,
    /// The strategy's Risks and Unknowns section.
    pub risks: Option<String>,
    pub mission_events: Vec<MissionEvent>,
}

impl Brief {
    pub fn blocked(&self) -> impl Iterator<Item = &BriefItem> {
        self.items
            .iter()
            .filter(|item| !item.open_blockers.is_empty())
    }
}

pub fn brief(
    index: &RecordIndex,
    policy: Option<&WorkflowPolicy>,
    read: &mut ReadContext<'_>,
    mission_id: &str,
) -> Result<Brief> {
    mission_objective_kind(index, mission_id)?;
    let record = index
        .issue_record(mission_id)
        .ok_or_else(|| index.not_found(crate::record_ref::RecordKind::Issue, mission_id))?;
    let mission = record.issue.clone();
    let category = |issue: &Issue| {
        policy
            .and_then(|policy| policy.status_category(&issue.status))
            .map(str::to_string)
    };
    let done = |id: &str| {
        index
            .issue_record(id)
            .is_some_and(|record| category(&record.issue).as_deref() == Some("done"))
    };
    let scope = mission_issue_ids(index, mission_id)?;

    let brief_item = |issue: &Issue, parent: Option<String>| {
        let record = index.issue_record(&issue.id);
        let labels = record.map(|record| record.labels.as_slice()).unwrap_or(&[]);
        BriefItem {
            issue: issue.clone(),
            status_category: category(issue),
            parent,
            size: labels
                .iter()
                .find_map(|label| label.strip_prefix(crate::lint::SIZE_LABEL_PREFIX))
                .map(str::to_uppercase),
            open_blockers: index
                .get_blockers(&issue.id)
                .into_iter()
                .filter(|blocker| !done(blocker))
                .collect(),
            // A done item's unknowns are settled.
            unknowns: record
                .filter(|_| category(issue).as_deref() != Some("done"))
                .and_then(|record| record.sections.section(IssueSectionName::Unknowns))
                .map(str::to_string),
            needs_human: labels.iter().any(|label| label == "needs-human"),
        }
    };
    let mut items = Vec::new();
    for (id, parent) in scope_tree(index, &scope) {
        if let Some(record) = index.issue_record(&id) {
            items.push(brief_item(&record.issue, parent));
        }
    }
    let human_decisions = items
        .iter()
        .filter(|item| item.status_category.as_deref() != Some("done"))
        .filter(|item| item.issue.issue_type == "decision" || item.needs_human)
        .map(|item| item.issue.clone())
        .collect();
    let risks = mission.strategy.as_ref().and_then(|strategy| {
        std::fs::read_to_string(read.repo.join(&strategy.path))
            .ok()
            .and_then(|text| atelier_core::markdown_section(&text, "risks and unknowns"))
            .map(|section| section.trim().to_string())
            .filter(|section| !section.is_empty())
    });
    Ok(Brief {
        status_category: category(&mission),
        outcome: record
            .sections
            .section(IssueSectionName::Outcome)
            .unwrap_or_default()
            .to_string(),
        strategy: mission
            .strategy
            .as_ref()
            .map(|strategy| strategy.path.clone()),
        generated_from: read.pinned("HEAD").ok(),
        criteria: crate::criteria::mission_criteria(index, read.repo, mission_id)?,
        items,
        human_decisions,
        risks,
        mission_events: mission_events(index, mission_id),
        mission,
    })
}

/// The mission's scope as both reviews list it: each item linked to the
/// mission, followed by its children, each child with its parent.
pub fn scope_tree(index: &RecordIndex, scope: &BTreeSet<String>) -> Vec<(String, Option<String>)> {
    let mut tree = Vec::new();
    for id in scope {
        let parent = index
            .issue_record(id)
            .and_then(|record| record.issue.parent_id.clone())
            .filter(|parent| scope.contains(parent));
        if parent.is_some() {
            continue;
        }
        tree.push((id.clone(), None));
        for child in index.child_ids(id) {
            if scope.contains(child) {
                tree.push((child.clone(), Some(id.clone())));
            }
        }
    }
    tree
}
