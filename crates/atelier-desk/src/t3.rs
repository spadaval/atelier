//! A client of T3 Code's MCP server (docs/architecture/desk.md, `GET
//! /api/threads`). It gets a token from the configured `t3` command, holds
//! it in memory only, and calls the server's tools over HTTP with it.

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// The scope that reads threads and can change nothing.
pub const READ: &str = "orchestration:read";
/// The statuses of a thread that is still at work.
pub const LIVE: [&str; 5] = ["preparing", "queued", "starting", "running", "waiting"];
const TTL: &str = "1h";
/// A token this many seconds from its expiry is replaced before use.
const MARGIN: i64 = 120;
const PROTOCOL: &str = "2025-06-18";

pub struct Client {
    command: PathBuf,
    url: String,
    scope: String,
    agent: ureq::Agent,
    token: Option<Token>,
    /// The MCP session `initialize` opened with the current token.
    session: Option<String>,
    next_id: u64,
    /// Set by `close`, after which no token is issued.
    closed: bool,
}

/// A bearer token and the T3 session it belongs to. It has no `Debug`, so
/// it cannot be printed by accident.
struct Token {
    secret: String,
    session: String,
    /// Seconds since the Unix epoch.
    expires_at: i64,
}

/// What `t3 auth session issue --json` prints.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Issued {
    session_id: String,
    token: String,
    expires_at: String,
}

/// One thread as `t3_thread_list` gives it; fields the Desk does not show
/// are ignored.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct T3Thread {
    pub thread_id: String,
    pub title: String,
    pub status: String,
    pub provider_instance_id: String,
    pub model: String,
    pub parent_thread_id: Option<String>,
    #[serde(default)]
    pub pull_requests: Option<Vec<T3PullRequest>>,
    #[serde(default)]
    pub link: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct T3PullRequest {
    pub repository: String,
    pub number: u32,
    pub url: String,
    #[serde(default)]
    pub state: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListPage {
    threads: Vec<T3Thread>,
    next_cursor: Option<u64>,
}

/// What one request got.
enum Posted {
    /// The server's result, and the MCP session it named, if any.
    Answer(Option<String>, Value),
    /// The token or the MCP session was refused and has been dropped; a
    /// new one may be tried.
    Refused(String),
}

impl Client {
    /// A client that runs `command` for tokens of `scope` and calls the MCP
    /// server at `url`. Nothing runs until the first call.
    pub fn new(command: PathBuf, url: String, scope: &str) -> Client {
        let agent = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .timeout_global(Some(Duration::from_secs(20)))
            .proxy(None)
            .build()
            .into();
        Client {
            command,
            url,
            scope: scope.to_string(),
            agent,
            token: None,
            session: None,
            next_id: 0,
            closed: false,
        }
    }

    /// T3 Code's live threads that have not been settled, in every project
    /// and with subagents, newest first.
    pub fn live_threads(&mut self) -> Result<Vec<T3Thread>> {
        let mut threads = Vec::new();
        let mut cursor = None;
        loop {
            let mut arguments = json!({
                "allProjects": true,
                "includeSubagents": true,
                "settled": false,
                "statuses": LIVE,
                "limit": 100,
            });
            if let Some(cursor) = cursor {
                arguments["cursor"] = json!(cursor);
            }
            let page: ListPage = serde_json::from_value(self.call("t3_thread_list", arguments)?)
                .context("t3_thread_list answered in a shape the Desk does not read")?;
            let empty = page.threads.is_empty();
            threads.extend(page.threads);
            match page.next_cursor {
                Some(next) if !empty => cursor = Some(next),
                _ => return Ok(threads),
            }
        }
    }

    /// Calls the MCP tool `tool` and returns its structured result. A token
    /// that is refused, or an MCP session the server no longer knows, is
    /// replaced and the call made once more.
    pub fn call(&mut self, tool: &str, arguments: Value) -> Result<Value> {
        let params = json!({ "name": tool, "arguments": arguments });
        let result = match self.attempt(&params)? {
            Posted::Answer(_, result) => result,
            Posted::Refused(_) => match self.attempt(&params)? {
                Posted::Answer(_, result) => result,
                Posted::Refused(why) => bail!("{why}"),
            },
        };
        if result["isError"] == true {
            let text = result["content"][0]["text"].as_str().unwrap_or_default();
            bail!("T3 Code refused {tool}: {text}");
        }
        match result.get("structuredContent") {
            Some(structured) => Ok(structured.clone()),
            None => {
                let text = result["content"][0]["text"].as_str().unwrap_or_default();
                serde_json::from_str(text).with_context(|| format!("{tool} gave no JSON result"))
            }
        }
    }

    /// Revokes the token held, if any. Every later call fails.
    pub fn close(&mut self) {
        self.closed = true;
        self.session = None;
        if let Some(token) = self.token.take() {
            self.revoke(&token);
        }
    }

    /// One `tools/call`, with a token and an MCP session.
    fn attempt(&mut self, params: &Value) -> Result<Posted> {
        self.fresh_token()?;
        if self.session.is_none() {
            let initialize = json!({
                "protocolVersion": PROTOCOL,
                "capabilities": {},
                "clientInfo": { "name": "atelier-desk", "version": env!("CARGO_PKG_VERSION") },
            });
            let session = match self.post("initialize", &initialize, None)? {
                Posted::Answer(session, _) => session,
                refused => return Ok(refused),
            };
            let session = session.context("T3 Code's MCP server opened no session")?;
            let note = json!({ "jsonrpc": "2.0", "method": "notifications/initialized" });
            self.send(&note, Some(&session))?;
            self.session = Some(session);
        }
        let session = self.session.clone();
        self.post("tools/call", params, session.as_deref())
    }

    /// Posts one JSON-RPC request. A JSON-RPC error is an error; a refused
    /// token or session is dropped and given as `Refused`.
    fn post(&mut self, method: &str, params: &Value, session: Option<&str>) -> Result<Posted> {
        self.next_id += 1;
        let id = self.next_id;
        let request = json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params });
        let mut response = self.send(&request, session)?;
        let status = response.status().as_u16();
        let header = |name: &str| {
            let value = response.headers().get(name);
            value.and_then(|v| v.to_str().ok()).map(str::to_string)
        };
        let named = header("mcp-session-id");
        let body = response.body_mut().read_to_string().unwrap_or_default();
        if status == 401 || status == 403 {
            if let Some(token) = self.token.take() {
                self.revoke(&token);
            }
            self.session = None;
            let refused = format!("T3 Code refused the Desk's token ({status})");
            return Ok(Posted::Refused(refused));
        }
        if session.is_some() && (status == 400 || status == 404) {
            self.session = None;
            let gone = format!("T3 Code's MCP server no longer knows the session ({status})");
            return Ok(Posted::Refused(gone));
        }
        if !(200..300).contains(&status) {
            bail!("T3 Code's MCP server answered {method} with {status}");
        }
        let message: Value =
            serde_json::from_str(&body).context("T3 Code's MCP server answered with no JSON")?;
        if let Some(error) = message.get("error") {
            let reason = error["message"].as_str().unwrap_or("no reason given");
            bail!("T3 Code's MCP server refused {method}: {reason}");
        }
        Ok(Posted::Answer(named, message["result"].clone()))
    }

    fn send(
        &self,
        body: &Value,
        session: Option<&str>,
    ) -> Result<ureq::http::Response<ureq::Body>> {
        let secret = &self.token.as_ref().expect("a token is held").secret;
        let mut request = (self.agent.post(&self.url))
            .header("Authorization", format!("Bearer {secret}"))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json, text/event-stream")
            .header("MCP-Protocol-Version", PROTOCOL);
        if let Some(session) = session {
            request = request.header("Mcp-Session-Id", session);
        }
        // ureq's errors name the address and the cause, never a header.
        (request.send(body.to_string()))
            .with_context(|| format!("cannot reach T3 Code's MCP server at {}", self.url))
    }

    /// Holds a token that is not near its expiry, issuing a new one and
    /// revoking the old one when needed.
    fn fresh_token(&mut self) -> Result<()> {
        if self.closed {
            bail!("the Desk is stopping");
        }
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
        if self
            .token
            .as_ref()
            .is_some_and(|t| t.expires_at - MARGIN > now)
        {
            return Ok(());
        }
        if let Some(old) = self.token.take() {
            self.revoke(&old);
        }
        self.session = None;
        self.token = Some(self.issue()?);
        Ok(())
    }

    fn issue(&self) -> Result<Token> {
        let output = Command::new(&self.command)
            .args(["auth", "session", "issue", "--subject", "mcp-client"])
            .args(["--scope", &self.scope, "--ttl", TTL])
            .args(["--label", "atelier-desk", "--json"])
            .stdin(Stdio::null())
            .output()
            .with_context(|| format!("cannot run {}", self.command.display()))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!(
                "{} auth session issue failed: {}",
                self.command.display(),
                stderr.trim()
            );
        }
        // Its output holds the token, so it is never quoted in an error.
        let stdout = String::from_utf8_lossy(&output.stdout);
        let json = stdout.find('{').map_or("", |start| &stdout[start..]);
        let issued: Issued = serde_json::from_str(json).map_err(|_| {
            anyhow!(
                "{} auth session issue printed no token as JSON",
                self.command.display()
            )
        })?;
        let expires_at = atelier_core::Time::parse_from_rfc3339(&issued.expires_at)
            .context("the token's expiry is not a time")?;
        Ok(Token {
            secret: issued.token,
            session: issued.session_id,
            expires_at: expires_at.timestamp(),
        })
    }

    /// Revokes `token`'s session. A failure is not reported: the token
    /// expires on its own.
    fn revoke(&self, token: &Token) {
        let _ = Command::new(&self.command)
            .args(["auth", "session", "revoke", &token.session])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}
