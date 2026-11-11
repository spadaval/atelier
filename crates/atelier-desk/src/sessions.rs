//! Starting a planning session for a mission as a new T3 Code thread
//! (docs/architecture/desk.md, `POST /api/sessions`). It writes no record.

use crate::api::SessionResponse;
use crate::config::T3Config;
use crate::t3::{Client, T3Project, OPERATE};
use crate::threads::Threads;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// How long the token that may start a thread lasts. It is revoked as soon
/// as the thread is started or refused.
const OPERATE_TTL: &str = "5m";

/// Starts a thread for mission `mission`, titled `title`, of the repository
/// at `dir` served as `repo`, in the T3 project whose workspace root is
/// `dir`. The token that may start it is issued only once that project is
/// found, and revoked before this returns.
pub fn start(
    t3: Option<&T3Config>,
    threads: &Threads,
    dir: &Path,
    repo: &str,
    mission: &str,
    title: &str,
) -> SessionResponse {
    let refused = |text: String| SessionResponse::Refused { text };
    let Some(t3) = t3 else {
        let text = "T3 Code is not configured: the Desk's configuration has no [t3] table. Nothing was started.";
        return refused(text.into());
    };
    let projects = match threads.projects() {
        Ok(projects) => projects,
        Err(error) => {
            return refused(format!(
                "T3 Code cannot be reached, so nothing was started: {error:#}."
            ))
        }
    };
    let Some(project) = project_of(&projects, dir) else {
        return refused(format!(
            "No T3 Code project has {} as its workspace root, so nothing was started. Add that folder as a project in T3 Code, then start the session again.",
            dir.display()
        ));
    };
    let message = (t3.session_message)
        .replace("{mission}", mission)
        .replace("{repo}", repo);
    let title = format!("Plan {mission}: {title}");
    let client = Client::new(t3.command.clone(), t3.url.clone(), OPERATE);
    let mut client = client.lasting(OPERATE_TTL);
    let launched = client.launch(&project.id, &title, &message, &request_id());
    client.close();
    match launched {
        Ok(launched) => SessionResponse::Started {
            thread_id: launched.thread_id,
            link: launched.link,
            project_id: launched.project_id,
        },
        Err(error) => refused(format!(
            "Starting the thread failed: {error:#}. Look in T3 Code's project for a thread titled \"{title}\" before trying again."
        )),
    }
}

/// The project whose workspace root is the directory `dir`, compared as
/// canonical paths, so a link or a `~` in either names the same folder.
fn project_of<'p>(projects: &'p [T3Project], dir: &Path) -> Option<&'p T3Project> {
    let dir = std::fs::canonicalize(dir).ok()?;
    projects.iter().find(|project| {
        std::fs::canonicalize(&project.workspace_root).is_ok_and(|root| root == dir)
    })
}

/// A key for one launch, so that the client's one retry, after a refused
/// token, replays it and starts no second thread.
fn request_id() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let now = SystemTime::now().duration_since(UNIX_EPOCH);
    let nanos = now.map_or(0, |since| since.as_nanos());
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    format!("atelier-desk-{}-{nanos}-{n}", std::process::id())
}
