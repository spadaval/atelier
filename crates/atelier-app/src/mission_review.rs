//! Mission reviews (atelier-rx22, atelier-acrt): one view model per review,
//! built from records and Git history and rendered by the CLI as dense
//! Markdown and as a static HTML page, so the two forms cannot disagree.

use std::collections::BTreeSet;

use anyhow::Result;
use atelier_core::{
    CloseReport, EvidenceResult, FindingDecision, Issue, IssueSectionName, ReviewEvent,
};

use crate::criteria::{mission_proof_until, CriterionMark, MissionProof};
use crate::decisions::{mission_decisions, Decisions};
use crate::git_freshness::ReadContext;
use crate::objective_graph::{mission_issue_ids, mission_objective_kind};
use crate::record_history::{mission_timeline, MissionTimeline};
use crate::record_index::RecordIndex;
use crate::workflow_policy::WorkflowPolicy;

/// One scoped item as the debrief shows it.
#[derive(Debug, Clone)]
pub struct ItemReview {
    pub issue: Issue,
    pub status_category: Option<String>,
    /// The parent within the mission's scope, for indenting children.
    pub parent: Option<String>,
    /// The Close Reason read as the implementer report; `None` while open.
    pub report: Option<CloseReport>,
}

/// A fact derived from records and history that departs from the plan.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Deviation {
    /// Linked to the mission after it started.
    ScopeAdded {
        id: String,
        time: chrono::DateTime<chrono::Utc>,
    },
    /// Retired with `supersede`; the reason is its Close Reason.
    ScopeDropped { id: String, reason: String },
    /// A finding deferred to a follow-up.
    DeferredFinding {
        finding: String,
        target: String,
        follow_up: String,
        summary: String,
    },
    /// The newest receipt on an item does not pass.
    NonPassEvidence {
        evidence: String,
        target: String,
        result: EvidenceResult,
        summary: String,
    },
    /// Review was waived as trivial.
    TrivialExemption {
        review: String,
        target: String,
        summary: String,
    },
    /// A done claim later withdrawn.
    Reopened {
        id: String,
        time: chrono::DateTime<chrono::Utc>,
    },
    /// A proven criterion whose deciding proof predates code that landed
    /// before the mission closed (or before HEAD while it is open).
    AgedProof {
        slug: String,
        evidence: String,
        commits: usize,
    },
}

/// A review event on the mission itself, such as a plan approval.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MissionEvent {
    pub id: String,
    pub event: &'static str,
    pub identity: String,
    pub commit: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct Debrief {
    pub mission: Issue,
    pub status_category: Option<String>,
    pub outcome: String,
    pub strategy: Option<String>,
    /// The commit the review was generated from; `None` without history.
    pub generated_from: Option<String>,
    pub timeline: MissionTimeline,
    pub criteria: MissionProof,
    pub items: Vec<ItemReview>,
    pub decisions: Decisions,
    pub deviations: Vec<Deviation>,
    pub mission_events: Vec<MissionEvent>,
    /// Open scoped items, then defer follow-ups outside the scope.
    pub follow_ups: Vec<Issue>,
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
    let scope = mission_issue_ids(index, mission_id)?;
    // Items linked to the mission directly, as opposed to their children.
    let linked = scope
        .iter()
        .filter(|id| {
            index
                .issue_record(id)
                .and_then(|record| record.issue.parent_id.as_ref())
                .is_none_or(|parent| !scope.contains(parent))
        })
        .cloned()
        .collect::<BTreeSet<_>>();
    let children = scope
        .iter()
        .filter_map(|id| {
            let parent = index.issue_record(id)?.issue.parent_id.clone()?;
            scope.contains(&parent).then(|| (id.clone(), parent))
        })
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

    let mut items = Vec::new();
    for id in &linked {
        let Some(record) = index.issue_record(id) else {
            continue;
        };
        items.push(item_review(index, policy, &record.issue, None));
        for child in index.child_ids(id) {
            if let Some(child) = index.issue_record(child) {
                items.push(item_review(index, policy, &child.issue, Some(id.clone())));
            }
        }
    }

    let mut deviations = Vec::new();
    for (id, moment) in &timeline.added_after_start {
        deviations.push(Deviation::ScopeAdded {
            id: id.clone(),
            time: moment.time,
        });
    }
    for item in &items {
        if item.issue.status == "superseded" {
            deviations.push(Deviation::ScopeDropped {
                id: item.issue.id.clone(),
                reason: item
                    .report
                    .as_ref()
                    .map(|report| report.outcome.clone())
                    .unwrap_or_default(),
            });
        }
    }
    let mut follow_up_ids = Vec::new();
    let mut reviewed = scope.clone();
    reviewed.insert(mission_id.to_string());
    let mut mission_events = Vec::new();
    for review in index.reviews() {
        if review.target == mission_id {
            mission_events.push(MissionEvent {
                id: review.header.id.clone(),
                event: event_name(&review.event),
                identity: review.identity.clone(),
                commit: review.commit.clone(),
                created_at: review.header.created_at,
                summary: review.summary.clone(),
            });
        }
        if !reviewed.contains(&review.target) {
            continue;
        }
        match &review.event {
            ReviewEvent::Disposition {
                finding,
                decision: FindingDecision::Defer,
                follow_up: Some(follow_up),
                ..
            } => {
                deviations.push(Deviation::DeferredFinding {
                    finding: finding.clone(),
                    target: review.target.clone(),
                    follow_up: follow_up.clone(),
                    summary: index
                        .reviews()
                        .find(|candidate| &candidate.header.id == finding)
                        .map(|finding| finding.summary.clone())
                        .unwrap_or_else(|| review.summary.clone()),
                });
                if !follow_up_ids.contains(follow_up) {
                    follow_up_ids.push(follow_up.clone());
                }
            }
            ReviewEvent::TrivialExemption => deviations.push(Deviation::TrivialExemption {
                review: review.header.id.clone(),
                target: review.target.clone(),
                summary: review.summary.clone(),
            }),
            _ => {}
        }
    }
    mission_events.sort_by(|a, b| (a.created_at, &a.id).cmp(&(b.created_at, &b.id)));
    for id in &scope {
        if let Some(receipt) = newest_receipt(index, id)? {
            if receipt.1 != EvidenceResult::Pass {
                deviations.push(Deviation::NonPassEvidence {
                    evidence: receipt.0,
                    target: id.clone(),
                    result: receipt.1,
                    summary: receipt.2,
                });
            }
        }
    }
    for proof in &criteria.proofs {
        if proof.mark != CriterionMark::Proven {
            continue;
        }
        if let Some(receipt) = proof.deciding() {
            if let Some(commits) = receipt.code_commits_since.filter(|commits| *commits > 0) {
                deviations.push(Deviation::AgedProof {
                    slug: proof.criterion.slug.clone(),
                    evidence: receipt.evidence_id.clone(),
                    commits,
                });
            }
        }
    }
    for (id, moment) in &timeline.reopened {
        deviations.push(Deviation::Reopened {
            id: id.clone(),
            time: moment.time,
        });
    }

    let mut follow_ups = items
        .iter()
        .filter(|item| item.status_category.as_deref() != Some("done"))
        .map(|item| item.issue.clone())
        .collect::<Vec<_>>();
    for id in follow_up_ids {
        if scope.contains(&id) {
            continue;
        }
        if let Some(record) = index.issue_record(&id) {
            follow_ups.push(record.issue.clone());
        }
    }

    Ok(Debrief {
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
        mission,
        generated_from,
        timeline,
        criteria,
        items,
        decisions: mission_decisions(index, policy, mission_id)?,
        deviations,
        mission_events,
        follow_ups,
    })
}

fn item_review(
    index: &RecordIndex,
    policy: Option<&WorkflowPolicy>,
    issue: &Issue,
    parent: Option<String>,
) -> ItemReview {
    let status_category = policy
        .and_then(|policy| policy.status_category(&issue.status))
        .map(str::to_string);
    let report = index
        .issue_record(&issue.id)
        .and_then(|record| record.sections.close_reason.as_deref())
        .filter(|_| status_category.as_deref() == Some("done"))
        .map(CloseReport::parse);
    ItemReview {
        issue: issue.clone(),
        status_category,
        parent,
        report,
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
