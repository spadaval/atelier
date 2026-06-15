#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

use atelier_sqlite::{ProjectionIndex, ProjectionIssue};

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

    let db = match ProjectionIndex::open(&db_path) {
        Ok(d) => d,
        Err(_) => return,
    };

    let id = "atelier-fuzz";
    let issue = fuzz_issue(id, &input.title, input.description.clone(), &input.priority);
    let _ = db.insert_issue(&issue);
    if db.get_issue(id).ok().flatten().is_some() {
        let _ = db.list_issues(None, None);
    }
});

fn fuzz_issue(id: &str, title: &str, description: Option<String>, priority: &str) -> ProjectionIssue {
    ProjectionIssue::new(id, title, description, priority)
}
