//! Complete mission summaries over directed membership and shared work facts.

use anyhow::Result;
use atelier_core::{Issue, IssuePriority};
use std::collections::{BTreeMap, BTreeSet};

use crate::issue_read::ObjectiveBucketTotals;
use crate::record_index::RecordIndex;
use crate::work_classification::{classify_issue, WorkClassification};
use crate::workflow_policy::WorkflowPolicy;

#[derive(Debug)]
pub struct MissionOverviewMission {
    pub issue: Issue,
    pub status_category: String,
    pub scope_ids: BTreeSet<String>,
    pub totals: ObjectiveBucketTotals,
    pub open_blockers: BTreeSet<String>,
}

#[derive(Debug, Default)]
pub struct MissionOverview {
    pub missions: Vec<MissionOverviewMission>,
    pub unassigned_nonterminal: usize,
    pub linked_only_to_hidden_done_missions: usize,
}

/// Select all matching missions before the CLI applies its display limit.
/// Membership follows outgoing advances links, including each root's children.
pub fn mission_overview(
    index: &RecordIndex,
    policy: Option<&WorkflowPolicy>,
    include_done: bool,
) -> Result<MissionOverview> {
    let classified = index
        .issues()
        .map(|issue| Ok((issue.id.clone(), classify_issue(index, policy, issue)?)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let mut overview = MissionOverview::default();
    let mut all_scope = BTreeSet::new();
    let mut visible_scope = BTreeSet::new();
    for mission in index.issues().filter(|issue| issue.issue_type == "mission") {
        let mut scope_ids = BTreeSet::new();
        for link in index.record_links().iter().filter(|link| {
            link.source_kind == "issue"
                && link.source_id == mission.id
                && link.target_kind == "issue"
                && link.relation_type == "advances"
                && index
                    .issue_record(&link.target_id)
                    .is_some_and(|record| record.issue.issue_type != "mission")
        }) {
            scope_ids.insert(link.target_id.clone());
            scope_ids.extend(crate::objective_graph::issue_descendant_ids(
                index,
                &link.target_id,
            )?);
        }
        all_scope.extend(scope_ids.iter().cloned());
        if !include_done && classified[&mission.id].classification == WorkClassification::Done {
            continue;
        }
        visible_scope.extend(scope_ids.iter().cloned());
        let mut totals = ObjectiveBucketTotals::default();
        let mut open_blockers = classified[&mission.id]
            .open_blockers
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        for id in &scope_ids {
            let work = &classified[id];
            open_blockers.extend(work.open_blockers.iter().cloned());
            match work.classification {
                WorkClassification::Active => totals.active += 1,
                WorkClassification::Ready => totals.ready += 1,
                WorkClassification::NeedsHuman => totals.human += 1,
                WorkClassification::Blocked => totals.blocked += 1,
                WorkClassification::Done => totals.done += 1,
                WorkClassification::Backlog => totals.backlog += 1,
            }
        }
        overview.missions.push(MissionOverviewMission {
            issue: mission.clone(),
            status_category: classified[&mission.id]
                .status_category
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            scope_ids,
            totals,
            open_blockers,
        });
    }
    overview.missions.sort_by(|left, right| {
        category_rank(&left.status_category)
            .cmp(&category_rank(&right.status_category))
            .then_with(|| {
                priority_rank(&left.issue.priority).cmp(&priority_rank(&right.issue.priority))
            })
            .then_with(|| left.issue.id.cmp(&right.issue.id))
    });
    for issue in index.issues().filter(|issue| issue.issue_type != "mission") {
        if classified[&issue.id].classification == WorkClassification::Done {
            continue;
        }
        if !all_scope.contains(&issue.id) {
            overview.unassigned_nonterminal += 1;
        } else if !visible_scope.contains(&issue.id) {
            overview.linked_only_to_hidden_done_missions += 1;
        }
    }
    Ok(overview)
}

fn category_rank(category: &str) -> u8 {
    match category {
        "active" => 0,
        "blocked" => 1,
        "todo" => 2,
        "done" => 3,
        _ => 4,
    }
}

fn priority_rank(priority: &str) -> u8 {
    IssuePriority::from_label(priority)
        .or_else(|_| IssuePriority::from_canonical_token(priority))
        .map(IssuePriority::sort_rank)
        .unwrap_or(4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_records::Records;

    fn policy() -> WorkflowPolicy {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(".atelier")).unwrap();
        std::fs::write(
            dir.path().join(".atelier/workflow.yaml"),
            crate::workflow_policy::STARTER_POLICY_YAML,
        )
        .unwrap();
        crate::workflow_policy::load(dir.path()).unwrap()
    }

    #[test]
    fn directed_membership_deduplicates_roots_and_preserves_shared_scope_and_blockers() {
        let mut records = Records::default();
        for (id, kind, status) in [
            ("mission-a", "mission", "in_progress"),
            ("mission-b", "mission", "draft"),
            ("epic", "feature", "todo"),
            ("child", "task", "todo"),
            ("direct", "task", "todo"),
            ("incoming", "task", "todo"),
            ("parent-only", "task", "todo"),
            ("blocker", "task", "todo"),
        ] {
            records.issue(id, kind, status);
        }
        records.parent("child", "epic");
        records.relate("mission-a", "epic", "advances");
        records.relate("mission-a", "child", "advances");
        records.relate("mission-a", "direct", "advances");
        records.relate("mission-b", "epic", "advances");
        records.relate("mission-b", "incoming", "advances");
        records.relate("mission-a", "mission-b", "advances");
        records.relate("incoming", "mission-a", "advances");
        records.block("epic", "blocker");
        records.block("mission-a", "blocker");
        let index = records.index();
        let overview = mission_overview(&index, Some(&policy()), false).unwrap();
        let first = &overview.missions[0];
        assert_eq!(first.issue.id, "mission-a");
        assert_eq!(
            first.scope_ids,
            ["child", "direct", "epic"].map(String::from).into()
        );
        // The child inherits its parent's open blocker.
        assert_eq!(first.totals.ready, 1);
        assert_eq!(first.totals.blocked, 2);
        assert_eq!(first.open_blockers, ["blocker"].map(String::from).into());
        assert_eq!(
            overview.missions[1].scope_ids,
            ["child", "epic", "incoming"].map(String::from).into()
        );
        assert_eq!(overview.unassigned_nonterminal, 2);
    }

    #[test]
    fn terminal_missions_hide_scope_only_until_all_is_requested() {
        let mut records = Records::default();
        records.issue("mission-active", "mission", "in_progress");
        records.issue("mission-done", "mission", "done");
        records.issue("mission-retired", "mission", "superseded");
        records.issue("shared", "task", "todo");
        records.issue("hidden", "task", "todo");
        records.relate("mission-active", "shared", "advances");
        records.relate("mission-done", "shared", "advances");
        records.relate("mission-done", "hidden", "advances");
        let index = records.index();
        let policy = policy();
        let visible = mission_overview(&index, Some(&policy), false).unwrap();
        assert_eq!(visible.missions.len(), 1);
        assert_eq!(visible.linked_only_to_hidden_done_missions, 1);
        let all = mission_overview(&index, Some(&policy), true).unwrap();
        assert_eq!(all.missions.len(), 3);
        assert_eq!(all.linked_only_to_hidden_done_missions, 0);
        assert_eq!(all.unassigned_nonterminal, 0);
    }

    #[test]
    fn complete_summaries_keep_counts_beyond_display_limits_and_stable_status_priority_order() {
        let mut records = Records::default();
        for (id, status, priority) in [
            ("active-high-z", "in_progress", "high"),
            ("active-high-a", "in_progress", "high"),
            ("active-low", "in_progress", "low"),
            ("blocked", "blocked", "critical"),
            ("draft", "draft", "critical"),
            ("done", "done", "critical"),
        ] {
            records.issue(id, "mission", status).issue.priority = priority.to_string();
        }
        for number in 0..24 {
            let id = format!("scope-{number:02}");
            records.issue(&id, "task", "todo");
            records.relate("active-high-a", &id, "advances");
        }
        let overview = mission_overview(&records.index(), Some(&policy()), true).unwrap();
        assert_eq!(
            overview
                .missions
                .iter()
                .map(|row| row.issue.id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "active-high-a",
                "active-high-z",
                "active-low",
                "blocked",
                "draft",
                "done"
            ]
        );
        assert_eq!(overview.missions[0].scope_ids.len(), 24);
        assert_eq!(overview.missions[0].totals.ready, 24);
    }
}
