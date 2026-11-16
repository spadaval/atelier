//! The reads behind the views a host such as T3 Code shows beside its
//! threads (docs/architecture/desk.md, Reads for a host's views): its
//! sidebar's missions, a project's strip, any thread by its identity,
//! the records' history from commit subjects, CI on a pull request, and the
//! items in no mission. Each read that runs a command bounds it and keeps
//! its answer a while, and none puts a value from the request on a command
//! line unchecked.

use super::{blocking, Desk, Failure};
use crate::api::{
    CheckRun, ChecksResponse, ChecksState, EventKind, HistoryEvent, HistoryResponse, HoldKind,
    InMotion, ItemRow, LaunchReady, LaunchResponse, MissionState, MotionHolder, Priority,
    RoadmapResponse, Role, SideItem, SideMission, SideRepo, SideState, SideThread, SidebarResponse,
    Status, WaitingRecord, WaitingThread,
};
use crate::{bounded, history, read};
use atelier_core::Repo;
use axum::extract::{Query, State};
use axum::response::Response;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

/// How long CI on a pull request is kept, and how long `gh` may take to
/// read it.
const CHECKS_KEPT: Duration = Duration::from_secs(60);
const GH_LIMIT: Duration = Duration::from_secs(15);
/// How long `git for-each-ref` may take to list the branches' times.
const REFS_LIMIT: Duration = Duration::from_secs(10);
/// The most pull requests whose CI is kept at once.
const CHECKS_HELD: usize = 200;
/// How far back `/api/launch` counts an item's submits and finds its steps.
const LAUNCH_DAYS: u32 = 60;

/// One repository's open missions, each with the threads on it and on its
/// items.
fn side_repo(repo: &mut Repo, name: &str) -> anyhow::Result<Option<SideRepo>> {
    let rows = read::missions(repo, false)?.map(|m| m.missions);
    let mut missions = Vec::new();
    for row in rows.unwrap_or_default() {
        let state = match row.state {
            MissionState::Proposed => SideState::Proposed,
            MissionState::Open if row.in_progress => SideState::InProgress,
            MissionState::Open => SideState::Agreed,
            _ => continue,
        };
        let Some(page) = read::mission(repo, &row.id)? else {
            continue;
        };
        let side = |a: &crate::api::Agent| SideThread {
            identity: a.identity.clone(),
            role: a.role,
        };
        let on_mission = page.threads.iter().filter(|a| a.item.is_none());
        let items = (page.items.iter())
            .map(|i| SideItem {
                id: i.id.clone(),
                title: i.title.clone(),
                status: i.status,
                threads: (page.threads.iter())
                    .filter(|a| a.item.as_deref() == Some(i.id.as_str()))
                    .filter(|a| matches!(a.role, Role::Implementer | Role::Reviewer))
                    .map(side)
                    .collect(),
            })
            .collect();
        let count = |done: bool| -> usize {
            let counted = row
                .counts
                .iter()
                .filter(|c| !done || c.status == Status::Done);
            counted.map(|c| c.count).sum()
        };
        missions.push(SideMission {
            id: row.id.clone(),
            title: row.title.clone(),
            state,
            done: count(true),
            total: count(false),
            waiting: row.waiting.len(),
            threads: on_mission.map(side).collect(),
            items,
        });
    }
    Ok(Some(SideRepo {
        repo: name.to_string(),
        missions,
        error: None,
    }))
}

/// `GET /api/sidebar`.
pub async fn sidebar(State(desk): State<Arc<Desk>>) -> Response {
    blocking(move || {
        let snapshot = desk.threads.snapshot();
        let failed = |name: &str, error| SideRepo {
            repo: name.to_string(),
            missions: Vec::new(),
            error: Some(error),
        };
        let mut repos = desk.every("sidebar", side_repo, failed, |_, _, _| {});
        // A thread at work on a mission that waits on a person counts
        // against the mission.
        let waiting: HashSet<&str> = (snapshot.threads.iter())
            .filter(|t| t.status == "waiting")
            .map(|t| t.thread_id.as_str())
            .collect();
        for mission in repos.iter_mut().flat_map(|r| &mut r.missions) {
            let mut on: HashSet<&str> = HashSet::new();
            let threads = mission
                .threads
                .iter()
                .chain(mission.items.iter().flat_map(|i| &i.threads));
            on.extend(threads.map(|t| t.identity.as_str()));
            mission.waiting += on.intersection(&waiting).count();
        }
        Ok(SidebarResponse { repos })
    })
    .await
}

#[derive(Deserialize)]
pub struct RepoQuery {
    repo: String,
}

fn rank(priority: Priority) -> u8 {
    match priority {
        Priority::Critical => 0,
        Priority::High => 1,
        Priority::Medium => 2,
        Priority::Low => 3,
    }
}

/// `GET /api/launch?repo=`.
pub async fn launch(State(desk): State<Arc<Desk>>, Query(q): Query<RepoQuery>) -> Response {
    blocking(move || {
        let repo = q.repo;
        let dir = desk.served(&repo)?;
        let missing = || format!("No repository {repo}.");
        let mut motion = desk.repo_route(&repo, "motion", missing, |r| read::motion(r, &repo))?;
        let inbox = desk.repo_route(&repo, "inbox", missing, |r| read::inbox(r, &repo))?;
        let snapshot = desk.threads.snapshot();
        for item in &mut motion.items {
            let agents = item.holder.iter_mut().chain(item.reviewer.iter_mut());
            agents.for_each(|agent| snapshot.join(std::slice::from_mut(agent)));
        }
        let ready = (motion.ready.iter())
            .filter(|r| !r.needs_human)
            .map(|r| LaunchReady {
                id: r.id.clone(),
                title: r.title.clone(),
                priority: r.priority,
            })
            .collect();
        let live = snapshot.live(&desk.repo_holds(&repo, &dir));
        let threads_waiting: Vec<WaitingThread> = (live.into_iter())
            .filter(|t| t.status == "waiting" && !t.holds.is_empty())
            .map(|t| WaitingThread {
                identity: t.id,
                title: t.title,
            })
            .collect();
        let mut needing: Vec<&ItemRow> = inbox.needs_human.iter().flat_map(|g| &g.items).collect();
        needing.sort_by_key(|i| rank(i.priority));
        let plans = (inbox.proposed.iter()).map(|m| (m.id.clone(), m.title.clone()));
        let items = needing.iter().map(|i| (i.id.clone(), i.title.clone()));
        let waiting_items = (plans.chain(items).take(3))
            .map(|(id, title)| WaitingRecord { id, title })
            .collect();
        let now = chrono::Utc::now();
        // The history fails soft: without it, ages come from the branches.
        let recent: Vec<HistoryEvent> = (desk.history(&repo, &dir).unwrap_or_default().into_iter())
            .filter(|e| history::within(&e.at, LAUNCH_DAYS, now))
            .collect();
        let times = branch_times(&dir);
        let mut in_motion: Vec<InMotion> = (motion.items.iter())
            .map(|item| {
                let submits = (recent.iter())
                    .filter(|e| e.id == item.id && e.kind == EventKind::Submitted)
                    .count();
                let stage = match item.status {
                    Status::InReview => format!("in review {}", submits.max(1)),
                    Status::ChangesRequested => "changes requested".into(),
                    Status::Approved => "approved".into(),
                    _ => "building".into(),
                };
                let changed = item.branch.as_ref().and_then(|b| times.get(b)).cloned();
                let last = (recent.iter().find(|e| e.id == item.id)).map(|e| e.at.clone());
                let age_seconds =
                    (last.or(changed)).and_then(|at| history::seconds_since(&at, now));
                let agent = match item.status {
                    Status::InReview => item.reviewer.as_ref().or(item.holder.as_ref()),
                    _ => item.holder.as_ref(),
                };
                let live = agent.is_some_and(|a| a.thread.is_some());
                let holder = agent.map(|a| MotionHolder {
                    identity: a.identity.clone(),
                    title: (a.thread.as_ref().map(|t| t.title.clone()))
                        .or_else(|| desk.threads.info(&a.identity).map(|i| i.title)),
                });
                InMotion {
                    id: item.id.clone(),
                    title: item.title.clone(),
                    stage,
                    age_seconds,
                    holder,
                    live,
                }
            })
            .collect();
        in_motion.sort_by_key(|i| (!i.live, i.age_seconds.unwrap_or(i64::MAX)));
        let waiting = inbox
            .needs_human
            .iter()
            .map(|g| g.items.len())
            .sum::<usize>()
            + inbox.proposed.len()
            + threads_waiting.len();
        Ok(LaunchResponse {
            ready,
            waiting,
            threads_waiting,
            waiting_items,
            in_motion,
        })
    })
    .await
}

/// When each branch, local and fetched, last had a commit, by the name
/// `git` gives it short (`<branch>` or `<remote>/<branch>`), as RFC 3339.
/// Empty where `git` cannot list them.
pub fn branch_times(dir: &Path) -> HashMap<String, String> {
    let mut command = Command::new("git");
    command.arg("-C").arg(dir).args([
        "for-each-ref",
        "--format=%(refname:short)%09%(committerdate:iso-strict)",
        "refs/heads",
        "refs/remotes",
    ]);
    let listed = bounded::run(atelier_core::git::unattended(&mut command), REFS_LIMIT);
    let text = String::from_utf8_lossy(&listed.unwrap_or_default()).into_owned();
    let pairs = text.lines().filter_map(|line| {
        let (name, at) = line.split_once('\t')?;
        Some((name.to_string(), at.to_string()))
    });
    pairs.collect()
}

#[derive(Deserialize)]
pub struct HistoryQuery {
    repo: String,
    /// Only the last `days` days.
    days: Option<u32>,
    /// Only these IDs, comma-separated.
    ids: Option<String>,
}

impl Desk {
    /// The steps in the commit subjects of the repository `name` at `dir`,
    /// newest first, kept while its commits are unchanged.
    fn history(&self, name: &str, dir: &Path) -> anyhow::Result<Vec<HistoryEvent>> {
        let remote = self.remote(name, dir).0;
        let read = |r: &mut Repo| history::read(&r.root, remote.as_deref()).map(Some);
        Ok(self.read(name, dir, "history", read)?.unwrap_or_default())
    }
}

/// `GET /api/history?repo=&days=&ids=`.
pub async fn history(State(desk): State<Arc<Desk>>, Query(q): Query<HistoryQuery>) -> Response {
    blocking(move || {
        let dir = desk.served(&q.repo)?;
        let mut found =
            (desk.history(&q.repo, &dir)).map_err(|e| Failure::Failed(format!("{e:#}")))?;
        if let Some(days) = q.days {
            let now = chrono::Utc::now();
            found.retain(|e| history::within(&e.at, days, now));
        }
        if let Some(ids) = &q.ids {
            let wanted: HashSet<&str> = ids.split(',').collect();
            found.retain(|e| wanted.contains(e.id.as_str()));
        }
        let records: HashMap<String, (HoldKind, String)> = (desk.indexes().into_iter())
            .flat_map(|i| i.records)
            .filter(|r| r.repo == q.repo)
            .map(|r| (r.id, (r.kind, r.title)))
            .collect();
        for event in &mut found {
            if let Some((kind, title)) = records.get(&event.id) {
                event.record = Some(*kind);
                event.title = Some(title.clone());
            }
        }
        Ok(HistoryResponse {
            repo: q.repo,
            events: found,
        })
    })
    .await
}

#[derive(Deserialize)]
pub struct ChecksQuery {
    repo: String,
    pr: u32,
}

type ChecksCache = Mutex<HashMap<(String, u32), (Instant, ChecksResponse)>>;

fn checks_cache() -> &'static ChecksCache {
    static CACHE: OnceLock<ChecksCache> = OnceLock::new();
    CACHE.get_or_init(Mutex::default)
}

/// CI in `gh pr view --json statusCheckRollup`'s output.
pub fn rollup(value: &serde_json::Value) -> ChecksResponse {
    let listed = value["statusCheckRollup"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let checks: Vec<CheckRun> = (listed.iter())
        .map(|c| {
            let name = c["name"]
                .as_str()
                .or(c["context"].as_str())
                .unwrap_or("check");
            let state = (c["conclusion"].as_str().filter(|s| !s.is_empty()))
                .or(c["state"].as_str())
                .or(c["status"].as_str())
                .unwrap_or("UNKNOWN");
            CheckRun {
                name: name.to_string(),
                state: state.to_string(),
            }
        })
        .collect();
    let bad = [
        "FAILURE",
        "ERROR",
        "CANCELLED",
        "TIMED_OUT",
        "ACTION_REQUIRED",
    ];
    let good = ["SUCCESS", "SKIPPED", "NEUTRAL"];
    let state = if checks.is_empty() {
        ChecksState::None
    } else if checks.iter().any(|c| bad.contains(&c.state.as_str())) {
        ChecksState::Failing
    } else if checks.iter().all(|c| good.contains(&c.state.as_str())) {
        ChecksState::Passing
    } else {
        ChecksState::Pending
    };
    ChecksResponse { state, checks }
}

/// `GET /api/checks?repo=&pr=`: read with `gh`, kept a minute. The number
/// is a `u32` before it reaches `gh`'s command line.
pub async fn checks(State(desk): State<Arc<Desk>>, Query(q): Query<ChecksQuery>) -> Response {
    blocking(move || {
        let dir = desk.served(&q.repo)?;
        let key = (q.repo.clone(), q.pr);
        if let Some((at, kept)) = checks_cache().lock().unwrap().get(&key) {
            if at.elapsed() < CHECKS_KEPT {
                return Ok(kept.clone());
            }
        }
        let mut command = Command::new(&desk.config.gh);
        command.current_dir(&dir).env("GH_PROMPT_DISABLED", "1");
        command.args([
            "pr",
            "view",
            &q.pr.to_string(),
            "--json",
            "statusCheckRollup",
        ]);
        let out =
            bounded::run(&mut command, GH_LIMIT).map_err(|e| Failure::Failed(format!("{e:#}")))?;
        let value: serde_json::Value = serde_json::from_slice(&out)
            .map_err(|e| Failure::Failed(format!("gh gave no JSON: {e}")))?;
        let found = rollup(&value);
        let mut cache = checks_cache().lock().unwrap();
        cache.retain(|_, (at, _)| at.elapsed() < CHECKS_KEPT);
        if cache.len() < CHECKS_HELD {
            cache.insert(key, (Instant::now(), found.clone()));
        }
        Ok(found)
    })
    .await
}

/// `GET /api/roadmap?repo=`.
pub async fn roadmap(State(desk): State<Arc<Desk>>, Query(q): Query<RepoQuery>) -> Response {
    blocking(move || {
        let repo = q.repo;
        let missing = || format!("No repository {repo}.");
        let loose = desk.repo_route(&repo, "loose", missing, read::loose)?;
        Ok(RoadmapResponse { repo, loose })
    })
    .await
}

#[derive(Deserialize)]
pub struct ThreadQuery {
    id: String,
    /// Present, with any value, for the thread's latest run and line.
    full: Option<String>,
}

/// The longest thread identity `/api/thread` looks up.
const IDENTITY_LENGTH: usize = 512;

/// `GET /api/thread?id=<identity>[&full=1]`: the identity goes to T3 Code
/// as a value in a JSON request, never on a command line.
pub async fn thread(State(desk): State<Arc<Desk>>, Query(q): Query<ThreadQuery>) -> Response {
    blocking(move || {
        let id = q.id;
        if id.is_empty() || id.len() > IDENTITY_LENGTH || id.chars().any(char::is_control) {
            return Err(Failure::BadRequest(format!(
                "A thread identity is 1 to {IDENTITY_LENGTH} characters with no control character."
            )));
        }
        let found = match q.full {
            Some(_) => desk.threads.info_full(&id),
            None => desk.threads.info(&id),
        };
        found.ok_or_else(|| Failure::NotFound(format!("T3 Code knows no thread {id}.")))
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::rollup;
    use crate::api::ChecksState;
    use serde_json::json;

    #[test]
    fn ci_is_read_from_the_rollup() {
        let state =
            |checks: serde_json::Value| rollup(&json!({ "statusCheckRollup": checks })).state;
        assert_eq!(state(json!([])), ChecksState::None);
        let ok = json!({"name": "rust", "conclusion": "SUCCESS", "status": "COMPLETED"});
        let skipped = json!({"name": "desk", "conclusion": "SKIPPED"});
        let running = json!({"name": "e2e", "conclusion": "", "status": "IN_PROGRESS"});
        let failed = json!({"context": "lint", "state": "FAILURE"});
        assert_eq!(state(json!([ok, skipped])), ChecksState::Passing);
        assert_eq!(state(json!([ok, running])), ChecksState::Pending);
        assert_eq!(state(json!([ok, running, failed])), ChecksState::Failing);
        let read = rollup(&json!({ "statusCheckRollup": [failed] }));
        assert_eq!(
            (read.checks[0].name.as_str(), read.checks[0].state.as_str()),
            ("lint", "FAILURE")
        );
    }
}
