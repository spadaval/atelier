#![no_main]

//! Fuzz target for CLI output functions.
//!
//! This tests the presentation layer (list, show, etc.) which handles
//! string truncation and formatting. The goal is to catch panics from
//! improper UTF-8 handling like byte slicing on multi-byte characters.

use arbitrary::Arbitrary;
use atelier_app::issue::{CreateIssueRequest, IssueJob, ListIssuesRequest};
use atelier_app::mission::{MissionJob, MissionStatusRequest};
use atelier_app::status::{
    GitStatusView, IssueSummary, ResultView, StatusViewModel, TrackerState,
};
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct CliOutputInput {
    /// Issue title - can contain any Unicode
    title: String,
    /// Issue description - can contain any Unicode
    description: Option<String>,
    /// Number of issues to create (for list testing)
    num_issues: u8,
    status: String,
    priority: String,
    quiet: bool,
}

fuzz_target!(|input: CliOutputInput| {
    let num_issues = (input.num_issues % 20).max(1);
    let mut current_work = Vec::new();
    for i in 0..num_issues {
        let title = if i == 0 {
            input.title.clone()
        } else {
            format!("{} #{}", input.title, i)
        };
        current_work.push(IssueSummary {
            id: format!("atelier-f{:04}", i),
            title,
            status: "in_progress".to_string(),
            issue_type: "task".to_string(),
            parent_id: None,
        });
    }

    let view = StatusViewModel {
        tracker_state: if input.quiet {
            TrackerState::Current
        } else {
            TrackerState::Stale {
                issue_count: current_work.len(),
            }
        },
        ready_work_count: current_work.len().saturating_sub(1),
        current_work,
        active_mission: None,
        current_mission_count: num_issues as usize,
        git: ResultView::Available(GitStatusView {
            branch: Some(input.title.chars().take(80).collect()),
            dirty_entries: input.description.into_iter().collect(),
        }),
        active_mission_snapshot: None,
    };

    let _ = view.quiet_line();
    let _ = view.current_work_lines();
    let _ = view.next_actions();

    let create_job = IssueJob::Create(CreateIssueRequest {
        title: input.title,
        description: None,
        priority: input.priority,
        template: None,
        labels: Vec::new(),
        issue_type: Some("task".to_string()),
        parent: None,
        quiet: input.quiet,
    });
    let list_job = IssueJob::List(ListIssuesRequest {
        status: input.status,
        category: None,
        label: None,
        priority: None,
        ready: input.quiet,
        blocked: false,
        quiet: input.quiet,
    });
    let mission_job = MissionJob::Status(MissionStatusRequest {
        id: Some(atelier_cli::app_crate_name().to_string()),
        quiet: input.quiet,
        closeout: false,
        verbose: !input.quiet,
    });
    let _ = create_job.command_group();
    let _ = list_job.command_group();
    let _ = mission_job.command_group();
});
