use anyhow::Result;

use crate::db::Database;
use crate::models::Issue;

const COMPACT_MAX_DEPTH: usize = 3;
const COMPACT_MAX_SIBLINGS: usize = 6;

fn status_icon(status: &str) -> &'static str {
    match status {
        "open" => " ",
        "closed" => "x",
        _ => "?",
    }
}

fn print_issue(issue: &Issue, indent: usize) {
    let prefix = "  ".repeat(indent);
    let icon = status_icon(&issue.status);
    println!(
        "{}[{}] #{} {} - {}",
        prefix, icon, issue.id, issue.priority, issue.title
    );
}

fn print_tree_recursive(
    db: &Database,
    parent_id: &str,
    indent: usize,
    status_filter: Option<&str>,
) -> Result<()> {
    let subissues = db.get_subissues(parent_id)?;
    for sub in subissues {
        let dominated_by_filter = match status_filter {
            Some("all") | None => false,
            Some(filter) => sub.status != filter,
        };
        if dominated_by_filter {
            continue;
        }
        print_issue(&sub, indent);
        print_tree_recursive(db, &sub.id, indent + 1, status_filter)?;
    }
    Ok(())
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct ProgressSummary {
    open: usize,
    closed: usize,
    other: usize,
}

impl ProgressSummary {
    fn add_issue(&mut self, issue: &Issue) {
        match issue.status.as_str() {
            "open" => self.open += 1,
            "closed" => self.closed += 1,
            _ => self.other += 1,
        }
    }

    fn add_summary(&mut self, summary: ProgressSummary) {
        self.open += summary.open;
        self.closed += summary.closed;
        self.other += summary.other;
    }

    fn total(self) -> usize {
        self.open + self.closed + self.other
    }

    fn format(self) -> String {
        let mut parts = vec![
            format!("open={}", self.open),
            format!("closed={}", self.closed),
        ];
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

fn filtered_subissues(
    db: &Database,
    parent_id: &str,
    status_filter: Option<&str>,
) -> Result<Vec<Issue>> {
    Ok(db
        .get_subissues(parent_id)?
        .into_iter()
        .filter(|issue| status_matches(issue, status_filter))
        .collect())
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

fn compact_issue_line(issue: &Issue, indent: usize, children: &[Issue]) {
    let prefix = "  ".repeat(indent);
    let child_summary = direct_child_summary(children);
    let child_suffix = if children.is_empty() {
        String::new()
    } else {
        format!(" children={} {}", children.len(), child_summary.format())
    };

    println!(
        "{}[{} {}] {} [{}] - {}{}",
        prefix, issue.status, issue.priority, issue.id, issue.issue_type, issue.title, child_suffix
    );
}

fn compact_tree_recursive(
    db: &Database,
    issue: &Issue,
    indent: usize,
    status_filter: Option<&str>,
) -> Result<()> {
    let children = filtered_subissues(db, &issue.id, status_filter)?;
    compact_issue_line(issue, indent, &children);

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
    // Get all top-level issues (no parent)
    let all_issues = db.list_issues(status_filter, None, None)?;
    let top_level: Vec<_> = all_issues
        .into_iter()
        .filter(|i| i.parent_id.is_none())
        .collect();

    if top_level.is_empty() {
        println!("No issues found.");
        return Ok(());
    }

    for issue in top_level {
        print_issue(&issue, 0);
        print_tree_recursive(db, &issue.id, 1, status_filter)?;
    }

    // Legend
    println!();
    println!("Legend: [ ] open, [x] closed");

    Ok(())
}

pub fn run_compact(db: &Database, status_filter: Option<&str>) -> Result<()> {
    let all_issues = db.list_issues(status_filter, None, None)?;
    let top_level: Vec<_> = all_issues
        .into_iter()
        .filter(|i| i.parent_id.is_none())
        .collect();

    if top_level.is_empty() {
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
    fn test_status_icon_open() {
        assert_eq!(status_icon("open"), " ");
    }

    #[test]
    fn test_status_icon_closed() {
        assert_eq!(status_icon("closed"), "x");
    }

    #[test]
    fn test_status_icon_unknown() {
        assert_eq!(status_icon("archived"), "?");
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
        let closed_id = db.create_issue("Closed issue", None, "medium").unwrap();
        let open_id = db.create_issue("Open issue", None, "medium").unwrap();
        db.close_issue(&closed_id).unwrap();
        run(&db, Some("open")).unwrap();
        let open_issues = db.list_issues(Some("open"), None, None).unwrap();
        assert_eq!(open_issues.len(), 1);
        assert_eq!(open_issues[0].id, open_id);
    }

    #[test]
    fn test_run_closed_filter() {
        let (db, _dir) = setup_test_db();
        let id = db.create_issue("Issue", None, "medium").unwrap();
        db.close_issue(&id).unwrap();
        run(&db, Some("closed")).unwrap();
        let closed = db.list_issues(Some("closed"), None, None).unwrap();
        assert_eq!(closed.len(), 1);
        assert_eq!(closed[0].id, id);
    }

    #[test]
    fn test_run_all_filter() {
        let (db, _dir) = setup_test_db();
        db.create_issue("Open issue", None, "medium").unwrap();
        let id = db.create_issue("Closed issue", None, "medium").unwrap();
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
        let open = db.create_subissue(&root, "Open", None, "medium").unwrap();
        let closed = db.create_subissue(&root, "Closed", None, "medium").unwrap();
        db.close_issue(&closed).unwrap();
        db.create_subissue(&open, "Nested open", None, "low")
            .unwrap();

        let summary = descendant_summary(&db, &root, Some("all")).unwrap();

        assert_eq!(summary.open, 2);
        assert_eq!(summary.closed, 1);
    }

    #[test]
    fn test_filtered_subissues_excludes_closed_children() {
        let (db, _dir) = setup_test_db();
        let root = db.create_issue("Root", None, "high").unwrap();
        db.create_subissue(&root, "Open", None, "medium").unwrap();
        let closed = db.create_subissue(&root, "Closed", None, "medium").unwrap();
        db.close_issue(&closed).unwrap();

        let children = filtered_subissues(&db, &root, Some("open")).unwrap();

        assert_eq!(children.len(), 1);
        assert_eq!(children[0].status, "open");
    }
}
