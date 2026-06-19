use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use atelier_app::forgejo::{
    ForgejoClient, ForgejoComment, ForgejoReviewComment, UreqForgejoTransport,
};
use atelier_app::pr as app_pr;
use atelier_sqlite::Database;

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
            role,
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
    role: &str,
) -> Result<()> {
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
            role,
        },
        &forgejo,
        &client,
    )?;
    println!("Review: {}", outcome.pull.url);
    println!("Issue:   {}", outcome.issue_id);
    println!("Owner:   {}", outcome.owner_id);
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
    issue_ref: Option<&str>,
    role: &str,
    body: &str,
) -> Result<()> {
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
            role,
            body,
        },
        &forgejo,
        &client,
    )?;
    println!("Comment: {}", outcome.comment.id);
    println!("Issue:   {}", outcome.issue_id);
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
    issue_ref: Option<&str>,
    role: &str,
    event: &str,
    body: &str,
) -> Result<()> {
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
            role,
            event,
            body,
        },
        &forgejo,
        &client,
    )?;
    println!("Review: {}", outcome.review.id);
    println!("State:  {}", outcome.review.state);
    println!("Issue:  {}", outcome.issue_id);
    Ok(())
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
