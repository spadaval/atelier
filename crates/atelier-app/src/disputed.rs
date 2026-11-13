//! The disagreements the records hold, which `list --disputed`, `show
//! <mission>` and a lead's `status` print: each rebuttal in an item's
//! Responses with what the next round made of it, and each round of a
//! plan's review whose reviewers gave different verdicts. They are found,
//! never enforced: nothing refuses or warns on one.

use crate::view::Located;
use atelier_core::{
    FindingName, Now, Outcome, Recommendation, Response, ReviewName, Status, Thread, Verdict,
};
use atelier_records::{ItemDir, MissionDir, Problem};

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

/// The disagreements of some items and missions.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Disputes {
    pub rebuttals: Vec<Rebuttal>,
    pub plans: Vec<SplitRound>,
}

impl Disputes {
    pub fn is_empty(&self) -> bool {
        self.rebuttals.is_empty() && self.plans.is_empty()
    }
}

/// Each finding of `item` whose line in Responses is a rebuttal, in the
/// order of Responses.
pub fn rebuttals(item: &ItemDir) -> Result<Vec<Rebuttal>, Problem> {
    let findings = item.findings()?;
    let mut found = Vec::new();
    for response in item.responses()? {
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
        }
    }
    for dir in missions {
        if mission.is_none_or(|id| dir.mission.id == id) {
            disputes.plans.extend(split_rounds(dir));
        }
    }
    Ok(disputes)
}
