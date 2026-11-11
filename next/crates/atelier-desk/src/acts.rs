//! The two acts the Desk performs, `ready` and `decide`, made by the
//! core's own functions as the `atelier` command makes them
//! (docs/architecture/desk.md, Act functions), and the answers given from
//! the Desk that no commit holds yet.

use crate::api::{ActRequest, ActResponse, DecideAct, Question, ReadyAct};
use crate::read::question_of;
use anyhow::Result;
use atelier_app::acts::question;
use atelier_app::{git, Done, Refused, Repo, Skip};
use atelier_core::Thread;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

/// Runs `request` on the repository at `dir` as `caller`, in the checkout
/// the act belongs in: the one with the base out for `ready`, the one with
/// the item's branch out for `decide`. Where no checkout has that branch
/// out, it opens `dir` and the function refuses with its own text. Gives
/// the response and, for an answer written, the item's mission and the
/// question as answered.
pub fn act(dir: &Path, caller: &str, request: &ActRequest) -> Result<(ActResponse, Answered)> {
    let skip = Skip::new(None);
    let thread = || Thread(caller.to_string());
    match request {
        ActRequest::Ready(ReadyAct { ids, .. }) => {
            let base = Repo::open_committed(dir, thread(), &skip)?.base;
            let mut repo = Repo::open(&checkout(dir, &base)?, thread(), &skip)?;
            Ok((response(atelier_app::ready(&mut repo, ids, caller)?), None))
        }
        ActRequest::Decide(DecideAct {
            id, answer, settle, ..
        }) => {
            let mut repo = Repo::open(&checkout(dir, id)?, thread(), &skip)?;
            let asked = repo.locate(id)?.record().map(question).transpose()?;
            let acted = atelier_app::decide(&mut repo, id, answer, (*settle).map(Into::into))?;
            let answered = match (&acted, asked.flatten()) {
                // The question as the checkout now holds it, answered.
                (Ok(_), Some((index, _))) => {
                    let located = repo.locate(id)?;
                    let Some(record) = located.record() else {
                        return Ok((response(acted), None));
                    };
                    let decision = record.decisions()?.into_iter().nth(index);
                    let mission = record.item.mission.clone();
                    decision.map(|decision| (mission, question_of(id, &decision)))
                }
                _ => None,
            };
            Ok((response(acted), answered))
        }
    }
}

/// An answer `decide` wrote: the item's mission, and the question
/// answered.
pub type Answered = Option<(Option<String>, Question)>;

/// The checkout with `branch` out, else `dir`.
fn checkout(dir: &Path, branch: &str) -> Result<PathBuf> {
    Ok(git::checkout_of(dir, branch)?.unwrap_or_else(|| dir.to_path_buf()))
}

fn response(acted: Result<Done, Refused>) -> ActResponse {
    match acted {
        Ok(Done {
            text,
            paths,
            commit,
        }) => ActResponse::Done {
            text,
            paths,
            commit,
        },
        Err(Refused { rules, text, next }) => ActResponse::Refused { rules, text, next },
    }
}

/// An answer given from the Desk to a question of `item`'s, in `mission`.
#[derive(Debug, Clone)]
struct Pending {
    repo: String,
    mission: Option<String>,
    answer: Question,
}

/// The answers given from the Desk that no commit holds yet, kept in
/// memory: pages are read from commits, and `submit` commits an answer.
/// A restart forgets them.
#[derive(Default)]
pub struct Answers(Mutex<Vec<Pending>>);

impl Answers {
    fn held(&self) -> MutexGuard<'_, Vec<Pending>> {
        self.0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Keeps `answer`, given on repository `repo` to an item in `mission`.
    pub fn add(&self, repo: &str, mission: Option<String>, answer: Question) {
        let mut held = self.held();
        held.retain(|p| !(p.repo == repo && same(&p.answer, &answer)));
        held.push(Pending {
            repo: repo.to_string(),
            mission,
            answer,
        });
    }

    /// The answers on `repo` to item `item`'s questions, or with `mission`
    /// to those of its items, less each one `committed` shows answered,
    /// which is forgotten.
    pub fn pending(&self, repo: &str, on: On, committed: &[Question]) -> Vec<Question> {
        let mut held = self.held();
        let picked = |p: &Pending| {
            p.repo == repo
                && match on {
                    On::Item(id) => p.answer.item == id,
                    On::Mission(id) => p.mission.as_deref() == Some(id),
                }
        };
        held.retain(|p| {
            let answered = committed
                .iter()
                .any(|c| same(c, &p.answer) && c.answer.is_some());
            !(picked(p) && answered)
        });
        held.iter()
            .filter(|p| picked(p))
            .map(|p| p.answer.clone())
            .collect()
    }

    /// Whether an answer given on `repo` to `item`'s `question` waits for
    /// its commit.
    pub fn waits(&self, repo: &str, item: &str, question: &str) -> bool {
        let held = self.held();
        held.iter()
            .any(|p| p.repo == repo && p.answer.item == item && p.answer.question == question)
    }
}

/// Which page asks for pending answers.
#[derive(Clone, Copy)]
pub enum On<'a> {
    Item(&'a str),
    Mission(&'a str),
}

/// Whether two are the same question of the same item.
fn same(a: &Question, b: &Question) -> bool {
    a.item == b.item && a.question == b.question
}
