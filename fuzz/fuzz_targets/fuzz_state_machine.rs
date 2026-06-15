#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

use atelier_sqlite::{ProjectionIndex, ProjectionIssue};

#[derive(Arbitrary, Debug, Clone)]
enum StateOp {
    // Issue lifecycle
    CreateIssue { title: String, priority: String },
    CloseIssue { idx: usize },
    ReopenIssue { idx: usize },
    ArchiveIssue { idx: usize },
    UnarchiveIssue { idx: usize },
    DeleteIssue { idx: usize },
    AddComment { idx: usize, content: String },
    // Queries (should never panic)
    ListIssues,
    ListArchived,
    ListReady,
    ListBlocked,
}

#[derive(Arbitrary, Debug)]
struct StateMachineInput {
    ops: Vec<StateOp>,
}

fuzz_target!(|input: StateMachineInput| {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(_) => return,
    };
    let db_path = dir.path().join("state.db");

    let db = match ProjectionIndex::open(&db_path) {
        Ok(d) => d,
        Err(_) => return,
    };

    let mut issue_ids: Vec<String> = Vec::new();

    for op in input.ops.iter().take(100) {
        match op {
            StateOp::CreateIssue { title, priority } => {
                let id = format!("atelier-fuzz-{}", issue_ids.len());
                if db
                    .insert_issue(&fuzz_issue(&id, title, None, priority))
                    .is_ok()
                {
                    issue_ids.push(id);
                }
            }
            StateOp::CloseIssue { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = db.get_issue(id);
                }
            }
            StateOp::ReopenIssue { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = db.get_issue(id);
                }
            }
            StateOp::ArchiveIssue { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = db.get_issue(id);
                }
            }
            StateOp::UnarchiveIssue { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = db.get_issue(id);
                }
            }
            StateOp::DeleteIssue { idx } => {
                if !issue_ids.is_empty() {
                    let idx_val = *idx % issue_ids.len();
                    let id = &issue_ids[idx_val];
                    let _ = db.get_issue(id);
                }
            }
            StateOp::AddComment { idx, content } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = db.add_comment(id, content);
                    let _ = db.get_comments(id);
                }
            }
            StateOp::ListIssues => {
                let _ = db.list_issues(None, None);
            }
            StateOp::ListArchived => {
                let _ = db.list_issues(Some("archived"), None);
            }
            StateOp::ListReady => {
                let _ = db.list_ready_issues();
            }
            StateOp::ListBlocked => {
                let _ = db.list_blocked_issues();
            }
        }
    }

    // Final consistency checks - should never panic
    let _ = db.list_ready_issues();
    let _ = db.list_blocked_issues();
    let _ = db.list_issues(None, None);
    let _ = db.list_issues(Some("archived"), None);
});

fn fuzz_issue(id: &str, title: &str, description: Option<String>, priority: &str) -> ProjectionIssue {
    ProjectionIssue::new(id, title, description, priority)
}
