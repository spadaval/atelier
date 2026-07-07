#![no_main]

use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

mod support;

fuzz_target!(|query: String| {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(_) => return,
    };
    let db_path = dir.path().join("state.db");

    let db = match support::open_cache(&db_path) {
        Some(db) => db,
        None => return,
    };

    support::index_issue(&db, "atelier-fuzza", "Test issue one", "todo", "medium");
    support::index_issue(&db, "atelier-fuzzb", "Another test", "todo", "high");
    support::index_issue(&db, "atelier-fuzzc", "Third issue", "todo", "low");

    // Fuzz lookup/listing paths with arbitrary input.
    let _ = db.get_issue(&query);
    let _ = db.search_issues(&query);
    let _ = db.list_issues(None, None, None);
});
