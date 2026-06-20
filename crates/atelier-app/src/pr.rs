use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, bail, Context, Result};
use atelier_records::activity::{
    create_issue_activity_with_metadata, list_issue_activities, ActivityAttemptLifecycle,
    ActivityAttemptMetadata, ActivityAttemptRole, ActivityEventType, ActivityPrAttribution,
};
use atelier_records::{issue_record_path, RecordStore};
use atelier_sqlite::Database;
use chrono::Utc;
use serde_json::{json, Value};

use crate::forgejo::{
    ForgejoClient, ForgejoComment, ForgejoPullRequest, ForgejoReview, ForgejoReviewComment,
    ForgejoTransport, ReviewEvent,
};
use crate::project_config::{ForgejoConfig, ProjectConfig};
use crate::workflow_policy::{self, REVIEW_FIELD};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrOpenRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub db_path: &'a Path,
    pub issue_ref: Option<&'a str>,
    pub role: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub source_branch: &'a str,
    pub target_branch: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrOpenOutcome {
    pub issue_id: String,
    pub owner_id: String,
    pub pull: ForgejoPullRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrLinkRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub db_path: &'a Path,
    pub issue_ref: Option<&'a str>,
    pub pull_request: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrLinkOutcome {
    pub issue_id: String,
    pub owner_id: String,
    pub pull: ForgejoPullRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrStatusRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub issue_ref: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrStatusOutcome {
    pub issue_id: String,
    pub number: u64,
    pub url: String,
    pub repo: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrShowRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub issue_ref: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrShowOutcome {
    pub issue_id: String,
    pub pull: ForgejoPullRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrCommentsRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub issue_ref: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrCommentsOutcome {
    pub issue_id: String,
    pub pull_comments: Vec<ForgejoComment>,
    pub review_comments: Vec<ForgejoReviewComment>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrCommentRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub issue_ref: Option<&'a str>,
    pub role: &'a str,
    pub body: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrCommentOutcome {
    pub issue_id: String,
    pub owner_id: String,
    pub comment: ForgejoComment,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrReviewRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub issue_ref: Option<&'a str>,
    pub role: &'a str,
    pub event: ReviewEvent,
    pub body: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrReviewOutcome {
    pub issue_id: String,
    pub owner_id: String,
    pub review: ForgejoReview,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrMergeRequest<'a> {
    pub repo_root: &'a Path,
    pub state_dir: &'a Path,
    pub db_path: &'a Path,
    pub issue_ref: Option<&'a str>,
    pub role: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrMergeOutcome {
    pub issue_id: String,
    pub owner_id: String,
    pub pull: ForgejoPullRequest,
}

pub fn open_with_client<T: ForgejoTransport>(
    db: &Database,
    request: PrOpenRequest<'_>,
    forgejo: &ForgejoConfig,
    client: &ForgejoClient<T>,
) -> Result<PrOpenOutcome> {
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    ensure_no_linked_pull_request(db, request.repo_root, &issue_id)?;
    validate_requested_pull_request_matches_policy(
        db,
        request.repo_root,
        &issue_id,
        request.source_branch,
        request.target_branch,
    )?;
    let pull = client.open_pull(
        request.role,
        request.title,
        request.body,
        request.source_branch,
        request.target_branch,
    )?;
    let owner_id = persist_pull_request(db, request.state_dir, request.db_path, &issue_id, &pull)?;
    record_pr_action(
        request.repo_root,
        request.state_dir,
        db,
        &owner_id,
        request.role,
        "open",
        forgejo,
        pull.number,
    )?;
    Ok(PrOpenOutcome {
        issue_id,
        owner_id,
        pull,
    })
}

pub fn link_with_client<T: ForgejoTransport>(
    db: &Database,
    request: PrLinkRequest<'_>,
    forgejo: &ForgejoConfig,
    client: &ForgejoClient<T>,
) -> Result<PrLinkOutcome> {
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    ensure_no_linked_pull_request(db, request.repo_root, &issue_id)?;
    let number = parse_pull_request_reference(request.pull_request, forgejo)?;
    let pull = client.show_pull(number)?;
    let owner_id = persist_pull_request(db, request.state_dir, request.db_path, &issue_id, &pull)?;
    Ok(PrLinkOutcome {
        issue_id,
        owner_id,
        pull,
    })
}

pub fn status(db: &Database, request: PrStatusRequest<'_>) -> Result<PrStatusOutcome> {
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let field = linked_pull_request(db, &issue_id)?;
    let forgejo = load_forgejo(request.repo_root)?;
    let number = pull_request_number(&field)?;
    Ok(PrStatusOutcome {
        issue_id,
        number,
        url: format!(
            "{}/{}/{}/pulls/{}",
            forgejo.host, forgejo.owner, forgejo.repo, number
        ),
        repo: format!("{}/{}", forgejo.owner, forgejo.repo),
    })
}

pub fn show_with_client<T: ForgejoTransport>(
    db: &Database,
    request: PrShowRequest<'_>,
    client: &ForgejoClient<T>,
) -> Result<PrShowOutcome> {
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let field = linked_pull_request(db, &issue_id)?;
    let pull = client.show_pull(pull_request_number(&field)?)?;
    Ok(PrShowOutcome { issue_id, pull })
}

pub fn comments_with_client<T: ForgejoTransport>(
    db: &Database,
    request: PrCommentsRequest<'_>,
    client: &ForgejoClient<T>,
) -> Result<PrCommentsOutcome> {
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let field = linked_pull_request(db, &issue_id)?;
    let number = pull_request_number(&field)?;
    Ok(PrCommentsOutcome {
        issue_id,
        pull_comments: client.pull_comments(number)?,
        review_comments: client.review_comments(number)?,
    })
}

pub fn comment_with_client<T: ForgejoTransport>(
    db: &Database,
    request: PrCommentRequest<'_>,
    forgejo: &ForgejoConfig,
    client: &ForgejoClient<T>,
) -> Result<PrCommentOutcome> {
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let field = linked_pull_request(db, &issue_id)?;
    let number = pull_request_number(&field)?;
    let comment = client.comment_pull(request.role, number, request.body)?;
    let owner_id = branch_owner_id(db, request.repo_root, &issue_id)?;
    record_pr_action(
        request.repo_root,
        request.state_dir,
        db,
        &owner_id,
        request.role,
        "comment",
        forgejo,
        number,
    )?;
    Ok(PrCommentOutcome {
        issue_id,
        owner_id,
        comment,
    })
}

pub fn review_with_client<T: ForgejoTransport>(
    db: &Database,
    request: PrReviewRequest<'_>,
    forgejo: &ForgejoConfig,
    client: &ForgejoClient<T>,
) -> Result<PrReviewOutcome> {
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let field = linked_pull_request(db, &issue_id)?;
    let number = pull_request_number(&field)?;
    let review = client.review_pull(request.role, number, request.event, request.body)?;
    let owner_id = branch_owner_id(db, request.repo_root, &issue_id)?;
    record_pr_action(
        request.repo_root,
        request.state_dir,
        db,
        &owner_id,
        request.role,
        "review",
        forgejo,
        number,
    )?;
    Ok(PrReviewOutcome {
        issue_id,
        owner_id,
        review,
    })
}

pub fn merge_with_client<T: ForgejoTransport>(
    db: &Database,
    request: PrMergeRequest<'_>,
    forgejo: &ForgejoConfig,
    client: &ForgejoClient<T>,
) -> Result<PrMergeOutcome> {
    let issue_id = infer_issue_id(db, request.repo_root, request.issue_ref)?;
    let field = linked_pull_request(db, &issue_id)?;
    let number = pull_request_number(&field)?;
    let current = client.show_pull(number)?;
    validate_remote_pull_matches_policy(db, request.repo_root, &current, &issue_id)?;
    let pull = if current.merged {
        current
    } else {
        client.merge_pull(request.role, number)?
    };
    if !pull.merged {
        bail!(
            "pull_request_unmerged: Forgejo PR {} did not report merged after merge; inspect `atelier review show --issue {}`",
            number,
            issue_id
        );
    }
    let owner_id =
        confirm_pull_request_merged(db, request.state_dir, request.db_path, &issue_id, &pull)?;
    record_pr_action(
        request.repo_root,
        request.state_dir,
        db,
        &owner_id,
        request.role,
        "merge",
        forgejo,
        number,
    )?;
    Ok(PrMergeOutcome {
        issue_id,
        owner_id,
        pull,
    })
}

pub fn load_forgejo(repo_root: &Path) -> Result<ForgejoConfig> {
    let config_path = repo_root.join(".atelier/config.toml");
    ProjectConfig::load(repo_root)?
        .require_forgejo(&config_path)
        .cloned()
}

pub fn parse_pull_request_reference(input: &str, forgejo: &ForgejoConfig) -> Result<u64> {
    let input = input.trim();
    if input.is_empty() {
        bail!("pull_request_invalid: PR reference must be a positive number or Forgejo PR URL");
    }
    if input.chars().all(|char| char.is_ascii_digit()) {
        return input
            .parse::<u64>()
            .ok()
            .filter(|number| *number > 0)
            .ok_or_else(|| anyhow!("pull_request_invalid: PR number must be positive"));
    }

    let path = pull_request_url_path(input, forgejo)?;
    let segments = path
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    match segments.as_slice() {
        [owner, repo, kind, number] if *owner == forgejo.owner && *repo == forgejo.repo => {
            if *kind != "pulls" && *kind != "pull" {
                bail!(
                    "pull_request_invalid: Forgejo PR URL must use /{}/{}/pulls/<number>",
                    forgejo.owner,
                    forgejo.repo
                );
            }
            number
                .parse::<u64>()
                .ok()
                .filter(|value| *value > 0)
                .ok_or_else(|| anyhow!("pull_request_invalid: PR URL number must be positive"))
        }
        _ => bail!(
            "pull_request_mismatch: PR URL must match configured Forgejo repo {}/{} at {}",
            forgejo.owner,
            forgejo.repo,
            forgejo.host.trim_end_matches('/')
        ),
    }
}

pub fn parse_review_event(value: &str) -> Result<ReviewEvent> {
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

pub fn persist_pull_request(
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
    if pull.source_branch != resolution.expected_branch
        || pull.target_branch != resolution.base_branch
    {
        bail!(
            "pull_request_mismatch: Forgejo PR branches are {} -> {}, but issue {} expects {} -> {}",
            pull.source_branch,
            pull.target_branch,
            owner_id,
            resolution.expected_branch,
            resolution.base_branch
        );
    }
    let value = pull_request_field_value(pull.number);
    let store = RecordStore::new(state_dir);
    let path = issue_record_path(&owner_id);
    let mut record = store.load_issue(&path)?;
    if let Some(existing) = record.issue.fields.get(REVIEW_FIELD) {
        if existing == &value {
            return Ok(owner_id);
        }
        bail!(
            "pull_request_mismatch: issue {} already has a different review field; inspect `atelier review status --issue {}` before replacing it",
            owner_id,
            owner_id
        );
    }
    record.issue.fields.insert(REVIEW_FIELD.to_string(), value);
    store.write_issue_atomic(&record)?;
    crate::projection::refresh_after_canonical_write(state_dir, db_path)?;
    Ok(owner_id)
}

pub fn confirm_pull_request_merged(
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
    let record = store.load_issue(&path)?;
    let field = record.issue.fields.get(REVIEW_FIELD).ok_or_else(|| {
        anyhow!(
            "pull_request_missing: issue {} has no linked review field; run `atelier review open --issue {}` first",
            owner_id,
            owner_id
        )
    })?;
    let number = pull_request_number(field)?;
    if pull.number != number {
        bail!(
            "pull_request_mismatch: linked pull_request number is {}, but Forgejo returned {}; run `atelier review status --issue {}`",
            number,
            pull.number,
            owner_id
        );
    }
    validate_remote_pull_matches_policy(db, repo_root, pull, &owner_id)?;
    store.write_issue_atomic(&record)?;
    crate::projection::refresh_after_canonical_write(state_dir, db_path)?;
    Ok(owner_id)
}

fn linked_pull_request(db: &Database, issue_id: &str) -> Result<Value> {
    workflow_policy::effective_pull_request_field(db, issue_id)?.ok_or_else(|| {
        anyhow!(
            "pull_request_missing: issue {} has no linked review field; run `atelier review open --issue {}` first",
            issue_id,
            issue_id
        )
    })
}

fn pull_request_number(value: &Value) -> Result<u64> {
    value
        .as_object()
        .filter(|object| {
            object.get("kind").and_then(Value::as_str) == Some("pull_request")
                && object.get("provider").and_then(Value::as_str) == Some("forgejo")
        })
        .and_then(|object| object.get("number"))
        .and_then(Value::as_u64)
        .filter(|number| *number > 0)
        .ok_or_else(|| {
            anyhow!("pull_request_invalid: field review must be a provider pull_request object")
        })
}

fn pull_request_field_value(number: u64) -> Value {
    json!({
        "kind": "pull_request",
        "provider": "forgejo",
        "number": number,
    })
}

fn pull_request_url_path<'a>(input: &'a str, forgejo: &ForgejoConfig) -> Result<&'a str> {
    let input = input.trim_end_matches('/');
    let host = forgejo.host.trim_end_matches('/');
    if let Some(path) = input.strip_prefix(&format!("{host}/")) {
        return Ok(path);
    }
    let host_without_scheme = host
        .strip_prefix("https://")
        .or_else(|| host.strip_prefix("http://"))
        .unwrap_or(host);
    for scheme in ["https://", "http://"] {
        if let Some(path) = input.strip_prefix(&format!("{scheme}{host_without_scheme}/")) {
            return Ok(path);
        }
    }
    bail!(
        "pull_request_mismatch: PR URL host must match configured Forgejo host {}",
        forgejo.host.trim_end_matches('/')
    )
}

fn validate_remote_pull_matches_policy(
    db: &Database,
    repo_root: &Path,
    pull: &ForgejoPullRequest,
    issue_id: &str,
) -> Result<()> {
    let policy = workflow_policy::load(repo_root)?;
    let resolution = workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?;
    if pull.source_branch != resolution.expected_branch
        || pull.target_branch != resolution.base_branch
    {
        bail!(
            "pull_request_mismatch: linked PR branches are {} -> {}, but issue {} expects {} -> {}; run `atelier review status --issue {}`",
            pull.source_branch,
            pull.target_branch,
            resolution.owner_id,
            resolution.expected_branch,
            resolution.base_branch,
            issue_id
        );
    }
    Ok(())
}

fn validate_requested_pull_request_matches_policy(
    db: &Database,
    repo_root: &Path,
    issue_id: &str,
    source_branch: &str,
    target_branch: &str,
) -> Result<()> {
    let policy = workflow_policy::load(repo_root)?;
    let resolution = workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?;
    if source_branch != resolution.expected_branch || target_branch != resolution.base_branch {
        bail!(
            "pull_request_mismatch: requested PR branches are {} -> {}, but issue {} expects {} -> {}; rerun `atelier review open --issue {} --source-branch {} --target-branch {}`",
            source_branch,
            target_branch,
            resolution.owner_id,
            resolution.expected_branch,
            resolution.base_branch,
            resolution.owner_id,
            resolution.expected_branch,
            resolution.base_branch
        );
    }
    Ok(())
}

fn infer_issue_id(db: &Database, repo_root: &Path, issue_ref: Option<&str>) -> Result<String> {
    if let Some(issue_ref) = issue_ref {
        return resolve_issue_ref(db, issue_ref);
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
        [] => bail!("pr_target_missing: pass --issue <id> or run from an owner branch"),
        _ => bail!(
            "pr_target_ambiguous: multiple active issues found ({}); pass --issue <id>",
            active.join(", ")
        ),
    }
}

fn resolve_issue_ref(db: &Database, issue_ref: &str) -> Result<String> {
    db.resolve_issue_ref(issue_ref)?
        .ok_or_else(|| anyhow!("Issue {issue_ref} was not found"))
}

fn ensure_no_linked_pull_request(db: &Database, repo_root: &Path, issue_id: &str) -> Result<()> {
    let policy = workflow_policy::load(repo_root)?;
    let resolution = workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?;
    if workflow_policy::effective_pull_request_field(db, issue_id)?.is_some() {
        bail!(
            "pull_request_active: issue {} already has a linked review artifact; inspect `atelier review status --issue {}` before opening another review",
            resolution.owner_id,
            resolution.owner_id
        );
    }
    Ok(())
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

fn branch_owner_id(db: &Database, repo_root: &Path, issue_id: &str) -> Result<String> {
    let policy = workflow_policy::load(repo_root)?;
    Ok(workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?.owner_id)
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
    let Some(role) = attempt_role_from_role_arg(role) else {
        return Ok(());
    };
    let owner_id = branch_owner_id(db, repo_root, issue_id)?;
    let remote_author = forgejo.role_author_for_role(role.as_str()).ok();
    let pull_request = format!("forgejo/{}/{}#{}", forgejo.owner, forgejo.repo, number);
    record_pr_action_in_state_dir(
        state_dir,
        &owner_id,
        role,
        action,
        &pull_request,
        remote_author,
    )
}

fn record_pr_action_in_state_dir(
    state_dir: &Path,
    issue_id: &str,
    role: ActivityAttemptRole,
    action: &str,
    pull_request: &str,
    remote_author: Option<&str>,
) -> Result<()> {
    let serial = current_attempt_serial_in_state_dir(state_dir, issue_id, role)?.unwrap_or(1);
    create_issue_activity_with_metadata(
        state_dir,
        issue_id,
        ActivityEventType::Comment,
        &current_actor(),
        Utc::now(),
        &format!("Recorded PR {action}"),
        Some(ActivityAttemptMetadata {
            role,
            serial,
            lifecycle: ActivityAttemptLifecycle::Updated,
            agent: current_agent(),
            subskill: current_subskill(),
        }),
        Some(ActivityPrAttribution {
            action: action.to_string(),
            pull_request: Some(pull_request.to_string()),
            remote_author: remote_author.map(str::to_string),
        }),
        &pr_action_body(role, action, pull_request, remote_author),
    )?;
    Ok(())
}

fn attempt_role_from_role_arg(role: &str) -> Option<ActivityAttemptRole> {
    match role {
        "worker" => Some(ActivityAttemptRole::Worker),
        "reviewer" => Some(ActivityAttemptRole::Reviewer),
        "validator" => Some(ActivityAttemptRole::Validator),
        _ => None,
    }
}

fn current_attempt_serial_in_state_dir(
    state_dir: &Path,
    issue_id: &str,
    role: ActivityAttemptRole,
) -> Result<Option<u32>> {
    Ok(list_issue_activities(state_dir, issue_id)?
        .into_iter()
        .filter_map(|activity| activity.attempt)
        .filter(|attempt| attempt.role == role)
        .map(|attempt| attempt.serial)
        .max())
}

fn current_actor() -> String {
    std::env::var("ATELIER_AGENT")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "agent".to_string())
}

fn current_agent() -> Option<String> {
    std::env::var("ATELIER_AGENT_ID")
        .or_else(|_| std::env::var("ATELIER_AGENT"))
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn current_subskill() -> Option<String> {
    std::env::var("ATELIER_SUBSKILL")
        .or_else(|_| std::env::var("ATELIER_AGENT_SUBSKILL"))
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn pr_action_body(
    role: ActivityAttemptRole,
    action: &str,
    pull_request: &str,
    remote_author: Option<&str>,
) -> String {
    format!(
        "role: {}\naction: {}\npull_request: {}\nremote_author: {}",
        scalar(role.as_str()),
        scalar(action),
        scalar(pull_request),
        option_scalar(remote_author)
    )
}

fn option_scalar(value: Option<&str>) -> String {
    value.map(scalar).unwrap_or_else(|| "null".to_string())
}

fn scalar(value: &str) -> String {
    serde_json::to_string(value).expect("string serialization cannot fail")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::forgejo::{ForgejoRequest, ForgejoResponse};
    use crate::project_config::ForgejoRoleAuthors;
    use atelier_core::Issue;
    use atelier_records::activity::ActivityAttemptRole;
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
            role_authors: Some(ForgejoRoleAuthors {
                worker: "worker".to_string(),
                reviewer: "reviewer".to_string(),
                validator: "validator".to_string(),
                manager: "manager".to_string(),
            }),
        }
    }

    fn write_workflow(repo_root: &Path) {
        let workflow = crate::workflow_policy::STARTER_POLICY_YAML
            .replace("base_branch: main", "base_branch: master");
        std::fs::create_dir_all(repo_root.join(".atelier")).unwrap();
        std::fs::write(repo_root.join(".atelier/workflow.yaml"), workflow).unwrap();
    }

    fn write_config(repo_root: &Path) {
        std::fs::create_dir_all(repo_root.join(".atelier")).unwrap();
        std::fs::write(
            repo_root.join(".atelier/config.toml"),
            r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier"

[paths]
state_root = ".atelier"
runtime_dir = ".atelier/runtime"
runtime_database = ".atelier/runtime/state.db"
cache_dir = ".atelier/cache"

[review]
mode = "provider"
provider = "forgejo"

[review.providers.forgejo]
host = "forge.example.test"
owner = "tools"
repo = "atelier"
admin_token_env = "FORGEJO_ADMIN_TOKEN"

[review.providers.forgejo.role_authors]
worker = "worker"
reviewer = "reviewer"
validator = "validator"
manager = "manager"
"#,
        )
        .unwrap();
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

    fn pull_request_fields(number: u64) -> BTreeMap<String, Value> {
        let mut fields = BTreeMap::new();
        fields.insert(REVIEW_FIELD.to_string(), pull_request_field_value(number));
        fields
    }

    fn pull_response(number: u64, state: &str, merged: bool, source_branch: &str) -> String {
        format!(
            r#"{{"number":{number},"url":"https://forge.example.test/tools/atelier/pulls/{number}","state":"{state}","merged":{merged},"head":{{"ref":"{source_branch}"}},"base":{{"ref":"master"}}}}"#
        )
    }

    fn comment_response(id: u64, body: &str) -> String {
        format!(
            r#"{{"id":{id},"body":{}}}"#,
            serde_json::to_string(body).unwrap()
        )
    }

    fn review_response(id: u64, state: &str, body: &str) -> String {
        format!(
            r#"{{"id":{id},"state":"{state}","body":{}}}"#,
            serde_json::to_string(body).unwrap()
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

        let issue_id = infer_issue_id(&db, dir.path(), None).unwrap();

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

        let error = infer_issue_id(&db, dir.path(), None)
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

        let error = infer_issue_id(&db, dir.path(), None)
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

        let issue_id = infer_issue_id(&db, dir.path(), None).unwrap();

        assert_eq!(issue_id, "atelier-active");
    }

    #[test]
    fn ensure_no_linked_pull_request_enforces_one_active_pr_per_owner() {
        let dir = setup_repo_on_branch("master");
        let db = Database::open(&dir.path().join(".atelier/runtime/state.db")).unwrap();
        insert_issue(
            &db,
            "atelier-epic",
            "epic",
            "todo",
            None,
            pull_request_fields(42),
        );
        insert_issue(
            &db,
            "atelier-child",
            "feature",
            "in_progress",
            Some("atelier-epic"),
            BTreeMap::new(),
        );

        let error = ensure_no_linked_pull_request(&db, dir.path(), "atelier-child")
            .unwrap_err()
            .to_string();

        assert!(error.contains("pull_request_active"));
        assert!(error.contains("atelier-epic"));
    }

    #[test]
    fn persist_pull_request_writes_owner_epic_field_and_child_inherits() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join(".atelier/runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        let epic = db
            .create_issue_with_type("Epic", None, "medium", "epic")
            .unwrap();
        let child = db.create_subissue(&epic, "Child", None, "medium").unwrap();
        crate::export::run_canonical(&db, &dir.path().join(".atelier"), false).unwrap();
        let pull = ForgejoPullRequest {
            number: 42,
            url: "https://forge.example.test/tools/atelier/pulls/42".to_string(),
            state: "open".to_string(),
            merged: false,
            source_branch: format!("epic/{epic}"),
            target_branch: "master".to_string(),
        };

        let owner =
            persist_pull_request(&db, &dir.path().join(".atelier"), &db_path, &child, &pull)
                .unwrap();
        let refreshed = Database::open(&db_path).unwrap();
        let inherited = workflow_policy::effective_pull_request_field(&refreshed, &child)
            .unwrap()
            .unwrap();

        assert_eq!(owner, epic);
        assert_eq!(inherited, pull_request_field_value(42));
    }

    #[test]
    fn pr_open_rejects_branch_mismatch_before_remote_create() {
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
            "in_progress",
            None,
            BTreeMap::new(),
        );
        crate::export::run_canonical(&db, &state_dir, false).unwrap();
        let transport = MockTransport::new(Vec::new());
        let client = ForgejoClient::new(forgejo_config(), &transport);

        let error = open_with_client(
            &db,
            PrOpenRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                role: "worker",
                title: "Title",
                body: "Body",
                source_branch: "codex/wrong",
                target_branch: "master",
            },
            &forgejo_config(),
            &client,
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("pull_request_mismatch"));
        assert!(error.contains("codex/wrong -> master"));
        assert!(error.contains("atelier-issue expects codex/atelier-issue -> master"));
        assert!(error.contains(
            "atelier review open --issue atelier-issue --source-branch codex/atelier-issue --target-branch master"
        ));
        assert!(transport.requests().is_empty());
        let refreshed = Database::open(&db_path).unwrap();
        assert!(
            workflow_policy::effective_pull_request_field(&refreshed, "atelier-issue")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn pr_open_persists_link_and_records_action_after_preflight() {
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
            "in_progress",
            None,
            BTreeMap::new(),
        );
        crate::export::run_canonical(&db, &state_dir, false).unwrap();
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 201,
            body: pull_response(42, "open", false, "codex/atelier-issue"),
        }]);
        let client = ForgejoClient::new(forgejo_config(), &transport);

        let outcome = open_with_client(
            &db,
            PrOpenRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                role: "worker",
                title: "Title",
                body: "Body",
                source_branch: "codex/atelier-issue",
                target_branch: "master",
            },
            &forgejo_config(),
            &client,
        )
        .unwrap();

        assert_eq!(outcome.issue_id, "atelier-issue");
        assert_eq!(outcome.owner_id, "atelier-issue");
        assert_eq!(outcome.pull.number, 42);
        let requests = transport.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].method, "POST");
        assert_eq!(requests[0].path, "/api/v1/repos/tools/atelier/pulls");
        let refreshed = Database::open(&db_path).unwrap();
        let field = workflow_policy::effective_pull_request_field(&refreshed, "atelier-issue")
            .unwrap()
            .unwrap();
        assert_eq!(field, pull_request_field_value(42));
        let activities = list_issue_activities(&state_dir, "atelier-issue").unwrap();
        assert_eq!(activities.len(), 1);
        assert_eq!(
            activities[0].attempt.as_ref().unwrap().role,
            ActivityAttemptRole::Worker
        );
        assert_eq!(
            activities[0].pr_attribution.as_ref().unwrap().action,
            "open"
        );
    }

    #[test]
    fn pr_link_fetches_remote_pull_and_persists_owner_field() {
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
            "in_progress",
            None,
            BTreeMap::new(),
        );
        crate::export::run_canonical(&db, &state_dir, false).unwrap();
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: pull_response(42, "open", false, "codex/atelier-issue"),
        }]);
        let client = ForgejoClient::new(forgejo_config(), &transport);

        let outcome = link_with_client(
            &db,
            PrLinkRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                pull_request: "https://forge.example.test/tools/atelier/pulls/42",
            },
            &forgejo_config(),
            &client,
        )
        .unwrap();

        assert_eq!(outcome.issue_id, "atelier-issue");
        assert_eq!(outcome.owner_id, "atelier-issue");
        assert_eq!(outcome.pull.number, 42);
        let requests = transport.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].method, "GET");
        assert_eq!(requests[0].path, "/api/v1/repos/tools/atelier/pulls/42");
        let refreshed = Database::open(&db_path).unwrap();
        let field = workflow_policy::effective_pull_request_field(&refreshed, "atelier-issue")
            .unwrap()
            .unwrap();
        assert_eq!(field, pull_request_field_value(42));
    }

    #[test]
    fn pr_comment_posts_to_linked_pull_and_records_owner_action() {
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
            "review",
            None,
            pull_request_fields(42),
        );
        crate::export::run_canonical(&db, &state_dir, false).unwrap();
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 201,
            body: comment_response(7, "Looks good"),
        }]);
        let client = ForgejoClient::new(forgejo_config(), &transport);

        let outcome = comment_with_client(
            &db,
            PrCommentRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                issue_ref: Some("atelier-issue"),
                role: "reviewer",
                body: "Looks good",
            },
            &forgejo_config(),
            &client,
        )
        .unwrap();

        assert_eq!(outcome.issue_id, "atelier-issue");
        assert_eq!(outcome.owner_id, "atelier-issue");
        assert_eq!(outcome.comment.id, 7);
        let requests = transport.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].method, "POST");
        assert_eq!(
            requests[0].path,
            "/api/v1/repos/tools/atelier/issues/42/comments"
        );
        let activities = list_issue_activities(&state_dir, "atelier-issue").unwrap();
        assert_eq!(activities.len(), 1);
        assert_eq!(
            activities[0].attempt.as_ref().unwrap().role,
            ActivityAttemptRole::Reviewer
        );
        assert_eq!(
            activities[0].pr_attribution.as_ref().unwrap().action,
            "comment"
        );
    }

    #[test]
    fn pr_review_posts_review_event_and_records_owner_action() {
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
            "review",
            None,
            pull_request_fields(42),
        );
        crate::export::run_canonical(&db, &state_dir, false).unwrap();
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: review_response(9, "APPROVED", "Approved"),
        }]);
        let client = ForgejoClient::new(forgejo_config(), &transport);

        let outcome = review_with_client(
            &db,
            PrReviewRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                issue_ref: Some("atelier-issue"),
                role: "reviewer",
                event: ReviewEvent::Approve,
                body: "Approved",
            },
            &forgejo_config(),
            &client,
        )
        .unwrap();

        assert_eq!(outcome.issue_id, "atelier-issue");
        assert_eq!(outcome.owner_id, "atelier-issue");
        assert_eq!(outcome.review.id, 9);
        assert_eq!(outcome.review.state, "APPROVED");
        let requests = transport.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].method, "POST");
        assert_eq!(
            requests[0].path,
            "/api/v1/repos/tools/atelier/pulls/42/reviews"
        );
        let activities = list_issue_activities(&state_dir, "atelier-issue").unwrap();
        assert_eq!(activities.len(), 1);
        assert_eq!(
            activities[0].attempt.as_ref().unwrap().role,
            ActivityAttemptRole::Reviewer
        );
        assert_eq!(
            activities[0].pr_attribution.as_ref().unwrap().action,
            "review"
        );
    }

    #[test]
    fn pr_merge_confirms_pull_request_attribution_and_preserves_status() {
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
        crate::export::run_canonical(&db, &state_dir, false).unwrap();
        let pull = ForgejoPullRequest {
            number: 42,
            url: "https://forge.example.test/tools/atelier/pulls/42".to_string(),
            state: "open".to_string(),
            merged: false,
            source_branch: "epic/atelier-epic".to_string(),
            target_branch: "master".to_string(),
        };
        persist_pull_request(&db, &state_dir, &db_path, "atelier-child", &pull).unwrap();
        let merge_db = Database::open(&db_path).unwrap();
        let before_status = merge_db.get_issue("atelier-epic").unwrap().unwrap().status;
        let transport = MockTransport::new(vec![
            ForgejoResponse {
                status: 200,
                body: pull_response(42, "open", false, "epic/atelier-epic"),
            },
            ForgejoResponse {
                status: 200,
                body: "{}".to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: pull_response(42, "closed", true, "epic/atelier-epic"),
            },
        ]);
        let client = ForgejoClient::new(forgejo_config(), &transport);

        let outcome = merge_with_client(
            &merge_db,
            PrMergeRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-child"),
                role: "validator",
            },
            &forgejo_config(),
            &client,
        )
        .unwrap();

        assert_eq!(outcome.issue_id, "atelier-child");
        assert_eq!(outcome.owner_id, "atelier-epic");
        assert!(outcome.pull.merged);
        let refreshed = Database::open(&db_path).unwrap();
        let after = refreshed.get_issue("atelier-epic").unwrap().unwrap();
        assert_eq!(after.status, before_status);
        assert!(after.closed_at.is_none());
        let field = workflow_policy::effective_pull_request_field(&refreshed, "atelier-child")
            .unwrap()
            .unwrap();
        assert_eq!(field, pull_request_field_value(42));
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
                .pull_request
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
            pull_request_fields(42),
        );
        crate::export::run_canonical(&db, &state_dir, false).unwrap();
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: pull_response(42, "closed", true, "codex/atelier-issue"),
        }]);
        let client = ForgejoClient::new(forgejo_config(), &transport);

        let outcome = merge_with_client(
            &db,
            PrMergeRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-issue"),
                role: "validator",
            },
            &forgejo_config(),
            &client,
        )
        .unwrap();

        assert_eq!(outcome.owner_id, "atelier-issue");
        assert!(outcome.pull.merged);
        let requests = transport.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].method, "GET");
        let refreshed = Database::open(&db_path).unwrap();
        let field = workflow_policy::effective_pull_request_field(&refreshed, "atelier-issue")
            .unwrap()
            .unwrap();
        assert_eq!(field, pull_request_field_value(42));
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
            pull_request_fields(42),
        );
        crate::export::run_canonical(&db, &state_dir, false).unwrap();
        let empty_transport = MockTransport::new(Vec::new());
        let client = ForgejoClient::new(forgejo_config(), &empty_transport);

        let missing = merge_with_client(
            &db,
            PrMergeRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-missing"),
                role: "validator",
            },
            &forgejo_config(),
            &client,
        )
        .unwrap_err()
        .to_string();
        assert!(missing.contains("pull_request_missing"));

        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: pull_response(42, "open", false, "codex/other"),
        }]);
        let client = ForgejoClient::new(forgejo_config(), &transport);
        let mismatch = merge_with_client(
            &db,
            PrMergeRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                db_path: &db_path,
                issue_ref: Some("atelier-linked"),
                role: "validator",
            },
            &forgejo_config(),
            &client,
        )
        .unwrap_err()
        .to_string();
        assert!(mismatch.contains("pull_request_mismatch"));
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
        crate::export::run_canonical(&db, &dir.path().join(".atelier"), false).unwrap();

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
                .pull_request
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
    fn status_outcome_resolves_linked_pr_without_rendering() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let db_path = state_dir.join("runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        write_config(dir.path());
        let db = Database::open(&db_path).unwrap();
        insert_issue(
            &db,
            "atelier-issue",
            "feature",
            "review",
            None,
            pull_request_fields(42),
        );
        crate::export::run_canonical(&db, &state_dir, false).unwrap();

        let outcome = status(
            &db,
            PrStatusRequest {
                repo_root: dir.path(),
                state_dir: &state_dir,
                issue_ref: Some("atelier-issue"),
            },
        )
        .unwrap();

        assert_eq!(outcome.issue_id, "atelier-issue");
        assert_eq!(outcome.number, 42);
        assert_eq!(
            outcome.url,
            "forge.example.test/tools/atelier/pulls/42".to_string()
        );
        assert_eq!(outcome.repo, "tools/atelier");
    }

    #[test]
    fn parse_review_event_rejects_unknown_values() {
        assert_eq!(parse_review_event("approve").unwrap(), ReviewEvent::Approve);
        let error = parse_review_event("merge").unwrap_err().to_string();
        assert!(error.contains("expected approve"));
    }

    #[test]
    fn parse_pull_request_reference_accepts_number_and_matching_url() {
        let config = forgejo_config();

        assert_eq!(parse_pull_request_reference("42", &config).unwrap(), 42);
        assert_eq!(
            parse_pull_request_reference(
                "https://forge.example.test/tools/atelier/pulls/42",
                &config,
            )
            .unwrap(),
            42
        );
    }

    #[test]
    fn parse_pull_request_reference_rejects_mismatched_url_context() {
        let config = forgejo_config();

        let host = parse_pull_request_reference(
            "https://other.example.test/tools/atelier/pulls/42",
            &config,
        )
        .unwrap_err()
        .to_string();
        assert!(host.contains("configured Forgejo host"));

        let repo = parse_pull_request_reference(
            "https://forge.example.test/tools/other/pulls/42",
            &config,
        )
        .unwrap_err()
        .to_string();
        assert!(repo.contains("configured Forgejo repo tools/atelier"));
    }
}
