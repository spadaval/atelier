use anyhow::{bail, Result};
use serde::Serialize;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use crate::db::Database;
use crate::record_store::{IssueSections, RecordStore};

const SLOW_VALIDATOR_WARNING_MS: u128 = 100;

#[derive(Debug, Serialize)]
pub struct ValidatorResult {
    pub target_kind: String,
    pub target_id: String,
    pub transition: String,
    pub validator: String,
    pub passed: bool,
    pub reason: String,
    pub elapsed_ms: u128,
}

pub fn validate(
    db: &Database,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validators: Vec<String>,
) -> Result<()> {
    let results = evaluate(db, target_kind, target_id, transition, validators)?;
    print_validation_results(&results);
    let failures = results
        .iter()
        .filter(|result| !result.passed)
        .map(|result| result.validator.as_str())
        .collect::<Vec<_>>();
    if !failures.is_empty() {
        bail!("workflow validation failed: {}", failures.join(", "));
    }
    Ok(())
}

pub fn evaluate(
    db: &Database,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validators: Vec<String>,
) -> Result<Vec<ValidatorResult>> {
    ensure_target_exists(db, target_kind, target_id)?;
    let validators = if validators.is_empty() {
        default_validators(target_kind, transition)
    } else {
        validators
    };
    let mut results = Vec::new();
    for validator in validators {
        let started = Instant::now();
        let (passed, reason) = evaluate_builtin(db, target_kind, target_id, &validator)?;
        let elapsed_ms = started.elapsed().as_millis();
        results.push(ValidatorResult {
            target_kind: target_kind.to_string(),
            target_id: target_id.to_string(),
            transition: transition.to_string(),
            validator,
            passed,
            reason,
            elapsed_ms,
        });
    }

    Ok(results)
}

pub fn default_validators(target_kind: &str, transition: &str) -> Vec<String> {
    let names: &[&str] = match (target_kind, transition) {
        ("issue", "start") => &[
            "durable_state_current",
            "issue_sections_parseable",
            "no_open_blockers",
        ],
        ("issue", "close") => &[
            "durable_state_current",
            "issue_sections_parseable",
            "no_open_blockers",
            "evidence_attached",
        ],
        ("mission", "close") => &[
            "durable_state_current",
            "issue_sections_parseable",
            "no_open_work",
            "evidence_attached",
            "no_open_blockers",
            "no_blocking_lints",
            "command_surface_current",
            "ignored_tests_reviewed",
            "git_worktree_clean",
        ],
        ("mission", _) => &[
            "durable_state_current",
            "issue_sections_parseable",
            "no_open_blockers",
        ],
        ("evidence", _) => &["durable_state_current"],
        ("tracker", "health") => &[
            "durable_state_current",
            "no_blocking_lints",
            "command_surface_current",
            "ignored_tests_reviewed",
            "git_worktree_clean",
        ],
        _ => &["durable_state_current"],
    };
    names.iter().map(|name| (*name).to_string()).collect()
}

pub fn print_validation_results(results: &[ValidatorResult]) {
    if let Some(first) = results.first() {
        println!(
            "Workflow Validation: {} {}",
            first.target_kind, first.target_id
        );
        println!(
            "{}",
            "=".repeat(first.target_kind.len() + first.target_id.len() + 21)
        );
        println!("Transition: {}", first.transition);
        println!("Validators: {}", results.len());
    } else {
        print_heading("Workflow Validation");
        println!("Validators: 0");
    }
    print_heading("Results");
    if results.is_empty() {
        println!("(none)");
        return;
    }
    for result in results {
        let status = if result.passed { "pass" } else { "fail" };
        println!("  {}  {}", status, result.validator);
        println!("      Reason: {}", result.reason);
        if let Some(warning) = slow_validator_warning(result) {
            println!("      Warning: {warning}");
        }
    }
}

fn slow_validator_warning(result: &ValidatorResult) -> Option<String> {
    if result.elapsed_ms > SLOW_VALIDATOR_WARNING_MS {
        Some(format!(
            "validator took {}ms; validators should stay under {}ms",
            result.elapsed_ms, SLOW_VALIDATOR_WARNING_MS
        ))
    } else {
        None
    }
}

fn print_heading(title: &str) {
    println!("{title}");
    println!("{}", "-".repeat(title.len()));
}

fn ensure_target_exists(db: &Database, kind: &str, id: &str) -> Result<()> {
    match kind {
        "issue" => {
            db.require_issue(id)?;
        }
        "tracker" => {
            if id != "health" {
                bail!("unsupported tracker validation target {id}; expected health");
            }
        }
        _ => {
            db.require_record(kind, id)?;
        }
    }
    Ok(())
}

fn evaluate_builtin(
    db: &Database,
    target_kind: &str,
    target_id: &str,
    validator: &str,
) -> Result<(bool, String)> {
    match validator {
        "durable_state_current" => {
            let state_dir = crate::storage_layout::StorageLayout::new(repo_root()?).canonical_dir();
            let stale = crate::commands::export::canonical_stale_entries(db, &state_dir)?;
            if stale.is_empty() {
                Ok((true, "canonical export is current".to_string()))
            } else {
                Ok((
                    false,
                    format!("canonical export is stale: {}", stale.join("; ")),
                ))
            }
        }
        "evidence_attached" => {
            let attached = db
                .list_record_links(target_kind, target_id)?
                .into_iter()
                .any(|link| {
                    link.relation_type == "validates"
                        && (link.source_kind == "evidence" || link.target_kind == "evidence")
                });
            if attached {
                Ok((true, "validating evidence is linked".to_string()))
            } else {
                Ok((false, "no validating evidence link found".to_string()))
            }
        }
        "no_open_blockers" => {
            let open = open_blockers(db, target_kind, target_id)?;
            if open.is_empty() {
                Ok((true, "no open blockers".to_string()))
            } else {
                Ok((false, format!("open blockers: {}", open.join(", "))))
            }
        }
        "no_open_work" => {
            let open = open_work(db, target_kind, target_id)?;
            if open.is_empty() {
                Ok((true, "no open linked work".to_string()))
            } else {
                Ok((false, format!("open linked work: {}", open.join(", "))))
            }
        }
        "git_worktree_clean" => git_worktree_clean(),
        "no_blocking_lints" => {
            let status = Command::new(std::env::current_exe()?)
                .arg("lint")
                .status()?;
            if status.success() {
                Ok((true, "lint passed".to_string()))
            } else {
                Ok((false, "atelier lint failed".to_string()))
            }
        }
        "ignored_tests_reviewed" => ignored_tests_reviewed(),
        "command_surface_current" => command_surface_current(),
        "issue_sections_parseable" => issue_sections_parseable(db, target_kind, target_id),
        "validation_criteria_satisfied" => Ok((
            false,
            "validation criteria records are not implemented in this staged slice".to_string(),
        )),
        "review_complete" => Ok((
            false,
            "review completion evidence is not linked".to_string(),
        )),
        other => Ok((false, format!("unsupported builtin validator: {other}"))),
    }
}

fn issue_sections_parseable(
    db: &Database,
    target_kind: &str,
    target_id: &str,
) -> Result<(bool, String)> {
    let issue_ids = match target_kind {
        "issue" => {
            let mut ids = BTreeSet::new();
            ids.insert(target_id.to_string());
            ids
        }
        "mission" => mission_issue_ids(db, target_id)?,
        _ => {
            return Ok((
                true,
                format!("issue sections do not apply to {target_kind} records"),
            ))
        }
    };
    if issue_ids.is_empty() {
        return Ok((true, "no linked issues require section checks".to_string()));
    }

    let state_dir = crate::storage_layout::StorageLayout::new(repo_root()?).canonical_dir();
    let store = RecordStore::new(&state_dir);
    let mut checked = 0;
    for issue_id in issue_ids {
        let record = match store.load_issue_by_id(&issue_id) {
            Ok(record) => record,
            Err(error) => return Ok((false, error.to_string())),
        };
        let invalid = record
            .sections
            .section_states()
            .into_iter()
            .filter(|state| state.required && (!state.present || state.empty))
            .map(|state| state.name.title().to_string())
            .collect::<Vec<_>>();
        if !invalid.is_empty() {
            let path = state_dir.join("issues").join(format!("{issue_id}.md"));
            return Ok((
                false,
                format!(
                    "issue {issue_id} has invalid sections {} in {}",
                    invalid.join(", "),
                    path.display()
                ),
            ));
        }
        checked += 1;
    }

    Ok((
        true,
        format!(
            "parsed required sections {} are present and non-empty for {checked} issue(s)",
            IssueSections::REQUIRED_NAMES
                .into_iter()
                .map(|name| name.title())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    ))
}

fn open_blockers(db: &Database, target_kind: &str, target_id: &str) -> Result<Vec<String>> {
    let mut blocker_ids = BTreeSet::new();
    match target_kind {
        "issue" => {
            for blocker in db.get_blockers(target_id)? {
                blocker_ids.insert(blocker);
            }
        }
        "mission" => {
            for blocker in mission_direct_blockers(db, target_id)? {
                blocker_ids.insert(blocker);
            }
            for issue_id in mission_issue_ids(db, target_id)? {
                for blocker in db.get_blockers(&issue_id)? {
                    blocker_ids.insert(blocker);
                }
            }
        }
        _ => return Ok(Vec::new()),
    }
    let mut open = blocker_ids
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter(|issue| issue.status == "open")
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    open.sort();
    Ok(open)
}

fn open_work(db: &Database, target_kind: &str, target_id: &str) -> Result<Vec<String>> {
    if target_kind != "mission" {
        return Ok(Vec::new());
    }
    let mut open = mission_issue_ids(db, target_id)?
        .into_iter()
        .filter_map(|id| db.get_issue(&id).ok().flatten())
        .filter(|issue| issue.status != "closed")
        .map(|issue| issue.id)
        .collect::<Vec<_>>();
    open.sort();
    Ok(open)
}

fn mission_direct_blockers(db: &Database, mission_id: &str) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for link in db.list_record_links("mission", mission_id)? {
        if link.relation_type != "blocked_by" {
            continue;
        }
        if link.source_kind == "issue"
            && link.target_kind == "mission"
            && link.target_id == mission_id
        {
            blockers.push(link.source_id);
        } else if link.target_kind == "issue"
            && link.source_kind == "mission"
            && link.source_id == mission_id
        {
            blockers.push(link.target_id);
        }
    }
    Ok(blockers)
}

fn mission_issue_ids(db: &Database, mission_id: &str) -> Result<BTreeSet<String>> {
    let mut issue_ids = BTreeSet::new();
    for link in db.list_record_links("mission", mission_id)? {
        if link.relation_type != "advances" {
            continue;
        }
        let linked_id = if link.source_kind == "issue"
            && link.target_kind == "mission"
            && link.target_id == mission_id
        {
            Some(link.source_id)
        } else if link.target_kind == "issue"
            && link.source_kind == "mission"
            && link.source_id == mission_id
        {
            Some(link.target_id)
        } else {
            None
        };
        if let Some(linked_id) = linked_id {
            collect_issue_and_descendants(db, &linked_id, &mut issue_ids)?;
        }
    }
    Ok(issue_ids)
}

fn collect_issue_and_descendants(
    db: &Database,
    issue_id: &str,
    issue_ids: &mut BTreeSet<String>,
) -> Result<()> {
    if !issue_ids.insert(issue_id.to_string()) {
        return Ok(());
    }
    for child in db.get_subissues(issue_id)? {
        collect_issue_and_descendants(db, &child.id, issue_ids)?;
    }
    Ok(())
}

fn git_worktree_clean() -> Result<(bool, String)> {
    let root = repo_root()?;
    let output = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=all"])
        .current_dir(&root)
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let message = if stderr.is_empty() {
            "git status failed".to_string()
        } else {
            format!("git status failed: {stderr}")
        };
        return Ok((false, message));
    }
    let dirty = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    if dirty.is_empty() {
        Ok((true, "git worktree is clean".to_string()))
    } else {
        let sample = dirty.iter().take(8).cloned().collect::<Vec<_>>().join("; ");
        let suffix = if dirty.len() > 8 {
            format!("; ... and {} more", dirty.len() - 8)
        } else {
            String::new()
        };
        Ok((
            false,
            format!(
                "git worktree has {} dirty {}: {sample}{suffix}",
                dirty.len(),
                if dirty.len() == 1 { "entry" } else { "entries" }
            ),
        ))
    }
}

fn ignored_tests_reviewed() -> Result<(bool, String)> {
    let inventory = crate::test_inventory::IgnoredTestInventory::scan_repo(&repo_root()?)?;
    Ok(inventory.status_reason())
}

fn command_surface_current() -> Result<(bool, String)> {
    crate::command_surface::status_reason(&repo_root()?)
}

fn repo_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;
    if output.status.success() {
        Ok(Path::new(String::from_utf8_lossy(&output.stdout).trim()).to_path_buf())
    } else {
        Ok(std::env::current_dir()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validator_result(elapsed_ms: u128) -> ValidatorResult {
        ValidatorResult {
            target_kind: "issue".to_string(),
            target_id: "atelier-test".to_string(),
            transition: "close".to_string(),
            validator: "durable_state_current".to_string(),
            passed: true,
            reason: "canonical export is current".to_string(),
            elapsed_ms,
        }
    }

    #[test]
    fn slow_validator_warning_starts_after_threshold() {
        assert!(slow_validator_warning(&validator_result(100)).is_none());

        let warning = slow_validator_warning(&validator_result(101)).unwrap();
        assert!(warning.contains("validator took 101ms"));
        assert!(warning.contains("under 100ms"));
    }

    #[test]
    fn default_validators_are_target_and_transition_aware() {
        assert_eq!(
            default_validators("issue", "start"),
            vec![
                "durable_state_current",
                "issue_sections_parseable",
                "no_open_blockers"
            ]
        );
        assert_eq!(
            default_validators("issue", "close"),
            vec![
                "durable_state_current",
                "issue_sections_parseable",
                "no_open_blockers",
                "evidence_attached"
            ]
        );
        assert_eq!(
            default_validators("mission", "close"),
            vec![
                "durable_state_current",
                "issue_sections_parseable",
                "no_open_work",
                "evidence_attached",
                "no_open_blockers",
                "no_blocking_lints",
                "ignored_tests_reviewed",
                "git_worktree_clean"
            ]
        );
        assert_eq!(
            default_validators("evidence", "attach"),
            vec!["durable_state_current"]
        );
        assert_eq!(
            default_validators("tracker", "health"),
            vec![
                "durable_state_current",
                "no_blocking_lints",
                "ignored_tests_reviewed",
                "git_worktree_clean"
            ]
        );
    }
}
