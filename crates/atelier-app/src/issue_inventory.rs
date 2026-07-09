use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const DEFAULT_ISSUE_INVENTORY_LIMIT: usize = 50;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueInventoryStatusFilter {
    All,
    Exact(String),
}

impl Default for IssueInventoryStatusFilter {
    fn default() -> Self {
        Self::All
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IssueInventoryFilters {
    pub status: IssueInventoryStatusFilter,
    pub category: Option<String>,
    pub issue_type: Option<String>,
    pub label: Option<String>,
    pub priority: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IssueInventoryLimit(usize);

impl IssueInventoryLimit {
    pub fn new(value: usize) -> Result<Self, InvalidIssueInventoryLimit> {
        if value == 0 {
            Err(InvalidIssueInventoryLimit)
        } else {
            Ok(Self(value))
        }
    }

    pub fn get(self) -> usize {
        self.0
    }
}

impl Default for IssueInventoryLimit {
    fn default() -> Self {
        Self(DEFAULT_ISSUE_INVENTORY_LIMIT)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidIssueInventoryLimit;

impl fmt::Display for InvalidIssueInventoryLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("issue inventory limit must be a positive integer")
    }
}

impl std::error::Error for InvalidIssueInventoryLimit {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueInventoryRequest {
    pub filters: IssueInventoryFilters,
    pub limit: IssueInventoryLimit,
}

impl Default for IssueInventoryRequest {
    fn default() -> Self {
        Self {
            filters: IssueInventoryFilters::default(),
            limit: IssueInventoryLimit::default(),
        }
    }
}

/// The persistence-neutral facts needed by the flat inventory.
///
/// Relationship, hierarchy, blocker, readiness, and queue-order facts are
/// intentionally absent from this snapshot because they do not participate in
/// inventory membership or ordering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueInventorySnapshot {
    pub id: String,
    pub issue_type: String,
    pub status: String,
    pub status_category: Option<String>,
    pub priority: String,
    pub title: String,
    pub labels: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueInventoryRow {
    pub id: String,
    pub issue_type: String,
    pub status: String,
    pub status_category: Option<String>,
    pub priority: String,
    pub title: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueInventoryView {
    pub rows: Vec<IssueInventoryRow>,
    pub matching_count: usize,
    pub limit: usize,
}

impl IssueInventoryView {
    pub fn shown_count(&self) -> usize {
        self.rows.len()
    }

    pub fn omitted_count(&self) -> usize {
        self.matching_count.saturating_sub(self.shown_count())
    }

    pub fn is_empty(&self) -> bool {
        self.matching_count == 0
    }

    pub fn is_truncated(&self) -> bool {
        self.omitted_count() > 0
    }
}

pub fn issue_inventory(
    snapshots: impl IntoIterator<Item = IssueInventorySnapshot>,
    request: &IssueInventoryRequest,
) -> IssueInventoryView {
    // Canonical record identity is the inventory identity. Using an ID-keyed
    // map also prevents relationship-expanded inputs from producing duplicate
    // rows if an adapter encounters the same canonical record more than once.
    let canonical_snapshots = snapshots.into_iter().fold(
        BTreeMap::<String, IssueInventorySnapshot>::new(),
        |mut snapshots, snapshot| {
            snapshots.entry(snapshot.id.clone()).or_insert(snapshot);
            snapshots
        },
    );

    let matching = canonical_snapshots
        .into_values()
        .filter(|snapshot| matches_filters(snapshot, &request.filters))
        .collect::<Vec<_>>();
    let matching_count = matching.len();
    let limit = request.limit.get();
    let rows = matching
        .into_iter()
        .take(limit)
        .map(|snapshot| IssueInventoryRow {
            id: snapshot.id,
            issue_type: snapshot.issue_type,
            status: snapshot.status,
            status_category: snapshot.status_category,
            priority: snapshot.priority,
            title: snapshot.title,
        })
        .collect();

    IssueInventoryView {
        rows,
        matching_count,
        limit,
    }
}

fn matches_filters(snapshot: &IssueInventorySnapshot, filters: &IssueInventoryFilters) -> bool {
    let status_matches = match &filters.status {
        IssueInventoryStatusFilter::All => true,
        IssueInventoryStatusFilter::Exact(status) => snapshot.status == *status,
    };

    status_matches
        && filters
            .category
            .as_ref()
            .is_none_or(|category| snapshot.status_category.as_ref() == Some(category))
        && filters
            .issue_type
            .as_ref()
            .is_none_or(|issue_type| snapshot.issue_type == *issue_type)
        && filters
            .label
            .as_ref()
            .is_none_or(|label| snapshot.labels.contains(label))
        && filters
            .priority
            .as_ref()
            .is_none_or(|priority| snapshot.priority == *priority)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(
        id: &str,
        issue_type: &str,
        status: &str,
        category: &str,
        priority: &str,
        labels: &[&str],
    ) -> IssueInventorySnapshot {
        IssueInventorySnapshot {
            id: id.to_string(),
            issue_type: issue_type.to_string(),
            status: status.to_string(),
            status_category: Some(category.to_string()),
            priority: priority.to_string(),
            title: format!("Title for {id}"),
            labels: labels.iter().map(|label| (*label).to_string()).collect(),
        }
    }

    #[test]
    fn default_inventory_is_flat_id_ordered_and_includes_done_records() {
        let records = vec![
            snapshot("atelier-z", "task", "done", "done", "low", &["leaf"]),
            snapshot(
                "atelier-a",
                "mission",
                "in_progress",
                "active",
                "high",
                &["objective"],
            ),
            snapshot("atelier-m", "epic", "todo", "todo", "critical", &["root"]),
            snapshot(
                "atelier-b",
                "task",
                "blocked",
                "blocked",
                "medium",
                &["leaf"],
            ),
        ];

        let view = issue_inventory(records, &IssueInventoryRequest::default());

        assert_eq!(
            view.rows
                .iter()
                .map(|row| row.id.as_str())
                .collect::<Vec<_>>(),
            ["atelier-a", "atelier-b", "atelier-m", "atelier-z"]
        );
        assert_eq!(view.matching_count, 4);
        assert_eq!(view.shown_count(), 4);
        assert_eq!(view.limit, DEFAULT_ISSUE_INVENTORY_LIMIT);
        assert!(!view.is_truncated());
    }

    #[test]
    fn duplicate_canonical_identity_produces_one_row() {
        let issue = snapshot("atelier-a", "task", "todo", "todo", "high", &["one"]);
        let view = issue_inventory([issue.clone(), issue], &IssueInventoryRequest::default());

        assert_eq!(view.matching_count, 1);
        assert_eq!(view.rows[0].id, "atelier-a");
    }

    #[test]
    fn exact_metadata_filters_compose_with_and_semantics() {
        let records = vec![
            snapshot(
                "atelier-a",
                "mission",
                "in_progress",
                "active",
                "high",
                &["cli"],
            ),
            snapshot("atelier-b", "mission", "review", "active", "high", &["cli"]),
            snapshot(
                "atelier-c",
                "mission",
                "in_progress",
                "active",
                "medium",
                &["cli"],
            ),
            snapshot(
                "atelier-d",
                "epic",
                "in_progress",
                "active",
                "high",
                &["cli"],
            ),
            snapshot(
                "atelier-e",
                "mission",
                "in_progress",
                "active",
                "high",
                &["docs"],
            ),
            snapshot(
                "atelier-f",
                "mission",
                "in_progress",
                "blocked",
                "high",
                &["cli"],
            ),
        ];
        let request = IssueInventoryRequest {
            filters: IssueInventoryFilters {
                status: IssueInventoryStatusFilter::Exact("in_progress".to_string()),
                category: Some("active".to_string()),
                issue_type: Some("mission".to_string()),
                label: Some("cli".to_string()),
                priority: Some("high".to_string()),
            },
            limit: IssueInventoryLimit::default(),
        };

        let view = issue_inventory(records, &request);

        assert_eq!(view.rows.len(), 1);
        assert_eq!(view.rows[0].id, "atelier-a");
    }

    #[test]
    fn default_and_explicit_positive_limits_apply_after_selection_and_ordering() {
        let records = (0..55)
            .rev()
            .map(|index| {
                snapshot(
                    &format!("atelier-{index:02}"),
                    "task",
                    "todo",
                    "todo",
                    "medium",
                    &["inventory"],
                )
            })
            .collect::<Vec<_>>();

        let default_view = issue_inventory(records.clone(), &IssueInventoryRequest::default());
        assert_eq!(default_view.shown_count(), 50);
        assert_eq!(default_view.matching_count, 55);
        assert_eq!(default_view.omitted_count(), 5);
        assert!(default_view.is_truncated());
        assert_eq!(default_view.rows[0].id, "atelier-00");
        assert_eq!(default_view.rows[49].id, "atelier-49");

        let explicit_view = issue_inventory(
            records,
            &IssueInventoryRequest {
                limit: IssueInventoryLimit::new(2).unwrap(),
                ..IssueInventoryRequest::default()
            },
        );
        assert_eq!(explicit_view.shown_count(), 2);
        assert_eq!(explicit_view.matching_count, 55);
        assert_eq!(explicit_view.rows[1].id, "atelier-01");
    }

    #[test]
    fn zero_limit_is_rejected() {
        assert_eq!(
            IssueInventoryLimit::new(0).unwrap_err().to_string(),
            "issue inventory limit must be a positive integer"
        );
    }

    #[test]
    fn empty_view_retains_selection_and_budget_facts() {
        let view = issue_inventory([], &IssueInventoryRequest::default());

        assert!(view.is_empty());
        assert_eq!(view.shown_count(), 0);
        assert_eq!(view.matching_count, 0);
        assert_eq!(view.omitted_count(), 0);
        assert_eq!(view.limit, DEFAULT_ISSUE_INVENTORY_LIMIT);
    }
}
