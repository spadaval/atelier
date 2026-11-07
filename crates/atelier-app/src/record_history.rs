//! One Git-derived query over the history of a set of issue records
//! (atelier-rx22). Mission start, scope added after it, and reopened items
//! all ask when a record first appeared or changed status; one `git log`
//! over the records' paths and the batched object reader answer them, so the
//! walk scales with those records' commits, not with review events.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::Result;
use atelier_core::{EvidenceResult, IssueSectionName, ReviewEvent};
use atelier_records::{issue_record_path, CanonicalIssueRecord};
use chrono::{DateTime, TimeZone, Utc};

use crate::git_freshness::{git_output, ReadContext};
use crate::record_index::RecordIndex;
use crate::workflow_policy::{status_is_done, WorkflowPolicy};

/// One committed version of an issue record; `record` is `None` where the
/// commit deleted the file or left it unreadable. `position` orders versions
/// along the walk [`issue_histories`] makes.
#[derive(Debug, Clone)]
pub struct RecordVersion {
    pub position: usize,
    pub commit: String,
    /// The commit this version's change is counted from, when it is not
    /// `commit`'s first parent: the fork, for the branch's entry commit.
    pub since: Option<String>,
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
/// When HEAD is a work branch off `base`, the base's first-parent line up to
/// the newest base commit HEAD holds (the fork) comes first, so merging the
/// base in does not fold the base's history into one merge (atelier-jnec).
/// The branch's line follows from the first commit that holds the fork,
/// counted as its change from the fork: branch commits from before it read
/// records as they were before the base's newer versions, so their changes
/// arrive with it instead. Empty without a commit to read.
pub fn issue_histories(
    read: &mut ReadContext<'_>,
    base: Option<&str>,
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
    let output = walk(read, &head, base, &paths)?;
    let entries = output.split('\0').filter(|entry| !entry.trim().is_empty());
    for (position, entry) in entries.enumerate() {
        let mut lines = entry.lines();
        let Some((commit, rest)) = lines.next().and_then(|line| line.split_once(' ')) else {
            continue;
        };
        let (seconds, since) = match rest.split_once(' ') {
            Some((seconds, since)) => (seconds, Some(since.trim().to_string())),
            None => (rest, None),
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
                    since: since.clone(),
                    time,
                    record,
                });
        }
    }
    Ok(history)
}

/// `git log` output, oldest first, of the commits that make up HEAD's
/// history of `paths`, as [`issue_histories`] describes it.
fn walk(
    read: &mut ReadContext<'_>,
    head: &str,
    base: Option<&str>,
    paths: &[String],
) -> Result<String> {
    let log = |read: &mut ReadContext<'_>, revisions: &str| {
        let mut args = vec![
            "log",
            "--reverse",
            "--first-parent",
            "--diff-merges=first-parent",
            "--format=%x00%H %ct",
            "--name-only",
            revisions,
            "--",
        ];
        args.extend(paths.iter().map(String::as_str));
        read.history(&args)
    };
    let Some(fork) = fork(read, head, base) else {
        return log(read, head);
    };
    let mut output = log(read, &fork)?;
    let line = read.history(&[
        "rev-list",
        "--reverse",
        "--first-parent",
        &format!("{fork}..{head}"),
    ])?;
    let mut entry = None;
    for commit in line.lines().filter(|line| !line.trim().is_empty()) {
        if read.ancestor(&fork, commit)? {
            entry = Some(commit.to_string());
            break;
        }
    }
    let Some(entry) = entry else {
        return Ok(output);
    };
    let mut args = vec!["diff", "--name-only", fork.as_str(), entry.as_str(), "--"];
    args.extend(paths.iter().map(String::as_str));
    let changed = read.history(&args)?;
    if !changed.trim().is_empty() {
        let seconds = read.history(&["log", "-1", "--format=%ct", &entry])?;
        output.push_str(&format!("\0{entry} {} {fork}\n{changed}", seconds.trim()));
    }
    output.push_str(&log(read, &format!("{entry}..{head}"))?);
    Ok(output)
}

/// The newest base commit HEAD holds, when HEAD is not on the base's line:
/// `None` on the base, behind it, or without a base branch.
fn fork(read: &ReadContext<'_>, head: &str, base: Option<&str>) -> Option<String> {
    let commit = |args: &[&str]| {
        git_output(read.repo, args)
            .ok()
            .and_then(|output| String::from_utf8(output).ok())
            .map(|output| output.trim().to_string())
            .filter(|output| !output.is_empty())
    };
    let base = commit(&[
        "rev-parse",
        "--verify",
        "--quiet",
        &format!("refs/heads/{}^{{commit}}", base?),
    ])?;
    commit(&["merge-base", head, &base]).filter(|fork| fork != head)
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
    /// As on [`RecordVersion`].
    pub since: Option<String>,
    pub time: DateTime<Utc>,
}

impl From<&RecordVersion> for Moment {
    fn from(version: &RecordVersion) -> Self {
        Self {
            position: version.position,
            commit: version.commit.clone(),
            since: version.since.clone(),
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
    /// Items first linked to the mission after it started, with where the
    /// link reached the walk and, by the record's own clock, when it was
    /// written (atelier-b3qt).
    pub added_after_start: Vec<(String, Moment)>,
    /// Items whose done claim was later withdrawn, with when.
    pub reopened: Vec<(String, Moment)>,
}

/// Derives the mission's start, the scope added after it, and reopened items
/// from one history query over the mission and its scope.
pub fn mission_timeline<'a>(
    read: &mut ReadContext<'_>,
    policy: Option<&WorkflowPolicy>,
    mission_id: &'a str,
    scope: &BTreeSet<String>,
    linked: &'a BTreeSet<String>,
    children: &'a [(String, String)],
) -> Result<MissionTimeline> {
    let mut ids = scope.clone();
    ids.insert(mission_id.to_string());
    let base = policy.map(|policy| policy.branch_policy.base_branch.as_str());
    let history = issue_histories(read, base, &ids)?;
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
    let done = |record: &CanonicalIssueRecord| status_is_done(policy, &record.issue.status);
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
    let first = |link: Link<'a>| {
        history
            .of(link.holder())
            .iter()
            .find(|version| {
                version
                    .record
                    .as_ref()
                    .is_some_and(|record| link.held_by(record))
            })
            .map(|version| (Moment::from(version), link))
    };
    let earlier = |a: Option<(Moment, Link<'a>)>, b: Option<(Moment, Link<'a>)>| match (a, b) {
        (Some(a), Some(b)) => Some(if a.0.position <= b.0.position { a } else { b }),
        (a, b) => a.or(b),
    };
    let mut joined_at = BTreeMap::new();
    for id in linked {
        // The link may live on either record, and older records stored it on
        // the mission's side, so the item joined at the earlier of the two.
        let joined = earlier(
            first(Link::Advances {
                holder: id,
                other: mission_id,
            }),
            first(Link::Advances {
                holder: mission_id,
                other: id,
            }),
        );
        if let Some(joined) = joined {
            joined_at.insert(id.as_str(), joined);
        }
    }
    let started_at = timeline.started.as_ref().map(|moment| moment.position);
    let mut added = Vec::new();
    for (id, (joined, link)) in &joined_at {
        if started_at.is_some_and(|started| joined.position > started) {
            added.push((id.to_string(), joined.clone(), *link));
        }
    }
    // A child joined when its parent's record first listed it; it is new
    // scope only when its parent was already in scope at the start.
    for (child, parent) in children {
        let parent_joined = joined_at
            .get(parent.as_str())
            .map(|(moment, _)| moment.position);
        let Some((joined, link)) = first(Link::Child { parent, child }) else {
            continue;
        };
        if let Some(started) = started_at {
            if joined.position > started && parent_joined.is_none_or(|at| at <= started) {
                added.push((child.clone(), joined, link));
            }
        }
    }
    // Dated by the record clock, as delivery is, but no earlier than the
    // start: a link written on a branch before the start joined the scope
    // only when it reached the mission's line after it.
    let started_time = timeline.started.as_ref().map(|moment| moment.time);
    for (id, mut joined, link) in added {
        if let Some(time) = written_at(read, &joined, link)? {
            joined.time = started_time.map_or(time, |started| time.max(started));
        }
        timeline.added_after_start.push((id, joined));
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

/// A link that puts an item in the mission's scope, on the record that
/// holds it.
#[derive(Debug, Clone, Copy)]
enum Link<'a> {
    /// `holder`'s record says it advances `other`.
    Advances { holder: &'a str, other: &'a str },
    /// `parent`'s record lists `child`.
    Child { parent: &'a str, child: &'a str },
}

impl Link<'_> {
    fn holder(&self) -> &str {
        match self {
            Self::Advances { holder, .. } => holder,
            Self::Child { parent, .. } => parent,
        }
    }

    fn held_by(&self, record: &CanonicalIssueRecord) -> bool {
        match self {
            Self::Advances { other, .. } => advances(record, other),
            Self::Child { child, .. } => record
                .relationships
                .children
                .iter()
                .any(|link| link.id == *child),
        }
    }
}

/// When the link was written, by the record's own clock, as delivery is
/// dated: the holder's `updated_at` in the first commit that held it among
/// those `joined` brought in: the commits since its first parent, or since
/// the fork for the branch's entry commit (atelier-a5vf). A merge brings in
/// its other parents' commits, so a link written on a branch is dated when
/// it was written there, not when the branch merged. `None` when no such
/// commit holds it.
fn written_at(
    read: &mut ReadContext<'_>,
    joined: &Moment,
    link: Link<'_>,
) -> Result<Option<DateTime<Utc>>> {
    let since = match &joined.since {
        Some(since) => Some(since.clone()),
        None => read.first_parent(&joined.commit)?,
    };
    let range = match since {
        Some(since) => format!("{since}..{}", joined.commit),
        None => joined.commit.clone(),
    };
    let path = Path::new(".atelier")
        .join(issue_record_path(link.holder()))
        .to_string_lossy()
        .into_owned();
    let listed = read.history(&[
        "log",
        "--reverse",
        "--no-merges",
        "--format=%H",
        &range,
        "--",
        &path,
    ])?;
    for commit in listed.lines().filter(|line| !line.trim().is_empty()) {
        let record = read
            .issue_at(commit, link.holder())
            .ok()
            .or_else(|| retired_epic_at(read, commit, link.holder()));
        if let Some(record) = record.filter(|record| link.held_by(record)) {
            return Ok(Some(record.issue.updated_at));
        }
    }
    Ok(None)
}

fn advances(record: &CanonicalIssueRecord, other: &str) -> bool {
    record
        .relationships
        .relates
        .iter()
        .any(|link| link.relation_type == "advances" && link.id == other)
}

/// A Done when edited after its item was started (atelier-r9hb): shown
/// where closing is judged, so a criterion changed once work began is seen
/// by whoever judges the work. It refuses nothing.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DoneWhenEdit {
    pub issue: String,
    /// The commit that changed the Done when.
    pub commit: String,
    /// The newest finding or failing receipt on the item pinned to an
    /// ancestor of `commit`, which the edit may answer.
    pub after: Option<Precedent>,
}

/// A record that judged the item before its Done when changed.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Precedent {
    Finding(String),
    FailingReceipt(String),
}

/// Every edit to the Done when of `ids` that HEAD holds, made in a commit
/// descending from one that started the item: moved its status into an
/// `active` category (`in_progress` without a policy). Edits, starts,
/// findings and receipts are ordered by Git ancestry, never by time, so
/// work merged from another branch is judged where it was written. Merges
/// are not read: the commit that made an edit is in HEAD's history itself.
/// Recording an after-merge check's validation is not an edit.
pub fn done_when_edits(
    read: &mut ReadContext<'_>,
    policy: Option<&WorkflowPolicy>,
    index: &RecordIndex,
    ids: &BTreeSet<String>,
) -> Result<Vec<DoneWhenEdit>> {
    let mut edits = Vec::new();
    let Ok(head) = read.pinned("HEAD") else {
        return Ok(edits);
    };
    let active = |record: &CanonicalIssueRecord| match policy {
        Some(policy) => policy.status_category(&record.issue.status) == Some("active"),
        None => record.issue.status == "in_progress",
    };
    let done_when = |record: &CanonicalIssueRecord| {
        record
            .sections
            .section(IssueSectionName::DoneWhen)
            .map(|text| text.trim().to_string())
    };
    for id in ids {
        let path = Path::new(".atelier")
            .join(issue_record_path(id))
            .to_string_lossy()
            .into_owned();
        let commits = read.history(&[
            "log",
            "--reverse",
            "--full-history",
            "--no-merges",
            "--format=%H %P",
            &head,
            "--",
            &path,
        ])?;
        let version = |read: &mut ReadContext<'_>, commit: &str| {
            read.issue_at(commit, id)
                .ok()
                .or_else(|| retired_epic_at(read, commit, id))
        };
        let mut starts = Vec::new();
        // Each edit, with its parent: an ancestor of the parent is a strict
        // ancestor of the edit.
        let mut changed = Vec::new();
        for line in commits.lines() {
            let mut parts = line.split_whitespace();
            let Some(commit) = parts.next() else {
                continue;
            };
            let Some(after) = version(read, commit) else {
                continue;
            };
            let before = match parts.next() {
                Some(parent) => version(read, parent).map(|record| (parent.to_string(), record)),
                None => None,
            };
            if active(&after) && before.as_ref().is_none_or(|(_, record)| !active(record)) {
                starts.push(commit.to_string());
            }
            if let Some((parent, before)) = before {
                let (before, after) = (done_when(&before), done_when(&after));
                // Ticking or carrying an after-merge check records its
                // validation; it does not change what done means (ADR 0027).
                let validated = matches!((&before, &after), (Some(before), Some(after))
                    if crate::after_merge::only_validated(before, after));
                if before != after && !validated {
                    changed.push((commit.to_string(), parent));
                }
            }
        }
        for (commit, parent) in changed {
            let mut started = false;
            for start in &starts {
                if read.ancestor(start, &parent)? {
                    started = true;
                    break;
                }
            }
            if started {
                let after = precedent(read, index, id, &parent)?;
                edits.push(DoneWhenEdit {
                    issue: id.clone(),
                    commit,
                    after,
                });
            }
        }
    }
    Ok(edits)
}

/// The newest finding or failing receipt on `id` pinned to an ancestor of
/// `parent`, by when it was recorded.
fn precedent(
    read: &mut ReadContext<'_>,
    index: &RecordIndex,
    id: &str,
    parent: &str,
) -> Result<Option<Precedent>> {
    let mut candidates = index
        .reviews()
        .filter(|review| review.target == id && matches!(review.event, ReviewEvent::Finding { .. }))
        .map(|review| {
            (
                review.header.created_at,
                review.commit.clone(),
                Precedent::Finding(review.header.id.clone()),
            )
        })
        .collect::<Vec<_>>();
    for link in index.list_record_links("issue", id)? {
        let evidence = match (link.relation_type.as_str(), &link) {
            ("validates", link) if link.source_kind == "evidence" => &link.source_id,
            ("validates", link) if link.target_kind == "evidence" => &link.target_id,
            _ => continue,
        };
        let record = index.require_evidence(evidence)?;
        if record.data.result == EvidenceResult::Fail {
            candidates.push((
                record.header.created_at,
                record.data.commit.clone(),
                Precedent::FailingReceipt(record.header.id.clone()),
            ));
        }
    }
    candidates.sort_by(|a, b| b.0.cmp(&a.0));
    for (_, pin, found) in candidates {
        // A pin that is not a full commit ID judged nothing in this history.
        if read.ancestor(&pin, parent).unwrap_or(false) {
            return Ok(Some(found));
        }
    }
    Ok(None)
}
