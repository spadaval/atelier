use anyhow::Result;
use std::path::Path;

use crate::commands;
use crate::db::Database;

pub fn run(db: &Database, state_dir: &Path, repo_root: &Path) -> Result<()> {
    let active_work = db.get_active_work_association()?;
    let active_mission = commands::mission::active_mission(db)?;
    let ready = db.list_ready_issues()?;
    let stale_entries = commands::export::canonical_stale_entries(db, state_dir)?;
    let tracker_state = if stale_entries.is_empty() {
        "current"
    } else {
        "stale"
    };

    println!("Atelier Prime");
    println!("=============");
    println!(
        "Repository: {}",
        repo_root
            .canonicalize()
            .unwrap_or_else(|_| repo_root.to_path_buf())
            .display()
    );
    println!("Tracker:    {tracker_state}");

    print_context_recovery(
        db,
        state_dir,
        active_mission.as_ref(),
        active_work.as_ref(),
        ready.len(),
    )?;
    print_core_rules();
    print_essential_commands(active_mission.as_ref(), active_work.as_ref());
    print_common_workflows(active_mission.as_ref(), active_work.as_ref());
    print_validation_checklist();
    print_repository_notes();
    Ok(())
}

fn print_context_recovery(
    db: &Database,
    state_dir: &Path,
    active_mission: Option<&crate::models::DomainRecord>,
    active_work: Option<&crate::models::WorkAssociation>,
    ready_count: usize,
) -> Result<()> {
    println!("\nContext Recovery");
    println!("----------------");
    println!(
        "  Durable tracker state: {} canonical Markdown",
        state_dir.display()
    );
    println!("  Ignored local state: .atelier/state.db, .atelier/runtime/, .atelier/cache/");
    match active_mission {
        Some(mission) => println!("  Active mission: {} - {}", mission.id, mission.title),
        None => println!(
            "  Active mission: none; use `atelier mission list` to inspect current missions."
        ),
    }
    match active_work {
        Some(work) => {
            let title = db
                .get_issue(&work.issue_id)?
                .map(|issue| issue.title)
                .unwrap_or_else(|| "(issue missing)".to_string());
            println!("  Active work: {} - {}", work.issue_id, title);
        }
        None => println!("  Active work: none; use `atelier issue list --ready` to choose work."),
    }
    println!("  Ready work: {ready_count}; use `atelier status` for live checkout state.");
    Ok(())
}

fn print_core_rules() {
    println!("\nCore Rules");
    println!("----------");
    println!("  `atelier status` - Check current checkout state before acting.");
    println!("  `.atelier/` Markdown - Treat committed records as durable tracker state.");
    println!("  `.atelier/state.db` - Treat the SQLite file as rebuildable local runtime state.");
    println!("  `atelier issue show <id>` - Read the assigned issue contract before editing.");
    println!("  `atelier history --issue <id>` - Inspect full canonical activity instead of relying on chat memory.");
}

fn print_essential_commands(
    active_mission: Option<&crate::models::DomainRecord>,
    active_work: Option<&crate::models::WorkAssociation>,
) {
    println!("\nEssential Commands");
    println!("------------------");
    println!("  `atelier prime` - Rebuild the operating map for this repository.");
    println!("  `atelier status` - Check active work, active mission, ready count, and tracker freshness.");
    println!("  `atelier mission status [<id>]` - Inspect mission readiness, blockers, evidence gaps, and closeout gates.");
    println!(
        "  `atelier mission show <id>` - Read the durable mission record and linked work graph."
    );
    println!("  `atelier issue list --ready` - Find executable open work with no open blockers.");
    println!("  `atelier issue show <id>` - Read Description, Outcome, Evidence, blockers, and next commands.");
    println!("  `atelier start <issue-id>` - Apply the configured start transition and record active local work.");
    println!("  `atelier issue transition <id> --options` - Inspect the next configured workflow steps for an issue.");
    println!("  `atelier issue close <id> --reason \"summary\"` - Apply the configured terminal transition and clear active local work.");
    println!("  `atelier abandon [issue-id] --reason \"...\"` - Clear local active work without changing issue status.");
    println!("  `atelier evidence record --target issue/<issue-id> --kind validation --result pass \"summary\"` - Create first-class proof on the accountable issue.");
    println!("  `atelier evidence attach <evidence-id> issue <issue-id>` - Reuse existing proof on an additional target when mirroring or sharing evidence.");
    println!("  `atelier history --mission <id>` - Inspect mission and linked-work activity without expanding show output.");
    println!("  `atelier export --check` - Verify committed Markdown projection freshness when handoff requires it.");
    println!("  `atelier lint` - Validate canonical tracker records before closeout.");
    println!("  `atelier doctor` - Check runtime and exported-state health when commands report tracker trouble.");
    if let Some(mission) = active_mission {
        println!(
            "  `atelier mission status {}` - Drill into the active mission named above.",
            mission.id
        );
    }
    if let Some(work) = active_work {
        println!(
            "  `atelier issue show {}` - Reopen the active work contract named above.",
            work.issue_id
        );
    }
}

fn print_common_workflows(
    active_mission: Option<&crate::models::DomainRecord>,
    active_work: Option<&crate::models::WorkAssociation>,
) {
    println!("\nCommon Workflows");
    println!("----------------");
    println!("  Start assigned issue: `atelier issue show <id>` to verify the contract, then `atelier start <id>` to record active work.");
    println!("  Recover after checkout: `atelier prime` to reload rules, then `atelier status` to inspect live state.");
    println!("  Advance active work: `atelier issue transition <id> --options` to inspect the next workflow step before review, validation, close, or archive.");
    println!("  Validate and close an issue: `atelier evidence record --target issue/<id> --kind validation --result pass \"summary\"`, `atelier issue transition <id> --options`, then `atelier issue close <id> --reason \"summary\"`.");
    println!("  Stop local work without closing: `atelier abandon [issue-id] --reason \"...\"`.");
    println!("  Inspect activity: `atelier history --issue <id>` for one issue or `atelier history --mission <id>` for mission-linked work.");
    match active_mission {
        Some(mission) => println!(
            "  Continue active mission: `atelier mission status {}` to find ready, blocked, and closeout work.",
            mission.id
        ),
        None => println!("  Find mission focus: `atelier mission list` to choose a current mission before coordinating work."),
    }
    match active_work {
        Some(work) => println!(
            "  Resume active work: `atelier issue show {}` to reload the contract before editing.",
            work.issue_id
        ),
        None => {
            println!("  Select work: `atelier issue list --ready` to choose an executable issue.")
        }
    }
}

fn print_validation_checklist() {
    println!("\nValidation/Closeout Checklist");
    println!("-----------------------------");
    println!("  `cargo fmt -- --check` - Verify Rust formatting when Rust files changed.");
    println!(
        "  `cargo nextest run <filter>` - Run focused Rust tests that prove the edited behavior."
    );
    println!("  `git diff --check` - Catch whitespace errors before handoff.");
    println!("  `atelier export --check` - Prove canonical tracker records are current when tracker state changed.");
    println!("  `atelier lint` - Prove tracker records remain parseable and policy-compliant.");
    println!(
        "  `atelier doctor` - Prove runtime health when the handoff claims repository readiness."
    );
}

fn print_repository_notes() {
    println!("\nRepository Notes");
    println!("----------------");
    println!("  `AGENTFACTORY.md` - Read the bound mission, tracker, proof, and validation rules.");
    println!("  `target/debug/atelier` - Use the local binary only when testing CLI edits before installation.");
    println!("  `atelier history` - Use canonical project history; local diagnostics are intentionally separate.");
}
