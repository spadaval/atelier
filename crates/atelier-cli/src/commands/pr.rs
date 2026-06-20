use std::env;
use std::path::Path;

use anyhow::{bail, Context, Result};
use atelier_app::forgejo::{
    ForgejoClient, ForgejoComment, ForgejoReviewComment, UreqForgejoTransport,
};
use atelier_app::pr as app_pr;
use atelier_app::project_config::{ProjectConfig, ReviewConfig};
use atelier_app::review_room;
use atelier_sqlite::Database;
use atelier_workflow as workflow_policy;

pub fn open(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: Option<&str>,
    role: Option<&str>,
    title: &str,
    body: &str,
    source_branch: &str,
    target_branch: &str,
) -> Result<()> {
    let role = resolve_review_role(db, repo_root, issue_ref, role)?;
    if review_mode(repo_root)? == ReviewMode::Room {
        let outcome = review_room::open(
            db,
            review_room::RoomOpenRequest {
                repo_root,
                state_dir,
                db_path,
                issue_ref,
                role: role.role.as_str(),
                title,
                body,
                source_branch,
                target_branch,
            },
        )?;
        println!("Review: {}", outcome.review_id);
        println!("Issue:   {}", outcome.issue_id);
        println!("Owner:   {}", outcome.owner_id);
        println!("Role:    {} ({})", role.role, role.source);
        println!("State:   {}", outcome.status);
        return Ok(());
    }
    let forgejo = app_pr::load_forgejo(repo_root)?;
    let token = env::var(&forgejo.admin_token_env).with_context(|| {
        format!(
            "forgejo_config_missing_token: environment variable {} is required for `atelier review open`",
            forgejo.admin_token_env
        )
    })?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let outcome = app_pr::open_with_client(
        db,
        app_pr::PrOpenRequest {
            repo_root,
            state_dir,
            db_path,
            issue_ref,
            role: role.role.as_str(),
            title,
            body,
            source_branch,
            target_branch,
        },
        &forgejo,
        &client,
    )?;
    println!("Review: {}", outcome.pull.url);
    println!("Issue:   {}", outcome.issue_id);
    println!("Owner:   {}", outcome.owner_id);
    println!("Role:    {} ({})", role.role, role.source);
    println!("State:   {}", outcome.pull.state);
    Ok(())
}

pub fn link(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: Option<&str>,
    pull_request: &str,
) -> Result<()> {
    if review_mode(repo_root)? == ReviewMode::Room {
        bail!("review_mode_invalid: `atelier review link` is only available when review.mode = \"provider\"");
    }
    let forgejo = app_pr::load_forgejo(repo_root)?;
    let token = env::var(&forgejo.admin_token_env).with_context(|| {
        format!(
            "forgejo_config_missing_token: environment variable {} is required for `atelier review link`",
            forgejo.admin_token_env
        )
    })?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let outcome = app_pr::link_with_client(
        db,
        app_pr::PrLinkRequest {
            repo_root,
            state_dir,
            db_path,
            issue_ref,
            pull_request,
        },
        &forgejo,
        &client,
    )?;
    println!("Review: {}", outcome.pull.url);
    println!("Number:  {}", outcome.pull.number);
    println!("Issue:   {}", outcome.issue_id);
    println!("Owner:   {}", outcome.owner_id);
    Ok(())
}

pub fn status(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    issue_ref: Option<&str>,
) -> Result<()> {
    if review_mode(repo_root)? == ReviewMode::Room {
        let outcome = review_room::status(
            db,
            review_room::RoomStatusRequest {
                repo_root,
                state_dir,
                issue_ref,
            },
        )?;
        println!("Review Status");
        println!("=============");
        println!("Issue:                {}", outcome.issue_id);
        println!("Room:                 {}", outcome.review_id);
        println!("State:                {}", outcome.status);
        println!("Current Approvals:    {}", outcome.approvals);
        println!("Unresolved Blocking:  {}", outcome.unresolved_blocking);
        println!("Unresolved Findings:  {}", outcome.unresolved_nonblocking);
        return Ok(());
    }
    let outcome = app_pr::status(
        db,
        app_pr::PrStatusRequest {
            repo_root,
            state_dir,
            issue_ref,
        },
    )?;
    println!("Review Status");
    println!("=============");
    println!("Issue:  {}", outcome.issue_id);
    println!("URL:    {}", outcome.url);
    println!("Number: {}", outcome.number);
    println!("Repo:   {}", outcome.repo);
    Ok(())
}

pub fn show(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    issue_ref: Option<&str>,
) -> Result<()> {
    if review_mode(repo_root)? == ReviewMode::Room {
        let outcome = review_room::show(
            db,
            review_room::RoomStatusRequest {
                repo_root,
                state_dir,
                issue_ref,
            },
        )?;
        println!("Review: {}", outcome.status.review_id);
        println!("Issue:   {}", outcome.status.issue_id);
        println!("Title:   {}", outcome.title);
        println!("State:   {}", outcome.status.status);
        println!(
            "Branch:  {} -> {}",
            outcome.source_branch, outcome.target_branch
        );
        println!("Events:");
        for event in outcome.events {
            println!(
                "  {} {}{}",
                event.id,
                event.kind,
                render_event_suffix(&event)
            );
        }
        return Ok(());
    }
    let forgejo = app_pr::load_forgejo(repo_root)?;
    let token = env::var(&forgejo.admin_token_env).with_context(|| {
        format!(
            "forgejo_config_missing_token: environment variable {} is required for `atelier review show`",
            forgejo.admin_token_env
        )
    })?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let outcome = app_pr::show_with_client(
        db,
        app_pr::PrShowRequest {
            repo_root,
            state_dir,
            issue_ref,
        },
        &client,
    )?;
    println!("Review: {}", outcome.pull.url);
    println!("Issue:   {}", outcome.issue_id);
    println!("Number:  {}", outcome.pull.number);
    println!("State:   {}", outcome.pull.state);
    println!("Merged:  {}", outcome.pull.merged);
    Ok(())
}

pub fn merge(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: Option<&str>,
    role: Option<&str>,
) -> Result<()> {
    let role = resolve_review_role(db, repo_root, issue_ref, role)?;
    if review_mode(repo_root)? == ReviewMode::Room {
        let outcome = review_room::merge(
            db,
            review_room::RoomMergeRequest {
                repo_root,
                state_dir,
                db_path,
                issue_ref,
                role: role.role.as_str(),
            },
        )?;
        println!("Review: {}", outcome.review_id);
        println!("Issue:   {}", outcome.issue_id);
        println!("Role:    {} ({})", role.role, role.source);
        println!("State:   {}", outcome.status);
        println!(
            "Next:    atelier issue transition {} --options",
            outcome.issue_id
        );
        return Ok(());
    }
    let forgejo = app_pr::load_forgejo(repo_root)?;
    let token = env::var(&forgejo.admin_token_env).with_context(|| {
        format!(
            "forgejo_config_missing_token: environment variable {} is required for `atelier review merge`",
            forgejo.admin_token_env
        )
    })?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let outcome = app_pr::merge_with_client(
        db,
        app_pr::PrMergeRequest {
            repo_root,
            state_dir,
            db_path,
            issue_ref,
            role: role.role.as_str(),
        },
        &forgejo,
        &client,
    )?;
    println!("Review: {}", outcome.pull.url);
    println!("Issue:   {}", outcome.issue_id);
    println!("Owner:   {}", outcome.owner_id);
    println!("Role:    {} ({})", role.role, role.source);
    println!("State:   {}", outcome.pull.state);
    println!("Merged:  {}", outcome.pull.merged);
    println!(
        "Next:    atelier issue transition {} --options",
        outcome.owner_id
    );
    Ok(())
}

pub fn comments(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    issue_ref: Option<&str>,
    unresolved: bool,
) -> Result<()> {
    if review_mode(repo_root)? == ReviewMode::Room {
        println!("Review Comments");
        println!("===============");
        let outcome = review_room::comments(
            db,
            review_room::RoomStatusRequest {
                repo_root,
                state_dir,
                issue_ref,
            },
        )?;
        let lines = outcome
            .into_iter()
            .filter(|event| !unresolved || event.kind == "finding")
            .map(|event| format!("{} {}{}", event.id, event.kind, render_event_suffix(&event)))
            .collect::<Vec<_>>();
        if lines.is_empty() {
            println!("(none)");
            return Ok(());
        }
        for line in lines {
            println!("{line}");
        }
        return Ok(());
    }
    let forgejo = app_pr::load_forgejo(repo_root)?;
    let token = env::var(&forgejo.admin_token_env).with_context(|| {
        format!(
            "forgejo_config_missing_token: environment variable {} is required for `atelier review comments`",
            forgejo.admin_token_env
        )
    })?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    println!("Review Comments");
    println!("===============");
    let outcome = app_pr::comments_with_client(
        db,
        app_pr::PrCommentsRequest {
            repo_root,
            state_dir,
            issue_ref,
        },
        &client,
    )?;
    let mut lines = render_pull_comment_lines(outcome.pull_comments);
    lines.extend(render_review_comment_lines(
        outcome.review_comments,
        unresolved,
    ));
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
    db_path: &Path,
    issue_ref: Option<&str>,
    role: Option<&str>,
    body: &str,
    finding: bool,
    severity: Option<&str>,
) -> Result<()> {
    let role = resolve_review_role(db, repo_root, issue_ref, role)?;
    if review_mode(repo_root)? == ReviewMode::Room {
        let outcome = review_room::comment(
            db,
            review_room::RoomCommentRequest {
                repo_root,
                state_dir,
                db_path,
                issue_ref,
                role: role.role.as_str(),
                body,
                finding,
                severity,
            },
        )?;
        println!("Comment: {}", outcome.event_id);
        println!("Issue:   {}", outcome.issue_id);
        println!("Review:  {}", outcome.review_id);
        println!("Role:    {} ({})", role.role, role.source);
        println!(
            "Next:    atelier review comments --issue {}",
            outcome.issue_id
        );
        return Ok(());
    }
    if finding || severity.is_some() {
        bail!("review_mode_invalid: --finding and --severity are only available for native review rooms");
    }
    let forgejo = app_pr::load_forgejo(repo_root)?;
    let token = env::var(&forgejo.admin_token_env)?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let outcome = app_pr::comment_with_client(
        db,
        app_pr::PrCommentRequest {
            repo_root,
            state_dir,
            issue_ref,
            role: role.role.as_str(),
            body,
        },
        &forgejo,
        &client,
    )?;
    println!("Comment: {}", outcome.comment.id);
    println!("Issue:   {}", outcome.issue_id);
    println!("Role:    {} ({})", role.role, role.source);
    println!(
        "Next:    atelier review comments --issue {}",
        outcome.issue_id
    );
    Ok(())
}

pub fn review(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: Option<&str>,
    role: Option<&str>,
    event: &str,
    body: &str,
) -> Result<()> {
    let role = resolve_review_role(db, repo_root, issue_ref, role)?;
    if review_mode(repo_root)? == ReviewMode::Room {
        let outcome = match event {
            "approve" => review_room::approve(
                db,
                review_room::RoomDecisionRequest {
                    repo_root,
                    state_dir,
                    db_path,
                    issue_ref,
                    role: role.role.as_str(),
                    body,
                },
            )?,
            "request-changes" => review_room::request_changes(
                db,
                review_room::RoomDecisionRequest {
                    repo_root,
                    state_dir,
                    db_path,
                    issue_ref,
                    role: role.role.as_str(),
                    body,
                },
            )?,
            _ => bail!(
                "review_room_invalid: unsupported room review event {}",
                event
            ),
        };
        println!("Review: {}", outcome.event_id);
        println!("State:  {}", outcome.status);
        println!("Issue:  {}", outcome.issue_id);
        println!("Role:   {} ({})", role.role, role.source);
        return Ok(());
    }
    let forgejo = app_pr::load_forgejo(repo_root)?;
    let token = env::var(&forgejo.admin_token_env)?;
    let event = app_pr::parse_review_event(event)?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let outcome = app_pr::review_with_client(
        db,
        app_pr::PrReviewRequest {
            repo_root,
            state_dir,
            issue_ref,
            role: role.role.as_str(),
            event,
            body,
        },
        &forgejo,
        &client,
    )?;
    println!("Review: {}", outcome.review.id);
    println!("State:  {}", outcome.review.state);
    println!("Issue:  {}", outcome.issue_id);
    println!("Role:   {} ({})", role.role, role.source);
    Ok(())
}

pub fn resolve(
    db: &Database,
    repo_root: &Path,
    state_dir: &Path,
    db_path: &Path,
    issue_ref: Option<&str>,
    finding: &str,
) -> Result<()> {
    let outcome = review_room::resolve(
        db,
        review_room::RoomResolveRequest {
            repo_root,
            state_dir,
            db_path,
            issue_ref,
            finding,
        },
    )?;
    println!("Resolved: {}", outcome.event_id);
    println!("Finding:  {}", finding);
    println!("Issue:    {}", outcome.issue_id);
    println!("Review:   {}", outcome.review_id);
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReviewMode {
    Room,
    Provider,
}

fn review_mode(repo_root: &Path) -> Result<ReviewMode> {
    match ProjectConfig::load(repo_root)?.review {
        ReviewConfig::Room => Ok(ReviewMode::Room),
        ReviewConfig::Provider(_) => Ok(ReviewMode::Provider),
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ResolvedReviewRole {
    role: String,
    source: String,
}

fn resolve_review_role(
    db: &Database,
    repo_root: &Path,
    issue_ref: Option<&str>,
    explicit_role: Option<&str>,
) -> Result<ResolvedReviewRole> {
    if let Some(role) = explicit_role {
        validate_review_role(role)?;
        return Ok(ResolvedReviewRole {
            role: role.to_string(),
            source: "explicit --role".to_string(),
        });
    }

    let issue_id = app_pr::infer_review_issue_id(db, repo_root, issue_ref)?;
    let owner_id = app_pr::review_owner_id(db, repo_root, &issue_id)?;
    let owner = db
        .get_issue(&owner_id)?
        .ok_or_else(|| anyhow::anyhow!("Issue {} was not found", owner_id))?;
    let policy = workflow_policy::load(repo_root)?;
    let Some(role) = policy.status_role(&owner.status) else {
        bail!(
            "review_role_missing: issue {} is in status '{}' and that status has no role; pass --role <worker|reviewer|validator|manager> or configure statuses.{}.role",
            owner.id,
            owner.status,
            owner.status
        );
    };
    Ok(ResolvedReviewRole {
        role: role.to_string(),
        source: format!("status {} ({})", owner.status, owner.id),
    })
}

fn validate_review_role(role: &str) -> Result<()> {
    if matches!(role, "worker" | "reviewer" | "validator" | "manager") {
        Ok(())
    } else {
        bail!(
            "review_role_invalid: role must be worker, reviewer, validator, or manager, got '{}'",
            role
        )
    }
}

fn render_event_suffix(event: &review_room::RoomEventView) -> String {
    let mut parts = Vec::new();
    if let Some(actor) = &event.actor {
        parts.push(format!("actor={actor}"));
    }
    if let Some(severity) = &event.severity {
        parts.push(format!("severity={severity}"));
    }
    if let Some(finding) = &event.finding {
        parts.push(format!("finding={finding}"));
    }
    if let Some(body) = &event.body {
        parts.push(single_line_body(body));
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!(" - {}", parts.join(" - "))
    }
}

fn render_pull_comment_lines(comments: Vec<ForgejoComment>) -> Vec<String> {
    comments
        .into_iter()
        .map(|comment| {
            format!(
                "comment {} - {}",
                comment.id,
                single_line_body(&comment.body)
            )
        })
        .collect()
}

fn render_review_comment_lines(
    comments: Vec<ForgejoReviewComment>,
    unresolved: bool,
) -> Vec<String> {
    comments
        .into_iter()
        .filter(|comment| !unresolved || !comment.resolved)
        .map(|comment| {
            let line = comment
                .line
                .map(|line| line.to_string())
                .unwrap_or_else(|| "-".to_string());
            format!(
                "review-comment {} {}:{} {}",
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

fn single_line_body(body: &str) -> String {
    let normalized = body.split_whitespace().collect::<Vec<_>>().join(" ");
    const LIMIT: usize = 80;
    if normalized.chars().count() <= LIMIT {
        normalized
    } else {
        let mut truncated = normalized.chars().take(LIMIT - 3).collect::<String>();
        truncated.push_str("...");
        truncated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_core::Issue;
    use atelier_sqlite::Database;
    use chrono::Utc;
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    fn setup_role_repo(status: &str) -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let db_path = state_dir.join("runtime/state.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        std::fs::write(
            state_dir.join("workflow.yaml"),
            atelier_workflow::STARTER_POLICY_YAML
                .replace("base_branch: main", "base_branch: master"),
        )
        .unwrap();
        let db = Database::open(&db_path).unwrap();
        let now = Utc::now();
        db.insert_issue_rebuild(&Issue {
            id: "atelier-role".to_string(),
            title: "role issue".to_string(),
            description: None,
            status: status.to_string(),
            issue_type: "task".to_string(),
            priority: "medium".to_string(),
            fields: BTreeMap::new(),
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
        })
        .unwrap();
        (dir, db)
    }

    #[test]
    fn explicit_review_role_wins() {
        let (dir, db) = setup_role_repo("review");

        let role =
            resolve_review_role(&db, dir.path(), Some("atelier-role"), Some("manager")).unwrap();

        assert_eq!(role.role, "manager");
        assert_eq!(role.source, "explicit --role");
    }

    #[test]
    fn review_role_infers_from_owner_status() {
        let (dir, db) = setup_role_repo("review");

        let role = resolve_review_role(&db, dir.path(), Some("atelier-role"), None).unwrap();

        assert_eq!(role.role, "reviewer");
        assert_eq!(role.source, "status review (atelier-role)");
    }

    #[test]
    fn review_role_requires_status_role_when_not_explicit() {
        let (dir, db) = setup_role_repo("todo");

        let error = resolve_review_role(&db, dir.path(), Some("atelier-role"), None)
            .unwrap_err()
            .to_string();

        assert!(error.contains("review_role_missing"));
        assert!(error.contains("statuses.todo.role"));
    }

    #[test]
    fn render_comment_lines_filters_resolved_comments() {
        let lines = render_review_comment_lines(
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

        assert_eq!(lines, vec!["review-comment 1 src/lib.rs:10 unresolved"]);
    }

    #[test]
    fn render_pull_comment_lines_include_body_summary() {
        let lines = render_pull_comment_lines(vec![ForgejoComment {
            id: 11,
            body: "Looks\n\n good from the top-level PR discussion".to_string(),
        }]);

        assert_eq!(
            lines,
            vec!["comment 11 - Looks good from the top-level PR discussion"]
        );
    }
}
