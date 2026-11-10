//! What `show`, `list` and `lead` print, and the missions `status` lists.
//! The wording lives here, beside `output.rs`, so it is cheap to change.

use crate::output::{condition, handoff, holder, priority_name, status_name, verdict_name};
use atelier_app::acts::{Condition, PLANNED, STARTED};
use atelier_app::{Entry, Filter, Group, Overview, LIFECYCLE};
use atelier_core::{Decision, Finding, Mission, Ready, Status, Thread};
use atelier_records::{ItemDir, MissionDir, ReviewFile};

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
    format!("{} ({}): {}\n", entry.id, facts.join(", "), entry.title)
}

/// A mission's status, whether it is in progress, and its lead.
fn mission_line(dir: &MissionDir, in_progress: bool) -> String {
    let mission = &dir.mission;
    let mut facts = vec![status_name(mission.status.status()).to_string()];
    if in_progress {
        facts.push("in progress".into());
    }
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
/// holds or blocks it; a ready item's other unmet conditions follow it.
fn by_status(items: &[&Entry]) -> String {
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
        }
    }
    match text.is_empty() {
        true => "  None yet.\n".into(),
        false => text,
    }
}

/// A question waiting on item `id`, with the command that answers it.
fn waiting(id: &str, question: &str) -> String {
    format!(
        "  {id}: {question}\n    Answer it in the checkout on branch {id}: atelier decide {id} \"<answer>\"\n"
    )
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
        "Item {}: {}\nStatus: {}\nMission: {mission}\nLabels: {}\nPriority: {}\nBlocked by: {}\n{}",
        item.id,
        item.title,
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
                atelier_core::Recommendation::FixNow if open.contains(finding) => "fix now, open",
                atelier_core::Recommendation::FixNow => "fix now, closed",
                atelier_core::Recommendation::Defer => "defer",
                atelier_core::Recommendation::NoAction => "no action",
            };
            text += &format!("    {} ({state}): {}\n", finding.name, finding.title);
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
/// Result, its items by status, its items' questions, unanswered first,
/// and its plan reviews.
pub fn mission(
    dir: &MissionDir,
    in_progress: bool,
    items: &[&Entry],
    questions: &[(String, Decision)],
    reviews: &[ReviewFile],
) -> String {
    let mission = &dir.mission;
    let progress = if in_progress { ", in progress" } else { "" };
    let lead = mission
        .lead
        .as_ref()
        .map_or("nobody".to_string(), Thread::to_string);
    let mut text = format!(
        "Mission {}: {}\nStatus: {}{progress}\nLead: {lead}\nLabels: {}\n{}",
        mission.id,
        mission.title,
        status_name(mission.status.status()),
        labels(&mission.labels),
        ready_line(mission.ready.as_ref()),
    );
    for heading in ["Outcome", "Success criteria", "Result"] {
        let content = dir.section(heading).unwrap_or_default().trim();
        if !content.is_empty() {
            text += &format!("\n## {heading}\n\n{content}\n");
        }
    }
    text += &format!("\nItems:\n{}", by_status(items));
    let mut questions: Vec<&(String, Decision)> = questions.iter().collect();
    questions.sort_by_key(|(_, d)| d.answer.is_some());
    text += "\nQuestions:\n";
    if questions.is_empty() {
        text += "  None.\n";
    }
    for (id, decision) in questions {
        text += &match &decision.answer {
            None => waiting(id, &decision.question),
            Some(answer) => format!(
                "  {id}: {} Answer by {}: {}\n",
                decision.question, answer.by, answer.text
            ),
        };
    }
    text += "\nPlan reviews:\n";
    if reviews.is_empty() {
        text += "  None.\n";
    }
    for review in reviews {
        text += &review_line(review);
    }
    text
}

pub fn unknown(id: &str, base: &str) -> String {
    format!(
        "No item or mission {id} is in {base}'s last commit or on a branch named {id}. Nothing changed.\n`atelier list` lists the open work.\n"
    )
}

/// `list`: each group under its mission, its items under their labels.
pub fn list(groups: &[Group], filter: &Filter) -> String {
    if groups.is_empty() {
        return match filter.done {
            true => "Nothing finished matches.\n".into(),
            false => "No open work matches.\n".into(),
        };
    }
    let mut text = String::new();
    for group in groups {
        text += &match (group.mission, &group.id) {
            (Some(dir), _) => format!("Mission {}", mission_line(dir, group.in_progress)),
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

/// What `status` adds on the base: the missions in progress and the items
/// waiting on a human.
pub fn overview(overview: &Overview) -> String {
    let mut text = String::new();
    if !overview.in_progress.is_empty() {
        text += "Missions in progress:\n";
        for dir in &overview.in_progress {
            text += &format!("  {}", mission_line(dir, true));
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
/// item stands, and a handoff for each item that can start now.
pub fn led(led: &[(MissionDir, Vec<Entry>)]) -> String {
    let mut text = String::new();
    for (dir, items) in led {
        let items: Vec<&Entry> = items.iter().collect();
        text += &format!(
            "You lead mission {}: {}\n{}",
            dir.mission.id,
            dir.mission.title,
            by_status(&items),
        );
        if items.iter().any(|e| !e.questions.is_empty()) {
            text += "Questions waiting on a decider, oldest first on each item:\n";
            for entry in &items {
                for question in &entry.questions {
                    text += &waiting(&entry.id, question);
                }
            }
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
        by_status(items),
        start_threads(items)
    )
}

pub fn lead_not_ready(id: &str, status: Status) -> String {
    let way = match status {
        Status::Draft => {
            format!("A planner makes it ready with: atelier ready {id} --by <name>")
        }
        _ => "Nothing is left to lead on it.".into(),
    };
    format!(
        "`atelier lead {id}` refused; nothing changed.\nMission {id} is {}, and `atelier lead` runs on a mission that is ready. {way}\n",
        status_name(status)
    )
}

pub fn lead_unknown(id: &str) -> String {
    format!(
        "`atelier lead {id}` refused; nothing changed.\nNo mission {id} is in this checkout. `atelier list` shows the missions.\n"
    )
}
