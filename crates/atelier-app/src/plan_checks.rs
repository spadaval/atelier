//! Mechanical plan checks (atelier-edaj): an open work item with no Done
//! when lines or a Done when line with no text, and a repository-relative
//! Markdown link, in an issue record or a mission's strategy, whose target
//! does not exist. They are warnings: `atelier check` prints them and its
//! exit code does not change, so records that pass today still pass.

use std::collections::BTreeSet;
use std::fmt;
use std::path::{Path, PathBuf};

use atelier_records::{issue_record_path, CanonicalIssueRecord, IssueBody};

/// One mechanical problem, at the line of the file it is on.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanWarning {
    /// The file, relative to the repository root.
    pub path: String,
    /// The 1-based line in that file.
    pub line: usize,
    /// The problem and what to do instead.
    pub message: String,
}

impl fmt::Display for PlanWarning {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Warning: {}:{}: {}",
            self.path, self.line, self.message
        )
    }
}

/// The warnings for `records` (only `scoped_id`'s, when given) and the
/// strategies their missions link. `open` says whether a record's status is
/// not done. A file that cannot be read gets no warnings, since `check`
/// already reports a missing record or strategy.
pub fn plan_warnings(
    state_dir: &Path,
    repo_root: &Path,
    records: &[CanonicalIssueRecord],
    scoped_id: Option<&str>,
    open: impl Fn(&CanonicalIssueRecord) -> bool,
) -> Vec<PlanWarning> {
    let mut warnings = Vec::new();
    let mut strategies = BTreeSet::new();
    for record in records
        .iter()
        .filter(|record| scoped_id.is_none_or(|id| record.issue.id == id))
    {
        let file = state_dir.join(issue_record_path(&record.issue.id));
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        let path = display_path(repo_root, &file);
        if open(record) && matches!(record.sections.body, IssueBody::Issue { .. }) {
            warnings.extend(done_when_warnings(&path, &text));
        }
        warnings.extend(link_warnings(&path, &text, &file, repo_root));
        if let Some(strategy) = record
            .issue
            .strategy
            .as_ref()
            .filter(|_| record.issue.issue_type == "mission")
        {
            strategies.insert(repo_root.join(&strategy.path));
        }
    }
    for file in strategies {
        if let Ok(text) = std::fs::read_to_string(&file) {
            let path = display_path(repo_root, &file);
            warnings.extend(link_warnings(&path, &text, &file, repo_root));
        }
    }
    warnings
}

fn display_path(repo_root: &Path, file: &Path) -> String {
    file.strip_prefix(repo_root)
        .unwrap_or(file)
        .display()
        .to_string()
}

/// The body lines of `text` with their 1-based line numbers, after any
/// front matter.
fn body_lines(text: &str) -> impl Iterator<Item = (usize, &str)> {
    let mut lines = text
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line));
    if text.starts_with("---\n") || text.starts_with("---\r\n") {
        lines.next();
        for (_, line) in lines.by_ref() {
            if line.trim_end() == "---" {
                break;
            }
        }
    }
    lines
}

/// The text of a level-two heading, as the record parser reads it.
fn level_two_heading(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("##")?;
    (rest.is_empty() || rest.starts_with(char::is_whitespace)).then(|| rest.trim())
}

fn done_when_warnings(path: &str, text: &str) -> Vec<PlanWarning> {
    let warning = |line, message: &str| PlanWarning {
        path: path.to_string(),
        line,
        message: message.to_string(),
    };
    let mut warnings = Vec::new();
    let mut heading = None;
    let mut boxes = 0;
    for (number, line) in body_lines(text) {
        if let Some(name) = level_two_heading(line) {
            if heading.is_some() {
                break;
            }
            if name == "Done when" {
                heading = Some(number);
            }
            continue;
        }
        if heading.is_none() {
            continue;
        }
        if let Some((_, text)) = crate::done_when::checkbox(line) {
            boxes += 1;
            if text.is_empty() {
                warnings.push(warning(
                    number,
                    "This Done when line has no text. Next: write the observable result that finishes the work after the box, or delete the line.",
                ));
            }
        }
    }
    if let (Some(number), 0) = (heading, boxes) {
        warnings.push(warning(
            number,
            "Done when has no `- [ ]` lines, so `atelier review request` has nothing to hold the work to. Next: write each result that finishes the work as its own `- [ ] ` line under this heading.",
        ));
    }
    warnings
}

/// Inline Markdown links (`[text](target)`, images too) outside code whose
/// repository-relative target exists neither beside `file` nor under the
/// repository root. Links with a scheme, anchors alone and the anchor part
/// of a link are not checked.
fn link_warnings(path: &str, text: &str, file: &Path, repo_root: &Path) -> Vec<PlanWarning> {
    let directory = file.parent().unwrap_or(repo_root);
    let mut warnings = Vec::new();
    let mut fence: Option<&str> = None;
    for (number, line) in body_lines(text) {
        let trimmed = line.trim_start();
        let marker = ["```", "~~~"]
            .into_iter()
            .find(|marker| trimmed.starts_with(marker));
        match (fence, marker) {
            (None, Some(marker)) => {
                fence = Some(marker);
                continue;
            }
            (Some(open), Some(marker)) if open == marker => {
                fence = None;
                continue;
            }
            (Some(_), _) => continue,
            (None, None) => {}
        }
        for target in link_targets(&without_code_spans(line)) {
            let Some(relative) = repository_relative(&target) else {
                continue;
            };
            let candidates: Vec<PathBuf> = match relative.strip_prefix('/') {
                Some(rooted) => vec![repo_root.join(rooted)],
                None => vec![directory.join(&relative), repo_root.join(&relative)],
            };
            if !candidates.iter().any(|candidate| candidate.exists()) {
                warnings.push(PlanWarning {
                    path: path.to_string(),
                    line: number,
                    message: format!(
                        "The link to {relative} points at no file, beside this file or from the repository root. Next: correct the path, or remove the link."
                    ),
                });
            }
        }
    }
    warnings
}

/// `line` with each inline code span replaced by spaces.
fn without_code_spans(line: &str) -> String {
    let mut output = String::with_capacity(line.len());
    let mut in_code = false;
    for character in line.chars() {
        if character == '`' {
            in_code = !in_code;
            output.push(' ');
        } else {
            output.push(if in_code { ' ' } else { character });
        }
    }
    output
}

/// The target of each inline link on `line`: the text after `](` up to the
/// closing parenthesis or a space before a title, without angle brackets.
fn link_targets(line: &str) -> Vec<String> {
    let mut targets = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find("](") {
        rest = &rest[start + 2..];
        let target = match rest.strip_prefix('<') {
            Some(inner) => inner.split_once('>').map(|(target, _)| target),
            None => rest
                .find(|c: char| c == ')' || c.is_whitespace())
                .map(|end| &rest[..end]),
        };
        if let Some(target) = target {
            targets.push(target.to_string());
        }
    }
    targets
}

/// The path part of a repository-relative link target; `None` for an empty
/// target, an anchor alone, or a target with a URL scheme (`https:`,
/// `mailto:`).
fn repository_relative(target: &str) -> Option<String> {
    let path = target.split(['#', '?']).next().unwrap_or_default();
    let scheme = path.split_once(':').is_some_and(|(scheme, _)| {
        scheme
            .chars()
            .next()
            .is_some_and(|first| first.is_ascii_alphabetic())
            && scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
    });
    (!path.is_empty() && !scheme).then(|| path.replace("%20", " "))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRONT: &str = "---\nid: x\nnote: \"[a](missing.md)\"\n---\n\n";

    #[test]
    fn done_when_without_lines_or_with_an_empty_line_warns_at_the_line() {
        let text = format!("{FRONT}## Goal\n\nShip.\n\n## Done when\n\nThe tests pass.\n\n## Scope\n\n- [ ] not done when\n");
        let warnings = done_when_warnings("r.md", &text);
        assert_eq!(warnings.len(), 1, "{warnings:?}");
        assert_eq!(warnings[0].line, 10);
        assert!(warnings[0]
            .message
            .starts_with("Done when has no `- [ ]` lines"));

        let text =
            format!("{FRONT}## Goal\n\nShip.\n\n## Done when\n\n- [ ] One.\n- [ ]\n- [x]   \n");
        let lines = done_when_warnings("r.md", &text)
            .into_iter()
            .map(|warning| warning.line)
            .collect::<Vec<_>>();
        assert_eq!(lines, [13, 14]);

        let text = format!("{FRONT}## Goal\n\nShip.\n\n## Done when\n\n- One.\n2. Two.\n");
        let warnings = done_when_warnings("r.md", &text);
        assert_eq!(warnings.len(), 1, "{warnings:?}");
        assert_eq!(warnings[0].line, 10);
        assert!(warnings[0].message.contains("its own `- [ ] ` line"));

        let text = format!("{FRONT}## Goal\n\nShip.\n\n## Done when\n\n- [ ] One.\n");
        assert_eq!(done_when_warnings("r.md", &text), []);
    }

    #[test]
    fn links_are_checked_beside_the_file_and_from_the_root_outside_code() {
        let root = tempfile::tempdir().unwrap();
        let docs = root.path().join("docs");
        std::fs::create_dir_all(docs.join("strategies")).unwrap();
        std::fs::write(docs.join("a.md"), "").unwrap();
        std::fs::write(docs.join("strategies/b.md"), "").unwrap();
        let file = docs.join("strategies/s.md");
        let text = format!(
            "{FRONT}See [a](docs/a.md), [b](b.md#part), [up](../a.md), [root](/docs/a.md \"t\"),\n\
             [web](https://example.com/x.md), [mail](mailto:a@b), [anchor](#here), ![img](<docs/a.md>).\n\
             Broken: [gone](docs/gone.md) and [also](./nope.md#x).\n\
             `[code](docs/code.md)`\n```\n[fenced](docs/fenced.md)\n```\n[after](missing/after.md)\n"
        );
        let warnings = link_warnings("s.md", &text, &file, root.path());
        let found = warnings
            .iter()
            .map(|warning| (warning.line, warning.message.clone()))
            .collect::<Vec<_>>();
        assert_eq!(found.len(), 3, "{found:#?}");
        assert_eq!(found[0].0, 8);
        assert!(found[0].1.contains("docs/gone.md"));
        assert!(found[1].1.contains("./nope.md"));
        assert_eq!(found[2].0, 13);
        assert!(found[2].1.contains("missing/after.md"));
    }
}
