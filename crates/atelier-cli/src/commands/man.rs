use anyhow::{bail, Result};

use crate::commands;
use atelier_app::command_storage::{command_storage, CommandStorageAccess};
use atelier_records::activity::{list_derived_issue_attempts, DerivedIssueAttemptState};
use atelier_sqlite::Database;

const ROLES: &[&str] = &["worker", "reviewer", "validator", "manager", "admin"];

pub fn run(role: Option<String>) -> Result<()> {
    let Some(role) = role else {
        print_role_index();
        return Ok(());
    };

    match role.as_str() {
        "worker" => run_stateful(Role::Worker),
        "reviewer" => run_stateful(Role::Reviewer),
        "validator" => run_stateful(Role::Validator),
        "manager" => run_stateful(Role::Manager),
        "admin" => run_admin(),
        _ => bail!(
            "unknown man role '{role}'\nValid roles: {}\nUse `atelier man` to list role guides.",
            ROLES.join(", ")
        ),
    }
}

#[derive(Debug, Clone, Copy)]
enum Role {
    Worker,
    Reviewer,
    Validator,
    Manager,
    Admin,
}

impl Role {
    fn title(self) -> &'static str {
        match self {
            Role::Worker => "Worker",
            Role::Reviewer => "Reviewer",
            Role::Validator => "Validator",
            Role::Manager => "Manager",
            Role::Admin => "Admin",
        }
    }
}

struct Snapshot {
    tracker: String,
    repo: String,
    active_mission: Option<String>,
    current_work: Vec<String>,
    active_sessions: Vec<String>,
    ready_count: usize,
    stale_count: usize,
}

fn run_stateful(role: Role) -> Result<()> {
    let storage = command_storage(CommandStorageAccess::ProjectionQuery).map_err(|error| {
        anyhow::anyhow!(
            "{error:#}\nRecovery: use `atelier man admin` for setup/repair guidance, or run `atelier doctor` when this is an initialized checkout."
        )
    })?;
    let repo = storage.repo_root().display().to_string();
    let snapshot = snapshot(storage.db(), &storage.state_dir(), &repo)?;
    print_role_guide(role, Some(&snapshot), None);
    Ok(())
}

fn run_admin() -> Result<()> {
    match command_storage(CommandStorageAccess::HealthRepair) {
        Ok(storage) => {
            let repo = storage.repo_root().display().to_string();
            let snapshot = snapshot(storage.db(), &storage.state_dir(), &repo)?;
            print_role_guide(Role::Admin, Some(&snapshot), None);
        }
        Err(error) => {
            print_role_guide(Role::Admin, None, Some(&format!("{error:#}")));
        }
    }
    Ok(())
}

fn snapshot(db: &Database, state_dir: &std::path::Path, repo: &str) -> Result<Snapshot> {
    let active_mission = commands::mission::active_mission(db)?
        .map(|mission| format!("{} - {}", mission.id, mission.title));
    let workflow_policy = commands::issue_workflow::load_issue_workflow_policy()?;
    let current_work = commands::status::current_work_issues(db, workflow_policy.as_ref())?
        .into_iter()
        .map(|issue| format!("{} - {}", issue.id, issue.title))
        .collect();
    let active_sessions = list_derived_issue_attempts(state_dir)?
        .into_iter()
        .filter(|session| session.state == DerivedIssueAttemptState::Active)
        .map(|session| {
            format!(
                "{} {} {} -> issue/{}",
                session.id,
                session.role,
                session.state.as_str(),
                session.issue_id
            )
        })
        .collect();
    let ready_count = db.list_ready_issues()?.len();
    let stale_count = atelier_app::export::canonical_stale_entries(db, state_dir)?.len();
    let tracker = if stale_count == 0 { "current" } else { "stale" }.to_string();
    Ok(Snapshot {
        tracker,
        repo: repo.to_string(),
        active_mission,
        current_work,
        active_sessions,
        ready_count,
        stale_count,
    })
}

fn print_role_index() {
    println!("Atelier Man");
    println!("===========");
    println!("Role guides filter the existing command surface for the job at hand.");
    println!();
    println!("Roles");
    println!("-----");
    println!("  worker    Implement assigned or ready issue work.");
    println!("  reviewer  Check proof, review outputs, and validate transitions.");
    println!("  validator Run explicit validation and record validation proof.");
    println!(
        "  manager   Create, organize, and coordinate missions, planning artifacts, and work."
    );
    println!("  admin     Set up, repair, migrate, and maintain Atelier state.");
    println!();
    println!("Commands");
    println!("--------");
    println!("  atelier man worker");
    println!("  atelier man reviewer");
    println!("  atelier man validator");
    println!("  atelier man manager");
    println!("  atelier man admin");
}

fn print_role_guide(role: Role, snapshot: Option<&Snapshot>, state_error: Option<&str>) {
    println!("Atelier Man: {}", role.title());
    println!("{}", "=".repeat("Atelier Man: ".len() + role.title().len()));
    print_current_state(snapshot, state_error);
    print_relevant_commands(role, snapshot);
    print_normal_loop(role);
    print_not_usually(role);
}

fn print_current_state(snapshot: Option<&Snapshot>, state_error: Option<&str>) {
    println!("\nCurrent State");
    println!("-------------");
    match snapshot {
        Some(snapshot) => {
            println!("  Repository: {}", snapshot.repo);
            println!("  Tracker:    {}", snapshot.tracker);
            if snapshot.stale_count > 0 {
                println!("  Stale records: {}", snapshot.stale_count);
            }
            match &snapshot.active_mission {
                Some(mission) => println!("  Active mission: {mission}"),
                None => println!("  Active mission: none"),
            }
            if snapshot.current_work.is_empty() {
                println!("  Current work:   none");
            } else {
                println!("  Current work:   {} issue(s)", snapshot.current_work.len());
                for work in snapshot.current_work.iter().take(3) {
                    println!("    {work}");
                }
                if snapshot.current_work.len() > 3 {
                    println!(
                        "    ... and {} more issue(s)",
                        snapshot.current_work.len() - 3
                    );
                }
            }
            if snapshot.active_sessions.is_empty() {
                println!("  Active sessions: none");
            } else {
                println!("  Active sessions: {}", snapshot.active_sessions.len());
                for session in snapshot.active_sessions.iter().take(3) {
                    println!("    {session}");
                }
                if snapshot.active_sessions.len() > 3 {
                    println!(
                        "    ... and {} more session(s)",
                        snapshot.active_sessions.len() - 3
                    );
                }
            }
            println!("  Ready work:     {}", snapshot.ready_count);
        }
        None => {
            println!("  Tracker: unavailable");
            if let Some(error) = state_error {
                println!("  State error: {error}");
            }
        }
    }
}

fn print_relevant_commands(role: Role, snapshot: Option<&Snapshot>) {
    println!("\nMost Relevant Commands");
    println!("----------------------");
    match role {
        Role::Worker => {
            if snapshot
                .map(|s| !s.current_work.is_empty())
                .unwrap_or(false)
            {
                println!("  1. atelier status - Review the checkout's current-work set.");
                println!("  2. atelier session list --active - Review active session context.");
                println!("  3. atelier evidence record --target issue/<id> --kind test -- <command> - Attach proof.");
            } else {
                println!("  1. atelier issue list --ready - Find executable work.");
                println!("  2. atelier issue show <id> - Read the issue contract before editing.");
                println!("  3. atelier start <id> - Move the issue into the current-work set.");
            }
        }
        Role::Reviewer => {
            println!("  1. atelier issue transition <id> --options - Inspect workflow gates.");
            println!("  2. atelier evidence show <evidence-id> - Inspect attached proof.");
            println!("  3. atelier pr comments --issue <id> - Inspect PR discussion.");
        }
        Role::Validator => {
            println!(
                "  1. atelier issue show <id> - Read the validation target and proof contract."
            );
            println!("  2. atelier evidence record --target issue/<id> --kind validation -- <command> - Attach validation proof.");
            println!("  3. atelier pr review --issue <id> --role validator --event approve --body \"...\" - Record PR validation judgment.");
        }
        Role::Manager => {
            if snapshot.and_then(|s| s.active_mission.as_ref()).is_some() {
                println!(
                    "  1. atelier mission status - Review active mission readiness and blockers."
                );
                println!("  2. atelier issue create \"...\" - Create an actionable work item.");
                println!("  3. atelier mission add-work <mission-id> <issue-id> - Link work to mission scope.");
            } else {
                println!("  1. atelier mission list - Choose mission focus.");
                println!("  2. atelier mission start <id> --switch - Set active mission focus.");
                println!("  3. atelier graph tree --compact - Inspect work hierarchy.");
            }
        }
        Role::Admin => {
            println!("  1. atelier init - Create tracker scaffolding when missing.");
            println!("  2. atelier doctor - Inspect runtime and projection health.");
            println!("  3. atelier doctor --fix - Repair ignored local state when safe.");
        }
    }
}

fn print_normal_loop(role: Role) {
    println!("\nNormal Loop");
    println!("-----------");
    match role {
        Role::Worker => {
            println!("  atelier status");
            println!("  atelier session list --active");
            println!("  atelier issue list --ready");
            println!("  atelier issue show <id>");
            println!("  atelier start <id>");
            println!("  atelier pr comment --issue <id> --role worker --body \"...\"");
            println!("  atelier evidence record --target issue/<id> --kind test -- <command>");
            println!("  atelier issue close <id> --reason \"...\"");
        }
        Role::Reviewer => {
            println!("  atelier mission status");
            println!("  atelier issue show <id>");
            println!("  atelier issue transition <id> --options");
            println!("  atelier pr comments --issue <id>");
            println!("  atelier pr comment --issue <id> --role reviewer --body \"...\"");
            println!("  atelier pr review --issue <id> --role reviewer --event request-changes --body \"...\"");
            println!(
                "  atelier evidence record --target issue/<id> --kind validation -- <command>"
            );
            println!("  atelier history --issue <id>");
        }
        Role::Validator => {
            println!("  atelier issue show <id>");
            println!("  atelier issue transition <id> --options");
            println!("  atelier evidence show <evidence-id>");
            println!(
                "  atelier evidence record --target issue/<id> --kind validation -- <command>"
            );
            println!("  atelier pr comments --issue <id>");
            println!(
                "  atelier pr review --issue <id> --role validator --event approve --body \"...\""
            );
        }
        Role::Manager => {
            println!("  atelier mission status");
            println!("  atelier issue create \"...\"");
            println!("  atelier mission add-work <mission-id> <issue-id>");
            println!("  atelier issue block <blocked-id> <blocker-id>");
            println!("  atelier worktree for-mission <mission-id>");
        }
        Role::Admin => {
            println!("  atelier init");
            println!("  atelier lint");
            println!("  atelier doctor");
            println!("  atelier doctor --fix");
            println!("  atelier branch status");
        }
    }
}

fn print_not_usually(role: Role) {
    println!("\nNot Usually For This Role");
    println!("-------------------------");
    match role {
        Role::Worker => {
            println!("  maintenance, diagnostics, raw workflow checks, bundle apply, branch merge");
        }
        Role::Reviewer => {
            println!("  init, maintenance delete, issue creation, mission scope mutation");
        }
        Role::Validator => {
            println!("  issue creation, mission scope mutation, merge decisions outside assigned validation");
        }
        Role::Manager => {
            println!(
                "  diagnostics slow, rebuild, maintenance delete except during explicit repair"
            );
        }
        Role::Admin => {
            println!("  ordinary issue implementation, evidence capture for feature proof, mission terminal judgment");
        }
    }
}
