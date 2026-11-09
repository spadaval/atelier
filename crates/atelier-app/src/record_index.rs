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

use atelier_core::{BaseExemption, EvidenceRecord, Issue, RecordLink, Relation, ReviewRecord};
use atelier_records::review::RetiredDisposition;
use atelier_records::{
    canonical_record_dirs, is_local_atelier_path, parse_evidence_record_file, parse_issue_record,
    validate_record_id, validate_record_kind, CanonicalIssueRecord, Relationships, BLOCKED_BY,
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
    /// Events about a branch no issue owns, on the light path
    /// (atelier-4nsl), each with the branch name as its target.
    pub branch_reviews: Vec<ReviewRecord>,
    /// Trivial exemptions of base commits, which name no issue.
    pub base_exemptions: Vec<BaseExemption>,
    /// Separate disposition events from before ADR 0025, already folded
    /// into their findings in `reviews`.
    pub retired_dispositions: Vec<RetiredDisposition>,
    /// One error per unreadable, unparsable, or unsupported file, in path
    /// order. Each error names the file.
    pub errors: Vec<RecordFileError>,
    /// The state directory the files were read from, whose repository
    /// explains a reference to a missing record.
    pub state_dir: Option<PathBuf>,
}
impl RecordFiles {
    pub fn into_index(self) -> Result<RecordIndex> {
        if let Some(error) = self.errors.into_iter().next() {
            return Err(error.error);
        }
        let repo_root = self
            .state_dir
            .as_deref()
            .and_then(Path::parent)
            .map(Path::to_path_buf);
        let mut index = RecordIndex::from_records(self.issues, self.evidence, self.reviews)
            .map_err(|error| match &repo_root {
                Some(repo_root) => explain_missing_reference(repo_root, error),
                None => error,
            })?
            .with_base_exemptions(self.base_exemptions)?
            .with_retired_dispositions(self.retired_dispositions)?
            .with_branch_reviews(self.branch_reviews)?;
        index.repo_root = repo_root;
        Ok(index)
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

/// [`read_record_files`] as `changes` would leave the files: each change's
/// text read in place of the file's, and a removed file left out.
pub(crate) fn read_changed_record_files(
    state_dir: &Path,
    changes: &[atelier_records::guard::Change],
) -> Result<RecordFiles> {
    let mut paths = Vec::new();
    if state_dir.is_dir() {
        collect_files(state_dir, state_dir, &mut paths)?;
    }
    let changed = changes
        .iter()
        .map(|change| (change.path.as_path(), change.text.as_deref()))
        .collect::<BTreeMap<_, _>>();
    paths.retain(|path| !changed.contains_key(path.as_path()));
    paths.extend(
        changed
            .iter()
            .filter(|(_, text)| text.is_some())
            .map(|(path, _)| path.to_path_buf()),
    );
    paths.sort();
    let read = |relative: &Path| {
        Ok((
            true,
            changed
                .get(relative)
                .copied()
                .flatten()
                .map(|text| text.as_bytes().to_vec()),
        ))
    };
    read_record_paths(state_dir, paths, &read).map(|(files, _)| files)
}

/// Raw errors and mismatches remain separate from parser errors until the
/// caller has completed its index checks. Parsing consumes those same bytes.
pub(crate) fn read_record_paths(
    state_dir: &Path,
    paths: Vec<PathBuf>,
    read: &impl Fn(&Path) -> Result<(bool, Option<Vec<u8>>)>,
) -> Result<(RecordFiles, Vec<PathBuf>)> {
    let mut files = RecordFiles {
        state_dir: Some(state_dir.to_path_buf()),
        ..RecordFiles::default()
    };
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
            Ok(Some(ParsedRecord::BranchReview(record))) => files.branch_reviews.push(*record),
            Ok(Some(ParsedRecord::BaseExemption(record))) => files.base_exemptions.push(*record),
            Ok(Some(ParsedRecord::RetiredDisposition(record))) => {
                files.retired_dispositions.push(*record)
            }
            Ok(None) => {}
            Err(error) => files.errors.push(RecordFileError {
                error: name_record_file(error, &relative),
                path: relative.clone(),
            }),
        }
    }
    // Read separate disposition events as folded into their findings, as
    // `atelier migrate dispositions` would write them (ADR 0025).
    let (_, fold_errors) = atelier_records::review::fold_retired_dispositions(
        &mut files.reviews,
        &files.retired_dispositions,
    );
    for (id, error) in fold_errors {
        let path = PathBuf::from("reviews").join(format!("{id}.md"));
        files.errors.push(RecordFileError {
            error: name_record_file(error, &path),
            path,
        });
    }
    files
        .errors
        .sort_by(|left, right| left.path.cmp(&right.path));
    Ok((files, changed))
}

/// Names the bad record file once: most parser errors already name it as
/// `.atelier/<path>`, so the generic context is added only when no message in
/// the chain does.
fn name_record_file(error: anyhow::Error, relative: &Path) -> anyhow::Error {
    let path = display_state_path(relative);
    if error.chain().any(|cause| cause.to_string().contains(&path)) {
        error
    } else {
        error.context(format!("Invalid record file {path}"))
    }
}

enum ParsedRecord {
    Issue(Box<CanonicalIssueRecord>),
    Evidence(Box<EvidenceRecord>),
    Review(Box<ReviewRecord>),
    BranchReview(Box<ReviewRecord>),
    BaseExemption(Box<BaseExemption>),
    RetiredDisposition(Box<RetiredDisposition>),
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
        match atelier_records::review::parse_review_file(&text, relative)? {
            atelier_records::review::ReviewFile::Event(record) => {
                ParsedRecord::Review(Box::new(record))
            }
            atelier_records::review::ReviewFile::BranchEvent(record) => {
                ParsedRecord::BranchReview(Box::new(record))
            }
            atelier_records::review::ReviewFile::BaseExemption(record) => {
                ParsedRecord::BaseExemption(Box::new(record))
            }
            atelier_records::review::ReviewFile::RetiredDisposition(record) => {
                ParsedRecord::RetiredDisposition(Box::new(record))
            }
        }
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

/// The `.atelier/<path>` of a stored record of `kind` (`issue`, `evidence`
/// or `review`), which load-time errors about the record name.
pub(crate) fn record_file(kind: &str, id: &str) -> PathBuf {
    let dir = match kind {
        "evidence" => "evidence",
        "review" => "reviews",
        _ => "issues",
    };
    PathBuf::from(display_state_path(&Path::new(dir).join(format!("{id}.md"))))
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
    /// Events about a branch no issue owns, which never count for an issue
    /// (atelier-4nsl).
    branch_reviews: BTreeMap<String, ReviewRecord>,
    base_exemptions: BTreeMap<String, BaseExemption>,
    retired_dispositions: BTreeMap<String, RetiredDisposition>,
    /// Evidence attachments and issue-to-issue relations, sorted.
    links: Vec<RecordLink>,
    /// `(kind, id)` to indices into `links` touching that record.
    links_by_record: BTreeMap<(String, String), Vec<usize>>,
    /// The repository the records were read from, when known, which says
    /// where a record missing from them is instead.
    repo_root: Option<PathBuf>,
}

/// Explains an error about a reference to a missing record, as
/// [`missing_reference_note`] does; any other error is returned as it is.
fn explain_missing_reference(repo_root: &Path, error: anyhow::Error) -> anyhow::Error {
    match missing_reference_note(repo_root, &error.to_string()) {
        Some(note) => anyhow!("{error}; {note}"),
        None => error,
    }
}

/// Why a record that `message` names as `... reference to missing <kind>
/// <id>` is missing, and how to recover, when Git shows it: it is committed
/// at HEAD but gone from the checkout; prune or someone deleted it while a
/// record on another branch still linked it (atelier-r7cd); or it exists
/// only on another branch, which is named (atelier-ilba). `None` for any
/// other message, or when Git shows none of these.
pub(crate) fn missing_reference_note(repo_root: &Path, message: &str) -> Option<String> {
    let (_, rest) = message.rsplit_once(" reference to missing ")?;
    let (kind, id) = rest.trim().split_once(' ')?;
    let path = stored_record_path(kind, id)?;
    let git = |args: &[&str]| {
        crate::git_freshness::git_output(repo_root, args)
            .ok()
            .and_then(|out| String::from_utf8(out).ok())
            .map(|out| out.trim().to_string())
    };
    let unlink = "or remove the link: delete it from the linking issue's relationships, or remove a review or evidence record whose only target is gone. Commands cannot load the records until one of these is done";
    if git(&["cat-file", "-t", &format!("HEAD:{path}")]).is_some() {
        return Some(format!(
            "{id} is committed at HEAD but missing from the checkout. Restore it with `git restore -- ':/{path}'`, {unlink}"
        ));
    }
    if let Some(commit) = git(&["log", "-1", "--diff-filter=D", "--format=%h", "--", &path])
        .filter(|commit| !commit.is_empty())
    {
        return Some(format!(
            "{id} was deleted in commit {commit} (for example by atelier prune). Restore it with `git restore --source={commit}^ -- ':/{path}'`, {unlink}"
        ));
    }
    let branches = branches_holding(repo_root, &path);
    (!branches.is_empty()).then(|| {
        format!(
            "{id} exists only on {}, not on this branch. Merge that branch in first, or make the link there",
            branch_list(&branches)
        )
    })
}

/// `.atelier/<dir>/<id>.md` for a record `kind` as a relationship stores it.
fn stored_record_path(kind: &str, id: &str) -> Option<String> {
    let dir = match kind {
        "issue" => "issues",
        "evidence" => "evidence",
        "review" => "reviews",
        _ => return None,
    };
    validate_record_id(id).ok()?;
    Some(format!(".atelier/{dir}/{id}.md"))
}

/// The branches other than the current one whose tip holds `path`: local
/// branches, or remote-tracking ones when no local branch does. Empty when
/// Git cannot say, as outside a repository.
fn branches_holding(repo_root: &Path, path: &str) -> Vec<String> {
    let Ok(refs) = crate::git_freshness::git_output(
        repo_root,
        &[
            "for-each-ref",
            "--format=%(refname)",
            "refs/heads",
            "refs/remotes",
        ],
    )
    .map(|out| String::from_utf8_lossy(&out).into_owned()) else {
        return Vec::new();
    };
    let current = crate::git_freshness::git_output(repo_root, &["symbolic-ref", "-q", "HEAD"])
        .map(|out| String::from_utf8_lossy(&out).trim().to_owned())
        .unwrap_or_default();
    let refs = refs
        .lines()
        .filter(|name| *name != current && !name.ends_with("/HEAD"))
        .collect::<Vec<_>>();
    if refs.is_empty() {
        return Vec::new();
    }
    let input = refs
        .iter()
        .map(|name| format!("{name}:{path}\n"))
        .collect::<String>();
    let Ok(found) = crate::git_freshness::git_input(
        repo_root,
        &["cat-file", "--batch-check=%(objecttype)"],
        input.as_bytes(),
    ) else {
        return Vec::new();
    };
    let holding = refs
        .iter()
        .zip(String::from_utf8_lossy(&found).lines())
        .filter(|(_, answer)| *answer == "blob")
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let local = holding
        .iter()
        .filter_map(|name| name.strip_prefix("refs/heads/"))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if !local.is_empty() {
        return local;
    }
    holding
        .iter()
        .filter_map(|name| name.strip_prefix("refs/remotes/"))
        .map(str::to_owned)
        .collect()
}

/// `branch a`, `branches a and b`, or the first three and a count.
fn branch_list(branches: &[String]) -> String {
    match branches {
        [one] => format!("branch {one}"),
        [first, second] => format!("branches {first} and {second}"),
        [first, second, third] => format!("branches {first}, {second} and {third}"),
        _ => format!(
            "branches {}, {}, {} and {} more",
            branches[0],
            branches[1],
            branches[2],
            branches.len() - 3
        ),
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
        Self::from_project_records(state_dir, files.issues, files.evidence, files.reviews)?
            .with_base_exemptions(files.base_exemptions)?
            .with_retired_dispositions(files.retired_dispositions)?
            .with_branch_reviews(files.branch_reviews)
    }

    /// Adds the separate disposition events read beside the other records,
    /// already folded into their findings, so their IDs still resolve as
    /// the reviews they were (ADR 0025).
    pub fn with_retired_dispositions(mut self, events: Vec<RetiredDisposition>) -> Result<Self> {
        for record in events {
            let id = record.id.clone();
            if self.record_exists("review", &id)
                || self.issues.contains_key(&id)
                || self.evidence.contains_key(&id)
                || self.base_exemptions.contains_key(&id)
            {
                bail!("Duplicate record ID in record files: {id}");
            }
            self.retired_dispositions.insert(id, record);
        }
        Ok(self)
    }

    /// Separate disposition events from before ADR 0025, which the
    /// findings they name already carry.
    pub fn retired_dispositions(&self) -> impl Iterator<Item = &RetiredDisposition> {
        self.retired_dispositions.values()
    }

    /// The issue a review file is about: an event's target, or a separate
    /// disposition event's.
    pub fn review_target(&self, id: &str) -> Option<&str> {
        self.reviews
            .get(id)
            .map(|record| record.target.as_str())
            .or_else(|| {
                self.retired_dispositions
                    .get(id)
                    .map(|record| record.target.as_str())
            })
    }

    /// Adds the events about branches read beside the other records
    /// (atelier-4nsl). They name no issue as their target, so only their IDs
    /// and a deferred finding's follow-up are checked.
    pub fn with_branch_reviews(mut self, reviews: Vec<ReviewRecord>) -> Result<Self> {
        if let Some(error) = self.branch_review_errors(&reviews).into_iter().next() {
            return Err(error);
        }
        self.branch_reviews = reviews
            .into_iter()
            .map(|record| (record.header.id.clone(), record))
            .collect();
        Ok(self)
    }

    /// What is wrong with `reviews`, events about branches, beside this
    /// index's records: an ID another record has, or a follow-up issue that
    /// does not exist.
    pub fn branch_review_errors(&self, reviews: &[ReviewRecord]) -> Vec<anyhow::Error> {
        let mut errors = Vec::new();
        let mut seen = BTreeSet::new();
        for record in reviews {
            let id = &record.header.id;
            if !seen.insert(id.as_str())
                || self.issues.contains_key(id)
                || self.evidence.contains_key(id)
                || self.record_exists("review", id)
                || self.base_exemptions.contains_key(id)
            {
                errors.push(anyhow!("Duplicate record ID in record files: {id}"));
            }
            if let Some(issue) = record
                .disposition()
                .and_then(|disposition| disposition.follow_up.as_ref())
            {
                if let Err(error) =
                    self.ensure_reference(("review", id), "follow-up", "issue", issue)
                {
                    errors.push(error);
                }
            }
        }
        errors
    }

    /// Events about branches no issue owns (atelier-4nsl); `reviews` never
    /// holds them, so nothing that judges an issue reads them.
    pub fn branch_reviews(&self) -> impl Iterator<Item = &ReviewRecord> {
        self.branch_reviews.values()
    }

    /// The event about a branch with this ID.
    pub fn branch_review(&self, id: &str) -> Option<&ReviewRecord> {
        self.branch_reviews.get(id)
    }

    /// Adds the base exemptions read beside the other records. They name no
    /// other record, so only their IDs are checked.
    pub fn with_base_exemptions(mut self, exemptions: Vec<BaseExemption>) -> Result<Self> {
        for record in exemptions {
            let id = record.header.id.clone();
            if self.issues.contains_key(&id)
                || self.evidence.contains_key(&id)
                || self.reviews.contains_key(&id)
                || self.base_exemptions.contains_key(&id)
            {
                bail!("Duplicate record ID in record files: {id}");
            }
            self.base_exemptions.insert(id, record);
        }
        Ok(self)
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
            Some(error) => Err(match state_dir.parent() {
                Some(repo_root) => explain_missing_reference(repo_root, error),
                None => error,
            }),
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
        Self::checked(state_dir, issues, evidence, reviews, true).1
    }

    /// The index and every validation error, as [`Self::validation_errors`]
    /// gives them; without `explain`, the errors do not say where a missing
    /// record is instead, so two sets of records compare by their errors
    /// alone.
    pub(crate) fn checked(
        state_dir: &Path,
        issues: Vec<CanonicalIssueRecord>,
        evidence: Vec<EvidenceRecord>,
        reviews: Vec<ReviewRecord>,
        explain: bool,
    ) -> (Self, Vec<anyhow::Error>) {
        let (index, errors) = Self::validated(state_dir, issues, evidence, reviews);
        let errors = match state_dir.parent() {
            Some(repo_root) if explain => errors
                .into_iter()
                .map(|error| explain_missing_reference(repo_root, error))
                .collect(),
            _ => errors,
        };
        (index, errors)
    }

    fn validated(
        state_dir: &Path,
        issues: Vec<CanonicalIssueRecord>,
        evidence: Vec<EvidenceRecord>,
        reviews: Vec<ReviewRecord>,
    ) -> (Self, Vec<anyhow::Error>) {
        let (mut index, mut errors) = Self::structural(issues, evidence, reviews);
        index.repo_root = state_dir.parent().map(Path::to_path_buf);
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

    /// The links among `issues` alone, for reading a past commit's plan;
    /// structural problems, such as links to records left out, are ignored.
    pub(crate) fn issue_graph(issues: Vec<CanonicalIssueRecord>) -> Self {
        Self::structural(issues, Vec::new(), Vec::new()).0
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
        read_record_files(state_dir)?.into_index()
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
                &record_file("issue", &record.issue.id),
            )?;
        }
        self.validate_mission_dependencies(policy)
    }
    pub fn reviews(&self) -> impl Iterator<Item = &ReviewRecord> {
        self.reviews.values()
    }
    /// Trivial exemptions of base commits, by ID.
    pub fn base_exemptions(&self) -> impl Iterator<Item = &BaseExemption> {
        self.base_exemptions.values()
    }
    pub fn base_exemption(&self, id: &str) -> Option<&BaseExemption> {
        self.base_exemptions.get(id)
    }
    pub fn require_review(&self, id: &str) -> Result<&ReviewRecord> {
        self.reviews
            .get(id)
            .ok_or_else(|| self.not_found(RecordKind::Review, id))
    }
    fn validate_reviews(&self, errors: &mut Vec<anyhow::Error>) {
        for record in self.reviews.values() {
            if let Err(error) = self.validate_review(record, errors) {
                errors.push(error);
            }
        }
    }

    fn validate_review(
        &self,
        record: &ReviewRecord,
        errors: &mut Vec<anyhow::Error>,
    ) -> Result<()> {
        // Every missing reference is reported here before any other check,
        // and checks that need a missing record are skipped.
        let mut exists = |relation: &str, kind: &str, id: &str| {
            self.ensure_reference(("review", &record.header.id), relation, kind, id)
                .map_err(|error| errors.push(error))
                .is_ok()
        };
        exists("review target", "issue", &record.target);
        if let Some(issue) = record
            .disposition()
            .and_then(|disposition| disposition.follow_up.as_ref())
        {
            exists("follow-up", "issue", issue);
        }
        Ok(())
    }

    /// The work item a `discovered_from` link names: the item itself, the
    /// item a review event is about, or the item a receipt validates.
    pub fn discovery_source(&self, kind: &str, id: &str) -> Option<String> {
        match kind {
            "issue" => self.issues.contains_key(id).then(|| id.to_owned()),
            "review" => self.review_target(id).map(str::to_owned),
            "evidence" => self
                .evidence
                .get(id)?
                .data
                .target
                .as_ref()
                .filter(|target| target.kind == "issue")
                .map(|target| target.id.clone()),
            _ => None,
        }
    }

    fn index_issue_graph(&mut self, errors: &mut Vec<anyhow::Error>) {
        let mut parents = BTreeMap::<String, String>::new();
        let mut relation_keys = BTreeSet::new();
        // `(blocker, blocked)` edges the blocked issue declares. The blocker
        // may store the same edge in `blocks`, as records written before
        // atelier-ihss do, so these join the graph once each after `blocks`.
        let mut declared_blockers = Vec::new();
        // Records written before atelier-ihss store a typed link on both
        // records; it is one relation, kept from whichever record comes first.
        let mut indexed_relations = BTreeSet::new();
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
                if let Err(error) =
                    self.ensure_reference(("issue", id), "blocks", "issue", &blocked.id)
                {
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
                if let Err(error) =
                    self.ensure_reference(("issue", id), "children", "issue", &child.id)
                {
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
                if let Err(error) = self.ensure_reference(
                    ("issue", id),
                    &relation.relation_type,
                    &relation.kind,
                    &relation.id,
                ) {
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
                if relation.relation_type == BLOCKED_BY {
                    declared_blockers.push((relation.id.clone(), id.clone()));
                    continue;
                }
                let pair = if *id < relation.id {
                    (id.clone(), relation.id.clone())
                } else {
                    (relation.id.clone(), id.clone())
                };
                if !indexed_relations.insert((pair, relation.relation_type.clone())) {
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
        for (blocker, blocked) in declared_blockers {
            let blocking = self.blocking.entry(blocker.clone()).or_default();
            if !blocking.contains(&blocked) {
                blocking.push(blocked.clone());
                self.blockers.entry(blocked).or_default().push(blocker);
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
        // A review's attachments mirror its typed references, which
        // `validate_reviews` checks.
        let checked_by_reviews = source_kind == "review";
        // A missing target is reported once, here, by `index_issue_graph` or
        // by `validate_reviews`.
        let exists =
            |kind: &str, id: &str, relation: &str, checked: bool, errors: &mut Vec<_>| match self
                .ensure_reference((source_kind, source_id), relation, kind, id)
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
                checked_by_reviews,
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
                // A blocker edge is queried through the blocker graph, as
                // `blocks` is, not as a typed link.
                let issue_relation = source_kind == "issue"
                    && relation.kind == "issue"
                    && relation.relation_type != BLOCKED_BY;
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

    /// Fails when `source`'s `relation` names a missing record, naming the
    /// source by its kind and record file.
    fn ensure_reference(
        &self,
        (source_kind, source_id): (&str, &str),
        relation: &str,
        kind: &str,
        id: &str,
    ) -> Result<()> {
        if self.record_exists(kind, id) {
            return Ok(());
        }
        let source = match source_kind {
            "evidence" => "Evidence record",
            "review" => "Review",
            _ => "Issue",
        };
        bail!(
            "{source} {source_id} in {} has {relation} reference to missing {kind} {id}",
            record_file(source_kind, source_id).display()
        )
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
                &record_file("issue", &record.issue.id),
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
        self.resolve_any(&[kind], reference).map(|(_, id)| id)
    }

    /// Resolves a reference that may name a record of any of `kinds`, tried
    /// in order, as [`Self::resolve`] does for one kind. A record of another
    /// kind is reported against the first of `kinds`; an unknown reference
    /// against the first kind with near matches, or else the first.
    pub fn resolve_any(
        &self,
        kinds: &[RecordKind],
        reference: &str,
    ) -> Result<(RecordKind, String)> {
        let expected = kinds[0];
        for &kind in kinds {
            if let Some(id) = record_ref::match_id(kind, reference, self.ids(kind))? {
                return Ok((kind, id));
            }
        }
        for other in RecordKind::ALL
            .into_iter()
            .filter(|other| !kinds.contains(other))
        {
            if let Ok(Some(id)) = record_ref::match_id(other, reference, self.ids(other)) {
                return Err(record_ref::wrong_kind(expected, other, &id));
            }
        }
        let kind = kinds
            .iter()
            .copied()
            .find(|&kind| record_ref::has_near_match(reference, self.ids(kind)))
            .unwrap_or(expected);
        Err(self.not_found(kind, reference))
    }

    /// The shared not-found message, with near matches among this kind's
    /// ids; or, for a full ID that another branch holds, that branch
    /// (atelier-ilba).
    pub fn not_found(&self, kind: RecordKind, reference: &str) -> anyhow::Error {
        let elsewhere = self.repo_root.as_deref().and_then(|repo_root| {
            let id = reference.trim().to_ascii_lowercase();
            let path = stored_record_path(kind.stored(), &id)?;
            let branches = branches_holding(repo_root, &path);
            (!branches.is_empty()).then_some((id, branches))
        });
        match elsewhere {
            Some((id, branches)) => anyhow!(
                "{} {id} not found on this branch; it exists only on {}. Next: merge that branch in first, or run this there",
                kind.noun(),
                branch_list(&branches)
            ),
            None => record_ref::not_found(kind, reference, self.ids(kind)),
        }
    }

    fn ids(&self, kind: RecordKind) -> Box<dyn Iterator<Item = &str> + '_> {
        match kind {
            RecordKind::Issue => Box::new(self.issues.keys().map(String::as_str)),
            RecordKind::Evidence => Box::new(self.evidence.keys().map(String::as_str)),
            RecordKind::Review => Box::new(
                self.reviews
                    .keys()
                    .chain(self.branch_reviews.keys())
                    .chain(self.base_exemptions.keys())
                    .map(String::as_str),
            ),
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

    /// Every issue-to-issue relation touching `issue_id`, once each, whichever
    /// record stores it. `issue_id_1` is the issue whose record declares the
    /// relation (for a relation both records store, the first by id).
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
            "review" => self.reviews.contains_key(id) || self.retired_dispositions.contains_key(id),
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

        assert_eq!(
            format!("{error:#}"),
            "Missing YAML front matter in .atelier/issues/atelier-bad1.md"
        );
    }

    fn load_error_with_issue_markdown(edit: impl Fn(&str) -> String) -> String {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        fs::create_dir_all(state_dir.join("issues")).unwrap();
        let markdown =
            atelier_records::render_issue_record(&issue_record("atelier-bad1", "task", "todo"))
                .unwrap();
        fs::write(state_dir.join("issues/atelier-bad1.md"), edit(&markdown)).unwrap();
        format!("{:#}", RecordIndex::load(&state_dir).unwrap_err())
    }

    #[test]
    fn load_names_a_bad_priority_record_once() {
        let error = load_error_with_issue_markdown(|markdown| {
            markdown
                .lines()
                .map(|line| {
                    if line.starts_with("priority:") {
                        "priority: \"low\""
                    } else {
                        line
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        });

        assert_eq!(
            error,
            "priority 'low' in .atelier/issues/atelier-bad1.md: records store \
             P0 (critical), P1 (high), P2 (medium) or P3 (low)"
        );
    }

    #[test]
    fn load_names_a_record_with_invalid_yaml_once() {
        let error =
            load_error_with_issue_markdown(|markdown| markdown.replacen("---\n", "---\n: [\n", 1));

        assert!(
            error.starts_with("Invalid YAML front matter in .atelier/issues/atelier-bad1.md: "),
            "unexpected error: {error}"
        );
        assert_eq!(error.matches("atelier-bad1.md").count(), 1, "{error}");
    }

    #[test]
    fn record_file_errors_add_the_path_only_when_missing() {
        let relative = Path::new("issues/atelier-bad1.md");

        let bare = name_record_file(anyhow::anyhow!("boom"), relative);
        assert_eq!(
            format!("{bare:#}"),
            "Invalid record file .atelier/issues/atelier-bad1.md: boom"
        );

        let named = name_record_file(
            anyhow::anyhow!("detail").context("invalid review .atelier/reviews/atelier-r1.md"),
            Path::new("reviews/atelier-r1.md"),
        );
        assert_eq!(
            format!("{named:#}"),
            "invalid review .atelier/reviews/atelier-r1.md: detail"
        );

        let short = name_record_file(
            anyhow::anyhow!("detail").context("invalid review reviews/atelier-r1.md"),
            Path::new("reviews/atelier-r1.md"),
        );
        assert_eq!(
            format!("{short:#}"),
            "Invalid record file .atelier/reviews/atelier-r1.md: \
             invalid review reviews/atelier-r1.md: detail"
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

    /// A deferred finding: its follow-up is a reference like its target.
    fn deferred_finding(id: &str, target: &str, follow_up: &str) -> ReviewRecord {
        let commit = "a".repeat(40);
        let text = format!(
            "---\nschema: atelier.review\nschema_version: 2\nid: {id}\ntitle: found\ntarget: {target}\nidentity: reviewer\ncommit: {commit}\ncreated_at: 2026-10-17T00:00:00Z\nevent:\n  event: finding\n  recommendation: defer\n  disposition:\n    decision: defer\n    identity: reviewer\n    commit: {commit}\n    recorded_at: 2026-10-18T00:00:00Z\n    follow_up: {follow_up}\n    reason: later\n---\n\nfound\n"
        );
        atelier_records::review::parse_review_record(&text, Path::new(&format!("reviews/{id}.md")))
            .unwrap()
    }

    fn review_validation_errors(reviews: Vec<ReviewRecord>) -> Vec<String> {
        let dir = tempdir().unwrap();
        RecordIndex::validation_errors(
            &dir.path().join(".atelier"),
            vec![issue_record("atelier-real", "task", "todo")],
            Vec::new(),
            reviews,
        )
        .iter()
        .map(ToString::to_string)
        .collect()
    }

    #[test]
    fn a_deferred_finding_reports_each_missing_reference_once() {
        let errors = review_validation_errors(vec![deferred_finding(
            "atelier-dsp2",
            "atelier-gone",
            "atelier-nofu",
        )]);
        let path = ".atelier/reviews/atelier-dsp2.md";
        assert_eq!(
            errors,
            [
                format!("Review atelier-dsp2 in {path} has review target reference to missing issue atelier-gone"),
                format!("Review atelier-dsp2 in {path} has follow-up reference to missing issue atelier-nofu"),
            ]
        );
        assert!(review_validation_errors(vec![deferred_finding(
            "atelier-dsp3",
            "atelier-real",
            "atelier-real",
        )])
        .is_empty());
    }

    /// A separate disposition event reads as folded into its finding, as
    /// the migration would write it, and still resolves as a review; one
    /// naming a missing finding is reported (ADR 0025).
    #[test]
    fn a_separate_disposition_event_reads_as_folded_into_its_finding() {
        let dir = tempdir().unwrap();
        let state = dir.path().join(".atelier");
        fs::create_dir_all(state.join("reviews")).unwrap();
        fs::create_dir_all(state.join("issues")).unwrap();
        let store = atelier_records::RecordStore::new(state.clone());
        store
            .write_issue_atomic(&issue_record("atelier-real", "task", "todo"))
            .unwrap();
        let commit = "a".repeat(40);
        let review = |id: &str, event: &str| {
            fs::write(
                state.join(format!("reviews/{id}.md")),
                format!("---\nschema: atelier.review\nschema_version: 1\nid: {id}\ntitle: t\ntarget: atelier-real\nidentity: reviewer\ncommit: {commit}\ncreated_at: 2026-10-17T00:00:00Z\nevent:\n{event}---\n\n{id} text\n"),
            )
            .unwrap();
        };
        review(
            "atelier-fnd1",
            "  event: finding\n  recommendation: no_action\n",
        );
        review("atelier-dsp1", "  event: disposition\n  finding: atelier-fnd1\n  decision: no_action\n  supersedes: null\n  resolution_commit: null\n  follow_up: null\n");
        let index = RecordIndex::load_structural(&state).unwrap();
        let disposition = index
            .require_review("atelier-fnd1")
            .unwrap()
            .disposition()
            .cloned()
            .unwrap();
        assert_eq!(
            (
                disposition.reason.as_str(),
                disposition.migrated_from.as_deref()
            ),
            ("atelier-dsp1 text", Some("atelier-dsp1"))
        );
        assert_eq!(index.review_target("atelier-dsp1"), Some("atelier-real"));
        review("atelier-dsp2", "  event: disposition\n  finding: atelier-gone\n  decision: no_action\n  supersedes: null\n  resolution_commit: null\n  follow_up: null\n");
        let error = format!("{:#}", RecordIndex::load_structural(&state).unwrap_err());
        assert!(
            error.contains("atelier-dsp2") && error.contains("name a finding that does not exist"),
            "{error}"
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
                "Evidence record atelier-ref in .atelier/evidence/atelier-ref.md has related reference to missing issue atelier-twin",
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
