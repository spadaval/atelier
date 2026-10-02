//! Git history primitives shared by evidence and review freshness checks.
use anyhow::{bail, Context, Result};
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

#[derive(Clone)]
pub struct CommitEdge {
    pub parent: String,
    pub commit: String,
    pub paths: Vec<PathBuf>,
}

/// Immutable Git facts for one command read phase. Mutable checks stay outside.
pub struct ReadContext<'a> {
    pub(crate) repo: &'a Path,
    head: Option<String>,
    clean: Option<bool>,
    ranges: HashMap<(String, String), Arc<Vec<CommitEdge>>>,
    graphs: HashMap<String, Arc<ParentGraph>>,
    objects: Option<ObjectReader>,
    defer_stderr: bool,
    edge_paths: HashMap<(String, String), Vec<PathBuf>>,
    rename: Option<Option<&'static str>>,
    authors: HashMap<String, Vec<u8>>,
}
impl<'a> ReadContext<'a> {
    pub fn new(repo: &'a Path) -> Self {
        Self {
            repo,
            head: None,
            clean: None,
            ranges: HashMap::new(),
            graphs: HashMap::new(),
            objects: None,
            defer_stderr: false,
            edge_paths: HashMap::new(),
            rename: None,
            authors: HashMap::new(),
        }
    }
    pub fn from_snapshot(repo: &'a Path, head: String, dirty: &[PathBuf]) -> Self {
        Self::from_head(repo, head).with_working_paths(dirty)
    }
    /// Capture immutable HEAD; working-path facts remain unobserved.
    pub fn from_head(repo: &'a Path, head: String) -> Self {
        let mut context = Self::new(repo);
        context.head = Some(head);
        context
    }
    /// Attach fresh working-path proof without replacing captured Git facts.
    pub fn with_working_paths(mut self, dirty: &[PathBuf]) -> Self {
        self.clean = Some(!dirty.iter().any(|path| !is_record_path(path)));
        self
    }
    /// Buffer native object diagnostics until earlier read guards succeed.
    pub fn defer_object_stderr(mut self) -> Self {
        self.defer_stderr = true;
        self
    }
    /// Publish buffered diagnostics before consuming the deferred history result.
    pub fn publish_object_stderr(&mut self) {
        self.defer_stderr = false;
        if let Some(stderr) = self
            .objects
            .as_mut()
            .and_then(|reader| reader.stderr.as_mut())
        {
            stderr.publish();
        }
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
        if self.clean.is_none() {
            self.clean = Some(working_code_paths(self.repo)?.is_empty());
        }
        Ok(self.clean.expect("checked code paths"))
    }
    pub(crate) fn edges(&mut self, pin: &str, end: &str) -> Result<Arc<Vec<CommitEdge>>> {
        let end = self.commit_id(end)?;
        let pin = self.commit_id(pin)?;
        let key = (pin.clone(), end.clone());
        if !self.ranges.contains_key(&key) {
            let edges = self.load_edges(&pin, &end)?;
            self.ranges.insert(key.clone(), Arc::new(edges));
        }
        Ok(Arc::clone(&self.ranges[&key]))
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
        if earlier == later {
            return Ok(true);
        }
        Ok(self
            .parent_graph(&later)?
            .reachable(&later)
            .contains(earlier.as_str()))
    }
    pub(crate) fn first_parent(&mut self, commit: &str) -> Result<Option<String>> {
        let commit = self.commit_id(commit)?;
        Ok(self.parent_graph(&commit)?.parents[&commit]
            .first()
            .cloned())
    }
    pub(crate) fn author_names(&mut self, commit: &str) -> Result<String> {
        let commit = self.commit_id(commit)?;
        if !self.authors.contains_key(&commit) {
            let names = git_output(self.repo, &["show", "-s", "--format=%an%n%ae", &commit])?;
            self.authors.insert(commit.clone(), names);
        }
        Ok(String::from_utf8(self.authors[&commit].clone())?)
    }
    /// Warm only explicitly requested ranges; unavailable optional proof pins
    /// remain for their consumer to classify. Consumers retain required queries
    /// after a failed prefetch. All mutable fences remain separate.
    pub fn prefetch_ranges(
        &mut self,
        ranges: impl IntoIterator<Item = (String, String)>,
    ) -> Result<()> {
        let mut edges = Vec::new();
        for (pin, end) in ranges {
            let selected = (|| {
                let pin = self.commit_id(&pin)?;
                let end = self.commit_id(&end)?;
                self.select_edges(&pin, &end)
            })();
            if let Ok(selected) = selected {
                edges.extend(selected);
            }
        }
        self.populate_edges(&edges)
    }
    fn parent_graph(&mut self, end: &str) -> Result<Arc<ParentGraph>> {
        let head = self.commit_id("HEAD")?;
        for endpoint in [&head, end] {
            if !self.graphs.contains_key(endpoint) {
                let text = String::from_utf8(git_output(
                    self.repo,
                    &["rev-list", "--parents", endpoint],
                )?)?;
                let mut graph = ParentGraph {
                    order: Vec::new(),
                    parents: HashMap::new(),
                };
                for line in text.lines() {
                    let mut parts = line.split_whitespace();
                    let commit = parts.next().context("invalid Git parent graph")?.to_owned();
                    graph.order.push(commit.clone());
                    graph
                        .parents
                        .insert(commit, parts.map(str::to_owned).collect());
                }
                self.graphs.insert(endpoint.to_owned(), Arc::new(graph));
            }
            let graph = &self.graphs[endpoint];
            if graph.parents.contains_key(end) {
                return Ok(Arc::clone(graph));
            }
        }
        bail!("Git parent graph is missing its read endpoint")
    }
    fn object_spec(&mut self, spec: &str) -> Result<Option<&GitObject>> {
        if self.objects.is_none() {
            self.objects = Some(ObjectReader::new(self.repo, self.defer_stderr)?);
        }
        self.objects.as_mut().expect("object reader").get(spec)
    }
    fn object(&mut self, commit: &str, path: &Path) -> Result<Option<&GitObject>> {
        let commit = self.commit_id(commit)?;
        let mut spec = format!("{commit}^{{tree}}");
        for component in path.components() {
            let Some((_, kind, bytes)) = self.object_spec(&spec)? else {
                return Ok(None);
            };
            if kind != "tree" {
                bail!("expected Git directory tree");
            }
            let Some((_, oid)) = tree_entry(
                bytes,
                component.as_os_str().as_encoded_bytes(),
                commit.len() / 2,
            )?
            else {
                return Ok(None);
            };
            spec = oid;
        }
        self.object_spec(&spec)
    }
    pub fn commit_id(&mut self, commit: &str) -> Result<String> {
        let commit = self.pinned(commit)?;
        let (oid, kind, _) = self.object_spec(&commit)?.context("Git commit missing")?;
        if kind != "commit" {
            bail!("expected Git commit object");
        }
        Ok(oid.clone())
    }
    pub(crate) fn blob(&mut self, commit: &str, path: &Path) -> Result<String> {
        let (_, kind, bytes) = self
            .object(commit, path)?
            .context("Git record object missing")?;
        if kind != "blob" {
            bail!("expected Git record blob");
        }
        Ok(std::str::from_utf8(bytes)
            .context("Git record blob is not UTF-8")?
            .to_owned())
    }
    pub(crate) fn entry(&mut self, commit: &str, path: &Path) -> Result<Option<(String, String)>> {
        let oid_bytes = self.pinned("HEAD")?.len() / 2;
        let Some((_, kind, bytes)) =
            self.object(commit, path.parent().context("record has no parent path")?)?
        else {
            return Ok(None);
        };
        if kind != "tree" {
            bail!("expected Git record directory tree");
        }
        let entry = tree_entry(
            bytes,
            path.file_name()
                .context("record has no filename")?
                .as_encoded_bytes(),
            oid_bytes,
        )?;
        if entry
            .as_ref()
            .is_some_and(|(mode, _)| matches!(mode.as_str(), "40000" | "160000"))
        {
            bail!("expected blob entry for {}", path.display());
        }
        Ok(entry)
    }
}
fn tree_entry(bytes: &[u8], name: &[u8], oid_bytes: usize) -> Result<Option<(String, String)>> {
    let mut rest = bytes;
    while !rest.is_empty() {
        let space = rest
            .iter()
            .position(|byte| *byte == b' ')
            .context("invalid Git tree mode")?;
        let nul = rest
            .iter()
            .position(|byte| *byte == 0)
            .context("invalid Git tree filename")?;
        if space >= nul {
            bail!("invalid Git tree entry");
        }
        let oid = rest
            .get(nul + 1..nul + 1 + oid_bytes)
            .context("truncated Git tree object ID")?;
        if &rest[space + 1..nul] == name {
            return Ok(Some((
                std::str::from_utf8(&rest[..space])?.into(),
                oid.iter().map(|byte| format!("{byte:02x}")).collect(),
            )));
        }
        rest = &rest[nul + 1 + oid_bytes..];
    }
    Ok(None)
}

type GitObject = (String, String, Vec<u8>);
struct ObjectReader {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
    objects: HashMap<String, Option<GitObject>>,
    failed: bool,
    stderr: Option<DeferredStderr>,
}
impl ObjectReader {
    fn new(repo: &Path, defer_stderr: bool) -> Result<Self> {
        let mut child = command(repo)
            .args(["cat-file", "--batch"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(if defer_stderr {
                Stdio::piped()
            } else {
                Stdio::inherit()
            })
            .spawn()?;
        let input = child.stdin.take().context("Git object stdin missing")?;
        let output = BufReader::new(child.stdout.take().context("Git object stdout missing")?);
        let stderr = child.stderr.take().map(DeferredStderr::new);
        Ok(Self {
            child,
            input,
            output,
            objects: HashMap::new(),
            failed: false,
            stderr,
        })
    }
    fn get(&mut self, spec: &str) -> Result<Option<&GitObject>> {
        if self.failed {
            bail!("Git object reader failed");
        }
        if !self.objects.contains_key(spec) {
            match self.receive(spec) {
                Ok(object) => {
                    self.objects.insert(spec.into(), object);
                }
                Err(error) => {
                    self.failed = true;
                    self.stop();
                    return Err(error);
                }
            }
        }
        Ok(self.objects[spec].as_ref())
    }
    fn stop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Some(stderr) = &mut self.stderr {
            stderr.join();
        }
    }
    fn receive(&mut self, spec: &str) -> Result<Option<GitObject>> {
        if spec.contains(['\0', '\n']) {
            bail!("invalid Git object query");
        }
        self.input.write_all(spec.as_bytes())?;
        self.input.write_all(b"\n")?;
        self.input.flush()?;
        let mut header = Vec::new();
        self.output.read_until(b'\n', &mut header)?;
        if header.pop() != Some(b'\n') {
            bail!("Git object reader ended before its response");
        }
        if header == format!("{spec} missing").as_bytes() {
            return Ok(None);
        }
        let fields = std::str::from_utf8(&header)?
            .split_whitespace()
            .collect::<Vec<_>>();
        if fields.len() != 3
            || !matches!(fields[0].len(), 40 | 64)
            || !fields[0].bytes().all(|b| b.is_ascii_hexdigit())
        {
            bail!("invalid Git object header");
        }
        let size = fields[2].parse::<usize>()?;
        let mut bytes = vec![0; size];
        self.output.read_exact(&mut bytes)?;
        let mut delimiter = [1];
        self.output.read_exact(&mut delimiter)?;
        if delimiter != [b'\n'] {
            bail!("invalid Git object framing");
        }
        Ok(Some((fields[0].to_owned(), fields[1].to_owned(), bytes)))
    }
}
impl Drop for ObjectReader {
    fn drop(&mut self) {
        self.stop();
    }
}

// Drain throughout the persistent child's lifetime, even while diagnostics are
// deferred. The same lock orders buffered publication and subsequent live writes.
struct DeferredStderr {
    buffered: Arc<Mutex<Option<Vec<u8>>>>,
    drain: Option<JoinHandle<()>>,
}
impl DeferredStderr {
    fn new(mut pipe: impl Read + Send + 'static) -> Self {
        let buffered = Arc::new(Mutex::new(Some(Vec::new())));
        let destination = Arc::clone(&buffered);
        let drain = std::thread::spawn(move || {
            let mut bytes = [0; 8192];
            while let Ok(size) = pipe.read(&mut bytes) {
                if size == 0 {
                    break;
                }
                let mut destination = destination.lock().expect("object stderr lock");
                if let Some(buffer) = destination.as_mut() {
                    buffer.extend_from_slice(&bytes[..size]);
                } else {
                    let _ = std::io::stderr().write_all(&bytes[..size]);
                }
            }
        });
        Self {
            buffered,
            drain: Some(drain),
        }
    }
    fn publish(&mut self) {
        let mut buffered = self.buffered.lock().expect("object stderr lock");
        if let Some(bytes) = buffered.take() {
            let _ = std::io::stderr().write_all(&bytes);
        }
    }
    fn join(&mut self) {
        if let Some(drain) = self.drain.take() {
            let _ = drain.join();
        }
    }
}

/// Pinned evidence and review history always describe original Git objects.
pub fn command(repo: &Path) -> Command {
    let mut command = Command::new("git");
    command.current_dir(repo).arg("--no-replace-objects");
    command
}
pub fn git_output(repo: &Path, args: &[&str]) -> Result<Vec<u8>> {
    git_input(repo, args, None)
}
fn git_input(repo: &Path, args: &[&str], input: Option<&[u8]>) -> Result<Vec<u8>> {
    let mut child = command(repo)
        .args(args)
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take();
    let output = std::thread::scope(|scope| -> Result<_> {
        let writer = input
            .map(|input| scope.spawn(move || stdin.as_mut().expect("Git stdin").write_all(input)));
        let output = child.wait_with_output()?;
        if let Some(writer) = writer {
            writer.join().expect("Git input thread panicked")?;
        }
        Ok(output)
    })?;
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
            .to_string(),
    )
}
pub fn blob(repo: &Path, commit: &str, path: &Path) -> Result<String> {
    let spec = format!("{commit}:{}", path.display());
    String::from_utf8(git_output(repo, &["show", &spec])?).context("Git record blob is not UTF-8")
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
    // Rename/copy sources are also affected paths: deletions must stay visible.
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
struct ParentGraph {
    order: Vec<String>,
    parents: HashMap<String, Vec<String>>,
}
impl ParentGraph {
    fn reachable(&self, end: &str) -> HashSet<&str> {
        let mut found = HashSet::new();
        let mut pending = vec![self
            .parents
            .get_key_value(end)
            .expect("graph endpoint")
            .0
            .as_str()];
        while let Some(commit) = pending.pop() {
            if found.insert(commit) {
                pending.extend(
                    self.parents
                        .get(commit)
                        .into_iter()
                        .flatten()
                        .map(String::as_str),
                );
            }
        }
        found
    }
}
impl ReadContext<'_> {
    // All introduced commits include side branches; merges prefer a parent
    // containing the tested pin, so older parents cannot reintroduce old work.
    fn load_edges(&mut self, pin: &str, end: &str) -> Result<Vec<CommitEdge>> {
        let mut edges = self.select_edges(pin, end)?;
        self.populate_edges(&edges)?;
        for edge in &mut edges {
            edge.paths = self.edge_paths[&(edge.parent.clone(), edge.commit.clone())].clone();
        }
        Ok(edges)
    }
    fn select_edges(&mut self, pin: &str, end: &str) -> Result<Vec<CommitEdge>> {
        if pin == end {
            return Ok(Vec::new());
        }
        let parents = self.parent_graph(end)?;
        let included = parents.reachable(end);
        if !included.contains(pin) {
            bail!("pin is not an ancestor of the read endpoint");
        }
        let excluded = parents.reachable(pin);
        let graph = parents
            .order
            .iter()
            .filter(|commit| {
                included.contains(commit.as_str()) && !excluded.contains(commit.as_str())
            })
            .map(|commit| {
                std::iter::once(commit.as_str())
                    .chain(parents.parents[commit].iter().map(String::as_str))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
        for parts in &graph {
            for parent in &parts[1..] {
                children.entry(parent).or_default().push(parts[0]);
            }
        }
        let mut tested_context = HashSet::new();
        let mut pending = vec![pin];
        while let Some(commit) = pending.pop() {
            if tested_context.insert(commit) {
                pending.extend(children.get(commit).into_iter().flatten().copied());
            }
        }
        let mut edges = Vec::new();
        for parts in graph {
            let commit = parts[0];
            let parents = &parts[1..];
            if let Some(parent) = parents
                .iter()
                .find(|parent| tested_context.contains(**parent))
                .or_else(|| parents.first())
            {
                edges.push(CommitEdge {
                    parent: parent.to_string(),
                    commit: commit.to_string(),
                    paths: Vec::new(),
                });
            }
        }
        Ok(edges)
    }
    fn populate_edges(&mut self, edges: &[CommitEdge]) -> Result<()> {
        let mut missing = edges
            .iter()
            .filter(|edge| {
                !self
                    .edge_paths
                    .contains_key(&(edge.parent.clone(), edge.commit.clone()))
            })
            .map(|edge| ((edge.parent.clone(), edge.commit.clone()), edge.clone()))
            .collect::<std::collections::BTreeMap<_, _>>()
            .into_values()
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            let input = missing
                .iter()
                .map(|edge| format!("{} {}\n", edge.commit, edge.parent))
                .collect::<String>();
            if self.rename.is_none() {
                let config = command(self.repo)
                    .args([
                        "config",
                        "--null",
                        "--type=bool-or-str",
                        "--get",
                        "diff.renames",
                    ])
                    .output()?;
                self.rename = Some(if config.status.code() == Some(1) {
                    Some("-M")
                } else if config.status.success() {
                    let value = String::from_utf8(config.stdout)?;
                    let value = value
                        .strip_suffix('\0')
                        .context("missing Git config delimiter")?;
                    if value.eq_ignore_ascii_case("copies") {
                        Some("-C")
                    } else {
                        match value {
                            "true" => Some("-M"),
                            "false" => None,
                            _ => bail!("invalid diff.renames configuration"),
                        }
                    }
                } else {
                    bail!(
                        "cannot read diff.renames: {}",
                        String::from_utf8_lossy(&config.stderr)
                    );
                });
            }
            let rename = self.rename.expect("checked rename configuration");
            let mut args = vec![
                "diff-tree",
                "--stdin",
                "-r",
                "-z",
                "--name-status",
                "--format=%x00%H%x00%an%n%ae",
                "--always",
            ];
            args.extend(rename);
            let output = git_input(self.repo, &args, Some(input.as_bytes()))?;
            let mut authors = Vec::new();
            let mut parts = output.split(|byte| *byte == 0).peekable();
            for edge in &mut missing {
                if parts.next() != Some(&b""[..]) || parts.next() != Some(edge.commit.as_bytes()) {
                    bail!("Git returned an incomplete history diff batch");
                }
                let names = parts.next().context("missing history author metadata")?;
                authors.push((edge.commit.clone(), names.to_vec()));
                let mut first = true;
                while parts.peek().is_some_and(|part| !part.is_empty()) {
                    let status = parts.next().expect("peeked status");
                    let status = if first {
                        status
                            .strip_prefix(b"\n")
                            .context("missing history diff separator")?
                    } else {
                        status
                    };
                    first = false;
                    status_paths(status, &mut parts, &mut edge.paths)?;
                }
                edge.paths.sort();
                edge.paths.dedup();
            }
            if parts.any(|part| !part.is_empty()) {
                bail!("Git returned unexpected history diff records");
            }
            self.authors.extend(authors);
        }
        for edge in missing {
            self.edge_paths
                .insert((edge.parent, edge.commit), edge.paths);
        }
        Ok(())
    }
}
pub fn working_paths(repo: &Path) -> Result<Vec<PathBuf>> {
    working_paths_matching(repo, |_| true)
}
pub(crate) fn working_code_paths(repo: &Path) -> Result<Vec<PathBuf>> {
    working_paths_matching(repo, |path| !is_record_path(path))
}
fn working_paths_matching(repo: &Path, include: impl Fn(&Path) -> bool) -> Result<Vec<PathBuf>> {
    let mut changed = working_status_paths(repo)?;
    changed.retain(|path| include(path));
    changed.extend(crate::committed_snapshot::flagged_modified_paths_matching(
        repo, "HEAD", include,
    )?);
    changed.sort();
    changed.dedup();
    Ok(changed)
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
    use std::fs;
    fn commit_edges(repo: &Path, pin: &str) -> Result<Vec<CommitEdge>> {
        Ok(ReadContext::new(repo).edges(pin, "HEAD")?.as_ref().clone())
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
    fn prefetched_union_keeps_exact_ranges_and_optional_bad_pins_lazy() {
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
            read.prefetch_ranges([
                (pin.clone(), head.clone()),
                (middle.clone(), head.clone()),
                (sibling.clone(), head.clone()),
                ("0".repeat(pin.len()), head.clone()),
                ("bad pin".into(), head.clone()),
                (pin.clone(), middle.clone()),
            ])
            .unwrap();
            let count = read.edge_paths.len();
            for (start, end) in [(&pin, &head), (&middle, &head), (&pin, &middle)] {
                let expected = ReadContext::new(repo.path()).edges(start, end).unwrap();
                let actual = read.edges(start, end).unwrap();
                assert_eq!(
                    actual
                        .iter()
                        .map(|e| (&e.parent, &e.commit, &e.paths))
                        .collect::<Vec<_>>(),
                    expected
                        .iter()
                        .map(|e| (&e.parent, &e.commit, &e.paths))
                        .collect::<Vec<_>>()
                );
            }
            assert_eq!(count, read.edge_paths.len());
            assert!(read.edges(&sibling, &head).is_err());
            assert!(read.edges(&"0".repeat(pin.len()), &head).is_err());
        }
    }

    #[test]
    fn native_batch_author_fields_match_show_encoding_and_unusual_identities() {
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
                    Some(&object),
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
                read.prefetch_ranges([(pin.clone(), oid.into())]).unwrap();
                let cached = &read.authors[oid];
                assert_eq!(
                    cached.split(|b| *b == b'\n').collect::<Vec<_>>(),
                    expected
                        .strip_suffix(b"\n")
                        .unwrap()
                        .split(|b| *b == b'\n')
                        .collect::<Vec<_>>()
                );
                assert_eq!(
                    read.author_names(oid).is_ok(),
                    String::from_utf8(expected).is_ok()
                );
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
    fn batched_edges_match_git_diff_for_rename_configuration_and_weird_paths() {
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
    fn batched_edges_preserve_tested_merge_parent_and_side_branch_reverts() {
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
        union
            .prefetch_ranges([
                (base.clone(), merge.clone()),
                (pin.clone(), merge.clone()),
                (side_pin.clone(), merge.clone()),
            ])
            .unwrap();
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
        // Exercise both pipes beyond their kernel buffer capacity.
        let input = format!("{merge} {pin}\n").repeat(5000);
        let output = git_input(
            repo.path(),
            &[
                "diff-tree",
                "--stdin",
                "-r",
                "--name-only",
                "--always",
                "--format=%H",
            ],
            Some(input.as_bytes()),
        )
        .unwrap();
        assert!(output.len() > 64 * 1024);
    }
    #[test]
    fn object_reads_pin_heads_and_parse_binary_tree_modes_in_both_hash_formats() {
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
    fn newline_batch_headers_preserve_binary_bodies_and_missing_multiline_paths() {
        for format in ["sha1", "sha256"] {
            let repo = repo(format);
            let name = "folder\n名/binary\n名";
            let bytes = b"first\n\0second\nlast";
            fs::create_dir(repo.path().join("folder\n名")).unwrap();
            fs::write(repo.path().join(name), bytes).unwrap();
            let pin = commit(repo.path(), "binary");
            let mut read = ReadContext::new(repo.path());
            assert!(read
                .object(&pin, Path::new("folder\n名/missing\npath"))
                .unwrap()
                .is_none());
            assert_eq!(
                read.object(&pin, Path::new(name)).unwrap().unwrap().2,
                bytes
            );
            assert_eq!(read.object_spec(&pin).unwrap().unwrap().1, "commit");
            assert!(read
                .object(&pin, Path::new("other\nmissing"))
                .unwrap()
                .is_none());
            assert_eq!(
                read.object(&pin, Path::new(name)).unwrap().unwrap().2,
                bytes
            );
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
    fn endpoint_graph_matches_native_ancestry_for_roots_siblings_and_shallow_boundaries() {
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
            assert_eq!(read.graphs.len(), 1);
            assert_eq!(read.edges(&root, &sibling).unwrap().len(), 1);
            assert_eq!(read.graphs.len(), 2);
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
                Some(&bytes),
            )
            .unwrap();
            assert_eq!(String::from_utf8(written).unwrap().trim(), root);
            let mut shallow = ReadContext::new(clone.path());
            assert!(!shallow.ancestor(&root, &head).unwrap());
            assert!(shallow.edges(&root, &head).is_err());
            assert_eq!(shallow.edges(&middle, &head).unwrap().len(), 1);
            assert!(shallow.parent_graph(&head).unwrap().parents[&middle].is_empty());
        }
    }
    #[test]
    fn attached_working_paths_preserve_warmed_head_and_objects() {
        for format in ["sha1", "sha256"] {
            let repo = repo(format);
            let head = commit(repo.path(), "captured");
            let mut read = ReadContext::from_head(repo.path(), head.clone()).defer_object_stderr();
            assert_eq!(read.clean, None);
            assert_eq!(read.commit_id("HEAD").unwrap(), head);
            let cached = read.objects.as_ref().unwrap().objects.len();
            commit(repo.path(), "later checkout");
            let mut read = read.with_working_paths(&[PathBuf::from("unfinished-code")]);
            read.publish_object_stderr();
            assert!(!read.code_clean().unwrap());
            assert_eq!(read.commit_id("HEAD").unwrap(), head);
            assert_eq!(read.objects.as_ref().unwrap().objects.len(), cached);
            assert!(!read.defer_stderr);
        }
    }
    #[test]
    fn deferred_large_stderr_drains_before_poison_and_is_discardable() {
        let mut child = Command::new("python3")
            .args(["-c", "import sys; sys.stdin.readline(); sys.stderr.buffer.write(b'x' * 1048576); sys.stderr.flush(); print('invalid object header', flush=True)"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut reader = ObjectReader {
            input: child.stdin.take().unwrap(),
            output: BufReader::new(child.stdout.take().unwrap()),
            stderr: Some(DeferredStderr::new(child.stderr.take().unwrap())),
            child,
            objects: HashMap::new(),
            failed: false,
        };
        assert!(reader.get("HEAD").is_err());
        assert!(reader.failed);
        let stderr = reader.stderr.as_ref().unwrap();
        assert!(stderr.drain.is_none());
        assert_eq!(
            stderr.buffered.lock().unwrap().as_ref().unwrap(),
            &vec![b'x'; 1048576]
        );
        assert!(reader.child.try_wait().unwrap().is_some());
        assert!(reader.get("HEAD").is_err());
        // Drop joins an already reaped child without publishing discarded bytes.
        drop(reader);
    }
    #[test]
    fn failed_object_reader_is_poisoned_and_reaped() {
        let repo = repo("sha1");
        let mut reader = ObjectReader::new(repo.path(), false).unwrap();
        reader.child.kill().unwrap();
        reader.child.wait().unwrap();
        assert!(reader.get("HEAD").is_err());
        assert!(reader.failed);
        assert!(reader.get("HEAD").is_err());
        assert!(reader.child.try_wait().unwrap().is_some());
    }
}
