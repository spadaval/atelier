//! T3 Code's live threads, polled into memory so no page waits on T3 Code,
//! and joined to the identities the records hold by comparing whole
//! strings (docs/architecture/desk.md, Tying a thread to an item).

use crate::api::{Agent, Hold, LiveThread, PullRequest, Side, T3State, ThreadMark};
use crate::config::T3Config;
use crate::t3::{self, Client, T3Project, T3Thread};
use anyhow::{bail, Result};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

/// How often the thread list is read.
pub const POLL: Duration = Duration::from_secs(10);

/// Every how many polls the recent threads, ended ones too, are read, for
/// the models of threads that are no longer live.
pub const RECENT_EVERY: usize = 6;

pub struct Threads {
    /// `None` when the configuration has no `[t3]` table.
    client: Option<Mutex<Client>>,
    polled: RwLock<Polled>,
    /// Each thread T3 Code has listed since the Desk started, live or
    /// not, by ID: its provider, model and link.
    known: RwLock<HashMap<String, Known>>,
    polls: AtomicUsize,
    read_cache: RwLock<HashMap<String, (Instant, Option<crate::api::ThreadInfo>)>>,
    full_cache: RwLock<HashMap<String, (Instant, Option<crate::api::ThreadInfo>)>>,
}

/// A thread item as one line: a tool call as its tool and first argument,
/// a message as its first line. The item's text may be cut short, so a
/// tool call's JSON is read with string matching, not parsed.
fn one_line(item: &serde_json::Value) -> Option<String> {
    let text = item.get("text")?.as_str()?;
    let kind = item.get("type").and_then(|t| t.as_str()).unwrap_or("");
    let field = |name: &str| -> Option<String> {
        let key = format!("\"{name}\": \"");
        let start = text.find(&key)? + key.len();
        let rest = &text[start..];
        let end = rest.find('"').unwrap_or(rest.len());
        Some(rest[..end].replace("\\n", " "))
    };
    let line = if kind.contains("tool") {
        let tool = field("toolName").unwrap_or_else(|| kind.to_string());
        let tool = tool.rsplit("__").next().unwrap_or(&tool).to_string();
        let arg = ["description", "command", "file_path", "pattern", "query", "prompt", "url"]
            .iter()
            .find_map(|k| field(k));
        match arg {
            Some(arg) => format!("{tool}: {arg}"),
            None => tool,
        }
    } else {
        text.lines().find(|l| !l.trim().is_empty())?.trim().to_string()
    };
    let line: String = line.chars().take(160).collect();
    Some(line)
}

#[derive(Clone)]
struct Known {
    provider: String,
    model: String,
    link: Option<String>,
}

#[derive(Default)]
struct Polled {
    /// The last list read, and when.
    list: Option<(Instant, Arc<Vec<T3Thread>>)>,
    /// Whether the last poll read it.
    reachable: bool,
    error: Option<String>,
}

/// The threads served now, and what is known of T3 Code.
pub struct Snapshot {
    pub state: T3State,
    pub threads: Arc<Vec<T3Thread>>,
}

impl Threads {
    pub fn new(config: Option<&T3Config>) -> Threads {
        let client = config.map(|t3| {
            let client = Client::new(t3.command.clone(), t3.url.clone(), t3::READ);
            Mutex::new(client)
        });
        Threads {
            client,
            polled: RwLock::default(),
            known: RwLock::default(),
            polls: AtomicUsize::new(0),
            read_cache: RwLock::default(),
            full_cache: RwLock::default(),
        }
    }

    pub fn configured(&self) -> bool {
        self.client.is_some()
    }

    /// Reads T3 Code's live threads once. A failure keeps the last list
    /// read and records why.
    pub fn poll(&self) {
        let Some(client) = &self.client else {
            return;
        };
        let mut client = client.lock().unwrap();
        let listed = client.live_threads();
        if (self.polls.fetch_add(1, Ordering::Relaxed)).is_multiple_of(RECENT_EVERY) {
            if let Ok(recent) = client.recent_threads(100) {
                self.learn(&recent);
            }
        }
        drop(client);
        if let Ok(threads) = &listed {
            self.learn(threads);
        }
        let mut polled = self.polled.write().unwrap();
        match listed {
            Ok(threads) => {
                polled.list = Some((Instant::now(), Arc::new(threads)));
                polled.reachable = true;
                polled.error = None;
            }
            Err(error) => {
                polled.reachable = false;
                polled.error = Some(format!("{error:#}"));
            }
        }
    }

    fn learn(&self, threads: &[T3Thread]) {
        let mut known = self.known.write().unwrap();
        for thread in threads {
            let seen = Known {
                provider: thread.provider_instance_id.clone(),
                model: thread.model.clone(),
                link: thread.link.clone(),
            };
            known.insert(thread.thread_id.clone(), seen);
        }
    }

    /// Fills in what T3 Code has said of `side`'s thread: its provider,
    /// its link, and its model where the review records none.
    pub fn describe(&self, side: &mut Side) {
        let known = self.known.read().unwrap();
        if let Some(found) = known.get(&side.identity) {
            side.provider = Some(found.provider.clone());
            side.link = found.link.clone();
            if side.model.is_none() {
                side.model = Some(found.model.clone());
            }
        }
    }

    /// Spike: a thread by identity, from the live list, else read from T3
    /// Code once and kept for a minute.
    pub fn info(&self, id: &str) -> Option<crate::api::ThreadInfo> {
        let snapshot = self.snapshot();
        if let Some(t) = snapshot.threads.iter().find(|t| t.thread_id == id) {
            return Some(crate::api::ThreadInfo {
                id: t.thread_id.clone(),
                title: t.title.clone(),
                status: t.status.clone(),
                updated_at: t.updated_at.clone(),
                parent: t.parent_thread_id.clone(),
                model: Some(t.model.clone()),
                live: true,
                started_at: None,
                last_line: None,
            });
        }
        {
            let cache = self.read_cache.read().unwrap();
            if let Some((at, info)) = cache.get(id) {
                if at.elapsed() < Duration::from_secs(60) {
                    return info.clone();
                }
            }
        }
        let client = self.client.as_ref()?;
        let read = {
            let mut client = client.lock().unwrap();
            client.call(
                "t3_thread_read",
                serde_json::json!({"threadId": id, "limit": 1, "maxCharsPerItem": 1}),
            )
        };
        let info = read.ok().and_then(|v| {
            let t = v.get("thread")?;
            let s = |k: &str| t.get(k).and_then(|x| x.as_str()).map(str::to_string);
            Some(crate::api::ThreadInfo {
                id: id.to_string(),
                title: s("title")?,
                status: s("status").unwrap_or_default(),
                updated_at: s("updatedAt").unwrap_or_default(),
                parent: s("parentThreadId"),
                model: s("model"),
                live: false,
                started_at: None,
                last_line: None,
            })
        });
        let mut cache = self.read_cache.write().unwrap();
        cache.insert(id.to_string(), (Instant::now(), info.clone()));
        info
    }

    /// Spike: a thread read from T3 Code with its latest run's start and
    /// its latest item as one line, kept 20 seconds.
    pub fn info_full(&self, id: &str) -> Option<crate::api::ThreadInfo> {
        {
            let cache = self.full_cache.read().unwrap();
            if let Some((at, info)) = cache.get(id) {
                if at.elapsed() < Duration::from_secs(20) {
                    return info.clone();
                }
            }
        }
        let live = self.snapshot().threads.iter().any(|t| t.thread_id == id);
        let client = self.client.as_ref()?;
        let read = {
            let mut client = client.lock().unwrap();
            client.call(
                "t3_thread_read",
                serde_json::json!({"threadId": id, "limit": 4, "maxCharsPerItem": 400, "view": "activity"}),
            )
        };
        let info = read.ok().and_then(|v| {
            let t = v.get("thread")?;
            let s = |k: &str| t.get(k).and_then(|x| x.as_str()).map(str::to_string);
            let runs = v.get("recentRuns").and_then(|r| r.as_array()).cloned().unwrap_or_default();
            let started_at = (runs.iter())
                .find_map(|r| r.get("startedAt").and_then(|x| x.as_str()).map(str::to_string))
                .or_else(|| s("createdAt"));
            // The newest output: an assistant message or a tool call, not a
            // checkpoint or a message sent to the thread.
            let items = v.get("items").and_then(|i| i.as_array()).cloned().unwrap_or_default();
            let item = (items.iter().rev())
                .find(|i| {
                    let kind = i.get("type").and_then(|t| t.as_str()).unwrap_or("");
                    kind == "assistant_message" || kind.contains("tool") || kind.contains("command")
                })
                .cloned();
            Some(crate::api::ThreadInfo {
                id: id.to_string(),
                title: s("title")?,
                status: s("status").unwrap_or_default(),
                updated_at: s("updatedAt").unwrap_or_default(),
                parent: s("parentThreadId"),
                model: s("model"),
                live,
                started_at,
                last_line: item.as_ref().and_then(one_line),
            })
        });
        let mut cache = self.full_cache.write().unwrap();
        cache.insert(id.to_string(), (Instant::now(), info.clone()));
        info
    }

    pub fn snapshot(&self) -> Snapshot {
        if !self.configured() {
            let error = "T3 Code is not configured: the Desk's configuration has no [t3] table.";
            return Snapshot {
                state: T3State {
                    error: Some(error.into()),
                    ..T3State::default()
                },
                threads: Arc::default(),
            };
        }
        let polled = self.polled.read().unwrap();
        let (age, threads) = match &polled.list {
            Some((at, threads)) => {
                let age = u32::try_from(at.elapsed().as_secs()).unwrap_or(u32::MAX);
                (Some(age), threads.clone())
            }
            None => (None, Arc::default()),
        };
        let never = (polled.list.is_none() && polled.error.is_none())
            .then(|| "T3 Code has not been polled yet.".to_string());
        Snapshot {
            state: T3State {
                reachable: polled.reachable,
                age_seconds: age,
                error: polled.error.clone().or(never),
            },
            threads,
        }
    }

    /// T3 Code's projects, read with the token that only reads.
    pub fn projects(&self) -> Result<Vec<T3Project>> {
        let Some(client) = &self.client else {
            bail!("T3 Code is not configured: the Desk's configuration has no [t3] table");
        };
        let projects = client.lock().unwrap().projects();
        projects
    }

    /// Revokes the token held.
    pub fn close(&self) {
        if let Some(client) = &self.client {
            client.lock().unwrap().close();
        }
    }
}

/// Polls `threads` every `POLL` on a thread of its own, from now until the
/// process ends.
pub fn keep_polling(threads: Arc<Threads>) {
    if !threads.configured() {
        return;
    }
    std::thread::spawn(move || loop {
        threads.poll();
        std::thread::sleep(POLL);
    });
}

impl Snapshot {
    fn find(&self, identity: &str) -> Option<&T3Thread> {
        self.threads.iter().find(|t| t.thread_id == identity)
    }

    /// Gives each agent its live thread, where its whole identity is one.
    pub fn join(&self, agents: &mut [Agent]) {
        for agent in agents {
            agent.thread = self.find(&agent.identity).map(|thread| ThreadMark {
                title: thread.title.clone(),
                status: thread.status.clone(),
                provider: thread.provider_instance_id.clone(),
                model: thread.model.clone(),
                link: thread.link.clone(),
            });
        }
    }

    /// Every live thread, each with what its identity holds among `holds`.
    pub fn live(&self, holds: &[(String, Hold)]) -> Vec<LiveThread> {
        let threads = self.threads.iter().map(|thread| {
            let held = holds
                .iter()
                .filter(|(identity, _)| *identity == thread.thread_id);
            let pull_requests = thread.pull_requests.iter().flatten();
            LiveThread {
                id: thread.thread_id.clone(),
                title: thread.title.clone(),
                status: thread.status.clone(),
                provider: thread.provider_instance_id.clone(),
                model: thread.model.clone(),
                parent: thread.parent_thread_id.clone(),
                pull_requests: (pull_requests)
                    .map(|pr| PullRequest {
                        repository: pr.repository.clone(),
                        number: pr.number,
                        url: pr.url.clone(),
                        state: pr.state.clone(),
                    })
                    .collect(),
                link: thread.link.clone(),
                updated_at: thread.updated_at.clone(),
                holds: held.map(|(_, hold)| hold.clone()).collect(),
            }
        });
        threads.collect()
    }
}
