//! `atelier debrief` (atelier-rx22, atelier-nexd, atelier-8bil): a mission's
//! after-action report, rendered from one view model as dense Markdown for
//! agents (ADR 0022) and, with `--html`, as a static page a human opens in
//! T3 Code's file panel. Both tell the problem, what the mission set out to
//! do and what it did in time order, then where it stands: results, how it
//! was validated, plan against actual, what needs the human, what is left
//! and what was found along the way.

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use atelier_app::criteria::{CriterionMark, CriterionProof};
use atelier_app::mission_review::{
    first_sentence, Ask, Change, Debrief, Event, OpenEntry, ReviewSummary, ValidationRun, WorkEntry,
};
use atelier_app::record_index::RecordIndex;
use atelier_core::{CloseReport, EvidenceResult, ReportPart};

use crate::human_output::{
    local_date, print_page, render_footer_section, render_lines_section, render_metadata,
    render_row_wrapped, short_commit, FooterAction, ItemLine, RenderContext,
};
use crate::review_page::{self, badge, escape, inline, prose, section, tile, Tone};

pub fn debrief(index: &RecordIndex, mission_ref: &str, html: bool, quiet: bool) -> Result<()> {
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
    if html {
        let path = write_review_page(&repo_root, &id, "debrief", &render_html(&view))?;
        if quiet {
            println!("{}", path.display());
        } else {
            println!("Wrote the debrief of {id} to {}", path.display());
            println!("Open it in T3 Code's file panel, or in any browser.");
        }
        return Ok(());
    }
    if quiet {
        println!("{}", quiet_counts(&view));
        return Ok(());
    }
    let context = RenderContext::for_stdout();
    print_page(
        &format!("Debrief: {}", mission_line(&view)),
        render_markdown(&view, context),
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
    let asked = view
        .asks
        .iter()
        .filter(|ask| matches!(ask, Ask::Decision(_) | Ask::NeedsHuman(_)))
        .count();
    let items = view
        .work
        .iter()
        .map(|entry| 1 + entry.children.len())
        .sum::<usize>()
        + view.left.len()
        + view.found.len()
        + view.dropped.len()
        + asked;
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
                + usize::from(reported(entry, ReportPart::Deviations).is_some())
                + usize::from(entry.added_after_start)
        })
        .sum::<usize>()
        + view
            .left
            .iter()
            .chain(&view.found)
            .map(|entry| entry.changes.len() + usize::from(entry.added_after_start))
            .sum::<usize>()
        + view.dropped.len()
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

/// The page under `.atelier/runtime/reviews/`, which is never committed.
fn write_review_page(
    repo_root: &Path,
    mission_id: &str,
    kind: &str,
    html: &str,
) -> Result<PathBuf> {
    let directory = repo_root.join(".atelier/runtime/reviews");
    std::fs::create_dir_all(&directory)
        .with_context(|| format!("cannot create {}", directory.display()))?;
    let path = directory.join(format!("{mission_id}-{kind}.html"));
    std::fs::write(&path, html).with_context(|| format!("cannot write {}", path.display()))?;
    Ok(path)
}

/// A report part with something to say: present, and not just "None".
fn reported(entry: &WorkEntry, part: ReportPart) -> Option<&str> {
    entry.report.as_ref()?.part(part).filter(|text| {
        !matches!(
            text.trim()
                .trim_end_matches('.')
                .to_ascii_lowercase()
                .as_str(),
            "none" | "n/a"
        )
    })
}

fn outcome(entry: &WorkEntry) -> &str {
    entry
        .report
        .as_ref()
        .map(|report: &CloseReport| report.outcome.as_str())
        .unwrap_or_default()
}

/// Whether the entry departed from its plan.
fn changed_course(entry: &WorkEntry) -> bool {
    !entry.changes.is_empty() || reported(entry, ReportPart::Deviations).is_some()
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
    }
}

/// How review went; `None` for a decision record, which is decided rather
/// than code-reviewed.
fn review_line(view: &Debrief, entry: &WorkEntry) -> Option<String> {
    if view
        .refs
        .get(&entry.id)
        .is_some_and(|issue| issue.issue_type == "decision")
    {
        return None;
    }
    if !entry.review.recorded {
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

fn render_markdown(view: &Debrief, context: RenderContext) -> Vec<Vec<String>> {
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
                first_sentence(&proof.criterion.text)
            )));
        }
        for problem in &view.criteria.criteria.parse.problems {
            intent.extend(wrap(format!("⚠ {problem}")));
        }
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
                    .and_then(|entry| entry.from.as_deref());
                let cue = if from.is_some() {
                    " | from review of the mission"
                } else {
                    ""
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
        let mut line = format!("Item review: {} of {delivered} approved", checks.approved);
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
            wrap(if detail.is_empty() {
                format!("⚠ {tag}{title}")
            } else {
                format!("⚠ {tag}{title} | {detail}")
            })
        })
        .collect::<Vec<_>>();
    let open_rows = |entries: &[OpenEntry], found: bool| {
        entries
            .iter()
            .flat_map(|entry| {
                let indent = if entry.parent.is_some() { "  " } else { "" };
                let mut row = format!("{indent}{}", item(&entry.id));
                if found {
                    match entry.from.as_deref() {
                        Some(from) if from == view.mission.id => {
                            row.push_str(" | from review of the mission");
                        }
                        Some(from) => row.push_str(&format!(" | from review of {from}")),
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
                wrap(row)
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
            [
                FooterAction::new(
                    "Write the HTML page",
                    format!("atelier debrief {} --html", view.mission.id),
                ),
                FooterAction::new(
                    "Record the human's review",
                    format!(
                        "atelier review record {} --identity \"<human>\" \"<summary>\"",
                        view.mission.id
                    ),
                ),
            ],
        ),
    ]
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
    let mut head = format!("{when} delivered {}", item(&entry.id));
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
    let mut fact = |label: &str, text: &str| {
        let line = if label == "└" {
            format!("└ {}", flat(text))
        } else {
            format!("{label}: {}", flat(text))
        };
        for line in render_row_wrapped(context, &line) {
            lines.push(format!("  {line}"));
        }
    };
    fact("Outcome", outcome(entry));
    for child in &entry.children {
        fact("└", &item(child));
    }
    if let Some(text) = reported(entry, ReportPart::Deviations) {
        fact("Changed", text);
    }
    for change in &entry.changes {
        fact("Changed", &change_text(change));
    }
    if let Some(text) = reported(entry, ReportPart::Surprises) {
        fact("Surprised by", text);
    }
    for note in &entry.decisions {
        fact(
            "Decided",
            &format!(
                "{}{} | because {}",
                if note.decision.one_way {
                    "one-way: "
                } else {
                    ""
                },
                note.decision.choice,
                note.decision.reason
            ),
        );
    }
    for note in &entry.malformed {
        fact(
            "⚠ Malformed decision note",
            &format!("{} #{} {}", note.issue_id, note.number, note.defect),
        );
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
        if let Some(text) = reported(entry, ReportPart::OpenItems) {
            fact("Left open", text);
        }
    }
    lines
}

fn status_tone(category: Option<&str>, status: &str) -> Tone {
    match (category, status) {
        (_, "superseded") => Tone::Neutral,
        (Some("done"), _) => Tone::Good,
        (Some("blocked"), _) => Tone::Bad,
        (Some("active"), _) => Tone::Warn,
        _ => Tone::Neutral,
    }
}

/// The status dot's and label's class for an issue.
fn status_class(view: &Debrief, id: &str) -> &'static str {
    match view.refs.get(id) {
        Some(issue) if issue.status == "superseded" => "back",
        Some(issue) => match issue.status_category.as_deref() {
            Some("done") => "done",
            Some("active") => "active",
            Some("blocked") => "warn",
            _ if issue.status == "backlog" || issue.status == "draft" => "back",
            _ => "todo",
        },
        None => "back",
    }
}

fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() > max {
        format!(
            "{}…",
            text.chars().take(max - 1).collect::<String>().trim_end()
        )
    } else {
        text.to_string()
    }
}

/// The card a chip opens: the issue's full title, type, status and ID, what
/// it is for, how it ended and what it is part of.
fn card(view: &Debrief, id: &str) -> String {
    let Some(issue) = view.refs.get(id) else {
        return String::new();
    };
    let mut card = format!(
        "<span class=\"card\"><b>{}</b><span class=\"m\">{} · {} · {}</span>",
        escape(&issue.title),
        escape(&capitalise(&issue.issue_type)),
        escape(&issue.status.replace('_', " ")),
        escape(&issue.id)
    );
    if !issue.summary.is_empty() {
        card.push_str(&format!(
            "<span class=\"s\">{}</span>",
            inline(&issue.summary)
        ));
    }
    if let Some(outcome) = &issue.outcome {
        card.push_str(&format!(
            "<span class=\"s\"><i>Outcome</i> {}</span>",
            inline(outcome)
        ));
    }
    if let Some(parent) = &issue.parent {
        card.push_str(&format!(
            "<span class=\"m\">Part of {} · {}</span>",
            escape(view.title_of(parent)),
            escape(parent)
        ));
    }
    card.push_str("</span>");
    card
}

/// An issue as a chip: its ID, title and status; focusing or hovering it
/// opens its card.
fn chip(view: &Debrief, id: &str, max: usize) -> String {
    let Some(issue) = view.refs.get(id) else {
        return format!("<span class=\"k\">{}</span>", escape(id));
    };
    let class = status_class(view, id);
    format!(
        "<span class=\"chip\" tabindex=\"0\"><span class=\"dot {class}\"></span><span class=\"k\">{}</span><span class=\"t\">{}</span><span class=\"st {class}\">{}</span>{}</span>",
        escape(id),
        escape(&truncate(&issue.title, max)),
        escape(&issue.status.replace('_', " ")),
        card(view, id)
    )
}

/// An issue's ID alone as a chip, beside a title already shown.
fn key_chip(view: &Debrief, id: &str) -> String {
    format!(
        "<span class=\"chip key\" tabindex=\"0\"><span class=\"k\">{}</span>{}</span>",
        escape(id),
        card(view, id)
    )
}

/// A review event or receipt as a chip: its ID and kind, with what it says.
fn record_chip(view: &Debrief, id: &str) -> String {
    record_chip_labelled(view, id, true)
}

/// A record chip, with or without its kind beside the ID.
fn record_chip_labelled(view: &Debrief, id: &str, kind: bool) -> String {
    let Some(record) = view.record_refs.get(id) else {
        return format!("<span class=\"k\">{}</span>", escape(id));
    };
    format!(
        "<span class=\"chip rec\" tabindex=\"0\"><span class=\"k\">{}</span>{}<span class=\"card\"><b>{}</b><span class=\"s\">{}</span><span class=\"m\">{}</span></span></span>",
        escape(id),
        if kind {
            format!("<span class=\"t\">{}</span>", escape(&record.kind))
        } else {
            String::new()
        },
        escape(&capitalise(&record.kind)),
        inline(&record.summary),
        escape(id)
    )
}

/// Record text as HTML with each mention of a known record as a chip.
fn rich(view: &Debrief, text: &str) -> String {
    link_mentions(view, &prose(text))
}

fn rich_inline(view: &Debrief, text: &str) -> String {
    link_mentions(view, &inline(text))
}

fn link_mentions(view: &Debrief, html: &str) -> String {
    let Some((slug, _)) = view.mission.id.rsplit_once('-') else {
        return html.to_string();
    };
    let prefix = format!("{slug}-");
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(at) = rest.find(&prefix) {
        let before = rest[..at].chars().next_back();
        let tail = &rest[at + prefix.len()..];
        let key_len = tail
            .char_indices()
            .find(|(_, c)| !(c.is_ascii_lowercase() || c.is_ascii_digit()))
            .map_or(tail.len(), |(index, _)| index);
        let id = &rest[at..at + prefix.len() + key_len];
        let bounded = before.is_none_or(|c| !(c.is_ascii_alphanumeric() || c == '-'))
            && tail[key_len..].chars().next().is_none_or(|c| c != '-');
        out.push_str(&rest[..at]);
        if bounded && view.refs.contains_key(id) {
            out.push_str(&chip(view, id, 36));
        } else if bounded && view.record_refs.contains_key(id) {
            out.push_str(&record_chip(view, id));
        } else {
            out.push_str(id);
        }
        rest = &rest[at + id.len()..];
    }
    out.push_str(rest);
    out
}

fn time_rail(time: Option<chrono::DateTime<chrono::Utc>>) -> String {
    match time {
        Some(time) => {
            let local = time.with_timezone(&chrono::Local);
            format!("<b>{}</b>{}", local.format("%H:%M"), local.format("%-d %b"))
        }
        None => String::new(),
    }
}

fn render_html(view: &Debrief) -> String {
    let mission = &view.mission;
    let closed = view.timeline.closed.is_some();
    let total = view.criteria.proofs.len();
    let proven = view.proven();
    let percent = |part: usize, whole: usize| if whole == 0 { 0 } else { part * 100 / whole };
    let one_way = view
        .work
        .iter()
        .flat_map(|entry| {
            entry
                .decisions
                .iter()
                .filter(|note| note.decision.one_way)
                .map(move |note| (entry.id.as_str(), note))
        })
        .collect::<Vec<_>>();

    // Sidebar.
    let mut side = format!("<h1>{}</h1>", escape(&mission.title));
    side.push_str(&format!(
        "<span class=\"status {}\">{}{}</span>",
        if view.asks.is_empty() { "done" } else { "warn" },
        status_word(view),
        match view.asks.len() {
            0 => String::new(),
            1 => " · 1 needs you".to_string(),
            n => format!(" · {n} need you"),
        }
    ));
    if total > 0 {
        side.push_str(&format!(
            "<div class=\"meter\"><span>Criteria proven</span><span>{proven} of {total}</span></div><div class=\"bar\"><span style=\"width:{}%\"></span></div>",
            percent(proven, total)
        ));
    }
    if view.planned_total > 0 {
        side.push_str(&format!(
            "<div class=\"meter\"><span>Planned work done</span><span>{} of {}</span></div><div class=\"bar\"><span style=\"width:{}%\"></span></div>",
            view.planned_done,
            view.planned_total,
            percent(view.planned_done, view.planned_total)
        ));
    }
    side.push_str("<nav>");
    if !view.asks.is_empty() {
        side.push_str(&format!(
            "<a href=\"#review\" class=\"ask-link\"><span class=\"dot warn\"></span>Needs your review <span class=\"n\">{}</span></a>",
            view.asks.len()
        ));
    }
    side.push_str("<div class=\"g\">Report</div>");
    let mut link = |anchor: &str, label: &str, count: Option<String>| {
        side.push_str(&format!(
            "<a href=\"#{anchor}\">{}{}</a>",
            escape(label),
            count
                .map(|count| format!(" <span class=\"n\">{}</span>", escape(&count)))
                .unwrap_or_default()
        ));
    };
    if view.why.is_some() {
        link("problem", "The problem", None);
    }
    link("intent", "What we set out to do", None);
    if !view.events.is_empty() {
        link("did", "What we did", None);
    }
    if total > 0 {
        link("results", "Results", Some(format!("{proven}/{total}")));
    }
    link("validation", "Validation", None);
    link("plan", "Plan vs actual", None);
    if !view.left.is_empty() {
        link("left", "Left to do", Some(view.left.len().to_string()));
    }
    if !view.found.is_empty() {
        link(
            "found",
            "Found along the way",
            Some(view.found.len().to_string()),
        );
    }
    if !view.work.is_empty() {
        side.push_str("<div class=\"g\">Delivered</div>");
        for entry in &view.work {
            side.push_str(&format!(
                "<a href=\"#{}\"><span class=\"dot done\"></span><span class=\"label\">{}</span></a>",
                escape(&entry.id),
                escape(view.title_of(&entry.id))
            ));
            for child in &entry.children {
                side.push_str(&format!(
                    "<a class=\"sub\" href=\"#{}\"><span class=\"dot {}\"></span><span class=\"label\">{}</span></a>",
                    escape(&entry.id),
                    status_class(view, child),
                    escape(view.title_of(child))
                ));
            }
        }
    }
    if !one_way.is_empty() {
        side.push_str("<div class=\"g\">One-way decisions</div>");
        for (entry, note) in &one_way {
            side.push_str(&format!(
                "<a class=\"dec-link\" href=\"#{}\" title=\"{}\"><span class=\"label\">{}</span></a>",
                escape(entry),
                escape(&note.decision.choice),
                escape(&note.decision.choice)
            ));
        }
    }
    side.push_str("</nav>");

    // Header.
    let mut main = format!("<h2>{}</h2><div class=\"props\">", escape(&mission.title));
    main.push_str(&format!(
        "<span class=\"lozenge {}\">{}</span>",
        match view.status_category.as_deref() {
            Some("done") => "done",
            _ => "todo",
        },
        status_word(view)
    ));
    main.push_str(&format!("<span class=\"k\">{}</span>", escape(&mission.id)));
    let window_start = view.timeline.started.as_ref().map(|moment| moment.time);
    let window_end = view
        .timeline
        .closed
        .as_ref()
        .map(|moment| moment.time)
        .or_else(|| last_delivery(view));
    if let Some(start) = window_start {
        let start = start.with_timezone(&chrono::Local);
        let end = window_end.map(|end| end.with_timezone(&chrono::Local));
        let window = match end {
            Some(end) if end.date_naive() == start.date_naive() => format!(
                "{} {} – {}",
                start.format("%-d %b"),
                start.format("%H:%M"),
                end.format("%H:%M")
            ),
            Some(end) => format!("{} – {}", start.format("%-d %b"), end.format("%-d %b")),
            None => format!("since {}", start.format("%-d %b")),
        };
        main.push_str(&format!("<span>{}</span>", escape(&window)));
    }
    if let Some(strategy) = &view.strategy {
        main.push_str(&format!("<span><code>{}</code></span>", escape(strategy)));
    }
    if let Some(commit) = &view.generated_from {
        main.push_str(&format!(
            "<span>at <code>{}</code></span>",
            escape(short_commit(commit))
        ));
    }
    main.push_str("</div>");
    let verdict = verdict(view)
        .into_iter()
        .map(|(text, anchor)| match anchor {
            Some(anchor) => format!("<a href=\"#{anchor}\">{}</a>", escape(&text)),
            None => escape(&text),
        })
        .collect::<Vec<_>>();
    if !verdict.is_empty() {
        main.push_str(&format!(
            "<p class=\"verdict\">{}</p>",
            verdict.join(" <span class=\"sep\">·</span> ")
        ));
    }

    // The problem.
    if let Some(why) = &view.why {
        main.push_str(&format!(
            "<section id=\"problem\" class=\"prose\"><h3>The problem</h3>{}</section>",
            rich(view, why)
        ));
    }

    // What we set out to do.
    let mut intent = String::new();
    if !view.outcome.trim().is_empty() {
        intent.push_str(&format!(
            "<div class=\"lede\">{}</div>",
            rich(view, &view.outcome)
        ));
    }
    if total > 0 || !view.criteria.criteria.parse.problems.is_empty() {
        intent.push_str("<h4>Success criteria</h4>");
        for proof in &view.criteria.proofs {
            intent.push_str(&format!(
                "<div class=\"claim\"><span class=\"s\">{}</span><span>{}</span></div>",
                escape(&proof.criterion.slug),
                rich_inline(view, &first_sentence(&proof.criterion.text))
            ));
        }
        for problem in &view.criteria.criteria.parse.problems {
            intent.push_str(&format!(
                "<div class=\"claim\"><span class=\"s status warn\">⚠</span><span>{}</span></div>",
                escape(problem)
            ));
        }
    }
    if !view.planned.is_empty() {
        intent.push_str("<h4>Planned work</h4><ul class=\"chips\">");
        for (id, parent) in &view.planned {
            intent.push_str(&format!(
                "<li{}>{}</li>",
                if parent.is_some() {
                    " class=\"kid\""
                } else {
                    ""
                },
                chip(view, id, 64)
            ));
        }
        intent.push_str("</ul>");
    }
    main.push_str(&format!(
        "<section id=\"intent\"><h3>What we set out to do</h3>{intent}</section>"
    ));

    // What we did.
    if !view.events.is_empty() {
        let mut entries = String::new();
        for timed in &view.events {
            entries.push_str(&event_html(view, timed.at, &timed.event));
        }
        main.push_str(&format!(
            "<section id=\"did\"><h3>What we did</h3>{entries}</section>"
        ));
    }

    // Results.
    if total > 0 {
        let mut rows = String::new();
        for proof in &view.criteria.proofs {
            let slug = &proof.criterion.slug;
            let (class, word) = match proof.mark {
                CriterionMark::Proven => ("done", "✓ Proven"),
                CriterionMark::Failing => ("warn", "⚠ Failing"),
                CriterionMark::Unproven => ("todo", "Unproven"),
            };
            let by = view
                .delivered_by
                .get(slug)
                .into_iter()
                .flatten()
                .map(|id| chip(view, id, 40))
                .collect::<Vec<_>>()
                .join(" ");
            let proof_html = match proof.deciding() {
                Some(receipt) => {
                    let mut html = format!(
                        "{} at <code>{}</code>",
                        record_chip_labelled(view, &receipt.evidence_id, false),
                        escape(short_commit(&receipt.commit))
                    );
                    if let Some(age) = proof_age(proof, closed) {
                        html.push_str(&format!(" <span class=\"age\">{}</span>", escape(&age)));
                    }
                    html
                }
                None => match awaited_by(view, slug) {
                    Some(by) => format!("awaits {}", chip(view, by, 40)),
                    None => "<span class=\"note\">no proof recorded</span>".into(),
                },
            };
            rows.push_str(&format!(
                "<div class=\"crit\"><span class=\"s\" title=\"{}\">{}</span><span class=\"status {class}\">{word}</span><span class=\"by\">{by}</span><span class=\"proof\">{proof_html}</span></div>",
                escape(&first_sentence(&proof.criterion.text)),
                escape(slug)
            ));
        }
        main.push_str(&format!(
            "<section id=\"results\"><h3>Results</h3><div class=\"crit head\"><span></span><span></span><span>Delivered by</span><span>Proof</span></div>{rows}</section>"
        ));
    }

    // Validation.
    main.push_str(&format!(
        "<section id=\"validation\"><h3>Validation</h3>{}</section>",
        validation_html(view, closed)
    ));

    // Plan vs actual.
    main.push_str(&format!(
        "<section id=\"plan\"><h3>Plan vs actual</h3>{}</section>",
        plan_html(view)
    ));

    // Needs your review.
    if !view.asks.is_empty() {
        let mut asks = String::new();
        for (number, ask) in view.asks.iter().enumerate() {
            let (title, detail) = ask_line(view, ask);
            let title = match ask {
                Ask::Decision(id) => format!("Decide: {}", chip(view, id, 80)),
                Ask::NeedsHuman(id) => chip(view, id, 80),
                _ => escape(&title),
            };
            asks.push_str(&format!(
                "<div class=\"ask\"><span class=\"k\">{}</span><span class=\"t\">{title}</span>{}</div>",
                number + 1,
                if detail.is_empty() {
                    String::new()
                } else {
                    format!("<span class=\"d\">{}</span>", rich_inline(view, &detail))
                }
            ));
        }
        main.push_str(&format!(
            "<section id=\"review\"><h3>Needs your review</h3>{asks}</section>"
        ));
    }

    // Left to do, and found along the way.
    if !view.left.is_empty() {
        main.push_str(&format!(
            "<section id=\"left\"><h3>Left to do</h3>{}</section>",
            open_rows_html(view, &view.left, false)
        ));
    }
    if !view.found.is_empty() {
        main.push_str(&format!(
            "<section id=\"found\"><h3>Found along the way</h3>{}</section>",
            open_rows_html(view, &view.found, true)
        ));
    }

    review_page::review_document(&format!("Debrief: {}", mission.title), &side, &main)
}

/// One moment of the timeline: a delivered item in full, anything else as
/// one line.
fn event_html(view: &Debrief, at: Option<chrono::DateTime<chrono::Utc>>, event: &Event) -> String {
    let line = |class: &str, text: String| {
        format!(
            "<div class=\"entry ev\"><div class=\"time\">{}</div><div class=\"rail\"><span class=\"dot {class}\"></span></div><div class=\"body\">{text}</div></div>",
            time_rail(at)
        )
    };
    match event {
        Event::Delivered(id) => view
            .entry(id)
            .map(|entry| work_entry_html(view, entry))
            .unwrap_or_default(),
        Event::Started => line("mark", "<b>Mission started</b>".into()),
        Event::Closed => line("mark", "<b>Mission closed</b>".into()),
        Event::Added(id) => line(
            status_class(view, id),
            format!("Added to the mission: {}", chip(view, id, 64)),
        ),
        Event::Filed(id) => {
            let from_mission = view
                .found
                .iter()
                .any(|entry| &entry.id == id && entry.from.is_some());
            line(
                "back",
                format!(
                    "Filed{}: {}",
                    if from_mission {
                        " from review of the mission"
                    } else {
                        ""
                    },
                    chip(view, id, 64)
                ),
            )
        }
        Event::Dropped(id) => {
            let reason = view
                .dropped
                .iter()
                .find(|entry| &entry.id == id)
                .and_then(|entry| entry.reason.as_deref())
                .unwrap_or_default();
            line(
                "back",
                format!(
                    "Dropped {}{}",
                    chip(view, id, 64),
                    if reason.trim().is_empty() {
                        String::new()
                    } else {
                        format!(" <span class=\"note\">{}</span>", rich_inline(view, reason))
                    }
                ),
            )
        }
        Event::Validated(number) => {
            let run = &view.validation.runs[*number];
            let marks = judged(run)
                .iter()
                .map(|(slug, result)| {
                    let (class, mark) = if *result == EvidenceResult::Pass {
                        ("done", "✓")
                    } else {
                        ("warn", "✗")
                    };
                    format!(
                        "<span class=\"status {class}\">{mark} {}</span>",
                        escape(slug)
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            line(
                "mark",
                format!(
                    "<b>Validated</b> by {} at <code>{}</code>: {marks} <a href=\"#validation\">How</a>",
                    escape(&run.identity),
                    escape(short_commit(&run.commit))
                ),
            )
        }
        Event::MissionReview(number) => {
            let event = &view.mission_events[*number];
            line(
                "mark",
                format!(
                    "<b>{}</b> by {}: {} {}",
                    escape(&capitalise(event.event)),
                    escape(&event.identity),
                    rich_inline(view, &first_sentence(&event.summary)),
                    record_chip(view, &event.id)
                ),
            )
        }
    }
}

fn validation_html(view: &Debrief, closed: bool) -> String {
    let checks = &view.validation;
    let mut html = String::new();
    let mut fact = |label: &str, body: String| {
        html.push_str(&format!(
            "<div class=\"fact\"><span>{}</span><div>{body}</div></div>",
            escape(label)
        ));
    };
    let delivered =
        checks.approved + checks.trivial + checks.not_approved.len() + checks.unreviewed.len();
    if delivered > 0 {
        let mut text = format!(
            "{} of {delivered} delivered items approved by an independent reviewer",
            checks.approved
        );
        if checks.trivial > 0 {
            text.push_str(&format!(", {} waived as trivial", checks.trivial));
        }
        text.push('.');
        if let Some(counts) = finding_counts(&checks.findings) {
            text.push_str(&format!(" {}.", capitalise(&counts)));
        }
        let mut body = format!("<p>{}</p>", escape(&text));
        for (label, ids) in [
            ("Not approved", &checks.not_approved),
            ("No review recorded", &checks.unreviewed),
        ] {
            if !ids.is_empty() {
                body.push_str(&format!(
                    "<p class=\"changed\">{label}: {}</p>",
                    ids.iter()
                        .map(|id| chip(view, id, 48))
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
            }
        }
        fact("Each item", body);
    }
    for run in &checks.runs {
        let mut body = format!(
            "<p><b>{}</b> at <code>{}</code>",
            escape(&run.identity),
            escape(short_commit(&run.commit))
        );
        if let Some(count) = run.code_commits_since.filter(|count| *count > 0) {
            body.push_str(&format!(
                " <span class=\"age\">· {}</span>",
                escape(&commits_since(count, closed))
            ));
        }
        body.push_str("</p>");
        for receipt in &run.receipts {
            let marks = receipt
                .criteria
                .iter()
                .map(|criterion| {
                    let (class, mark) = if criterion.result == EvidenceResult::Pass {
                        ("done", "✓")
                    } else {
                        ("warn", "✗")
                    };
                    format!(
                        "<span class=\"status {class}\">{mark} {}</span>",
                        escape(&criterion.slug)
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            body.push_str(&format!(
                "<details class=\"rcpt\"><summary>{marks} <span class=\"sum\">{}</span> {}</summary>{}</details>",
                rich_inline(view, &first_sentence(&receipt.summary)),
                record_chip(view, &receipt.id),
                rich(view, &receipt.summary)
            ));
        }
        if !run.later_work.is_empty() {
            body.push_str(&format!(
                "<p class=\"changed\">Delivered after this validation: {}</p>",
                run.later_work
                    .iter()
                    .map(|id| chip(view, id, 48))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        fact(
            &run.at
                .with_timezone(&chrono::Local)
                .format("%-d %b %H:%M")
                .to_string(),
            body,
        );
    }
    for (slug, mark, by) in &checks.awaiting {
        fact(
            "Awaiting",
            format!(
                "<p><span class=\"s\">{}</span> {}{}</p>",
                escape(slug),
                escape(criterion_word(*mark)),
                by.as_deref()
                    .map(|by| format!(": {}", chip(view, by, 64)))
                    .unwrap_or_default()
            ),
        );
    }
    if html.is_empty() {
        html = "<p class=\"note\">No review or validation recorded.</p>".into();
    }
    html
}

fn plan_html(view: &Debrief) -> String {
    let mut html = String::new();
    let mut fact = |label: &str, body: String| {
        html.push_str(&format!(
            "<div class=\"fact\"><span>{}</span><div>{body}</div></div>",
            escape(label)
        ));
    };
    if view.planned_total > 0 {
        fact(
            "Planned",
            format!(
                "<p>{} of {} done</p>",
                view.planned_done,
                atelier_core::count_noun(view.planned_total, "item")
            ),
        );
    }
    let changed = view
        .work
        .iter()
        .filter(|entry| changed_course(entry))
        .map(|entry| format!("<li>{}</li>", chip(view, &entry.id, 64)))
        .collect::<String>();
    if !changed.is_empty() {
        fact(
            "Changed course",
            format!("<ul class=\"chips\">{changed}</ul>"),
        );
    }
    let added = view
        .added
        .iter()
        .map(|id| format!("<li>{}</li>", chip(view, id, 64)))
        .collect::<String>();
    if !added.is_empty() {
        fact("Added", format!("<ul class=\"chips\">{added}</ul>"));
    }
    let dropped = view
        .dropped
        .iter()
        .map(|entry| {
            format!(
                "<li>{} <span class=\"note\">{}</span>{}</li>",
                chip(view, &entry.id, 64),
                rich_inline(view, entry.reason.as_deref().unwrap_or_default()),
                changes_html(view, &entry.changes)
            )
        })
        .collect::<String>();
    if !dropped.is_empty() {
        fact("Dropped", format!("<ul class=\"chips\">{dropped}</ul>"));
    }
    if !view.found.is_empty() {
        fact(
            "Found",
            format!(
                "<p><a href=\"#found\">{} still open</a></p>",
                atelier_core::count_noun(view.found.len(), "item")
            ),
        );
    }
    if html.is_empty() {
        html = "<p class=\"note\">Delivered as planned.</p>".into();
    }
    html
}

fn open_rows_html(view: &Debrief, entries: &[OpenEntry], found: bool) -> String {
    entries
        .iter()
        .map(|entry| {
            let mut source = Vec::new();
            if found {
                match entry.from.as_deref() {
                    Some(from) if from == view.mission.id => {
                        source.push("from review of the mission".to_string());
                    }
                    Some(from) => {
                        source.push(format!("from review of {}", chip(view, from, 56)));
                    }
                    None => {}
                }
                if let Some(at) = entry.filed_at {
                    source.push(escape(
                        &at.with_timezone(&chrono::Local)
                            .format("%-d %b %H:%M")
                            .to_string(),
                    ));
                }
            } else if entry.added_after_start {
                source.push("added after the start".into());
            }
            let source = if source.is_empty() {
                String::new()
            } else {
                format!("<span class=\"from\">{}</span>", source.join(" · "))
            };
            format!(
                "<div class=\"row{}\">{}{}{source}</div>",
                if entry.parent.is_some() { " kid" } else { "" },
                chip(view, &entry.id, 72),
                changes_html(view, &entry.changes)
            )
        })
        .collect()
}

fn work_entry_html(view: &Debrief, entry: &WorkEntry) -> String {
    let issue = view.refs.get(&entry.id);
    let mut meta = vec![
        key_chip(view, &entry.id),
        escape(
            &issue
                .map(|issue| capitalise(&issue.issue_type))
                .unwrap_or_default(),
        ),
    ];
    if !entry.criteria.is_empty() {
        meta.push(escape(&entry.criteria.join(", ")));
    }
    if let Some(parent) = &entry.parent {
        meta.push(format!("part of {}", chip(view, parent, 40)));
    }
    if entry.added_after_start {
        meta.push("added after the start".into());
    }
    let mut facts = String::new();
    let mut fact = |label: &str, body: String| {
        facts.push_str(&format!(
            "<div class=\"fact\"><span>{}</span><div>{body}</div></div>",
            escape(label)
        ));
    };
    let mut changed = Vec::new();
    if let Some(text) = reported(entry, ReportPart::Deviations) {
        changed.push(rich(view, text));
    }
    changed.extend(
        entry
            .changes
            .iter()
            .map(|change| format!("<p>{}</p>", change_html(view, change))),
    );
    let notable = !changed.is_empty()
        || !entry.malformed.is_empty()
        || entry.decisions.iter().any(|note| note.decision.one_way);
    if !changed.is_empty() {
        fact(
            "Changed",
            format!("<div class=\"changed\">{}</div>", changed.join("")),
        );
    }
    if let Some(text) = reported(entry, ReportPart::Surprises) {
        fact("Surprised by", rich(view, text));
    }
    if !entry.decisions.is_empty() || !entry.malformed.is_empty() {
        let mut items = entry
            .decisions
            .iter()
            .map(|note| {
                format!(
                    "<li>{}{} <span class=\"r\">{}</span></li>",
                    if note.decision.one_way {
                        "<span class=\"oneway\">one-way</span>"
                    } else {
                        ""
                    },
                    rich_inline(view, &note.decision.choice),
                    rich_inline(view, &note.decision.reason)
                )
            })
            .collect::<String>();
        for note in &entry.malformed {
            items.push_str(&format!(
                "<li class=\"changed\">⚠ Malformed decision note {} on {}: {}</li>",
                note.number,
                chip(view, &note.issue_id, 40),
                rich_inline(view, &note.defect)
            ));
        }
        fact("Decided", format!("<ul>{items}</ul>"));
    }
    if let Some(record) = view
        .decisions
        .records
        .iter()
        .find(|record| record.issue.id == entry.id)
    {
        let mut cues = Vec::new();
        if let Some(adr) = &record.issue.adr {
            cues.push(format!("ADR <code>{}</code>", escape(&adr.path)));
        }
        for id in &record.supersedes {
            cues.push(format!("replaces {}", chip(view, id, 48)));
        }
        if !cues.is_empty() {
            fact("Record", format!("<p>{}</p>", cues.join(" · ")));
        }
    }
    if let Some(review) = review_line(view, entry) {
        fact("Review", format!("<p>{}</p>", escape(&review)));
    }
    if !entry.follow_ups.is_empty() {
        fact(
            "Filed",
            format!(
                "<ul class=\"chips\">{}</ul>",
                entry
                    .follow_ups
                    .iter()
                    .map(|id| format!("<li>{}</li>", chip(view, id, 72)))
                    .collect::<String>()
            ),
        );
    } else if let Some(text) = reported(entry, ReportPart::OpenItems) {
        fact("Left open", rich(view, text));
    }
    let children = if entry.children.is_empty() {
        String::new()
    } else {
        format!(
            "<ul class=\"chips\">{}</ul>",
            entry
                .children
                .iter()
                .map(|child| format!("<li>{}</li>", chip(view, child, 72)))
                .collect::<String>()
        )
    };
    let details = if notable || facts.is_empty() {
        facts
    } else {
        format!("<details class=\"more\"><summary>Review, decisions and follow-ups</summary>{facts}</details>")
    };
    format!(
        "<div class=\"entry{}\" id=\"{}\"><div class=\"time\">{}</div><div class=\"rail\"><span class=\"dot done\"></span></div><div class=\"body\"><div class=\"title\">{}</div><div class=\"meta\">{}</div><div class=\"outcome\">{}</div>{children}{details}</div></div>",
        if notable { "" } else { " quiet" },
        escape(&entry.id),
        time_rail(entry.closed_at),
        escape(view.title_of(&entry.id)),
        meta.join(" · "),
        rich(view, outcome(entry))
    )
}

/// A derived change, naming a receipt by its chip.
fn change_html(view: &Debrief, change: &Change) -> String {
    match change {
        Change::NonPassEvidence {
            evidence,
            result,
            summary,
        } => format!(
            "Newest proof {} is {}: {}",
            record_chip(view, evidence),
            escape(result.as_str()),
            rich_inline(view, &first_sentence(summary))
        ),
        other => rich_inline(view, &change_text(other)),
    }
}

/// Derived changes on an open row, as notes under its chip.
fn changes_html(view: &Debrief, changes: &[Change]) -> String {
    changes
        .iter()
        .map(|change| {
            format!(
                "<span class=\"changed note-line\">{}</span>",
                change_html(view, change)
            )
        })
        .collect()
}

fn capitalise(word: &str) -> String {
    let mut chars = word.chars();
    chars
        .next()
        .map(|first| first.to_uppercase().collect::<String>() + chars.as_str())
        .unwrap_or_default()
}

pub fn brief(index: &RecordIndex, mission_ref: &str, html: bool, quiet: bool) -> Result<()> {
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
    if html {
        let path = write_review_page(&repo_root, &id, "brief", &render_brief_html(&view))?;
        if quiet {
            println!("{}", path.display());
        } else {
            println!("Wrote the brief of {id} to {}", path.display());
            println!("Open it in T3 Code's file panel, or in any browser.");
        }
        return Ok(());
    }
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
            optional("Success Criteria", criteria),
            render_lines_section("Scope", scope),
            optional("Blockers", blocked),
            optional("Unknowns", unknowns),
            optional("Open Human Decisions", human),
            optional("Risks", risks),
            render_lines_section("Mission Reviews", events),
            render_footer_section(
                context,
                [
                    FooterAction::new(
                        "Write the HTML page",
                        format!("atelier brief {} --html", mission.id),
                    ),
                    FooterAction::new(
                        "Record the human's plan approval",
                        format!(
                            "atelier review approve {} --identity \"<human>\" \"<summary>\"",
                            mission.id
                        ),
                    ),
                ],
            ),
        ],
        context,
    );
    Ok(())
}

fn render_brief_html(view: &atelier_app::mission_review::Brief) -> String {
    let mission = &view.mission;
    let approvals = view
        .mission_events
        .iter()
        .filter(|event| event.event == "approval")
        .count();
    let open = view
        .items
        .iter()
        .filter(|item| item.status_category.as_deref() != Some("done"))
        .count();
    let mut body = format!(
        "<header><p class=\"meta\">Brief · <span class=\"id\">{}</span> · {}</p><h1>{}</h1>",
        escape(&mission.id),
        badge(
            &mission.status,
            status_tone(view.status_category.as_deref(), &mission.status)
        ),
        escape(&mission.title)
    );
    let mut facts = Vec::new();
    if let Some(strategy) = &view.strategy {
        facts.push(format!("Strategy <code>{}</code>", escape(strategy)));
    }
    if let Some(commit) = &view.generated_from {
        facts.push(format!(
            "generated from <code>{}</code>",
            escape(short_commit(commit))
        ));
    }
    body.push_str(&format!("<p class=\"meta\">{}</p>", facts.join(" · ")));
    body.push_str(&format!(
        "<div class=\"outcome\">{}</div><div class=\"tiles\">",
        prose(&view.outcome)
    ));
    body.push_str(&tile(
        view.criteria.parse.criteria.len(),
        "success criteria",
        "the claims the mission must show",
        Tone::Neutral,
    ));
    body.push_str(&tile(
        view.items.len(),
        "items in scope",
        &format!("{open} open"),
        Tone::Neutral,
    ));
    let blocked = view.blocked().count();
    body.push_str(&tile(
        blocked,
        "blocked",
        "items waiting on open blockers",
        if blocked > 0 { Tone::Warn } else { Tone::Good },
    ));
    body.push_str(&tile(
        view.human_decisions.len(),
        "human decisions",
        "open decisions and needs-human work",
        if view.human_decisions.is_empty() {
            Tone::Good
        } else {
            Tone::Warn
        },
    ));
    body.push_str(&tile(
        approvals,
        "plan approvals",
        "review events on the mission",
        if approvals > 0 {
            Tone::Good
        } else {
            Tone::Warn
        },
    ));
    body.push_str("</div><nav class=\"toc\"><a href=\"#criteria\">Criteria</a><a href=\"#scope\">Scope</a><a href=\"#unknowns\">Unknowns</a><a href=\"#decisions\">Human decisions</a><a href=\"#risks\">Risks</a><a href=\"#reviews\">Reviews</a></nav></header>");

    let mut criteria =
        String::from("<table><thead><tr><th>Criterion</th><th>Claim</th></tr></thead><tbody>");
    for criterion in &view.criteria.parse.criteria {
        criteria.push_str(&format!(
            "<tr><td class=\"id\">{}</td><td>{}</td></tr>",
            escape(&criterion.slug),
            inline(&criterion.text)
        ));
    }
    criteria.push_str("</tbody></table>");
    for problem in &view.criteria.parse.problems {
        criteria.push_str(&format!(
            "<p>{} {}</p>",
            badge("problem", Tone::Bad),
            escape(problem)
        ));
    }
    if view.criteria.parse.criteria.is_empty() && view.criteria.parse.problems.is_empty() {
        criteria = "<p class=\"muted\">None defined.</p>".to_string();
    }
    body.push_str(&section("criteria", "Success Criteria", &criteria));

    let mut scope = String::from("<table><thead><tr><th>Item</th><th>Status</th><th>Size</th><th>Title</th><th>Blocked by</th></tr></thead><tbody>");
    for item in &view.items {
        let issue = &item.issue;
        scope.push_str(&format!(
            "<tr><td class=\"id\">{}{}</td><td>{}</td><td>{}</td><td>{}{} <span class=\"muted\">{}</span></td><td class=\"id\">{}</td></tr>",
            if item.parent.is_some() { "↳ " } else { "" },
            escape(&issue.id),
            badge(&issue.status, status_tone(item.status_category.as_deref(), &issue.status)),
            escape(item.size.as_deref().unwrap_or("")),
            escape(&issue.title),
            if item.needs_human { format!(" {}", badge("needs a human", Tone::Warn)) } else { String::new() },
            escape(&issue.issue_type),
            escape(&item.open_blockers.join(", "))
        ));
    }
    scope.push_str("</tbody></table>");
    body.push_str(&section("scope", "Scope", &scope));

    let unknowns = view
        .items
        .iter()
        .filter_map(|item| Some((item, item.unknowns.as_deref()?)))
        .map(|(item, text)| {
            format!(
                "<dt class=\"id\">{}</dt><dd>{}</dd>",
                escape(&item.issue.id),
                prose(text)
            )
        })
        .collect::<String>();
    body.push_str(&section(
        "unknowns",
        "Unknowns",
        &if unknowns.is_empty() {
            "<p class=\"muted\">None recorded.</p>".to_string()
        } else {
            format!("<dl>{unknowns}</dl>")
        },
    ));
    let decisions = view
        .human_decisions
        .iter()
        .map(|issue| {
            format!(
                "<li><span class=\"id\">{}</span> {} {}</li>",
                escape(&issue.id),
                escape(&issue.issue_type),
                escape(&issue.title)
            )
        })
        .collect::<String>();
    body.push_str(&section(
        "decisions",
        "Open Human Decisions",
        &if decisions.is_empty() {
            "<p class=\"muted\">None.</p>".to_string()
        } else {
            format!("<ul class=\"facts\">{decisions}</ul>")
        },
    ));
    body.push_str(&section(
        "risks",
        "Risks",
        &view.risks.as_deref().map(prose).unwrap_or_else(|| {
            if view.strategy.is_some() {
                "<p class=\"muted\">The strategy names none.</p>".to_string()
            } else {
                "<p class=\"muted\">No strategy is linked.</p>".to_string()
            }
        }),
    ));
    let mut events = view
        .mission_events
        .iter()
        .map(event_line)
        .collect::<Vec<_>>();
    if approvals == 0 {
        events.push(NO_PLAN_APPROVAL.to_string());
    }
    let events = events
        .iter()
        .map(|line| format!("<li>{}</li>", escape(line)))
        .collect::<String>();
    body.push_str(&section(
        "reviews",
        "Mission Reviews",
        &format!("<ul class=\"facts\">{events}</ul>"),
    ));
    review_page::page(
        &format!("Brief: {} {}", mission.id, mission.title),
        &body,
        &format!(
            "Generated by atelier brief {}. A read-only document; record approval with atelier review approve {}.",
            mission.id, mission.id
        ),
    )
}
