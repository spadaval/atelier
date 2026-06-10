use anyhow::{bail, Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use serde_json::json;
use std::path::Path;
use std::str::FromStr;

use crate::activity::{list_all_issue_activities, list_issue_activities, ActivityEventType};

#[derive(Debug, Clone, Serialize)]
struct HistoryItem {
    id: String,
    issue: String,
    event_type: String,
    actor: String,
    created_at: String,
    summary: String,
    body: String,
}

pub fn run(
    state_dir: &Path,
    issue: Option<&str>,
    since: Option<&str>,
    until: Option<&str>,
    event_type: Option<&str>,
    limit: usize,
    json_output: bool,
) -> Result<()> {
    if limit == 0 {
        bail!("--limit must be greater than 0");
    }
    let since = since.map(parse_time_filter).transpose()?;
    let until = until.map(parse_time_filter).transpose()?;
    let event_type = event_type.map(ActivityEventType::from_str).transpose()?;

    let activities = match issue {
        Some(issue_id) => list_issue_activities(state_dir, issue_id)?,
        None => list_all_issue_activities(state_dir)?,
    };
    let items = activities
        .into_iter()
        .filter(|activity| since.is_none_or(|since| activity.created_at >= since))
        .filter(|activity| until.is_none_or(|until| activity.created_at <= until))
        .filter(|activity| event_type.is_none_or(|event_type| activity.event_type == event_type))
        .rev()
        .take(limit)
        .map(|activity| HistoryItem {
            id: activity.id,
            issue: activity.subject_id,
            event_type: activity.event_type.to_string(),
            actor: activity.actor,
            created_at: activity.created_at.to_rfc3339(),
            summary: activity.summary,
            body: activity.body,
        })
        .collect::<Vec<_>>();

    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "ok": true,
                "command": "history",
                "data": {
                    "items": items,
                    "count": items.len(),
                    "filters": {
                        "issue": issue,
                        "since": since.map(|time| time.to_rfc3339()),
                        "until": until.map(|time| time.to_rfc3339()),
                        "type": event_type.map(|kind| kind.to_string()),
                        "limit": limit
                    }
                },
                "warnings": []
            }))?
        );
    } else if items.is_empty() {
        println!("No history found.");
    } else {
        for item in items {
            println!(
                "{} [{}] {} {} - {}",
                item.created_at, item.event_type, item.issue, item.actor, item.summary
            );
            if !item.body.trim().is_empty() {
                println!("  {}", item.body.replace('\n', "\n  "));
            }
        }
    }
    Ok(())
}

fn parse_time_filter(value: &str) -> Result<DateTime<Utc>> {
    if let Ok(time) = DateTime::parse_from_rfc3339(value) {
        return Ok(time.with_timezone(&Utc));
    }
    let date = NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .with_context(|| format!("Invalid date or RFC3339 timestamp '{value}'"))?;
    Ok(date.and_hms_opt(0, 0, 0).unwrap().and_utc())
}
