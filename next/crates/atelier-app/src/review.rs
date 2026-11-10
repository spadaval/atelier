//! What a round's review carries from the rounds before it.

use atelier_core::{is_open, Decision, Finding, FindingName, Response};
use atelier_records::{ItemDir, MissionDir, Problem, ReviewFile};

/// A fix-now finding an earlier round left open.
pub struct Carried {
    /// The finding, under its latest name.
    pub finding: Finding,
    /// The text of each finding in its chain, oldest first: the one first
    /// raised, then each that carried it as kept.
    pub texts: Vec<(FindingName, String)>,
    /// The author's line for it in the Report's Responses.
    pub response: Option<Response>,
}

/// The fix-now findings that the rounds before `round` leave open, which
/// round `round`'s review file carries. Its own file is not read, so what
/// it carries does not change as its reviewer writes in it.
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
    let open = findings.iter().filter(|f| is_open(f, &findings, decisions));
    let carried = open.map(|finding| {
        let mut texts = vec![(finding.name.clone(), text(&finding.name))];
        let mut at = finding;
        // Each step goes back a round, so the chain ends.
        while let Some(was) = at
            .was
            .as_ref()
            .filter(|was| was.review.round < at.name.review.round)
        {
            let Some(found) = findings.iter().find(|f| f.name == *was) else {
                break;
            };
            texts.push((found.name.clone(), text(&found.name)));
            at = found;
        }
        texts.reverse();
        let response = responses
            .iter()
            .find(|response| *response.finding() == finding.name)
            .cloned();
        Carried {
            finding: finding.clone(),
            texts,
            response,
        }
    });
    Ok(carried.collect())
}
