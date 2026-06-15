#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

mod support;

use atelier_records::{CreateIssueRecord, RecordStore, UpdateIssueRecord};

#[derive(Arbitrary, Debug)]
struct CreateIssueInput {
    title: String,
    description: Option<String>,
    priority: String,
    label: String,
    parent_title: String,
}

fuzz_target!(|input: CreateIssueInput| {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(_) => return,
    };
    let state_dir = dir.path().join(".atelier");
    let store = RecordStore::new(&state_dir);
    let title = support::bounded_text(&input.title, 160, "Fuzz issue");
    let parent_title = support::bounded_text(&input.parent_title, 160, "Fuzz parent");
    let priority = support::priority(&input.priority);
    let labels = vec![support::bounded_text(&input.label, 64, "fuzz")];

    let parent = store.create_issue_record(CreateIssueRecord {
        title: &parent_title,
        description: Some("Parent created by fuzz target"),
        priority: "medium",
        issue_type: "epic",
        labels: &[],
        status: "todo",
        parent_id: None,
    });

    let parent_id = parent.as_ref().ok().map(|record| record.issue.id.as_str());
    let issue = store.create_issue_record(CreateIssueRecord {
        title: &title,
        description: input.description.as_deref(),
        priority,
        issue_type: "task",
        labels: &labels,
        status: "todo",
        parent_id,
    });

    if let Ok(issue) = issue {
        let _ = store.load_issue_by_id(&issue.issue.id);
        let _ = store.update_issue_record(
            &issue.issue.id,
            UpdateIssueRecord {
                title: Some(&title),
                priority: Some(priority),
                add_labels: &labels,
                ..UpdateIssueRecord::default()
            },
        );
        let _ = store.load_issues();
    }
});
