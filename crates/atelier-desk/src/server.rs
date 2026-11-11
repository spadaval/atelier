//! The HTTP server: the reading routes, the acts and sessions, the `Host`
//! and app checks, the cache, and the built app's files.

use crate::acts::{self, Answers, On};
use crate::api::{
    ActRequest, ApiError, Hold, InboxRepo, InboxResponse, RepoSummary, ReposResponse,
    SessionRequest, ThreadsResponse,
};
use crate::config::Config;
use crate::threads::Threads;
use crate::{read, sessions};
use anyhow::Result;
use atelier_app::{git, Repo, Skip};
use atelier_records::validate_id;
use axum::extract::{Path as UrlPath, Query, Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

/// The header the Desk's app sends with an act or a session, with the
/// value `1`. A page of another site cannot send it without the browser
/// asking this server first, which it does not answer.
pub const APP_HEADER: &str = "x-atelier-desk";

/// A response kept with the commits it was read from.
type Slot = (Vec<String>, Arc<dyn Any + Send + Sync>);

pub struct Desk {
    config: Config,
    /// T3 Code's live threads, as last polled.
    pub threads: Arc<Threads>,
    /// By repository and route.
    cache: Mutex<HashMap<(String, String), Slot>>,
    /// How many responses were built rather than taken from the cache.
    pub built: AtomicUsize,
    /// Answers given with `decide` that no commit holds yet.
    answers: Answers,
    /// Held while an act runs, so two acts never write at once.
    acting: Mutex<()>,
}

impl Desk {
    pub fn new(config: Config) -> Arc<Desk> {
        Arc::new(Desk {
            threads: Arc::new(Threads::new(config.t3.as_ref())),
            config,
            cache: Mutex::default(),
            built: AtomicUsize::new(0),
            answers: Answers::default(),
            acting: Mutex::default(),
        })
    }

    /// Whether `name`, a host name without its port, is one this Desk
    /// answers to.
    fn answers_to(&self, name: &str) -> bool {
        self.config
            .hosts
            .iter()
            .any(|a| a.eq_ignore_ascii_case(name))
    }

    /// Runs `request` with the core's own function, under the configured
    /// caller.
    fn act(&self, request: ActRequest) -> Result<crate::api::ActResponse, Failure> {
        let repo = match &request {
            ActRequest::Ready(ready) => &ready.repo,
            ActRequest::Decide(decide) => {
                checked(&decide.id)?;
                &decide.repo
            }
        };
        let dir = self.served(repo)?;
        let _one = self.acting.lock().unwrap_or_else(|p| p.into_inner());
        let (response, answered) = acts::act(&dir, &self.config.caller, &request)
            .map_err(|error| Failure::Failed(format!("{error:#}")))?;
        if let Some(answer) = answered {
            self.answers.add(repo, answer);
        }
        Ok(response)
    }

    fn served(&self, name: &str) -> Result<std::path::PathBuf, Failure> {
        (self.config.repository(name))
            .ok_or_else(|| Failure::NotFound(format!("No repository {name} is served.")))
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

    /// What each identity holds in the records of every repository
    /// served. A repository that does not read holds nothing here;
    /// `/api/repos` gives its error.
    fn holds(&self) -> Vec<(String, Hold)> {
        let mut holds = Vec::new();
        for (name, dir, duplicate) in self.config.repositories() {
            if duplicate.is_none() {
                holds.extend(self.repo_holds(&name, &dir));
            }
        }
        holds
    }

    fn repo_holds(&self, name: &str, dir: &Path) -> Vec<(String, Hold)> {
        let held = self.read(name, dir, "holds", |r| read::holds(r, name));
        held.ok().flatten().unwrap_or_default()
    }

    /// The response for `route` of the repository served as `name`.
    fn repo_route<T: Clone + Send + Sync + 'static>(
        &self,
        name: &str,
        route: &str,
        missing: impl FnOnce() -> String,
        build: impl FnOnce(&mut Repo) -> Result<Option<T>>,
    ) -> Result<T, Failure> {
        let dir = self.served(name)?;
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
        .route("/api/threads", get(threads))
        .merge(
            Router::new()
                .route("/api/act", post(act))
                .route("/api/sessions", post(session))
                .route_layer(middleware::from_fn_with_state(desk.clone(), from_the_app)),
        )
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
    if name.is_some_and(|name| desk.answers_to(name)) {
        return next.run(request).await;
    }
    let error = "This Desk does not answer to that host name.".to_string();
    (StatusCode::FORBIDDEN, Json(ApiError { error })).into_response()
}

/// Refuses an act or a session that does not come from the Desk's own
/// app: one without its header, or whose `Origin` is not a host the Desk
/// answers to.
async fn from_the_app(State(desk): State<Arc<Desk>>, request: Request, next: Next) -> Response {
    let headers = request.headers();
    let marked = headers.get(APP_HEADER).is_some_and(|value| value == "1");
    let origin = headers.get(header::ORIGIN).map(|origin| {
        let name = origin.to_str().ok().and_then(origin_host);
        name.is_some_and(|name| desk.answers_to(name))
    });
    if marked && origin != Some(false) {
        return next.run(request).await;
    }
    let error = format!(
        "Only the Desk's own app may act: a request needs the header {APP_HEADER}: 1, and an Origin, when it has one, that this Desk answers to."
    );
    (StatusCode::FORBIDDEN, Json(ApiError { error })).into_response()
}

/// The host name of an `Origin` such as `http://localhost:5173`, without
/// its port; `None` for one with no scheme, such as `null`.
fn origin_host(origin: &str) -> Option<&str> {
    let (_, rest) = origin.split_once("://")?;
    let authority = rest.split('/').next()?;
    (!authority.is_empty()).then(|| host_name(authority))
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
        Ok(ReposResponse {
            repos,
            built_from: crate::BUILT_FROM.into(),
        })
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
        let mut page = desk.repo_route(&repo, &format!("mission {id}"), missing, |r| {
            read::mission(r, &id)
        })?;
        let snapshot = desk.threads.snapshot();
        snapshot.join(&mut page.threads);
        page.t3 = snapshot.state;
        page.pending_answers = (desk.answers).pending(&repo, On::Mission(&id), &page.questions);
        Ok(page)
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
        let mut page = desk.repo_route(&repo, &format!("item {id}"), missing, |r| {
            read::item(r, &id)
        })?;
        let snapshot = desk.threads.snapshot();
        snapshot.join(&mut page.threads);
        page.t3 = snapshot.state;
        page.pending_answers = (desk.answers).pending(&repo, On::Item(&id), &page.decisions);
        // A checkout can be added without a commit, so this is not cached.
        let checkout = git::checkout_of(&desk.served(&repo)?, &id);
        let checkout = checkout.map_err(|error| Failure::Failed(format!("{error:#}")))?;
        page.decide_at = checkout.map(|path| path.display().to_string());
        Ok(page)
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
        let snapshot = desk.threads.snapshot();
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
                    Ok(inbox) => {
                        let mut inbox = inbox.expect("an inbox is always built");
                        let answers = &desk.answers;
                        (inbox.questions).retain(|q| !answers.waits(&name, &q.item, q.index));
                        let live = snapshot.live(&desk.repo_holds(&name, &dir));
                        let waiting = live.into_iter().filter(|thread| {
                            thread.status == "waiting" && !thread.holds.is_empty()
                        });
                        inbox.threads = waiting.collect();
                        inbox
                    }
                    Err(error) => failed(format!("{error:#}")),
                },
            });
        }
        Ok(InboxResponse {
            repos,
            t3: snapshot.state,
        })
    })
    .await
}

/// T3 Code's live threads as last polled, with what each holds in every
/// repository's records.
async fn threads(State(desk): State<Arc<Desk>>) -> Response {
    blocking(move || {
        let snapshot = desk.threads.snapshot();
        Ok(ThreadsResponse {
            threads: snapshot.live(&desk.holds()),
            t3: snapshot.state,
        })
    })
    .await
}

/// `POST /api/act`: `ready` or `decide`, made by the core's own function.
async fn act(State(desk): State<Arc<Desk>>, Json(request): Json<ActRequest>) -> Response {
    blocking(move || desk.act(request)).await
}

/// `POST /api/sessions`: a planning session for a mission, as a new T3
/// Code thread in the repository's T3 project.
async fn session(
    State(desk): State<Arc<Desk>>,
    Json(SessionRequest { repo, mission }): Json<SessionRequest>,
) -> Response {
    blocking(move || {
        checked(&mission)?;
        let missing = || format!("No mission {mission} is in the base's last commit.");
        let page = desk.repo_route(&repo, &format!("mission {mission}"), missing, |r| {
            read::mission(r, &mission)
        })?;
        let dir = desk.served(&repo)?;
        let t3 = desk.config.t3.as_ref();
        let title = &page.mission.title;
        Ok(sessions::start(
            t3,
            &desk.threads,
            &dir,
            &repo,
            &mission,
            title,
        ))
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
    fn an_origin_gives_its_host_name() {
        assert_eq!(origin_host("http://localhost:5173"), Some("localhost"));
        assert_eq!(
            origin_host("https://desk.example.ts.net"),
            Some("desk.example.ts.net")
        );
        assert_eq!(origin_host("http://[::1]:3780"), Some("[::1]"));
        assert_eq!(origin_host("null"), None);
        assert_eq!(origin_host("http://"), None);
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
