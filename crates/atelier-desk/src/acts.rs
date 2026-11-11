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
/// the response and, for an answer written, the [`Answer`].
pub fn act(
    dir: &Path,
    caller: &str,
    request: &ActRequest,
) -> Result<(ActResponse, Option<Answer>)> {
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
                    decision.map(|decision| Answer {
                        mission: record.item.mission.clone(),
                        index,
                        question: question_of(id, &decision),
                    })
                }
                _ => None,
            };
            Ok((response(acted), answered))
        }
    }
}

/// An answer `decide` wrote.
#[derive(Debug, Clone)]
pub struct Answer {
    /// The item's mission.
    pub mission: Option<String>,
    /// Which of the item's decisions it answers, counting from 0 in the
    /// order its Decisions section gives them: headings may repeat.
    pub index: usize,
    /// The question, answered.
    pub question: Question,
}

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

/// An answer given from the Desk on repository `repo`.
#[derive(Debug, Clone)]
struct Pending {
    repo: String,
    answer: Answer,
}

impl Pending {
    fn is(&self, repo: &str, item: &str, index: usize) -> bool {
        self.repo == repo && self.answer.question.item == item && self.answer.index == index
    }
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

    /// Keeps `answer`, given on repository `repo`.
    pub fn add(&self, repo: &str, answer: Answer) {
        let mut held = self.held();
        let (item, index) = (&answer.question.item, answer.index);
        held.retain(|p| !p.is(repo, item, index));
        held.push(Pending {
            repo: repo.to_string(),
            answer,
        });
    }

    /// The answers on `repo` to item `item`'s questions, or with `mission`
    /// to those of its items, less each one `committed` shows answered,
    /// which is forgotten. `committed` holds every decision of each item
    /// it names, in its record's order, as the item and mission pages do.
    pub fn pending(&self, repo: &str, on: On, committed: &[Question]) -> Vec<Question> {
        let mut held = self.held();
        let picked = |p: &Pending| {
            p.repo == repo
                && match on {
                    On::Item(id) => p.answer.question.item == id,
                    On::Mission(id) => p.answer.mission.as_deref() == Some(id),
                }
        };
        held.retain(|p| !(picked(p) && answered(committed, &p.answer)));
        held.iter()
            .filter(|p| picked(p))
            .map(|p| p.answer.question.clone())
            .collect()
    }

    /// Whether an answer given on `repo` to `item`'s decision `index`
    /// waits for its commit.
    pub fn waits(&self, repo: &str, item: &str, index: usize) -> bool {
        self.held().iter().any(|p| p.is(repo, item, index))
    }
}

/// Whether `committed` holds `answer`'s decision, the same question at the
/// same place among its item's decisions, answered.
fn answered(committed: &[Question], answer: &Answer) -> bool {
    let asked = &answer.question;
    let mut decisions = committed.iter().filter(|c| c.item == asked.item);
    decisions
        .nth(answer.index)
        .is_some_and(|c| c.question == asked.question && c.answer.is_some())
}

/// Which page asks for pending answers.
#[derive(Clone, Copy)]
pub enum On<'a> {
    Item(&'a str),
    Mission(&'a str),
}
