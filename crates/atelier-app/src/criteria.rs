//! Mission success criteria and their proof (atelier-et6q, atelier-ho7o).
//! One parser reads a mission's criteria; one lookup finds the receipts that
//! judge each one. `work mission` and `debrief` share both.

use std::path::Path;

use anyhow::{Context, Result};
use atelier_core::{
    parse_criteria, strategy_criteria_section, CriteriaParse, Criterion, EvidenceResult,
    IssueSectionName,
};

use crate::git_freshness::{git_output, ReadContext};
use crate::objective_graph::{mission_issue_ids, mission_objective_kind};
use crate::record_index::RecordIndex;

/// Where a mission's criteria are written.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CriteriaSource {
    /// The Success Criteria heading of the linked strategy file.
    Strategy(String),
    /// The mission record's own Success criteria section.
    Mission,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MissionCriteria {
    pub mission_id: String,
    pub source: CriteriaSource,
    pub parse: CriteriaParse,
}

/// Reads a mission's criteria from its strategy, or from its own Success
/// criteria section when it links none. An unreadable strategy, or one with
/// no Success Criteria heading, is a problem, not an error.
pub fn mission_criteria(
    index: &RecordIndex,
    repo_root: &Path,
    mission_id: &str,
) -> Result<MissionCriteria> {
    mission_objective_kind(index, mission_id)?;
    let record = index
        .issue_record(mission_id)
        .ok_or_else(|| index.not_found(crate::record_ref::RecordKind::Issue, mission_id))?;
    if let Some(strategy) = &record.issue.strategy {
        let path = strategy.path.clone();
        let parse = match std::fs::read_to_string(repo_root.join(&path)) {
            Ok(text) => match strategy_criteria_section(&text) {
                Some(section) => parse_criteria(&section),
                None => problem(format!("strategy {path} has no Success Criteria heading")),
            },
            Err(error) => problem(format!("strategy {path} cannot be read: {error}")),
        };
        return Ok(MissionCriteria {
            mission_id: mission_id.to_string(),
            source: CriteriaSource::Strategy(path),
            parse,
        });
    }
    let section = record
        .sections
        .section(IssueSectionName::SuccessCriteria)
        .unwrap_or_default();
    Ok(MissionCriteria {
        mission_id: mission_id.to_string(),
        source: CriteriaSource::Mission,
        parse: parse_criteria(section),
    })
}

fn problem(message: String) -> CriteriaParse {
    CriteriaParse {
        criteria: Vec::new(),
        problems: vec![message],
    }
}

/// The missions whose criteria a receipt on `issue_id` may name: the issue
/// itself when it is a mission, else every mission whose scope holds it.
pub fn missions_for(index: &RecordIndex, issue_id: &str) -> Result<Vec<String>> {
    if index
        .issue_record(issue_id)
        .is_some_and(|record| record.issue.issue_type == "mission")
    {
        return Ok(vec![issue_id.to_string()]);
    }
    let mut missions = Vec::new();
    for mission in index.issues().filter(|issue| issue.issue_type == "mission") {
        if mission_issue_ids(index, &mission.id)?.contains(issue_id) {
            missions.push(mission.id.clone());
        }
    }
    missions.sort();
    Ok(missions)
}

/// The one mission whose criteria a receipt on `issue_id` names: the issue
/// itself when it is a mission, else the single mission whose scope holds
/// it. An item in several missions has none, so proof never counts toward
/// a mission it was not meant for; record such criteria on the mission.
pub fn criteria_mission(index: &RecordIndex, issue_id: &str) -> Result<Option<String>> {
    Ok(match missions_for(index, issue_id)?.as_slice() {
        [mission] => Some(mission.clone()),
        _ => None,
    })
}

/// The mission a new receipt's criteria belong to, or what is wrong with
/// them: each slug the target's single mission does not define, or criteria
/// named where no single mission owns them.
pub fn receipt_criteria_mission(
    index: &RecordIndex,
    repo_root: &Path,
    target_id: &str,
    slugs: &[&str],
) -> Result<std::result::Result<String, Vec<String>>> {
    let missions = missions_for(index, target_id)?;
    let mission = match missions.as_slice() {
        [] => {
            return Ok(Err(vec![format!(
                "names criteria {}, but its target {target_id} is in no mission, so no criteria are defined for it",
                slugs.join(", ")
            )]))
        }
        [mission] => mission.clone(),
        several => {
            return Ok(Err(vec![format!(
                "names criteria {}, but its target {target_id} is in missions {}; target the mission whose criteria it judges",
                slugs.join(", "),
                several.join(", ")
            )]))
        }
    };
    let problems = undefined_criteria(index, repo_root, &mission, slugs)?;
    Ok(if problems.is_empty() {
        Ok(mission)
    } else {
        Err(problems)
    })
}

/// Each slug `mission_id` does not define, or that it is not a mission.
pub fn undefined_criteria(
    index: &RecordIndex,
    repo_root: &Path,
    mission_id: &str,
    slugs: &[&str],
) -> Result<Vec<String>> {
    if !index
        .issue_record(mission_id)
        .is_some_and(|record| record.issue.issue_type == "mission")
    {
        return Ok(vec![format!(
            "names criteria of {mission_id}, which is not a mission"
        )]);
    }
    let criteria = mission_criteria(index, repo_root, mission_id)?;
    Ok(slugs
        .iter()
        .filter(|slug| criteria.parse.get(slug).is_none())
        .map(|slug| format!("names criterion {slug}, which mission {mission_id} does not define"))
        .collect())
}

/// One receipt's judgement of one criterion.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CriterionReceipt {
    pub evidence_id: String,
    pub target_id: String,
    pub result: EvidenceResult,
    pub commit: String,
    pub captured_at: chrono::DateTime<chrono::Utc>,
    /// Whether the pinned commit is in HEAD's history; only such receipts
    /// decide a mark.
    pub in_history: bool,
    /// Commits since the pin that change files outside the tracker, counted
    /// for the deciding receipt only.
    pub code_commits_since: Option<usize>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CriterionMark {
    /// The newest receipt in HEAD's history passes.
    Proven,
    /// The newest receipt in HEAD's history fails.
    Failing,
    /// No receipt in HEAD's history, or the newest is blocked or
    /// not-applicable.
    Unproven,
}

impl CriterionMark {
    pub fn word(self) -> &'static str {
        match self {
            Self::Proven => "proven",
            Self::Failing => "failing",
            Self::Unproven => "unproven",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CriterionProof {
    pub criterion: Criterion,
    pub mark: CriterionMark,
    /// The receipts naming the criterion, newest first.
    pub receipts: Vec<CriterionReceipt>,
}

impl CriterionProof {
    /// The receipt the mark comes from.
    pub fn deciding(&self) -> Option<&CriterionReceipt> {
        self.receipts.iter().find(|receipt| receipt.in_history)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MissionProof {
    pub criteria: MissionCriteria,
    pub proofs: Vec<CriterionProof>,
    /// Receipts in the mission's scope that name a criterion it does not
    /// define, as `(evidence id, slug)`.
    pub orphaned: Vec<(String, String)>,
}

/// Every criterion of the mission with the receipts recorded against it. The newest receipt whose pin is in HEAD's history decides the
/// mark. Item-level freshness, which any later code commit breaks, is
/// reported for the deciding receipt as a count of code commits since its
/// pin rather than applied.
pub fn mission_proof(
    index: &RecordIndex,
    read: &mut ReadContext<'_>,
    mission_id: &str,
) -> Result<MissionProof> {
    mission_proof_until(index, read, mission_id, None)
}

/// [`mission_proof`] with the deciding receipt's age counted up to `until`
/// rather than HEAD, as a debrief counts it up to the mission's close.
pub fn mission_proof_until(
    index: &RecordIndex,
    read: &mut ReadContext<'_>,
    mission_id: &str,
    until: Option<&str>,
) -> Result<MissionProof> {
    let repo_root = read.repo;
    let criteria = mission_criteria(index, repo_root, mission_id)?;
    // A receipt belongs to the mission it was recorded against, wherever its
    // target has moved since.
    let mut receipts = index
        .evidence_records()
        .into_iter()
        .filter(|record| record.data.criteria_mission.as_deref() == Some(mission_id))
        .collect::<Vec<_>>();
    receipts.sort_by(|a, b| {
        (b.data.captured_at, &b.header.id).cmp(&(a.data.captured_at, &a.header.id))
    });
    // Without a commit there is no history, and so no receipt to count.
    let head = git_output(repo_root, &["rev-parse", "HEAD"])
        .ok()
        .and_then(|head| String::from_utf8(head).ok())
        .map(|head| head.trim().to_string());
    let mut proofs = Vec::new();
    for criterion in &criteria.parse.criteria {
        let mut naming = Vec::new();
        let mut decided = false;
        for record in &receipts {
            let Some(judged) = record
                .data
                .criteria
                .iter()
                .find(|judged| judged.slug == criterion.slug)
            else {
                continue;
            };
            let in_history = match &head {
                Some(head) => read.ancestor(&record.data.commit, head)?,
                None => false,
            };
            // Only the deciding receipt's age is shown, so only it is counted.
            let end = until.or(head.as_deref());
            let code_commits_since = match (end, in_history && !decided) {
                (Some(end), true) if read.ancestor(&record.data.commit, end)? => {
                    Some(code_commits_between(read, &record.data.commit, end)?)
                }
                (Some(_), true) => Some(0),
                _ => None,
            };
            decided |= in_history;
            naming.push(CriterionReceipt {
                evidence_id: record.header.id.clone(),
                target_id: record
                    .data
                    .target
                    .as_ref()
                    .map(|target| target.id.clone())
                    .unwrap_or_default(),
                result: judged.result,
                commit: record.data.commit.clone(),
                captured_at: record.data.captured_at,
                in_history,
                code_commits_since,
            });
        }
        let mark = match naming
            .iter()
            .find(|receipt| receipt.in_history)
            .map(|receipt| receipt.result)
        {
            Some(EvidenceResult::Pass) => CriterionMark::Proven,
            Some(EvidenceResult::Fail) => CriterionMark::Failing,
            // A blocked check did not run, and not-applicable proves nothing.
            Some(EvidenceResult::Blocked | EvidenceResult::NotApplicable) | None => {
                CriterionMark::Unproven
            }
        };
        proofs.push(CriterionProof {
            criterion: criterion.clone(),
            mark,
            receipts: naming,
        });
    }
    let mut orphaned = Vec::new();
    for record in &receipts {
        for judged in &record.data.criteria {
            if criteria.parse.get(&judged.slug).is_none() {
                orphaned.push((record.header.id.clone(), judged.slug.clone()));
            }
        }
    }
    Ok(MissionProof {
        criteria,
        proofs,
        orphaned,
    })
}

fn code_commits_between(read: &mut ReadContext<'_>, from: &str, to: &str) -> Result<usize> {
    let range = format!("{from}..{to}");
    let count = read.history(&[
        "rev-list",
        "--count",
        &range,
        "--",
        ".",
        ":(exclude).atelier",
    ])?;
    count
        .trim()
        .parse()
        .with_context(|| format!("cannot count code commits in {range}: {count}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every strategy in this repository reads as criteria without a problem
    /// (atelier-ho7o): the parser fits the convention the strategies follow.
    #[test]
    fn this_repositorys_strategies_parse_without_problems() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/strategies");
        let mut checked = 0;
        for entry in std::fs::read_dir(&root).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|extension| extension != "md") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            let Some(section) = strategy_criteria_section(&text) else {
                continue;
            };
            let parsed = parse_criteria(&section);
            assert!(
                parsed.problems.is_empty() && !parsed.criteria.is_empty(),
                "{}: {:?}",
                path.display(),
                parsed.problems
            );
            checked += 1;
        }
        assert!(checked > 0, "no strategy had a Success Criteria heading");
    }
}
