//! Canonical mission-plan revision and review-state projection.
//!
//! Review events remain append-only issue activity sidecars. This module owns
//! their typed payload, the versioned material-plan digest, and deterministic
//! projection. It intentionally does not use the issue `review` field because
//! plan approval grants no code-review or merge authority.

use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use icu_normalizer::ComposingNormalizerBorrowed;
use icu_properties::props::DefaultIgnorableCodePoint;
use icu_properties::CodePointSetData;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use crate::activity::{
    list_issue_activities, validate_activity_timestamp, ActivityEventType, IssueActivity,
};
use crate::{CanonicalIssueRecord, RecordStore};

pub const MISSION_GRAPH_REVISION_VERSION: &str = "mission-graph-v2";
pub const STABLE_ACTOR_IDENTITY_VERSION: &str = "actor-v1";
pub const LEGACY_GRANDFATHER_STATUS: &str = "in_progress";
pub const LEGACY_GRANDFATHER_MIGRATION_ID: &str = "independent-mission-plan-review-v1";
pub const LEGACY_GRANDFATHER_MIGRATION_ACTOR: &str =
    "actor-v1:atelier.local/mission-review-migration";
pub const MISSION_PLAN_REVIEW_CUTOVER_MANIFEST_PATH: &str = "mission-plan-review-cutover.yaml";
pub const MISSION_PLAN_REVIEW_CUTOVER_SCHEMA: &str = "atelier.mission-plan-review-cutover";
pub const MISSION_PLAN_REVIEW_CUTOVER_SCHEMA_VERSION: u32 = 1;
pub const LEGACY_GRANDFATHER_RECEIPT_VERSION: &str = "mission-plan-cutover-receipt-v1";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionPlanReviewCutoverManifest {
    pub schema: String,
    pub schema_version: u32,
    pub migration_id: String,
    pub cutover_at: DateTime<Utc>,
    pub eligible_missions: Vec<LegacyGrandfatherEligibility>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyGrandfatherEligibility {
    pub mission_id: String,
    pub graph_revision: MissionGraphRevision,
    pub legacy_status: String,
    pub receipt_activity_id: String,
}

#[derive(Serialize)]
struct LegacyGrandfatherReceiptPayload<'a> {
    version: &'static str,
    migration_id: &'a str,
    cutover_at: DateTime<Utc>,
    mission_id: &'a str,
    graph_revision: &'a MissionGraphRevision,
    legacy_status: &'a str,
    receipt_activity_id: &'a str,
}

impl MissionPlanReviewCutoverManifest {
    pub fn validate(&self) -> Result<()> {
        if self.schema != MISSION_PLAN_REVIEW_CUTOVER_SCHEMA {
            bail!("mission-plan cutover schema must be '{MISSION_PLAN_REVIEW_CUTOVER_SCHEMA}'");
        }
        if self.schema_version != MISSION_PLAN_REVIEW_CUTOVER_SCHEMA_VERSION {
            bail!(
                "mission-plan cutover schema_version must be {MISSION_PLAN_REVIEW_CUTOVER_SCHEMA_VERSION}"
            );
        }
        if self.migration_id != LEGACY_GRANDFATHER_MIGRATION_ID {
            bail!("mission-plan cutover migration_id must be '{LEGACY_GRANDFATHER_MIGRATION_ID}'");
        }
        validate_activity_timestamp(self.cutover_at).context(
            "mission-plan cutover cutover_at must use canonical activity timestamp precision",
        )?;
        if self.eligible_missions.is_empty() {
            bail!("mission-plan cutover manifest must contain at least one eligible mission");
        }
        if self
            .eligible_missions
            .windows(2)
            .any(|pair| pair[0].mission_id >= pair[1].mission_id)
        {
            bail!(
                "mission-plan cutover eligible_missions must be sorted by mission_id with no duplicates"
            );
        }
        let expected_activity_id = crate::activity::timestamp_activity_id(self.cutover_at);
        for eligibility in &self.eligible_missions {
            crate::record_id::validate_record_id(&eligibility.mission_id).with_context(|| {
                format!(
                    "Invalid mission-plan cutover mission_id {}",
                    eligibility.mission_id
                )
            })?;
            eligibility.graph_revision.validate()?;
            if eligibility.legacy_status != LEGACY_GRANDFATHER_STATUS {
                bail!(
                    "mission-plan cutover legacy_status for {} must be '{LEGACY_GRANDFATHER_STATUS}'",
                    eligibility.mission_id
                );
            }
            if eligibility.receipt_activity_id != expected_activity_id {
                bail!(
                    "mission-plan cutover receipt_activity_id for {} must be '{}' derived from cutover_at",
                    eligibility.mission_id,
                    expected_activity_id
                );
            }
        }
        Ok(())
    }

    pub fn eligibility_for(&self, mission_id: &str) -> Option<&LegacyGrandfatherEligibility> {
        self.eligible_missions
            .binary_search_by(|entry| entry.mission_id.as_str().cmp(mission_id))
            .ok()
            .map(|index| &self.eligible_missions[index])
    }

    pub fn receipt_for(&self, eligibility: &LegacyGrandfatherEligibility) -> Result<String> {
        self.validate()?;
        let payload = LegacyGrandfatherReceiptPayload {
            version: LEGACY_GRANDFATHER_RECEIPT_VERSION,
            migration_id: &self.migration_id,
            cutover_at: self.cutover_at,
            mission_id: &eligibility.mission_id,
            graph_revision: &eligibility.graph_revision,
            legacy_status: &eligibility.legacy_status,
            receipt_activity_id: &eligibility.receipt_activity_id,
        };
        let bytes = serde_json::to_vec(&payload)
            .context("Failed to render mission-plan cutover receipt")?;
        let digest = Sha256::digest(bytes);
        Ok(format!(
            "{LEGACY_GRANDFATHER_RECEIPT_VERSION}:sha256:{}",
            lower_hex(&digest)
        ))
    }
}

pub fn load_mission_plan_review_cutover_manifest(
    state_dir: &Path,
) -> Result<Option<MissionPlanReviewCutoverManifest>> {
    let path = state_dir.join(MISSION_PLAN_REVIEW_CUTOVER_MANIFEST_PATH);
    if !path.exists() {
        return Ok(None);
    }
    let text =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
    let manifest: MissionPlanReviewCutoverManifest =
        serde_yaml::from_str(&text).with_context(|| format!("Invalid {}", path.display()))?;
    manifest
        .validate()
        .with_context(|| format!("Invalid {}", path.display()))?;
    Ok(Some(manifest))
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MissionGraphRevision(pub String);

impl MissionGraphRevision {
    pub fn validate(&self) -> Result<()> {
        let prefix = format!("{MISSION_GRAPH_REVISION_VERSION}:sha256:");
        let Some(digest) = self.0.strip_prefix(&prefix) else {
            bail!(
                "graph revision '{}' must start with '{}'; unsupported revision algorithm",
                self.0,
                prefix
            );
        };
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            bail!("graph revision '{}' has an invalid SHA-256 digest", self.0);
        }
        Ok(())
    }
}

impl std::fmt::Display for MissionGraphRevision {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionPlanFindingSeverity {
    Blocking,
    NonBlocking,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum MissionPlanReviewEvent {
    Request {
        graph_revision: MissionGraphRevision,
        authors: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        material_editors: Vec<String>,
    },
    MaterialEditAttribution {
        previous_graph_revision: MissionGraphRevision,
        graph_revision: MissionGraphRevision,
        editors: Vec<String>,
    },
    Finding {
        graph_revision: MissionGraphRevision,
        finding_id: String,
        severity: MissionPlanFindingSeverity,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        affected_issue_ids: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        dependency_path: Vec<String>,
    },
    ChangeRequest {
        graph_revision: MissionGraphRevision,
        request_id: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        affected_issue_ids: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        dependency_path: Vec<String>,
    },
    Resolution {
        graph_revision: MissionGraphRevision,
        target_id: String,
        disposition: String,
    },
    Approval {
        graph_revision: MissionGraphRevision,
    },
    LegacyGrandfather {
        graph_revision: MissionGraphRevision,
        legacy_status: String,
        migration_id: String,
        cutover_receipt: String,
    },
}

impl MissionPlanReviewEvent {
    pub fn graph_revision(&self) -> &MissionGraphRevision {
        match self {
            Self::Request { graph_revision, .. }
            | Self::MaterialEditAttribution { graph_revision, .. }
            | Self::Finding { graph_revision, .. }
            | Self::ChangeRequest { graph_revision, .. }
            | Self::Resolution { graph_revision, .. }
            | Self::Approval { graph_revision }
            | Self::LegacyGrandfather { graph_revision, .. } => graph_revision,
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.graph_revision().validate()?;
        match self {
            Self::Request {
                authors,
                material_editors,
                ..
            } => {
                validate_actor_list("authors", authors, false)?;
                validate_actor_list("material_editors", material_editors, true)
            }
            Self::MaterialEditAttribution {
                previous_graph_revision,
                graph_revision,
                editors,
            } => {
                previous_graph_revision.validate()?;
                if previous_graph_revision == graph_revision {
                    bail!("material-edit attribution must change the graph revision");
                }
                validate_actor_list("editors", editors, false)
            }
            Self::Finding {
                finding_id,
                affected_issue_ids,
                dependency_path,
                ..
            } => {
                validate_event_id("finding_id", finding_id)?;
                validate_sorted_unique("affected_issue_ids", affected_issue_ids)?;
                validate_path("dependency_path", dependency_path)
            }
            Self::ChangeRequest {
                request_id,
                affected_issue_ids,
                dependency_path,
                ..
            } => {
                validate_event_id("request_id", request_id)?;
                validate_sorted_unique("affected_issue_ids", affected_issue_ids)?;
                validate_path("dependency_path", dependency_path)
            }
            Self::Resolution {
                target_id,
                disposition,
                ..
            } => {
                validate_event_id("target_id", target_id)?;
                validate_nonempty("disposition", disposition)
            }
            Self::Approval { .. } => Ok(()),
            Self::LegacyGrandfather {
                legacy_status,
                migration_id,
                cutover_receipt,
                ..
            } => {
                if legacy_status != LEGACY_GRANDFATHER_STATUS {
                    bail!(
                        "legacy_status must be '{LEGACY_GRANDFATHER_STATUS}' for mission-plan grandfathering"
                    );
                }
                if migration_id != LEGACY_GRANDFATHER_MIGRATION_ID {
                    bail!(
                        "migration_id must be '{LEGACY_GRANDFATHER_MIGRATION_ID}' for mission-plan grandfathering"
                    );
                }
                validate_versioned_sha256(
                    "cutover_receipt",
                    cutover_receipt,
                    LEGACY_GRANDFATHER_RECEIPT_VERSION,
                )?;
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MissionPlanResolution {
    pub actor: String,
    pub activity_id: String,
    pub disposition: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MissionPlanFindingState {
    pub id: String,
    pub graph_revision: MissionGraphRevision,
    pub reviewer: String,
    pub severity: MissionPlanFindingSeverity,
    pub affected_issue_ids: Vec<String>,
    pub dependency_path: Vec<String>,
    pub resolution: Option<MissionPlanResolution>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MissionPlanChangeRequestState {
    pub id: String,
    pub graph_revision: MissionGraphRevision,
    pub reviewer: String,
    pub affected_issue_ids: Vec<String>,
    pub dependency_path: Vec<String>,
    pub resolution: Option<MissionPlanResolution>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionPlanAuthorizationKind {
    Approval,
    LegacyGrandfather,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MissionPlanAuthorization {
    pub kind: MissionPlanAuthorizationKind,
    pub graph_revision: MissionGraphRevision,
    pub actor: String,
    pub activity_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cutover_receipt: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionPlanReviewFreshness {
    FreshApproval,
    FreshGrandfather,
    Stale,
    Unapproved,
    ProvenanceIncomplete,
    BlockedByReview,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MissionPlanReviewState {
    pub mission_id: String,
    pub current_graph_revision: MissionGraphRevision,
    pub authors: Vec<String>,
    pub material_editors: Vec<String>,
    pub provenance_complete: bool,
    pub findings: Vec<MissionPlanFindingState>,
    pub change_requests: Vec<MissionPlanChangeRequestState>,
    pub authorization: Option<MissionPlanAuthorization>,
    pub freshness: MissionPlanReviewFreshness,
}

impl MissionPlanReviewState {
    pub fn has_current_approval(&self) -> bool {
        self.freshness == MissionPlanReviewFreshness::FreshApproval
    }
}

#[derive(Debug, Clone)]
struct Provenance {
    authors: BTreeSet<StableActorIdentity>,
    editors: BTreeSet<StableActorIdentity>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct StableActorIdentity(String);

impl StableActorIdentity {
    fn parse(field: &str, value: &str) -> Result<Self> {
        validate_nonempty(field, value)?;
        let prefix = format!("{STABLE_ACTOR_IDENTITY_VERSION}:");
        let Some(namespaced) = value.strip_prefix(&prefix) else {
            bail!(
                "{field} '{value}' must use the authenticated stable actor namespace '{prefix}<authority>/<immutable-subject>'"
            );
        };
        let Some((authority, subject)) = namespaced.split_once('/') else {
            bail!(
                "{field} '{value}' must contain an authenticated authority and immutable subject separated by '/'"
            );
        };
        validate_actor_authority(field, value, authority)?;
        if subject.is_empty() || subject.contains('/') {
            bail!("{field} '{value}' must contain one non-empty immutable subject");
        }
        if value.trim() != value || value.chars().any(char::is_whitespace) {
            bail!(
                "{field} '{value}' is not a canonical stable actor identity; whitespace is not allowed"
            );
        }
        if value.chars().any(char::is_control) {
            bail!(
                "{field} '{value}' is not a canonical stable actor identity; control characters are not allowed"
            );
        }
        let default_ignorables = CodePointSetData::new::<DefaultIgnorableCodePoint>();
        if let Some(character) = value
            .chars()
            .find(|character| default_ignorables.contains(*character))
        {
            bail!(
                "{field} '{value}' contains default-ignorable Unicode code point U+{:04X}",
                character as u32
            );
        }
        if !ComposingNormalizerBorrowed::new_nfc().is_normalized(value) {
            bail!(
                "{field} '{value}' is not NFC; noncanonical Unicode actor aliases are not allowed"
            );
        }
        if !subject
            .chars()
            .all(|character| character.is_alphanumeric() || matches!(character, '-' | '_' | '.'))
        {
            bail!(
                "{field} '{value}' has an invalid immutable subject; use Unicode letters/numbers or '-', '_' and '.'"
            );
        }
        if !subject.chars().next().is_some_and(char::is_alphanumeric)
            || !subject
                .chars()
                .next_back()
                .is_some_and(char::is_alphanumeric)
        {
            bail!("{field} '{value}' immutable subject must start and end with a letter or number");
        }
        Ok(Self(value.to_string()))
    }
}

pub fn validate_stable_actor_identity(value: &str) -> Result<()> {
    StableActorIdentity::parse("actor", value).map(|_| ())
}

fn validate_actor_authority(field: &str, value: &str, authority: &str) -> Result<()> {
    let labels = authority.split('.').collect::<Vec<_>>();
    if labels.len() < 2
        || labels.iter().any(|label| {
            label.is_empty()
                || !label
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
                || !label
                    .as_bytes()
                    .first()
                    .is_some_and(u8::is_ascii_alphanumeric)
                || !label
                    .as_bytes()
                    .last()
                    .is_some_and(u8::is_ascii_alphanumeric)
        })
    {
        bail!(
            "{field} '{value}' has a noncanonical authenticated authority; use a lowercase DNS-style authority"
        );
    }
    Ok(())
}

#[derive(Debug, Serialize)]
struct RevisionPayload<'a> {
    algorithm: &'static str,
    mission_id: &'a str,
    roots: BTreeSet<String>,
    records: BTreeMap<String, RevisionRecord<'a>>,
    hierarchy_edges: BTreeSet<(String, String)>,
    advances_edges: BTreeSet<(String, String)>,
    dependency_edges: BTreeSet<RevisionDependencyEdge>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum RevisionDependencyEdge {
    Blocks {
        blocker_id: String,
        blocked_id: String,
    },
    BlockedBy {
        left_id: String,
        right_id: String,
    },
}

#[derive(Debug, Serialize)]
struct RevisionRecord<'a> {
    id: &'a str,
    issue_type: &'a str,
    title: &'a str,
    priority: &'a str,
    labels: Vec<&'a str>,
    description: &'a str,
    outcome: &'a str,
    evidence: &'a str,
}

/// Calculate the exact current material revision for a mission and its scope.
pub fn mission_graph_revision(state_dir: &Path, mission_id: &str) -> Result<MissionGraphRevision> {
    let issues = RecordStore::new(state_dir).load_issues()?;
    graph_revision_from_records(&issues, mission_id)
}

pub fn validate_mission_plan_review_event_references(
    state_dir: &Path,
    mission_id: &str,
    event: &MissionPlanReviewEvent,
) -> Result<()> {
    let issues = RecordStore::new(state_dir).load_issues()?;
    validate_event_references_from_records(&issues, mission_id, event)
}

#[derive(Debug)]
struct MissionReviewReferenceGraph {
    reviewed_issue_ids: BTreeSet<String>,
    referenceable_issue_ids: BTreeSet<String>,
    dependency_steps: BTreeSet<(String, String)>,
}

fn mission_review_reference_graph(
    issues: &[CanonicalIssueRecord],
    mission_id: &str,
) -> Result<MissionReviewReferenceGraph> {
    let by_id = issues
        .iter()
        .map(|record| (record.issue.id.as_str(), record))
        .collect::<BTreeMap<_, _>>();
    let mission = by_id
        .get(mission_id)
        .copied()
        .ok_or_else(|| anyhow!("Mission-plan review references missing mission {mission_id}"))?;
    if mission.issue.issue_type != "mission" {
        bail!(
            "Mission-plan review subject {mission_id} has issue_type '{}'; expected mission",
            mission.issue.issue_type
        );
    }

    let mut reviewed_issue_ids = BTreeSet::from([mission_id.to_string()]);
    let mut pending = mission
        .relationships
        .relates
        .iter()
        .filter(|relation| relation.relation_type == "advances")
        .map(|relation| {
            if relation.kind != "issue" {
                bail!(
                    "Mission {mission_id} advances non-issue target {}/{}",
                    relation.kind,
                    relation.id
                );
            }
            Ok(relation.id.clone())
        })
        .collect::<Result<Vec<_>>>()?;
    while let Some(id) = pending.pop() {
        if !reviewed_issue_ids.insert(id.clone()) {
            continue;
        }
        let record = by_id
            .get(id.as_str())
            .copied()
            .ok_or_else(|| anyhow!("Mission {mission_id} reaches missing issue {id}"))?;
        for child in &record.relationships.children {
            if child.kind != "issue" {
                bail!(
                    "Issue {id} has non-issue hierarchy target {}/{}",
                    child.kind,
                    child.id
                );
            }
            pending.push(child.id.clone());
        }
    }

    let mut all_dependency_steps = BTreeSet::new();
    for record in issues {
        for blocked in &record.relationships.blocks {
            if blocked.kind != "issue" {
                bail!(
                    "Issue {} has non-issue dependency target {}/{}",
                    record.issue.id,
                    blocked.kind,
                    blocked.id
                );
            }
            if !by_id.contains_key(blocked.id.as_str()) {
                bail!(
                    "Mission {mission_id} dependency graph references missing issue {}",
                    blocked.id
                );
            }
            all_dependency_steps.insert((blocked.id.clone(), record.issue.id.clone()));
        }
    }
    let mut referenceable_issue_ids = reviewed_issue_ids.clone();
    let mut dependency_steps = BTreeSet::new();
    let mut pending = reviewed_issue_ids.iter().cloned().collect::<Vec<_>>();
    while let Some(blocked_id) = pending.pop() {
        for (_, blocker_id) in all_dependency_steps
            .iter()
            .filter(|(candidate, _)| candidate == &blocked_id)
        {
            if dependency_steps.insert((blocked_id.clone(), blocker_id.clone()))
                && referenceable_issue_ids.insert(blocker_id.clone())
            {
                pending.push(blocker_id.clone());
            }
        }
    }
    Ok(MissionReviewReferenceGraph {
        reviewed_issue_ids,
        referenceable_issue_ids,
        dependency_steps,
    })
}

fn validate_event_references_from_records(
    issues: &[CanonicalIssueRecord],
    mission_id: &str,
    event: &MissionPlanReviewEvent,
) -> Result<()> {
    let (affected, path): (&[String], &[String]) = match event {
        MissionPlanReviewEvent::Finding {
            affected_issue_ids,
            dependency_path,
            ..
        }
        | MissionPlanReviewEvent::ChangeRequest {
            affected_issue_ids,
            dependency_path,
            ..
        } => (affected_issue_ids, dependency_path),
        _ => return Ok(()),
    };
    let graph = mission_review_reference_graph(issues, mission_id)?;
    for id in affected.iter().chain(path) {
        if !graph.referenceable_issue_ids.contains(id) {
            bail!(
                "Mission-plan review for {mission_id} references issue {id} outside the current mission graph"
            );
        }
    }
    if path.len() == 1 {
        bail!(
            "Mission-plan review dependency path for {mission_id} must contain at least one directed dependency edge"
        );
    }
    let mut path_nodes = BTreeSet::new();
    for id in path {
        if !path_nodes.insert(id) {
            bail!(
                "Mission-plan review dependency path for {mission_id} repeats issue {id}; paths must be simple and acyclic"
            );
        }
    }
    for step in path.windows(2) {
        if !graph
            .dependency_steps
            .contains(&(step[0].clone(), step[1].clone()))
        {
            bail!(
                "Mission-plan review dependency path for {mission_id} does not follow directed blocked-to-blocker edge {} -> {}",
                step[0],
                step[1]
            );
        }
    }
    if !path.is_empty() && !path.iter().any(|id| graph.reviewed_issue_ids.contains(id)) {
        bail!(
            "Mission-plan review dependency path for {mission_id} does not reach reviewed mission work"
        );
    }
    Ok(())
}

fn graph_revision_from_records(
    issues: &[CanonicalIssueRecord],
    mission_id: &str,
) -> Result<MissionGraphRevision> {
    let by_id = issues
        .iter()
        .map(|record| (record.issue.id.as_str(), record))
        .collect::<BTreeMap<_, _>>();
    let mission = by_id
        .get(mission_id)
        .copied()
        .ok_or_else(|| anyhow!("Mission-plan review references missing mission {mission_id}"))?;
    if mission.issue.issue_type != "mission" {
        bail!(
            "Mission-plan review subject {mission_id} has issue_type '{}'; expected mission",
            mission.issue.issue_type
        );
    }

    let roots = mission
        .relationships
        .relates
        .iter()
        .filter(|relation| relation.relation_type == "advances")
        .map(|relation| {
            if relation.kind != "issue" {
                bail!(
                    "Mission {mission_id} advances non-issue target {}/{}",
                    relation.kind,
                    relation.id
                );
            }
            if !by_id.contains_key(relation.id.as_str()) {
                bail!(
                    "Mission {mission_id} advances missing issue {}",
                    relation.id
                );
            }
            Ok(relation.id.clone())
        })
        .collect::<Result<BTreeSet<_>>>()?;

    let mut reviewed = BTreeSet::from([mission_id.to_string()]);
    let mut pending = roots.iter().cloned().collect::<Vec<_>>();
    while let Some(id) = pending.pop() {
        if !reviewed.insert(id.clone()) {
            continue;
        }
        let record = by_id
            .get(id.as_str())
            .copied()
            .ok_or_else(|| anyhow!("Mission {mission_id} reaches missing issue {id}"))?;
        for child in &record.relationships.children {
            if child.kind != "issue" {
                bail!(
                    "Issue {id} has non-issue hierarchy target {}/{}",
                    child.kind,
                    child.id
                );
            }
            pending.push(child.id.clone());
        }
    }

    let mut records = BTreeMap::new();
    let mut hierarchy_edges = BTreeSet::new();
    let mut advances_edges = BTreeSet::new();
    let mut dependency_edges = BTreeSet::new();
    for id in &reviewed {
        let record = by_id
            .get(id.as_str())
            .copied()
            .ok_or_else(|| anyhow!("Mission {mission_id} reaches missing issue {id}"))?;
        let mut labels = record.labels.iter().map(String::as_str).collect::<Vec<_>>();
        labels.sort_unstable();
        records.insert(
            id.clone(),
            RevisionRecord {
                id: &record.issue.id,
                issue_type: &record.issue.issue_type,
                title: &record.issue.title,
                priority: &record.issue.priority,
                labels,
                description: &record.sections.description,
                outcome: &record.sections.outcome,
                evidence: &record.sections.evidence,
            },
        );
        for child in &record.relationships.children {
            if reviewed.contains(&child.id) {
                hierarchy_edges.insert((id.clone(), child.id.clone()));
            }
        }
        for relation in &record.relationships.relates {
            if relation.kind == "issue" && relation.relation_type == "advances" {
                advances_edges.insert((id.clone(), relation.id.clone()));
            }
        }
    }
    // Block edges are stored on the blocker. Include edges touching reviewed
    // scope so external prerequisites remain material without importing their
    // authored content into the mission revision.
    for record in issues {
        for blocked in &record.relationships.blocks {
            if blocked.kind == "issue"
                && (reviewed.contains(&record.issue.id) || reviewed.contains(&blocked.id))
            {
                dependency_edges.insert(RevisionDependencyEdge::Blocks {
                    blocker_id: record.issue.id.clone(),
                    blocked_id: blocked.id.clone(),
                });
            }
        }
        for relation in &record.relationships.relates {
            if relation.kind == "issue"
                && relation.relation_type == "blocked_by"
                && (reviewed.contains(&record.issue.id) || reviewed.contains(&relation.id))
            {
                let (left_id, right_id) = if record.issue.id < relation.id {
                    (record.issue.id.clone(), relation.id.clone())
                } else {
                    (relation.id.clone(), record.issue.id.clone())
                };
                dependency_edges.insert(RevisionDependencyEdge::BlockedBy { left_id, right_id });
            }
        }
    }

    let payload = RevisionPayload {
        algorithm: MISSION_GRAPH_REVISION_VERSION,
        mission_id,
        roots,
        records,
        hierarchy_edges,
        advances_edges,
        dependency_edges,
    };
    let bytes = serde_json::to_vec(&payload).context("Failed to render mission graph revision")?;
    let digest = Sha256::digest(bytes);
    Ok(MissionGraphRevision(format!(
        "{MISSION_GRAPH_REVISION_VERSION}:sha256:{}",
        lower_hex(&digest)
    )))
}

fn lower_hex(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut encoded, "{byte:02x}").expect("writing to a String cannot fail");
    }
    encoded
}

/// Rebuild current mission-plan state directly from canonical records and
/// append-only activity sidecars.
pub fn mission_plan_review_state(
    state_dir: &Path,
    mission_id: &str,
) -> Result<MissionPlanReviewState> {
    let issues = RecordStore::new(state_dir).load_issues()?;
    mission_plan_review_state_from_records(state_dir, mission_id, &issues)
}

/// Project review state from a caller-provided canonical graph slice.
///
/// The slice must include the mission, its complete advances/child scope, and
/// every dependency endpoint touching that scope. This preserves the revision
/// contract while allowing decision-safe readers to avoid reparsing unrelated
/// repository records.
pub fn mission_plan_review_state_from_records(
    state_dir: &Path,
    mission_id: &str,
    issues: &[CanonicalIssueRecord],
) -> Result<MissionPlanReviewState> {
    let mission = issues
        .iter()
        .find(|record| record.issue.id == mission_id)
        .ok_or_else(|| anyhow!("Mission-plan review references missing mission {mission_id}"))?;
    let revision = graph_revision_from_records(issues, mission_id)?;
    let activities = list_issue_activities(state_dir, mission_id)?;
    let cutover_manifest = load_mission_plan_review_cutover_manifest(state_dir)?;
    project_mission_plan_review(
        mission_id,
        &mission.issue.status,
        revision,
        &activities,
        cutover_manifest.as_ref(),
    )
}

pub fn project_mission_plan_review(
    mission_id: &str,
    mission_status: &str,
    current_graph_revision: MissionGraphRevision,
    activities: &[IssueActivity],
    cutover_manifest: Option<&MissionPlanReviewCutoverManifest>,
) -> Result<MissionPlanReviewState> {
    current_graph_revision.validate()?;
    if let Some(manifest) = cutover_manifest {
        manifest.validate()?;
    }
    let mut ordered = activities.iter().collect::<Vec<_>>();
    ordered.sort_by(|left, right| {
        left.created_at
            .cmp(&right.created_at)
            .then(left.id.cmp(&right.id))
    });

    let mut requests = BTreeMap::<MissionGraphRevision, Provenance>::new();
    let mut attributions = BTreeMap::<
        MissionGraphRevision,
        (MissionGraphRevision, BTreeSet<StableActorIdentity>),
    >::new();
    let mut findings = Vec::<MissionPlanFindingState>::new();
    let mut changes = Vec::<MissionPlanChangeRequestState>::new();
    let mut ids = BTreeSet::new();
    let mut authorizations = Vec::new();
    let mut grandfather_receipts = 0usize;

    for activity in ordered {
        if activity.subject_kind != "issue" || activity.subject_id != mission_id {
            continue;
        }
        if activity.event_type != ActivityEventType::MissionPlanReview {
            continue;
        }
        let activity_actor = StableActorIdentity::parse("activity actor", &activity.actor)?;
        let event = activity.mission_plan_review.as_ref().ok_or_else(|| {
            anyhow!(
                "Mission-plan review activity {} is missing typed metadata",
                activity.id
            )
        })?;
        event
            .validate()
            .with_context(|| format!("Invalid mission-plan review activity {}", activity.id))?;
        match event {
            MissionPlanReviewEvent::Request {
                graph_revision,
                authors,
                material_editors,
            } => {
                let provenance = Provenance {
                    authors: stable_actor_set("authors", authors)?,
                    editors: stable_actor_set("material_editors", material_editors)?,
                };
                if requests
                    .insert(graph_revision.clone(), provenance)
                    .is_some()
                {
                    bail!("Duplicate provenance request for graph revision {graph_revision}");
                }
            }
            MissionPlanReviewEvent::MaterialEditAttribution {
                previous_graph_revision,
                graph_revision,
                editors,
            } => {
                if attributions
                    .insert(
                        graph_revision.clone(),
                        (
                            previous_graph_revision.clone(),
                            stable_actor_set("editors", editors)?,
                        ),
                    )
                    .is_some()
                {
                    bail!("Duplicate material-edit attribution for {graph_revision}");
                }
            }
            MissionPlanReviewEvent::Finding {
                graph_revision,
                finding_id,
                severity,
                affected_issue_ids,
                dependency_path,
            } => {
                if !ids.insert(finding_id.clone()) {
                    bail!("Duplicate mission-plan review decision id {finding_id}");
                }
                findings.push(MissionPlanFindingState {
                    id: finding_id.clone(),
                    graph_revision: graph_revision.clone(),
                    reviewer: activity.actor.clone(),
                    severity: *severity,
                    affected_issue_ids: affected_issue_ids.clone(),
                    dependency_path: dependency_path.clone(),
                    resolution: None,
                });
            }
            MissionPlanReviewEvent::ChangeRequest {
                graph_revision,
                request_id,
                affected_issue_ids,
                dependency_path,
            } => {
                if !ids.insert(request_id.clone()) {
                    bail!("Duplicate mission-plan review decision id {request_id}");
                }
                changes.push(MissionPlanChangeRequestState {
                    id: request_id.clone(),
                    graph_revision: graph_revision.clone(),
                    reviewer: activity.actor.clone(),
                    affected_issue_ids: affected_issue_ids.clone(),
                    dependency_path: dependency_path.clone(),
                    resolution: None,
                });
            }
            MissionPlanReviewEvent::Resolution {
                graph_revision,
                target_id,
                disposition,
            } => {
                let (target_revision, target_reviewer) = findings
                    .iter()
                    .find(|item| item.id == *target_id)
                    .map(|item| (&item.graph_revision, item.reviewer.as_str()))
                    .or_else(|| {
                        changes
                            .iter()
                            .find(|item| item.id == *target_id)
                            .map(|item| (&item.graph_revision, item.reviewer.as_str()))
                    })
                    .ok_or_else(|| {
                        anyhow!("Resolution references unknown mission-plan decision {target_id}")
                    })?;
                if target_revision != graph_revision {
                    bail!(
                        "Resolution {target_id} names revision {graph_revision}, but the decision belongs to {target_revision}"
                    );
                }
                let provenance = resolve_provenance(target_revision, &requests, &attributions)?
                    .ok_or_else(|| {
                        anyhow!(
                            "Resolution {target_id} has incomplete author/material-editor provenance"
                        )
                    })?;
                if target_reviewer == activity.actor
                    || (!provenance.authors.contains(&activity_actor)
                        && !provenance.editors.contains(&activity_actor))
                {
                    bail!(
                        "Actor '{}' cannot resolve {target_id}; a mission author or material editor other than the decision reviewer must resolve it",
                        activity.actor
                    );
                }
                let resolution = MissionPlanResolution {
                    actor: activity.actor.clone(),
                    activity_id: activity.id.clone(),
                    disposition: disposition.clone(),
                };
                if let Some(finding) = findings.iter_mut().find(|item| item.id == *target_id) {
                    if finding.resolution.replace(resolution).is_some() {
                        bail!("Mission-plan review decision {target_id} has multiple resolutions");
                    }
                } else if let Some(change) = changes.iter_mut().find(|item| item.id == *target_id) {
                    if change.resolution.replace(resolution).is_some() {
                        bail!("Mission-plan review decision {target_id} has multiple resolutions");
                    }
                }
            }
            MissionPlanReviewEvent::Approval { graph_revision } => {
                let provenance = resolve_provenance(graph_revision, &requests, &attributions)?
                    .ok_or_else(|| {
                        anyhow!(
                            "Approval by '{}' for {graph_revision} has incomplete author/material-editor provenance",
                            activity.actor
                        )
                    })?;
                if provenance.authors.contains(&activity_actor)
                    || provenance.editors.contains(&activity_actor)
                {
                    bail!(
                        "Reviewer '{}' is not independent for {graph_revision}; reviewer matches an author or material editor",
                        activity.actor
                    );
                }
                ensure_no_blocking_review_state(graph_revision, &findings, &changes)?;
                authorizations.push(MissionPlanAuthorization {
                    kind: MissionPlanAuthorizationKind::Approval,
                    graph_revision: graph_revision.clone(),
                    actor: activity.actor.clone(),
                    activity_id: activity.id.clone(),
                    legacy_status: None,
                    migration_id: None,
                    cutover_receipt: None,
                });
            }
            MissionPlanReviewEvent::LegacyGrandfather {
                graph_revision,
                legacy_status,
                migration_id,
                cutover_receipt,
            } => {
                grandfather_receipts += 1;
                if grandfather_receipts > 1 {
                    bail!(
                        "Mission {mission_id} has duplicate legacy-grandfather receipts; expected exactly one"
                    );
                }
                if activity_actor.0 != LEGACY_GRANDFATHER_MIGRATION_ACTOR {
                    bail!(
                        "Legacy grandfather activity {} must be produced by migration actor '{LEGACY_GRANDFATHER_MIGRATION_ACTOR}'",
                        activity.id
                    );
                }
                let manifest = cutover_manifest.ok_or_else(|| {
                    anyhow!(
                        "Legacy grandfather activity {} is hand-authored or late; {} is missing",
                        activity.id,
                        MISSION_PLAN_REVIEW_CUTOVER_MANIFEST_PATH
                    )
                })?;
                let eligibility = manifest.eligibility_for(mission_id).ok_or_else(|| {
                    anyhow!(
                        "Legacy grandfather activity {} names mission {mission_id}, which is not eligible in the cutover manifest",
                        activity.id
                    )
                })?;
                validate_legacy_grandfather_receipt(
                    manifest,
                    eligibility,
                    activity,
                    graph_revision,
                    legacy_status,
                    migration_id,
                    cutover_receipt,
                )?;
                authorizations.push(MissionPlanAuthorization {
                    kind: MissionPlanAuthorizationKind::LegacyGrandfather,
                    graph_revision: graph_revision.clone(),
                    actor: activity.actor.clone(),
                    activity_id: activity.id.clone(),
                    legacy_status: Some(legacy_status.clone()),
                    migration_id: Some(migration_id.clone()),
                    cutover_receipt: Some(cutover_receipt.clone()),
                });
            }
        }
    }

    if cutover_manifest
        .and_then(|manifest| manifest.eligibility_for(mission_id))
        .is_some()
        && grandfather_receipts != 1
    {
        bail!(
            "Mission {mission_id} is eligible in the cutover manifest but has no legacy-grandfather receipt; expected exactly one"
        );
    }

    for revision in attributions.keys() {
        if requests.contains_key(revision) {
            bail!(
                "Graph revision {revision} has both a provenance request and material-edit attribution"
            );
        }
        if resolve_provenance(revision, &requests, &attributions)?.is_none() {
            bail!("Material-edit attribution for {revision} has incomplete prior provenance");
        }
    }
    // A later attribution event may reveal a contributor who was not known at
    // approval time. Recheck every approval against the complete canonical
    // stream so event ordering cannot preserve a non-independent approval.
    for authorization in &authorizations {
        if authorization.kind != MissionPlanAuthorizationKind::Approval {
            continue;
        }
        let provenance =
            resolve_provenance(&authorization.graph_revision, &requests, &attributions)?
                .ok_or_else(|| {
                    anyhow!(
                        "Approval by '{}' for {} has incomplete author/material-editor provenance",
                        authorization.actor,
                        authorization.graph_revision
                    )
                })?;
        let authorization_actor =
            StableActorIdentity::parse("authorization actor", &authorization.actor)?;
        if provenance.authors.contains(&authorization_actor)
            || provenance.editors.contains(&authorization_actor)
        {
            bail!(
                "Reviewer '{}' is not independent for {}; reviewer matches an author or material editor",
                authorization.actor,
                authorization.graph_revision
            );
        }
    }

    let provenance = resolve_provenance(&current_graph_revision, &requests, &attributions)?;
    let provenance_complete = provenance.is_some();
    let (authors, material_editors) = provenance
        .map(|value| {
            (
                value.authors.into_iter().map(|actor| actor.0).collect(),
                value.editors.into_iter().map(|actor| actor.0).collect(),
            )
        })
        .unwrap_or_default();
    let authorization = authorizations.pop();
    let freshness = match authorization.as_ref() {
        Some(value) if value.graph_revision != current_graph_revision => {
            MissionPlanReviewFreshness::Stale
        }
        Some(value) => {
            if has_blocking_review_state(&value.graph_revision, &findings, &changes) {
                MissionPlanReviewFreshness::BlockedByReview
            } else {
                match value.kind {
                    MissionPlanAuthorizationKind::Approval => {
                        MissionPlanReviewFreshness::FreshApproval
                    }
                    MissionPlanAuthorizationKind::LegacyGrandfather
                        if value.legacy_status.as_deref() == Some(mission_status) =>
                    {
                        MissionPlanReviewFreshness::FreshGrandfather
                    }
                    MissionPlanAuthorizationKind::LegacyGrandfather => {
                        MissionPlanReviewFreshness::Stale
                    }
                }
            }
        }
        None if !provenance_complete => MissionPlanReviewFreshness::ProvenanceIncomplete,
        None => MissionPlanReviewFreshness::Unapproved,
    };

    Ok(MissionPlanReviewState {
        mission_id: mission_id.to_string(),
        current_graph_revision,
        authors,
        material_editors,
        provenance_complete,
        findings,
        change_requests: changes,
        authorization,
        freshness,
    })
}

/// Validate every mission's review stream during canonical rebuild/check.
pub fn validate_mission_plan_reviews(state_dir: &Path) -> Result<()> {
    let issues = RecordStore::new(state_dir).load_issues()?;
    let cutover_manifest = load_mission_plan_review_cutover_manifest(state_dir)?;
    if let Some(manifest) = cutover_manifest.as_ref() {
        for eligibility in &manifest.eligible_missions {
            let issue = issues
                .iter()
                .find(|issue| issue.issue.id == eligibility.mission_id)
                .ok_or_else(|| {
                    anyhow!(
                        "Mission-plan cutover manifest references missing mission {}",
                        eligibility.mission_id
                    )
                })?;
            if issue.issue.issue_type != "mission" {
                bail!(
                    "Mission-plan cutover manifest references {} with issue_type '{}'; expected mission",
                    eligibility.mission_id,
                    issue.issue.issue_type
                );
            }
        }
    }
    for issue in &issues {
        let activities = list_issue_activities(state_dir, &issue.issue.id)?;
        if issue.issue.issue_type != "mission"
            && activities
                .iter()
                .any(|activity| activity.mission_plan_review.is_some())
        {
            bail!(
                "Mission-plan review activity is attached to non-mission issue {} ({})",
                issue.issue.id,
                issue.issue.issue_type
            );
        }
        if issue.issue.issue_type != "mission" {
            continue;
        }
        for activity in &activities {
            let Some(event) = activity.mission_plan_review.as_ref() else {
                continue;
            };
            validate_event_references_from_records(&issues, &issue.issue.id, event).with_context(
                || {
                    format!(
                        "Invalid mission-plan review activity {} for {}",
                        activity.id, issue.issue.id
                    )
                },
            )?;
        }
        let revision = graph_revision_from_records(&issues, &issue.issue.id)?;
        project_mission_plan_review(
            &issue.issue.id,
            &issue.issue.status,
            revision,
            &activities,
            cutover_manifest.as_ref(),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_legacy_grandfather_receipt(
    manifest: &MissionPlanReviewCutoverManifest,
    eligibility: &LegacyGrandfatherEligibility,
    activity: &IssueActivity,
    graph_revision: &MissionGraphRevision,
    legacy_status: &str,
    migration_id: &str,
    cutover_receipt: &str,
) -> Result<()> {
    if activity.id != eligibility.receipt_activity_id {
        bail!(
            "Legacy grandfather activity {} is late or forged; cutover manifest requires receipt activity {}",
            activity.id,
            eligibility.receipt_activity_id
        );
    }
    if activity.created_at != manifest.cutover_at {
        bail!(
            "Legacy grandfather activity {} was created at {}, after or outside exact cutover time {}",
            activity.id,
            activity.created_at,
            manifest.cutover_at
        );
    }
    if graph_revision != &eligibility.graph_revision {
        bail!(
            "Legacy grandfather activity {} names graph revision {}, but cutover eligibility requires {}",
            activity.id,
            graph_revision,
            eligibility.graph_revision
        );
    }
    if legacy_status != eligibility.legacy_status {
        bail!(
            "Legacy grandfather activity {} names legacy status '{}', but cutover eligibility requires '{}'",
            activity.id,
            legacy_status,
            eligibility.legacy_status
        );
    }
    if migration_id != manifest.migration_id {
        bail!(
            "Legacy grandfather activity {} names migration '{}', but cutover manifest requires '{}'",
            activity.id,
            migration_id,
            manifest.migration_id
        );
    }
    let expected_receipt = manifest.receipt_for(eligibility)?;
    if cutover_receipt != expected_receipt {
        bail!(
            "Legacy grandfather activity {} has a forged cutover receipt; expected {}",
            activity.id,
            expected_receipt
        );
    }
    Ok(())
}

fn resolve_provenance(
    revision: &MissionGraphRevision,
    requests: &BTreeMap<MissionGraphRevision, Provenance>,
    attributions: &BTreeMap<
        MissionGraphRevision,
        (MissionGraphRevision, BTreeSet<StableActorIdentity>),
    >,
) -> Result<Option<Provenance>> {
    fn visit(
        revision: &MissionGraphRevision,
        requests: &BTreeMap<MissionGraphRevision, Provenance>,
        attributions: &BTreeMap<
            MissionGraphRevision,
            (MissionGraphRevision, BTreeSet<StableActorIdentity>),
        >,
        visiting: &mut BTreeSet<MissionGraphRevision>,
    ) -> Result<Option<Provenance>> {
        if let Some(provenance) = requests.get(revision) {
            return Ok(Some(provenance.clone()));
        }
        let Some((previous, editors)) = attributions.get(revision) else {
            return Ok(None);
        };
        if !visiting.insert(revision.clone()) {
            bail!("Material-edit attribution contains a revision cycle at {revision}");
        }
        let result = visit(previous, requests, attributions, visiting)?;
        visiting.remove(revision);
        Ok(result.map(|mut provenance| {
            provenance.editors.extend(editors.iter().cloned());
            provenance
        }))
    }
    visit(revision, requests, attributions, &mut BTreeSet::new())
}

fn ensure_no_blocking_review_state(
    revision: &MissionGraphRevision,
    findings: &[MissionPlanFindingState],
    changes: &[MissionPlanChangeRequestState],
) -> Result<()> {
    if let Some(finding) = findings.iter().find(|finding| {
        finding.graph_revision == *revision
            && finding.severity == MissionPlanFindingSeverity::Blocking
            && finding.resolution.is_none()
    }) {
        bail!(
            "Cannot approve {revision}; blocking finding {} is unresolved",
            finding.id
        );
    }
    if let Some(change) = changes
        .iter()
        .find(|change| change.graph_revision == *revision && change.resolution.is_none())
    {
        bail!(
            "Cannot approve {revision}; change request {} is unresolved",
            change.id
        );
    }
    Ok(())
}

fn has_blocking_review_state(
    revision: &MissionGraphRevision,
    findings: &[MissionPlanFindingState],
    changes: &[MissionPlanChangeRequestState],
) -> bool {
    findings.iter().any(|finding| {
        finding.graph_revision == *revision
            && finding.severity == MissionPlanFindingSeverity::Blocking
            && finding.resolution.is_none()
    }) || changes
        .iter()
        .any(|change| change.graph_revision == *revision && change.resolution.is_none())
}

fn validate_actor_list(field: &str, values: &[String], allow_empty: bool) -> Result<()> {
    if values.is_empty() && !allow_empty {
        bail!("{field} must contain at least one stable actor identity");
    }
    validate_sorted_unique(field, values)?;
    for value in values {
        StableActorIdentity::parse(field, value)?;
    }
    Ok(())
}

fn stable_actor_set(field: &str, values: &[String]) -> Result<BTreeSet<StableActorIdentity>> {
    values
        .iter()
        .map(|value| StableActorIdentity::parse(field, value))
        .collect()
}

fn validate_sorted_unique(field: &str, values: &[String]) -> Result<()> {
    if values.windows(2).any(|pair| pair[0] >= pair[1]) {
        bail!("{field} must be sorted and contain no duplicates");
    }
    Ok(())
}

fn validate_path(field: &str, values: &[String]) -> Result<()> {
    for value in values {
        validate_nonempty(field, value)?;
    }
    Ok(())
}

fn validate_event_id(field: &str, value: &str) -> Result<()> {
    validate_nonempty(field, value)?;
    if !value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
    {
        bail!("{field} '{value}' may contain only letters, digits, '-' and '_'");
    }
    Ok(())
}

fn validate_versioned_sha256(field: &str, value: &str, version: &str) -> Result<()> {
    let prefix = format!("{version}:sha256:");
    let Some(digest) = value.strip_prefix(&prefix) else {
        bail!("{field} '{value}' must start with '{prefix}'");
    };
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        bail!("{field} '{value}' has an invalid SHA-256 digest");
    }
    Ok(())
}

fn validate_nonempty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        bail!("{field} must not be empty");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activity::{create_issue_activity, create_mission_plan_review_activity};
    use crate::{
        issue_relates_relationship, issue_relationship_target, IssueSections, Relationships,
    };
    use atelier_core::Issue;
    use chrono::{DateTime, Utc};
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    fn at(offset: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(1_780_000_000 + offset, 0).unwrap()
    }

    fn issue(
        id: &str,
        issue_type: &str,
        title: &str,
        relationships: Relationships,
    ) -> CanonicalIssueRecord {
        CanonicalIssueRecord {
            issue: Issue {
                id: id.to_string(),
                title: title.to_string(),
                description: None,
                status: if issue_type == "mission" {
                    "draft"
                } else {
                    "todo"
                }
                .to_string(),
                issue_type: issue_type.to_string(),
                priority: "high".to_string(),
                fields: BTreeMap::new(),
                parent_id: None,
                created_at: at(0),
                updated_at: at(0),
                closed_at: None,
            },
            labels: vec!["planning".to_string()],
            sections: IssueSections {
                description: format!("Intent for {id}"),
                outcome: format!("Outcome for {id}"),
                evidence: format!("Closeout for {id}"),
                notes: Some("Context-only note".to_string()),
            },
            relationships,
        }
    }

    fn graph() -> Vec<CanonicalIssueRecord> {
        vec![
            issue(
                "atelier-m100",
                "mission",
                "Mission",
                Relationships {
                    relates: vec![
                        issue_relates_relationship("atelier-e100", "advances"),
                        issue_relates_relationship("atelier-t100", "advances"),
                    ],
                    ..Relationships::default()
                },
            ),
            issue(
                "atelier-e100",
                "epic",
                "Epic",
                Relationships {
                    children: vec![
                        issue_relationship_target("atelier-t100"),
                        issue_relationship_target("atelier-v100"),
                    ],
                    ..Relationships::default()
                },
            ),
            issue(
                "atelier-t100",
                "feature",
                "Implementation",
                Relationships::default(),
            ),
            issue(
                "atelier-v100",
                "validation",
                "Validation",
                Relationships::default(),
            ),
            issue(
                "atelier-x100",
                "task",
                "External prerequisite",
                Relationships {
                    blocks: vec![issue_relationship_target("atelier-t100")],
                    ..Relationships::default()
                },
            ),
        ]
    }

    fn revision(records: &[CanonicalIssueRecord]) -> MissionGraphRevision {
        graph_revision_from_records(records, "atelier-m100").unwrap()
    }

    #[test]
    fn every_material_edit_class_changes_revision_but_notes_and_order_do_not() {
        let baseline = graph();
        let expected = revision(&baseline);

        let mut mission_intent = baseline.clone();
        mission_intent[0].sections.description.push_str(" changed");
        assert_ne!(revision(&mission_intent), expected, "mission intent");

        let mut scope_roots = baseline.clone();
        scope_roots[0].relationships.relates.remove(1);
        assert_ne!(revision(&scope_roots), expected, "scope roots");

        let mut work_content = baseline.clone();
        work_content[2].sections.outcome.push_str(" changed");
        assert_ne!(revision(&work_content), expected, "reachable work content");

        let mut hierarchy = baseline.clone();
        hierarchy[1]
            .relationships
            .children
            .retain(|target| target.id != "atelier-t100");
        assert_ne!(revision(&hierarchy), expected, "hierarchy");

        let mut blockers = baseline.clone();
        blockers[4].relationships.blocks.clear();
        assert_ne!(revision(&blockers), expected, "dependency/blocker edges");

        let mut direct_mission_blocker = baseline.clone();
        direct_mission_blocker[0]
            .relationships
            .relates
            .push(issue_relates_relationship("atelier-x100", "blocked_by"));
        assert_ne!(
            revision(&direct_mission_blocker),
            expected,
            "direct mission blocked_by edge"
        );

        let mut closeout = baseline.clone();
        closeout[3].sections.evidence.push_str(" changed");
        assert_ne!(revision(&closeout), expected, "closeout coverage");

        let mut non_material = baseline.clone();
        non_material[0].sections.notes = Some("A new context-only note".to_string());
        non_material[0].issue.status = "in_progress".to_string();
        non_material[0].issue.updated_at = at(100);
        assert_eq!(revision(&non_material), expected);

        non_material.reverse();
        non_material[4].relationships.relates.reverse();
        assert_eq!(revision(&non_material), expected, "canonical ordering");
    }

    fn append_event(
        state_dir: &Path,
        mission_id: &str,
        offset: i64,
        actor: &str,
        event: MissionPlanReviewEvent,
    ) {
        create_mission_plan_review_activity(
            state_dir,
            mission_id,
            actor,
            at(offset),
            "Mission plan review event",
            event,
            "Inspectable review detail.",
        )
        .unwrap();
    }

    fn write_graph(state_dir: &Path) {
        let store = RecordStore::new(state_dir);
        for record in graph() {
            store.write_issue_atomic(&record).unwrap();
        }
    }

    fn cutover_manifest(
        mission_id: &str,
        graph_revision: MissionGraphRevision,
        cutover_at: DateTime<Utc>,
    ) -> MissionPlanReviewCutoverManifest {
        MissionPlanReviewCutoverManifest {
            schema: MISSION_PLAN_REVIEW_CUTOVER_SCHEMA.to_string(),
            schema_version: MISSION_PLAN_REVIEW_CUTOVER_SCHEMA_VERSION,
            migration_id: LEGACY_GRANDFATHER_MIGRATION_ID.to_string(),
            cutover_at,
            eligible_missions: vec![LegacyGrandfatherEligibility {
                mission_id: mission_id.to_string(),
                graph_revision,
                legacy_status: LEGACY_GRANDFATHER_STATUS.to_string(),
                receipt_activity_id: crate::activity::timestamp_activity_id(cutover_at),
            }],
        }
    }

    fn write_cutover_manifest(state_dir: &Path, manifest: &MissionPlanReviewCutoverManifest) {
        manifest.validate().unwrap();
        std::fs::write(
            state_dir.join(MISSION_PLAN_REVIEW_CUTOVER_MANIFEST_PATH),
            serde_yaml::to_string(manifest).unwrap(),
        )
        .unwrap();
    }

    fn grandfather_event(
        manifest: &MissionPlanReviewCutoverManifest,
        mission_id: &str,
    ) -> MissionPlanReviewEvent {
        let eligibility = manifest.eligibility_for(mission_id).unwrap();
        MissionPlanReviewEvent::LegacyGrandfather {
            graph_revision: eligibility.graph_revision.clone(),
            legacy_status: eligibility.legacy_status.clone(),
            migration_id: manifest.migration_id.clone(),
            cutover_receipt: manifest.receipt_for(eligibility).unwrap(),
        }
    }

    #[test]
    fn typed_events_round_trip_and_project_fresh_independent_approval() {
        let directory = tempdir().unwrap();
        let state_dir = directory.path().join(".atelier");
        std::fs::create_dir_all(&state_dir).unwrap();
        write_graph(&state_dir);
        let previous = mission_graph_revision(&state_dir, "atelier-m100").unwrap();
        let store = RecordStore::new(&state_dir);
        let mut implementation = store.load_issue_by_id("atelier-t100").unwrap();
        implementation
            .sections
            .outcome
            .push_str(" after material edit");
        store.write_issue_atomic(&implementation).unwrap();
        let current = mission_graph_revision(&state_dir, "atelier-m100").unwrap();

        append_event(
            &state_dir,
            "atelier-m100",
            1,
            "actor-v1:example.com/planner",
            MissionPlanReviewEvent::Request {
                graph_revision: previous.clone(),
                authors: vec!["actor-v1:example.com/author".to_string()],
                material_editors: Vec::new(),
            },
        );
        append_event(
            &state_dir,
            "atelier-m100",
            2,
            "actor-v1:example.com/editor",
            MissionPlanReviewEvent::MaterialEditAttribution {
                previous_graph_revision: previous,
                graph_revision: current.clone(),
                editors: vec!["actor-v1:example.com/editor".to_string()],
            },
        );
        append_event(
            &state_dir,
            "atelier-m100",
            3,
            "actor-v1:example.com/reviewer",
            MissionPlanReviewEvent::Finding {
                graph_revision: current.clone(),
                finding_id: "finding-1".to_string(),
                severity: MissionPlanFindingSeverity::Blocking,
                affected_issue_ids: vec!["atelier-t100".to_string()],
                dependency_path: vec!["atelier-x100".to_string(), "atelier-t100".to_string()],
            },
        );
        append_event(
            &state_dir,
            "atelier-m100",
            4,
            "actor-v1:example.com/author",
            MissionPlanReviewEvent::Resolution {
                graph_revision: current.clone(),
                target_id: "finding-1".to_string(),
                disposition: "declared external prerequisite".to_string(),
            },
        );
        append_event(
            &state_dir,
            "atelier-m100",
            5,
            "actor-v1:example.com/reviewer",
            MissionPlanReviewEvent::ChangeRequest {
                graph_revision: current.clone(),
                request_id: "change-1".to_string(),
                affected_issue_ids: vec!["atelier-v100".to_string()],
                dependency_path: Vec::new(),
            },
        );
        append_event(
            &state_dir,
            "atelier-m100",
            6,
            "actor-v1:example.com/author",
            MissionPlanReviewEvent::Resolution {
                graph_revision: current.clone(),
                target_id: "change-1".to_string(),
                disposition: "validation ownership clarified".to_string(),
            },
        );
        append_event(
            &state_dir,
            "atelier-m100",
            7,
            "actor-v1:example.com/reviewer",
            MissionPlanReviewEvent::Approval {
                graph_revision: current.clone(),
            },
        );

        let activities = list_issue_activities(&state_dir, "atelier-m100").unwrap();
        for activity in &activities {
            let rendered = activity.to_markdown().unwrap();
            let reparsed = IssueActivity::from_markdown(
                &rendered,
                &crate::activity::record_activity_path("issue", "atelier-m100", &activity.id),
            )
            .unwrap();
            assert_eq!(&reparsed, activity);
            assert_eq!(reparsed.to_markdown().unwrap(), rendered);
        }

        let state = mission_plan_review_state(&state_dir, "atelier-m100").unwrap();
        assert_eq!(state.current_graph_revision, current);
        assert_eq!(state.authors, vec!["actor-v1:example.com/author"]);
        assert_eq!(state.material_editors, vec!["actor-v1:example.com/editor"]);
        assert_eq!(state.findings.len(), 1);
        assert!(state.findings[0].resolution.is_some());
        assert_eq!(state.change_requests.len(), 1);
        assert!(state.change_requests[0].resolution.is_some());
        assert_eq!(
            state.authorization.as_ref().unwrap().actor,
            "actor-v1:example.com/reviewer"
        );
        assert_eq!(state.freshness, MissionPlanReviewFreshness::FreshApproval);
        assert!(state.has_current_approval());

        create_issue_activity(
            &state_dir,
            "atelier-m100",
            ActivityEventType::Note,
            "actor-v1:example.com/observer",
            at(8),
            "Context only",
            "This note does not change the reviewed plan.",
        )
        .unwrap();
        assert_eq!(
            mission_plan_review_state(&state_dir, "atelier-m100")
                .unwrap()
                .freshness,
            MissionPlanReviewFreshness::FreshApproval
        );
    }

    #[test]
    fn material_change_stales_approval_and_requires_attribution_for_new_revision() {
        let directory = tempdir().unwrap();
        let state_dir = directory.path().join(".atelier");
        std::fs::create_dir_all(&state_dir).unwrap();
        write_graph(&state_dir);
        let original = mission_graph_revision(&state_dir, "atelier-m100").unwrap();
        append_event(
            &state_dir,
            "atelier-m100",
            1,
            "actor-v1:example.com/author",
            MissionPlanReviewEvent::Request {
                graph_revision: original.clone(),
                authors: vec!["actor-v1:example.com/author".to_string()],
                material_editors: Vec::new(),
            },
        );
        append_event(
            &state_dir,
            "atelier-m100",
            2,
            "actor-v1:example.com/reviewer",
            MissionPlanReviewEvent::Approval {
                graph_revision: original,
            },
        );

        let store = RecordStore::new(&state_dir);
        let mut mission = store.load_issue_by_id("atelier-m100").unwrap();
        mission.sections.outcome.push_str(" materially changed");
        store.write_issue_atomic(&mission).unwrap();
        let state = mission_plan_review_state(&state_dir, "atelier-m100").unwrap();
        assert_eq!(state.freshness, MissionPlanReviewFreshness::Stale);
        assert!(!state.provenance_complete);
        assert_eq!(
            state.authorization.unwrap().actor,
            "actor-v1:example.com/reviewer"
        );
    }

    #[test]
    fn direct_mission_blocked_by_change_stales_approval() {
        let directory = tempdir().unwrap();
        let state_dir = directory.path().join(".atelier");
        std::fs::create_dir_all(&state_dir).unwrap();
        write_graph(&state_dir);
        let original = mission_graph_revision(&state_dir, "atelier-m100").unwrap();
        append_event(
            &state_dir,
            "atelier-m100",
            1,
            "actor-v1:example.com/author",
            MissionPlanReviewEvent::Request {
                graph_revision: original.clone(),
                authors: vec!["actor-v1:example.com/author".to_string()],
                material_editors: Vec::new(),
            },
        );
        append_event(
            &state_dir,
            "atelier-m100",
            2,
            "actor-v1:example.com/reviewer",
            MissionPlanReviewEvent::Approval {
                graph_revision: original.clone(),
            },
        );

        let store = RecordStore::new(&state_dir);
        let mut mission = store.load_issue_by_id("atelier-m100").unwrap();
        mission
            .relationships
            .relates
            .push(issue_relates_relationship("atelier-x100", "blocked_by"));
        store.write_issue_atomic(&mission).unwrap();

        let state = mission_plan_review_state(&state_dir, "atelier-m100").unwrap();
        assert_ne!(state.current_graph_revision, original);
        assert_eq!(state.freshness, MissionPlanReviewFreshness::Stale);
        assert!(!state.provenance_complete);
    }

    #[test]
    fn rejects_provenance_loss_non_independent_approval_and_unresolved_findings() {
        let revision = MissionGraphRevision(format!(
            "{MISSION_GRAPH_REVISION_VERSION}:sha256:{}",
            "a".repeat(64)
        ));
        let make = |offset, actor: &str, event| IssueActivity {
            id: format!("20260701T00000{offset}000000Z"),
            subject_kind: "issue".to_string(),
            subject_id: "atelier-m100".to_string(),
            event_type: ActivityEventType::MissionPlanReview,
            actor: actor.to_string(),
            created_at: at(offset),
            summary: "review".to_string(),
            pr_attribution: None,
            workflow_transition: None,
            mission_plan_review: Some(event),
            body: String::new(),
        };

        let approval = make(
            1,
            "actor-v1:example.com/reviewer",
            MissionPlanReviewEvent::Approval {
                graph_revision: revision.clone(),
            },
        );
        let error = project_mission_plan_review(
            "atelier-m100",
            "draft",
            revision.clone(),
            std::slice::from_ref(&approval),
            None,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("incomplete author/material-editor provenance"));

        let request = make(
            0,
            "actor-v1:example.com/author",
            MissionPlanReviewEvent::Request {
                graph_revision: revision.clone(),
                authors: vec!["actor-v1:example.com/author".to_string()],
                material_editors: Vec::new(),
            },
        );
        let self_approval = make(
            1,
            "actor-v1:example.com/author",
            MissionPlanReviewEvent::Approval {
                graph_revision: revision.clone(),
            },
        );
        let error = project_mission_plan_review(
            "atelier-m100",
            "draft",
            revision.clone(),
            &[request.clone(), self_approval],
            None,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("not independent"));

        let finding = make(
            1,
            "actor-v1:example.com/reviewer",
            MissionPlanReviewEvent::Finding {
                graph_revision: revision.clone(),
                finding_id: "blocking-1".to_string(),
                severity: MissionPlanFindingSeverity::Blocking,
                affected_issue_ids: Vec::new(),
                dependency_path: Vec::new(),
            },
        );
        let approval = make(
            2,
            "actor-v1:example.com/reviewer",
            MissionPlanReviewEvent::Approval {
                graph_revision: revision.clone(),
            },
        );
        let error = project_mission_plan_review(
            "atelier-m100",
            "draft",
            revision,
            &[request, finding, approval],
            None,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("blocking finding blocking-1 is unresolved"));

        let revision = MissionGraphRevision(format!(
            "{MISSION_GRAPH_REVISION_VERSION}:sha256:{}",
            "c".repeat(64)
        ));
        let request = make(
            0,
            "actor-v1:example.com/author",
            MissionPlanReviewEvent::Request {
                graph_revision: revision.clone(),
                authors: vec!["actor-v1:example.com/author".to_string()],
                material_editors: Vec::new(),
            },
        );
        let approval = make(
            1,
            "actor-v1:example.com/reviewer",
            MissionPlanReviewEvent::Approval {
                graph_revision: revision.clone(),
            },
        );
        let later_change = make(
            2,
            "actor-v1:example.com/reviewer",
            MissionPlanReviewEvent::ChangeRequest {
                graph_revision: revision.clone(),
                request_id: "later-change".to_string(),
                affected_issue_ids: Vec::new(),
                dependency_path: Vec::new(),
            },
        );
        assert_eq!(
            project_mission_plan_review(
                "atelier-m100",
                "draft",
                revision,
                &[request, approval, later_change],
                None,
            )
            .unwrap()
            .freshness,
            MissionPlanReviewFreshness::BlockedByReview
        );
    }

    #[test]
    fn rejects_whitespace_variant_self_approval() {
        let revision = MissionGraphRevision(format!(
            "{MISSION_GRAPH_REVISION_VERSION}:sha256:{}",
            "d".repeat(64)
        ));
        let make = |offset, actor: &str, event| IssueActivity {
            id: format!("20260701T00000{offset}000000Z"),
            subject_kind: "issue".to_string(),
            subject_id: "atelier-m100".to_string(),
            event_type: ActivityEventType::MissionPlanReview,
            actor: actor.to_string(),
            created_at: at(offset),
            summary: "review".to_string(),
            pr_attribution: None,
            workflow_transition: None,
            mission_plan_review: Some(event),
            body: String::new(),
        };
        let request = make(
            0,
            "actor-v1:example.com/same-actor",
            MissionPlanReviewEvent::Request {
                graph_revision: revision.clone(),
                authors: vec!["actor-v1:example.com/same-actor".to_string()],
                material_editors: Vec::new(),
            },
        );
        let whitespace_alias = make(
            1,
            "actor-v1:example.com/same-actor ",
            MissionPlanReviewEvent::Approval {
                graph_revision: revision.clone(),
            },
        );

        let error = project_mission_plan_review(
            "atelier-m100",
            "draft",
            revision,
            &[request, whitespace_alias],
            None,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("canonical stable actor identity"));
        assert!(error.contains("whitespace is not allowed"));
    }

    #[test]
    fn rejects_default_ignorable_and_non_nfc_actor_aliases_before_independence() {
        let revision = MissionGraphRevision(format!(
            "{MISSION_GRAPH_REVISION_VERSION}:sha256:{}",
            "9".repeat(64)
        ));
        let make = |offset, actor: &str, event| IssueActivity {
            id: format!("20260701T00000{offset}000000Z"),
            subject_kind: "issue".to_string(),
            subject_id: "atelier-m100".to_string(),
            event_type: ActivityEventType::MissionPlanReview,
            actor: actor.to_string(),
            created_at: at(offset),
            summary: "review".to_string(),
            pr_attribution: None,
            workflow_transition: None,
            mission_plan_review: Some(event),
            body: String::new(),
        };
        let request = make(
            0,
            "actor-v1:example.com/café",
            MissionPlanReviewEvent::Request {
                graph_revision: revision.clone(),
                authors: vec!["actor-v1:example.com/café".to_string()],
                material_editors: Vec::new(),
            },
        );
        for (alias, code_point) in [
            ("actor-v1:example.com/café\u{200B}", "U+200B"),
            ("actor-v1:example.com/café\u{200D}", "U+200D"),
        ] {
            let approval = make(
                1,
                alias,
                MissionPlanReviewEvent::Approval {
                    graph_revision: revision.clone(),
                },
            );
            let error = project_mission_plan_review(
                "atelier-m100",
                "draft",
                revision.clone(),
                &[request.clone(), approval],
                None,
            )
            .unwrap_err()
            .to_string();
            assert!(error.contains("default-ignorable Unicode"), "{error}");
            assert!(error.contains(code_point), "{error}");
        }

        let decomposed_alias = make(
            1,
            "actor-v1:example.com/cafe\u{301}",
            MissionPlanReviewEvent::Approval {
                graph_revision: revision.clone(),
            },
        );
        let error = project_mission_plan_review(
            "atelier-m100",
            "draft",
            revision,
            &[request, decomposed_alias],
            None,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("is not NFC"), "{error}");
    }

    #[test]
    fn grandfather_requires_exact_cutover_status_and_migration_provenance() {
        let directory = tempdir().unwrap();
        let state_dir = directory.path().join(".atelier");
        std::fs::create_dir_all(&state_dir).unwrap();
        write_graph(&state_dir);
        let revision = mission_graph_revision(&state_dir, "atelier-m100").unwrap();
        let manifest = cutover_manifest("atelier-m100", revision.clone(), at(1));
        write_cutover_manifest(&state_dir, &manifest);
        let activity = create_mission_plan_review_activity(
            &state_dir,
            "atelier-m100",
            LEGACY_GRANDFATHER_MIGRATION_ACTOR,
            at(1),
            "Grandfather active mission at review-policy cutover",
            grandfather_event(&manifest, "atelier-m100"),
            "Created by the versioned mission-review migration.",
        )
        .unwrap();

        let draft_state = mission_plan_review_state(&state_dir, "atelier-m100").unwrap();
        assert_eq!(draft_state.freshness, MissionPlanReviewFreshness::Stale);

        let store = RecordStore::new(&state_dir);
        let mut mission = store.load_issue_by_id("atelier-m100").unwrap();
        mission.issue.status = LEGACY_GRANDFATHER_STATUS.to_string();
        store.write_issue_atomic(&mission).unwrap();
        let active_state = mission_plan_review_state(&state_dir, "atelier-m100").unwrap();
        assert_eq!(
            active_state.freshness,
            MissionPlanReviewFreshness::FreshGrandfather
        );
        let authorization = active_state.authorization.unwrap();
        assert_eq!(
            authorization.legacy_status.as_deref(),
            Some(LEGACY_GRANDFATHER_STATUS)
        );
        assert_eq!(
            authorization.migration_id.as_deref(),
            Some(LEGACY_GRANDFATHER_MIGRATION_ID)
        );
        assert_eq!(
            authorization.cutover_receipt.as_deref(),
            Some(
                manifest
                    .receipt_for(&manifest.eligible_missions[0])
                    .unwrap()
                    .as_str()
            )
        );
        validate_mission_plan_reviews(&state_dir).unwrap();

        let mut invented_activity = activity.clone();
        invented_activity.actor = "actor-v1:example.com/any-actor".to_string();
        let error = project_mission_plan_review(
            "atelier-m100",
            LEGACY_GRANDFATHER_STATUS,
            revision.clone(),
            &[invented_activity],
            Some(&manifest),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("must be produced by migration actor"));

        let bogus_migration = MissionPlanReviewEvent::LegacyGrandfather {
            graph_revision: revision,
            legacy_status: LEGACY_GRANDFATHER_STATUS.to_string(),
            migration_id: "invented-migration".to_string(),
            cutover_receipt: format!(
                "{LEGACY_GRANDFATHER_RECEIPT_VERSION}:sha256:{}",
                "a".repeat(64)
            ),
        };
        assert!(bogus_migration
            .validate()
            .unwrap_err()
            .to_string()
            .contains("migration_id must be"));
    }

    #[test]
    fn rejects_forged_late_duplicate_and_wrong_mission_grandfather_receipts() {
        let revision = MissionGraphRevision(format!(
            "{MISSION_GRAPH_REVISION_VERSION}:sha256:{}",
            "e".repeat(64)
        ));
        let manifest = cutover_manifest("atelier-m100", revision.clone(), at(1));
        let event = grandfather_event(&manifest, "atelier-m100");
        let receipt = IssueActivity {
            id: crate::activity::timestamp_activity_id(at(1)),
            subject_kind: "issue".to_string(),
            subject_id: "atelier-m100".to_string(),
            event_type: ActivityEventType::MissionPlanReview,
            actor: LEGACY_GRANDFATHER_MIGRATION_ACTOR.to_string(),
            created_at: at(1),
            summary: "cutover receipt".to_string(),
            pr_attribution: None,
            workflow_transition: None,
            mission_plan_review: Some(event),
            body: String::new(),
        };

        let error = project_mission_plan_review(
            "atelier-m100",
            LEGACY_GRANDFATHER_STATUS,
            revision.clone(),
            std::slice::from_ref(&receipt),
            None,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("hand-authored or late"));

        let mut late = receipt.clone();
        late.id = crate::activity::timestamp_activity_id(at(2));
        late.created_at = at(2);
        let error = project_mission_plan_review(
            "atelier-m100",
            LEGACY_GRANDFATHER_STATUS,
            revision.clone(),
            &[late],
            Some(&manifest),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("late or forged"));

        let mut post_cutover = receipt.clone();
        post_cutover.created_at = at(2);
        let error = project_mission_plan_review(
            "atelier-m100",
            LEGACY_GRANDFATHER_STATUS,
            revision.clone(),
            &[post_cutover],
            Some(&manifest),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("after or outside exact cutover time"));

        let mut forged = receipt.clone();
        let MissionPlanReviewEvent::LegacyGrandfather {
            cutover_receipt, ..
        } = forged.mission_plan_review.as_mut().unwrap()
        else {
            unreachable!()
        };
        *cutover_receipt = format!(
            "{LEGACY_GRANDFATHER_RECEIPT_VERSION}:sha256:{}",
            "f".repeat(64)
        );
        let error = project_mission_plan_review(
            "atelier-m100",
            LEGACY_GRANDFATHER_STATUS,
            revision.clone(),
            &[forged],
            Some(&manifest),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("forged cutover receipt"));

        let error = project_mission_plan_review(
            "atelier-m100",
            LEGACY_GRANDFATHER_STATUS,
            revision.clone(),
            &[receipt.clone(), receipt.clone()],
            Some(&manifest),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("duplicate legacy-grandfather receipts"));

        let mut wrong_mission = receipt.clone();
        wrong_mission.subject_id = "atelier-m999".to_string();
        let error = project_mission_plan_review(
            "atelier-m999",
            LEGACY_GRANDFATHER_STATUS,
            revision,
            &[wrong_mission],
            Some(&manifest),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("not eligible in the cutover manifest"));

        let error = project_mission_plan_review(
            "atelier-m100",
            LEGACY_GRANDFATHER_STATUS,
            manifest.eligible_missions[0].graph_revision.clone(),
            &[],
            Some(&manifest),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("has no legacy-grandfather receipt"));

        let mut duplicate_eligibility = manifest.clone();
        duplicate_eligibility
            .eligible_missions
            .push(duplicate_eligibility.eligible_missions[0].clone());
        let error = duplicate_eligibility.validate().unwrap_err().to_string();
        assert!(error.contains("sorted by mission_id with no duplicates"));
    }

    #[test]
    fn cutover_manifest_rejects_sub_microsecond_precision_before_receipt_hashing() {
        let revision = MissionGraphRevision(format!(
            "{MISSION_GRAPH_REVISION_VERSION}:sha256:{}",
            "e".repeat(64)
        ));
        let cutover_at = DateTime::parse_from_rfc3339("2026-05-28T20:26:41.123456789Z")
            .unwrap()
            .with_timezone(&Utc);
        let manifest = cutover_manifest("atelier-m100", revision, cutover_at);

        let error = manifest.validate().unwrap_err();
        let error = format!("{error:#}");
        assert!(error.contains("must use microsecond precision"), "{error}");
        let error = manifest
            .receipt_for(&manifest.eligible_missions[0])
            .unwrap_err();
        let error = format!("{error:#}");
        assert!(error.contains("must use microsecond precision"), "{error}");
    }

    #[test]
    fn rejects_grandfather_receipt_with_wrong_revision_or_legacy_status() {
        let eligible_revision = MissionGraphRevision(format!(
            "{MISSION_GRAPH_REVISION_VERSION}:sha256:{}",
            "e".repeat(64)
        ));
        let wrong_revision = MissionGraphRevision(format!(
            "{MISSION_GRAPH_REVISION_VERSION}:sha256:{}",
            "d".repeat(64)
        ));
        let manifest = cutover_manifest("atelier-m100", eligible_revision.clone(), at(1));
        let eligibility = &manifest.eligible_missions[0];
        let receipt = IssueActivity {
            id: eligibility.receipt_activity_id.clone(),
            subject_kind: "issue".to_string(),
            subject_id: "atelier-m100".to_string(),
            event_type: ActivityEventType::MissionPlanReview,
            actor: LEGACY_GRANDFATHER_MIGRATION_ACTOR.to_string(),
            created_at: manifest.cutover_at,
            summary: "cutover receipt".to_string(),
            pr_attribution: None,
            workflow_transition: None,
            mission_plan_review: None,
            body: String::new(),
        };
        let cutover_receipt = manifest.receipt_for(eligibility).unwrap();

        let error = validate_legacy_grandfather_receipt(
            &manifest,
            eligibility,
            &receipt,
            &wrong_revision,
            LEGACY_GRANDFATHER_STATUS,
            LEGACY_GRANDFATHER_MIGRATION_ID,
            &cutover_receipt,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("names graph revision"), "{error}");

        let error = validate_legacy_grandfather_receipt(
            &manifest,
            eligibility,
            &receipt,
            &eligible_revision,
            "draft",
            LEGACY_GRANDFATHER_MIGRATION_ID,
            &cutover_receipt,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("names legacy status 'draft'"), "{error}");
    }

    #[test]
    fn rejects_malformed_typed_activity_metadata() {
        let relative = crate::activity::record_activity_path(
            "issue",
            "atelier-m100",
            "20260701T000000000000Z",
        );
        let malformed = r#"---
schema: "atelier.activity"
schema_version: 1
id: "20260701T000000000000Z"
subject_kind: "issue"
subject_id: "atelier-m100"
event_type: "mission_plan_review"
actor: "actor-v1:example.com/reviewer"
created_at: "2026-07-01T00:00:00.000000Z"
summary: "Approval"
---
"#;
        let error = IssueActivity::from_markdown(malformed, &relative)
            .unwrap_err()
            .to_string();
        assert!(error.contains("missing mission_plan_review metadata"));

        let mut invalid = MissionPlanReviewEvent::Approval {
            graph_revision: MissionGraphRevision("unversioned".to_string()),
        };
        assert!(invalid
            .validate()
            .unwrap_err()
            .to_string()
            .contains("must start"));
        invalid = MissionPlanReviewEvent::Request {
            graph_revision: MissionGraphRevision(format!(
                "{MISSION_GRAPH_REVISION_VERSION}:sha256:{}",
                "b".repeat(64)
            )),
            authors: Vec::new(),
            material_editors: Vec::new(),
        };
        assert!(invalid
            .validate()
            .unwrap_err()
            .to_string()
            .contains("authors must contain"));
    }
}
