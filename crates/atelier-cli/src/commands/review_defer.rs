//! `atelier review defer`: defers every open finding on an item that was
//! recommended defer and has no decision yet, in one step (atelier-stnv).
//! Each finding gets a backlog follow-up discovered from it, under one
//! parent per item, and a defer disposition naming that follow-up, so
//! deferring costs no more than fixing.
use crate::human_output::{count_noun, short_commit};
use anyhow::{bail, Result};
use atelier_app::record_index::RecordIndex;
use atelier_app::record_ref::RecordKind;
use atelier_app::workflow_policy::WorkflowPolicy;
use atelier_app::{git_freshness as git, review_gate, storage_layout::StorageLayout};
use atelier_core::{Disposition, FindingDecision, Issue, ReviewEvent, ReviewRecord};
use atelier_records::{
    issue_relationship_target, relates_relationship, CanonicalIssueRecord, IssueSections,
    Relationships, DISCOVERED_FROM,
};
use chrono::Utc;

/// The label that marks the parent of an item's deferred follow-ups; with
/// its `discovered_from` link to the item, it is how a later run finds the
/// parent again.
pub const DEFERRED_LABEL: &str = "deferred-findings";
/// The work type of the parent and of each follow-up.
const ISSUE_TYPE: &str = "task";
/// The status the parent and each follow-up are created in.
const STATUS: &str = "backlog";
/// The longest title a follow-up takes from its finding's first line.
const TITLE_LIMIT: usize = 120;

/// One finding to defer, and the title its follow-up gets.
struct Planned<'a> {
    finding: &'a ReviewRecord,
    title: String,
    priority: &'static str,
}

pub fn run(reference: &str, identity: &str, dry_run: bool, quiet: bool) -> Result<()> {
    if identity.trim().is_empty() {
        bail!("review defer requires an explicit --identity, recorded with each disposition");
    }
    let layout = StorageLayout::discover()?;
    let repo = layout.repo_root();
    let state_dir = layout.canonical_dir();
    let index = RecordIndex::load_structural(&state_dir)?;
    let item = index.resolve(RecordKind::Issue, reference)?;
    let policy = atelier_app::workflow_policy::load(repo)?;
    if !policy.issue_types.contains_key(ISSUE_TYPE)
        || !atelier_app::workflow_policy::creatable_statuses(&policy, ISSUE_TYPE)?.contains(STATUS)
    {
        bail!(
            "review_defer_policy: the workflow policy has no {ISSUE_TYPE} type created in {STATUS}, which the follow-ups use. Next: defer each finding with `atelier review disposition <finding> --decision defer --follow-up <issue>`"
        );
    }
    let scope = review_gate::scope(&index, &item)?;
    let mut planned = index
        .reviews()
        .filter(|record| scope.contains(&record.target))
        .filter(|record| {
            matches!(
                record.event,
                ReviewEvent::Finding {
                    recommendation: FindingDecision::Defer,
                    disposition: None,
                }
            )
        })
        .map(|finding| {
            let (priority, title) = follow_up_title(&finding.summary);
            Planned {
                finding,
                title,
                priority,
            }
        })
        .collect::<Vec<_>>();
    planned.sort_by_key(|plan| {
        (
            plan.finding.header.created_at,
            plan.finding.header.id.clone(),
        )
    });
    let parent = existing_parent(&index, &policy, &item);
    if planned.is_empty() {
        if !quiet {
            println!(
                "No finding on {item} is recommended defer without a decision; nothing to defer."
            );
        }
        return Ok(());
    }

    if dry_run {
        if quiet {
            for plan in &planned {
                println!("{}", plan.finding.header.id);
            }
            return Ok(());
        }
        let under = match &parent {
            Some(parent) => format!("under {parent}"),
            None => format!("under a new {STATUS} parent \"{}\"", parent_title(&item)),
        };
        println!(
            "Would defer {} on {item} {under}:",
            count_noun(planned.len(), "finding")
        );
        for plan in &planned {
            println!(
                "  {} → new {STATUS} follow-up \"{}\"",
                plan.finding.header.id, plan.title
            );
        }
        println!("Dry run: nothing was written.");
        return Ok(());
    }

    let pin = git::head_commit(repo)?;
    let store = atelier_app::project_config::record_store(&state_dir)?;
    let now = Utc::now();
    let (parent_id, created_parent) = match &parent {
        Some(parent) => (parent.clone(), false),
        None => (store.allocate_issue_id()?, true),
    };
    let mut filed = Vec::new();
    store.batch(|| {
        let mut children = Vec::new();
        for plan in &planned {
            let finding = &plan.finding.header.id;
            let id = store.allocate_issue_id()?;
            let body = format!(
                "Deferred from finding {finding} on {} @{}, by `atelier review defer`.\n\n{}",
                plan.finding.target,
                short_commit(&plan.finding.commit),
                plan.finding.summary.trim()
            );
            store.write_issue_atomic(&new_issue(
                &id,
                &plan.title,
                plan.priority,
                &body,
                Vec::new(),
                relates_relationship("review", finding, DISCOVERED_FROM),
                now,
            ))?;
            children.push(id.clone());
            let disposition = Disposition {
                decision: FindingDecision::Defer,
                identity: identity.to_owned(),
                commit: pin.clone(),
                recorded_at: now,
                resolution_commit: None,
                follow_up: Some(id.clone()),
                reason: format!("Deferred to {STATUS} follow-up {id} under {parent_id}."),
                migrated_from: None,
            };
            atelier_records::review::validate_disposition(&disposition)?;
            store.set_disposition(finding, disposition)?;
            filed.push((finding.clone(), id, plan.title.clone()));
        }
        if created_parent {
            let body = format!(
                "Collect the follow-ups deferred from review findings on {item}, one per finding, filed by `atelier review defer {item}`."
            );
            let mut record = new_issue(
                &parent_id,
                &parent_title(&item),
                "medium",
                &body,
                vec![DEFERRED_LABEL.to_owned()],
                relates_relationship("issue", &item, DISCOVERED_FROM),
                now,
            );
            record.relationships.children = children
                .iter()
                .map(|child| issue_relationship_target(child))
                .collect();
            store.write_issue_atomic(&record)?;
        } else {
            for child in &children {
                store.add_issue_child(&parent_id, child)?;
            }
        }
        Ok(())
    })?;

    if quiet {
        for (_, follow_up, _) in &filed {
            println!("{follow_up}");
        }
        return Ok(());
    }
    println!(
        "Deferred {} on {item} under {parent_id}{} @{}:",
        count_noun(filed.len(), "finding"),
        if created_parent { " (new)" } else { "" },
        short_commit(&pin)
    );
    for (finding, follow_up, title) in &filed {
        println!("  {finding} → {follow_up} \"{title}\"");
    }
    Ok(())
}

/// The open parent an earlier run filed for `item`: an issue labelled
/// [`DEFERRED_LABEL`], discovered from the item, not yet in a done status.
/// A parent someone closed is not reused, so new follow-ups never land
/// under finished work.
fn existing_parent(index: &RecordIndex, policy: &WorkflowPolicy, item: &str) -> Option<String> {
    index
        .issue_records()
        .filter(|record| record.labels.iter().any(|label| label == DEFERRED_LABEL))
        .filter(|record| {
            record.relationships.relates.iter().any(|relation| {
                relation.relation_type == DISCOVERED_FROM
                    && relation.kind == "issue"
                    && relation.id == item
            })
        })
        .filter(|record| policy.status_category(&record.issue.status) != Some("done"))
        .min_by_key(|record| (record.issue.created_at, record.issue.id.clone()))
        .map(|record| record.issue.id.clone())
}

fn parent_title(item: &str) -> String {
    format!("Deferred review findings on {item}")
}

/// A follow-up's priority and title from its finding's first line: a
/// leading `critical:`, `high:`, `medium:` or `low:` sets the priority and is
/// dropped, and the rest is cut to [`TITLE_LIMIT`] characters.
fn follow_up_title(summary: &str) -> (&'static str, String) {
    let line = summary.lines().next().unwrap_or_default().trim();
    let (priority, rest) = ["critical", "high", "medium", "low"]
        .into_iter()
        .find_map(|priority| {
            line.strip_prefix(priority)
                .and_then(|rest| rest.strip_prefix(':'))
                .map(|rest| (priority, rest.trim()))
        })
        .unwrap_or(("medium", line));
    let rest = if rest.is_empty() {
        "Deferred review finding"
    } else {
        rest
    };
    let title = if rest.chars().count() > TITLE_LIMIT {
        let cut = rest.chars().take(TITLE_LIMIT - 1).collect::<String>();
        format!("{}…", cut.trim_end())
    } else {
        rest.to_owned()
    };
    (priority, title)
}

fn new_issue(
    id: &str,
    title: &str,
    priority: &str,
    body: &str,
    labels: Vec<String>,
    discovered_from: atelier_records::RelatesRelationship,
    now: chrono::DateTime<Utc>,
) -> CanonicalIssueRecord {
    CanonicalIssueRecord {
        issue: Issue {
            id: id.to_owned(),
            title: title.to_owned(),
            description: Some(body.to_owned()),
            status: STATUS.to_owned(),
            issue_type: ISSUE_TYPE.to_owned(),
            priority: priority.to_owned(),
            strategy: None,
            adr: None,
            workflow_branch: None,
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
        },
        labels,
        sections: IssueSections::for_type(ISSUE_TYPE, Some(body)),
        relationships: Relationships {
            relates: vec![discovered_from],
            ..Relationships::default()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_title_takes_the_severity_as_priority_and_is_cut_to_the_limit() {
        assert_eq!(
            follow_up_title("high: src/a.rs:3 leaks a handle\n\nmore"),
            ("high", "src/a.rs:3 leaks a handle".to_owned())
        );
        assert_eq!(
            follow_up_title("no severity here"),
            ("medium", "no severity here".to_owned())
        );
        let (_, long) = follow_up_title(&format!("low: {}", "x".repeat(300)));
        assert_eq!(long.chars().count(), TITLE_LIMIT);
        assert!(long.ends_with('…'));
    }
}
