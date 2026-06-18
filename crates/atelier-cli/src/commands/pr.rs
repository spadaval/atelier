use std::collections::BTreeSet;
use std::env;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, bail, Context, Result};
use atelier_app::forgejo::{
    ForgejoClient, ForgejoPullRequest, ForgejoReviewComment, ReviewEvent, UreqForgejoTransport,
};
use atelier_app::project_config::{ForgejoConfig, ProjectConfig};
use atelier_app::workflow_policy::{self, FORGE_PR_FIELD};
use atelier_records::{issue_record_path, RecordStore};
use atelier_sqlite::Database;
use serde_json::{json, Value};

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
}

fn infer_issue_id(
    db: &Database,
    state_dir: &Path,
    repo_root: &Path,
    issue_ref: Option<&str>,
) -> Result<String> {
    if let Some(issue_ref) = issue_ref {
        return resolve_id(db, issue_ref);
    }
    if let Some(issue_id) = active_session_issue(state_dir)? {
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
        [] => bail!("pr_target_missing: pass --issue <id>, start a session linked to an issue, or run from an owner branch"),
        _ => bail!(
            "pr_target_ambiguous: multiple active issues found ({}); pass --issue <id>",
            active.join(", ")
        ),
    }
}

fn active_session_issue(state_dir: &Path) -> Result<Option<String>> {
    let sessions = RecordStore::new(state_dir).load_sessions()?;
    let active = sessions
        .into_iter()
        .filter(|session| session.header.status == "active")
        .filter_map(|session| session.data.target)
        .filter(|target| target.kind == "issue")
        .map(|target| target.id)
        .collect::<BTreeSet<_>>();
    if active.len() == 1 {
        Ok(active.into_iter().next())
    } else {
        Ok(None)
    }
}

fn issue_from_current_owner_branch(db: &Database, repo_root: &Path) -> Result<Option<String>> {
    let branch = current_branch()?;
    let policy = workflow_policy::load(repo_root)?;
    let mut owners = BTreeSet::new();
    for issue in db.list_issues(Some("all"), None, None)? {
        if let Ok(resolution) = workflow_policy::resolve_branch_lifecycle(&policy, db, &issue.id) {
            if resolution.expected_branch == branch {
                owners.insert(resolution.owner_id);
            }
        }
    }
    if owners.len() == 1 {
        Ok(owners.into_iter().next())
    } else {
        Ok(None)
    }
}

fn current_branch() -> Result<String> {
    let output = Command::new("git")
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
    use atelier_app::forgejo::ForgejoPullRequest;
    use atelier_app::project_config::{ForgejoConfig, ForgejoSudoUsers};
    use tempfile::tempdir;

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

    #[test]
    fn persist_forge_pr_writes_owner_epic_field_and_child_inherits() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join(".atelier/runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        let workflow = atelier_app::workflow_policy::STARTER_POLICY_YAML
            .replace("schema_version: 1", "schema_version: 2")
            .replace("base_branch: main", "base_branch: master")
            + r#"
fields:
  forge_pr:
    type: object
    required: [provider, host, owner, repo, number, url, source_branch, target_branch]
"#;
        std::fs::write(dir.path().join(".atelier/workflow.yaml"), workflow).unwrap();
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
