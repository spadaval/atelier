#![no_main]

use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

mod support;

use atelier_sqlite::ProjectionIndex;

fuzz_target!(|query: String| {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(_) => return,
    };
    let conn = match support::open_projection(&dir.path().join("state.db")) {
        Ok(conn) => conn,
        Err(_) => return,
    };
    let projection = ProjectionIndex::new(&conn);

    let _ = support::insert_issue(&conn, "atelier-0001", "Test issue one", "todo");
    let _ = support::insert_issue(&conn, "atelier-0002", "Another test", "in_progress");
    let _ = support::insert_issue(&conn, "atelier-0003", "Third issue", "done");
    let _ = projection.replace_issue_search_text("atelier-0001", "Outcome search marker");
    let _ = projection.replace_issue_search_text("atelier-0002", &query);

    let _ = projection.search_issues(&query);
    let _ = projection.list_issues(Some("all"), None, None);
    let _ = projection.list_issues(Some(support::issue_status(&query)), None, Some("medium"));
});
