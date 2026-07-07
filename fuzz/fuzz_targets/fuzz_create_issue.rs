#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

mod support;

#[derive(Arbitrary, Debug)]
struct CreateIssueInput {
    title: String,
    description: Option<String>,
    priority: String,
}

fuzz_target!(|input: CreateIssueInput| {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(_) => return,
    };
    let db_path = dir.path().join("state.db");

    let db = match support::open_cache(&db_path) {
        Some(db) => db,
        None => return,
    };

    let id = "atelier-fuzz";
    let _ = &input.description;
    support::index_issue(&db, id, &input.title, "todo", &input.priority);
    if db.get_issue(id).ok().flatten().is_some() {
        let _ = db.list_issues(None, None, None);
    }
});
