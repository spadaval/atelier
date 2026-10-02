//! Git history primitives shared by evidence and review freshness checks.
use anyhow::{bail, Context, Result};
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    sync::Arc,
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
    ancestors: HashSet<(String, String)>,
    objects: Option<ObjectReader>,
    edge_paths: HashMap<(String, String), Vec<PathBuf>>,
    rename: Option<Option<&'static str>>,
}
impl<'a> ReadContext<'a> {
    pub fn new(repo: &'a Path) -> Self {
        Self {
            repo,
            head: None,
            clean: None,
            ranges: HashMap::new(),
            ancestors: HashSet::new(),
            objects: None,
            edge_paths: HashMap::new(),
            rename: None,
        }
    }
    pub fn from_snapshot(repo: &'a Path, head: String, dirty: &[PathBuf]) -> Self {
        let mut context = Self::new(repo);
        context.head = Some(head);
        context.clean = Some(!dirty.iter().any(|path| !is_record_path(path)));
        context
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
            self.ancestors.insert(key.clone());
            for edge in &edges {
                self.ancestors.insert((edge.commit.clone(), end.clone()));
            }
            self.ranges.insert(key.clone(), Arc::new(edges));
        }
        Ok(Arc::clone(&self.ranges[&key]))
    }
    pub(crate) fn ancestor(&mut self, earlier: &str, later: &str) -> Result<bool> {
        let later = self.pinned(later)?;
        let earlier = self.pinned(earlier)?;
        let key = (earlier.clone(), later.clone());
        if self.ancestors.contains(&key) {
            return Ok(true);
        }
        let valid = git_output(
            self.repo,
            &["merge-base", "--is-ancestor", &earlier, &later],
        )
        .is_ok();
        if valid {
            self.ancestors.insert(key);
        }
        Ok(valid)
    }
    fn object_spec(&mut self, spec: &str) -> Result<Option<&GitObject>> {
        if self.objects.is_none() {
            self.objects = Some(ObjectReader::new(self.repo)?);
        }
        self.objects.as_mut().expect("object reader").get(spec)
    }
    fn object(&mut self, commit: &str, path: &Path) -> Result<Option<&GitObject>> {
        let commit = self.pinned(commit)?;
        self.object_spec(&format!("{commit}:{}", path.display()))
    }
    fn commit_id(&mut self, commit: &str) -> Result<String> {
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
        let name = path
            .file_name()
            .context("record has no filename")?
            .as_encoded_bytes();
        let mut rest = bytes.as_slice();
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
                let mode = std::str::from_utf8(&rest[..space])?;
                if matches!(mode, "40000" | "160000") {
                    bail!("expected blob entry for {}", path.display());
                }
                return Ok(Some((
                    mode.into(),
                    oid.iter().map(|byte| format!("{byte:02x}")).collect(),
                )));
            }
            rest = &rest[nul + 1 + oid_bytes..];
        }
        Ok(None)
    }
}
type GitObject = (String, String, Vec<u8>);
struct ObjectReader {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
    objects: HashMap<String, Option<GitObject>>,
    failed: bool,
}
impl ObjectReader {
    fn new(repo: &Path) -> Result<Self> {
        let mut child = Command::new("git")
            .current_dir(repo)
            .args(["cat-file", "--batch", "-z"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let input = child.stdin.take().context("Git object stdin missing")?;
        let output = BufReader::new(child.stdout.take().context("Git object stdout missing")?);
        Ok(Self {
            child,
            input,
            output,
            objects: HashMap::new(),
            failed: false,
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
                    let _ = self.child.kill();
                    let _ = self.child.wait();
                    return Err(error);
                }
            }
        }
        Ok(self.objects[spec].as_ref())
    }
    fn receive(&mut self, spec: &str) -> Result<Option<GitObject>> {
        if spec.contains('\0') {
            bail!("invalid Git object query");
        }
        self.input.write_all(spec.as_bytes())?;
        self.input.write_all(&[0])?;
        self.input.flush()?;
        let mut header = Vec::new();
        self.output.read_until(b'\n', &mut header)?;
        let missing = format!("{spec} missing\n");
        if !header.is_empty() && missing.as_bytes().starts_with(&header) {
            // Missing-object output echoes the query, which can contain newlines.
            let mut remainder = vec![0; missing.len() - header.len()];
            self.output.read_exact(&mut remainder)?;
            header.extend(remainder);
            if header != missing.as_bytes() {
                bail!("invalid Git missing-object response");
            }
            return Ok(None);
        }
        if header.pop() != Some(b'\n') {
            bail!("Git object reader ended before its response");
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
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub fn git_output(repo: &Path, args: &[&str]) -> Result<Vec<u8>> {
    git_input(repo, args, None)
}
fn git_input(repo: &Path, args: &[&str], input: Option<&[u8]>) -> Result<Vec<u8>> {
    let mut child = Command::new("git")
        .current_dir(repo)
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
/// Inspects all introduced commits, including side-branch edit/revert history.
/// Merge results are compared to a parent containing the tested pin when possible:
/// older parents must not reintroduce changes already present at the pin.
pub fn commit_edges(repo: &Path, pin: &str) -> Result<Vec<CommitEdge>> {
    Ok(ReadContext::new(repo).edges(pin, "HEAD")?.as_ref().clone())
}
impl ReadContext<'_> {
    fn load_edges(&mut self, pin: &str, end: &str) -> Result<Vec<CommitEdge>> {
        let range = format!("{pin}..{end}");
        let commits =
            String::from_utf8(git_output(self.repo, &["rev-list", "--parents", &range])?)?;
        let graph = commits
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        if pin != end && !graph.iter().any(|parts| parts[1..].contains(&pin)) {
            bail!("pin is not an ancestor of the read endpoint");
        }
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
        let mut missing = edges
            .iter()
            .filter(|edge| {
                !self
                    .edge_paths
                    .contains_key(&(edge.parent.clone(), edge.commit.clone()))
            })
            .cloned()
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            let input = missing
                .iter()
                .map(|edge| format!("{} {}\n", edge.commit, edge.parent))
                .collect::<String>();
            if self.rename.is_none() {
                let config = Command::new("git")
                    .current_dir(self.repo)
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
                "--format=%x00%H",
                "--always",
            ];
            args.extend(rename);
            let output = git_input(self.repo, &args, Some(input.as_bytes()))?;
            let mut parts = output.split(|byte| *byte == 0).peekable();
            for edge in &mut missing {
                if parts.next() != Some(&b""[..]) || parts.next() != Some(edge.commit.as_bytes()) {
                    bail!("Git returned an incomplete history diff batch");
                }
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
        }
        for edge in missing {
            self.edge_paths
                .insert((edge.parent, edge.commit), edge.paths);
        }
        for edge in &mut edges {
            edge.paths = self.edge_paths[&(edge.parent.clone(), edge.commit.clone())].clone();
        }
        Ok(edges)
    }
}
pub fn working_paths(repo: &Path) -> Result<Vec<PathBuf>> {
    working_paths_matching(repo, |_| true)
}
pub(crate) fn working_code_paths(repo: &Path) -> Result<Vec<PathBuf>> {
    working_paths_matching(repo, |path| !is_record_path(path))
}
fn working_paths_matching(repo: &Path, include: impl Fn(&Path) -> bool) -> Result<Vec<PathBuf>> {
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
    changed.retain(|path| include(path));
    changed.extend(crate::committed_snapshot::flagged_modified_paths_matching(
        repo, "HEAD", include,
    )?);
    changed.sort();
    changed.dedup();
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
        commit(repo.path(), "side revert");
        git_output(repo.path(), &["checkout", "-qb", "tested", &base]).unwrap();
        fs::write(repo.path().join("proof"), "tested").unwrap();
        let pin = commit(repo.path(), "tested pin");
        git_output(repo.path(), &["merge", "--no-ff", "-qm", "merge", "side"]).unwrap();
        let merge = head_commit(repo.path()).unwrap();
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
        let repo = repo("sha1");
        let name = "binary\n名";
        let bytes = b"first\n\0second\nlast";
        fs::write(repo.path().join(name), bytes).unwrap();
        let pin = commit(repo.path(), "binary");
        let mut reader = ObjectReader::new(repo.path()).unwrap();
        assert!(reader
            .get(&format!("{pin}:missing\npath"))
            .unwrap()
            .is_none());
        assert_eq!(
            reader.get(&format!("{pin}:{name}")).unwrap().unwrap().2,
            bytes
        );
        assert_eq!(reader.get(&pin).unwrap().unwrap().1, "commit");
        assert!(reader
            .get(&format!("{pin}:other\nmissing"))
            .unwrap()
            .is_none());
        assert_eq!(
            reader.get(&format!("{pin}:{name}")).unwrap().unwrap().2,
            bytes
        );
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
    fn failed_object_reader_is_poisoned_and_reaped() {
        let repo = repo("sha1");
        let mut reader = ObjectReader::new(repo.path()).unwrap();
        reader.child.kill().unwrap();
        reader.child.wait().unwrap();
        assert!(reader.get("HEAD").is_err());
        assert!(reader.failed);
        assert!(reader.get("HEAD").is_err());
        assert!(reader.child.try_wait().unwrap().is_some());
    }
}
