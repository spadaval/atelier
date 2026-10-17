//! Flat canonical inventory: validate filters, select once, then limit display.

use anyhow::{anyhow, bail, Result};
use atelier_core::{invalid_value_message, IssuePriority};
use atelier_records::CanonicalIssueRecord;

use crate::record_index::RecordIndex;
use crate::workflow_policy::WorkflowPolicy;

pub const DEFAULT_ISSUE_INVENTORY_LIMIT: usize = 50;

#[derive(Clone, Copy, Debug, Default)]
pub struct IssueInventoryFilters<'a> {
    pub status: Option<&'a str>,
    pub category: Option<&'a str>,
    pub issue_type: Option<&'a str>,
    pub label: Option<&'a str>,
    pub priority: Option<&'a str>,
}

pub struct IssueInventoryView<'a> {
    pub rows: Vec<&'a CanonicalIssueRecord>,
    pub matching_count: usize,
    pub policy: Option<&'a WorkflowPolicy>,
}

pub fn issue_inventory<'a>(
    index: &'a RecordIndex,
    policy: Option<&'a WorkflowPolicy>,
    filters: IssueInventoryFilters<'_>,
    limit: usize,
) -> Result<IssueInventoryView<'a>> {
    if let Some(issue_type) = filters.issue_type {
        let policy = policy.ok_or_else(|| anyhow!("--issue-type requires a workflow policy"))?;
        if !policy.issue_types.contains_key(issue_type) {
            let valid = policy.issue_types.keys().collect::<Vec<_>>();
            bail!(invalid_value_message("issue type", issue_type, &valid));
        }
    }
    if let Some(status) = filters.status {
        if let Some(policy) = policy.filter(|policy| !policy.statuses.contains_key(status)) {
            let mut valid = policy
                .statuses
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>();
            valid.push("all");
            bail!(invalid_value_message("status", status, &valid));
        }
    }
    if let Some(category) = filters.category {
        let policy = policy.ok_or_else(|| anyhow!("--category requires a workflow policy"))?;
        let valid = policy
            .statuses
            .values()
            .map(|status| status.category.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        if !valid.contains(category) {
            let valid = valid.into_iter().collect::<Vec<_>>();
            bail!(invalid_value_message("category", category, &valid));
        }
    }
    if limit == 0 {
        bail!("issue inventory limit must be a positive integer");
    }
    let priority = filters
        .priority
        .map(IssuePriority::from_cli_input)
        .transpose()?;
    let mut matching_count = 0;
    let mut rows = Vec::new();
    // RecordIndex owns unique identity and ID order. Selection uses actual labels.
    for record in index.issue_records() {
        let issue = &record.issue;
        if filters.status.is_none_or(|status| issue.status == status)
            && filters.category.is_none_or(|category| {
                policy.and_then(|policy| policy.status_category(&issue.status)) == Some(category)
            })
            && filters
                .issue_type
                .is_none_or(|kind| issue.issue_type == kind)
            && filters
                .label
                .is_none_or(|label| record.labels.iter().any(|value| value == label))
            && priority.is_none_or(|priority| issue.priority == priority.label())
        {
            matching_count += 1;
            if rows.len() < limit {
                rows.push(record);
            }
        }
    }
    Ok(IssueInventoryView {
        rows,
        matching_count,
        policy,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_records::Records;
    use crate::workflow_policy::{load, STARTER_POLICY_YAML};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn canonical_inventory_is_flat_complete_and_limited_after_id_ordering() {
        let mut records = Records::default();
        for n in (0..55).rev() {
            records.issue(
                &format!("atelier-{n:02}"),
                if n == 0 { "mission" } else { "task" },
                if n == 54 { "done" } else { "todo" },
            );
        }
        let index = records.index();
        let default = issue_inventory(
            &index,
            None,
            IssueInventoryFilters::default(),
            DEFAULT_ISSUE_INVENTORY_LIMIT,
        )
        .unwrap();
        assert_eq!(default.matching_count, 55);
        assert_eq!(default.rows.len(), 50);
        assert_eq!(default.rows[0].issue.id, "atelier-00");
        assert_eq!(default.rows[49].issue.id, "atelier-49");
        assert_eq!(default.rows[0].issue.issue_type, "mission");
        let all = issue_inventory(&index, None, IssueInventoryFilters::default(), 55).unwrap();
        assert_eq!(all.rows[54].issue.status, "done");
        let two = issue_inventory(&index, None, IssueInventoryFilters::default(), 2).unwrap();
        assert_eq!(two.matching_count, 55);
        assert_eq!(two.rows.len(), 2);
        assert_eq!(two.rows[1].issue.id, "atelier-01");
        assert!(std::ptr::eq(
            two.rows[0],
            index.issue_record("atelier-00").unwrap()
        ));
    }

    #[test]
    fn exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join(".atelier")).unwrap();
        fs::write(
            dir.path().join(".atelier/workflow.yaml"),
            STARTER_POLICY_YAML.replace("in_progress", "working"),
        )
        .unwrap();
        let policy = load(dir.path()).unwrap();
        let mut records = Records::default();
        for (id, kind, status, priority, label) in [
            ("atelier-a", "bug", "working", "high", "cli"),
            ("atelier-b", "bug", "todo", "high", "cli"),
            ("atelier-c", "task", "working", "high", "cli"),
            ("atelier-d", "bug", "working", "low", "cli"),
            ("atelier-e", "bug", "working", "high", "docs"),
        ] {
            let record = records.issue(id, kind, status);
            record.issue.priority = priority.to_string();
            record.labels.push(label.to_string());
        }
        let index = records.index();
        let filters = IssueInventoryFilters {
            status: Some("working"),
            category: Some("active"),
            issue_type: Some("bug"),
            priority: Some("high"),
            label: Some("cli"),
        };
        let view = issue_inventory(&index, Some(&policy), filters, 50).unwrap();
        assert_eq!(view.matching_count, 1);
        assert_eq!(view.rows[0].issue.id, "atelier-a");
        let empty = issue_inventory(
            &index,
            Some(&policy),
            IssueInventoryFilters {
                category: Some("blocked"),
                ..filters
            },
            50,
        )
        .unwrap();
        assert_eq!(empty.matching_count, 0);
        assert!(empty.rows.is_empty());
        for filters in [
            IssueInventoryFilters {
                status: Some("unknown"),
                ..Default::default()
            },
            IssueInventoryFilters {
                category: Some("unknown"),
                ..Default::default()
            },
            IssueInventoryFilters {
                issue_type: Some("unknown"),
                ..Default::default()
            },
            IssueInventoryFilters {
                priority: Some("HIGH"),
                ..Default::default()
            },
        ] {
            assert!(issue_inventory(&index, Some(&policy), filters, 50).is_err());
        }
    }

    #[test]
    fn zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory() {
        let index = Records::default().index();
        assert_eq!(
            issue_inventory(&index, None, IssueInventoryFilters::default(), 0)
                .err()
                .unwrap()
                .to_string(),
            "issue inventory limit must be a positive integer"
        );
        for filters in [
            IssueInventoryFilters {
                category: Some("active"),
                ..Default::default()
            },
            IssueInventoryFilters {
                issue_type: Some("task"),
                ..Default::default()
            },
        ] {
            assert!(issue_inventory(&index, None, filters, 50).is_err());
        }
        let view = issue_inventory(&index, None, IssueInventoryFilters::default(), 50).unwrap();
        assert!(view.rows.is_empty());
        assert_eq!(view.matching_count, 0);
    }
}
