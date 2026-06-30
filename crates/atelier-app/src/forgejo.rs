use std::collections::BTreeMap;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use crate::project_config::ForgejoConfig;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForgejoRequest {
    pub method: &'static str,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub headers: BTreeMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForgejoResponse {
    pub status: u16,
    pub body: String,
}

pub trait ForgejoTransport {
    fn send(&self, request: ForgejoRequest) -> Result<ForgejoResponse>;
}

#[derive(Debug, Clone)]
pub struct UreqForgejoTransport {
    base_url: String,
    admin_token: String,
}

impl UreqForgejoTransport {
    pub fn new(base_url: impl Into<String>, admin_token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            admin_token: admin_token.into(),
        }
    }
}

impl ForgejoTransport for UreqForgejoTransport {
    fn send(&self, request: ForgejoRequest) -> Result<ForgejoResponse> {
        let mut url = format!("{}{}", self.base_url, request.path);
        if !request.query.is_empty() {
            let query = request
                .query
                .iter()
                .map(|(key, value)| format!("{}={}", percent_encode(key), percent_encode(value)))
                .collect::<Vec<_>>()
                .join("&");
            url.push('?');
            url.push_str(&query);
        }
        let agent = ureq::AgentBuilder::new().build();
        let mut http = agent.request(request.method, &url);
        http = http.set("Authorization", &format!("token {}", self.admin_token));
        http = http.set("Accept", "application/json");
        for (key, value) in &request.headers {
            http = http.set(key, value);
        }
        let response = if let Some(body) = request.body {
            http.set("Content-Type", "application/json")
                .send_string(&body)
        } else {
            http.call()
        };
        match response {
            Ok(response) => Ok(ForgejoResponse {
                status: response.status(),
                body: response.into_string().unwrap_or_default(),
            }),
            Err(ureq::Error::Status(status, response)) => Ok(ForgejoResponse {
                status,
                body: response.into_string().unwrap_or_default(),
            }),
            Err(error) => Err(anyhow!(
                "forgejo_api_error: request to {} failed: {}",
                url,
                error
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForgejoClient<T> {
    config: ForgejoConfig,
    transport: T,
}

impl<T: ForgejoTransport> ForgejoClient<T> {
    pub fn new(config: ForgejoConfig, transport: T) -> Self {
        Self { config, transport }
    }

    pub fn find_open_pull(
        &self,
        source_branch: &str,
        target_branch: &str,
    ) -> Result<Option<ForgejoPullRequest>> {
        let response = self.send(ForgejoRequest {
            method: "GET",
            path: self.repo_path("pulls"),
            query: vec![
                ("state".to_string(), "open".to_string()),
                ("head".to_string(), source_branch.to_string()),
                ("base".to_string(), target_branch.to_string()),
            ],
            headers: BTreeMap::new(),
            body: None,
        })?;
        let pulls = serde_json::from_str::<Vec<PullResponse>>(&response.body)
            .context("forgejo_api_error: failed to parse pull request search response")?;
        Ok(pulls.into_iter().next().map(Into::into))
    }

    pub fn open_pull(
        &self,
        role: &str,
        title: &str,
        body: &str,
        source_branch: &str,
        target_branch: &str,
    ) -> Result<ForgejoPullRequest> {
        let payload = serde_json::to_string(&OpenPullPayload {
            title,
            body,
            head: source_branch,
            base: target_branch,
        })?;
        let response =
            self.send(self.write_request(role, "POST", self.repo_path("pulls"), payload)?)?;
        serde_json::from_str::<PullResponse>(&response.body)
            .map(Into::into)
            .context("forgejo_api_error: failed to parse created pull request response")
    }

    pub fn show_pull(&self, number: u64) -> Result<ForgejoPullRequest> {
        let response = self.send(ForgejoRequest {
            method: "GET",
            path: self.repo_path(&format!("pulls/{number}")),
            query: Vec::new(),
            headers: BTreeMap::new(),
            body: None,
        })?;
        serde_json::from_str::<PullResponse>(&response.body)
            .map(Into::into)
            .context("forgejo_api_error: failed to parse pull request response")
    }

    pub fn merge_pull(&self, role: &str, number: u64) -> Result<ForgejoPullRequest> {
        let payload = serde_json::to_string(&MergePullPayload {
            merge_method: "merge",
        })?;
        self.send(self.write_request(
            role,
            "POST",
            self.repo_path(&format!("pulls/{number}/merge")),
            payload,
        )?)?;
        self.show_pull(number)
    }

    pub fn comment_pull(&self, role: &str, number: u64, body: &str) -> Result<ForgejoComment> {
        let payload = serde_json::to_string(&CommentPayload { body })?;
        let response = self.send(self.write_request(
            role,
            "POST",
            self.repo_path(&format!("issues/{number}/comments")),
            payload,
        )?)?;
        serde_json::from_str::<CommentResponse>(&response.body)
            .map(Into::into)
            .context("forgejo_api_error: failed to parse pull request comment response")
    }

    pub fn pull_comments(&self, number: u64) -> Result<Vec<ForgejoComment>> {
        let response = self.send(ForgejoRequest {
            method: "GET",
            path: self.repo_path(&format!("issues/{number}/comments")),
            query: Vec::new(),
            headers: BTreeMap::new(),
            body: None,
        })?;
        serde_json::from_str::<Vec<CommentResponse>>(&response.body)
            .map(|comments| comments.into_iter().map(Into::into).collect())
            .context("forgejo_api_error: failed to parse pull request comments response")
    }

    pub fn review_pull(
        &self,
        role: &str,
        number: u64,
        event: ReviewEvent,
        body: &str,
    ) -> Result<ForgejoReview> {
        let payload = serde_json::to_string(&ReviewPayload {
            event: event.as_str(),
            body,
        })?;
        let response = self.send(self.write_request(
            role,
            "POST",
            self.repo_path(&format!("pulls/{number}/reviews")),
            payload,
        )?)?;
        serde_json::from_str::<ReviewResponse>(&response.body)
            .map(Into::into)
            .context("forgejo_api_error: failed to parse pull request review response")
    }

    pub fn review_comments(&self, number: u64) -> Result<Vec<ForgejoReviewComment>> {
        let response = self.send(ForgejoRequest {
            method: "GET",
            path: self.repo_path(&format!("pulls/{number}/reviews/comments")),
            query: Vec::new(),
            headers: BTreeMap::new(),
            body: None,
        })?;
        serde_json::from_str::<Vec<ReviewCommentResponse>>(&response.body)
            .map(|comments| comments.into_iter().map(Into::into).collect())
            .context("forgejo_api_error: failed to parse pull request review comments response")
    }

    pub fn user_exists(&self, username: &str) -> Result<bool> {
        let response = self.transport.send(ForgejoRequest {
            method: "GET",
            path: format!("/api/v1/users/{}", percent_encode(username)),
            query: Vec::new(),
            headers: BTreeMap::new(),
            body: None,
        })?;
        match response.status {
            200..=299 => Ok(true),
            404 => Ok(false),
            status => Err(anyhow!(
                "forgejo_api_error: GET /api/v1/users/{} failed with status {}: {}",
                username,
                status,
                response.body
            )),
        }
    }

    pub fn create_user(
        &self,
        username: &str,
        email: &str,
        full_name: &str,
        password: &str,
    ) -> Result<()> {
        let payload = serde_json::to_string(&CreateUserPayload {
            username,
            email,
            full_name,
            password,
            must_change_password: false,
            restricted: true,
            send_notify: false,
        })?;
        self.send(ForgejoRequest {
            method: "POST",
            path: "/api/v1/admin/users".to_string(),
            query: Vec::new(),
            headers: BTreeMap::new(),
            body: Some(payload),
        })?;
        Ok(())
    }

    pub fn add_collaborator(&self, username: &str, permission: &str) -> Result<()> {
        let payload = serde_json::to_string(&AddCollaboratorPayload { permission })?;
        self.send(ForgejoRequest {
            method: "PUT",
            path: self.repo_path(&format!("collaborators/{}", percent_encode(username))),
            query: Vec::new(),
            headers: BTreeMap::new(),
            body: Some(payload),
        })?;
        Ok(())
    }

    pub fn collaborator_permission(&self, username: &str) -> Result<Option<String>> {
        let path = self.repo_path(&format!(
            "collaborators/{}/permission",
            percent_encode(username)
        ));
        let response = self.transport.send(ForgejoRequest {
            method: "GET",
            path: path.clone(),
            query: Vec::new(),
            headers: BTreeMap::new(),
            body: None,
        })?;
        match response.status {
            200..=299 => {
                let parsed = serde_json::from_str::<CollaboratorPermissionResponse>(&response.body)
                    .context(
                        "forgejo_api_error: failed to parse collaborator permission response",
                    )?;
                Ok(Some(parsed.permission))
            }
            404 => Ok(None),
            status => Err(anyhow!(
                "forgejo_api_error: GET {} failed with status {}: {}",
                path,
                status,
                response.body
            )),
        }
    }

    pub fn verify_sudo_user(&self, username: &str) -> Result<bool> {
        let mut headers = BTreeMap::new();
        headers.insert("Sudo".to_string(), username.to_string());
        let response = self.transport.send(ForgejoRequest {
            method: "GET",
            path: "/api/v1/user".to_string(),
            query: Vec::new(),
            headers,
            body: None,
        })?;
        match response.status {
            200..=299 => {
                let parsed = serde_json::from_str::<serde_json::Value>(&response.body)
                    .context("forgejo_api_error: failed to parse sudo user response")?;
                let login = parsed
                    .get("login")
                    .or_else(|| parsed.get("username"))
                    .and_then(|value| value.as_str());
                Ok(login == Some(username))
            }
            403 | 404 => Ok(false),
            status => Err(anyhow!(
                "forgejo_api_error: GET /api/v1/user with Sudo failed with status {}: {}",
                status,
                response.body
            )),
        }
    }

    fn send(&self, request: ForgejoRequest) -> Result<ForgejoResponse> {
        let method = request.method;
        let path = request.path.clone();
        let response = self.transport.send(request)?;
        if (200..300).contains(&response.status) {
            Ok(response)
        } else {
            Err(anyhow!(
                "forgejo_api_error: {} {} failed with status {}: {}",
                method,
                path,
                response.status,
                response.body
            ))
        }
    }

    fn write_request(
        &self,
        role: &str,
        method: &'static str,
        path: String,
        body: String,
    ) -> Result<ForgejoRequest> {
        let mut headers = BTreeMap::new();
        headers.insert(
            "Sudo".to_string(),
            self.config.role_author_for_role(role)?.to_string(),
        );
        Ok(ForgejoRequest {
            method,
            path,
            query: Vec::new(),
            headers,
            body: Some(body),
        })
    }

    fn repo_path(&self, suffix: &str) -> String {
        format!(
            "/api/v1/repos/{}/{}/{}",
            percent_encode(&self.config.owner),
            percent_encode(&self.config.repo),
            suffix
        )
    }
}

#[derive(Debug, Serialize)]
struct CreateUserPayload<'a> {
    username: &'a str,
    email: &'a str,
    full_name: &'a str,
    password: &'a str,
    must_change_password: bool,
    restricted: bool,
    send_notify: bool,
}

#[derive(Debug, Serialize)]
struct AddCollaboratorPayload<'a> {
    permission: &'a str,
}

#[derive(Debug, Deserialize)]
struct CollaboratorPermissionResponse {
    permission: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForgejoPullRequest {
    pub number: u64,
    pub url: String,
    pub state: String,
    pub merged: bool,
    pub source_branch: String,
    pub target_branch: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForgejoComment {
    pub id: u64,
    pub body: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForgejoReview {
    pub id: u64,
    pub state: String,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForgejoReviewComment {
    pub id: u64,
    pub path: String,
    pub line: Option<u64>,
    pub body: String,
    pub resolved: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ReviewEvent {
    Approve,
    RequestChanges,
    Comment,
}

impl ReviewEvent {
    fn as_str(self) -> &'static str {
        match self {
            Self::Approve => "APPROVE",
            Self::RequestChanges => "REQUEST_CHANGES",
            Self::Comment => "COMMENT",
        }
    }
}

#[derive(Debug, Serialize)]
struct OpenPullPayload<'a> {
    title: &'a str,
    body: &'a str,
    head: &'a str,
    base: &'a str,
}

#[derive(Debug, Serialize)]
struct MergePullPayload<'a> {
    #[serde(rename = "Do")]
    merge_method: &'a str,
}

#[derive(Debug, Serialize)]
struct CommentPayload<'a> {
    body: &'a str,
}

#[derive(Debug, Serialize)]
struct ReviewPayload<'a> {
    event: &'a str,
    body: &'a str,
}

#[derive(Debug, Deserialize)]
struct PullResponse {
    number: u64,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    html_url: Option<String>,
    state: String,
    #[serde(default)]
    merged: bool,
    head: PullBranchResponse,
    base: PullBranchResponse,
}

#[derive(Debug, Deserialize)]
struct PullBranchResponse {
    #[serde(rename = "ref")]
    branch: String,
}

impl From<PullResponse> for ForgejoPullRequest {
    fn from(value: PullResponse) -> Self {
        Self {
            number: value.number,
            url: value.html_url.or(value.url).unwrap_or_default(),
            state: value.state,
            merged: value.merged,
            source_branch: value.head.branch,
            target_branch: value.base.branch,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CommentResponse {
    id: u64,
    body: String,
}

impl From<CommentResponse> for ForgejoComment {
    fn from(value: CommentResponse) -> Self {
        Self {
            id: value.id,
            body: value.body,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ReviewResponse {
    id: u64,
    state: String,
    body: Option<String>,
}

impl From<ReviewResponse> for ForgejoReview {
    fn from(value: ReviewResponse) -> Self {
        Self {
            id: value.id,
            state: value.state,
            body: value.body,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ReviewCommentResponse {
    id: u64,
    path: String,
    line: Option<u64>,
    body: String,
    #[serde(default)]
    resolved: bool,
}

impl From<ReviewCommentResponse> for ForgejoReviewComment {
    fn from(value: ReviewCommentResponse) -> Self {
        Self {
            id: value.id,
            path: value.path,
            line: value.line,
            body: value.body,
            resolved: value.resolved,
        }
    }
}

fn percent_encode(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{byte:02X}").chars().collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_config::{ForgejoConfig, ForgejoRoleAuthors};
    use std::cell::RefCell;

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

    fn config() -> ForgejoConfig {
        ForgejoConfig {
            host: "forge.example.test".to_string(),
            owner: "tools".to_string(),
            repo: "atelier".to_string(),
            role_authors: Some(ForgejoRoleAuthors {
                worker: "forge-worker".to_string(),
                reviewer: "forge-reviewer".to_string(),
                validator: "forge-validator".to_string(),
                manager: "forge-manager".to_string(),
            }),
        }
    }

    fn pull_response(number: u64, merged: bool) -> String {
        format!(
            r#"{{"number":{number},"url":"https://forge.example.test/tools/atelier/pulls/{number}","state":"open","merged":{merged},"head":{{"ref":"codex/work"}},"base":{{"ref":"master"}}}}"#
        )
    }

    #[test]
    fn opens_pull_with_role_sudo_header_and_payload() {
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 201,
            body: pull_response(42, false),
        }]);
        let client = ForgejoClient::new(config(), &transport);

        let pull = client
            .open_pull("worker", "Title", "Body", "codex/work", "master")
            .unwrap();

        assert_eq!(pull.number, 42);
        let requests = transport.requests();
        assert_eq!(requests[0].method, "POST");
        assert_eq!(requests[0].path, "/api/v1/repos/tools/atelier/pulls");
        assert_eq!(
            requests[0].headers.get("Sudo").map(String::as_str),
            Some("forge-worker")
        );
        let body = requests[0].body.as_deref().unwrap();
        assert!(body.contains("\"head\":\"codex/work\""));
        assert!(body.contains("\"base\":\"master\""));
    }

    #[test]
    fn finds_and_shows_pull_request_state() {
        let transport = MockTransport::new(vec![
            ForgejoResponse {
                status: 200,
                body: format!("[{}]", pull_response(7, false)),
            },
            ForgejoResponse {
                status: 200,
                body: pull_response(7, true),
            },
        ]);
        let client = ForgejoClient::new(config(), &transport);

        let found = client
            .find_open_pull("codex/work", "master")
            .unwrap()
            .unwrap();
        let shown = client.show_pull(7).unwrap();

        assert!(!found.merged);
        assert!(shown.merged);
        let requests = transport.requests();
        assert_eq!(requests[0].path, "/api/v1/repos/tools/atelier/pulls");
        assert_eq!(
            requests[0].query,
            vec![
                ("state".to_string(), "open".to_string()),
                ("head".to_string(), "codex/work".to_string()),
                ("base".to_string(), "master".to_string()),
            ]
        );
        assert_eq!(requests[1].path, "/api/v1/repos/tools/atelier/pulls/7");
    }

    #[test]
    fn merges_pull_with_role_sudo_header_and_confirms_state() {
        let transport = MockTransport::new(vec![
            ForgejoResponse {
                status: 200,
                body: "{}".to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: pull_response(42, true),
            },
        ]);
        let client = ForgejoClient::new(config(), &transport);

        let pull = client.merge_pull("validator", 42).unwrap();

        assert!(pull.merged);
        let requests = transport.requests();
        assert_eq!(requests[0].method, "POST");
        assert_eq!(
            requests[0].path,
            "/api/v1/repos/tools/atelier/pulls/42/merge"
        );
        assert_eq!(
            requests[0].headers.get("Sudo").map(String::as_str),
            Some("forge-validator")
        );
        assert_eq!(requests[0].body.as_deref(), Some(r#"{"Do":"merge"}"#));
        assert_eq!(requests[1].method, "GET");
        assert_eq!(requests[1].path, "/api/v1/repos/tools/atelier/pulls/42");
    }

    #[test]
    fn comments_and_reviews_with_distinct_sudo_authorship() {
        let transport = MockTransport::new(vec![
            ForgejoResponse {
                status: 201,
                body: r#"{"id":11,"body":"Looks good"}"#.to_string(),
            },
            ForgejoResponse {
                status: 201,
                body: r#"{"id":12,"state":"APPROVED","body":"Approved"}"#.to_string(),
            },
        ]);
        let client = ForgejoClient::new(config(), &transport);

        let comment = client.comment_pull("reviewer", 42, "Looks good").unwrap();
        let review = client
            .review_pull("validator", 42, ReviewEvent::Approve, "Approved")
            .unwrap();

        assert_eq!(comment.id, 11);
        assert_eq!(review.state, "APPROVED");
        let requests = transport.requests();
        assert_eq!(
            requests[0].path,
            "/api/v1/repos/tools/atelier/issues/42/comments"
        );
        assert_eq!(
            requests[0].headers.get("Sudo").map(String::as_str),
            Some("forge-reviewer")
        );
        assert_eq!(
            requests[1].path,
            "/api/v1/repos/tools/atelier/pulls/42/reviews"
        );
        assert_eq!(
            requests[1].headers.get("Sudo").map(String::as_str),
            Some("forge-validator")
        );
        assert!(requests[1].body.as_deref().unwrap().contains("APPROVE"));
    }

    #[test]
    fn lists_top_level_pull_comments() {
        let transport = MockTransport::new(vec![ForgejoResponse {
            status: 200,
            body: r#"[{"id":11,"body":"Looks good"}]"#.to_string(),
        }]);
        let client = ForgejoClient::new(config(), &transport);

        let comments = client.pull_comments(42).unwrap();

        assert_eq!(comments[0].id, 11);
        assert_eq!(comments[0].body, "Looks good");
        let requests = transport.requests();
        assert_eq!(requests[0].method, "GET");
        assert_eq!(
            requests[0].path,
            "/api/v1/repos/tools/atelier/issues/42/comments"
        );
    }

    #[test]
    fn provisions_role_users_and_repository_permissions() {
        let transport = MockTransport::new(vec![
            ForgejoResponse {
                status: 404,
                body: "missing".to_string(),
            },
            ForgejoResponse {
                status: 201,
                body: r#"{"login":"atelier-worker"}"#.to_string(),
            },
            ForgejoResponse {
                status: 204,
                body: String::new(),
            },
            ForgejoResponse {
                status: 200,
                body: r#"{"permission":"write"}"#.to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: r#"{"login":"atelier-worker"}"#.to_string(),
            },
        ]);
        let client = ForgejoClient::new(config(), &transport);

        assert!(!client.user_exists("atelier-worker").unwrap());
        client
            .create_user(
                "atelier-worker",
                "atelier-worker@localhost.invalid",
                "Atelier Worker",
                "secret",
            )
            .unwrap();
        client.add_collaborator("atelier-worker", "write").unwrap();
        assert_eq!(
            client
                .collaborator_permission("atelier-worker")
                .unwrap()
                .as_deref(),
            Some("write")
        );
        assert!(client.verify_sudo_user("atelier-worker").unwrap());

        let requests = transport.requests();
        assert_eq!(requests[0].method, "GET");
        assert_eq!(requests[0].path, "/api/v1/users/atelier-worker");
        assert_eq!(requests[1].method, "POST");
        assert_eq!(requests[1].path, "/api/v1/admin/users");
        let body = requests[1].body.as_deref().unwrap();
        assert!(body.contains("\"username\":\"atelier-worker\""));
        assert!(body.contains("\"email\":\"atelier-worker@localhost.invalid\""));
        assert!(body.contains("\"restricted\":true"));
        assert!(body.contains("\"send_notify\":false"));
        assert_eq!(requests[2].method, "PUT");
        assert_eq!(
            requests[2].path,
            "/api/v1/repos/tools/atelier/collaborators/atelier-worker"
        );
        assert_eq!(
            requests[2].body.as_deref(),
            Some(r#"{"permission":"write"}"#)
        );
        assert_eq!(
            requests[4].headers.get("Sudo").map(String::as_str),
            Some("atelier-worker")
        );
    }

    #[test]
    fn lists_review_comments_and_surfaces_api_failures() {
        let transport = MockTransport::new(vec![
            ForgejoResponse {
                status: 200,
                body:
                    r#"[{"id":5,"path":"src/lib.rs","line":12,"body":"Fix this","resolved":false}]"#
                        .to_string(),
            },
            ForgejoResponse {
                status: 500,
                body: "remote exploded".to_string(),
            },
        ]);
        let client = ForgejoClient::new(config(), &transport);

        let comments = client.review_comments(42).unwrap();
        let error = client.show_pull(42).unwrap_err().to_string();

        assert_eq!(comments[0].path, "src/lib.rs");
        assert!(!comments[0].resolved);
        assert!(error.contains("forgejo_api_error"));
        assert!(error.contains("GET /api/v1/repos/tools/atelier/pulls/42 failed"));
    }
}
