//! The one way past a rule (docs/design.md,
//! Hooks and the bypass). A rule is a refusal that depends on the state of
//! the work or on who asks. Given a reason, a [`Skip`] passes every rule
//! one invocation meets: the caller says each with the reason, and the
//! commit the invocation makes carries each as a trailer, with the thread
//! that ran it. A refusal of anything else is not passed.
//!
//! Every refusal of a rule goes through [`Skip::refuse`], so a rule cannot
//! refuse without offering the skip.

use crate::actions::{check, mission_refusals, refusals, Action, Facts, MissionFacts, Refusal};
use crate::model::Thread;
use crate::{entry, say};
use anyhow::Result;
use std::cell::RefCell;

/// The variable that gives a command a skip's reason, which the Git hooks
/// an action's commit or push runs are given from its [`Skip`].
pub const VARIABLE: &str = "ATELIER_SKIP";

/// The key of the trailer that holds one rule passed, as `<rule>: <reason>`.
pub const TRAILER: &str = "Atelier-Skip";

/// The key of the trailer that names the thread that passed a commit's
/// rules. A binary from before it reads only [`TRAILER`], so a commit
/// holding both reads there as it always did.
pub const BY_TRAILER: &str = "Atelier-Skip-By";

/// A refused action: the rules it breaks, one line each, which a reason would
/// pass, or none when what refused it is no rule; what it says; and the way
/// out, where it names one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refused {
    pub rules: Vec<String>,
    pub text: String,
    /// The way out, without its line break.
    pub next: Option<String>,
}

impl Refused {
    /// A refusal that says `text` and then `next`, the way out.
    pub fn then(text: String, next: String) -> Refused {
        let next = next.strip_suffix('\n').map(str::to_string).unwrap_or(next);
        Refused {
            rules: Vec::new(),
            text,
            next: Some(next),
        }
    }
}

impl From<String> for Refused {
    fn from(text: String) -> Refused {
        Refused {
            rules: Vec::new(),
            text,
            next: None,
        }
    }
}

/// One invocation's skip: the reason given, if any, the thread that gave
/// it, and each rule it has passed. Its caller makes one per invocation and
/// gives it to whatever opens a repository, makes a commit or prints the
/// outcome, so a passed rule reaches that invocation's commit and output
/// and no other's.
#[derive(Debug, Default)]
pub struct Skip {
    reason: Option<String>,
    by: Option<Thread>,
    passed: RefCell<Vec<String>>,
}

impl Skip {
    /// A skip for `reason`, its whitespace made one line. A blank reason is
    /// none.
    pub fn new(reason: Option<&str>) -> Skip {
        let reason = reason.map(one_line);
        Skip {
            reason: reason.filter(|reason| !reason.is_empty()),
            by: None,
            passed: RefCell::default(),
        }
    }

    /// This skip, given by `thread`, the thread running the invocation,
    /// which the commit's trailers name beside each rule it passes. A skip
    /// with no thread writes the rules alone, as one did before
    /// 2026-11-18 (atelier-nv22).
    pub fn by(self, thread: Thread) -> Skip {
        Skip {
            by: Some(thread),
            ..self
        }
    }

    /// The reason given, if any.
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// A refusal of `rules`, each said in one line, which says `refused`.
    /// Given a reason, every rule passes and nothing is refused.
    pub fn refuse(
        &self,
        rules: impl IntoIterator<Item = String>,
        refused: impl Into<Refused>,
    ) -> Option<Refused> {
        let rules: Vec<String> = rules.into_iter().collect();
        if self.pass(rules.iter().cloned()) {
            return None;
        }
        Some(Refused {
            rules,
            ..refused.into()
        })
    }

    /// Whether a reason is given, so that `rules` pass. Each is kept for the
    /// output and the commit.
    pub fn pass(&self, rules: impl IntoIterator<Item = String>) -> bool {
        let Some(reason) = &self.reason else {
            return false;
        };
        let mut passed = self.passed.borrow_mut();
        for rule in rules {
            // A trailer is one line: a merge check's command may hold several.
            let rule = one_line(&rule);
            let line = format!("{rule}: {reason}");
            if !passed.contains(&line) {
                passed.push(line);
            }
        }
        true
    }

    /// Whether a reason is given that names `name`, so that `rule` passes,
    /// as [`Skip::pass`] passes it. A rule that fails the work itself, as a
    /// merge check does, passes only this way, so that a reason written for
    /// another rule cannot carry the work past it (atelier-7ci6).
    pub fn pass_naming(&self, name: &str, rule: String) -> bool {
        match &self.reason {
            Some(reason) if names(reason, name) => self.pass([rule]),
            _ => false,
        }
    }

    /// Each rule passed so far, as `<rule>: <reason>`.
    pub fn passed(&self) -> Vec<String> {
        self.passed.borrow().clone()
    }

    /// The trailers of a commit of what this invocation wrote: one
    /// `Atelier-Skip: <rule>: <reason>` for each rule passed so far, then
    /// `Atelier-Skip-By: <thread>`; none when no rule was passed.
    pub fn trailers(&self) -> Vec<String> {
        let passed = self.passed();
        if passed.is_empty() {
            return Vec::new();
        }
        let mut trailers: Vec<String> = (passed.iter())
            .map(|line| format!("{TRAILER}: {line}"))
            .collect();
        trailers.extend(self.by.iter().map(|by| format!("{BY_TRAILER}: {by}")));
        trailers
    }

    /// What the caller is told when `action` is refused, or `None` when it may
    /// run, as when a reason passed every refusal.
    pub fn refusal(&self, action: Action, facts: &Facts) -> Result<Option<Refused>> {
        match check(action, facts) {
            Ok(_) => Ok(None),
            Err(refusal) => {
                let refused = refused(action, &refusal, facts)?;
                Ok(self.gated(action, facts, refused))
            }
        }
    }

    /// `refused`, which refuses `action` and names the way out, as the caller
    /// gets it: `None` when a reason passes every refusal of the action.
    pub fn gated(
        &self,
        action: Action,
        facts: &Facts,
        refused: impl Into<Refused>,
    ) -> Option<Refused> {
        let all = refusals(action, facts).unwrap_or_else(|problem| vec![Refusal::Record(problem)]);
        self.gate(action, &facts.item.item.id, &all, refused)
    }

    /// [`Skip::gated`] for the refusals `all` of `action` on `id`: only when
    /// every one is a rule can they be skipped.
    pub fn gate(
        &self,
        action: Action,
        id: &str,
        all: &[Refusal],
        refused: impl Into<Refused>,
    ) -> Option<Refused> {
        if !all.iter().all(|refusal| refusal.is_rule(action)) {
            return Some(refused.into());
        }
        self.refuse(all.iter().flat_map(|r| say::rules(action, id, r)), refused)
    }

    /// What the caller is told when `action`, a mission's, is refused, or
    /// `None` when it may run, as when a reason passed every refusal. A
    /// record that does not read is an error.
    pub fn mission_gate(
        &self,
        action: Action,
        id: &str,
        facts: &MissionFacts,
    ) -> Result<Option<Refused>> {
        let all = mission_refusals(action, facts)?;
        Ok(match all.first() {
            None => None,
            Some(first) => self.gate(action, id, &all, say::mission_refused(action, id, first)),
        })
    }
}

/// Whether `reason` names `name`: writes it followed by a colon, at its
/// start or after a space, quote or backtick, each with its whitespace made
/// one line. So the short check `true` is not named by "construe:".
fn names(reason: &str, name: &str) -> bool {
    let named = format!("{}:", one_line(name));
    let reason = one_line(reason);
    (reason.match_indices(&named)).any(|(at, _)| {
        reason[..at]
            .chars()
            .next_back()
            .is_none_or(|before| before == ' ' || "\"'`".contains(before))
    })
}

/// `text` with its whitespace made single spaces, as a trailer's one line.
fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// What the caller is told when `action` is refused for `refusal`. Only a
/// refusal that says what comes next reads the item's findings, so a
/// reviewer's draft that does not parse yet cannot hide its role line.
pub fn refused(action: Action, refusal: &Refusal, facts: &Facts) -> Result<Refused> {
    let entry = match refusal {
        Refusal::Record(problem) => return Err(problem.clone().into()),
        Refusal::From { .. } | Refusal::Unmet(_) => Some(entry(facts)?),
        Refusal::Role(_) | Refusal::FromMission { .. } | Refusal::NotHolder { .. } => None,
    };
    let id = &facts.item.item.id;
    Ok(say::refusal(
        action,
        refusal,
        facts.caller,
        id,
        entry.as_ref(),
        facts.item.dropped(),
    ))
}
