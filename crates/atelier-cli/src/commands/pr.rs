use std::collections::BTreeSet;
use std::env;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, bail, Context, Result};
use atelier_app::forgejo::{
    ForgejoClient, ForgejoPullRequest, ForgejoReviewComment, ForgejoTransport, ReviewEvent,
    UreqForgejoTransport,
};
use atelier_app::project_config::{ForgejoConfig, ProjectConfig};
use atelier_app::workflow_policy::{self, FORGE_PR_FIELD};
use atelier_records::{issue_record_path, RecordStore};
use atelier_sqlite::Database;
use serde_json::{json, Value};

use crate::commands::activity_log;
use crate::commands::agent_factory::resolve_id;

pub fn open(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: Option<&str>,
    role: &str,
    title: &str,
    body: &str,
    source_branch: &str,
    target_branch: &str,
) -> Result<()> {
    let issue_id = infer_issue_id(db, state_dir, repo_root, issue_ref)?;
    ensure_no_linked_forge_pr(db, repo_root, &issue_id)?;
    let config_path = repo_root.join(".atelier/config.toml");
    let config = ProjectConfig::load(repo_root)?;
    let forgejo = config.require_forgejo(&config_path)?.clone();
    let token = env::var(&forgejo.admin_token_env).with_context(|| {
        format!(
            "forgejo_config_missing_token: environment variable {} is required for `atelier pr open`",
            forgejo.admin_token_env
        )
    })?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let pull = client.open_pull(role, title, body, source_branch, target_branch)?;
    let owner_id = persist_forge_pr(db, state_dir, db_path, &issue_id, &forgejo, &pull)?;
    record_pr_action(
        repo_root,
        state_dir,
        db,
        &owner_id,
        role,
        "open",
        &forgejo,
        pull.number,
    )?;
    println!("PR:      {}", pull.url);
    println!("Issue:   {issue_id}");
    println!("Owner:   {owner_id}");
    println!("State:   {}", pull.state);
    Ok(())
}

pub fn status(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    issue_ref: Option<&str>,
) -> Result<()> {
    let issue_id = infer_issue_id(db, state_dir, repo_root, issue_ref)?;
    let field = linked_forge_pr(db, &issue_id)?;
    println!("PR Status");
    println!("=========");
    print_forge_pr_summary(&issue_id, &field);
    Ok(())
}

pub fn show(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    issue_ref: Option<&str>,
) -> Result<()> {
    let issue_id = infer_issue_id(db, state_dir, repo_root, issue_ref)?;
    let field = linked_forge_pr(db, &issue_id)?;
    let forgejo = load_forgejo(repo_root)?;
    let token = env::var(&forgejo.admin_token_env).with_context(|| {
        format!(
            "forgejo_config_missing_token: environment variable {} is required for `atelier pr show`",
            forgejo.admin_token_env
        )
    })?;
    let number = forge_pr_number(&field)?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let pull = client.show_pull(number)?;
    println!("PR:      {}", pull.url);
    println!("Issue:   {issue_id}");
    println!("Number:  {}", pull.number);
    println!("State:   {}", pull.state);
    println!("Merged:  {}", pull.merged);
    Ok(())
}

pub fn merge(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: Option<&str>,
    role: &str,
) -> Result<()> {
    let issue_id = infer_issue_id(db, state_dir, repo_root, issue_ref)?;
    let field = linked_forge_pr(db, &issue_id)?;
    let forgejo = load_forgejo(repo_root)?;
    validate_linked_forge_pr(&field, &forgejo, &issue_id)?;
    let token = env::var(&forgejo.admin_token_env).with_context(|| {
        format!(
            "forgejo_config_missing_token: environment variable {} is required for `atelier pr merge`",
            forgejo.admin_token_env
        )
    })?;
    let number = forge_pr_number(&field)?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let (owner_id, pull) = merge_with_client(
        db, repo_root, state_dir, db_path, &issue_id, role, &forgejo, &client,
    )?;
    let action = "merge";
    record_pr_action(
        repo_root, state_dir, db, &owner_id, role, action, &forgejo, number,
    )?;
    println!("PR:      {}", pull.url);
    println!("Issue:   {issue_id}");
    println!("Owner:   {owner_id}");
    println!("State:   {}", pull.state);
    println!("Merged:  {}", pull.merged);
    println!("Next:    atelier issue transition {owner_id} --options");
    Ok(())
}

fn merge_with_client<T: ForgejoTransport>(
    db: &Database,
    _repo_root: &Path,
    state_dir: &Path,
    db_path: &Path,
    issue_id: &str,
    role: &str,
    forgejo: &ForgejoConfig,
    client: &ForgejoClient<T>,
) -> Result<(String, ForgejoPullRequest)> {
    let field = linked_forge_pr(db, issue_id)?;
    validate_linked_forge_pr(&field, forgejo, issue_id)?;
    let number = forge_pr_number(&field)?;
    let current = client.show_pull(number)?;
    validate_remote_pull_matches_linked_field(&current, &field, issue_id)?;
    let pull = if current.merged {
        current
    } else {
        client.merge_pull(role, number)?
    };
    if !pull.merged {
        bail!(
            "forge_pr_unmerged: Forgejo PR {} did not report merged after merge; inspect `atelier pr show --issue {}`",
            number,
            issue_id
        );
    }
    let owner_id = confirm_forge_pr_merged(db, state_dir, db_path, issue_id, &pull)?;
    Ok((owner_id, pull))
}

pub fn comments(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    issue_ref: Option<&str>,
    unresolved: bool,
) -> Result<()> {
    let issue_id = infer_issue_id(db, state_dir, repo_root, issue_ref)?;
    let field = linked_forge_pr(db, &issue_id)?;
    let forgejo = load_forgejo(repo_root)?;
    let token = env::var(&forgejo.admin_token_env).with_context(|| {
        format!(
            "forgejo_config_missing_token: environment variable {} is required for `atelier pr comments`",
            forgejo.admin_token_env
        )
    })?;
    let number = forge_pr_number(&field)?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    println!("PR Comments");
    println!("===========");
    let lines = render_comment_lines(client.review_comments(number)?, unresolved);
    if lines.is_empty() {
        println!("(none)");
        return Ok(());
    }
    for line in lines {
        println!("{line}");
    }
    Ok(())
}

pub fn comment(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    issue_ref: Option<&str>,
    role: &str,
    body: &str,
) -> Result<()> {
    let issue_id = infer_issue_id(db, state_dir, repo_root, issue_ref)?;
    let field = linked_forge_pr(db, &issue_id)?;
    let forgejo = load_forgejo(repo_root)?;
    let token = env::var(&forgejo.admin_token_env)?;
    let number = forge_pr_number(&field)?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let comment = client.comment_pull(role, number, body)?;
    let owner_id = branch_owner_id(db, repo_root, &issue_id)?;
    record_pr_action(
        repo_root, state_dir, db, &owner_id, role, "comment", &forgejo, number,
    )?;
    println!("Comment: {}", comment.id);
    println!("Issue:   {issue_id}");
    Ok(())
}

pub fn review(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    issue_ref: Option<&str>,
    role: &str,
    event: &str,
    body: &str,
) -> Result<()> {
    let issue_id = infer_issue_id(db, state_dir, repo_root, issue_ref)?;
    let field = linked_forge_pr(db, &issue_id)?;
    let forgejo = load_forgejo(repo_root)?;
    let token = env::var(&forgejo.admin_token_env)?;
    let number = forge_pr_number(&field)?;
    let event = parse_review_event(event)?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let review = client.review_pull(role, number, event, body)?;
    let owner_id = branch_owner_id(db, repo_root, &issue_id)?;
    record_pr_action(
        repo_root, state_dir, db, &owner_id, role, "review", &forgejo, number,
    )?;
    println!("Review: {}", review.id);
    println!("State:  {}", review.state);
    println!("Issue:  {issue_id}");
    Ok(())
}

pub fn persist_forge_pr(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    issue_id: &str,
    forgejo: &ForgejoConfig,
    pull: &ForgejoPullRequest,
) -> Result<String> {
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "cannot determine repository root for {}",
            state_dir.display()
        )
    })?;
    let policy = workflow_policy::load(repo_root)?;
    let resolution = workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?;
    let owner_id = resolution.owner_id;
    let value = json!({
        "provider": "forgejo",
        "host": forgejo.host,
        "owner": forgejo.owner,
        "repo": forgejo.repo,
        "number": pull.number,
        "url": pull.url,
        "source_branch": pull.source_branch,
        "target_branch": pull.target_branch,
    });
    let store = RecordStore::new(state_dir);
    let path = issue_record_path(&owner_id);
    let mut record = store.load_issue(&path)?;
    if let Some(existing) = record.issue.fields.get(FORGE_PR_FIELD) {
        if existing == &value {
            return Ok(owner_id);
        }
        bail!(
            "forge_pr_mismatch: issue {} already has a different forge_pr field; inspect `atelier pr status --issue {}` before replacing it",
            owner_id,
            owner_id
        );
    }
    record
        .issue
        .fields
        .insert(FORGE_PR_FIELD.to_string(), value);
    store.write_issue_atomic(&record)?;
    atelier_app::projection::refresh_after_canonical_write(state_dir, db_path)?;
    Ok(owner_id)
}

pub fn confirm_forge_pr_merged(
    db: &Database,
    state_dir: &Path,
    db_path: &Path,
    issue_id: &str,
    pull: &ForgejoPullRequest,
) -> Result<String> {
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "cannot determine repository root for {}",
            state_dir.display()
        )
    })?;
    let policy = workflow_policy::load(repo_root)?;
    let resolution = workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?;
    let owner_id = resolution.owner_id;
    let store = RecordStore::new(state_dir);
    let path = issue_record_path(&owner_id);
    let mut record = store.load_issue(&path)?;
    let field = record.issue.fields.get_mut(FORGE_PR_FIELD).ok_or_else(|| {
        anyhow!(
            "forge_pr_missing: issue {} has no linked forge_pr field; run `atelier pr open --issue {}` first",
            owner_id,
            owner_id
        )
    })?;
    validate_remote_pull_matches_linked_field(pull, field, &owner_id)?;
    if let Some(object) = field.as_object_mut() {
        object.insert("state".to_string(), json!(pull.state));
        object.insert("merged".to_string(), json!(true));
    }
    store.write_issue_atomic(&record)?;
    atelier_app::projection::refresh_after_canonical_write(state_dir, db_path)?;
    Ok(owner_id)
}

fn record_pr_action(
    repo_root: &Path,
    state_dir: &Path,
    db: &Database,
    issue_id: &str,
    role: &str,
    action: &str,
    forgejo: &ForgejoConfig,
    number: u64,
) -> Result<()> {
    let Some(role) = activity_log::attempt_role_from_cli(role) else {
        return Ok(());
    };
    let owner_id = branch_owner_id(db, repo_root, issue_id)?;
    let remote_author = forgejo.sudo_user_for_role(role.as_str()).ok();
    activity_log::record_pr_action_in_state_dir(
        state_dir,
        &owner_id,
        role,
        action,
        &forge_pr_identifier(forgejo, number),
        remote_author,
    )?;
    Ok(())
}

fn branch_owner_id(db: &Database, repo_root: &Path, issue_id: &str) -> Result<String> {
    let policy = workflow_policy::load(repo_root)?;
    Ok(workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?.owner_id)
}

fn forge_pr_identifier(forgejo: &ForgejoConfig, number: u64) -> String {
    format!("forgejo/{}/{}#{}", forgejo.owner, forgejo.repo, number)
}

fn load_forgejo(repo_root: &Path) -> Result<ForgejoConfig> {
    let config_path = repo_root.join(".atelier/config.toml");
    ProjectConfig::load(repo_root)?
        .require_forgejo(&config_path)
        .cloned()
}

fn linked_forge_pr(db: &Database, issue_id: &str) -> Result<Value> {
    workflow_policy::effective_forge_pr_field(db, issue_id)?.ok_or_else(|| {
        anyhow!(
            "forge_pr_missing: issue {} has no linked forge_pr field; run `atelier pr open --issue {}` first",
            issue_id,
            issue_id
        )
    })
}

fn forge_pr_number(value: &Value) -> Result<u64> {
    value.get("number").and_then(Value::as_u64).ok_or_else(|| {
        anyhow!("forge_pr_invalid: field forge_pr.number must be a positive integer")
    })
}

fn forge_pr_string<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("forge_pr_invalid: field forge_pr.{key} must be a non-empty string"))
}

fn validate_linked_forge_pr(value: &Value, forgejo: &ForgejoConfig, issue_id: &str) -> Result<()> {
    let provider = forge_pr_string(value, "provider")?;
    if provider != "forgejo" {
        bail!(
            "forge_pr_mismatch: linked forge_pr provider is '{}'; expected forgejo; run `atelier pr status --issue {}`",
            provider,
            issue_id
        );
    }
    let host = forge_pr_string(value, "host")?;
    let owner = forge_pr_string(value, "owner")?;
    let repo = forge_pr_string(value, "repo")?;
    if host != forgejo.host || owner != forgejo.owner || repo != forgejo.repo {
        bail!(
            "forge_pr_mismatch: linked forge_pr points to {}/{}/{}, but configured Forgejo repo is {}/{}/{}; run `atelier pr status --issue {}`",
            host,
            owner,
            repo,
            forgejo.host,
            forgejo.owner,
            forgejo.repo,
            issue_id
        );
    }
    Ok(())
}

fn validate_remote_pull_matches_linked_field(
    pull: &ForgejoPullRequest,
    value: &Value,
    issue_id: &str,
) -> Result<()> {
    let number = forge_pr_number(value)?;
    if pull.number != number {
        bail!(
            "forge_pr_mismatch: linked forge_pr number is {}, but Forgejo returned {}; run `atelier pr status --issue {}`",
            number,
            pull.number,
            issue_id
        );
    }
    let source_branch = forge_pr_string(value, "source_branch")?;
    let target_branch = forge_pr_string(value, "target_branch")?;
    if pull.source_branch != source_branch || pull.target_branch != target_branch {
        bail!(
            "forge_pr_mismatch: linked PR branches are {} -> {}, but forge_pr records {} -> {}; run `atelier pr status --issue {}`",
            pull.source_branch,
            pull.target_branch,
            source_branch,
            target_branch,
            issue_id
        );
    }
    Ok(())
}

fn print_forge_pr_summary(issue_id: &str, value: &Value) {
    println!("Issue:  {issue_id}");
    println!(
        "URL:    {}",
        value.get("url").and_then(Value::as_str).unwrap_or("")
    );
    println!(
        "Number: {}",
        value.get("number").and_then(Value::as_u64).unwrap_or(0)
    );
    println!(
        "Repo:   {}/{}",
        value.get("owner").and_then(Value::as_str).unwrap_or(""),
        value.get("repo").and_then(Value::as_str).unwrap_or("")
    );
    if let Some(merged) = value.get("merged").and_then(Value::as_bool) {
        println!("Merged: {merged}");
    }
}

fn infer_issue_id(
    db: &Database,
    _state_dir: &Path,
    repo_root: &Path,
    issue_ref: Option<&str>,
) -> Result<String> {
    if let Some(issue_ref) = issue_ref {
        return resolve_id(db, issue_ref);
    }
    if let Some(issue_id) = issue_from_current_linked_pr_branch(db, repo_root)? {
        return Ok(issue_id);
    }
    if let Some(issue_id) = issue_from_current_owner_branch(db, repo_root)? {
        return Ok(issue_id);
    }
    let active = db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| {
            issue.status == "in_progress"
                || issue.status == "review"
                || issue.status == "validation"
        })
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    match active.as_slice() {
        [one] => Ok(one.clone()),
        [] => bail!(
            "pr_target_missing: pass --issue <id>, run from a linked PR source branch, or run from an owner branch"
        ),
        _ => bail!(
            "pr_target_ambiguous: multiple active issues found ({}); pass --issue <id>",
            active.join(", ")
        ),
    }
}

fn ensure_no_linked_forge_pr(db: &Database, repo_root: &Path, issue_id: &str) -> Result<()> {
    let policy = workflow_policy::load(repo_root)?;
    let resolution = workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?;
    if workflow_policy::effective_forge_pr_field(db, issue_id)?.is_some() {
        bail!(
            "forge_pr_active: issue {} already has a linked forge_pr; inspect `atelier pr status --issue {}` before opening another PR",
            resolution.owner_id,
            resolution.owner_id
        );
    }
    Ok(())
}

fn issue_from_current_linked_pr_branch(db: &Database, repo_root: &Path) -> Result<Option<String>> {
    let branch = current_branch(repo_root)?;
    let policy = workflow_policy::load(repo_root)?;
    let mut owners = BTreeSet::new();
    for issue in db.list_issues(Some("all"), None, None)? {
        let Some(field) = workflow_policy::effective_forge_pr_field(db, &issue.id)? else {
            continue;
        };
        if field.get("source_branch").and_then(Value::as_str) == Some(branch.as_str()) {
            let resolution = workflow_policy::resolve_branch_lifecycle(&policy, db, &issue.id)?;
            owners.insert(resolution.owner_id);
        }
    }
    resolve_single_branch_target("linked PR source branch", &branch, owners)
}

fn issue_from_current_owner_branch(db: &Database, repo_root: &Path) -> Result<Option<String>> {
    let branch = current_branch(repo_root)?;
    let policy = workflow_policy::load(repo_root)?;
    let mut owners = BTreeSet::new();
    for issue in db.list_issues(Some("all"), None, None)? {
        if let Ok(resolution) = workflow_policy::resolve_branch_lifecycle(&policy, db, &issue.id) {
            if resolution.expected_branch == branch {
                owners.insert(resolution.owner_id);
            }
        }
    }
    resolve_single_branch_target("owner branch", &branch, owners)
}

fn resolve_single_branch_target(
    context: &str,
    branch: &str,
    owners: BTreeSet<String>,
) -> Result<Option<String>> {
    match owners.len() {
        0 => Ok(None),
        1 => Ok(owners.into_iter().next()),
        _ => bail!(
            "pr_target_ambiguous: current {} {} matches multiple owners ({}); pass --issue <id>",
            context,
            branch,
            owners.into_iter().collect::<Vec<_>>().join(", ")
        ),
    }
}

fn current_branch(repo_root: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["branch", "--show-current"])
        .output()
        .context("failed to inspect current git branch")?;
    if !output.status.success() {
        bail!("failed to inspect current git branch");
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn parse_review_event(value: &str) -> Result<ReviewEvent> {
    match value {
        "approve" | "APPROVE" => Ok(ReviewEvent::Approve),
        "request-changes" | "REQUEST_CHANGES" => Ok(ReviewEvent::RequestChanges),
        "comment" | "COMMENT" => Ok(ReviewEvent::Comment),
        other => bail!(
            "invalid review event '{}'; expected approve, request-changes, or comment",
            other
        ),
    }
}

fn render_comment_lines(comments: Vec<ForgejoReviewComment>, unresolved: bool) -> Vec<String> {
    comments
        .into_iter()
        .filter(|comment| !unresolved || !comment.resolved)
        .map(|comment| {
            let line = comment
                .line
                .map(|line| line.to_string())
                .unwrap_or_else(|| "-".to_string());
            format!(
                "{} {}:{} {}",
                comment.id,
                comment.path,
                line,
                if comment.resolved {
                    "resolved"
                } else {
                    "unresolved"
                }
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_app::forgejo::{ForgejoPullRequest, ForgejoRequest, ForgejoResponse};
    use atelier_app::project_config::{ForgejoConfig, ForgejoSudoUsers};
    use atelier_core::Issue;
    use atelier_records::activity::{list_issue_activities, ActivityAttemptRole};
    use chrono::Utc;
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    #[derive(Debug)]
    struct MockTransport {
        requests: RefCell<Vec<ForgejoRequest>>,
        responses: RefCell<Vec<ForgejoResponse>>,
    }

    impl MockTransport {
        fn new(responses: Vec<ForgejoResponse>) -> Self {
            Self {
                requests: RefCell::new(Vec::new()),
                responses: RefCell::new(responses.into_iter().rev().collect()),
            }
        }

        fn requests(&self) -> Vec<ForgejoRequest> {
            self.requests.borrow().clone()
        }
    }

    impl ForgejoTransport for &MockTransport {
        fn send(&self, request: ForgejoRequest) -> Result<ForgejoResponse> {
            self.requests.borrow_mut().push(request);
            self.responses
                .borrow_mut()
                .pop()
                .ok_or_else(|| anyhow!("missing mock response"))
        }
    }

    fn forgejo_config() -> ForgejoConfig {
        ForgejoConfig {
            host: "forge.example.test".to_string(),
            owner: "tools".to_string(),
            repo: "atelier".to_string(),
            admin_token_env: "FORGEJO_ADMIN_TOKEN".to_string(),
            sudo_users: ForgejoSudoUsers {
                worker: "worker".to_string(),
                reviewer: "reviewer".to_string(),
                validator: "validator".to_string(),
                manager: "manager".to_string(),
                admin: "admin".to_string(),
            },
        }
    }

    fn write_workflow(repo_root: &Path) {
        let workflow = atelier_app::workflow_policy::STARTER_POLICY_YAML
            .replace("schema_version: 1", "schema_version: 2")
            .replace("base_branch: main", "base_branch: master")
            + r#"
fields:
  forge_pr:
    type: object
    required: [provider, host, owner, repo, number, url, source_branch, target_branch]
"#;
        std::fs::create_dir_all(repo_root.join(".atelier")).unwrap();
        std::fs::write(repo_root.join(".atelier/workflow.yaml"), workflow).unwrap();
    }

    fn setup_repo_on_branch(branch: &str) -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        write_workflow(dir.path());
        assert!(Command::new("git")
            .args(["init", "-b", "master"])
            .current_dir(dir.path())
            .status()
            .unwrap()
            .success());
        if branch != "master" {
            assert!(Command::new("git")
                .args(["checkout", "-b", branch])
                .current_dir(dir.path())
                .status()
                .unwrap()
                .success());
        }
        dir
    }

    fn insert_issue(
        db: &Database,
        id: &str,
        issue_type: &str,
        status: &str,
        parent_id: Option<&str>,
        fields: BTreeMap<String, Value>,
    ) {
        let now = Utc::now();
        db.insert_issue_rebuild(&Issue {
            id: id.to_string(),
            title: id.to_string(),
            description: None,
            status: status.to_string(),
            issue_type: issue_type.to_string(),
            priority: "medium".to_string(),
            fields,
            parent_id: parent_id.map(str::to_string),
            created_at: now,
            updated_at: now,
            closed_at: None,
        })
        .unwrap();
    }

    fn forge_pr_fields(source_branch: &str, number: u64) -> BTreeMap<String, Value> {
        let mut fields = BTreeMap::new();
        fields.insert(
            FORGE_PR_FIELD.to_string(),
            json!({
                "provider": "forgejo",
                "host": "forge.example.test",
                "owner": "tools",
                "repo": "atelier",
                "number": number,
                "url": format!("https://forge.example.test/tools/atelier/pulls/{number}"),
                "source_branch": source_branch,
                "target_branch": "master",
            }),
        );
        fields
    }

    fn pull_response(number: u64, state: &str, merged: bool, source_branch: &str) -> String {
        format!(
            r#"{{"number":{number},"url":"https://forge.example.test/tools/atelier/pulls/{number}","state":"{state}","merged":{merged},"head":{{"ref":"{source_branch}"}},"base":{{"ref":"master"}}}}"#
        )
    }

    fn session_record(state_dir: &Path, target_id: &str) {
        let session_dir = state_dir.join("sessions");
        std::fs::create_dir_all(&session_dir).unwrap();
        std::fs::write(
            session_dir.join("atelier-session.md"),
            format!(
                r#"---
id: session-test
type: session
status: active
created_at: "2026-06-18T00:00:00Z"
updated_at: "2026-06-18T00:00:00Z"
target:
  kind: issue
  id: {target_id}
---
"#
            ),
        )
        .unwrap();
    }

    #[test]
    fn infer_issue_id_prefers_linked_pr_source_branch_over_active_work() {
        let dir = setup_repo_on_branch("codex/linked-pr");
        let db = Database::open(&dir.path().join(".atelier/runtime/state.db")).unwrap();
        insert_issue(
            &db,
            "atelier-epic",
            "epic",
            "todo",
            None,
            forge_pr_fields("codex/linked-pr", 42),
        );
        insert_issue(
            &db,
            "atelier-active",
            "feature",
            "in_progress",
            None,
            BTreeMap::new(),
        );

        let issue_id = infer_issue_id(&db, &dir.path().join(".atelier"), dir.path(), None).unwrap();

        assert_eq!(issue_id, "atelier-epic");
    }

    #[test]
    fn infer_issue_id_uses_owner_branch_before_active_work() {
        let dir = setup_repo_on_branch("epic/atelier-epic");
        let db = Database::open(&dir.path().join(".atelier/runtime/state.db")).unwrap();
        insert_issue(&db, "atelier-epic", "epic", "todo", None, BTreeMap::new());
        insert_issue(
            &db,
            "atelier-active",
            "feature",
            "in_progress",
            None,
            BTreeMap::new(),
        );

        let issue_id = infer_issue_id(&db, &dir.path().join(".atelier"), dir.path(), None).unwrap();

        assert_eq!(issue_id, "atelier-epic");
    }

    #[test]
    fn infer_issue_id_rejects_ambiguous_active_work() {
        let dir = setup_repo_on_branch("master");
        let db = Database::open(&dir.path().join(".atelier/runtime/state.db")).unwrap();
        insert_issue(
            &db,
            "atelier-one",
            "feature",
            "in_progress",
            None,
            BTreeMap::new(),
        );
        insert_issue(
            &db,
            "atelier-two",
            "feature",
            "review",
            None,
            BTreeMap::new(),
        );

        let error = infer_issue_id(&db, &dir.path().join(".atelier"), dir.path(), None)
            .unwrap_err()
            .to_string();

        assert!(error.contains("pr_target_ambiguous"));
        assert!(error.contains("atelier-one"));
        assert!(error.contains("atelier-two"));
    }

    #[test]
    fn infer_issue_id_rejects_missing_target() {
        let dir = setup_repo_on_branch("master");
        let db = Database::open(&dir.path().join(".atelier/runtime/state.db")).unwrap();
        insert_issue(
            &db,
            "atelier-waiting",
            "feature",
            "todo",
            None,
            BTreeMap::new(),
        );

        let error = infer_issue_id(&db, &dir.path().join(".atelier"), dir.path(), None)
            .unwrap_err()
            .to_string();

        assert!(error.contains("pr_target_missing"));
        assert!(error.contains("pass --issue <id>"));
    }

    #[test]
    fn infer_issue_id_does_not_use_active_session_target() {
        let dir = setup_repo_on_branch("master");
        let db = Database::open(&dir.path().join(".atelier/runtime/state.db")).unwrap();
        insert_issue(
            &db,
            "atelier-session",
            "feature",
            "todo",
            None,
            BTreeMap::new(),
        );
        insert_issue(
            &db,
            "atelier-active",
            "feature",
            "in_progress",
            None,
            BTreeMap::new(),
        );
        session_record(&dir.path().join(".atelier"), "atelier-session");

        let issue_id = infer_issue_id(&db, &dir.path().join(".atelier"), dir.path(), None).unwrap();

        assert_eq!(issue_id, "atelier-active");
    }

    #[test]
    fn ensure_no_linked_forge_pr_enforces_one_active_pr_per_owner() {
        let dir = setup_repo_on_branch("master");
        let db = Database::open(&dir.path().join(".atelier/runtime/state.db")).unwrap();
        insert_issue(
            &db,
            "atelier-epic",
            "epic",
            "todo",
            None,
            forge_pr_fields("codex/linked-pr", 42),
        );
        insert_issue(
            &db,
            "atelier-child",
            "feature",
            "in_progress",
            Some("atelier-epic"),
            BTreeMap::new(),
        );

        let error = ensure_no_linked_forge_pr(&db, dir.path(), "atelier-child")
            .unwrap_err()
            .to_string();

        assert!(error.contains("forge_pr_active"));
        assert!(error.contains("atelier-epic"));
    }

    #[test]
    fn persist_forge_pr_writes_owner_epic_field_and_child_inherits() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join(".atelier/runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        let epic = db
            .create_issue_with_type("Epic", None, "medium", "epic")
            .unwrap();
        let child = db.create_subissue(&epic, "Child", None, "medium").unwrap();
        atelier_app::export::run_canonical(&db, &dir.path().join(".atelier"), false).unwrap();
        let pull = ForgejoPullRequest {
            number: 42,
            url: "https://forge.example.test/tools/atelier/pulls/42".to_string(),
            state: "open".to_string(),
            merged: false,
            source_branch: "codex/work".to_string(),
            target_branch: "master".to_string(),
        };

        let owner = persist_forge_pr(
            &db,
            &dir.path().join(".atelier"),
            &db_path,
            &child,
            &forgejo_config(),
            &pull,
        )
        .unwrap();
        let refreshed = Database::open(&db_path).unwrap();
        let inherited = workflow_policy::effective_forge_pr_field(&refreshed, &child)
            .unwrap()
            .unwrap();

        assert_eq!(owner, epic);
        assert_eq!(inherited["number"], 42);
        assert_eq!(inherited["provider"], "forgejo");
    }

    #[test]
    fn pr_merge_merges_updates_forge_pr_records_attribution_and_preserves_status() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let db_path = state_dir.join("runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        insert_issue(
            &db,
            "atelier-epic",
            "epic",
            "in_progress",
            None,
            BTreeMap::new(),
        );
        insert_issue(
            &db,
            "atelier-child",
            "feature",
            "validation",
            Some("atelier-epic"),
            BTreeMap::new(),
        );
        atelier_app::export::run_canonical(&db, &state_dir, false).unwrap();
        let pull = ForgejoPullRequest {
            number: 42,
            url: "https://forge.example.test/tools/atelier/pulls/42".to_string(),
            state: "open".to_string(),
            merged: false,
            source_branch: "codex/work".to_string(),
            target_branch: "master".to_string(),
        };
        persist_forge_pr(
            &db,
            &state_dir,
            &db_path,
            "atelier-child",
            &forgejo_config(),
            &pull,
        )
        .unwrap();
        let merge_db = Database::open(&db_path).unwrap();
        let before_status = merge_db.get_issue("atelier-epic").unwrap().unwrap().status;
        let transport = MockTransport::new(vec![
            ForgejoResponse {
                status: 200,
                body: pull_response(42, "open", false, "codex/work"),
            },
            ForgejoResponse {
                status: 200,
                body: "{}".to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: pull_response(42, "closed", true, "codex/work"),
            },
        ]);
        let client = ForgejoClient::new(forgejo_config(), &transport);

        let (owner_id, merged_pull) = merge_with_client(
            &merge_db,
            dir.path(),
            &state_dir,
            &db_path,
            "atelier-child",
            "validator",
            &forgejo_config(),
            &client,
        )
        .unwrap();
        record_pr_action(
            dir.path(),
            &state_dir,
            &merge_db,
            &owner_id,
            "validator",
            "merge",
            &forgejo_config(),
            merged_pull.number,
        )
        .unwrap();

        assert_eq!(owner_id, "atelier-epic");
        assert!(merged_pull.merged);
        let refreshed = Database::open(&db_path).unwrap();
        let after = refreshed.get_issue("atelier-epic").unwrap().unwrap();
        assert_eq!(after.status, before_status);
        assert!(after.closed_at.is_none());
        let field = workflow_policy::effective_forge_pr_field(&refreshed, "atelier-child")
            .unwrap()
            .unwrap();
        assert_eq!(field["state"], "closed");
        assert_eq!(field["merged"], true);
        let activities = list_issue_activities(&state_dir, "atelier-epic").unwrap();
        assert_eq!(activities.len(), 1);
        assert_eq!(
            activities[0].attempt.as_ref().unwrap().role,
            ActivityAttemptRole::Validator
        );
        assert_eq!(
            activities[0].pr_attribution.as_ref().unwrap().action,
            "merge"
        );
        assert_eq!(
            activities[0]
                .pr_attribution
                .as_ref()
                .unwrap()
                .forge_pr
                .as_deref(),
            Some("forgejo/tools/atelier#42")
        );
        let requests = transport.requests();
        assert_eq!(requests[0].method, "GET");
        assert_eq!(requests[1].method, "POST");
        assert_eq!(
            requests[1].path,
            "/api/v1/repos/tools/atelier/pulls/42/merge"
        );
        assert_eq!(requests[2].method, "GET");
    }

    #[test]
    fn pr_merge_confirms_already_merged_without_posting_merge_again() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let db_path = state_dir.join("runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        insert_issue(
            &db,
            "atelier-issue",
            "feature",
            "validation",
            None,
            forge_pr_fields("codex/work", 42),
        );
        atelier_app::export::run_canonical(&db, &state_dir, false).unwrap();
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: pull_response(42, "closed", true, "codex/work"),
        }]);
        let client = ForgejoClient::new(forgejo_config(), &transport);

        let (owner_id, pull) = merge_with_client(
            &db,
            dir.path(),
            &state_dir,
            &db_path,
            "atelier-issue",
            "validator",
            &forgejo_config(),
            &client,
        )
        .unwrap();

        assert_eq!(owner_id, "atelier-issue");
        assert!(pull.merged);
        let requests = transport.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].method, "GET");
        let refreshed = Database::open(&db_path).unwrap();
        let field = workflow_policy::effective_forge_pr_field(&refreshed, "atelier-issue")
            .unwrap()
            .unwrap();
        assert_eq!(field["state"], "closed");
        assert_eq!(field["merged"], true);
    }

    #[test]
    fn pr_merge_rejects_missing_and_mismatched_pr_context() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let db_path = state_dir.join("runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        insert_issue(
            &db,
            "atelier-missing",
            "feature",
            "validation",
            None,
            BTreeMap::new(),
        );
        insert_issue(
            &db,
            "atelier-linked",
            "feature",
            "validation",
            None,
            forge_pr_fields("codex/work", 42),
        );
        atelier_app::export::run_canonical(&db, &state_dir, false).unwrap();
        let empty_transport = MockTransport::new(Vec::new());
        let client = ForgejoClient::new(forgejo_config(), &empty_transport);

        let missing = merge_with_client(
            &db,
            dir.path(),
            &state_dir,
            &db_path,
            "atelier-missing",
            "validator",
            &forgejo_config(),
            &client,
        )
        .unwrap_err()
        .to_string();
        assert!(missing.contains("forge_pr_missing"));

        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: pull_response(42, "open", false, "codex/other"),
        }]);
        let client = ForgejoClient::new(forgejo_config(), &transport);
        let mismatch = merge_with_client(
            &db,
            dir.path(),
            &state_dir,
            &db_path,
            "atelier-linked",
            "validator",
            &forgejo_config(),
            &client,
        )
        .unwrap_err()
        .to_string();
        assert!(mismatch.contains("forge_pr_mismatch"));
        assert!(mismatch.contains("codex/other -> master"));
        assert_eq!(transport.requests().len(), 1);
    }

    #[test]
    fn record_pr_action_writes_owner_activity_with_remote_author_metadata() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join(".atelier/runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        let epic = db
            .create_issue_with_type("Epic", None, "medium", "epic")
            .unwrap();
        let child = db.create_subissue(&epic, "Child", None, "medium").unwrap();
        atelier_app::export::run_canonical(&db, &dir.path().join(".atelier"), false).unwrap();

        record_pr_action(
            dir.path(),
            &dir.path().join(".atelier"),
            &db,
            &child,
            "reviewer",
            "review",
            &forgejo_config(),
            42,
        )
        .unwrap();

        let owner_activities = list_issue_activities(&dir.path().join(".atelier"), &epic).unwrap();
        let child_activities = list_issue_activities(&dir.path().join(".atelier"), &child).unwrap();
        assert!(child_activities.is_empty());
        assert_eq!(owner_activities.len(), 1);
        let activity = &owner_activities[0];
        assert_eq!(
            activity.attempt.as_ref().unwrap().role,
            ActivityAttemptRole::Reviewer
        );
        assert_eq!(activity.pr_attribution.as_ref().unwrap().action, "review");
        assert_eq!(
            activity
                .pr_attribution
                .as_ref()
                .unwrap()
                .forge_pr
                .as_deref(),
            Some("forgejo/tools/atelier#42")
        );
        assert_eq!(
            activity
                .pr_attribution
                .as_ref()
                .unwrap()
                .remote_author
                .as_deref(),
            Some("reviewer")
        );
    }

    #[test]
    fn parse_review_event_rejects_unknown_values() {
        assert_eq!(parse_review_event("approve").unwrap(), ReviewEvent::Approve);
        let error = parse_review_event("merge").unwrap_err().to_string();
        assert!(error.contains("expected approve"));
    }

    #[test]
    fn render_comment_lines_filters_resolved_comments() {
        let lines = render_comment_lines(
            vec![
                ForgejoReviewComment {
                    id: 1,
                    path: "src/lib.rs".to_string(),
                    line: Some(10),
                    body: "fix".to_string(),
                    resolved: false,
                },
                ForgejoReviewComment {
                    id: 2,
                    path: "src/lib.rs".to_string(),
                    line: Some(12),
                    body: "done".to_string(),
                    resolved: true,
                },
            ],
            true,
        );

        assert_eq!(lines, vec!["1 src/lib.rs:10 unresolved"]);
    }
}
