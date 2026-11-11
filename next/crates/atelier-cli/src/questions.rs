//! `escalate` and `decide`: a question the implementer cannot settle with
//! a fix or a rebuttal, and a decider's answer to it. Both write the item's
//! Decisions section in the checkout on its branch and commit nothing;
//! `submit` commits it with the item.

use crate::commands::{facts, gated, refusal, refused, Outcome};
use crate::repo::{git_at, item_dir, item_path, Links, Repo};
use crate::{git, output};
use anyhow::{anyhow, bail, Result};
use atelier_app::acts::{check, question, unmet, Act, Facts, Refusal};
use atelier_core::{Answer, Decision, FindingName, Outcome as Settled, Status};
use atelier_records::{answer, ask, validate_id};

pub fn escalate(repo: &mut Repo, question: &str, finding: Option<&str>) -> Result<Outcome> {
    let Some(located) = repo.current()? else {
        let branch = repo.branch.as_deref();
        return Ok(Outcome::Refused(output::not_on_item("escalate", branch)));
    };
    let record = located.record().expect("the current item has a record");
    let links = repo.links(record)?;
    let facts = facts(repo, &located, &links, None);
    if let Some(refused) = refusal(Act::Escalate, &facts)? {
        return Ok(refused);
    }
    let id = record.item.id.clone();
    let Some(question) = one_line(question) else {
        return Ok(Outcome::Refused(output::not_one_line(
            Act::Escalate,
            &id,
            "question",
        )));
    };
    let finding = match finding {
        None => None,
        Some(text) => {
            let open = record.open_findings()?;
            let name = text.parse::<FindingName>().ok();
            match name.filter(|name| open.iter().any(|f| f.name == *name)) {
                Some(name) => Some(name),
                None => {
                    let known = record
                        .findings()?
                        .iter()
                        .any(|f| f.name.to_string() == text);
                    let refused = output::not_open(&id, text, known, &open);
                    return Ok(Outcome::Refused(refused));
                }
            }
        }
    };
    let asked = format!("{}: {question}", chrono::Utc::now().format("%Y-%m-%d"));
    let mut dir = record.clone();
    dir.item.body = ask(&dir.item.body, &asked, finding.as_ref());
    let expected = Decision {
        question: asked.clone(),
        finding: finding.clone(),
        answer: None,
        outcome: None,
    };
    if dir.decisions()?.last() != Some(&expected) {
        bail!(output::unreadable(&item_path(&id)));
    }
    repo.store().write_item(&dir.item)?;
    let decider = decider(
        repo,
        &Facts {
            item: &dir,
            ..facts
        },
        &links,
    )?;
    Ok(Outcome::Done(output::escalated(
        &id,
        &asked,
        finding.as_ref(),
        decider.as_deref().unwrap_or_default(),
    )))
}

pub fn decide(
    repo: &mut Repo,
    id: &str,
    answer_text: &str,
    settled: Option<Settled>,
) -> Result<Outcome> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    let located = repo.locate(id)?;
    let Some(record) = located.record() else {
        return Ok(Outcome::Refused(output::no_item(id, &repo.base)));
    };
    // Where the implementer works, so its `submit` commits the answer.
    if repo.branch.as_deref() != Some(id) {
        let checkout = git::checkout_of(&repo.root, id)?;
        let branch = repo.branch.as_deref();
        let refused = output::elsewhere("decide", id, branch, checkout.as_deref());
        return Ok(Outcome::Refused(refused));
    }
    let links = repo.links(record)?;
    let facts = facts(repo, &located, &links, None);
    let text = match check(Act::Decide, &facts) {
        Ok(_) => None,
        Err(Refusal::Unmet(unmet)) => {
            let handoff = decider(repo, &facts, &links)?;
            let caller = &repo.caller;
            Some(output::not_decider(id, caller, &unmet, handoff.as_deref()))
        }
        Err(refusal) => Some(refused(Act::Decide, &refusal, &facts)?),
    };
    if let Some(refused) = text.and_then(|text| gated(Act::Decide, &facts, text)) {
        return Ok(refused);
    }
    let Some(text) = one_line(answer_text) else {
        return Ok(Outcome::Refused(output::not_one_line(
            Act::Decide,
            id,
            "answer",
        )));
    };
    let (index, asked) = question(record)?.expect("decide's check found a question");
    if let Some(settled) = settled {
        let open = record.open_findings()?;
        let about = asked.finding.as_ref();
        if about.is_none_or(|name| !open.iter().any(|f| f.name == *name)) {
            return Ok(Outcome::Refused(output::not_settled(id, &asked, settled)));
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
        bail!(output::unreadable(&item_path(id)));
    }
    repo.store().write_item(&dir.item)?;
    let waiting = question(&dir)?.map(|(_, waiting)| waiting);
    Ok(Outcome::Done(output::decided(
        id,
        &answered,
        waiting.as_ref(),
        facts.holder(),
        record.item.status == Status::Approved,
    )))
}

/// What tells the implementer of `facts`' item who answers the oldest
/// question waiting on it, if one waits: the item's mission's lead, when
/// the mission is ready and the lead may decide it, else a new thread. The
/// lead reads the item from its branch's last commit, so it is told how to
/// commit a question it cannot see yet.
pub(crate) fn decider(repo: &Repo, facts: &Facts, links: &Links) -> Result<Option<String>> {
    let Some((_, asked)) = question(facts.item)? else {
        return Ok(None);
    };
    let id = &facts.item.item.id;
    let mut lead = None;
    if let (Some((mission, Some(Status::Ready))), Some(thread)) = (&facts.mission, &links.lead) {
        let as_lead = Facts {
            caller: thread,
            mission: facts.mission.clone(),
            ..*facts
        };
        if unmet(Act::Decide, &as_lead)?.is_empty() {
            lead = Some((mission.as_str(), thread));
        }
    }
    let path = item_path(id);
    let commit = match lead.is_some() && repo.uncommitted(id)?.contains(&path) {
        true => Some(output::commit_line(
            git_at(&repo.root),
            &[path],
            &format!("Ask about {id}"),
        )),
        false => None,
    };
    let review = (asked.finding.as_ref())
        .map(|finding| format!("{}/reviews/{}.md", item_dir(id), finding.review));
    let said = output::decider(id, &asked, review.as_deref(), lead, commit.as_deref());
    Ok(Some(said))
}

/// `text` trimmed, when it is one line and not empty: a question is a
/// heading, and an answer one line under it.
fn one_line(text: &str) -> Option<&str> {
    let text = text.trim();
    (!text.is_empty() && !text.contains(['\n', '\r'])).then_some(text)
}
