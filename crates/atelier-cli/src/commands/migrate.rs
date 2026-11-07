//! One-shot record migrations: retired epic records to parent issues
//! (atelier-9vq3), and separate disposition events into their findings
//! (ADR 0025).
//!
//! The old shapes no longer load, so each migration reads the raw Markdown
//! instead of loading the record index, plans every rewrite, validates each
//! result with the record parser, and writes nothing unless all of them are
//! valid.
use anyhow::{bail, Context, Result};
use atelier_records::review::{parse_review_file, RetiredDisposition, ReviewFile};
use std::collections::BTreeMap;
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

/// Quiet output is the IDs of the epic records retyped, or that a dry run
/// would retype, one per line; the close-check note goes to stderr.
pub fn epics(to: &str, apply: bool, quiet: bool) -> Result<()> {
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
        if quiet {
            println!("{}", plan.id);
            continue;
        }
        let change = plan
            .status_change
            .as_ref()
            .map(|(from, to)| format!(", status {from} -> {to}"))
            .unwrap_or_default();
        println!("{}: epic -> {to}{change}", plan.id);
    }
    let policy_changed = new_policy != policy_text;
    if policy_changed && !quiet {
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
        let note = format!(
            "Note: the {to} workflow's close transition has no work.terminal check, so a parent could close before its children. Add `- work.terminal` to its validators in {}.",
            atelier_app::workflow_policy::WORKFLOW_POLICY_PATH
        );
        if quiet {
            eprintln!("{note}");
        } else {
            println!("{note}");
        }
    }
    if plans.is_empty() && !policy_changed {
        if !quiet {
            println!("No epic records or epic policy entries to migrate.");
        }
        return Ok(());
    }
    if !apply {
        if quiet {
            return Ok(());
        }
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
    if quiet {
        return Ok(());
    }
    println!(
        "Migrated {}. Run atelier check, review the Done when sections, and commit.",
        crate::human_output::count_noun(plans.len(), "epic record")
    );
    Ok(())
}

/// The rewritten record for an epic, or `None` for any other record.
fn plan_epic(path: &Path, text: &str, to: &str, draft_status: &str) -> Result<Option<Plan>> {
    let Some(migrated) = atelier_app::epic_migration::migrate_epic_text(text, to, draft_status)
        .with_context(|| format!("{}", path.display()))?
    else {
        return Ok(None);
    };
    let relative = Path::new("issues").join(path.file_name().context("record file name")?);
    let record = atelier_records::parse_issue_record(&migrated.text, &relative)
        .with_context(|| format!("migrated {} would not be a valid record", path.display()))?;
    Ok(Some(Plan {
        path: path.to_path_buf(),
        id: record.issue.id,
        text: migrated.text,
        status_change: migrated.status_change,
    }))
}

/// Folds each finding's latest disposition event into the finding as its
/// disposition field, naming the event, and removes every disposition event
/// (ADR 0025). Superseded events are dropped with the latest: the gates read
/// only the latest, and Git history keeps them all. Quiet output is the IDs
/// of the findings that take a disposition, one per line.
pub fn dispositions(apply: bool, quiet: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let dir = layout.canonical_dir().join("reviews");
    let mut findings = BTreeMap::new();
    let mut events: BTreeMap<String, Vec<(std::path::PathBuf, RetiredDisposition)>> =
        BTreeMap::new();
    let mut paths = match fs::read_dir(&dir) {
        Ok(entries) => entries
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<std::io::Result<Vec<_>>>()?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(error) => return Err(error).context(format!("cannot read {}", dir.display())),
    };
    paths.sort();
    for path in paths {
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let relative = Path::new("reviews").join(path.file_name().context("review file name")?);
        match parse_review_file(&fs::read_to_string(&path)?, &relative)? {
            ReviewFile::RetiredDisposition(event) => events
                .entry(event.finding.clone())
                .or_default()
                .push((path, event)),
            ReviewFile::Event(record) => {
                findings.insert(record.header.id.clone(), record);
            }
            ReviewFile::BaseExemption(_) => {}
        }
    }
    let mut plans = Vec::new();
    for (finding, chain) in &events {
        let record = findings.get(finding).with_context(|| {
            format!("disposition events of {finding} name a finding that does not exist; restore it, or remove those events by hand")
        })?;
        if record.disposition().is_some()
            || !matches!(record.event, atelier_core::ReviewEvent::Finding { .. })
        {
            bail!("{finding} is not a finding without a disposition, so its disposition events cannot be folded into it");
        }
        // The latest is the one no other event supersedes.
        let latest = chain
            .iter()
            .filter(|(_, event)| {
                !chain
                    .iter()
                    .any(|(_, other)| other.supersedes.as_deref() == Some(event.id.as_str()))
            })
            .collect::<Vec<_>>();
        let [(_, latest)] = latest.as_slice() else {
            bail!("finding {finding} has competing latest disposition events; run atelier check");
        };
        let folded = latest.folded();
        atelier_records::review::validate_disposition(&folded)
            .with_context(|| format!("disposition event {}", latest.id))?;
        plans.push((finding.clone(), folded, chain));
    }
    for (finding, folded, chain) in &plans {
        if quiet {
            println!("{finding}");
            continue;
        }
        let dropped = match chain.len() - 1 {
            0 => String::new(),
            n => format!(
                ", removing {} it superseded",
                crate::human_output::count_noun(n, "earlier event")
            ),
        };
        println!(
            "{finding}: {} by {} from {}{dropped}",
            folded.decision.as_str(),
            folded.identity,
            folded.migrated_from.as_deref().unwrap_or_default()
        );
    }
    if plans.is_empty() {
        if !quiet {
            println!("No disposition events to migrate.");
        }
        return Ok(());
    }
    if !apply {
        if !quiet {
            println!(
                "Dry run: rerun with --apply to write these changes, then review and commit them."
            );
        }
        return Ok(());
    }
    let store = atelier_app::project_config::record_store(&layout.canonical_dir())?;
    for (finding, folded, chain) in plans.iter() {
        store.set_disposition(finding, folded.clone())?;
        for (path, _) in chain.iter() {
            fs::remove_file(path).with_context(|| format!("cannot remove {}", path.display()))?;
        }
    }
    if !quiet {
        println!(
            "Migrated {}. Run atelier check and commit the changes.",
            crate::human_output::count_noun(plans.len(), "finding")
        );
    }
    Ok(())
}
