use anyhow::{bail, Result};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RecordKindSpec {
    pub kind: &'static str,
    pub schema: &'static str,
    pub schema_version: i64,
    pub canonical_dir: Option<&'static str>,
}

pub const ISSUE_KIND: RecordKindSpec = RecordKindSpec {
    kind: "issue",
    schema: "atelier.issue",
    schema_version: 1,
    canonical_dir: Some("issues"),
};

pub const FIRST_CLASS_RECORD_KINDS: &[RecordKindSpec] = &[
    RecordKindSpec {
        kind: "mission",
        schema: "atelier.mission",
        schema_version: 1,
        canonical_dir: Some("missions"),
    },
    RecordKindSpec {
        kind: "milestone",
        schema: "atelier.milestone",
        schema_version: 1,
        canonical_dir: Some("milestones"),
    },
    RecordKindSpec {
        kind: "plan",
        schema: "atelier.plan",
        schema_version: 1,
        canonical_dir: Some("plans"),
    },
    RecordKindSpec {
        kind: "evidence",
        schema: "atelier.evidence",
        schema_version: 1,
        canonical_dir: Some("evidence"),
    },
];

pub const NON_CANONICAL_RECORD_KINDS: &[RecordKindSpec] = &[RecordKindSpec {
    kind: "workflow_validator",
    schema: "atelier.workflow_validator",
    schema_version: 1,
    canonical_dir: None,
}];

pub fn record_kind(kind: &str) -> Option<&'static RecordKindSpec> {
    std::iter::once(&ISSUE_KIND)
        .chain(FIRST_CLASS_RECORD_KINDS.iter())
        .chain(NON_CANONICAL_RECORD_KINDS.iter())
        .find(|spec| spec.kind == kind)
}

pub fn canonical_record_kind(kind: &str) -> Result<&'static RecordKindSpec> {
    let Some(spec) = FIRST_CLASS_RECORD_KINDS
        .iter()
        .find(|spec| spec.kind == kind && spec.canonical_dir.is_some())
    else {
        bail!(
            "Record kind '{}' is not a canonical first-class record",
            kind
        );
    };
    Ok(spec)
}

pub fn validate_canonical_record_kind(kind: &str) -> Result<()> {
    canonical_record_kind(kind).map(|_| ())
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
    let Some(dir) = spec.canonical_dir else {
        bail!("Record kind '{}' has no canonical directory", spec.kind);
    };
    Ok(PathBuf::from(dir).join(format!("{id}.md")))
}

pub fn issue_record_path(id: &str) -> PathBuf {
    PathBuf::from(ISSUE_KIND.canonical_dir.expect("issue has canonical dir"))
        .join(format!("{id}.md"))
}

pub fn canonical_record_dirs() -> Vec<&'static str> {
    std::iter::once(ISSUE_KIND.canonical_dir.expect("issue has canonical dir"))
        .chain(
            FIRST_CLASS_RECORD_KINDS
                .iter()
                .filter_map(|spec| spec.canonical_dir),
        )
        .collect()
}

fn all_record_kind_names() -> Vec<&'static str> {
    std::iter::once(ISSUE_KIND.kind)
        .chain(FIRST_CLASS_RECORD_KINDS.iter().map(|spec| spec.kind))
        .chain(NON_CANONICAL_RECORD_KINDS.iter().map(|spec| spec.kind))
        .collect()
}
