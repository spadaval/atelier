use anyhow::{anyhow, bail, Context, Result};
use fs2::FileExt;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::activity::IssueActivity;
use crate::db::Database;
use crate::models::Issue;
use crate::projection_index;
use crate::record_store::{
    self, CanonicalDomainRecord, IssueSections, Relationships, FIRST_CLASS_RECORD_KINDS,
};

#[derive(Debug)]
struct CanonicalIssue {
    issue: Issue,
    labels: Vec<String>,
    sections: IssueSections,
    relationships: Relationships,
}

#[derive(Debug)]
struct RebuildProjection {
    issues: Vec<CanonicalIssue>,
    records: Vec<CanonicalDomainRecord>,
    child_edges: Vec<(String, String)>,
    dependency_edges: Vec<(String, String)>,
    relations: Vec<(String, String, String)>,
    record_links: Vec<(String, String, String, String, String)>,
}

pub fn run(state_dir: &Path, db_path: &Path) -> Result<()> {
    let _lock = ProjectionRebuildLock::acquire(db_path)?;
    let rebuild = load_projection(state_dir)?;
    write_rebuilt_database(state_dir, db_path, &rebuild, None)?;
    eprintln!("Rebuilt {} from {}", db_path.display(), state_dir.display());
    Ok(())
}

pub fn refresh_projection_preserving_runtime(state_dir: &Path, db_path: &Path) -> Result<()> {
    let _lock = ProjectionRebuildLock::acquire(db_path)?;
    let rebuild = load_projection(state_dir)?;
    write_rebuilt_database(state_dir, db_path, &rebuild, Some(db_path))?;
    eprintln!(
        "Refreshed projection in {} from {}",
        db_path.display(),
        state_dir.display()
    );
    Ok(())
}

struct ProjectionRebuildLock {
    file: File,
}

impl ProjectionRebuildLock {
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
                format!("Failed to open projection rebuild lock {}", path.display())
            })?;

        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            match file.try_lock_exclusive() {
                Ok(()) => {
                    file.set_len(0).with_context(|| {
                        format!(
                            "Failed to refresh projection rebuild lock {}",
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
                        format!("Failed to write projection rebuild lock {}", path.display())
                    })?;
                    return Ok(Self { file });
                }
                Err(error) if is_lock_contention(&error) && Instant::now() < deadline => {
                    thread::sleep(Duration::from_millis(25));
                }
                Err(error) => {
                    return Err(error).with_context(|| {
                        format!(
                            "Projection rebuild is already running for {}; retry the command after the current rebuild finishes. \
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

impl Drop for ProjectionRebuildLock {
    fn drop(&mut self) {
        if let Err(error) = self.file.unlock() {
            tracing::warn!("failed to unlock projection rebuild lock: {}", error);
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

pub(crate) fn validate_canonical_state(state_dir: &Path) -> Result<()> {
    load_projection(state_dir).map(|_| ())
}

fn load_projection(state_dir: &Path) -> Result<RebuildProjection> {
    ProjectionLoader::new(state_dir).load()
}

struct ProjectionLoader<'a> {
    state_dir: &'a Path,
    store: record_store::RecordStore,
    issues: Vec<CanonicalIssue>,
    records: Vec<CanonicalDomainRecord>,
    issue_ids: BTreeSet<String>,
    global_ids: BTreeSet<String>,
    record_refs: BTreeSet<(String, String)>,
    canonical_paths: BTreeSet<PathBuf>,
    activity_subject_ids: BTreeSet<String>,
}

impl<'a> ProjectionLoader<'a> {
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
            activity_subject_ids: BTreeSet::new(),
        }
    }

    fn load(mut self) -> Result<RebuildProjection> {
        self.load_issues()?;
        self.load_issue_activities()?;
        self.load_domain_records()?;
        ensure_no_unsupported_canonical_files(self.state_dir, &self.canonical_paths)?;

        let (child_edges, dependency_edges, relations) = self.validate_issue_relationships()?;
        let record_links = self.collect_record_links()?;
        validate_issue_child_cycles(&child_edges)?;
        validate_dependency_cycles(&dependency_edges)?;

        self.issues.sort_by(|a, b| a.issue.id.cmp(&b.issue.id));
        self.records
            .sort_by(|a, b| (&a.record.kind, &a.record.id).cmp(&(&b.record.kind, &b.record.id)));
        Ok(RebuildProjection {
            issues: self.issues,
            records: self.records,
            child_edges,
            dependency_edges,
            relations,
            record_links,
        })
    }

    fn load_issues(&mut self) -> Result<()> {
        for relative in self.store.discover_issue_paths()? {
            self.canonical_paths.insert(relative.clone());
            let record = self.store.load_issue(&relative)?;
            let issue = CanonicalIssue {
                issue: record.issue,
                labels: record.labels,
                sections: record.sections,
                relationships: record.relationships,
            };
            self.register_issue_id(&issue.issue.id)?;
            self.issues.push(issue);
        }
        Ok(())
    }

    fn register_issue_id(&mut self, id: &str) -> Result<()> {
        if !self.issue_ids.insert(id.to_string()) {
            bail!("Duplicate issue ID in canonical projection: {}", id);
        }
        if !self.global_ids.insert(id.to_string()) {
            bail!("Duplicate record ID in canonical projection: {}", id);
        }
        Ok(())
    }

    fn load_issue_activities(&mut self) -> Result<()> {
        for relative in discover_issue_activity_paths(self.state_dir)? {
            let activity = IssueActivity::load(self.state_dir, &relative)?;
            self.canonical_paths.insert(relative);
            self.activity_subject_ids.insert(activity.subject_id);
        }
        Ok(())
    }

    fn load_domain_records(&mut self) -> Result<()> {
        for spec in FIRST_CLASS_RECORD_KINDS {
            for relative in discover_record_paths(self.state_dir, spec)? {
                self.canonical_paths.insert(relative.clone());
                let record = self.store.load_domain_record(&relative, spec)?;
                self.register_domain_record(&record)?;
                self.records.push(record);
            }
        }
        Ok(())
    }

    fn register_domain_record(&mut self, record: &CanonicalDomainRecord) -> Result<()> {
        if !self.global_ids.insert(record.record.id.clone()) {
            bail!(
                "Duplicate record ID in canonical projection: {}",
                record.record.id
            );
        }
        if !self
            .record_refs
            .insert((record.record.kind.clone(), record.record.id.clone()))
        {
            bail!(
                "Duplicate {} ID in canonical projection: {}",
                record.record.kind,
                record.record.id
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
        let mut graph = IssueRelationshipProjection::default();
        for subject_id in &self.activity_subject_ids {
            ensure_issue_exists(subject_id, &self.issue_ids, "activity", subject_id)?;
        }
        for issue in &self.issues {
            graph.collect_issue(issue, &self.issue_ids, &self.record_refs)?;
        }
        Ok((graph.child_edges, graph.dependency_edges, graph.relations))
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
            collect_record_relationship_links(
                &mut record_links,
                &mut record_link_keys,
                &record.record.kind,
                &record.record.id,
                &record.relationships,
                &self.issue_ids,
                &self.record_refs,
            )?;
        }
        Ok(record_links)
    }
}

#[derive(Default)]
struct IssueRelationshipProjection {
    relations: Vec<(String, String, String)>,
    relation_keys: BTreeSet<(String, String, String)>,
    child_edges: Vec<(String, String)>,
    child_edge_keys: BTreeSet<(String, String)>,
    dependency_edges: Vec<(String, String)>,
    dependency_edge_keys: BTreeSet<(String, String)>,
}

impl IssueRelationshipProjection {
    fn collect_issue(
        &mut self,
        issue: &CanonicalIssue,
        issue_ids: &BTreeSet<String>,
        record_refs: &BTreeSet<(String, String)>,
    ) -> Result<()> {
        self.collect_blocks(issue, issue_ids)?;
        self.collect_children(issue, issue_ids)?;
        self.collect_relations(issue, issue_ids, record_refs)
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
    ) -> Result<()> {
        for relation in &issue.relationships.relates {
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

fn collect_issue_record_paths(root: &Path, dir: &Path, records: &mut Vec<PathBuf>) -> Result<()> {
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
            collect_issue_record_paths(root, &path, records)?;
        } else if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .context("Failed to relativize canonical issue path")?
                .to_path_buf();
            if crate::storage_layout::is_local_atelier_path(&relative) {
                continue;
            }
            if relative.extension().and_then(|ext| ext.to_str()) != Some("md") {
                bail!(
                    "Unsupported canonical issue file {}; expected Markdown .md record",
                    display_state_path(&relative)
                );
            }
            records.push(relative);
        }
    }
    Ok(())
}

fn discover_record_paths(
    state_dir: &Path,
    spec: &record_store::RecordKindSpec,
) -> Result<Vec<PathBuf>> {
    let dir_name = spec.canonical_dir.ok_or_else(|| {
        anyhow!(
            "Record kind '{}' does not have a canonical directory",
            spec.kind
        )
    })?;
    let record_dir = state_dir.join(dir_name);
    if !record_dir.exists() {
        return Ok(Vec::new());
    }
    let mut records = Vec::new();
    collect_issue_record_paths(state_dir, &record_dir, &mut records)?;
    records.sort();
    Ok(records)
}

fn discover_issue_activity_paths(state_dir: &Path) -> Result<Vec<PathBuf>> {
    let issue_dir = state_dir.join("issues");
    if !issue_dir.exists() {
        return Ok(Vec::new());
    }

    let mut records = Vec::new();
    collect_issue_activity_paths(state_dir, &issue_dir, &mut records)?;
    records.sort();
    Ok(records)
}

fn collect_issue_activity_paths(root: &Path, dir: &Path, records: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(".activity"))
            {
                collect_canonical_files(root, &path, records)?;
            } else {
                collect_issue_activity_paths(root, &path, records)?;
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
        bail!(
            "Unsupported canonical projection file {}",
            display_state_path(&relative)
        );
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
            .context("Failed to relativize canonical projection path")?;
        if crate::storage_layout::is_local_atelier_path(relative) {
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

fn write_rebuilt_database(
    state_dir: &Path,
    db_path: &Path,
    rebuild: &RebuildProjection,
    preserve_runtime_from: Option<&Path>,
) -> Result<()> {
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
        db.transaction(|| {
            for issue in &rebuild.issues {
                let mut row = issue.issue.clone();
                row.parent_id = None;
                row.description = Some(issue.sections.searchable_text());
                db.insert_issue_rebuild(&row)?;
            }
            for (child_id, parent_id) in &rebuild.child_edges {
                let updated_at = rebuild
                    .issues
                    .iter()
                    .find(|issue| issue.issue.id == *child_id)
                    .map(|issue| issue.issue.updated_at)
                    .ok_or_else(|| anyhow!("Missing child issue {child_id}"))?;
                db.update_parent_import(child_id, Some(parent_id), &updated_at)?;
            }
            for issue in &rebuild.issues {
                for label in &issue.labels {
                    db.add_label(&issue.issue.id, label)?;
                }
            }
            for (blocked_id, blocker_id) in &rebuild.dependency_edges {
                db.add_dependency(blocked_id, blocker_id)?;
            }
            for (source, target, relation_type) in &rebuild.relations {
                db.add_typed_relation(&source, &target, relation_type)?;
            }
            for record in &rebuild.records {
                db.insert_record_rebuild(&record.record)?;
            }
            for (source_kind, source_id, target_kind, target_id, relation_type) in
                &rebuild.record_links
            {
                db.add_record_link(
                    source_kind,
                    source_id,
                    target_kind,
                    target_id,
                    relation_type,
                )?;
            }
            if let Some(source_db_path) = preserve_runtime_from.filter(|path| path.exists()) {
                copy_runtime_state(&db, source_db_path)?;
            }
            Ok(())
        })?;
        projection_index::refresh(&db, state_dir)?;
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

fn copy_runtime_state(db: &Database, source_db_path: &Path) -> Result<()> {
    let source = source_db_path.to_string_lossy();
    db.conn
        .execute("ATTACH DATABASE ?1 AS runtime_src", [&source.as_ref()])
        .with_context(|| {
            format!(
                "Failed to attach runtime state {}",
                source_db_path.display()
            )
        })?;
    let copy_result = (|| -> Result<()> {
        db.conn.execute(
            "INSERT OR IGNORE INTO sessions
             (id, started_at, ended_at, active_issue_id, handoff_notes, last_action, agent_id)
             SELECT id, started_at, ended_at, active_issue_id, handoff_notes, last_action, agent_id
             FROM runtime_src.sessions
             WHERE active_issue_id IS NULL
                OR EXISTS (SELECT 1 FROM issues WHERE issues.id = runtime_src.sessions.active_issue_id)",
            [],
        )?;
        db.conn.execute(
            "INSERT OR IGNORE INTO work_associations
             (issue_id, status, branch, worktree_path, started_at, finished_at)
             SELECT issue_id, status, branch, worktree_path, started_at, finished_at
             FROM runtime_src.work_associations
             WHERE EXISTS (SELECT 1 FROM issues WHERE issues.id = runtime_src.work_associations.issue_id)",
            [],
        )?;
        Ok(())
    })();
    copy_result?;
    Ok(())
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
    match target_kind {
        "milestone" => "has_checkpoint",
        _ => "advances",
    }
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
    use super::*;
    use tempfile::tempdir;

    use crate::commands::export;
    use crate::record_store::issue_record_path;

    fn setup_test_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::open(&db_path).unwrap();
        (db, dir)
    }

    #[test]
    fn rebuild_round_trips_canonical_issue_state() {
        let (db, dir) = setup_test_db();
        let parent = db
            .create_issue("Parent", Some("Parent body"), "high")
            .unwrap();
        let child = db
            .create_subissue(&parent, "Child", Some("Child body"), "low")
            .unwrap();
        db.add_label(&child, "alpha").unwrap();
        db.add_label(&child, "zeta").unwrap();
        db.add_dependency(&child, &parent).unwrap();
        db.add_typed_relation(&parent, &child, "derived").unwrap();

        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let rebuilt_path = dir.path().join(".atelier/runtime/state.db");
        run(&state_dir, &rebuilt_path).unwrap();
        let rebuilt = Database::open(&rebuilt_path).unwrap();

        let rebuilt_parent = rebuilt.get_issue(&parent).unwrap().unwrap();
        let rebuilt_child = rebuilt.get_issue(&child).unwrap().unwrap();
        assert_eq!(rebuilt_parent.title, "Parent");
        assert_eq!(rebuilt_child.title, "Child");
        assert_eq!(rebuilt_child.parent_id, Some(parent.clone()));
        assert_eq!(rebuilt_child.priority, "low");
        assert_eq!(rebuilt.get_labels(&child).unwrap(), vec!["alpha", "zeta"]);
        assert_eq!(rebuilt.get_blockers(&child).unwrap(), vec![parent.clone()]);
        assert_eq!(rebuilt.get_blocking(&parent).unwrap(), vec![child.clone()]);

        let rebuilt_state_dir = dir.path().join(".rebuilt-state");
        export::run_canonical(&rebuilt, &rebuilt_state_dir, false).unwrap();
        assert_eq!(
            fs::read_to_string(state_dir.join(issue_record_path(&child))).unwrap(),
            fs::read_to_string(rebuilt_state_dir.join(issue_record_path(&child))).unwrap()
        );
        assert!(!rebuilt_state_dir.join("graph.json").exists());
        assert_eq!(rebuilt.get_typed_relations(&parent).unwrap().len(), 1);
    }

    #[test]
    fn rebuild_allows_parent_records_after_children() {
        let (db, dir) = setup_test_db();
        let child = db.create_issue("Child", Some("Child body"), "low").unwrap();
        let parent = db
            .create_issue("Parent", Some("Parent body"), "high")
            .unwrap();
        db.update_parent(&child, Some(&parent)).unwrap();

        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let rebuilt_path = dir.path().join(".atelier/runtime/state.db");
        run(&state_dir, &rebuilt_path).unwrap();
        let rebuilt = Database::open(&rebuilt_path).unwrap();

        let rebuilt_child = rebuilt.get_issue(&child).unwrap().unwrap();
        assert_eq!(rebuilt_child.parent_id, Some(parent.clone()));
    }

    #[test]
    fn rebuild_round_trips_canonical_domain_records() {
        let (db, dir) = setup_test_db();
        let mission_sections = record_store::mission_sections_from_inputs(
            "Mission",
            Some("Mission body"),
            vec!["keep contract".to_string()],
            Vec::new(),
            Vec::new(),
        );
        let mission_body = record_store::render_mission_sections(&mission_sections);
        let mission_id = db
            .create_record(
                "mission",
                "Mission",
                "ready",
                Some(&mission_body),
                record_store::MISSION_EMPTY_DATA_JSON,
            )
            .unwrap();
        let plan_id = db
            .create_record(
                "plan",
                "Plan",
                "open",
                Some("Plan body"),
                r#"{"revision":1,"revisions":[]}"#,
            )
            .unwrap();
        let evidence_id = db
            .create_record(
                "evidence",
                "Evidence",
                "pass",
                Some("Evidence body"),
                r#"{"kind":"test","result":"pass"}"#,
            )
            .unwrap();
        db.add_record_link("mission", &mission_id, "plan", &plan_id, "planned_by")
            .unwrap();
        db.add_record_link(
            "evidence",
            &evidence_id,
            "mission",
            &mission_id,
            "validates",
        )
        .unwrap();

        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();
        let mission_path = state_dir.join("missions").join(format!("{mission_id}.md"));
        let mission_markdown = fs::read_to_string(&mission_path).unwrap();
        assert!(mission_markdown.contains("schema: \"atelier.mission\""));
        assert!(mission_markdown.contains("schema_version: 1"));
        assert!(!mission_markdown.contains("\ndata: "));
        assert!(mission_markdown.contains("labels:\n- \"mission\"\n"));
        assert!(mission_markdown.contains("## Intent\n\nMission body"));
        assert!(mission_markdown.contains("## Constraints\n\n- keep contract"));
        assert!(mission_markdown.contains(&format!(
            "  attachments:\n  - kind: \"plan\"\n    id: \"{plan_id}\"\n    role: \"planned_by\"\n"
        )));
        assert!(!mission_markdown.contains(&format!("id: \"{evidence_id}\"")));

        let evidence_path = state_dir.join("evidence").join(format!("{evidence_id}.md"));
        let evidence_markdown = fs::read_to_string(&evidence_path).unwrap();
        assert!(evidence_markdown.contains(&format!("id: \"{mission_id}\"")));

        let rebuilt_path = dir.path().join(".atelier/runtime/state.db");
        run(&state_dir, &rebuilt_path).unwrap();
        let rebuilt = Database::open(&rebuilt_path).unwrap();

        let mission = rebuilt.get_record("mission", &mission_id).unwrap().unwrap();
        assert_eq!(mission.title, "Mission");
        assert!(mission
            .body
            .as_deref()
            .unwrap()
            .contains("## Intent\n\nMission body"));
        assert!(mission
            .body
            .as_deref()
            .unwrap()
            .contains("## Constraints\n\n- keep contract"));
        assert_eq!(mission.data_json, "{}");
        assert!(rebuilt.get_record("plan", &plan_id).unwrap().is_some());
        assert!(rebuilt
            .get_record("evidence", &evidence_id)
            .unwrap()
            .is_some());

        let mission_links = rebuilt.list_record_links("mission", &mission_id).unwrap();
        assert!(mission_links.iter().any(|link| {
            link.source_kind == "mission"
                && link.source_id == mission_id
                && link.target_kind == "plan"
                && link.target_id == plan_id
                && link.relation_type == "planned_by"
        }));
        assert!(mission_links.iter().any(|link| {
            link.source_kind == "evidence"
                && link.source_id == evidence_id
                && link.target_kind == "mission"
                && link.target_id == mission_id
                && link.relation_type == "validates"
        }));
    }

    #[test]
    fn record_table_rejects_non_canonical_record_kinds() {
        let (db, _dir) = setup_test_db();
        let error = db
            .create_record(
                "workflow_validator",
                "Deferred validator",
                "open",
                None,
                "{}",
            )
            .unwrap_err();
        assert!(error
            .to_string()
            .contains("not a canonical first-class record"));
    }

    #[test]
    fn rebuild_rejects_global_id_collision_across_record_kinds() {
        let (db, dir) = setup_test_db();
        let issue_id = db.create_issue("Issue", None, "medium").unwrap();
        let mission_sections = record_store::mission_sections_from_inputs(
            "Mission",
            Some("Mission body"),
            vec!["global ID collision is rejected".to_string()],
            Vec::new(),
            Vec::new(),
        );
        let mission_body = record_store::render_mission_sections(&mission_sections);
        let mission_id = db
            .create_record(
                "mission",
                "Mission",
                "ready",
                Some(&mission_body),
                record_store::MISSION_EMPTY_DATA_JSON,
            )
            .unwrap();

        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();
        let old_path = state_dir.join("missions").join(format!("{mission_id}.md"));
        let new_path = state_dir.join("missions").join(format!("{issue_id}.md"));
        let mission_markdown = fs::read_to_string(&old_path).unwrap().replace(
            &format!("id: \"{mission_id}\""),
            &format!("id: \"{issue_id}\""),
        );
        fs::write(&new_path, mission_markdown).unwrap();
        fs::remove_file(old_path).unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/runtime/state.db")).unwrap_err();
        assert!(error.to_string().contains(&format!(
            "Duplicate record ID in canonical projection: {issue_id}"
        )));
    }

    #[test]
    fn rebuild_succeeds_without_manifest_or_graph() {
        let (db, dir) = setup_test_db();
        db.create_issue("Standalone", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();

        assert!(!state_dir.join("manifest.json").exists());
        assert!(!state_dir.join("graph.json").exists());
        run(&state_dir, &dir.path().join(".atelier/runtime/state.db")).unwrap();
    }

    #[test]
    fn rebuild_recreates_canonical_projection_and_resets_runtime_state() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Runtime reset", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let session_id = db.start_session().unwrap();
        db.set_session_issue(session_id, &id).unwrap();
        db.start_work_association(&id, Some("branch"), Some("/tmp/worktree"))
            .unwrap();
        assert!(db.get_current_session().unwrap().is_some());
        assert!(db.get_active_work_association().unwrap().is_some());

        let db_path = dir.path().join(".atelier/runtime/state.db");
        run(&state_dir, &db_path).unwrap();
        let rebuilt = Database::open(&db_path).unwrap();

        assert!(rebuilt.require_issue(&id).is_ok());
        assert!(rebuilt.runtime_state_tables_available().unwrap());
        assert!(rebuilt.get_current_session().unwrap().is_none());
        assert!(rebuilt.get_active_work_association().unwrap().is_none());
    }

    #[test]
    fn refresh_projection_preserves_valid_runtime_state() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join(".atelier/runtime/state.db");
        fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        let db = Database::open(&db_path).unwrap();
        let id = db
            .create_issue("Runtime preserved", None, "medium")
            .unwrap();
        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let session_id = db.start_session().unwrap();
        db.set_session_issue(session_id, &id).unwrap();
        db.start_work_association(&id, Some("branch"), Some("/tmp/worktree"))
            .unwrap();
        drop(db);

        refresh_projection_preserving_runtime(&state_dir, &db_path).unwrap();
        let refreshed = Database::open(&db_path).unwrap();

        assert!(refreshed.require_issue(&id).is_ok());
        assert_eq!(
            refreshed
                .get_current_session()
                .unwrap()
                .and_then(|session| session.active_issue_id),
            Some(id.clone())
        );
        assert_eq!(
            refreshed
                .get_active_work_association()
                .unwrap()
                .map(|work| work.issue_id),
            Some(id)
        );
    }

    #[test]
    fn rebuild_accepts_issue_activity_sidecars() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Activity", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();
        write_activity_sidecar(&state_dir, &id);

        run(&state_dir, &dir.path().join(".atelier/runtime/state.db")).unwrap();
        validate_canonical_state(&state_dir).unwrap();
    }

    #[test]
    fn rebuild_rejects_activity_for_missing_issue() {
        let (db, dir) = setup_test_db();
        db.create_issue("Only issue", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();
        write_activity_sidecar(&state_dir, "atelier-miss");

        let error = run(&state_dir, &dir.path().join(".atelier/runtime/state.db")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Issue atelier-miss has activity reference to missing issue atelier-miss"));
    }

    #[test]
    fn rebuild_reports_path_id_mismatch() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Mismatch", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();
        let wrong_id = "atelier-zzzz";
        fs::rename(
            state_dir.join(issue_record_path(&id)),
            state_dir.join(issue_record_path(wrong_id)),
        )
        .unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/runtime/state.db")).unwrap_err();
        assert!(error.to_string().contains(&format!(
            "does not match canonical path .atelier/issues/{id}.md"
        )));
    }

    #[test]
    fn rebuild_reports_malformed_front_matter() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Malformed", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();
        fs::write(state_dir.join(issue_record_path(&id)), "not front matter\n").unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/runtime/state.db")).unwrap_err();
        assert!(error.to_string().contains(&format!(
            "Missing YAML front matter in .atelier/issues/{id}.md"
        )));
    }

    #[test]
    fn rebuild_reports_schema_mismatch() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Wrong schema", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();
        let path = state_dir.join(issue_record_path(&id));
        let text = fs::read_to_string(&path)
            .unwrap()
            .replace("schema: \"atelier.issue\"", "schema: \"atelier.graph\"");
        fs::write(path, text).unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/runtime/state.db")).unwrap_err();
        assert!(error
            .to_string()
            .contains("Unsupported schema 'atelier.graph'"));
    }

    #[test]
    fn rebuild_reports_dangling_dependency_and_duplicate_link() {
        let (db, dir) = setup_test_db();
        let id = db.create_issue("Source", None, "medium").unwrap();
        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let missing_id = "atelier-zzzz";
        let path = state_dir.join(issue_record_path(&id));
        let text = fs::read_to_string(&path).unwrap().replace(
            "  blocks: []",
            &format!("  blocks:\n  - kind: \"issue\"\n    id: \"{missing_id}\""),
        );
        fs::write(&path, text).unwrap();
        let error = run(&state_dir, &dir.path().join(".atelier/runtime/state.db")).unwrap_err();
        assert!(error.to_string().contains(&format!(
            "{id} has blocks reference to missing issue {missing_id}"
        )));

        let text = fs::read_to_string(&path)
            .unwrap()
            .replace(
                &format!("  blocks:\n  - kind: \"issue\"\n    id: \"{missing_id}\""),
                "  blocks: []",
            )
            .replace(
                "  relates: []",
                &format!(
                    "  relates:\n  - kind: \"issue\"\n    id: \"{id}\"\n    type: \"related\"\n  - kind: \"issue\"\n    id: \"{id}\"\n    type: \"related\""
                ),
            );
        fs::write(&path, text).unwrap();
        let error = run(&state_dir, &dir.path().join(".atelier/runtime/state.db")).unwrap_err();
        assert!(error.to_string().contains(&format!(
            "Duplicate relationships.relates target issue {id} (related)"
        )));
    }

    #[test]
    fn rebuild_reports_invalid_relation_type() {
        let (db, dir) = setup_test_db();
        let first = db.create_issue("First", None, "medium").unwrap();
        let second = db.create_issue("Second", None, "medium").unwrap();
        db.add_typed_relation(&first, &second, "related").unwrap();
        let state_dir = dir.path().join(".atelier");
        export::run_canonical(&db, &state_dir, false).unwrap();

        let path = [first.as_str(), second.as_str()]
            .into_iter()
            .map(|id| state_dir.join(issue_record_path(id)))
            .find(|path| {
                fs::read_to_string(path)
                    .map(|text| text.contains("type: \"related\""))
                    .unwrap_or(false)
            })
            .unwrap();
        let text = fs::read_to_string(&path)
            .unwrap()
            .replace("type: \"related\"", "type: \"\"");
        fs::write(path, text).unwrap();

        let error = run(&state_dir, &dir.path().join(".atelier/runtime/state.db")).unwrap_err();
        assert!(error.to_string().contains("Relation type cannot be empty"));
    }

    fn write_activity_sidecar(state_dir: &Path, issue_id: &str) {
        let activity_path = state_dir
            .join("issues")
            .join(format!("{issue_id}.activity"))
            .join("20260610T181920123456Z.md");
        fs::create_dir_all(activity_path.parent().unwrap()).unwrap();
        fs::write(
            activity_path,
            format!(
                "---\nschema: \"atelier.activity\"\nschema_version: 1\nid: \"20260610T181920123456Z\"\nsubject_kind: \"issue\"\nsubject_id: \"{issue_id}\"\nevent_type: \"comment\"\nactor: \"tester\"\ncreated_at: \"2026-06-10T18:19:20.123456Z\"\nsummary: \"Activity\"\n---\n\nBody\n"
            ),
        )
        .unwrap();
    }
}
