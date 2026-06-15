use anyhow::{bail, Result};

use crate::command_storage::{command_storage, CommandStorageAccess};
use crate::commands;
use crate::db::Database;

const ROLES: &[&str] = &["worker", "reviewer", "manager", "admin"];

pub fn run(role: Option<String>) -> Result<()> {
    let Some(role) = role else {
        print_role_index();
        return Ok(());
    };

    match role.as_str() {
        "worker" => run_stateful(Role::Worker),
        "reviewer" => run_stateful(Role::Reviewer),
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
    Manager,
    Admin,
}

impl Role {
    fn title(self) -> &'static str {
        match self {
            Role::Worker => "Worker",
            Role::Reviewer => "Reviewer",
            Role::Manager => "Manager",
            Role::Admin => "Admin",
        }
    }
}

struct Snapshot {
    tracker: String,
    repo: String,
    active_mission: Option<String>,
    active_work: Option<String>,
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
    let active_work = db.get_active_work_association()?.map(|work| {
        let title = db
            .get_issue(&work.issue_id)
            .ok()
            .flatten()
            .map(|issue| issue.title)
            .unwrap_or_else(|| "(issue missing)".to_string());
        format!("{} - {}", work.issue_id, title)
    });
    let ready_count = db.list_ready_issues()?.len();
    let stale_count = commands::export::canonical_stale_entries(db, state_dir)?.len();
    let tracker = if stale_count == 0 { "current" } else { "stale" }.to_string();
    Ok(Snapshot {
        tracker,
        repo: repo.to_string(),
        active_mission,
        active_work,
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
    println!("  manager   Create, organize, and coordinate missions, plans, and work.");
    println!("  admin     Set up, repair, migrate, and maintain Atelier state.");
    println!();
    println!("Commands");
    println!("--------");
    println!("  atelier man worker");
    println!("  atelier man reviewer");
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
            match &snapshot.active_work {
                Some(work) => println!("  Active work:    {work}"),
                None => println!("  Active work:    none"),
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
            if snapshot.and_then(|s| s.active_work.as_ref()).is_some() {
                println!("  1. atelier issue show <active-id> - Reopen the active work contract.");
                println!("  2. atelier evidence record --target issue/<id> --kind test --result pass -- <command> - Attach proof.");
                println!("  3. atelier issue transition <id> --options - Inspect allowed next workflow steps.");
            } else {
                println!("  1. atelier issue list --ready - Find executable work.");
                println!("  2. atelier issue show <id> - Read the issue contract before editing.");
                println!("  3. atelier start <id> - Record active local work.");
            }
        }
        Role::Reviewer => {
            println!("  1. atelier issue transition <id> --options - Inspect workflow gates.");
            println!("  2. atelier evidence show <evidence-id> - Inspect attached proof.");
            println!("  3. atelier lint <id> - Validate focused tracker state.");
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
            println!("  atelier issue list --ready");
            println!("  atelier issue show <id>");
            println!("  atelier start <id>");
            println!("  atelier evidence record --target issue/<id> --kind test --result pass -- <command>");
            println!("  atelier issue close <id> --reason \"...\"");
        }
        Role::Reviewer => {
            println!("  atelier mission status");
            println!("  atelier issue show <id>");
            println!("  atelier issue transition <id> --options");
            println!("  atelier evidence record --target issue/<id> --kind validation --result pass -- <command>");
            println!("  atelier history --issue <id>");
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
            println!("  atelier export --check");
        }
    }
}

fn print_not_usually(role: Role) {
    println!("\nNot Usually For This Role");
    println!("-------------------------");
    match role {
        Role::Worker => {
            println!(
                "  maintenance, diagnostics, raw workflow checks, bulk plan apply, branch merge"
            );
        }
        Role::Reviewer => {
            println!("  init, maintenance delete, issue creation, mission scope mutation");
        }
        Role::Manager => {
            println!(
                "  diagnostics slow, rebuild, maintenance delete except during explicit repair"
            );
        }
        Role::Admin => {
            println!("  ordinary issue implementation, evidence capture for feature proof, mission closeout judgment");
        }
    }
}
