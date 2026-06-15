#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

mod support;

use atelier_sqlite::ProjectionIndex;

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
    let conn = match support::open_projection(&dir.path().join("state.db")) {
        Ok(conn) => conn,
        Err(_) => return,
    };
    let projection = ProjectionIndex::new(&conn);

    let mut issue_ids: Vec<String> = Vec::new();

    for op in input.ops.iter().take(100) {
        match op {
            DependencyOp::CreateIssue { title } => {
                let id = format!("atelier-f{:04}", issue_ids.len());
                let title = support::bounded_text(title, 160, "Dependency fuzz issue");
                if support::insert_issue(&conn, &id, &title, "todo").is_ok() {
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
                    let _ = support::add_dependency(&conn, blocked, blocker);
                }
            }
            DependencyOp::RemoveDependency {
                blocked_idx,
                blocker_idx,
            } => {
                if issue_ids.len() >= 2 {
                    let blocked = &issue_ids[*blocked_idx % issue_ids.len()];
                    let blocker = &issue_ids[*blocker_idx % issue_ids.len()];
                    let _ = conn.execute(
                        "DELETE FROM dependencies WHERE blocked_id = ?1 AND blocker_id = ?2",
                        rusqlite::params![blocked, blocker],
                    );
                }
            }
            DependencyOp::CloseIssue { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = conn.execute(
                        "UPDATE issues SET status = 'done' WHERE id = ?1",
                        rusqlite::params![id],
                    );
                }
            }
            DependencyOp::ReopenIssue { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = conn.execute(
                        "UPDATE issues SET status = 'todo' WHERE id = ?1",
                        rusqlite::params![id],
                    );
                }
            }
            DependencyOp::CheckReady => {
                let _ = projection.ready_issues();
            }
            DependencyOp::CheckBlocked => {
                let _ = projection.blocked_issues();
            }
        }
    }

    let _ = projection.ready_issues();
    let _ = projection.blocked_issues();
    let _ = projection.list_issues(None, None, None);
});
