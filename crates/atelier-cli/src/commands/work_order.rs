use chrono::{DateTime, Utc};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

use atelier_core::IssuePriority;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorkRowState {
    Ready,
    Blocked,
    Active,
    Review,
    Validation,
    Done,
    NotReady,
}

impl WorkRowState {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
            Self::Active => "active",
            Self::Review => "review",
            Self::Validation => "validation",
            Self::Done => "done",
            Self::NotReady => "not-ready",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WorkOrderRow {
    pub id: String,
    pub status_category: Option<String>,
    pub priority: String,
    pub updated_at: DateTime<Utc>,
    pub open_blockers: Vec<String>,
}

impl WorkOrderRow {
    pub(crate) fn state(&self) -> WorkRowState {
        row_state(self.status_category.as_deref(), self.open_blockers.len())
    }
}

pub(crate) fn row_state(category: Option<&str>, open_blocker_count: usize) -> WorkRowState {
    match category {
        Some("done") => WorkRowState::Done,
        _ if open_blocker_count > 0 => WorkRowState::Blocked,
        Some("todo") => WorkRowState::Ready,
        Some("active") => WorkRowState::Active,
        Some("review") => WorkRowState::Review,
        Some("validation") => WorkRowState::Validation,
        Some("blocked") => WorkRowState::Blocked,
        _ => WorkRowState::NotReady,
    }
}

pub(crate) fn ordered_work_indices(rows: &[WorkOrderRow]) -> Vec<usize> {
    let visible_ids = rows
        .iter()
        .map(|row| row.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut indegree = vec![0usize; rows.len()];
    let mut dependents_by_blocker = BTreeMap::<&str, Vec<usize>>::new();

    for (blocked_index, row) in rows.iter().enumerate() {
        let mut visible_blockers = row
            .open_blockers
            .iter()
            .map(String::as_str)
            .filter(|blocker_id| visible_ids.contains(blocker_id))
            .collect::<BTreeSet<_>>();
        for blocker_id in visible_blockers.iter() {
            indegree[blocked_index] += 1;
            dependents_by_blocker
                .entry(blocker_id)
                .or_default()
                .push(blocked_index);
        }
        visible_blockers.clear();
    }

    let mut available = (0..rows.len())
        .filter(|index| indegree[*index] == 0)
        .collect::<Vec<_>>();
    sort_indices_by_display_order(rows, &mut available);

    let mut ordered = Vec::with_capacity(rows.len());
    let mut emitted = vec![false; rows.len()];
    while let Some(index) = pop_front(&mut available) {
        if emitted[index] {
            continue;
        }
        emitted[index] = true;
        ordered.push(index);
        if let Some(dependents) = dependents_by_blocker.get(rows[index].id.as_str()) {
            for dependent in dependents {
                indegree[*dependent] = indegree[*dependent].saturating_sub(1);
                if indegree[*dependent] == 0 {
                    available.push(*dependent);
                }
            }
            sort_indices_by_display_order(rows, &mut available);
        }
    }

    if ordered.len() < rows.len() {
        let mut remaining = (0..rows.len())
            .filter(|index| !emitted[*index])
            .collect::<Vec<_>>();
        sort_indices_by_display_order(rows, &mut remaining);
        ordered.extend(remaining);
    }

    ordered
}

pub(crate) fn order_work_rows<T, F>(rows: Vec<T>, mut to_work_row: F) -> Vec<T>
where
    F: FnMut(&T) -> WorkOrderRow,
{
    let keys = rows.iter().map(&mut to_work_row).collect::<Vec<_>>();
    let mut keyed = rows.into_iter().map(Some).collect::<Vec<_>>();
    ordered_work_indices(&keys)
        .into_iter()
        .filter_map(|index| keyed[index].take())
        .collect()
}

fn pop_front<T>(items: &mut Vec<T>) -> Option<T> {
    if items.is_empty() {
        None
    } else {
        Some(items.remove(0))
    }
}

fn sort_indices_by_display_order(rows: &[WorkOrderRow], indices: &mut [usize]) {
    indices.sort_by_key(|index| display_key(&rows[*index]));
}

fn display_key(row: &WorkOrderRow) -> (u8, u8, Reverse<DateTime<Utc>>, &str) {
    (
        state_rank(row.state()),
        priority_rank(&row.priority),
        Reverse(row.updated_at),
        row.id.as_str(),
    )
}

fn state_rank(state: WorkRowState) -> u8 {
    match state {
        WorkRowState::Ready => 0,
        WorkRowState::Active => 1,
        WorkRowState::Blocked => 2,
        WorkRowState::Review => 3,
        WorkRowState::Validation => 4,
        WorkRowState::Done => 5,
        WorkRowState::NotReady => 6,
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

    fn row(id: &str, category: &str, priority: &str, blockers: &[&str]) -> WorkOrderRow {
        WorkOrderRow {
            id: id.to_string(),
            status_category: Some(category.to_string()),
            priority: priority.to_string(),
            updated_at: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
            open_blockers: blockers.iter().map(|id| id.to_string()).collect(),
        }
    }

    fn ordered_ids(rows: &[WorkOrderRow]) -> Vec<String> {
        ordered_work_indices(rows)
            .into_iter()
            .map(|index| rows[index].id.clone())
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
        assert_eq!(rows[0].state().label(), "blocked");
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
        low.updated_at = Utc.with_ymd_and_hms(2026, 1, 3, 0, 0, 0).unwrap();
        let mut newer = row("atelier-b", "todo", "high", &[]);
        newer.updated_at = Utc.with_ymd_and_hms(2026, 1, 2, 0, 0, 0).unwrap();
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
    fn row_state_labels_cover_work_view_vocabulary() {
        let cases = [
            ((Some("todo"), 0), "ready"),
            ((Some("todo"), 1), "blocked"),
            ((Some("active"), 0), "active"),
            ((Some("review"), 0), "review"),
            ((Some("validation"), 0), "validation"),
            ((Some("done"), 1), "done"),
            ((Some("unknown"), 0), "not-ready"),
        ];

        for ((category, blockers), expected) in cases {
            assert_eq!(row_state(category, blockers).label(), expected);
        }
    }
}
