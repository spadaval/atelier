//! What `show`, `list` and `lead` print, and the missions `status` lists.
//! The wording lives here, beside `output.rs`, so it is cheap to change.

use crate::output::{
    condition, handoff, holder, mission_status_name, priority_name, settled, status_name,
    verdict_name,
};
use atelier_core::actions::{Condition, PLANNED, STARTED};
use atelier_core::model::{
    Finding, Mission, MissionStatus, Outcome, Ready, Recommendation, Status, Thread,
};
use atelier_core::records::{ItemDir, MissionDir, ReviewFile};
use atelier_core::{Disputes, Entry, Filter, Group, Judged, Overview, LIFECYCLE};
use std::time::SystemTime;

/// Where `show` read a record from, and the other unmerged branches that
/// add or change it.
pub struct Whence {
    /// The ref read: the base, a local branch, or `<remote>/<branch>`.
    pub from: String,
    /// For a remote-tracking ref, when this clone last fetched, if ever.
    pub fetched: Option<Option<SystemTime>>,
    pub proposed: bool,
    /// The local branch and its remote-tracking branch have diverged.
    pub diverged: bool,
    /// The other unmerged branches that add it.
    pub also_added: Vec<String>,
    /// Each unmerged branch that changes the base's copy, with the status
    /// of its copy there, by name; `None` where it removes it.
    pub changes: Vec<(String, Option<&'static str>)>,
    /// A mission's stop that is written and not published, said whole.
    pub unpublished: Option<String>,
}

/// A record change an unmerged branch proposes, as `list` shows it.
pub struct Change {
    pub id: String,
    pub title: String,
    /// The branch, as it is shown.
    pub on: String,
    /// The status of the base's copy, and of the branch's, if it keeps one.
    pub before: &'static str,
    pub after: Option<&'static str>,
}

/// How long ago `time` was, in the largest whole unit.
fn ago(time: SystemTime) -> String {
    let seconds = SystemTime::now()
        .duration_since(time)
        .map_or(0, |since| since.as_secs());
    let (count, unit) = match seconds {
        0..60 => (seconds, "second"),
        60..3600 => (seconds / 60, "minute"),
        3600..86400 => (seconds / 3600, "hour"),
        _ => (seconds / 86400, "day"),
    };
    let plural = if count == 1 { "" } else { "s" };
    format!("{count} {unit}{plural} ago")
}

/// The age of the last fetch of `remote`.
fn fetch_age(remote: &str, fetched: Option<SystemTime>) -> String {
    match fetched {
        Some(time) => format!("{remote} was last fetched {}", ago(time)),
        None => format!("{remote} was never fetched in this clone"),
    }
}

/// `list`'s first line, where a forge's remote is configured: its branches
/// are read as this clone last fetched them.
pub fn fetched(remote: &str, fetched: Option<SystemTime>) -> String {
    format!(
        "Read from this clone's branches, local and of {remote}; {}. Nothing here fetches: `git fetch {remote}` brings the latest.\n",
        fetch_age(remote, fetched)
    )
}

/// The lines under `show`'s title: where the record was read from, and
/// what other branches propose for it.
fn whence(whence: &Whence) -> String {
    let mut text = format!("Read from: {}", whence.from);
    if let Some(fetched) = whence.fetched {
        let remote = whence.from.split('/').next().unwrap_or_default();
        text += &format!(" ({})", fetch_age(remote, fetched));
    }
    if whence.proposed {
        text += ", where it is proposed: it is not on the base until that branch merges";
    }
    if whence.diverged {
        text +=
            ". The local branch and the remote-tracking one have diverged, and this one was read";
    }
    text += "\n";
    if !whence.also_added.is_empty() {
        text += &format!("Also added on: {}\n", whence.also_added.join(", "));
    }
    if !whence.changes.is_empty() {
        let changes: Vec<String> = (whence.changes.iter())
            .map(|(on, status)| match status {
                Some(status) => format!("{on} ({status} there)"),
                None => format!("{on} (removed there)"),
            })
            .collect();
        text += &format!(
            "Changed on branches not yet merged: {}\n",
            changes.join(", ")
        );
    }
    if let Some(unpublished) = &whence.unpublished {
        text += unpublished;
    }
    text
}

/// Where an item stands: who holds it, what blocks it, and whether a
/// thread could start it now.
fn standing(entry: &Entry) -> Vec<String> {
    let mut facts = Vec::new();
    if STARTED.contains(&entry.status) {
        facts.push(holder(entry));
    }
    if entry.status == Status::InReview {
        facts.push(match &entry.reviewer {
            Some(reviewer) => format!("review held by {reviewer}"),
            None => "no reviewer yet".into(),
        });
    }
    let waiting: Vec<String> = (entry.waiting_on().iter())
        .map(|(id, s)| match s {
            Some(status) => format!("{id}, which is {}", status_name(*status)),
            None => format!("{id}, which is not found"),
        })
        .collect();
    if !waiting.is_empty() {
        facts.push(format!("blocked by {}", waiting.join(", and ")));
    }
    if let Some(time) = &entry.approved {
        facts.push(format!("approved since {time}"));
    }
    if entry.needs_human() && PLANNED.contains(&entry.status) {
        facts.push("waits on a human".into());
    } else if entry.startable() {
        facts.push("can start now".into());
    }
    facts
}

/// One item on one line: its ID, where it stands, and its title. `status`
/// leaves out the status where a heading gives it.
fn line(entry: &Entry, status: bool) -> String {
    let mut facts = Vec::new();
    if status {
        facts.push(status_name(entry.status).to_string());
    }
    facts.push(priority_name(entry.priority).to_string());
    facts.extend(standing(entry));
    if let Some(on) = &entry.on {
        facts.push(format!("on {on}"));
    }
    format!("{} ({}): {}\n", entry.id, facts.join(", "), entry.title)
}

/// What a mission is shown as: proposed while its record is open on a
/// branch that has not merged, or is the old `draft`; agreed, or in
/// progress, once it is open on the base; done; or dropped, wherever it is
/// read from.
pub fn shown_as(status: MissionStatus, proposed: bool, in_progress: bool) -> &'static str {
    match status {
        MissionStatus::Done => "done",
        MissionStatus::Dropped => "dropped",
        MissionStatus::Draft => "proposed",
        MissionStatus::Open if proposed => "proposed",
        MissionStatus::Open if in_progress => "in progress",
        MissionStatus::Open => "agreed",
    }
}

/// A mission's status, whether it is in progress, its lead, and the
/// branch it is proposed on.
fn mission_line(dir: &MissionDir, in_progress: bool, on: Option<String>) -> String {
    let mission = &dir.mission;
    let shown = shown_as(mission.status, on.is_some(), in_progress);
    let mut facts = vec![match &on {
        Some(on) => format!("{shown} on {on}"),
        None => shown.to_string(),
    }];
    if let Some(lead) = &mission.lead {
        facts.push(format!("led by {lead}"));
    }
    format!("{} ({}): {}\n", mission.id, facts.join(", "), mission.title)
}

fn ready_line(ready: Option<&Ready>) -> String {
    match ready {
        Some(ready) => format!("Made ready by {} at {}\n", ready.by, ready.at),
        None => "Not made ready yet\n".into(),
    }
}

fn labels(labels: &[String]) -> String {
    match labels.is_empty() {
        true => "none".into(),
        false => labels.join(", "),
    }
}

/// A mission's items under a heading for each status, each with what
/// holds or blocks it; a ready item's other unmet conditions follow it,
/// and with `needs`, as a lead is shown them, what a started item's holder
/// still needs.
fn by_status(items: &[&Entry], needs: bool) -> String {
    let mut text = String::new();
    for status in LIFECYCLE {
        let at: Vec<&&Entry> = items.iter().filter(|e| e.status == status).collect();
        if at.is_empty() {
            continue;
        }
        let name = status_name(status);
        text += &format!("  {}{}:\n", name[..1].to_uppercase(), &name[1..]);
        for entry in at {
            text += &format!("    {}", line(entry, false));
            let other = entry
                .unmet
                .iter()
                .filter(|c| !matches!(c, Condition::Blocked { .. }));
            for unmet in other.filter(|_| status == Status::Ready) {
                text += &format!("    {}\n", condition(&entry.id, unmet));
            }
            for need in entry.needs.iter().filter(|_| needs) {
                text += &format!("    {}\n", condition(&entry.id, need));
            }
        }
    }
    match text.is_empty() {
        true => "  None yet.\n".into(),
        false => text,
    }
}

/// What a later round made of a rebutted finding.
fn judged(judged: &Judged) -> String {
    match judged {
        Judged::Kept => "kept".into(),
        Judged::Withdrawn => "withdrawn".into(),
        Judged::Fixed => "fixed".into(),
        Judged::Lowered(Recommendation::NoAction) => "kept at no action".into(),
        Judged::Lowered(_) => "kept at defer".into(),
        Judged::Decided(Outcome::Defer) => "decided: defer".into(),
        Judged::Decided(Outcome::NoAction) => "decided: no action".into(),
    }
}

/// Each disagreement of `disputes`, two lines for a rebuttal and one for a
/// split plan round, indented under a heading the caller gives.
fn disputes(disputes: &Disputes) -> String {
    let mut text = String::new();
    for rebuttal in &disputes.rebuttals {
        let next = match rebuttal.judged.as_slice() {
            [] => "not yet reviewed".to_string(),
            judged => (judged.iter())
                .map(|(name, j)| format!("{} in {name}", self::judged(j)))
                .collect::<Vec<_>>()
                .join(", "),
        };
        text += &format!(
            "  {}/{} ({next}): {}\n    Rebutted: {}\n",
            rebuttal.item, rebuttal.finding, rebuttal.title, rebuttal.fact
        );
    }
    for split in &disputes.plans {
        let reviews: Vec<String> = (split.reviews.iter())
            .map(|(name, by, model, verdict)| {
                let by = by.as_ref().map_or("nobody".to_string(), Thread::to_string);
                let model = model.as_ref().map_or(String::new(), |m| format!(", {m}"));
                format!("{name} {} by {by}{model}", verdict_name(*verdict))
            })
            .collect();
        text += &format!(
            "  {} plan, round {}: {}\n",
            split.mission,
            split.round,
            reviews.join("; ")
        );
    }
    text
}

/// `list --disputed`, of every mission or of `mission`.
pub fn disputed(found: &Disputes, mission: Option<&str>) -> String {
    let of = mission.map_or(String::new(), |id| format!(" in mission {id}"));
    if found.is_empty() {
        return format!("Nothing is disputed{of}: no finding was rebutted, and no plan-review round's reviewers disagreed.\n");
    }
    format!("Disputed{of}: each rebutted finding with what the next round made of it, and each plan-review round whose reviewers' verdicts differ.\n{}", disputes(found))
}

/// The handoff for each item of `items` a thread could start now, other
/// than one waiting on a human.
fn start_threads(items: &[&Entry]) -> String {
    let tasks: Vec<String> = items
        .iter()
        .filter(|e| e.startable() && !e.needs_human())
        .map(|e| format!("Run atelier start {} and follow what it prints.", e.id))
        .collect();
    let (who, then) = match tasks.len() {
        0 => return "No item can start now.\n".into(),
        1 => ("An implementer", "Then wait for it to finish"),
        _ => ("Each implementer", "Then wait for each to finish"),
    };
    handoff(who, &tasks, &format!("{then}, and run `atelier status`."))
}

/// `show` on an item: its fields, its sections as written, and each review
/// with its findings. `open` names the fix-now findings still open.
pub fn item(
    dir: &ItemDir,
    whence: &Whence,
    entry: &Entry,
    mission: Option<&(String, Option<Status>)>,
    findings: &[Finding],
    open: &[Finding],
) -> String {
    let item = &dir.item;
    let mut status = vec![status_name(item.status).to_string()];
    status.extend(standing(entry));
    let mission = match mission {
        Some((id, Some(status))) => format!("{id} ({})", status_name(*status)),
        Some((id, None)) => format!("{id} (not in the base's last commit)"),
        None => "none".into(),
    };
    let blockers: Vec<String> = entry
        .blockers
        .iter()
        .map(|(id, s)| format!("{id} ({})", s.map_or("not found", status_name)))
        .collect();
    let mut text = format!(
        "Item {}: {}\n{}Status: {}\nMission: {mission}\nLabels: {}\nPriority: {}\nBlocked by: {}\n{}",
        item.id,
        item.title,
        self::whence(whence),
        status.join(", "),
        labels(&item.labels),
        priority_name(item.priority),
        labels(&blockers),
        ready_line(item.ready.as_ref()),
    );
    text += &format!("\n{}\n", item.body.trim());
    if dir.reviews.is_empty() {
        return text + "\nReviews: none yet.\n";
    }
    text += "\nReviews:\n";
    for review in &dir.reviews {
        text += &review_line(review);
        for finding in findings.iter().filter(|f| f.name.review == review.name) {
            let state = match finding.recommendation {
                atelier_core::model::Recommendation::FixNow if open.contains(finding) => {
                    "fix now, open"
                }
                atelier_core::model::Recommendation::FixNow => "fix now, closed",
                atelier_core::model::Recommendation::Defer => "defer",
                atelier_core::model::Recommendation::NoAction => "no action",
            };
            let settled = finding.settlement.as_ref().map(settled);
            let settled = settled.map_or(String::new(), |s| format!(", {s}"));
            text += &format!(
                "    {} ({state}{settled}): {}\n",
                finding.name, finding.title
            );
        }
    }
    text
}

fn review_line(file: &ReviewFile) -> String {
    let review = &file.review;
    let by = review
        .reviewers
        .holder()
        .map_or("nobody".to_string(), Thread::to_string);
    let model = review
        .model
        .as_ref()
        .map_or(String::new(), |m| format!(", {m}"));
    let verdict = review.verdict.map_or("no verdict yet", verdict_name);
    let commit = review.commit.get(..7).unwrap_or(&review.commit);
    format!(
        "  Round {}, review {} by {by}{model}, of commit {commit}: {verdict}\n",
        review.round, file.name
    )
}

/// `show` on a mission: its fields, its Outcome, Success criteria and
/// Result, its items by status, its disagreements, and its plan reviews.
pub fn mission(
    dir: &MissionDir,
    whence: &Whence,
    in_progress: bool,
    items: &[&Entry],
    disputed: &Disputes,
    reviews: &[ReviewFile],
) -> String {
    let mission = &dir.mission;
    let lead = mission
        .lead
        .as_ref()
        .map_or("nobody".to_string(), Thread::to_string);
    let planners: Vec<String> = mission.planners.0.iter().map(Thread::to_string).collect();
    let planners = match planners.is_empty() {
        true => "none recorded".to_string(),
        false => planners.join(", "),
    };
    let mut text = format!(
        "Mission {}: {}\n{}Status: {}\nLead: {lead}\nPlanners: {planners}\nLabels: {}\n",
        mission.id,
        mission.title,
        self::whence(whence),
        shown_as(mission.status, whence.proposed, in_progress),
        labels(&mission.labels),
    );
    for heading in ["Outcome", "Success criteria", "Result"] {
        let content = dir.section(heading).unwrap_or_default().trim();
        if !content.is_empty() {
            text += &format!("\n## {heading}\n\n{content}\n");
        }
    }
    text += &format!("\nItems:\n{}", by_status(items, false));
    text += "\nDisputed:\n";
    text += &match disputed.is_empty() {
        true => "  None.\n".to_string(),
        false => disputes(disputed),
    };
    text += "\nPlan reviews:\n";
    if reviews.is_empty() {
        text += "  None.\n";
    }
    for review in reviews {
        text += &review_line(review);
    }
    text
}

/// A stop of agreed mission `id` written on the unmerged branches `on`.
pub fn stop_waits(id: &str, on: &[String]) -> String {
    format!(
        "Stop not published: mission {id} is dropped on {}, which has not merged, so the stop stops nothing yet. It is published once that branch merges.\n",
        on.join(", ")
    )
}

/// A stop of mission `id`'s proposed plan written on its branch `branch`
/// and not pushed to `remote`, or with none, not committed.
pub fn plan_stop_unpublished(id: &str, branch: &str, remote: Option<&str>) -> String {
    match remote {
        Some(remote) => format!(
            "Stop not published: mission {id}'s plan is dropped here, and {remote}'s copy of {branch} is not, so other clones still read it as proposed. Push it with `git push {remote} {branch}`, and close its pull request with `gh pr close {branch}`.\n"
        ),
        None => format!(
            "Stop not published: mission {id}'s plan is dropped in this checkout and not committed on {branch}. Commit it.\n"
        ),
    }
}

pub fn unknown(id: &str, base: &str) -> String {
    format!(
        "No item or mission {id} is in {base}'s last commit or on a branch named {id}. Nothing changed.\n`atelier list` lists the open work.\n"
    )
}

/// `list`: each group under its mission, its items under their labels.
/// `on` gives the branch a proposed mission is read from.
pub fn list(groups: &[Group], filter: &Filter, on: &dyn Fn(&str) -> Option<String>) -> String {
    if groups.is_empty() {
        return match filter.done {
            true => "Nothing finished matches.\n".into(),
            false => "No open work matches.\n".into(),
        };
    }
    groups_text(groups, on)
}

fn groups_text(groups: &[Group], on: &dyn Fn(&str) -> Option<String>) -> String {
    let mut text = String::new();
    for group in groups {
        text += &match (group.mission, &group.id) {
            (Some(dir), id) => {
                let on = id.as_deref().and_then(on);
                format!("Mission {}", mission_line(dir, group.in_progress, on))
            }
            (None, Some(id)) => format!("Mission {id} (not in the base's last commit):\n"),
            (None, None) => "In no mission:\n".into(),
        };
        let mut last: Option<Vec<String>> = None;
        for entry in &group.items {
            let set = entry.label_set();
            if last.as_ref() != Some(&set) {
                text += &match set.is_empty() {
                    true => "  No label:\n".to_string(),
                    false => format!("  Labelled {}:\n", set.join(", ")),
                };
                last = Some(set);
            }
            text += &format!("    {}", line(entry, true));
        }
    }
    text
}

/// What is proposed on branches not yet merged: the proposed missions and
/// items, by mission, and the record changes. With `only`, as `list
/// --proposed` asks, it says so when nothing is.
pub fn proposed(
    groups: &[Group],
    changes: &[Change],
    only: bool,
    on: &dyn Fn(&str) -> Option<String>,
) -> String {
    if groups.is_empty() && changes.is_empty() {
        return match only {
            true => "Nothing matching is proposed on a branch not yet merged.\n".into(),
            false => String::new(),
        };
    }
    let mut text = String::new();
    if !groups.is_empty() {
        text += "\nProposed, on branches not yet merged:\n";
        text += &groups_text(groups, on);
    }
    if !changes.is_empty() {
        text +=
            "\nRecord changes on branches not yet merged, which take effect when each merges:\n";
        for change in changes {
            let after = match change.after {
                Some(after) if after == change.before => {
                    format!("changed there, still {after}")
                }
                Some(after) => format!("{} on the base, {after} there", change.before),
                None => "removed there".into(),
            };
            text += &format!(
                "  {} (on {}: {after}): {}\n",
                change.id, change.on, change.title
            );
        }
    }
    text
}

/// The missions dropped on their branch, not merged: each with its
/// reason and its branch.
pub fn dropped_missions(missions: &[(String, String, Option<String>, String)]) -> String {
    if missions.is_empty() {
        return String::new();
    }
    let mut text = "\nMissions dropped on their branch, not merged:\n".to_string();
    for (id, title, why, on) in missions {
        let why = why.as_deref().unwrap_or("no reason given");
        text += &format!("  {id} (dropped on {on}: {why}): {title}\n");
    }
    text
}

/// The items dropped on their branch whose drop has not reached the base.
pub fn unreached(items: &[(String, String, Option<String>)]) -> String {
    if items.is_empty() {
        return String::new();
    }
    let mut text = "Dropped on their branch, not yet on the base:\n".to_string();
    for (id, title, why) in items {
        let why = why.as_deref().unwrap_or("no reason given");
        text += &format!(
            "  {id} (dropped on its branch; the drop has not reached the base: {why}): {title}\n"
        );
    }
    text
}

/// `list --deferred`: each finding, as `<item>/<finding>`, with its title.
pub fn deferred(findings: &[(String, Finding)]) -> String {
    let Some((id, first)) = findings.first() else {
        return "No deferred finding of a done item is left to settle.\n".into();
    };
    let mut text = "Deferred findings of done items that nobody has settled:\n".to_string();
    for (id, finding) in findings {
        text += &format!("  {id}/{}: {}\n", finding.name, finding.title);
    }
    let name = format!("{id}/{}", first.name);
    text + &format!(
        "Settle each once: take it into a new item, or drop it with a reason.\nThen: atelier new \"<title>\" --from {name}, or atelier drop {name} --reason \"<why>\"\n"
    )
}

/// What `status` adds on the base: the missions in progress and the items
/// waiting on a human.
pub fn overview(overview: &Overview) -> String {
    let mut text = String::new();
    if !overview.in_progress.is_empty() {
        text += "Missions in progress:\n";
        for dir in &overview.in_progress {
            text += &format!("  {}", mission_line(dir, true, None));
        }
    }
    if !overview.waiting.is_empty() {
        text += "Waiting on a human (labelled needs-human):\n";
        for entry in &overview.waiting {
            text += &format!("  {}", line(entry, true));
        }
    }
    text
}

/// What `status` tells a mission's lead: each mission it leads, where each
/// item stands, what is disputed in it, and a handoff for each item that
/// can start now.
pub fn led(led: &[(MissionDir, Vec<Entry>, Disputes)]) -> String {
    let mut text = String::new();
    for (dir, items, disputed) in led {
        let items: Vec<&Entry> = items.iter().collect();
        text += &format!(
            "You lead mission {}: {}\n{}",
            dir.mission.id,
            dir.mission.title,
            by_status(&items, true),
        );
        if !disputed.is_empty() {
            text += "Disputed in this mission (nothing waits on these; a lead steps in by moving disputed work out of an item's Scope into an item of its own):\n";
            text += &disputes(disputed);
        }
        text += &start_threads(&items);
    }
    text
}

/// What `lead` says once the caller leads the mission: who led it before,
/// where each item stands, and a handoff for each that can start now.
pub fn lead(
    mission: &Mission,
    before: Option<&Thread>,
    caller: &Thread,
    items: &[&Entry],
) -> String {
    let said = match before {
        Some(thread) if thread == caller => {
            format!(
                "You already lead mission {}: {}\n",
                mission.id, mission.title
            )
        }
        Some(thread) => format!(
            "You now lead mission {}: {}\nThread {thread} led it before.\n",
            mission.id, mission.title
        ),
        None => format!(
            "You now lead mission {}: {}\nNobody led it before.\n",
            mission.id, mission.title
        ),
    };
    format!(
        "{said}Its items:\n{}{}",
        by_status(items, true),
        start_threads(items)
    )
}

pub fn lead_not_ready(id: &str, status: Status) -> String {
    format!(
        "`atelier lead {id}` refused; nothing changed.\nMission {id} is {}, and `atelier lead` runs on a mission that is open. Nothing is left to lead on it.\n",
        mission_status_name(status)
    )
}

pub fn lead_unknown(id: &str) -> String {
    format!(
        "`atelier lead {id}` refused; nothing changed.\nNo mission {id} is in this checkout. `atelier list` shows the missions.\n"
    )
}
