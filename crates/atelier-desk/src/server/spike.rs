//! Spike reads for T3 Code's sidebar, quick launch, mission control and
//! the project roadmap. Rough: no tests, no generated types.

use super::{blocking, Desk, Failure};
use crate::api::{MissionState, Role};
use crate::read;
use axum::extract::{Query, State};
use axum::response::Response;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

#[derive(Clone, Serialize)]
pub struct SideThread {
    identity: String,
    role: Role,
}

#[derive(Clone, Serialize)]
pub struct SideItem {
    id: String,
    title: String,
    status: crate::api::Status,
    threads: Vec<SideThread>,
}

#[derive(Clone, Serialize)]
pub struct SideMission {
    id: String,
    title: String,
    /// `in_progress`, `agreed` or `proposed`.
    state: &'static str,
    done: usize,
    total: usize,
    waiting: usize,
    threads: Vec<SideThread>,
    items: Vec<SideItem>,
}

#[derive(Clone, Serialize)]
pub struct SideRepo {
    repo: String,
    missions: Vec<SideMission>,
    /// Added: why the repository could not be read, when it could not.
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
pub struct Sidebar {
    repos: Vec<SideRepo>,
}

/// One repository's open missions with their threads and items' threads.
fn side_repo(repo: &mut atelier_core::Repo, name: &str) -> anyhow::Result<Option<SideRepo>> {
    let rows = read::missions(repo, false)?.map(|m| m.missions).unwrap_or_default();
    let mut missions = Vec::new();
    for row in rows {
        let state = match row.state {
            MissionState::Proposed => "proposed",
            MissionState::Open if row.in_progress => "in_progress",
            MissionState::Open => "agreed",
            _ => continue,
        };
        let Some(page) = read::mission(repo, &row.id)? else {
            continue;
        };
        let side = |a: &crate::api::Agent| SideThread {
            identity: a.identity.clone(),
            role: a.role,
        };
        let threads = page.threads.iter().filter(|a| a.item.is_none()).map(side).collect();
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
        let total = row.counts.iter().map(|c| c.count as usize).sum();
        let done = (row.counts.iter())
            .filter(|c| matches!(c.status, crate::api::Status::Done))
            .map(|c| c.count as usize)
            .sum();
        missions.push(SideMission {
            id: row.id.clone(),
            title: row.title.clone(),
            state,
            done,
            total,
            waiting: row.waiting.len(),
            threads,
            items,
        });
    }
    Ok(Some(SideRepo {
        repo: name.to_string(),
        missions,
        error: None,
    }))
}

/// `GET /api/sidebar`: every open mission in every repository, for T3
/// Code's sidebar.
pub async fn sidebar(State(desk): State<Arc<Desk>>) -> Response {
    blocking(move || {
        let snapshot = desk.threads.snapshot();
        let mut repos = desk.every(
            "spike-sidebar",
            side_repo,
            |name, error| SideRepo {
                repo: name.to_string(),
                missions: Vec::new(),
                error: Some(error),
            },
            |_, _, _| {},
        );
        // Threads waiting on a person count against the mission they work in.
        for repo in &mut repos {
            for mission in &mut repo.missions {
                let mut ids: HashSet<&str> = mission.threads.iter().map(|t| t.identity.as_str()).collect();
                for item in &mission.items {
                    ids.extend(item.threads.iter().map(|t| t.identity.as_str()));
                }
                let waiting = (snapshot.threads.iter())
                    .filter(|t| t.status == "waiting" && ids.contains(t.thread_id.as_str()))
                    .count();
                mission.waiting += waiting;
            }
        }
        Ok(Sidebar { repos })
    })
    .await
}

#[derive(Deserialize)]
pub struct RepoQuery {
    repo: String,
}

#[derive(Serialize)]
struct Ready {
    id: String,
    title: String,
    priority: crate::api::Priority,
}

#[derive(Serialize)]
struct WaitingThread {
    identity: String,
    title: String,
}

#[derive(Serialize)]
struct Named {
    id: String,
    title: String,
}

#[derive(Serialize)]
struct Holder {
    identity: String,
    title: Option<String>,
}

#[derive(Serialize)]
struct InMotion {
    id: String,
    title: String,
    /// `building`, `in review N`, `changes requested` or `approved`.
    stage: String,
    /// Since its latest recorded step.
    age_seconds: Option<i64>,
    holder: Option<Holder>,
    /// Added: whether its holder is a live thread in T3 Code.
    live: bool,
}

#[derive(Serialize)]
pub struct Launch {
    ready: Vec<Ready>,
    waiting: usize,
    threads_waiting: Vec<WaitingThread>,
    /// The top three things waiting on the human here.
    waiting_items: Vec<Named>,
    /// Every started item here, live first.
    in_motion: Vec<InMotion>,
}

/// `GET /api/launch?repo=`: what can start now in one repository and
/// what waits on the human there.
pub async fn launch(State(desk): State<Arc<Desk>>, Query(RepoQuery { repo }): Query<RepoQuery>) -> Response {
    blocking(move || {
        let dir = desk.served(&repo)?;
        let missing = || format!("No repository {repo}.");
        let mut motion = desk.repo_route(&repo, "motion", missing, |r| read::motion(r, &repo))?;
        let snapshot0 = desk.threads.snapshot();
        for item in &mut motion.items {
            let agents = item.holder.iter_mut().chain(item.reviewer.iter_mut());
            agents.for_each(|agent| snapshot0.join(std::slice::from_mut(agent)));
        }
        let inbox = desk.repo_route(&repo, "inbox", missing, |r| read::inbox(r, &repo))?;
        let ready = (motion.ready.iter())
            .filter(|r| !r.needs_human)
            .map(|r| Ready {
                id: r.id.clone(),
                title: r.title.clone(),
                priority: r.priority,
            })
            .collect();
        let snapshot = desk.threads.snapshot();
        let live = snapshot.live(&desk.repo_holds(&repo, &dir));
        let threads_waiting: Vec<WaitingThread> = (live.into_iter())
            .filter(|t| t.status == "waiting" && !t.holds.is_empty())
            .map(|t| WaitingThread {
                identity: t.id,
                title: t.title,
            })
            .collect();
        let needs: usize = inbox.needs_human.iter().map(|g| g.items.len()).sum();
        // Waiting on him: proposed plans first (they go ahead unless he
        // stops them), then needs-human items by priority.
        let mut needing: Vec<&crate::api::ItemRow> = inbox.needs_human.iter().flat_map(|g| &g.items).collect();
        let rank = |p: &crate::api::Priority| match p {
            crate::api::Priority::Critical => 0,
            crate::api::Priority::High => 1,
            crate::api::Priority::Medium => 2,
            crate::api::Priority::Low => 3,
        };
        needing.sort_by_key(|i| rank(&i.priority));
        let waiting_items = (inbox.proposed.iter().map(|m| Named { id: m.id.clone(), title: m.title.clone() }))
            .chain(needing.iter().map(|i| Named { id: i.id.clone(), title: i.title.clone() }))
            .take(3)
            .collect();
        let recent = events(&dir, Some(60)).unwrap_or_default();
        let now = chrono_now();
        let mut in_motion: Vec<InMotion> = (motion.items.iter())
            .map(|item| {
                let submits = recent.iter().filter(|e| e.id == item.id && e.kind == "submitted").count();
                let stage = match item.status {
                    crate::api::Status::InReview => format!("in review {}", submits.max(1)),
                    crate::api::Status::ChangesRequested => "changes requested".into(),
                    crate::api::Status::Approved => "approved".into(),
                    _ => "building".into(),
                };
                let last = recent.iter().find(|e| e.id == item.id).map(|e| e.at.clone()).or(item.changed_at.clone());
                let age_seconds = last.and_then(|at| seconds_since(&at, now));
                let agent = if matches!(item.status, crate::api::Status::InReview) { item.reviewer.as_ref().or(item.holder.as_ref()) } else { item.holder.as_ref() };
                let live = agent.and_then(|a| a.thread.as_ref()).is_some();
                let holder = agent.map(|a| Holder {
                    identity: a.identity.clone(),
                    title: a.thread.as_ref().map(|t| t.title.clone()).or_else(|| desk.threads.info(&a.identity).map(|i| i.title)),
                });
                InMotion { id: item.id.clone(), title: item.title.clone(), stage, age_seconds, holder, live }
            })
            .collect();
        in_motion.sort_by_key(|i| (!i.live, i.age_seconds.unwrap_or(i64::MAX)));
        Ok(Launch {
            ready,
            waiting: needs + inbox.proposed.len() + threads_waiting.len(),
            threads_waiting,
            waiting_items,
            in_motion,
        })
    })
    .await
}

/// One step in the records' history, read from a commit's subject.
#[derive(Clone, Serialize)]
pub struct Event {
    commit: String,
    at: String,
    /// `planned`, `plan_review`, `started`, `submitted`, `review`,
    /// `merged`, `readied`, `closed`, `dropped`, `led`.
    kind: &'static str,
    id: String,
    /// `item` or `mission`, where the repository holds the ID.
    record: Option<&'static str>,
    /// The record's title, where the repository holds the ID.
    title: Option<String>,
    round: Option<u32>,
    verdict: Option<String>,
    subject: String,
}

#[derive(Serialize)]
pub struct History {
    repo: String,
    events: Vec<Event>,
}

#[derive(Deserialize)]
pub struct HistoryQuery {
    repo: String,
    /// Only the last `days` days.
    days: Option<u32>,
    /// Only these IDs, comma-separated.
    ids: Option<String>,
}

fn ids_in(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for word in text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-')) {
        let Some((head, tail)) = word.rsplit_once('-') else { continue };
        if !head.is_empty()
            && head.starts_with(|c: char| c.is_ascii_lowercase())
            && tail.len() == 4
            && tail.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        {
            found.push(word.to_string());
        }
    }
    found
}

/// The events a commit subject records, if it records any.
fn parse(subject: &str) -> Vec<(&'static str, String, Option<u32>, Option<String>)> {
    let first = |rest: &str| ids_in(rest).into_iter().next();
    let round = |rest: &str| -> Option<(u32, String)> {
        let (_, after) = rest.split_once(", round ")?;
        let (n, verdict) = after.split_once(':')?;
        Some((n.trim().parse().ok()?, verdict.trim().to_string()))
    };
    if let Some(rest) = subject.strip_prefix("Review the plan of ") {
        if let (Some(id), Some((n, v))) = (first(rest), round(rest)) {
            return vec![("plan_review", id, Some(n), Some(v))];
        }
    } else if let Some(rest) = subject.strip_prefix("Review ") {
        if let (Some(id), Some((n, v))) = (first(rest), round(rest)) {
            return vec![("review", id, Some(n), Some(v))];
        }
    } else if let Some(rest) = subject.strip_prefix("Start ") {
        if let Some(id) = first(rest) {
            if rest.trim() == id {
                return vec![("started", id, None, None)];
            }
        }
    } else if let Some(rest) = subject.strip_prefix("Submit ") {
        if let Some(id) = first(rest) {
            return vec![("submitted", id, None, None)];
        }
    } else if let Some(rest) = subject.strip_prefix("Merge ") {
        if rest.starts_with("branch ") || rest.starts_with("master into") {
            return vec![];
        }
        let head = rest.split(':').next().unwrap_or(rest);
        let head = head.trim_end_matches(" into master").trim();
        if let Some(id) = first(head) {
            if head == id || head.ends_with(&format!("/{id}")) {
                return vec![("merged", id, None, None)];
            }
        }
    } else if let Some(rest) = subject.strip_prefix("Plan mission ") {
        if let Some(id) = first(rest) {
            return vec![("planned", id, None, None)];
        }
    } else if let Some(rest) = subject.strip_prefix("Ready ") {
        return ids_in(rest).into_iter().map(|id| ("readied", id, None, None)).collect();
    } else if let Some(rest) = subject.strip_prefix("Close ") {
        if let Some(id) = first(rest) {
            return vec![("closed", id, None, None)];
        }
    } else if let Some(rest) = subject.strip_prefix("Drop ") {
        if let Some(id) = first(rest) {
            if rest.trim() == id {
                return vec![("dropped", id, None, None)];
            }
        }
    } else if let Some(rest) = subject.strip_prefix("Lead ") {
        if let Some(id) = first(rest) {
            return vec![("led", id, None, None)];
        }
    }
    vec![]
}

fn events(dir: &Path, days: Option<u32>) -> anyhow::Result<Vec<Event>> {
    let mut command = Command::new("git");
    command.arg("-C").arg(dir).args(["log", "--remotes=origin", "--branches", "--format=%H%x09%cI%x09%s"]);
    if let Some(days) = days {
        command.arg(format!("--since={days}.days"));
    }
    let output = command.output()?;
    anyhow::ensure!(output.status.success(), "git log failed");
    let mut found = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let mut parts = line.splitn(3, '\t');
        let (Some(commit), Some(at), Some(subject)) = (parts.next(), parts.next(), parts.next()) else {
            continue;
        };
        for (kind, id, round, verdict) in parse(subject) {
            found.push(Event {
                commit: commit.to_string(),
                at: at.to_string(),
                kind,
                id,
                record: None,
                title: None,
                round,
                verdict,
                subject: subject.to_string(),
            });
        }
    }
    // The same review recorded on two branches before a merge shows once.
    let mut seen = HashSet::new();
    found.retain(|e| seen.insert((e.kind, e.id.clone(), e.round, e.subject.clone(), e.at.clone())));
    found.sort_by(|a, b| b.at.cmp(&a.at));
    Ok(found)
}

/// `GET /api/history?repo=&days=&ids=`: the records' history from commit
/// subjects, newest first.
pub async fn history(State(desk): State<Arc<Desk>>, Query(q): Query<HistoryQuery>) -> Response {
    blocking(move || {
        let dir = desk.served(&q.repo)?;
        let mut found = events(&dir, q.days).map_err(|e| Failure::Failed(format!("{e:#}")))?;
        if let Some(ids) = &q.ids {
            let wanted: HashSet<&str> = ids.split(',').collect();
            found.retain(|e| wanted.contains(e.id.as_str()));
        }
        let kinds: HashMap<String, (&'static str, String)> = (desk.indexes().into_iter())
            .flat_map(|i| i.records)
            .filter(|r| r.repo == q.repo)
            .map(|r| {
                let kind = match r.kind {
                    crate::api::HoldKind::Item => "item",
                    crate::api::HoldKind::Mission => "mission",
                };
                (r.id, (kind, r.title))
            })
            .collect();
        for event in &mut found {
            if let Some((kind, title)) = kinds.get(&event.id) {
                event.record = Some(kind);
                event.title = Some(title.clone());
            }
        }
        Ok(History {
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

#[derive(Clone, Serialize)]
pub struct Check {
    name: String,
    /// `SUCCESS`, `FAILURE`, `SKIPPED`, … or the status while it runs.
    state: String,
}

#[derive(Clone, Serialize)]
pub struct Checks {
    /// `passing`, `failing`, `pending` or `none`.
    state: &'static str,
    checks: Vec<Check>,
}

fn checks_cache() -> &'static Mutex<HashMap<(String, u32), (Instant, Checks)>> {
    static CACHE: OnceLock<Mutex<HashMap<(String, u32), (Instant, Checks)>>> = OnceLock::new();
    CACHE.get_or_init(Mutex::default)
}

/// `GET /api/checks?repo=&pr=`: CI on a pull request, from `gh`, kept a
/// minute.
pub async fn checks(State(desk): State<Arc<Desk>>, Query(q): Query<ChecksQuery>) -> Response {
    blocking(move || {
        let key = (q.repo.clone(), q.pr);
        if let Some((at, kept)) = checks_cache().lock().unwrap().get(&key) {
            if at.elapsed() < Duration::from_secs(60) {
                return Ok(kept.clone());
            }
        }
        let dir = desk.served(&q.repo)?;
        let output = Command::new(&desk.config.gh)
            .current_dir(&dir)
            .env("GH_PROMPT_DISABLED", "1")
            .args(["pr", "view", &q.pr.to_string(), "--json", "statusCheckRollup"])
            .output()
            .map_err(|e| Failure::Failed(e.to_string()))?;
        if !output.status.success() {
            return Err(Failure::Failed(String::from_utf8_lossy(&output.stderr).trim().to_string()));
        }
        let value: serde_json::Value =
            serde_json::from_slice(&output.stdout).map_err(|e| Failure::Failed(e.to_string()))?;
        let rollup = value["statusCheckRollup"].as_array().cloned().unwrap_or_default();
        let checks: Vec<Check> = (rollup.iter())
            .map(|c| {
                let name = c["name"].as_str().or(c["context"].as_str()).unwrap_or("check");
                let state = (c["conclusion"].as_str().filter(|s| !s.is_empty()))
                    .or(c["state"].as_str())
                    .or(c["status"].as_str())
                    .unwrap_or("UNKNOWN");
                Check {
                    name: name.to_string(),
                    state: state.to_string(),
                }
            })
            .collect();
        let bad = ["FAILURE", "ERROR", "CANCELLED", "TIMED_OUT", "ACTION_REQUIRED"];
        let good = ["SUCCESS", "SKIPPED", "NEUTRAL"];
        let state = if checks.is_empty() {
            "none"
        } else if checks.iter().any(|c| bad.contains(&c.state.as_str())) {
            "failing"
        } else if checks.iter().all(|c| good.contains(&c.state.as_str())) {
            "passing"
        } else {
            "pending"
        };
        let found = Checks { state, checks };
        checks_cache().lock().unwrap().insert(key, (Instant::now(), found.clone()));
        Ok(found)
    })
    .await
}

#[derive(Serialize)]
pub struct Roadmap {
    repo: String,
    /// Items in no mission that are ready or draft, or started.
    loose: Vec<crate::api::ItemRow>,
}

/// `GET /api/roadmap?repo=`: the items in no mission, for the roadmap's
/// Next and Later.
pub async fn roadmap(State(desk): State<Arc<Desk>>, Query(RepoQuery { repo }): Query<RepoQuery>) -> Response {
    blocking(move || {
        let missing = || format!("No repository {repo}.");
        let loose = desk.repo_route(&repo, "spike-loose", missing, read::loose)?;
        Ok(Roadmap { repo, loose })
    })
    .await
}

/// Seconds since the Unix epoch, now.
fn chrono_now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

/// Seconds from an RFC 3339 time to `now`, read with `date`.
fn seconds_since(at: &str, now: i64) -> Option<i64> {
    let out = Command::new("date").args(["-d", at, "+%s"]).output().ok()?;
    let then: i64 = String::from_utf8(out.stdout).ok()?.trim().parse().ok()?;
    Some((now - then).max(0))
}
