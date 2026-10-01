use anyhow::{anyhow, bail, Result};
use std::path::Path;

use atelier_app::project_config::ProjectConfig;
use atelier_app::record_index::{built_in_issue_link_types, RecordIndex};
use atelier_records::{
    validate_relation_type, RecordStore, WELL_KNOWN_LINK_TYPES, WELL_KNOWN_RELATION_TYPES,
};

const BLOCKED_BY_ROLE: &str = "blocked_by";

struct LinkEndpoint {
    kind: String,
    id: String,
}

pub fn link_issue(
    index: &RecordIndex,
    state_dir: &Path,
    issue_ref: &str,
    target_ref: &str,
    role: &str,
) -> Result<()> {
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "Cannot determine repository root for {}",
            state_dir.display()
        )
    })?;
    validate_issue_link_role(repo_root, role)?;
    let source = resolve_link_endpoint(index, issue_ref)?;
    let target = resolve_link_endpoint(index, target_ref)?;
    let store = RecordStore::new(state_dir);
    let changed = if source.kind == "issue" && target.kind == "issue" && role == BLOCKED_BY_ROLE {
        store.add_issue_block(&source.id, &target.id)?
    } else if source.kind == "issue" && target.kind == "issue" {
        store.add_issue_relation(&source.id, &target.id, role)?
    } else {
        bail!("issue link only supports issue records");
    };
    if changed {
        println!("Linked {} -> {} ({role})", source.id, target.id);
    } else {
        println!(
            "Link {} -> {} ({role}) already exists",
            source.id, target.id
        );
    }
    print_link_next_commands(&source.id, &target.id);
    Ok(())
}

pub fn unlink_issue(
    index: &RecordIndex,
    state_dir: &Path,
    issue_ref: &str,
    target_ref: &str,
    role: &str,
) -> Result<()> {
    let repo_root = state_dir.parent().ok_or_else(|| {
        anyhow!(
            "Cannot determine repository root for {}",
            state_dir.display()
        )
    })?;
    validate_issue_link_role(repo_root, role)?;
    let source = resolve_link_endpoint(index, issue_ref)?;
    let target = resolve_link_endpoint(index, target_ref)?;
    let store = RecordStore::new(state_dir);
    let changed = if source.kind == "issue" && target.kind == "issue" && role == BLOCKED_BY_ROLE {
        store.remove_issue_block(&source.id, &target.id)?
    } else if source.kind == "issue" && target.kind == "issue" {
        store.remove_issue_relation(&source.id, &target.id, role)?
    } else {
        bail!("issue unlink only supports issue records");
    };
    if changed {
        println!("Unlinked {} -> {} ({role})", source.id, target.id);
    } else {
        println!("No link {} -> {} ({role}) exists", source.id, target.id);
    }
    print_link_next_commands(&source.id, &target.id);
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

fn resolve_link_endpoint(index: &RecordIndex, reference: &str) -> Result<LinkEndpoint> {
    if let Some(issue_id) = index.resolve_issue_ref(reference)? {
        return Ok(LinkEndpoint {
            kind: "issue".to_string(),
            id: issue_id,
        });
    }
    if let Some(kind) = index.record_kind_for_id(reference) {
        bail!("{reference} is a {kind} record, not an issue record");
    }
    Err(anyhow!("Issue {reference} was not found"))
}

fn print_link_next_commands(issue_id: &str, target_id: &str) {
    println!("Next Commands");
    println!("-------------");
    println!("  atelier issue show {issue_id}");
    println!("  atelier issue transition {issue_id}");
    println!("  atelier issue show {target_id}");
}
