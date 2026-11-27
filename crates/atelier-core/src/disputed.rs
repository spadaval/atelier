//! The disagreements the records hold, which `list --disputed`, `show
//! <mission>` and a lead's `status` print: each rebuttal in an item's
//! Responses with what the next round made of it, each round of a plan's
//! review whose reviewers gave different verdicts, and each open fix-now
//! finding answered `not built`, which waits on the lead. Nothing refuses
//! on a rebuttal, nor on one split round: two in a row end the review of a
//! plan change (`plan::SPLITS`). The third `merge` holds until the lead
//! rules.

use crate::model::{
    FindingName, Now, Outcome, Recommendation, Response, ReviewName, Status, Thread, Verdict,
};
use crate::records::{ItemDir, MissionDir, Problem};
use crate::review::waits_on_lead;
use crate::view::Located;

/// What a later round's reviewer made of a rebutted finding it carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Judged {
    /// The finding stands: the rebuttal did not hold.
    Kept,
    /// The rebuttal held, or the finding was wrong.
    Withdrawn,
    Fixed,
    /// Kept, but no longer for this item: defer or no action.
    Lowered(Recommendation),
    /// Settled by a decision, in a review written before `decide` was
    /// removed.
    Decided(Outcome),
}

/// A finding the implementer rebutted, and what each review of the next
/// round that carries it made of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rebuttal {
    pub item: String,
    pub mission: Option<String>,
    pub finding: FindingName,
    pub title: String,
    /// The rebuttal's fact, as Responses gives it.
    pub fact: String,
    /// Each carrying finding of a review with a verdict, and what it made
    /// of the rebutted one; empty while no such review has judged it.
    pub judged: Vec<(FindingName, Judged)>,
}

/// A round of a mission's plan review whose reviewers gave different
/// verdicts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitRound {
    pub mission: String,
    pub round: u32,
    /// Each review of the round with a verdict: its name, its reviewer, the
    /// model it gave, and the verdict.
    pub reviews: Vec<(ReviewName, Option<Thread>, Option<String>, Verdict)>,
}

/// An open fix-now finding the implementer answered `not built`: the
/// question to the lead, which `merge` holds until the lead rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotBuilt {
    pub item: String,
    pub mission: Option<String>,
    pub finding: FindingName,
    pub title: String,
    /// Why it was not built, as Responses gives it.
    pub reason: String,
}

/// The disagreements of some items and missions.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Disputes {
    pub rebuttals: Vec<Rebuttal>,
    pub plans: Vec<SplitRound>,
    pub not_built: Vec<NotBuilt>,
}

impl Disputes {
    pub fn is_empty(&self) -> bool {
        self.rebuttals.is_empty() && self.plans.is_empty() && self.not_built.is_empty()
    }
}

/// Each open fix-now finding of `item` that waits on the lead, in the
/// order of its reviews.
pub fn not_built(item: &ItemDir) -> Result<Vec<NotBuilt>, Problem> {
    let (findings, responses) = (item.findings()?, item.responses());
    let mut found = Vec::new();
    for finding in item.open_findings()? {
        if let Some(reason) = waits_on_lead(&finding.name, &findings, &responses) {
            found.push(NotBuilt {
                item: item.item.id.clone(),
                mission: item.item.mission.clone(),
                reason: reason.to_string(),
                finding: finding.name,
                title: finding.title,
            });
        }
    }
    Ok(found)
}

/// Each finding of `item` whose line in Responses is a rebuttal, in the
/// order of Responses.
pub fn rebuttals(item: &ItemDir) -> Result<Vec<Rebuttal>, Problem> {
    let findings = item.findings()?;
    let mut found = Vec::new();
    for response in item.responses() {
        let Response::Rebutted { finding, fact } = response else {
            continue;
        };
        let title = (findings.iter().find(|f| f.name == finding))
            .map_or(String::new(), |f| f.title.clone());
        let judged = findings
            .iter()
            .filter(|later| {
                later.was.as_ref() == Some(&finding)
                    && later.name.review.round > finding.review.round
            })
            .filter(|later| {
                let review = item.reviews.iter().find(|r| r.name == later.name.review);
                review.is_some_and(|r| r.review.verdict.is_some())
            })
            .filter_map(|later| {
                let judged = match (later.now, later.decided) {
                    (_, Some(outcome)) => Judged::Decided(outcome),
                    (Some(Now::Fixed), _) => Judged::Fixed,
                    (Some(Now::Withdrawn), _) => Judged::Withdrawn,
                    (Some(Now::Kept), _) if later.recommendation != Recommendation::FixNow => {
                        Judged::Lowered(later.recommendation)
                    }
                    (Some(Now::Kept), _) => Judged::Kept,
                    (None, None) => return None,
                };
                Some((later.name.clone(), judged))
            })
            .collect();
        found.push(Rebuttal {
            item: item.item.id.clone(),
            mission: item.item.mission.clone(),
            finding,
            title,
            fact,
            judged,
        });
    }
    Ok(found)
}

/// Each round of `mission`'s plan review in which one reviewer approved
/// and another asked for changes.
pub fn split_rounds(mission: &MissionDir) -> Vec<SplitRound> {
    let mut rounds: Vec<u32> = mission.reviews.iter().map(|r| r.name.round).collect();
    rounds.sort();
    rounds.dedup();
    rounds
        .into_iter()
        .filter_map(|round| {
            let reviews: Vec<_> = (mission.reviews.iter())
                .filter(|r| r.name.round == round)
                .filter_map(|r| {
                    let verdict = r.review.verdict?;
                    let reviewer = r.review.reviewers.holder().cloned();
                    Some((r.name.clone(), reviewer, r.review.model.clone(), verdict))
                })
                .collect();
            let split = reviews.iter().any(|r| r.3 == Verdict::Approve)
                && reviews.iter().any(|r| r.3 == Verdict::Changes);
            split.then(|| SplitRound {
                mission: mission.mission.id.clone(),
                round,
                reviews,
            })
        })
        .collect()
}

/// The disagreements among `items`, open and done, each read as `status`
/// reads it, and `missions`' plan reviews; with `mission`, only that
/// mission's and its items'. A dropped item's are left out.
pub fn disputes(
    items: &[Located],
    missions: &[MissionDir],
    mission: Option<&str>,
) -> Result<Disputes, Problem> {
    let mut disputes = Disputes::default();
    for record in items.iter().filter_map(Located::record) {
        let of = mission.is_none_or(|id| record.item.mission.as_deref() == Some(id));
        if of && record.item.status != Status::Dropped {
            disputes.rebuttals.extend(rebuttals(record)?);
            if record.item.status != Status::Done {
                disputes.not_built.extend(not_built(record)?);
            }
        }
    }
    for dir in missions {
        if mission.is_none_or(|id| dir.mission.id == id) {
            disputes.plans.extend(split_rounds(dir));
        }
    }
    Ok(disputes)
}
