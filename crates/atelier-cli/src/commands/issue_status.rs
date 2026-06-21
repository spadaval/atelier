use anyhow::Result;
use std::collections::BTreeSet;

use crate::commands;
use atelier_sqlite::Database;

pub fn run(db: &Database, issue_id: &str, quiet: bool) -> Result<()> {
    let issue = db.require_issue(issue_id)?;
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let active_issues = commands::status::current_work_issues(db, workflow_policy.as_ref())?;
    let active_issue_ids = active_issues
        .iter()
        .map(|issue| issue.id.as_str())
        .collect::<BTreeSet<_>>();
    let snapshot =
        commands::objective_status::snapshot_for_issue_objective(db, &issue.id, &active_issue_ids)?;
    let evidence_gaps = evidence_gaps(db, &snapshot.issue_ids)?;
    let open_work = open_work_ids(&snapshot);

    if quiet {
        println!(
            "issue={} health={} ready={} active={} blocked={} done={} backlog={} blockers={} proof_gaps={}",
            issue.id,
            snapshot.health(),
            snapshot.ready,
            snapshot.active,
            snapshot.blocked,
            snapshot.done,
            snapshot.backlog,
            snapshot.open_blockers.len(),
            evidence_gaps.len()
        );
        return Ok(());
    }

    println!("Issue Status {} - {}", issue.id, issue.title);
    println!("{}", "=".repeat(16 + issue.id.len() + issue.title.len()));
    println!("Health:   {}", snapshot.health());
    println!("Type:     {}", issue.issue_type);
    println!("Status:   {}", issue.status);

    println!();
    println!("Work");
    println!("----");
    if snapshot.active > 0 {
        println!(
            "Total: ready {}, active {}, blocked {}, done {}, backlog {}",
            snapshot.ready, snapshot.active, snapshot.blocked, snapshot.done, snapshot.backlog
        );
    } else {
        println!(
            "Total: ready {}, blocked {}, done {}, backlog {}",
            snapshot.ready, snapshot.blocked, snapshot.done, snapshot.backlog
        );
    }

    print_ready_work(db, &snapshot)?;
    print_blocked_work(db, &snapshot)?;
    print_blockers(&snapshot);
    print_evidence(&evidence_gaps);
    print_terminal(&snapshot, &open_work, &evidence_gaps);
    print_next_commands(&issue.id, &snapshot, &open_work, &evidence_gaps);
    Ok(())
}

fn print_ready_work(
    db: &Database,
    snapshot: &commands::objective_status::ObjectiveStatusSnapshot,
) -> Result<()> {
    println!();
    println!("Ready Work");
    println!("----------");
    if snapshot.ready_issues.is_empty() {
        println!("(none)");
        return Ok(());
    }
    for issue in snapshot.ready_issues.iter().take(5) {
        println!(
            "  ready {} - {} | no open blockers; {}; {}",
            issue.id,
            issue.title,
            commands::objective_status::parent_context(issue),
            commands::objective_status::proof_context(db, &issue.id)?
        );
    }
    Ok(())
}

fn print_blocked_work(
    db: &Database,
    snapshot: &commands::objective_status::ObjectiveStatusSnapshot,
) -> Result<()> {
    println!();
    println!("Blocked Work");
    println!("------------");
    if snapshot.blocked_issues.is_empty() {
        println!("(none)");
        return Ok(());
    }
    for issue in snapshot.blocked_issues.iter().take(5) {
        let blockers = commands::objective_status::open_issue_blockers_with_default(db, &issue.id)?;
        println!(
            "  blocked {} - {} | {} blocker{}; details: atelier issue blocked {}",
            issue.id,
            issue.title,
            blockers.len(),
            plural_suffix(blockers.len()),
            issue.id
        );
    }
    Ok(())
}

fn print_blockers(snapshot: &commands::objective_status::ObjectiveStatusSnapshot) {
    println!();
    println!("Blockers");
    println!("--------");
    if snapshot.open_blockers.is_empty() {
        println!("Open Blockers: none");
    } else {
        println!(
            "Open Blockers: {} open - {}",
            snapshot.open_blockers.len(),
            compact_strings(&snapshot.open_blockers)
        );
        println!("  Next: close or unblock listed blockers");
    }
}

fn print_evidence(evidence_gaps: &[String]) {
    println!();
    println!("Evidence");
    println!("--------");
    if evidence_gaps.is_empty() {
        println!("Attached Proof: complete");
    } else {
        println!(
            "Attached Proof: missing - issue proof gaps: {}",
            compact_strings(evidence_gaps)
        );
        println!("  Hint: {}", commands::issue::evidence_help_hint());
        println!("  Next: atelier evidence record --target issue/<id> --kind validation \"...\"");
        println!("  Next: atelier evidence attach <evidence-id> issue <issue-id>");
    }
}

fn print_terminal(
    snapshot: &commands::objective_status::ObjectiveStatusSnapshot,
    open_work: &[String],
    evidence_gaps: &[String],
) {
    println!();
    println!("Terminal Checks");
    println!("---------------");
    if snapshot.issue_ids.is_empty() {
        println!("Work: missing");
        println!("  Next: create or link accountable child work before closing the objective");
    } else if open_work.is_empty() {
        println!("Work: closed");
    } else {
        println!("Work: open - {}", compact_strings(open_work));
        println!("  Next: atelier issue transition <issue-id> close --reason \"...\"");
    }
    if snapshot.open_blockers.is_empty() {
        println!("Blockers: clear");
    } else {
        println!(
            "Blockers: open - {}",
            compact_strings(&snapshot.open_blockers)
        );
        println!("  Next: close or unblock the blocker issues.");
    }
    if evidence_gaps.is_empty() {
        println!("Attached Proof: complete");
    } else {
        println!(
            "Attached Proof: missing - issue proof gaps: {}",
            compact_strings(evidence_gaps)
        );
        println!("  Hint: {}", commands::issue::evidence_help_hint());
        println!("  Next: atelier evidence record --target issue/<id> --kind validation \"...\"");
    }
}

fn print_next_commands(
    issue_id: &str,
    snapshot: &commands::objective_status::ObjectiveStatusSnapshot,
    open_work: &[String],
    evidence_gaps: &[String],
) {
    println!();
    println!("Next Commands");
    println!("-------------");
    println!("  Open objective record: atelier issue show {issue_id}");
    if let Some(issue) = snapshot.active_issues.first() {
        println!(
            "  Inspect current work transitions: atelier issue transition {} --options",
            issue.id
        );
    } else if let Some(issue) = snapshot.selectable_issues.first() {
        println!(
            "  Inspect ready work transitions: atelier issue transition {} --options",
            issue.id
        );
    } else if let Some(issue_id) = open_work.first() {
        println!("  Close or defer open work: atelier issue transition {issue_id} --options");
    } else if let Some(issue_id) = evidence_gaps.first() {
        println!("  Record missing proof: atelier evidence record --target issue/{issue_id} --kind validation \"...\"");
    } else {
        println!(
            "  Inspect objective close readiness: atelier issue transition {issue_id} --options"
        );
    }
}

fn evidence_gaps(db: &Database, issue_ids: &BTreeSet<String>) -> Result<Vec<String>> {
    let mut gaps = Vec::new();
    for issue_id in issue_ids {
        if !commands::objective_status::has_validating_evidence(db, issue_id)? {
            gaps.push(issue_id.clone());
        }
    }
    Ok(gaps)
}

fn open_work_ids(snapshot: &commands::objective_status::ObjectiveStatusSnapshot) -> Vec<String> {
    snapshot
        .active_issues
        .iter()
        .chain(snapshot.ready_issues.iter())
        .chain(snapshot.blocked_issues.iter())
        .map(|issue| issue.id.clone())
        .chain(
            snapshot
                .issue_ids
                .iter()
                .filter(|id| {
                    !snapshot
                        .active_issues
                        .iter()
                        .any(|issue| issue.id.as_str() == id.as_str())
                        && !snapshot
                            .ready_issues
                            .iter()
                            .any(|issue| issue.id.as_str() == id.as_str())
                        && !snapshot
                            .blocked_issues
                            .iter()
                            .any(|issue| issue.id.as_str() == id.as_str())
                })
                .cloned(),
        )
        .collect()
}

fn compact_strings(values: &[String]) -> String {
    let mut rendered = values.iter().take(8).cloned().collect::<Vec<_>>();
    if values.len() > 8 {
        rendered.push(format!("... and {} more", values.len() - 8));
    }
    rendered.join(", ")
}

fn plural_suffix(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}
