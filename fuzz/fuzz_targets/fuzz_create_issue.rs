#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

use atelier::db::Database;
use atelier::models::Issue;

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

    let db = match Database::open(&db_path) {
        Ok(d) => d,
        Err(_) => return,
    };

    let id = "atelier-fuzz";
    let issue = fuzz_issue(id, &input.title, input.description.clone(), &input.priority);
    let _ = db.insert_issue_rebuild(&issue);
    if db.get_issue(id).ok().flatten().is_some() {
        let _ = db.list_issues(None, None, None);
    }
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
