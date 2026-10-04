//! Git history primitives shared by evidence and review freshness checks.
use anyhow::{bail, Context, Result};
use std::{
    collections::HashSet,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub struct CommitEdge {
    pub parent: String,
    pub commit: String,
    pub paths: Vec<PathBuf>,
}

/// One read phase's captured endpoint and fresh checkout proof.
/// Create a new context after mutations or external checks.
pub struct ReadContext<'a> {
    pub(crate) repo: &'a Path,
    head: Option<String>,
    clean: Option<bool>,
}
impl<'a> ReadContext<'a> {
    pub fn new(repo: &'a Path) -> Self {
        Self {
            repo,
            head: None,
            clean: None,
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
        let oid = String::from_utf8(object_output(
            self.repo,
            &["rev-parse", "--verify", &expression],
        )?)?
        .trim()
        .to_ascii_lowercase();
        let kind =
            object_output(self.repo, &["cat-file", "-t", &oid]).context("Git commit missing")?;
        if kind != b"commit\n" {
            bail!("expected Git commit object");
        }
        Ok(oid)
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
        Ok(git_output(
            self.repo,
            &["merge-base", "--is-ancestor", &earlier, &later],
        )
        .is_ok())
    }
    pub(crate) fn first_parent(&mut self, commit: &str) -> Result<Option<String>> {
        let commit = self.commit_id(commit)?;
        let parents = String::from_utf8(git_output(
            self.repo,
            &["rev-list", "--parents", "-n", "1", &commit],
        )?)?;
        let mut parts = parents.split_whitespace();
        if parts.next() != Some(commit.as_str()) {
            bail!("invalid Git parent query");
        }
        Ok(parts.next().map(str::to_owned))
    }
    pub(crate) fn author_names(&mut self, commit: &str) -> Result<String> {
        let commit = self.commit_id(commit)?;
        Ok(String::from_utf8(git_output(
            self.repo,
            &["show", "-s", "--format=%an%n%ae", &commit],
        )?)?)
    }
    pub(crate) fn blob(&mut self, commit: &str, path: &Path) -> Result<String> {
        let commit = self.commit_id(commit)?;
        let spec = format!(
            "{commit}:{}",
            path.to_str().context("record path is not UTF-8")?
        );
        let kind = object_output(self.repo, &["cat-file", "-t", &spec])
            .context("Git record object missing")?;
        if kind != b"blob\n" {
            bail!("expected Git record blob");
        }
        String::from_utf8(object_output(self.repo, &["cat-file", "blob", &spec])?)
            .context("Git record blob is not UTF-8")
    }
    pub(crate) fn entry(&mut self, commit: &str, path: &Path) -> Result<Option<(String, String)>> {
        let commit = self.commit_id(commit)?;
        let name = path.to_str().context("record path is not UTF-8")?;
        let output = object_output(
            self.repo,
            &["--literal-pathspecs", "ls-tree", "-z", &commit, "--", name],
        )?;
        for row in output
            .split(|byte| *byte == 0)
            .filter(|row| !row.is_empty())
        {
            let tab = row
                .iter()
                .position(|byte| *byte == b'\t')
                .context("invalid Git tree entry")?;
            if &row[tab + 1..] != name.as_bytes() {
                continue;
            }
            let mut fields = std::str::from_utf8(&row[..tab])?.split_whitespace();
            let mode = fields.next().context("missing Git entry mode")?;
            if fields.next() != Some("blob") {
                bail!("expected blob entry for {}", path.display());
            }
            let oid = fields.next().context("missing Git blob ID")?;
            return Ok(Some((mode.into(), oid.into())));
        }
        Ok(None)
    }
    /// Inspect every introduced commit, including side-branch edit/revert history.
    /// Merges prefer a parent containing the tested pin to avoid reintroducing old work.
    pub(crate) fn edges(&mut self, pin: &str, end: &str) -> Result<Vec<CommitEdge>> {
        let end = self.commit_id(end)?;
        let pin = self.commit_id(pin)?;
        git_output(self.repo, &["merge-base", "--is-ancestor", &pin, &end])?;
        let range = format!("{pin}..{end}");
        let commits =
            String::from_utf8(git_output(self.repo, &["rev-list", "--parents", &range])?)?;
        let descendants = String::from_utf8(git_output(
            self.repo,
            &["rev-list", "--ancestry-path", &range],
        )?)?;
        let mut tested_context: HashSet<&str> = descendants.lines().collect();
        tested_context.insert(&pin);
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
                edges.push(CommitEdge {
                    parent: parent.to_string(),
                    commit: commit.to_owned(),
                    paths: changed_paths(self.repo, parent, commit)?,
                });
            }
        }
        Ok(edges)
    }
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
/// Native object warnings stay visible at this serial read point. Failed query
/// diagnostics stay in the error so optional missing proof can be classified.
fn object_output(repo: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let output = command(repo).args(args).stdin(Stdio::null()).output()?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let _ = std::io::stderr().write_all(&output.stderr);
    Ok(output.stdout)
}
pub fn head_commit(repo: &Path) -> Result<String> {
    Ok(
        String::from_utf8(git_output(repo, &["rev-parse", "HEAD"])?)?
            .trim()
            .to_owned(),
    )
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
        ReadContext::new(repo).edges(pin, "HEAD")
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
