//! What the tool reads from a record's Markdown body: an item's sections,
//! its Decisions entries and Responses lines, and a review's findings.

use crate::document::{sections, unfenced_lines};
use crate::{ids, Problem};
use atelier_core::{
    Answer, Decision, Finding, FindingName, Now, Outcome, Recommendation, Response, ReviewName,
    Settlement, Thread,
};
use std::path::Path;

/// The content under the first heading `heading` of `level` in `text`.
pub(crate) fn section<'a>(text: &'a str, level: usize, heading: &str) -> Option<&'a str> {
    sections(text, level)
        .into_iter()
        .find(|(found, _)| *found == heading)
        .map(|(_, content)| content)
}

/// A review's `## Finding <k>: <title>` sections.
pub(crate) fn findings(
    file: &Path,
    review: &ReviewName,
    body: &str,
) -> Result<Vec<Finding>, Problem> {
    let mut findings: Vec<Finding> = Vec::new();
    for (heading, content) in sections(body, 2) {
        let Some(rest) = heading.strip_prefix("Finding ") else {
            continue;
        };
        let problem = |message: String| Problem::new(file, format!("## {heading}"), message);
        let (number, title) = rest
            .split_once(": ")
            .and_then(|(number, title)| {
                // One spelling per number, as a finding's name has, so the
                // name read is the one the heading is found by.
                let n = number.parse::<u32>().ok();
                Some((n.filter(|n| *n > 0 && n.to_string() == number)?, title))
            })
            .ok_or_else(|| problem("a finding heading is `## Finding <k>: <title>`".into()))?;
        let name = FindingName {
            review: review.clone(),
            number,
        };
        if findings.iter().any(|f| f.name == name) {
            return Err(problem(format!("finding {number} appears twice")));
        }
        findings.push(finding(name, title, content).map_err(problem)?);
    }
    Ok(findings)
}

fn finding(name: FindingName, title: &str, content: &str) -> Result<Finding, String> {
    let mut lines = fields(content).into_iter();
    let recommendation = lines
        .next()
        .and_then(|line| line.strip_prefix("Recommendation: "))
        .ok_or("its first line is not `Recommendation: fix now | defer | no action`")?;
    let recommendation = match recommendation {
        "fix now" => Recommendation::FixNow,
        "defer" => Recommendation::Defer,
        "no action" => Recommendation::NoAction,
        other => {
            return Err(format!(
                "`Recommendation: {other}` is not fix now, defer or no action"
            ))
        }
    };
    let (mut was, mut now, mut decided, mut settlement) = (None, None, None, None);
    for line in lines {
        if let Some(value) = line.strip_prefix("Was: ") {
            set(&mut was, "Was", value.parse()?)?;
        } else if let Some(value) = line.strip_prefix("Now: ") {
            let value = match value {
                "fixed" => Now::Fixed,
                "withdrawn" => Now::Withdrawn,
                "kept" => Now::Kept,
                other => return Err(format!("`Now: {other}` is not fixed, withdrawn or kept")),
            };
            set(&mut now, "Now", value)?;
        } else if let Some(value) = line.strip_prefix("Decided: ") {
            set(&mut decided, "Decided", outcome("Decided", value)?)?;
        } else if let Some(value) = line.strip_prefix("Taken: ") {
            let (item, by) = value
                .split_once(" by ")
                .ok_or_else(|| format!("`{line}` is not `Taken: <item> by <thread>`"))?;
            ids::validate_id(item)?;
            let taken = Settlement::Taken {
                item: item.to_string(),
                by: Thread(by.to_string()),
            };
            set(&mut settlement, "Taken or Dropped", taken)?;
        } else if let Some(value) = line.strip_prefix("Dropped by ") {
            let (by, reason) = value
                .split_once(": ")
                .ok_or_else(|| format!("`{line}` is not `Dropped by <thread>: <reason>`"))?;
            let dropped = Settlement::Dropped {
                by: Thread(by.to_string()),
                reason: reason.to_string(),
            };
            set(&mut settlement, "Taken or Dropped", dropped)?;
        } else if line.starts_with("Recommendation: ") {
            return Err("it has two Recommendation lines".into());
        }
    }
    if was.is_some() != (now.is_some() || decided.is_some()) || (now.is_some() && decided.is_some())
    {
        return Err("a carried finding has a `Was:` line and one `Now:` or `Decided:` line".into());
    }
    // Keeping a fix-now finding moves it to this one, which must stay fix now.
    if now == Some(Now::Kept) && recommendation != Recommendation::FixNow {
        return Err("a finding carried as `Now: kept` is `Recommendation: fix now`".into());
    }
    Ok(Finding {
        name,
        title: title.to_string(),
        recommendation,
        was,
        now,
        decided,
        settlement,
    })
}

/// `body`, a review's, with the line that records `settlement` added to
/// finding `number`, right after its Recommendation line: that line is
/// outside any fence, so the new one is read whatever else the finding
/// holds. `None` when the review has no such finding.
pub(crate) fn settle(body: &str, number: u32, settlement: &Settlement) -> Option<String> {
    let prefix = format!("Finding {number}: ");
    let (_, content) = sections(body, 2)
        .into_iter()
        .find(|(heading, _)| heading.starts_with(&prefix))?;
    let (offset, first) = unfenced_lines(content)
        .into_iter()
        .find(|(_, line)| !line.trim().is_empty())?;
    // The content is a slice of the body.
    let at = content.as_ptr() as usize - body.as_ptr() as usize + offset + first.len();
    let line = match settlement {
        Settlement::Taken { item, by } => format!("Taken: {item} by {by}"),
        Settlement::Dropped { by, reason } => format!("Dropped by {by}: {reason}"),
    };
    let (head, rest) = body.split_at(at);
    Some(format!("{head}\n{line}{rest}"))
}

/// The `### <date>: <question>` entries of an item's `## Decisions`.
pub(crate) fn decisions(file: &Path, body: &str) -> Result<Vec<Decision>, Problem> {
    let Some(entries) = section(body, 2, "Decisions") else {
        return Ok(Vec::new());
    };
    sections(entries, 3)
        .into_iter()
        .map(|(question, content)| {
            decision(question, content)
                .map_err(|message| Problem::new(file, format!("Decisions: {question}"), message))
        })
        .collect()
}

fn decision(question: &str, content: &str) -> Result<Decision, String> {
    let (mut finding, mut answer, mut outcome_line) = (None, None, None);
    for line in fields(content) {
        if let Some(value) = line.strip_prefix("Finding: ") {
            set(&mut finding, "Finding", value.parse::<FindingName>()?)?;
        } else if let Some(value) = line.strip_prefix("Answer by ") {
            let (by, text) = value
                .split_once(": ")
                .ok_or_else(|| format!("`{line}` is not `Answer by <thread>: <answer>`"))?;
            let text = text.to_string();
            set(
                &mut answer,
                "Answer",
                Answer {
                    by: Thread(by.to_string()),
                    text,
                },
            )?;
        } else if let Some(value) = line.strip_prefix("Outcome: ") {
            set(&mut outcome_line, "Outcome", outcome("Outcome", value)?)?;
        }
    }
    if outcome_line.is_some() && (finding.is_none() || answer.is_none()) {
        return Err("an `Outcome:` needs a `Finding:` and an `Answer by`".into());
    }
    Ok(Decision {
        question: question.to_string(),
        finding,
        answer,
        outcome: outcome_line,
    })
}

/// `body` with a new, unanswered question at the end of its
/// `## Decisions`, about `finding` when one is given, adding the section at
/// the end of the body when it has none. `question` is the entry's heading,
/// `<date>: <question>`.
pub fn ask(body: &str, question: &str, finding: Option<&FindingName>) -> String {
    let mut entry = format!("### {question}");
    if let Some(finding) = finding {
        entry += &format!("\n\nFinding: {finding}");
    }
    match section(body, 2, "Decisions") {
        Some(content) if content.trim().is_empty() => add_after(body, content, &entry),
        Some(content) => add_after(body, content, &format!("\n{entry}")),
        None => {
            let sep = if body.is_empty() || body.ends_with('\n') {
                ""
            } else {
                "\n"
            };
            format!("{body}{sep}\n## Decisions\n\n{entry}\n")
        }
    }
}

/// `body` with `Answer by <by>: <text>`, and `Outcome: <outcome>` when one
/// is given, under the entry of `## Decisions` at `index`, which
/// [`decisions`] reads in the same order. `None` when there is no such
/// entry.
pub fn answer(
    body: &str,
    index: usize,
    by: &Thread,
    text: &str,
    outcome: Option<Outcome>,
) -> Option<String> {
    let entries = section(body, 2, "Decisions")?;
    let (_, content) = *sections(entries, 3).get(index)?;
    let mut lines = format!("Answer by {by}: {text}");
    if let Some(outcome) = outcome {
        lines += &format!("\nOutcome: {outcome}");
    }
    Some(add_after(body, content, &lines))
}

/// `body` with `lines` on the line after the last line of `content`, a
/// section of `body`, that is not blank.
fn add_after(body: &str, content: &str, lines: &str) -> String {
    let start = content.as_ptr() as usize - body.as_ptr() as usize;
    let (head, tail) = body.split_at(start + content.trim_end().len());
    let end = if tail.starts_with('\n') { "" } else { "\n" };
    format!("{head}\n{lines}{end}{tail}")
}

/// The lines of an item's `### Responses`, under `## Report`.
pub(crate) fn responses(file: &Path, body: &str) -> Result<Vec<Response>, Problem> {
    let Some(lines) = section(body, 2, "Report").and_then(|report| section(report, 3, "Responses"))
    else {
        return Ok(Vec::new());
    };
    fields(lines)
        .into_iter()
        .map(|line| response(line).map_err(|message| Problem::new(file, "Responses", message)))
        .collect()
}

fn response(line: &str) -> Result<Response, String> {
    let form = || {
        format!("`{line}` is not `<finding>: fixed in <commit>` or `<finding>: rebutted: <fact>`")
    };
    let (finding, reply) = line
        .strip_prefix("- ")
        .unwrap_or(line)
        .split_once(": ")
        .ok_or_else(form)?;
    let finding: FindingName = finding.parse()?;
    if let Some(commit) = reply.strip_prefix("fixed in ") {
        Ok(Response::Fixed {
            finding,
            commit: commit.to_string(),
        })
    } else if let Some(fact) = reply.strip_prefix("rebutted: ") {
        Ok(Response::Rebutted {
            finding,
            fact: fact.to_string(),
        })
    } else {
        Err(form())
    }
}

/// The non-blank lines of `text` outside fenced code blocks, where a
/// record's field lines are; an example in a fence is not a field.
fn fields(text: &str) -> Vec<&str> {
    unfenced_lines(text)
        .into_iter()
        .map(|(_, line)| line.trim_end())
        .filter(|line| !line.is_empty())
        .collect()
}

fn outcome(key: &str, value: &str) -> Result<Outcome, String> {
    match value {
        "defer" => Ok(Outcome::Defer),
        "no action" => Ok(Outcome::NoAction),
        other => Err(format!("`{key}: {other}` is not defer or no action")),
    }
}

fn set<T>(slot: &mut Option<T>, key: &str, value: T) -> Result<(), String> {
    if slot.replace(value).is_some() {
        return Err(format!("it has two {key} lines"));
    }
    Ok(())
}

/// The checklist of an item's `## Done when`: each list line outside a
/// fenced code block that starts `[ ]` or `[x]`, as whether it is ticked and
/// its text. A line that wraps it, right under it, not blank and neither a
/// list line nor a heading, is part of its text, joined by a space.
pub(crate) fn done_when(body: &str) -> Vec<(bool, String)> {
    let Some(content) = section(body, 2, "Done when") else {
        return Vec::new();
    };
    let mut checks: Vec<(bool, String)> = Vec::new();
    // Whether the line read last is a check's or wraps one, and where the
    // line right under it starts; a fence between them ends the check.
    let (mut wrapping, mut under) = (false, None);
    for (start, line) in unfenced_lines(content) {
        let follows = under == Some(start);
        under = Some(start + line.len() + 1);
        if let Some(found) = check(line) {
            checks.push(found);
            wrapping = true;
            continue;
        }
        let text = line.trim();
        wrapping &= follows && !text.is_empty() && !listed(text) && !heading(text);
        if let (true, Some((_, last))) = (wrapping, checks.last_mut()) {
            if !last.is_empty() {
                last.push(' ');
            }
            last.push_str(text);
        }
    }
    checks
}

/// Whether `text`, trimmed, is a heading: one to six `#`, then a space or
/// nothing, as in `document.rs`. `#123` is text.
fn heading(text: &str) -> bool {
    let hashes = text.len() - text.trim_start_matches('#').len();
    (1..=6).contains(&hashes) && (text.len() == hashes || text[hashes..].starts_with(' '))
}

/// Whether `text`, trimmed, starts a list line: `-`, `*` or `+`, or a number
/// with `.` or `)`, each followed by a space.
fn listed(text: &str) -> bool {
    let numbered = text.trim_start_matches(|c: char| c.is_ascii_digit());
    let ordered =
        numbered.len() < text.len() && [". ", ") "].iter().any(|m| numbered.starts_with(m));
    ordered
        || ["- ", "* ", "+ "]
            .iter()
            .any(|marker| text.starts_with(marker))
}

fn check(line: &str) -> Option<(bool, String)> {
    let item = line.trim_start();
    let rest = ["- ", "* ", "+ "]
        .iter()
        .find_map(|marker| item.strip_prefix(marker))?;
    let (ticked, text) = if let Some(text) = rest.strip_prefix("[ ]") {
        (false, text)
    } else {
        let text = rest.strip_prefix("[x]").or(rest.strip_prefix("[X]"))?;
        (true, text)
    };
    (text.is_empty() || text.starts_with(' ')).then(|| (ticked, text.trim().to_string()))
}

/// Whether `## <heading>`, an item's Report or a mission's Result, holds a
/// line that is neither blank nor a heading. A line inside a fenced code
/// block, such as a transcript, counts.
pub(crate) fn written(body: &str, heading: &str) -> bool {
    let Some(report) = section(body, 2, heading) else {
        return false;
    };
    let written = report
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    let headings = unfenced_lines(report)
        .into_iter()
        .filter(|(_, line)| {
            let hashes = line.len() - line.trim_start_matches('#').len();
            (1..=6).contains(&hashes) && (line.len() == hashes || line[hashes..].starts_with(' '))
        })
        .count();
    written > headings
}

/// Why a record was dropped: the first line under its `## <heading>`, an
/// item's Report or a mission's Result, when that line starts `Dropped: `.
pub(crate) fn dropped<'a>(body: &'a str, heading: &str) -> Option<&'a str> {
    let first = section(body, 2, heading)?
        .lines()
        .find(|line| !line.trim().is_empty())?;
    first.strip_prefix("Dropped: ").map(str::trim)
}

/// `body` with `text` as the first lines under its `## <heading>`, before
/// any other text or subsection, or under that heading added at its end.
pub fn put_first(body: &str, heading: &str, text: &str) -> String {
    let text = text.trim();
    let Some(content) = section(body, 2, heading) else {
        let sep = if body.is_empty() || body.ends_with('\n') {
            ""
        } else {
            "\n"
        };
        return format!("{body}{sep}\n## {heading}\n\n{text}\n");
    };
    // The content is a slice of the body, starting after the heading's line.
    let at = content.as_ptr() as usize - body.as_ptr() as usize;
    let (head, rest) = body.split_at(at);
    let head = if head.ends_with('\n') {
        head.to_string()
    } else {
        format!("{head}\n")
    };
    let rest = rest.trim_start_matches('\n');
    let sep = if rest.is_empty() { "" } else { "\n" };
    format!("{head}\n{text}\n{sep}{rest}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_done_when_checklist_outside_fences() {
        let body = "## Done when\n\n- [ ] One\n  wrapped\n* [x] Two\n  - [X] Nested\n- [ ]\n\
                    - [-] Not a box\n- [ ]x glued\n```\n- [ ] In a fence\n```\n## Report\n- [ ] Elsewhere\n";
        assert_eq!(
            done_when(body),
            [
                (false, "One wrapped".to_string()),
                (true, "Two".to_string()),
                (true, "Nested".to_string()),
                (false, String::new()),
            ]
        );
        assert!(done_when("## Goal\n- [ ] x\n").is_empty());
    }

    #[test]
    fn a_done_when_line_wrapped_over_several_lines_is_read_whole() {
        let body = "## Done when\n\n- [ ] `slugify(\"a,  b\")` returns `a-b`,\n  and `slugify(\"--\")`\nreturns an empty slug.\n\
                    - [x] Next\n\n  after a blank line\n- [ ] Before a fence\n```\nfenced\n```\n  after it\n\
                    - [ ] Above a list\n  1. numbered\n- [ ] The output links to\n  #123 and explains the fix.\n\
                    - [ ] Above a heading\n### Notes\n";
        assert_eq!(
            done_when(body),
            [
                (
                    false,
                    "`slugify(\"a,  b\")` returns `a-b`, and `slugify(\"--\")` returns an empty slug."
                        .to_string()
                ),
                (true, "Next".to_string()),
                (false, "Before a fence".to_string()),
                (false, "Above a list".to_string()),
                (
                    false,
                    "The output links to #123 and explains the fix.".to_string()
                ),
                (false, "Above a heading".to_string()),
            ]
        );
    }

    #[test]
    fn a_report_is_written_when_it_holds_more_than_headings() {
        let report_written = |body| written(body, "Report");
        assert!(!report_written("## Goal\ng\n"));
        assert!(!report_written(
            "## Report\n\n### Validation\n\n### Responses\n\n## Next\nx\n"
        ));
        assert!(report_written("## Report\nDone.\n"));
        assert!(report_written(
            "## Report\n### Validation\n```\n# a shell comment\n```\n"
        ));
        assert!(report_written("## Report\n#hashtag is text\n"));
    }

    #[test]
    fn a_question_asked_and_answered_reads_back_in_order() {
        let path = Path::new("item.md");
        let finding: FindingName = "1-aaaa.1".parse().unwrap();
        let by = Thread("d1".into());
        // With no Decisions section, with an empty one before the Report,
        // and after an answered entry.
        for body in [
            "## Goal\ng",
            "## Goal\ng\n\n## Decisions\n\n## Report\nr\n",
            "## Decisions\n\n### 2026-11-09: Old?\n\nAnswer by d0: Yes.\n",
        ] {
            let asked = ask(body, "2026-11-10: Why?", Some(&finding));
            let asked = ask(&asked, "2026-11-10: And?", None);
            let before = decisions(path, body).unwrap().len();
            let read = decisions(path, &asked).unwrap();
            assert_eq!(read.len(), before + 2, "{asked}");
            assert_eq!(read[before].question, "2026-11-10: Why?");
            assert_eq!(read[before].finding.as_ref(), Some(&finding));
            assert!(read[before..].iter().all(|d| d.answer.is_none()));
            assert_eq!(read[before + 1].finding, None);
            assert!(asked.contains("## Report\nr\n") == body.contains("## Report"));

            let answered = answer(&asked, before, &by, "Not now.", Some(Outcome::Defer)).unwrap();
            let answered = answer(&answered, before + 1, &by, "Yes.", None).unwrap();
            let read = decisions(path, &answered).unwrap();
            assert_eq!(read[before].answer.as_ref().unwrap().text, "Not now.");
            assert_eq!(read[before].outcome, Some(Outcome::Defer));
            assert_eq!(read[before + 1].answer.as_ref().unwrap().by, by);
            assert_eq!(read[before + 1].outcome, None);
            assert_eq!(answer(&answered, before + 2, &by, "x", None), None);
        }
        assert_eq!(
            ask("## Goal\ng\n", "2026-11-10: Why?", None),
            "## Goal\ng\n\n## Decisions\n\n### 2026-11-10: Why?\n"
        );
        assert_eq!(
            answer(
                "## Decisions\n\n### Q\n\nFinding: 1-aaaa.1\n\n### R\n",
                0,
                &by,
                "No.",
                Some(Outcome::NoAction)
            )
            .unwrap(),
            "## Decisions\n\n### Q\n\nFinding: 1-aaaa.1\nAnswer by d1: No.\nOutcome: no action\n\n### R\n"
        );
    }

    #[test]
    fn an_identity_holding_a_slash_reads_back_whole_from_each_line_that_names_one() {
        let by = Thread("t3-root/codex-child".into());
        let path = Path::new("item.md");
        // `Answer by <thread>: <answer>`.
        let asked = ask("## Goal\ng\n", "2026-11-11: Why?", None);
        let answered = answer(&asked, 0, &by, "Because: reasons.", None).unwrap();
        let read = decisions(path, &answered).unwrap();
        let answer = read[0].answer.as_ref().unwrap();
        assert_eq!(
            (&answer.by, answer.text.as_str()),
            (&by, "Because: reasons.")
        );
        // `Taken: <item> by <thread>` and `Dropped by <thread>: <reason>`.
        let review = "1-aaaa".parse().unwrap();
        let body = "## Finding 1: One\nRecommendation: defer\n";
        for settlement in [
            Settlement::Taken {
                item: "demo-x1y2".into(),
                by: by.clone(),
            },
            Settlement::Dropped {
                by: by.clone(),
                reason: "gone: for good".into(),
            },
        ] {
            let settled = settle(body, 1, &settlement).unwrap();
            let read = findings(Path::new("r.md"), &review, &settled).unwrap();
            assert_eq!(read[0].settlement, Some(settlement), "{settled}");
        }
    }

    #[test]
    fn a_settlement_goes_right_after_the_recommendation_where_a_fence_cannot_hide_it() {
        let taken = Settlement::Taken {
            item: "demo-x1y2".into(),
            by: Thread("t9".into()),
        };
        let body = "## Summary\ns\n## Finding 1: One\n\nRecommendation: defer\n```\nopen fence\n## Finding 10: Ten\nRecommendation: defer";
        assert_eq!(
            settle(body, 1, &taken).unwrap(),
            "## Summary\ns\n## Finding 1: One\n\nRecommendation: defer\nTaken: demo-x1y2 by t9\n```\nopen fence\n## Finding 10: Ten\nRecommendation: defer"
        );
        let body =
            "## Finding 1: One\nRecommendation: fix now\n## Finding 10: Ten\nRecommendation: defer";
        let dropped = Settlement::Dropped {
            by: Thread("t9".into()),
            reason: "gone".into(),
        };
        let settled = settle(body, 10, &dropped).unwrap();
        assert!(settled.ends_with("Recommendation: defer\nDropped by t9: gone"));
        let review = "1-aaaa".parse().unwrap();
        let read = findings(Path::new("r.md"), &review, &settled).unwrap();
        assert_eq!(read[1].settlement, Some(dropped));
        assert_eq!(read[0].settlement, None);
        assert_eq!(settle(body, 2, &taken), None);
        // A heading whose number is spelt another way is not read as that
        // finding, so no name is listed that settling cannot find.
        for heading in ["## Finding 01: One", "## Finding +1: One"] {
            let body = format!("{heading}\nRecommendation: defer\n");
            assert!(
                findings(Path::new("r.md"), &review, &body).is_err(),
                "{heading}"
            );
        }
    }

    #[test]
    fn a_line_put_first_under_a_heading_is_read_back_as_the_drop_reason() {
        let put = |body| put_first(body, "Report", "Dropped: not needed");
        // Under a Report with text and a subsection, before both.
        let full = "\n## Goal\ng\n\n## Report\n\nIt half works.\n\n### Responses\n\n- 1-aaaa.1: fixed in abc1234\n\n## Decisions\n";
        assert_eq!(
            put(full),
            "\n## Goal\ng\n\n## Report\n\nDropped: not needed\n\nIt half works.\n\n### Responses\n\n- 1-aaaa.1: fixed in abc1234\n\n## Decisions\n"
        );
        // Under an empty Report, one that ends the body, and none at all.
        assert_eq!(
            put("## Report\n\n## Decisions\n"),
            "## Report\n\nDropped: not needed\n\n## Decisions\n"
        );
        assert_eq!(
            put("## Goal\n\n## Report"),
            "## Goal\n\n## Report\n\nDropped: not needed\n"
        );
        assert_eq!(
            put("\n## Goal\ng"),
            "\n## Goal\ng\n\n## Report\n\nDropped: not needed\n"
        );
        // A heading in a fence is text, so a section is added.
        assert_eq!(
            put("```\n## Report\n```\n"),
            "```\n## Report\n```\n\n## Report\n\nDropped: not needed\n"
        );
        for body in [full, "## Report\n\n## Decisions\n", "\n## Goal\ng"] {
            assert_eq!(dropped(&put(body), "Report"), Some("not needed"));
        }
        // Only the section's first line, and only under the heading named.
        assert_eq!(dropped(full, "Report"), None);
        assert_eq!(dropped("## Report\nx\nDropped: late\n", "Report"), None);
        assert_eq!(dropped("## Result\nDropped: why\n", "Report"), None);
        assert_eq!(dropped("## Result\nDropped: why\n", "Result"), Some("why"));
    }
}
