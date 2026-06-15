use atelier_sqlite::{
    snapshot_sources, FreshnessProblem, MissionWorkSummary, ProjectionIndex, WorkflowStatusRow,
};
use chrono::Utc;
use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn open_projection(path: &Path) -> Connection {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let conn = Connection::open(path).unwrap();
    ProjectionIndex::init_schema(&conn).unwrap();
    conn.execute_batch(
        r#"
        CREATE TABLE issues (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL,
            issue_type TEXT NOT NULL,
            priority TEXT NOT NULL,
            parent_id TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            closed_at TEXT
        );
        CREATE TABLE labels (issue_id TEXT NOT NULL, label TEXT NOT NULL);
        CREATE TABLE dependencies (blocker_id TEXT NOT NULL, blocked_id TEXT NOT NULL);
        CREATE TABLE comments (id INTEGER PRIMARY KEY AUTOINCREMENT, issue_id TEXT NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL);
        CREATE TABLE records (
            id TEXT PRIMARY KEY,
            kind TEXT NOT NULL,
            title TEXT NOT NULL,
            status TEXT NOT NULL,
            body TEXT,
            data_json TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE record_links (
            source_kind TEXT NOT NULL,
            source_id TEXT NOT NULL,
            target_kind TEXT NOT NULL,
            target_id TEXT NOT NULL,
            relation_type TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        "#,
    )
    .unwrap();
    conn
}

fn insert_issue(conn: &Connection, id: &str, title: &str, status: &str) {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO issues
         (id, title, description, status, issue_type, priority, parent_id, created_at, updated_at, closed_at)
         VALUES (?1, ?2, ?3, ?4, 'task', 'high', NULL, ?5, ?5, NULL)",
        params![id, title, format!("Body for {title}"), status, now],
    )
    .unwrap();
}

#[test]
fn source_snapshots_track_only_canonical_state_not_runtime_files() {
    let dir = tempdir().unwrap();
    let state_dir = dir.path().join(".atelier");
    fs::create_dir_all(state_dir.join("issues/atelier-0abc.activity")).unwrap();
    fs::create_dir_all(state_dir.join("runtime")).unwrap();
    fs::write(state_dir.join("issues/atelier-0abc.md"), "issue").unwrap();
    fs::write(
        state_dir.join("issues/atelier-0abc.activity/20260615T000000000000Z.md"),
        "activity",
    )
    .unwrap();
    fs::write(state_dir.join("runtime/state.db"), "runtime").unwrap();
    fs::write(state_dir.join("workflow.yaml"), "workflow").unwrap();

    let paths = snapshot_sources(&state_dir)
        .unwrap()
        .into_iter()
        .map(|entry| entry.path)
        .collect::<Vec<_>>();

    assert_eq!(
        paths,
        vec![
            "issues/atelier-0abc.md".to_string(),
            "workflow.yaml".to_string()
        ]
    );
}

#[test]
fn projection_freshness_reports_rebuild_boundary_changes() {
    let dir = tempdir().unwrap();
    let state_dir = dir.path().join(".atelier");
    fs::create_dir_all(state_dir.join("issues")).unwrap();
    fs::write(state_dir.join("issues/atelier-0abc.md"), "first").unwrap();
    fs::write(state_dir.join("workflow.yaml"), "workflow").unwrap();
    let conn = open_projection(&state_dir.join("runtime/state.db"));
    let projection = ProjectionIndex::new(&conn);

    let missing = projection.check_freshness(&state_dir).unwrap();
    assert!(missing
        .problems
        .iter()
        .all(|problem| matches!(problem, FreshnessProblem::MissingMetadata { .. })));

    projection.refresh_sources(&state_dir).unwrap();
    assert!(projection.check_freshness(&state_dir).unwrap().is_fresh());

    fs::write(state_dir.join("issues/atelier-0abc.md"), "changed").unwrap();
    fs::write(state_dir.join("issues/atelier-0def.md"), "new").unwrap();
    let stale = projection.check_freshness(&state_dir).unwrap();

    assert!(stale.problems.contains(&FreshnessProblem::ChangedSource {
        path: "issues/atelier-0abc.md".to_string(),
    }));
    assert!(stale.problems.contains(&FreshnessProblem::UnindexedSource {
        path: "issues/atelier-0def.md".to_string(),
    }));
}

#[test]
fn query_projection_covers_records_links_workflow_and_mission_summary() {
    let dir = tempdir().unwrap();
    let conn = open_projection(&dir.path().join("state.db"));
    let projection = ProjectionIndex::new(&conn);
    let now = Utc::now().to_rfc3339();

    insert_issue(&conn, "atelier-0aaa", "Ready work", "todo");
    insert_issue(&conn, "atelier-0bbb", "Blocked work", "todo");
    insert_issue(&conn, "atelier-0ccc", "Blocking work", "todo");
    insert_issue(&conn, "atelier-0ddd", "Done work", "done");
    conn.execute(
        "INSERT INTO dependencies (blocker_id, blocked_id) VALUES ('atelier-0ccc', 'atelier-0bbb')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO records (id, kind, title, status, body, data_json, created_at, updated_at)
         VALUES ('atelier-0mmm', 'mission', 'Mission', 'ready', NULL, '{}', ?1, ?1)",
        [&now],
    )
    .unwrap();
    for issue in [
        "atelier-0aaa",
        "atelier-0bbb",
        "atelier-0ccc",
        "atelier-0ddd",
    ] {
        conn.execute(
            "INSERT INTO record_links
             (source_kind, source_id, target_kind, target_id, relation_type, created_at)
             VALUES ('mission', 'atelier-0mmm', 'issue', ?1, 'advances', ?2)",
            params![issue, now],
        )
        .unwrap();
    }
    projection
        .replace_workflow_statuses(&[
            WorkflowStatusRow {
                status: "todo".to_string(),
                category: "todo".to_string(),
            },
            WorkflowStatusRow {
                status: "done".to_string(),
                category: "done".to_string(),
            },
        ])
        .unwrap();
    projection
        .replace_issue_search_text("atelier-0aaa", "domain search marker")
        .unwrap();

    assert_eq!(
        projection
            .ready_issues()
            .unwrap()
            .into_iter()
            .map(|issue| issue.id)
            .collect::<Vec<_>>(),
        vec!["atelier-0aaa".to_string(), "atelier-0ccc".to_string()]
    );
    assert_eq!(
        projection.search_issues("marker").unwrap()[0].id,
        "atelier-0aaa"
    );
    assert_eq!(
        projection.record_kind_for_id("atelier-0mmm").unwrap(),
        Some("mission".to_string())
    );
    assert_eq!(
        projection
            .record_links("mission", "atelier-0mmm")
            .unwrap()
            .len(),
        4
    );
    assert_eq!(
        projection.workflow_statuses().unwrap(),
        vec![
            WorkflowStatusRow {
                status: "done".to_string(),
                category: "done".to_string(),
            },
            WorkflowStatusRow {
                status: "todo".to_string(),
                category: "todo".to_string(),
            },
        ]
    );
    assert_eq!(
        projection.mission_work_summary("atelier-0mmm").unwrap(),
        MissionWorkSummary {
            mission_id: "atelier-0mmm".to_string(),
            ready: 2,
            blocked: 1,
            done: 1,
            backlog: 0,
        }
    );
}
