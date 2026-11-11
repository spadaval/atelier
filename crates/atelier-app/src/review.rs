//! What a round's review carries from the rounds before it.

use atelier_core::{is_open, Decision, Finding, FindingName, Response};
use atelier_records::{ItemDir, MissionDir, Problem, ReviewFile};

/// A finding an earlier round left open, or that a decision settled and no
/// round has carried yet.
pub struct Carried {
    /// The finding, under its latest name.
    pub finding: Finding,
    /// The text of each finding in its chain, oldest first: the one first
    /// raised, then each that carried it as kept.
    pub texts: Vec<(FindingName, String)>,
    /// The author's line for it in the Report's Responses.
    pub response: Option<Response>,
    /// The decision that settled it, which the review carries as
    /// `Decided:`.
    pub decision: Option<Decision>,
}

/// The finding named `name` among `findings`, then each finding it carries,
/// back to the one first raised.
pub fn chain<'a>(name: &FindingName, findings: &'a [Finding]) -> Vec<&'a Finding> {
    let mut chain: Vec<&Finding> = findings
        .iter()
        .find(|f| f.name == *name)
        .into_iter()
        .collect();
    // Each step goes back a round, so the chain ends.
    while let Some(was) = chain.last().and_then(|at| {
        at.was
            .as_ref()
            .filter(|was| was.review.round < at.name.review.round)
    }) {
        let Some(found) = findings.iter().find(|f| f.name == *was) else {
            break;
        };
        chain.push(found);
    }
    chain
}

/// The findings that round `round`'s review file carries from the rounds
/// before it: each fix-now finding they leave open, and each a decision
/// settled that none of them carries. Its own file is not read, so what it
/// carries does not change as its reviewer writes in it.
pub fn carried(item: &ItemDir, round: u32) -> Result<Vec<Carried>, Problem> {
    carry(&item.reviews, &item.responses()?, &item.decisions()?, round)
}

/// What round `round` of a mission's plan reviews carries, as [`carried`]
/// gives it for an item. A mission has no Responses and no decisions.
pub fn carried_plan(mission: &MissionDir, round: u32) -> Result<Vec<Carried>, Problem> {
    carry(&mission.reviews, &[], &[], round)
}

fn carry(
    reviews: &[ReviewFile],
    responses: &[Response],
    decisions: &[Decision],
    round: u32,
) -> Result<Vec<Carried>, Problem> {
    let earlier: Vec<&ReviewFile> = reviews.iter().filter(|r| r.name.round < round).collect();
    let mut findings = Vec::new();
    for review in &earlier {
        findings.extend(review.findings()?);
    }
    let text = |name: &FindingName| {
        earlier
            .iter()
            .find(|review| review.name == name.review)
            .and_then(|review| review.finding_text(name.number))
            .unwrap_or_default()
            .to_string()
    };
    let settled = |finding: &Finding| {
        let carried = findings
            .iter()
            .any(|f| f.was.as_ref() == Some(&finding.name));
        let decision = decisions
            .iter()
            .find(|d| d.finding.as_ref() == Some(&finding.name) && d.outcome.is_some());
        decision.filter(|_| !carried).cloned()
    };
    let mut carried = Vec::new();
    for finding in &findings {
        let decision = settled(finding);
        if decision.is_none() && !is_open(finding, &findings, decisions) {
            continue;
        }
        let texts = chain(&finding.name, &findings).into_iter().rev();
        let response = responses
            .iter()
            .find(|response| *response.finding() == finding.name)
            .cloned();
        carried.push(Carried {
            finding: finding.clone(),
            texts: texts.map(|f| (f.name.clone(), text(&f.name))).collect(),
            response,
            decision,
        });
    }
    Ok(carried)
}
