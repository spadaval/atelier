#![no_main]

use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

use atelier::db::Database;
use atelier::models::Issue;

fuzz_target!(|query: String| {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(_) => return,
    };
    let db_path = dir.path().join("state.db");

    let db = match Database::open(&db_path) {
        Ok(d) => d,
        Err(_) => return,
    };

    let _ = db.insert_issue_rebuild(&fuzz_issue(
        "atelier-fuzz-a",
        "Test issue one",
        Some("Description here".to_string()),
        "medium",
    ));
    let _ = db.insert_issue_rebuild(&fuzz_issue("atelier-fuzz-b", "Another test", None, "high"));
    let _ = db.insert_issue_rebuild(&fuzz_issue(
        "atelier-fuzz-c",
        "Third issue",
        Some("More content".to_string()),
        "low",
    ));

    // Fuzz lookup/listing paths with arbitrary input.
    let _ = db.get_issue(&query);
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
