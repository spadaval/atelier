//! `atelier debrief` (atelier-rx22): a finished mission's review, rendered
//! from one view model as dense Markdown for agents (ADR 0022) and, with
//! `--html`, as a static page a human opens in T3 Code's file panel.

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use atelier_app::criteria::{CriterionMark, CriterionProof};
use atelier_app::mission_review::{Debrief, Deviation, ItemReview};
use atelier_app::record_index::RecordIndex;
use atelier_core::{CloseReport, ReportPart};

use crate::human_output::{
    decision_full_rows, print_page, render_footer_section, render_lines_section, render_metadata,
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
        let path = write_review_page(
            &repo_root,
            &id,
            "debrief",
            &render_html(&view, policy.as_ref()),
        )?;
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
    format!(
        "criteria={} proven={} failing={} unproven={} items={} decisions={} deviations={} follow_ups={}",
        view.criteria.proofs.len(),
        marks(CriterionMark::Proven),
        marks(CriterionMark::Failing),
        marks(CriterionMark::Unproven),
        view.items.len(),
        view.decisions.notes.len() + view.decisions.records.len(),
        view.deviations.len(),
        view.follow_ups.len()
    )
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

fn criterion_mark(mark: CriterionMark) -> &'static str {
    match mark {
        CriterionMark::Proven => "✓ proven",
        CriterionMark::Failing => "⚠ failing",
        CriterionMark::Unproven => "unproven",
    }
}

fn criterion_proof_cue(proof: &CriterionProof) -> Option<String> {
    let receipt = proof.deciding()?;
    let mut cue = format!(
        "{} {} at {}",
        receipt.evidence_id,
        receipt.result.as_str(),
        short_commit(&receipt.commit)
    );
    match receipt.code_commits_since {
        Some(0) | None => {}
        Some(1) => cue.push_str(" | 1 code commit before close"),
        Some(count) => cue.push_str(&format!(" | {count} code commits before close")),
    }
    Some(cue)
}

/// Each deviation as one line of the Markdown form.
pub(crate) fn deviation_line(deviation: &Deviation) -> String {
    match deviation {
        Deviation::ScopeAdded { id, date } => format!("scope added {id} after the start, {date}"),
        Deviation::ScopeDropped { id, reason } => format!("scope dropped {id} | {reason}"),
        Deviation::DeferredFinding {
            finding,
            target,
            follow_up,
            summary,
        } => format!("deferred finding {finding} on {target} to {follow_up} | {summary}"),
        Deviation::NonPassEvidence {
            evidence,
            target,
            result,
            summary,
        } => format!(
            "non-pass evidence {evidence} on {target} {} | {summary}",
            result.as_str()
        ),
        Deviation::TrivialExemption {
            review,
            target,
            summary,
        } => format!("trivial exemption {review} on {target} | {summary}"),
        Deviation::Reopened { id, date } => format!("reopened {id}, {date}"),
        Deviation::AgedProof {
            slug,
            evidence,
            commits,
        } => format!(
            "aged proof {slug} by {evidence} | {} after it before close",
            atelier_core::count_noun(*commits, "code commit")
        ),
    }
}

/// The report's parts in order, each `(title, text)`; an older record's
/// Close Reason is its Outcome alone.
fn report_parts(report: &CloseReport) -> Vec<(&'static str, &str)> {
    let mut parts = vec![("Outcome", report.outcome.as_str())];
    for part in ReportPart::ALL {
        if let Some(text) = report.part(part) {
            parts.push((part.title(), text));
        }
    }
    parts
}

fn render_markdown(
    view: &Debrief,
    policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
    context: RenderContext,
) -> Vec<Vec<String>> {
    let mut metadata = vec![("Outcome", view.outcome.replace('\n', " "))];
    if let Some(strategy) = &view.strategy {
        metadata.push(("Strategy", strategy.clone()));
    }
    if let Some(started) = &view.timeline.started {
        metadata.push((
            "Started",
            format!(
                "{} at {}",
                started.time.format("%Y-%m-%d"),
                short_commit(&started.commit)
            ),
        ));
    }
    if let Some(closed) = &view.timeline.closed {
        metadata.push((
            "Closed",
            format!(
                "{} at {}",
                closed.time.format("%Y-%m-%d"),
                short_commit(&closed.commit)
            ),
        ));
    }
    if let Some(commit) = &view.generated_from {
        metadata.push(("Generated from", short_commit(commit).to_string()));
    }

    let mut criteria = Vec::new();
    for proof in &view.criteria.proofs {
        let mut row = format!(
            "{} {} {}",
            proof.criterion.slug,
            criterion_mark(proof.mark),
            proof.criterion.text
        );
        if let Some(cue) = criterion_proof_cue(proof) {
            row.push_str(&format!(" | {cue}"));
        }
        criteria.extend(render_row_wrapped(context, &row));
    }
    for problem in &view.criteria.criteria.parse.problems {
        criteria.extend(render_row_wrapped(context, &format!("⚠ {problem}")));
    }

    let mut items = Vec::new();
    for item in &view.items {
        let indent = if item.parent.is_some() { "  " } else { "" };
        items.extend(item_line(item).render_wrapped(context, indent));
        if let Some(report) = &item.report {
            for (title, text) in report_parts(report) {
                let body = text.lines().map(str::trim).collect::<Vec<_>>().join(" ");
                for line in render_row_wrapped(context, &format!("{title}: {body}")) {
                    items.push(format!("{indent}  {line}"));
                }
            }
        }
    }

    let deviations = view
        .deviations
        .iter()
        .flat_map(|deviation| render_row_wrapped(context, &deviation_line(deviation)))
        .collect::<Vec<_>>();
    let events = view
        .mission_events
        .iter()
        .flat_map(|event| {
            render_row_wrapped(
                context,
                &format!(
                    "{} {} {} by {} at {} | {}",
                    event.event,
                    event.id,
                    event.date,
                    event.identity,
                    short_commit(&event.commit),
                    event.summary
                ),
            )
        })
        .collect::<Vec<_>>();
    let follow_ups = view
        .follow_ups
        .iter()
        .flat_map(|issue| {
            let category = policy.and_then(|policy| policy.status_category(&issue.status));
            ItemLine::new(&issue.id, &issue.status, &issue.priority, &issue.title)
                .state(category, None)
                .render_wrapped(context, "")
        })
        .collect::<Vec<_>>();
    let optional = |title: &str, lines: Vec<String>| {
        if lines.is_empty() {
            Vec::new()
        } else {
            render_lines_section(title, lines)
        }
    };
    vec![
        render_metadata(None, &metadata),
        optional("Success Criteria", criteria),
        render_lines_section("Items", items),
        optional(
            "Decisions",
            decision_full_rows(context, policy, &view.decisions),
        ),
        render_lines_section("Deviations", deviations),
        optional("Mission Reviews", events),
        render_lines_section("Follow-ups", follow_ups),
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

fn item_line(item: &ItemReview) -> ItemLine<'_> {
    let issue = &item.issue;
    ItemLine::new(&issue.id, &issue.status, &issue.priority, &issue.title)
        .issue_type(&issue.issue_type)
        .state(item.status_category.as_deref(), None)
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

fn mark_badge(mark: CriterionMark) -> String {
    match mark {
        CriterionMark::Proven => badge("proven", Tone::Good),
        CriterionMark::Failing => badge("failing", Tone::Bad),
        CriterionMark::Unproven => badge("unproven", Tone::Warn),
    }
}

fn render_html(
    view: &Debrief,
    policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
) -> String {
    let mission = &view.mission;
    let marks = |mark: CriterionMark| {
        view.criteria
            .proofs
            .iter()
            .filter(|proof| proof.mark == mark)
            .count()
    };
    let done = view
        .items
        .iter()
        .filter(|item| item.status_category.as_deref() == Some("done"))
        .count();
    let one_way = view
        .decisions
        .notes
        .iter()
        .filter(|note| note.decision.one_way)
        .count();
    let mut body = String::new();
    body.push_str(&format!(
        "<header><p class=\"meta\">Debrief · <span class=\"id\">{}</span> · {}</p><h1>{}</h1>",
        escape(&mission.id),
        badge(
            &mission.status,
            status_tone(view.status_category.as_deref(), &mission.status)
        ),
        escape(&mission.title)
    ));
    let mut facts = Vec::new();
    if let Some(strategy) = &view.strategy {
        facts.push(format!("Strategy <code>{}</code>", escape(strategy)));
    }
    if let Some(started) = &view.timeline.started {
        facts.push(format!("started {}", started.time.format("%Y-%m-%d")));
    }
    if let Some(closed) = &view.timeline.closed {
        facts.push(format!("closed {}", closed.time.format("%Y-%m-%d")));
    }
    if let Some(commit) = &view.generated_from {
        facts.push(format!(
            "generated from <code>{}</code>",
            escape(short_commit(commit))
        ));
    }
    body.push_str(&format!("<p class=\"meta\">{}</p>", facts.join(" · ")));
    body.push_str(&format!(
        "<div class=\"outcome\">{}</div>",
        prose(&view.outcome)
    ));
    body.push_str("<div class=\"tiles\">");
    let total = view.criteria.proofs.len();
    body.push_str(&tile(
        marks(CriterionMark::Proven),
        &format!("of {total} criteria proven"),
        &format!(
            "{} failing, {} unproven",
            marks(CriterionMark::Failing),
            marks(CriterionMark::Unproven)
        ),
        if total > 0 && marks(CriterionMark::Proven) == total {
            Tone::Good
        } else if marks(CriterionMark::Failing) > 0 {
            Tone::Bad
        } else {
            Tone::Warn
        },
    ));
    body.push_str(&tile(
        done,
        &format!("of {} items done", view.items.len()),
        &format!("{} open or retired", view.items.len() - done),
        if done == view.items.len() {
            Tone::Good
        } else {
            Tone::Warn
        },
    ));
    body.push_str(&tile(
        view.decisions.notes.len() + view.decisions.records.len(),
        "decisions",
        &format!("{one_way} one-way"),
        if one_way > 0 {
            Tone::Warn
        } else {
            Tone::Neutral
        },
    ));
    body.push_str(&tile(
        view.deviations.len(),
        "deviations",
        "derived from records and Git",
        if view.deviations.is_empty() {
            Tone::Good
        } else {
            Tone::Warn
        },
    ));
    body.push_str(&tile(
        view.follow_ups.len(),
        "follow-ups",
        "open items and deferred follow-ups",
        if view.follow_ups.is_empty() {
            Tone::Good
        } else {
            Tone::Neutral
        },
    ));
    body.push_str("</div><nav class=\"toc\"><a href=\"#criteria\">Criteria</a><a href=\"#items\">Items</a><a href=\"#decisions\">Decisions</a><a href=\"#deviations\">Deviations</a><a href=\"#reviews\">Reviews</a><a href=\"#follow-ups\">Follow-ups</a></nav></header>");

    // Criteria
    let mut criteria = String::from(
        "<table><thead><tr><th>Criterion</th><th>Mark</th><th>Claim</th><th>Proof</th></tr></thead><tbody>",
    );
    for proof in &view.criteria.proofs {
        criteria.push_str(&format!(
            "<tr><td class=\"id\">{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            escape(&proof.criterion.slug),
            mark_badge(proof.mark),
            inline(&proof.criterion.text),
            criterion_proof_cue(proof)
                .map(|cue| escape(&cue))
                .unwrap_or_else(|| "<span class=\"muted\">none</span>".into())
        ));
    }
    criteria.push_str("</tbody></table>");
    for problem in &view.criteria.criteria.parse.problems {
        criteria.push_str(&format!(
            "<p>{} {}</p>",
            badge("problem", Tone::Bad),
            escape(problem)
        ));
    }
    body.push_str(&section("criteria", "Success Criteria", &criteria));

    // Items
    let mut items = String::new();
    for item in &view.items {
        let issue = &item.issue;
        items.push_str(&format!(
            "<details{}><summary><span class=\"id\">{}</span> {} {} <span class=\"muted\">{}</span></summary><div>",
            if item.parent.is_some() { " class=\"child\"" } else { "" },
            escape(&issue.id),
            badge(&issue.status, status_tone(item.status_category.as_deref(), &issue.status)),
            escape(&issue.title),
            escape(&issue.issue_type)
        ));
        match &item.report {
            Some(report) => {
                items.push_str("<dl>");
                for (title, text) in report_parts(report) {
                    items.push_str(&format!(
                        "<dt>{}</dt><dd>{}</dd>",
                        escape(title),
                        prose(text)
                    ));
                }
                items.push_str("</dl>");
            }
            None => items.push_str("<p class=\"muted\">Not closed; no report yet.</p>"),
        }
        items.push_str("</div></details>");
    }
    body.push_str(&section("items", "Items", &items));

    // Decisions
    let mut decisions = String::from(
        "<table><thead><tr><th>Where</th><th>Decision</th><th>Why</th><th>Rejected</th></tr></thead><tbody>",
    );
    let note_row = |note: &atelier_app::decisions::NoteDecision| {
        format!(
            "<tr><td class=\"id\">{} <span class=\"muted\">{}</span></td><td>{}{}</td><td>{}</td><td>{}</td></tr>",
            escape(&note.issue_id),
            escape(&note.decision.date),
            if note.decision.one_way {
                format!("{} ", badge("one-way", Tone::Warn))
            } else {
                String::new()
            },
            escape(&note.decision.choice),
            escape(&note.decision.reason),
            escape(&note.decision.rejected.join("; "))
        )
    };
    for note in view
        .decisions
        .notes
        .iter()
        .filter(|note| note.decision.one_way)
    {
        decisions.push_str(&note_row(note));
    }
    for record in &view.decisions.records {
        let issue = &record.issue;
        let mut cues = Vec::new();
        if let Some(adr) = &issue.adr {
            cues.push(format!("ADR <code>{}</code>", escape(&adr.path)));
        }
        if !record.supersedes.is_empty() {
            cues.push(format!(
                "supersedes {}",
                escape(&record.supersedes.join(", "))
            ));
        }
        if !record.superseded_by.is_empty() {
            cues.push(format!(
                "superseded by {}",
                escape(&record.superseded_by.join(", "))
            ));
        }
        let category = policy.and_then(|policy| policy.status_category(&issue.status));
        decisions.push_str(&format!(
            "<tr><td class=\"id\">{}</td><td>{} {}</td><td colspan=\"2\">{}</td></tr>",
            escape(&issue.id),
            badge(&issue.status, status_tone(category, &issue.status)),
            escape(&issue.title),
            cues.join(" · ")
        ));
    }
    for note in view
        .decisions
        .notes
        .iter()
        .filter(|note| !note.decision.one_way)
    {
        decisions.push_str(&note_row(note));
    }
    decisions.push_str("</tbody></table>");
    if view.decisions.notes.is_empty() && view.decisions.records.is_empty() {
        decisions = "<p class=\"muted\">None recorded.</p>".to_string();
    }
    body.push_str(&section("decisions", "Decisions", &decisions));

    // Deviations, reviews, follow-ups
    let list = |lines: Vec<String>| {
        if lines.is_empty() {
            "<p class=\"muted\">None.</p>".to_string()
        } else {
            format!(
                "<ul class=\"facts\">{}</ul>",
                lines
                    .iter()
                    .map(|line| format!("<li>{}</li>", escape(line)))
                    .collect::<String>()
            )
        }
    };
    body.push_str(&section(
        "deviations",
        "Deviations",
        &list(view.deviations.iter().map(deviation_line).collect()),
    ));
    body.push_str(&section(
        "reviews",
        "Mission Reviews",
        &list(
            view.mission_events
                .iter()
                .map(|event| {
                    format!(
                        "{} {} {} by {} at {}: {}",
                        event.event,
                        event.id,
                        event.date,
                        event.identity,
                        short_commit(&event.commit),
                        event.summary
                    )
                })
                .collect(),
        ),
    ));
    body.push_str(&section(
        "follow-ups",
        "Follow-ups",
        &list(
            view.follow_ups
                .iter()
                .map(|issue| format!("{} {} {}", issue.id, issue.status, issue.title))
                .collect(),
        ),
    ));
    review_page::page(
        &format!("Debrief: {} {}", mission.id, mission.title),
        &body,
        &format!(
            "Generated by atelier debrief {} from {}. A read-only document; record decisions with atelier commands.",
            mission.id,
            view.generated_from
                .as_deref()
                .map(short_commit)
                .unwrap_or("an uncommitted checkout")
        ),
    )
}

/// `atelier brief` (atelier-acrt): a mission's plan before work starts, in
/// the same two forms as `debrief`.
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
        .flat_map(|event| {
            render_row_wrapped(
                context,
                &format!(
                    "{} {} {} by {} at {} | {}",
                    event.event,
                    event.id,
                    event.date,
                    event.identity,
                    short_commit(&event.commit),
                    event.summary
                ),
            )
        })
        .collect::<Vec<_>>();
    if approvals == 0 {
        events.push("No plan approval recorded.".to_string());
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
        &view
            .risks
            .as_deref()
            .map(prose)
            .unwrap_or_else(|| "<p class=\"muted\">The strategy names none.</p>".to_string()),
    ));
    let events = view
        .mission_events
        .iter()
        .map(|event| {
            format!(
                "<li>{} {} by {} at <code>{}</code>: {}</li>",
                escape(event.event),
                escape(&event.date),
                escape(&event.identity),
                escape(short_commit(&event.commit)),
                escape(&event.summary)
            )
        })
        .collect::<String>();
    body.push_str(&section(
        "reviews",
        "Mission Reviews",
        &if events.is_empty() {
            "<p class=\"muted\">No plan approval recorded.</p>".to_string()
        } else {
            format!("<ul class=\"facts\">{events}</ul>")
        },
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
