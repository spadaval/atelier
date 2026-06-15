#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Write;
use tempfile::tempdir;

mod support;

use atelier_records::{CreateIssueRecord, RecordStore};

fuzz_target!(|data: &[u8]| {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(_) => return,
    };
    let state_dir = dir.path().join(".atelier");
    let import_path = dir.path().join("import.json");

    let mut file = match std::fs::File::create(&import_path) {
        Ok(f) => f,
        Err(_) => return,
    };
    if file.write_all(data).is_err() {
        return;
    }
    drop(file);

    let store = RecordStore::new(&state_dir);

    if let Ok(content) = std::fs::read_to_string(&import_path) {
        #[derive(serde::Deserialize)]
        struct ExportData {
            #[serde(default)]
            issues: Vec<serde_json::Value>,
            #[serde(default)]
            records: Vec<serde_json::Value>,
        }

        if let Ok(export_data) = serde_json::from_str::<ExportData>(&content) {
            for issue in export_data.issues.into_iter().take(50) {
                if let Some(title) = issue.get("title").and_then(|t| t.as_str()) {
                    let desc = issue.get("description").and_then(|d| d.as_str());
                    let priority = issue
                        .get("priority")
                        .and_then(|p| p.as_str())
                        .unwrap_or("medium");
                    let title = support::bounded_text(title, 160, "Imported fuzz issue");
                    let _ = store.create_issue_record(CreateIssueRecord {
                        title: &title,
                        description: desc,
                        priority: support::priority(priority),
                        issue_type: "task",
                        labels: &[],
                        status: "todo",
                        parent_id: None,
                    });
                }
            }
            for record in export_data.records.into_iter().take(25) {
                let title = record
                    .get("title")
                    .and_then(|value| value.as_str())
                    .unwrap_or("Imported fuzz evidence");
                let body = record.get("body").and_then(|value| value.as_str());
                let title = support::bounded_text(title, 160, "Imported fuzz evidence");
                let data_json = record
                    .get("data_json")
                    .and_then(|value| value.as_str())
                    .unwrap_or("{}");
                let _ = store.create_domain_record("evidence", &title, "pass", body, data_json);
            }
        } else if let Ok(values) = serde_json::from_str::<Vec<serde_json::Value>>(&content) {
            for value in values.into_iter().take(50) {
                if let Some(title) = value.as_str() {
                    let title = support::bounded_text(title, 160, "Imported fuzz issue");
                    let _ = store.create_issue_record(CreateIssueRecord {
                        title: &title,
                        description: None,
                        priority: "medium",
                        issue_type: "task",
                        labels: &[],
                        status: "todo",
                        parent_id: None,
                    });
                }
            }
        }
    }

    let _ = store.load_issues();
});
