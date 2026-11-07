//! One-shot record migrations: retired epic records to parent issues
//! (atelier-9vq3), separate disposition events into their findings
//! (ADR 0025), and After merge lines into loose ends (atelier-8bh4).
//!
//! Epic records no longer load, so that migration reads the raw Markdown
//! instead of loading the record index; separate disposition events still
//! load, folded into their findings, and that migration writes the fold.
//! Each plans every rewrite and writes nothing unless all of them are valid.
use anyhow::{bail, Context, Result};
use std::{fs, path::Path};

use atelier_app::storage_layout::StorageLayout;
use atelier_records::CanonicalIssueRecord;

const POLICY_EPIC_TYPE: &str = "  epic:\n    label: Epic\n";
const POLICY_EPIC_APPLIES: &str = "    - epic\n";

struct Plan {
    path: std::path::PathBuf,
    id: String,
    text: String,
    status_change: Option<(String, String)>,
}

/// Quiet output is the IDs of the epic records retyped, or that a dry run
/// would retype, one per line.
pub fn epics(to: &str, apply: bool, quiet: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    let policy_path = repo.join(atelier_app::workflow_policy::WORKFLOW_POLICY_PATH);
    let policy_text = fs::read_to_string(&policy_path)
        .with_context(|| format!("cannot read {}", policy_path.display()))?;
    let new_policy = policy_text
        .replace(POLICY_EPIC_TYPE, "")
        .replace(POLICY_EPIC_APPLIES, "");
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
            "{}: remove the epic issue type",
            atelier_app::workflow_policy::WORKFLOW_POLICY_PATH,
        );
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
    // The retyped records pass the rules `atelier check` applies before any
    // is written (atelier-ilba).
    let changes = plans
        .iter()
        .map(|plan| atelier_records::guard::Change {
            path: Path::new("issues").join(plan.path.file_name().unwrap_or_default()),
            text: Some(plan.text.clone()),
        })
        .collect::<Vec<_>>();
    atelier_app::record_guard::check_changes(&layout.canonical_dir(), &changes)?;
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

/// Writes the fold Atelier reads separate disposition events as (ADR 0025):
/// each finding with events takes its latest one as its disposition field,
/// naming the event, and every disposition event is removed, superseded ones
/// included. The gates read only the latest, and Git history keeps them all.
/// A finding migrated already takes the events that continue from its
/// folded one. Optional cleanup: the result reads the same before and after.
/// Quiet output is the IDs of the findings that take a disposition, one per
/// line.
pub fn dispositions(apply: bool, quiet: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let files = atelier_app::record_index::read_record_files(&layout.canonical_dir())?;
    if let Some(first) = files.errors.into_iter().next() {
        return Err(first.error);
    }
    let mut plans = Vec::new();
    for finding in &files.reviews {
        let events = files
            .retired_dispositions
            .iter()
            .filter(|event| event.finding == finding.header.id)
            .collect::<Vec<_>>();
        if events.is_empty() {
            continue;
        }
        let disposition = finding
            .disposition()
            .context("a finding with disposition events reads as disposed")?;
        plans.push((finding.header.id.clone(), disposition.clone(), events));
    }
    for (finding, folded, events) in &plans {
        if quiet {
            println!("{finding}");
            continue;
        }
        let dropped = match events.len() - 1 {
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
    // Every rewrite is checked together, and none is written unless the
    // records they leave pass (atelier-ilba).
    store.batch(|| {
        for (finding, folded, events) in &plans {
            store.set_disposition(finding, folded.clone())?;
            for event in events {
                store.remove_retired_disposition(&event.id)?;
            }
        }
        Ok(())
    })?;
    if !quiet {
        println!(
            "Migrated {}. Run atelier check and commit the changes.",
            crate::human_output::count_noun(plans.len(), "finding")
        );
    }
    Ok(())
}

/// One open After merge line and the loose end it becomes.
struct LooseEndPlan<'r> {
    item: &'r CanonicalIssueRecord,
    line: atelier_app::after_merge::MarkedLine,
    needs_human: bool,
    /// The loose end a run of this migration already made for the line.
    existing: Option<String>,
}

/// Turns each unticked After merge line (ADR 0027) into a loose end
/// `blocked_by` its item (ADR 0028, decision 3): the check becomes the loose
/// end's Done when, the reason goes in its Goal, and the human's judgment or
/// a device or place no agent can reach labels it needs-human. On an item
/// that is not done the line's stand-in stays as an ordinary Done when line;
/// a done item is left as it is. A line carried to an existing issue, or
/// one an earlier run already turned into a loose end, is skipped, so the
/// migration runs any number of times. Quiet output is the IDs of the loose
/// ends made, or of the items whose lines a dry run would migrate.
pub fn after_merge(
    index: &atelier_app::record_index::RecordIndex,
    apply: bool,
    quiet: bool,
) -> Result<()> {
    use atelier_app::after_merge;
    use atelier_app::workflow_policy::WORKFLOW_POLICY_PATH;
    use atelier_workflow::LOOSE_END_TYPE;

    let layout = StorageLayout::discover()?;
    let policy = atelier_app::workflow_policy::load(layout.repo_root())?;
    if !policy.issue_types.contains_key(LOOSE_END_TYPE) {
        bail!(
            "migrate_no_loose_end: {WORKFLOW_POLICY_PATH} declares no `{LOOSE_END_TYPE}` issue type, so there is nothing to turn After merge lines into. Next: add `{LOOSE_END_TYPE}` to its issue_types with a workflow that has no `start` transition, as the policy `atelier init` writes does (its `on_base` workflow), then rerun `atelier migrate after-merge`"
        );
    }
    let initial_status = policy
        .workflow_for_issue_type(LOOSE_END_TYPE)?
        .initial_status
        .clone();
    let done =
        |item: &CanonicalIssueRecord| policy.status_category(&item.issue.status) == Some("done");
    let plans = after_merge::open_lines(index, done)
        .into_iter()
        .map(|open| LooseEndPlan {
            item: open.item,
            needs_human: open
                .line
                .reason
                .as_deref()
                .is_some_and(after_merge::reason_needs_human),
            existing: open.existing,
            line: open.line,
        })
        .collect::<Vec<_>>();
    if plans.is_empty() {
        if !quiet {
            println!("No open After merge lines to migrate.");
        }
        return Ok(());
    }
    for plan in &plans {
        let item = &plan.item.issue.id;
        if quiet {
            if !apply {
                println!("{item}");
            }
            continue;
        }
        let reason = match &plan.line.reason {
            Some(reason) => format!("reason: {reason}"),
            None => "no reason given".to_owned(),
        };
        let made = match &plan.existing {
            Some(id) => format!("already loose end {id}"),
            None => format!(
                "a loose end blocked_by {item}, {} ({reason})",
                if plan.needs_human {
                    "labelled needs-human"
                } else {
                    "not labelled needs-human"
                }
            ),
        };
        let rest = if done(plan.item) {
            format!("; {item} is done, so its record is unchanged")
        } else {
            match &plan.line.stand_in {
                Some(stand_in) => {
                    format!("; its stand-in stays on {item} as a Done when line: {stand_in}")
                }
                None => format!("; the line leaves {item}, which gave no stand-in"),
            }
        };
        println!("- {item}: \"{}\" becomes {made}{rest}", plan.line.check);
    }
    if !apply {
        if !quiet {
            println!(
                "Dry run: rerun with --apply to write these changes, then review and commit them."
            );
        }
        return Ok(());
    }
    let edits = plans
        .iter()
        .filter(|plan| !done(plan.item))
        .map(|plan| plan.item.issue.id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let now = chrono::Utc::now();
    let mut items = Vec::new();
    for id in &edits {
        let mut record = index
            .issue_record(id)
            .cloned()
            .with_context(|| format!("{id} is not an issue"))?;
        if let atelier_records::IssueBody::Issue { done_when, .. } = &mut record.sections.body {
            *done_when = done_when
                .lines()
                .filter_map(|line| match after_merge::parse(line) {
                    Some(marked)
                        if plans
                            .iter()
                            .any(|plan| plan.item.issue.id == *id && plan.line == marked) =>
                    {
                        marked
                            .stand_in
                            .as_ref()
                            .map(|stand_in| format!("{}{stand_in}", marked.prefix))
                    }
                    _ => Some(line.to_owned()),
                })
                .collect::<Vec<_>>()
                .join("\n");
        }
        record.issue.updated_at = now;
        items.push(record);
    }
    let store = atelier_app::project_config::record_store(&layout.canonical_dir())?;
    // The loose ends and the items they leave are checked as one write, so
    // none is written unless all of them pass (atelier-ilba).
    let made = store.batch(|| {
        let mut made = Vec::new();
        for plan in plans.iter().filter(|plan| plan.existing.is_none()) {
            let id = store.allocate_issue_id()?;
            let record = loose_end(&id, plan, &initial_status, now);
            store.write_issue_atomic(&record)?;
            made.push(id);
        }
        for record in &items {
            store.write_issue_atomic(record)?;
        }
        Ok(made)
    })?;
    if quiet {
        for id in &made {
            println!("{id}");
        }
        return Ok(());
    }
    println!(
        "Made {} and updated {}. Run atelier check, review the new records, and commit them.",
        crate::human_output::count_noun(made.len(), "loose end"),
        crate::human_output::count_noun(items.len(), "item")
    );
    Ok(())
}

/// The loose end `plan`'s line becomes.
fn loose_end(
    id: &str,
    plan: &LooseEndPlan<'_>,
    status: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> CanonicalIssueRecord {
    let item = &plan.item.issue;
    let check = plan.line.check.trim();
    let mut title = check.trim_end_matches(['.', ';', ',']).to_owned();
    if let Some(first) = title.chars().next() {
        title.replace_range(..first.len_utf8(), &first.to_uppercase().to_string());
    }
    if title.chars().count() > TITLE_LIMIT {
        let cut = title
            .char_indices()
            .nth(TITLE_LIMIT)
            .map_or(title.len(), |(at, _)| at);
        let cut = title[..cut].rfind(' ').unwrap_or(cut);
        title = format!("{}…", title[..cut].trim_end());
    }
    let why = match &plan.line.reason {
        Some(reason) => format!("It waits for the merge because of: {reason}."),
        None => "Its After merge line gave no reason for waiting.".to_owned(),
    };
    let labels = if plan.needs_human {
        vec![atelier_app::work_classification::NEEDS_HUMAN_LABEL.to_owned()]
    } else {
        Vec::new()
    };
    CanonicalIssueRecord {
        issue: atelier_core::Issue {
            id: id.to_owned(),
            title,
            description: None,
            status: status.to_owned(),
            issue_type: atelier_workflow::LOOSE_END_TYPE.to_owned(),
            priority: item.priority.clone(),
            strategy: None,
            adr: None,
            workflow_branch: None,
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
        },
        labels,
        sections: atelier_records::IssueSections {
            body: atelier_records::IssueBody::Issue {
                goal: format!(
                    "Run, once {} ({}) is merged, the check its After merge line held; `atelier migrate after-merge` moved it here. {why}",
                    item.id, item.title
                ),
                done_when: format!("- [ ] {check}"),
                scope: None,
                context: None,
                unknowns: None,
                design: None,
                background: None,
            },
            notes: None,
            report: None,
        },
        relationships: atelier_records::Relationships {
            relates: vec![atelier_records::relates_relationship(
                "issue",
                &item.id,
                atelier_records::BLOCKED_BY,
            )],
            ..atelier_records::Relationships::default()
        },
    }
}

/// Longest loose-end title the migration writes, in characters.
const TITLE_LIMIT: usize = 100;
