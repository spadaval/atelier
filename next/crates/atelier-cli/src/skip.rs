//! `ATELIER_SKIP`, the one way past a rule
//! (docs/strategies/redesign-architecture.md, Hooks and the bypass). A rule
//! is a refusal that depends on the state of the work or on who asks. Set to
//! a reason, the variable passes every rule one command meets: the command
//! prints each with the reason, and the commit it makes carries each as a
//! trailer. A refusal of anything else is not passed, and says so.
//!
//! Every refusal of a rule goes through [`refuse`], so a rule cannot refuse
//! without offering the skip. One command is one process, so the rules
//! passed are kept here until the commit and the output take them.

use crate::commands::Outcome;
use crate::output;
use std::sync::Mutex;

const VARIABLE: &str = "ATELIER_SKIP";

/// Each rule passed, with the reason, as `<rule>: <reason>`.
static PASSED: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// The variable's value with its whitespace made one line: `None` when it
/// is unset, and empty when it is blank, which every command refuses.
pub fn variable() -> Option<String> {
    let value = std::env::var_os(VARIABLE)?;
    let words: Vec<&str> = value.to_str().unwrap_or("?").split_whitespace().collect();
    Some(words.join(" "))
}

/// The reason given, if any.
pub fn reason() -> Option<String> {
    variable().filter(|reason| !reason.is_empty())
}

/// A refusal of `rules`, each said in one line, whose `text` ends with the
/// way out. Given a reason, every rule passes and nothing is refused;
/// otherwise the refusal ends with the skip, after the way out.
pub fn refuse(rules: impl IntoIterator<Item = String>, text: String) -> Option<Outcome> {
    (!pass(rules)).then(|| Outcome::Refused(text + &output::skip_offer()))
}

/// Whether a reason is given, so that `rules` pass. Each is kept for the
/// output and the commit.
pub fn pass(rules: impl IntoIterator<Item = String>) -> bool {
    let Some(reason) = reason() else {
        return false;
    };
    let mut passed = PASSED.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    for rule in rules {
        let line = format!("{rule}: {reason}");
        if !passed.contains(&line) {
            passed.push(line);
        }
    }
    true
}

/// Each rule passed so far, as `<rule>: <reason>`.
pub fn passed() -> Vec<String> {
    PASSED
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone()
}
