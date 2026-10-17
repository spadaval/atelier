//! In-memory index over the tracker's record files.
//!
//! Every command parses the record files it needs and answers queries from
//! this index; there is no persistent cache. Loading validates record
//! integrity (parse, unique ids, resolvable references, hierarchy shape, and
//! acyclic graphs) so queries never see a partially valid graph.

use anyhow::{anyhow, bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use atelier_core::{EvidenceRecord, Issue, RecordLink, Relation, ReviewEvent, ReviewRecord};
use atelier_records::{
    canonical_record_dirs, is_local_atelier_path, parse_evidence_record_file, parse_issue_record,
    validate_record_id, validate_record_kind, CanonicalIssueRecord, Relationships,
    WELL_KNOWN_LINK_TYPES, WELL_KNOWN_RELATION_TYPES,
};

use crate::record_ref::{self, RecordKind};
use crate::workflow_policy::{WorkflowPolicy, WORKFLOW_POLICY_PATH};

/// Record files read from one state directory, including the files that
/// failed to parse.
#[derive(Debug, Default)]
pub struct RecordFiles {
    pub issues: Vec<CanonicalIssueRecord>,
    pub evidence: Vec<EvidenceRecord>,
    pub reviews: Vec<ReviewRecord>,
    /// One error per unreadable, unparsable, or unsupported file, in path
    /// order. Each error names the file.
    pub errors: Vec<RecordFileError>,
}
impl RecordFiles {
    pub fn into_index(self) -> Result<RecordIndex> {
        if let Some(error) = self.errors.into_iter().next() {
            return Err(error.error);
        }
        RecordIndex::from_records(self.issues, self.evidence, self.reviews)
    }
}

#[derive(Debug)]
pub struct RecordFileError {
    /// Path relative to the state directory.
    pub path: PathBuf,
    pub error: anyhow::Error,
}

/// Reads and parses every record file under `state_dir` without stopping at
/// the first bad file.
pub fn read_record_files(state_dir: &Path) -> Result<RecordFiles> {
    if !state_dir.is_dir() {
        return Ok(RecordFiles::default());
    }
    let mut paths = Vec::new();
    collect_files(state_dir, state_dir, &mut paths)?;
    paths.sort();
    read_record_paths(state_dir, paths, &|_| Ok((true, None))).map(|(files, _)| files)
}

/// Raw errors and mismatches remain separate from parser errors until the
/// caller has completed its index checks. Parsing consumes those same bytes.
pub(crate) fn read_record_paths(
    state_dir: &Path,
    paths: Vec<PathBuf>,
    read: &impl Fn(&Path) -> Result<(bool, Option<Vec<u8>>)>,
) -> Result<(RecordFiles, Vec<PathBuf>)> {
    let mut files = RecordFiles::default();
    let mut changed = Vec::new();
    for relative in paths {
        let (matches, bytes) = read(&relative)?;
        let result = if matches {
            read_record_file(state_dir, &relative, bytes)
        } else {
            Ok(None)
        };
        if !matches {
            changed.push(relative.clone());
        }
        match result {
            Ok(Some(ParsedRecord::Issue(record))) => files.issues.push(*record),
            Ok(Some(ParsedRecord::Evidence(record))) => files.evidence.push(*record),
            Ok(Some(ParsedRecord::Review(record))) => files.reviews.push(*record),
            Ok(None) => {}
            Err(error) => files.errors.push(RecordFileError {
                path: relative.clone(),
                error: error.context(format!(
                    "Invalid record file {}",
                    display_state_path(&relative)
                )),
            }),
        }
    }
    Ok((files, changed))
}

enum ParsedRecord {
    Issue(Box<CanonicalIssueRecord>),
    Evidence(Box<EvidenceRecord>),
    Review(Box<ReviewRecord>),
}

fn read_record_file(
    state_dir: &Path,
    relative: &Path,
    bytes: Option<Vec<u8>>,
) -> Result<Option<ParsedRecord>> {
    let top = relative
        .components()
        .next()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .unwrap_or_default();
    let in_record_dir =
        relative.components().count() > 1 && canonical_record_dirs().iter().any(|dir| *dir == top);
    if !in_record_dir {
        if matches!(
            relative.to_str(),
            Some("workflow.yaml" | "manifest.json" | "graph.json" | "mission-control.json")
        ) {
            return Ok(None);
        }
        bail!("Unsupported record file {}", display_state_path(relative));
    }
    if relative.extension().and_then(|ext| ext.to_str()) != Some("md") {
        let kind = if top == "issues" {
            "issue"
        } else {
            top.as_str()
        };
        bail!(
            "Unsupported {kind} record file {}; expected .md record",
            display_state_path(relative)
        );
    }
    let bytes = match bytes {
        Some(bytes) => bytes,
        None => fs::read(state_dir.join(relative))
            .with_context(|| format!("Failed to read {}", display_state_path(relative)))?,
    };
    let text = String::from_utf8(bytes)
        .with_context(|| format!("Record file {} is not UTF-8", display_state_path(relative)))?;
    Ok(Some(if top == "issues" {
        ParsedRecord::Issue(Box::new(parse_issue_record(&text, relative)?))
    } else if top == "reviews" {
        ParsedRecord::Review(Box::new(atelier_records::review::parse_review_record(
            &text, relative,
        )?))
    } else {
        ParsedRecord::Evidence(Box::new(parse_evidence_record_file(&text, relative)?))
    }))
}

fn collect_files(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .context("Failed to relativize record-file path")?;
        if is_local_atelier_path(relative) {
            continue;
        }
        // Everything except a real directory is read as a record file, so a
        // symlink is followed when it points at a file and is reported as an
        // invalid record file otherwise (dangling, or a linked directory,
        // which is not traversed to avoid link cycles). Nothing is skipped.
        if entry.file_type()?.is_dir() {
            collect_files(root, &path, files)?;
        } else {
            files.push(relative.to_path_buf());
        }
    }
    Ok(())
}

fn display_state_path(relative: &Path) -> String {
    format!(".atelier/{}", relative.to_string_lossy().replace('\\', "/"))
}

/// Queryable graph of every issue and evidence record.
#[derive(Debug, Default)]
pub struct RecordIndex {
    issues: BTreeMap<String, CanonicalIssueRecord>,
    /// Issue ids ordered by most recently updated first, then id.
    issues_by_recency: Vec<String>,
    /// Parent id to child ids, in recency order.
    children: BTreeMap<String, Vec<String>>,
    /// Blocked issue id to sorted blocker ids.
    blockers: BTreeMap<String, Vec<String>>,
    /// Blocker issue id to sorted blocked ids.
    blocking: BTreeMap<String, Vec<String>>,
    /// Issue id to every issue-to-issue relation touching it.
    relations: BTreeMap<String, Vec<Relation>>,
    evidence: BTreeMap<String, EvidenceRecord>,
    reviews: BTreeMap<String, ReviewRecord>,
    /// Evidence attachments and issue-to-issue relations, sorted.
    links: Vec<RecordLink>,
    /// `(kind, id)` to indices into `links` touching that record.
    links_by_record: BTreeMap<(String, String), Vec<usize>>,
}

/// A reference to a missing record often means prune removed it while a
/// record on another branch still linked it (atelier-r7cd). Say so, and how to
/// recover, when Git history shows the record was deleted.
fn explain_pruned_reference(repo_root: &Path, error: anyhow::Error) -> anyhow::Error {
    let message = error.to_string();
    let Some((_, rest)) = message.rsplit_once(" reference to missing ") else {
        return error;
    };
    let Some((kind, id)) = rest.trim().split_once(' ') else {
        return error;
    };
    let dir = match kind {
        "issue" => "issues",
        "evidence" => "evidence",
        "review" => "reviews",
        _ => return error,
    };
    let path = format!(".atelier/{dir}/{id}.md");
    let git = |args: &[&str]| {
        crate::git_freshness::git_output(repo_root, args)
            .ok()
            .and_then(|out| String::from_utf8(out).ok())
            .map(|out| out.trim().to_string())
    };
    let unlink = "or remove the link: delete it from the linking issue's relationships, or remove a review or evidence record whose only target is gone. Commands cannot load the records until one of these is done";
    if git(&["cat-file", "-t", &format!("HEAD:{path}")]).is_some() {
        return anyhow!(
            "{message}; {id} is committed at HEAD but missing from the checkout. Restore it with `git restore -- ':/{path}'`, {unlink}"
        );
    }
    match git(&["log", "-1", "--diff-filter=D", "--format=%h", "--", &path])
        .filter(|commit| !commit.is_empty())
    {
        Some(commit) => anyhow!(
            "{message}; {id} was deleted in commit {commit} (for example by atelier prune). Restore it with `git restore --source={commit}^ -- ':/{path}'`, {unlink}"
        ),
        None => error,
    }
}

impl RecordIndex {
    /// Loads and validates every record under `state_dir`, failing on the
    /// first invalid record file and naming it.
    pub fn load(state_dir: &Path) -> Result<Self> {
        let files = read_record_files(state_dir)?;
        if let Some(first) = files.errors.into_iter().next() {
            return Err(first.error);
        }
        Self::from_project_records(state_dir, files.issues, files.evidence, files.reviews)
    }

    /// Builds an index from already-parsed records and applies the
    /// repository's link-type configuration and workflow policy. Fails with
    /// the first error; `validation_errors` reports them all.
    pub fn from_project_records(
        state_dir: &Path,
        issues: Vec<CanonicalIssueRecord>,
        evidence: Vec<EvidenceRecord>,
        reviews: Vec<ReviewRecord>,
    ) -> Result<Self> {
        let (index, errors) = Self::validated(state_dir, issues, evidence, reviews);
        match errors.into_iter().next() {
            Some(error) => Err(error),
            None => Ok(index),
        }
    }

    /// Every independent validation error, in the order `from_project_records`
    /// would meet them. Invalid edges are skipped so later checks still run.
    pub fn validation_errors(
        state_dir: &Path,
        issues: Vec<CanonicalIssueRecord>,
        evidence: Vec<EvidenceRecord>,
        reviews: Vec<ReviewRecord>,
    ) -> Vec<anyhow::Error> {
        Self::validated(state_dir, issues, evidence, reviews).1
    }

    fn validated(
        state_dir: &Path,
        issues: Vec<CanonicalIssueRecord>,
        evidence: Vec<EvidenceRecord>,
        reviews: Vec<ReviewRecord>,
    ) -> (Self, Vec<anyhow::Error>) {
        let (index, mut errors) = Self::structural(issues, evidence, reviews);
        if let Some(repo_root) = state_dir.parent() {
            errors = errors
                .into_iter()
                .map(|error| explain_pruned_reference(repo_root, error))
                .collect();
        }
        let Some(repo_root) = state_dir.parent() else {
            errors.push(anyhow!(
                "Cannot determine repository root for {}",
                state_dir.display()
            ));
            return (index, errors);
        };
        index.validate_link_types(repo_root, state_dir, &mut errors);
        index.validate_against_policy(repo_root, &mut errors);
        (index, errors)
    }

    /// Builds an index and validates the record graph's structure.
    pub fn from_records(
        issues: Vec<CanonicalIssueRecord>,
        evidence: Vec<EvidenceRecord>,
        reviews: Vec<ReviewRecord>,
    ) -> Result<Self> {
        let (index, errors) = Self::structural(issues, evidence, reviews);
        match errors.into_iter().next() {
            Some(error) => Err(error),
            None => Ok(index),
        }
    }

    fn structural(
        issues: Vec<CanonicalIssueRecord>,
        evidence: Vec<EvidenceRecord>,
        reviews: Vec<ReviewRecord>,
    ) -> (Self, Vec<anyhow::Error>) {
        let mut index = Self::default();
        let mut errors = Vec::new();
        let mut global_ids = BTreeSet::new();
        let mut dropped = BTreeSet::new();
        for record in issues {
            let id = record.issue.id.clone();
            if !global_ids.insert(id.clone()) {
                errors.push(anyhow!("Duplicate issue ID in record files: {id}"));
                dropped.insert(("issue".to_owned(), id));
                continue;
            }
            index.issues.insert(id, record);
        }
        for record in evidence {
            let id = record.header.id.clone();
            if !global_ids.insert(id.clone()) {
                errors.push(anyhow!("Duplicate record ID in record files: {id}"));
                dropped.insert(("evidence".to_owned(), id));
                continue;
            }
            index.evidence.insert(id, record);
        }
        for record in reviews {
            if !global_ids.insert(record.header.id.clone()) {
                errors.push(anyhow!(
                    "Duplicate record ID in record files: {}",
                    record.header.id
                ));
                dropped.insert(("review".to_owned(), record.header.id.clone()));
                continue;
            }
            index.reviews.insert(record.header.id.clone(), record);
        }
        index.validate_reviews(&mut errors);
        index.index_issue_graph(&mut errors);
        index.index_record_links(&mut errors);
        index.validate_hierarchy_shapes(&mut errors);
        for (edges, relation) in [(&index.children, "children"), (&index.blocking, "blocks")] {
            if let Err(error) = validate_acyclic(edges, relation) {
                errors.push(error);
            }
        }
        index.order();
        // A duplicate ID drops a record; references to that kind and ID are
        // not also reported missing (atelier-ip9e).
        errors.retain(|error| {
            !error
                .to_string()
                .rsplit_once(" reference to missing ")
                .and_then(|(_, target)| target.split_once(' '))
                .is_some_and(|(kind, id)| dropped.contains(&(kind.to_owned(), id.to_owned())))
        });
        (index, errors)
    }

    /// Structural load for guards which resolve trusted policy before validation.
    pub fn load_structural(state_dir: &Path) -> Result<Self> {
        read_record_files(state_dir)?
            .into_index()
            .map_err(|error| match state_dir.parent() {
                Some(repo_root) => explain_pruned_reference(repo_root, error),
                None => error,
            })
    }
    pub fn validate_policy(&self, policy: &WorkflowPolicy, repo: &Path) -> Result<()> {
        let mut errors = Vec::new();
        self.validate_link_types(repo, &repo.join(".atelier"), &mut errors);
        if let Some(error) = errors.into_iter().next() {
            return Err(error);
        }
        for record in self.issues.values() {
            crate::workflow_policy::validate_issue_against_policy(
                policy,
                &record.issue,
                &repo.join(WORKFLOW_POLICY_PATH),
            )?;
        }
        self.validate_mission_dependencies(policy)
    }
    pub fn reviews(&self) -> impl Iterator<Item = &ReviewRecord> {
        self.reviews.values()
    }
    pub fn require_review(&self, id: &str) -> Result<&ReviewRecord> {
        self.reviews
            .get(id)
            .ok_or_else(|| self.not_found(RecordKind::Review, id))
    }
    pub fn latest_disposition(&self, finding: &str) -> Option<&ReviewRecord> {
        self.reviews.values().find(|record| matches!(&record.event, ReviewEvent::Disposition { finding: target, .. } if target == finding) && !self.reviews.values().any(|successor| matches!(&successor.event, ReviewEvent::Disposition { supersedes: Some(previous), .. } if previous == &record.header.id)))
    }
    fn validate_reviews(&self, errors: &mut Vec<anyhow::Error>) {
        let mut successors = BTreeMap::new();
        let mut chains = BTreeMap::<String, Vec<String>>::new();
        let mut roots = BTreeSet::new();
        for record in self.reviews.values() {
            if let Err(error) =
                self.validate_review(record, &mut successors, &mut chains, &mut roots)
            {
                errors.push(error);
            }
        }
        if let Err(error) = validate_acyclic(&chains, "disposition supersedes") {
            errors.push(error);
        }
    }

    fn validate_review<'a>(
        &self,
        record: &'a ReviewRecord,
        successors: &mut BTreeMap<&'a String, &'a String>,
        chains: &mut BTreeMap<String, Vec<String>>,
        roots: &mut BTreeSet<&'a String>,
    ) -> Result<()> {
        self.ensure_issue(&record.target, "review target", &record.header.id)?;
        if let ReviewEvent::Disposition {
            finding,
            supersedes,
            follow_up,
            ..
        } = &record.event
        {
            let original = self.require_review(finding)?;
            if !matches!(original.event, ReviewEvent::Finding { .. })
                || original.target != record.target
            {
                bail!(
                    "disposition {} must reference a finding for the same target",
                    record.header.id
                );
            }
            if let Some(issue) = follow_up {
                self.ensure_issue(issue, "follow-up", &record.header.id)?;
            }
            if let Some(previous) = supersedes {
                let previous_record = self.require_review(previous)?;
                if !matches!(&previous_record.event, ReviewEvent::Disposition { finding: other, .. } if other == finding)
                {
                    bail!("disposition supersedes a decision for another finding");
                }
                if successors.insert(previous, &record.header.id).is_some() {
                    bail!("competing disposition successors for {previous}");
                }
                chains
                    .entry(record.header.id.clone())
                    .or_default()
                    .push(previous.clone());
            } else if !roots.insert(finding) {
                bail!("competing disposition roots for {finding}");
            }
        }
        Ok(())
    }

    fn index_issue_graph(&mut self, errors: &mut Vec<anyhow::Error>) {
        let mut parents = BTreeMap::<String, String>::new();
        let mut relation_keys = BTreeSet::new();
        for record in self.issues.values() {
            let id = &record.issue.id;
            for blocked in &record.relationships.blocks {
                if blocked.kind != "issue" {
                    errors.push(anyhow!(
                        "Issue {id} has blocks target {} {}; blocks must target issue records",
                        blocked.kind,
                        blocked.id
                    ));
                    continue;
                }
                if let Err(error) = self.ensure_issue(&blocked.id, "blocks", id) {
                    errors.push(error);
                    continue;
                }
                let blocking = self.blocking.entry(id.clone()).or_default();
                if blocking.contains(&blocked.id) {
                    errors.push(anyhow!("Duplicate blocks edge {id} blocks {}", blocked.id));
                    continue;
                }
                blocking.push(blocked.id.clone());
                self.blockers
                    .entry(blocked.id.clone())
                    .or_default()
                    .push(id.clone());
            }
            for child in &record.relationships.children {
                if child.kind != "issue" {
                    errors.push(anyhow!(
                        "Issue {id} has children target {} {}; issue hierarchy children must target issue records",
                        child.kind,
                        child.id
                    ));
                    continue;
                }
                if let Err(error) = self.ensure_issue(&child.id, "children", id) {
                    errors.push(error);
                    continue;
                }
                let children = self.children.entry(id.clone()).or_default();
                if children.contains(&child.id) {
                    errors.push(anyhow!(
                        "Duplicate children edge {id} contains {}",
                        child.id
                    ));
                    continue;
                }
                if let Some(existing) = parents.get(&child.id) {
                    errors.push(anyhow!(
                        "Issue {} appears under multiple parents: {existing}, {id}",
                        child.id
                    ));
                    continue;
                }
                children.push(child.id.clone());
                parents.insert(child.id.clone(), id.clone());
            }
            for relation in &record.relationships.relates {
                if let Err(error) =
                    self.ensure_record(&relation.kind, &relation.id, &relation.relation_type, id)
                {
                    errors.push(error);
                    continue;
                }
                if relation.kind != "issue" {
                    continue;
                }
                let key = (
                    id.clone(),
                    relation.id.clone(),
                    relation.relation_type.clone(),
                );
                if !relation_keys.insert(key) {
                    errors.push(anyhow!(
                        "Duplicate typed link {id} -> {} ({})",
                        relation.id,
                        relation.relation_type
                    ));
                    continue;
                }
                let relation = Relation {
                    issue_id_1: id.clone(),
                    issue_id_2: relation.id.clone(),
                    relation_type: relation.relation_type.clone(),
                    created_at: record.issue.updated_at,
                };
                for endpoint in [&relation.issue_id_1, &relation.issue_id_2] {
                    self.relations
                        .entry(endpoint.clone())
                        .or_default()
                        .push(relation.clone());
                }
            }
        }
        for (child, parent) in parents {
            if let Some(record) = self.issues.get_mut(&child) {
                record.issue.parent_id = Some(parent);
            }
        }
    }

    fn index_record_links(&mut self, errors: &mut Vec<anyhow::Error>) {
        let mut keys = BTreeSet::new();
        let mut links = Vec::new();
        for record in self.issues.values() {
            self.collect_links(
                "issue",
                &record.issue.id,
                &record.relationships,
                record.issue.updated_at,
                &mut keys,
                &mut links,
                errors,
            );
        }
        for record in self.evidence.values() {
            self.collect_links(
                "evidence",
                &record.header.id,
                &record.header.relationships,
                record.header.created_at,
                &mut keys,
                &mut links,
                errors,
            );
        }
        for record in self.reviews.values() {
            self.collect_links(
                "review",
                &record.header.id,
                &record.header.relationships,
                record.header.created_at,
                &mut keys,
                &mut links,
                errors,
            );
        }
        links.sort_by(|left, right| link_key(left).cmp(&link_key(right)));
        for (position, link) in links.iter().enumerate() {
            for endpoint in [
                (link.source_kind.clone(), link.source_id.clone()),
                (link.target_kind.clone(), link.target_id.clone()),
            ] {
                self.links_by_record
                    .entry(endpoint)
                    .or_default()
                    .push(position);
            }
        }
        self.links = links;
    }

    /// Validates every relationship target and collects the links that
    /// queries see: evidence attachments and issue-to-issue relations. Issue
    /// children and relates targets were already checked by
    /// `index_issue_graph`, so missing ones are skipped here, not reported twice.
    #[allow(clippy::too_many_arguments)]
    fn collect_links(
        &self,
        source_kind: &str,
        source_id: &str,
        relationships: &Relationships,
        created_at: chrono::DateTime<chrono::Utc>,
        keys: &mut BTreeSet<(String, String, String, String, String)>,
        links: &mut Vec<RecordLink>,
        errors: &mut Vec<anyhow::Error>,
    ) {
        let mut push = |target_kind: &str, target_id: &str, relation_type: &str, queried| {
            let key = (
                source_kind.to_string(),
                source_id.to_string(),
                target_kind.to_string(),
                target_id.to_string(),
                relation_type.to_string(),
            );
            if !keys.insert(key) {
                bail!(
                    "Duplicate relationship {source_kind} {source_id} -> {target_kind} {target_id} ({relation_type})"
                );
            }
            if queried {
                links.push(RecordLink {
                    source_kind: source_kind.to_string(),
                    source_id: source_id.to_string(),
                    target_kind: target_kind.to_string(),
                    target_id: target_id.to_string(),
                    relation_type: relation_type.to_string(),
                    created_at,
                });
            }
            Ok(())
        };
        let checked_by_issue_graph = source_kind == "issue";
        // A missing target is reported once, here or by `index_issue_graph`.
        let exists =
            |kind: &str, id: &str, relation: &str, checked: bool, errors: &mut Vec<_>| match self
                .ensure_record(kind, id, relation, source_id)
            {
                Ok(()) => true,
                Err(error) => {
                    if !checked {
                        errors.push(error);
                    }
                    false
                }
            };
        for child in &relationships.children {
            if exists(
                &child.kind,
                &child.id,
                "children",
                checked_by_issue_graph,
                errors,
            ) && !(source_kind == "issue" && child.kind == "issue")
            {
                if let Err(error) = push(&child.kind, &child.id, "advances", false) {
                    errors.push(error);
                }
            }
        }
        for attachment in &relationships.attachments {
            if exists(
                &attachment.kind,
                &attachment.id,
                &attachment.role,
                false,
                errors,
            ) {
                if let Err(error) = push(
                    &attachment.kind,
                    &attachment.id,
                    &attachment.role,
                    matches!(source_kind, "evidence" | "review"),
                ) {
                    errors.push(error);
                }
            }
        }
        for relation in &relationships.relates {
            if exists(
                &relation.kind,
                &relation.id,
                &relation.relation_type,
                checked_by_issue_graph,
                errors,
            ) {
                let issue_relation = source_kind == "issue" && relation.kind == "issue";
                if let Err(error) = push(
                    &relation.kind,
                    &relation.id,
                    &relation.relation_type,
                    issue_relation,
                ) {
                    errors.push(error);
                }
            }
        }
    }

    fn ensure_issue(&self, id: &str, relation: &str, source_id: &str) -> Result<()> {
        if self.issues.contains_key(id) {
            Ok(())
        } else {
            bail!("Issue {source_id} has {relation} reference to missing issue {id}")
        }
    }

    fn ensure_record(&self, kind: &str, id: &str, relation: &str, source_id: &str) -> Result<()> {
        if self.record_exists(kind, id) {
            Ok(())
        } else {
            bail!("{source_id} has {relation} reference to missing {kind} {id}")
        }
    }

    fn validate_hierarchy_shapes(&self, errors: &mut Vec<anyhow::Error>) {
        for (parent_id, children) in &self.children {
            let parent_type = &self.issues[parent_id].issue.issue_type;
            for child_id in children {
                let child_type = &self.issues[child_id].issue.issue_type;
                if child_type == "mission" {
                    errors.push(anyhow!(
                        "workflow_issue_hierarchy_invalid: mission issue {child_id} cannot have parent {parent_id}; link mission work with advances relationships"
                    ));
                } else if parent_type == "mission" {
                    errors.push(anyhow!(
                        "workflow_issue_hierarchy_invalid: issue {child_id} cannot be child of mission {parent_id}; link mission work with advances relationships"
                    ));
                } else if self.children.contains_key(child_id) {
                    errors.push(anyhow!(
                        "workflow_issue_hierarchy_invalid: issue {child_id} is a child of {parent_id} and has children of its own; parents are one level deep"
                    ));
                }
            }
        }
    }

    fn order(&mut self) {
        let mut ids = self.issues.keys().cloned().collect::<Vec<_>>();
        ids.sort_by(|left, right| {
            self.issues[right]
                .issue
                .updated_at
                .cmp(&self.issues[left].issue.updated_at)
                .then_with(|| left.cmp(right))
        });
        let rank = ids
            .iter()
            .enumerate()
            .map(|(rank, id)| (id.clone(), rank))
            .collect::<BTreeMap<_, _>>();
        for children in self.children.values_mut() {
            children.sort_by_key(|id| rank[id]);
        }
        for ids in self.blockers.values_mut().chain(self.blocking.values_mut()) {
            ids.sort();
        }
        for relations in self.relations.values_mut() {
            relations.sort_by(|left, right| {
                (
                    left.created_at,
                    &left.issue_id_1,
                    &left.issue_id_2,
                    &left.relation_type,
                )
                    .cmp(&(
                        right.created_at,
                        &right.issue_id_1,
                        &right.issue_id_2,
                        &right.relation_type,
                    ))
            });
        }
        self.issues_by_recency = ids;
    }

    fn validate_link_types(
        &self,
        repo_root: &Path,
        state_dir: &Path,
        errors: &mut Vec<anyhow::Error>,
    ) {
        let custom = if state_dir.join("config.toml").exists() {
            match crate::project_config::ProjectConfig::load(repo_root) {
                Ok(config) => config
                    .issue_links
                    .custom_context_types
                    .into_iter()
                    .collect(),
                Err(error) => {
                    errors.push(error);
                    return;
                }
            }
        } else {
            BTreeSet::new()
        };
        for record in self.issues.values() {
            for relation in &record.relationships.relates {
                if let Err(error) =
                    validate_issue_link_type(&record.issue.id, &relation.relation_type, &custom)
                {
                    errors.push(error);
                }
            }
        }
    }

    fn validate_against_policy(&self, repo_root: &Path, errors: &mut Vec<anyhow::Error>) {
        let policy_path = repo_root.join(WORKFLOW_POLICY_PATH);
        let policy_exists = policy_path.exists();
        if !policy_exists
            && self.issues.values().all(|record| {
                record.issue.strategy.is_none() && record.issue.workflow_branch.is_none()
            })
        {
            return;
        }
        let policy = match crate::workflow_policy::load(repo_root) {
            Ok(policy) => policy,
            Err(error) => {
                errors.push(error);
                return;
            }
        };
        for record in self.issues.values() {
            if let Err(error) = crate::workflow_policy::validate_issue_against_policy(
                &policy,
                &record.issue,
                &policy_path,
            ) {
                errors.push(error);
            }
        }
        if policy_exists {
            errors.extend(self.mission_dependency_errors(&policy));
        }
    }

    fn validate_mission_dependencies(&self, policy: &WorkflowPolicy) -> Result<()> {
        match self.mission_dependency_errors(policy).into_iter().next() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    fn mission_dependency_errors(&self, policy: &WorkflowPolicy) -> Vec<anyhow::Error> {
        let mut errors = Vec::new();
        for record in self.issues.values() {
            let issue = &record.issue;
            if issue.issue_type == "mission"
                && matches!(issue.status.as_str(), "ready" | "in_progress")
            {
                match crate::objective_graph::dependency_closure(self, policy, &issue.id) {
                    Ok(closure) => {
                        if let Some(reason) = closure.failure_reason() {
                            errors.push(anyhow!(
                                "workflow_mission_dependency_bypass: mission {} has executable status '{}' despite {reason}",
                                issue.id,
                                issue.status
                            ));
                        }
                    }
                    Err(error) => errors.push(error),
                }
            }
        }
        errors
    }

    // ----- Issue queries -------------------------------------------------

    pub fn issue_record(&self, id: &str) -> Option<&CanonicalIssueRecord> {
        self.issues.get(id)
    }

    /// Every issue record, ordered by id.
    pub fn issue_records(&self) -> impl Iterator<Item = &CanonicalIssueRecord> {
        self.issues.values()
    }

    pub fn get_issue(&self, id: &str) -> Option<Issue> {
        self.issues.get(id).map(|record| record.issue.clone())
    }

    pub fn require_issue(&self, id: &str) -> Result<Issue> {
        self.get_issue(id)
            .ok_or_else(|| self.not_found(RecordKind::Issue, id))
    }

    /// Resolves a full issue id or a unique short id (the part after the
    /// project slug), ignoring case. Bundle files use this to tell existing
    /// issues from their own references; commands use [`Self::resolve`].
    pub fn resolve_issue_ref(&self, issue_ref: &str) -> Result<Option<String>> {
        record_ref::match_id(RecordKind::Issue, issue_ref, self.ids(RecordKind::Issue))
    }

    /// Resolves the record a command names by full or short id. A record of
    /// another kind gets a hint naming its own show command; an unknown id
    /// gets the shared not-found message.
    pub fn resolve(&self, kind: RecordKind, reference: &str) -> Result<String> {
        if let Some(id) = record_ref::match_id(kind, reference, self.ids(kind))? {
            return Ok(id);
        }
        for other in RecordKind::ALL.into_iter().filter(|other| *other != kind) {
            if let Ok(Some(id)) = record_ref::match_id(other, reference, self.ids(other)) {
                return Err(record_ref::wrong_kind(kind, other, &id));
            }
        }
        Err(self.not_found(kind, reference))
    }

    /// The shared not-found message, with near matches among this kind's ids.
    pub fn not_found(&self, kind: RecordKind, reference: &str) -> anyhow::Error {
        record_ref::not_found(kind, reference, self.ids(kind))
    }

    fn ids(&self, kind: RecordKind) -> Box<dyn Iterator<Item = &str> + '_> {
        match kind {
            RecordKind::Issue => Box::new(self.issues.keys().map(String::as_str)),
            RecordKind::Evidence => Box::new(self.evidence.keys().map(String::as_str)),
            RecordKind::Review => Box::new(self.reviews.keys().map(String::as_str)),
        }
    }

    /// All issues, most recently updated first.
    pub fn issues(&self) -> impl Iterator<Item = &Issue> {
        self.issues_by_recency
            .iter()
            .map(|id| &self.issues[id].issue)
    }

    pub fn get_subissues(&self, parent_id: &str) -> Vec<Issue> {
        self.child_ids(parent_id)
            .iter()
            .map(|id| self.issues[id].issue.clone())
            .collect()
    }

    pub fn child_ids(&self, parent_id: &str) -> &[String] {
        self.children.get(parent_id).map_or(&[], Vec::as_slice)
    }

    /// Issues that block `issue_id`, sorted by id.
    pub fn get_blockers(&self, issue_id: &str) -> Vec<String> {
        self.blockers.get(issue_id).cloned().unwrap_or_default()
    }

    /// Issues that `issue_id` blocks, sorted by id.
    pub fn get_blocking(&self, issue_id: &str) -> Vec<String> {
        self.blocking.get(issue_id).cloned().unwrap_or_default()
    }

    /// Every issue-to-issue relation touching `issue_id`. `issue_id_1` is the
    /// issue whose record declares the relation.
    pub fn get_typed_relations(&self, issue_id: &str) -> Vec<Relation> {
        self.relations.get(issue_id).cloned().unwrap_or_default()
    }

    /// Issues downstream of `source_id`: children and `derived`, `caused-by`,
    /// and `falsifies` relations transitively, plus `assumption` relations one
    /// hop from the source.
    pub fn downstream_impact(&self, source_id: &str) -> Vec<Issue> {
        const TRANSITIVE: &[&str] = &["derived", "caused-by", "falsifies"];
        let mut visited = BTreeSet::from([source_id.to_string()]);
        let mut queue = std::collections::VecDeque::from([source_id.to_string()]);
        let mut affected = Vec::new();
        while let Some(current) = queue.pop_front() {
            let mut next = self.child_ids(&current).to_vec();
            next.extend(self.related_ids(&current, TRANSITIVE));
            for id in next {
                if visited.insert(id.clone()) {
                    affected.push(self.issues[&id].issue.clone());
                    queue.push_back(id);
                }
            }
            if current == source_id {
                for id in self.related_ids(&current, &["assumption"]) {
                    if visited.insert(id.clone()) {
                        affected.push(self.issues[&id].issue.clone());
                    }
                }
            }
        }
        affected
    }

    fn related_ids(&self, issue_id: &str, relation_types: &[&str]) -> Vec<String> {
        let mut ids = Vec::new();
        for relation_type in relation_types {
            let mut linked = self
                .relations
                .get(issue_id)
                .into_iter()
                .flatten()
                .filter(|relation| relation.relation_type == *relation_type)
                .map(|relation| {
                    if relation.issue_id_1 == issue_id {
                        relation.issue_id_2.clone()
                    } else {
                        relation.issue_id_1.clone()
                    }
                })
                .collect::<Vec<_>>();
            linked.sort();
            linked.dedup();
            ids.extend(linked);
        }
        ids
    }

    // ----- Record queries ------------------------------------------------

    fn record_exists(&self, kind: &str, id: &str) -> bool {
        match kind {
            "issue" => self.issues.contains_key(id),
            "evidence" => self.evidence.contains_key(id),
            "review" => self.reviews.contains_key(id),
            _ => false,
        }
    }

    pub fn evidence(&self, id: &str) -> Option<&EvidenceRecord> {
        self.evidence.get(id)
    }

    pub fn require_evidence(&self, id: &str) -> Result<&EvidenceRecord> {
        self.evidence
            .get(id)
            .ok_or_else(|| self.not_found(RecordKind::Evidence, id))
    }

    /// Evidence records, most recently updated first.
    pub fn evidence_records(&self) -> Vec<&EvidenceRecord> {
        let mut records = self.evidence.values().collect::<Vec<_>>();
        records.sort_by(|left, right| {
            right
                .header
                .updated_at
                .cmp(&left.header.updated_at)
                .then_with(|| right.header.id.cmp(&left.header.id))
        });
        records
    }

    /// Links touching one record. Fails if the record does not exist.
    pub fn list_record_links(&self, kind: &str, id: &str) -> Result<Vec<RecordLink>> {
        validate_record_kind(kind)?;
        validate_record_id(id)?;
        if !self.record_exists(kind, id) {
            let kind = match kind {
                "evidence" => RecordKind::Evidence,
                "review" => RecordKind::Review,
                _ => RecordKind::Issue,
            };
            return Err(self.not_found(kind, id));
        }
        Ok(self
            .links_by_record
            .get(&(kind.to_string(), id.to_string()))
            .into_iter()
            .flatten()
            .map(|position| self.links[*position].clone())
            .collect())
    }

    /// Every evidence attachment and issue-to-issue relation, sorted.
    pub fn record_links(&self) -> &[RecordLink] {
        &self.links
    }
}

fn link_key(link: &RecordLink) -> (&str, &str, &str, &str, &str) {
    (
        &link.source_kind,
        &link.source_id,
        &link.target_kind,
        &link.target_id,
        &link.relation_type,
    )
}

fn validate_acyclic(edges: &BTreeMap<String, Vec<String>>, relation: &str) -> Result<()> {
    fn visit<'a>(
        node: &'a str,
        edges: &'a BTreeMap<String, Vec<String>>,
        visiting: &mut BTreeSet<&'a str>,
        visited: &mut BTreeSet<&'a str>,
    ) -> bool {
        if visited.contains(node) {
            return false;
        }
        if !visiting.insert(node) {
            return true;
        }
        for next in edges.get(node).into_iter().flatten() {
            if visit(next, edges, visiting, visited) {
                return true;
            }
        }
        visiting.remove(node);
        visited.insert(node);
        false
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for node in edges.keys() {
        if visit(node, edges, &mut visiting, &mut visited) {
            bail!("relationships.{relation} contains a cycle");
        }
    }
    Ok(())
}

fn validate_issue_link_type(
    issue_id: &str,
    relation_type: &str,
    custom: &BTreeSet<String>,
) -> Result<()> {
    if matches!(relation_type, "validates" | "evidenced_by") {
        bail!(
            "workflow_issue_link_type_invalid: issue {issue_id} has proof relationship '{relation_type}' to another issue; proof roles are reserved for evidence attachments. Use 'advances' for mission scope or attach evidence with role 'validates'."
        );
    }
    if WELL_KNOWN_LINK_TYPES.contains(&relation_type)
        || WELL_KNOWN_RELATION_TYPES.contains(&relation_type)
        || custom.contains(relation_type)
    {
        return Ok(());
    }
    let configured = if custom.is_empty() {
        "(none)".to_string()
    } else {
        custom.iter().cloned().collect::<Vec<_>>().join(", ")
    };
    bail!(
        "workflow_issue_link_type_invalid: issue {issue_id} has unconfigured context link type '{relation_type}'; built-in workflow link types are {}; configured custom context-only link types are {configured}; configure custom issue links in .atelier/config.toml [issue_links].custom_context_types",
        built_in_issue_link_types().join(", "),
    )
}

pub fn built_in_issue_link_types() -> Vec<&'static str> {
    let mut types = WELL_KNOWN_LINK_TYPES.to_vec();
    types.extend(WELL_KNOWN_RELATION_TYPES.iter().copied());
    types.sort();
    types.dedup();
    types
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_records::{issue_record, link, Records};
    use tempfile::tempdir;

    #[test]
    fn sequential_reads_preserve_supplied_bytes_and_raw_error_priority() {
        let dir = tempdir().unwrap();
        let bytes = (0..130)
            .map(|number| {
                let id = format!("atelier-{number:04}");
                (
                    PathBuf::from(format!("issues/{id}.md")),
                    atelier_records::render_issue_record(&issue_record(&id, "task", "todo"))
                        .unwrap()
                        .into_bytes(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let paths = bytes.keys().cloned().collect::<Vec<_>>();
        let (files, changed) = read_record_paths(dir.path(), paths.clone(), &|path| {
            Ok((true, Some(bytes[path].clone())))
        })
        .unwrap();
        assert!(changed.is_empty() && files.errors.is_empty());
        assert_eq!(files.issues.len(), paths.len());
        assert_eq!(files.issues[0].issue.id, "atelier-0000");
        assert_eq!(files.issues[129].issue.id, "atelier-0129");
        assert_eq!(files.into_index().unwrap().issues().count(), paths.len());
        let error = read_record_paths(dir.path(), paths.clone(), &|path| {
            if path == paths[64] {
                bail!("first lexical raw fault");
            }
            if path == paths[129] {
                bail!("later lexical raw fault");
            }
            if path == paths[0] {
                return Ok((true, Some(vec![0xff])));
            }
            Ok((false, None))
        })
        .unwrap_err();
        assert_eq!(error.to_string(), "first lexical raw fault");
    }

    #[test]
    fn parsing_consumes_supplied_bytes_without_a_second_read() {
        let dir = tempdir().unwrap();
        let relative = PathBuf::from("issues/atelier-byte.md");
        fs::create_dir(dir.path().join("issues")).unwrap();
        let bytes =
            atelier_records::render_issue_record(&issue_record("atelier-byte", "task", "todo"))
                .unwrap()
                .into_bytes();
        fs::write(dir.path().join(&relative), &bytes).unwrap();
        let (files, changed) = read_record_paths(dir.path(), vec![relative.clone()], &|path| {
            let bytes = fs::read(dir.path().join(path))?;
            fs::write(dir.path().join(path), "invalid after the supplied read")?;
            Ok((true, Some(bytes)))
        })
        .unwrap();
        assert!(changed.is_empty() && files.errors.is_empty());
        assert_eq!(files.issues[0].issue.id, "atelier-byte");
        assert_eq!(read_record_files(dir.path()).unwrap().errors.len(), 1);
        let error = read_record_paths(
            dir.path(),
            vec![relative.clone(), PathBuf::from("later")],
            &|path| {
                if path == relative {
                    Ok((false, None))
                } else {
                    bail!("raw read failure")
                }
            },
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "raw read failure");
    }
    #[cfg(unix)]
    #[test]
    fn default_reader_still_skips_config_before_reading_its_bytes() {
        let dir = tempdir().unwrap();
        std::os::unix::fs::symlink("missing", dir.path().join("workflow.yaml")).unwrap();
        std::os::unix::fs::symlink("missing", dir.path().join("manifest.json")).unwrap();
        fs::write(dir.path().join("config.toml"), "local data").unwrap();
        let files = read_record_files(dir.path()).unwrap();
        assert!(files.errors.is_empty() && files.issues.is_empty());
        assert!(files.into_index().unwrap().issues().next().is_none());
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_record_files_are_read_or_reported() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        fs::create_dir_all(state_dir.join("issues")).unwrap();
        let target = dir.path().join("elsewhere.md");
        fs::write(
            &target,
            atelier_records::render_issue_record(&issue_record("atelier-link", "task", "todo"))
                .unwrap(),
        )
        .unwrap();
        std::os::unix::fs::symlink(&target, state_dir.join("issues/atelier-link.md")).unwrap();
        std::os::unix::fs::symlink(
            dir.path().join("missing.md"),
            state_dir.join("issues/atelier-gone.md"),
        )
        .unwrap();
        std::os::unix::fs::symlink(dir.path(), state_dir.join("issues/loop")).unwrap();

        let files = read_record_files(&state_dir).unwrap();

        assert_eq!(files.issues.len(), 1);
        assert_eq!(files.issues[0].issue.id, "atelier-link");
        let errors = files
            .errors
            .iter()
            .map(|error| error.path.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        assert_eq!(errors, ["issues/atelier-gone.md", "issues/loop"]);
    }

    fn impact_ids(index: &RecordIndex, source: &str) -> Vec<String> {
        let mut ids = index
            .downstream_impact(source)
            .into_iter()
            .map(|issue| issue.id)
            .collect::<Vec<_>>();
        ids.sort();
        ids
    }

    #[test]
    fn downstream_impact_follows_children_then_relations_transitively() {
        let mut records = Records::default();
        records.issue("atelier-root", "feature", "todo");
        records.issue("atelier-why1", "task", "todo");
        records.issue("atelier-why2", "task", "todo");
        records.issue("atelier-derived", "task", "todo");
        records.parent("atelier-why1", "atelier-root");
        records.parent("atelier-why2", "atelier-root");
        records.relate("atelier-why1", "atelier-derived", "derived");

        assert_eq!(
            impact_ids(&records.index(), "atelier-root"),
            ["atelier-derived", "atelier-why1", "atelier-why2"]
        );
    }

    #[test]
    fn downstream_impact_follows_impact_relations_transitively() {
        let mut records = Records::default();
        for id in [
            "atelier-a",
            "atelier-b",
            "atelier-c",
            "atelier-d",
            "atelier-e",
        ] {
            records.issue(id, "task", "todo");
        }
        records.relate("atelier-a", "atelier-b", "derived");
        records.relate("atelier-b", "atelier-c", "caused-by");
        records.relate("atelier-c", "atelier-d", "falsifies");
        records.relate("atelier-e", "atelier-a", "related");

        assert_eq!(
            impact_ids(&records.index(), "atelier-a"),
            ["atelier-b", "atelier-c", "atelier-d"]
        );
    }

    #[test]
    fn downstream_impact_follows_assumptions_one_hop() {
        let mut records = Records::default();
        for id in ["atelier-a1", "atelier-a2", "atelier-a3"] {
            records.issue(id, "task", "todo");
        }
        records.relate("atelier-a1", "atelier-a2", "assumption");
        records.relate("atelier-a2", "atelier-a3", "assumption");

        assert_eq!(impact_ids(&records.index(), "atelier-a1"), ["atelier-a2"]);
    }

    #[test]
    fn load_names_the_invalid_record_file() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        fs::create_dir_all(state_dir.join("issues")).unwrap();
        fs::write(state_dir.join("issues/atelier-bad1.md"), "not a record").unwrap();

        let error = RecordIndex::load(&state_dir).unwrap_err();

        assert!(
            format!("{error:#}").contains("Invalid record file .atelier/issues/atelier-bad1.md"),
            "unexpected error: {error:#}"
        );
    }

    #[test]
    fn read_record_files_reports_every_invalid_file() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        fs::create_dir_all(state_dir.join("issues")).unwrap();
        fs::create_dir_all(state_dir.join("evidence")).unwrap();
        fs::write(state_dir.join("issues/atelier-bad1.md"), "broken").unwrap();
        fs::write(state_dir.join("evidence/atelier-bad2.md"), "broken").unwrap();
        fs::write(state_dir.join("stray.txt"), "stray").unwrap();
        fs::create_dir_all(state_dir.join("runtime")).unwrap();
        fs::write(state_dir.join("runtime/.canonical-mutation.lock"), "").unwrap();

        let files = read_record_files(&state_dir).unwrap();

        let paths = files
            .errors
            .iter()
            .map(|error| error.path.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            paths,
            [
                "evidence/atelier-bad2.md",
                "issues/atelier-bad1.md",
                "stray.txt"
            ]
        );
    }

    #[test]
    fn graph_derives_parents_blockers_and_relations() {
        let mut epic = issue_record("atelier-epic", "feature", "todo");
        epic.relationships.children.push(link("atelier-child"));
        let mut blocker = issue_record("atelier-blocker", "task", "todo");
        blocker.relationships.blocks.push(link("atelier-child"));
        let mut mission = issue_record("atelier-mission", "mission", "todo");
        mission
            .relationships
            .relates
            .push(atelier_records::issue_relates_relationship(
                "atelier-epic",
                "advances",
            ));
        let child = issue_record("atelier-child", "task", "todo");

        let index =
            RecordIndex::from_records(vec![epic, blocker, mission, child], Vec::new(), Vec::new())
                .unwrap();

        assert_eq!(
            index
                .get_issue("atelier-child")
                .unwrap()
                .parent_id
                .as_deref(),
            Some("atelier-epic")
        );
        assert_eq!(index.get_blockers("atelier-child"), ["atelier-blocker"]);
        assert_eq!(index.get_blocking("atelier-blocker"), ["atelier-child"]);
        assert_eq!(index.get_typed_relations("atelier-epic").len(), 1);
        assert_eq!(
            index.resolve_issue_ref("child").unwrap().as_deref(),
            Some("atelier-child")
        );
    }

    #[test]
    fn duplicate_id_is_reported_once_without_missing_reference_cascade() {
        let dir = tempdir().unwrap();
        let state = dir.path().join(".atelier");
        let owner = issue_record("atelier-dup", "task", "todo");
        // Evidence sharing the issue's ID is dropped; the reviewer's evidence
        // still names it.
        let shadow =
            crate::test_records::evidence_record("atelier-dup", "atelier-dup", "validates");
        let mut referrer =
            crate::test_records::evidence_record("atelier-ref", "atelier-dup", "validates");
        referrer
            .header
            .relationships
            .attachments
            .push(atelier_records::attachment_relationship(
                "evidence",
                "atelier-dup",
                "related",
            ));
        let errors = RecordIndex::validation_errors(
            &state,
            vec![owner.clone()],
            vec![shadow.clone(), referrer.clone()],
            Vec::new(),
        )
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
        assert_eq!(errors, ["Duplicate record ID in record files: atelier-dup"]);
        let first =
            RecordIndex::from_records(vec![owner], vec![shadow, referrer], Vec::new()).unwrap_err();
        assert_eq!(
            first.to_string(),
            "Duplicate record ID in record files: atelier-dup"
        );

        // A reference of another kind to the same ID is still missing.
        let mut referrer =
            crate::test_records::evidence_record("atelier-ref", "atelier-real", "validates");
        referrer
            .header
            .relationships
            .attachments
            .push(atelier_records::attachment_relationship(
                "issue",
                "atelier-twin",
                "related",
            ));
        let twin =
            crate::test_records::evidence_record("atelier-twin", "atelier-real", "validates");
        let errors = RecordIndex::validation_errors(
            &state,
            vec![issue_record("atelier-real", "task", "todo")],
            vec![twin.clone(), twin, referrer],
            Vec::new(),
        )
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
        assert_eq!(
            errors,
            [
                "Duplicate record ID in record files: atelier-twin",
                "atelier-ref has related reference to missing issue atelier-twin",
            ]
        );
    }

    #[test]
    fn graph_rejects_missing_references_and_cycles() {
        let mut dangling = issue_record("atelier-a", "task", "todo");
        dangling.relationships.blocks.push(link("atelier-missing"));
        let error = RecordIndex::from_records(vec![dangling], Vec::new(), Vec::new()).unwrap_err();
        assert!(error.to_string().contains("missing issue atelier-missing"));

        let mut first = issue_record("atelier-a", "task", "todo");
        first.relationships.blocks.push(link("atelier-b"));
        let mut second = issue_record("atelier-b", "task", "todo");
        second.relationships.blocks.push(link("atelier-a"));
        let error =
            RecordIndex::from_records(vec![first, second], Vec::new(), Vec::new()).unwrap_err();
        assert_eq!(error.to_string(), "relationships.blocks contains a cycle");
    }

    #[test]
    fn any_issue_can_parent_one_level_deep() {
        let mut task = issue_record("atelier-task", "task", "todo");
        task.relationships.children.push(link("atelier-child"));
        let child = issue_record("atelier-child", "task", "todo");
        RecordIndex::from_records(vec![task.clone(), child.clone()], Vec::new(), Vec::new())
            .unwrap();

        let mut nested = child;
        nested
            .relationships
            .children
            .push(link("atelier-grandchild"));
        let grandchild = issue_record("atelier-grandchild", "bug", "todo");
        let error =
            RecordIndex::from_records(vec![task, nested, grandchild], Vec::new(), Vec::new())
                .unwrap_err();
        assert!(
            error.to_string().contains("parents are one level deep"),
            "{error}"
        );
    }
}
