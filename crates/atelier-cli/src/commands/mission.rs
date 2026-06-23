use anyhow::{bail, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use atelier_app::use_cases as app_use_cases;
use atelier_core::Issue;
use atelier_records as record_store;
use atelier_sqlite::{Database, RecordSummary};

const MISSION_TERMINAL_TRANSITION: &str = "close";

pub fn status(
    db: &Database,
    state_dir: &Path,
    id: Option<&str>,
    quiet: bool,
    verbose: bool,
) -> Result<()> {
    match id {
        Some(id) => status_one(db, state_dir, id, quiet, verbose),
        None => status_dashboard(db, state_dir, quiet),
    }
}

pub fn transition_options(db: &Database, id: &str) -> Result<()> {
    let issue = db.require_issue(id)?;
    if issue.issue_type != "mission" {
        bail!("{id} is not a mission objective issue");
    }
    crate::commands::issue::transition_options(db, id)
}

fn status_dashboard(db: &Database, state_dir: &Path, quiet: bool) -> Result<()> {
    let records = current_mission_records(db)?;
    let mut rows = records
        .into_iter()
        .map(|record| {
            Ok(MissionListRow {
                summary: mission_list_summary(db, &record.id)?,
                record,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    rows.sort_by(compare_mission_list_rows);
    let tracker = tracker_health(db, state_dir);

    if quiet {
        println!(
            "missions={} blocked={} terminal_ready={} tracker={}",
            rows.len(),
            rows.iter()
                .filter(|row| row.summary.open_blockers > 0 || row.summary.total_work().blocked > 0)
                .count(),
            rows.iter()
                .filter(|row| row.summary.terminal_ready())
                .count(),
            tracker.status_token()
        );
        for row in &rows {
            println!(
                "{} ready={} blocked={} done={} backlog={}",
                row.record.id,
                row.summary.total_work().ready,
                row.summary.total_work().blocked,
                row.summary.total_work().done,
                row.summary.total_work().backlog
            );
        }
        return Ok(());
    }

    println!("Mission Status");
    println!("==============");
    println!(
        "{} | tracker {}",
        mission_list_summary_line(&rows),
        tracker.status_text()
    );
    if rows.is_empty() {
        println!("(none)");
    } else {
        for row in &rows {
            let work = row.summary.total_work();
            let health = mission_health(&row.summary);
            println!(
                "  {} [{}] {} - {} | {} | terminal {}",
                row.record.id,
                health,
                mission_lifecycle_status(&row.record),
                row.record.title,
                work.to_compact_text(),
                if row.summary.terminal_ready() {
                    "ready"
                } else {
                    "not ready"
                }
            );
        }
    }
    print_mission_heading("Next Commands");
    if let Some(row) = rows.first() {
        println!("  atelier mission status {}", row.record.id);
    }
    println!("  atelier issue list --status all");
    println!("  atelier issue list --ready");
    Ok(())
}

fn status_one(db: &Database, state_dir: &Path, id: &str, quiet: bool, verbose: bool) -> Result<()> {
    let mission = mission_objective_summary(db, id)?;
    let mission_title = mission.title.as_str();
    let summary = mission_list_summary(db, &mission.id)?;
    let tracker = tracker_health(db, state_dir);
    let active_work = active_work_for_mission(db, &mission.id)?;
    let terminal = mission_terminal_status(db, state_dir, &mission, &summary)?;
    let validator_failures = terminal.validator_failure_count();

    if quiet {
        let work = summary.total_work();
        println!(
            "{} health={} ready={} blocked={} done={} backlog={} blockers={} validator_failures={} tracker={} terminal_ready={}",
            mission.id,
            mission_health_for(&mission, &summary),
            work.ready,
            work.blocked,
            work.done,
            work.backlog,
            summary.open_blockers,
            validator_failures,
            tracker.status_token(),
            if mission_lifecycle_status(&mission) == "closed" {
                "complete"
            } else if terminal.ready() {
                "yes"
            } else {
                "no"
            }
        );
        return Ok(());
    }

    let identity = format!(
        "Mission Status {} [{}] - {}",
        mission.id,
        mission_lifecycle_status(&mission),
        mission_title
    );
    println!("{identity}");
    println!("{}", "=".repeat(identity.len()));
    println!("Health:   {}", mission_health_for(&mission, &summary));
    println!("Tracker:  {}", tracker.status_text());
    println!(
        "Terminal: {}",
        if mission_lifecycle_status(&mission) == "closed" {
            "complete"
        } else if terminal.ready() {
            "ready"
        } else {
            "blocked"
        }
    );

    print_mission_heading("Work");
    println!("Total: {}", summary.total_work().to_compact_text());
    if summary.epics.is_empty() {
        println!("Epics: none");
    } else {
        for epic in &summary.epics {
            println!(
                "  [epic] {} [{}] {} - {} | {}",
                epic.issue.id,
                epic.issue.status,
                epic.issue.priority,
                epic.issue.title,
                epic.work.to_compact_text()
            );
        }
    }
    if !summary.other_work.is_empty() {
        println!("Other: {}", summary.other_work.to_compact_text());
    }

    print_mission_heading("Selectable Work");
    if summary.selectable_work.is_empty() {
        println!("(none)");
    } else {
        for issue in summary.selectable_work.iter().take(5) {
            let state = mission_issue_state(db, issue)?;
            println!(
                "  {state} {} - {} | no open blockers; {}; {}",
                issue.id,
                issue.title,
                crate::commands::objective_status::parent_context(issue),
                crate::commands::objective_status::proof_context(db, &issue.id)?
            );
        }
        if summary.selectable_work.len() > 5 {
            println!(
                "  {} more ready work item(s) omitted",
                summary.selectable_work.len() - 5
            );
        }
    }

    print_mission_heading("Blocked Work");
    if summary.blocked_work.is_empty() {
        println!("(none)");
    } else {
        for blocked in summary.blocked_work.iter().take(5) {
            println!(
                "  blocked {} - {} | {} blocker{}; {}; {}",
                blocked.issue.id,
                blocked.issue.title,
                blocked.blockers.len(),
                plural_suffix(blocked.blockers.len()),
                crate::commands::objective_status::parent_context(&blocked.issue),
                crate::commands::objective_status::proof_context(db, &blocked.issue.id)?
            );
        }
        if summary.blocked_work.len() > 5 {
            println!(
                "  {} more blocked work item(s) omitted",
                summary.blocked_work.len() - 5
            );
        }
        if let Some(blocked) = summary.blocked_work.first() {
            println!(
                "  Inspect blockers: atelier issue blocked {}",
                blocked.issue.id
            );
        }
    }

    print_mission_heading("Blockers");
    if summary.open_blockers == 0 && summary.total_work().blocked == 0 {
        println!("(none)");
    } else {
        println!(
            "Mission blockers: {}",
            count_label(summary.open_blockers, "open")
        );
        println!(
            "Blocked work: {}",
            count_label(summary.total_work().blocked, "blocked")
        );
    }

    print_mission_heading("Evidence");
    if summary.evidence_count == 0 {
        println!("Direct mission evidence: none");
    } else {
        println!("Direct mission evidence: {}", summary.evidence_count);
    }

    print_mission_heading("Reliability");
    print_reliability_summary(db, state_dir, &mission, &summary, &tracker, &terminal)?;

    print_mission_heading("Terminal Checks");
    if mission_lifecycle_status(&mission) == "closed" {
        println!("Mission is closed.");
    } else {
        terminal.print_human();
    }

    let show_advanced_validator_detail = verbose
        || terminal
            .validator_results
            .iter()
            .any(|result| !result.passed && result.validator == "ignored_tests_reviewed");
    if show_advanced_validator_detail {
        print_mission_heading("Advanced Validator Detail");
        if validator_failures == 0 {
            println!("All advanced terminal validators passed.");
        } else {
            println!(
                "{} advanced terminal validator failure detected.",
                validator_failures
            );
            for result in terminal
                .validator_results
                .iter()
                .filter(|result| !result.passed)
            {
                println!("  fail  {} - {}", result.validator, result.reason);
            }
        }
    }

    print_mission_heading("Active Work");
    if active_work.is_empty() {
        println!("(none)");
    } else {
        for work in active_work {
            println!("  {} [{}] - {}", work.id, work.status, work.title);
        }
    }

    print_status_next_commands(&mission, &summary, &terminal);
    Ok(())
}

fn print_status_next_commands(
    mission: &RecordSummary,
    summary: &MissionListSummary,
    terminal: &MissionTerminalStatus,
) {
    print_mission_heading("Next Commands");
    let lifecycle = mission_lifecycle_status(mission);
    println!(
        "  Inspect mission record (durable intent and linked work): atelier issue show {}",
        mission.id
    );
    match lifecycle.as_str() {
        "closed" => {
            println!(
                "  Inspect mission history: atelier history --mission {}",
                mission.id
            );
            return;
        }
        "draft" => {
            println!(
                "  Shape mission work or move to ready when gates permit: atelier issue show {}",
                mission.id
            );
        }
        _ => {
            println!(
            "  Refresh mission status (current blockers and terminal checks): atelier mission status {}",
                mission.id
            );
        }
    }
    if terminal.ready() {
        println!(
            "  Close mission (all terminal checks pass): atelier issue transition {} close --reason \"...\"",
            mission.id
        );
    } else {
        println!(
            "  Inspect terminal check detail: atelier mission status {} --verbose",
            mission.id
        );
        if summary.total_work().blocked > 0 || summary.open_blockers > 0 {
            println!("  Resolve open blockers before assigning more implementation work");
        } else if let Some(issue) = summary.selectable_work.first() {
            println!(
                "  Inspect selectable mission work transitions ({} selectable issue(s)): atelier issue transition {} --options",
                summary.selectable_work.len(),
                issue.id
            );
        }
    }
}

fn current_mission_records(db: &Database) -> Result<Vec<RecordSummary>> {
    mission_records_for_filter(db, Some("current"))
}

fn mission_records_for_filter(db: &Database, status: Option<&str>) -> Result<Vec<RecordSummary>> {
    let records = db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| issue.issue_type == "mission")
        .map(issue_mission_summary)
        .collect::<Vec<_>>();
    Ok(match status {
        None | Some("all") => records,
        Some("current") => records
            .into_iter()
            .filter(|record| is_current_mission_status(&mission_lifecycle_status(record)))
            .collect(),
        Some(status) => {
            let status = normalize_mission_status(status)?;
            records
                .into_iter()
                .filter(|record| mission_lifecycle_status(record) == status)
                .collect()
        }
    })
}

fn mission_objective_summary(db: &Database, id: &str) -> Result<RecordSummary> {
    if let Some(issue) = db.get_issue(id)? {
        if issue.issue_type == "mission" {
            return Ok(issue_mission_summary(issue));
        }
    }
    bail!("{id} is not a mission objective issue")
}

fn issue_mission_summary(issue: Issue) -> RecordSummary {
    let id = issue.id.clone();
    RecordSummary {
        kind: "issue".to_string(),
        id: id.clone(),
        title: issue.title,
        status: issue.status,
        created_at: issue.created_at,
        updated_at: issue.updated_at,
        source_path: format!("issues/{id}.md"),
    }
}

fn normalize_mission_status(status: &str) -> Result<&str> {
    match status {
        "draft" | "ready" | "active" | "superseded" | "closed" => Ok(status),
        _ => bail!(
            "Invalid mission status '{}'. Must be one of: draft, ready, active, superseded, closed",
            status
        ),
    }
}

fn mission_lifecycle_status(record: &RecordSummary) -> String {
    record.status.clone()
}

fn is_current_mission_status(status: &str) -> bool {
    !matches!(status, "closed" | "superseded")
}

pub fn issue_advances_mission(db: &Database, mission_id: &str, issue_id: &str) -> Result<bool> {
    Ok(crate::commands::objective_status::mission_issue_ids(db, mission_id)?.contains(issue_id))
}

struct MissionTerminalStatus {
    mission_id: String,
    has_work: bool,
    open_work: Vec<String>,
    open_blockers: Vec<String>,
    validator_results: Vec<crate::commands::workflow::ValidatorResult>,
}

impl MissionTerminalStatus {
    fn ready(&self) -> bool {
        self.validator_results.iter().all(|result| result.passed)
    }

    fn validator_failure_count(&self) -> usize {
        self.validator_results
            .iter()
            .filter(|result| !result.passed)
            .count()
    }

    fn print_human(&self) {
        if self.ready() {
            println!("All required terminal checks pass.");
            for result in &self.validator_results {
                if let Some(line) = terminal_validator_status_line(result, &self.mission_id) {
                    if line.summary.contains(" - ") {
                        println!("{}", line.summary);
                    }
                }
            }
            return;
        }
        if !self.has_work {
            println!("Work: missing");
            println!("  Next: atelier issue link <mission-id> <issue-id> --role advances");
        } else if self.open_work.is_empty() {
            println!("Work: closed");
        } else {
            println!("Work: open - {}", compact_strings(&self.open_work));
            println!("  Next: atelier issue transition <issue-id> close --reason \"...\"");
        }
        if self.open_blockers.is_empty() {
            println!("Blockers: clear");
        } else {
            println!("Blockers: open - {}", compact_strings(&self.open_blockers));
            println!("  Next: close or unblock the blocker issues.");
        }
        for result in &self.validator_results {
            if let Some(line) = terminal_validator_status_line(result, &self.mission_id) {
                println!("{}", line.summary);
                if let Some(next) = line.next {
                    println!("  Next: {next}");
                }
            }
        }
    }
}

#[derive(Default)]
struct IssueSectionGapSummary {
    malformed: Vec<String>,
    missing_outcome: Vec<String>,
}

fn print_reliability_summary(
    db: &Database,
    state_dir: &Path,
    mission: &RecordSummary,
    summary: &MissionListSummary,
    tracker: &TrackerHealth,
    terminal: &MissionTerminalStatus,
) -> Result<()> {
    let section_gaps = mission_issue_section_gaps(db, state_dir, &mission.id)?;

    if tracker.stale_entries.is_empty() {
        println!("Projection Freshness: current");
    } else {
        println!(
            "Projection Freshness: stale - {}",
            compact_strings(&tracker.stale_entries)
        );
        println!("  Next: atelier doctor --fix");
    }

    if let Some(result) = terminal_validator_result(terminal, "issue.sections_parseable") {
        if result.passed && section_gaps.malformed.is_empty() {
            println!("Malformed Work: none");
        } else {
            let reason = if section_gaps.malformed.is_empty() {
                result.reason.clone()
            } else {
                compact_strings(&section_gaps.malformed)
            };
            println!("Malformed Work: found - {reason}");
            println!("  Next: atelier lint");
        }
    }

    print_section_gap_signal("Missing Outcome Sections", &section_gaps.missing_outcome);
    print_graph_hygiene_signal(summary);

    print_reliability_validator_signal(
        terminal,
        "command_surface_current",
        "Docs/Help Drift",
        "clear",
        "detected",
        "update docs, help text, or command-surface tests",
    );
    print_reliability_validator_signal(
        terminal,
        "ignored_tests_reviewed",
        "Ignored Test Review",
        "current",
        "needed",
        "assign owners or remove stale ignored tests",
    );

    if terminal.open_blockers.is_empty() {
        println!("Open Blockers: none");
    } else {
        println!(
            "Open Blockers: {} open - {}",
            terminal.open_blockers.len(),
            compact_strings(&terminal.open_blockers)
        );
        println!("  Next: close or unblock listed blockers");
    }

    println!("Drill-downs:");
    println!("  atelier mission status {} --verbose", mission.id);
    println!("  atelier lint");
    Ok(())
}

fn print_graph_hygiene_signal(summary: &MissionListSummary) {
    if summary.duplicate_reachability.is_empty() {
        println!("Graph Hygiene: clear");
        return;
    }

    let details = summary
        .duplicate_reachability
        .iter()
        .map(format_duplicate_reachability)
        .collect::<Vec<_>>();
    println!(
        "Graph Hygiene: warning - duplicate reachability for {} issue(s): {}",
        summary.duplicate_reachability.len(),
        compact_strings(&details)
    );
    println!(
        "  Totals count each unique issue once. Keep mission links on root issues or epics and let child issues flow through hierarchy."
    );
}

fn print_section_gap_signal(label: &str, ids: &[String]) {
    if ids.is_empty() {
        println!("{label}: none");
    } else {
        println!("{label}: {} issue(s) - {}", ids.len(), compact_strings(ids));
        println!("  Next: atelier lint");
    }
}

fn print_reliability_validator_signal(
    terminal: &MissionTerminalStatus,
    validator: &str,
    label: &str,
    pass_text: &str,
    fail_text: &str,
    next: &str,
) {
    let Some(result) = terminal_validator_result(terminal, validator) else {
        return;
    };
    if result.passed {
        println!("{label}: {pass_text}");
    } else {
        println!("{label}: {fail_text} - {}", result.reason);
        println!("  Next: {next}");
    }
}

fn terminal_validator_result<'a>(
    terminal: &'a MissionTerminalStatus,
    validator: &str,
) -> Option<&'a crate::commands::workflow::ValidatorResult> {
    terminal
        .validator_results
        .iter()
        .find(|result| result.validator == validator)
}

fn mission_issue_section_gaps(
    db: &Database,
    state_dir: &Path,
    mission_id: &str,
) -> Result<IssueSectionGapSummary> {
    let mut gaps = IssueSectionGapSummary::default();
    for issue_id in crate::commands::objective_status::mission_issue_ids(db, mission_id)? {
        match app_use_cases::load_canonical_issue(state_dir, &issue_id) {
            Ok(record) => {
                for state in record.sections.section_states() {
                    if !state.required || (state.present && !state.empty) {
                        continue;
                    }
                    if state.name == record_store::IssueSectionName::Outcome {
                        gaps.missing_outcome.push(issue_id.clone());
                    }
                }
            }
            Err(error) => {
                let diagnostic = error.to_string();
                if diagnostic.contains("section 'Outcome'")
                    || diagnostic.contains("section Outcome")
                    || diagnostic.contains("section `Outcome`")
                {
                    gaps.missing_outcome.push(issue_id.clone());
                }
                gaps.malformed.push(format!("{issue_id}: {diagnostic}"));
            }
        }
    }
    gaps.malformed.sort();
    gaps.missing_outcome.sort();
    Ok(gaps)
}

struct TerminalCheckStatusLine {
    summary: String,
    next: Option<String>,
}

fn terminal_validator_status_line(
    result: &crate::commands::workflow::ValidatorResult,
    mission_id: &str,
) -> Option<TerminalCheckStatusLine> {
    let (label, pass_text, fail_text, next) = terminal_validator_user_text(&result.validator)?;
    if result.passed {
        let summary = if result.validator == "git.worktree_clean"
            && result.reason != "git checkout is clean"
        {
            format!("{label}: {pass_text} - {}", result.reason)
        } else {
            format!("{label}: {pass_text}")
        };
        Some(TerminalCheckStatusLine {
            summary,
            next: None,
        })
    } else {
        let next = next.replace("{mission}", mission_id);
        Some(TerminalCheckStatusLine {
            summary: format!("{label}: {fail_text} - {}", result.reason),
            next: Some(next),
        })
    }
}

fn terminal_validator_user_text(
    validator: &str,
) -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    match validator {
        "tracker.current" => Some(("Tracker State", "current", "stale", "atelier doctor --fix")),
        "issue.sections_parseable" => Some((
            "Linked Issue Records",
            "parseable",
            "malformed",
            "atelier lint",
        )),
        "lint.none_blocking" => Some(("Blocking Lints", "clear", "failing", "atelier lint")),
        "command_surface_current" => Some((
            "Docs/Help Drift",
            "clear",
            "detected",
            "update docs, help text, or command-surface tests",
        )),
        "ignored_tests_reviewed" => Some((
            "Ignored Test Review",
            "current",
            "needed",
            "assign owners or remove stale ignored tests",
        )),
        "validation.criteria_satisfied" => Some((
            "Validation Criteria",
            "satisfied",
            "incomplete",
            "atelier mission status {mission}",
        )),
        "objective.work_present" => Some((
            "Linked Work",
            "present",
            "missing",
            "atelier issue link {mission} <issue-id> --role advances",
        )),
        "objective.work_terminal" => Some((
            "Linked Work Terminal",
            "closed",
            "open",
            "atelier mission status {mission}",
        )),
        "objective.blockers_none_open" => Some((
            "Direct Objective Blockers",
            "clear",
            "open",
            "atelier issue blocked {mission}",
        )),
        "git.worktree_clean" => Some((
            "Checkout",
            "clean",
            "dirty",
            "commit or remove untracked checkout changes",
        )),
        "no_open_work" | "blockers.none_open" | "evidence.attached" => None,
        _ => Some((
            "Additional Terminal Check",
            "passed",
            "failed",
            "atelier mission status {mission}",
        )),
    }
}

pub(crate) fn mission_validation_criteria_gate(
    db: &Database,
    mission_id: &str,
) -> Result<(bool, String)> {
    let approval = mission_workflow_approval(db, mission_id)?;
    if approval.is_empty() {
        return Ok((
            true,
            "no explicit linked terminal validation work requires workflow approval".to_string(),
        ));
    }
    if approval.open.is_empty() && approval.blocked.is_empty() {
        return Ok((
            true,
            format!(
                "workflow approval complete via linked terminal validation work: {}",
                compact_strings(&approval.issue_ids())
            ),
        ));
    }
    let mut pending = approval
        .open
        .iter()
        .map(|issue| issue.id.clone())
        .collect::<Vec<_>>();
    pending.extend(approval.blocked.iter().map(|issue| issue.id.clone()));
    pending.sort();
    Ok((
        false,
        format!(
            "workflow approval is still pending on linked terminal validation work: {}",
            compact_strings(&pending)
        ),
    ))
}

fn validating_evidence_records(
    db: &Database,
    target_kind: &str,
    target_id: &str,
) -> Result<Vec<RecordSummary>> {
    let mut records = db
        .list_record_links(target_kind, target_id)?
        .into_iter()
        .filter(|link| link.relation_type == "validates")
        .filter_map(|link| {
            if link.source_kind == "evidence" {
                Some(link.source_id)
            } else if link.target_kind == "evidence" {
                Some(link.target_id)
            } else {
                None
            }
        })
        .map(|id| db.require_record("evidence", &id))
        .collect::<Result<Vec<_>>>()?;
    records.sort_by(|a, b| a.id.cmp(&b.id));
    records.dedup_by(|a, b| a.id == b.id);
    Ok(records)
}

fn validating_evidence_ids(
    db: &Database,
    target_kind: &str,
    target_id: &str,
) -> Result<Vec<String>> {
    Ok(validating_evidence_records(db, target_kind, target_id)?
        .into_iter()
        .map(|record| record.id)
        .collect())
}

#[derive(Default)]
struct MissionWorkflowApproval {
    done: Vec<Issue>,
    open: Vec<Issue>,
    blocked: Vec<Issue>,
}

impl MissionWorkflowApproval {
    fn is_empty(&self) -> bool {
        self.done.is_empty() && self.open.is_empty() && self.blocked.is_empty()
    }

    fn issue_ids(&self) -> Vec<String> {
        let mut ids = self
            .done
            .iter()
            .chain(self.open.iter())
            .chain(self.blocked.iter())
            .map(|issue| issue.id.clone())
            .collect::<Vec<_>>();
        ids.sort();
        ids
    }
}

fn mission_workflow_approval(db: &Database, mission_id: &str) -> Result<MissionWorkflowApproval> {
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let mut approval = MissionWorkflowApproval::default();
    for issue_id in crate::commands::objective_status::mission_issue_ids(db, mission_id)? {
        let issue = db.require_issue(&issue_id)?;
        if issue.issue_type != "validation" {
            continue;
        }
        match crate::commands::issue_workflow::issue_status_category(
            workflow_policy.as_ref(),
            &issue.status,
        )
        .as_deref()
        {
            Some("done") => approval.done.push(issue),
            Some("blocked") => approval.blocked.push(issue),
            _ => approval.open.push(issue),
        }
    }
    approval.done.sort_by(|a, b| a.id.cmp(&b.id));
    approval.open.sort_by(|a, b| a.id.cmp(&b.id));
    approval.blocked.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(approval)
}

fn compact_strings(values: &[String]) -> String {
    const LIMIT: usize = 8;
    if values.len() <= LIMIT {
        values.join(", ")
    } else {
        format!(
            "{}, ... and {} more",
            values[..LIMIT].join(", "),
            values.len() - LIMIT
        )
    }
}

fn plural_suffix(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

fn mission_terminal_status(
    db: &Database,
    _state_dir: &Path,
    mission: &RecordSummary,
    _summary: &MissionListSummary,
) -> Result<MissionTerminalStatus> {
    let has_work =
        !crate::commands::objective_status::mission_issue_ids(db, &mission.id)?.is_empty();
    let open_work = crate::commands::objective_status::open_objective_work(db, &mission.id)?;
    let objective_kind =
        crate::commands::objective_status::mission_objective_kind(db, &mission.id)?;
    let open_blockers = crate::commands::objective_status::open_objective_blockers(
        db,
        objective_kind,
        &mission.id,
    )?;
    let validator_results = match crate::commands::workflow::evaluate(
        db,
        objective_kind,
        &mission.id,
        MISSION_TERMINAL_TRANSITION,
        Vec::new(),
    ) {
        Ok(results) => results,
        Err(error) => vec![crate::commands::workflow::ValidatorResult {
            target_kind: objective_kind.to_string(),
            target_id: mission.id.clone(),
            transition: MISSION_TERMINAL_TRANSITION.to_string(),
            validator: "workflow_policy".to_string(),
            passed: false,
            reason: format!("{error:#}; run `atelier lint` for workflow/config diagnostics"),
            help: None,
            elapsed_ms: 0,
        }],
    };
    Ok(MissionTerminalStatus {
        mission_id: mission.id.clone(),
        has_work,
        open_work,
        open_blockers,
        validator_results,
    })
}

struct MissionListRow {
    record: RecordSummary,
    summary: MissionListSummary,
}

#[derive(Default)]
struct MissionListSummary {
    work: WorkCounts,
    other_work: WorkCounts,
    epics: Vec<MissionListEpic>,
    selectable_work: Vec<Issue>,
    blocked_work: Vec<BlockedMissionWork>,
    open_blockers: usize,
    evidence_count: usize,
    approval_pending_count: usize,
    duplicate_reachability: Vec<DuplicateReachability>,
}

struct MissionListEpic {
    issue: Issue,
    work: WorkCounts,
}

struct BlockedMissionWork {
    issue: Issue,
    blockers: Vec<String>,
}

struct DuplicateReachability {
    issue_id: String,
    roots: Vec<String>,
}

#[derive(Clone, Copy, Default)]
struct WorkCounts {
    ready: usize,
    blocked: usize,
    done: usize,
    backlog: usize,
}

fn mission_list_summary(db: &Database, mission_id: &str) -> Result<MissionListSummary> {
    let mut summary = MissionListSummary::default();
    let mut seen_blockers = BTreeSet::new();
    let mut seen_work = BTreeSet::new();
    let mut linked_work = Vec::new();
    let objective_kind = crate::commands::objective_status::mission_objective_kind(db, mission_id)?;
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;

    for blocker_id in
        crate::commands::objective_status::direct_blocker_ids(db, objective_kind, mission_id)?
    {
        if seen_blockers.insert(blocker_id.clone()) {
            let issue = db.require_issue(&blocker_id)?;
            if crate::commands::issue_workflow::issue_blocks_work(workflow_policy.as_ref(), &issue)
            {
                summary.open_blockers += 1;
            }
        }
    }

    for linked_id in mission_linked_work_roots(db, objective_kind, mission_id)? {
        if seen_work.insert(linked_id.clone()) {
            let issue = db.require_issue(&linked_id)?;
            linked_work.push(issue);
        }
    }
    summary.evidence_count = validating_evidence_ids(db, objective_kind, mission_id)?.len();

    let linked_epic_ids = linked_work
        .iter()
        .filter(|issue| issue.issue_type == "epic")
        .map(|issue| issue.id.clone())
        .collect::<BTreeSet<_>>();
    let mission_issue_ids = crate::commands::objective_status::mission_issue_ids(db, mission_id)?;

    for issue_id in &mission_issue_ids {
        let issue = db.require_issue(issue_id)?;
        summary.work.add_bucket(issue_bucket(db, &issue)?);
        if issue.issue_type == "validation"
            && !crate::commands::issue_workflow::issue_is_done(workflow_policy.as_ref(), &issue)
        {
            summary.approval_pending_count += 1;
        }
    }
    summary.duplicate_reachability = duplicate_reachability(db, &mission_issue_ids, &seen_work)?;

    for issue in linked_work {
        if issue.issue_type == "epic" {
            summary.epics.push(MissionListEpic {
                work: epic_work_counts(db, &issue.id)?,
                issue,
            });
        } else if !has_ancestor_in_set(db, &issue, &linked_epic_ids)? {
            summary.other_work.add_bucket(issue_bucket(db, &issue)?);
        }
    }

    for issue_id in mission_issue_ids {
        let issue = db.require_issue(&issue_id)?;
        if !crate::commands::objective_status::is_selectable_work(db, &issue)? {
            continue;
        }
        let blockers =
            crate::commands::objective_status::open_issue_blockers_with_default(db, &issue.id)?;
        if !blockers.is_empty() {
            summary
                .blocked_work
                .push(BlockedMissionWork { issue, blockers });
            continue;
        }
        if mission_issue_state(db, &issue)? == "ready" {
            summary.selectable_work.push(issue);
        }
    }

    summary.epics = order_mission_epics(db, summary.epics)?;
    summary.selectable_work = crate::commands::objective_status::order_issues_by_work_with_default(
        db,
        summary.selectable_work,
    )?;
    summary.blocked_work = order_blocked_work(db, summary.blocked_work)?;
    Ok(summary)
}

fn mission_linked_work_roots(
    db: &Database,
    objective_kind: &str,
    mission_id: &str,
) -> Result<Vec<String>> {
    if objective_kind != "issue" {
        bail!("{mission_id} is not a mission objective issue");
    }
    let mut roots = BTreeSet::new();
    for relation in db.get_typed_relations(mission_id)? {
        if relation.relation_type != "advances" {
            continue;
        }
        let linked_id = if relation.issue_id_1 == mission_id {
            relation.issue_id_2
        } else {
            relation.issue_id_1
        };
        roots.insert(linked_id);
    }
    Ok(roots.into_iter().collect())
}

impl MissionListSummary {
    fn total_work(&self) -> WorkCounts {
        self.work
    }

    fn terminal_ready(&self) -> bool {
        let work = self.total_work();
        work.done > 0
            && work.ready == 0
            && work.blocked == 0
            && work.backlog == 0
            && self.open_blockers == 0
            && self.approval_pending_count == 0
    }
}

fn mission_list_summary_line(rows: &[MissionListRow]) -> String {
    let mut statuses = BTreeMap::<String, usize>::new();
    let mut blocked_missions = 0;
    for row in rows {
        *statuses
            .entry(mission_lifecycle_status(&row.record))
            .or_default() += 1;
        if row.summary.open_blockers > 0 || row.summary.total_work().blocked > 0 {
            blocked_missions += 1;
        }
    }
    let status_text = mission_status_summary_text(rows.len(), statuses);
    let blocked_text = count_label(blocked_missions, "blocked");
    format!("{status_text} | {blocked_text}")
}

fn compare_mission_list_rows(a: &MissionListRow, b: &MissionListRow) -> std::cmp::Ordering {
    mission_status_rank(&mission_lifecycle_status(&a.record))
        .cmp(&mission_status_rank(&mission_lifecycle_status(&b.record)))
        .then_with(|| {
            if mission_lifecycle_status(&a.record) != "ready"
                && mission_lifecycle_status(&b.record) != "ready"
            {
                mission_lifecycle_status(&a.record).cmp(&mission_lifecycle_status(&b.record))
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .then_with(|| b.record.updated_at.cmp(&a.record.updated_at))
        .then_with(|| a.record.id.cmp(&b.record.id))
}

fn mission_status_rank(status: &str) -> u8 {
    match status {
        "active" => 0,
        "ready" => 1,
        "draft" => 2,
        "superseded" => 3,
        "closed" => 5,
        _ => 4,
    }
}

fn mission_status_summary_text(total: usize, statuses: BTreeMap<String, usize>) -> String {
    if statuses.len() == 1 {
        let (status, count) = statuses
            .into_iter()
            .next()
            .unwrap_or_else(|| ("mission".to_string(), total));
        format!("{count} {status} {}", plural_noun(count, "mission"))
    } else if statuses.is_empty() {
        "0 missions".to_string()
    } else {
        format!("{total} missions | {}", joined_plain_counts(statuses))
    }
}

fn joined_plain_counts(counts: BTreeMap<String, usize>) -> String {
    counts
        .into_iter()
        .map(|(status, count)| format!("{count} {status}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn epic_work_counts(db: &Database, epic_id: &str) -> Result<WorkCounts> {
    let mut counts = WorkCounts::default();
    let mut seen = BTreeSet::new();
    collect_descendant_work_counts(db, epic_id, &mut seen, &mut counts)?;
    Ok(counts)
}

fn collect_descendant_work_counts(
    db: &Database,
    parent_id: &str,
    seen: &mut BTreeSet<String>,
    counts: &mut WorkCounts,
) -> Result<()> {
    for child in db.get_subissues(parent_id)? {
        if !seen.insert(child.id.clone()) {
            continue;
        }
        counts.add_bucket(issue_bucket(db, &child)?);
        collect_descendant_work_counts(db, &child.id, seen, counts)?;
    }
    Ok(())
}

fn duplicate_reachability(
    db: &Database,
    mission_issue_ids: &BTreeSet<String>,
    linked_root_ids: &BTreeSet<String>,
) -> Result<Vec<DuplicateReachability>> {
    let mut duplicates = Vec::new();
    for issue_id in mission_issue_ids {
        if let Some(roots) = duplicate_reachability_roots(db, issue_id, linked_root_ids)? {
            duplicates.push(DuplicateReachability {
                issue_id: issue_id.clone(),
                roots,
            });
        }
    }
    duplicates.sort_by(|a, b| a.issue_id.cmp(&b.issue_id));
    Ok(duplicates)
}

fn duplicate_reachability_roots(
    db: &Database,
    issue_id: &str,
    linked_root_ids: &BTreeSet<String>,
) -> Result<Option<Vec<String>>> {
    let mut roots = Vec::new();
    let mut current_id = Some(issue_id.to_string());
    while let Some(id) = current_id {
        if linked_root_ids.contains(&id) {
            roots.push(id.clone());
        }
        current_id = db.require_issue(&id)?.parent_id;
    }
    if roots.len() > 1 {
        Ok(Some(roots))
    } else {
        Ok(None)
    }
}

fn format_duplicate_reachability(duplicate: &DuplicateReachability) -> String {
    let mut qualifiers = duplicate
        .roots
        .iter()
        .map(|root| {
            if root == &duplicate.issue_id {
                "direct".to_string()
            } else {
                root.clone()
            }
        })
        .collect::<Vec<_>>();
    qualifiers.sort();
    format!("{} ({})", duplicate.issue_id, qualifiers.join(" + "))
}

fn has_ancestor_in_set(
    db: &Database,
    issue: &Issue,
    ancestor_ids: &BTreeSet<String>,
) -> Result<bool> {
    let mut parent_id = issue.parent_id.clone();
    while let Some(id) = parent_id {
        if ancestor_ids.contains(&id) {
            return Ok(true);
        }
        parent_id = db.require_issue(&id)?.parent_id;
    }
    Ok(false)
}

impl WorkCounts {
    fn add_bucket(&mut self, bucket: &str) {
        match bucket {
            "ready" => self.ready += 1,
            "blocked" => self.blocked += 1,
            "done" => self.done += 1,
            _ => self.backlog += 1,
        }
    }

    fn is_empty(self) -> bool {
        self.ready == 0 && self.blocked == 0 && self.done == 0 && self.backlog == 0
    }

    fn to_compact_text(self) -> String {
        let mut parts = Vec::new();
        if self.ready > 0 {
            parts.push(count_label(self.ready, "ready"));
        }
        if self.blocked > 0 {
            parts.push(count_label(self.blocked, "blocked"));
        }
        if self.done > 0 {
            parts.push(count_label(self.done, "done"));
        }
        if self.backlog > 0 {
            parts.push(count_label(self.backlog, "backlog"));
        }
        if parts.is_empty() {
            "none".to_string()
        } else {
            parts.join(", ")
        }
    }
}

struct TrackerHealth {
    stale_entries: Vec<String>,
}

impl TrackerHealth {
    fn status_token(&self) -> &'static str {
        if self.stale_entries.is_empty() {
            "ok"
        } else {
            "stale"
        }
    }

    fn status_text(&self) -> String {
        if self.stale_entries.is_empty() {
            "ok".to_string()
        } else {
            format!("stale ({} findings)", self.stale_entries.len())
        }
    }
}

fn tracker_health(db: &Database, state_dir: &Path) -> TrackerHealth {
    let stale_entries = atelier_app::export::canonical_stale_entries(db, state_dir)
        .unwrap_or_else(|error| vec![format!("committed-state check failed: {error:#}")]);
    TrackerHealth { stale_entries }
}

fn mission_health(summary: &MissionListSummary) -> &'static str {
    let work = summary.total_work();
    if summary.open_blockers > 0 || work.blocked > 0 {
        "blocked"
    } else if summary.terminal_ready() {
        "terminal"
    } else if work.ready > 0 {
        "ready"
    } else if summary.approval_pending_count > 0 {
        "needs-evidence"
    } else {
        "steady"
    }
}

fn mission_health_for(mission: &RecordSummary, summary: &MissionListSummary) -> &'static str {
    if mission_lifecycle_status(mission) == "closed" {
        "closed"
    } else {
        mission_health(summary)
    }
}

fn active_work_for_mission(db: &Database, mission_id: &str) -> Result<Vec<Issue>> {
    let issue_ids = crate::commands::objective_status::mission_issue_ids(db, mission_id)?;
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let issues = db
        .list_issues(Some("all"), None, None)?
        .into_iter()
        .filter(|issue| issue_ids.contains(&issue.id))
        .filter(|issue| {
            crate::commands::issue_workflow::issue_status_category(
                workflow_policy.as_ref(),
                &issue.status,
            )
            .as_deref()
                == Some("active")
        })
        .collect::<Vec<_>>();
    crate::commands::objective_status::order_issues_by_work_with_default(db, issues)
}

fn count_label(count: usize, label: &str) -> String {
    format!("{count} {label}")
}

fn plural_noun(count: usize, noun: &str) -> String {
    if count == 1 {
        noun.to_string()
    } else {
        format!("{noun}s")
    }
}

fn print_mission_heading(title: &str) {
    println!("\n{title}");
    println!("{}", "-".repeat(title.len()));
}

fn issue_bucket(db: &Database, issue: &Issue) -> Result<&'static str> {
    let workflow_policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    if crate::commands::issue_workflow::issue_is_done(workflow_policy.as_ref(), issue) {
        return Ok("done");
    }
    if !crate::commands::objective_status::open_issue_blockers_with_default(db, &issue.id)?
        .is_empty()
    {
        return Ok("blocked");
    }
    mission_issue_state(db, issue).map(|state| if state == "ready" { "ready" } else { "backlog" })
}

fn mission_issue_state(db: &Database, issue: &Issue) -> Result<&'static str> {
    Ok(
        crate::commands::objective_status::work_order_row_for_issue_with_default(db, issue)?
            .state()
            .label(),
    )
}

fn order_blocked_work(
    db: &Database,
    blocked: Vec<BlockedMissionWork>,
) -> Result<Vec<BlockedMissionWork>> {
    let rows = blocked
        .iter()
        .map(|row| {
            crate::commands::objective_status::work_order_row_for_issue_with_default(db, &row.issue)
        })
        .collect::<Result<Vec<_>>>()?;
    let mut keyed = blocked.into_iter().map(Some).collect::<Vec<_>>();
    Ok(crate::commands::work_order::ordered_work_indices(&rows)
        .into_iter()
        .filter_map(|index| keyed[index].take())
        .collect())
}

fn order_mission_epics(db: &Database, epics: Vec<MissionListEpic>) -> Result<Vec<MissionListEpic>> {
    let rows = epics
        .iter()
        .map(|epic| {
            crate::commands::objective_status::work_order_row_for_issue_with_default(
                db,
                &epic.issue,
            )
        })
        .collect::<Result<Vec<_>>>()?;
    let mut keyed = epics.into_iter().map(Some).collect::<Vec<_>>();
    Ok(crate::commands::work_order::ordered_work_indices(&rows)
        .into_iter()
        .filter_map(|index| keyed[index].take())
        .collect())
}
