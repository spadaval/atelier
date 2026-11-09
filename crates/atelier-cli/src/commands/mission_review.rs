//! `atelier debrief` (atelier-rx22, atelier-nexd, atelier-8bil): a mission's
//! after-action report, rendered from one view model as dense Markdown for
//! agents (ADR 0022); a human reads the same view on the site
//! (`atelier serve`), whose page the footer names when the site's address is
//! set. It tells the problem, what the mission set out to do and what it did
//! in time order, then where it stands: results, how it was validated, plan
//! against actual, what needs the human, what is left and what was found
//! along the way.

use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use atelier_app::criteria::{CriterionMark, CriterionProof};
use atelier_app::decisions::{MalformedDecision, NoteDecision};
use atelier_app::mission_review::{
    first_sentence, Ask, Change, Debrief, Event, OpenEntry, ReviewSummary, ValidationRun, WorkEntry,
};
use atelier_app::record_index::RecordIndex;
use atelier_core::{CloseReport, EvidenceResult, ReportPart};

use crate::human_output::{
    local_date, print_page, render_footer_section, render_lines_section, render_metadata,
    render_row_wrapped, render_text_section, short_commit, FooterAction, ItemLine, RenderContext,
};

pub fn debrief(index: &RecordIndex, mission_ref: &str, quiet: bool) -> Result<()> {
    let id = crate::commands::issue::resolve_id(index, mission_ref)?;
    let issue = index.require_issue(&id)?;
    if issue.issue_type != "mission" {
        bail!(
            "{id} is {}, not a mission; debrief reviews a mission",
            atelier_app::record_ref::with_article(&issue.issue_type)
        );
    }
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let repo_root = crate::commands::workflow::repo_root()?;
    let mut read = atelier_app::git_freshness::ReadContext::new(&repo_root);
    let view = atelier_app::mission_review::debrief(index, policy.as_ref(), &mut read, &id)?;
    if quiet {
        println!("{}", quiet_counts(&view));
        return Ok(());
    }
    let context = RenderContext::for_stdout();
    print_page(
        &format!("Debrief: {}", mission_line(&view)),
        render_markdown(&view, site_page(&repo_root, &id, "debrief"), context),
        context,
    );
    Ok(())
}

/// `criteria=<n> proven=<n> failing=<n> unproven=<n> items=<n> decisions=<n>
/// deviations=<n> follow_ups=<n>`, where follow-ups count the open work found
/// along the way.
fn quiet_counts(view: &Debrief) -> String {
    let marks = |mark: CriterionMark| {
        view.criteria
            .proofs
            .iter()
            .filter(|proof| proof.mark == mark)
            .count()
    };
    let items = view
        .work
        .iter()
        .flat_map(|entry| {
            std::iter::once(&entry.id).chain(entry.children.iter().map(|child| &child.id))
        })
        .chain(
            view.left
                .iter()
                .chain(&view.found)
                .chain(&view.dropped)
                .map(|entry| &entry.id),
        )
        .chain(view.asks.iter().filter_map(|ask| match ask {
            Ask::Decision(id) | Ask::NeedsHuman(id) => Some(id),
            _ => None,
        }))
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    format!(
        "criteria={} proven={} failing={} unproven={} items={items} decisions={} deviations={} follow_ups={}",
        view.criteria.proofs.len(),
        marks(CriterionMark::Proven),
        marks(CriterionMark::Failing),
        marks(CriterionMark::Unproven),
        view.decisions.notes.len() + view.decisions.records.len(),
        deviation_count(view),
        view.found.len()
    )
}

/// Departures from the plan: reported deviations, derived changes, scope
/// added after the start, and dropped scope.
fn deviation_count(view: &Debrief) -> usize {
    view.work
        .iter()
        .map(|entry| {
            entry.changes.len()
                + reports(entry)
                    .filter(|report| reported(*report, ReportPart::Deviations).is_some())
                    .count()
        })
        .sum::<usize>()
        + view
            .left
            .iter()
            .chain(&view.found)
            .map(|entry| entry.changes.len())
            .sum::<usize>()
        + view.timeline.added_after_start.len()
        + view.dropped.len()
}

/// The decisions recorded on one issue, and its malformed
/// decision notes.
fn decisions_on<'a>(
    view: &'a Debrief,
    id: &str,
) -> (Vec<&'a NoteDecision>, Vec<&'a MalformedDecision>) {
    let notes = view
        .decisions
        .notes
        .iter()
        .filter(|note| note.issue_id == id)
        .collect::<Vec<_>>();
    let malformed = view
        .decisions
        .malformed
        .iter()
        .filter(|note| note.issue_id == id)
        .collect();
    (notes, malformed)
}

fn decision_text(note: &NoteDecision) -> String {
    format!(
        "{} | because {}",
        note.decision.choice, note.decision.reason
    )
}

fn malformed_text(note: &MalformedDecision) -> String {
    format!("{} #{} {}", note.issue_id, note.number, note.defect)
}

/// An open, dropped or asked item's decisions as indented Markdown lines.
fn decision_lines(view: &Debrief, id: &str, context: RenderContext) -> Vec<String> {
    let (notes, malformed) = decisions_on(view, id);
    let mut lines = Vec::new();
    for (label, text) in notes
        .iter()
        .map(|note| ("Decided", decision_text(note)))
        .chain(
            malformed
                .iter()
                .map(|note| ("⚠ Malformed decision note", malformed_text(note))),
        )
    {
        for line in render_row_wrapped(context, &format!("{label}: {text}")) {
            lines.push(format!("  {line}"));
        }
    }
    lines
}

fn mission_line(view: &Debrief) -> String {
    let mission = &view.mission;
    ItemLine::new(
        &mission.id,
        &mission.status,
        &mission.priority,
        &mission.title,
    )
    .issue_type("mission")
    .state(view.status_category.as_deref(), None)
    .text()
}

/// One review event on the mission, as the brief shows it.
fn event_line(event: &atelier_app::mission_review::MissionEvent) -> String {
    format!(
        "{} {} {} by {} at {} | {}",
        event.event,
        event.id,
        local_date(event.created_at),
        event.identity,
        short_commit(&event.commit),
        event.summary
    )
}

const NO_PLAN_APPROVAL: &str = "No plan approval recorded.";

/// The brief's next step, as a label and a command: record the human's plan
/// approval until one is recorded, then move the approved plan on
/// (atelier-mg7w): out of its initial status, or into its ready work.
fn brief_next_step(
    policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
    mission: &atelier_core::Issue,
    approvals: usize,
) -> (&'static str, String) {
    if approvals == 0 {
        return (
            "Record the human's plan approval",
            format!(
                "atelier review approve {} --identity \"<human>\" \"<summary>\"",
                mission.id
            ),
        );
    }
    let initial = policy
        .and_then(|policy| policy.workflow_for_issue_type(&mission.issue_type).ok())
        .is_some_and(|workflow| workflow.initial_status == mission.status);
    if initial {
        (
            "Move the approved plan on",
            format!("atelier issue transition {}", mission.id),
        )
    } else {
        ("List the work ready to start", "atelier work ready".into())
    }
}

/// The site's page for a mission's `brief` or `debrief`, when this user has
/// set the site's address in `site-url` under `$XDG_CONFIG_HOME/atelier` (else
/// `~/.config/atelier`), the one place every repository on the host reads it
/// (atelier-59fz). With no address set, or no `project_slug` to name the
/// repository by, there is no page to name.
fn site_page(repo_root: &Path, mission_id: &str, page: &str) -> Option<String> {
    let site = std::fs::read_to_string(site_url_file()?).ok()?;
    let site = site.lines().next()?.trim().trim_end_matches('/');
    if site.is_empty() {
        return None;
    }
    let slug = atelier_app::project_config::ProjectConfig::load(repo_root)
        .ok()?
        .project_slug;
    Some(format!("{site}/{slug}/mission/{mission_id}/{page}"))
}

fn site_url_file() -> Option<PathBuf> {
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|dir| dir.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|home| Path::new(&home).join(".config")))?;
    Some(config.join("atelier").join("site-url"))
}

/// A report part with something to say: present, and not just "None".
fn reported(report: Option<&CloseReport>, part: ReportPart) -> Option<&str> {
    report?.part(part).filter(|text| {
        !matches!(
            text.trim()
                .trim_end_matches('.')
                .to_ascii_lowercase()
                .as_str(),
            "none" | "n/a"
        )
    })
}

fn outcome(report: Option<&CloseReport>) -> &str {
    report
        .map(|report| report.outcome.as_str())
        .unwrap_or_default()
}

/// The entry's report, then each done child's.
fn reports(entry: &WorkEntry) -> impl Iterator<Item = Option<&CloseReport>> {
    std::iter::once(entry.report.as_ref())
        .chain(entry.children.iter().map(|child| child.report.as_ref()))
}

/// Whether the entry, or a child shown in it, departed from its plan.
fn changed_course(entry: &WorkEntry) -> bool {
    !entry.changes.is_empty()
        || reports(entry).any(|report| reported(report, ReportPart::Deviations).is_some())
}

fn criterion_word(mark: CriterionMark) -> &'static str {
    match mark {
        CriterionMark::Proven => "✓ proven",
        CriterionMark::Failing => "⚠ failing",
        CriterionMark::Unproven => "unproven",
    }
}

/// The deciding receipt and its age: code commits up to the mission's close,
/// or since the receipt while the mission is open.
fn proof_cue(proof: &CriterionProof, closed: bool) -> Option<String> {
    let receipt = proof.deciding()?;
    let mut cue = format!(
        "{} {} at {}",
        receipt.evidence_id,
        receipt.result.as_str(),
        short_commit(&receipt.commit)
    );
    if let Some(age) = proof_age(proof, closed) {
        cue.push_str(&format!(" | {age}"));
    }
    Some(cue)
}

fn proof_age(proof: &CriterionProof, closed: bool) -> Option<String> {
    let count = proof
        .deciding()?
        .code_commits_since
        .filter(|count| *count > 0)?;
    Some(commits_since(count, closed))
}

fn commits_since(count: usize, closed: bool) -> String {
    format!(
        "{} {}",
        atelier_core::count_noun(count, "code commit"),
        if closed { "before close" } else { "since" }
    )
}

/// A derived change in words; the HTML names the receipt by a chip.
fn change_text(change: &Change) -> String {
    match change {
        Change::Reopened(time) => format!("Reopened on {} after a done claim.", local_date(*time)),
        Change::NonPassEvidence {
            evidence,
            result,
            summary,
        } => format!(
            "Newest proof {evidence} is {}: {}",
            result.as_str(),
            first_sentence(summary)
        ),
        Change::TrivialExemption { summary } => {
            format!("Review waived as trivial: {}", first_sentence(summary))
        }
        Change::DoneWhenEdited(edit) => crate::human_output::done_when_edit(edit),
    }
}

/// How review went; `None` for a decision record, which is decided rather
/// than code-reviewed, and for declined work nobody reviewed, which merged
/// nothing (atelier-stnv).
fn review_line(view: &Debrief, entry: &WorkEntry) -> Option<String> {
    let issue = view.refs.get(&entry.id);
    if issue.is_some_and(|issue| issue.issue_type == "decision") {
        return None;
    }
    if !entry.review.recorded {
        if issue.is_some_and(|issue| issue.status == "declined") {
            return None;
        }
        return Some("No review recorded.".into());
    }
    Some(review_summary(&entry.review))
}

fn review_summary(review: &ReviewSummary) -> String {
    let verdict = if review.approved {
        if review.approvers.is_empty() {
            "Approved".to_string()
        } else {
            format!("Approved by {}", review.approvers.join(", "))
        }
    } else if review.trivial {
        "Waived as trivial".to_string()
    } else {
        "Not approved".to_string()
    };
    match finding_counts(review) {
        None => format!("{verdict}; no findings."),
        Some(counts) => format!("{verdict}; {counts}."),
    }
}

/// `7 findings: 3 fixed, 4 deferred`, or `None` without findings.
fn finding_counts(review: &ReviewSummary) -> Option<String> {
    let findings = review.findings();
    if findings == 0 {
        return None;
    }
    let parts = [
        (review.fixed, "fixed"),
        (review.deferred, "deferred"),
        (review.no_action, "left as is"),
        (review.wont_fix, "won't fix"),
        (review.undisposed, "awaiting a decision"),
    ]
    .iter()
    .filter(|(count, _)| *count > 0)
    .map(|(count, word)| format!("{count} {word}"))
    .collect::<Vec<_>>();
    Some(format!(
        "{}: {}",
        atelier_core::count_noun(findings, "finding"),
        parts.join(", ")
    ))
}

fn ask_line(view: &Debrief, ask: &Ask) -> (String, String) {
    let summary = |id: &str| {
        view.refs
            .get(id)
            .map(|issue| issue.summary.clone())
            .unwrap_or_default()
    };
    match ask {
        Ask::Decision(id) => (format!("Decide: {}", view.title_of(id)), summary(id)),
        Ask::NeedsHuman(id) => (view.title_of(id).to_string(), summary(id)),
        Ask::Failing(slug) => (
            format!("Criterion {slug} is failing"),
            criterion_text(view, slug),
        ),
        Ask::Unproven(slugs) => (
            format!(
                "{} without passing proof: {}",
                if slugs.len() == 1 {
                    "1 criterion".to_string()
                } else {
                    format!("{} criteria", slugs.len())
                },
                slugs.join(", ")
            ),
            if slugs.len() == 1 {
                criterion_text(view, &slugs[0])
            } else {
                String::new()
            },
        ),
    }
}

fn criterion_text(view: &Debrief, slug: &str) -> String {
    view.criteria
        .proofs
        .iter()
        .find(|proof| proof.criterion.slug == slug)
        .map(|proof| first_sentence(&proof.criterion.text))
        .unwrap_or_default()
}

/// A criterion's whole text on one line: the plan states it in full, since a
/// reader judges each mark against all of it.
fn full_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The open item that will prove a criterion without proof.
fn awaited_by<'a>(view: &'a Debrief, slug: &str) -> Option<&'a str> {
    view.validation
        .awaiting
        .iter()
        .find(|(awaiting, ..)| awaiting == slug)
        .and_then(|(_, _, by)| by.as_deref())
}

fn last_delivery(view: &Debrief) -> Option<chrono::DateTime<chrono::Utc>> {
    view.work.iter().filter_map(|entry| entry.closed_at).max()
}

fn status_word(view: &Debrief) -> &'static str {
    match (
        view.status_category.as_deref(),
        view.mission.status.as_str(),
    ) {
        (Some("done"), _) => "Done",
        (_, "draft") => "Draft",
        _ => "In progress",
    }
}

/// The one-line verdict both forms open with: criteria proven, then what
/// needs the human, is left, and was found.
fn verdict(view: &Debrief) -> Vec<(String, Option<&'static str>)> {
    let mut parts = Vec::new();
    let total = view.criteria.proofs.len();
    if total > 0 {
        parts.push((
            format!("{} of {total} criteria proven", view.proven()),
            Some("results"),
        ));
    }
    if !view.asks.is_empty() {
        parts.push((
            match view.asks.len() {
                1 => "1 needs your review".to_string(),
                n => format!("{n} need your review"),
            },
            Some("review"),
        ));
    }
    if !view.left.is_empty() {
        parts.push((format!("{} left to do", view.left.len()), Some("left")));
    }
    if !view.found.is_empty() {
        parts.push((
            format!("{} found along the way", view.found.len()),
            Some("found"),
        ));
    }
    parts
}

/// Each criterion a receipt judged, with its result: `S1 pass, S2 fail`.
fn judged(run: &ValidationRun) -> Vec<(String, EvidenceResult)> {
    run.receipts
        .iter()
        .flat_map(|receipt| {
            receipt
                .criteria
                .iter()
                .map(|criterion| (criterion.slug.clone(), criterion.result))
        })
        .collect()
}

fn local_time(time: chrono::DateTime<chrono::Utc>) -> String {
    time.with_timezone(&chrono::Local)
        .format("%Y-%m-%d %H:%M")
        .to_string()
}

fn render_markdown(
    view: &Debrief,
    page: Option<String>,
    context: RenderContext,
) -> Vec<Vec<String>> {
    let closed = view.timeline.closed.is_some();
    let flat = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
    let wrap = |line: String| render_row_wrapped(context, &line);
    let paragraphs = |text: &str| {
        text.split("\n\n")
            .map(flat)
            .filter(|paragraph| !paragraph.is_empty())
            .flat_map(&wrap)
            .collect::<Vec<_>>()
    };
    let item = |id: &str| {
        view.refs
            .get(id)
            .map(|issue| {
                ItemLine::new(&issue.id, &issue.status, "", &issue.title)
                    .issue_type(&issue.issue_type)
                    .state(issue.status_category.as_deref(), None)
                    .text()
                    .replace("  ", " ")
            })
            .unwrap_or_else(|| id.to_string())
    };

    let mut verdict_line = status_word(view).to_lowercase();
    for (part, _) in verdict(view) {
        verdict_line.push_str(&format!(" | {part}"));
    }
    let mut metadata = vec![("Status", verdict_line)];
    if let Some(started) = &view.timeline.started {
        metadata.push(("Started", local_time(started.time)));
    }
    if let Some(closed) = &view.timeline.closed {
        metadata.push(("Closed", local_time(closed.time)));
    } else if let Some(last) = last_delivery(view) {
        metadata.push(("Last delivery", local_time(last)));
    }
    if let Some(strategy) = &view.strategy {
        metadata.push(("Strategy", strategy.clone()));
    }
    if let Some(commit) = &view.generated_from {
        metadata.push(("Generated from", short_commit(commit).to_string()));
    }

    let problem = view.why.as_deref().map(paragraphs).unwrap_or_default();

    let mut intent = paragraphs(&view.outcome);
    if !view.criteria.proofs.is_empty() || !view.criteria.criteria.parse.problems.is_empty() {
        if !intent.is_empty() {
            intent.push(String::new());
        }
        for proof in &view.criteria.proofs {
            intent.extend(wrap(format!(
                "{} {}",
                proof.criterion.slug,
                full_text(&proof.criterion.text)
            )));
        }
        for problem in &view.criteria.criteria.parse.problems {
            intent.extend(wrap(format!("⚠ {problem}")));
        }
    }
    // Decisions on the mission itself bind all of its work.
    let mission_decisions = decision_lines(view, &view.mission.id, context);
    if !mission_decisions.is_empty() {
        intent.push(String::new());
        intent.extend(mission_decisions);
    }
    let planned = view
        .planned
        .iter()
        .flat_map(|(id, parent)| {
            let indent = if parent.is_some() { "  " } else { "" };
            wrap(format!("{indent}{}", item(id)))
        })
        .collect::<Vec<_>>();

    let mut did = Vec::new();
    for timed in &view.events {
        let when = timed.at.map(local_time).unwrap_or_else(|| "undated".into());
        match &timed.event {
            Event::Started => did.extend(wrap(format!("{when} started the mission"))),
            Event::Closed => did.extend(wrap(format!("{when} closed the mission"))),
            Event::Added(id) => did.extend(wrap(format!("{when} added {}", item(id)))),
            Event::Filed(id) => {
                let from = view
                    .found
                    .iter()
                    .find(|entry| &entry.id == id)
                    .and_then(origin);
                let cue = match from {
                    Some((from, how)) if from == view.mission.id => {
                        format!(" | {how} the mission")
                    }
                    Some((from, how)) => format!(" | {how} {from}"),
                    None => String::new(),
                };
                did.extend(wrap(format!("{when} filed {}{cue}", item(id))));
            }
            Event::Dropped(id) => {
                let reason = view
                    .dropped
                    .iter()
                    .find(|entry| &entry.id == id)
                    .and_then(|entry| entry.reason.as_deref())
                    .unwrap_or_default();
                let mut line = format!("{when} dropped {}", item(id));
                if !reason.trim().is_empty() {
                    line.push_str(&format!(" | {}", flat(reason)));
                }
                did.extend(wrap(line));
            }
            Event::Validated(number) => {
                let run = &view.validation.runs[*number];
                let judged = judged(run)
                    .iter()
                    .map(|(slug, result)| format!("{slug} {}", result.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ");
                did.extend(wrap(format!(
                    "{when} validated by {} at {} | {judged}",
                    run.identity,
                    short_commit(&run.commit)
                )));
            }
            Event::MissionReview(number) => {
                let event = &view.mission_events[*number];
                did.extend(wrap(format!(
                    "{when} {} {} by {} at {} | {}",
                    event.event,
                    event.id,
                    event.identity,
                    short_commit(&event.commit),
                    flat(&event.summary)
                )));
            }
            Event::BaseExemption(number) => {
                let event = &view.base_exemptions[*number];
                did.extend(wrap(format!(
                    "{when} base commit {} exempted as trivial ({}) by {} | {}",
                    short_commit(&event.commit),
                    event.id,
                    event.identity,
                    flat(&event.summary)
                )));
            }
            Event::Delivered(id) => {
                let Some(entry) = view.entry(id) else {
                    continue;
                };
                did.extend(delivered_lines(view, entry, &when, context));
            }
        }
    }

    let mut results = Vec::new();
    for proof in &view.criteria.proofs {
        let slug = &proof.criterion.slug;
        let mut row = format!("{slug} {}", criterion_word(proof.mark));
        let by = view.delivered_by.get(slug).cloned().unwrap_or_default();
        if !by.is_empty() {
            row.push_str(&format!(" | by {}", by.join(", ")));
        }
        if let Some(cue) = proof_cue(proof, closed) {
            row.push_str(&format!(" | {cue}"));
        } else if let Some(by) = awaited_by(view, slug) {
            row.push_str(&format!(" | awaits {by}"));
        }
        results.extend(wrap(row));
    }

    let mut validation = Vec::new();
    let checks = &view.validation;
    let delivered =
        checks.approved + checks.trivial + checks.not_approved.len() + checks.unreviewed.len();
    if delivered > 0 {
        let mut line = format!(
            "Item review: {} of {delivered} delivered items approved",
            checks.approved
        );
        if checks.trivial > 0 {
            line.push_str(&format!(", {} waived as trivial", checks.trivial));
        }
        if let Some(counts) = finding_counts(&checks.findings) {
            line.push_str(&format!(" | {counts}"));
        }
        validation.extend(wrap(line));
        if !checks.not_approved.is_empty() {
            validation.extend(wrap(format!(
                "Not approved: {}",
                checks.not_approved.join(", ")
            )));
        }
        if !checks.unreviewed.is_empty() {
            validation.extend(wrap(format!(
                "No review recorded: {}",
                checks.unreviewed.join(", ")
            )));
        }
    }
    for run in &checks.runs {
        let mut head = format!(
            "{} {} at {} | {}",
            local_time(run.at),
            run.identity,
            short_commit(&run.commit),
            atelier_core::count_noun(run.receipts.len(), "receipt")
        );
        if let Some(count) = run.code_commits_since.filter(|count| *count > 0) {
            head.push_str(&format!(" | {}", commits_since(count, closed)));
        }
        validation.extend(wrap(head));
        for receipt in &run.receipts {
            let criteria = receipt
                .criteria
                .iter()
                .map(|criterion| format!("{} {}", criterion.slug, criterion.result.as_str()))
                .collect::<Vec<_>>()
                .join(", ");
            for line in wrap(format!(
                "{criteria} {} | {}",
                receipt.id,
                flat(&receipt.summary)
            )) {
                validation.push(format!("  {line}"));
            }
        }
        if !run.later_work.is_empty() {
            for line in wrap(format!("Delivered since: {}", run.later_work.join(", "))) {
                validation.push(format!("  {line}"));
            }
        }
    }
    for (slug, mark, by) in &checks.awaiting {
        let mut line = format!("Awaiting: {slug} {}", criterion_word(*mark));
        if let Some(by) = by {
            line.push_str(&format!(" | {}", item(by)));
        }
        validation.extend(wrap(line));
    }

    let mut plan = Vec::new();
    if view.planned_total > 0 {
        plan.extend(wrap(format!(
            "Planned: {} of {} done",
            view.planned_done,
            atelier_core::count_noun(view.planned_total, "item")
        )));
    }
    let changed = view
        .work
        .iter()
        .filter(|entry| changed_course(entry))
        .map(|entry| entry.id.clone())
        .collect::<Vec<_>>();
    if !changed.is_empty() {
        plan.extend(wrap(format!("Changed course: {}", changed.join(", "))));
    }
    for id in &view.added {
        plan.extend(wrap(format!("Added: {}", item(id))));
    }
    for entry in &view.dropped {
        let mut row = format!(
            "Dropped: {} | {}",
            item(&entry.id),
            flat(entry.reason.as_deref().unwrap_or_default())
        );
        for change in &entry.changes {
            row.push_str(&format!(" | {}", change_text(change)));
        }
        plan.extend(wrap(row));
        plan.extend(decision_lines(view, &entry.id, context));
    }
    if !view.found.is_empty() {
        plan.extend(wrap(format!(
            "Found: {} still open",
            atelier_core::count_noun(view.found.len(), "item")
        )));
    }

    let asks = view
        .asks
        .iter()
        .flat_map(|ask| {
            let tag = match ask {
                Ask::Decision(id) | Ask::NeedsHuman(id) => format!("{id} "),
                _ => String::new(),
            };
            let (title, detail) = ask_line(view, ask);
            let mut lines = wrap(if detail.is_empty() {
                format!("⚠ {tag}{title}")
            } else {
                format!("⚠ {tag}{title} | {detail}")
            });
            if let Ask::Decision(id) | Ask::NeedsHuman(id) = ask {
                lines.extend(decision_lines(view, id, context));
            }
            lines
        })
        .collect::<Vec<_>>();
    let open_rows = |entries: &[OpenEntry], found: bool| {
        entries
            .iter()
            .flat_map(|entry| {
                let indent = if entry.parent.is_some() { "  " } else { "" };
                let mut row = format!("{indent}{}", item(&entry.id));
                if found {
                    match origin(entry) {
                        Some((from, how)) if from == view.mission.id => {
                            row.push_str(&format!(" | {how} the mission"));
                        }
                        Some((from, how)) => row.push_str(&format!(" | {how} {from}")),
                        None => {}
                    }
                    if let Some(at) = entry.filed_at {
                        row.push_str(&format!(" | filed {}", local_time(at)));
                    }
                } else if entry.added_after_start {
                    row.push_str(" | added after the start");
                }
                for change in &entry.changes {
                    row.push_str(&format!(" | {}", change_text(change)));
                }
                let mut lines = wrap(row);
                lines.extend(decision_lines(view, &entry.id, context));
                lines
            })
            .collect::<Vec<_>>()
    };

    let optional = |title: &str, lines: Vec<String>| {
        if lines.is_empty() {
            Vec::new()
        } else {
            render_lines_section(title, lines)
        }
    };
    vec![
        render_metadata(None, &metadata),
        optional("The Problem", problem),
        optional("What We Set Out To Do", intent),
        optional("Planned Work", planned),
        optional("What We Did", did),
        optional("Results", results),
        optional("Validation", validation),
        optional("Plan vs Actual", plan),
        optional("Needs Your Review", asks),
        optional("Left To Do", open_rows(&view.left, false)),
        optional("Found Along The Way", open_rows(&view.found, true)),
        render_footer_section(
            context,
            page.map(|url| FooterAction::new("Send the human the page", url))
                .into_iter()
                .chain([FooterAction::new(
                    "Record the human's review",
                    format!(
                        "atelier review record {} --identity \"<human>\" \"<summary>\"",
                        view.mission.id
                    ),
                )]),
        ),
    ]
}

/// A done child's report as labelled facts: its Outcome, then its non-empty
/// Deviations, Surprises, Decisions and, when review filed no follow-ups for
/// the entry, Open items.
fn report_facts(report: Option<&CloseReport>, open_items: bool) -> Vec<(&'static str, &str)> {
    let mut facts = Vec::new();
    let outcome = outcome(report);
    if !outcome.is_empty() {
        facts.push(("Outcome", outcome));
    }
    for (label, part) in [
        ("Changed", ReportPart::Deviations),
        ("Surprised by", ReportPart::Surprises),
        ("Decided", ReportPart::Decisions),
        ("Left open", ReportPart::OpenItems),
    ] {
        if part == ReportPart::OpenItems && !open_items {
            continue;
        }
        if let Some(text) = reported(report, part) {
            facts.push((label, text));
        }
    }
    facts
}

/// A delivered item in the Markdown timeline, with its facts indented.
fn delivered_lines(
    view: &Debrief,
    entry: &WorkEntry,
    when: &str,
    context: RenderContext,
) -> Vec<String> {
    let flat = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
    let item = |id: &str| {
        view.refs
            .get(id)
            .map(|issue| {
                ItemLine::new(&issue.id, &issue.status, "", &issue.title)
                    .issue_type(&issue.issue_type)
                    .state(issue.status_category.as_deref(), None)
                    .text()
                    .replace("  ", " ")
            })
            .unwrap_or_else(|| id.to_string())
    };
    let mut lines = Vec::new();
    // Declined work is finished without merging, so it is not delivered
    // (atelier-stnv).
    let verb = match view.refs.get(&entry.id) {
        Some(issue) if issue.status == "declined" => "declined",
        _ => "delivered",
    };
    let mut head = format!("{when} {verb} {}", item(&entry.id));
    if !entry.criteria.is_empty() {
        head.push_str(&format!(" | {}", entry.criteria.join(", ")));
    }
    if let Some(parent) = &entry.parent {
        head.push_str(&format!(" | under {parent}"));
    }
    if entry.added_after_start {
        head.push_str(" | added after the start");
    }
    lines.extend(render_row_wrapped(context, &head));
    let mut indented = |indent: &str, label: &str, text: &str| {
        let line = if label == "└" {
            format!("└ {}", flat(text))
        } else {
            format!("{label}: {}", flat(text))
        };
        for line in render_row_wrapped(context, &line) {
            lines.push(format!("{indent}{line}"));
        }
    };
    indented("  ", "Outcome", outcome(entry.report.as_ref()));
    // Each child's own report sits under its line, so it is not read as the
    // parent's.
    for child in &entry.children {
        indented("  ", "└", &item(&child.id));
        for (label, text) in report_facts(child.report.as_ref(), entry.follow_ups.is_empty()) {
            indented("    ", label, text);
        }
    }
    let mut fact = |label: &str, text: &str| indented("  ", label, text);
    if let Some(text) = reported(entry.report.as_ref(), ReportPart::Deviations) {
        fact("Changed", text);
    }
    for change in &entry.changes {
        fact("Changed", &change_text(change));
    }
    if let Some(text) = reported(entry.report.as_ref(), ReportPart::Surprises) {
        fact("Surprised by", text);
    }
    // The report's own Decisions part, then the decision notes it may point at.
    if let Some(text) = reported(entry.report.as_ref(), ReportPart::Decisions) {
        fact("Decided", text);
    }
    for note in &entry.decisions {
        fact("Decided", &decision_text(note));
    }
    for note in &entry.malformed {
        fact("⚠ Malformed decision note", &malformed_text(note));
    }
    if let Some(record) = view
        .decisions
        .records
        .iter()
        .find(|record| record.issue.id == entry.id)
    {
        if let Some(adr) = &record.issue.adr {
            fact("ADR", &adr.path);
        }
        for id in &record.supersedes {
            fact("Replaces", &item(id));
        }
    }
    if let Some(review) = review_line(view, entry) {
        fact("Review", &review);
    }
    for follow_up in &entry.follow_ups {
        fact("Filed", &item(follow_up));
    }
    if entry.follow_ups.is_empty() {
        if let Some(text) = reported(entry.report.as_ref(), ReportPart::OpenItems) {
            fact("Left open", text);
        }
    }
    lines
}

/// Where filed work came from, and how: `(source, "from review of")` for a
/// deferred finding, `(source, "found in")` for a `discovered_from` link.
fn origin(entry: &OpenEntry) -> Option<(&str, &'static str)> {
    let how = if entry.discovered {
        "found in"
    } else {
        "from review of"
    };
    entry.from.as_deref().map(|from| (from, how))
}

pub fn brief(index: &RecordIndex, mission_ref: &str, quiet: bool) -> Result<()> {
    let id = crate::commands::issue::resolve_id(index, mission_ref)?;
    let issue = index.require_issue(&id)?;
    if issue.issue_type != "mission" {
        bail!(
            "{id} is {}, not a mission; brief reviews a mission",
            atelier_app::record_ref::with_article(&issue.issue_type)
        );
    }
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let repo_root = crate::commands::workflow::repo_root()?;
    let mut read = atelier_app::git_freshness::ReadContext::new(&repo_root);
    let view = atelier_app::mission_review::brief(index, policy.as_ref(), &mut read, &id)?;
    let approvals = view
        .mission_events
        .iter()
        .filter(|event| event.event == "approval")
        .count();
    if quiet {
        println!(
            "criteria={} items={} blocked={} unknowns={} human_decisions={} approvals={approvals}",
            view.criteria.parse.criteria.len(),
            view.items.len(),
            view.blocked().count(),
            view.items
                .iter()
                .filter(|item| item.unknowns.is_some())
                .count(),
            view.human_decisions.len()
        );
        return Ok(());
    }
    let context = RenderContext::for_stdout();
    let mission = &view.mission;
    let title = ItemLine::new(
        &mission.id,
        &mission.status,
        &mission.priority,
        &mission.title,
    )
    .issue_type("mission")
    .state(view.status_category.as_deref(), None)
    .text();
    let mut metadata = vec![("Outcome", view.outcome.replace('\n', " "))];
    // A missing Sizing or Derisking section is named under the Outcome, so
    // an unsized plan shows; a written one follows as its own section.
    let mut plan: Vec<String> = Vec::new();
    for (title, text) in view.plan_sections() {
        match text {
            Some(text) => {
                if !plan.is_empty() {
                    plan.push(String::new());
                }
                plan.extend(render_text_section(title, Some(text)));
            }
            None => metadata.push((title, NOT_WRITTEN.to_string())),
        }
    }
    if let Some(strategy) = &view.strategy {
        metadata.push(("Strategy", strategy.clone()));
    }
    if let Some(commit) = &view.generated_from {
        metadata.push(("Generated from", short_commit(commit).to_string()));
    }
    let mut criteria = Vec::new();
    for criterion in &view.criteria.parse.criteria {
        criteria.extend(render_row_wrapped(
            context,
            &format!("{} {}", criterion.slug, criterion.text),
        ));
    }
    for problem in &view.criteria.parse.problems {
        criteria.extend(render_row_wrapped(context, &format!("⚠ {problem}")));
    }
    let mut scope = Vec::new();
    for item in &view.items {
        let issue = &item.issue;
        let mut line = ItemLine::new(&issue.id, &issue.status, &issue.priority, &issue.title)
            .issue_type(&issue.issue_type)
            .state(item.status_category.as_deref(), None);
        if let Some(size) = &item.size {
            line = line.cue(format!("size {size}"));
        }
        if !item.open_blockers.is_empty() {
            line = line.cue(format!("blockers: {}", item.open_blockers.join(", ")));
        }
        if item.needs_human {
            line = line.cue("needs a human");
        }
        let indent = if item.parent.is_some() { "  " } else { "" };
        scope.extend(line.render_wrapped(context, indent));
    }
    let blocked = view
        .blocked()
        .flat_map(|item| {
            render_row_wrapped(
                context,
                &format!(
                    "{} blocked by {}",
                    item.issue.id,
                    item.open_blockers.join(", ")
                ),
            )
        })
        .collect::<Vec<_>>();
    let unknowns = view
        .items
        .iter()
        .filter_map(|item| Some((item, item.unknowns.as_deref()?)))
        .flat_map(|(item, text)| {
            render_row_wrapped(
                context,
                &format!(
                    "{}: {}",
                    item.issue.id,
                    text.lines().map(str::trim).collect::<Vec<_>>().join(" ")
                ),
            )
        })
        .collect::<Vec<_>>();
    let human = view
        .human_decisions
        .iter()
        .flat_map(|issue| {
            let category = policy
                .as_ref()
                .and_then(|policy| policy.status_category(&issue.status));
            ItemLine::new(&issue.id, &issue.status, &issue.priority, &issue.title)
                .issue_type(&issue.issue_type)
                .state(category, None)
                .render_wrapped(context, "")
        })
        .collect::<Vec<_>>();
    let risks = view
        .risks
        .as_deref()
        .map(|text| text.lines().map(str::to_string).collect::<Vec<_>>())
        .unwrap_or_default();
    let mut events = view
        .mission_events
        .iter()
        .flat_map(|event| render_row_wrapped(context, &event_line(event)))
        .collect::<Vec<_>>();
    if approvals == 0 {
        events.push(NO_PLAN_APPROVAL.to_string());
    }
    let optional = |title: &str, lines: Vec<String>| {
        if lines.is_empty() {
            Vec::new()
        } else {
            render_lines_section(title, lines)
        }
    };
    print_page(
        &format!("Brief: {title}"),
        [
            render_metadata(None, &metadata),
            plan,
            optional("Success Criteria", criteria),
            render_lines_section("Scope", scope),
            optional("Blockers", blocked),
            optional("Unknowns", unknowns),
            optional("Open Human Decisions", human),
            optional("Risks", risks),
            render_lines_section("Mission Reviews", events),
            render_footer_section(
                context,
                site_page(&repo_root, &id, "brief")
                    .map(|url| FooterAction::new("Send the human the page", url))
                    .into_iter()
                    .chain([{
                        let (label, command) = brief_next_step(policy.as_ref(), mission, approvals);
                        FooterAction::new(label, command)
                    }]),
            ),
        ],
        context,
    );
    Ok(())
}

/// What the brief shows for a strategy section that is not written.
const NOT_WRITTEN: &str = "not written";

#[cfg(test)]
mod tests {
    use super::*;

    /// A won't-fix finding is counted apart from no_action (atelier-stnv).
    #[test]
    fn finding_counts_name_wont_fix_apart_from_no_action() {
        let review = ReviewSummary {
            no_action: 1,
            wont_fix: 2,
            ..ReviewSummary::default()
        };
        assert_eq!(
            finding_counts(&review).as_deref(),
            Some("3 findings: 1 left as is, 2 won't fix")
        );
    }
}
