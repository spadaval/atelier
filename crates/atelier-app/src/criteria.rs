//! Mission success criteria and their proof (atelier-et6q, atelier-ho7o).
//! One parser reads a mission's criteria; one lookup finds the receipts that
//! judge each one. `work mission` and `debrief` share both.

use std::path::Path;

use anyhow::Result;
use atelier_core::{
    parse_criteria, strategy_criteria_section, CriteriaParse, Criterion, EvidenceRecord,
    EvidenceResult, IssueSectionName,
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

/// What is wrong with the criteria a receipt names: each slug that no
/// mission of its target defines, or criteria named with no mission at all.
pub fn receipt_criteria_problems(
    index: &RecordIndex,
    repo_root: &Path,
    target_id: &str,
    slugs: &[&str],
) -> Result<Vec<String>> {
    if slugs.is_empty() {
        return Ok(Vec::new());
    }
    let missions = missions_for(index, target_id)?;
    if missions.is_empty() {
        return Ok(vec![format!(
            "names criteria {}, but its target {target_id} is in no mission, so no criteria are defined for it",
            slugs.join(", ")
        )]);
    }
    let defined = missions
        .iter()
        .map(|mission| mission_criteria(index, repo_root, mission))
        .collect::<Result<Vec<_>>>()?;
    Ok(slugs
        .iter()
        .filter(|slug| {
            !defined
                .iter()
                .any(|criteria| criteria.parse.get(slug).is_some())
        })
        .map(|slug| {
            format!(
                "names criterion {slug}, which mission {} does not define",
                missions.join(" or ")
            )
        })
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
    /// Commits since the pin that change files outside the tracker; `None`
    /// when the pin is not in HEAD's history.
    pub code_commits_since: Option<usize>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CriterionMark {
    /// The newest receipt in HEAD's history passes.
    Proven,
    /// The newest receipt in HEAD's history fails or is blocked.
    Failing,
    /// No receipt in HEAD's history, or only a not-applicable one.
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
        self.receipts
            .iter()
            .find(|receipt| receipt.code_commits_since.is_some())
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

/// Every criterion of the mission with the receipts in its scope that name
/// it. The newest receipt whose pin is in HEAD's history decides the mark;
/// item-level freshness, which any later code commit breaks, is reported as
/// a count of code commits since the pin rather than applied.
pub fn mission_proof(
    index: &RecordIndex,
    read: &mut ReadContext<'_>,
    mission_id: &str,
) -> Result<MissionProof> {
    let repo_root = read.repo;
    let criteria = mission_criteria(index, repo_root, mission_id)?;
    let mut scope = vec![mission_id.to_string()];
    scope.extend(mission_issue_ids(index, mission_id)?);
    let mut receipts: Vec<(&EvidenceRecord, String)> = Vec::new();
    for id in &scope {
        for evidence_id in validating_evidence(index, id)? {
            let record = index.require_evidence(&evidence_id)?;
            if !record.data.criteria.is_empty()
                && !receipts
                    .iter()
                    .any(|(seen, _)| seen.header.id == record.header.id)
            {
                receipts.push((record, id.clone()));
            }
        }
    }
    receipts.sort_by(|(a, _), (b, _)| {
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
        for (record, target_id) in &receipts {
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
            naming.push(CriterionReceipt {
                evidence_id: record.header.id.clone(),
                target_id: target_id.clone(),
                result: judged.result,
                commit: record.data.commit.clone(),
                captured_at: record.data.captured_at,
                code_commits_since: match (&head, in_history) {
                    (Some(head), true) => {
                        Some(code_commits_between(repo_root, &record.data.commit, head)?)
                    }
                    _ => None,
                },
            });
        }
        let mark = match naming
            .iter()
            .find(|receipt| receipt.code_commits_since.is_some())
            .map(|receipt| receipt.result)
        {
            Some(EvidenceResult::Pass) => CriterionMark::Proven,
            Some(EvidenceResult::Fail | EvidenceResult::Blocked) => CriterionMark::Failing,
            Some(EvidenceResult::NotApplicable) | None => CriterionMark::Unproven,
        };
        proofs.push(CriterionProof {
            criterion: criterion.clone(),
            mark,
            receipts: naming,
        });
    }
    let mut orphaned = Vec::new();
    for (record, _) in &receipts {
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

/// Evidence attached to the issue with the `validates` role.
fn validating_evidence(index: &RecordIndex, issue_id: &str) -> Result<Vec<String>> {
    Ok(index
        .list_record_links("issue", issue_id)?
        .into_iter()
        .filter(|link| link.relation_type == "validates")
        .filter_map(|link| {
            if link.source_kind == "evidence" {
                Some(link.source_id)
            } else if link.target_kind == "evidence" {
                Some(link.target_id)
            } else {
                None
            }
        })
        .collect())
}

fn code_commits_between(repo: &Path, from: &str, to: &str) -> Result<usize> {
    let range = format!("{from}..{to}");
    let count = git_output(
        repo,
        &[
            "rev-list",
            "--count",
            &range,
            "--",
            ".",
            ":(exclude).atelier",
        ],
    )?;
    Ok(String::from_utf8(count)?.trim().parse().unwrap_or(0))
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
