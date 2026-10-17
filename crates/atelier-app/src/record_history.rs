//! One Git-derived query over the history of a set of issue records
//! (atelier-rx22). Mission start, scope added after it, and reopened items
//! all ask when a record first appeared or changed status; one `git log`
//! over the records' paths and the batched object reader answer them, so the
//! walk scales with those records' commits, not with review events.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::Result;
use atelier_records::{issue_record_path, CanonicalIssueRecord};
use chrono::{DateTime, TimeZone, Utc};

use crate::git_freshness::ReadContext;
use crate::workflow_policy::WorkflowPolicy;

/// One committed version of an issue record; `record` is `None` where the
/// commit deleted the file or left it unreadable. `position` orders versions
/// along HEAD's first-parent history.
#[derive(Debug, Clone)]
pub struct RecordVersion {
    pub position: usize,
    pub commit: String,
    pub time: DateTime<Utc>,
    pub record: Option<CanonicalIssueRecord>,
}

/// Each record's versions, oldest first.
#[derive(Debug, Clone, Default)]
pub struct RecordHistory {
    pub versions: BTreeMap<String, Vec<RecordVersion>>,
}

impl RecordHistory {
    pub fn of(&self, id: &str) -> &[RecordVersion] {
        self.versions.get(id).map_or(&[], Vec::as_slice)
    }
}

/// Every commit on HEAD's first-parent history that changed one of the
/// records, read as that commit left it. A merge counts as the change it
/// made to its first parent, so work merged from a branch appears when it
/// reached this history, and branch-local states never interleave by date.
/// Empty without a commit to read.
pub fn issue_histories(
    read: &mut ReadContext<'_>,
    ids: &BTreeSet<String>,
) -> Result<RecordHistory> {
    let mut history = RecordHistory::default();
    if ids.is_empty() {
        return Ok(history);
    }
    let Ok(head) = read.pinned("HEAD") else {
        return Ok(history);
    };
    let paths = ids
        .iter()
        .map(|id| {
            Path::new(".atelier")
                .join(issue_record_path(id))
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    let mut args = vec![
        "log",
        "--reverse",
        "--first-parent",
        "--diff-merges=first-parent",
        "--format=%x00%H %ct",
        "--name-only",
        head.as_str(),
        "--",
    ];
    args.extend(paths.iter().map(String::as_str));
    let output = read.history(&args)?;
    let entries = output.split('\0').filter(|entry| !entry.trim().is_empty());
    for (position, entry) in entries.enumerate() {
        let mut lines = entry.lines();
        let Some((commit, seconds)) = lines.next().and_then(|line| line.split_once(' ')) else {
            continue;
        };
        let time = seconds
            .trim()
            .parse::<i64>()
            .ok()
            .and_then(|seconds| Utc.timestamp_opt(seconds, 0).single())
            .unwrap_or_default();
        for path in lines.filter(|line| !line.trim().is_empty()) {
            let Some(id) = Path::new(path)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .filter(|id| ids.contains(*id))
            else {
                continue;
            };
            let record = read
                .issue_at(commit, id)
                .ok()
                .or_else(|| retired_epic_at(read, commit, id));
            history
                .versions
                .entry(id.to_string())
                .or_default()
                .push(RecordVersion {
                    position,
                    commit: commit.to_string(),
                    time,
                    record,
                });
        }
    }
    Ok(history)
}

/// A version in the retired epic shape, read as the parent issue it became,
/// so the history before `atelier migrate epics` still shows its links and
/// status.
fn retired_epic_at(
    read: &mut ReadContext<'_>,
    commit: &str,
    id: &str,
) -> Option<CanonicalIssueRecord> {
    let relative = issue_record_path(id);
    let text = read
        .blob(commit, &Path::new(".atelier").join(&relative))
        .ok()?;
    let migrated = crate::epic_migration::migrate_epic_text(&text, "task", "draft")
        .ok()
        .flatten()?;
    atelier_records::parse_issue_record(&migrated.text, &relative).ok()
}

/// A dated moment in a record's history.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Moment {
    pub position: usize,
    pub commit: String,
    pub time: DateTime<Utc>,
}

impl From<&RecordVersion> for Moment {
    fn from(version: &RecordVersion) -> Self {
        Self {
            position: version.position,
            commit: version.commit.clone(),
            time: version.time,
        }
    }
}

/// What a mission's history says about its scope.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct MissionTimeline {
    /// The first version that left `draft`, or the mission's first version.
    pub started: Option<Moment>,
    /// The version that made the mission's current done claim.
    pub closed: Option<Moment>,
    /// Items first linked to the mission after it started, with when.
    pub added_after_start: Vec<(String, Moment)>,
    /// Items whose done claim was later withdrawn, with when.
    pub reopened: Vec<(String, Moment)>,
}

/// Derives the mission's start, the scope added after it, and reopened items
/// from one history query over the mission and its scope.
pub fn mission_timeline(
    read: &mut ReadContext<'_>,
    policy: Option<&WorkflowPolicy>,
    mission_id: &str,
    scope: &BTreeSet<String>,
    linked: &BTreeSet<String>,
    children: &[(String, String)],
) -> Result<MissionTimeline> {
    let mut ids = scope.clone();
    ids.insert(mission_id.to_string());
    let history = issue_histories(read, &ids)?;
    let mission = history.of(mission_id);
    let started = mission
        .iter()
        .find(|version| {
            version
                .record
                .as_ref()
                .is_some_and(|record| record.issue.status != "draft")
        })
        .map(Moment::from);
    let done = |record: &CanonicalIssueRecord| {
        policy.and_then(|policy| policy.status_category(&record.issue.status)) == Some("done")
    };
    // Walking back from the newest version, the oldest of the trailing run
    // of done versions made the current claim.
    let closed = mission
        .iter()
        .rev()
        .take_while(|version| version.record.as_ref().is_some_and(done))
        .last()
        .map(Moment::from);
    let mut timeline = MissionTimeline {
        started,
        closed,
        ..MissionTimeline::default()
    };
    let first = |versions: &[RecordVersion], test: &dyn Fn(&CanonicalIssueRecord) -> bool| {
        versions
            .iter()
            .find(|version| version.record.as_ref().is_some_and(test))
            .map(Moment::from)
    };
    let earlier = |a: Option<Moment>, b: Option<Moment>| match (a, b) {
        (Some(a), Some(b)) => Some(if a.position <= b.position { a } else { b }),
        (a, b) => a.or(b),
    };
    let mut joined_at = std::collections::BTreeMap::new();
    for id in linked {
        // The link may live on either record, and older records stored it on
        // the mission's side, so the item joined at the earlier of the two.
        let joined = earlier(
            first(history.of(id), &|record| advances(record, mission_id)),
            first(mission, &|record| advances(record, id)),
        );
        if let Some(joined) = joined {
            joined_at.insert(id.clone(), joined);
        }
    }
    let started_at = timeline.started.as_ref().map(|moment| moment.position);
    for (id, joined) in &joined_at {
        if started_at.is_some_and(|started| joined.position > started) {
            timeline
                .added_after_start
                .push((id.clone(), joined.clone()));
        }
    }
    // A child joined when its parent's record first listed it; it is new
    // scope only when its parent was already in scope at the start.
    for (child, parent) in children {
        let parent_joined = joined_at.get(parent).map(|moment| moment.position);
        let Some(joined) = first(history.of(parent), &|record| {
            record
                .relationships
                .children
                .iter()
                .any(|link| &link.id == child)
        }) else {
            continue;
        };
        if let Some(started) = started_at {
            if joined.position > started && parent_joined.is_none_or(|at| at <= started) {
                timeline.added_after_start.push((child.clone(), joined));
            }
        }
    }
    for id in scope {
        let mut was_done = false;
        for version in history.of(id) {
            let Some(record) = &version.record else {
                continue;
            };
            if done(record) {
                was_done = true;
            } else if was_done {
                timeline.reopened.push((id.clone(), Moment::from(version)));
                was_done = false;
            }
        }
    }
    timeline
        .added_after_start
        .sort_by(|(a, x), (b, y)| (x.position, a).cmp(&(y.position, b)));
    timeline
        .reopened
        .sort_by(|(a, x), (b, y)| (x.position, a).cmp(&(y.position, b)));
    Ok(timeline)
}

fn advances(record: &CanonicalIssueRecord, other: &str) -> bool {
    record
        .relationships
        .relates
        .iter()
        .any(|link| link.relation_type == "advances" && link.id == other)
}
