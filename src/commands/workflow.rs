use anyhow::Result;
use serde::Serialize;
use serde_json::json;
use std::path::Path;
use std::process::Command;

use crate::db::Database;

#[derive(Debug, Serialize)]
struct ValidatorResult {
    target_kind: String,
    target_id: String,
    transition: String,
    validator: String,
    passed: bool,
    reason: String,
}

pub fn validate(
    db: &Database,
    target_kind: &str,
    target_id: &str,
    transition: &str,
    validators: Vec<String>,
    json_output: bool,
) -> Result<()> {
    ensure_target_exists(db, target_kind, target_id)?;
    let validators = if validators.is_empty() {
        vec!["durable_state_current".to_string()]
    } else {
        validators
    };
    let mut results = Vec::new();
    for validator in validators {
        let (passed, reason) = evaluate_builtin(db, target_kind, target_id, &validator)?;
        results.push(ValidatorResult {
            target_kind: target_kind.to_string(),
            target_id: target_id.to_string(),
            transition: transition.to_string(),
            validator,
            passed,
            reason,
        });
    }

    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "data": results }))?
        );
    } else {
        print_validation_results(&results);
    }
    Ok(())
}

fn print_validation_results(results: &[ValidatorResult]) {
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
            let state_dir = repo_root()?.join(".atelier-state");
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
        "no_open_blockers" if target_kind == "issue" => {
            let blockers = db.get_blockers(target_id)?;
            let open = blockers
                .into_iter()
                .filter_map(|id| db.get_issue(&id).ok().flatten())
                .filter(|issue| issue.status == "open")
                .map(|issue| issue.id)
                .collect::<Vec<_>>();
            if open.is_empty() {
                Ok((true, "no open blockers".to_string()))
            } else {
                Ok((false, format!("open blockers: {}", open.join(", "))))
            }
        }
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

fn repo_root() -> Result<std::path::PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;
    if output.status.success() {
        Ok(Path::new(String::from_utf8_lossy(&output.stdout).trim()).to_path_buf())
    } else {
        Ok(std::env::current_dir()?)
    }
}
