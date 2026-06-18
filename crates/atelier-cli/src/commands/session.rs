use anyhow::Result;
use atelier_records::activity::{
    list_derived_issue_attempts, DerivedIssueAttempt, DerivedIssueAttemptActivity,
    DerivedIssueAttemptState,
};
use std::path::Path;

pub fn show(state_dir: &Path, id: &str) -> Result<()> {
    let attempt = load_derived_attempt(state_dir, id)?;
    print_attempt(&attempt);
    Ok(())
}

pub fn list(state_dir: &Path, active: bool) -> Result<()> {
    let mut records = list_derived_issue_attempts(state_dir)?
        .into_iter()
        .filter(|attempt| !active || attempt.state == DerivedIssueAttemptState::Active)
        .collect::<Vec<_>>();
    records.sort_by(|left, right| {
        left.issue_id
            .cmp(&right.issue_id)
            .then(left.role.cmp(&right.role))
            .then(left.serial.cmp(&right.serial))
    });
    println!("Sessions");
    println!("--------");
    if records.is_empty() {
        println!("(none)");
        return Ok(());
    }
    for record in records {
        println!(
            "  {:<32} {:<9} {:<9} serial={} issue/{} recent=\"{}\"",
            record.id,
            record.state.as_str(),
            record.role,
            record.serial,
            record.issue_id,
            recent_activity_summary(&record)
        );
    }
    Ok(())
}

fn load_derived_attempt(state_dir: &Path, id: &str) -> Result<DerivedIssueAttempt> {
    list_derived_issue_attempts(state_dir)?
        .into_iter()
        .find(|attempt| attempt.id == id)
        .ok_or_else(|| anyhow::anyhow!("Session {id} was not found in issue activity"))
}

fn print_attempt(attempt: &DerivedIssueAttempt) {
    println!(
        "{} [session] {} - {} attempt {} for issue {}",
        attempt.id,
        attempt.state.as_str(),
        attempt.role,
        attempt.serial,
        attempt.issue_id
    );
    println!("Issue:       {}", attempt.issue_id);
    println!("Role:        {}", attempt.role);
    println!("Serial:      {}", attempt.serial);
    println!("State:       {}", attempt.state.as_str());
    println!("Actor:       {}", attempt.actor);
    println!(
        "Agent:       {}",
        attempt.agent.as_deref().unwrap_or("(none)")
    );
    println!(
        "Subskill:    {}",
        attempt.subskill.as_deref().unwrap_or("(none)")
    );
    println!("Started:     {}", attempt.started_at.to_rfc3339());
    println!("Updated:     {}", attempt.updated_at.to_rfc3339());
    println!(
        "Ended:       {}",
        attempt
            .ended_at
            .map(|ended| ended.to_rfc3339())
            .unwrap_or_else(|| "(active)".to_string())
    );
    println!("Activity:");
    if attempt.activities.is_empty() {
        println!("  (none)");
    } else {
        for activity in &attempt.activities {
            print_activity(activity);
        }
    }
}

fn recent_activity_summary(attempt: &DerivedIssueAttempt) -> String {
    attempt
        .activities
        .last()
        .map(|activity| {
            format!(
                "{} {} - {}",
                activity.event_type,
                activity.lifecycle.as_str(),
                activity.summary
            )
        })
        .unwrap_or_else(|| "(none)".to_string())
}

fn print_activity(activity: &DerivedIssueAttemptActivity) {
    println!(
        "  {} {} {} - {}",
        activity.created_at.to_rfc3339(),
        activity.event_type,
        activity.lifecycle.as_str(),
        activity.summary
    );
}
