use anyhow::{anyhow, bail, Context, Result};
use fs2::FileExt;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use atelier_core::{Issue, Record};
use atelier_records as record_store;
use atelier_records::activity::IssueActivity;
use atelier_records::{
    Relationships, FIRST_CLASS_RECORD_KINDS, WELL_KNOWN_LINK_TYPES, WELL_KNOWN_RELATION_TYPES,
};
use atelier_sqlite::source_freshness;
use atelier_sqlite::{
    Database, EvidenceCacheRow, EvidenceTargetCacheRow, IssueBlockCacheRow, IssueCacheRow,
    IssueRelationCacheRow, RecordSourceCacheRow, ReviewRoomCacheRow,
};

#[derive(Debug)]
struct CanonicalIssue {
    issue: Issue,
    labels: Vec<String>,
    relationships: Relationships,
}

#[derive(Debug)]
struct CacheRebuild {
    issues: Vec<CanonicalIssue>,
    records: Vec<Record>,
    child_edges: Vec<(String, String)>,
    dependency_edges: Vec<(String, String)>,
    relations: Vec<(String, String, String)>,
}

pub fn run(state_dir: &Path, db_path: &Path) -> Result<()> {
    let _lock = CacheRebuildLock::acquire(db_path)?;
    let rebuild = load_cache_rebuild(state_dir)?;
    write_rebuilt_database(state_dir, db_path, &rebuild)?;
    tracing::info!("Rebuilt {} from {}", db_path.display(), state_dir.display());
    Ok(())
}

pub fn refresh_cache(state_dir: &Path, db_path: &Path) -> Result<()> {
    let _lock = CacheRebuildLock::acquire(db_path)?;
    let rebuild = load_cache_rebuild(state_dir)?;
    write_rebuilt_database(state_dir, db_path, &rebuild)?;
    tracing::info!(
        "Rebuilt domain cache in {} from {}",
        db_path.display(),
        state_dir.display()
    );
    Ok(())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IncrementalRepair {
    Repaired,
    NeedsFullRebuild,
}

pub fn repair_incremental(
    db: &Database,
    state_dir: &Path,
    report: &source_freshness::SourceFreshnessReport,
) -> Result<IncrementalRepair> {
    if report.problems.is_empty() {
        return Ok(IncrementalRepair::Repaired);
    }
    if report.problems.len() > 32 {
        return Ok(IncrementalRepair::NeedsFullRebuild);
    }

    let stored_sources = db.record_source_cache_rows()?;
    let stored_by_path = stored_sources
        .iter()
        .map(|entry| (entry.path.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let store = record_store::RecordStore::new(state_dir);
    let mut problems = report.problems.iter().collect::<Vec<_>>();
    problems.sort_by_key(|problem| match problem {
        source_freshness::SourceFreshnessProblem::MissingMetadata { .. } => 0,
        source_freshness::SourceFreshnessProblem::MissingSource { path } => stored_by_path
            .get(path.as_str())
            .map(|source| match source.record_kind.as_str() {
                "review" => 1,
                "evidence" => 2,
                "issue" => 3,
                _ => 4,
            })
            .unwrap_or(4),
        source_freshness::SourceFreshnessProblem::ChangedSource { path }
        | source_freshness::SourceFreshnessProblem::UnindexedSource { path } => {
            match canonical_spec_for_path(path).map(|spec| spec.kind) {
                Some("issue") => 5,
                Some("evidence") => 6,
                Some("review") => 7,
                _ => 8,
            }
        }
    });

    for problem in problems {
        match problem {
            source_freshness::SourceFreshnessProblem::MissingMetadata { .. } => {
                return Ok(IncrementalRepair::NeedsFullRebuild);
            }
            source_freshness::SourceFreshnessProblem::MissingSource { path } => {
                let Some(source) = stored_by_path.get(path.as_str()) else {
                    return Ok(IncrementalRepair::NeedsFullRebuild);
                };
                if remove_missing_source(db, source)? == IncrementalRepair::NeedsFullRebuild {
                    return Ok(IncrementalRepair::NeedsFullRebuild);
                }
            }
            source_freshness::SourceFreshnessProblem::ChangedSource { path }
            | source_freshness::SourceFreshnessProblem::UnindexedSource { path } => {
                let Some(spec) = canonical_spec_for_path(path) else {
                    return Ok(IncrementalRepair::NeedsFullRebuild);
                };
                let relative = Path::new(path);
                let record = store.load_record_at(relative, spec).with_context(|| {
                    format!(
                        "Failed to parse changed record file {}",
                        display_state_path(relative)
                    )
                })?;
                if index_changed_record(db, state_dir, path, record)?
                    == IncrementalRepair::NeedsFullRebuild
                {
                    return Ok(IncrementalRepair::NeedsFullRebuild);
                }
            }
        }
    }
    Ok(IncrementalRepair::Repaired)
}

struct CacheRebuildLock {
    file: File,
}

impl CacheRebuildLock {
    fn acquire(db_path: &Path) -> Result<Self> {
        let path = rebuild_lock_path(db_path)?;
        let parent = path
            .parent()
            .ok_or_else(|| anyhow!("Cannot determine parent directory for {}", path.display()))?;
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&path)
            .with_context(|| {
                format!(
                    "Failed to open domain-cache rebuild lock {}",
                    path.display()
                )
            })?;

        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            match file.try_lock_exclusive() {
                Ok(()) => {
                    file.set_len(0).with_context(|| {
                        format!(
                            "Failed to refresh domain-cache rebuild lock {}",
                            path.display()
                        )
                    })?;
                    writeln!(
                        file,
                        "pid={}\nstarted_at={}",
                        process::id(),
                        chrono::Utc::now().to_rfc3339()
                    )
                    .with_context(|| {
                        format!(
                            "Failed to write domain-cache rebuild lock {}",
                            path.display()
                        )
                    })?;
                    return Ok(Self { file });
                }
                Err(error) if is_lock_contention(&error) && Instant::now() < deadline => {
                    thread::sleep(Duration::from_millis(25));
                }
                Err(error) => {
                    return Err(error).with_context(|| {
                        format!(
                            "Domain-cache rebuild is already running for {}; retry the command after the current rebuild finishes. \
                             If no Atelier command appears to be running, inspect the rebuild lock file {} before retrying.",
                            db_path.display(),
                            path.display()
                        )
                    });
                }
            }
        }
    }
}

impl Drop for CacheRebuildLock {
    fn drop(&mut self) {
        if let Err(error) = self.file.unlock() {
            tracing::warn!("failed to unlock domain-cache rebuild lock: {}", error);
        }
    }
}

fn is_lock_contention(error: &std::io::Error) -> bool {
    matches!(
        error.kind(),
        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::AlreadyExists
    )
}

fn rebuild_lock_path(db_path: &Path) -> Result<PathBuf> {
    let file_name = db_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("Database path has no file name: {}", db_path.display()))?;
    Ok(db_path.with_file_name(format!(".{file_name}.rebuild.lock")))
}

pub fn validate_canonical_state(state_dir: &Path) -> Result<()> {
    load_cache_rebuild(state_dir).map(|_| ())
}

fn load_cache_rebuild(state_dir: &Path) -> Result<CacheRebuild> {
    CacheRebuildLoader::new(state_dir).load()
}

struct CacheRebuildLoader<'a> {
    state_dir: &'a Path,
    store: record_store::RecordStore,
    issues: Vec<CanonicalIssue>,
    records: Vec<Record>,
    issue_ids: BTreeSet<String>,
    global_ids: BTreeSet<String>,
    record_refs: BTreeSet<(String, String)>,
    canonical_paths: BTreeSet<PathBuf>,
    activity_issue_subject_ids: BTreeSet<String>,
    activity_record_refs: BTreeSet<(String, String)>,
}

impl<'a> CacheRebuildLoader<'a> {
    fn new(state_dir: &'a Path) -> Self {
        Self {
            state_dir,
            store: record_store::RecordStore::new(state_dir),
            issues: Vec::new(),
            records: Vec::new(),
            issue_ids: BTreeSet::new(),
            global_ids: BTreeSet::new(),
            record_refs: BTreeSet::new(),
            canonical_paths: BTreeSet::new(),
            activity_issue_subject_ids: BTreeSet::new(),
            activity_record_refs: BTreeSet::new(),
        }
    }

    fn load(mut self) -> Result<CacheRebuild> {
        self.load_issues()?;
        self.load_issue_activities()?;
        self.load_records()?;
        ensure_no_unsupported_canonical_files(self.state_dir, &self.canonical_paths)?;

        let (child_edges, dependency_edges, relations) = self.validate_issue_relationships()?;
        self.collect_record_links()?;
        validate_issue_hierarchy_shapes(&self.issues, &child_edges)?;
        self.validate_issue_fields(&child_edges)?;
        validate_issue_child_cycles(&child_edges)?;
        validate_dependency_cycles(&dependency_edges)?;

        self.issues.sort_by(|a, b| a.issue.id.cmp(&b.issue.id));
        self.records.sort_by(|a, b| {
            (&a.header().kind, &a.header().id).cmp(&(&b.header().kind, &b.header().id))
        });
        Ok(CacheRebuild {
            issues: self.issues,
            records: self.records,
            child_edges,
            dependency_edges,
            relations,
        })
    }

    fn load_issues(&mut self) -> Result<()> {
        for relative in self.store.discover_issue_paths()? {
            self.canonical_paths.insert(relative.clone());
            let record = self.store.load_issue(&relative)?;
            let issue = CanonicalIssue {
                issue: record.issue,
                labels: record.labels,
                relationships: record.relationships,
            };
            self.register_issue_id(&issue.issue.id)?;
            self.issues.push(issue);
        }
        Ok(())
    }

    fn register_issue_id(&mut self, id: &str) -> Result<()> {
        if !self.issue_ids.insert(id.to_string()) {
            bail!("Duplicate issue ID in record files: {}", id);
        }
        if !self.global_ids.insert(id.to_string()) {
            bail!("Duplicate record ID in record files: {}", id);
        }
        Ok(())
    }

    fn load_issue_activities(&mut self) -> Result<()> {
        for relative in discover_activity_paths(self.state_dir)? {
            let activity = IssueActivity::load(self.state_dir, &relative)?;
            self.canonical_paths.insert(relative);
            if activity.subject_kind == "issue" {
                self.activity_issue_subject_ids
                    .insert(activity.subject_id.clone());
            }
            self.activity_record_refs
                .insert((activity.subject_kind, activity.subject_id));
        }
        Ok(())
    }

    fn load_records(&mut self) -> Result<()> {
        for spec in FIRST_CLASS_RECORD_KINDS {
            for relative in discover_record_paths(self.state_dir, spec)? {
                self.canonical_paths.insert(relative.clone());
                let record = self.store.load_record_at(&relative, spec)?;
                self.register_record(&record)?;
                self.records.push(record);
            }
        }
        Ok(())
    }

    fn register_record(&mut self, record: &Record) -> Result<()> {
        let header = record.header();
        if !self.global_ids.insert(header.id.clone()) {
            bail!("Duplicate record ID in record files: {}", header.id);
        }
        if !self
            .record_refs
            .insert((header.kind.clone(), header.id.clone()))
        {
            bail!(
                "Duplicate {} ID in record files: {}",
                header.kind,
                header.id
            );
        }
        Ok(())
    }

    fn validate_issue_relationships(
        &self,
    ) -> Result<(
        Vec<(String, String)>,
        Vec<(String, String)>,
        Vec<(String, String, String)>,
    )> {
        let custom_issue_link_types = self.custom_issue_link_types()?;
        let mut graph = IssueRelationshipIndex::default();
        for subject_id in &self.activity_issue_subject_ids {
            ensure_issue_exists(subject_id, &self.issue_ids, "activity", subject_id)?;
        }
        for (kind, id) in &self.activity_record_refs {
            if kind != "issue" && !self.record_refs.contains(&(kind.clone(), id.clone())) {
                bail!("Activity references missing {kind} record: {id}");
            }
        }
        for issue in &self.issues {
            graph.collect_issue(
                issue,
                &self.issue_ids,
                &self.record_refs,
                &custom_issue_link_types,
            )?;
        }
        Ok((graph.child_edges, graph.dependency_edges, graph.relations))
    }

    fn custom_issue_link_types(&self) -> Result<BTreeSet<String>> {
        let repo_root = self.state_dir.parent().ok_or_else(|| {
            anyhow!(
                "Cannot determine repository root for {}",
                self.state_dir.display()
            )
        })?;
        if !self.state_dir.join("config.toml").exists() {
            return Ok(BTreeSet::new());
        }
        Ok(crate::project_config::ProjectConfig::load(repo_root)?
            .issue_links
            .custom_context_types
            .into_iter()
            .collect())
    }

    fn collect_record_links(&self) -> Result<Vec<(String, String, String, String, String)>> {
        let mut record_links = Vec::new();
        let mut record_link_keys = BTreeSet::new();
        for issue in &self.issues {
            collect_record_relationship_links(
                &mut record_links,
                &mut record_link_keys,
                "issue",
                &issue.issue.id,
                &issue.relationships,
                &self.issue_ids,
                &self.record_refs,
            )?;
        }
        for record in &self.records {
            let header = record.header();
            collect_record_relationship_links(
                &mut record_links,
                &mut record_link_keys,
                &header.kind,
                &header.id,
                &header.relationships,
                &self.issue_ids,
                &self.record_refs,
            )?;
        }
        Ok(record_links)
    }

    fn validate_issue_fields(&self, child_edges: &[(String, String)]) -> Result<()> {
        let parent_by_child = child_edges
            .iter()
            .map(|(child_id, parent_id)| (child_id.as_str(), parent_id.as_str()))
            .collect::<BTreeMap<_, _>>();
        for issue in &self.issues {
            if let Some(parent_id) = parent_by_child.get(issue.issue.id.as_str()) {
                if issue
                    .issue
                    .fields
                    .contains_key(crate::workflow_policy::REVIEW_FIELD)
                {
                    bail!(
                        "workflow_issue_field_invalid: issue {} defines review directly, but child issues inherit review from parent issue {}; move review to the owning epic or remove it from the child",
                        issue.issue.id,
                        parent_id
                    );
                }
            }
        }
        let repo_root = self.state_dir.parent().ok_or_else(|| {
            anyhow!(
                "Cannot determine repository root for {}",
                self.state_dir.display()
            )
        })?;
        let policy_path = repo_root.join(crate::workflow_policy::WORKFLOW_POLICY_PATH);
        if !policy_path.exists()
            && self
                .issues
                .iter()
                .all(|issue| issue.issue.fields.is_empty())
        {
            return Ok(());
        }
        let policy = crate::workflow_policy::load(repo_root)?;
        for issue in &self.issues {
            crate::workflow_policy::validate_issue_against_policy(
                &policy,
                &issue.issue,
                &policy_path,
            )?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct IssueRelationshipIndex {
    relations: Vec<(String, String, String)>,
    relation_keys: BTreeSet<(String, String, String)>,
    child_edges: Vec<(String, String)>,
    child_edge_keys: BTreeSet<(String, String)>,
    dependency_edges: Vec<(String, String)>,
    dependency_edge_keys: BTreeSet<(String, String)>,
}

impl IssueRelationshipIndex {
    fn collect_issue(
        &mut self,
        issue: &CanonicalIssue,
        issue_ids: &BTreeSet<String>,
        record_refs: &BTreeSet<(String, String)>,
        custom_issue_link_types: &BTreeSet<String>,
    ) -> Result<()> {
        self.collect_blocks(issue, issue_ids)?;
        self.collect_children(issue, issue_ids)?;
        self.collect_relations(issue, issue_ids, record_refs, custom_issue_link_types)
    }

    fn collect_blocks(
        &mut self,
        issue: &CanonicalIssue,
        issue_ids: &BTreeSet<String>,
    ) -> Result<()> {
        for blocked in &issue.relationships.blocks {
            if blocked.kind != "issue" {
                bail!(
                    "Issue {} has blocks target {} {}; blocks must target issue records",
                    issue.issue.id,
                    blocked.kind,
                    blocked.id
                );
            }
            ensure_issue_exists(&blocked.id, issue_ids, "blocks", &issue.issue.id)?;
            let key = (blocked.id.clone(), issue.issue.id.clone());
            if !self.dependency_edge_keys.insert(key.clone()) {
                bail!("Duplicate blocks edge {} blocks {}", key.1, key.0);
            }
            self.dependency_edges.push(key);
        }
        Ok(())
    }

    fn collect_children(
        &mut self,
        issue: &CanonicalIssue,
        issue_ids: &BTreeSet<String>,
    ) -> Result<()> {
        for child in &issue.relationships.children {
            if child.kind != "issue" {
                bail!(
                    "Issue {} has children target {} {}; issue hierarchy children must target issue records",
                    issue.issue.id,
                    child.kind,
                    child.id
                );
            }
            ensure_issue_exists(&child.id, issue_ids, "children", &issue.issue.id)?;
            let key = (child.id.clone(), issue.issue.id.clone());
            if !self.child_edge_keys.insert(key.clone()) {
                bail!("Duplicate children edge {} contains {}", key.1, key.0);
            }
            self.child_edges.push(key);
        }
        Ok(())
    }

    fn collect_relations(
        &mut self,
        issue: &CanonicalIssue,
        issue_ids: &BTreeSet<String>,
        record_refs: &BTreeSet<(String, String)>,
        custom_issue_link_types: &BTreeSet<String>,
    ) -> Result<()> {
        for relation in &issue.relationships.relates {
            validate_issue_link_type(
                &issue.issue.id,
                &relation.relation_type,
                custom_issue_link_types,
            )?;
            ensure_record_exists(
                &relation.kind,
                &relation.id,
                issue_ids,
                record_refs,
                &relation.relation_type,
                &issue.issue.id,
            )?;
            if relation.kind != "issue" {
                continue;
            }
            let key = (
                issue.issue.id.clone(),
                relation.id.clone(),
                relation.relation_type.clone(),
            );
            if !self.relation_keys.insert(key.clone()) {
                bail!("Duplicate typed link {} -> {} ({})", key.0, key.1, key.2);
            }
            self.relations.push(key);
        }
        Ok(())
    }
}

fn validate_issue_link_type(
    issue_id: &str,
    relation_type: &str,
    custom_issue_link_types: &BTreeSet<String>,
) -> Result<()> {
    if matches!(relation_type, "validates" | "evidenced_by") {
        bail!(
            "workflow_issue_link_type_invalid: issue {} has proof relationship '{}' to another issue; proof roles are reserved for evidence attachments. Use 'advances' for mission scope or attach evidence with role 'validates'.",
            issue_id,
            relation_type
        );
    }
    if WELL_KNOWN_LINK_TYPES.contains(&relation_type)
        || WELL_KNOWN_RELATION_TYPES.contains(&relation_type)
        || custom_issue_link_types.contains(relation_type)
    {
        return Ok(());
    }
    let configured = if custom_issue_link_types.is_empty() {
        "(none)".to_string()
    } else {
        custom_issue_link_types
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(", ")
    };
    bail!(
        "workflow_issue_link_type_invalid: issue {} has unconfigured context link type '{}'; built-in workflow link types are {}; configured custom context-only link types are {}; configure custom issue links in .atelier/config.toml [issue_links].custom_context_types",
        issue_id,
        relation_type,
        built_in_issue_link_types().join(", "),
        configured
    )
}

fn built_in_issue_link_types() -> Vec<&'static str> {
    let mut types = WELL_KNOWN_LINK_TYPES.to_vec();
    types.extend(WELL_KNOWN_RELATION_TYPES.iter().copied());
    types.sort();
    types.dedup();
    types
}

fn discover_record_paths(
    state_dir: &Path,
    spec: &record_store::RecordKindSpec,
) -> Result<Vec<PathBuf>> {
    let dir_name = spec.canonical_dir.ok_or_else(|| {
        anyhow!(
            "Record kind '{}' does not have a record-file directory",
            spec.kind
        )
    })?;
    let record_dir = state_dir.join(dir_name);
    if !record_dir.exists() {
        return Ok(Vec::new());
    }
    let mut records = Vec::new();
    collect_canonical_record_paths(
        state_dir,
        &record_dir,
        spec.extension,
        spec.kind,
        &mut records,
    )?;
    records.sort();
    Ok(records)
}

fn collect_canonical_record_paths(
    root: &Path,
    dir: &Path,
    extension: &str,
    kind_name: &str,
    records: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(".activity"))
            {
                continue;
            }
            collect_canonical_record_paths(root, &path, extension, kind_name, records)?;
        } else if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .context("Failed to relativize record-file path")?
                .to_path_buf();
            if crate::storage_layout::is_local_atelier_path(&relative) {
                continue;
            }
            if relative.extension().and_then(|ext| ext.to_str()) != Some(extension) {
                bail!(
                    "Unsupported {} record file {}; expected .{} record",
                    kind_name,
                    display_state_path(&relative),
                    extension
                );
            }
            records.push(relative);
        }
    }
    Ok(())
}

fn discover_activity_paths(state_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut records = Vec::new();
    for root_name in ["issues"] {
        let root_dir = state_dir.join(root_name);
        if root_dir.exists() {
            collect_activity_paths(state_dir, &root_dir, &mut records)?;
        }
    }
    records.sort();
    Ok(records)
}

fn collect_activity_paths(root: &Path, dir: &Path, records: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(".activity"))
            {
                collect_activity_files(root, &path, records)?;
            } else {
                collect_activity_paths(root, &path, records)?;
            }
        }
    }
    Ok(())
}

fn ensure_no_unsupported_canonical_files(
    state_dir: &Path,
    expected: &BTreeSet<PathBuf>,
) -> Result<()> {
    if !state_dir.exists() {
        return Ok(());
    }
    for relative in canonical_files_under(state_dir)? {
        if relative == Path::new("manifest.json") || relative == Path::new("graph.json") {
            continue;
        }
        if relative == Path::new("workflow.yaml")
            || relative == Path::new(crate::workflow_policy::WORKFLOW_POLICY_PATH)
        {
            continue;
        }
        let in_canonical_dir = record_store::canonical_record_dirs()
            .iter()
            .any(|dir| relative.starts_with(dir));
        if in_canonical_dir && expected.contains(&relative) {
            continue;
        }
        if in_canonical_dir && relative.extension().and_then(|ext| ext.to_str()) == Some("md") {
            continue;
        }
        if relative == Path::new("mission-control.json") {
            continue;
        }
        bail!("Unsupported record file {}", display_state_path(&relative));
    }
    Ok(())
}

fn canonical_files_under(state_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_canonical_files(state_dir, state_dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_canonical_files(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .context("Failed to relativize record-file path")?;
        if crate::storage_layout::is_local_atelier_path(relative) {
            continue;
        }
        if relative.components().any(|component| {
            component
                .as_os_str()
                .to_string_lossy()
                .ends_with(".activity")
        }) {
            continue;
        }
        if path.is_dir() {
            collect_canonical_files(root, &path, files)?;
        } else if path.is_file() {
            files.push(relative.to_path_buf());
        }
    }
    Ok(())
}

fn collect_activity_files(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .context("Failed to relativize activity path")?;
        if crate::storage_layout::is_local_atelier_path(relative) {
            continue;
        }
        if path.is_dir() {
            collect_activity_files(root, &path, files)?;
        } else if path.is_file() {
            files.push(relative.to_path_buf());
        }
    }
    Ok(())
}

fn write_rebuilt_database(state_dir: &Path, db_path: &Path, rebuild: &CacheRebuild) -> Result<()> {
    let parent = db_path.parent().ok_or_else(|| {
        anyhow!(
            "Cannot determine parent directory for {}",
            db_path.display()
        )
    })?;
    fs::create_dir_all(parent).with_context(|| format!("Failed to create {}", parent.display()))?;

    let tmp_path = unique_rebuild_path(db_path)?;
    if tmp_path.exists() {
        fs::remove_file(&tmp_path)
            .with_context(|| format!("Failed to remove stale {}", tmp_path.display()))?;
    }

    {
        let db = Database::open(&tmp_path)?;
        let parent_by_child = rebuild
            .child_edges
            .iter()
            .map(|(child, parent)| (child.as_str(), parent.as_str()))
            .collect::<BTreeMap<_, _>>();
        for issue in &rebuild.issues {
            index_canonical_issue(
                &db,
                state_dir,
                issue,
                parent_by_child.get(issue.issue.id.as_str()).copied(),
                &rebuild.dependency_edges,
                &rebuild.relations,
            )?;
        }
        for record in &rebuild.records {
            index_domain_record(&db, state_dir, record)?;
        }
    }

    fs::rename(&tmp_path, db_path).with_context(|| {
        format!(
            "Failed to move rebuilt database from {} to {}",
            tmp_path.display(),
            db_path.display()
        )
    })?;
    Ok(())
}

fn index_canonical_issue(
    db: &Database,
    state_dir: &Path,
    issue: &CanonicalIssue,
    parent_id: Option<&str>,
    dependency_edges: &[(String, String)],
    relations: &[(String, String, String)],
) -> Result<()> {
    let id = issue.issue.id.as_str();
    let source_path = record_store::issue_record_path(id)
        .to_string_lossy()
        .replace('\\', "/");
    let row = IssueCacheRow {
        id: issue.issue.id.clone(),
        title: issue.issue.title.clone(),
        status: issue.issue.status.clone(),
        issue_type: issue.issue.issue_type.clone(),
        priority: issue.issue.priority.clone(),
        fields: issue.issue.fields.clone(),
        parent_id: parent_id.map(str::to_string),
        created_at: issue.issue.created_at,
        updated_at: issue.issue.updated_at,
        closed_at: issue.issue.closed_at,
    };
    let blocks = dependency_edges
        .iter()
        .filter(|(_, blocker_id)| blocker_id == id)
        .map(|(blocked_id, blocker_id)| IssueBlockCacheRow {
            blocker_id: blocker_id.clone(),
            blocked_id: blocked_id.clone(),
        })
        .collect::<Vec<_>>();
    let relations = relations
        .iter()
        .filter(|(source_id, _, _)| source_id == id)
        .map(
            |(source_issue_id, target_issue_id, relation_type)| IssueRelationCacheRow {
                source_issue_id: source_issue_id.clone(),
                target_issue_id: target_issue_id.clone(),
                relation_type: relation_type.clone(),
                created_at: issue.issue.updated_at,
            },
        )
        .collect::<Vec<_>>();
    let source = record_source_row(state_dir, &source_path, "issue", id)?;
    db.index_issue(&row, &issue.labels, &blocks, &relations, &source)
}

fn index_domain_record(db: &Database, state_dir: &Path, record: &Record) -> Result<()> {
    let header = record.header();
    let spec = record_store::canonical_record_kind(&header.kind)?;
    let source_path = record_store::canonical_record_path(spec, &header.id)?
        .to_string_lossy()
        .replace('\\', "/");
    match record {
        Record::Evidence(evidence) => {
            let row = EvidenceCacheRow {
                id: header.id.clone(),
                title: header.title.clone(),
                status: header.status.clone(),
                evidence_type: evidence.data.evidence_type.clone(),
                captured_at: evidence.data.captured_at,
                proof_scope: evidence.data.proof_scope.clone(),
                agent_identity: evidence.data.agent_identity.clone(),
                independence_level: evidence.data.independence_level.clone(),
                created_at: header.created_at,
                updated_at: header.updated_at,
            };
            let targets = header
                .relationships
                .attachments
                .iter()
                .map(|target| EvidenceTargetCacheRow {
                    evidence_id: header.id.clone(),
                    target_kind: target.kind.clone(),
                    target_id: target.id.clone(),
                    role: target.role.clone(),
                })
                .collect::<Vec<_>>();
            let source = record_source_row(
                state_dir,
                &source_path,
                header.kind.as_str(),
                header.id.as_str(),
            )?;
            db.index_evidence(&row, &targets, &source)
        }
        Record::Review(review) => {
            let (approvals, unresolved_blocking, unresolved_nonblocking) =
                review_room_counts(&review.events);
            let row = ReviewRoomCacheRow {
                id: header.id.clone(),
                issue_id: review.issue_id.clone(),
                title: header.title.clone(),
                status: header.status.clone(),
                source_branch: review.source_branch.clone(),
                target_branch: review.target_branch.clone(),
                approvals,
                unresolved_blocking,
                unresolved_nonblocking,
                created_at: header.created_at,
                updated_at: header.updated_at,
            };
            let source = record_source_row(
                state_dir,
                &source_path,
                header.kind.as_str(),
                header.id.as_str(),
            )?;
            db.index_review_room(&row, &source)
        }
        Record::Issue(_) => bail!("issue records must use the issue indexer"),
    }
}

fn record_source_row(
    state_dir: &Path,
    source_path: &str,
    record_kind: &str,
    record_id: &str,
) -> Result<RecordSourceCacheRow> {
    let path = state_dir.join(source_path);
    let metadata = fs::metadata(&path)
        .with_context(|| format!("Failed to inspect record-file source {}", path.display()))?;
    let modified_micros = metadata.modified().ok().and_then(|modified| {
        modified
            .duration_since(UNIX_EPOCH)
            .ok()
            .map(|duration| duration.as_micros() as i64)
    });
    Ok(RecordSourceCacheRow {
        path: source_path.to_string(),
        record_kind: record_kind.to_string(),
        record_id: record_id.to_string(),
        size_bytes: metadata.len() as i64,
        modified_micros,
        content_hash: None,
        indexed_at: chrono::Utc::now(),
    })
}

fn review_room_counts(events: &[serde_json::Value]) -> (i64, i64, i64) {
    let latest_change_request = events
        .iter()
        .enumerate()
        .filter_map(|(index, event)| {
            (event.get("kind").and_then(serde_json::Value::as_str) == Some("changes_requested"))
                .then_some(index)
        })
        .max();
    let approvals = events
        .iter()
        .enumerate()
        .filter(|(index, event)| {
            latest_change_request.is_none_or(|latest| *index > latest)
                && event.get("kind").and_then(serde_json::Value::as_str) == Some("approval")
        })
        .count() as i64;
    let resolved = events
        .iter()
        .filter(|event| event.get("kind").and_then(serde_json::Value::as_str) == Some("resolved"))
        .filter_map(|event| event.get("finding").and_then(serde_json::Value::as_str))
        .collect::<BTreeSet<_>>();
    let unresolved = events
        .iter()
        .filter(|event| event.get("kind").and_then(serde_json::Value::as_str) == Some("finding"))
        .filter(|event| {
            event
                .get("id")
                .and_then(serde_json::Value::as_str)
                .is_none_or(|id| !resolved.contains(id))
        })
        .collect::<Vec<_>>();
    let blocking = unresolved
        .iter()
        .filter(|event| {
            event.get("severity").and_then(serde_json::Value::as_str) == Some("blocking")
        })
        .count() as i64;
    let nonblocking = unresolved.len() as i64 - blocking;
    (approvals, blocking, nonblocking)
}

fn canonical_spec_for_path(path: &str) -> Option<&'static record_store::RecordKindSpec> {
    let first = Path::new(path)
        .components()
        .next()
        .and_then(|component| component.as_os_str().to_str())?;
    record_store::CANONICAL_RECORD_KINDS
        .iter()
        .find(|spec| spec.canonical_dir == Some(first))
}

fn index_changed_record(
    db: &Database,
    state_dir: &Path,
    path: &str,
    record: Record,
) -> Result<IncrementalRepair> {
    match record {
        Record::Issue(record) => {
            let relationships = &record.header.relationships;
            if !relationships.children.is_empty()
                || !relationships.attachments.is_empty()
                || relationships
                    .relates
                    .iter()
                    .any(|link| link.kind != "issue")
            {
                return Ok(IncrementalRepair::NeedsFullRebuild);
            }
            for target in &relationships.blocks {
                if target.kind != "issue" || db.issue_cache_row(&target.id)?.is_none() {
                    return Ok(IncrementalRepair::NeedsFullRebuild);
                }
            }
            for target in &relationships.relates {
                if db.issue_cache_row(&target.id)?.is_none() {
                    return Ok(IncrementalRepair::NeedsFullRebuild);
                }
            }
            let parent_id = db
                .issue_cache_row(&record.header.id)?
                .and_then(|row| row.parent_id);
            let issue_id = record.header.id.clone();
            let dependency_edges = relationships
                .blocks
                .iter()
                .map(|target| (target.id.clone(), issue_id.clone()))
                .collect::<Vec<_>>();
            let relations = relationships
                .relates
                .iter()
                .map(|target| {
                    (
                        issue_id.clone(),
                        target.id.clone(),
                        target.relation_type.clone(),
                    )
                })
                .collect::<Vec<_>>();
            let issue = CanonicalIssue {
                issue: Issue {
                    id: record.header.id,
                    title: record.header.title,
                    description: None,
                    status: record.header.status,
                    issue_type: record.issue_type,
                    priority: record.priority,
                    fields: record.fields,
                    parent_id: parent_id.clone(),
                    created_at: record.header.created_at,
                    updated_at: record.header.updated_at,
                    closed_at: record.closed_at,
                },
                labels: record.header.labels,
                relationships: record.header.relationships,
            };
            index_canonical_issue(
                db,
                state_dir,
                &issue,
                parent_id.as_deref(),
                &dependency_edges,
                &relations,
            )?;
        }
        Record::Evidence(record) => {
            for target in &record.header.relationships.attachments {
                let exists = match target.kind.as_str() {
                    "issue" => db.issue_cache_row(&target.id)?.is_some(),
                    "evidence" => db.evidence_cache_row(&target.id)?.is_some(),
                    "review" => db.review_room_cache_row(&target.id)?.is_some(),
                    _ => false,
                };
                if !exists {
                    return Ok(IncrementalRepair::NeedsFullRebuild);
                }
            }
            index_domain_record(db, state_dir, &Record::Evidence(record))?;
        }
        Record::Review(record) => {
            if db.issue_cache_row(&record.issue_id)?.is_none() {
                return Ok(IncrementalRepair::NeedsFullRebuild);
            }
            index_domain_record(db, state_dir, &Record::Review(record))?;
        }
    }
    let source = db.record_source_cache_row(path)?.ok_or_else(|| {
        anyhow!(
            "domain indexer did not record source metadata for {}",
            display_state_path(Path::new(path))
        )
    })?;
    if source.path != path {
        bail!("indexed source path changed unexpectedly for {path}");
    }
    Ok(IncrementalRepair::Repaired)
}

fn remove_missing_source(
    db: &Database,
    source: &RecordSourceCacheRow,
) -> Result<IncrementalRepair> {
    match source.record_kind.as_str() {
        "issue" => {
            if db.issue_cache_has_graph_edges(&source.record_id)? {
                return Ok(IncrementalRepair::NeedsFullRebuild);
            }
            db.remove_indexed_issue(&source.record_id, &source.path)?;
        }
        "evidence" => db.remove_indexed_evidence(&source.record_id, &source.path)?,
        "review" => db.remove_indexed_review_room(&source.record_id, &source.path)?,
        _ => return Ok(IncrementalRepair::NeedsFullRebuild),
    }
    Ok(IncrementalRepair::Repaired)
}

fn unique_rebuild_path(db_path: &Path) -> Result<PathBuf> {
    let file_name = db_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("Database path has no file name: {}", db_path.display()))?;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("System clock is before UNIX epoch")?
        .as_nanos();
    Ok(db_path.with_file_name(format!(
        ".{file_name}.{}.{}.rebuild-tmp",
        process::id(),
        nanos
    )))
}

fn collect_record_relationship_links(
    record_links: &mut Vec<(String, String, String, String, String)>,
    record_link_keys: &mut BTreeSet<(String, String, String, String, String)>,
    source_kind: &str,
    source_id: &str,
    relationships: &Relationships,
    issue_ids: &BTreeSet<String>,
    record_refs: &BTreeSet<(String, String)>,
) -> Result<()> {
    for child in &relationships.children {
        ensure_record_exists(
            &child.kind,
            &child.id,
            issue_ids,
            record_refs,
            "children",
            source_id,
        )?;
        if source_kind == "issue" && child.kind == "issue" {
            continue;
        }
        push_record_link(
            record_links,
            record_link_keys,
            source_kind,
            source_id,
            &child.kind,
            &child.id,
            child_relation_type(&child.kind),
        )?;
    }
    for attachment in &relationships.attachments {
        ensure_record_exists(
            &attachment.kind,
            &attachment.id,
            issue_ids,
            record_refs,
            &attachment.role,
            source_id,
        )?;
        push_record_link(
            record_links,
            record_link_keys,
            source_kind,
            source_id,
            &attachment.kind,
            &attachment.id,
            &attachment.role,
        )?;
    }
    for relation in &relationships.relates {
        ensure_record_exists(
            &relation.kind,
            &relation.id,
            issue_ids,
            record_refs,
            &relation.relation_type,
            source_id,
        )?;
        if source_kind == "issue" && relation.kind == "issue" {
            continue;
        }
        push_record_link(
            record_links,
            record_link_keys,
            source_kind,
            source_id,
            &relation.kind,
            &relation.id,
            &relation.relation_type,
        )?;
    }
    Ok(())
}

fn push_record_link(
    record_links: &mut Vec<(String, String, String, String, String)>,
    record_link_keys: &mut BTreeSet<(String, String, String, String, String)>,
    source_kind: &str,
    source_id: &str,
    target_kind: &str,
    target_id: &str,
    relation_type: &str,
) -> Result<()> {
    let link = (
        source_kind.to_string(),
        source_id.to_string(),
        target_kind.to_string(),
        target_id.to_string(),
        relation_type.to_string(),
    );
    if !record_link_keys.insert(link.clone()) {
        bail!(
            "Duplicate relationship {} {} -> {} {} ({})",
            source_kind,
            source_id,
            target_kind,
            target_id,
            relation_type
        );
    }
    record_links.push(link);
    Ok(())
}

fn child_relation_type(target_kind: &str) -> &'static str {
    let _ = target_kind;
    "advances"
}

fn ensure_record_exists(
    kind: &str,
    id: &str,
    issue_ids: &BTreeSet<String>,
    record_refs: &BTreeSet<(String, String)>,
    relation: &str,
    source_id: &str,
) -> Result<()> {
    if (kind == "issue" && issue_ids.contains(id))
        || record_refs.contains(&(kind.to_string(), id.to_string()))
    {
        Ok(())
    } else {
        bail!("{source_id} has {relation} reference to missing {kind} {id}")
    }
}

fn validate_issue_child_cycles(edges: &[(String, String)]) -> Result<()> {
    let mut parents = BTreeMap::<String, String>::new();
    for (child, parent) in edges {
        if let Some(existing) = parents.insert(child.clone(), parent.clone()) {
            bail!("Issue {child} appears under multiple parents: {existing}, {parent}");
        }
    }
    validate_directed_acyclic(edges, "children")
}

fn validate_issue_hierarchy_shapes(
    issues: &[CanonicalIssue],
    edges: &[(String, String)],
) -> Result<()> {
    let issue_types = issues
        .iter()
        .map(|issue| (issue.issue.id.as_str(), issue.issue.issue_type.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut children_by_parent = BTreeMap::<&str, Vec<&str>>::new();
    for (child_id, parent_id) in edges {
        let child_type = issue_types
            .get(child_id.as_str())
            .copied()
            .unwrap_or("unknown");
        let parent_type = issue_types
            .get(parent_id.as_str())
            .copied()
            .unwrap_or("unknown");
        if child_type == "mission" {
            bail!(
                "workflow_issue_hierarchy_invalid: mission issue {child_id} cannot have parent {parent_id}; link mission work with advances relationships"
            );
        }
        if child_type == "epic" {
            bail!(
                "workflow_issue_hierarchy_invalid: epic issue {child_id} cannot have parent {parent_id}; epics are root work packages"
            );
        }
        if parent_type != "epic" {
            bail!(
                "workflow_issue_hierarchy_invalid: issue {child_id} cannot be child of {parent_type} {parent_id}; only epics can own child work"
            );
        }
        children_by_parent
            .entry(parent_id.as_str())
            .or_default()
            .push(child_id.as_str());
    }
    for issue in issues {
        if issue.issue.issue_type != "epic" {
            if let Some(children) = children_by_parent.get(issue.issue.id.as_str()) {
                bail!(
                    "workflow_issue_hierarchy_invalid: {} issue {} cannot own child work {}; only epics can own child work",
                    issue.issue.issue_type,
                    issue.issue.id,
                    children.join(", ")
                );
            }
        }
    }
    Ok(())
}

fn validate_dependency_cycles(edges: &[(String, String)]) -> Result<()> {
    validate_directed_acyclic(edges, "blocks")
}

fn validate_directed_acyclic(edges: &[(String, String)], relation: &str) -> Result<()> {
    let mut graph = BTreeMap::<String, Vec<String>>::new();
    for (target, source) in edges {
        graph
            .entry(source.clone())
            .or_default()
            .push(target.clone());
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for node in graph.keys() {
        if has_cycle(node, &graph, &mut visiting, &mut visited) {
            bail!("relationships.{relation} contains a cycle");
        }
    }
    Ok(())
}

fn has_cycle(
    node: &str,
    graph: &BTreeMap<String, Vec<String>>,
    visiting: &mut BTreeSet<String>,
    visited: &mut BTreeSet<String>,
) -> bool {
    if visited.contains(node) {
        return false;
    }
    if !visiting.insert(node.to_string()) {
        return true;
    }
    if let Some(children) = graph.get(node) {
        for child in children {
            if has_cycle(child, graph, visiting, visited) {
                return true;
            }
        }
    }
    visiting.remove(node);
    visited.insert(node.to_string());
    false
}

fn ensure_issue_exists(
    id: &str,
    issue_ids: &BTreeSet<String>,
    relation: &str,
    source_id: &str,
) -> Result<()> {
    if issue_ids.contains(id) {
        Ok(())
    } else {
        bail!(
            "Issue {source_id} has {} reference to missing issue {id}",
            relation
        )
    }
}

fn display_state_path(relative_path: &Path) -> String {
    format!(
        ".atelier/{}",
        relative_path.to_string_lossy().replace('\\', "/")
    )
}

#[cfg(test)]
mod tests {
    use atelier_core::{
        AttachmentRelationship, EvidenceRecord, EvidenceRecordData, IssueSections, RecordHeader,
        RelatesRelationship, ReviewRecord,
    };
    use atelier_records::{CanonicalIssueRecord, RecordStore};
    use chrono::{DateTime, Utc};
    use serde_json::json;
    use std::io::{Seek, SeekFrom};
    use tempfile::TempDir;

    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct CacheSnapshot {
        issues: Vec<(IssueCacheRow, Vec<String>)>,
        evidence: Vec<(EvidenceCacheRow, Vec<EvidenceTargetCacheRow>)>,
        rooms: Vec<ReviewRoomCacheRow>,
        sources: Vec<(String, String, String, i64, Option<i64>)>,
    }

    fn timestamp(revision: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(1_750_000_000 + revision, 0).unwrap()
    }

    fn setup() -> (TempDir, PathBuf, PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let state_dir = directory.path().join(".atelier");
        fs::create_dir_all(&state_dir).unwrap();
        let db_path = state_dir.join("runtime/state.db");
        (directory, state_dir, db_path)
    }

    fn write_issue(
        state_dir: &Path,
        id: &str,
        title: &str,
        revision: i64,
        labels: Vec<&str>,
        relationships: Relationships,
    ) {
        let record = CanonicalIssueRecord {
            issue: Issue {
                id: id.to_string(),
                title: title.to_string(),
                description: None,
                status: "todo".to_string(),
                issue_type: "task".to_string(),
                priority: "high".to_string(),
                fields: BTreeMap::new(),
                parent_id: None,
                created_at: timestamp(1),
                updated_at: timestamp(revision),
                closed_at: None,
            },
            labels: labels.into_iter().map(str::to_string).collect(),
            sections: IssueSections::unchecked_from_body(Some(
                "## Description\n\nFixture issue\n\n## Outcome\n\nCache rows are reproducible.",
            )),
            relationships,
        };
        RecordStore::new(state_dir)
            .write_issue_atomic(&record)
            .unwrap();
    }

    fn write_evidence(state_dir: &Path, id: &str, title: &str, target_id: &str, revision: i64) {
        let record = Record::Evidence(EvidenceRecord {
            header: RecordHeader {
                kind: "evidence".to_string(),
                id: id.to_string(),
                title: title.to_string(),
                status: "pass".to_string(),
                labels: Vec::new(),
                relationships: Relationships {
                    attachments: vec![AttachmentRelationship {
                        kind: "issue".to_string(),
                        id: target_id.to_string(),
                        role: "validates".to_string(),
                    }],
                    ..Relationships::default()
                },
                created_at: timestamp(1),
                updated_at: timestamp(revision),
            },
            data: EvidenceRecordData {
                evidence_type: "test".to_string(),
                captured_at: timestamp(revision),
                command: Some("cargo nextest run".to_string()),
                path: None,
                uri: None,
                producer: None,
                proof_scope: Some("domain cache".to_string()),
                agent_identity: Some("worker".to_string()),
                independence_level: Some("implementer".to_string()),
                residual_risks: Vec::new(),
                follow_up_ids: Vec::new(),
                exit_code: Some(0),
                exit_status: Some("0".to_string()),
                success: Some(true),
                spawn_error: None,
                output: None,
                target: None,
            },
            summary: "Focused proof".to_string(),
        });
        RecordStore::new(state_dir)
            .write_record_atomic(&record)
            .unwrap();
    }

    fn write_review(state_dir: &Path, id: &str, title: &str, issue_id: &str, revision: i64) {
        let events = if revision > 1 {
            vec![
                json!({"id": "event-1", "kind": "opened"}),
                json!({"id": "finding-2", "kind": "finding", "severity": "blocking"}),
                json!({"id": "event-3", "kind": "resolved", "finding": "finding-2"}),
                json!({"id": "event-4", "kind": "approval"}),
            ]
        } else {
            vec![json!({"id": "event-1", "kind": "opened"})]
        };
        let record = Record::Review(ReviewRecord {
            header: RecordHeader {
                kind: "review".to_string(),
                id: id.to_string(),
                title: title.to_string(),
                status: "open".to_string(),
                labels: vec!["review".to_string()],
                relationships: Relationships::default(),
                created_at: timestamp(1),
                updated_at: timestamp(revision),
            },
            mode: "room".to_string(),
            issue_id: issue_id.to_string(),
            source_branch: format!("epic/{issue_id}"),
            target_branch: "main".to_string(),
            events,
        });
        RecordStore::new(state_dir)
            .write_record_atomic(&record)
            .unwrap();
    }

    fn write_domain_set(state_dir: &Path, suffix: &str, revision: i64) -> [String; 3] {
        let issue_id = format!("atelier-{suffix}");
        let evidence_id = format!("atelier-{suffix}e");
        let review_id = format!("atelier-{suffix}r");
        write_issue(
            state_dir,
            &issue_id,
            &format!("Issue {revision}"),
            revision,
            vec![if revision == 1 { "initial" } else { "changed" }],
            Relationships::default(),
        );
        write_evidence(
            state_dir,
            &evidence_id,
            &format!("Evidence {revision}"),
            &issue_id,
            revision,
        );
        write_review(
            state_dir,
            &review_id,
            &format!("Review {revision}"),
            &issue_id,
            revision,
        );
        [issue_id, evidence_id, review_id]
    }

    fn domain_paths(ids: &[String; 3]) -> [String; 3] {
        [
            format!("issues/{}.md", ids[0]),
            format!("evidence/{}.md", ids[1]),
            format!("reviews/{}.yaml", ids[2]),
        ]
    }

    fn report(
        problems: Vec<source_freshness::SourceFreshnessProblem>,
    ) -> source_freshness::SourceFreshnessReport {
        source_freshness::SourceFreshnessReport {
            checked: true,
            source_count: problems.len(),
            problems,
        }
    }

    fn snapshot(database: &Database) -> CacheSnapshot {
        let issue_rows = database
            .query_issue_cache(&atelier_sqlite::IssueCacheQuery::default())
            .unwrap();
        let issues = issue_rows
            .into_iter()
            .map(|row| {
                let labels = database.issue_cache_labels(&row.id).unwrap();
                (row, labels)
            })
            .collect();
        let evidence_rows = database
            .query_evidence_cache(&atelier_sqlite::EvidenceCacheQuery::default())
            .unwrap();
        let evidence = evidence_rows
            .into_iter()
            .map(|row| {
                let targets = database.evidence_cache_targets(&row.id).unwrap();
                (row, targets)
            })
            .collect();
        let rooms = database
            .query_issue_cache(&atelier_sqlite::IssueCacheQuery::default())
            .unwrap()
            .into_iter()
            .flat_map(|issue| database.review_room_cache_for_issue(&issue.id).unwrap())
            .collect();
        let sources = database
            .record_source_cache_rows()
            .unwrap()
            .into_iter()
            .map(|source| {
                (
                    source.path,
                    source.record_kind,
                    source.record_id,
                    source.size_bytes,
                    source.modified_micros,
                )
            })
            .collect();
        CacheSnapshot {
            issues,
            evidence,
            rooms,
            sources,
        }
    }

    fn assert_matches_full(state_dir: &Path, incremental: &Database, full_path: &Path) {
        run(state_dir, full_path).unwrap();
        let full = Database::open(full_path).unwrap();
        assert_eq!(snapshot(incremental), snapshot(&full));
    }

    #[test]
    fn full_and_incremental_add_change_delete_are_equivalent_for_all_domains() {
        let (_directory, state_dir, incremental_path) = setup();
        let base = write_domain_set(&state_dir, "base", 1);
        run(&state_dir, &incremental_path).unwrap();
        let incremental = Database::open(&incremental_path).unwrap();

        write_domain_set(&state_dir, "base", 2);
        let changed = domain_paths(&base)
            .into_iter()
            .map(|path| source_freshness::SourceFreshnessProblem::ChangedSource { path })
            .collect();
        assert_eq!(
            repair_incremental(&incremental, &state_dir, &report(changed)).unwrap(),
            IncrementalRepair::Repaired
        );
        assert_matches_full(
            &state_dir,
            &incremental,
            &state_dir.join("runtime/full-changed.db"),
        );

        let added = write_domain_set(&state_dir, "added", 1);
        let unindexed = domain_paths(&added)
            .into_iter()
            .rev()
            .map(|path| source_freshness::SourceFreshnessProblem::UnindexedSource { path })
            .collect();
        assert_eq!(
            repair_incremental(&incremental, &state_dir, &report(unindexed)).unwrap(),
            IncrementalRepair::Repaired
        );
        assert_matches_full(
            &state_dir,
            &incremental,
            &state_dir.join("runtime/full-added.db"),
        );

        let added_paths = domain_paths(&added);
        for path in &added_paths {
            fs::remove_file(state_dir.join(path)).unwrap();
        }
        let missing = added_paths
            .into_iter()
            .rev()
            .map(|path| source_freshness::SourceFreshnessProblem::MissingSource { path })
            .collect();
        assert_eq!(
            repair_incremental(&incremental, &state_dir, &report(missing)).unwrap(),
            IncrementalRepair::Repaired
        );
        assert_matches_full(
            &state_dir,
            &incremental,
            &state_dir.join("runtime/full-deleted.db"),
        );
    }

    #[test]
    fn one_issue_repair_is_bounded_and_has_one_transaction_owner() {
        let (_directory, state_dir, db_path) = setup();
        let ids = write_domain_set(&state_dir, "bound", 1);
        run(&state_dir, &db_path).unwrap();
        let database = Database::open(&db_path).unwrap();
        let paths = domain_paths(&ids);
        let untouched_source = database
            .record_source_cache_row(&paths[1])
            .unwrap()
            .unwrap();

        write_issue(
            &state_dir,
            &ids[0],
            "Bounded update",
            2,
            vec!["changed"],
            Relationships::default(),
        );
        let outcome = repair_incremental(
            &database,
            &state_dir,
            &report(vec![
                source_freshness::SourceFreshnessProblem::ChangedSource {
                    path: paths[0].clone(),
                },
            ]),
        )
        .unwrap();

        assert_eq!(outcome, IncrementalRepair::Repaired);
        assert_eq!(
            database.issue_cache_row(&ids[0]).unwrap().unwrap().title,
            "Bounded update"
        );
        assert_eq!(
            database
                .record_source_cache_row(&paths[1])
                .unwrap()
                .unwrap(),
            untouched_source
        );
    }

    #[test]
    fn cross_domain_graph_change_requests_one_safe_full_rebuild() {
        let (_directory, state_dir, db_path) = setup();
        let first = write_domain_set(&state_dir, "graph", 1);
        run(&state_dir, &db_path).unwrap();
        let database = Database::open(&db_path).unwrap();
        write_issue(
            &state_dir,
            &first[0],
            "Graph update",
            2,
            vec!["changed"],
            Relationships {
                attachments: vec![AttachmentRelationship {
                    kind: "evidence".to_string(),
                    id: first[1].clone(),
                    role: "evidenced_by".to_string(),
                }],
                ..Relationships::default()
            },
        );

        let outcome = repair_incremental(
            &database,
            &state_dir,
            &report(vec![
                source_freshness::SourceFreshnessProblem::ChangedSource {
                    path: domain_paths(&first)[0].clone(),
                },
            ]),
        )
        .unwrap();
        assert_eq!(outcome, IncrementalRepair::NeedsFullRebuild);
        drop(database);
        run(&state_dir, &db_path).unwrap();
        let rebuilt = Database::open(&db_path).unwrap();
        assert_eq!(
            rebuilt.issue_cache_row(&first[0]).unwrap().unwrap().title,
            "Graph update"
        );
    }

    #[test]
    fn issue_repair_preserves_inbound_relations() {
        let (_directory, state_dir, db_path) = setup();
        let first = write_domain_set(&state_dir, "inbound", 1);
        let second = write_domain_set(&state_dir, "source", 1);
        write_issue(
            &state_dir,
            &second[0],
            "Inbound source",
            1,
            vec!["initial"],
            Relationships {
                relates: vec![RelatesRelationship {
                    kind: "issue".to_string(),
                    id: first[0].clone(),
                    relation_type: "related".to_string(),
                }],
                ..Relationships::default()
            },
        );
        run(&state_dir, &db_path).unwrap();
        let database = Database::open(&db_path).unwrap();

        write_issue(
            &state_dir,
            &first[0],
            "Incremental target update",
            2,
            vec!["changed"],
            Relationships::default(),
        );
        let outcome = repair_incremental(
            &database,
            &state_dir,
            &report(vec![
                source_freshness::SourceFreshnessProblem::ChangedSource {
                    path: domain_paths(&first)[0].clone(),
                },
            ]),
        )
        .unwrap();

        assert_eq!(outcome, IncrementalRepair::Repaired);
        assert_eq!(database.issue_cache_relations(&second[0]).unwrap().len(), 1);
        assert_eq!(
            database.issue_cache_row(&first[0]).unwrap().unwrap().title,
            "Incremental target update"
        );
    }

    #[test]
    fn parse_failure_leaves_rows_and_source_metadata_unchanged() {
        let (_directory, state_dir, db_path) = setup();
        let ids = write_domain_set(&state_dir, "atomic", 1);
        run(&state_dir, &db_path).unwrap();
        let database = Database::open(&db_path).unwrap();
        let path = domain_paths(&ids)[0].clone();
        let before_row = database.issue_cache_row(&ids[0]).unwrap();
        let before_source = database.record_source_cache_row(&path).unwrap();
        fs::write(state_dir.join(&path), "invalid record").unwrap();

        let error = repair_incremental(
            &database,
            &state_dir,
            &report(vec![
                source_freshness::SourceFreshnessProblem::ChangedSource { path },
            ]),
        )
        .unwrap_err();
        assert!(error
            .to_string()
            .contains("Failed to parse changed record file"));
        assert_eq!(database.issue_cache_row(&ids[0]).unwrap(), before_row);
        assert_eq!(
            database
                .record_source_cache_row(&format!("issues/{}.md", ids[0]))
                .unwrap(),
            before_source
        );
    }

    #[test]
    fn full_rebuild_replaces_schema_mismatch_as_disposable_state() {
        let (_directory, state_dir, db_path) = setup();
        let ids = write_domain_set(&state_dir, "schema", 1);
        fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        drop(Database::open(&db_path).unwrap());
        let mut file = OpenOptions::new().write(true).open(&db_path).unwrap();
        file.seek(SeekFrom::Start(60)).unwrap();
        file.write_all(&99_u32.to_be_bytes()).unwrap();
        drop(file);
        assert!(
            atelier_sqlite::cache_incompatibility(&Database::open(&db_path).err().unwrap())
                .is_some()
        );

        run(&state_dir, &db_path).unwrap();
        let rebuilt = Database::open(&db_path).unwrap();
        assert!(rebuilt.issue_cache_row(&ids[0]).unwrap().is_some());
    }
}
