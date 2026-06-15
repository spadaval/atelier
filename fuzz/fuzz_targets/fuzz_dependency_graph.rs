#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

use atelier::db::Database;
use atelier::models::Issue;

#[derive(Arbitrary, Debug, Clone)]
enum DependencyOp {
    CreateIssue { title: String },
    AddDependency { blocked_idx: usize, blocker_idx: usize },
    RemoveDependency { blocked_idx: usize, blocker_idx: usize },
    CloseIssue { idx: usize },
    ReopenIssue { idx: usize },
    CheckReady,
    CheckBlocked,
}

#[derive(Arbitrary, Debug)]
struct DependencyGraphInput {
    ops: Vec<DependencyOp>,
}

fuzz_target!(|input: DependencyGraphInput| {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(_) => return,
    };
    let db_path = dir.path().join("state.db");

    let db = match Database::open(&db_path) {
        Ok(d) => d,
        Err(_) => return,
    };

    // Track created issue IDs
    let mut issue_ids: Vec<String> = Vec::new();

    for op in input.ops.iter().take(100) {
        // Limit operations to prevent timeout
        match op {
            DependencyOp::CreateIssue { title } => {
                let id = format!("atelier-fuzz-{}", issue_ids.len());
                if db
                    .insert_issue_rebuild(&fuzz_issue(&id, title, None, "medium"))
                    .is_ok()
                {
                    issue_ids.push(id);
                }
            }
            DependencyOp::AddDependency {
                blocked_idx,
                blocker_idx,
            } => {
                if issue_ids.len() >= 2 {
                    let blocked = &issue_ids[*blocked_idx % issue_ids.len()];
                    let blocker = &issue_ids[*blocker_idx % issue_ids.len()];
                    // This should never panic, even with cycles or self-blocks
                    let _ = db.add_dependency(blocked, blocker);
                }
            }
            DependencyOp::RemoveDependency {
                blocked_idx,
                blocker_idx,
            } => {
                if issue_ids.len() >= 2 {
                    let blocked = &issue_ids[*blocked_idx % issue_ids.len()];
                    let blocker = &issue_ids[*blocker_idx % issue_ids.len()];
                    let _ = (blocked, blocker);
                }
            }
            DependencyOp::CloseIssue { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = db.get_issue(id);
                }
            }
            DependencyOp::ReopenIssue { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = db.get_issue(id);
                }
            }
            DependencyOp::CheckReady => {
                // Should never panic or hang
                let _ = db.list_ready_issues();
            }
            DependencyOp::CheckBlocked => {
                // Should never panic or hang
                let _ = db.list_blocked_issues();
            }
        }
    }

    // Final verification - these should never panic
    let _ = db.list_ready_issues();
    let _ = db.list_blocked_issues();
    let _ = db.list_issues(None, None, None);
});

fn fuzz_issue(id: &str, title: &str, description: Option<String>, priority: &str) -> Issue {
    let now = chrono::Utc::now();
    Issue {
        id: id.to_string(),
        title: title.to_string(),
        description,
        status: "todo".to_string(),
        issue_type: "task".to_string(),
        priority: priority.to_string(),
        parent_id: None,
        created_at: now,
        updated_at: now,
        closed_at: None,
    }
}
