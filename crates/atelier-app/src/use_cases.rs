//! App-layer use-case entrypoints for command families that still render in CLI.

use anyhow::{anyhow, bail, Result};
use atelier_core::InvalidValue;
use std::path::PathBuf;

use crate::record_index::RecordIndex;
use crate::record_ref::RecordKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceTargetArg {
    pub kind: String,
    pub id: String,
}

/// Discovers the enclosing repository's state directory and loads every
/// record in it.
pub fn load_repository() -> Result<(PathBuf, RecordIndex)> {
    let state_dir = crate::storage_layout::StorageLayout::discover()?.canonical_dir();
    let index = RecordIndex::load(&state_dir).map_err(|error| {
        anyhow!("{error:#}\nNext: run `atelier check`, fix every named record file, and rerun.")
    })?;
    Ok((state_dir, index))
}

pub fn resolve_issue_ref(index: &RecordIndex, issue_ref: &str) -> Result<String> {
    index.resolve(RecordKind::Issue, issue_ref)
}

/// Parses `--target <kind>/<id>`; any other shape is an invalid value.
pub fn parse_evidence_target_arg(target: &str) -> Result<EvidenceTargetArg> {
    let malformed = || InvalidValue::new("target", target, &["issue/<id>", "evidence/<id>"]);
    let Some((kind, id)) = target.split_once('/') else {
        bail!(malformed());
    };
    if kind.trim().is_empty() || id.trim().is_empty() {
        bail!(malformed());
    }
    Ok(EvidenceTargetArg {
        kind: kind.to_string(),
        id: id.to_string(),
    })
}

/// Resolves an evidence target's id. Evidence targets issues and evidence
/// records; any other kind is an invalid value.
pub fn resolve_evidence_target_ref(index: &RecordIndex, kind: &str, id: &str) -> Result<String> {
    match kind {
        "issue" => index.resolve(RecordKind::Issue, id),
        "evidence" => index.resolve(RecordKind::Evidence, id),
        _ => Err(InvalidValue::new("target kind", kind, &["issue", "evidence"]).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_core::EvidenceRecordData;
    use atelier_records::RecordStore;
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn evidence_target_parser_requires_kind_id_syntax() {
        let target = parse_evidence_target_arg("issue/atelier-1234").unwrap();
        assert_eq!(target.kind, "issue");
        assert_eq!(target.id, "atelier-1234");

        assert!(parse_evidence_target_arg("atelier-1234").is_err());
        assert!(parse_evidence_target_arg("issue/").is_err());
    }

    #[test]
    fn concrete_evidence_service_round_trips_domain_type() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let now = Utc::now();
        let evidence = RecordStore::for_project(&state_dir, "atelier")
            .create_evidence(
                "Typed evidence",
                "recorded",
                "Concrete evidence service proof.",
                EvidenceRecordData {
                    commit: "a".repeat(40),
                    result: atelier_core::EvidenceResult::Pass,
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
            RecordStore::new(&state_dir)
                .load_evidence_by_id(&evidence.header.id)
                .unwrap(),
            evidence
        );
    }
}
