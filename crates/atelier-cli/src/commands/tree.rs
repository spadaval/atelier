use std::collections::BTreeSet;

use anyhow::Result;

use crate::commands::work_order::WorkOrderRow;
use atelier_app::workflow_policy::WorkflowPolicy;
use atelier_core::{DomainRecord, Issue};
use atelier_sqlite::Database;

const COMPACT_MAX_DEPTH: usize = 3;
const COMPACT_MAX_SIBLINGS: usize = 6;

fn print_issue(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
    indent: usize,
) -> Result<()> {
    let prefix = "  ".repeat(indent);
    let row = work_order_row_for_issue(db, workflow_policy, issue)?;
    println!(
        "{}[{}] #{} {} - {}{}",
        prefix,
        row.state().label(),
        issue.id,
        issue.priority,
        issue.title,
        blocker_suffix(&issue.id, &row.open_blockers)
    );
    Ok(())
}

fn print_mission(mission: &DomainRecord, indent: usize) {
    let prefix = "  ".repeat(indent);
    println!(
        "{}[mission {}] #{} - {}",
        prefix, mission.status, mission.id, mission.title
    );
}

fn print_tree_recursive(
    db: &Database,
    parent_id: &str,
    indent: usize,
    status_filter: Option<&str>,
) -> Result<()> {
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let subissues =
        order_issues_by_work(db, workflow_policy.as_ref(), db.get_subissues(parent_id)?)?;
    for sub in subissues {
        let dominated_by_filter = match status_filter {
            Some("all") | None => false,
            Some(filter) => sub.status != filter,
        };
        if dominated_by_filter {
            continue;
        }
        print_issue(db, workflow_policy.as_ref(), &sub, indent)?;
        print_tree_recursive(db, &sub.id, indent + 1, status_filter)?;
    }
    Ok(())
}

fn mission_linked_issue_ids(db: &Database, mission_id: &str) -> Result<Vec<String>> {
    let mut issue_ids = BTreeSet::new();
    for link in db.list_record_links("mission", mission_id)? {
        if link.relation_type != "advances" {
            continue;
        }
        if link.source_kind == "issue" {
            issue_ids.insert(link.source_id);
        } else if link.target_kind == "issue" {
            issue_ids.insert(link.target_id);
        }
    }
    Ok(issue_ids.into_iter().collect())
}

fn mission_linked_root_ids(db: &Database) -> Result<BTreeSet<String>> {
    let mut issue_ids = BTreeSet::new();
    for mission in db.list_records("mission", None)? {
        for issue_id in mission_linked_issue_ids(db, &mission.id)? {
            issue_ids.insert(issue_id);
        }
    }
    Ok(issue_ids)
}

fn print_mission_trees(
    db: &Database,
    status_filter: Option<&str>,
    linked_issue_ids: &mut BTreeSet<String>,
) -> Result<()> {
    for mission in db.list_records("mission", None)? {
        let issue_ids = mission_linked_issue_ids(db, &mission.id)?;
        if issue_ids.is_empty() {
            continue;
        }
        print_mission(&mission, 0);
        let issues = issue_ids
            .into_iter()
            .map(|issue_id| db.require_issue(&issue_id))
            .collect::<Result<Vec<_>>>()?;
        let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
        for issue in order_issues_by_work(db, workflow_policy.as_ref(), issues)? {
            if !status_matches(&issue, status_filter) {
                continue;
            }
            linked_issue_ids.insert(issue.id.clone());
            print_issue(db, workflow_policy.as_ref(), &issue, 1)?;
            print_tree_recursive(db, &issue.id, 2, status_filter)?;
        }
    }
    Ok(())
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct ProgressSummary {
    todo: usize,
    done: usize,
    other: usize,
}

impl ProgressSummary {
    fn add_issue(&mut self, issue: &Issue) {
        match issue.status.as_str() {
            "todo" => self.todo += 1,
            "done" => self.done += 1,
            _ => self.other += 1,
        }
    }

    fn add_summary(&mut self, summary: ProgressSummary) {
        self.todo += summary.todo;
        self.done += summary.done;
        self.other += summary.other;
    }

    fn total(self) -> usize {
        self.todo + self.done + self.other
    }

    fn format(self) -> String {
        let mut parts = vec![format!("todo={}", self.todo), format!("done={}", self.done)];
        if self.other > 0 {
            parts.push(format!("other={}", self.other));
        }
        parts.join(" ")
    }
}

fn status_matches(issue: &Issue, status_filter: Option<&str>) -> bool {
    match status_filter {
        Some("all") | None => true,
        Some(filter) => issue.status == filter,
    }
}

fn order_issues_by_work(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issues: Vec<Issue>,
) -> Result<Vec<Issue>> {
    let rows = issues
        .iter()
        .map(|issue| work_order_row_for_issue(db, workflow_policy, issue))
        .collect::<Result<Vec<_>>>()?;
    let mut keyed = issues.into_iter().map(Some).collect::<Vec<_>>();
    Ok(crate::commands::work_order::ordered_work_indices(&rows)
        .into_iter()
        .filter_map(|index| keyed[index].take())
        .collect())
}

fn work_order_row_for_issue(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
) -> Result<WorkOrderRow> {
    Ok(WorkOrderRow {
        id: issue.id.clone(),
        status_category: crate::commands::issue_workflow::issue_status_category(
            workflow_policy,
            &issue.status,
        ),
        priority: issue.priority.clone(),
        updated_at: issue.updated_at,
        open_blockers: open_blockers(db, &issue.id, workflow_policy)?,
    })
}

fn open_blockers(
    db: &Database,
    issue_id: &str,
    workflow_policy: Option<&WorkflowPolicy>,
) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for blocker_id in db.get_blockers(issue_id)? {
        let blocker = db.require_issue(&blocker_id)?;
        if crate::commands::issue_workflow::issue_blocks_work(workflow_policy, &blocker) {
            blockers.push(blocker_id);
        }
    }
    blockers.sort();
    Ok(blockers)
}

fn blocker_suffix(issue_id: &str, blockers: &[String]) -> String {
    if blockers.is_empty() {
        String::new()
    } else {
        format!(
            " ({} blocker{}; details: atelier issue blocked {issue_id})",
            blockers.len(),
            plural_suffix(blockers.len())
        )
    }
}

fn plural_suffix(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

fn filtered_subissues(
    db: &Database,
    parent_id: &str,
    status_filter: Option<&str>,
) -> Result<Vec<Issue>> {
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let issues = db
        .get_subissues(parent_id)?
        .into_iter()
        .filter(|issue| status_matches(issue, status_filter))
        .collect();
    order_issues_by_work(db, workflow_policy.as_ref(), issues)
}

fn descendant_summary(
    db: &Database,
    parent_id: &str,
    status_filter: Option<&str>,
) -> Result<ProgressSummary> {
    let mut summary = ProgressSummary::default();

    for child in filtered_subissues(db, parent_id, status_filter)? {
        summary.add_issue(&child);
        summary.add_summary(descendant_summary(db, &child.id, status_filter)?);
    }

    Ok(summary)
}

fn direct_child_summary(children: &[Issue]) -> ProgressSummary {
    let mut summary = ProgressSummary::default();
    for child in children {
        summary.add_issue(child);
    }
    summary
}

fn issue_set_summary(
    db: &Database,
    issues: &[Issue],
    status_filter: Option<&str>,
) -> Result<ProgressSummary> {
    let mut summary = ProgressSummary::default();
    for issue in issues {
        summary.add_issue(issue);
        summary.add_summary(descendant_summary(db, &issue.id, status_filter)?);
    }
    Ok(summary)
}

fn compact_issue_line(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: &Issue,
    indent: usize,
    children: &[Issue],
) -> Result<()> {
    let prefix = "  ".repeat(indent);
    let child_summary = direct_child_summary(children);
    let child_suffix = if children.is_empty() {
        String::new()
    } else {
        format!(" children={} {}", children.len(), child_summary.format())
    };

    let row = work_order_row_for_issue(db, workflow_policy, issue)?;
    println!(
        "{}[{} {}] {} [{}] - {}{}{}",
        prefix,
        row.state().label(),
        issue.priority,
        issue.id,
        issue.issue_type,
        issue.title,
        blocker_suffix(&issue.id, &row.open_blockers),
        child_suffix
    );
    Ok(())
}

fn compact_missions(
    db: &Database,
    status_filter: Option<&str>,
) -> Result<Vec<(DomainRecord, Vec<Issue>)>> {
    let mut rows = Vec::new();
    for mission in db.list_records("mission", None)? {
        let issues = mission_linked_issue_ids(db, &mission.id)?
            .into_iter()
            .map(|issue_id| db.require_issue(&issue_id))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter(|issue| status_matches(issue, status_filter))
            .collect::<Vec<_>>();
        if !issues.is_empty() {
            let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
            rows.push((
                mission,
                order_issues_by_work(db, workflow_policy.as_ref(), issues)?,
            ));
        }
    }
    Ok(rows)
}

fn print_compact_mission(mission: &DomainRecord, issues: &[Issue]) {
    let summary = direct_child_summary(issues);
    println!(
        "[mission {}] {} - {} linked={} {}",
        mission.status,
        mission.id,
        mission.title,
        issues.len(),
        summary.format()
    );
}

fn compact_tree_recursive(
    db: &Database,
    issue: &Issue,
    indent: usize,
    status_filter: Option<&str>,
) -> Result<()> {
    let children = filtered_subissues(db, &issue.id, status_filter)?;
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    compact_issue_line(db, workflow_policy.as_ref(), issue, indent, &children)?;

    if children.is_empty() {
        return Ok(());
    }

    if indent >= COMPACT_MAX_DEPTH {
        let summary = descendant_summary(db, &issue.id, status_filter)?;
        println!(
            "{}... {} descendants collapsed ({})",
            "  ".repeat(indent + 1),
            summary.total(),
            summary.format()
        );
        return Ok(());
    }

    for child in children.iter().take(COMPACT_MAX_SIBLINGS) {
        compact_tree_recursive(db, child, indent + 1, status_filter)?;
    }

    if children.len() > COMPACT_MAX_SIBLINGS {
        let mut omitted_summary = ProgressSummary::default();
        for child in children.iter().skip(COMPACT_MAX_SIBLINGS) {
            omitted_summary.add_issue(child);
            omitted_summary.add_summary(descendant_summary(db, &child.id, status_filter)?);
        }
        println!(
            "{}... {} more children omitted ({})",
            "  ".repeat(indent + 1),
            children.len() - COMPACT_MAX_SIBLINGS,
            omitted_summary.format()
        );
    }

    Ok(())
}

pub fn run(db: &Database, status_filter: Option<&str>) -> Result<()> {
    let mut linked_issue_ids = BTreeSet::new();
    print_mission_trees(db, status_filter, &mut linked_issue_ids)?;

    // Get all top-level issues (no parent)
    let all_issues = db.list_issues(status_filter, None, None)?;
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let top_level: Vec<_> = all_issues
        .into_iter()
        .filter(|i| i.parent_id.is_none() && !linked_issue_ids.contains(&i.id))
        .collect();
    let top_level = order_issues_by_work(db, workflow_policy.as_ref(), top_level)?;

    if top_level.is_empty() && linked_issue_ids.is_empty() {
        println!("No issues found.");
        return Ok(());
    }

    for issue in top_level {
        print_issue(db, workflow_policy.as_ref(), &issue, 0)?;
        print_tree_recursive(db, &issue.id, 1, status_filter)?;
    }

    // Legend
    println!();
    println!("Legend: ready, blocked, active, review, validation, done, not-ready");

    Ok(())
}

pub fn run_compact(db: &Database, status_filter: Option<&str>) -> Result<()> {
    let linked_root_ids = mission_linked_root_ids(db)?;
    let all_issues = db.list_issues(status_filter, None, None)?;
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let top_level: Vec<_> = all_issues
        .into_iter()
        .filter(|i| i.parent_id.is_none() && !linked_root_ids.contains(&i.id))
        .collect();
    let top_level = order_issues_by_work(db, workflow_policy.as_ref(), top_level)?;
    let missions = compact_missions(db, status_filter)?;

    if top_level.is_empty() && missions.is_empty() {
        println!("No issues found.");
        return Ok(());
    }

    println!("Compact Issue Hierarchy");
    println!("=======================");
    println!(
        "Limits: depth={} siblings={}",
        COMPACT_MAX_DEPTH, COMPACT_MAX_SIBLINGS
    );
    println!();

    for (mission, issues) in &missions {
        print_compact_mission(mission, issues);
        for issue in issues.iter().take(COMPACT_MAX_SIBLINGS) {
            compact_tree_recursive(db, issue, 1, status_filter)?;
        }
        if issues.len() > COMPACT_MAX_SIBLINGS {
            println!(
                "  ... {} more linked issues omitted",
                issues.len() - COMPACT_MAX_SIBLINGS
            );
        }
    }

    for issue in top_level.iter().take(COMPACT_MAX_SIBLINGS) {
        compact_tree_recursive(db, &issue, 0, status_filter)?;
    }

    if top_level.len() > COMPACT_MAX_SIBLINGS {
        let omitted = &top_level[COMPACT_MAX_SIBLINGS..];
        let omitted_summary = issue_set_summary(db, omitted, status_filter)?;
        println!(
            "... {} more root issues omitted ({})",
            omitted.len(),
            omitted_summary.format()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn setup_test_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::open(&db_path).unwrap();
        (db, dir)
    }

    #[test]
    fn test_blocker_suffix_empty() {
        assert_eq!(blocker_suffix("atelier-a", &[]), "");
    }

    #[test]
    fn test_blocker_suffix_names_drill_down() {
        assert_eq!(
            blocker_suffix("atelier-a", &["atelier-b".to_string()]),
            " (1 blocker; details: atelier issue blocked atelier-a)"
        );
    }

    #[test]
    fn test_run_empty() {
        let (db, _dir) = setup_test_db();
        run(&db, None).unwrap();
        let issues = db.list_issues(None, None, None).unwrap();
        assert!(issues.is_empty());
    }

    #[test]
    fn test_run_single_issue() {
        let (db, _dir) = setup_test_db();
        let id = db.create_issue("Test issue", None, "medium").unwrap();
        run(&db, None).unwrap();
        let issues = db.list_issues(None, None, None).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id, id);
    }

    #[test]
    fn test_run_with_hierarchy() {
        let (db, _dir) = setup_test_db();
        let parent = db.create_issue("Parent", None, "high").unwrap();
        let c1 = db
            .create_subissue(&parent, "Child 1", None, "medium")
            .unwrap();
        let c2 = db.create_subissue(&parent, "Child 2", None, "low").unwrap();
        run(&db, None).unwrap();
        let subs = db.get_subissues(parent).unwrap();
        assert_eq!(subs.len(), 2);
        assert!(subs.iter().any(|s| s.id == c1));
        assert!(subs.iter().any(|s| s.id == c2));
    }

    #[test]
    fn test_run_nested_hierarchy() {
        let (db, _dir) = setup_test_db();
        let grandparent = db.create_issue("Grandparent", None, "high").unwrap();
        let parent = db
            .create_subissue(&grandparent, "Parent", None, "medium")
            .unwrap();
        let child = db.create_subissue(&parent, "Child", None, "low").unwrap();
        run(&db, None).unwrap();
        let child_issue = db.get_issue(&child).unwrap().unwrap();
        assert_eq!(child_issue.parent_id, Some(parent.clone()));
        let parent_issue = db.get_issue(&parent).unwrap().unwrap();
        assert_eq!(parent_issue.parent_id, Some(grandparent));
    }

    #[test]
    fn test_run_with_status_filter() {
        let (db, _dir) = setup_test_db();
        let done_id = db.create_issue("Done issue", None, "medium").unwrap();
        let todo_id = db.create_issue("Todo issue", None, "medium").unwrap();
        db.close_issue(&done_id).unwrap();
        run(&db, Some("todo")).unwrap();
        let todo_issues = db.list_issues(Some("todo"), None, None).unwrap();
        assert_eq!(todo_issues.len(), 1);
        assert_eq!(todo_issues[0].id, todo_id);
    }

    #[test]
    fn test_run_done_filter() {
        let (db, _dir) = setup_test_db();
        let id = db.create_issue("Issue", None, "medium").unwrap();
        db.close_issue(&id).unwrap();
        run(&db, Some("done")).unwrap();
        let done = db.list_issues(Some("done"), None, None).unwrap();
        assert_eq!(done.len(), 1);
        assert_eq!(done[0].id, id);
    }

    #[test]
    fn test_run_all_filter() {
        let (db, _dir) = setup_test_db();
        db.create_issue("Todo issue", None, "medium").unwrap();
        let id = db.create_issue("Done issue", None, "medium").unwrap();
        db.close_issue(&id).unwrap();
        run(&db, Some("all")).unwrap();
        let all = db.list_issues(Some("all"), None, None).unwrap();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_run_accepts_empty_flat_and_hierarchical_trees() {
        let (db, _dir) = setup_test_db();
        assert!(run(&db, None).is_ok());

        let root = db.create_issue("Root", None, "high").unwrap();
        let child = db.create_subissue(&root, "Child", None, "medium").unwrap();
        db.create_subissue(&child, "Grandchild", None, "medium")
            .unwrap();

        assert!(run(&db, None).is_ok());
    }

    #[test]
    fn test_progress_summary_counts_mixed_statuses() {
        let (db, _dir) = setup_test_db();
        let root = db.create_issue("Root", None, "high").unwrap();
        let todo = db.create_subissue(&root, "Todo", None, "medium").unwrap();
        let done = db.create_subissue(&root, "Done", None, "medium").unwrap();
        db.close_issue(&done).unwrap();
        db.create_subissue(&todo, "Nested todo", None, "low")
            .unwrap();

        let summary = descendant_summary(&db, &root, Some("all")).unwrap();

        assert_eq!(summary.todo, 2);
        assert_eq!(summary.done, 1);
    }

    #[test]
    fn test_filtered_subissues_excludes_done_children() {
        let (db, _dir) = setup_test_db();
        let root = db.create_issue("Root", None, "high").unwrap();
        db.create_subissue(&root, "Todo", None, "medium").unwrap();
        let done = db.create_subissue(&root, "Done", None, "medium").unwrap();
        db.close_issue(&done).unwrap();

        let children = filtered_subissues(&db, &root, Some("todo")).unwrap();

        assert_eq!(children.len(), 1);
        assert_eq!(children[0].status, "todo");
    }
}
