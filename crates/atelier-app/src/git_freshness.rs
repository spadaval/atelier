//! Git history primitives shared by evidence and review freshness checks.
use anyhow::{bail, Context, Result};
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    rc::Rc,
};

#[derive(Clone)]
pub struct CommitEdge {
    pub parent: String,
    pub commit: String,
    pub paths: Vec<PathBuf>,
    /// A merge commit's other parents.
    pub sides: Vec<String>,
}

/// One read phase's captured endpoint and fresh checkout proof.
/// Create a new context after mutations or external checks.
pub struct ReadContext<'a> {
    pub(crate) repo: &'a Path,
    head: Option<String>,
    clean: Option<bool>,
    /// Git objects are immutable and every query is pinned to a full commit
    /// ID, so answers are memoized for the read phase (atelier-0349).
    cache: ObjectCache,
    /// One `git cat-file --batch` process serves every object read, instead
    /// of a Git process per query (atelier-0349).
    reader: Option<ObjectReader>,
}

struct ObjectReader {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: BufReader<ChildStdout>,
}

struct Object {
    oid: String,
    kind: String,
    data: Vec<u8>,
}

struct TreeEntry {
    mode: String,
    name: Vec<u8>,
    oid: String,
}

impl ObjectReader {
    fn spawn(repo: &Path) -> Result<Self> {
        let mut child = command(repo)
            .args(["cat-file", "--batch"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("cannot start git cat-file --batch")?;
        let stdin = child.stdin.take().context("git cat-file stdin")?;
        let stdout = BufReader::new(child.stdout.take().context("git cat-file stdout")?);
        Ok(Self {
            child,
            stdin: Some(stdin),
            stdout,
        })
    }
    /// The object an expression names, or `None` when it names none.
    fn read(&mut self, expression: &str) -> Result<Option<Object>> {
        if expression.contains(['\n', '\r']) {
            bail!("Git object expression contains a line break");
        }
        let stdin = self.stdin.as_mut().context("git cat-file is closed")?;
        stdin.write_all(expression.as_bytes())?;
        stdin.write_all(b"\n")?;
        stdin.flush()?;
        let mut header = String::new();
        if self.stdout.read_line(&mut header)? == 0 {
            bail!("git cat-file --batch exited");
        }
        let header = header.strip_suffix('\n').unwrap_or(&header);
        let fields = header.split(' ').collect::<Vec<_>>();
        let object = match fields.as_slice() {
            [oid, kind, size]
                if matches!(oid.len(), 40 | 64)
                    && oid.bytes().all(|b| b.is_ascii_hexdigit())
                    && size.bytes().all(|b| b.is_ascii_digit()) =>
            {
                let mut data = vec![0; size.parse()?];
                self.stdout.read_exact(&mut data)?;
                let mut newline = [0];
                self.stdout.read_exact(&mut newline)?;
                if newline != *b"\n" {
                    bail!("invalid git cat-file --batch output");
                }
                Object {
                    oid: oid.to_ascii_lowercase(),
                    kind: (*kind).to_owned(),
                    data,
                }
            }
            _ if header.ends_with(" missing") => return Ok(None),
            _ => bail!("git cat-file --batch: {header}"),
        };
        Ok(Some(object))
    }
}

impl Drop for ObjectReader {
    fn drop(&mut self) {
        drop(self.stdin.take());
        let _ = self.child.wait();
    }
}

#[derive(Default)]
struct ObjectCache {
    commit_ids: HashMap<String, String>,
    blobs: HashMap<(String, PathBuf), String>,
    entries: HashMap<(String, PathBuf), Option<(String, String)>>,
    ancestors: HashMap<(String, String), bool>,
    /// Ancestry queries per later commit; often-asked commits get their
    /// whole ancestor set listed once instead of a query per pair.
    ancestor_queries: HashMap<String, usize>,
    ancestor_sets: HashMap<String, Rc<HashSet<String>>>,
    changed: HashMap<(String, String), Vec<PathBuf>>,
    edges: HashMap<(String, String), Rc<Vec<CommitEdge>>>,
    status_only: HashMap<(String, String, PathBuf), bool>,
    first_parents: HashMap<String, Option<String>>,
    authors: HashMap<String, String>,
    trees: HashMap<String, Rc<Vec<TreeEntry>>>,
    commit_trees: HashMap<String, String>,
    histories: HashMap<Vec<String>, String>,
    commits: HashMap<String, Vec<u8>>,
    merge_resolutions: HashMap<String, Option<MergeResolution>>,
    /// Whether tree diffs matched `git diff` on this read phase's first
    /// edge; `None` until checked, `false` to use `git diff` throughout.
    tree_diff_matches: Option<bool>,
    /// Whether `git show` prints author fields unconverted.
    utf8_log_output: Option<bool>,
}

impl<'a> ReadContext<'a> {
    pub fn new(repo: &'a Path) -> Self {
        Self {
            repo,
            head: None,
            clean: None,
            cache: ObjectCache::default(),
            reader: None,
        }
    }
    pub fn from_snapshot(repo: &'a Path, head: String, dirty: &[PathBuf]) -> Self {
        Self::from_head(repo, head).with_working_paths(dirty)
    }
    pub fn from_head(repo: &'a Path, head: String) -> Self {
        Self {
            repo,
            head: Some(head),
            clean: None,
            cache: ObjectCache::default(),
            reader: None,
        }
    }
    /// Attach successful fresh working-path proof for this same read phase.
    pub fn with_working_paths(mut self, dirty: &[PathBuf]) -> Self {
        self.clean = Some(!dirty.iter().any(|path| !is_record_path(path)));
        self
    }
    pub(crate) fn pinned(&mut self, commit: &str) -> Result<String> {
        let expression = if let Some(suffix) = commit.strip_prefix("HEAD") {
            if !suffix.is_empty() && !suffix.starts_with(['^', '~']) {
                bail!("Git context requires a pinned commit");
            }
            if self.head.is_none() {
                self.head = Some(head_commit(self.repo)?);
            }
            format!("{}{suffix}", self.head.as_ref().expect("pinned HEAD"))
        } else {
            commit.to_owned()
        };
        let base = expression.split(['^', '~']).next().unwrap_or("");
        if !matches!(base.len(), 40 | 64) || !base.bytes().all(|b| b.is_ascii_hexdigit()) {
            bail!("Git context requires a full commit ID");
        }
        Ok(expression)
    }
    pub(crate) fn code_clean(&mut self) -> Result<bool> {
        match self.clean {
            Some(clean) => Ok(clean),
            None => Ok(working_code_paths(self.repo)?.is_empty()),
        }
    }
    pub fn commit_id(&mut self, commit: &str) -> Result<String> {
        let expression = self.pinned(commit)?;
        if let Some(oid) = self.cache.commit_ids.get(&expression) {
            return Ok(oid.clone());
        }
        let object = self
            .object(&expression)?
            .with_context(|| format!("Git commit missing: {expression}"))?;
        if object.kind != "commit" {
            bail!("expected Git commit object");
        }
        let tree = object
            .data
            .split(|byte| *byte == b'\n')
            .next()
            .and_then(|line| line.strip_prefix(b"tree "))
            .and_then(|tree| std::str::from_utf8(tree).ok())
            .context("Git commit has no tree")?
            .to_owned();
        self.cache.commit_trees.insert(object.oid.clone(), tree);
        self.cache.commits.insert(object.oid.clone(), object.data);
        self.cache.commit_ids.insert(expression, object.oid.clone());
        Ok(object.oid)
    }
    fn object(&mut self, expression: &str) -> Result<Option<Object>> {
        if self.reader.is_none() {
            self.reader = Some(ObjectReader::spawn(self.repo)?);
        }
        let result = self
            .reader
            .as_mut()
            .expect("object reader")
            .read(expression);
        if result.is_err() {
            // The batch stream may be out of step; start a new one next time.
            self.reader = None;
        }
        result
    }
    fn commit_tree(&mut self, commit: &str) -> Result<String> {
        let commit = self.commit_id(commit)?;
        self.cache
            .commit_trees
            .get(&commit)
            .cloned()
            .context("Git commit tree missing")
    }
    fn tree_entries(&mut self, tree: &str) -> Result<Rc<Vec<TreeEntry>>> {
        if let Some(entries) = self.cache.trees.get(tree) {
            return Ok(entries.clone());
        }
        let object = self
            .object(tree)?
            .with_context(|| format!("Git tree missing: {tree}"))?;
        if object.kind != "tree" {
            bail!("expected Git tree object");
        }
        let hash_len = tree.len() / 2;
        let mut entries = Vec::new();
        let mut rest = object.data.as_slice();
        while !rest.is_empty() {
            let space = rest
                .iter()
                .position(|byte| *byte == b' ')
                .context("invalid Git tree entry")?;
            let nul = rest
                .iter()
                .position(|byte| *byte == 0)
                .context("invalid Git tree entry")?;
            if nul < space || rest.len() < nul + 1 + hash_len {
                bail!("invalid Git tree entry");
            }
            let mut oid = String::with_capacity(hash_len * 2);
            for byte in &rest[nul + 1..nul + 1 + hash_len] {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                oid.push(HEX[usize::from(byte >> 4)] as char);
                oid.push(HEX[usize::from(byte & 0xf)] as char);
            }
            entries.push(TreeEntry {
                mode: std::str::from_utf8(&rest[..space])?.to_owned(),
                name: rest[space + 1..nul].to_vec(),
                oid,
            });
            rest = &rest[nul + 1 + hash_len..];
        }
        // Sorted by name, not Git's tree order, so lookups and diffs can merge.
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        let entries = Rc::new(entries);
        self.cache.trees.insert(tree.to_owned(), entries.clone());
        Ok(entries)
    }
    /// Paths that differ between two commits, as `git diff --name-status`
    /// lists them without rename detection: both sides of a rename appear.
    fn changed_paths(&mut self, before: &str, after: &str) -> Result<Vec<PathBuf>> {
        match self.cache.tree_diff_matches {
            Some(true) => self.tree_changed_paths(before, after),
            Some(false) => changed_paths(self.repo, before, after),
            None => {
                // `git diff` reports configuration errors and is the reference;
                // check the in-process diff against it once.
                let native = changed_paths(self.repo, before, after)?;
                let matches = self.tree_changed_paths(before, after).ok().as_ref() == Some(&native);
                self.cache.tree_diff_matches = Some(matches);
                Ok(native)
            }
        }
    }
    fn tree_changed_paths(&mut self, before: &str, after: &str) -> Result<Vec<PathBuf>> {
        let before = self.commit_tree(before)?;
        let after = self.commit_tree(after)?;
        let mut paths = Vec::new();
        self.diff_trees(Some(&before), Some(&after), "", &mut paths)?;
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
    fn diff_trees(
        &mut self,
        before: Option<&str>,
        after: Option<&str>,
        prefix: &str,
        paths: &mut Vec<PathBuf>,
    ) -> Result<()> {
        if before == after {
            return Ok(());
        }
        let left = match before {
            Some(tree) => self.tree_entries(tree)?,
            None => Rc::default(),
        };
        let right = match after {
            Some(tree) => self.tree_entries(tree)?,
            None => Rc::default(),
        };
        let (mut l, mut r) = (0, 0);
        while l < left.len() || r < right.len() {
            let order = match (left.get(l), right.get(r)) {
                (Some(old), Some(new)) => old.name.cmp(&new.name),
                (Some(_), None) => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Greater,
            };
            let (old, new) = match order {
                std::cmp::Ordering::Less => (left.get(l), None),
                std::cmp::Ordering::Greater => (None, right.get(r)),
                std::cmp::Ordering::Equal => (left.get(l), right.get(r)),
            };
            if old.is_some() {
                l += 1;
            }
            if new.is_some() {
                r += 1;
            }
            let name = old
                .or(new)
                .map(|entry| entry.name.as_slice())
                .unwrap_or_default();
            if let (Some(old), Some(new)) = (old, new) {
                if old.mode == new.mode && old.oid == new.oid {
                    continue;
                }
            }
            let path = format!("{prefix}{}", std::str::from_utf8(name)?);
            let tree = |entry: Option<&TreeEntry>| {
                entry
                    .filter(|entry| entry.mode == "40000")
                    .map(|entry| entry.oid.clone())
            };
            let (old_tree, new_tree) = (tree(old), tree(new));
            if old_tree.is_some() || new_tree.is_some() {
                self.diff_trees(
                    old_tree.as_deref(),
                    new_tree.as_deref(),
                    &format!("{path}/"),
                    paths,
                )?;
            }
            if old.is_some_and(|entry| entry.mode != "40000")
                || new.is_some_and(|entry| entry.mode != "40000")
            {
                paths.push(PathBuf::from(path));
            }
        }
        Ok(())
    }
    pub(crate) fn ancestor(&mut self, earlier: &str, later: &str) -> Result<bool> {
        let later = self.pinned(later)?;
        let earlier = self.pinned(earlier)?;
        let Ok(later) = self.commit_id(&later) else {
            return Ok(false);
        };
        let Ok(earlier) = self.commit_id(&earlier) else {
            return Ok(false);
        };
        let key = (earlier, later);
        if let Some(known) = self.cache.ancestors.get(&key) {
            return Ok(*known);
        }
        let asked = self
            .cache
            .ancestor_queries
            .entry(key.1.clone())
            .or_default();
        *asked += 1;
        let ancestor = if *asked > 3 {
            let set = match self.cache.ancestor_sets.get(&key.1) {
                Some(set) => set.clone(),
                None => {
                    let listed = String::from_utf8(git_output(self.repo, &["rev-list", &key.1])?)?;
                    let set = Rc::new(listed.lines().map(str::to_owned).collect::<HashSet<_>>());
                    self.cache.ancestor_sets.insert(key.1.clone(), set.clone());
                    set
                }
            };
            set.contains(&key.0)
        } else {
            git_output(self.repo, &["merge-base", "--is-ancestor", &key.0, &key.1]).is_ok()
        };
        self.cache.ancestors.insert(key, ancestor);
        Ok(ancestor)
    }
    pub(crate) fn first_parent(&mut self, commit: &str) -> Result<Option<String>> {
        let commit = self.commit_id(commit)?;
        if let Some(parent) = self.cache.first_parents.get(&commit) {
            return Ok(parent.clone());
        }
        let parents = String::from_utf8(git_output(
            self.repo,
            &["rev-list", "--parents", "-n", "1", &commit],
        )?)?;
        let mut parts = parents.split_whitespace();
        if parts.next() != Some(commit.as_str()) {
            bail!("invalid Git parent query");
        }
        let parent = parts.next().map(str::to_owned);
        self.cache.first_parents.insert(commit, parent.clone());
        Ok(parent)
    }
    /// Memoized [`merge_resolution`] for a full commit ID.
    pub(crate) fn merge_resolution(&mut self, commit: &str) -> Result<Option<MergeResolution>> {
        let commit = self.commit_id(commit)?;
        if let Some(resolution) = self.cache.merge_resolutions.get(&commit) {
            return Ok(resolution.clone());
        }
        let resolution = merge_resolution(self.repo, &commit)?;
        self.cache
            .merge_resolutions
            .insert(commit, resolution.clone());
        Ok(resolution)
    }
    /// Commits reachable from `tips` but not from `not`.
    pub(crate) fn only_in(&mut self, tips: &[String], not: &str) -> Result<HashSet<String>> {
        let mut args = vec!["rev-list"];
        args.extend(tips.iter().map(String::as_str));
        args.extend(["--not", not]);
        Ok(String::from_utf8(git_output(self.repo, &args)?)?
            .lines()
            .map(str::to_owned)
            .collect())
    }
    /// Output of a `git log` whose revisions are full commit IDs, so the
    /// same query in this read phase has the same answer.
    pub(crate) fn history(&mut self, args: &[&str]) -> Result<String> {
        let key = args.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
        if let Some(output) = self.cache.histories.get(&key) {
            return Ok(output.clone());
        }
        let output = String::from_utf8(git_output(self.repo, args)?)?;
        self.cache.histories.insert(key, output.clone());
        Ok(output)
    }
    /// Memoized answer for a pure comparison of two blobs at one path.
    pub(crate) fn status_only(
        &mut self,
        before: &str,
        after: &str,
        path: &Path,
        compare: impl FnOnce(&mut Self) -> Result<bool>,
    ) -> Result<bool> {
        let key = (before.to_owned(), after.to_owned(), path.to_path_buf());
        if let Some(known) = self.cache.status_only.get(&key) {
            return Ok(*known);
        }
        let answer = compare(self)?;
        self.cache.status_only.insert(key, answer);
        Ok(answer)
    }
    /// Fill the `history` cache for many queries, running them in parallel.
    /// Failures are left uncached so the later `history` call reports them.
    pub(crate) fn prefetch_histories(&mut self, queries: Vec<Vec<String>>) {
        let mut pending = queries
            .into_iter()
            .filter(|query| !self.cache.histories.contains_key(query))
            .collect::<Vec<_>>();
        pending.sort();
        pending.dedup();
        if pending.is_empty() {
            return;
        }
        let workers = std::thread::available_parallelism()
            .map_or(4, usize::from)
            .clamp(1, 8);
        let chunk = pending.len().div_ceil(workers);
        let repo = self.repo;
        let results = std::thread::scope(|scope| {
            let handles = pending
                .chunks(chunk)
                .map(|queries| {
                    scope.spawn(move || {
                        queries
                            .iter()
                            .filter_map(|query| {
                                let args = query.iter().map(String::as_str).collect::<Vec<_>>();
                                let output = git_output(repo, &args).ok()?;
                                Some((query.clone(), String::from_utf8(output).ok()?))
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect::<Vec<_>>();
            handles
                .into_iter()
                .flat_map(|handle| handle.join().unwrap_or_default())
                .collect::<Vec<_>>()
        });
        self.cache.histories.extend(results);
    }
    pub(crate) fn author_names(&mut self, commit: &str) -> Result<String> {
        let commit = self.commit_id(commit)?;
        if let Some(names) = self.cache.authors.get(&commit) {
            return Ok(names.clone());
        }
        let names = match self.plain_author(&commit)? {
            Some(names) => names,
            None => String::from_utf8(git_output(
                self.repo,
                &["show", "-s", "--format=%an%n%ae", &commit],
            )?)?,
        };
        self.cache.authors.insert(commit, names.clone());
        Ok(names)
    }
    /// `%an%n%ae` read from the commit object when `git show` would print it
    /// unchanged: a UTF-8 commit, UTF-8 log output and an ordinary identity.
    /// Anything else returns `None` so `git show` decides.
    fn plain_author(&mut self, commit: &str) -> Result<Option<String>> {
        if self.cache.utf8_log_output.is_none() {
            let encoding = command(self.repo)
                .args(["config", "--get", "i18n.logOutputEncoding"])
                .stdin(Stdio::null())
                .output()?;
            let encoding = String::from_utf8_lossy(&encoding.stdout)
                .trim()
                .to_lowercase();
            self.cache.utf8_log_output = Some(matches!(encoding.as_str(), "" | "utf-8" | "utf8"));
        }
        if self.cache.utf8_log_output != Some(true) {
            return Ok(None);
        }
        let Some(data) = self.cache.commits.get(commit) else {
            return Ok(None);
        };
        let Ok(text) = std::str::from_utf8(data) else {
            return Ok(None);
        };
        let headers = text.split("\n\n").next().unwrap_or("");
        let mut author = None;
        for line in headers.lines() {
            if line.starts_with("encoding ") || line.starts_with(' ') {
                return Ok(None);
            }
            if let Some(ident) = line.strip_prefix("author ") {
                if author.replace(ident).is_some() {
                    return Ok(None);
                }
            }
        }
        let Some(ident) = author else {
            return Ok(None);
        };
        let Some((name, rest)) = ident.split_once(" <") else {
            return Ok(None);
        };
        let Some((email, date)) = rest.split_once("> ") else {
            return Ok(None);
        };
        let ordinary = |part: &str| {
            !part.is_empty()
                && part.trim() == part
                && !part.contains(['<', '>'])
                && !part.chars().any(char::is_control)
        };
        let mut date = date.split(' ');
        let timestamp = date
            .next()
            .is_some_and(|t| !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit()));
        let zone = date.next().is_some_and(|z| {
            z.len() == 5 && z.starts_with(['+', '-']) && z[1..].bytes().all(|b| b.is_ascii_digit())
        });
        if !(ordinary(name) && ordinary(email) && timestamp && zone && date.next().is_none()) {
            return Ok(None);
        }
        Ok(Some(format!("{name}\n{email}\n")))
    }
    pub(crate) fn blob(&mut self, commit: &str, path: &Path) -> Result<String> {
        let commit = self.commit_id(commit)?;
        let key = (commit.clone(), path.to_path_buf());
        if let Some(text) = self.cache.blobs.get(&key) {
            return Ok(text.clone());
        }
        // Read by the entry's object ID: a path may contain line breaks,
        // which a batch expression cannot.
        let (_, oid) = self
            .entry(&commit, path)?
            .with_context(|| format!("Git record object missing: {commit}:{}", path.display()))?;
        let object = self
            .object(&oid)?
            .with_context(|| format!("Git record object missing: {oid}"))?;
        if object.kind != "blob" {
            bail!("expected Git record blob");
        }
        let text = String::from_utf8(object.data).context("Git record blob is not UTF-8")?;
        self.cache.blobs.insert(key, text.clone());
        Ok(text)
    }
    pub(crate) fn entry(&mut self, commit: &str, path: &Path) -> Result<Option<(String, String)>> {
        let commit = self.commit_id(commit)?;
        let key = (commit.clone(), path.to_path_buf());
        if let Some(entry) = self.cache.entries.get(&key) {
            return Ok(entry.clone());
        }
        let entry = self.uncached_entry(&commit, path)?;
        self.cache.entries.insert(key, entry.clone());
        Ok(entry)
    }
    fn uncached_entry(&mut self, commit: &str, path: &Path) -> Result<Option<(String, String)>> {
        let name = path.to_str().context("record path is not UTF-8")?;
        let mut tree = self.commit_tree(commit)?;
        let components = name.split('/').collect::<Vec<_>>();
        for (position, component) in components.iter().enumerate() {
            let entries = self.tree_entries(&tree)?;
            let Ok(found) =
                entries.binary_search_by(|entry| entry.name.as_slice().cmp(component.as_bytes()))
            else {
                return Ok(None);
            };
            let found = &entries[found];
            if position + 1 == components.len() {
                if matches!(found.mode.as_str(), "40000" | "160000") {
                    bail!("expected blob entry for {}", path.display());
                }
                return Ok(Some((found.mode.clone(), found.oid.clone())));
            }
            if found.mode != "40000" {
                return Ok(None);
            }
            tree = found.oid.clone();
        }
        Ok(None)
    }
    /// Inspect every introduced commit, including side-branch edit/revert history.
    /// Merges prefer a parent containing the tested pin to avoid reintroducing old work.
    pub(crate) fn edges(&mut self, pin: &str, end: &str) -> Result<Rc<Vec<CommitEdge>>> {
        let end = self.commit_id(end)?;
        let pin = self.commit_id(pin)?;
        let key = (pin.clone(), end.clone());
        if let Some(edges) = self.cache.edges.get(&key) {
            return Ok(edges.clone());
        }
        let edges = Rc::new(self.uncached_edges(&pin, &end)?);
        self.cache.edges.insert(key, edges.clone());
        Ok(edges)
    }
    fn uncached_edges(&mut self, pin: &str, end: &str) -> Result<Vec<CommitEdge>> {
        git_output(self.repo, &["merge-base", "--is-ancestor", pin, end])?;
        let range = format!("{pin}..{end}");
        let commits =
            String::from_utf8(git_output(self.repo, &["rev-list", "--parents", &range])?)?;
        let mut tested_context = ancestry_path(&commits, pin)?;
        tested_context.insert(pin);
        let mut edges = Vec::new();
        for line in commits.lines() {
            let mut parts = line.split_whitespace();
            let commit = parts.next().context("invalid Git parent query")?;
            let parents = parts.collect::<Vec<_>>();
            if let Some(parent) = parents
                .iter()
                .find(|parent| tested_context.contains(**parent))
                .or_else(|| parents.first())
            {
                let key = (parent.to_string(), commit.to_owned());
                let paths = match self.cache.changed.get(&key) {
                    Some(paths) => paths.clone(),
                    None => {
                        let paths = self.changed_paths(parent, commit)?;
                        self.cache.changed.insert(key, paths.clone());
                        paths
                    }
                };
                edges.push(CommitEdge {
                    parent: parent.to_string(),
                    commit: commit.to_owned(),
                    paths,
                    sides: parents
                        .iter()
                        .filter(|other| *other != parent)
                        .map(|other| other.to_string())
                        .collect(),
                });
            }
        }
        Ok(edges)
    }
}

/// The commits of `rev-list --parents pin..end` output that descend from
/// `pin`, which is what `rev-list --ancestry-path pin..end` lists, computed
/// from the same output instead of a second walk (atelier-v5tt).
fn ancestry_path<'a>(commits: &'a str, pin: &str) -> Result<HashSet<&'a str>> {
    let mut parents = HashMap::new();
    for line in commits.lines() {
        let mut parts = line.split_whitespace();
        let commit = parts.next().context("invalid Git parent query")?;
        parents.insert(commit, parts.collect::<Vec<_>>());
    }
    // Iterative post-order walk: a commit descends from the pin when one of
    // its parents is the pin or, inside the range, descends from it.
    let mut descends: HashMap<&str, bool> = HashMap::new();
    for &start in parents.keys() {
        let mut stack = vec![(start, false)];
        while let Some((commit, expanded)) = stack.pop() {
            if descends.contains_key(commit) {
                continue;
            }
            let commit_parents = &parents[commit];
            if expanded {
                let on_path = commit_parents
                    .iter()
                    .any(|parent| *parent == pin || descends.get(parent).copied().unwrap_or(false));
                descends.insert(commit, on_path);
                continue;
            }
            stack.push((commit, true));
            for parent in commit_parents {
                if parents.contains_key(parent) && !descends.contains_key(parent) {
                    stack.push((parent, false));
                }
            }
        }
    }
    Ok(descends
        .into_iter()
        .filter_map(|(commit, on_path)| on_path.then_some(commit))
        .collect())
}

/// Pinned evidence and review history always describe original Git objects.
pub fn command(repo: &Path) -> Command {
    let mut command = Command::new("git");
    command.current_dir(repo).arg("--no-replace-objects");
    command
}
pub fn git_output(repo: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let output = command(repo).args(args).stdin(Stdio::null()).output()?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(output.stdout)
}
pub fn head_commit(repo: &Path) -> Result<String> {
    Ok(
        String::from_utf8(git_output(repo, &["rev-parse", "HEAD"])?)?
            .trim()
            .to_owned(),
    )
}
/// Whether `commit` merges two parents into exactly the tree Git's own merge
/// of them produces: no conflicts and no edits made while merging, so it
/// introduces nothing that is not on one side or the other (atelier-n7g1).
pub fn clean_merge(repo: &Path, commit: &str) -> Result<bool> {
    Ok(merge_resolution(repo, commit)?.is_some_and(|merge| merge.conflicted.is_empty()))
}

/// A two-parent merge that matches Git's own merge of its parents except in
/// records whose conflicts were resolved by hand (atelier-1isk).
#[derive(Clone)]
pub struct MergeResolution {
    /// The record paths Git could not merge; the merge commit may differ from
    /// Git's result only here.
    pub conflicted: Vec<PathBuf>,
    /// The parents' merge base, set when there are conflicts.
    pub merge_base: Option<String>,
}

/// `None` when `commit` is not a two-parent merge, a non-record path
/// conflicted, or the commit differs from Git's merge outside the conflicts.
pub fn merge_resolution(repo: &Path, commit: &str) -> Result<Option<MergeResolution>> {
    let line = String::from_utf8(git_output(
        repo,
        &["rev-list", "--parents", "-n", "1", commit],
    )?)?;
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let [_, first, second] = parts.as_slice() else {
        return Ok(None);
    };
    // Exit status 1 reports conflicts; the tree and conflicted paths still
    // follow, NUL-terminated.
    let output = command(repo)
        .args([
            "merge-tree",
            "--write-tree",
            "--no-messages",
            "--name-only",
            "-z",
            first,
            second,
        ])
        .stderr(Stdio::null())
        .output()?;
    if !matches!(output.status.code(), Some(0 | 1)) {
        return Ok(None);
    }
    let paths = |bytes: &[u8]| {
        bytes
            .split(|byte| *byte == 0)
            .filter(|field| !field.is_empty())
            .map(|path| PathBuf::from(String::from_utf8_lossy(path).into_owned()))
            .collect::<Vec<_>>()
    };
    let Some(end) = output.stdout.iter().position(|byte| *byte == 0) else {
        return Ok(None);
    };
    let merged = std::str::from_utf8(&output.stdout[..end])?.to_owned();
    let mut conflicted = paths(&output.stdout[end..]);
    conflicted.sort();
    conflicted.dedup();
    if conflicted.iter().any(|path| !is_record_path(path)) {
        return Ok(None);
    }
    let tree = String::from_utf8(git_output(
        repo,
        &["rev-parse", &format!("{commit}^{{tree}}")],
    )?)?;
    let differing = paths(&git_output(
        repo,
        &[
            "diff-tree",
            "-r",
            "--no-renames",
            "--name-only",
            "-z",
            &merged,
            tree.trim(),
        ],
    )?);
    if differing.iter().any(|path| !conflicted.contains(path)) {
        return Ok(None);
    }
    let merge_base = if conflicted.is_empty() {
        None
    } else {
        let bases = String::from_utf8(git_output(repo, &["merge-base", "--all", first, second])?)?;
        let bases = bases.lines().collect::<Vec<_>>();
        let [base] = bases.as_slice() else {
            return Ok(None);
        };
        Some((*base).to_owned())
    };
    Ok(Some(MergeResolution {
        conflicted,
        merge_base,
    }))
}

pub fn blob(repo: &Path, commit: &str, path: &Path) -> Result<String> {
    ReadContext::new(repo).blob(commit, path)
}
fn status_paths<'a>(
    status: &[u8],
    parts: &mut impl Iterator<Item = &'a [u8]>,
    paths: &mut Vec<PathBuf>,
) -> Result<()> {
    let (&kind, score) = status.split_first().context("missing Git diff status")?;
    if !b"ACDMRTUXB".contains(&kind) || !score.iter().all(u8::is_ascii_digit) {
        bail!("invalid Git diff status");
    }
    // Rename/copy sources are affected paths too: deletions must stay visible.
    for _ in 0..if matches!(kind, b'R' | b'C') { 2 } else { 1 } {
        let path = parts
            .next()
            .filter(|path| !path.is_empty())
            .context("missing Git diff path")?;
        paths.push(PathBuf::from(std::str::from_utf8(path)?));
    }
    Ok(())
}
pub fn changed_paths(repo: &Path, before: &str, after: &str) -> Result<Vec<PathBuf>> {
    let output = git_output(repo, &["diff", "--name-status", "-z", before, after, "--"])?;
    let mut parts = output.split(|byte| *byte == 0);
    let mut paths = Vec::new();
    while let Some(status) = parts.next().filter(|status| !status.is_empty()) {
        status_paths(status, &mut parts, &mut paths)?;
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
}

pub fn working_paths(repo: &Path) -> Result<Vec<PathBuf>> {
    let mut changed = working_status_paths(repo)?;
    changed.extend(crate::committed_snapshot::flagged_modified_paths(
        repo, "HEAD",
    )?);
    changed.sort();
    changed.dedup();
    Ok(changed)
}
pub(crate) fn working_code_paths(repo: &Path) -> Result<Vec<PathBuf>> {
    Ok(working_paths(repo)?
        .into_iter()
        .filter(|path| !is_record_path(path))
        .collect())
}
pub fn working_status_paths(repo: &Path) -> Result<Vec<PathBuf>> {
    let status = git_output(
        repo,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )?;
    let mut records = status
        .split(|byte| *byte == 0)
        .filter(|part| !part.is_empty());
    let mut changed = Vec::new();
    while let Some(record) = records.next() {
        if record.len() < 4 || record[2] != b' ' {
            bail!("invalid Git status record");
        }
        changed.push(PathBuf::from(std::str::from_utf8(&record[3..])?));
        if record[..2].iter().any(|byte| matches!(byte, b'R' | b'C')) {
            changed.push(PathBuf::from(std::str::from_utf8(
                records.next().context("missing Git rename source")?,
            )?));
        }
    }
    Ok(changed)
}
/// Record-only paths; policy and project config are intentionally excluded.
pub fn is_record_path(path: &Path) -> bool {
    let mut parts = path.components();
    parts
        .next()
        .is_some_and(|part| part.as_os_str() == ".atelier")
        && parts.next().is_some_and(|part| {
            ["issues", "evidence", "reviews"]
                .iter()
                .any(|dir| part.as_os_str() == *dir)
        })
        && path.extension().is_some_and(|extension| extension == "md")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io::Write};

    fn git_input(repo: &Path, args: &[&str], input: &[u8]) -> Result<Vec<u8>> {
        let mut child = command(repo)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let mut stdin = child.stdin.take().context("Git stdin missing")?;
        let output = std::thread::scope(|scope| -> Result<_> {
            let writer = scope.spawn(move || stdin.write_all(input));
            let output = child.wait_with_output()?;
            writer.join().expect("Git input thread panicked")?;
            Ok(output)
        })?;
        if !output.status.success() {
            bail!(
                "git test input failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(output.stdout)
    }
    fn commit_edges(repo: &Path, pin: &str) -> Result<Vec<CommitEdge>> {
        ReadContext::new(repo)
            .edges(pin, "HEAD")
            .map(|edges| edges.to_vec())
    }

    fn repo(format: &str) -> tempfile::TempDir {
        let repo = tempfile::tempdir().unwrap();
        git_output(
            repo.path(),
            &["init", "-q", &format!("--object-format={format}")],
        )
        .unwrap();
        git_output(repo.path(), &["config", "user.name", "Author"]).unwrap();
        git_output(
            repo.path(),
            &["config", "user.email", "author@example.test"],
        )
        .unwrap();
        repo
    }
    fn commit(repo: &Path, message: &str) -> String {
        git_output(repo, &["add", "."]).unwrap();
        git_output(repo, &["commit", "--allow-empty", "-qm", message]).unwrap();
        head_commit(repo).unwrap()
    }
    #[test]
    fn ancestry_path_matches_git_on_branchy_history() {
        let repo = repo("sha1");
        let path = repo.path();
        let mut commits = vec![commit(path, "root")];
        // Side branches, merges of merges and a criss-cross, so pins land on
        // both sides of every merge.
        for round in 0..3 {
            let base = commits.last().unwrap().clone();
            git_output(path, &["checkout", "-qb", &format!("side{round}"), &base]).unwrap();
            fs::write(path.join(format!("side{round}")), "side").unwrap();
            commits.push(commit(path, "side work"));
            commits.push(commit(path, "more side work"));
            git_output(path, &["checkout", "-q", "--detach", &base]).unwrap();
            fs::write(path.join(format!("main{round}")), "main").unwrap();
            commits.push(commit(path, "main work"));
            git_output(
                path,
                &[
                    "merge",
                    "-q",
                    "--no-ff",
                    "-m",
                    "merge",
                    &format!("side{round}"),
                ],
            )
            .unwrap();
            commits.push(head_commit(path).unwrap());
            git_output(path, &["checkout", "-q", &format!("side{round}")]).unwrap();
            git_output(
                path,
                &[
                    "merge",
                    "-q",
                    "--no-ff",
                    "-m",
                    "criss",
                    &commits[commits.len() - 2],
                ],
            )
            .unwrap();
            commits.push(head_commit(path).unwrap());
            git_output(
                path,
                &["checkout", "-q", "--detach", &commits[commits.len() - 2]],
            )
            .unwrap();
            git_output(
                path,
                &[
                    "merge",
                    "-q",
                    "--no-ff",
                    "-m",
                    "cross",
                    &format!("side{round}"),
                ],
            )
            .unwrap();
            commits.push(head_commit(path).unwrap());
        }
        let end = commits.last().unwrap().clone();
        for pin in &commits {
            if git_output(path, &["merge-base", "--is-ancestor", pin, &end]).is_err() {
                continue;
            }
            let range = format!("{pin}..{end}");
            let listed =
                String::from_utf8(git_output(path, &["rev-list", "--parents", &range]).unwrap())
                    .unwrap();
            let native = ancestry_path(&listed, pin).unwrap();
            let git = String::from_utf8(
                git_output(path, &["rev-list", "--ancestry-path", &range]).unwrap(),
            )
            .unwrap();
            assert_eq!(native, git.lines().collect::<HashSet<_>>(), "pin {pin}");
        }
    }
    #[test]
    fn native_reads_keep_exact_ranges_and_reject_bad_pins() {
        for format in ["sha1", "sha256"] {
            let repo = repo(format);
            fs::write(repo.path().join("first"), "pin").unwrap();
            let pin = commit(repo.path(), "pin");
            fs::write(repo.path().join("first"), "second").unwrap();
            let middle = commit(repo.path(), "middle");
            fs::write(repo.path().join("last"), "last").unwrap();
            let head = commit(repo.path(), "head");
            git_output(repo.path(), &["checkout", "-qb", "sibling", &pin]).unwrap();
            let sibling = commit(repo.path(), "outside endpoint");
            git_output(repo.path(), &["checkout", "-q", "master"]).unwrap();
            let mut read = ReadContext::new(repo.path());
            for (start, end, expected) in [
                (&pin, &head, vec![head.as_str(), middle.as_str()]),
                (&middle, &head, vec![head.as_str()]),
                (&pin, &middle, vec![middle.as_str()]),
            ] {
                let actual = read.edges(start, end).unwrap();
                assert_eq!(
                    actual
                        .iter()
                        .map(|edge| edge.commit.as_str())
                        .collect::<Vec<_>>(),
                    expected
                );
            }
            assert!(read.edges(&sibling, &head).is_err());
            assert!(read.edges(&"0".repeat(pin.len()), &head).is_err());
        }
    }

    #[test]
    fn native_author_fields_preserve_show_encoding_and_unusual_identities() {
        let repo = repo("sha1");
        let pin = commit(repo.path(), "pin");
        for (identity, encoding) in [
            (
                b"Ren\xc3\xa9 \xe5\x90\x8d <name+tag@example.test>".as_slice(),
                "UTF-8",
            ),
            (b" spaces  <addr@example.test>".as_slice(), "UTF-8"),
            (b"Ren\xe9 <latin@example.test>".as_slice(), "ISO-8859-1"),
            (b"line\nname <mail@example.test>".as_slice(), "UTF-8"),
            (b"null\0name <mail@example.test>".as_slice(), "UTF-8"),
        ] {
            let tree = String::from_utf8(
                git_output(repo.path(), &["rev-parse", &format!("{pin}^{{tree}}")]).unwrap(),
            )
            .unwrap();
            let mut object = format!("tree {}\nparent {pin}\nauthor ", tree.trim()).into_bytes();
            object.extend_from_slice(identity);
            object.extend_from_slice(format!(" 1600000000 +0000\ncommitter Author <a@example.test> 1600000000 +0000\nencoding {encoding}\n\nmessage\n").as_bytes());
            let oid = String::from_utf8(
                git_input(
                    repo.path(),
                    &[
                        "hash-object",
                        "--literally",
                        "-w",
                        "-t",
                        "commit",
                        "--stdin",
                    ],
                    &object,
                )
                .unwrap(),
            )
            .unwrap();
            let oid = oid.trim();
            git_output(repo.path(), &["update-ref", "refs/heads/master", oid]).unwrap();
            for output_encoding in ["UTF-8", "ISO-8859-1"] {
                git_output(
                    repo.path(),
                    &["config", "i18n.logOutputEncoding", output_encoding],
                )
                .unwrap();
                let expected =
                    git_output(repo.path(), &["show", "-s", "--format=%an%n%ae", oid]).unwrap();
                let mut read = ReadContext::new(repo.path());
                let expected = String::from_utf8(expected);
                let actual = read.author_names(oid);
                assert_eq!(actual.is_ok(), expected.is_ok());
                if let Ok(expected) = expected {
                    assert_eq!(actual.unwrap(), expected);
                }
            }
        }
    }

    #[test]
    fn captured_first_parent_matches_native_roots_merges_shallow_and_grafts() {
        let repo = repo("sha1");
        let root = commit(repo.path(), "root");
        let middle = commit(repo.path(), "middle");
        git_output(repo.path(), &["checkout", "-qb", "side", &root]).unwrap();
        commit(repo.path(), "side");
        git_output(repo.path(), &["checkout", "-q", "master"]).unwrap();
        git_output(repo.path(), &["merge", "--no-ff", "-qm", "merge", "side"]).unwrap();
        let head = head_commit(repo.path()).unwrap();
        let compare = |path: &Path, ids: &[String]| {
            for oid in ids {
                let expected = git_output(
                    path,
                    &["rev-parse", "--verify", &format!("{oid}^{{commit}}^1")],
                )
                .ok()
                .map(|b| String::from_utf8(b).unwrap().trim().to_owned());
                assert_eq!(ReadContext::new(path).first_parent(oid).unwrap(), expected);
            }
        };
        compare(repo.path(), &[root.clone(), middle.clone(), head.clone()]);
        fs::write(repo.path().join(".git/shallow"), format!("{head}\n")).unwrap();
        assert!(git_output(repo.path(), &["cat-file", "-e", &middle]).is_ok());
        compare(repo.path(), std::slice::from_ref(&head));
        let clone = tempfile::tempdir().unwrap();
        git_output(
            repo.path(),
            &[
                "clone",
                "-q",
                "--depth=1",
                &format!("file://{}", repo.path().display()),
                clone.path().to_str().unwrap(),
            ],
        )
        .unwrap();
        assert!(git_output(clone.path(), &["cat-file", "-e", &middle]).is_err());
        compare(clone.path(), std::slice::from_ref(&head));
        fs::remove_file(repo.path().join(".git/shallow")).unwrap();
        fs::write(repo.path().join(".git/info/grafts"), format!("{middle}\n")).unwrap();
        compare(repo.path(), &[middle, head]);
    }

    #[test]
    fn native_edges_match_git_diff_for_rename_configuration_and_weird_paths() {
        let repo = repo("sha1");
        fs::write(
            repo.path().join("source"),
            "same repeated content\n".repeat(100),
        )
        .unwrap();
        let pin = commit(repo.path(), "pin");
        fs::rename(
            repo.path().join("source"),
            repo.path().join("renamed\n名 file"),
        )
        .unwrap();
        fs::write(repo.path().join("\nleading"), "new").unwrap();
        fs::write(repo.path().join(&pin), "hash-looking filename").unwrap();
        commit(repo.path(), "rename");
        commit(repo.path(), "empty");
        fs::copy(
            repo.path().join("renamed\n名 file"),
            repo.path().join("copy"),
        )
        .unwrap();
        fs::write(
            repo.path().join("renamed\n名 file"),
            "same repeated content\n".repeat(101),
        )
        .unwrap();
        commit(repo.path(), "copy and modify");
        for value in [
            None,
            Some("false"),
            Some("true"),
            Some("copies"),
            Some("on"),
            Some("off"),
            Some("0"),
            Some("2"),
            Some("0x0"),
            Some("0x1"),
            Some("0k"),
            Some("1k"),
            Some(""),
            Some("COPIES"),
            Some("cOpIeS"),
            Some("TRUE"),
            Some("No"),
        ] {
            if let Some(value) = value {
                git_output(repo.path(), &["config", "diff.renames", value]).unwrap();
            } else {
                let _ = git_output(repo.path(), &["config", "--unset", "diff.renames"]);
            }
            for limit in ["0", "1"] {
                git_output(repo.path(), &["config", "diff.renameLimit", limit]).unwrap();
                let edges = commit_edges(repo.path(), &pin).unwrap();
                assert_eq!(edges.len(), 3);
                for edge in edges {
                    if edge.parent == pin {
                        assert!(edge.paths.contains(&PathBuf::from("source")));
                        assert!(edge.paths.contains(&PathBuf::from("renamed\n名 file")));
                    }
                    assert_eq!(
                        edge.paths,
                        changed_paths(repo.path(), &edge.parent, &edge.commit).unwrap(),
                        "configuration {value:?}, limit {limit}"
                    );
                }
            }
        }
    }
    #[test]
    fn tree_diff_matches_git_diff_for_nested_type_and_mode_changes() {
        for format in ["sha1", "sha256"] {
            let repo = repo(format);
            let root = repo.path();
            fs::create_dir_all(root.join("a/b")).unwrap();
            fs::write(root.join("a/b/file"), "one").unwrap();
            fs::write(root.join("a/keep"), "keep").unwrap();
            fs::write(root.join("swap"), "file").unwrap();
            let mut commits = vec![commit(root, "base")];
            fs::write(root.join("a/b/file"), "two").unwrap();
            fs::create_dir_all(root.join("new/deep")).unwrap();
            fs::write(root.join("new/deep/x"), "x").unwrap();
            commits.push(commit(root, "nested edit and add"));
            fs::remove_dir_all(root.join("a/b")).unwrap();
            fs::remove_file(root.join("swap")).unwrap();
            fs::create_dir(root.join("swap")).unwrap();
            fs::write(root.join("swap/inner"), "dir").unwrap();
            commits.push(commit(root, "delete dir, file becomes dir"));
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(root.join("a/keep"), fs::Permissions::from_mode(0o755)).unwrap();
            std::os::unix::fs::symlink("a/keep", root.join("link")).unwrap();
            commits.push(commit(root, "mode change and symlink"));
            fs::remove_dir_all(root.join("swap")).unwrap();
            fs::write(root.join("swap"), "file again").unwrap();
            commits.push(commit(root, "dir becomes file"));
            let mut read = ReadContext::new(root);
            for (before, after) in commits
                .iter()
                .zip(&commits[1..])
                .chain([(&commits[0], &commits[4])])
            {
                assert_eq!(
                    read.tree_changed_paths(before, after).unwrap(),
                    changed_paths(root, before, after).unwrap(),
                    "{format} {before}..{after}"
                );
            }
        }
    }
    #[test]
    fn rename_configuration_preserves_git_string_errors_and_valueless_true() {
        let repo = repo("sha1");
        fs::write(repo.path().join("source"), "same content").unwrap();
        let pin = commit(repo.path(), "pin");
        fs::rename(repo.path().join("source"), repo.path().join("renamed")).unwrap();
        let end = commit(repo.path(), "rename");
        for value in [" true ", "true\n", " copies ", "bad", "1junk"] {
            git_output(repo.path(), &["config", "diff.renames", value]).unwrap();
            assert!(
                changed_paths(repo.path(), &pin, &end).is_err(),
                "Git accepted {value:?}"
            );
            assert!(
                commit_edges(repo.path(), &pin).is_err(),
                "batch accepted {value:?}"
            );
        }
        let _ = git_output(repo.path(), &["config", "--unset", "diff.renames"]);
        let config = repo.path().join(".git/config");
        let mut text = fs::read_to_string(&config).unwrap();
        text.push_str("\n[diff]\n\trenames\n");
        fs::write(config, text).unwrap();
        let edge = commit_edges(repo.path(), &pin).unwrap().remove(0);
        assert_eq!(edge.paths, changed_paths(repo.path(), &pin, &end).unwrap());
    }
    #[test]
    fn native_edges_preserve_tested_merge_parent_and_side_branch_reverts() {
        let repo = repo("sha1");
        fs::write(repo.path().join("base"), "base").unwrap();
        let base = commit(repo.path(), "base");
        git_output(repo.path(), &["checkout", "-qb", "side"]).unwrap();
        fs::write(repo.path().join("base"), "changed").unwrap();
        commit(repo.path(), "side change");
        fs::write(repo.path().join("base"), "base").unwrap();
        let side_pin = commit(repo.path(), "side revert");
        git_output(repo.path(), &["checkout", "-qb", "tested", &base]).unwrap();
        fs::write(repo.path().join("proof"), "tested").unwrap();
        let pin = commit(repo.path(), "tested pin");
        git_output(repo.path(), &["merge", "--no-ff", "-qm", "merge", "side"]).unwrap();
        let merge = head_commit(repo.path()).unwrap();
        let mut union = ReadContext::new(repo.path());
        for (start, expected_parent) in [(&pin, &pin), (&side_pin, &side_pin)] {
            let selected = union.edges(start, &merge).unwrap();
            assert_eq!(
                &selected
                    .iter()
                    .find(|edge| edge.commit == merge)
                    .unwrap()
                    .parent,
                expected_parent
            );
            for edge in selected.iter() {
                assert_eq!(
                    edge.paths,
                    changed_paths(repo.path(), &edge.parent, &edge.commit).unwrap()
                );
            }
        }
        let edges = commit_edges(repo.path(), &pin).unwrap();
        assert_eq!(edges.len(), 3);
        assert_eq!(
            edges
                .iter()
                .find(|edge| edge.commit == merge)
                .unwrap()
                .parent,
            pin
        );
        assert!(edges
            .iter()
            .any(|edge| edge.paths.contains(&PathBuf::from("base"))));
        for edge in edges {
            assert_eq!(
                edge.paths,
                changed_paths(repo.path(), &edge.parent, &edge.commit).unwrap()
            );
        }
    }
    #[test]
    fn native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats() {
        for format in ["sha1", "sha256"] {
            let repo = repo(format);
            fs::create_dir_all(repo.path().join(".atelier/reviews")).unwrap();
            let path = Path::new(".atelier/reviews/record\n名.md");
            fs::write(repo.path().join(path), "old").unwrap();
            let first = commit(repo.path(), "first");
            let mut read = ReadContext::new(repo.path());
            assert_eq!(read.blob("HEAD", path).unwrap(), "old");
            assert_eq!(read.entry("HEAD", path).unwrap().unwrap().0, "100644");
            assert!(read
                .entry("HEAD", Path::new(".atelier/reviews/missing"))
                .unwrap()
                .is_none());
            assert!(read
                .blob("HEAD", Path::new(".atelier/reviews/missing"))
                .is_err());
            fs::write(repo.path().join(path), "new").unwrap();
            commit(repo.path(), "second");
            assert_eq!(read.blob("HEAD", path).unwrap(), "old");
            assert_eq!(
                ReadContext::new(repo.path()).blob("HEAD", path).unwrap(),
                "new"
            );
            assert!(read.blob(&format!("{first}:invalid"), path).is_err());
            #[cfg(unix)]
            {
                use std::os::unix::fs::{symlink, PermissionsExt};
                fs::set_permissions(repo.path().join(path), fs::Permissions::from_mode(0o755))
                    .unwrap();
                symlink("record\n名.md", repo.path().join(".atelier/reviews/link")).unwrap();
                commit(repo.path(), "modes");
                let mut modes = ReadContext::new(repo.path());
                assert_eq!(modes.entry("HEAD", path).unwrap().unwrap().0, "100755");
                assert_eq!(
                    modes
                        .entry("HEAD", Path::new(".atelier/reviews/link"))
                        .unwrap()
                        .unwrap()
                        .0,
                    "120000"
                );
                assert!(modes.entry("HEAD", Path::new(".atelier/reviews")).is_err());
            }
        }
    }
    #[test]
    fn native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8() {
        for format in ["sha1", "sha256"] {
            let repo = repo(format);
            let name = "folder\n名/binary\n名";
            let bytes = b"first\n\0second\nlast";
            fs::create_dir(repo.path().join("folder\n名")).unwrap();
            fs::write(repo.path().join(name), bytes).unwrap();
            fs::write(repo.path().join("folder\n名/text\n名"), "text\n").unwrap();
            let pin = commit(repo.path(), "binary");
            let mut read = ReadContext::new(repo.path());
            assert!(read
                .entry(&pin, Path::new("folder\n名/missing\npath"))
                .unwrap()
                .is_none());
            assert_eq!(
                git_output(repo.path(), &["cat-file", "blob", &format!("{pin}:{name}")]).unwrap(),
                bytes
            );
            // NUL bytes are valid UTF-8; invalid UTF-8 is rejected explicitly.
            assert_eq!(read.blob(&pin, Path::new(name)).unwrap().as_bytes(), bytes);
            assert_eq!(
                read.blob(&pin, Path::new("folder\n名/text\n名")).unwrap(),
                "text\n"
            );
            assert!(read.blob(&pin, Path::new("other\nmissing")).is_err());
            fs::write(repo.path().join(name), [0xff]).unwrap();
            commit(repo.path(), "invalid UTF-8");
            assert!(ReadContext::new(repo.path())
                .blob("HEAD", Path::new(name))
                .is_err());
        }
    }
    #[test]
    fn replacement_refs_cannot_reinterpret_pinned_objects_or_history() {
        let repo = repo("sha1");
        fs::write(repo.path().join("code"), "original").unwrap();
        let pin = commit(repo.path(), "original");
        fs::write(repo.path().join("code"), "changed").unwrap();
        let end = commit(repo.path(), "changed");
        git_output(repo.path(), &["replace", &pin, &end]).unwrap();
        let interpreted = Command::new("git")
            .current_dir(repo.path())
            .args(["show", &format!("{pin}:code")])
            .output()
            .unwrap();
        assert_eq!(interpreted.stdout, b"changed");
        assert_eq!(
            blob(repo.path(), &pin, Path::new("code")).unwrap(),
            "original"
        );
        let mut read = ReadContext::new(repo.path());
        assert_eq!(read.blob(&pin, Path::new("code")).unwrap(), "original");
        assert!(read
            .edges(&pin, &end)
            .unwrap()
            .iter()
            .any(|edge| edge.paths.contains(&PathBuf::from("code"))));
    }
    #[test]
    fn range_boundaries_reject_wrong_missing_and_nonancestor_pins() {
        let repo = repo("sha1");
        let base = commit(repo.path(), "base");
        let first_branch = String::from_utf8(
            git_output(repo.path(), &["symbolic-ref", "--short", "HEAD"]).unwrap(),
        )
        .unwrap();
        commit(repo.path(), "first child");
        let first = head_commit(repo.path()).unwrap();
        let mut read = ReadContext::new(repo.path());
        assert!(read.edges(&first, &first).unwrap().is_empty());
        assert_eq!(read.edges(&base.to_uppercase(), "HEAD").unwrap().len(), 1);
        assert!(read.edges(&"0".repeat(40), "HEAD").is_err());
        let tree =
            String::from_utf8(git_output(repo.path(), &["rev-parse", "HEAD^{tree}"]).unwrap())
                .unwrap();
        assert!(read.edges(tree.trim(), "HEAD").is_err());
        assert!(read.edges(tree.trim(), tree.trim()).is_err());
        assert!(read.edges(&"0".repeat(40), &"0".repeat(40)).is_err());
        git_output(repo.path(), &["tag", "-am", "annotated", "proof", &base]).unwrap();
        let tag =
            String::from_utf8(git_output(repo.path(), &["rev-parse", "refs/tags/proof"]).unwrap())
                .unwrap();
        assert!(read.edges(tag.trim(), tag.trim()).is_err());
        assert_eq!(
            read.commit_id(&format!("{}^{{commit}}", tag.trim()))
                .unwrap(),
            base
        );
        git_output(repo.path(), &["checkout", "-qb", "fork", &base]).unwrap();
        commit(repo.path(), "fork child");
        assert!(ReadContext::new(repo.path()).edges(&first, "HEAD").is_err());
        git_output(repo.path(), &["checkout", "-q", first_branch.trim()]).unwrap();
        assert!(ReadContext::new(repo.path()).edges(&base, "HEAD").is_ok());
    }
    #[test]
    fn working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints() {
        let repo = repo("sha1");
        fs::write(repo.path().join("code.rs"), "code").unwrap();
        fs::write(repo.path().join("tracked"), "original").unwrap();
        fs::write(repo.path().join(".gitignore"), "ignored\n").unwrap();
        commit(repo.path(), "base");
        fs::create_dir_all(repo.path().join(".atelier/issues")).unwrap();
        git_output(repo.path(), &["mv", "code.rs", ".atelier/issues/code.md"]).unwrap();
        fs::write(repo.path().join("tracked"), "staged").unwrap();
        git_output(repo.path(), &["add", "tracked"]).unwrap();
        fs::write(repo.path().join("tracked"), "unstaged").unwrap();
        fs::write(repo.path().join("untracked\n名"), "new").unwrap();
        fs::write(repo.path().join("ignored"), "ignored").unwrap();
        for rename in ["true", "false", "copies"] {
            git_output(repo.path(), &["config", "status.renames", rename]).unwrap();
            let actual = working_paths(repo.path()).unwrap();
            assert_eq!(
                actual,
                [
                    ".atelier/issues/code.md",
                    "code.rs",
                    "tracked",
                    "untracked\n名"
                ]
                .map(PathBuf::from)
            );
            assert!(working_code_paths(repo.path())
                .unwrap()
                .contains(&PathBuf::from("code.rs")));
        }
    }
    #[test]
    fn native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries() {
        for format in ["sha1", "sha256"] {
            let repo = repo(format);
            let root = commit(repo.path(), "root");
            let branch = String::from_utf8(
                git_output(repo.path(), &["symbolic-ref", "--short", "HEAD"]).unwrap(),
            )
            .unwrap();
            let middle = commit(repo.path(), "middle");
            let head = commit(repo.path(), "head");
            git_output(repo.path(), &["checkout", "-qb", "outside", &root]).unwrap();
            let sibling = commit(repo.path(), "sibling");
            git_output(repo.path(), &["checkout", branch.trim()]).unwrap();
            let mut read = ReadContext::new(repo.path());
            assert!(read.edges(&head, &head).unwrap().is_empty());
            assert!(read.edges(&root, &root).unwrap().is_empty());
            assert_eq!(read.edges(&root.to_uppercase(), &head).unwrap().len(), 2);
            assert!(read.ancestor(&root.to_uppercase(), &head).unwrap());
            assert!(!read.ancestor(&sibling, &head).unwrap());
            assert!(read.edges(&sibling, &head).is_err());
            assert!(read.edges(&"0".repeat(root.len()), &head).is_err());
            assert_eq!(read.edges(&root, &sibling).unwrap().len(), 1);
            for earlier in [&root, &middle, &head, &sibling] {
                for later in [&root, &middle, &head, &sibling] {
                    assert_eq!(
                        read.ancestor(earlier, later).unwrap(),
                        git_output(
                            repo.path(),
                            &["merge-base", "--is-ancestor", earlier, later]
                        )
                        .is_ok()
                    );
                }
            }
            let clone = tempfile::tempdir().unwrap();
            let url = format!("file://{}", repo.path().display());
            git_output(clone.path(), &["clone", "-q", "--depth=2", &url, "."]).unwrap();
            // Keep the excluded root object present: topology must still honor
            // the native shallow boundary rather than raw parent headers.
            let bytes = git_output(repo.path(), &["cat-file", "commit", &root]).unwrap();
            let written = git_input(
                clone.path(),
                &["hash-object", "-t", "commit", "-w", "--stdin"],
                &bytes,
            )
            .unwrap();
            assert_eq!(String::from_utf8(written).unwrap().trim(), root);
            let mut shallow = ReadContext::new(clone.path());
            assert!(!shallow.ancestor(&root, &head).unwrap());
            assert!(shallow.edges(&root, &head).is_err());
            assert_eq!(shallow.edges(&middle, &head).unwrap().len(), 1);
            assert!(shallow.first_parent(&middle).unwrap().is_none());
        }
    }
    #[test]
    fn working_paths_read_fresh_hidden_bytes_and_modes() {
        for format in ["sha1", "sha256"] {
            let repo = repo(format);
            fs::write(repo.path().join("hidden-code"), "original").unwrap();
            fs::create_dir_all(repo.path().join(".atelier/issues")).unwrap();
            fs::write(repo.path().join(".atelier/issues/hidden.md"), "original").unwrap();
            commit(repo.path(), "tracked bytes");
            git_output(
                repo.path(),
                &["update-index", "--skip-worktree", "hidden-code"],
            )
            .unwrap();
            git_output(
                repo.path(),
                &[
                    "update-index",
                    "--assume-unchanged",
                    ".atelier/issues/hidden.md",
                ],
            )
            .unwrap();
            let status = working_status_paths(repo.path()).unwrap();
            assert!(status.is_empty());
            fs::write(repo.path().join("hidden-code"), "changed after status").unwrap();
            fs::write(
                repo.path().join(".atelier/issues/hidden.md"),
                "changed after status",
            )
            .unwrap();
            let expected = vec![
                PathBuf::from(".atelier/issues/hidden.md"),
                PathBuf::from("hidden-code"),
            ];
            assert_eq!(working_paths(repo.path()).unwrap(), expected);
            assert_eq!(
                working_code_paths(repo.path()).unwrap(),
                vec![PathBuf::from("hidden-code")]
            );
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::write(repo.path().join("hidden-code"), "original").unwrap();
                fs::write(repo.path().join(".atelier/issues/hidden.md"), "original").unwrap();
                git_output(repo.path(), &["config", "core.filemode", "false"]).unwrap();
                assert!(working_status_paths(repo.path()).unwrap().is_empty());
                fs::set_permissions(
                    repo.path().join("hidden-code"),
                    fs::Permissions::from_mode(0o755),
                )
                .unwrap();
                assert_eq!(
                    working_paths(repo.path()).unwrap(),
                    vec![PathBuf::from("hidden-code")]
                );
            }
        }
    }
    #[test]
    fn attached_working_paths_preserve_captured_head() {
        for format in ["sha1", "sha256"] {
            let repo = repo(format);
            let head = commit(repo.path(), "captured");
            let mut read = ReadContext::from_head(repo.path(), head.clone());
            assert_eq!(read.clean, None);
            assert_eq!(read.commit_id("HEAD").unwrap(), head);
            commit(repo.path(), "later checkout");
            let mut read = read.with_working_paths(&[PathBuf::from("unfinished-code")]);
            assert!(!read.code_clean().unwrap());
            assert_eq!(read.commit_id("HEAD").unwrap(), head);
        }
    }
}
