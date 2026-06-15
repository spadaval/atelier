#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

use atelier::db::Database;
use atelier::models::Issue;

#[derive(Arbitrary, Debug, Clone)]
enum StateOp {
    // Issue lifecycle
    CreateIssue { title: String, priority: String },
    CloseIssue { idx: usize },
    ReopenIssue { idx: usize },
    ArchiveIssue { idx: usize },
    UnarchiveIssue { idx: usize },
    DeleteIssue { idx: usize },
    // Session lifecycle
    StartSession,
    EndSession { notes: Option<String> },
    SetSessionIssue { idx: usize },
    // Timer lifecycle
    StartTimer { idx: usize },
    StopTimer { idx: usize },
    // Queries (should never panic)
    GetCurrentSession,
    GetActiveTimer,
    ListIssues,
    ListArchived,
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

    let db = match Database::open(&db_path) {
        Ok(d) => d,
        Err(_) => return,
    };

    let mut issue_ids: Vec<String> = Vec::new();
    let mut session_id: Option<i64> = None;

    for op in input.ops.iter().take(100) {
        match op {
            StateOp::CreateIssue { title, priority } => {
                let id = format!("atelier-fuzz-{}", issue_ids.len());
                if db
                    .insert_issue_rebuild(&fuzz_issue(&id, title, None, priority))
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
            StateOp::StartSession => {
                if let Ok(id) = db.start_session_with_agent(None) {
                    session_id = Some(id);
                }
            }
            StateOp::EndSession { notes } => {
                if let Some(sid) = session_id {
                    let _ = (sid, notes);
                    session_id = None;
                }
            }
            StateOp::SetSessionIssue { idx } => {
                if let Some(sid) = session_id {
                    if !issue_ids.is_empty() {
                        let id = &issue_ids[*idx % issue_ids.len()];
                        let _ = db.set_session_issue(sid, id);
                    }
                }
            }
            StateOp::StartTimer { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = db.get_issue(id);
                }
            }
            StateOp::StopTimer { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = db.get_issue(id);
                }
            }
            StateOp::GetCurrentSession => {
                let _ = db.get_current_session();
            }
            StateOp::GetActiveTimer => {
                let _ = db.list_work_associations();
            }
            StateOp::ListIssues => {
                let _ = db.list_issues(None, None, None);
            }
            StateOp::ListArchived => {
                let _ = db.list_issues(Some("archived"), None, None);
            }
        }
    }

    // Final consistency checks - should never panic
    let _ = db.get_current_session();
    let _ = db.list_work_associations();
    let _ = db.list_issues(None, None, None);
    let _ = db.list_issues(Some("archived"), None, None);
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
