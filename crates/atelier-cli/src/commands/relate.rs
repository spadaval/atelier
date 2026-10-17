use crate::human_output::{print_footer, FooterAction};
use anyhow::{anyhow, bail, Result};
use std::path::Path;

use atelier_app::project_config::ProjectConfig;
use atelier_app::record_index::{built_in_issue_link_types, RecordIndex};
use atelier_records::{
    validate_relation_type, RecordStore, WELL_KNOWN_LINK_TYPES, WELL_KNOWN_RELATION_TYPES,
};

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
    validate_issue_link_role(repo_root, role)?;
    let source = resolve_link_endpoint(index, issue_ref)?;
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
        return Ok(());
    }
    match (unlink, changed) {
        (false, true) => println!("Linked {source} -> {target} ({role})"),
        (false, false) => println!("Link {source} -> {target} ({role}) already exists"),
        (true, true) => println!("Unlinked {source} -> {target} ({role})"),
        (true, false) => println!("No link {source} -> {target} ({role}) exists"),
    }
    print_link_next_commands(&source, &target);
    Ok(())
}

fn validate_issue_link_role(repo_root: &Path, role: &str) -> Result<()> {
    if matches!(role, "validates" | "evidenced_by") {
        bail!(
            "Invalid issue link role '{}'. Proof roles are reserved for evidence links; use `atelier evidence record --target issue/<id> --kind validation \"...\"` or `atelier evidence attach <evidence-id> issue <id>`.",
            role
        );
    }
    if WELL_KNOWN_LINK_TYPES.contains(&role) || WELL_KNOWN_RELATION_TYPES.contains(&role) {
        return Ok(());
    }
    validate_relation_type(role)?;
    let config = ProjectConfig::load(repo_root)?;
    if config
        .issue_links
        .custom_context_types
        .iter()
        .any(|configured| configured == role)
    {
        return Ok(());
    }
    let configured = if config.issue_links.custom_context_types.is_empty() {
        "(none)".to_string()
    } else {
        config.issue_links.custom_context_types.join(", ")
    };
    bail!(
        "Invalid issue link role '{}'. Built-in workflow roles: {}. Configured custom context-only roles: {}. Configure custom issue links in .atelier/config.toml [issue_links].custom_context_types.",
        role,
        built_in_issue_link_types().join(", "),
        configured
    )
}

fn resolve_link_endpoint(index: &RecordIndex, reference: &str) -> Result<String> {
    if let Some(issue_id) = index.resolve_issue_ref(reference)? {
        return Ok(issue_id);
    }
    if let Some(kind) = index.record_kind_for_id(reference) {
        bail!("{reference} is a {kind} record, not an issue record");
    }
    Err(anyhow!("Issue {reference} was not found"))
}

fn print_link_next_commands(issue_id: &str, target_id: &str) {
    print_footer([
        FooterAction::new(
            "Inspect this issue",
            format!("atelier issue show {issue_id}"),
        ),
        FooterAction::new(
            "Show transition options",
            format!("atelier issue transition {issue_id}"),
        ),
        FooterAction::new(
            "Inspect the target",
            format!("atelier issue show {target_id}"),
        ),
    ]);
}
