//! Decisions across a scope (atelier-fip8): decision notes made inside work
//! items, and decision records with the ADR they produced and the decisions
//! that superseded them.

use anyhow::Result;
use atelier_core::{DecisionNote, Issue};

use crate::objective_graph::{mission_issue_ids, mission_objective_kind};
use crate::record_index::RecordIndex;
use crate::workflow_policy::WorkflowPolicy;

/// The link role a newer decision record uses to replace an older one.
pub const SUPERSEDES: &str = "supersedes";

/// One decision note, with the item it was made in.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoteDecision {
    pub issue_id: String,
    pub decision: DecisionNote,
}

/// A decision note `check` refuses, with its defect. `number` counts the
/// item's decision notes from 1, as `check` names them.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MalformedDecision {
    pub issue_id: String,
    pub number: usize,
    pub date: String,
    pub defect: String,
}

/// A decision record. It is superseded once a decided record that
/// `supersedes` it exists.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordDecision {
    pub issue: Issue,
    /// Closed with its Decision, and not retired by its own status.
    pub decided: bool,
    pub superseded_by: Vec<String>,
    pub supersedes: Vec<String>,
}

impl RecordDecision {
    /// Decided, and not replaced by a decided record. An open decision
    /// record is a proposal and changes nothing until it is decided.
    pub fn is_current(&self) -> bool {
        self.decided && self.superseded_by.is_empty()
    }

    /// Replaced by a decided record or retired by its own status.
    pub fn is_superseded(&self) -> bool {
        !self.superseded_by.is_empty() || self.issue.status == "superseded"
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Decisions {
    pub notes: Vec<NoteDecision>,
    pub records: Vec<RecordDecision>,
    pub malformed: Vec<MalformedDecision>,
}

impl Decisions {
    /// One-way notes, then reversible ones, each in record order.
    pub fn notes_one_way_first(&self) -> impl Iterator<Item = &NoteDecision> {
        let one_way = self.notes.iter().filter(|note| note.decision.one_way);
        one_way.chain(self.notes.iter().filter(|note| !note.decision.one_way))
    }
}

/// The decisions in a mission's scope: the mission record, the items that
/// advance it, and their descendants.
pub fn mission_decisions(
    index: &RecordIndex,
    policy: Option<&WorkflowPolicy>,
    mission_id: &str,
) -> Result<Decisions> {
    mission_objective_kind(index, mission_id)?;
    let mut ids = vec![mission_id.to_string()];
    ids.extend(mission_issue_ids(index, mission_id)?);
    Ok(decisions_in(index, policy, &ids))
}

/// The current decisions that bear on one item (atelier-1mk4): the decision
/// notes on the item, its parent and its missions, and the current decision
/// records in its missions' scope or among those records themselves.
pub fn item_decisions(
    index: &RecordIndex,
    policy: Option<&WorkflowPolicy>,
    issue_id: &str,
) -> Result<Decisions> {
    let issue = index.require_issue(issue_id)?;
    let missions = if issue.issue_type == "mission" {
        vec![issue.id.clone()]
    } else {
        let mut missions = Vec::new();
        for mission in index.issues().filter(|issue| issue.issue_type == "mission") {
            if mission_issue_ids(index, &mission.id)?.contains(issue_id) {
                missions.push(mission.id.clone());
            }
        }
        missions.sort();
        missions
    };
    let mut ids = vec![issue.id.clone()];
    ids.extend(issue.parent_id.clone());
    ids.extend(missions.iter().cloned());
    let mut decisions = decisions_in(index, policy, &ids);
    for mission in &missions {
        for id in mission_issue_ids(index, mission)? {
            let Some(record) = index.issue_record(&id) else {
                continue;
            };
            if record.issue.issue_type == "decision"
                && !decisions.records.iter().any(|known| known.issue.id == id)
            {
                decisions
                    .records
                    .push(record_decision(index, policy, &record.issue));
            }
        }
    }
    decisions.records.retain(RecordDecision::is_current);
    Ok(decisions)
}

/// The decisions recorded on the given issues, in the order given.
pub fn decisions_in(
    index: &RecordIndex,
    policy: Option<&WorkflowPolicy>,
    ids: &[String],
) -> Decisions {
    let mut decisions = Decisions::default();
    for id in ids {
        let Some(record) = index.issue_record(id) else {
            continue;
        };
        let notes = record.sections.note_entries();
        let parsed = notes
            .iter()
            .filter_map(|note| Some((note, DecisionNote::from_note(note)?)));
        for (number, (note, decision)) in parsed.enumerate() {
            match decision {
                Ok(decision) => decisions.notes.push(NoteDecision {
                    issue_id: id.clone(),
                    decision,
                }),
                Err(defect) => decisions.malformed.push(MalformedDecision {
                    issue_id: id.clone(),
                    number: number + 1,
                    date: note.date.clone(),
                    defect,
                }),
            }
        }
        if record.issue.issue_type == "decision" {
            decisions
                .records
                .push(record_decision(index, policy, &record.issue));
        }
    }
    decisions
}

/// A decision record with the decided records that supersede it and the
/// records it supersedes.
pub fn record_decision(
    index: &RecordIndex,
    policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> RecordDecision {
    let mut superseded_by = Vec::new();
    let mut supersedes = Vec::new();
    for relation in index.get_typed_relations(&issue.id) {
        if relation.relation_type != SUPERSEDES {
            continue;
        }
        // The record holding the link is the newer decision.
        if relation.issue_id_2 == issue.id {
            let newer = index.issue_record(&relation.issue_id_1);
            if newer.is_some_and(|newer| decided(policy, &newer.issue)) {
                superseded_by.push(relation.issue_id_1);
            }
        } else {
            supersedes.push(relation.issue_id_2);
        }
    }
    superseded_by.sort();
    supersedes.sort();
    RecordDecision {
        issue: issue.clone(),
        decided: decided(policy, issue),
        superseded_by,
        supersedes,
    }
}

/// A decision record whose Decision is in force: closed, and not retired.
fn decided(policy: Option<&WorkflowPolicy>, issue: &Issue) -> bool {
    issue.issue_type == "decision"
        && issue.status != "superseded"
        && policy.and_then(|policy| policy.status_category(&issue.status)) == Some("done")
}
