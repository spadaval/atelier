//! What waits on the human in one repository (atelier-uma8), as the site's
//! inbox lists it: the work `atelier status` counts under Needs a human, each
//! with the missions it belongs to, and the draft missions planning review
//! has reached.

use std::collections::BTreeSet;

use anyhow::Result;
use atelier_core::ReviewEvent;

use crate::board::BoardRow;
use crate::mission_overview::mission_overview;
use crate::record_index::RecordIndex;
use crate::work_classification::status_view;
use crate::workflow_policy::WorkflowPolicy;

/// Mission statuses that retire a mission without doing it; an inbox entry
/// never links one, since its brief and debrief are not what the entry asks
/// for (finding atelier-ei8w).
const RETIRED: &[&str] = &["superseded", "declined"];

/// One needs-human item and the missions it belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboxItem {
    pub row: BoardRow,
    /// Open or done missions whose scope holds the item or that it links
    /// under any role, by ID.
    pub missions: Vec<String>,
}

/// What waits on the human in one repository's records.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Inbox {
    /// In the order `atelier status` lists its needs-human work.
    pub items: Vec<InboxItem>,
    /// Missions in their workflow's initial status that an approval
    /// targets, in `atelier work missions` order.
    pub drafts: Vec<String>,
}

impl Inbox {
    pub fn len(&self) -> usize {
        self.items.len() + self.drafts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub fn inbox(index: &RecordIndex, policy: &WorkflowPolicy) -> Result<Inbox> {
    let overview = mission_overview(index, Some(policy), true)?;
    let is_mission = |id: &str| {
        index.issue_record(id).is_some_and(|record| {
            record.issue.issue_type == "mission" && !RETIRED.contains(&record.issue.status.as_str())
        })
    };
    let items = status_view(index, Some(policy))?
        .work
        .human
        .iter()
        .map(|row| {
            let mut missions = overview
                .missions
                .iter()
                .filter(|mission| mission.scope_ids.contains(&row.id))
                .map(|mission| mission.issue.id.clone())
                .collect::<BTreeSet<_>>();
            // blocked_by and blocks links live in the blocker graph, not
            // among the typed relations (atelier-jx4p).
            missions.extend(
                index
                    .get_typed_relations(&row.id)
                    .into_iter()
                    .flat_map(|relation| [relation.issue_id_1, relation.issue_id_2])
                    .chain(index.get_blockers(&row.id))
                    .chain(index.get_blocking(&row.id))
                    .filter(|id| *id != row.id),
            );
            InboxItem {
                row: BoardRow::from_work_row(index, row),
                missions: missions.into_iter().filter(|id| is_mission(id)).collect(),
            }
        })
        .collect();
    let drafts = overview
        .missions
        .iter()
        .map(|mission| &mission.issue)
        .filter(|mission| {
            policy
                .workflow_for_issue_type(&mission.issue_type)
                .is_ok_and(|workflow| workflow.initial_status == mission.status)
        })
        .filter(|mission| {
            index.reviews().any(|review| {
                review.target == mission.id && matches!(review.event, ReviewEvent::Approval)
            })
        })
        .map(|mission| mission.id.clone())
        .collect();
    Ok(Inbox { items, drafts })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_records::Records;

    #[test]
    fn an_item_links_its_scope_and_direct_missions_but_no_retired_one() {
        let mut records = Records::default();
        records.issue("m-open", "mission", "todo");
        records.issue("m-done", "mission", "done");
        records.issue("m-gone", "mission", "superseded");
        for (id, mission, role) in [
            ("t-scope", "m-open", "advances"),
            ("t-after", "m-done", "blocked_by"),
            ("t-old", "m-gone", "related"),
        ] {
            records.issue(id, "task", "todo").labels = vec!["needs-human".to_owned()];
            records.relate(id, mission, role);
        }
        let index = records.index();
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(".atelier")).unwrap();
        std::fs::write(
            dir.path().join(".atelier/workflow.yaml"),
            crate::workflow_policy::starter_policy_yaml("main"),
        )
        .unwrap();
        let policy = crate::workflow_policy::load(dir.path()).unwrap();
        let inbox = inbox(&index, &policy).unwrap();
        let links = inbox
            .items
            .iter()
            .map(|item| (item.row.id.as_str(), item.missions.clone()))
            .collect::<Vec<_>>();
        assert_eq!(
            links,
            [
                ("t-after", vec!["m-done".to_owned()]),
                ("t-old", vec![]),
                ("t-scope", vec!["m-open".to_owned()]),
            ]
        );
    }
}
