//! App-layer use-case entrypoints for command families that still render in CLI.
//!
//! These functions centralize storage-mode selection and record argument
//! resolution for migrated dispatch paths. CLI code may still render command
//! outcomes, but it should ask this module for the app-owned storage context
//! instead of choosing storage access modes or interpreting record ids itself.

use anyhow::{bail, Result};
use std::path::Path;

use crate::command_storage::{command_storage, CommandStorage, CommandStorageAccess};
use atelier_core::{EvidenceRecord, EvidenceRecordData, ReviewRecord};
use atelier_records::{CanonicalIssueRecord, RecordStore};
use atelier_sqlite::Database;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceTargetArg {
    pub kind: String,
    pub id: String,
}

pub fn status_storage() -> Result<CommandStorage> {
    command_storage(CommandStorageAccess::DegradedProjectionQuery)
}

pub fn mission_query_storage() -> Result<CommandStorage> {
    command_storage(CommandStorageAccess::DegradedProjectionQuery)
}

pub fn mission_mutation_storage() -> Result<CommandStorage> {
    command_storage(CommandStorageAccess::CanonicalMutation)
}

pub fn evidence_query_storage() -> Result<CommandStorage> {
    command_storage(CommandStorageAccess::ProjectionQuery)
}

pub fn evidence_mutation_storage() -> Result<CommandStorage> {
    command_storage(CommandStorageAccess::CanonicalMutation)
}

pub fn plan_mutation_storage() -> Result<CommandStorage> {
    command_storage(CommandStorageAccess::CanonicalMutation)
}

pub fn workflow_query_storage() -> Result<CommandStorage> {
    command_storage(CommandStorageAccess::ProjectionQuery)
}

pub fn refreshed_mutation_db(storage: &CommandStorage) -> Result<Database> {
    open_database(&storage.db_path())
}

pub fn open_database(db_path: &Path) -> Result<Database> {
    Database::open(db_path).map_err(Into::into)
}

pub fn refresh_after_canonical_write(state_dir: &Path, db_path: &Path) -> Result<()> {
    crate::projection::refresh_after_canonical_write(state_dir, db_path)
}

pub fn load_canonical_evidence(state_dir: &Path, id: &str) -> Result<EvidenceRecord> {
    RecordStore::new(state_dir).load_evidence_by_id(id)
}

pub fn load_canonical_issue(state_dir: &Path, id: &str) -> Result<CanonicalIssueRecord> {
    RecordStore::new(state_dir).load_issue_by_id(id)
}

pub fn write_canonical_issue(state_dir: &Path, record: &CanonicalIssueRecord) -> Result<()> {
    RecordStore::new(state_dir).write_issue_atomic(record)
}

pub fn write_canonical_evidence(state_dir: &Path, record: &EvidenceRecord) -> Result<()> {
    RecordStore::new(state_dir).write_evidence_atomic(record)
}

pub fn load_review(state_dir: &Path, id: &str) -> Result<ReviewRecord> {
    RecordStore::new(state_dir).load_review_by_id(id)
}

pub fn write_review(state_dir: &Path, record: &ReviewRecord) -> Result<()> {
    RecordStore::new(state_dir).write_review_atomic(record)
}

pub fn create_evidence_record(
    state_dir: &Path,
    summary: &str,
    status: &str,
    body: &str,
    data: EvidenceRecordData,
) -> Result<EvidenceRecord> {
    RecordStore::new(state_dir).create_evidence(summary, status, body, data)
}

pub fn add_record_relationship(
    state_dir: &Path,
    source_kind: &str,
    source_id: &str,
    target_kind: &str,
    target_id: &str,
    relation: &str,
) -> Result<bool> {
    RecordStore::new(state_dir).add_relates_relationship(
        source_kind,
        source_id,
        target_kind,
        target_id,
        relation,
    )
}

pub fn remove_record_relationship(
    state_dir: &Path,
    source_kind: &str,
    source_id: &str,
    target_kind: &str,
    target_id: &str,
    relation: &str,
) -> Result<bool> {
    RecordStore::new(state_dir).remove_relates_relationship(
        source_kind,
        source_id,
        target_kind,
        target_id,
        relation,
    )
}

pub fn add_attachment_relationship(
    state_dir: &Path,
    source_kind: &str,
    source_id: &str,
    target_kind: &str,
    target_id: &str,
    relation: &str,
) -> Result<bool> {
    RecordStore::new(state_dir).add_attachment_relationship(
        source_kind,
        source_id,
        target_kind,
        target_id,
        relation,
    )
}

pub fn resolve_issue_ref(storage: &CommandStorage, issue_ref: &str) -> Result<String> {
    let db = storage.db();
    if let Some(id) = db.resolve_issue_ref(issue_ref)? {
        return Ok(id);
    }

    if let Some(actual_kind) = db.record_kind_for_id(issue_ref)? {
        bail!("{}", wrong_kind_message("issue", &actual_kind, issue_ref));
    }

    bail!("Issue {issue_ref} was not found")
}

pub fn resolve_record_ref(storage: &CommandStorage, kind: &str, id: &str) -> Result<String> {
    let db = storage.db();
    if kind == "issue" {
        resolve_issue_ref(storage, id)
    } else if db.get_record(kind, id)?.is_some() {
        Ok(id.to_string())
    } else if let Some(actual_kind) = db.record_kind_for_id(id)? {
        bail!("{}", wrong_kind_message(kind, &actual_kind, id));
    } else {
        Ok(id.to_string())
    }
}

pub fn resolve_optional_record_ref(
    storage: &CommandStorage,
    kind: &str,
    id: Option<String>,
) -> Result<Option<String>> {
    id.map(|id| resolve_record_ref(storage, kind, &id))
        .transpose()
}

pub fn parse_evidence_target_arg(target: &str) -> Result<EvidenceTargetArg> {
    let Some((kind, id)) = target.split_once('/') else {
        bail!("--target must use kind/id syntax, for example issue/atelier-1234");
    };
    if kind.trim().is_empty() || id.trim().is_empty() {
        bail!("--target must use kind/id syntax, for example issue/atelier-1234");
    }
    Ok(EvidenceTargetArg {
        kind: kind.to_string(),
        id: id.to_string(),
    })
}

pub fn resolve_evidence_target_ref(
    storage: &CommandStorage,
    kind: &str,
    id: &str,
) -> Result<String> {
    if matches!(kind, "issue" | "epic") {
        resolve_issue_ref(storage, id)
    } else {
        Ok(id.to_string())
    }
}

fn wrong_kind_message(expected_kind: &str, actual_kind: &str, id: &str) -> String {
    let suggested = show_command_for_kind(actual_kind)
        .map(|command| format!(" Use `{command} {id}`."))
        .unwrap_or_default();
    format!("{id} is a {actual_kind} record, not a {expected_kind} record.{suggested}")
}

fn show_command_for_kind(kind: &str) -> Option<&'static str> {
    match kind {
        "issue" | "epic" => Some("atelier issue show"),
        "mission" => Some("atelier issue show"),
        "evidence" => Some("atelier evidence show"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_core::{RecordHeader, Relationships};
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn use_case_storage_selectors_are_named_for_target_workflows() {
        let selectors = [
            "status",
            "mission_query",
            "mission_mutation",
            "evidence_query",
            "evidence_mutation",
            "plan_mutation",
            "workflow_query",
        ];

        assert_eq!(selectors.len(), 7);
    }

    #[test]
    fn evidence_target_parser_requires_kind_id_syntax() {
        let target = parse_evidence_target_arg("issue/atelier-1234").unwrap();
        assert_eq!(target.kind, "issue");
        assert_eq!(target.id, "atelier-1234");

        assert!(parse_evidence_target_arg("atelier-1234").is_err());
        assert!(parse_evidence_target_arg("issue/").is_err());
    }

    #[test]
    fn concrete_evidence_and_review_services_round_trip_domain_types() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let now = Utc::now();
        let evidence = create_evidence_record(
            &state_dir,
            "Typed evidence",
            "recorded",
            "Concrete evidence service proof.",
            EvidenceRecordData {
                evidence_type: "test".to_string(),
                captured_at: now,
                command: None,
                path: None,
                uri: None,
                producer: None,
                proof_scope: None,
                agent_identity: None,
                independence_level: None,
                residual_risks: Vec::new(),
                follow_up_ids: Vec::new(),
                exit_code: None,
                exit_status: None,
                success: None,
                spawn_error: None,
                output: None,
                target: None,
            },
        )
        .unwrap();
        assert_eq!(
            load_canonical_evidence(&state_dir, &evidence.header.id).unwrap(),
            evidence
        );

        let review = ReviewRecord {
            header: RecordHeader {
                kind: "review".to_string(),
                id: "atelier-rvw1".to_string(),
                title: "Typed review".to_string(),
                status: "open".to_string(),
                labels: vec!["review".to_string()],
                relationships: Relationships::default(),
                created_at: now,
                updated_at: now,
            },
            mode: "room".to_string(),
            issue_id: "atelier-issue".to_string(),
            source_branch: "feature/atelier-issue".to_string(),
            target_branch: "main".to_string(),
            events: Vec::new(),
        };
        write_review(&state_dir, &review).unwrap();
        assert_eq!(load_review(&state_dir, "atelier-rvw1").unwrap(), review);
    }
}
