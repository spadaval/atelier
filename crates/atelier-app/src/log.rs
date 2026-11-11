//! `log`, the history of acts (docs/design.md, The log), as data. Nothing
//! stores it: one `git log` lists every commit that changed a record, over
//! the base's tip and the tips of started items' branches, with each
//! record's old and new version; the repository's one `git cat-file`
//! process reads those versions; and an act is found by comparing a
//! record's two versions across one commit.

use crate::repo::item_path;
use crate::{git, Repo};
use anyhow::{bail, Context, Result};
use atelier_core::{
    Decision, Finding, FindingName, Item, Mission, MissionStatus, Outcome, Review, ReviewName,
    Settlement, Status, Thread, Threads, Verdict,
};
use atelier_records::{
    parse_item, parse_mission, parse_review, validate_id, ItemDir, MissionDir, ReadError,
    ReviewFile,
};
use chrono::{DateTime, NaiveDate, Utc};
use std::collections::{BTreeSet, HashMap};
use std::path::Path;
use std::rc::Rc;

/// One act, found in one commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Logged {
    /// The commit's committer time.
    pub time: DateTime<Utc>,
    /// The commit, as Git abbreviates it.
    pub commit: String,
    /// The item or mission the act is on.
    pub id: String,
    /// The thread the act is shown with, where the log's table gives one.
    pub thread: Option<Thread>,
    pub act: Happened,
    /// Each rule the act's commit carries an `Atelier-Skip` trailer for, as
    /// `<rule>: <reason>`.
    pub skips: Vec<String>,
}

/// A status a record moved from and to in one commit; `from` is `None`
/// when the commit added the record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Option<Status>,
    pub to: Status,
}

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
    Escalate {
        question: String,
        finding: Option<FindingName>,
    },
    Decide {
        question: String,
        answer: String,
        outcome: Option<Outcome>,
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
    /// Made ready, by the name given with `--by`.
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
}

impl Happened {
    /// The act's name, as its line shows it.
    pub fn name(&self) -> &'static str {
        match self {
            Happened::Start { .. } => "start",
            Happened::TakeOver { .. } | Happened::ReviewTakeOver { .. } => "take-over",
            Happened::Submit { .. } => "submit",
            Happened::Review { .. } => "review",
            Happened::Verdict { .. } => "verdict",
            Happened::Escalate { .. } => "escalate",
            Happened::Decide { .. } => "decide",
            Happened::Merge { .. } => "merge",
            Happened::Settle { .. } => "settle",
            Happened::Lead { .. } => "lead",
            Happened::Ready { .. } => "ready",
            Happened::Drop { .. } => "drop",
            Happened::Close { .. } => "close",
        }
    }

    /// Its row in the log's table, which orders the acts of one commit.
    fn row(&self) -> u8 {
        match self {
            Happened::Start { .. } | Happened::TakeOver { .. } => 0,
            Happened::Submit { .. } => 1,
            Happened::Review { .. } | Happened::ReviewTakeOver { .. } => 2,
            Happened::Verdict { .. } => 3,
            Happened::Escalate { .. } => 4,
            Happened::Decide { .. } => 5,
            Happened::Merge { .. } => 6,
            Happened::Settle { .. } => 7,
            Happened::Lead { .. } => 8,
            Happened::Ready { .. } => 9,
            Happened::Drop { .. } | Happened::Close { .. } => 10,
        }
    }

    /// Whether Atelier makes the commit of this act, so that a skip its
    /// commit carries is this act's.
    fn commits(&self) -> bool {
        matches!(
            self,
            Happened::Start { .. }
                | Happened::TakeOver { .. }
                | Happened::Submit { .. }
                | Happened::Verdict { .. }
                | Happened::Merge { .. }
                | Happened::Ready { .. }
                | Happened::Drop { .. }
                | Happened::Close { .. }
                | Happened::Lead { .. }
        )
    }
}

/// A version of a record that does not read, which the walk passed over:
/// the acts across it are found by comparing the versions on each side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassedOver {
    pub time: DateTime<Utc>,
    pub commit: String,
    pub path: String,
    pub id: String,
    pub problem: String,
}

/// What `log` shows: the acts, oldest first, and the versions passed over.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct History {
    pub acts: Vec<Logged>,
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

/// The history of acts in `repo`: on the item or mission `of` when it is
/// given, with a mission's plan reviews and its items, and committed at or
/// after `since` when it is given. `None` when no record of `of` is on the
/// base's tip or on a branch named for it.
pub fn history(
    repo: &mut Repo,
    of: Option<&str>,
    since: Option<DateTime<Utc>>,
) -> Result<Option<History>> {
    let tips: Vec<(String, String)> = git::branches(&repo.root)?
        .into_iter()
        .filter(|(name, _)| validate_id(name).is_ok())
        .collect();
    let base_tip = repo.base_tip.clone();
    let mission = match of {
        None => None,
        Some(id) => {
            validate_id(id).map_err(|message| anyhow::anyhow!(message))?;
            let mission = format!(".atelier/missions/{id}/mission.md");
            let is_mission = repo.reader().blob(&format!("{base_tip}:{mission}"))?;
            let tip = tips.iter().find(|(name, _)| name == id);
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
    revisions.extend(tips.iter().map(|(_, tip)| tip.clone()));
    let commits = walk(&repo.root, &revisions)?;
    let mut walker = Walker {
        repo,
        versions: HashMap::new(),
        pending: HashMap::new(),
        said: BTreeSet::new(),
        history: History::default(),
    };
    for commit in &commits {
        walker.commit(commit)?;
    }
    let mut history = walker.history;
    // Ordered by time; a commit's acts keep the table's order, and commits
    // of one time the walk's, parents first.
    history.acts.sort_by_key(|logged| logged.time);

    let keep: Box<dyn Fn(&str) -> bool> = match (of, mission) {
        (None, _) => Box::new(|_| true),
        (Some(id), None) => Box::new(move |found| found == id),
        (Some(id), Some(mission)) => {
            let ids: BTreeSet<String> = history
                .acts
                .iter()
                .map(|logged| logged.id.clone())
                .collect();
            let mut members = BTreeSet::new();
            for item in ids {
                if in_mission(repo, &base_tip, &tips, &item, mission)? {
                    members.insert(item);
                }
            }
            Box::new(move |found| found == id || members.contains(found))
        }
    };
    let recent = |time: &DateTime<Utc>| since.is_none_or(|since| *time >= since);
    history
        .acts
        .retain(|logged| keep(&logged.id) && recent(&logged.time));
    history
        .passed_over
        .retain(|passed| keep(&passed.id) && recent(&passed.time));
    Ok(Some(history))
}

/// Whether the item `id`'s record, on the base's tip or else on its
/// branch, names `mission`.
fn in_mission(
    repo: &mut Repo,
    base_tip: &str,
    tips: &[(String, String)],
    id: &str,
    mission: &str,
) -> Result<bool> {
    let path = item_path(id);
    let mut text = repo.reader().blob(&format!("{base_tip}:{path}"))?;
    if text.is_none() {
        if let Some((_, tip)) = tips.iter().find(|(name, _)| name == id) {
            text = repo.reader().blob(&format!("{tip}:{path}"))?;
        }
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
/// record paths, and the other side's acts are lost.
fn walk(root: &Path, revisions: &[String]) -> Result<Vec<Commit>> {
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
        "--format=%x01%H%x02%h%x02%P%x02%ct%x02%(trailers:key=Atelier-Merge,valueonly,separator=%x1f)%x02%(trailers:key=Atelier-Skip,valueonly,separator=%x1f)%x03",
    ];
    args.extend(revisions.iter().map(String::as_str));
    args.extend(["--", ".atelier/items", ".atelier/missions"]);
    let listed = git::listing(root, &args)?;
    listed.split('\x01').skip(1).map(commit).collect()
}

/// One commit as [`walk`]'s `git log` prints it: its fields, each ended by
/// \x02 and the last by \x03, then for each changed file `:<modes> <old>
/// <new> <status>` and its path, each ended by a NUL.
fn commit(entry: &str) -> Result<Commit> {
    let unreadable = || format!("git log printed a commit this tool cannot read: {entry:?}");
    let (header, raw) = entry.split_once('\x03').with_context(unreadable)?;
    let fields: Vec<&str> = header.split('\x02').collect();
    let [hash, short, parents, time, merged, skips] = fields.as_slice() else {
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

/// A version of a record that reads, with what the acts are found in.
enum Version {
    Item {
        item: Item,
        decisions: Vec<Decision>,
        dropped: Option<String>,
    },
    Mission {
        mission: Mission,
        dropped: Option<String>,
    },
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
    /// Each version read, or what is wrong with it, by its path and blob.
    versions: HashMap<(String, String), Result<Rc<Version>, String>>,
    /// After each commit that has one, the records whose version there does
    /// not read, by path. A version's blob says whether it reads; which
    /// version stands for it depends on where in history it is.
    pending: HashMap<String, Pending>,
    /// Each version passed over and said so, by its path and blob.
    said: BTreeSet<(String, String)>,
    history: History,
}

impl Walker<'_, '_> {
    fn commit(&mut self, commit: &Commit) -> Result<()> {
        let mut found: Vec<(String, Option<Thread>, Happened)> = Vec::new();
        // The status moves the item acts leave to a verdict.
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
                let (acts, verdict) = item_acts(old.as_deref(), new);
                found.extend(
                    acts.into_iter()
                        .map(|(thread, act)| (id.clone(), thread, act)),
                );
                if let Some(status) = verdict {
                    verdicts.push((id.clone(), status));
                }
            }
            for (record, old, new) in &others {
                let id = record.id().to_string();
                let acts = match record {
                    Record::Mission(_) => mission_acts(old.as_deref(), new),
                    _ => review_acts(old.as_deref(), new),
                };
                found.extend(
                    acts.into_iter()
                        .map(|(thread, act)| (id.clone(), thread, act)),
                );
            }
        }
        // Each goes to its item's last verdict, of the latest round, as
        // review files come in path order.
        for (id, moved) in verdicts {
            let last = found.iter_mut().rev().find_map(|(of, _, act)| match act {
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
        found.sort_by_key(|(_, _, act)| act.row());
        let carries = found.iter().any(|(_, _, act)| act.commits());
        for (id, thread, act) in found {
            let skips = match !carries || act.commits() {
                true => commit.skips.clone(),
                false => Vec::new(),
            };
            self.history.acts.push(Logged {
                time: commit.time,
                commit: commit.short.clone(),
                id,
                thread,
                act,
                skips,
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
        let at = |repo: &mut Repo, commit: &str| -> Result<Option<Item>> {
            let text = repo.reader().blob(&format!("{commit}:{path}"))?;
            Ok(text.and_then(|text| parse_item(Path::new(&path), &text).ok()))
        };
        let Some(new) = at(self.repo, &commit.hash)? else {
            return Ok(None);
        };
        let old = at(self.repo, &commit.parents[1])?;
        let from = old.map(|old| old.status);
        if new.status != Status::Done || from == Some(Status::Done) {
            return Ok(None);
        }
        let act = Happened::Merge {
            status: Move {
                from,
                to: Status::Done,
            },
            trivial: new.trivial.clone(),
        };
        Ok(Some((
            id.to_string(),
            new.implementers.holder().cloned(),
            act,
        )))
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
            let decisions = dir.decisions().map_err(|p| p.to_string())?;
            let dropped = dir.dropped().map(str::to_string);
            Ok(Version::Item {
                item: dir.item,
                decisions,
                dropped,
            })
        }
        Record::Mission(_) => {
            let mission = parse_mission(file, text).map_err(|p| p.to_string())?;
            let dir = MissionDir {
                path: file.to_path_buf(),
                mission,
                reviews: Vec::new(),
            };
            let dropped = dir.dropped().map(str::to_string);
            Ok(Version::Mission {
                mission: dir.mission,
                dropped,
            })
        }
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

type Acts = Vec<(Option<Thread>, Happened)>;

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

/// The acts across an item's versions `old` and `new`, and the status move
/// they leave to a verdict in the same commit.
fn item_acts(old: Option<&Version>, new: &Version) -> (Acts, Option<Move>) {
    let Version::Item {
        item: new,
        decisions,
        dropped,
    } = new
    else {
        return (Vec::new(), None);
    };
    let (old, old_decisions) = match old {
        Some(Version::Item {
            item, decisions, ..
        }) => (Some(item), decisions.as_slice()),
        _ => (None, &[][..]),
    };
    let from = old.map(|old| old.status);
    let moved = (from != Some(new.status)).then_some(Move {
        from,
        to: new.status,
    });
    let holder = new.implementers.holder().cloned();
    let mut acts = Vec::new();
    let mut building = moved.filter(|status| status.to == Status::Building);
    for (before, thread) in gained(old.map(|old| &old.implementers), &new.implementers) {
        let status = building.take();
        let act = match before {
            None => Happened::Start { status },
            Some(before) => Happened::TakeOver {
                from: before.clone(),
                status,
            },
        };
        acts.push((Some(thread.clone()), act));
    }
    for (at, decision) in decisions.iter().enumerate() {
        let asked = old_decisions.get(at);
        if asked.is_none() {
            let act = Happened::Escalate {
                question: decision.question.clone(),
                finding: decision.finding.clone(),
            };
            acts.push((holder.clone(), act));
        }
        let answered = asked.is_some_and(|asked| asked.answer.is_some());
        if let (Some(answer), false) = (&decision.answer, answered) {
            let act = Happened::Decide {
                question: decision.question.clone(),
                answer: answer.text.clone(),
                outcome: decision.outcome,
            };
            acts.push((Some(answer.by.clone()), act));
        }
    }
    let mut verdict = None;
    if let Some(status) = moved {
        match status.to {
            Status::InReview => acts.push((holder.clone(), Happened::Submit { status })),
            Status::Ready => {
                let by = new.ready.as_ref().map(|ready| ready.by.clone());
                acts.push((None, Happened::Ready { by, status }));
            }
            Status::Dropped => {
                let reason = dropped.clone();
                acts.push((None, Happened::Drop { status, reason }));
            }
            Status::Done => acts.push((None, Happened::Close { status })),
            Status::Approved | Status::ChangesRequested => verdict = Some(status),
            Status::Draft | Status::Building => {}
        }
    }
    (acts, verdict)
}

/// The acts across a mission's versions `old` and `new`.
fn mission_acts(old: Option<&Version>, new: &Version) -> Acts {
    let Version::Mission {
        mission: new,
        dropped,
    } = new
    else {
        return Vec::new();
    };
    let old = match old {
        Some(Version::Mission { mission, .. }) => Some(mission),
        _ => None,
    };
    let mut acts = Vec::new();
    let before = old.and_then(|old| old.lead.clone());
    if new.lead.is_some() && new.lead != before {
        acts.push((new.lead.clone(), Happened::Lead { from: before }));
    }
    let from = old.map(|old| old.status.status());
    let to = new.status.status();
    if from != Some(to) {
        let status = Move { from, to };
        match new.status {
            MissionStatus::Ready => {
                let by = new.ready.as_ref().map(|ready| ready.by.clone());
                acts.push((None, Happened::Ready { by, status }));
            }
            MissionStatus::Dropped => {
                let reason = dropped.clone();
                acts.push((None, Happened::Drop { status, reason }));
            }
            MissionStatus::Done => acts.push((None, Happened::Close { status })),
            MissionStatus::Draft => {}
        }
    }
    acts
}

/// The acts across a review's versions `old` and `new`. A verdict's status
/// move is its item's, added by the caller.
fn review_acts(old: Option<&Version>, new: &Version) -> Acts {
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
    let mut acts = Vec::new();
    for (before, thread) in gained(old.map(|old| &old.reviewers), &new.reviewers) {
        let act = match before {
            None => Happened::Review {
                review: name.clone(),
            },
            Some(before) => Happened::ReviewTakeOver {
                review: name.clone(),
                from: before.clone(),
            },
        };
        acts.push((Some(thread.clone()), act));
    }
    if let Some(verdict) = new
        .verdict
        .filter(|_| new.verdict != old.and_then(|old| old.verdict))
    {
        let act = Happened::Verdict {
            review: name.clone(),
            verdict,
            status: None,
        };
        acts.push((new.reviewers.holder().cloned(), act));
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
        let act = Happened::Settle {
            finding: finding.name.clone(),
            settlement: settlement.clone(),
        };
        acts.push((Some(by), act));
    }
    acts
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
            "abc123\x02abc\x02p1 p2\x0217944170\x02demo-ab12\x02rule: why\x1fother: why\x03\0\n\
                     :100644 100644 1111111 0000000 D\0.atelier/items/demo-ab12/item.md\0\
                     :000000 100644 0000000 2222222 A\0.atelier/missions/demo-cd34/mission.md\0";
        let commit = commit(entry).unwrap();
        assert_eq!(commit.hash, "abc123");
        assert_eq!(commit.short, "abc");
        assert_eq!(commit.parents, ["p1", "p2"]);
        assert_eq!(commit.merged.as_deref(), Some("demo-ab12"));
        assert_eq!(commit.skips, ["rule: why", "other: why"]);
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
