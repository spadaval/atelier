//! `escalate`: a question the implementer cannot settle with a fix or a
//! rebuttal, which a decider answers with `decide` (`atelier_app::decide`).
//! It writes the item's Decisions section in the checkout on its branch and
//! commits nothing; `submit` commits it with the item.

use crate::commands::Outcome;
use crate::output;
use anyhow::{bail, Result};
use atelier_app::acts::{Act, Facts};
use atelier_app::repo::{facts, item_path};
use atelier_app::{decider, one_line, Repo};
use atelier_core::{Decision, FindingName};
use atelier_records::ask;

pub fn escalate(repo: &mut Repo, question: &str, finding: Option<&str>) -> Result<Outcome> {
    let Some(located) = repo.current()? else {
        let branch = repo.branch.as_deref();
        return Ok(Outcome::Refused(output::not_on_item("escalate", branch)));
    };
    let record = located.record().expect("the current item has a record");
    let links = repo.links(record)?;
    let facts = facts(repo, &located, &links, None);
    if let Some(refused) = repo.skip.refusal(Act::Escalate, &facts)? {
        return Ok(refused.into());
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
