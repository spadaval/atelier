//! App-layer use-case entrypoints for command families that still render in CLI.
//!
//! These functions centralize cache-use selection and record argument
//! resolution for migrated dispatch paths. CLI code may still render command
//! outcomes, but it should ask this module for the app-owned cache boundary
//! instead of choosing freshness policy or interpreting record ids itself.

use anyhow::{bail, Result};
use std::path::Path;

use crate::cache_manager::{CacheAccess, CacheManager, CacheUse};
use atelier_core::{EvidenceRecord, EvidenceRecordData, Record};
use atelier_records::{CanonicalIssueRecord, RecordStore};
use atelier_sqlite::Database;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceTargetArg {
    pub kind: String,
    pub id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CacheCommandRoute {
    pub command_family: &'static str,
    pub cache_use: CacheUse,
}

/// Cache-dependent read families and the stale-data policy they require.
/// Keep this inventory aligned with the central CLI dispatch.
pub const CACHE_COMMAND_ROUTES: &[CacheCommandRoute] = &[
    CacheCommandRoute {
        command_family: "status",
        cache_use: CacheUse::Orientation,
    },
    CacheCommandRoute {
        command_family: "issue show",
        cache_use: CacheUse::Orientation,
    },
    CacheCommandRoute {
        command_family: "issue list/options/blockers",
        cache_use: CacheUse::Decision,
    },
    CacheCommandRoute {
        command_family: "work/mission/epic",
        cache_use: CacheUse::Decision,
    },
    CacheCommandRoute {
        command_family: "evidence show/list",
        cache_use: CacheUse::Decision,
    },
    CacheCommandRoute {
        command_family: "bundle preview",
        cache_use: CacheUse::Decision,
    },
    CacheCommandRoute {
        command_family: "review read",
        cache_use: CacheUse::Decision,
    },
    CacheCommandRoute {
        command_family: "history/graph",
        cache_use: CacheUse::Decision,
    },
    CacheCommandRoute {
        command_family: "workflow check",
        cache_use: CacheUse::Decision,
    },
    CacheCommandRoute {
        command_family: "branch decision",
        cache_use: CacheUse::Decision,
    },
];

fn cache(cache_use: CacheUse) -> Result<CacheAccess> {
    CacheManager::discover()?.get_cache(cache_use)
}

pub fn status_cache() -> Result<CacheAccess> {
    cache(CacheUse::Orientation)
}

pub fn issue_detail_cache() -> Result<CacheAccess> {
    cache(CacheUse::Orientation)
}

pub fn work_query_cache() -> Result<CacheAccess> {
    cache(CacheUse::Decision)
}

pub fn issue_query_cache() -> Result<CacheAccess> {
    cache(CacheUse::Decision)
}

pub fn mutation_cache() -> Result<CacheAccess> {
    cache(CacheUse::Decision)
}

pub fn evidence_query_cache() -> Result<CacheAccess> {
    cache(CacheUse::Decision)
}

pub fn bundle_query_cache() -> Result<CacheAccess> {
    cache(CacheUse::Decision)
}

pub fn review_cache() -> Result<CacheAccess> {
    cache(CacheUse::Decision)
}

pub fn history_query_cache() -> Result<CacheAccess> {
    cache(CacheUse::Decision)
}

pub fn workflow_query_cache() -> Result<CacheAccess> {
    cache(CacheUse::Decision)
}

pub fn branch_decision_cache() -> Result<CacheAccess> {
    cache(CacheUse::Decision)
}

/// Lint owns canonical parse diagnostics and must be able to report malformed
/// record files before cache repair is possible. It opens the last cache state
/// without repairing it, then validates record files directly.
pub fn lint_cache() -> Result<CacheAccess> {
    CacheManager::discover()?.open_cache_for_health()
}

pub fn open_database(db_path: &Path) -> Result<Database> {
    Database::open(db_path).map_err(Into::into)
}

pub fn load_canonical_record(state_dir: &Path, kind: &str, id: &str) -> Result<Record> {
    RecordStore::new(state_dir).load_record_by_id(kind, id)
}

pub fn load_canonical_evidence(state_dir: &Path, id: &str) -> Result<EvidenceRecord> {
    match load_canonical_record(state_dir, "evidence", id)? {
        Record::Evidence(record) => Ok(record),
        other => bail!("Expected evidence record {id}, found {}", other.kind()),
    }
}

pub fn load_canonical_issue(state_dir: &Path, id: &str) -> Result<CanonicalIssueRecord> {
    RecordStore::new(state_dir).load_issue_by_id(id)
}

pub fn write_canonical_issue(state_dir: &Path, record: &CanonicalIssueRecord) -> Result<()> {
    RecordStore::new(state_dir).write_issue_atomic(record)
}

pub fn write_canonical_record(state_dir: &Path, record: &Record) -> Result<()> {
    RecordStore::new(state_dir).write_record_atomic(record)
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

pub fn resolve_issue_ref(storage: &CacheAccess, issue_ref: &str) -> Result<String> {
    let db = storage.db();
    if let Some(id) = db.resolve_issue_ref(issue_ref)? {
        return Ok(id);
    }

    if let Some(actual_kind) = db.record_kind_for_id(issue_ref)? {
        bail!("{}", wrong_kind_message("issue", &actual_kind, issue_ref));
    }

    bail!("Issue {issue_ref} was not found")
}

pub fn resolve_record_ref(storage: &CacheAccess, kind: &str, id: &str) -> Result<String> {
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
    storage: &CacheAccess,
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

pub fn resolve_evidence_target_ref(storage: &CacheAccess, kind: &str, id: &str) -> Result<String> {
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
    use super::{parse_evidence_target_arg, CACHE_COMMAND_ROUTES};
    use crate::cache_manager::CacheUse;

    #[test]
    fn cache_command_inventory_limits_degraded_reads_to_orientation() {
        let orientation = CACHE_COMMAND_ROUTES
            .iter()
            .filter(|route| route.cache_use == CacheUse::Orientation)
            .map(|route| route.command_family)
            .collect::<Vec<_>>();

        assert_eq!(orientation, ["status", "issue show"]);
        assert!(CACHE_COMMAND_ROUTES
            .iter()
            .any(|route| route.command_family == "work/mission/epic"));
        assert!(CACHE_COMMAND_ROUTES
            .iter()
            .any(|route| route.command_family == "workflow check"));
        assert!(CACHE_COMMAND_ROUTES
            .iter()
            .any(|route| route.command_family == "branch decision"));
    }

    #[test]
    fn evidence_target_parser_requires_kind_id_syntax() {
        let target = parse_evidence_target_arg("issue/atelier-1234").unwrap();
        assert_eq!(target.kind, "issue");
        assert_eq!(target.id, "atelier-1234");

        assert!(parse_evidence_target_arg("atelier-1234").is_err());
        assert!(parse_evidence_target_arg("issue/").is_err());
    }
}
