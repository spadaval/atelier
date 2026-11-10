use crate::body::section;
use crate::{ids, ItemDir, MissionDir, Problem};
use atelier_core::{is_open, Finding, FindingName, Verdict};
use std::path::Path;

/// What is wrong with `items` and `missions`, which are every item and
/// mission of a checkout, so that a `blocked_by` or a `mission` can be
/// checked against them. Reading already refused what does not parse, such
/// as an unknown status.
pub fn validate(items: &[ItemDir], missions: &[MissionDir]) -> Vec<Problem> {
    let mut problems = Vec::new();
    for dir in missions {
        let mission = &dir.mission;
        id_problem(&dir.path, &mission.id, &mut problems);
        if section(&mission.body, 2, "Outcome").is_none() {
            let message = "the mission has no `## Outcome` section";
            problems.push(Problem::new(&dir.path, "Outcome", message));
        }
    }
    for dir in items {
        validate_item(dir, items, missions, &mut problems);
    }
    problems
}

/// A problem with `id` when it is no ID or not the name of the directory
/// its record `path` is in.
fn id_problem(path: &Path, id: &str, problems: &mut Vec<Problem>) {
    let dir_name = path
        .parent()
        .and_then(|p| p.file_name())
        .unwrap_or_default();
    if let Err(message) = ids::validate_id(id) {
        problems.push(Problem::new(path, "id", message));
    } else if dir_name != id {
        let message = format!("`{id}` is not its directory's name");
        problems.push(Problem::new(path, "id", message));
    }
}

fn validate_item(
    dir: &ItemDir,
    items: &[ItemDir],
    missions: &[MissionDir],
    problems: &mut Vec<Problem>,
) {
    let item = &dir.item;
    id_problem(&dir.path, &item.id, problems);
    let mut problem =
        |field: &str, message: String| problems.push(Problem::new(&dir.path, field, message));
    if let Some(mission) = &item.mission {
        if !missions.iter().any(|m| m.mission.id == *mission) {
            problem("mission", format!("`{mission}` names no mission"));
        }
    }
    for blocker in &item.blocked_by {
        if !items.iter().any(|other| other.item.id == *blocker) {
            problem("blocked_by", format!("`{blocker}` names no item"));
        }
    }
    if let Some(cause) = &item.caused_by {
        if ids::validate_id(cause).is_err() && !is_commit(cause) {
            problem(
                "caused_by",
                format!("`{cause}` is neither an item ID nor a commit hash"),
            );
        }
    }
    for heading in ["Goal", "Done when"] {
        if section(&item.body, 2, heading).is_none() {
            problem(heading, format!("the item has no `## {heading}` section"));
        }
    }

    let mut findings: Vec<Finding> = Vec::new();
    // A review whose findings do not read is reported once; names that
    // point into it are not reported again.
    let mut unread = Vec::new();
    for review in &dir.reviews {
        if review.review.round != review.name.round {
            let message = format!("round {} is not the file name's round", review.review.round);
            problems.push(Problem::new(&review.path, "round", message));
        }
        match review.findings() {
            Ok(found) => findings.extend(found),
            Err(problem) => {
                problems.push(problem);
                unread.push(&review.name);
            }
        }
    }
    let decisions = dir.decisions().unwrap_or_else(|p| {
        problems.push(p);
        Vec::new()
    });
    let responses = dir.responses().unwrap_or_else(|p| {
        problems.push(p);
        Vec::new()
    });
    let named = |name: &FindingName| {
        findings.iter().any(|f| f.name == *name) || unread.contains(&&name.review)
    };

    for review in &dir.reviews {
        let own = findings.iter().filter(|f| f.name.review == review.name);
        for finding in own {
            let field = format!("## Finding {}: {}", finding.name.number, finding.title);
            if let Some(was) = &finding.was {
                if !named(was) || was.review.round >= review.name.round {
                    let message = format!("`Was: {was}` names no finding of an earlier round");
                    problems.push(Problem::new(&review.path, field, message));
                }
            }
            if review.review.verdict == Some(Verdict::Approve)
                && is_open(finding, &findings, &decisions)
            {
                let message = format!("approve, while fix-now finding {} is open", finding.name);
                problems.push(Problem::new(&review.path, "verdict", message));
            }
        }
    }
    for decision in &decisions {
        if let Some(name) = decision.finding.as_ref().filter(|name| !named(name)) {
            let field = format!("Decisions: {}", decision.question);
            problems.push(Problem::new(
                &dir.path,
                field,
                format!("`Finding: {name}` names no finding of this item's reviews"),
            ));
        }
    }
    for response in &responses {
        if !named(response.finding()) {
            let message = format!(
                "`{}` names no finding of this item's reviews",
                response.finding()
            );
            problems.push(Problem::new(&dir.path, "Responses", message));
        }
    }
}

/// An abbreviated or full commit hash, SHA-1 or SHA-256.
fn is_commit(text: &str) -> bool {
    (7..=64).contains(&text.len())
        && text
            .chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}
