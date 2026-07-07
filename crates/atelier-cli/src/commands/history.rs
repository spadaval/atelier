use anyhow::{bail, Result};
use chrono::{DateTime, Local, Utc};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use atelier_core::{Issue, RecordLink};
use atelier_records::activity::{list_all_issue_activities, list_issue_activities, IssueActivity};
use atelier_sqlite::{Database, RecordSummary};

pub const DEFAULT_LIMIT: usize = 20;

const SOURCE_BOUNDARY: &str =
    "durable .atelier issue activity, records, evidence, status roles, review artifacts, and record links; local runtime diagnostics excluded";

#[derive(Debug, Clone)]
pub struct HistoryOptions {
    pub issue: Option<String>,
    pub limit: usize,
    pub quiet: bool,
}

#[derive(Debug)]
struct HistoryScope {
    label: String,
    source_boundary: &'static str,
    issue_ids: Option<BTreeSet<String>>,
    record_ids: Option<BTreeSet<(String, String)>>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone)]
struct HistoryRow {
    timestamp: DateTime<Utc>,
    event_kind: String,
    actor: Option<String>,
    target_kind: String,
    target_id: String,
    title: String,
    summary: String,
    sort_key: String,
}

#[derive(Debug)]
struct Lookup {
    issues: BTreeMap<String, Issue>,
    records: BTreeMap<(String, String), RecordSummary>,
}

pub fn run(db: &Database, state_dir: &Path, options: HistoryOptions) -> Result<()> {
    if options.limit == 0 {
        bail!("--limit must be greater than 0");
    }
    let scope = HistoryScope::build(db, &options)?;
    let rows = collect_rows(db, state_dir, &scope)?;
    render_history(&scope, &options, rows)
}

impl HistoryScope {
    fn build(db: &Database, options: &HistoryOptions) -> Result<Self> {
        if let Some(issue_id) = options.issue.as_deref() {
            let issue = db.require_issue(issue_id)?;
            let issue_ids = BTreeSet::from([issue.id.clone()]);
            return Ok(Self {
                label: format!("issue {} - {}", issue.id, issue.title),
                source_boundary: SOURCE_BOUNDARY,
                issue_ids: Some(issue_ids),
                record_ids: Some(records_linked_to_issues(db, &[issue.id.as_str()])?),
                next_commands: vec![
                    format!("atelier issue show {}", issue.id),
                    format!(
                        "atelier history --issue {} --limit {}",
                        issue.id, options.limit
                    ),
                ],
            });
        }

        Ok(Self {
            label: "repository".to_string(),
            source_boundary: SOURCE_BOUNDARY,
            issue_ids: None,
            record_ids: None,
            next_commands: vec![
                "atelier issue show <id>".to_string(),
                "atelier issue show <mission-id>".to_string(),
                format!("atelier history --limit {}", options.limit),
                "atelier history --issue <id>".to_string(),
            ],
        })
    }

    fn includes_issue(&self, issue_id: &str) -> bool {
        self.issue_ids
            .as_ref()
            .is_none_or(|ids| ids.contains(issue_id))
    }

    fn includes_record(&self, kind: &str, id: &str) -> bool {
        self.record_ids
            .as_ref()
            .is_none_or(|ids| ids.contains(&(kind.to_string(), id.to_string())))
    }

    fn includes_link(&self, link: &RecordLink) -> bool {
        if self.record_ids.is_none() && self.issue_ids.is_none() {
            return true;
        }
        if self.issue_ids.is_some() {
            return (link.source_kind == "issue" && self.includes_issue(&link.source_id))
                || (link.target_kind == "issue" && self.includes_issue(&link.target_id));
        }
        (link.source_kind == "issue" && self.includes_issue(&link.source_id))
            || (link.target_kind == "issue" && self.includes_issue(&link.target_id))
            || self.includes_record(&link.source_kind, &link.source_id)
            || self.includes_record(&link.target_kind, &link.target_id)
    }
}

fn collect_rows(db: &Database, state_dir: &Path, scope: &HistoryScope) -> Result<Vec<HistoryRow>> {
    let lookup = Lookup::load(db)?;
    let mut rows = Vec::new();

    if let Some(issue_ids) = scope.issue_ids.as_ref() {
        for issue_id in issue_ids {
            rows.extend(
                list_issue_activities(state_dir, issue_id)?
                    .into_iter()
                    .map(|activity| activity_row(activity, &lookup)),
            );
        }
    } else {
        rows.extend(
            list_all_issue_activities(state_dir)?
                .into_iter()
                .map(|activity| activity_row(activity, &lookup)),
        );
    }

    for issue in lookup
        .issues
        .values()
        .filter(|issue| scope.includes_issue(&issue.id))
    {
        rows.push(HistoryRow {
            timestamp: issue.created_at,
            event_kind: "issue_created".to_string(),
            actor: None,
            target_kind: "issue".to_string(),
            target_id: issue.id.clone(),
            title: issue.title.clone(),
            summary: format!("Created issue {}", issue.title),
            sort_key: format!("issue/{}/created", issue.id),
        });
        if let Some(closed_at) = issue.closed_at {
            rows.push(HistoryRow {
                timestamp: closed_at,
                event_kind: "issue_closed".to_string(),
                actor: None,
                target_kind: "issue".to_string(),
                target_id: issue.id.clone(),
                title: issue.title.clone(),
                summary: format!("Closed issue {}", issue.title),
                sort_key: format!("issue/{}/closed", issue.id),
            });
        }
    }

    for record in lookup
        .records
        .values()
        .filter(|record| scope.includes_record(&record.kind, &record.id))
    {
        rows.push(HistoryRow {
            timestamp: record.created_at,
            event_kind: format!("{}_created", record.kind),
            actor: None,
            target_kind: record.kind.clone(),
            target_id: record.id.clone(),
            title: record.title.clone(),
            summary: format!("Created {} {}", record.kind, record.title),
            sort_key: format!("{}/{}/created", record.kind, record.id),
        });
    }

    for link in db
        .list_all_record_links()?
        .into_iter()
        .filter(|link| scope.includes_link(link))
    {
        rows.push(link_row(&link, &lookup));
    }

    rows.sort_by(|a, b| {
        b.timestamp
            .cmp(&a.timestamp)
            .then(a.sort_key.cmp(&b.sort_key))
    });
    Ok(rows)
}

impl Lookup {
    fn load(db: &Database) -> Result<Self> {
        let issues = db
            .list_issues(Some("all"), None, None)?
            .into_iter()
            .map(|issue| (issue.id.clone(), issue))
            .collect();
        let mut records = BTreeMap::new();
        for kind in ["evidence"] {
            for record in db.list_records(kind, None)? {
                records.insert((record.kind.clone(), record.id.clone()), record);
            }
        }
        Ok(Self { issues, records })
    }

    fn target_title(&self, kind: &str, id: &str) -> String {
        if kind == "issue" {
            return self
                .issues
                .get(id)
                .map(|issue| issue.title.clone())
                .unwrap_or_else(|| "(missing issue)".to_string());
        }
        self.records
            .get(&(kind.to_string(), id.to_string()))
            .map(|record| record.title.clone())
            .unwrap_or_else(|| format!("({kind} record)"))
    }
}

fn activity_row(activity: IssueActivity, lookup: &Lookup) -> HistoryRow {
    let title = lookup.target_title(&activity.subject_kind, &activity.subject_id);
    let sort_key = format!(
        "{}/{}/activity/{}",
        activity.subject_kind, activity.subject_id, activity.id
    );
    let summary = if matches!(
        activity.event_type.as_str(),
        "comment" | "note" | "handoff" | "plan"
    ) {
        let preview = activity.body.lines().next().unwrap_or("").trim();
        if preview.is_empty() {
            activity.summary
        } else {
            format!("{}: {}", activity.summary, preview)
        }
    } else {
        activity.summary
    };
    HistoryRow {
        timestamp: activity.created_at,
        event_kind: activity.event_type.to_string(),
        actor: Some(activity.actor),
        target_kind: activity.subject_kind.clone(),
        target_id: activity.subject_id.clone(),
        title,
        summary,
        sort_key,
    }
}

fn link_row(link: &RecordLink, lookup: &Lookup) -> HistoryRow {
    let (target_kind, target_id) = display_target_for_link(link);
    let event_kind = if link.source_kind == "evidence" || link.target_kind == "evidence" {
        "evidence_attached"
    } else {
        "link_added"
    };
    let summary = if event_kind == "evidence_attached" {
        let evidence_id = if link.source_kind == "evidence" {
            &link.source_id
        } else {
            &link.target_id
        };
        format!(
            "Attached evidence {} to {}/{} ({})",
            evidence_id, target_kind, target_id, link.relation_type
        )
    } else {
        format!(
            "Linked {}/{} {} {}/{}",
            link.source_kind, link.source_id, link.relation_type, link.target_kind, link.target_id
        )
    };
    HistoryRow {
        timestamp: link.created_at,
        event_kind: event_kind.to_string(),
        actor: None,
        target_kind: target_kind.to_string(),
        target_id: target_id.to_string(),
        title: lookup.target_title(target_kind, target_id),
        summary,
        sort_key: format!(
            "link/{}/{}/{}/{}/{}",
            link.source_kind, link.source_id, link.target_kind, link.target_id, link.relation_type
        ),
    }
}

fn display_target_for_link(link: &RecordLink) -> (&str, &str) {
    if link.target_kind != "evidence" {
        (&link.target_kind, &link.target_id)
    } else {
        (&link.source_kind, &link.source_id)
    }
}

fn collect_linked_evidence_records(
    db: &Database,
    issue_ids: &BTreeSet<String>,
    record_ids: &mut BTreeSet<(String, String)>,
) -> Result<()> {
    for link in db.list_all_record_links()? {
        let issue_match = (link.source_kind == "issue" && issue_ids.contains(&link.source_id))
            || (link.target_kind == "issue" && issue_ids.contains(&link.target_id));
        if !issue_match {
            continue;
        }
        if link.source_kind == "evidence" {
            record_ids.insert(("evidence".to_string(), link.source_id));
        } else if link.target_kind == "evidence" {
            record_ids.insert(("evidence".to_string(), link.target_id));
        }
    }
    Ok(())
}

fn records_linked_to_issues(
    db: &Database,
    issue_ids: &[&str],
) -> Result<BTreeSet<(String, String)>> {
    let issue_ids = issue_ids
        .iter()
        .map(|id| (*id).to_string())
        .collect::<BTreeSet<_>>();
    let mut records = BTreeSet::new();
    collect_linked_evidence_records(db, &issue_ids, &mut records)?;
    Ok(records)
}

fn render_history(
    scope: &HistoryScope,
    options: &HistoryOptions,
    rows: Vec<HistoryRow>,
) -> Result<()> {
    if options.quiet {
        println!("events {}", rows.len());
        for row in rows.iter().take(options.limit) {
            println!("{}", row.timestamp.to_rfc3339());
        }
        return Ok(());
    }

    println!("History");
    println!("=======");
    println!("Scope:          {}", scope.label);
    println!("Source:         {}", scope.source_boundary);
    println!("Ordering:       newest first, timestamp then record/path");
    println!("Limit:          {}", options.limit);

    if rows.is_empty() {
        println!("\nNo durable history found for {}.", scope.label);
        println!("This scope has no durable activity, records, evidence, or links yet.");
        print_next_commands(&scope.next_commands);
        return Ok(());
    }

    let visible_count = rows.len().min(options.limit);
    println!("Showing:        {} of {} events", visible_count, rows.len());
    println!("\nEvents");
    println!("------");
    for row in rows.iter().take(options.limit) {
        println!("  {}", event_sentence(row));
        println!(
            "    {} | {} | {} | {}/{}",
            format_timestamp(row.timestamp),
            row.event_kind,
            row.actor.as_deref().unwrap_or("(system)"),
            row.target_kind,
            row.target_id
        );
    }
    if rows.len() > options.limit {
        println!(
            "Omitted:        {} older events hidden by --limit {}",
            rows.len() - options.limit,
            options.limit
        );
    }
    print_next_commands(&scope.next_commands);
    Ok(())
}

fn event_sentence(row: &HistoryRow) -> String {
    let title = compact_text(&row.title);
    let summary = compact_text(&row.summary);
    if summary.is_empty() || summary == title {
        title
    } else {
        format!("{title}: {summary}")
    }
}

fn print_next_commands(commands: &[String]) {
    println!("\nNext Commands");
    println!("-------------");
    for command in commands {
        println!("  {command}");
    }
}

fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    timestamp
        .with_timezone(&Local)
        .format("%Y-%m-%d %H:%M %Z")
        .to_string()
}

fn compact_text(value: &str) -> String {
    let normalized = value.split_whitespace().collect::<Vec<_>>().join(" ");
    const LIMIT: usize = 96;
    if normalized.chars().count() <= LIMIT {
        normalized
    } else {
        let mut output = normalized.chars().take(LIMIT - 3).collect::<String>();
        output.push_str("...");
        output
    }
}
