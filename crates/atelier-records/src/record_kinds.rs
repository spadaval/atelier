use anyhow::{bail, Result};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RecordKindSpec {
    pub kind: &'static str,
    pub schema: &'static str,
    pub schema_version: i64,
    pub canonical_dir: &'static str,
    pub extension: &'static str,
}

pub const ISSUE_KIND: RecordKindSpec = RecordKindSpec {
    kind: "issue",
    schema: "atelier.issue",
    schema_version: 2,
    canonical_dir: "issues",
    extension: "md",
};

pub const EVIDENCE_KIND: RecordKindSpec = RecordKindSpec {
    kind: "evidence",
    schema: "atelier.evidence",
    schema_version: 2,
    canonical_dir: "evidence",
    extension: "md",
};

pub const REVIEW_KIND: RecordKindSpec = RecordKindSpec {
    kind: "review",
    schema: "atelier.review",
    schema_version: 1,
    canonical_dir: "reviews",
    extension: "md",
};

pub const CANONICAL_RECORD_KINDS: &[RecordKindSpec] = &[ISSUE_KIND, REVIEW_KIND, EVIDENCE_KIND];

pub fn record_kind(kind: &str) -> Option<&'static RecordKindSpec> {
    CANONICAL_RECORD_KINDS.iter().find(|spec| spec.kind == kind)
}

pub fn canonical_record_kind(kind: &str) -> Result<&'static RecordKindSpec> {
    let Some(spec) = record_kind(kind) else {
        bail!(
            "Record kind '{}' is not a first-class record-file type",
            kind
        );
    };
    Ok(spec)
}

pub fn validate_record_kind(kind: &str) -> Result<()> {
    if record_kind(kind).is_some() {
        Ok(())
    } else {
        bail!(
            "Invalid record kind '{}'. Valid values: {}",
            kind,
            all_record_kind_names().join(", ")
        )
    }
}

pub fn canonical_record_path(spec: &RecordKindSpec, id: &str) -> Result<PathBuf> {
    Ok(PathBuf::from(spec.canonical_dir).join(format!("{id}.{}", spec.extension)))
}

pub fn issue_record_path(id: &str) -> PathBuf {
    canonical_record_path(&ISSUE_KIND, id).expect("issue has canonical dir")
}

pub fn canonical_record_dirs() -> Vec<&'static str> {
    CANONICAL_RECORD_KINDS
        .iter()
        .map(|spec| spec.canonical_dir)
        .collect()
}

fn all_record_kind_names() -> Vec<&'static str> {
    CANONICAL_RECORD_KINDS
        .iter()
        .map(|spec| spec.kind)
        .collect()
}
