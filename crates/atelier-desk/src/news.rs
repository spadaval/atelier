//! Telling the human when a plan is proposed and when it merges
//! (docs/product/desk.md, Being told). After each round of fetches the
//! Desk compares each repository's missions with what it saw last, keeps
//! what changed as news, and runs the configured `[notify] command` for
//! each piece. What it has seen is kept in its state directory, so a
//! restart tells nothing twice and the first look at a repository tells
//! nothing at all.

use crate::api::{NewsEvent, NewsKind};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

/// How many pieces of news are kept.
const KEPT: usize = 50;

/// One mission as the Desk observes it.
#[derive(Clone)]
pub struct Seen {
    pub id: String,
    pub title: String,
    /// Open on a branch that has not merged.
    pub proposed: bool,
    /// The base's last commit holds it.
    pub agreed: bool,
}

pub struct News {
    /// Where what it has seen is kept; `None` keeps it in memory only.
    path: Option<PathBuf>,
    /// The Desk's address, for links; without one a link is a path.
    url: Option<String>,
    command: Option<Vec<String>>,
    state: Mutex<State>,
}

#[derive(Default, Serialize, Deserialize)]
struct State {
    seen: BTreeMap<String, Known>,
    events: Vec<NewsEvent>,
}

#[derive(Default, Serialize, Deserialize)]
struct Known {
    proposed: BTreeSet<String>,
    agreed: BTreeSet<String>,
}

impl News {
    pub fn new(path: Option<PathBuf>, url: Option<String>, command: Option<Vec<String>>) -> News {
        let state = (path.as_ref())
            .and_then(|path| std::fs::read(path).ok())
            .and_then(|text| serde_json::from_slice(&text).ok())
            .unwrap_or_default();
        News {
            path,
            url,
            command,
            state: Mutex::new(state),
        }
    }

    /// The address of mission `id`'s page in `repo`.
    pub fn link(&self, repo: &str, id: &str) -> String {
        let seg = |s: &str| {
            s.bytes()
                .map(|b| match b {
                    b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                        (b as char).to_string()
                    }
                    _ => format!("%{b:02X}"),
                })
                .collect::<String>()
        };
        let path = format!("/r/{}/mission/{}", seg(repo), seg(id));
        match &self.url {
            Some(url) => format!("{}{path}", url.trim_end_matches('/')),
            None => path,
        }
    }

    /// Compares `missions`, all of `repo`'s, with what was seen of it
    /// last, keeps and sends the news, and gives it. The first look at a
    /// repository only remembers.
    pub fn observe(&self, repo: &str, missions: &[Seen]) -> Vec<NewsEvent> {
        let mut state = self.state.lock().unwrap();
        let now = Known {
            proposed: (missions.iter().filter(|m| m.proposed))
                .map(|m| m.id.clone())
                .collect(),
            agreed: (missions.iter().filter(|m| m.agreed))
                .map(|m| m.id.clone())
                .collect(),
        };
        let mut events = Vec::new();
        if let Some(was) = state.seen.get(repo) {
            let at = chrono::Utc::now().to_rfc3339();
            for mission in missions {
                let kind = match (mission.proposed, mission.agreed) {
                    (true, _)
                        if !was.proposed.contains(&mission.id)
                            && !was.agreed.contains(&mission.id) =>
                    {
                        NewsKind::Proposed
                    }
                    // A plan reaches the base only by merging, so a mission
                    // agreed now and not before merged, whether or not a
                    // look saw it proposed.
                    (_, true) if !was.agreed.contains(&mission.id) => NewsKind::Merged,
                    _ => continue,
                };
                events.push(NewsEvent {
                    repo: repo.to_string(),
                    mission: mission.id.clone(),
                    title: mission.title.clone(),
                    kind,
                    at: at.clone(),
                    link: self.link(repo, &mission.id),
                });
            }
        }
        let changed = state
            .seen
            .get(repo)
            .is_none_or(|was| was.proposed != now.proposed || was.agreed != now.agreed);
        state.seen.insert(repo.to_string(), now);
        for event in &events {
            state.events.insert(0, event.clone());
        }
        state.events.truncate(KEPT);
        if changed {
            self.save(&state);
        }
        drop(state);
        for event in &events {
            self.send(event);
        }
        events
    }

    /// The news kept, newest first.
    pub fn recent(&self) -> Vec<NewsEvent> {
        self.state.lock().unwrap().events.clone()
    }

    fn save(&self, state: &State) {
        let Some(path) = &self.path else {
            return;
        };
        let written = (|| -> std::io::Result<()> {
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)?;
            }
            let text = serde_json::to_vec_pretty(state).map_err(std::io::Error::other)?;
            let partial = path.with_extension("json.partial");
            std::fs::write(&partial, text)?;
            std::fs::rename(partial, path)
        })();
        if let Err(error) = written {
            eprintln!(
                "atelier-desk: cannot keep what it has told of in {}: {error}",
                path.display()
            );
        }
    }

    /// Runs the `[notify] command` with the event's title, a line of text
    /// and its link. A failure is logged; the news stays on the inbox.
    fn send(&self, event: &NewsEvent) {
        let Some((program, first)) = self.command.as_deref().and_then(|c| c.split_first()) else {
            return;
        };
        let (title, text) = words(event);
        let ran = Command::new(program)
            .args(first)
            .args([&title, &text, &event.link])
            .output();
        match ran {
            Ok(output) if output.status.success() => {}
            Ok(output) => eprintln!(
                "atelier-desk: the notify command failed for {} {}: {}",
                event.repo,
                event.mission,
                String::from_utf8_lossy(&output.stderr).trim()
            ),
            Err(error) => {
                eprintln!("atelier-desk: cannot run the notify command {program}: {error}")
            }
        }
    }
}

/// The title and the line of text the human is sent for `event`.
pub fn words(event: &NewsEvent) -> (String, String) {
    match event.kind {
        NewsKind::Proposed => (
            format!("Plan proposed: {}", event.title),
            format!(
                "Mission {} in {} is proposed and under plan review. It merges once two reviewers approve; stop it from its page if it should not.",
                event.mission, event.repo
            ),
        ),
        NewsKind::Merged => (
            format!("Plan merged: {}", event.title),
            format!(
                "Mission {} in {} has merged: its plan is agreed and its items can start. You can still stop it from its page.",
                event.mission, event.repo
            ),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seen(id: &str, proposed: bool, agreed: bool) -> Seen {
        Seen {
            id: id.into(),
            title: format!("Mission {id}"),
            proposed,
            agreed,
        }
    }

    #[test]
    fn a_plan_is_news_when_proposed_and_when_merged_and_never_twice() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("news.json");
        let sent = dir.path().join("sent");
        let script = format!(
            "printf '%s|%s|%s\\n' \"$1\" \"$2\" \"$3\" >> {}",
            sent.display()
        );
        let command = vec!["sh".into(), "-c".into(), script, "notify".into()];
        let news = News::new(
            Some(path.clone()),
            Some("http://desk:3780/".into()),
            Some(command.clone()),
        );
        // The first look remembers and tells nothing.
        assert!(news
            .observe("demo", &[seen("demo-old1", true, false)])
            .is_empty());
        let proposed = news.observe(
            "demo",
            &[
                seen("demo-old1", true, false),
                seen("demo-new1", true, false),
            ],
        );
        assert_eq!(proposed.len(), 1);
        assert_eq!(proposed[0].kind, NewsKind::Proposed);
        assert_eq!(
            proposed[0].link,
            "http://desk:3780/r/demo/mission/demo-new1"
        );
        // A restart reads what was seen.
        let news = News::new(Some(path), None, Some(command));
        assert!(news
            .observe(
                "demo",
                &[
                    seen("demo-old1", true, false),
                    seen("demo-new1", true, false)
                ]
            )
            .is_empty());
        let merged = news.observe(
            "demo",
            &[
                seen("demo-old1", true, false),
                seen("demo-new1", false, true),
            ],
        );
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].kind, NewsKind::Merged);
        // A plan that leaves the proposed without merging is no news.
        assert!(news
            .observe("demo", &[seen("demo-new1", false, true)])
            .is_empty());
        // A plan proposed and merged between two looks is news that it
        // merged, as is one merged while the Desk was down.
        let quick = news.observe(
            "demo",
            &[
                seen("demo-new1", false, true),
                seen("demo-new2", false, true),
            ],
        );
        assert_eq!(quick.len(), 1);
        assert_eq!(quick[0].mission, "demo-new2");
        assert_eq!(quick[0].kind, NewsKind::Merged);
        let recent = news.recent();
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[1].kind, NewsKind::Merged);
        let sent = std::fs::read_to_string(sent).unwrap();
        let lines: Vec<&str> = sent.lines().collect();
        assert_eq!(lines.len(), 3, "{sent}");
        assert!(lines[0]
            .starts_with("Plan proposed: Mission demo-new1|Mission demo-new1 in demo is proposed"));
        assert!(lines[0].ends_with("|http://desk:3780/r/demo/mission/demo-new1"));
        assert!(lines[1].starts_with("Plan merged: Mission demo-new1|"));
        assert!(lines[1].ends_with("|/r/demo/mission/demo-new1"));
    }
}
