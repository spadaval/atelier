//! `atelier serve` (atelier-zpmb, ADR 0030): a read-only JSON API over every
//! Atelier repository under the given roots. Each repository is read from
//! one captured commit of its base branch, never from a checkout, and
//! nothing is written to any repository.
//!
//! Only work that depends on nothing but a base commit is cached: the
//! records read from it and the views built from them, keyed by repository
//! and commit, and only the latest commit of each repository is kept. At
//! most two loads or view builds run at once; a request that waits longer
//! than five seconds for one gets a 503.

pub mod api;
pub mod discovery;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use atelier_app::commit_records::CommitRecords;
use atelier_app::git_freshness::ReadContext;
use axum::body::Body;
use axum::extract::{Path as UrlPath, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use serde::Serialize;
use tokio::sync::Semaphore;

use api::{
    ApiError, BriefResponse, DebriefResponse, MissionLinks, MissionResponse, MissionSummary,
    RepoCounts, RepoSummary, ReposResponse, Review, StrategyResponse,
};
use discovery::{Base, CommitFacts};

/// Where and how the site reads repositories.
#[derive(Debug, Clone)]
pub struct SiteOptions {
    /// Directories searched for checkouts holding `.atelier/`.
    pub roots: Vec<PathBuf>,
    /// How many loads and view builds may run at once.
    pub max_builds: usize,
    /// How long a request waits for one of them before a 503.
    pub wait: Duration,
}

impl SiteOptions {
    pub fn new(roots: Vec<PathBuf>) -> Self {
        Self {
            roots,
            max_builds: 2,
            wait: Duration::from_secs(5),
        }
    }
}

/// The server's state: the latest scan, and each repository's latest base
/// commit with what has been read and built from it.
pub struct Site {
    options: SiteOptions,
    /// The latest scan, with when it started.
    fleet: tokio::sync::Mutex<Option<(Instant, Arc<Fleet>)>>,
    /// What the policy and config of each repository's latest default and
    /// base commits say.
    facts: Arc<Mutex<HashMap<(PathBuf, String), CommitFacts>>>,
    snapshots: Mutex<HashMap<PathBuf, Arc<Snapshot>>>,
    builds: Semaphore,
    built: AtomicUsize,
}

impl Site {
    pub fn new(options: SiteOptions) -> Self {
        // Reading a repository never takes Git's optional locks.
        atelier_app::git_freshness::read_only();
        Self {
            builds: Semaphore::new(options.max_builds),
            options,
            fleet: tokio::sync::Mutex::new(None),
            facts: Arc::new(Mutex::new(HashMap::new())),
            snapshots: Mutex::new(HashMap::new()),
            built: AtomicUsize::new(0),
        }
    }

    /// How many record loads and view builds have run, for tests of the
    /// cache.
    pub fn builds_run(&self) -> usize {
        self.built.load(Ordering::Relaxed)
    }

    /// The repositories under the roots and their base commits, as a scan
    /// that started after this request arrived found them: one scan serves
    /// every request that was waiting when it started. A scan drops what was
    /// kept for repositories that are gone or whose base moved.
    async fn fleet(&self) -> Result<Arc<Fleet>, Reply> {
        let asked = Instant::now();
        let mut latest = self.fleet.lock().await;
        if let Some((started, fleet)) = latest.as_ref() {
            if *started >= asked {
                return Ok(fleet.clone());
            }
        }
        let started = Instant::now();
        let roots = self.options.roots.clone();
        let facts = self.facts.clone();
        let fleet = Arc::new(
            tokio::task::spawn_blocking(move || Fleet::read(&roots, &facts))
                .await
                .map_err(|error| Reply::error(StatusCode::INTERNAL_SERVER_ERROR, error))?,
        );
        let current = fleet
            .repos
            .iter()
            .filter_map(|entry| entry.located.as_ref().ok())
            .map(|located| (located.common_dir.clone(), located.base.commit.clone()))
            .collect::<HashMap<_, _>>();
        self.snapshots
            .lock()
            .expect("snapshot lock")
            .retain(|dir, snapshot| current.get(dir) == Some(&snapshot.commit));
        *latest = Some((started, fleet.clone()));
        Ok(fleet)
    }

    fn snapshot(&self, located: &Located) -> Arc<Snapshot> {
        let mut snapshots = self.snapshots.lock().expect("snapshot lock");
        match snapshots.get(&located.common_dir) {
            Some(snapshot) if snapshot.commit == located.base.commit => snapshot.clone(),
            _ => {
                let snapshot = Arc::new(Snapshot {
                    common_dir: located.common_dir.clone(),
                    commit: located.base.commit.clone(),
                    loaded: OnceLock::new(),
                    views: Mutex::new(HashMap::new()),
                });
                snapshots.insert(located.common_dir.clone(), snapshot.clone());
                snapshot
            }
        }
    }

    /// Runs `work` on a blocking thread once one of the build slots is free,
    /// unless `cached` answers first, before or after the wait. A wait longer
    /// than `wait` is a 503.
    async fn build<T: Send + 'static>(
        &self,
        cached: impl Fn() -> Option<T>,
        work: impl FnOnce() -> T + Send + 'static,
    ) -> Result<T, Reply> {
        if let Some(value) = cached() {
            return Ok(value);
        }
        let _permit = tokio::time::timeout(self.options.wait, self.builds.acquire())
            .await
            .map_err(|_| {
                Reply::json(
                    StatusCode::SERVICE_UNAVAILABLE,
                    &ApiError {
                        error: format!(
                            "the server waited {} s for one of its {} view builds; try again",
                            self.options.wait.as_secs(),
                            self.options.max_builds
                        ),
                    },
                )
            })?
            .expect("the build semaphore is never closed");
        if let Some(value) = cached() {
            return Ok(value);
        }
        self.built.fetch_add(1, Ordering::Relaxed);
        tokio::task::spawn_blocking(work)
            .await
            .map_err(|error| Reply::error(StatusCode::INTERNAL_SERVER_ERROR, error))
    }

    /// The records of a repository's base commit, loaded once per commit.
    async fn loaded(&self, snapshot: &Arc<Snapshot>) -> Result<Arc<Loaded>, Reply> {
        let shared = snapshot.clone();
        let loaded = self
            .build(
                || snapshot.loaded.get().cloned(),
                move || shared.loaded.get_or_init(|| shared.load()).clone(),
            )
            .await?;
        loaded.map_err(|error| Reply::json(StatusCode::INTERNAL_SERVER_ERROR, &ApiError { error }))
    }

    /// A view of one mission, built once per base commit.
    async fn view(
        &self,
        snapshot: &Arc<Snapshot>,
        mission: &str,
        kind: ViewKind,
    ) -> Result<Reply, Reply> {
        let key = (mission.to_owned(), kind);
        let cached = || snapshot.views.lock().expect("view lock").get(&key).cloned();
        let shared = snapshot.clone();
        let id = mission.to_owned();
        let reply = self
            .build(cached, move || {
                let reply = match shared.loaded.get_or_init(|| shared.load()) {
                    Ok(loaded) => shared.build_view(loaded, &id, kind),
                    Err(error) => Reply::json(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &ApiError {
                            error: error.clone(),
                        },
                    ),
                };
                shared
                    .views
                    .lock()
                    .expect("view lock")
                    .insert((id, kind), reply.clone());
                reply
            })
            .await?;
        Ok(reply)
    }
}

/// One scan of the roots: every repository found, by the name in its URLs.
struct Fleet {
    repos: Vec<FleetEntry>,
}

struct FleetEntry {
    repo: String,
    located: std::result::Result<Located, String>,
}

struct Located {
    common_dir: PathBuf,
    base: Base,
}

impl Fleet {
    /// Scans `roots`, reading each repository's refs afresh and the facts of
    /// its commits from `facts` where they are known. Afterwards `facts`
    /// keeps only the commits this scan read.
    fn read(roots: &[PathBuf], facts: &Mutex<HashMap<(PathBuf, String), CommitFacts>>) -> Self {
        let mut taken = HashMap::<String, usize>::new();
        let mut known = std::mem::take(&mut *facts.lock().expect("facts lock"));
        let mut kept = HashMap::new();
        let repos = discovery::discover(roots)
            .into_iter()
            .map(|found| {
                let located = found.common_dir.clone().and_then(|common_dir| {
                    let mut facts_of = |commit: &str| {
                        let key = (common_dir.clone(), commit.to_owned());
                        let facts = known
                            .remove(&key)
                            .or_else(|| kept.get(&key).cloned())
                            .unwrap_or_else(|| discovery::commit_facts(&common_dir, commit));
                        kept.insert(key, facts.clone());
                        facts
                    };
                    discovery::base(&common_dir, &mut facts_of)
                        .map(|base| Located {
                            common_dir: common_dir.clone(),
                            base,
                        })
                        .map_err(|error| format!("{error:#}"))
                });
                let name = located
                    .as_ref()
                    .ok()
                    .and_then(|located| located.base.slug.clone())
                    .unwrap_or_else(|| found.directory_name());
                let seen = taken.entry(name.clone()).or_default();
                *seen += 1;
                let repo = match *seen {
                    1 => name,
                    count => format!("{name}-{count}"),
                };
                FleetEntry { repo, located }
            })
            .collect();
        *facts.lock().expect("facts lock") = kept;
        Self { repos }
    }

    fn get(&self, repo: &str) -> Result<&FleetEntry, Reply> {
        self.repos
            .iter()
            .find(|entry| entry.repo == repo)
            .ok_or_else(|| {
                Reply::json(
                    StatusCode::NOT_FOUND,
                    &ApiError {
                        error: format!(
                            "no Atelier repository named {repo}; known: {}",
                            self.repos
                                .iter()
                                .map(|entry| entry.repo.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    },
                )
            })
    }
}

/// What has been read and built from one base commit of one repository.
struct Snapshot {
    common_dir: PathBuf,
    commit: String,
    loaded: OnceLock<std::result::Result<Arc<Loaded>, String>>,
    views: Mutex<HashMap<(String, ViewKind), Reply>>,
}

struct Loaded {
    records: CommitRecords,
    counts: RepoCounts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ViewKind {
    Brief,
    Debrief,
    Strategy,
}

impl Snapshot {
    fn read(&self) -> ReadContext<'_> {
        ReadContext::at_commit(&self.common_dir, self.commit.clone())
    }

    fn load(&self) -> std::result::Result<Arc<Loaded>, String> {
        let mut read = self.read();
        let records = atelier_app::commit_records::load(&mut read)
            .map_err(|error| format!("cannot read the records at {}: {error:#}", self.commit))?;
        let status =
            atelier_app::work_classification::status_view(&records.index, Some(&records.policy))
                .map_err(|error| format!("{error:#}"))?;
        Ok(Arc::new(Loaded {
            counts: RepoCounts {
                work: (&status.work).into(),
                current_missions: status.current_mission_count,
            },
            records,
        }))
    }

    fn build_view(&self, loaded: &Loaded, mission: &str, kind: ViewKind) -> Reply {
        let mut read = self.read();
        let records = &loaded.records;
        let base_commit = self.commit.clone();
        let built = match kind {
            ViewKind::Brief => atelier_app::mission_review::brief(
                &records.index,
                Some(&records.policy),
                &mut read,
                mission,
            )
            .map(|brief| Reply::json(StatusCode::OK, &BriefResponse { base_commit, brief })),
            ViewKind::Debrief => atelier_app::mission_review::debrief(
                &records.index,
                Some(&records.policy),
                &mut read,
                mission,
            )
            .map(|debrief| {
                Reply::json(
                    StatusCode::OK,
                    &DebriefResponse {
                        base_commit,
                        debrief,
                    },
                )
            }),
            ViewKind::Strategy => Ok(self.strategy(records, &mut read, mission)),
        };
        built.unwrap_or_else(|error| {
            Reply::json(
                StatusCode::INTERNAL_SERVER_ERROR,
                &ApiError {
                    error: format!("{error:#}"),
                },
            )
        })
    }

    fn strategy(
        &self,
        records: &CommitRecords,
        read: &mut ReadContext<'_>,
        mission: &str,
    ) -> Reply {
        let Some(path) = records
            .index
            .issue_record(mission)
            .and_then(|record| record.issue.strategy.as_ref())
            .map(|strategy| strategy.path.clone())
        else {
            return Reply::json(
                StatusCode::NOT_FOUND,
                &ApiError {
                    error: format!("mission {mission} links no strategy"),
                },
            );
        };
        match atelier_app::commit_records::file_at(read, &path) {
            Ok(markdown) => Reply::json(
                StatusCode::OK,
                &StrategyResponse {
                    base_commit: self.commit.clone(),
                    mission: mission.to_owned(),
                    path,
                    markdown,
                },
            ),
            Err(error) => Reply::json(
                StatusCode::NOT_FOUND,
                &ApiError {
                    error: format!(
                        "mission {mission} links strategy {path}, which commit {} does not hold: {error:#}",
                        self.commit
                    ),
                },
            ),
        }
    }
}

/// A JSON response, cheap to clone so a built view can be kept and served
/// again.
#[derive(Clone)]
struct Reply {
    status: StatusCode,
    body: Arc<[u8]>,
}

impl Reply {
    fn json<T: Serialize>(status: StatusCode, value: &T) -> Self {
        match serde_json::to_vec(value) {
            Ok(body) => Self {
                status,
                body: body.into(),
            },
            Err(error) => Self::error(StatusCode::INTERNAL_SERVER_ERROR, error),
        }
    }

    fn error(status: StatusCode, error: impl std::fmt::Display) -> Self {
        let body = serde_json::to_vec(&ApiError {
            error: error.to_string(),
        })
        .expect("an error message serializes");
        Self {
            status,
            body: body.into(),
        }
    }
}

impl IntoResponse for Reply {
    fn into_response(self) -> Response {
        let mut response = Response::new(Body::from(self.body.to_vec()));
        *response.status_mut() = self.status;
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        if self.status == StatusCode::SERVICE_UNAVAILABLE {
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, HeaderValue::from_static("1"));
        }
        response
    }
}

fn mission_links(repo: &str, id: &str, strategy: bool) -> MissionLinks {
    let summary = format!("/api/{repo}/mission/{id}");
    MissionLinks {
        brief: format!("{summary}/brief"),
        debrief: format!("{summary}/debrief"),
        strategy: strategy.then(|| format!("{summary}/strategy")),
        summary,
    }
}

fn mission_summary(repo: &str, records: &CommitRecords, id: &str) -> Option<MissionSummary> {
    let issue = &records.index.issue_record(id)?.issue;
    Some(MissionSummary {
        id: issue.id.clone(),
        title: issue.title.clone(),
        status: issue.status.clone(),
        status_category: records
            .policy
            .status_category(&issue.status)
            .map(str::to_owned),
        review: Review::for_mission(&records.policy, issue),
        links: mission_links(repo, &issue.id, issue.strategy.is_some()),
    })
}

/// The router of every API route, over `site`.
pub fn router(site: Arc<Site>) -> Router {
    Router::new()
        .route("/api/repos", get(repos))
        .route("/api/{repo}/mission/{id}", get(mission))
        .route("/api/{repo}/mission/{id}/brief", get(brief))
        .route("/api/{repo}/mission/{id}/debrief", get(debrief))
        .route("/api/{repo}/mission/{id}/strategy", get(strategy))
        .fallback(|| async {
            Reply::json(
                StatusCode::NOT_FOUND,
                &ApiError {
                    error: "no such route; the API is GET /api/repos and /api/<repo>/mission/<id>[/brief|/debrief|/strategy]".into(),
                },
            )
        })
        .with_state(site)
}

async fn repos(State(site): State<Arc<Site>>) -> Reply {
    match repo_summaries(&site).await {
        Ok(repos) => Reply::json(StatusCode::OK, &ReposResponse { repos }),
        Err(reply) => reply,
    }
}

async fn repo_summaries(site: &Site) -> Result<Vec<RepoSummary>, Reply> {
    let fleet = site.fleet().await?;
    let mut repos = Vec::new();
    for entry in &fleet.repos {
        let mut summary = RepoSummary {
            repo: entry.repo.clone(),
            base_branch: None,
            base_commit: None,
            error: None,
            counts: None,
            missions: Vec::new(),
        };
        match &entry.located {
            Err(error) => summary.error = Some(error.clone()),
            Ok(located) => {
                summary.base_branch = Some(located.base.branch.clone());
                summary.base_commit = Some(located.base.commit.clone());
                let snapshot = site.snapshot(located);
                match site.loaded(&snapshot).await {
                    Ok(loaded) => {
                        summary.counts = Some(loaded.counts.clone());
                        summary.missions = loaded
                            .records
                            .index
                            .issues()
                            .filter(|issue| issue.issue_type == "mission")
                            .filter_map(|issue| {
                                mission_summary(&entry.repo, &loaded.records, &issue.id)
                            })
                            .collect();
                    }
                    Err(reply) if reply.status == StatusCode::SERVICE_UNAVAILABLE => {
                        return Err(reply)
                    }
                    Err(reply) => summary.error = Some(error_text(&reply)),
                }
            }
        }
        repos.push(summary);
    }
    Ok(repos)
}

fn error_text(reply: &Reply) -> String {
    serde_json::from_slice::<serde_json::Value>(&reply.body)
        .ok()
        .and_then(|value| value.get("error")?.as_str().map(str::to_owned))
        .unwrap_or_default()
}

/// The repository's snapshot, its records and the mission's full ID: a 404
/// for an unknown repository, mission or a record that is not a mission,
/// and a 500 for a repository whose base or records cannot be read.
async fn locate(
    site: &Site,
    repo: &str,
    mission: &str,
) -> Result<(Arc<Snapshot>, Arc<Loaded>, String), Reply> {
    let fleet = site.fleet().await?;
    let entry = fleet.get(repo)?;
    let located = entry.located.as_ref().map_err(|error| {
        Reply::json(
            StatusCode::INTERNAL_SERVER_ERROR,
            &ApiError {
                error: error.clone(),
            },
        )
    })?;
    let snapshot = site.snapshot(located);
    let loaded = site.loaded(&snapshot).await?;
    let index = &loaded.records.index;
    let not_found = |error: String| Reply::json(StatusCode::NOT_FOUND, &ApiError { error });
    let id = index
        .resolve_issue_ref(mission)
        .ok()
        .flatten()
        .ok_or_else(|| not_found(format!("no mission {mission} in {repo}")))?;
    let issue = &index
        .issue_record(&id)
        .expect("a resolved issue exists")
        .issue;
    if issue.issue_type != "mission" {
        return Err(not_found(format!(
            "{id} is {}, not a mission",
            atelier_app::record_ref::with_article(&issue.issue_type)
        )));
    }
    Ok((snapshot, loaded, id))
}

async fn mission(
    State(site): State<Arc<Site>>,
    UrlPath((repo, mission)): UrlPath<(String, String)>,
) -> Reply {
    match locate(&site, &repo, &mission).await {
        Ok((snapshot, loaded, id)) => Reply::json(
            StatusCode::OK,
            &MissionResponse {
                mission: mission_summary(&repo, &loaded.records, &id)
                    .expect("a located mission exists"),
                repo,
                base_commit: snapshot.commit.clone(),
            },
        ),
        Err(reply) => reply,
    }
}

async fn view_route(site: &Site, repo: &str, mission: &str, kind: ViewKind) -> Reply {
    let (snapshot, _, id) = match locate(site, repo, mission).await {
        Ok(located) => located,
        Err(reply) => return reply,
    };
    site.view(&snapshot, &id, kind)
        .await
        .unwrap_or_else(|reply| reply)
}

async fn brief(
    State(site): State<Arc<Site>>,
    UrlPath((repo, mission)): UrlPath<(String, String)>,
) -> Reply {
    view_route(&site, &repo, &mission, ViewKind::Brief).await
}

async fn debrief(
    State(site): State<Arc<Site>>,
    UrlPath((repo, mission)): UrlPath<(String, String)>,
) -> Reply {
    view_route(&site, &repo, &mission, ViewKind::Debrief).await
}

async fn strategy(
    State(site): State<Arc<Site>>,
    UrlPath((repo, mission)): UrlPath<(String, String)>,
) -> Reply {
    view_route(&site, &repo, &mission, ViewKind::Strategy).await
}

/// Requests answered in process, through the router with no socket, for
/// tests and tools that read the API without serving it.
pub struct Client {
    runtime: tokio::runtime::Runtime,
    router: Router,
    site: Arc<Site>,
}

impl Client {
    pub fn new(options: SiteOptions) -> Self {
        let site = Arc::new(Site::new(options));
        Self {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .expect("a runtime starts"),
            router: router(site.clone()),
            site,
        }
    }

    pub fn site(&self) -> &Site {
        &self.site
    }

    /// The status and body of `GET path`.
    pub fn get(&self, path: &str) -> (u16, String) {
        self.get_all(&[path.to_owned()]).remove(0)
    }

    /// The status and body of `GET` on each path, all sent at once.
    pub fn get_all(&self, paths: &[String]) -> Vec<(u16, String)> {
        self.runtime.block_on(async {
            let requests = paths
                .iter()
                .map(|path| {
                    let router = self.router.clone();
                    let request = axum::http::Request::get(path.as_str())
                        .body(Body::empty())
                        .expect("a GET request builds");
                    tokio::spawn(async move {
                        use tower::ServiceExt;
                        let response = router.oneshot(request).await.expect("the router answers");
                        let status = response.status().as_u16();
                        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                            .await
                            .expect("a body reads");
                        (status, String::from_utf8_lossy(&body).into_owned())
                    })
                })
                .collect::<Vec<_>>();
            let mut answers = Vec::new();
            for request in requests {
                answers.push(request.await.expect("a request task finishes"));
            }
            answers
        })
    }
}

/// `~/projects`, the default root.
pub fn default_root() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| Path::new(&home).join("projects"))
}

/// Serves the API on `bind` until interrupted, on a small runtime: two
/// worker threads and four blocking threads, so the host's other services
/// keep their cores. Once listening it prints what it serves, or with
/// `quiet` only its URL. A root that is not a directory is refused first.
pub fn serve(bind: SocketAddr, roots: Vec<PathBuf>, quiet: bool) -> Result<()> {
    if let Some(root) = roots.iter().find(|root| !root.is_dir()) {
        anyhow::bail!(
            "{} is not a directory; name a directory holding Atelier repositories with --root",
            root.display()
        );
    }
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .max_blocking_threads(4)
        .thread_name("atelier-serve")
        .enable_all()
        .build()
        .context("cannot start the server's runtime")?;
    runtime.block_on(async move {
        let listener = tokio::net::TcpListener::bind(bind)
            .await
            .with_context(|| format!("cannot listen on {bind}"))?;
        let address = listener.local_addr()?;
        if quiet {
            println!("http://{address}");
        } else {
            println!(
                "Serving every Atelier repository under {} at http://{address}/api/repos",
                roots
                    .iter()
                    .map(|root| root.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        let site = Arc::new(Site::new(SiteOptions::new(roots)));
        axum::serve(listener, router(site))
            .with_graceful_shutdown(async {
                let _ = tokio::signal::ctrl_c().await;
            })
            .await
            .context("the server stopped")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    /// At most `max_builds` builds run at once, and a request that cannot
    /// get a slot within `wait` is a 503 that names the wait.
    #[test]
    fn builds_are_capped_and_a_long_wait_is_a_503() {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap();
        let site = Arc::new(Site::new(SiteOptions {
            wait: Duration::from_millis(300),
            ..SiteOptions::new(Vec::new())
        }));
        let running = Arc::new(AtomicUsize::new(0));
        let most = Arc::new(AtomicUsize::new(0));
        let answers = runtime.block_on(async {
            let tasks = (0..3)
                .map(|_| {
                    let (site, running, most) = (site.clone(), running.clone(), most.clone());
                    tokio::spawn(async move {
                        site.build(
                            || None,
                            move || {
                                let now = running.fetch_add(1, Ordering::SeqCst) + 1;
                                most.fetch_max(now, Ordering::SeqCst);
                                std::thread::sleep(Duration::from_millis(1000));
                                running.fetch_sub(1, Ordering::SeqCst);
                            },
                        )
                        .await
                        .map_err(|reply| (reply.status, error_text(&reply)))
                    })
                })
                .collect::<Vec<_>>();
            let mut answers = Vec::new();
            for task in tasks {
                answers.push(task.await.unwrap());
            }
            answers
        });
        assert_eq!(most.load(Ordering::SeqCst), 2);
        let refused = answers
            .iter()
            .filter_map(|answer| answer.as_ref().err())
            .collect::<Vec<_>>();
        assert_eq!(refused.len(), 1, "{refused:?}");
        assert_eq!(refused[0].0, StatusCode::SERVICE_UNAVAILABLE);
        assert!(refused[0]
            .1
            .contains("waited 0 s for one of its 2 view builds"));
        assert_eq!(site.builds_run(), 2);
    }
}
