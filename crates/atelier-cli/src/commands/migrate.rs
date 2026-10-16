//! One-shot migration of retired epic records to parent issues (atelier-9vq3).
//!
//! Epic records no longer parse, so this reads the raw Markdown instead of
//! loading the record index, plans every rewrite, validates each result with
//! the record parser, and writes nothing unless all of them are valid.
use anyhow::{bail, Context, Result};
use std::{fs, path::Path};

use atelier_app::storage_layout::StorageLayout;

const POLICY_EPIC_TYPE: &str = "  epic:\n    label: Epic\n";
const POLICY_EPIC_APPLIES: &str = "    - epic\n";
/// The starter policy's work `close` before parents existed; the migration
/// adds the check that makes a parent close after its children.
const POLICY_WORK_CLOSE: &str = "      close:\n        from: [in_progress, review]\n        to: done\n        required_fields: [close_reason]\n        description: Done claim; commit it on the work branch before review.\n        validators:\n        - blockers.none_open\n";
const POLICY_WORK_CLOSE_WITH_CHILDREN: &str = "      close:\n        from: [in_progress, review]\n        to: done\n        required_fields: [close_reason]\n        description: Done claim; commit it on the work branch before review. A parent closes after its children.\n        validators:\n        - blockers.none_open\n        - work.terminal\n";

struct Plan {
    path: std::path::PathBuf,
    id: String,
    text: String,
    status_change: Option<(String, String)>,
}

pub fn epics(to: &str, apply: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    let policy_path = repo.join(atelier_app::workflow_policy::WORKFLOW_POLICY_PATH);
    let policy_text = fs::read_to_string(&policy_path)
        .with_context(|| format!("cannot read {}", policy_path.display()))?;
    let mut new_policy = policy_text
        .replace(POLICY_EPIC_TYPE, "")
        .replace(POLICY_EPIC_APPLIES, "");
    let close_check_added = match new_policy.find("\n  work:\n") {
        Some(work) if new_policy[work..].contains(POLICY_WORK_CLOSE) => {
            let at = work
                + new_policy[work..]
                    .find(POLICY_WORK_CLOSE)
                    .unwrap_or_default();
            new_policy.replace_range(
                at..at + POLICY_WORK_CLOSE.len(),
                POLICY_WORK_CLOSE_WITH_CHILDREN,
            );
            true
        }
        _ => false,
    };
    let policy = atelier_workflow::parse_policy_text(
        &new_policy,
        atelier_app::workflow_policy::WORKFLOW_POLICY_PATH,
    )
    .context("the workflow policy without the epic type is invalid; edit it by hand")?;
    if new_policy.contains("epic") {
        bail!(
            "{} still mentions epic after removing the standard epic entries; remove the remaining references by hand, then rerun",
            policy_path.display()
        );
    }
    let close_has_check = policy
        .workflow_for_issue_type(to)
        .ok()
        .and_then(|workflow| workflow.transitions.get("close"))
        .is_some_and(|close| {
            close
                .validators
                .iter()
                .any(|validator| validator.builtin == "work.terminal")
        });
    if !policy.issue_types.contains_key(to) {
        bail!("issue type '{to}' is not declared in the workflow policy; pass --to with a declared type");
    }
    let workflow = policy.workflow_for_issue_type(to)?;
    let draft_status = if workflow
        .transitions
        .values()
        .any(|t| t.from.iter().any(|s| s == "backlog"))
    {
        "backlog"
    } else {
        workflow.initial_status.as_str()
    };

    let mut plans = Vec::new();
    let issues_dir = layout.canonical_dir().join("issues");
    let mut paths = fs::read_dir(&issues_dir)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    paths.sort();
    for path in paths {
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        if let Some(plan) = plan_epic(&path, &text, to, draft_status)? {
            plans.push(plan);
        }
    }

    for plan in &plans {
        let change = plan
            .status_change
            .as_ref()
            .map(|(from, to)| format!(", status {from} -> {to}"))
            .unwrap_or_default();
        println!("{}: epic -> {to}{change}", plan.id);
    }
    let policy_changed = new_policy != policy_text;
    if policy_changed {
        println!(
            "{}: remove the epic issue type{}",
            atelier_app::workflow_policy::WORKFLOW_POLICY_PATH,
            if close_check_added {
                ", and add work.terminal to the work close so a parent closes after its children"
            } else {
                ""
            }
        );
    }
    if !close_has_check {
        println!(
            "Note: the {to} workflow's close transition has no work.terminal check, so a parent could close before its children. Add `- work.terminal` to its validators in {}.",
            atelier_app::workflow_policy::WORKFLOW_POLICY_PATH
        );
    }
    if plans.is_empty() && !policy_changed {
        println!("No epic records or epic policy entries to migrate.");
        return Ok(());
    }
    if !apply {
        println!(
            "Dry run: rerun with --apply to write these changes, then review and commit them."
        );
        return Ok(());
    }
    for plan in &plans {
        fs::write(&plan.path, &plan.text)
            .with_context(|| format!("cannot write {}", plan.path.display()))?;
    }
    if policy_changed {
        fs::write(&policy_path, new_policy)?;
    }
    println!(
        "Migrated {} epic record(s). Run atelier check, review the Done when sections, and commit.",
        plans.len()
    );
    Ok(())
}

/// The rewritten record for an epic, or `None` for any other record.
fn plan_epic(path: &Path, text: &str, to: &str, draft_status: &str) -> Result<Option<Plan>> {
    let Some((front_matter, body)) = atelier_records::split_document(text) else {
        return Ok(None);
    };
    let is_epic = front_matter
        .lines()
        .any(|line| matches!(line.trim(), "issue_type: \"epic\"" | "issue_type: epic"));
    if !is_epic {
        return Ok(None);
    }
    let mut status_change = None;
    let front_matter = front_matter
        .lines()
        .map(|line| match line.trim() {
            "issue_type: \"epic\"" | "issue_type: epic" => format!("issue_type: \"{to}\""),
            "status: \"draft\"" | "status: draft" => {
                status_change = Some(("draft".to_string(), draft_status.to_string()));
                format!("status: \"{draft_status}\"")
            }
            _ => line.to_string(),
        })
        .collect::<Vec<_>>()
        .join("\n");

    let sections = split_sections(body);
    let section = |name: &str| {
        sections
            .iter()
            .find(|(heading, _)| heading == name)
            .map(|(_, content)| content.trim().to_string())
            .filter(|content| !content.is_empty())
    };
    let goal = section("Outcome")
        .or_else(|| section("Goal"))
        .unwrap_or_else(|| "Outcome was not specified.".to_string());
    let done_when = match (section("Proof"), section("Delivers")) {
        (Some(proof), Some(delivers)) => format!("{proof}\n\nDelivers:\n\n{delivers}"),
        (Some(proof), None) => proof,
        (None, Some(delivers)) => format!("Every child issue is done, delivering:\n\n{delivers}"),
        (None, None) => section("Done when").unwrap_or_else(|| {
            "Every child issue is done. (Migrated from an epic with no Proof route; refine this before starting.)".to_string()
        }),
    };
    let mut new_body = format!("## Goal\n\n{goal}\n\n## Done when\n\n{done_when}\n");
    for name in [
        "Scope",
        "Context",
        "Unknowns",
        "Design",
        "Notes",
        "Close Reason",
    ] {
        if let Some(content) = section(name) {
            new_body.push_str(&format!("\n## {name}\n\n{content}\n"));
        }
    }
    for (heading, _) in &sections {
        if !matches!(
            heading.as_str(),
            "Outcome"
                | "Goal"
                | "Proof"
                | "Delivers"
                | "Done when"
                | "Scope"
                | "Context"
                | "Unknowns"
                | "Design"
                | "Notes"
                | "Close Reason"
        ) {
            bail!(
                "{} has an unexpected section '{heading}'; migrate it by hand",
                path.display()
            );
        }
    }
    let text = format!("---\n{front_matter}\n---\n\n{new_body}");
    let relative = Path::new("issues").join(path.file_name().context("record file name")?);
    let record = atelier_records::parse_issue_record(&text, &relative)
        .with_context(|| format!("migrated {} would not be a valid record", path.display()))?;
    Ok(Some(Plan {
        path: path.to_path_buf(),
        id: record.issue.id,
        text,
        status_change,
    }))
}

/// Level-two sections in order, as `(heading, content)`.
fn split_sections(body: &str) -> Vec<(String, String)> {
    let mut sections: Vec<(String, String)> = Vec::new();
    for line in body.lines() {
        let heading = line
            .strip_prefix("## ")
            .filter(|_| !line.starts_with("###"))
            .map(str::trim);
        match heading {
            Some(heading) => sections.push((heading.to_string(), String::new())),
            None => {
                if let Some((_, content)) = sections.last_mut() {
                    content.push_str(line);
                    content.push('\n');
                }
            }
        }
    }
    sections
}
