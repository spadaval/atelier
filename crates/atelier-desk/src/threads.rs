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
