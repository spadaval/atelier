#![no_main]

use arbitrary::Arbitrary;
use atelier_records::{CreateIssueRecord, RecordStore};
use chrono::Utc;
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

mod support;

#[derive(Arbitrary, Debug, Clone)]
enum StateOp {
    CreateIssue { title: String, priority: String },
    CloseIssue { idx: usize },
    ReopenIssue { idx: usize },
    SetStatus { idx: usize, status: String },
    CreateMission { title: String },
    LinkIssueToMission { issue_idx: usize, mission_idx: usize },
    CheckWorkflow,
    ListIssues,
    LoadDomainRecord { mission_idx: usize },
}

#[derive(Arbitrary, Debug)]
struct StateMachineInput {
    ops: Vec<StateOp>,
}

fuzz_target!(|input: StateMachineInput| {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(_) => return,
    };
    let repo_root = dir.path();
    let state_dir = repo_root.join(".atelier");
    if std::fs::create_dir_all(&state_dir).is_err() {
        return;
    }
    if std::fs::write(
        state_dir.join("workflow.yaml"),
        atelier_workflow::STARTER_POLICY_YAML,
    )
    .is_err()
    {
        return;
    }

    let store = RecordStore::new(&state_dir);
    let policy = atelier_workflow::load(repo_root).ok();
    let mut issue_ids: Vec<String> = Vec::new();
    let mut mission_ids: Vec<String> = Vec::new();

    for op in input.ops.iter().take(100) {
        match op {
            StateOp::CreateIssue { title, priority } => {
                let title = support::bounded_text(title, 160, "State fuzz issue");
                if let Ok(record) = store.create_issue_record(CreateIssueRecord {
                    title: &title,
                    description: None,
                    priority: support::priority(priority),
                    issue_type: "task",
                    labels: &[],
                    status: "todo",
                    parent_id: None,
                }) {
                    issue_ids.push(record.issue.id);
                }
            }
            StateOp::CloseIssue { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = set_issue_status(&store, policy.as_ref(), repo_root, id, "done");
                }
            }
            StateOp::ReopenIssue { idx } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let _ = set_issue_status(&store, policy.as_ref(), repo_root, id, "todo");
                }
            }
            StateOp::SetStatus { idx, status } => {
                if !issue_ids.is_empty() {
                    let id = &issue_ids[*idx % issue_ids.len()];
                    let status = support::issue_status(status);
                    let _ = set_issue_status(&store, policy.as_ref(), repo_root, id, status);
                }
            }
            StateOp::CreateMission { title } => {
                let title = support::bounded_text(title, 160, "State fuzz mission");
                if let Ok(record) = store.create_domain_record("mission", &title, "ready", None, "{}")
                {
                    mission_ids.push(record.record.id);
                }
            }
            StateOp::LinkIssueToMission {
                issue_idx,
                mission_idx,
            } => {
                if !issue_ids.is_empty() && !mission_ids.is_empty() {
                    let issue_id = &issue_ids[*issue_idx % issue_ids.len()];
                    let mission_id = &mission_ids[*mission_idx % mission_ids.len()];
                    let _ = store.add_record_relationship(
                        "mission", mission_id, "issue", issue_id, "advances",
                    );
                }
            }
            StateOp::CheckWorkflow => {
                let issues = store
                    .load_issues()
                    .map(|records| records.into_iter().map(|record| record.issue));
                if let Ok(issues) = issues {
                    let _ = atelier_workflow::check_issues(repo_root, issues);
                }
            }
            StateOp::ListIssues => {
                let _ = store.load_issues();
            }
            StateOp::LoadDomainRecord { mission_idx } => {
                if !mission_ids.is_empty() {
                    let mission_id = &mission_ids[*mission_idx % mission_ids.len()];
                    let _ = store.load_domain_record_by_id("mission", mission_id);
                }
            }
        }
    }

    let _ = store.load_issues();
});

fn set_issue_status(
    store: &RecordStore,
    policy: Option<&atelier_workflow::WorkflowPolicy>,
    repo_root: &std::path::Path,
    id: &str,
    status: &str,
) -> anyhow::Result<()> {
    let mut record = store.load_issue_by_id(id)?;
    record.issue.status = status.to_string();
    record.issue.closed_at = (status == "done").then(Utc::now);
    record.issue.updated_at = Utc::now();
    if let Some(policy) = policy {
        atelier_workflow::validate_issue_against_policy(
            policy,
            &record.issue,
            &repo_root.join(atelier_workflow::WORKFLOW_POLICY_PATH),
        )?;
    }
    store.write_issue_atomic(&record)
}
