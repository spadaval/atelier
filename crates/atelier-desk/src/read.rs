//! Each response, built from `atelier-app`'s views of a repository opened
//! for reading from commits. Nothing here decides what an item's status,
//! holder or conditions are: that is the views'.

use crate::api::*;
use anyhow::Result;
use atelier_app::acts::{check_mission, question, unsettled, Act, MissionFacts, STARTED};
use atelier_app::plan::plan_changed_at;
use atelier_app::say::condition_text;
use atelier_app::{
    annotate, entries, git, in_progress, item_view, items_of, list, mission_view, Entry, Filter,
    Located, Repo, Skip, LIFECYCLE,
};
use atelier_core::{is_deferred, is_open, Decision, Finding, Thread, Time};
use atelier_records::{validate_id, CommitReader, ItemDir, MissionDir, ReadError, ReviewFile};
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
/// its `HEAD`, the base's tip, and the tip of each branch named for an
/// item.
pub fn commits(repo: &Repo) -> Result<Vec<String>> {
    let mut key = vec![
        repo.root.display().to_string(),
        git::head(&repo.root)?,
        repo.base_tip.clone(),
    ];
    for (name, tip) in git::branches(&repo.root)? {
        if validate_id(&name).is_ok() {
            key.push(format!("{name} {tip}"));
        }
    }
    Ok(key)
}

/// Every item and mission, and what `status` shows of each item.
struct Records {
    items: Vec<Located>,
    missions: Vec<MissionDir>,
    entries: Vec<Entry>,
}

impl Records {
    fn read(repo: &mut Repo) -> Result<Records> {
        let items = repo.everything()?;
        let missions = repo.missions()?;
        let entries = entries(&items, &missions, &repo.caller)?;
        Ok(Records {
            items,
            missions,
            entries,
        })
    }

    fn located(&self, id: &str) -> Option<&Located> {
        self.items.iter().find(|located| located.id == id)
    }

    fn row(&self, mission: &MissionDir) -> Result<MissionRow> {
        let items = items_of(&mission.mission.id, &self.entries);
        let in_progress = in_progress(mission, &self.entries);
        mission_row(mission, &items, &self.items, in_progress)
    }
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
        drafts: missions
            .filter(|m| m.mission.status == atelier_core::MissionStatus::Draft)
            .count(),
        waiting: waiting.iter().map(|group| group.items.len()).sum(),
        error: None,
    }))
}

/// The missions, open or with `done` finished, by ID, and the count of
/// items in no mission that `list` keeps with the same flag.
pub fn missions(repo: &mut Repo, done: bool) -> Result<Option<MissionsResponse>> {
    let records = Records::read(repo)?;
    let finished = |dir: &&MissionDir| {
        use atelier_core::MissionStatus::{Done, Dropped};
        matches!(dir.mission.status, Done | Dropped)
    };
    let mut kept: Vec<&MissionDir> = records
        .missions
        .iter()
        .filter(|m| finished(m) == done)
        .collect();
    kept.sort_by(|a, b| a.mission.id.cmp(&b.mission.id));
    let filter = Filter {
        done,
        ..Filter::default()
    };
    let groups = list(&records.entries, &records.missions, &repo.caller, &filter);
    let loose = groups.iter().find(|group| group.id.is_none());
    Ok(Some(MissionsResponse {
        commit: repo.base_tip.clone(),
        missions: kept
            .into_iter()
            .map(|m| records.row(m))
            .collect::<Result<_>>()?,
        loose: loose.map_or(0, |group| group.items.len()),
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
    let dir = &view.dir;
    let items: Vec<&Entry> = view.items.iter().collect();
    let sections = SECTIONS.iter().filter_map(|heading| {
        let markdown = dir.section(heading)?.trim();
        (!markdown.is_empty()).then(|| Section {
            heading: heading.to_string(),
            markdown: markdown.to_string(),
        })
    });
    let facts = MissionFacts {
        mission: dir,
        items: &[],
        review: None,
    };
    let base_tip = repo.base_tip.clone();
    let findings = dir.findings()?;
    let mut plan_reviews = Vec::new();
    for file in &dir.reviews {
        let changed = plan_changed_at(repo, id, &file.review.commit, &base_tip)?;
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
    Ok(Some(MissionPage {
        commit: base_tip.clone(),
        mission: mission_row(dir, &items, &view.located, view.in_progress)?,
        sections: sections.collect(),
        dropped: dir.dropped().map(str::to_string),
        can_ready: check_mission(Act::ReadyMission, &facts).is_ok(),
        documents: documents(&repo.root, &base_tip, id)?,
        items: rows.collect::<Result<_>>()?,
        questions: (view.questions.iter())
            .map(|(item, decision)| question_of(item, decision))
            .collect(),
        plan_reviews,
        threads,
        t3: T3State::default(),
    }))
}

/// Each identity `dir` names, with its role, once per role: its lead and
/// the reviewers of its plan reviews.
fn named_in_mission(dir: &MissionDir) -> Vec<(String, Role)> {
    let lead = dir
        .mission
        .lead
        .iter()
        .map(|lead| (lead.0.clone(), Role::Lead));
    let reviewers = dir.reviews.iter().flat_map(|file| &file.review.reviewers.0);
    let reviewers = reviewers.map(|reviewer| (reviewer.0.clone(), Role::Reviewer));
    once_each(lead.chain(reviewers))
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
    let agents = named.into_iter().map(|(identity, role)| Agent {
        identity,
        role,
        item: item.map(str::to_string),
        thread: None,
    });
    agents.collect()
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
    located: &[Located],
    in_progress: bool,
) -> Result<MissionRow> {
    let mission = &dir.mission;
    let counts = LIFECYCLE.iter().filter_map(|status| {
        let count = items.iter().filter(|e| e.status == *status).count();
        (count > 0).then(|| StatusCount {
            status: (*status).into(),
            count,
        })
    });
    let mut waiting = Vec::new();
    for entry in items.iter().filter(|entry| !entry.finished()) {
        let record = located
            .iter()
            .find(|l| l.id == entry.id)
            .and_then(Located::record);
        let asked = match record {
            Some(record) => question(record)?.is_some(),
            None => false,
        };
        if entry.needs_human() || asked {
            waiting.push(entry.id.clone());
        }
    }
    Ok(MissionRow {
        id: mission.id.clone(),
        title: mission.title.clone(),
        status: mission.status.into(),
        labels: mission.labels.clone(),
        lead: mission.lead.as_ref().map(Thread::to_string),
        ready: mission.ready.as_ref().map(|ready| ReadyMark {
            by: ready.by.clone(),
            at: time(&ready.at),
        }),
        created_at: time(&mission.created_at),
        in_progress,
        counts: counts.collect(),
        waiting,
    })
}

/// An item's row, from its entry and its records, found among `located`.
fn item_row(entry: &Entry, located: &[Located]) -> Result<ItemRow> {
    let located = located.iter().find(|l| l.id == entry.id);
    let record = located.and_then(Located::record);
    let conditions = |conditions: &[atelier_app::acts::Condition]| {
        let texts = conditions.iter().map(|c| condition_text(&entry.id, c));
        texts.collect::<Vec<_>>()
    };
    let asked = match record {
        Some(record) => question(record)?.map(|(_, decision)| decision.question),
        None => None,
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
            atelier_core::Status::Ready => conditions(&entry.unmet),
            _ => Vec::new(),
        },
        needs: conditions(&entry.needs),
        branch: (located.and_then(|l| l.branch.as_ref())).map(|branch| BranchMark {
            merged: branch.merged,
        }),
        question: asked,
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
            atelier_core::Settlement::Taken { item, by } => Settlement::Taken {
                item: item.clone(),
                by: by.to_string(),
            },
            atelier_core::Settlement::Dropped { by, reason } => Settlement::Dropped {
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
            atelier_core::Response::Fixed { finding, commit } => ResponseLine::Fixed {
                finding: finding.to_string(),
                commit,
            },
            atelier_core::Response::Rebutted { finding, fact } => ResponseLine::Rebutted {
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
    }))
}

/// The markdown file at `path`, from the root, at the base's tip.
pub fn doc(repo: &mut Repo, path: &str) -> Result<Option<DocResponse>> {
    let mut reader = CommitReader::new(&repo.root);
    match reader.text(&repo.base_tip, path) {
        Err(ReadError::Missing { .. } | ReadError::NotAFile { .. }) => Ok(None),
        Err(error) => Err(error.into()),
        Ok(markdown) => Ok(Some(DocResponse {
            commit: repo.base_tip.clone(),
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
    let drafts = records
        .missions
        .iter()
        .filter(|m| m.mission.status == atelier_core::MissionStatus::Draft);
    let mut questions = Vec::new();
    for record in records.items.iter().filter_map(Located::record) {
        if !STARTED.contains(&record.item.status) {
            continue;
        }
        if let Some((_, decision)) = question(record)? {
            questions.push(InboxQuestion {
                item: record.item.id.clone(),
                mission: record.item.mission.clone(),
                question: decision.question,
                finding: decision.finding.as_ref().map(ToString::to_string),
            });
        }
    }
    Ok(Some(InboxRepo {
        repo: name.to_string(),
        commit: Some(repo.base_tip.clone()),
        error: None,
        needs_human,
        drafts: drafts.map(|m| records.row(m)).collect::<Result<_>>()?,
        questions,
        threads: Vec::new(),
    }))
}

fn time(time: &Time) -> String {
    time.to_rfc3339()
}
