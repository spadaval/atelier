//! `atelier debrief` (atelier-rx22, atelier-nexd): a mission's review,
//! rendered from one view model as dense Markdown for agents (ADR 0022) and,
//! with `--html`, as a static page a human opens in T3 Code's file panel.
//! Both lead with what needs the human, then why the mission existed, its
//! criteria, its work in closing order, the open work and the decisions.

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use atelier_app::criteria::{CriterionMark, CriterionProof};
use atelier_app::decisions::DecisionRow;
use atelier_app::mission_review::{Ask, Change, Debrief, ReviewSummary, WorkEntry};
use atelier_app::record_index::RecordIndex;
use atelier_core::{CloseReport, ReportPart};

use crate::human_output::{
    decision_sections, local_date, print_page, render_footer_section, render_lines_section,
    render_metadata, render_row_wrapped, short_commit, FooterAction, ItemLine, RenderContext,
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
        render_markdown(&view, policy.as_ref(), context),
        context,
    );
    Ok(())
}

/// `criteria=<n> proven=<n> failing=<n> unproven=<n> items=<n> decisions=<n>
/// deviations=<n> follow_ups=<n>`.
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
        .map(|entry| 1 + entry.children.len())
        .sum::<usize>()
        + view.open.len()
        + view.dropped.len();
    format!(
        "criteria={} proven={} failing={} unproven={} items={items} decisions={} deviations={} follow_ups={}",
        view.criteria.proofs.len(),
        marks(CriterionMark::Proven),
        marks(CriterionMark::Failing),
        marks(CriterionMark::Unproven),
        view.decisions.notes.len() + view.decisions.records.len(),
        deviation_count(view),
        view.open.iter().filter(|entry| entry.from.is_some()).count()
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
            .open
            .iter()
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

/// One review event on the mission, as both reviews and both forms show it.
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
    Some(format!(
        "{} {}",
        atelier_core::count_noun(count, "code commit"),
        if closed { "before close" } else { "since" }
    ))
}

fn change_line(change: &Change) -> String {
    change_text(change, true)
}

/// A derived change in words; the HTML leaves out the receipt's ID.
fn change_text(change: &Change, with_ids: bool) -> String {
    match change {
        Change::Reopened(time) => format!("Reopened on {} after a done claim.", local_date(*time)),
        Change::NonPassEvidence {
            evidence,
            result,
            summary,
        } if with_ids => format!("Newest proof {evidence} is {}: {summary}", result.as_str()),
        Change::NonPassEvidence {
            result, summary, ..
        } => {
            format!("Newest proof is {}: {summary}", result.as_str())
        }
        Change::TrivialExemption { summary } => format!("Review waived as trivial: {summary}"),
    }
}

/// How review went; `None` for a decision record, which is decided rather
/// than code-reviewed.
fn review_line(view: &Debrief, entry: &WorkEntry) -> Option<String> {
    let review = &entry.review;
    if view
        .refs
        .get(&entry.id)
        .is_some_and(|issue| issue.issue_type == "decision")
    {
        return None;
    }
    if !review.recorded {
        return Some("No review recorded.".into());
    }
    Some(review_summary(review))
}

fn review_summary(review: &ReviewSummary) -> String {
    let mut parts = Vec::new();
    for (count, word) in [
        (review.fixed, "fixed"),
        (review.deferred, "deferred"),
        (review.no_action, "left as is"),
        (review.undisposed, "awaiting a decision"),
    ] {
        if count > 0 {
            parts.push(format!("{count} {word}"));
        }
    }
    let verdict = if review.approved {
        "Approved"
    } else if review.trivial {
        "Waived as trivial"
    } else {
        "Not approved"
    };
    let findings = review.fixed + review.deferred + review.no_action + review.undisposed;
    if findings == 0 {
        format!("{verdict}; no findings.")
    } else {
        format!(
            "{verdict}; {}: {}.",
            atelier_core::count_noun(findings, "finding"),
            parts.join(", ")
        )
    }
}

fn ask_line(view: &Debrief, ask: &Ask) -> (String, String) {
    match ask {
        Ask::Decision(id) => (
            format!("Decide: {}", view.title_of(id)),
            view.refs
                .get(id)
                .map(|issue| issue.summary.clone())
                .unwrap_or_default(),
        ),
        Ask::NeedsHuman(id) => (
            view.title_of(id).to_string(),
            view.refs
                .get(id)
                .map(|issue| issue.summary.clone())
                .unwrap_or_default(),
        ),
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
        .map(|proof| atelier_app::mission_review::first_sentence(&proof.criterion.text))
        .unwrap_or_default()
}

fn last_delivery(view: &Debrief) -> Option<chrono::DateTime<chrono::Utc>> {
    view.work.iter().filter_map(|entry| entry.closed_at).max()
}

fn render_markdown(
    view: &Debrief,
    policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
    context: RenderContext,
) -> Vec<Vec<String>> {
    let closed = view.timeline.closed.is_some();
    let flat = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut metadata = vec![("Outcome", flat(&view.outcome))];
    if let Some(started) = &view.timeline.started {
        metadata.push((
            "Started",
            started
                .time
                .with_timezone(&chrono::Local)
                .format("%Y-%m-%d %H:%M")
                .to_string(),
        ));
    }
    if let Some(closed) = &view.timeline.closed {
        metadata.push((
            "Closed",
            closed
                .time
                .with_timezone(&chrono::Local)
                .format("%Y-%m-%d %H:%M")
                .to_string(),
        ));
    } else if let Some(last) = last_delivery(view) {
        metadata.push((
            "Last delivery",
            last.with_timezone(&chrono::Local)
                .format("%Y-%m-%d %H:%M")
                .to_string(),
        ));
    }
    if let Some(strategy) = &view.strategy {
        metadata.push(("Strategy", strategy.clone()));
    }
    if let Some(commit) = &view.generated_from {
        metadata.push(("Generated from", short_commit(commit).to_string()));
    }
    let wrap = |line: String| render_row_wrapped(context, &line);
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
    let why = view
        .why
        .as_deref()
        .map(|text| {
            text.split("\n\n")
                .map(flat)
                .filter(|paragraph| !paragraph.is_empty())
                .flat_map(&wrap)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let mut criteria = Vec::new();
    for proof in &view.criteria.proofs {
        let slug = &proof.criterion.slug;
        let mut row = format!(
            "{slug} {} {}",
            criterion_word(proof.mark),
            atelier_app::mission_review::first_sentence(&proof.criterion.text)
        );
        let by = view.delivered_by.get(slug).cloned().unwrap_or_default();
        if !by.is_empty() {
            row.push_str(&format!(" | by {}", by.join(", ")));
        }
        if let Some(cue) = proof_cue(proof, closed) {
            row.push_str(&format!(" | {cue}"));
        }
        criteria.extend(wrap(row));
    }
    for problem in &view.criteria.criteria.parse.problems {
        criteria.extend(wrap(format!("⚠ {problem}")));
    }

    let mut work = Vec::new();
    for entry in &view.work {
        let when = entry
            .closed_at
            .map(|time| {
                time.with_timezone(&chrono::Local)
                    .format("%Y-%m-%d %H:%M")
                    .to_string()
            })
            .unwrap_or_else(|| "undated".into());
        let mut head = format!("{when} {}", item(&entry.id));
        if !entry.criteria.is_empty() {
            head.push_str(&format!(" | {}", entry.criteria.join(", ")));
        }
        if let Some(parent) = &entry.parent {
            head.push_str(&format!(" | under {parent}"));
        }
        if entry.added_after_start {
            head.push_str(" | added after the start");
        }
        work.extend(wrap(head));
        let mut fact = |label: &str, text: &str| {
            let line = if label == "└" {
                format!("└ {}", flat(text))
            } else {
                format!("{label}: {}", flat(text))
            };
            for line in wrap(line) {
                work.push(format!("  {line}"));
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
            fact("Changed", &change_line(change));
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
        if let Some(review) = review_line(view, entry) {
            fact("Review", &review);
        }
        for follow_up in &entry.follow_ups {
            fact("Left open", &item(follow_up));
        }
        if entry.follow_ups.is_empty() {
            if let Some(text) = reported(entry, ReportPart::OpenItems) {
                fact("Left open", text);
            }
        }
    }

    let mut open = Vec::new();
    // Planned work first, then review's follow-ups, then dropped work.
    let ordered = view
        .open
        .iter()
        .filter(|entry| entry.from.is_none())
        .chain(view.open.iter().filter(|entry| entry.from.is_some()));
    for entry in ordered {
        let indent = if entry.parent.is_some() { "  " } else { "" };
        let mut row = format!("{indent}{}", item(&entry.id));
        if let Some(from) = &entry.from {
            if from == &view.mission.id {
                row.push_str(" | from review of the mission");
            } else {
                row.push_str(&format!(" | from {from}"));
            }
        }
        if entry.added_after_start {
            row.push_str(" | added after the start");
        }
        for change in &entry.changes {
            row.push_str(&format!(" | {}", change_line(change)));
        }
        open.extend(wrap(row));
    }
    for entry in &view.dropped {
        let mut row = format!(
            "dropped {} | {}",
            item(&entry.id),
            flat(entry.reason.as_deref().unwrap_or_default())
        );
        for change in &entry.changes {
            row.push_str(&format!(" | {}", change_line(change)));
        }
        open.extend(wrap(row));
    }
    let events = view
        .mission_events
        .iter()
        .flat_map(|event| wrap(event_line(event)))
        .collect::<Vec<_>>();
    let optional = |title: &str, lines: Vec<String>| {
        if lines.is_empty() {
            Vec::new()
        } else {
            render_lines_section(title, lines)
        }
    };
    let mut panels = vec![
        render_metadata(None, &metadata),
        optional("Needs Your Review", asks),
        optional("Why", why),
        optional("Success Criteria", criteria),
        optional("Work", work),
        optional("Open Work", open),
    ];
    panels.extend(decision_sections(context, policy, &view.decisions));
    panels.push(optional("Mission Reviews", events));
    panels.push(render_footer_section(
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
    ));
    panels
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

/// An issue named by its title; hovering or focusing it shows its full
/// title, summary, type, status and ID.
fn issue_ref(view: &Debrief, id: &str, label: Option<&str>) -> String {
    let Some(issue) = view.refs.get(id) else {
        return escape(label.unwrap_or(id));
    };
    format!(
        "<span class=\"ref\" tabindex=\"0\">{}<span class=\"pop\"><b>{}</b>{}<span class=\"m\">{} · {} · {}</span></span></span>",
        escape(label.unwrap_or(&issue.title)),
        escape(&issue.title),
        inline(&issue.summary),
        escape(&issue.issue_type),
        escape(&issue.status),
        escape(&issue.id)
    )
}

/// Record text as HTML with each mention of a known record turned into a
/// popover reference: an issue by its title, a review or receipt by its ID.
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
            let title = view.title_of(id);
            let label = if title.chars().count() > 60 {
                format!("{}…", title.chars().take(59).collect::<String>())
            } else {
                title.to_string()
            };
            out.push_str(&issue_ref(view, id, Some(&label)));
        } else if let Some(record) = view.record_refs.get(id).filter(|_| bounded) {
            out.push_str(&format!(
                "<span class=\"ref\" tabindex=\"0\">{}<span class=\"pop\"><b>{}</b>{}<span class=\"m\">{}</span></span></span>",
                escape(id),
                escape(&capitalise(&record.kind)),
                inline(&record.summary),
                escape(id)
            ));
        } else {
            out.push_str(id);
        }
        rest = &rest[at + id.len()..];
    }
    out.push_str(rest);
    out
}

/// The status dot's class for an issue.
fn dot(view: &Debrief, id: &str) -> &'static str {
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
    let proven = view
        .criteria
        .proofs
        .iter()
        .filter(|proof| proof.mark == CriterionMark::Proven)
        .count();
    let percent = |part: usize, whole: usize| if whole == 0 { 0 } else { part * 100 / whole };

    // Sidebar.
    let mut side = format!("<h1>{}</h1>", escape(&mission.title));
    let status_word = match (view.status_category.as_deref(), mission.status.as_str()) {
        (Some("done"), _) => "Done".to_string(),
        (_, "draft") => "Draft".to_string(),
        _ => "In progress".to_string(),
    };
    side.push_str(&format!(
        "<span class=\"status {}\">{}{}</span>",
        if view.asks.is_empty() { "done" } else { "warn" },
        escape(&status_word),
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
            "<a href=\"#review\"><span class=\"dot warn\"></span>Needs your review <span class=\"n\">{}</span></a>",
            view.asks.len()
        ));
    }
    if view.why.is_some() {
        side.push_str("<a href=\"#why\">Why</a>");
    }
    if total > 0 {
        side.push_str(&format!(
            "<a href=\"#criteria\">Success criteria <span class=\"n\">{proven}/{total}</span></a>"
        ));
    }
    if !view.work.is_empty() {
        side.push_str("<div class=\"g\">Work</div>");
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
                    dot(view, child),
                    escape(view.title_of(child))
                ));
            }
        }
    }
    side.push_str("<div class=\"g\">After</div>");
    if !view.open.is_empty() || !view.dropped.is_empty() {
        side.push_str(&format!(
            "<a href=\"#open\"><span class=\"dot back\"></span>Open work <span class=\"n\">{}</span></a>",
            view.open.len()
        ));
    }
    let decision_count = view.decisions.notes.len() + view.decisions.records.len();
    if decision_count > 0 {
        side.push_str(&format!(
            "<a href=\"#decisions\">Decisions <span class=\"n\">{decision_count}</span></a>"
        ));
    }
    if !view.mission_events.is_empty() {
        side.push_str("<a href=\"#reviews\">Mission reviews</a>");
    }
    side.push_str("</nav>");

    // Header.
    let mut main = format!("<h2>{}</h2><div class=\"props\">", escape(&mission.title));
    main.push_str(&format!("<span>{}</span>", escape(&status_word)));
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
        let name = strategy.rsplit('/').next().unwrap_or(strategy);
        main.push_str(&format!("<span><code>{}</code></span>", escape(name)));
    }
    if let Some(commit) = &view.generated_from {
        main.push_str(&format!(
            "<span>at <code>{}</code></span>",
            escape(short_commit(commit))
        ));
    }
    main.push_str("</div>");
    if !view.outcome.trim().is_empty() {
        main.push_str(&format!(
            "<div class=\"lede\">{}</div>",
            rich(view, &view.outcome)
        ));
    }

    // Needs your review.
    if !view.asks.is_empty() {
        let mut asks = String::new();
        for (number, ask) in view.asks.iter().enumerate() {
            let (title, detail) = ask_line(view, ask);
            let title = match ask {
                Ask::Decision(id) => format!("Decide: {}", issue_ref(view, id, None)),
                Ask::NeedsHuman(id) => issue_ref(view, id, None),
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

    // Why.
    if let Some(why) = &view.why {
        main.push_str(&format!(
            "<section id=\"why\" class=\"why\"><h3>Why</h3>{}</section>",
            rich(view, why)
        ));
    }

    // Criteria.
    if total > 0 || !view.criteria.criteria.parse.problems.is_empty() {
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
                .map(|id| {
                    let anchor = view
                        .work
                        .iter()
                        .find(|entry| &entry.id == id || entry.children.contains(id))
                        .map(|entry| entry.id.as_str())
                        .unwrap_or(id);
                    format!(
                        "<a href=\"#{}\">{}</a>",
                        escape(anchor),
                        escape(view.title_of(id))
                    )
                })
                .collect::<Vec<_>>();
            let mut by = by.join(", ");
            if let Some(age) = proof_age(proof, closed) {
                by.push_str(&format!(
                    " <span class=\"age\">· proof {}</span>",
                    escape(&age)
                ));
            }
            let proof_title = proof_cue(proof, closed)
                .map(|cue| format!(" title=\"{}\"", escape(&cue)))
                .unwrap_or_default();
            rows.push_str(&format!(
                "<div class=\"crit\"><span class=\"s\">{}</span><span class=\"status {class}\"{proof_title}>{word}</span><span>{}</span><span class=\"by\">{by}</span></div>",
                escape(slug),
                rich_inline(view, &atelier_app::mission_review::first_sentence(&proof.criterion.text))
            ));
        }
        for problem in &view.criteria.criteria.parse.problems {
            rows.push_str(&format!(
                "<div class=\"crit\"><span class=\"s\"></span><span class=\"status warn\">⚠</span><span>{}</span><span></span></div>",
                escape(problem)
            ));
        }
        main.push_str(&format!(
            "<section id=\"criteria\"><h3>Success criteria</h3>{rows}</section>"
        ));
    }

    // Work.
    if !view.work.is_empty() {
        let mut entries = String::new();
        for entry in &view.work {
            entries.push_str(&work_entry_html(view, entry));
        }
        main.push_str(&format!("<section><h3>Work</h3>{entries}</section>"));
    }

    // Open work.
    if !view.open.is_empty() || !view.dropped.is_empty() {
        let mut rows = String::new();
        let planned = view.open.iter().filter(|entry| entry.from.is_none());
        let from_review = view.open.iter().filter(|entry| entry.from.is_some());
        for entry in planned {
            let status = view
                .refs
                .get(&entry.id)
                .map(|issue| issue.status.as_str())
                .unwrap_or("");
            rows.push_str(&format!(
                "<div class=\"row{}\"><span class=\"dot {}\"></span><span>{}{}{}</span><span class=\"from\">{}</span></div>",
                if entry.parent.is_some() { " kid" } else { "" },
                dot(view, &entry.id),
                issue_ref(view, &entry.id, None),
                if entry.added_after_start { " <span class=\"note\">added after the start</span>" } else { "" },
                changes_html(view, &entry.changes),
                escape(status)
            ));
        }
        let from_review = from_review.collect::<Vec<_>>();
        if !from_review.is_empty() {
            rows.push_str("<div class=\"sub\">From review</div>");
            for entry in from_review {
                let from = entry.from.as_deref().unwrap_or_default();
                let source = if from == view.mission.id {
                    "review of the mission".to_string()
                } else if view.work.iter().any(|work| work.id == from) {
                    format!(
                        "<a href=\"#{}\">{}</a>",
                        escape(from),
                        escape(view.title_of(from))
                    )
                } else {
                    issue_ref(view, from, None)
                };
                rows.push_str(&format!(
                    "<div class=\"row\"><span class=\"dot {}\"></span><span>{}</span><span class=\"from\">{source}</span></div>",
                    dot(view, &entry.id),
                    issue_ref(view, &entry.id, None)
                ));
            }
        }
        if !view.dropped.is_empty() {
            rows.push_str("<div class=\"sub\">Dropped</div>");
            for entry in &view.dropped {
                rows.push_str(&format!(
                    "<div class=\"row\"><span class=\"dot back\"></span><span>{} <span class=\"note\">{}</span>{}</span><span></span></div>",
                    issue_ref(view, &entry.id, None),
                    rich_inline(view, entry.reason.as_deref().unwrap_or_default()),
                    changes_html(view, &entry.changes)
                ));
            }
        }
        main.push_str(&format!(
            "<section id=\"open\"><h3>Open work</h3>{rows}</section>"
        ));
    }

    // Decisions.
    if decision_count > 0 {
        let mut rows = String::new();
        for (title, group) in view.decisions.groups() {
            if !matches!(title, "One-way Decisions" | "Reversible Decisions") {
                rows.push_str(&format!("<div class=\"sub\">{}</div>", escape(title)));
            }
            for row in group {
                rows.push_str(&decision_row_html(view, row));
            }
        }
        main.push_str(&format!(
            "<section id=\"decisions\"><h3>Decisions</h3>{rows}</section>"
        ));
    }

    // Mission reviews.
    if !view.mission_events.is_empty() {
        let rows = view
            .mission_events
            .iter()
            .map(|event| {
                format!(
                    "<div class=\"row\"><span></span><span>{}</span><span></span></div>",
                    rich_inline(view, &event_line(event))
                )
            })
            .collect::<String>();
        main.push_str(&format!(
            "<section id=\"reviews\"><h3>Mission reviews</h3>{rows}</section>"
        ));
    }

    review_page::review_document(&format!("Debrief: {}", mission.title), &side, &main)
}

fn decision_row_html(view: &Debrief, row: DecisionRow<'_>) -> String {
    let anchor = |id: &str| {
        view.work
            .iter()
            .find(|entry| entry.id == id || entry.children.iter().any(|child| child == id))
            .map(|entry| {
                format!(
                    "<a href=\"#{}\">{}</a>",
                    escape(&entry.id),
                    escape(view.title_of(&entry.id))
                )
            })
            .unwrap_or_else(|| issue_ref(view, id, None))
    };
    match row {
        DecisionRow::Note(note) => format!(
            "<div class=\"dec\"><span>{}{}</span><span class=\"r\">{}{}</span><span class=\"w\">{}</span></div>",
            if note.decision.one_way { "<span class=\"oneway\">one-way</span>" } else { "" },
            rich_inline(view, &note.decision.choice),
            rich_inline(view, &note.decision.reason),
            if note.decision.rejected.is_empty() {
                String::new()
            } else {
                format!(" Rejected: {}.", rich_inline(view, &note.decision.rejected.join("; ")))
            },
            anchor(&note.issue_id)
        ),
        DecisionRow::Record(record) => {
            let mut cues = Vec::new();
            if let Some(adr) = &record.issue.adr {
                cues.push(format!("ADR <code>{}</code>", escape(&adr.path)));
            }
            for id in &record.supersedes {
                cues.push(format!("replaces {}", issue_ref(view, id, None)));
            }
            for id in &record.superseded_by {
                cues.push(format!("replaced by {}", issue_ref(view, id, None)));
            }
            let summary = view
                .refs
                .get(&record.issue.id)
                .map(|issue| issue.summary.clone())
                .unwrap_or_default();
            format!(
                "<div class=\"dec\"><span>{}</span><span class=\"r\">{}{}</span><span class=\"w\">{}</span></div>",
                issue_ref(view, &record.issue.id, None),
                rich_inline(view, &summary),
                if cues.is_empty() { String::new() } else { format!(" {}", cues.join(" · ")) },
                escape(&record.issue.status)
            )
        }
        DecisionRow::Malformed(note) => format!(
            "<div class=\"dec\"><span class=\"status warn\">⚠ Malformed decision note</span><span class=\"r\">{}</span><span class=\"w\">{}</span></div>",
            rich_inline(view, &note.defect),
            anchor(&note.issue_id)
        ),
    }
}

fn work_entry_html(view: &Debrief, entry: &WorkEntry) -> String {
    let issue = view.refs.get(&entry.id);
    let mut meta = vec![issue
        .map(|issue| capitalise(&issue.issue_type))
        .unwrap_or_default()];
    if !entry.criteria.is_empty() {
        meta.push(entry.criteria.join(", "));
    }
    if let Some(parent) = &entry.parent {
        meta.push(format!("part of {}", view.title_of(parent)));
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
            .map(|change| format!("<p>{}</p>", rich_inline(view, &change_text(change, false)))),
    );
    let notable = !changed.is_empty() || entry.decisions.iter().any(|note| note.decision.one_way);
    if !changed.is_empty() {
        fact(
            "Changed",
            format!("<div class=\"changed\">{}</div>", changed.join("")),
        );
    }
    if let Some(text) = reported(entry, ReportPart::Surprises) {
        fact("Surprised by", rich(view, text));
    }
    if !entry.decisions.is_empty() {
        let items = entry
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
        fact("Decided", format!("<ul>{items}</ul>"));
    }
    if let Some(review) = review_line(view, entry) {
        fact("Review", format!("<p>{}</p>", escape(&review)));
    }
    let mut left = entry
        .follow_ups
        .iter()
        .map(|id| format!("<li>{}</li>", issue_ref(view, id, None)))
        .collect::<String>();
    if let Some(text) = reported(entry, ReportPart::OpenItems) {
        if left.is_empty() {
            left = rich(view, text);
        }
    }
    if !left.is_empty() {
        let body = if left.starts_with("<li>") {
            format!("<ul>{left}</ul>")
        } else {
            left
        };
        fact("Left open", body);
    }
    let children = if entry.children.is_empty() {
        String::new()
    } else {
        format!(
            "<ul class=\"kids\">{}</ul>",
            entry
                .children
                .iter()
                .map(|child| format!(
                    "<li><span class=\"dot {}\"></span>{}</li>",
                    dot(view, child),
                    issue_ref(view, child, None)
                ))
                .collect::<String>()
        )
    };
    let details = if notable {
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
        escape(&meta.join(" · ")),
        rich(view, outcome(entry))
    )
}

/// Derived changes on an open or dropped row, as notes under its title.
fn changes_html(view: &Debrief, changes: &[Change]) -> String {
    changes
        .iter()
        .map(|change| {
            format!(
                "<span class=\"changed note-line\">{}</span>",
                rich_inline(view, &change_text(change, false))
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
