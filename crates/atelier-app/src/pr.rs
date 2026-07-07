use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, bail, Context, Result};
use atelier_core::IssueReview;
use atelier_records::activity::{
    create_issue_activity_with_metadata, ActivityEventType, ActivityPrAttribution,
};
use atelier_records::{issue_record_path, RecordStore};
use atelier_sqlite::Database;
use chrono::Utc;

use crate::forgejo::{
    ForgejoClient, ForgejoComment, ForgejoPullRequest, ForgejoReview, ForgejoReviewComment,
    ForgejoTransport, ReviewEvent,
};
use crate::project_config::{load_forgejo_with_workflow_role_authors, ForgejoConfig};
use crate::workflow_policy;

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
pub struct ReviewOpenContext {
    pub issue_id: String,
    pub owner_id: String,
    pub title: String,
    pub body: String,
    pub source_branch: String,
    pub target_branch: String,
}

pub fn derive_review_open_context(
    db: &Database,
    repo_root: &Path,
    issue_ref: Option<&str>,
) -> Result<ReviewOpenContext> {
    let issue_id = infer_issue_id(db, repo_root, issue_ref)?;
    let policy = workflow_policy::load(repo_root)?;
    let resolution = workflow_policy::resolve_branch_lifecycle(&policy, db, &issue_id)?;
    db.require_issue(&resolution.owner_id)?;
    let owner =
        RecordStore::new(repo_root.join(".atelier")).load_issue_by_id(&resolution.owner_id)?;
    let title = format!("{}: {}", owner.issue.id, owner.issue.title);
    let mut body = format!("Atelier review for {}.", owner.issue.id);
    let description = owner.sections.description.trim();
    if !description.is_empty() {
        body.push_str("\n\n");
        body.push_str(description);
    }
    body.push_str(&format!(
        "\n\nInspect with `atelier issue show {}`.",
        owner.issue.id
    ));
    Ok(ReviewOpenContext {
        issue_id,
        owner_id: owner.issue.id,
        title,
        body,
        source_branch: resolution.expected_branch,
        target_branch: resolution.base_branch,
    })
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
    load_forgejo_with_workflow_role_authors(repo_root)
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

pub fn infer_review_issue_id(
    db: &Database,
    repo_root: &Path,
    issue_ref: Option<&str>,
) -> Result<String> {
    infer_issue_id(db, repo_root, issue_ref)
}

pub fn review_owner_id(db: &Database, repo_root: &Path, issue_id: &str) -> Result<String> {
    branch_owner_id(db, repo_root, issue_id)
}

pub fn persist_pull_request(
    db: &Database,
    state_dir: &Path,
    _db_path: &Path,
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
    let review = IssueReview::ForgejoPullRequest {
        number: pull.number,
    };
    let store = RecordStore::new(state_dir);
    let path = issue_record_path(&owner_id);
    let mut record = store.load_issue(&path)?;
    if let Some(existing) = record.issue.review()? {
        if existing == review {
            return Ok(owner_id);
        }
        bail!(
            "pull_request_mismatch: issue {} already has a different review field; inspect `atelier review show --issue {}` before replacing it",
            owner_id,
            owner_id
        );
    }
    record.issue.set_review(review);
    store.write_issue_atomic(&record)?;
    Ok(owner_id)
}

pub fn confirm_pull_request_merged(
    db: &Database,
    state_dir: &Path,
    _db_path: &Path,
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
    let review = record.issue.review()?.ok_or_else(|| {
        anyhow!(
            "pull_request_missing: issue {} has no linked review field; run `atelier review open --issue {}` first",
            owner_id,
            owner_id
        )
    })?;
    let number = pull_request_number(&review)?;
    if pull.number != number {
        bail!(
            "pull_request_mismatch: linked pull_request number is {}, but Forgejo returned {}; run `atelier review show --issue {}`",
            number,
            pull.number,
            owner_id
        );
    }
    validate_remote_pull_matches_policy(db, repo_root, pull, &owner_id)?;
    store.write_issue_atomic(&record)?;
    Ok(owner_id)
}

pub fn linked_pull_request_merge_status_with_client<T: ForgejoTransport>(
    db: &Database,
    repo_root: &Path,
    issue_id: &str,
    client: &ForgejoClient<T>,
) -> Result<(bool, String)> {
    let Some(field) = workflow_policy::effective_pull_request_field(db, issue_id)? else {
        return Ok((
            false,
            format!("no linked review field; run `atelier review open --issue {issue_id}`"),
        ));
    };
    let review = provider_review_from_value(&field)?;
    let number = pull_request_number(&review)?;
    let pull = client.show_pull(number)?;
    let policy = workflow_policy::load(repo_root)?;
    let resolution = workflow_policy::resolve_branch_lifecycle(&policy, db, issue_id)?;
    if pull.source_branch != resolution.expected_branch
        || pull.target_branch != resolution.base_branch
    {
        return Ok((
            false,
            format!(
                "linked PR branches are {} -> {}, but issue {} expects {} -> {}; run `atelier review show --issue {}`",
                pull.source_branch,
                pull.target_branch,
                resolution.owner_id,
                resolution.expected_branch,
                resolution.base_branch,
                issue_id
            ),
        ));
    }
    if pull.merged {
        Ok((true, format!("linked PR {} is merged", pull.number)))
    } else {
        Ok((
            false,
            format!(
                "linked PR {} is {} and not merged; run `atelier review show --issue {}`",
                pull.number, pull.state, issue_id
            ),
        ))
    }
}

fn linked_pull_request(db: &Database, issue_id: &str) -> Result<IssueReview> {
    let field = workflow_policy::effective_pull_request_field(db, issue_id)?.ok_or_else(|| {
        anyhow!(
            "pull_request_missing: issue {} has no linked review field; run `atelier review open --issue {}` first",
            issue_id,
            issue_id
        )
    })?;
    provider_review_from_value(&field)
}

fn provider_review_from_value(value: &serde_json::Value) -> Result<IssueReview> {
    IssueReview::from_value(value)
        .map_err(|_| {
            anyhow!("pull_request_invalid: field review must be a provider pull_request object")
        })
        .and_then(|review| match review {
            IssueReview::ForgejoPullRequest { .. } => Ok(review),
            IssueReview::Room { .. } => Err(anyhow!(
                "pull_request_invalid: field review must be a provider pull_request object"
            )),
        })
}

fn pull_request_number(review: &IssueReview) -> Result<u64> {
    match review {
        IssueReview::ForgejoPullRequest { number } => Ok(*number),
        IssueReview::Room { .. } => Err(anyhow!(
            "pull_request_invalid: field review must be a provider pull_request object"
        )),
    }
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
            "pull_request_mismatch: linked PR branches are {} -> {}, but issue {} expects {} -> {}; run `atelier review show --issue {}`",
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
            "pull_request_mismatch: requested PR branches are {} -> {}, but issue {} expects {} -> {}; `atelier review open --issue {}` derives these branches from workflow state",
            source_branch,
            target_branch,
            resolution.owner_id,
            resolution.expected_branch,
            resolution.base_branch,
            resolution.owner_id
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
            "pull_request_active: issue {} already has a linked review artifact; inspect `atelier review show --issue {}` before opening another review",
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
    let owner_id = branch_owner_id(db, repo_root, issue_id)?;
    let remote_author = forgejo.role_author_for_role(role).ok();
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
    role: &str,
    action: &str,
    pull_request: &str,
    remote_author: Option<&str>,
) -> Result<()> {
    create_issue_activity_with_metadata(
        state_dir,
        issue_id,
        ActivityEventType::Comment,
        &current_actor(),
        Utc::now(),
        &format!("Recorded PR {action}"),
        Some(ActivityPrAttribution {
            action: action.to_string(),
            role: role.to_string(),
            pull_request: Some(pull_request.to_string()),
            remote_author: remote_author.map(str::to_string),
        }),
        &pr_action_body(role, action, pull_request, remote_author),
    )?;
    Ok(())
}

fn current_actor() -> String {
    std::env::var("ATELIER_AGENT")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "agent".to_string())
}

fn pr_action_body(
    role: &str,
    action: &str,
    pull_request: &str,
    remote_author: Option<&str>,
) -> String {
    format!(
        "role: {}\naction: {}\npull_request: {}\nremote_author: {}",
        scalar(role),
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
    use crate::workflow_policy::REVIEW_FIELD;
    use atelier_core::Issue;
    use atelier_records::activity::list_issue_activities;
    use atelier_records::{
        relationship_target, CanonicalIssueRecord, IssueSections, Relationships,
    };
    use atelier_sqlite::{IssueCacheRow, RecordSourceCacheRow};
    use chrono::Utc;
    use serde_json::Value;
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
            .replace("base_branch: main", "base_branch: master")
            .replace(
                "          - review.open: { role: worker }",
                "          - review.open:\n              provider: forgejo\n              role: worker\n              role_authors:\n                worker: worker\n                reviewer: reviewer\n                validator: validator\n                manager: manager",
            );
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

    fn fixture_issue(
        id: &str,
        issue_type: &str,
        status: &str,
        parent_id: Option<&str>,
        fields: BTreeMap<String, Value>,
    ) -> Issue {
        let now = Utc::now();
        Issue {
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
        }
    }

    fn insert_issue(
        db: &Database,
        id: &str,
        issue_type: &str,
        status: &str,
        parent_id: Option<&str>,
        fields: BTreeMap<String, Value>,
    ) {
        index_fixture_issue(
            db,
            &fixture_issue(id, issue_type, status, parent_id, fields),
        );
    }

    fn index_fixture_issue(db: &Database, issue: &Issue) {
        db.index_issue(
            &IssueCacheRow {
                id: issue.id.clone(),
                title: issue.title.clone(),
                status: issue.status.clone(),
                issue_type: issue.issue_type.clone(),
                priority: issue.priority.clone(),
                fields: issue.fields.clone(),
                parent_id: issue.parent_id.clone(),
                created_at: issue.created_at,
                updated_at: issue.updated_at,
                closed_at: issue.closed_at,
            },
            &[],
            &[],
            &[],
            &RecordSourceCacheRow {
                path: format!("issues/{}.md", issue.id),
                record_kind: "issue".to_string(),
                record_id: issue.id.clone(),
                size_bytes: 0,
                modified_micros: None,
                content_hash: None,
                indexed_at: Utc::now(),
            },
        )
        .unwrap();
    }

    fn insert_record_issue(
        db: &Database,
        state_dir: &Path,
        id: &str,
        issue_type: &str,
        status: &str,
        parent_id: Option<&str>,
        fields: BTreeMap<String, Value>,
    ) {
        let issue = fixture_issue(id, issue_type, status, parent_id, fields);
        RecordStore::new(state_dir)
            .write_issue_atomic(&CanonicalIssueRecord {
                issue: issue.clone(),
                labels: Vec::new(),
                sections: IssueSections::unchecked_from_body(Some(
                    "## Description\n\nFixture issue\n\n## Outcome\n\nFixture outcome.",
                )),
                relationships: Relationships::default(),
            })
            .unwrap();
        if let Some(parent_id) = parent_id {
            let store = RecordStore::new(state_dir);
            let parent_path = issue_record_path(parent_id);
            let mut parent = store.load_issue(&parent_path).unwrap();
            parent
                .relationships
                .children
                .push(relationship_target("issue", id));
            store.write_issue_atomic(&parent).unwrap();
        }
        index_fixture_issue(db, &issue);
    }

    fn pull_request_fields(number: u64) -> BTreeMap<String, Value> {
        let mut fields = BTreeMap::new();
        fields.insert(
            REVIEW_FIELD.to_string(),
            IssueReview::ForgejoPullRequest { number }.to_value(),
        );
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

    #[test]
    fn derived_open_context_uses_the_branch_owner_and_workflow_branches() {
        let dir = setup_repo_on_branch("master");
        let db = Database::open(&dir.path().join(".atelier/runtime/state.db")).unwrap();
        let state_dir = dir.path().join(".atelier");
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-epic",
            "epic",
            "review",
            None,
            BTreeMap::new(),
        );
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-child",
            "task",
            "in_progress",
            Some("atelier-epic"),
            BTreeMap::new(),
        );

        let context = derive_review_open_context(&db, dir.path(), Some("atelier-child")).unwrap();

        assert_eq!(context.issue_id, "atelier-child");
        assert_eq!(context.owner_id, "atelier-epic");
        assert_eq!(context.title, "atelier-epic: atelier-epic");
        assert_eq!(context.source_branch, "epic/atelier-epic");
        assert_eq!(context.target_branch, "master");
        assert!(context.body.contains("atelier issue show atelier-epic"));
    }

    fn review_response(id: u64, state: &str, body: &str) -> String {
        format!(
            r#"{{"id":{id},"state":"{state}","body":{}}}"#,
            serde_json::to_string(body).unwrap()
        )
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
        let state_dir = dir.path().join(".atelier");
        let epic = "atelier-epic".to_string();
        let child = "atelier-child".to_string();
        insert_record_issue(
            &db,
            &state_dir,
            &epic,
            "epic",
            "todo",
            None,
            BTreeMap::new(),
        );
        insert_record_issue(
            &db,
            &state_dir,
            &child,
            "task",
            "todo",
            Some(&epic),
            BTreeMap::new(),
        );
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
        drop(db);
        crate::rebuild::run(&dir.path().join(".atelier"), &db_path).unwrap();
        let refreshed = Database::open(&db_path).unwrap();
        let inherited = workflow_policy::effective_pull_request_field(&refreshed, &child)
            .unwrap()
            .unwrap();
        let owner_record = RecordStore::new(dir.path().join(".atelier"))
            .load_issue_by_id(&epic)
            .unwrap();

        assert_eq!(owner, epic);
        assert_eq!(
            owner_record.issue.review().unwrap(),
            Some(IssueReview::ForgejoPullRequest { number: 42 })
        );
        assert_eq!(
            inherited,
            IssueReview::ForgejoPullRequest { number: 42 }.to_value()
        );
    }

    #[test]
    fn pr_open_rejects_branch_mismatch_before_remote_create() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let db_path = state_dir.join("runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-issue",
            "feature",
            "in_progress",
            None,
            BTreeMap::new(),
        );
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
        assert!(error.contains("atelier-issue expects feature/atelier-issue -> master"));
        assert!(error.contains("atelier review open --issue atelier-issue"));
        assert!(error.contains("derives these branches from workflow state"));
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
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-issue",
            "feature",
            "in_progress",
            None,
            BTreeMap::new(),
        );
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 201,
            body: pull_response(42, "open", false, "feature/atelier-issue"),
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
                source_branch: "feature/atelier-issue",
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
        drop(db);
        crate::rebuild::run(&state_dir, &db_path).unwrap();
        let refreshed = Database::open(&db_path).unwrap();
        let field = workflow_policy::effective_pull_request_field(&refreshed, "atelier-issue")
            .unwrap()
            .unwrap();
        assert_eq!(
            field,
            IssueReview::ForgejoPullRequest { number: 42 }.to_value()
        );
        let activities = list_issue_activities(&state_dir, "atelier-issue").unwrap();
        assert_eq!(activities.len(), 1);
        assert_eq!(
            activities[0].pr_attribution.as_ref().unwrap().role,
            "worker"
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
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-issue",
            "feature",
            "in_progress",
            None,
            BTreeMap::new(),
        );
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: pull_response(42, "open", false, "feature/atelier-issue"),
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
        drop(db);
        crate::rebuild::run(&state_dir, &db_path).unwrap();
        let refreshed = Database::open(&db_path).unwrap();
        let field = workflow_policy::effective_pull_request_field(&refreshed, "atelier-issue")
            .unwrap()
            .unwrap();
        assert_eq!(
            field,
            IssueReview::ForgejoPullRequest { number: 42 }.to_value()
        );
    }

    #[test]
    fn pr_comment_posts_to_linked_pull_and_records_owner_action() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let db_path = state_dir.join("runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-issue",
            "feature",
            "review",
            None,
            pull_request_fields(42),
        );
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
            activities[0].pr_attribution.as_ref().unwrap().role,
            "reviewer"
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
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-issue",
            "feature",
            "review",
            None,
            pull_request_fields(42),
        );
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
            activities[0].pr_attribution.as_ref().unwrap().role,
            "reviewer"
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
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-epic",
            "epic",
            "in_progress",
            None,
            BTreeMap::new(),
        );
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-child",
            "feature",
            "validation",
            Some("atelier-epic"),
            BTreeMap::new(),
        );
        let pull = ForgejoPullRequest {
            number: 42,
            url: "https://forge.example.test/tools/atelier/pulls/42".to_string(),
            state: "open".to_string(),
            merged: false,
            source_branch: "epic/atelier-epic".to_string(),
            target_branch: "master".to_string(),
        };
        persist_pull_request(&db, &state_dir, &db_path, "atelier-child", &pull).unwrap();
        drop(db);
        crate::rebuild::run(&state_dir, &db_path).unwrap();
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
        assert_eq!(
            field,
            IssueReview::ForgejoPullRequest { number: 42 }.to_value()
        );
        let activities = list_issue_activities(&state_dir, "atelier-epic").unwrap();
        assert_eq!(activities.len(), 1);
        assert_eq!(
            activities[0].pr_attribution.as_ref().unwrap().role,
            "validator"
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
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-issue",
            "feature",
            "validation",
            None,
            pull_request_fields(42),
        );
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: pull_response(42, "closed", true, "feature/atelier-issue"),
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
        assert_eq!(
            field,
            IssueReview::ForgejoPullRequest { number: 42 }.to_value()
        );
    }

    #[test]
    fn pr_merge_rejects_missing_and_mismatched_pr_context() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let db_path = state_dir.join("runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-missing",
            "feature",
            "validation",
            None,
            BTreeMap::new(),
        );
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-linked",
            "feature",
            "validation",
            None,
            pull_request_fields(42),
        );
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
    fn linked_pull_request_merge_status_reports_required_states() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let db_path = state_dir.join("runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-hw9t",
            "epic",
            "validation",
            None,
            BTreeMap::new(),
        );
        let empty_transport = MockTransport::new(Vec::new());
        let empty_client = ForgejoClient::new(forgejo_config(), &empty_transport);

        let (passed, reason) = linked_pull_request_merge_status_with_client(
            &db,
            dir.path(),
            "atelier-hw9t",
            &empty_client,
        )
        .unwrap();
        assert!(!passed);
        assert!(reason.contains("atelier review open --issue atelier-hw9t"));

        insert_record_issue(
            &db,
            &state_dir,
            "atelier-linked",
            "epic",
            "validation",
            None,
            pull_request_fields(42),
        );

        let open_transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: pull_response(42, "open", false, "epic/atelier-linked"),
        }]);
        let open_client = ForgejoClient::new(forgejo_config(), &open_transport);
        let (passed, reason) = linked_pull_request_merge_status_with_client(
            &db,
            dir.path(),
            "atelier-linked",
            &open_client,
        )
        .unwrap();
        assert!(!passed);
        assert!(reason.contains("not merged"));
        assert!(reason.contains("atelier review show --issue atelier-linked"));

        let closed_transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: pull_response(42, "closed", false, "epic/atelier-linked"),
        }]);
        let closed_client = ForgejoClient::new(forgejo_config(), &closed_transport);
        let (passed, reason) = linked_pull_request_merge_status_with_client(
            &db,
            dir.path(),
            "atelier-linked",
            &closed_client,
        )
        .unwrap();
        assert!(!passed);
        assert!(reason.contains("closed and not merged"));

        let merged_transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: pull_response(42, "closed", true, "epic/atelier-linked"),
        }]);
        let merged_client = ForgejoClient::new(forgejo_config(), &merged_transport);
        let (passed, reason) = linked_pull_request_merge_status_with_client(
            &db,
            dir.path(),
            "atelier-linked",
            &merged_client,
        )
        .unwrap();
        assert!(passed);
        assert_eq!(reason, "linked PR 42 is merged");
    }

    #[test]
    fn linked_pull_request_merge_status_rejects_branch_mismatch() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let db_path = state_dir.join("runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        insert_record_issue(
            &db,
            &state_dir,
            "atelier-hw9t",
            "epic",
            "validation",
            None,
            pull_request_fields(42),
        );
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: pull_response(42, "closed", true, "epic/other"),
        }]);
        let client = ForgejoClient::new(forgejo_config(), &transport);

        let (passed, reason) =
            linked_pull_request_merge_status_with_client(&db, dir.path(), "atelier-hw9t", &client)
                .unwrap();

        assert!(!passed);
        assert!(reason.contains("linked PR branches"));
        assert!(reason.contains("atelier review show --issue atelier-hw9t"));
    }

    #[test]
    fn record_pr_action_writes_owner_activity_with_remote_author_metadata() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join(".atelier/runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        write_workflow(dir.path());
        let db = Database::open(&db_path).unwrap();
        let state_dir = dir.path().join(".atelier");
        let epic = "atelier-epic".to_string();
        let child = "atelier-child".to_string();
        insert_record_issue(
            &db,
            &state_dir,
            &epic,
            "epic",
            "todo",
            None,
            BTreeMap::new(),
        );
        insert_record_issue(
            &db,
            &state_dir,
            &child,
            "task",
            "todo",
            Some(&epic),
            BTreeMap::new(),
        );

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
        assert_eq!(activity.pr_attribution.as_ref().unwrap().role, "reviewer");
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
