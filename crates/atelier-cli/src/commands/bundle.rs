use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::human_output::local_date;
use atelier_app::record_index::RecordIndex;
use atelier_core::{EvidenceRecordData, Issue};
use atelier_records::{
    validate_issue_type, validate_priority, validate_record_kind, validate_status,
};
use atelier_records::{CanonicalIssueRecord, RecordStore, Relationships};

pub fn preview(index: &RecordIndex, input: &str) -> Result<()> {
    let bundle = load_bundle(index, input)?;
    validate_bundle(index, &bundle)?;
    let plan = BundleGraphPlan::from_bundle(index, &bundle)?;
    print_bundle_summary(preview_bundle_summary(&bundle, &plan))
}

pub fn apply(index: &RecordIndex, state_dir: &Path, input: &str, yes: bool) -> Result<()> {
    if !yes {
        bail!("bundle apply requires --yes");
    }
    let _transaction = atelier_records::mutation_lock::CanonicalMutationLock::exclusive(state_dir)?;
    let bundle = load_bundle(index, input)?;
    validate_bundle(index, &bundle)?;
    let plan = BundleGraphPlan::from_bundle(index, &bundle)?;

    let project_slug =
        atelier_app::project_config::ProjectConfig::load_from_state_dir(state_dir)?.project_slug;
    let summary = apply_bundle_file(state_dir, &project_slug, &bundle, &plan)?;
    print_bundle_summary(summary)
}

fn load_bundle(index: &RecordIndex, input: &str) -> Result<BundleFile> {
    let bytes = fs::read(input).with_context(|| format!("Failed to read bundle file {input}"))?;
    let parse_error = |error: serde_json::Error| {
        let renamed = error
            .to_string()
            .contains("unknown field `depends_on`")
            .then_some("; the issue field depends_on is now blocked_by, matching atelier issue link --role blocked_by")
            .unwrap_or_default();
        anyhow::anyhow!("Failed to parse bundle {input} as JSON: {error}{renamed}")
    };
    let mut value: Value = serde_json::from_slice(&bytes).map_err(parse_error)?;
    if let Some(issues) = value
        .pointer_mut("/resources/issues")
        .and_then(Value::as_array_mut)
    {
        let original = issues.clone();
        for issue in issues {
            fill_update_from_record(index, &original, issue)?;
        }
    }
    serde_json::from_value(value).map_err(parse_error)
}

/// An `update` entry names an existing record by `id` and lists only the
/// fields it changes. Fill the rest from the record, so an update passes the
/// same validation, planning, and staged-graph checks as a create.
/// The existing issue a bundle parent reference names, through an `id` or
/// the `client_ref` of an update entry.
fn parent_ref_id(index: &RecordIndex, bundle_issues: &[Value], parent: &Value) -> Option<String> {
    if let Some(id) = parent.get("id").and_then(Value::as_str) {
        return index.resolve_issue_ref(id).ok().flatten();
    }
    let client_ref = parent.get("client_ref").and_then(Value::as_str)?;
    let entry = bundle_issues
        .iter()
        .find(|entry| entry.get("client_ref").and_then(Value::as_str) == Some(client_ref))?;
    let id = entry.get("id").and_then(Value::as_str)?;
    index.resolve_issue_ref(id).ok().flatten()
}

fn fill_update_from_record(
    index: &RecordIndex,
    bundle_issues: &[Value],
    issue: &mut Value,
) -> Result<()> {
    let Some(object) = issue.as_object_mut() else {
        return Ok(());
    };
    if object.get("operation").and_then(Value::as_str) != Some("update") {
        if object.contains_key("id") {
            bail!("Bundle issue field `id` names the record to change and needs \"operation\": \"update\"");
        }
        return Ok(());
    }
    let client_ref = object
        .get("client_ref")
        .and_then(Value::as_str)
        .unwrap_or("(missing client_ref)")
        .to_string();
    let id = object
        .get("id")
        .and_then(Value::as_str)
        .with_context(|| format!("Update {client_ref} needs the id of the record it changes"))?;
    let id = index
        .resolve_issue_ref(id)?
        .with_context(|| format!("Update {client_ref} names missing issue {id}"))?;
    let record = index
        .issue_record(&id)
        .with_context(|| format!("Update {client_ref} names missing issue {id}"))?;
    if object.contains_key("status") {
        bail!("Update {client_ref} cannot set status; change {id}'s status with atelier issue transition");
    }
    let repo_root = atelier_app::storage_layout::find_repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    if object.contains_key("body") && policy.status_category(&record.issue.status) == Some("done") {
        bail!(
            "issue_body_done: update {client_ref} would change the body of {id}, which is {}; its done claim and proof were judged against the current acceptance. File follow-up work instead",
            record.issue.status
        );
    }
    if let Some(parent) = object.get("parent") {
        if !index.get_subissues(&id).is_empty() {
            bail!("Update {client_ref} cannot give {id} a parent: it has children of its own, and parents are one level deep");
        }
        if let Some(current) = &record.issue.parent_id {
            let same =
                parent_ref_id(index, bundle_issues, parent).as_deref() == Some(current.as_str());
            if !same {
                bail!("Update {client_ref} cannot move {id} from parent {current}; use atelier issue update --parent");
            }
        }
    }
    if let Some(issue_type) = object.get("issue_type").and_then(Value::as_str) {
        if issue_type != record.issue.issue_type {
            bail!(
                "Update {client_ref} cannot change {id} from {} to {issue_type}; use atelier issue update --issue-type",
                record.issue.issue_type
            );
        }
    }
    if let Some(body) = object.get("body").and_then(Value::as_str) {
        let parsed = atelier_records::parse_issue_sections_for_type(
            body,
            Path::new("<bundle issue body>"),
            &record.issue.issue_type,
        )
        .with_context(|| format!("Invalid body for {client_ref}"))?;
        if parsed.notes.is_some() || parsed.close_reason.is_some() {
            bail!(
                "Update {client_ref} replaces typed sections only; add notes with the notes field"
            );
        }
    } else {
        let mut sections = record.clone();
        sections.sections.notes = None;
        sections.sections.close_reason = None;
        let rendered = atelier_records::render_issue_record(&sections)?;
        let (_, body) = atelier_records::split_document(&rendered)
            .with_context(|| format!("cannot read the body of {id}"))?;
        object.insert("body".into(), Value::String(body.to_string()));
    }
    object.insert("id".into(), Value::String(id.clone()));
    object
        .entry("title")
        .or_insert_with(|| Value::String(record.issue.title.clone()));
    object.insert(
        "issue_type".into(),
        Value::String(record.issue.issue_type.clone()),
    );
    object.entry("priority").or_insert_with(|| {
        Value::String(
            atelier_core::IssuePriority::from_canonical_token(&record.issue.priority)
                .map(|priority| priority.label().to_string())
                .unwrap_or_else(|_| record.issue.priority.clone()),
        )
    });
    object
        .entry("labels")
        .or_insert_with(|| json!(record.labels));
    if let Some(strategy) = &record.issue.strategy {
        object
            .entry("strategy")
            .or_insert_with(|| json!({ "path": strategy.path }));
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleFile {
    schema: String,
    schema_version: i64,
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(rename = "metadata", default)]
    _metadata: Value,
    #[serde(default)]
    resources: BundleResources,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleResources {
    #[serde(default)]
    issues: Vec<BundleIssue>,
    #[serde(default)]
    missions: Vec<serde_json::Value>,
    #[serde(default)]
    evidence: Vec<BundleEvidence>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleIssue {
    client_ref: String,
    title: String,
    #[serde(default)]
    operation: Option<String>,
    /// The existing record an `update` changes.
    #[serde(default)]
    id: Option<String>,
    body: String,
    #[serde(default)]
    strategy: Option<atelier_core::IssueStrategy>,
    #[serde(default = "default_issue_type")]
    issue_type: String,
    #[serde(default = "default_priority")]
    priority: String,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    parent: Option<BundleRef>,
    #[serde(default)]
    blocked_by: Vec<BundleRef>,
    #[serde(default)]
    blocks: Vec<BundleRef>,
    #[serde(default)]
    advances: Vec<BundleRef>,
    #[serde(default)]
    notes: Vec<BundleNote>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleEvidence {
    client_ref: String,
    title: String,
    #[serde(default)]
    operation: Option<String>,
    evidence_type: String,
    result: String,
    commit: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    validates: Vec<BundleRef>,
    #[serde(default)]
    artifact: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleRef {
    #[serde(default)]
    client_ref: Option<String>,
    #[serde(default)]
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleNote {
    body: String,
    #[serde(default)]
    author: Option<String>,
    #[serde(default)]
    created_at: Option<String>,
}

#[derive(Debug, Clone)]
struct ResolvedRef {
    kind: String,
    id: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum PlannedReference {
    ClientRef(String),
    Id(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct PlannedRef {
    kind: String,
    reference: PlannedReference,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct PlannedRelationship {
    source: PlannedRef,
    target: PlannedRef,
    role: String,
    /// The target's own entry declared the edge, so only the target's
    /// record stores it (atelier-ihss).
    declared_by_target: bool,
}

#[derive(Debug)]
struct PlannedNote<'a> {
    target: PlannedRef,
    note: &'a BundleNote,
}

/// The complete, validated mutation plan for a bundle.
///
/// Relationship direction is canonical rather than dependent on the authored
/// field: parents point to children and blockers point to blocked issues. This
/// makes `blocked_by` and `blocks` normalize to the same graph edge. The edge
/// is stored only on the record whose entry declared it.
#[derive(Debug)]
struct BundleGraphPlan<'a> {
    issues: Vec<&'a BundleIssue>,
    evidence: Vec<&'a BundleEvidence>,
    notes: Vec<PlannedNote<'a>>,
    relationships: Vec<PlannedRelationship>,
}

impl<'a> BundleGraphPlan<'a> {
    fn from_bundle(index: &RecordIndex, bundle: &'a BundleFile) -> Result<Self> {
        let mut issues = bundle.resources.issues.iter().collect::<Vec<_>>();
        issues.sort_by(|left, right| left.client_ref.cmp(&right.client_ref));
        let mut evidence = bundle.resources.evidence.iter().collect::<Vec<_>>();
        evidence.sort_by(|left, right| left.client_ref.cmp(&right.client_ref));

        let mut notes = Vec::new();
        let mut relationships = BTreeSet::new();
        for issue in &issues {
            let issue_ref = planned_client_ref(bundle, &issue.client_ref)?;
            for note in &issue.notes {
                notes.push(PlannedNote {
                    target: issue_ref.clone(),
                    note,
                });
            }
            if let Some(parent) = &issue.parent {
                insert_planned_relationship(
                    &mut relationships,
                    planned_bundle_ref(index, bundle, parent)?,
                    issue_ref.clone(),
                    "parent",
                    false,
                )?;
            }
            for blocker in &issue.blocked_by {
                insert_planned_relationship(
                    &mut relationships,
                    planned_bundle_ref(index, bundle, blocker)?,
                    issue_ref.clone(),
                    "blocks",
                    true,
                )?;
            }
            for blocked in &issue.blocks {
                insert_planned_relationship(
                    &mut relationships,
                    issue_ref.clone(),
                    planned_bundle_ref(index, bundle, blocked)?,
                    "blocks",
                    false,
                )?;
            }
            // Validation ensures exactly one endpoint is a mission. Plan the
            // edge from the mission so declaring it on both sides is a duplicate.
            for target in &issue.advances {
                let target_ref = planned_bundle_ref(index, bundle, target)?;
                let declared_by_target = issue.issue_type != "mission";
                let (source, target) = if declared_by_target {
                    (target_ref, issue_ref.clone())
                } else {
                    (issue_ref.clone(), target_ref)
                };
                insert_planned_relationship(
                    &mut relationships,
                    source,
                    target,
                    "advances",
                    declared_by_target,
                )?;
            }
        }
        for record in &evidence {
            let source = planned_client_ref(bundle, &record.client_ref)?;
            for target in &record.validates {
                insert_planned_relationship(
                    &mut relationships,
                    source.clone(),
                    planned_bundle_ref(index, bundle, target)?,
                    "validates",
                    false,
                )?;
            }
        }

        Ok(Self {
            issues,
            evidence,
            notes,
            relationships: relationships.into_iter().collect(),
        })
    }
}

fn insert_planned_relationship(
    relationships: &mut BTreeSet<PlannedRelationship>,
    source: PlannedRef,
    target: PlannedRef,
    role: &str,
    declared_by_target: bool,
) -> Result<()> {
    let relationship = PlannedRelationship {
        source,
        target,
        role: role.to_string(),
        declared_by_target,
    };
    // The same edge declared from its other endpoint is a duplicate too.
    let from_other_endpoint = PlannedRelationship {
        declared_by_target: !declared_by_target,
        ..relationship.clone()
    };
    if relationships.contains(&from_other_endpoint) || !relationships.insert(relationship.clone()) {
        bail!(
            "Duplicate bundle relationship after normalization: {} -> {} ({})",
            planned_ref_label(&relationship.source),
            planned_ref_label(&relationship.target),
            relationship.role
        );
    }
    Ok(())
}

fn validate_bundle(index: &RecordIndex, bundle: &BundleFile) -> Result<()> {
    if bundle.schema != "atelier.bundle" {
        bail!("Invalid bundle schema '{}'", bundle.schema);
    }
    if bundle.schema_version != 2 {
        bail!(
            "Unsupported bundle schema_version {}",
            bundle.schema_version
        );
    }
    if bundle.title.trim().is_empty() {
        bail!("Bundle title cannot be empty");
    }
    if !bundle.resources.missions.is_empty() {
        bail!(
            "resources.missions is no longer supported; author missions in resources.issues with issue_type \"mission\" and advances links"
        );
    }

    let mut refs = BTreeSet::new();
    for (client_ref, kind) in bundle.client_refs() {
        validate_client_ref(&client_ref)?;
        if !refs.insert(client_ref.clone()) {
            bail!("Duplicate client_ref '{}'", client_ref);
        }
        if matches!(kind, "issue" | "mission") {
            continue;
        }
        validate_record_kind(kind)?;
    }

    let mut updated = BTreeSet::new();
    for issue in &bundle.resources.issues {
        if let Some(id) = &issue.id {
            if !updated.insert(id.clone()) {
                bail!(
                    "Bundle updates {id} more than once (again as {}); merge the changes into one entry",
                    issue.client_ref
                );
            }
        }
    }
    for issue in &bundle.resources.issues {
        validate_issue_operation(issue)?;
        if issue.title.trim().is_empty() {
            bail!("Issue {} title cannot be empty", issue.client_ref);
        }
        validate_bundle_issue_type(&issue.issue_type)?;
        atelier_records::parse_issue_sections_for_type(
            &issue.body,
            Path::new("<bundle issue body>"),
            &issue.issue_type,
        )
        .with_context(|| format!("Invalid body for {}", issue.client_ref))?;
        if let Some(strategy) = &issue.strategy {
            if issue.issue_type != "mission" {
                bail!("strategy is only valid for mission {}", issue.client_ref);
            }
            atelier_core::IssueStrategy::new(&strategy.path)?;
        }
        validate_priority(&issue.priority)?;
        if let Some(status) = &issue.status {
            validate_bundle_issue_status(&issue.issue_type, status)?;
        }
        validate_refs_exist(index, bundle, issue.parent.iter(), &issue.client_ref)?;
        validate_refs_exist(index, bundle, issue.blocked_by.iter(), &issue.client_ref)?;
        validate_refs_exist(index, bundle, issue.blocks.iter(), &issue.client_ref)?;
        validate_refs_exist(index, bundle, issue.advances.iter(), &issue.client_ref)?;
    }
    for evidence in &bundle.resources.evidence {
        validate_operation(evidence.operation.as_deref(), &evidence.client_ref)?;
        if !matches!(evidence.commit.len(), 40 | 64)
            || !evidence.commit.bytes().all(|b| b.is_ascii_hexdigit())
        {
            bail!(
                "Evidence {} requires a full Git commit ID",
                evidence.client_ref
            );
        }
        if evidence.title.trim().is_empty() {
            bail!("Evidence {} title cannot be empty", evidence.client_ref);
        }
        if ![
            "test",
            "log",
            "screenshot",
            "report",
            "benchmark",
            "external",
        ]
        .contains(&evidence.evidence_type.as_str())
        {
            bail!(
                "Invalid evidence_type '{}' for {}",
                evidence.evidence_type,
                evidence.client_ref
            );
        }
        if !["pass", "fail", "blocked", "not-applicable"].contains(&evidence.result.as_str()) {
            bail!(
                "Invalid evidence result '{}' for {}",
                evidence.result,
                evidence.client_ref
            );
        }
        validate_refs_exist(
            index,
            bundle,
            evidence.validates.iter(),
            &evidence.client_ref,
        )?;
    }
    validate_bundle_hierarchy(index, bundle)?;
    Ok(())
}

fn validate_bundle_hierarchy(index: &RecordIndex, bundle: &BundleFile) -> Result<()> {
    for issue in &bundle.resources.issues {
        if issue.issue_type == "mission" && issue.parent.is_some() {
            bail!(
                "Mission issue {} cannot have parent; use advances links for mission work",
                issue.client_ref
            );
        }
        if let Some(parent) = &issue.parent {
            let parent_type =
                bundle_issue_type_for_ref(index, bundle, parent)?.ok_or_else(|| {
                    anyhow::anyhow!(
                        "Issue parent for {} must resolve to an issue",
                        issue.client_ref
                    )
                })?;
            if parent_type == "mission" {
                bail!(
                    "Issue {} cannot be child of a mission; use advances links for mission work",
                    issue.client_ref
                );
            }
            // Parents are one level deep: the parent cannot itself be a child,
            // and this child cannot be a parent in the same bundle.
            let parent_is_child = match (&parent.client_ref, &parent.id) {
                (Some(client_ref), _) => bundle.resources.issues.iter().any(|other| {
                    &other.client_ref == client_ref
                        && (other.parent.is_some()
                            || other.id.as_ref().is_some_and(|id| {
                                index
                                    .get_issue(id)
                                    .is_some_and(|record| record.parent_id.is_some())
                            }))
                }),
                (None, Some(id)) => index
                    .resolve_issue_ref(id)?
                    .and_then(|id| index.get_issue(&id))
                    .is_some_and(|other| other.parent_id.is_some()),
                (None, None) => false,
            };
            let is_parent = bundle.resources.issues.iter().any(|other| {
                other
                    .parent
                    .as_ref()
                    .and_then(|parent| parent.client_ref.as_ref())
                    == Some(&issue.client_ref)
            });
            if parent_is_child || is_parent {
                bail!(
                    "Issue {} would nest a parent under a parent; parents are one level deep",
                    issue.client_ref
                );
            }
        }
        for target in &issue.advances {
            let target_type =
                bundle_issue_type_for_ref(index, bundle, target)?.ok_or_else(|| {
                    anyhow::anyhow!(
                        "advances target for {} must resolve to an issue",
                        issue.client_ref
                    )
                })?;
            atelier_app::workflow_policy::validate_advances_endpoints(
                &issue.client_ref,
                &issue.issue_type,
                &target_type,
            )?;
        }
    }
    Ok(())
}

fn bundle_issue_type_for_ref(
    index: &RecordIndex,
    bundle: &BundleFile,
    reference: &BundleRef,
) -> Result<Option<String>> {
    if let Some(client_ref) = &reference.client_ref {
        if let Some(issue) = bundle
            .resources
            .issues
            .iter()
            .find(|issue| &issue.client_ref == client_ref)
        {
            return Ok(Some(issue.issue_type.clone()));
        }
        return Ok(None);
    }
    if let Some(id) = &reference.id {
        return Ok(index
            .resolve_issue_ref(id)?
            .and_then(|id| index.get_issue(&id))
            .map(|issue| issue.issue_type));
    }
    Ok(None)
}

fn validate_issue_operation(issue: &BundleIssue) -> Result<()> {
    match (issue.operation.as_deref().unwrap_or("create"), &issue.id) {
        ("create", None) | ("update", Some(_)) => Ok(()),
        ("update", None) => bail!(
            "Update {} needs the id of the record it changes",
            issue.client_ref
        ),
        (other, _) => bail!(
            "Unsupported operation '{other}' for {}; bundle issues support create and update",
            issue.client_ref
        ),
    }
}

fn validate_operation(operation: Option<&str>, client_ref: &str) -> Result<()> {
    match operation.unwrap_or("create") {
        "create" => Ok(()),
        other => {
            bail!(
                "Unsupported operation '{other}' for {client_ref}; bundle evidence is create-only"
            )
        }
    }
}

fn validate_bundle_issue_type(issue_type: &str) -> Result<()> {
    validate_issue_type(issue_type)?;
    let repo_root = atelier_app::storage_layout::find_repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    if policy.issue_types.contains_key(issue_type) {
        Ok(())
    } else {
        bail!(
            "Invalid issue_type '{}'. Valid values: {}",
            issue_type,
            policy
                .issue_types
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// Bundles create work in the same statuses as `issue create --status`, so
/// they cannot skip a transition and its checks (atelier-9co2).
fn validate_bundle_issue_status(issue_type: &str, status: &str) -> Result<()> {
    validate_status(status)?;
    let repo_root = atelier_app::storage_layout::find_repo_root()?;
    let policy = atelier_app::workflow_policy::load(&repo_root)?;
    if !policy.statuses.contains_key(status) {
        bail!(
            "Invalid bundle issue status '{status}'; status is not defined in .atelier/workflow.yaml"
        )
    }
    let allowed = atelier_app::workflow_policy::creatable_statuses(&policy, issue_type)?;
    if !allowed.contains(status) {
        bail!(
            "Invalid bundle issue status '{status}'; {issue_type} work cannot be created in it; use one of: {}. Other statuses are reached with atelier issue transition after the bundle is applied.",
            allowed.into_iter().collect::<Vec<_>>().join(", ")
        )
    }
    Ok(())
}

fn workflow_initial_issue_status(issue_type: &str) -> Result<String> {
    let repo_root = atelier_app::storage_layout::find_repo_root()?;
    Ok(
        atelier_app::workflow_policy::configured_initial_status(&repo_root, issue_type)?
            .unwrap_or_else(|| "todo".to_string()),
    )
}

fn apply_bundle_file(
    state_dir: &Path,
    project_slug: &str,
    bundle: &BundleFile,
    plan: &BundleGraphPlan<'_>,
) -> Result<Value> {
    let stage_parent = state_dir
        .parent()
        .with_context(|| format!("Cannot determine parent for {}", state_dir.display()))?;
    ensure_no_bundle_recovery_artifacts(state_dir)?;
    let stage = create_bundle_dir(stage_parent, "stage", "staging")?;
    let result = (|| {
        let source_fingerprint =
            crate::commands::bulk_canonical::canonical_tree_fingerprint(state_dir)?;
        copy_state_tree(state_dir, &stage)?;
        fs::create_dir_all(stage.join("issues"))?;
        fs::create_dir_all(stage.join("evidence"))?;
        #[cfg(feature = "test-hooks")]
        crate::commands::bulk_canonical::test_pause_after_snapshot()?;
        let summary = apply_bundle_to_state(&stage, project_slug, bundle, plan)?;
        RecordIndex::load(&stage).map_err(|error| {
            anyhow::anyhow!(
                "Bundle apply rejected the staged canonical state: {error}\nNext: fix the reported workflow or graph violation in the bundle, then run bundle apply again with --yes"
            )
        })?;
        crate::commands::bulk_canonical::ensure_unchanged(state_dir, &source_fingerprint)?;
        install_bundle_stage(&stage, state_dir)?;
        Ok(summary)
    })();
    if let Err(error) = cleanup_bundle_stage(&stage) {
        if result.is_ok() {
            eprintln!(
                "Bundle apply committed successfully, but staging cleanup failed for {}: {}. Do not retry this bundle; verify with `atelier check`, then remove the leftover staging directory.",
                stage.display(),
                error
            );
        }
    }
    result
}

fn cleanup_bundle_stage(stage: &Path) -> Result<()> {
    #[cfg(feature = "test-hooks")]
    if std::env::var_os("ATELIER_TEST_BUNDLE_STAGE_CLEANUP_FAILURE").is_some() {
        bail!("injected bundle stage cleanup failure");
    }
    fs::remove_dir_all(stage).with_context(|| format!("Failed to remove {}", stage.display()))
}

fn ensure_no_bundle_recovery_artifacts(state_dir: &Path) -> Result<()> {
    let repo_root = state_dir
        .parent()
        .with_context(|| format!("Cannot determine parent for {}", state_dir.display()))?;
    let runtime = state_dir.join("runtime");
    let mut artifacts = Vec::new();
    collect_bundle_recovery_artifacts(repo_root, ".atelier-bundle-stage-", &mut artifacts)?;
    if runtime.exists() {
        collect_bundle_recovery_artifacts(&runtime, ".atelier-bundle-backup-", &mut artifacts)?;
    }
    artifacts.sort();
    if !artifacts.is_empty() {
        bail!(
            "bundle_recovery_required: leftover bundle recovery artifact(s): {}. Do not retry the bundle. Next: run `atelier check`, inspect the live records and each named real directory, then remove the artifact before applying a different bundle",
            artifacts
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(())
}

fn collect_bundle_recovery_artifacts(
    parent: &Path,
    prefix: &str,
    artifacts: &mut Vec<PathBuf>,
) -> Result<()> {
    let entries = fs::read_dir(parent)
        .with_context(|| format!("Failed to read {}", parent.display()))?
        .collect::<std::io::Result<Vec<_>>>()?
        .into_iter();
    for entry in entries {
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        if !name.starts_with(prefix) {
            continue;
        }
        let path = entry.path();
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to inspect {}", path.display()))?;
        if file_type.is_symlink() || !file_type.is_dir() {
            bail!(
                "bundle_recovery_artifact_unsafe: {} matches a bundle recovery name but is not a real directory. Next: inspect the path without following links and resolve it manually before running bundle apply",
                path.display()
            );
        }
        artifacts.push(path);
    }
    Ok(())
}

fn apply_bundle_to_state(
    state_dir: &Path,
    project_slug: &str,
    bundle: &BundleFile,
    plan: &BundleGraphPlan<'_>,
) -> Result<Value> {
    let mut resolved = BTreeMap::<String, ResolvedRef>::new();
    let mut created = BTreeMap::<String, Vec<Value>>::new();
    let store = RecordStore::for_project(state_dir, project_slug);

    for issue in &plan.issues {
        if let Some(id) = &issue.id {
            let mut record = store.load_issue_by_id(id)?;
            let sections = atelier_records::parse_issue_sections_for_type(
                &issue.body,
                Path::new("<bundle issue body>"),
                &issue.issue_type,
            )?;
            record.issue.title = issue.title.clone();
            record.issue.priority = issue.priority.clone();
            record.issue.strategy = issue.strategy.clone();
            record.issue.description = Some(issue.body.clone());
            record.issue.updated_at = chrono::Utc::now();
            record.labels = sorted(issue.labels.clone());
            record.sections.body = sections.body;
            check_bundle_record(&record)?;
            store.write_issue_atomic(&record)?;
            resolved.insert(
                issue.client_ref.clone(),
                ResolvedRef {
                    kind: "issue".to_string(),
                    id: id.clone(),
                },
            );
            created
                .entry("updated".to_string())
                .or_default()
                .push(json!({
                    "client_ref": issue.client_ref,
                    "id": id,
                    "title": issue.title,
                }));
            continue;
        }
        let description = Some(issue.body.clone());
        let now = chrono::Utc::now();
        let id = store.allocate_issue_id()?;
        let status = match &issue.status {
            Some(status) => status.clone(),
            None => workflow_initial_issue_status(&issue.issue_type)?,
        };
        let sections = atelier_records::parse_issue_sections_for_type(
            &issue.body,
            Path::new("<bundle issue body>"),
            &issue.issue_type,
        )?;
        let record = CanonicalIssueRecord {
            issue: Issue {
                id: id.clone(),
                title: issue.title.clone(),
                description,
                status: status.clone(),
                issue_type: issue.issue_type.clone(),
                priority: issue.priority.clone(),
                strategy: issue.strategy.clone(),
                workflow_branch: None,
                parent_id: None,
                created_at: now,
                updated_at: now,
                closed_at: None,
            },
            labels: sorted(issue.labels.clone()),
            sections,
            relationships: Relationships::default(),
        };
        check_bundle_record(&record)?;
        store.write_issue_atomic(&record)?;
        resolved.insert(
            issue.client_ref.clone(),
            ResolvedRef {
                kind: "issue".to_string(),
                id: id.clone(),
            },
        );
        created
            .entry("issues".to_string())
            .or_default()
            .push(json!({
                "client_ref": issue.client_ref,
                "id": id,
                "title": issue.title,
            }));
    }

    for evidence in &plan.evidence {
        let data = EvidenceRecordData {
            commit: evidence.commit.clone(),
            result: evidence
                .result
                .parse::<atelier_core::EvidenceResult>()
                .map_err(anyhow::Error::msg)?,
            evidence_type: evidence.evidence_type.clone(),
            captured_at: chrono::Utc::now(),
            command: None,
            path: evidence.artifact.clone(),
            uri: None,
            producer: None,
            proof_scope: None,
            agent_identity: None,
            independence_level: None,
            residual_risks: Vec::new(),
            follow_up_ids: Vec::new(),
            exit_code: None,
            exit_status: Some(evidence.result.clone()),
            success: None,
            spawn_error: None,
            output: None,
            target: None,
        };
        let record = store.create_evidence(&evidence.title, "recorded", &evidence.body, data)?;
        let id = record.header.id;
        resolved.insert(
            evidence.client_ref.clone(),
            ResolvedRef {
                kind: "evidence".to_string(),
                id: id.clone(),
            },
        );
        created
            .entry("evidence".to_string())
            .or_default()
            .push(json!({
                "client_ref": evidence.client_ref,
                "id": id,
                "title": evidence.title,
            }));
    }

    for note in &plan.notes {
        let target = resolve_planned_ref(&resolved, &note.target)?;
        let body = match &note.note.author {
            Some(author) if !author.trim().is_empty() => {
                format!("[{author}] {}", note.note.body)
            }
            _ => note.note.body.clone(),
        };
        let _ = &note.note.created_at;
        let mut record = store.load_issue_by_id(&target.id)?;
        record
            .sections
            .append_note(local_date(chrono::Utc::now()), "note", &body);
        store.write_issue_atomic(&record)?;
    }

    let mut applied_relationships = Vec::new();
    for relationship in &plan.relationships {
        let source = resolve_planned_ref(&resolved, &relationship.source)?;
        let target = resolve_planned_ref(&resolved, &relationship.target)?;
        apply_planned_relationship(
            &store,
            &source,
            &target,
            &relationship.role,
            relationship.declared_by_target,
        )?;
        applied_relationships.push(resolved_relationship_value(
            &source,
            &target,
            &relationship.role,
        ));
    }

    Ok(json!({
        "applied": true,
        "preview": false,
        "title": bundle.title,
        "description": bundle.description,
        "records": created,
        "relationships": applied_relationships,
        "notes": plan.notes.len(),
    }))
}

/// A record a bundle writes passes the per-record rules `atelier check`
/// applies, and a mission's strategy names an existing file.
fn check_bundle_record(record: &CanonicalIssueRecord) -> Result<()> {
    let repo_root = atelier_app::storage_layout::find_repo_root()?;
    let findings = atelier_app::lint::issue_findings([record], &repo_root, false);
    if let Some(finding) = findings.first() {
        bail!(
            "Bundle record {} ({}) is invalid: {}",
            record.issue.id,
            record.issue.title,
            finding.message
        );
    }
    if let Some(strategy) = &record.issue.strategy {
        if !repo_root.join(&strategy.path).is_file() {
            bail!(
                "Bundle mission {} links strategy {}, which does not exist",
                record.issue.id,
                strategy.path
            );
        }
    }
    Ok(())
}

fn copy_state_tree(source: &Path, dest: &Path) -> Result<()> {
    copy_tree(source, dest, true)
}

pub(crate) fn copy_dir_recursive(source: &Path, dest: &Path) -> Result<()> {
    copy_tree(source, dest, false)
}

fn copy_tree(source: &Path, dest: &Path, exclude_runtime: bool) -> Result<()> {
    fs::create_dir_all(dest).with_context(|| format!("Failed to create {}", dest.display()))?;
    for entry in
        fs::read_dir(source).with_context(|| format!("Failed to read {}", source.display()))?
    {
        let entry = entry?;
        if exclude_runtime && entry.file_name() == "runtime" {
            continue;
        }
        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to inspect {}", source_path.display()))?;
        if file_type.is_symlink() {
            bail!(
                "Canonical bulk mutation refuses symbolic link {}",
                source_path.display()
            );
        } else if file_type.is_dir() {
            copy_dir_recursive(&source_path, &dest_path)?;
        } else if file_type.is_file() {
            fs::copy(&source_path, &dest_path).with_context(|| {
                format!(
                    "Failed to copy {} to {}",
                    source_path.display(),
                    dest_path.display()
                )
            })?;
        } else {
            bail!(
                "Canonical bulk mutation refuses special filesystem entry {}",
                source_path.display()
            );
        }
    }
    Ok(())
}

fn install_bundle_stage(stage: &Path, state_dir: &Path) -> Result<()> {
    install_bundle_stage_with(stage, state_dir, |_| Ok(()))
}

fn install_bundle_stage_with(
    stage: &Path,
    state_dir: &Path,
    mut before_install: impl FnMut(&str) -> Result<()>,
) -> Result<()> {
    install_directories(
        stage,
        state_dir,
        &["issues", "evidence"],
        &mut before_install,
    )
}

pub(crate) fn install_directories(
    stage: &Path,
    state_dir: &Path,
    names: &[&str],
    before_install: &mut impl FnMut(&str) -> Result<()>,
) -> Result<()> {
    let backup_parent = state_dir.join("runtime");
    fs::create_dir_all(&backup_parent)
        .with_context(|| format!("Failed to create {}", backup_parent.display()))?;
    let backup = create_bundle_dir(&backup_parent, "backup", "backup")?;
    let result = (|| {
        for name in names {
            crate::commands::bulk_canonical::validate_directory_path(&stage.join(name), true)?;
            crate::commands::bulk_canonical::validate_directory_path(&state_dir.join(name), false)?;
        }

        let mut prepared = Vec::new();
        for name in names {
            let destination = state_dir.join(name);
            if destination.exists() {
                if let Err(error) = fs::rename(&destination, backup.join(name)) {
                    restore_bundle_backups(&backup, state_dir, &prepared)?;
                    return Err(error).with_context(|| {
                        format!(
                            "Failed to prepare bundle install for {}",
                            destination.display()
                        )
                    });
                }
                prepared.push(name);
            }
        }

        let mut installed = Vec::new();
        for name in names {
            let staged = stage.join(name);
            let destination = state_dir.join(name);
            let install_result = before_install(name)
                .and_then(|()| fs::rename(&staged, &destination).map_err(anyhow::Error::from));
            if let Err(error) = install_result {
                for installed_name in installed.iter().rev() {
                    let installed_path = state_dir.join(installed_name);
                    if installed_path.exists() {
                        fs::remove_dir_all(&installed_path).with_context(|| {
                            format!(
                                "Failed to remove partially installed {} during rollback",
                                installed_path.display()
                            )
                        })?;
                    }
                }
                restore_bundle_backups(&backup, state_dir, &prepared)?;
                return Err(error).with_context(|| {
                    format!(
                        "Failed to install staged bundle at {}",
                        destination.display()
                    )
                });
            }
            installed.push(name);
        }
        Ok(())
    })();
    if result.is_ok() {
        if let Err(error) = cleanup_bundle_backup(&backup) {
            eprintln!(
                "Bundle apply committed successfully, but backup cleanup failed for {}: {}. Do not retry this bundle. Verify with `atelier check`, then remove the named backup; future bundle applies will refuse while it remains.",
                backup.display(),
                error
            );
        }
    } else if backup.exists() && fs::read_dir(&backup)?.next().is_none() {
        let _ = fs::remove_dir(&backup);
    }
    result
}

fn cleanup_bundle_backup(backup: &Path) -> Result<()> {
    #[cfg(feature = "test-hooks")]
    if std::env::var_os("ATELIER_TEST_BUNDLE_BACKUP_CLEANUP_FAILURE").is_some() {
        bail!("injected bundle backup cleanup failure");
    }
    fs::remove_dir_all(backup).with_context(|| format!("Failed to remove {}", backup.display()))
}

fn create_bundle_dir(parent: &Path, kind: &str, label: &str) -> Result<PathBuf> {
    let base = format!(
        ".atelier-bundle-{kind}-{}-{}",
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    );
    allocate_bundle_dir(parent, &base, label)
}

fn allocate_bundle_dir(parent: &Path, base: &str, label: &str) -> Result<PathBuf> {
    for suffix in 0..=99 {
        let candidate = if suffix == 0 {
            parent.join(base)
        } else {
            parent.join(format!("{base}-{suffix:02}"))
        };
        match fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("Failed to create {}", candidate.display()))
            }
        }
    }
    bail!(
        "Failed to allocate bundle {label} directory in {}",
        parent.display()
    )
}

fn restore_bundle_backups(backup: &Path, state_dir: &Path, names: &[&str]) -> Result<()> {
    for name in names.iter().rev() {
        let source = backup.join(name);
        let destination = state_dir.join(name);
        if source.exists() {
            fs::rename(&source, &destination).with_context(|| {
                format!(
                    "Failed to restore {} from {} after bundle install failure",
                    destination.display(),
                    source.display()
                )
            })?;
        }
    }
    Ok(())
}

impl BundleFile {
    fn client_refs(&self) -> Vec<(String, &'static str)> {
        let mut refs = Vec::new();
        refs.extend(
            self.resources
                .issues
                .iter()
                .map(|record| (record.client_ref.clone(), "issue")),
        );
        refs.extend(
            self.resources
                .evidence
                .iter()
                .map(|record| (record.client_ref.clone(), "evidence")),
        );
        refs
    }

    fn client_kind(&self, client_ref: &str) -> Option<&'static str> {
        self.client_refs()
            .into_iter()
            .find_map(|(candidate, kind)| (candidate == client_ref).then_some(kind))
    }
}

fn validate_client_ref(client_ref: &str) -> Result<()> {
    let mut chars = client_ref.chars();
    let Some(first) = chars.next() else {
        bail!("client_ref cannot be empty");
    };
    if !first.is_ascii_alphabetic() {
        bail!(
            "client_ref '{}' must start with an ASCII letter",
            client_ref
        );
    }
    if !chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | ':' | '-')) {
        bail!("client_ref '{}' contains an invalid character", client_ref);
    }
    Ok(())
}

fn validate_refs_exist<'a>(
    index: &RecordIndex,
    bundle: &BundleFile,
    refs: impl Iterator<Item = &'a BundleRef>,
    owner: &str,
) -> Result<()> {
    for reference in refs {
        validate_ref_exists(index, bundle, reference, owner)?;
    }
    Ok(())
}

fn validate_ref_exists(
    index: &RecordIndex,
    bundle: &BundleFile,
    reference: &BundleRef,
    owner: &str,
) -> Result<()> {
    match (&reference.client_ref, &reference.id) {
        (Some(client_ref), None) => {
            if bundle.client_kind(client_ref).is_none() {
                bail!(
                    "Reference '{}' for {} does not resolve in this file",
                    client_ref,
                    owner
                );
            }
        }
        (None, Some(id)) => {
            resolve_existing_ref(index, id)
                .with_context(|| format!("Reference id '{}' for {} does not resolve", id, owner))?;
        }
        _ => bail!(
            "Reference for {} must contain exactly one of client_ref or id",
            owner
        ),
    }
    Ok(())
}

fn resolve_existing_ref(index: &RecordIndex, id: &str) -> Result<ResolvedRef> {
    if let Some(issue_id) = index.resolve_issue_ref(id)? {
        return Ok(ResolvedRef {
            kind: "issue".to_string(),
            id: issue_id,
        });
    }
    if index.evidence(id).is_some() {
        return Ok(ResolvedRef {
            kind: "evidence".to_string(),
            id: id.to_string(),
        });
    }
    bail!("Record id '{}' not found", id)
}

fn planned_client_ref(bundle: &BundleFile, client_ref: &str) -> Result<PlannedRef> {
    let kind = bundle
        .client_kind(client_ref)
        .with_context(|| format!("Reference '{client_ref}' does not resolve in this file"))?;
    Ok(PlannedRef {
        kind: kind.to_string(),
        reference: PlannedReference::ClientRef(client_ref.to_string()),
    })
}

fn planned_bundle_ref(
    index: &RecordIndex,
    bundle: &BundleFile,
    reference: &BundleRef,
) -> Result<PlannedRef> {
    match (&reference.client_ref, &reference.id) {
        (Some(client_ref), None) => planned_client_ref(bundle, client_ref),
        (None, Some(id)) => {
            let resolved = resolve_existing_ref(index, id)?;
            Ok(PlannedRef {
                kind: resolved.kind,
                reference: PlannedReference::Id(resolved.id),
            })
        }
        _ => bail!("Reference must contain exactly one of client_ref or id"),
    }
}

fn resolve_planned_ref(
    resolved: &BTreeMap<String, ResolvedRef>,
    reference: &PlannedRef,
) -> Result<ResolvedRef> {
    match &reference.reference {
        PlannedReference::ClientRef(client_ref) => resolved
            .get(client_ref)
            .cloned()
            .with_context(|| format!("Reference '{client_ref}' has not been allocated")),
        PlannedReference::Id(id) => Ok(ResolvedRef {
            kind: reference.kind.clone(),
            id: id.clone(),
        }),
    }
}

fn apply_planned_relationship(
    store: &RecordStore,
    source: &ResolvedRef,
    target: &ResolvedRef,
    role: &str,
    declared_by_target: bool,
) -> Result<()> {
    match role {
        "parent" => {
            if source.kind != "issue" || target.kind != "issue" {
                bail!("parent relationship endpoints must both be issues");
            }
            store.add_issue_child(&source.id, &target.id)?;
        }
        "blocks" => {
            if source.kind != "issue" || target.kind != "issue" {
                bail!("blocks relationship endpoints must both be issues");
            }
            store.add_issue_block(&target.id, &source.id, !declared_by_target)?;
        }
        // Stored on the declaring record only, as `atelier issue link` does.
        "advances" if declared_by_target => {
            store.add_issue_relation(&target.id, &source.id, "advances")?;
        }
        "advances" => {
            store.add_issue_relation(&source.id, &target.id, "advances")?;
        }
        relation_type => add_relationship(store, source, target, relation_type)?,
    }
    Ok(())
}

fn add_relationship(
    store: &RecordStore,
    source: &ResolvedRef,
    target: &ResolvedRef,
    relation_type: &str,
) -> Result<()> {
    store.add_record_relationship(
        &source.kind,
        &source.id,
        &target.kind,
        &target.id,
        relation_type,
    )?;
    Ok(())
}

fn preview_bundle_summary(bundle: &BundleFile, plan: &BundleGraphPlan<'_>) -> Value {
    let mut records = BTreeMap::<String, Vec<Value>>::new();
    for issue in &bundle.resources.issues {
        let key = if issue.id.is_some() {
            "updated"
        } else {
            "issues"
        };
        records.entry(key.to_string()).or_default().push(json!({
            "client_ref": issue.client_ref,
            "kind": "issue",
            "id": issue.id,
            "title": issue.title,
        }));
    }
    for evidence in &bundle.resources.evidence {
        records
            .entry("evidence".to_string())
            .or_default()
            .push(json!({
                "client_ref": evidence.client_ref,
                "kind": "evidence",
                "title": evidence.title,
            }));
    }
    for records in records.values_mut() {
        records.sort_by(|left, right| {
            left["client_ref"]
                .as_str()
                .cmp(&right["client_ref"].as_str())
        });
    }
    json!({
        "applied": false,
        "preview": true,
        "title": bundle.title,
        "description": bundle.description,
        "records": records,
        "relationships": plan.relationships.iter().map(planned_relationship_value).collect::<Vec<_>>(),
        "notes": plan.notes.len(),
    })
}

fn planned_relationship_value(relationship: &PlannedRelationship) -> Value {
    json!({
        "source": planned_ref_value(&relationship.source),
        "target": planned_ref_value(&relationship.target),
        "role": relationship.role,
    })
}

fn planned_ref_value(reference: &PlannedRef) -> Value {
    match &reference.reference {
        PlannedReference::ClientRef(client_ref) => {
            json!({ "kind": reference.kind, "client_ref": client_ref })
        }
        PlannedReference::Id(id) => json!({ "kind": reference.kind, "id": id }),
    }
}

fn resolved_relationship_value(source: &ResolvedRef, target: &ResolvedRef, role: &str) -> Value {
    json!({
        "source": { "kind": source.kind, "id": source.id },
        "target": { "kind": target.kind, "id": target.id },
        "role": role,
    })
}

fn planned_ref_label(reference: &PlannedRef) -> String {
    match &reference.reference {
        PlannedReference::ClientRef(client_ref) => format!("{}/{client_ref}", reference.kind),
        PlannedReference::Id(id) => format!("{}/{id}", reference.kind),
    }
}

fn print_bundle_summary(summary: Value) -> Result<()> {
    let preview = summary["preview"].as_bool().unwrap_or(false);
    if preview {
        println!("Bundle preview is valid.");
    } else {
        println!("Bundle applied.");
    }

    println!(
        "Applied:       {}",
        summary["applied"].as_bool().unwrap_or(false)
    );
    println!("Preview:       {preview}");

    if let Some(records) = summary["records"].as_object() {
        println!();
        println!("Records");
        println!("-------");
        for key in ["issues", "updated", "evidence"] {
            let count = records
                .get(key)
                .and_then(|value| value.as_array())
                .map(|items| items.len())
                .unwrap_or(0);
            println!("  {key}: {count}");
        }
    }

    let relationship_count = summary["relationships"]
        .as_array()
        .map(|relationships| relationships.len())
        .or_else(|| {
            summary["relationships"]
                .as_u64()
                .map(|count| count as usize)
        })
        .unwrap_or(0);
    println!("  relationships: {relationship_count}");
    println!("  notes: {}", summary["notes"].as_u64().unwrap_or_default());

    if let Some(records) = summary["records"].as_object() {
        let rows = ["issues", "updated", "evidence"]
            .into_iter()
            .flat_map(|key| {
                records
                    .get(key)
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .map(move |record| (key, record))
            })
            .collect::<Vec<_>>();
        if !rows.is_empty() {
            println!();
            println!("Client References");
            println!("-----------------");
            for (key, record) in rows {
                let client_ref = record["client_ref"].as_str().unwrap_or("unknown");
                let title = record["title"].as_str().unwrap_or_default();
                let target = match (record["id"].as_str(), key) {
                    (Some(id), "updated") => format!("{id} (updated)"),
                    (Some(id), _) => id.to_string(),
                    (None, _) if preview => "(new)".to_string(),
                    (None, _) => "unknown".to_string(),
                };
                println!("  {client_ref} -> {target}  {title}");
            }
        }
    }

    if let Some(relationships) = summary["relationships"].as_array() {
        if !relationships.is_empty() {
            println!();
            println!("Relationships");
            println!("-------------");
            for relationship in relationships {
                let source = summary_ref_label(&relationship["source"]);
                let target = summary_ref_label(&relationship["target"]);
                let role = relationship["role"].as_str().unwrap_or("unknown");
                println!("  {source} -> {target} ({role})");
            }
        }
    }

    if let Some(id) = first_created_id(&summary, "issues") {
        println!();
        println!("Next Commands");
        println!("-------------");
        println!("  atelier issue show {id}");
        println!("  atelier check");
    } else if !preview {
        println!();
        println!("Next Commands");
        println!("-------------");
        println!("  atelier check");
    }
    Ok(())
}

fn summary_ref_label(reference: &Value) -> String {
    let kind = reference["kind"].as_str().unwrap_or("record");
    let identifier = reference["client_ref"]
        .as_str()
        .or_else(|| reference["id"].as_str())
        .unwrap_or("unknown");
    format!("{kind}/{identifier}")
}

fn first_created_id<'a>(summary: &'a Value, kind: &str) -> Option<&'a str> {
    summary["records"][kind]
        .as_array()
        .and_then(|items| items.first())
        .and_then(|item| item["id"].as_str())
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

fn default_issue_type() -> String {
    "task".to_string()
}

fn default_priority() -> String {
    "medium".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write_tree(root: &Path, issues: &str, evidence: &str) {
        fs::create_dir_all(root.join("issues")).unwrap();
        fs::create_dir_all(root.join("evidence")).unwrap();
        fs::write(root.join("issues/sentinel.md"), issues).unwrap();
        fs::write(root.join("evidence/sentinel.md"), evidence).unwrap();
    }

    #[test]
    fn canonical_copy_excludes_only_top_level_runtime() {
        let dir = tempdir().unwrap();
        let source = dir.path().join("source");
        fs::create_dir_all(source.join("runtime")).unwrap();
        fs::write(source.join("runtime/derived"), "derived").unwrap();
        fs::create_dir_all(source.join("unknown/runtime/empty")).unwrap();
        fs::write(source.join("unknown/runtime/raw"), [0, 255, 10]).unwrap();

        let canonical = dir.path().join("canonical");
        copy_state_tree(&source, &canonical).unwrap();
        assert!(!canonical.join("runtime").exists());
        assert!(canonical.join("unknown/runtime/empty").is_dir());
        assert_eq!(
            fs::read(canonical.join("unknown/runtime/raw")).unwrap(),
            [0, 255, 10]
        );

        let recursive = dir.path().join("recursive");
        copy_dir_recursive(&source, &recursive).unwrap();
        assert_eq!(
            fs::read_to_string(recursive.join("runtime/derived")).unwrap(),
            "derived"
        );
    }

    #[cfg(unix)]
    #[test]
    fn canonical_copy_preserves_file_modes_and_rejects_unsafe_entries() {
        use std::os::unix::fs::{symlink, PermissionsExt};

        let dir = tempdir().unwrap();
        let source = dir.path().join("source");
        fs::create_dir_all(&source).unwrap();
        let file = source.join("executable");
        fs::write(&file, "#!/bin/sh\n").unwrap();
        fs::set_permissions(&file, fs::Permissions::from_mode(0o751)).unwrap();
        let copied = dir.path().join("copied");
        copy_state_tree(&source, &copied).unwrap();
        assert_eq!(
            fs::metadata(copied.join("executable"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o751
        );

        let outside = dir.path().join("outside");
        fs::write(&outside, "untouched").unwrap();
        let unsafe_path = source.join("unsafe");
        symlink(&outside, &unsafe_path).unwrap();
        for copy in [copy_state_tree, copy_dir_recursive] {
            let error = copy(&source, &copied).unwrap_err();
            assert_eq!(
                error.to_string(),
                format!(
                    "Canonical bulk mutation refuses symbolic link {}",
                    unsafe_path.display()
                )
            );
        }
        fs::remove_file(&unsafe_path).unwrap();
        assert!(std::process::Command::new("mkfifo")
            .arg(&unsafe_path)
            .status()
            .unwrap()
            .success());
        for copy in [copy_state_tree, copy_dir_recursive] {
            let error = copy(&source, &copied).unwrap_err();
            assert_eq!(
                error.to_string(),
                format!(
                    "Canonical bulk mutation refuses special filesystem entry {}",
                    unsafe_path.display()
                )
            );
        }
        assert_eq!(fs::read_to_string(outside).unwrap(), "untouched");
    }

    #[test]
    fn bundle_allocator_exhaustion_preserves_every_collision_and_error_label() {
        let dir = tempdir().unwrap();
        for suffix in 0..=99 {
            let name = if suffix == 0 {
                "occupied".to_string()
            } else {
                format!("occupied-{suffix:02}")
            };
            fs::write(dir.path().join(name), "untouched").unwrap();
        }
        for label in ["backup", "staging"] {
            let error = allocate_bundle_dir(dir.path(), "occupied", label).unwrap_err();
            assert_eq!(
                error.to_string(),
                format!(
                    "Failed to allocate bundle {label} directory in {}",
                    dir.path().display()
                )
            );
        }
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 100);
        for entry in fs::read_dir(dir.path()).unwrap() {
            assert_eq!(
                fs::read_to_string(entry.unwrap().path()).unwrap(),
                "untouched"
            );
        }
    }

    #[test]
    fn bundle_allocator_preserves_names_and_reports_noncollision_errors() {
        let dir = tempdir().unwrap();
        for (kind, label) in [("stage", "staging"), ("backup", "backup")] {
            let allocated = create_bundle_dir(dir.path(), kind, label).unwrap();
            assert!(allocated.is_dir());
            assert!(allocated
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(&format!(".atelier-bundle-{kind}-{}-", std::process::id())));
        }
        let parent = dir.path().join("file");
        fs::write(&parent, "untouched").unwrap();
        let error = allocate_bundle_dir(&parent, "candidate", "staging").unwrap_err();
        assert_eq!(
            error.to_string(),
            format!("Failed to create {}", parent.join("candidate").display())
        );
        assert_eq!(fs::read_to_string(parent).unwrap(), "untouched");
    }

    #[test]
    fn canonical_copy_preserves_create_and_read_error_contexts() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("missing");
        let dest = dir.path().join("dest");
        for copy in [copy_state_tree, copy_dir_recursive] {
            assert_eq!(
                copy(&missing, &dest).unwrap_err().to_string(),
                format!("Failed to read {}", missing.display())
            );
        }
        let file = dir.path().join("file");
        fs::write(&file, "untouched").unwrap();
        for copy in [copy_state_tree, copy_dir_recursive] {
            assert_eq!(
                copy(&dir.path().join("source"), &file)
                    .unwrap_err()
                    .to_string(),
                format!("Failed to create {}", file.display())
            );
        }
        assert_eq!(fs::read_to_string(file).unwrap(), "untouched");
    }

    #[test]
    fn bundle_issue_uses_typed_markdown_body() {
        let issue = serde_json::from_value::<BundleIssue>(serde_json::json!({ "client_ref": "future.parent", "title": "Future outcome", "issue_type": "feature", "body": "## Goal\n\nA meaningful outcome.\n\n## Done when\n\nEvery child is done.\n\n## Unknowns\n\nOne open question." })).unwrap();
        let sections = atelier_records::parse_issue_sections_for_type(
            &issue.body,
            Path::new("<bundle>"),
            &issue.issue_type,
        )
        .unwrap();
        assert!(matches!(
            sections.body,
            atelier_core::IssueBody::Issue {
                unknowns: Some(_),
                ..
            }
        ));
    }

    #[test]
    fn second_directory_install_failure_restores_both_live_directories() {
        let dir = tempdir().unwrap();
        let state = dir.path().join(".atelier");
        let stage = dir.path().join("stage");
        write_tree(&state, "old issues", "old evidence");
        write_tree(&stage, "new issues", "new evidence");

        let error = install_bundle_stage_with(&stage, &state, |name| {
            if name == "evidence" {
                bail!("injected second-directory install failure");
            }
            Ok(())
        })
        .unwrap_err();

        assert!(format!("{error:#}").contains("second-directory"));
        assert_eq!(
            fs::read_to_string(state.join("issues/sentinel.md")).unwrap(),
            "old issues"
        );
        assert_eq!(
            fs::read_to_string(state.join("evidence/sentinel.md")).unwrap(),
            "old evidence"
        );
        assert!(!fs::read_to_string(state.join("issues/sentinel.md"))
            .unwrap()
            .contains("new"));
        assert!(!fs::read_dir(state.join("runtime"))
            .unwrap()
            .filter_map(|entry| entry.ok())
            .any(|entry| entry
                .file_name()
                .to_string_lossy()
                .starts_with(".atelier-bundle-backup-")));
    }

    #[cfg(unix)]
    #[test]
    fn bundle_install_rejects_symlink_and_special_file_destinations_without_touching_targets() {
        use std::os::unix::fs::symlink;

        for unsafe_kind in ["symlink", "file"] {
            let dir = tempdir().unwrap();
            let state = dir.path().join(".atelier");
            let stage = dir.path().join("stage");
            fs::create_dir_all(&state).unwrap();
            write_tree(&stage, "new issues", "new evidence");
            fs::create_dir_all(state.join("evidence")).unwrap();
            fs::write(state.join("evidence/sentinel.md"), "old evidence").unwrap();
            let outside = dir.path().join("outside");
            if unsafe_kind == "symlink" {
                fs::create_dir_all(&outside).unwrap();
                fs::write(outside.join("sentinel.md"), "outside").unwrap();
                symlink(&outside, state.join("issues")).unwrap();
            } else {
                fs::write(state.join("issues"), "not a directory").unwrap();
            }

            let error = install_bundle_stage(&stage, &state).unwrap_err();
            assert!(
                error.to_string().contains("real directory")
                    || error.to_string().contains("symbolic-link"),
                "{error:#}"
            );
            assert_eq!(
                fs::read_to_string(state.join("evidence/sentinel.md")).unwrap(),
                "old evidence"
            );
            if unsafe_kind == "symlink" {
                assert_eq!(
                    fs::read_to_string(outside.join("sentinel.md")).unwrap(),
                    "outside"
                );
            } else {
                assert_eq!(
                    fs::read_to_string(state.join("issues")).unwrap(),
                    "not a directory"
                );
            }
        }
    }

    #[cfg(unix)]
    #[test]
    fn bundle_backup_allocator_never_reuses_or_removes_unsafe_collisions() {
        use std::os::unix::fs::symlink;

        let dir = tempdir().unwrap();
        let outside = dir.path().join("outside");
        fs::create_dir_all(&outside).unwrap();
        fs::write(outside.join("sentinel"), "untouched").unwrap();
        symlink(&outside, dir.path().join("backup-test")).unwrap();
        fs::write(dir.path().join("backup-test-01"), "special collision").unwrap();

        let allocated = allocate_bundle_dir(dir.path(), "backup-test", "backup").unwrap();

        assert_eq!(allocated, dir.path().join("backup-test-02"));
        assert!(allocated.is_dir());
        assert_eq!(
            fs::read_to_string(outside.join("sentinel")).unwrap(),
            "untouched"
        );
        assert_eq!(
            fs::read_to_string(dir.path().join("backup-test-01")).unwrap(),
            "special collision"
        );
    }

    #[cfg(unix)]
    #[test]
    fn bundle_recovery_scan_rejects_unsafe_stage_and_backup_types() {
        use std::os::unix::fs::symlink;

        let dir = tempdir().unwrap();
        let state = dir.path().join(".atelier");
        fs::create_dir_all(state.join("runtime")).unwrap();
        let outside = dir.path().join("outside");
        fs::create_dir_all(&outside).unwrap();
        fs::write(outside.join("sentinel"), "untouched").unwrap();
        let stage = dir.path().join(".atelier-bundle-stage-unsafe");
        symlink(&outside, &stage).unwrap();

        let error = ensure_no_bundle_recovery_artifacts(&state).unwrap_err();
        assert!(error
            .to_string()
            .contains("bundle_recovery_artifact_unsafe"));
        assert!(error.to_string().contains(&stage.display().to_string()));
        assert_eq!(
            fs::read_to_string(outside.join("sentinel")).unwrap(),
            "untouched"
        );

        fs::remove_file(stage).unwrap();
        let backup = state.join("runtime").join(".atelier-bundle-backup-unsafe");
        fs::write(&backup, "not a directory").unwrap();
        let error = ensure_no_bundle_recovery_artifacts(&state).unwrap_err();
        assert!(error
            .to_string()
            .contains("bundle_recovery_artifact_unsafe"));
        assert!(error.to_string().contains(&backup.display().to_string()));
        assert_eq!(fs::read_to_string(backup).unwrap(), "not a directory");
    }
}
