//! Reading records at a commit, without a checkout.
//!
//! The object reader is copied from `ReadContext` and `ObjectReader` in
//! `crates/atelier-app/src/git_freshness.rs`: one `git cat-file --batch`
//! process, started on first use, answers every object read, and a tree is
//! walked one component at a time. Trimmed: the caches, the pinning of
//! `HEAD`, the working-tree reads, the history and diff queries, and the
//! socket that stood in for the child's stdin. That socket kept a write to a
//! dead child from killing a binary that restores the default SIGPIPE; this
//! binary keeps Rust's default of ignoring SIGPIPE, so such a write fails
//! with an error. The loader on top of it is new.

use crate::{
    parse_item, parse_mission, store::review_file, ItemDir, MissionDir, Problem, ReviewFile,
};
use std::fmt;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

/// Why a file could not be read at a commit.
#[derive(Debug)]
pub enum ReadError {
    /// The commit has nothing at the path.
    Missing { commit: String, path: String },
    /// The path is a directory, a symbolic link or a submodule.
    NotAFile {
        commit: String,
        path: String,
        mode: String,
    },
    /// Git failed, or answered something this reader cannot read.
    Git(String),
    /// The file is there, and it does not read as a record.
    Record(Problem),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadError::Missing { commit, path } => write!(f, "{path} is not in commit {commit}"),
            ReadError::NotAFile { commit, path, mode } => write!(
                f,
                "{path} in commit {commit} is not a regular file (mode {mode})"
            ),
            ReadError::Git(message) => write!(f, "Git failed: {message}"),
            ReadError::Record(problem) => problem.fmt(f),
        }
    }
}

impl std::error::Error for ReadError {}

/// Reads files and records from commits of the repository at `repo`.
pub struct CommitReader {
    repo: PathBuf,
    batch: Option<Batch>,
}

struct Batch {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

struct Object {
    kind: String,
    data: Vec<u8>,
}

struct Entry {
    mode: String,
    name: Vec<u8>,
    oid: String,
}

const ITEMS: &str = ".atelier/items";
const MISSIONS: &str = ".atelier/missions";

impl CommitReader {
    pub fn new(repo: impl Into<PathBuf>) -> Self {
        CommitReader {
            repo: repo.into(),
            batch: None,
        }
    }

    /// The item `id` and its reviews as `commit` holds them, or `None` when
    /// the commit has no `item.md` for it.
    pub fn item(&mut self, commit: &str, id: &str) -> Result<Option<ItemDir>, ReadError> {
        let dir = format!("{ITEMS}/{id}");
        let path = format!("{dir}/item.md");
        let text = match self.text(commit, &path) {
            Err(ReadError::Missing { .. }) => return Ok(None),
            text => text?,
        };
        let path = shown(commit, &path);
        let item = parse_item(&path, &text).map_err(ReadError::Record)?;
        let reviews = self.reviews(commit, &dir)?;
        Ok(Some(ItemDir {
            path,
            item,
            reviews,
        }))
    }

    /// The reviews in `reviews/` of the record directory `dir` in `commit`,
    /// oldest round first.
    fn reviews(&mut self, commit: &str, dir: &str) -> Result<Vec<ReviewFile>, ReadError> {
        let mut reviews = Vec::new();
        for name in self.names(commit, &format!("{dir}/reviews"))? {
            if let Some(stem) = name.strip_suffix(".md") {
                let path = format!("{dir}/reviews/{name}");
                let text = self.text(commit, &path)?;
                let file = review_file(shown(commit, &path), stem, &text);
                reviews.push(file.map_err(ReadError::Record)?);
            }
        }
        reviews.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(reviews)
    }

    /// The mission `id` as `commit` holds it, or `None` when the commit has
    /// no `mission.md` for it.
    pub fn mission(&mut self, commit: &str, id: &str) -> Result<Option<MissionDir>, ReadError> {
        let dir = format!("{MISSIONS}/{id}");
        let path = format!("{dir}/mission.md");
        let text = match self.text(commit, &path) {
            Err(ReadError::Missing { .. }) => return Ok(None),
            text => text?,
        };
        let path = shown(commit, &path);
        let mission = parse_mission(&path, &text).map_err(ReadError::Record)?;
        let reviews = self.reviews(commit, &dir)?;
        Ok(Some(MissionDir {
            path,
            mission,
            reviews,
        }))
    }

    /// The names of the directories under `.atelier/items/` in `commit`.
    pub fn item_ids(&mut self, commit: &str) -> Result<Vec<String>, ReadError> {
        self.dirs(commit, ITEMS)
    }

    /// The names of the directories under `.atelier/missions/` in `commit`.
    pub fn mission_ids(&mut self, commit: &str) -> Result<Vec<String>, ReadError> {
        self.dirs(commit, MISSIONS)
    }

    /// The paths, from the root, of mission `id`'s documents in `commit`:
    /// the regular files of its directory, at any depth, except its
    /// `mission.md` and anything under its `reviews/`. In path order; none
    /// when the commit has no such directory.
    pub fn mission_documents(&mut self, commit: &str, id: &str) -> Result<Vec<String>, ReadError> {
        let dir = format!("{MISSIONS}/{id}");
        let mut paths = Vec::new();
        self.files(commit, &dir, &mut paths)?;
        let (record, reviews) = (format!("{dir}/mission.md"), format!("{dir}/reviews/"));
        paths.retain(|path| *path != record && !path.starts_with(&reviews));
        paths.sort();
        Ok(paths)
    }

    /// Adds to `paths` the path of each regular file under `dir` in
    /// `commit`, at any depth.
    fn files(&mut self, commit: &str, dir: &str, paths: &mut Vec<String>) -> Result<(), ReadError> {
        for entry in self.tree_at(commit, dir)?.unwrap_or_default() {
            let path = format!("{dir}/{}", name(entry.name)?);
            match entry.mode.as_str() {
                "40000" => self.files(commit, &path, paths)?,
                "100644" | "100755" => paths.push(path),
                _ => {}
            }
        }
        Ok(())
    }

    fn dirs(&mut self, commit: &str, dir: &str) -> Result<Vec<String>, ReadError> {
        let Some(entries) = self.tree_at(commit, dir)? else {
            return Ok(Vec::new());
        };
        entries
            .into_iter()
            .filter(|entry| entry.mode == "40000")
            .map(|entry| name(entry.name))
            .collect()
    }

    /// The text of the regular file at `path` in `commit`.
    pub fn text(&mut self, commit: &str, path: &str) -> Result<String, ReadError> {
        let (parent, file) = path.rsplit_once('/').unwrap_or(("", path));
        let entry = self
            .tree_at(commit, parent)?
            .and_then(|entries| entries.into_iter().find(|e| e.name == file.as_bytes()))
            .ok_or_else(|| ReadError::Missing {
                commit: commit.to_string(),
                path: path.to_string(),
            })?;
        if !matches!(entry.mode.as_str(), "100644" | "100755") {
            return Err(ReadError::NotAFile {
                commit: commit.to_string(),
                path: path.to_string(),
                mode: entry.mode,
            });
        }
        let data = self.object(&entry.oid, "blob")?.data;
        String::from_utf8(data).map_err(|_| {
            ReadError::Record(Problem::new(
                &shown(commit, path),
                "",
                "the file is not UTF-8",
            ))
        })
    }

    /// The text of the blob `object` names, such as a blob's ID, whole or
    /// abbreviated as Git printed it, or `<commit>:<path>`; `None` when it
    /// names nothing.
    pub fn blob(&mut self, object: &str) -> Result<Option<String>, ReadError> {
        let result = self.read(object);
        if result.is_err() {
            // The batch stream may be out of step; start a new one next time.
            self.batch = None;
        }
        let data = match result? {
            None => return Ok(None),
            Some(found) if found.kind == "blob" => found.data,
            Some(found) => {
                return Err(ReadError::Git(format!(
                    "{object} is a {}, not a blob",
                    found.kind
                )))
            }
        };
        let text = String::from_utf8(data).map_err(|_| {
            ReadError::Record(Problem::new(Path::new(object), "", "the file is not UTF-8"))
        })?;
        Ok(Some(text))
    }

    /// The content of the file staged at `path`, from the index Git names
    /// in `GIT_INDEX_FILE` when it sets that for a hook, else the
    /// checkout's.
    pub fn staged(&mut self, path: &str) -> Result<Vec<u8>, ReadError> {
        Ok(self.object(&format!(":0:{path}"), "blob")?.data)
    }

    /// The names of the entries of the directory `dir` in `commit`; none
    /// when it is not there.
    fn names(&mut self, commit: &str, dir: &str) -> Result<Vec<String>, ReadError> {
        let entries = self.tree_at(commit, dir)?.unwrap_or_default();
        entries.into_iter().map(|entry| name(entry.name)).collect()
    }

    /// The entries of the directory at `dir` in `commit`, or `None` when the
    /// commit has no directory there. `""` is the root.
    fn tree_at(&mut self, commit: &str, dir: &str) -> Result<Option<Vec<Entry>>, ReadError> {
        let object = self.object(commit, "commit")?;
        let tree = object
            .data
            .split(|byte| *byte == b'\n')
            .next()
            .and_then(|line| line.strip_prefix(b"tree "))
            .and_then(|tree| std::str::from_utf8(tree).ok())
            .ok_or_else(|| ReadError::Git(format!("commit {commit} names no tree")))?
            .to_string();
        let mut entries = self.tree(&tree)?;
        for component in dir.split('/').filter(|part| !part.is_empty()) {
            let Some(found) = entries
                .into_iter()
                .find(|entry| entry.name == component.as_bytes() && entry.mode == "40000")
            else {
                return Ok(None);
            };
            entries = self.tree(&found.oid)?;
        }
        Ok(Some(entries))
    }

    fn tree(&mut self, oid: &str) -> Result<Vec<Entry>, ReadError> {
        let data = self.object(oid, "tree")?.data;
        let invalid = || ReadError::Git(format!("tree {oid} does not parse"));
        let hash_len = oid.len() / 2;
        let mut entries = Vec::new();
        let mut rest = data.as_slice();
        while !rest.is_empty() {
            let space = rest.iter().position(|b| *b == b' ').ok_or_else(invalid)?;
            let nul = rest.iter().position(|b| *b == 0).ok_or_else(invalid)?;
            if nul < space || rest.len() < nul + 1 + hash_len {
                return Err(invalid());
            }
            let oid = rest[nul + 1..nul + 1 + hash_len]
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect();
            entries.push(Entry {
                mode: String::from_utf8_lossy(&rest[..space]).into_owned(),
                name: rest[space + 1..nul].to_vec(),
                oid,
            });
            rest = &rest[nul + 1 + hash_len..];
        }
        Ok(entries)
    }

    /// The object `expression` names, which must be of `kind`.
    fn object(&mut self, expression: &str, kind: &str) -> Result<Object, ReadError> {
        let result = self.read(expression);
        if result.is_err() {
            // The batch stream may be out of step; start a new one next time.
            self.batch = None;
        }
        match result? {
            Some(object) if object.kind == kind => Ok(object),
            Some(object) => Err(ReadError::Git(format!(
                "{expression} is a {}, not a {kind}",
                object.kind
            ))),
            None => Err(ReadError::Git(format!("{expression} names no {kind}"))),
        }
    }

    fn read(&mut self, expression: &str) -> Result<Option<Object>, ReadError> {
        if expression.contains(['\n', '\r']) {
            return Err(ReadError::Git(
                "a Git object name holds a line break".into(),
            ));
        }
        if self.batch.is_none() {
            self.batch = Some(Batch::spawn(&self.repo)?);
        }
        let batch = self.batch.as_mut().expect("the batch was just started");
        let mut header = String::new();
        let answered = batch
            .stdin
            .write_all(format!("{expression}\n").as_bytes())
            .and_then(|()| batch.stdin.flush())
            .and_then(|()| batch.stdout.read_line(&mut header));
        if !matches!(answered, Ok(read) if read > 0) {
            return Err(batch.failure());
        }
        let header = header.strip_suffix('\n').unwrap_or(&header);
        if header.ends_with(" missing") || header.ends_with(" ambiguous") {
            return Ok(None);
        }
        let fields: Vec<&str> = header.split(' ').collect();
        let [oid, kind, size] = fields.as_slice() else {
            return Err(ReadError::Git(format!("git cat-file --batch: {header}")));
        };
        let size: usize = size
            .parse()
            .ok()
            .filter(|_| matches!(oid.len(), 40 | 64))
            .ok_or_else(|| ReadError::Git(format!("git cat-file --batch: {header}")))?;
        let mut data = vec![0; size + 1];
        batch
            .stdout
            .read_exact(&mut data)
            .map_err(|_| batch.failure())?;
        if data.pop() != Some(b'\n') {
            return Err(ReadError::Git(
                "git cat-file --batch: output out of step".into(),
            ));
        }
        Ok(Some(Object {
            kind: kind.to_string(),
            data,
        }))
    }
}

impl Batch {
    fn spawn(repo: &Path) -> Result<Batch, ReadError> {
        let mut child = Command::new("git")
            .current_dir(repo)
            .args(["--no-replace-objects", "cat-file", "--batch"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| ReadError::Git(format!("cannot start git cat-file: {error}")))?;
        let stdin = child.stdin.take().expect("stdin is piped");
        let stdout = BufReader::new(child.stdout.take().expect("stdout is piped"));
        Ok(Batch {
            child,
            stdin,
            stdout,
        })
    }

    /// The error for a batch process that stopped answering, with what it
    /// wrote to stderr.
    fn failure(&mut self) -> ReadError {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let mut stderr = String::new();
        if let Some(mut pipe) = self.child.stderr.take() {
            let _ = pipe.read_to_string(&mut stderr);
        }
        ReadError::Git(format!("git cat-file --batch stopped: {}", stderr.trim()))
    }
}

impl Drop for Batch {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// How a file read at a commit is named in a problem: `<commit>:<path>`.
fn shown(commit: &str, path: &str) -> PathBuf {
    PathBuf::from(format!("{commit}:{path}"))
}

fn name(bytes: Vec<u8>) -> Result<String, ReadError> {
    String::from_utf8(bytes).map_err(|_| ReadError::Git("a tree entry's name is not UTF-8".into()))
}
