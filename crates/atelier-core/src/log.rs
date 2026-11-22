//! `log`, the history of actions (docs/design.md, The log), as data. Nothing
//! stores it: one `git log` lists every commit that changed a record, over
//! the base's tip and the tips of started items' branches, with each
//! record's old and new version; the repository's one `git cat-file`
//! process reads those versions; and an action is found by comparing a
//! record's two versions across one commit. A plan's merge is found by a
//! second listing, of the base's first-parent merges that changed a mission.

use crate::actions::STARTED;
use crate::model::{
    Finding, FindingName, Item, MissionStatus, Review, ReviewName, Settlement, Status, Thread,
    Threads, Verdict,
};
use crate::plan::{changed_sections, is_plan_file};
use crate::records::{
    old_status, parse_item, parse_mission, parse_review, validate_id, ItemDir, MissionDir,
    ReadError, ReviewFile,
};
use crate::repo::{item_path, mission_path};
use crate::skip::{BY_TRAILER, TRAILER};
use crate::{git, Repo, Walked};
use anyhow::{bail, Context, Result};
use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use std::collections::{BTreeSet, HashMap};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

/// One action, found in one commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Logged {
    /// The commit's committer time.
    pub time: DateTime<Utc>,
    /// The commit, as Git abbreviates it.
    pub commit: String,
    /// The item or mission the action is on.
    pub id: String,
    /// The thread the action is shown with: the one that ran it, where its
    /// commit names one, else the one the log's table gives, if any.
    pub thread: Option<Thread>,
    pub action: Happened,
    /// Each rule the action's commit carries an `Atelier-Skip` trailer for, as
    /// `<rule>: <reason>`.
    pub skips: Vec<String>,
    /// The thread that passed `skips`, as the commit's `Atelier-Skip-By`
    /// trailer names it; `None` in a commit made before that trailer.
    pub skipped_by: Option<Thread>,
}

/// A status a record moved from and to in one commit, each as that record
/// spells it, so an old item's `draft` and `ready` show as history has
/// them; `from` is `None` when the commit added the record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Option<&'static str>,
    pub to: &'static str,
}

impl Move {
    /// `<from> -> <to>`; `new` for a record the commit added.
    fn shown(&self) -> String {
        format!("{} -> {}", self.from.unwrap_or("new"), self.to)
    }
}

/// How the item record `text`, whose status reads as `status`, spells it.
fn spelled(status: Status, text: &str) -> &'static str {
    match status {
        Status::Open => old_status(text).unwrap_or("open"),
        Status::Building => "building",
        Status::InReview => "in_review",
        Status::ChangesRequested => "changes_requested",
        Status::Approved => "approved",
        Status::Done => "done",
        Status::Dropped => "dropped",
    }
}

/// How much older than the commit that adds it a new plan's `created_at` may
/// be. An older record was made elsewhere and only written here then, as
/// the cutover's conversion wrote missions planned days before.
const NEW_PLAN: TimeDelta = TimeDelta::hours(24);

/// What happened, with what changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Happened {
    /// The item's first implementer, with its status move when the commit
    /// made it `building`.
    Start {
        status: Option<Move>,
    },
    /// Another implementer, after `from`.
    TakeOver {
        from: Thread,
        status: Option<Move>,
    },
    Submit {
        status: Move,
    },
    /// A review file appeared, with its first reviewer.
    Review {
        review: ReviewName,
    },
    /// Another reviewer of `review`, after `from`.
    ReviewTakeOver {
        review: ReviewName,
        from: Thread,
    },
    /// A review's verdict was set, with its item's status move when the
    /// commit made it `approved` or `changes_requested`.
    Verdict {
        review: ReviewName,
        verdict: Verdict,
        status: Option<Move>,
    },
    Merge {
        status: Move,
        trivial: Option<String>,
    },
    Settle {
        finding: FindingName,
        settlement: Settlement,
    },
    /// The mission's lead was set, or changed from `from`.
    Lead {
        from: Option<Thread>,
    },
    /// The mission's record first appeared, as a new plan.
    Plan {
        status: Move,
    },
    /// A merge on the base's first-parent line agreed the mission's plan or,
    /// where `amends`, changed an agreed one.
    PlanMerge {
        amends: bool,
    },
    /// A go-ahead, by the name recorded with it, which only history from
    /// before items and missions were open holds.
    Ready {
        by: Option<String>,
        status: Move,
    },
    Drop {
        status: Move,
        reason: Option<String>,
    },
    Close {
        status: Move,
    },
    /// A record changed in a commit that passed a rule, with no action above
    /// found on it, such as a `set` of a started item or the merge
    /// of one: shown so that no passed rule goes unseen. `merge` when the
    /// commit is an Atelier merge.
    Passed {
        merge: bool,
    },
}

impl Happened {
    /// The action's name, as its line shows it.
    pub fn name(&self) -> &'static str {
        match self {
            Happened::Start { .. } => "start",
            Happened::TakeOver { .. } | Happened::ReviewTakeOver { .. } => "take-over",
            Happened::Submit { .. } => "submit",
            Happened::Review { .. } => "review",
            Happened::Verdict { .. } => "verdict",
            Happened::Merge { .. } => "merge",
            Happened::Settle { .. } => "settle",
            Happened::Lead { .. } => "lead",
            Happened::Plan { .. } => "plan",
            // Not `merge`, by which the gardener counts merged items.
            Happened::PlanMerge { .. } => "plan-merge",
            Happened::Ready { .. } => "ready",
            Happened::Drop { .. } => "drop",
            Happened::Close { .. } => "close",
            Happened::Passed { merge: true } => "merge",
            Happened::Passed { merge: false } => "change",
        }
    }

    /// What changed, in the parts its line shows (docs/design.md, A line).
    pub fn changed(&self) -> Vec<String> {
        let moved = |status: &Option<Move>| status.iter().map(Move::shown).collect::<Vec<_>>();
        match self {
            Happened::Start { status } => moved(status),
            Happened::TakeOver { from, status } => {
                [vec![format!("from {from}")], moved(status)].concat()
            }
            Happened::Submit { status }
            | Happened::Close { status }
            | Happened::Plan { status } => vec![status.shown()],
            Happened::PlanMerge { amends } => amends
                .then(|| "amendment".to_string())
                .into_iter()
                .collect(),
            Happened::Review { review } => vec![review.to_string()],
            Happened::ReviewTakeOver { review, from } => {
                vec![review.to_string(), format!("from {from}")]
            }
            Happened::Verdict {
                review,
                verdict,
                status,
            } => {
                let verdict = match verdict {
                    Verdict::Approve => "approve",
                    Verdict::Changes => "changes",
                };
                [vec![format!("{review} {verdict}")], moved(status)].concat()
            }
            Happened::Merge { status, trivial } => {
                let trivial = trivial.iter().map(|why| format!("trivial: {why}"));
                [status.shown()].into_iter().chain(trivial).collect()
            }
            Happened::Settle {
                finding,
                settlement,
            } => vec![match settlement {
                Settlement::Taken { item, .. } => format!("{finding} taken into {item}"),
                Settlement::Dropped { reason, .. } => format!("{finding} dropped: {reason}"),
            }],
            Happened::Lead { from } => from.iter().map(|from| format!("from {from}")).collect(),
            Happened::Ready { by, status } => {
                let by = by.iter().map(|by| format!("by {by}"));
                [status.shown()].into_iter().chain(by).collect()
            }
            Happened::Drop { status, reason } => {
                [status.shown()].into_iter().chain(reason.clone()).collect()
            }
            Happened::Passed { .. } => Vec::new(),
        }
    }

    /// Its row in the log's table, which orders the actions of one commit.
    fn row(&self) -> u8 {
        match self {
            Happened::Plan { .. } => 0,
            Happened::Start { .. } | Happened::TakeOver { .. } => 1,
            Happened::Submit { .. } => 2,
            Happened::Review { .. } | Happened::ReviewTakeOver { .. } => 3,
            Happened::Verdict { .. } => 4,
            Happened::Merge { .. } => 5,
            Happened::PlanMerge { .. } => 6,
            Happened::Settle { .. } => 7,
            Happened::Lead { .. } => 8,
            Happened::Ready { .. } => 9,
            Happened::Drop { .. } | Happened::Close { .. } => 10,
            Happened::Passed { .. } => 11,
        }
    }

    /// Whether Atelier makes the commit of this action, so that a skip its
    /// commit carries is this action's.
    fn commits(&self) -> bool {
        matches!(
            self,
            Happened::Start { .. }
                | Happened::TakeOver { .. }
                | Happened::Submit { .. }
                | Happened::Verdict { .. }
                | Happened::Merge { .. }
                | Happened::Plan { .. }
                | Happened::PlanMerge { .. }
                | Happened::Ready { .. }
                | Happened::Drop { .. }
                | Happened::Close { .. }
                | Happened::Lead { .. }
                | Happened::Passed { .. }
        )
    }
}

/// A version of a record that does not read, which the walk passed over:
/// the actions across it are found by comparing the versions on each side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassedOver {
    pub time: DateTime<Utc>,
    pub commit: String,
    pub path: String,
    pub id: String,
    pub problem: String,
}

/// What `log` shows: the actions, oldest first, and the versions passed over.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct History {
    pub actions: Vec<Logged>,
    pub passed_over: Vec<PassedOver>,
}

/// The time `text` gives, an RFC 3339 time or a `YYYY-MM-DD` date, which
/// is that day's start in UTC.
pub fn since(text: &str) -> Result<DateTime<Utc>, String> {
    if let Ok(time) = DateTime::parse_from_rfc3339(text) {
        return Ok(time.with_timezone(&Utc));
    }
    NaiveDate::parse_from_str(text, "%Y-%m-%d")
        .ok()
        .filter(|_| text.len() == 10)
        .and_then(|day| day.and_hms_opt(0, 0, 0))
        .map(|start| start.and_utc())
        .ok_or_else(|| {
            format!("`{text}` is not an RFC 3339 time, such as 2026-11-11T09:00:00Z, or a date, such as 2026-11-11")
        })
}

/// The history of actions in `repo`: on the item or mission `of` when it is
/// given, with a mission's plan reviews and its items, and committed at or
/// after `since` when it is given. `None` when no record of `of` is on the
/// base's tip or on a branch named for it.
pub fn history(
    repo: &mut Repo,
    of: Option<&str>,
    since: Option<DateTime<Utc>>,
) -> Result<Option<History>> {
    history_until(repo, of, since, None)
}

/// [`history`], given up with an error once `deadline` has passed: its Git
/// listings are killed then, and no further commit or record is read.
pub fn history_until(
    repo: &mut Repo,
    of: Option<&str>,
    since: Option<DateTime<Utc>>,
    deadline: Option<Instant>,
) -> Result<Option<History>> {
    in_time(deadline)?;
    // Every branch the convention names, local and remote-tracking, as
    // `show` reads them: a mission's lookup by name reads the tip `show`
    // reads, and the walk takes every tip, a backlog branch's included.
    let walked = repo.walked()?;
    let tips: Vec<(String, String)> = (walked.iter())
        .filter_map(|branch| Some((branch.record.clone()?, branch.tip.clone())))
        .collect();
    let base_tip = repo.base_tip.clone();
    let mission = match of {
        None => None,
        Some(id) => {
            validate_id(id).map_err(|message| anyhow::anyhow!(message))?;
            let mission = format!(".atelier/missions/{id}/mission.md");
            let tip = tips.iter().find(|(name, _)| name == id);
            // A proposed mission is only on its branch.
            let mut is_mission = repo.reader().blob(&format!("{base_tip}:{mission}"))?;
            if let (None, Some((_, tip))) = (&is_mission, tip) {
                is_mission = repo.reader().blob(&format!("{tip}:{mission}"))?;
            }
            let mut item = repo
                .reader()
                .blob(&format!("{base_tip}:{}", item_path(id)))?;
            if let (None, Some((_, tip))) = (&item, tip) {
                item = repo.reader().blob(&format!("{tip}:{}", item_path(id)))?;
            }
            match (is_mission, item) {
                (Some(_), _) => Some(id),
                (None, Some(_)) => None,
                (None, None) => return Ok(None),
            }
        }
    };

    let mut revisions = vec![base_tip.clone()];
    for tip in walked.iter().flat_map(|branch| &branch.tips) {
        if !revisions.contains(tip) {
            revisions.push(tip.clone());
        }
    }
    let commits = walk(&repo.root, &revisions, deadline)?;
    let merges = mission_merges(&repo.root, &base_tip, deadline)?;
    let mut walker = Walker {
        repo,
        deadline,
        merges,
        appeared: BTreeSet::new(),
        versions: HashMap::new(),
        pending: HashMap::new(),
        said: BTreeSet::new(),
        history: History::default(),
    };
    for commit in &commits {
        in_time(deadline)?;
        walker.commit(commit)?;
    }
    let mut history = walker.history;
    // Ordered by time; a commit's actions keep the table's order, and commits
    // of one time the walk's, parents first.
    history.actions.sort_by_key(|logged| logged.time);

    let keep: Box<dyn Fn(&str) -> bool> = match (of, mission) {
        (None, _) => Box::new(|_| true),
        (Some(id), None) => Box::new(move |found| found == id),
        (Some(id), Some(mission)) => {
            let ids: BTreeSet<String> = history
                .actions
                .iter()
                .map(|logged| logged.id.clone())
                .collect();
            let mut members = BTreeSet::new();
            for item in ids {
                in_time(deadline)?;
                if in_mission(repo, &base_tip, &walked, &item, mission)? {
                    members.insert(item);
                }
            }
            Box::new(move |found| found == id || members.contains(found))
        }
    };
    let recent = |time: &DateTime<Utc>| since.is_none_or(|since| *time >= since);
    history
        .actions
        .retain(|logged| keep(&logged.id) && recent(&logged.time));
    history
        .passed_over
        .retain(|passed| keep(&passed.id) && recent(&passed.time));
    Ok(Some(history))
}

fn in_time(deadline: Option<Instant>) -> Result<()> {
    if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
        bail!("the history was not read by its deadline");
    }
    Ok(())
}

/// Whether the item `id`'s record names `mission`: on the base's tip,
/// else on its own branch, else, as `show` finds a record the base does not
/// hold, on the first unmerged branch of `walked`, by name, that adds it,
/// such as the mission's or a backlog branch.
fn in_mission(
    repo: &mut Repo,
    base_tip: &str,
    walked: &[Walked],
    id: &str,
    mission: &str,
) -> Result<bool> {
    let path = item_path(id);
    let mut text = repo.reader().blob(&format!("{base_tip}:{path}"))?;
    let own = walked.iter().filter(|b| b.record.as_deref() == Some(id));
    let adding = walked.iter().filter(|b| !b.merged);
    for branch in own.chain(adding) {
        if text.is_some() {
            break;
        }
        text = repo.reader().blob(&format!("{}:{path}", branch.tip))?;
    }
    let item = text.and_then(|text| parse_item(Path::new(&path), &text).ok());
    Ok(item.is_some_and(|item| item.mission.as_deref() == Some(mission)))
}

/// A commit that changed a record, as the walk lists it.
struct Commit {
    hash: String,
    short: String,
    parents: Vec<String>,
    time: DateTime<Utc>,
    /// The item an `Atelier-Merge:` trailer names.
    merged: Option<String>,
    skips: Vec<String>,
    /// The thread an `Atelier-Skip-By:` trailer names.
    skipped_by: Option<Thread>,
    /// Each record file it changed from its parent, with the old and new
    /// blob as Git abbreviates them, `None` where it has none.
    changes: Vec<Change>,
}

struct Change {
    path: String,
    old: Option<String>,
    new: Option<String>,
}

/// Every commit in the history of `revisions` that changed a record,
/// parents before children. `--full-history` follows both sides of every
/// merge: without it Git follows only the side a merge matches on the
/// record paths, and the other side's actions are lost.
fn walk(root: &Path, revisions: &[String], deadline: Option<Instant>) -> Result<Vec<Commit>> {
    let trailer = |key: &str| format!("%(trailers:key={key},valueonly,separator=%x1f)");
    let format = format!(
        "--format=%x01%H%x02%h%x02%P%x02%ct%x02{}%x02{}%x02{}%x03",
        trailer("Atelier-Merge"),
        trailer(TRAILER),
        trailer(BY_TRAILER),
    );
    let mut args = vec![
        "log",
        "-z",
        "--no-color",
        "--no-show-signature",
        "--topo-order",
        "--reverse",
        "--full-history",
        // Each commit's parents are the nearest commits listed, so what one
        // passed over reaches the next on its line of history.
        "--parents",
        "--root",
        "--no-renames",
        "--raw",
        &format,
    ];
    args.extend(revisions.iter().map(String::as_str));
    args.extend(["--", ".atelier/items", ".atelier/missions"]);
    let listed = git::listing_until(root, &args, deadline)?;
    listed.split('\x01').skip(1).map(commit).collect()
}

/// Each merge on `base_tip`'s first-parent line that changed a mission's
/// files against its first parent, with those files. [`walk`] cannot say:
/// it lists no files for a merge, and the parents it gives are the nearest
/// listed commits, which drop a base that had no record yet.
fn mission_merges(
    root: &Path,
    base_tip: &str,
    deadline: Option<Instant>,
) -> Result<HashMap<String, Vec<String>>> {
    let args = [
        "log",
        "-z",
        "--no-color",
        "--no-show-signature",
        "--first-parent",
        "--merges",
        "-m",
        "--no-renames",
        "--name-only",
        "--format=%x01%H%x03",
        base_tip,
        "--",
        ".atelier/missions",
    ];
    let listed = git::listing_until(root, &args, deadline)?;
    let merge = |entry: &str| {
        let (hash, files) = entry
            .split_once('\x03')
            .with_context(|| format!("git log printed a merge this tool cannot read: {entry:?}"))?;
        let files = git::names(files.trim_start_matches(['\0', '\n']));
        Ok((hash.to_string(), files))
    };
    listed.split('\x01').skip(1).map(merge).collect()
}

/// One commit as [`walk`]'s `git log` prints it: its fields, each ended by
/// \x02 and the last by \x03, then for each changed file `:<modes> <old>
/// <new> <status>` and its path, each ended by a NUL.
fn commit(entry: &str) -> Result<Commit> {
    let unreadable = || format!("git log printed a commit this tool cannot read: {entry:?}");
    let (header, raw) = entry.split_once('\x03').with_context(unreadable)?;
    let fields: Vec<&str> = header.split('\x02').collect();
    let [hash, short, parents, time, merged, skips, skipped_by] = fields.as_slice() else {
        bail!(unreadable());
    };
    let seconds: i64 = time.parse().with_context(unreadable)?;
    let values = |text: &str| -> Vec<String> {
        text.split('\x1f')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .collect()
    };
    let mut changes = Vec::new();
    let mut tokens = raw
        .split('\0')
        .map(|token| token.trim_start_matches('\n'))
        .filter(|token| !token.is_empty());
    while let Some(meta) = tokens.next() {
        let path = tokens.next().with_context(unreadable)?;
        let fields: Vec<&str> = meta.trim_start_matches(':').split(' ').collect();
        let [_, _, old, new, _] = fields.as_slice() else {
            bail!(unreadable());
        };
        let blob = |oid: &str| (!oid.bytes().all(|b| b == b'0')).then(|| oid.to_string());
        changes.push(Change {
            path: path.to_string(),
            old: blob(old),
            new: blob(new),
        });
    }
    Ok(Commit {
        hash: hash.to_string(),
        short: short.to_string(),
        parents: parents
            .split(' ')
            .filter(|p| !p.is_empty())
            .map(str::to_string)
            .collect(),
        time: DateTime::from_timestamp(seconds, 0).with_context(unreadable)?,
        merged: values(merged).into_iter().next(),
        skips: values(skips),
        skipped_by: values(skipped_by).into_iter().next().map(Thread),
        changes,
    })
}

/// A record file, by its path.
enum Record {
    Item(String),
    Mission(String),
    /// A review of the item or mission `id`, by its file's stem.
    Review {
        id: String,
        stem: String,
    },
}

impl Record {
    fn of(path: &str) -> Option<Record> {
        let parts: Vec<&str> = path.split('/').collect();
        match parts.as_slice() {
            [".atelier", "items", id, "item.md"] => Some(Record::Item(id.to_string())),
            [".atelier", "missions", id, "mission.md"] => Some(Record::Mission(id.to_string())),
            [".atelier", "items" | "missions", id, "reviews", file] => {
                let stem = file.strip_suffix(".md")?;
                Some(Record::Review {
                    id: id.to_string(),
                    stem: stem.to_string(),
                })
            }
            _ => None,
        }
    }

    fn id(&self) -> &str {
        match self {
            Record::Item(id) | Record::Mission(id) | Record::Review { id, .. } => id,
        }
    }
}

/// A version of a record that reads, with what the actions are found in.
enum Version {
    Item {
        item: Item,
        /// Its status as the record spells it.
        stored: &'static str,
        dropped: Option<String>,
    },
    Mission(MissionDir),
    Review {
        name: ReviewName,
        review: Review,
        findings: Vec<Finding>,
    },
}

/// What a version that does not read stands for on one line of history:
/// its blob, and the last version before it there that reads.
type Pending = HashMap<String, (String, Option<Rc<Version>>)>;

struct Walker<'r, 's> {
    repo: &'r mut Repo<'s>,
    deadline: Option<Instant>,
    /// Each version read, or what is wrong with it, by its path and blob.
    versions: HashMap<(String, String), Result<Rc<Version>, String>>,
    /// After each commit that has one, the records whose version there does
    /// not read, by path. A version's blob says whether it reads; which
    /// version stands for it depends on where in history it is.
    pending: HashMap<String, Pending>,
    /// Each version passed over and said so, by its path and blob.
    said: BTreeSet<(String, String)>,
    /// The base's merges that changed a mission's files, by [`mission_merges`].
    merges: HashMap<String, Vec<String>>,
    /// The missions a version of which has been read, so that only a
    /// record's first appearance is a plan.
    appeared: BTreeSet<String>,
    history: History,
}

impl Walker<'_, '_> {
    fn commit(&mut self, commit: &Commit) -> Result<()> {
        let mut found: Vec<(String, Option<Thread>, Happened)> = Vec::new();
        // The status moves the item actions leave to a verdict.
        let mut verdicts: Vec<(String, Move)> = Vec::new();
        // What its parents passed over, the first parent's first.
        let mut pending = Pending::new();
        for parent in &commit.parents {
            for (path, entry) in self.pending.get(parent).into_iter().flatten() {
                pending.entry(path.clone()).or_insert_with(|| entry.clone());
            }
        }
        if commit.parents.len() > 1 {
            // An Atelier merge gives its item's merge line alone; what its
            // branch brought is found on that side. Another merge gives
            // nothing.
            if let Some(id) = &commit.merged {
                found.extend(self.merge(commit, id)?);
            }
        } else {
            let mut items = Vec::new();
            let mut others = Vec::new();
            for change in &commit.changes {
                let Some(record) = Record::of(&change.path) else {
                    continue;
                };
                let path = &change.path;
                let parent = commit.parents.first();
                let old = match (&change.old, parent) {
                    (Some(blob), Some(parent)) => match self.read(path, blob, parent)? {
                        Ok(version) => Some(version),
                        // Passed over where it was committed, it stands for
                        // the version before it there.
                        Err(problem) => match pending.get(path) {
                            Some((bad, before)) if bad == blob => before.clone(),
                            _ => {
                                self.passed_over(commit, path, blob, record.id(), problem);
                                None
                            }
                        },
                    },
                    _ => None,
                };
                let Some(blob) = &change.new else {
                    pending.remove(path);
                    continue;
                };
                let new = match self.read(path, blob, &commit.hash)? {
                    Ok(version) => version,
                    Err(problem) => {
                        self.passed_over(commit, path, blob, record.id(), problem);
                        pending.insert(path.clone(), (blob.clone(), old));
                        continue;
                    }
                };
                pending.remove(path);
                match record {
                    Record::Item(id) => items.push((id, old, new)),
                    _ => others.push((record, old, new)),
                }
            }
            for (id, old, new) in &items {
                let (actions, verdict) = item_actions(old.as_deref(), new);
                found.extend(
                    actions
                        .into_iter()
                        .map(|(thread, action)| (id.clone(), thread, action)),
                );
                if let Some(status) = verdict {
                    verdicts.push((id.clone(), status));
                }
            }
            for (record, old, new) in &others {
                let id = record.id().to_string();
                let actions = match record {
                    Record::Mission(_) => {
                        let mut actions = mission_actions(old.as_deref(), new);
                        if self.appeared.insert(id.clone()) && old.is_none() {
                            actions.extend(planned(new, commit.time));
                        }
                        actions
                    }
                    _ => review_actions(old.as_deref(), new),
                };
                found.extend(
                    actions
                        .into_iter()
                        .map(|(thread, action)| (id.clone(), thread, action)),
                );
            }
        }
        found.extend(self.plan_merges(commit)?);
        // Each goes to its item's last verdict, of the latest round, as
        // review files come in path order.
        for (id, moved) in verdicts {
            let last = found
                .iter_mut()
                .rev()
                .find_map(|(of, _, action)| match action {
                    Happened::Verdict { status, .. } if *of == id => Some(status),
                    _ => None,
                });
            if let Some(status) = last {
                *status = Some(moved);
            }
        }
        if !pending.is_empty() {
            self.pending.insert(commit.hash.clone(), pending);
        }
        // A record the commit changed with no action of its own, beside one
        // that has, still shows the rules the commit passed: a `set` of a
        // started item beside a `drop` of another.
        if !commit.skips.is_empty() {
            let merge = commit.parents.len() > 1;
            for id in self.records_of(commit)? {
                if !found.iter().any(|(of, _, _)| *of == id) {
                    found.push((id, None, Happened::Passed { merge }));
                }
            }
        }
        found.sort_by_key(|(_, _, action)| action.row());
        // Each record shows the commit's skips once: on its actions Atelier
        // commits, else on all of its actions. A verdict's carries them and
        // its review line beside it does not; a settlement carries them
        // beside an item that the same commit created.
        let carries: BTreeSet<String> = (found.iter())
            .filter(|(_, _, action)| action.commits())
            .map(|(id, _, _)| id.clone())
            .collect();
        for (id, thread, action) in found {
            let skips = match !carries.contains(&id) || action.commits() {
                true => commit.skips.clone(),
                false => Vec::new(),
            };
            let skipped_by = commit.skipped_by.clone().filter(|_| !skips.is_empty());
            // The thread that ran the command which made the commit, where
            // the commit names it: a lead's merge of an implementer's item
            // is the lead's.
            let thread = match (action.commits(), &commit.skipped_by) {
                (true, Some(by)) => Some(by.clone()),
                _ => thread,
            };
            self.history.actions.push(Logged {
                time: commit.time,
                commit: commit.short.clone(),
                id,
                thread,
                action,
                skips,
                skipped_by,
            });
        }
        Ok(())
    }

    /// The merge line of the Atelier merge `commit` of the item `id`:
    /// its record there against its record on the merged branch.
    fn merge(
        &mut self,
        commit: &Commit,
        id: &str,
    ) -> Result<Option<(String, Option<Thread>, Happened)>> {
        let path = item_path(id);
        let at = |repo: &mut Repo, commit: &str| -> Result<Option<(Item, &'static str)>> {
            let Some(text) = repo.reader().blob(&format!("{commit}:{path}"))? else {
                return Ok(None);
            };
            let item = parse_item(Path::new(&path), &text).ok();
            Ok(item.map(|item| {
                let stored = spelled(item.status, &text);
                (item, stored)
            }))
        };
        let Some((new, _)) = at(self.repo, &commit.hash)? else {
            return Ok(None);
        };
        let from = at(self.repo, &commit.parents[1])?.map(|(_, stored)| stored);
        // The merge of its own branch brings a started item to the base,
        // where it reads as done; a merge before that wrote `done`.
        let merged = new.status == Status::Done || STARTED.contains(&new.status);
        if !merged || from == Some("done") {
            return Ok(None);
        }
        let action = Happened::Merge {
            status: Move { from, to: "done" },
            trivial: new.trivial.clone(),
        };
        Ok(Some((
            id.to_string(),
            new.implementers.holder().cloned(),
            action,
        )))
    }

    /// The plan merges of `commit`, where it is a merge on the base's
    /// first-parent line: each mission it brought to the base open, and each
    /// agreed one whose plan it changed, compared with its first parent as
    /// `merge` judges a plan change.
    fn plan_merges(&mut self, commit: &Commit) -> Result<Vec<(String, Option<Thread>, Happened)>> {
        let Some(files) = self.merges.remove(&commit.hash) else {
            return Ok(Vec::new());
        };
        let ids: BTreeSet<&str> = (files.iter())
            .filter_map(|file| file.strip_prefix(".atelier/missions/")?.split('/').next())
            .collect();
        let mut merged = Vec::new();
        for id in ids {
            let Some(Some(new)) = self.mission_at(&commit.hash, id)? else {
                continue;
            };
            let before = self.mission_at(&format!("{}^", commit.hash), id)?;
            let proposed = match &before {
                None => true,
                Some(old) => {
                    (old.as_ref()).is_some_and(|old| old.mission.status == MissionStatus::Draft)
                }
            };
            let happened = match proposed {
                true => new.mission.status == MissionStatus::Open,
                // An earlier version that does not read has only its files
                // to compare.
                false => {
                    files.iter().any(|file| is_plan_file(id, file))
                        || (before.flatten())
                            .is_some_and(|old| !changed_sections(&old, &new).is_empty())
                }
            };
            if happened {
                let action = Happened::PlanMerge { amends: !proposed };
                let planner = new.mission.planners.holder().cloned();
                merged.push((id.to_string(), planner, action));
            }
        }
        Ok(merged)
    }

    /// The mission `id` as `commit` holds it: `None` when it has none, and
    /// `Some(None)` when its version there does not read.
    fn mission_at(&mut self, commit: &str, id: &str) -> Result<Option<Option<MissionDir>>> {
        let path = mission_path(id);
        let text = match self.repo.reader().blob(&format!("{commit}:{path}")) {
            Ok(Some(text)) => text,
            Ok(None) => return Ok(None),
            Err(ReadError::Record(_)) => return Ok(Some(None)),
            Err(error) => return Err(error.into()),
        };
        Ok(Some(mission_dir(&path, &text).ok()))
    }

    /// The IDs of the records `commit` changed, each once: for an Atelier
    /// merge, the item or mission it names, else those its merge changed
    /// against the base it merged into, as a backlog branch's merge does.
    fn records_of(&mut self, commit: &Commit) -> Result<Vec<String>> {
        let paths: Vec<String> = match commit.parents.as_slice() {
            [base, _, ..] => {
                if let Some(id) = commit.merged.as_ref().filter(|id| validate_id(id).is_ok()) {
                    return Ok(vec![id.clone()]);
                }
                let listed = git::listing_until(
                    &self.repo.root,
                    &[
                        "diff",
                        "--name-only",
                        "-z",
                        "--no-renames",
                        base,
                        &commit.hash,
                        "--",
                        ".atelier/items",
                        ".atelier/missions",
                    ],
                    self.deadline,
                )?;
                git::names(&listed)
            }
            _ => (commit.changes.iter())
                .map(|change| change.path.clone())
                .collect(),
        };
        let mut ids: Vec<String> = Vec::new();
        for path in paths {
            if let Some(record) = Record::of(&path) {
                let id = record.id().to_string();
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
        Ok(ids)
    }

    /// The version of the record at `path` whose blob is `blob`, read from
    /// `at`, a commit that holds it, or what is wrong with it.
    fn read(&mut self, path: &str, blob: &str, at: &str) -> Result<Result<Rc<Version>, String>> {
        if let Some(read) = self.versions.get(&key(path, blob)) {
            return Ok(read.clone());
        }
        let read = match self.repo.reader().blob(&format!("{at}:{path}")) {
            Ok(Some(text)) => version(path, &text).map(Rc::new),
            Ok(None) => Err(format!("{at} has no {path}")),
            // The file's content, such as bytes that are not UTF-8, and not
            // Git, is at fault.
            Err(ReadError::Record(problem)) => Err(problem.to_string()),
            Err(error) => return Err(error.into()),
        };
        self.versions.insert(key(path, blob), read.clone());
        Ok(read)
    }

    /// Says once that the version of `path` whose blob is `blob` does not
    /// read, as of `commit`, where the walk first meets it.
    fn passed_over(&mut self, commit: &Commit, path: &str, blob: &str, id: &str, problem: String) {
        if self.said.insert(key(path, blob)) {
            self.history.passed_over.push(PassedOver {
                time: commit.time,
                commit: commit.short.clone(),
                path: path.to_string(),
                id: id.to_string(),
                problem,
            });
        }
    }
}

fn key(path: &str, blob: &str) -> (String, String) {
    (path.to_string(), blob.to_string())
}

/// The record at `path` read from `text`, or what is wrong with it.
fn version(path: &str, text: &str) -> Result<Version, String> {
    let file = Path::new(path);
    let record = Record::of(path).ok_or("not a record")?;
    match record {
        Record::Item(_) => {
            let item = parse_item(file, text).map_err(|p| p.to_string())?;
            let dir = ItemDir {
                path: file.to_path_buf(),
                item,
                reviews: Vec::new(),
            };
            let dropped = dir.dropped().map(str::to_string);
            Ok(Version::Item {
                stored: spelled(dir.item.status, text),
                item: dir.item,
                dropped,
            })
        }
        Record::Mission(_) => mission_dir(path, text).map(Version::Mission),
        Record::Review { stem, .. } => {
            let name: ReviewName = stem.parse()?;
            let review = parse_review(file, text).map_err(|p| p.to_string())?;
            let file = ReviewFile {
                path: file.to_path_buf(),
                name: name.clone(),
                review,
            };
            let findings = file.findings().map_err(|p| p.to_string())?;
            Ok(Version::Review {
                name,
                review: file.review,
                findings,
            })
        }
    }
}

/// The mission record at `path` read from `text`, without its reviews.
fn mission_dir(path: &str, text: &str) -> Result<MissionDir, String> {
    let file = Path::new(path);
    Ok(MissionDir {
        path: file.to_path_buf(),
        mission: parse_mission(file, text).map_err(|p| p.to_string())?,
        reviews: Vec::new(),
    })
}

type Actions = Vec<(Option<Thread>, Happened)>;

/// The threads `new` gained after `old`'s, each with the one before it.
fn gained<'a>(old: Option<&Threads>, new: &'a Threads) -> Vec<(Option<&'a Thread>, &'a Thread)> {
    let old = old.map(|threads| threads.0.as_slice()).unwrap_or_default();
    let same = old
        .iter()
        .zip(&new.0)
        .take_while(|(old, new)| old == new)
        .count();
    (same..new.0.len())
        .map(|at| (at.checked_sub(1).map(|before| &new.0[before]), &new.0[at]))
        .collect()
}

/// The actions across an item's versions `old` and `new`, and the status move
/// they leave to a verdict in the same commit.
fn item_actions(old: Option<&Version>, new: &Version) -> (Actions, Option<Move>) {
    let Version::Item {
        item: new,
        stored: to,
        dropped,
    } = new
    else {
        return (Vec::new(), None);
    };
    let (old, from) = match old {
        Some(Version::Item { item, stored, .. }) => (Some(item), Some(*stored)),
        _ => (None, None),
    };
    let moved = (from != Some(*to)).then_some(Move { from, to: *to });
    let holder = new.implementers.holder().cloned();
    let mut actions = Vec::new();
    let mut building = moved.filter(|_| new.status == Status::Building);
    for (before, thread) in gained(old.map(|old| &old.implementers), &new.implementers) {
        let status = building.take();
        let action = match before {
            None => Happened::Start { status },
            Some(before) => Happened::TakeOver {
                from: before.clone(),
                status,
            },
        };
        actions.push((Some(thread.clone()), action));
    }
    let mut verdict = None;
    if let Some(status) = moved {
        match new.status {
            Status::InReview => actions.push((holder.clone(), Happened::Submit { status })),
            Status::Open if *to == "ready" => {
                let by = new.ready.as_ref().map(|ready| ready.by.clone());
                actions.push((None, Happened::Ready { by, status }));
            }
            Status::Dropped => {
                let reason = dropped.clone();
                actions.push((None, Happened::Drop { status, reason }));
            }
            Status::Done => actions.push((None, Happened::Close { status })),
            Status::Approved | Status::ChangesRequested => verdict = Some(status),
            Status::Open | Status::Building => {}
        }
    }
    (actions, verdict)
}

/// The actions across a mission's versions `old` and `new`.
fn mission_actions(old: Option<&Version>, new: &Version) -> Actions {
    let Version::Mission(dir) = new else {
        return Vec::new();
    };
    let new = &dir.mission;
    let old = match old {
        Some(Version::Mission(old)) => Some(&old.mission),
        _ => None,
    };
    let mut actions = Vec::new();
    let before = old.and_then(|old| old.lead.clone());
    if new.lead.is_some() && new.lead != before {
        actions.push((new.lead.clone(), Happened::Lead { from: before }));
    }
    let from = old.map(|old| old.status.name());
    let to = new.status.name();
    if from != Some(to) {
        let status = Move { from, to };
        match new.status {
            // Made ready, in a record from when a mission had a go-ahead;
            // an old draft written open by `submit` is no action of its own.
            MissionStatus::Open => {
                if let Some(ready) = new.ready.as_ref().filter(|_| from.is_some()) {
                    let by = Some(ready.by.clone());
                    actions.push((None, Happened::Ready { by, status }));
                }
            }
            MissionStatus::Dropped => {
                let reason = dir.dropped().map(str::to_string);
                actions.push((None, Happened::Drop { status, reason }));
            }
            MissionStatus::Done => actions.push((None, Happened::Close { status })),
            MissionStatus::Draft => {}
        }
    }
    actions
}

/// The plan that `new`, the first version of a mission's record, committed
/// at `time`, is: open, with no lead or Result, and created no more than
/// [`NEW_PLAN`] before. A record converted from before the cutover is none.
fn planned(new: &Version, time: DateTime<Utc>) -> Option<(Option<Thread>, Happened)> {
    let Version::Mission(dir) = new else {
        return None;
    };
    let mission = &dir.mission;
    let is_new = mission.status == MissionStatus::Open
        && mission.lead.is_none()
        && !dir.result_written()
        && time - mission.created_at.with_timezone(&Utc) <= NEW_PLAN;
    let status = Move {
        from: None,
        to: mission.status.name(),
    };
    is_new.then(|| {
        (
            mission.planners.0.first().cloned(),
            Happened::Plan { status },
        )
    })
}

/// The actions across a review's versions `old` and `new`. A verdict's status
/// move is its item's, added by the caller.
fn review_actions(old: Option<&Version>, new: &Version) -> Actions {
    let Version::Review {
        name,
        review: new,
        findings,
    } = new
    else {
        return Vec::new();
    };
    let (old, old_findings) = match old {
        Some(Version::Review {
            review, findings, ..
        }) => (Some(review), findings.as_slice()),
        _ => (None, &[][..]),
    };
    let mut actions = Vec::new();
    for (before, thread) in gained(old.map(|old| &old.reviewers), &new.reviewers) {
        let action = match before {
            None => Happened::Review {
                review: name.clone(),
            },
            Some(before) => Happened::ReviewTakeOver {
                review: name.clone(),
                from: before.clone(),
            },
        };
        actions.push((Some(thread.clone()), action));
    }
    if let Some(verdict) = new
        .verdict
        .filter(|_| new.verdict != old.and_then(|old| old.verdict))
    {
        let action = Happened::Verdict {
            review: name.clone(),
            verdict,
            status: None,
        };
        actions.push((new.reviewers.holder().cloned(), action));
    }
    for finding in findings {
        let Some(settlement) = &finding.settlement else {
            continue;
        };
        let before = old_findings.iter().find(|old| old.name == finding.name);
        if before.is_some_and(|before| before.settlement.is_some()) {
            continue;
        }
        let by = match settlement {
            Settlement::Taken { by, .. } | Settlement::Dropped { by, .. } => by.clone(),
        };
        let action = Happened::Settle {
            finding: finding.name.clone(),
            settlement: settlement.clone(),
        };
        actions.push((Some(by), action));
    }
    actions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn since_takes_an_rfc_3339_time_or_a_date_in_utc() {
        assert_eq!(
            since("2026-11-11T09:00:00+02:00").unwrap().to_rfc3339(),
            "2026-11-11T07:00:00+00:00"
        );
        assert_eq!(
            since("2026-11-11").unwrap().to_rfc3339(),
            "2026-11-11T00:00:00+00:00"
        );
        for bad in ["2026-11-1", "yesterday", "2026-13-01", "2026-11-11 09:00"] {
            assert!(since(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn a_commit_reads_with_its_fields_and_changes() {
        let entry =
            "abc123\x02abc\x02p1 p2\x0217944170\x02demo-ab12\x02rule: why\x1fother: why\x02t1\x03\0\n\
                     :100644 100644 1111111 0000000 D\0.atelier/items/demo-ab12/item.md\0\
                     :000000 100644 0000000 2222222 A\0.atelier/missions/demo-cd34/mission.md\0";
        let commit = commit(entry).unwrap();
        assert_eq!(commit.hash, "abc123");
        assert_eq!(commit.short, "abc");
        assert_eq!(commit.parents, ["p1", "p2"]);
        assert_eq!(commit.merged.as_deref(), Some("demo-ab12"));
        assert_eq!(commit.skips, ["rule: why", "other: why"]);
        assert_eq!(commit.skipped_by, Some(Thread("t1".into())));
        assert_eq!(commit.changes.len(), 2);
        assert_eq!(commit.changes[0].old.as_deref(), Some("1111111"));
        assert_eq!(commit.changes[0].new, None);
        assert_eq!(commit.changes[1].old, None);
        assert_eq!(
            commit.changes[1].path,
            ".atelier/missions/demo-cd34/mission.md"
        );
    }

    #[test]
    fn gained_threads_each_follow_the_one_before() {
        let threads =
            |names: &[&str]| Threads(names.iter().map(|n| Thread(n.to_string())).collect());
        let names = |found: Vec<(Option<&Thread>, &Thread)>| -> Vec<(Option<String>, String)> {
            found
                .into_iter()
                .map(|(before, thread)| (before.map(|t| t.0.clone()), thread.0.clone()))
                .collect()
        };
        let new = threads(&["a", "b"]);
        assert_eq!(
            names(gained(None, &new)),
            [(None, "a".into()), (Some("a".into()), "b".into())]
        );
        assert_eq!(
            names(gained(Some(&threads(&["a"])), &new)),
            [(Some("a".into()), "b".into())]
        );
        assert!(gained(Some(&new), &new).is_empty());
    }
}
