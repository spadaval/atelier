//! The HTTP server: the reading routes, the `Host` check, the cache, and
//! the built app's files.

use crate::api::{ApiError, InboxRepo, InboxResponse, RepoSummary, ReposResponse};
use crate::config::Config;
use crate::read;
use anyhow::Result;
use atelier_app::{Repo, Skip};
use atelier_records::validate_id;
use axum::extract::{Path as UrlPath, Query, Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

/// A response kept with the commits it was read from.
type Slot = (Vec<String>, Arc<dyn Any + Send + Sync>);

pub struct Desk {
    config: Config,
    /// By repository and route.
    cache: Mutex<HashMap<(String, String), Slot>>,
    /// How many responses were built rather than taken from the cache.
    pub built: AtomicUsize,
}

impl Desk {
    pub fn new(config: Config) -> Arc<Desk> {
        Arc::new(Desk {
            config,
            cache: Mutex::default(),
            built: AtomicUsize::new(0),
        })
    }

    /// The response for `route` of the repository `name` at `dir`, from the
    /// cache while the commits it was read from are unchanged, else from
    /// `build`; `None` when `build` finds nothing.
    fn read<T: Clone + Send + Sync + 'static>(
        &self,
        name: &str,
        dir: &Path,
        route: &str,
        build: impl FnOnce(&mut Repo) -> Result<Option<T>>,
    ) -> Result<Option<T>> {
        // A `Repo` is not `Send`, so it and its skip live in this call,
        // which runs on a blocking thread.
        let skip = Skip::new(None);
        let mut repo = read::open(dir, &self.config.caller, &skip)?;
        let commits = read::commits(&repo)?;
        let slot = (name.to_string(), route.to_string());
        if let Some((at, kept)) = self.cache.lock().unwrap().get(&slot) {
            if *at == commits {
                return Ok(kept.downcast_ref::<T>().cloned());
            }
        }
        self.built.fetch_add(1, Ordering::Relaxed);
        let Some(built) = build(&mut repo)? else {
            return Ok(None);
        };
        let kept: Arc<dyn Any + Send + Sync> = Arc::new(built.clone());
        self.cache.lock().unwrap().insert(slot, (commits, kept));
        Ok(Some(built))
    }

    /// The response for `route` of the repository served as `name`.
    fn repo_route<T: Clone + Send + Sync + 'static>(
        &self,
        name: &str,
        route: &str,
        missing: impl FnOnce() -> String,
        build: impl FnOnce(&mut Repo) -> Result<Option<T>>,
    ) -> Result<T, Failure> {
        let dir = (self.config.repository(name))
            .ok_or_else(|| Failure::NotFound(format!("No repository {name} is served.")))?;
        match self.read(name, &dir, route, build) {
            Ok(Some(found)) => Ok(found),
            Ok(None) => Err(Failure::NotFound(missing())),
            Err(error) => Err(Failure::Failed(format!("{error:#}"))),
        }
    }
}

pub enum Failure {
    BadRequest(String),
    NotFound(String),
    Failed(String),
}

impl IntoResponse for Failure {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Failure::BadRequest(error) => (StatusCode::BAD_REQUEST, error),
            Failure::NotFound(error) => (StatusCode::NOT_FOUND, error),
            Failure::Failed(error) => (StatusCode::INTERNAL_SERVER_ERROR, error),
        };
        (status, Json(ApiError { error })).into_response()
    }
}

/// Runs `work` on a blocking thread and answers with its JSON.
async fn blocking<T: Serialize + Send + 'static>(
    work: impl FnOnce() -> Result<T, Failure> + Send + 'static,
) -> Response {
    match tokio::task::spawn_blocking(work).await {
        Ok(Ok(body)) => Json(body).into_response(),
        Ok(Err(failure)) => failure.into_response(),
        Err(error) => Failure::Failed(format!("the read failed: {error}")).into_response(),
    }
}

pub fn router(config: Config) -> Router {
    router_for(Desk::new(config))
}

pub fn router_for(desk: Arc<Desk>) -> Router {
    let api = Router::new()
        .route("/api/repos", get(repos))
        .route("/api/repos/{repo}/missions", get(missions))
        .route("/api/repos/{repo}/missions/{id}", get(mission))
        .route("/api/repos/{repo}/doc", get(doc))
        .route("/api/repos/{repo}/items/{id}", get(item))
        .route("/api/inbox", get(inbox))
        .layer(middleware::from_fn(json_errors));
    api.fallback(app_file)
        .layer(middleware::from_fn_with_state(desk.clone(), allowed_host))
        .with_state(desk)
}

/// Any path no route takes: under `/api`, however it is spelled, a JSON
/// 404; elsewhere a file of the built app, or its `index.html` for a path
/// that names none, when an app is configured.
async fn app_file(State(desk): State<Arc<Desk>>, request: Request) -> Response {
    let path = request.uri().path();
    let app = desk.config.app.as_ref();
    match app.filter(|_| path != "/api" && !path.starts_with("/api/")) {
        Some(app) => {
            let index = ServeFile::new(app.join("index.html"));
            let files = ServeDir::new(app).fallback(index);
            match files.oneshot(request).await {
                Ok(response) => response.into_response(),
                Err(never) => match never {},
            }
        }
        None => Failure::NotFound("No such route.".into()).into_response(),
    }
}

/// Gives an error from the API's routes that is not JSON, such as axum's
/// own for a query that does not parse or a method a route does not take,
/// the `{error}` body every other error has, keeping its status and its
/// `Allow` header.
async fn json_errors(request: Request, next: Next) -> Response {
    let response = next.run(request).await;
    let content = response.headers().get(header::CONTENT_TYPE);
    let json = content.is_some_and(|c| c.as_bytes().starts_with(b"application/json"));
    if response.status().is_success() || json {
        return response;
    }
    let (parts, body) = response.into_parts();
    let text = axum::body::to_bytes(body, 64 * 1024)
        .await
        .unwrap_or_default();
    let error = match String::from_utf8_lossy(&text).trim() {
        "" => format!(
            "{}.",
            parts
                .status
                .canonical_reason()
                .unwrap_or("The request failed")
        ),
        text => text.to_string(),
    };
    let mut answer = (parts.status, Json(ApiError { error })).into_response();
    if let Some(allow) = parts.headers.get(header::ALLOW) {
        answer.headers_mut().insert(header::ALLOW, allow.clone());
    }
    answer
}

/// Refuses a request whose `Host` is not one the Desk answers to.
async fn allowed_host(State(desk): State<Arc<Desk>>, request: Request, next: Next) -> Response {
    let host = request.headers().get(header::HOST);
    let name = host.and_then(|host| host.to_str().ok()).map(host_name);
    let allowed = desk.config.hosts.iter();
    if name.is_some_and(|name| allowed.clone().any(|a| a.eq_ignore_ascii_case(name))) {
        return next.run(request).await;
    }
    let error = "This Desk does not answer to that host name.".to_string();
    (StatusCode::FORBIDDEN, Json(ApiError { error })).into_response()
}

/// A `Host` header's name, without its port.
fn host_name(host: &str) -> &str {
    match host.strip_prefix('[') {
        Some(rest) => rest.find(']').map_or(host, |end| &host[..end + 2]),
        None => host.rsplit_once(':').map_or(host, |(name, _)| name),
    }
}

async fn repos(State(desk): State<Arc<Desk>>) -> Response {
    blocking(move || {
        let mut repos = Vec::new();
        for (name, dir, duplicate) in desk.config.repositories() {
            let failed = |error: String| RepoSummary {
                name: name.clone(),
                path: dir.display().to_string(),
                error: Some(error),
                ..RepoSummary::default()
            };
            repos.push(match duplicate {
                Some(error) => failed(error),
                None => match desk.read(&name, &dir, "summary", |r| read::summary(r, &name)) {
                    Ok(summary) => summary.expect("a summary is always built"),
                    Err(error) => failed(format!("{error:#}")),
                },
            });
        }
        Ok(ReposResponse { repos })
    })
    .await
}

#[derive(Deserialize)]
struct Done {
    #[serde(default)]
    done: bool,
}

async fn missions(
    State(desk): State<Arc<Desk>>,
    UrlPath(repo): UrlPath<String>,
    Query(Done { done }): Query<Done>,
) -> Response {
    blocking(move || {
        let route = format!("missions?done={done}");
        desk.repo_route(&repo, &route, String::new, |r| read::missions(r, done))
    })
    .await
}

fn checked(id: &str) -> Result<(), Failure> {
    validate_id(id).map_err(Failure::BadRequest)
}

async fn mission(
    State(desk): State<Arc<Desk>>,
    UrlPath((repo, id)): UrlPath<(String, String)>,
) -> Response {
    blocking(move || {
        checked(&id)?;
        let missing = || format!("No mission {id} is in the base's last commit.");
        desk.repo_route(&repo, &format!("mission {id}"), missing, |r| {
            read::mission(r, &id)
        })
    })
    .await
}

async fn item(
    State(desk): State<Arc<Desk>>,
    UrlPath((repo, id)): UrlPath<(String, String)>,
) -> Response {
    blocking(move || {
        checked(&id)?;
        let missing =
            || format!("No item {id} is in the base's last commit or on a branch named {id}.");
        desk.repo_route(&repo, &format!("item {id}"), missing, |r| {
            read::item(r, &id)
        })
    })
    .await
}

#[derive(Deserialize)]
struct DocPath {
    path: String,
}

async fn doc(
    State(desk): State<Arc<Desk>>,
    UrlPath(repo): UrlPath<String>,
    Query(DocPath { path }): Query<DocPath>,
) -> Response {
    blocking(move || {
        let path = normalized(&path).ok_or_else(|| {
            Failure::BadRequest(format!(
                "`{path}` is not a Markdown file's path in the repository"
            ))
        })?;
        let missing = || format!("No file {path} is in the base's last commit.");
        desk.repo_route(&repo, &format!("doc {path}"), missing, |r| {
            read::doc(r, &path)
        })
    })
    .await
}

/// `path`, from the repository's root, with `.` and `..` resolved; `None`
/// when it leaves the root or does not name a `.md` file.
fn normalized(path: &str) -> Option<String> {
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            part => parts.push(part),
        }
    }
    let path = parts.join("/");
    path.ends_with(".md").then_some(path)
}

async fn inbox(State(desk): State<Arc<Desk>>) -> Response {
    blocking(move || {
        let mut repos = Vec::new();
        for (name, dir, duplicate) in desk.config.repositories() {
            let failed = |error: String| InboxRepo {
                repo: name.clone(),
                error: Some(error),
                ..InboxRepo::default()
            };
            repos.push(match duplicate {
                Some(error) => failed(error),
                None => match desk.read(&name, &dir, "inbox", |r| read::inbox(r, &name)) {
                    Ok(inbox) => inbox.expect("an inbox is always built"),
                    Err(error) => failed(format!("{error:#}")),
                },
            });
        }
        Ok(InboxResponse { repos })
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_host_name_loses_its_port() {
        assert_eq!(host_name("localhost:3780"), "localhost");
        assert_eq!(host_name("desk.example.ts.net"), "desk.example.ts.net");
        assert_eq!(host_name("[::1]:3780"), "[::1]");
        assert_eq!(host_name("[::1]"), "[::1]");
    }

    #[test]
    fn a_document_path_stays_in_the_repository() {
        assert_eq!(normalized("docs/./a/../b.md").as_deref(), Some("docs/b.md"));
        assert_eq!(normalized("/docs//b.md").as_deref(), Some("docs/b.md"));
        assert_eq!(normalized("../b.md"), None);
        assert_eq!(normalized("docs/../../b.md"), None);
        assert_eq!(normalized("docs/b.txt"), None);
        assert_eq!(normalized(".md/.."), None);
    }
}
