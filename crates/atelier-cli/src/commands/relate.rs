use crate::human_output::{print_footer, FooterAction};
use anyhow::{anyhow, bail, Result};
use std::path::Path;

use atelier_app::project_config::ProjectConfig;
use atelier_app::record_index::{built_in_issue_link_types, RecordIndex};
use atelier_core::InvalidValue;
use atelier_records::RecordStore;

const BLOCKED_BY_ROLE: &str = "blocked_by";
/// The inverse of `blocked_by`: the target waits for the source.
const BLOCKS_ROLE: &str = "blocks";

pub fn change_issue_link(
    index: &RecordIndex,
    state_dir: &Path,
    issue_ref: &str,
    target_ref: &str,
    role: &str,
    unlink: bool,
    quiet: bool,
) -> Result<()> {
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "Cannot determine repository root for {}",
            state_dir.display()
        )
    })?;
    // The first record declares the link and is the only one written.
    let declared_by_blocker = role == BLOCKS_ROLE;
    let (issue_ref, target_ref, role) = if declared_by_blocker {
        (target_ref, issue_ref, BLOCKED_BY_ROLE)
    } else {
        (issue_ref, target_ref, role)
    };
    let source = resolve_link_endpoint(index, issue_ref)?;
    validate_issue_link_role(repo_root, role, &source)?;
    let target = resolve_link_endpoint(index, target_ref)?;
    if role == "advances" && !unlink {
        atelier_app::workflow_policy::validate_advances_endpoints(
            &source,
            &index.require_issue(&source)?.issue_type,
            &index.require_issue(&target)?.issue_type,
        )?;
    }
    let store = RecordStore::new(state_dir);
    let changed = match (unlink, role == BLOCKED_BY_ROLE) {
        (false, true) => store.add_issue_block(&source, &target, declared_by_blocker)?,
        (false, false) => store.add_issue_relation(&source, &target, role)?,
        (true, true) => store.remove_issue_block(&source, &target)?,
        (true, false) => store.remove_issue_relation(&source, &target, role)?,
    };
    if quiet {
        println!(
            "{}",
            if declared_by_blocker {
                &target
            } else {
                &source
            }
        );
        return Ok(());
    }
    match (unlink, changed) {
        (false, true) => println!("Linked {source} -> {target} ({role})"),
        (false, false) => println!("Link {source} -> {target} ({role}) already exists"),
        (true, true) => println!("Unlinked {source} -> {target} ({role})"),
        (true, false) => println!("No link {source} -> {target} ({role}) exists"),
    }
    let policy = crate::commands::issue_workflow::load_issue_workflow_policy()?;
    let has_transitions = crate::commands::issue_workflow::issue_has_transitions(
        policy.as_ref(),
        &index.require_issue(&source)?,
    );
    print_link_next_commands(&source, &target, has_transitions);
    Ok(())
}

/// Proof roles name evidence links, which `evidence record` and `evidence
/// attach` write; an issue link never takes one.
const PROOF_ROLES: [&str; 2] = ["validates", "evidenced_by"];

fn validate_issue_link_role(repo_root: &Path, role: &str, source: &str) -> Result<()> {
    let builtin = || {
        built_in_issue_link_types()
            .into_iter()
            .filter(|known| !PROOF_ROLES.contains(known))
    };
    if builtin().any(|known| known == role) {
        return Ok(());
    }
    let config = ProjectConfig::load(repo_root)?;
    let custom = config
        .issue_links
        .custom_context_types
        .iter()
        .map(String::as_str)
        .filter(|configured| !PROOF_ROLES.contains(configured));
    if custom.clone().any(|configured| configured == role) {
        return Ok(());
    }
    let mut valid = builtin().chain([BLOCKS_ROLE]).collect::<Vec<_>>();
    valid.sort_unstable();
    valid.extend(custom);
    let invalid = InvalidValue::new("role", role, &valid);
    bail!(if PROOF_ROLES.contains(&role) {
        invalid.with_hint(&format!(
            "Proof roles are reserved for evidence links; use `atelier evidence record --target issue/{source} --kind validation \"...\"` or `atelier evidence attach <evidence-id> issue {source}`."
        ))
    } else {
        invalid.with_hint("Custom roles are configured in .atelier/config.toml [issue_links].custom_context_types.")
    })
}

fn resolve_link_endpoint(index: &RecordIndex, reference: &str) -> Result<String> {
    index.resolve(atelier_app::record_ref::RecordKind::Issue, reference)
}

/// A status with no transitions out, as most done statuses have, offers no
/// transition options.
fn print_link_next_commands(issue_id: &str, target_id: &str, has_transitions: bool) {
    let mut actions = vec![FooterAction::new(
        "Inspect this issue",
        format!("atelier issue show {issue_id}"),
    )];
    if has_transitions {
        actions.push(FooterAction::new(
            "Show transition options",
            format!("atelier issue transition {issue_id}"),
        ));
    }
    actions.push(FooterAction::new(
        "Inspect the target",
        format!("atelier issue show {target_id}"),
    ));
    print_footer(actions);
}
