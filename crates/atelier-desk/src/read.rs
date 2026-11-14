//! Each response, built from `atelier-core`'s views of a repository opened
//! for reading from commits, local branches and the forge's
//! remote-tracking branches. Nothing here decides what an item's status,
//! holder or conditions are, or which copy of a record counts: that is the
//! views'.

use crate::api::*;
use anyhow::Result;
use atelier_core::acts::{unsettled, STARTED};
use atelier_core::model::{
    is_deferred, is_open, Decision, Finding, MissionStatus as Stored, Named, Outcome as Settled,
    Recommendation as Rec, Thread, Time,
};
use atelier_core::plan::plan_changed_at;
use atelier_core::records::{CommitReader, ItemDir, MissionDir, ReadError, ReviewFile};
use atelier_core::say::condition_text;
use atelier_core::{
    annotate, disputes, entries, git, in_progress, item_view, items_of, list, mission_view, Entry,
    Filter, Judged, Located, Record, Repo, Skip, LIFECYCLE,
};
use std::collections::BTreeMap;
use std::path::Path;

/// Opens the repository at `dir` for reading from commits, for `caller`,
/// from the checkout that has its base branch out where one does, since
/// `config.toml` is read at that checkout's `HEAD`.
pub fn open<'s>(dir: &Path, caller: &str, skip: &'s Skip) -> Result<Repo<'s>> {
    let thread = || Thread(caller.to_string());
    let repo = Repo::open_committed(dir, thread(), skip)?;
    match git::checkout_of(&repo.root, &repo.base)? {
        Some(checkout) if checkout != repo.root => Repo::open_committed(&checkout, thread(), skip),
        _ => Ok(repo),
    }
}

/// What a response read from `repo` depends on: the checkout opened and
/// its `HEAD`, the base's tip, and the tip of every local and
/// remote-tracking branch, so a commit or a fetch builds it again.
pub fn commits(repo: &Repo) -> Result<Vec<String>> {
    let mut key = vec![
        repo.root.display().to_string(),
        git::head(&repo.root)?,
        repo.base_tip.clone(),
    ];
    let refs = git::run(
        &repo.root,
        &[
            "for-each-ref",
            "--format=%(refname) %(objectname)",
            "refs/heads",
            "refs/remotes",
        ],
    )?;
    key.extend(refs.lines().map(str::to_string));
    Ok(key)
}

/// Every item and mission, as `list` reads them, and what `status` shows
/// of each item.
struct Records {
    items: Vec<Located>,
    located: Vec<Located<MissionDir>>,
    /// Each mission's record that counts, with a stop on the forge's base
    /// marked.
    missions: Vec<MissionDir>,
    /// The IDs of the proposed missions.
    proposed: Vec<String>,
    entries: Vec<Entry>,
}

impl Records {
    fn read(repo: &mut Repo) -> Result<Records> {
        let items = repo.everything()?;
        let located = repo.all_missions()?;
        let mut missions: Vec<MissionDir> =
            located.iter().filter_map(|l| l.record().cloned()).collect();
        repo.published_stops(&located, &mut missions)?;
        let proposed = (located.iter())
            .filter(|l| l.proposed())
            .map(|l| l.id.clone())
            .collect();
        let mut entries = entries(&items, &missions, &repo.caller)?;
        // A plan dropped on its branch proposes nothing, its items
        // included, as `list` has it.
        let dropped = |id: &String| {
            let found = missions.iter().find(|m| m.mission.id == *id);
            found.is_some_and(|m| m.mission.status == Stored::Dropped)
        };
        entries.retain(|e| !(e.proposed && e.mission.as_ref().is_some_and(dropped)));
        Ok(Records {
            items,
            located,
            missions,
            proposed,
            entries,
        })
    }

    fn located(&self, id: &str) -> Option<&Located> {
        self.items.iter().find(|located| located.id == id)
    }

    fn mission(&self, id: &str) -> Option<(&Located<MissionDir>, &MissionDir)> {
        let located = self.located.iter().find(|l| l.id == id)?;
        let dir = self.missions.iter().find(|m| m.mission.id == id)?;
        Some((located, dir))
    }

    fn row(&self, repo: &Repo, dir: &MissionDir) -> Result<MissionRow> {
        let id = &dir.mission.id;
        let items = items_of(id, &self.entries);
        let located = self.located.iter().find(|l| l.id == *id);
        let in_progress = in_progress(dir, &self.entries);
        let state = located.map_or(Ok(raw_state(dir, None)), |l| state_of(repo, l, dir))?;
        let branch = located.and_then(Located::source).map(|b| b.name.clone());
        mission_row(dir, &items, in_progress, state, branch)
    }

    fn rows(&self, repo: &Repo, keep: impl Fn(&MissionRow) -> bool) -> Result<Vec<MissionRow>> {
        let mut rows = Vec::new();
        for dir in &self.missions {
            let row = self.row(repo, dir)?;
            if keep(&row) {
                rows.push(row);
            }
        }
        rows.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(rows)
    }
}

/// Where mission `dir`, read from `located`, stands. An agreed mission's
/// stop counts once it merges, wherever it is written, its own branch too.
/// A plan stopped on its branch is shown stopped here once the stop is on
/// the forge's copy of the branch; [`settle_state`] keeps it pending while
/// its pull request is open or not known.
fn state_of(repo: &Repo, located: &Located<MissionDir>, dir: &MissionDir) -> Result<MissionState> {
    let stop_on = stop_on(located);
    let base_open = (located.base.as_ref()).is_some_and(|b| b.mission.status != Stored::Dropped);
    let read_dropped = located
        .record()
        .is_some_and(|r| r.mission.status == Stored::Dropped);
    if dir.mission.status == Stored::Dropped && base_open && read_dropped {
        // Dropped on a branch of an agreed mission, not merged yet. One the
        // forge's base holds was marked by `published_stops`, and its
        // record that counts is not dropped.
        return Ok(MissionState::StopPending);
    }
    if dir.mission.status == Stored::Dropped && located.base.is_none() {
        if let Some(branch) = located.source() {
            // Dropped on the plan's own branch: published once the
            // forge's copy holds it.
            let published = match (branch.remote, repo.remote()) {
                (true, _) => true,
                (false, Some(remote)) => {
                    let name = repo.branch_of(&located.id);
                    git::remote_tip(&repo.root, remote, &name)? == Some(branch.tip.clone())
                }
                (false, None) => true,
            };
            if !published {
                return Ok(MissionState::StopPending);
            }
        }
    }
    if located.proposed() {
        return Ok(MissionState::Proposed);
    }
    Ok(raw_state(dir, stop_on))
}

fn raw_state(dir: &MissionDir, stop_on: Option<String>) -> MissionState {
    match dir.mission.status {
        Stored::Done => MissionState::Done,
        Stored::Dropped => MissionState::Stopped,
        Stored::Draft => MissionState::Proposed,
        Stored::Open if stop_on.is_some() => MissionState::StopPending,
        Stored::Open => MissionState::Open,
    }
}

/// The unmerged branch that drops an agreed mission whose base copy is
/// not dropped: a stop written and not merged.
fn stop_on(located: &Located<MissionDir>) -> Option<String> {
    let base = located.base.as_ref()?;
    if base.mission.status == Stored::Dropped {
        return None;
    }
    let dropped = |dir: &MissionDir| dir.mission.status == Stored::Dropped;
    let mut on = (located.elsewhere.iter()).chain(located.branch.as_ref().filter(|b| !b.merged));
    on.find(|b| b.record.as_ref().is_some_and(dropped))
        .map(|b| b.name.clone())
}

/// What the forge says of a branch's pull request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrState {
    /// The forge's list was read: the branch's pull request, or none.
    Known(Option<PrMark>),
    /// The list has not been read.
    Unknown,
}

impl PrState {
    pub fn mark(&self) -> Option<PrMark> {
        match self {
            PrState::Known(pr) => pr.clone(),
            PrState::Unknown => None,
        }
    }
}

/// `row`'s state once its pull request is known: a plan stopped on its
/// branch is pending while its pull request is open, or while the forge's
/// list has never been read, since closing it is not confirmed.
pub fn settle_state(row: &mut MissionRow, pr: &PrState) {
    let unconfirmed = match pr {
        PrState::Known(pr) => pr.as_ref().is_some_and(|pr| pr.state == "OPEN"),
        PrState::Unknown => true,
    };
    if row.state == MissionState::Stopped && row.branch.is_some() && unconfirmed {
        row.state = MissionState::StopPending;
    }
}

/// The branch name `shown`, a ref as the reader shows it, has on the
/// remote: `<remote>/<branch>` loses its remote.
pub fn local_name(repo: &Repo, shown: &str) -> String {
    match repo.remote() {
        Some(remote) => (shown.strip_prefix(&format!("{remote}/")))
            .unwrap_or(shown)
            .to_string(),
        None => shown.to_string(),
    }
}

/// Every mission, as the news compares them.
pub fn plans(repo: &mut Repo) -> Result<Vec<crate::news::Seen>> {
    let located = repo.all_missions()?;
    Ok((located.iter())
        .filter_map(|l| {
            Some(crate::news::Seen {
                id: l.id.clone(),
                title: l.record()?.mission.title.clone(),
                proposed: l.proposed(),
                agreed: l.base.is_some(),
            })
        })
        .collect())
}

fn needing_human() -> Filter {
    Filter {
        needs_human: true,
        ..Filter::default()
    }
}

pub fn summary(repo: &mut Repo, name: &str) -> Result<Option<RepoSummary>> {
    let records = Records::read(repo)?;
    let waiting = list(
        &records.entries,
        &records.missions,
        &records.proposed,
        &repo.caller,
        &needing_human(),
    );
    let missions = records.missions.iter();
    Ok(Some(RepoSummary {
        name: name.to_string(),
        path: repo.root.display().to_string(),
        base: Some(repo.base.clone()),
        base_tip: Some(repo.base_tip.clone()),
        in_progress: (missions.clone())
            .filter(|m| in_progress(m, &records.entries))
            .count(),
        proposed: (records.rows(repo, |row| row.state == MissionState::Proposed)?).len(),
        waiting: waiting.iter().map(|group| group.items.len()).sum(),
        error: None,
        fetched: FetchMark::default(),
    }))
}

/// Every mission, by ID, for the server to keep those not finished,
/// proposed ones among them, or with `done` the finished ones once their
/// state is settled; and the count of items in no mission that `list`
/// keeps with the same flag.
pub fn missions(repo: &mut Repo, done: bool) -> Result<Option<MissionsResponse>> {
    let records = Records::read(repo)?;
    let filter = Filter {
        done,
        ..Filter::default()
    };
    let groups = list(
        &records.entries,
        &records.missions,
        &records.proposed,
        &repo.caller,
        &filter,
    );
    let loose = groups.iter().find(|group| group.id.is_none());
    Ok(Some(MissionsResponse {
        commit: repo.base_tip.clone(),
        // Every mission: the server settles each one's state with its pull
        // request, and then keeps the finished or the unfinished.
        missions: records.rows(repo, |_| true)?,
        loose: loose.map_or(0, |group| group.items.len()),
        fetched: FetchMark::default(),
    }))
}

const SECTIONS: [&str; 6] = [
    "Why",
    "Outcome",
    "Success criteria",
    "Scope",
    "Authority",
    "Result",
];

pub fn mission(repo: &mut Repo, id: &str) -> Result<Option<MissionPage>> {
    let Some(view) = mission_view(repo, id)? else {
        return Ok(None);
    };
    let mut dirs = vec![view.dir.clone()];
    repo.published_stops(std::slice::from_ref(&view.mission), &mut dirs)?;
    let dir = &dirs[0];
    let items: Vec<&Entry> = view.items.iter().collect();
    let sections = SECTIONS.iter().filter_map(|heading| {
        let markdown = dir.section(heading)?.trim();
        (!markdown.is_empty()).then(|| Section {
            heading: heading.to_string(),
            markdown: markdown.to_string(),
        })
    });
    let source = view.mission.source();
    let read_at = source.map_or(repo.base_tip.clone(), |b| b.tip.clone());
    let findings = dir.findings()?;
    let mut plan_reviews = Vec::new();
    for file in &dir.reviews {
        let changed = plan_changed_at(repo, id, &file.review.commit, &read_at)?;
        plan_reviews.push(review(file, &findings, &[], changed)?);
    }
    let rows = items.iter().map(|entry| item_row(entry, &view.located));
    let mut threads = agents(named_in_mission(dir), None);
    for entry in &items {
        let record = view.located.iter().find(|l| l.id == entry.id);
        if let Some(record) = record.and_then(Located::record) {
            threads.extend(agents(named_in_item(record)?, Some(&entry.id)));
        }
    }
    let state = state_of(repo, &view.mission, dir)?;
    let branch = source.map(|b| b.name.clone());
    let stop_on = stop_on(&view.mission).filter(|_| state == MissionState::StopPending);
    let row = mission_row(dir, &items, view.in_progress, state, branch)?;
    let stop_commands = match state {
        MissionState::Open => stop_commands(id),
        _ => Vec::new(),
    };
    Ok(Some(MissionPage {
        commit: repo.base_tip.clone(),
        sections: sections.collect(),
        dropped: dir.dropped().map(str::to_string),
        documents: documents(&repo.root, &read_at, id)?,
        read_at,
        pull_request: None,
        round: round(dir),
        stop_on,
        can_stop: state == MissionState::Proposed && repo.remote().is_some(),
        stop_commands,
        items: rows.collect::<Result<_>>()?,
        plan_reviews,
        threads,
        t3: T3State::default(),
        fetched: FetchMark::default(),
        mission: row,
    }))
}

/// The commands that stop agreed mission `id` by the record-only path,
/// run in a checkout of the repository.
fn stop_commands(id: &str) -> Vec<String> {
    vec![
        format!("atelier start --backlog stop-{id}"),
        format!("atelier drop {id} --reason \"<why>\""),
        format!("git commit -m \"Drop {id}\" .atelier/missions/{id}/mission.md"),
        "atelier submit".to_string(),
        "atelier merge".to_string(),
    ]
}

/// The verdicts of `dir`'s last round of plan review.
fn round(dir: &MissionDir) -> Option<RoundMark> {
    let last = dir.reviews.iter().map(|r| r.name.round).max()?;
    let reviews = (dir.reviews.iter())
        .filter(|r| r.name.round == last)
        .map(|r| RoundReview {
            name: r.name.to_string(),
            reviewer: r.review.reviewers.holder().map(Thread::to_string),
            model: r.review.model.clone(),
            verdict: r.review.verdict.map(Into::into),
        });
    Some(RoundMark {
        round: last,
        reviews: reviews.collect(),
    })
}

/// Each identity `dir` names, with its role, once per role: its planners,
/// its lead and the reviewers of its plan reviews.
fn named_in_mission(dir: &MissionDir) -> Vec<(String, Role)> {
    let planners = (dir.mission.planners.0.iter()).map(|t| (t.0.clone(), Role::Planner));
    let lead = dir
        .mission
        .lead
        .iter()
        .map(|lead| (lead.0.clone(), Role::Lead));
    let reviewers = dir.reviews.iter().flat_map(|file| &file.review.reviewers.0);
    let reviewers = reviewers.map(|reviewer| (reviewer.0.clone(), Role::Reviewer));
    once_each(planners.chain(lead).chain(reviewers))
}

/// Each identity `record` names, with its role, once per role: its
/// implementers, the reviewers of its reviews and the deciders of its
/// answers.
fn named_in_item(record: &ItemDir) -> Result<Vec<(String, Role)>> {
    let implementers = record.item.implementers.0.iter();
    let implementers = implementers.map(|thread| (thread.0.clone(), Role::Implementer));
    let reviewers = record
        .reviews
        .iter()
        .flat_map(|file| &file.review.reviewers.0);
    let reviewers = reviewers.map(|reviewer| (reviewer.0.clone(), Role::Reviewer));
    let decisions = record.decisions()?;
    let answers = decisions.iter().filter_map(|d| d.answer.as_ref());
    let deciders = answers.map(|answer| (answer.by.0.clone(), Role::Decider));
    Ok(once_each(implementers.chain(reviewers).chain(deciders)))
}

fn once_each(named: impl Iterator<Item = (String, Role)>) -> Vec<(String, Role)> {
    let mut seen = Vec::new();
    for pair in named {
        if !seen.contains(&pair) {
            seen.push(pair);
        }
    }
    seen
}

/// Agents for `named`, on `item`, before T3 Code's threads are joined.
fn agents(named: Vec<(String, Role)>, item: Option<&str>) -> Vec<Agent> {
    let agents = named
        .into_iter()
        .map(|(identity, role)| agent(identity, role, item));
    agents.collect()
}

fn agent(identity: String, role: Role, item: Option<&str>) -> Agent {
    Agent {
        identity,
        role,
        item: item.map(str::to_string),
        thread: None,
    }
}

/// What each identity holds in the records of the repository served as
/// `name`: every item, by the record the cross-branch rule picks, and
/// every mission.
pub fn holds(repo: &mut Repo, name: &str) -> Result<Option<Vec<(String, Hold)>>> {
    let records = Records::read(repo)?;
    let hold = |kind, id: &str, (identity, role)| {
        let id = id.to_string();
        let repo = name.to_string();
        (
            identity,
            Hold {
                repo,
                kind,
                id,
                role,
            },
        )
    };
    let mut holds = Vec::new();
    for record in records.items.iter().filter_map(Located::record) {
        let named = named_in_item(record)?.into_iter();
        holds.extend(named.map(|named| hold(HoldKind::Item, &record.item.id, named)));
    }
    for dir in &records.missions {
        let named = named_in_mission(dir).into_iter();
        holds.extend(named.map(|named| hold(HoldKind::Mission, &dir.mission.id, named)));
    }
    Ok(Some(holds))
}

/// Each document of mission `id` at `commit`, titled by its first `# `
/// heading, else by its file name.
fn documents(root: &Path, commit: &str, id: &str) -> Result<Vec<Document>> {
    let mut reader = CommitReader::new(root);
    let mut documents = Vec::new();
    for path in reader.mission_documents(commit, id)? {
        let name = path.rsplit('/').next().unwrap_or(&path).to_string();
        let heading = match path.ends_with(".md") {
            true => reader.text(commit, &path).ok().and_then(|text| {
                let line = text.lines().find_map(|line| line.strip_prefix("# "));
                line.map(|line| line.trim().to_string())
            }),
            false => None,
        };
        documents.push(Document {
            title: heading.unwrap_or(name),
            path,
        });
    }
    Ok(documents)
}

fn mission_row(
    dir: &MissionDir,
    items: &[&Entry],
    in_progress: bool,
    state: MissionState,
    branch: Option<String>,
) -> Result<MissionRow> {
    let mission = &dir.mission;
    let counts = LIFECYCLE.iter().filter_map(|status| {
        let count = items.iter().filter(|e| e.status == *status).count();
        (count > 0).then(|| StatusCount {
            status: (*status).into(),
            count,
        })
    });
    let waiting = (items.iter())
        .filter(|entry| !entry.finished() && entry.needs_human())
        .map(|entry| entry.id.clone());
    Ok(MissionRow {
        id: mission.id.clone(),
        title: mission.title.clone(),
        status: mission.status.into(),
        state,
        labels: mission.labels.clone(),
        lead: mission.lead.as_ref().map(Thread::to_string),
        created_at: time(&mission.created_at),
        in_progress,
        counts: counts.collect(),
        waiting: waiting.collect(),
        branch,
    })
}

/// An item's row, from its entry and its records, found among `located`.
fn item_row(entry: &Entry, located: &[Located]) -> Result<ItemRow> {
    let located = located.iter().find(|l| l.id == entry.id);
    let record = located.and_then(Located::record);
    let conditions = |conditions: &[atelier_core::acts::Condition]| {
        let texts = conditions.iter().map(|c| condition_text(&entry.id, c));
        texts.collect::<Vec<_>>()
    };
    Ok(ItemRow {
        id: entry.id.clone(),
        title: entry.title.clone(),
        status: entry.status.into(),
        priority: entry.priority.into(),
        labels: entry.labels.clone(),
        needs_human: entry.needs_human(),
        startable: entry.startable(),
        blockers: (entry.blockers.iter())
            .map(|(id, status)| Blocker {
                id: id.clone(),
                status: status.map(Into::into),
            })
            .collect(),
        waiting_on: entry
            .waiting_on()
            .iter()
            .map(|(id, _)| id.clone())
            .collect(),
        holder: entry.holder.as_ref().map(Thread::to_string),
        reviewer: entry.reviewer.as_ref().map(Thread::to_string),
        cannot_start: match entry.status {
            atelier_core::model::Status::Ready => conditions(&entry.unmet),
            _ => Vec::new(),
        },
        needs: conditions(&entry.needs),
        branch: (located.and_then(|l| l.source().or(l.branch.as_ref()))).map(|branch| BranchMark {
            merged: branch.merged,
            name: branch.name.clone(),
        }),
        report_written: record.is_some_and(ItemDir::report_written),
        dropped: record.and_then(ItemDir::dropped).map(str::to_string),
        approved_at: entry.approved.clone(),
    })
}

fn question_of(item: &str, decision: &Decision) -> Question {
    Question {
        item: item.to_string(),
        question: decision.question.clone(),
        finding: decision.finding.as_ref().map(ToString::to_string),
        answer: decision.answer.as_ref().map(|answer| AnswerMark {
            by: answer.by.to_string(),
            text: answer.text.clone(),
        }),
        outcome: decision.outcome.map(Into::into),
    }
}

/// A review, its findings judged against `all`, every finding of its
/// record, and the record's `decisions`.
fn review(
    file: &ReviewFile,
    all: &[Finding],
    decisions: &[Decision],
    changed_since: Option<Vec<String>>,
) -> Result<Review> {
    let findings = file.findings()?.into_iter().map(|f| FindingRow {
        name: f.name.to_string(),
        title: f.title.clone(),
        recommendation: f.recommendation.into(),
        was: f.was.as_ref().map(ToString::to_string),
        now: f.now.map(Into::into),
        decided: f.decided.map(Into::into),
        settlement: f.settlement.as_ref().map(|settlement| match settlement {
            atelier_core::model::Settlement::Taken { item, by } => Settlement::Taken {
                item: item.clone(),
                by: by.to_string(),
            },
            atelier_core::model::Settlement::Dropped { by, reason } => Settlement::Dropped {
                by: by.to_string(),
                reason: reason.clone(),
            },
        }),
        text: file
            .finding_text(f.name.number)
            .map(|text| text.trim().to_string()),
        open: is_open(&f, all, decisions),
        deferred: is_deferred(&f, all, decisions),
    });
    let review = &file.review;
    Ok(Review {
        name: file.name.to_string(),
        round: file.name.round,
        commit: review.commit.clone(),
        reviewers: review.reviewers.0.iter().map(Thread::to_string).collect(),
        model: review.model.clone(),
        verdict: review.verdict.map(Into::into),
        created_at: time(&review.created_at),
        findings: findings.collect(),
        changed_since,
    })
}

pub fn item(repo: &mut Repo, id: &str) -> Result<Option<ItemPage>> {
    let Some(view) = item_view(repo, id)? else {
        return Ok(None);
    };
    let record = view.record();
    let item = &record.item;
    let decisions = record.decisions()?;
    let section = |heading| record.section(heading).map(|s| s.trim().to_string());
    let reviews = record.reviews.iter();
    let responses = record
        .responses()?
        .into_iter()
        .map(|response| match response {
            atelier_core::model::Response::Fixed { finding, commit } => ResponseLine::Fixed {
                finding: finding.to_string(),
                commit,
            },
            atelier_core::model::Response::Rebutted { finding, fact } => ResponseLine::Rebutted {
                finding: finding.to_string(),
                fact,
            },
        });
    Ok(Some(ItemPage {
        commit: repo.base_tip.clone(),
        row: item_row(&view.entry, std::slice::from_ref(&view.located))?,
        mission: item.mission.clone(),
        blocked_by: item.blocked_by.clone(),
        implementers: item.implementers.0.iter().map(Thread::to_string).collect(),
        ready: item.ready.as_ref().map(|ready| ReadyMark {
            by: ready.by.clone(),
            at: time(&ready.at),
        }),
        trivial: item.trivial.clone(),
        caused_by: item.caused_by.clone(),
        created_at: time(&item.created_at),
        read_from: view.read_from.into(),
        body: item.body.clone(),
        goal: section("Goal"),
        scope: section("Scope"),
        report: section("Report"),
        done_when: (record.done_when().into_iter())
            .map(|(ticked, text)| DoneWhen { ticked, text })
            .collect(),
        decisions: decisions.iter().map(|d| question_of(id, d)).collect(),
        responses: responses.collect(),
        reviews: reviews
            .map(|file| review(file, &view.findings, &decisions, None))
            .collect::<Result<_>>()?,
        unsettled: unsettled(record)?
            .iter()
            .map(|f| f.name.to_string())
            .collect(),
        threads: agents(named_in_item(record)?, None),
        t3: T3State::default(),
        fetched: FetchMark::default(),
    }))
}

/// The markdown file at `path`, from the root, at `at`, a commit, or at
/// the base's tip.
pub fn doc(repo: &mut Repo, path: &str, at: Option<&str>) -> Result<Option<DocResponse>> {
    let commit = at.unwrap_or(&repo.base_tip).to_string();
    if !git::has_commit(&repo.root, &commit)? {
        return Ok(None);
    }
    let mut reader = CommitReader::new(&repo.root);
    match reader.text(&commit, path) {
        Err(ReadError::Missing { .. } | ReadError::NotAFile { .. }) => Ok(None),
        Err(error) => Err(error.into()),
        Ok(markdown) => Ok(Some(DocResponse {
            commit,
            path: path.to_string(),
            markdown,
        })),
    }
}

/// What waits on the human in one repository.
pub fn inbox(repo: &mut Repo, name: &str) -> Result<Option<InboxRepo>> {
    let records = Records::read(repo)?;
    let mut needs_human = Vec::new();
    for group in list(
        &records.entries,
        &records.missions,
        &records.proposed,
        &repo.caller,
        &needing_human(),
    ) {
        let mut items = Vec::new();
        for entry in group.items {
            let mut entry = entry.clone();
            if let Some(record) = records.located(&entry.id).and_then(Located::record) {
                annotate(repo, &mut entry, record)?;
            }
            items.push(item_row(&entry, &records.items)?);
        }
        needs_human.push(NeedsHuman {
            mission_title: group.mission.map(|dir| dir.mission.title.clone()),
            mission: group.id,
            items,
        });
    }
    let proposed = records.rows(repo, |row| row.state == MissionState::Proposed)?;
    Ok(Some(InboxRepo {
        repo: name.to_string(),
        commit: Some(repo.base_tip.clone()),
        error: None,
        needs_human,
        proposed,
        threads: Vec::new(),
        fetched: FetchMark::default(),
    }))
}

/// What is proposed in one repository: its proposed plans, and the record
/// changes on every other unmerged branch but a started item's own.
pub fn proposed(repo: &mut Repo, name: &str) -> Result<Option<ProposedRepo>> {
    let records = Records::read(repo)?;
    let mut plans = Vec::new();
    let mut plan_branches = Vec::new();
    // Proposed as the mission's page shows it: open on a branch that has
    // not merged, or an old `draft` on the base; and an agreed mission
    // whose own unmerged branch changes it, an amendment, which merges by
    // its rules.
    // Records on a branch beside its mission's, each as a change of that
    // branch: an agreed mission's own drop, not merged yet.
    let mut beside: Vec<(String, ChangedRecord)> = Vec::new();
    for dir in &records.missions {
        let row = records.row(repo, dir)?;
        let Some((located, _)) = records.mission(&dir.mission.id) else {
            continue;
        };
        let source = located.source().map(|b| b.name.clone());
        let agreed = located.base.as_ref();
        // A plan's own branch holds the plan, its items too, whether it is
        // proposed or stopped there: none of it is a change of its own.
        // An agreed mission's branch is a change like any branch's.
        if agreed.is_none() {
            plan_branches.extend(source.clone());
        }
        // An agreed mission's own unmerged branch, by what it holds there:
        // a drop, which is a change of that branch, or an amendment.
        let own = (located.branch.as_ref()).filter(|b| !b.merged && agreed.is_some());
        let own = own.and_then(|b| Some((b, b.record.as_ref()?)));
        let mut amends = false;
        if let (Some(base), Some((branch, record))) = (agreed, own) {
            let dropped = |dir: &MissionDir| dir.mission.status == Stored::Dropped;
            if dropped(record) && !dropped(base) {
                let change = ChangedRecord {
                    kind: HoldKind::Mission,
                    id: dir.mission.id.clone(),
                    title: record.mission.title.clone(),
                    change: Change::Dropped,
                };
                beside.push((branch.name.clone(), change));
            } else if !dropped(record) && !base.finished() {
                amends = true;
            }
        }
        if row.state != MissionState::Proposed && !amends {
            continue;
        }
        plans.push(ProposedPlan {
            mission: row,
            amends,
            pull_request: None,
            round: round(dir),
        });
    }
    plans.sort_by(|a, b| a.mission.id.cmp(&b.mission.id));
    let own_item = |shown: &str| match repo.config.named(&local_name(repo, shown)) {
        // A started item's branch; a branch named for a mission has a
        // `Located` among the items too, with no record.
        Some(Named::Record(id)) => {
            (records.items.iter()).any(|l| l.id == id && l.record().is_some())
        }
        _ => false,
    };
    let mut by_branch: BTreeMap<String, Vec<ChangedRecord>> = BTreeMap::new();
    let mut note = |branch: &str, record: ChangedRecord| {
        if !plan_branches.iter().any(|b| b == branch) && !own_item(branch) {
            by_branch
                .entry(branch.to_string())
                .or_default()
                .push(record);
        }
    };
    for located in &records.items {
        changes_of(located, HoldKind::Item, &mut note);
    }
    for located in &records.located {
        changes_of(located, HoldKind::Mission, &mut note);
    }
    for (branch, record) in beside {
        note(&branch, record);
    }
    let changes = by_branch
        .into_iter()
        .map(|(branch, records)| RecordChanges {
            branch,
            pull_request: None,
            records,
        })
        .collect();
    Ok(Some(ProposedRepo {
        repo: name.to_string(),
        commit: Some(repo.base_tip.clone()),
        error: None,
        fetched: FetchMark::default(),
        plans,
        changes,
    }))
}

/// Each change an unmerged branch proposes to `located`: a record it adds,
/// where the base has none, and each change to the base's copy.
fn changes_of<R: Record>(
    located: &Located<R>,
    kind: HoldKind,
    note: &mut impl FnMut(&str, ChangedRecord),
) {
    let changed = |change, title: &str| ChangedRecord {
        kind,
        id: located.id.clone(),
        title: title.to_string(),
        change,
    };
    match &located.base {
        None => {
            if !located.proposed() {
                return;
            }
            let adding = located.source().into_iter().chain(located.also_added());
            for branch in adding {
                if let Some(record) = &branch.record {
                    note(&branch.name, changed(Change::Added, record.title()));
                }
            }
        }
        Some(base) => {
            for branch in located.changes() {
                let change = match &branch.record {
                    None => Change::Removed,
                    Some(r) if r.is_dropped() && !base.is_dropped() => Change::Dropped,
                    Some(_) => Change::Changed,
                };
                let title = branch.record.as_ref().map_or(base.title(), R::title);
                note(&branch.name, changed(change, title));
            }
        }
    }
}

/// Work in motion in one repository: each started item, read from its
/// branch, with its holder and reviewer, and each agreed mission with a
/// started item, with its lead.
pub fn motion(repo: &mut Repo, name: &str) -> Result<Option<MotionRepo>> {
    let records = Records::read(repo)?;
    let mut items = Vec::new();
    for entry in records.entries.iter() {
        if !STARTED.contains(&entry.status) {
            continue;
        }
        let located = records.located(&entry.id);
        let branch = located.and_then(|l| l.source().or(l.branch.as_ref()));
        let id = Some(entry.id.as_str());
        items.push(MotionItem {
            id: entry.id.clone(),
            title: entry.title.clone(),
            status: entry.status.into(),
            mission: entry.mission.clone(),
            branch: branch.map(|b| b.name.clone()),
            holder: (entry.holder.as_ref()).map(|t| agent(t.to_string(), Role::Implementer, id)),
            reviewer: (entry.reviewer.as_ref()).map(|t| agent(t.to_string(), Role::Reviewer, id)),
        });
    }
    items.sort_by(|a, b| (&a.mission, &a.id).cmp(&(&b.mission, &b.id)));
    let mut missions = Vec::new();
    for dir in &records.missions {
        if in_progress(dir, &records.entries) {
            let lead = dir.mission.lead.as_ref();
            missions.push(MotionMission {
                mission: records.row(repo, dir)?,
                lead: lead.map(|t| agent(t.to_string(), Role::Lead, None)),
            });
        }
    }
    missions.sort_by(|a, b| a.mission.id.cmp(&b.mission.id));
    Ok(Some(MotionRepo {
        repo: name.to_string(),
        commit: Some(repo.base_tip.clone()),
        error: None,
        fetched: FetchMark::default(),
        items,
        missions,
    }))
}

/// What `atelier list --disputed` gives in one repository, with the
/// thread on each side and the model its review records.
pub fn disputed(repo: &mut Repo, name: &str) -> Result<Option<DisputedRepo>> {
    let records = Records::read(repo)?;
    let found = disputes(&records.items, &records.missions, None)?;
    let reviewer_of = |item: &ItemDir, review: &atelier_core::model::ReviewName| {
        let file = item.reviews.iter().find(|r| r.name == *review)?;
        let identity = file.review.reviewers.holder()?.to_string();
        Some(side(identity, file.review.model.clone()))
    };
    let mut rebuttals = Vec::new();
    for rebuttal in found.rebuttals {
        let record = records.located(&rebuttal.item).and_then(Located::record);
        let Some(record) = record else {
            continue;
        };
        let implementer = record.item.implementers.holder();
        rebuttals.push(RebuttalRow {
            implementer: implementer.map(|t| side(t.to_string(), None)),
            raised_by: reviewer_of(record, &rebuttal.finding.review),
            judged: (rebuttal.judged.iter())
                .map(|(finding, judged)| JudgedRow {
                    finding: finding.to_string(),
                    judged: judged_words(*judged),
                    reviewer: reviewer_of(record, &finding.review),
                })
                .collect(),
            item: rebuttal.item,
            mission: rebuttal.mission,
            finding: rebuttal.finding.to_string(),
            title: rebuttal.title,
            fact: rebuttal.fact,
        });
    }
    let plans = (found.plans.into_iter())
        .map(|split| SplitRoundRow {
            mission: split.mission,
            round: split.round,
            reviews: (split.reviews.into_iter())
                .map(|(review, reviewer, model, verdict)| SplitReview {
                    name: review.to_string(),
                    reviewer: reviewer.map(|t| side(t.to_string(), model)),
                    verdict: verdict.into(),
                })
                .collect(),
        })
        .collect();
    Ok(Some(DisputedRepo {
        repo: name.to_string(),
        commit: Some(repo.base_tip.clone()),
        error: None,
        fetched: FetchMark::default(),
        rebuttals,
        plans,
    }))
}

fn side(identity: String, model: Option<String>) -> Side {
    Side {
        identity,
        model,
        provider: None,
        link: None,
    }
}

fn judged_words(judged: Judged) -> String {
    let settled = |outcome| match outcome {
        Settled::Defer => "defer",
        Settled::NoAction => "no action",
    };
    match judged {
        Judged::Kept => "kept".into(),
        Judged::Withdrawn => "withdrawn".into(),
        Judged::Fixed => "fixed".into(),
        Judged::Lowered(Rec::Defer) => "lowered to defer".into(),
        Judged::Lowered(Rec::NoAction) => "lowered to no action".into(),
        Judged::Lowered(Rec::FixNow) => "kept".into(),
        Judged::Decided(outcome) => format!("decided: {}", settled(outcome)),
    }
}

fn time(time: &Time) -> String {
    time.to_rfc3339()
}
