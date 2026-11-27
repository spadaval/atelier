//! A mission's review pace, which `status` shows its lead: how many rounds
//! its items took, how long a round took, and what the rounds after the
//! first found.

use crate::model::{Recommendation, Time};
use crate::repo::item_dir;
use crate::view::Located;
use crate::{git, Repo};
use anyhow::Result;
use chrono::Duration;
use std::collections::{BTreeSet, HashMap};

/// What a mission's reviews cost, counted from its items' review records.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Pace {
    /// The items with at least one round that has a verdict.
    pub items: usize,
    /// The rounds that have a verdict.
    pub rounds: usize,
    /// For each round whose verdict commit was found: the time from the
    /// review's opening, its `created_at`, to the commit that wrote its
    /// verdict.
    pub took: Vec<Duration>,
    /// The findings a round after the first raised that no earlier round
    /// had, which carry no `Was:`.
    pub later_new: usize,
    /// Of those, the ones to fix now.
    pub later_fix_now: usize,
}

impl Pace {
    /// The middle of [`Pace::took`], the lower of the two middle ones when
    /// their count is even.
    pub fn median(&self) -> Option<Duration> {
        let mut took = self.took.clone();
        took.sort();
        took.get(took.len().saturating_sub(1) / 2).copied()
    }

    pub fn longest(&self) -> Option<Duration> {
        self.took.iter().max().copied()
    }
}

/// The pace of mission `mission`, from its items among `items`, each as
/// [`Located::record`] reads it. A verdict's time is the latest commit
/// that wrote a `verdict:` line in its review file, in the history of the
/// tip the item was read from; one `git log` serves the whole mission.
pub fn pace(repo: &Repo, items: &[Located], mission: &str) -> Result<Pace> {
    let of_mission: Vec<&Located> = (items.iter())
        .filter(|located| {
            located
                .record()
                .is_some_and(|dir| dir.item.mission.as_deref() == Some(mission))
        })
        .collect();
    let mut tips = BTreeSet::new();
    let mut paths = Vec::new();
    for located in &of_mission {
        let tip = located
            .source()
            .map_or(&repo.base_tip, |branch| &branch.tip);
        tips.insert(tip.as_str());
        paths.push(format!("{}/reviews", item_dir(&located.id)));
    }
    let written = match paths.is_empty() {
        true => HashMap::new(),
        false => verdicts_written(repo, &tips, &paths)?,
    };
    let mut pace = Pace::default();
    for located in of_mission {
        let Some(dir) = located.record() else {
            continue;
        };
        let judged: Vec<_> = (dir.reviews.iter())
            .filter(|file| file.review.verdict.is_some())
            .collect();
        if judged.is_empty() {
            continue;
        }
        pace.items += 1;
        pace.rounds += judged.len();
        for file in judged {
            let path = format!("{}/reviews/{}.md", item_dir(&located.id), file.name);
            if let Some(at) = written.get(&path) {
                pace.took.push(*at - file.review.created_at);
            }
            if file.name.round < 2 {
                continue;
            }
            // A review that does not read counts no findings; `check` and
            // `show` say what is wrong with it.
            for finding in file.findings().unwrap_or_default() {
                if finding.was.is_none() {
                    pace.later_new += 1;
                    if finding.recommendation == Recommendation::FixNow {
                        pace.later_fix_now += 1;
                    }
                }
            }
        }
    }
    Ok(pace)
}

/// For each review file under `paths`, the time of the latest commit in
/// the history of `tips` that added or changed a `verdict:` line in it.
fn verdicts_written(
    repo: &Repo,
    tips: &BTreeSet<&str>,
    paths: &[String],
) -> Result<HashMap<String, Time>> {
    let mut args = vec!["log", "--format=%x01%cI", "--name-only", "-G^verdict: [ac]"];
    args.extend(tips.iter().copied());
    args.push("--");
    args.extend(paths.iter().map(String::as_str));
    let out = git::run(&repo.root, &args)?;
    let mut written = HashMap::new();
    let mut at = None;
    for line in out.lines() {
        if let Some(time) = line.strip_prefix('\u{1}') {
            at = Time::parse_from_rfc3339(time.trim()).ok();
        } else if let (Some(at), false) = (at, line.trim().is_empty()) {
            // Newest first: the first commit seen for a file is its latest.
            written.entry(line.trim().to_string()).or_insert(at);
        }
    }
    Ok(written)
}
