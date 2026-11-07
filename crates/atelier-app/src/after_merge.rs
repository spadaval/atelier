//! Done when checks that run after the merge (ADR 0027).
//!
//! A Done when check is checked before merge unless its line is marked:
//!
//! ```text
//! - [ ] After merge (<reason>): <check>. Stand-in: <what runs before merge>.
//! ```
//!
//! Validation on the base ticks the box (`[x]`); a check moved to another
//! item names it (`carried to <id>`). The marker is plain Done when text, so
//! records keep their format and every Atelier reads them. Nothing here
//! refuses: the checks are shown where work is judged.

use atelier_core::Issue;
use atelier_records::{CanonicalIssueRecord, IssueSectionName};

use crate::workflow_policy::WorkflowPolicy;

/// The reasons a check may wait for the merge, as agent-facing text names
/// them.
pub const REASONS: &str = "the deployed or merged system, a device or place no agent can reach, the human's judgment, or time and real use";

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CheckState {
    Open,
    Validated,
    /// Carried to the named item, which now holds it.
    Carried(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AfterMergeCheck {
    pub check: String,
    pub reason: Option<String>,
    pub stand_in: Option<String>,
    pub state: CheckState,
}

/// The after-merge checks of a Done when, in line order.
pub fn checks(done_when: &str) -> Vec<AfterMergeCheck> {
    done_when.lines().filter_map(parse_line).collect()
}

/// The after-merge checks of `record` still waiting for validation.
pub fn open_checks(record: &CanonicalIssueRecord) -> Vec<AfterMergeCheck> {
    record
        .sections
        .section(IssueSectionName::DoneWhen)
        .map(checks)
        .unwrap_or_default()
        .into_iter()
        .filter(|check| check.state == CheckState::Open)
        .collect()
}

/// Done work, not superseded, with an after-merge check still open.
pub fn awaiting_validation(record: &CanonicalIssueRecord, policy: Option<&WorkflowPolicy>) -> bool {
    is_done(&record.issue, policy) && !open_checks(record).is_empty()
}

fn is_done(issue: &Issue, policy: Option<&WorkflowPolicy>) -> bool {
    let category = match policy {
        Some(policy) => policy.status_category(&issue.status) == Some("done"),
        None => issue.status == "done",
    };
    category && issue.status != "superseded"
}

/// Whether `after` differs from `before` only by after-merge checks that
/// were open and are now validated or carried: recording validation, not
/// editing what done means.
pub fn only_validated(before: &str, after: &str) -> bool {
    let before = before.trim().lines().collect::<Vec<_>>();
    let after = after.trim().lines().collect::<Vec<_>>();
    before.len() == after.len()
        && before.iter().zip(&after).all(|(before, after)| {
            before.trim() == after.trim()
                || matches!(
                    (parse_line(before), parse_line(after)),
                    (Some(old), Some(new))
                        if old.state == CheckState::Open && new.state != CheckState::Open
                )
        })
}

fn parse_line(line: &str) -> Option<AfterMergeCheck> {
    let mut rest = strip_bullet(line.trim());
    let mut checked = false;
    for (box_text, ticked) in [("[ ]", false), ("[x]", true), ("[X]", true)] {
        if let Some(after) = rest.strip_prefix(box_text) {
            rest = after.trim_start();
            checked = ticked;
            break;
        }
    }
    const MARKER: &str = "after merge";
    if rest.len() < MARKER.len() || !rest.is_char_boundary(MARKER.len()) {
        return None;
    }
    if !rest[..MARKER.len()].eq_ignore_ascii_case(MARKER) {
        return None;
    }
    rest = rest[MARKER.len()..].trim_start();
    let mut reason = None;
    if let Some(inner) = rest.strip_prefix('(') {
        let close = inner.find(')')?;
        reason = Some(inner[..close].trim().to_string()).filter(|reason| !reason.is_empty());
        rest = inner[close + 1..].trim_start();
    }
    let body = rest.strip_prefix(':')?.trim();
    let (check, stand_in) = match find_ignore_case(body, "stand-in:") {
        Some(at) => (
            body[..at].trim(),
            Some(body[at + "stand-in:".len()..].trim().to_string()).filter(|text| !text.is_empty()),
        ),
        None => (body, None),
    };
    let state = if checked {
        CheckState::Validated
    } else {
        match carried_to(body) {
            Some(id) => CheckState::Carried(id),
            None => CheckState::Open,
        }
    };
    Some(AfterMergeCheck {
        check: check.trim_end_matches([';', ',']).trim().to_string(),
        reason,
        stand_in,
        state,
    })
}

/// `- `, `* `, `+ `, or `1. ` and `1) ` before a line's text.
fn strip_bullet(line: &str) -> &str {
    for bullet in ["- ", "* ", "+ "] {
        if let Some(rest) = line.strip_prefix(bullet) {
            return rest.trim_start();
        }
    }
    let digits = line.len() - line.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    if digits > 0 {
        if let Some(rest) = line[digits..]
            .strip_prefix(". ")
            .or_else(|| line[digits..].strip_prefix(") "))
        {
            return rest.trim_start();
        }
    }
    line
}

/// The byte offset of `needle` (ASCII) in `text`, ignoring ASCII case.
fn find_ignore_case(text: &str, needle: &str) -> Option<usize> {
    text.to_ascii_lowercase().find(needle)
}

/// The item a check names with `carried to <id>`.
fn carried_to(body: &str) -> Option<String> {
    let at = find_ignore_case(body, "carried to ")? + "carried to ".len();
    let id = body[at..]
        .trim_start()
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
        .next()?
        .trim_end_matches('-');
    (!id.is_empty()).then(|| id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(line: &str) -> AfterMergeCheck {
        let mut found = checks(line);
        assert_eq!(found.len(), 1, "{line}");
        found.remove(0)
    }

    #[test]
    fn a_marked_line_gives_its_reason_check_and_stand_in() {
        let check = one(
            "- [ ] After merge (device): the launcher opens on the phone. Stand-in: the emulator test passes.",
        );
        assert_eq!(check.reason.as_deref(), Some("device"));
        assert_eq!(check.check, "the launcher opens on the phone.");
        assert_eq!(check.stand_in.as_deref(), Some("the emulator test passes."));
        assert_eq!(check.state, CheckState::Open);
    }

    #[test]
    fn reason_and_stand_in_may_be_missing() {
        let check = one("after merge: it works in production");
        assert_eq!(check.reason, None);
        assert_eq!(check.stand_in, None);
        assert_eq!(check.check, "it works in production");
        assert_eq!(one("1. After merge (): x").reason, None);
    }

    #[test]
    fn a_ticked_box_validates_and_carried_to_names_the_item() {
        assert_eq!(
            one("- [x] After merge (human): Sumeet likes it").state,
            CheckState::Validated
        );
        assert_eq!(
            one("- [ ] After merge (use): a week of use. Carried to atelier-ab12.").state,
            CheckState::Carried("atelier-ab12".into())
        );
    }

    #[test]
    fn unmarked_lines_and_prose_are_not_after_merge_checks() {
        for text in [
            "The test passes.",
            "- [ ] The test passes.",
            "After merging, the base builds.",
            "After merge the base builds.",
            "After merge (device the phone opens",
            "",
        ] {
            assert!(checks(text).is_empty(), "{text}");
        }
    }

    #[test]
    fn validating_or_carrying_an_open_check_is_not_a_done_when_edit() {
        let open = "Tests pass.\n- [ ] After merge (device): opens. Stand-in: emulator.";
        assert!(only_validated(
            open,
            "Tests pass.\n- [x] After merge (device): opens. Stand-in: emulator."
        ));
        assert!(only_validated(
            open,
            "Tests pass.\n- [ ] After merge (device): opens. Stand-in: emulator. Carried to atelier-ab12."
        ));
        // Moving a check to after merge, or changing another line, is an edit.
        assert!(!only_validated(
            "Tests pass.\n- Opens on the phone.",
            "Tests pass.\n- [ ] After merge (device): opens on the phone."
        ));
        assert!(!only_validated(
            open,
            "Tests run.\n- [x] After merge (device): opens. Stand-in: emulator."
        ));
        assert!(!only_validated(
            "- [x] After merge (device): opens.",
            "- [ ] After merge (device): opens."
        ));
    }
}
