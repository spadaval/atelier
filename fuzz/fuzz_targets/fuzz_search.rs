#![no_main]

use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

use atelier_sqlite::{ProjectionIndex, ProjectionIssue};

fuzz_target!(|query: String| {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(_) => return,
    };
    let db_path = dir.path().join("state.db");

    let db = match ProjectionIndex::open(&db_path) {
        Ok(d) => d,
        Err(_) => return,
    };

    let _ = db.insert_issue(&fuzz_issue(
        "atelier-fuzz-a",
        "Test issue one",
        Some("Description here".to_string()),
        "medium",
    ));
    let _ = db.insert_issue(&fuzz_issue("atelier-fuzz-b", "Another test", None, "high"));
    let _ = db.insert_issue(&fuzz_issue(
        "atelier-fuzz-c",
        "Third issue",
        Some("More content".to_string()),
        "low",
    ));

    // Fuzz lookup/listing paths with arbitrary input.
    let _ = db.get_issue(&query);
    let _ = db.list_issues(None, None);
});

fn fuzz_issue(id: &str, title: &str, description: Option<String>, priority: &str) -> ProjectionIssue {
    ProjectionIssue::new(id, title, description, priority)
}
