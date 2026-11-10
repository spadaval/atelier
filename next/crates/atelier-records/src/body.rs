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
                Some((number.parse::<u32>().ok().filter(|n| *n > 0)?, title))
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
/// its text. Other lines, such as a wrapped line's continuation, are text.
pub(crate) fn done_when(body: &str) -> Vec<(bool, String)> {
    let Some(content) = section(body, 2, "Done when") else {
        return Vec::new();
    };
    unfenced_lines(content)
        .into_iter()
        .filter_map(|(_, line)| check(line))
        .collect()
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

/// Whether an item's `## Report` holds a line that is neither blank nor a
/// heading. A line inside a fenced code block, such as a transcript, counts.
pub(crate) fn report_written(body: &str) -> bool {
    let Some(report) = section(body, 2, "Report") else {
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
                (false, "One".to_string()),
                (true, "Two".to_string()),
                (true, "Nested".to_string()),
                (false, String::new()),
            ]
        );
        assert!(done_when("## Goal\n- [ ] x\n").is_empty());
    }

    #[test]
    fn a_report_is_written_when_it_holds_more_than_headings() {
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
}
