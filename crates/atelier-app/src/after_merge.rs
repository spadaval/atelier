//! The retired After merge marker (ADR 0027), which loose ends replace
//! (ADR 0028, decision 3). Old records may still carry it on a Done when
//! line:
//!
//! ```text
//! - [ ] After merge (<reason>): <check>. Stand-in: <what runs before merge>.
//! ```
//!
//! It is recognised only so `review request` need not ask for such a line to
//! be ticked, and so `atelier migrate after-merge` can turn each open one
//! into a loose end (atelier-8bh4). Everywhere else it is an ordinary Done
//! when line.

/// An old After merge line, as `migrate after-merge` reads it.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MarkedLine {
    pub ticked: bool,
    pub reason: Option<String>,
    /// The check, without its stand-in.
    pub check: String,
    pub stand_in: Option<String>,
    /// What a `carried to <id>` clause names; the old rule counted it only
    /// when it is an existing issue.
    pub carried_to: Option<String>,
    /// The line up to the marker: indentation, bullet and box.
    pub prefix: String,
}

/// Whether `line` carries the After merge marker, ticked or not.
pub fn is_marked(line: &str) -> bool {
    parse(line).is_some()
}

/// The parts of a marked line; `None` for any other line.
pub fn parse(line: &str) -> Option<MarkedLine> {
    let trimmed = line.trim_start();
    let mut rest = strip_bullet(trimmed);
    let mut ticked = false;
    for (box_text, checked) in [("[ ]", false), ("[x]", true), ("[X]", true)] {
        if let Some(after) = rest.strip_prefix(box_text) {
            rest = after.trim_start();
            ticked = checked;
            break;
        }
    }
    let prefix = line[..line.len() - rest.len()].to_owned();
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
    let (check, stand_in) = match body.to_ascii_lowercase().find("stand-in:") {
        Some(at) => (
            body[..at].trim(),
            Some(body[at + "stand-in:".len()..].trim().to_string()).filter(|text| !text.is_empty()),
        ),
        None => (body, None),
    };
    Some(MarkedLine {
        ticked,
        reason,
        check: check.trim_end_matches([';', ',']).trim().to_string(),
        stand_in,
        carried_to: carried_to(body),
        prefix,
    })
}

/// An open After merge line: one `atelier migrate after-merge` turns into a
/// loose end.
#[derive(Debug, Clone)]
pub struct OpenLine<'r> {
    pub item: &'r atelier_records::CanonicalIssueRecord,
    pub line: MarkedLine,
    /// The loose end an earlier run of the migration made for the line.
    pub existing: Option<String>,
}

/// The open After merge lines in `index`, the migration's work: each
/// unticked line not carried to an existing issue, on an item not
/// superseded, apart from one on a done item (`done`) an earlier run already
/// made a loose end for. `atelier check` notes them (atelier-l457).
pub fn open_lines<'r>(
    index: &'r crate::record_index::RecordIndex,
    done: impl Fn(&atelier_records::CanonicalIssueRecord) -> bool,
) -> Vec<OpenLine<'r>> {
    use atelier_workflow::LOOSE_END_TYPE;
    let migrated = |item: &str, check: &str| {
        index
            .issue_records()
            .filter(|record| {
                record.issue.issue_type == LOOSE_END_TYPE
                    && index
                        .get_blockers(&record.issue.id)
                        .iter()
                        .any(|id| id == item)
            })
            .find(|record| {
                record
                    .sections
                    .section(atelier_records::IssueSectionName::DoneWhen)
                    .unwrap_or_default()
                    .lines()
                    .any(|line| {
                        crate::done_when::checkbox(line)
                            .is_some_and(|(_, text)| text == check.trim())
                    })
            })
            .map(|record| record.issue.id.clone())
    };
    let mut open = Vec::new();
    for item in index.issue_records() {
        if item.issue.status == "superseded" {
            continue;
        }
        let done_when = item
            .sections
            .section(atelier_records::IssueSectionName::DoneWhen)
            .unwrap_or_default();
        for line in done_when.lines().filter_map(parse) {
            let carried = line
                .carried_to
                .as_deref()
                .is_some_and(|id| index.issue_record(id).is_some());
            if line.ticked || carried {
                continue;
            }
            let existing = migrated(&item.issue.id, &line.check);
            // A line already migrated on a done item has nothing left to do.
            if existing.is_some() && done(item) {
                continue;
            }
            open.push(OpenLine {
                item,
                line,
                existing,
            });
        }
    }
    open
}

/// Whether a check that waited for the merge for `reason` is work for the
/// human: the human's judgment, or a device or place no agent can reach.
/// The other reasons, the deployed or merged system and time and real use,
/// are not. Words of the reason decide, since the marker's reasons were
/// free text such as `device`, `Sumeet's judgement` or `needs Sumeet's PC`.
pub fn reason_needs_human(reason: &str) -> bool {
    const HUMAN: &[&str] = &[
        "human",
        "humans",
        "judgment",
        "judgement",
        "taste",
        "device",
        "devices",
        "phone",
        "tablet",
        "pc",
        "desktop",
        "laptop",
        "machine",
        "hardware",
        "place",
        "game",
        "console",
        "headset",
    ];
    reason
        .to_ascii_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .any(|word| HUMAN.contains(&word))
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

/// The id the last `carried to <id>` clause in `body` names.
fn carried_to(body: &str) -> Option<String> {
    const CLAUSE: &str = "carried to ";
    let start = body.to_ascii_lowercase().rfind(CLAUSE)? + CLAUSE.len();
    let id = body[start..]
        .trim_start()
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
        .next()?
        .trim_end_matches('-');
    (!id.is_empty()).then(|| id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(line: &str) -> MarkedLine {
        parse(line).unwrap_or_else(|| panic!("{line} is not marked"))
    }

    #[test]
    fn a_marked_line_gives_its_reason_check_stand_in_and_prefix() {
        let line = one(
            "  - [ ] After merge (device): the launcher opens on the phone. Stand-in: the emulator test passes.",
        );
        assert!(!line.ticked);
        assert_eq!(line.reason.as_deref(), Some("device"));
        assert_eq!(line.check, "the launcher opens on the phone.");
        assert_eq!(line.stand_in.as_deref(), Some("the emulator test passes."));
        assert_eq!(line.carried_to, None);
        assert_eq!(line.prefix, "  - [ ] ");
    }

    #[test]
    fn reason_stand_in_and_box_may_be_missing() {
        let line = one("after merge: it works in production");
        assert_eq!(line.reason, None);
        assert_eq!(line.stand_in, None);
        assert_eq!(line.check, "it works in production");
        assert_eq!(line.prefix, "");
        assert_eq!(one("1. After merge (): x").reason, None);
        assert_eq!(one("1. After merge (): x").prefix, "1. ");
    }

    #[test]
    fn ticks_and_carried_to_are_read() {
        assert!(one("- [x] After merge (human): Sumeet likes it").ticked);
        assert_eq!(
            one("- [ ] After merge (use): a week of use. Carried to atelier-ab12.")
                .carried_to
                .as_deref(),
            Some("atelier-ab12")
        );
    }

    #[test]
    fn unmarked_lines_and_prose_are_not_marked() {
        for text in [
            "The test passes.",
            "- [ ] The test passes.",
            "After merging, the base builds.",
            "After merge the base builds.",
            "After merge (device the phone opens",
            "",
        ] {
            assert!(!is_marked(text), "{text}");
        }
    }

    #[test]
    fn the_human_reasons_need_the_human() {
        for reason in [
            "device",
            "human",
            "Sumeet's judgement",
            "needs Sumeet's PC",
            "needs the game",
            "a place no agent can reach",
        ] {
            assert!(reason_needs_human(reason), "{reason}");
        }
        for reason in ["deployed", "merged system", "use", "time and real use", ""] {
            assert!(!reason_needs_human(reason), "{reason}");
        }
    }
}
