//! `decide`: the Desk's answer to a question an item's Decisions section
//! holds from before `escalate` was removed. The `atelier` command has no
//! `decide`, and no command asks a question now; atelier-v958 removes this
//! with the Desk's decide page. It writes the item's Decisions section in
//! the checkout on its branch and commits nothing; `submit` commits it with
//! the item.

use crate::acts::{check, question, Act, Refusal};
use crate::plan::Done;
use crate::repo::{facts, item_path};
use crate::skip::{refused, Refused};
use crate::{git, say, Repo};
use anyhow::{anyhow, bail, Result};
use atelier_core::{Answer, Decision, Outcome as Settled, Status};
use atelier_records::{answer, validate_id};

/// `decide`: answers, as `repo`'s caller, the oldest question waiting on
/// item `id`, in the checkout on its branch, and with `settled` settles the
/// finding it is about; or refuses, writing nothing.
pub fn decide(
    repo: &mut Repo,
    id: &str,
    answer_text: &str,
    settled: Option<Settled>,
) -> Result<Result<Done, Refused>> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    let located = repo.locate(id)?;
    let Some(record) = located.record() else {
        return Ok(Err(say::no_item(id, &repo.base)));
    };
    // Where the implementer works, so its `submit` commits the answer.
    if repo.branch.as_deref() != Some(id) {
        let checkout = git::checkout_of(&repo.root, id)?;
        let branch = repo.branch.as_deref();
        return Ok(Err(say::elsewhere(
            "decide",
            id,
            branch,
            checkout.as_deref(),
        )));
    }
    let links = repo.links(record)?;
    let facts = facts(repo, &located, &links, None);
    let text = match check(Act::Decide, &facts) {
        Ok(_) => None,
        Err(Refusal::Unmet(unmet)) => Some(say::not_decider(id, &repo.caller, &unmet)),
        Err(refusal) => Some(refused(Act::Decide, &refusal, &facts)?),
    };
    if let Some(refused) = text.and_then(|text| repo.skip.gated(Act::Decide, &facts, text)) {
        return Ok(Err(refused));
    }
    let Some(text) = one_line(answer_text) else {
        return Ok(Err(say::not_one_line(Act::Decide, id, "answer").into()));
    };
    let (index, asked) = question(record)?.expect("decide's check found a question");
    if let Some(settled) = settled {
        let open = record.open_findings()?;
        let about = asked.finding.as_ref();
        if about.is_none_or(|name| !open.iter().any(|f| f.name == *name)) {
            return Ok(Err(say::not_settled(id, &asked, settled)));
        }
    }
    let mut dir = record.clone();
    dir.item.body = answer(&dir.item.body, index, &repo.caller, text, settled)
        .expect("the question is one of the item's");
    let answered = Decision {
        answer: Some(Answer {
            by: repo.caller.clone(),
            text: text.to_string(),
        }),
        outcome: settled,
        ..asked
    };
    if dir.decisions()?.get(index) != Some(&answered) {
        bail!(say::unreadable(&item_path(id)));
    }
    repo.store().write_item(&dir.item)?;
    let waiting = question(&dir)?.map(|(_, waiting)| waiting);
    Ok(Ok(Done {
        text: say::decided(
            id,
            &answered,
            waiting.as_ref(),
            facts.holder(),
            record.item.status == Status::Approved,
        ),
        paths: vec![item_path(id)],
        commit: None,
    }))
}

/// `text` trimmed, when it is one line and not empty: an answer is one
/// line under its question.
fn one_line(text: &str) -> Option<&str> {
    let text = text.trim();
    (!text.is_empty() && !text.contains(['\n', '\r'])).then_some(text)
}
