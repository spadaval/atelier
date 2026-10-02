use chrono::{DateTime, Utc};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

use atelier_app::work_classification::{ClassifiedWork, WorkClassification};
use atelier_core::{Issue, IssuePriority};

pub(crate) fn ordered_work_indices(rows: &[(Issue, ClassifiedWork)]) -> Vec<usize> {
    let by_id = rows
        .iter()
        .enumerate()
        .map(|(index, row)| (row.0.id.as_str(), index))
        .collect::<BTreeMap<_, _>>();
    let mut indegree = vec![0usize; rows.len()];
    let mut dependents = vec![Vec::new(); rows.len()];
    for (blocked, row) in rows.iter().enumerate() {
        let blockers = row
            .1
            .open_blockers
            .iter()
            .filter_map(|id| by_id.get(id.as_str()).copied())
            .collect::<BTreeSet<_>>();
        indegree[blocked] = blockers.len();
        for blocker in blockers {
            dependents[blocker].push(blocked);
        }
    }
    let mut available = (0..rows.len())
        .filter(|index| indegree[*index] == 0)
        .map(|index| (display_key(&rows[index]), index))
        .collect::<BTreeSet<_>>();
    let mut ordered = Vec::with_capacity(rows.len());
    while let Some((_, index)) = available.pop_first() {
        ordered.push(index);
        for dependent in &dependents[index] {
            indegree[*dependent] -= 1;
            if indegree[*dependent] == 0 {
                available.insert((display_key(&rows[*dependent]), *dependent));
            }
        }
    }
    // Cyclic rows and their dependents retain the same deterministic fallback.
    let mut remaining = (0..rows.len())
        .filter(|index| indegree[*index] > 0)
        .collect::<Vec<_>>();
    remaining.sort_by_key(|index| display_key(&rows[*index]));
    ordered.extend(remaining);
    ordered
}

fn display_key(row: &(Issue, ClassifiedWork)) -> (u8, u8, Reverse<DateTime<Utc>>, &str) {
    (
        state_rank(row.1.classification),
        priority_rank(&row.0.priority),
        Reverse(row.0.updated_at),
        row.0.id.as_str(),
    )
}

fn state_rank(state: WorkClassification) -> u8 {
    match state {
        WorkClassification::Ready => 0,
        WorkClassification::Active => 1,
        WorkClassification::Blocked => 2,
        WorkClassification::Done => 3,
        WorkClassification::Backlog => 4,
    }
}

pub(crate) fn priority_rank(priority: &str) -> u8 {
    IssuePriority::from_label(priority)
        .map(|priority| priority.sort_rank())
        .unwrap_or(4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn row(id: &str, category: &str, priority: &str, blockers: &[&str]) -> (Issue, ClassifiedWork) {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        (
            Issue {
                id: id.to_string(),
                title: id.to_string(),
                description: None,
                status: category.to_string(),
                issue_type: "task".to_string(),
                priority: priority.to_string(),
                strategy: None,
                workflow_branch: None,
                parent_id: None,
                created_at: now,
                updated_at: now,
                closed_at: None,
            },
            ClassifiedWork {
                status_category: Some(category.to_string()),
                classification: match category {
                    "done" => WorkClassification::Done,
                    _ if !blockers.is_empty() => WorkClassification::Blocked,
                    "todo" => WorkClassification::Ready,
                    "active" => WorkClassification::Active,
                    _ => WorkClassification::Backlog,
                },
                open_blockers: blockers.iter().map(|id| id.to_string()).collect(),
            },
        )
    }

    fn ordered_ids(rows: &[(Issue, ClassifiedWork)]) -> Vec<String> {
        ordered_work_indices(rows)
            .into_iter()
            .map(|index| rows[index].0.id.clone())
            .collect()
    }

    #[test]
    fn orders_simple_visible_blocker_chain() {
        let rows = vec![
            row("atelier-c", "todo", "high", &["atelier-b"]),
            row("atelier-b", "todo", "high", &["atelier-a"]),
            row("atelier-a", "todo", "high", &[]),
        ];

        assert_eq!(
            ordered_ids(&rows),
            vec!["atelier-a", "atelier-b", "atelier-c"]
        );
    }

    #[test]
    fn orders_diamond_dependencies_before_join() {
        let rows = vec![
            row("atelier-d", "todo", "high", &["atelier-b", "atelier-c"]),
            row("atelier-c", "todo", "medium", &["atelier-a"]),
            row("atelier-b", "todo", "high", &["atelier-a"]),
            row("atelier-a", "todo", "low", &[]),
        ];

        assert_eq!(
            ordered_ids(&rows),
            vec!["atelier-a", "atelier-b", "atelier-c", "atelier-d"]
        );
    }

    #[test]
    fn hidden_blockers_do_not_insert_phantom_rows() {
        let rows = vec![
            row("atelier-b", "todo", "high", &["atelier-hidden"]),
            row("atelier-a", "todo", "medium", &[]),
        ];

        assert_eq!(ordered_ids(&rows), vec!["atelier-a", "atelier-b"]);
        assert_eq!(rows[0].1.classification, WorkClassification::Blocked);
    }

    #[test]
    fn done_blockers_are_ignored_by_open_blocker_input() {
        let rows = vec![
            row("atelier-b", "todo", "high", &[]),
            row("atelier-a", "done", "critical", &[]),
        ];

        assert_eq!(ordered_ids(&rows), vec!["atelier-b", "atelier-a"]);
    }

    #[test]
    fn ties_use_priority_update_time_and_id() {
        let mut low = row("atelier-c", "todo", "low", &[]);
        low.0.updated_at = Utc.with_ymd_and_hms(2026, 1, 3, 0, 0, 0).unwrap();
        let mut newer = row("atelier-b", "todo", "high", &[]);
        newer.0.updated_at = Utc.with_ymd_and_hms(2026, 1, 2, 0, 0, 0).unwrap();
        let older = row("atelier-a", "todo", "high", &[]);

        assert_eq!(
            ordered_ids(&[low, older, newer]),
            vec!["atelier-b", "atelier-a", "atelier-c"]
        );
    }

    #[test]
    fn cycles_are_deterministic_and_preserve_all_rows() {
        let rows = vec![
            row("atelier-b", "todo", "medium", &["atelier-a"]),
            row("atelier-a", "todo", "high", &["atelier-b"]),
            row("atelier-c", "todo", "low", &[]),
        ];

        assert_eq!(
            ordered_ids(&rows),
            vec!["atelier-c", "atelier-a", "atelier-b"]
        );
    }
    #[test]
    fn duplicate_edges_and_cycle_tail_preserve_every_row_once() {
        let rows = vec![
            row("tail", "todo", "critical", &["a"]),
            row("b", "todo", "medium", &["a", "a"]),
            row("a", "todo", "high", &["b"]),
            row("root", "todo", "low", &[]),
            row("child", "todo", "high", &["root", "root"]),
        ];
        assert_eq!(ordered_ids(&rows), vec!["root", "child", "tail", "a", "b"]);
    }
}
