use atelier_workflow::{configured_initial_status, load, ValidatorParams, STARTER_POLICY_YAML};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

struct TestRepo {
    path: PathBuf,
}

impl TestRepo {
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn write_policy(text: &str) -> TestRepo {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "atelier-workflow-policy-{}-{nanos}",
        std::process::id()
    ));
    fs::create_dir_all(path.join(".atelier")).unwrap();
    fs::write(path.join(".atelier/workflow.yaml"), text).unwrap();
    let dir = TestRepo { path };
    dir
}

#[test]
fn starter_policy_exposes_task_transitions_and_categories() {
    let dir = write_policy(STARTER_POLICY_YAML);
    let policy = load(dir.path()).unwrap();
    let workflow = policy.workflow_for_issue_type("task").unwrap();
    let start = workflow.transitions.get("start").unwrap();
    let close = workflow.transitions.get("close").unwrap();

    assert_eq!(workflow.initial_status, "todo");
    assert_eq!(start.from, vec!["todo".to_string(), "blocked".to_string()]);
    assert_eq!(start.to, "in_progress");
    assert_eq!(close.required_fields, vec!["close_reason".to_string()]);
    assert_eq!(policy.status_category("in_progress"), Some("active"));
    assert!(policy.workflow_allows_status("task", "done").unwrap());
    assert!(!policy.workflow_allows_status("task", "review").unwrap());
    assert_eq!(
        configured_initial_status(dir.path(), "task").unwrap(),
        Some("todo".to_string())
    );
}

#[test]
fn evidence_validator_params_are_part_of_policy_contract() {
    let dir = write_policy(STARTER_POLICY_YAML);
    let policy = load(dir.path()).unwrap();
    let validator = policy.validators.get("proof_attached").unwrap();

    assert_eq!(validator.builtin, "evidence_attached");
    assert_eq!(
        validator.params,
        Some(ValidatorParams::EvidenceAttached {
            min_count: 1,
            kind: None,
        })
    );
}

#[test]
fn invalid_transition_statuses_are_rejected_when_loading_policy() {
    let invalid = STARTER_POLICY_YAML.replace("from: [todo, blocked]", "from: [done]");
    let dir = write_policy(&invalid);
    let error = load(dir.path()).unwrap_err().to_string();

    assert!(error.contains("workflow_config_invalid_transition"));
    assert!(error.contains("cannot leave terminal status 'done'"));
}
