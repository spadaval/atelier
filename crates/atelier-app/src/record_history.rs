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
/// commit deleted the file or left it unreadable.
#[derive(Debug, Clone)]
pub struct RecordVersion {
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

/// Every commit in HEAD's history that touched one of the records, read as
/// that commit left it. Empty without a commit to read.
pub fn issue_histories(read: &mut ReadContext<'_>, ids: &BTreeSet<String>) -> Result<RecordHistory> {
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
        "--date-order",
        "--format=%x00%H %ct",
        "--name-only",
        head.as_str(),
        "--",
    ];
    args.extend(paths.iter().map(String::as_str));
    let output = read.history(&args)?;
    for entry in output.split('\0').filter(|entry| !entry.trim().is_empty()) {
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
            let record = read.issue_at(commit, id).ok();
            history
                .versions
                .entry(id.to_string())
                .or_default()
                .push(RecordVersion {
                    commit: commit.to_string(),
                    time,
                    record,
                });
        }
    }
    Ok(history)
}

/// A dated moment in a record's history.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Moment {
    pub commit: String,
    pub time: DateTime<Utc>,
}

impl From<&RecordVersion> for Moment {
    fn from(version: &RecordVersion) -> Self {
        Self {
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
    let mut timeline = MissionTimeline {
        started,
        ..MissionTimeline::default()
    };
    for id in linked {
        let joined = history
            .of(id)
            .iter()
            .find(|version| version.record.as_ref().is_some_and(|record| advances(record, mission_id)))
            .or_else(|| {
                // Older records stored the link on the mission's side.
                mission.iter().find(|version| {
                    version
                        .record
                        .as_ref()
                        .is_some_and(|record| advances(record, id))
                })
            })
            .map(Moment::from);
        if let (Some(joined), Some(started)) = (joined, &timeline.started) {
            if joined.time > started.time {
                timeline.added_after_start.push((id.clone(), joined));
            }
        }
    }
    let done = |record: &CanonicalIssueRecord| {
        policy.and_then(|policy| policy.status_category(&record.issue.status)) == Some("done")
    };
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
        .sort_by(|(a, x), (b, y)| (x.time, a).cmp(&(y.time, b)));
    timeline
        .reopened
        .sort_by(|(a, x), (b, y)| (x.time, a).cmp(&(y.time, b)));
    Ok(timeline)
}

fn advances(record: &CanonicalIssueRecord, other: &str) -> bool {
    record
        .relationships
        .relates
        .iter()
        .any(|link| link.relation_type == "advances" && link.id == other)
}
