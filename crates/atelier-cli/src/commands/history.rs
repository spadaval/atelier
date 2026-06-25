use anyhow::{bail, Result};
use chrono::{DateTime, Duration, Local, NaiveDate, Utc};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use atelier_core::{Issue, RecordLink};
use atelier_records::activity::{list_all_issue_activities, list_issue_activities, IssueActivity};
use atelier_sqlite::{Database, RecordSummary};

pub const DEFAULT_LIMIT: usize = 20;

const SOURCE_BOUNDARY: &str =
    "canonical .atelier issue activity, records, evidence, status roles, review artifacts, and record links; local runtime diagnostics excluded";

#[derive(Debug, Clone)]
pub struct HistoryOptions {
    pub mission: Option<String>,
    pub issue: Option<String>,
    pub epic: Option<String>,
    pub include_descendants: bool,
    pub event_kind: Option<String>,
    pub actor: Option<String>,
    pub since: Option<String>,
    pub limit: usize,
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
    let since = parse_since(options.since.as_deref())?;
    let all_rows = collect_rows(db, state_dir, &scope)?;
    let unfiltered_count = all_rows.len();
    let filtered_rows = all_rows
        .into_iter()
        .filter(|row| {
            options
                .event_kind
                .as_ref()
                .is_none_or(|kind| row.event_kind == *kind)
        })
        .filter(|row| {
            options
                .actor
                .as_ref()
                .is_none_or(|actor| row.actor.as_deref() == Some(actor.as_str()))
        })
        .filter(|row| since.is_none_or(|since| row.timestamp >= since))
        .collect::<Vec<_>>();

    render_history(&scope, &options, since, unfiltered_count, filtered_rows)
}

impl HistoryScope {
    fn build(db: &Database, options: &HistoryOptions) -> Result<Self> {
        let selected = [
            options.mission.is_some(),
            options.issue.is_some(),
            options.epic.is_some(),
        ]
        .into_iter()
        .filter(|selected| *selected)
        .count();
        if selected > 1 {
            bail!("Choose only one history scope: --mission, --issue, or --epic");
        }

        if let Some(mission_id) = options.mission.as_deref() {
            let mission = db.require_issue(mission_id)?;
            if mission.issue_type != "mission" {
                bail!("{} is not a mission issue", mission.id);
            }
            let mut issue_ids = BTreeSet::from([mission.id.clone()]);
            issue_ids.extend(crate::commands::objective_status::mission_issue_ids(
                db,
                &mission.id,
            )?);
            let mut record_ids = BTreeSet::new();
            collect_linked_evidence_records(db, &issue_ids, &mut record_ids)?;
            return Ok(Self {
                label: format!("mission {} - {}", mission.id, mission.title),
                source_boundary: SOURCE_BOUNDARY,
                issue_ids: Some(issue_ids),
                record_ids: Some(record_ids),
                next_commands: vec![
                    format!("atelier issue show {}", mission.id),
                    format!(
                        "atelier history --mission {} --limit {}",
                        mission.id, options.limit
                    ),
                    "atelier history --event-kind <kind>".to_string(),
                ],
            });
        }

        if let Some(issue_id) = options.issue.as_deref() {
            let issue = db.require_issue(issue_id)?;
            let mut issue_ids = BTreeSet::new();
            if options.include_descendants {
                collect_issue_and_descendants(db, &issue.id, &mut issue_ids)?;
            } else {
                issue_ids.insert(issue.id.clone());
            }
            collect_linked_evidence_records(db, &issue_ids, &mut BTreeSet::new())?;
            let mut next_commands = vec![
                format!("atelier issue show {}", issue.id),
                format!(
                    "atelier history --issue {} --limit {}",
                    issue.id, options.limit
                ),
            ];
            if !options.include_descendants {
                next_commands.push(format!(
                    "atelier history --issue {} --include-descendants",
                    issue.id
                ));
            }
            next_commands.push("atelier history --event-kind <kind>".to_string());
            return Ok(Self {
                label: format!("issue {} - {}", issue.id, issue.title),
                source_boundary: SOURCE_BOUNDARY,
                issue_ids: Some(issue_ids),
                record_ids: Some(records_linked_to_issues(db, &[issue.id.as_str()])?),
                next_commands,
            });
        }

        if let Some(epic_id) = options.epic.as_deref() {
            let epic = db.require_issue(epic_id)?;
            if epic.issue_type != "epic" {
                bail!("{} is not an epic issue", epic.id);
            }
            let mut issue_ids = BTreeSet::new();
            collect_issue_and_descendants(db, &epic.id, &mut issue_ids)?;
            return Ok(Self {
                label: format!("epic {} - {} (including descendants)", epic.id, epic.title),
                source_boundary: SOURCE_BOUNDARY,
                issue_ids: Some(issue_ids),
                record_ids: Some(records_linked_to_issues(db, &[epic.id.as_str()])?),
                next_commands: vec![
                    format!("atelier issue show {}", epic.id),
                    format!(
                        "atelier history --epic {} --limit {}",
                        epic.id, options.limit
                    ),
                    "atelier history --event-kind <kind>".to_string(),
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
                "atelier history --mission <id>".to_string(),
                "atelier history --issue <id>".to_string(),
                "atelier history --event-kind <kind>".to_string(),
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

fn collect_issue_and_descendants(
    db: &Database,
    issue_id: &str,
    issue_ids: &mut BTreeSet<String>,
) -> Result<()> {
    if !issue_ids.insert(issue_id.to_string()) {
        return Ok(());
    }
    for child in db.get_subissues(issue_id)? {
        collect_issue_and_descendants(db, &child.id, issue_ids)?;
    }
    Ok(())
}

fn render_history(
    scope: &HistoryScope,
    options: &HistoryOptions,
    since: Option<DateTime<Utc>>,
    unfiltered_count: usize,
    rows: Vec<HistoryRow>,
) -> Result<()> {
    println!("History");
    println!("=======");
    println!("Scope:          {}", scope.label);
    println!("Source:         {}", scope.source_boundary);
    println!("Ordering:       newest first, timestamp then record/path");
    println!("Limit:          {}", options.limit);
    println!("Filters:        {}", filter_summary(options, since));

    if unfiltered_count == 0 {
        println!("\nNo canonical history found for {}.", scope.label);
        println!("This scope has no canonical activity, records, evidence, or links yet.");
        print_next_commands(&scope.next_commands);
        return Ok(());
    }

    if rows.is_empty() {
        println!(
            "\nHistory exists for {}, but no events matched the current filters.",
            scope.label
        );
        println!("Widen the filters or inspect the unfiltered scope.");
        print_next_commands(&scope.next_commands);
        return Ok(());
    }

    let visible_count = rows.len().min(options.limit);
    println!(
        "Showing:        {} of {} matching events",
        visible_count,
        rows.len()
    );
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
            "Omitted:        {} older matching events hidden by --limit {}",
            rows.len() - options.limit,
            options.limit
        );
    }
    print_next_commands(&scope.next_commands);
    Ok(())
}

fn filter_summary(options: &HistoryOptions, since: Option<DateTime<Utc>>) -> String {
    let mut filters = Vec::new();
    if let Some(event_kind) = options.event_kind.as_deref() {
        filters.push(format!("event kind {event_kind}"));
    }
    if let Some(actor) = options.actor.as_deref() {
        filters.push(format!("actor {actor}"));
    }
    if let Some(since) = since {
        filters.push(format!("since {}", since.to_rfc3339()));
    }
    if filters.is_empty() {
        "(none)".to_string()
    } else {
        filters.join(", ")
    }
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

fn parse_since(value: Option<&str>) -> Result<Option<DateTime<Utc>>> {
    let Some(value) = value else {
        return Ok(None);
    };
    let value = value.trim();
    if value.is_empty() {
        bail!("--since cannot be empty");
    }
    if let Some(duration) = parse_duration(value) {
        return Ok(Some(Utc::now() - duration));
    }
    if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        return Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(
            date.and_hms_opt(0, 0, 0).expect("midnight is a valid time"),
            Utc,
        )));
    }
    if let Ok(timestamp) = DateTime::parse_from_rfc3339(value) {
        return Ok(Some(timestamp.with_timezone(&Utc)));
    }
    bail!("--since must be a duration like 7d/12h/30m, a YYYY-MM-DD date, or an RFC3339 timestamp")
}

fn parse_duration(value: &str) -> Option<Duration> {
    let (number, unit) = value.split_at(value.len().checked_sub(1)?);
    let amount = number.parse::<i64>().ok()?;
    if amount <= 0 {
        return None;
    }
    match unit {
        "d" => Some(Duration::days(amount)),
        "h" => Some(Duration::hours(amount)),
        "m" => Some(Duration::minutes(amount)),
        _ => None,
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
