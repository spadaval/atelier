//! Done when checks that run after the merge (ADR 0027).
//!
//! A Done when check is checked before merge unless its line is marked:
//!
//! ```text
//! - [ ] After merge (<reason>): <check>. Stand-in: <what runs before merge>.
//! ```
//!
//! Validation on the base ticks the box (`[x]`); a check moved to another
//! item names it (`carried to <id>`, an existing issue). The marker is plain
//! Done when text, so records keep their format and every Atelier reads
//! them. Nothing here refuses: the checks are shown where work is judged.

use atelier_core::Issue;
use atelier_records::{CanonicalIssueRecord, IssueSectionName};

use crate::record_index::RecordIndex;
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

/// The after-merge checks of a Done when, in line order. A check counts as
/// carried only when the item it names is an issue in `index`.
pub fn checks(done_when: &str, index: &RecordIndex) -> Vec<AfterMergeCheck> {
    checks_known(done_when, &|id| index.issue_record(id).is_some())
}

/// The after-merge checks of `record` still waiting for validation.
pub fn open_checks(record: &CanonicalIssueRecord, index: &RecordIndex) -> Vec<AfterMergeCheck> {
    record
        .sections
        .section(IssueSectionName::DoneWhen)
        .map(|text| checks(text, index))
        .unwrap_or_default()
        .into_iter()
        .filter(|check| check.state == CheckState::Open)
        .collect()
}

/// Done work, not superseded, with an after-merge check still open.
pub fn awaiting_validation(
    record: &CanonicalIssueRecord,
    policy: Option<&WorkflowPolicy>,
    index: &RecordIndex,
) -> bool {
    is_done(&record.issue, policy) && !open_checks(record, index).is_empty()
}

fn is_done(issue: &Issue, policy: Option<&WorkflowPolicy>) -> bool {
    let category = match policy {
        Some(policy) => policy.status_category(&issue.status) == Some("done"),
        None => issue.status == "done",
    };
    category && issue.status != "superseded"
}

/// Whether `line` carries the After merge marker, ticked or not.
pub fn is_marked(line: &str) -> bool {
    marked(line).is_some()
}

/// Whether `after` differs from `before` only by recording validation: open
/// after-merge checks ticked, or given a `carried to <id>` clause naming an
/// issue in `index`, with their reason, check and stand-in otherwise
/// unchanged. Any other change edits what done means.
pub fn only_validated(before: &str, after: &str, index: &RecordIndex) -> bool {
    only_validated_known(before, after, &|id| index.issue_record(id).is_some())
}

fn checks_known(done_when: &str, known: &dyn Fn(&str) -> bool) -> Vec<AfterMergeCheck> {
    done_when
        .lines()
        .filter_map(|line| parse_line(line, known))
        .collect()
}

fn only_validated_known(before: &str, after: &str, known: &dyn Fn(&str) -> bool) -> bool {
    let before = before.trim().lines().collect::<Vec<_>>();
    let after = after.trim().lines().collect::<Vec<_>>();
    before.len() == after.len()
        && before
            .iter()
            .zip(&after)
            .all(|(old, new)| old.trim() == new.trim() || records_validation(old, new, known))
}

/// `new` is `old`, an open check, ticked or carried and otherwise the same.
fn records_validation(old: &str, new: &str, known: &dyn Fn(&str) -> bool) -> bool {
    let (Some(old_line), Some(new_line)) = (marked(old), marked(new)) else {
        return false;
    };
    let Some(old_check) = parse_line(old, known) else {
        return false;
    };
    if old_check.state != CheckState::Open || old_line.reason != new_line.reason {
        return false;
    }
    let same = |body: &str| clean(body) == clean(old_line.body);
    if new_line.ticked {
        return same(new_line.body);
    }
    match carried_clause(new_line.body) {
        Some((start, end, id)) if known(&id) => {
            let mut rest = new_line.body[..start].to_string();
            rest.push_str(&new_line.body[end..]);
            same(&rest)
        }
        _ => false,
    }
}

/// Text compared for equality: trimmed, without trailing punctuation.
fn clean(text: &str) -> &str {
    text.trim().trim_end_matches(['.', ';', ',', ' '])
}

/// The parts of a marked line.
struct Marked<'a> {
    ticked: bool,
    reason: Option<String>,
    /// Everything after the colon.
    body: &'a str,
}

fn marked(line: &str) -> Option<Marked<'_>> {
    let mut rest = strip_bullet(line.trim());
    let mut ticked = false;
    for (box_text, checked) in [("[ ]", false), ("[x]", true), ("[X]", true)] {
        if let Some(after) = rest.strip_prefix(box_text) {
            rest = after.trim_start();
            ticked = checked;
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
    Some(Marked {
        ticked,
        reason,
        body: rest.strip_prefix(':')?.trim(),
    })
}

fn parse_line(line: &str, known: &dyn Fn(&str) -> bool) -> Option<AfterMergeCheck> {
    let Marked {
        ticked,
        reason,
        body,
    } = marked(line)?;
    let (check, stand_in) = match find_ignore_case(body, "stand-in:") {
        Some(at) => (
            body[..at].trim(),
            Some(body[at + "stand-in:".len()..].trim().to_string()).filter(|text| !text.is_empty()),
        ),
        None => (body, None),
    };
    let state = if ticked {
        CheckState::Validated
    } else {
        match carried_clause(body) {
            Some((_, _, id)) if known(&id) => CheckState::Carried(id),
            _ => CheckState::Open,
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

/// The last `carried to <id>` clause in `body`: where it starts, where it
/// ends, and the id it names.
fn carried_clause(body: &str) -> Option<(usize, usize, String)> {
    const CLAUSE: &str = "carried to ";
    let start = body.to_ascii_lowercase().rfind(CLAUSE)?;
    let after = start + CLAUSE.len();
    let rest = &body[after..];
    let lead = rest.len() - rest.trim_start().len();
    let id = rest
        .trim_start()
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
        .next()?
        .trim_end_matches('-');
    (!id.is_empty()).then(|| (start, after + lead + id.len(), id.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn known(id: &str) -> bool {
        id == "atelier-ab12"
    }

    fn one(line: &str) -> AfterMergeCheck {
        let mut found = checks_known(line, &known);
        assert_eq!(found.len(), 1, "{line}");
        found.remove(0)
    }

    fn validated(before: &str, after: &str) -> bool {
        only_validated_known(before, after, &known)
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
    fn carried_to_counts_only_when_it_names_an_existing_issue() {
        for line in [
            "- [ ] After merge (human): the data is carried to the server and looks right.",
            "- [ ] After merge (use): a week of use. Carried to atelier-zz99.",
        ] {
            assert_eq!(one(line).state, CheckState::Open, "{line}");
        }
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
            assert!(checks_known(text, &known).is_empty(), "{text}");
        }
    }

    #[test]
    fn validating_or_carrying_an_open_check_is_not_a_done_when_edit() {
        let open = "Tests pass.\n- [ ] After merge (device): opens. Stand-in: emulator.";
        assert!(validated(
            open,
            "Tests pass.\n- [x] After merge (device): opens. Stand-in: emulator."
        ));
        assert!(validated(
            open,
            "Tests pass.\n- [ ] After merge (device): opens. Stand-in: emulator. Carried to atelier-ab12."
        ));
        assert!(validated(
            "- After merge (use): a week of use",
            "- After merge (use): a week of use; carried to atelier-ab12"
        ));
    }

    #[test]
    fn changing_a_check_while_ticking_or_carrying_it_is_a_done_when_edit() {
        let open =
            "- [ ] After merge (device): the launcher opens on the phone. Stand-in: emulator.";
        for after in [
            // Ticked, but the check or stand-in changed.
            "- [x] After merge (device): it compiles.",
            "- [x] After merge (device): the launcher opens on the phone. Stand-in: none.",
            // Ticked under another reason.
            "- [x] After merge (human): the launcher opens on the phone. Stand-in: emulator.",
            // Carried, but reworded, or to no existing issue.
            "- [ ] After merge (device): it compiles. Carried to atelier-ab12.",
            "- [ ] After merge (device): the launcher opens on the phone. Stand-in: emulator. Carried to atelier-zz99.",
        ] {
            assert!(!validated(open, after), "{after}");
        }
        // Moving a check to after merge, changing another line, or unticking.
        assert!(!validated(
            "Tests pass.\n- Opens on the phone.",
            "Tests pass.\n- [ ] After merge (device): opens on the phone."
        ));
        assert!(!validated(
            "Tests pass.\n- [ ] After merge (device): opens.",
            "Tests run.\n- [x] After merge (device): opens."
        ));
        assert!(!validated(
            "- [x] After merge (device): opens.",
            "- [ ] After merge (device): opens."
        ));
    }
}
