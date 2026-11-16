//! The records' steps read from commit subjects, for mission control's
//! progress log, the project home's week and the stage ages a host shows
//! (docs/architecture/desk.md, History from commit subjects). It is
//! approximate: it reads the subjects Atelier's commands write, so a step
//! committed under another subject is missed, and one a subject merely
//! looks like is counted.

use crate::api::{EventKind, HistoryEvent};
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::HashSet;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

/// The most commits one read looks at, newest first.
pub const COMMITS: usize = 20_000;
/// How long one `git log` may run.
pub const LIMIT: Duration = Duration::from_secs(15);

/// The words in `text` that look like record IDs: a lower-case slug, a
/// hyphen, and four lower-case letters or digits.
pub fn ids_in(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for word in text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-')) {
        let Some((head, tail)) = word.rsplit_once('-') else {
            continue;
        };
        if !head.is_empty()
            && head.starts_with(|c: char| c.is_ascii_lowercase())
            && tail.len() == 4
            && tail
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        {
            found.push(word.to_string());
        }
    }
    found
}

/// One step a subject records: its kind, ID, and a review's round and
/// verdict.
type Step = (EventKind, String, Option<u32>, Option<String>);

/// The steps a commit subject records, if it records any.
pub fn parse(subject: &str) -> Vec<Step> {
    let first = |rest: &str| ids_in(rest).into_iter().next();
    let round = |rest: &str| -> Option<(u32, String)> {
        let (_, after) = rest.split_once(", round ")?;
        let (n, verdict) = after.split_once(':')?;
        Some((n.trim().parse().ok()?, verdict.trim().to_string()))
    };
    let one = |kind, id| vec![(kind, id, None, None)];
    if let Some(rest) = subject.strip_prefix("Review the plan of ") {
        if let (Some(id), Some((n, v))) = (first(rest), round(rest)) {
            return vec![(EventKind::PlanReview, id, Some(n), Some(v))];
        }
    } else if let Some(rest) = subject.strip_prefix("Review ") {
        if let (Some(id), Some((n, v))) = (first(rest), round(rest)) {
            return vec![(EventKind::Review, id, Some(n), Some(v))];
        }
    } else if let Some(rest) = subject.strip_prefix("Start ") {
        if let Some(id) = first(rest).filter(|id| rest.trim() == id) {
            return one(EventKind::Started, id);
        }
    } else if let Some(rest) = subject.strip_prefix("Submit ") {
        if let Some(id) = first(rest) {
            return one(EventKind::Submitted, id);
        }
    } else if let Some(rest) = subject.strip_prefix("Merge ") {
        if rest.starts_with("branch ") || rest.starts_with("master into") {
            return vec![];
        }
        let head = rest.split(':').next().unwrap_or(rest);
        let head = head.trim_end_matches(" into master").trim();
        if let Some(id) = first(head) {
            if head == id || head.ends_with(&format!("/{id}")) {
                return one(EventKind::Merged, id);
            }
        }
    } else if let Some(rest) = subject.strip_prefix("Plan mission ") {
        if let Some(id) = first(rest) {
            return one(EventKind::Planned, id);
        }
    } else if let Some(rest) = subject.strip_prefix("Ready ") {
        let ids = ids_in(rest).into_iter();
        return ids.map(|id| (EventKind::Readied, id, None, None)).collect();
    } else if let Some(rest) = subject.strip_prefix("Close ") {
        if let Some(id) = first(rest) {
            return one(EventKind::Closed, id);
        }
    } else if let Some(rest) = subject.strip_prefix("Drop ") {
        if let Some(id) = first(rest).filter(|id| rest.trim() == id) {
            return one(EventKind::Dropped, id);
        }
    } else if let Some(rest) = subject.strip_prefix("Lead ") {
        if let Some(id) = first(rest) {
            return one(EventKind::Led, id);
        }
    }
    vec![]
}

/// The steps in `git log`'s output in the form `%H%x09%cI%x09%s`, newest
/// first, with a step recorded on two branches before a merge shown once.
pub fn events(log: &str) -> Vec<HistoryEvent> {
    let mut found = Vec::new();
    for line in log.lines() {
        let mut parts = line.splitn(3, '\t');
        let (Some(commit), Some(at), Some(subject)) = (parts.next(), parts.next(), parts.next())
        else {
            continue;
        };
        for (kind, id, round, verdict) in parse(subject) {
            found.push(HistoryEvent {
                commit: commit.to_string(),
                at: at.to_string(),
                kind,
                id,
                record: None,
                title: None,
                round,
                verdict,
                subject: subject.to_string(),
            });
        }
    }
    let mut seen = HashSet::new();
    found.retain(|e| {
        let key = (
            e.kind,
            e.id.clone(),
            e.round,
            e.subject.clone(),
            e.at.clone(),
        );
        seen.insert(key)
    });
    // Newest first by the time itself: committers' offsets differ, so the
    // text does not sort. A time that does not read sorts last.
    found.sort_by_key(|e| std::cmp::Reverse(DateTime::parse_from_rfc3339(&e.at).ok()));
    found
}

/// The steps on the local branches of `dir` and on `remote`'s, from the
/// newest `COMMITS` commits.
pub fn read(dir: &Path, remote: Option<&str>) -> Result<Vec<HistoryEvent>> {
    let mut command = Command::new("git");
    command.arg("-C").arg(dir).args([
        "log",
        "--branches",
        &format!("--max-count={COMMITS}"),
        "--format=%H%x09%cI%x09%s",
    ]);
    if let Some(remote) = remote {
        command.arg(format!("--remotes={remote}"));
    }
    let out = crate::bounded::run(atelier_core::git::unattended(&mut command), LIMIT)?;
    Ok(events(&String::from_utf8_lossy(&out)))
}

/// Seconds from the RFC 3339 time `at` to `now`; never negative.
pub fn seconds_since(at: &str, now: DateTime<Utc>) -> Option<i64> {
    let then = DateTime::parse_from_rfc3339(at).ok()?;
    Some((now - then.with_timezone(&Utc)).num_seconds().max(0))
}

/// Whether the RFC 3339 time `at` is within `days` days before `now`.
pub fn within(at: &str, days: u32, now: DateTime<Utc>) -> bool {
    seconds_since(at, now).is_some_and(|s| s <= i64::from(days) * 86_400)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(subject: &str) -> Vec<(EventKind, String, Option<u32>, Option<String>)> {
        parse(subject)
    }

    #[test]
    fn each_command_subject_is_a_step() {
        use EventKind::*;
        let id = |s: &str| s.to_string();
        let cases = [
            (
                "Plan mission atelier-61gh",
                vec![(Planned, id("atelier-61gh"), None, None)],
            ),
            (
                "Review the plan of atelier-61gh, round 1: changes requested",
                vec![(
                    PlanReview,
                    id("atelier-61gh"),
                    Some(1),
                    Some(id("changes requested")),
                )],
            ),
            (
                "Start atelier-cdxg",
                vec![(Started, id("atelier-cdxg"), None, None)],
            ),
            (
                "Submit atelier-jzaw for review",
                vec![(Submitted, id("atelier-jzaw"), None, None)],
            ),
            (
                "Review atelier-jzaw, round 2: approved",
                vec![(Review, id("atelier-jzaw"), Some(2), Some(id("approved")))],
            ),
            (
                "Merge atelier-jzaw: Show the project, not the system inbox",
                vec![(Merged, id("atelier-jzaw"), None, None)],
            ),
            (
                "Merge origin/atelier-jzaw into master",
                vec![(Merged, id("atelier-jzaw"), None, None)],
            ),
            (
                "Ready atelier-c1z3, atelier-jy8k: go",
                vec![
                    (Readied, id("atelier-c1z3"), None, None),
                    (Readied, id("atelier-jy8k"), None, None),
                ],
            ),
            (
                "Close atelier-61gh with its result",
                vec![(Closed, id("atelier-61gh"), None, None)],
            ),
            (
                "Drop atelier-yclc",
                vec![(Dropped, id("atelier-yclc"), None, None)],
            ),
            (
                "Lead atelier-to83",
                vec![(Led, id("atelier-to83"), None, None)],
            ),
        ];
        for (subject, want) in cases {
            assert_eq!(kinds(subject), want, "{subject}");
        }
    }

    #[test]
    fn other_subjects_are_no_step() {
        for subject in [
            "Merge branch 'master' into atelier-cdxg",
            "Merge master into atelier-cdxg",
            "Start the Desk on a new port",
            "Start atelier-cdxg and more",
            "Drop atelier-yclc's old field",
            "Review the docs",
            "Fix a typo",
        ] {
            assert_eq!(kinds(subject), vec![], "{subject}");
        }
    }

    #[test]
    fn a_step_on_two_branches_shows_once_newest_first() {
        let log = "a\t2026-11-16T10:00:00Z\tReview x-aaaa, round 1: approved\n\
                   b\t2026-11-16T10:00:00Z\tReview x-aaaa, round 1: approved\n\
                   c\t2026-11-16T06:00:00-05:00\tMerge x-aaaa: It\n\
                   d\t2026-11-16T09:00:00Z\tA commit\n";
        let found = events(log);
        let seen: Vec<_> = found.iter().map(|e| (e.commit.as_str(), e.kind)).collect();
        assert_eq!(seen, [("c", EventKind::Merged), ("a", EventKind::Review)]);
    }

    #[test]
    fn ages_are_read_from_rfc_3339() {
        let now = DateTime::parse_from_rfc3339("2026-11-16T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(seconds_since("2026-11-16T11:00:00+00:00", now), Some(3600));
        assert_eq!(seconds_since("2026-11-16T13:00:00Z", now), Some(0));
        assert_eq!(seconds_since("not a time", now), None);
        assert!(within("2026-11-10T12:00:00Z", 7, now));
        assert!(!within("2026-11-08T12:00:00Z", 7, now));
    }
}
