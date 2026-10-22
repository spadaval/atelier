//! Path-limited `git log --root --format=%H` over record files, answered in
//! process (atelier-7b6e).
//!
//! The review gate asks, for each retained review event, which commit
//! introduced it and where its issue scope began, and evidence asks where
//! each claim was made. Each was a `git log` walk with its own pathspec, so
//! the walks grew with review records. [`RecordGraph`] lists the commit graph
//! once and diffs every commit against each of its parents once, limited to
//! the record directories. It then replays Git's revision walk for any
//! pathspec in those directories: the same queue ordered by committer date,
//! the same history simplification and the same diff filter. Parity tests
//! compare its answers with the `git log` each query replaces.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::path::Path;

use anyhow::{bail, Context, Result};

use crate::git_freshness::{git_input, git_output};

/// The directories a [`RecordGraph`] covers; every pathspec lies in one.
const RECORD_DIRS: [&str; 2] = [".atelier/issues/", ".atelier/reviews/"];

/// The parent position of a root commit's diff from nothing.
const ROOT: u32 = u32::MAX;

/// Which commits a [`PathLog`] lists.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Listing {
    /// `git log -- <paths>`: commits that change a path, under Git's default
    /// history simplification. A merge with no change from one of its
    /// parents is skipped, and only the first such parent is followed.
    Changes,
    /// `git log --diff-merges=first-parent --no-patch --no-renames
    /// --diff-filter=A -- <paths>`: commits whose diff from their first
    /// parent, or from nothing for a root, adds a path. Asking for merge
    /// diffs turns history simplification off, so every commit reachable
    /// from the start is examined. A clean merge of a branch that added a
    /// path is therefore listed as well as the branch's own commit.
    Additions,
}

/// One path-limited `git log --root --format=%H` query.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PathLog {
    /// The full commit ID the walk starts from.
    pub start: String,
    /// Pathspecs: record files, or a record directory ending in `/`.
    pub paths: Vec<String>,
    pub listing: Listing,
    /// `--reverse`: the commits in the opposite of walk order.
    pub reverse: bool,
}

impl PathLog {
    /// The `git log` arguments whose output this query reproduces, one
    /// commit ID per line; the parity tests' oracle.
    #[cfg(test)]
    pub(crate) fn git_args(&self) -> Vec<String> {
        let mut args = vec!["log", "--root"];
        if self.reverse {
            args.push("--reverse");
        }
        if self.listing == Listing::Additions {
            args.extend([
                "--diff-merges=first-parent",
                "--no-patch",
                "--no-renames",
                "--diff-filter=A",
            ]);
        }
        args.extend(["--format=%H", &self.start, "--"]);
        args.into_iter()
            .map(str::to_owned)
            .chain(self.paths.iter().cloned())
            .collect()
    }
}

/// The history reachable from one commit, with every commit's changes to
/// the record directories against each of its parents.
pub(crate) struct RecordGraph {
    ids: Vec<String>,
    index: HashMap<String, u32>,
    /// Committer timestamps, the key of Git's walk order.
    dates: Vec<i64>,
    /// Parents as Git reads them, after grafts and shallow boundaries.
    parents: Vec<Vec<u32>>,
    /// Every record path any diff touched, by name.
    paths: BTreeMap<String, u32>,
    /// For each path, the diffs that change it, as (commit, parent position).
    changes: Vec<Vec<(u32, u32)>>,
    /// For each path, the commits whose first-parent or root diff adds it.
    additions: Vec<Vec<u32>>,
}

impl RecordGraph {
    /// Two Git commands, whatever the number of records: `rev-list` for the
    /// graph and one `diff-tree` for every commit and parent.
    pub(crate) fn load(repo: &Path, head: &str) -> Result<Self> {
        let listed = String::from_utf8(git_output(
            repo,
            &["rev-list", "--parents", "--timestamp", head],
        )?)?;
        let mut ids = Vec::new();
        let mut dates = Vec::new();
        let mut parent_ids = Vec::new();
        for line in listed.lines() {
            let mut parts = line.split(' ');
            let (Some(date), Some(commit)) = (parts.next(), parts.next()) else {
                bail!("invalid Git rev-list line: {line}");
            };
            dates.push(
                date.parse::<i64>()
                    .with_context(|| format!("invalid Git commit date in: {line}"))?,
            );
            ids.push(commit.to_owned());
            parent_ids.push(parts.collect::<Vec<_>>());
        }
        let index = ids
            .iter()
            .enumerate()
            .map(|(position, id)| (id.clone(), position as u32))
            .collect::<HashMap<_, _>>();
        let mut parents = Vec::with_capacity(ids.len());
        let mut diffs = Vec::new();
        let mut input = String::new();
        for (commit, listed) in parent_ids.iter().enumerate() {
            let commit = commit as u32;
            let known = listed
                .iter()
                .map(|parent| {
                    index
                        .get(*parent)
                        .copied()
                        .with_context(|| format!("Git parent {parent} was not listed"))
                })
                .collect::<Result<Vec<_>>>()?;
            if known.is_empty() {
                input.push_str(&format!("{}\n", ids[commit as usize]));
                diffs.push((commit, ROOT));
            }
            for (position, parent) in listed.iter().enumerate() {
                input.push_str(&format!("{} {parent}\n", ids[commit as usize]));
                diffs.push((commit, position as u32));
            }
            parents.push(known);
        }
        let mut args = vec![
            "diff-tree",
            "--stdin",
            "-z",
            "-r",
            "--root",
            "--always",
            "--no-renames",
            "--no-abbrev",
            "--",
        ];
        args.extend(RECORD_DIRS);
        let output = git_input(repo, &args, input.as_bytes())?;
        let mut graph = Self {
            ids,
            index,
            dates,
            parents,
            paths: BTreeMap::new(),
            changes: Vec::new(),
            additions: Vec::new(),
        };
        graph.read_diffs(&output, &diffs)?;
        Ok(graph)
    }

    /// Reads `diff-tree --stdin --always -z` output: one commit ID per input
    /// line, in input order, each followed by its raw changes.
    fn read_diffs(&mut self, output: &[u8], diffs: &[(u32, u32)]) -> Result<()> {
        let mut tokens = output.split(|byte| *byte == 0);
        let mut expected = diffs.iter();
        let mut current = None;
        while let Some(token) = tokens.next() {
            if token.is_empty() {
                continue;
            }
            if token[0] != b':' {
                let diff = expected
                    .next()
                    .context("Git diff-tree printed more commits than it was given")?;
                if token != self.ids[diff.0 as usize].as_bytes() {
                    bail!("Git diff-tree answered out of order");
                }
                current = Some(*diff);
                continue;
            }
            let (commit, position) =
                current.context("Git diff-tree printed a change before its commit")?;
            let status = *token.last().context("empty Git diff-tree status")?;
            let path = std::str::from_utf8(tokens.next().context("Git diff-tree path missing")?)
                .context("record path is not UTF-8")?;
            let next = self.paths.len() as u32;
            let path = *self.paths.entry(path.to_owned()).or_insert(next);
            if path == next {
                self.changes.push(Vec::new());
                self.additions.push(Vec::new());
            }
            self.changes[path as usize].push((commit, position));
            if status == b'A' && matches!(position, 0 | ROOT) {
                self.additions[path as usize].push(commit);
            }
        }
        if expected.next().is_some() {
            bail!("Git diff-tree answered fewer commits than it was given");
        }
        Ok(())
    }

    /// The paths a pathspec matches: the file itself and anything below it,
    /// as Git matches a pathspec without wildcards.
    fn matching(&self, spec: &str) -> Result<Vec<u32>> {
        if !RECORD_DIRS.iter().any(|dir| spec.starts_with(dir))
            || spec.contains(['*', '?', '[', '\\'])
            || spec.starts_with(':')
        {
            bail!("pathspec {spec} is not a record path");
        }
        let mut matched = self
            .paths
            .get(spec)
            .copied()
            .into_iter()
            .collect::<Vec<_>>();
        let directory = if spec.ends_with('/') {
            spec.to_owned()
        } else {
            format!("{spec}/")
        };
        matched.extend(
            self.paths
                .range(directory.clone()..)
                .take_while(|(path, _)| path.starts_with(&directory))
                .map(|(_, id)| *id),
        );
        Ok(matched)
    }

    /// The output of `query.git_args()`, one commit ID per entry.
    ///
    /// Git pops the newest commit by committer date from its queue, earlier
    /// queued first among equal dates; before listing a commit it compares
    /// it with its parents and queues the parents it follows (`revision.c`,
    /// `process_parents` and `try_to_simplify_commit`).
    pub(crate) fn log(&self, query: &PathLog) -> Result<Vec<String>> {
        if query.paths.is_empty() {
            bail!("a record history query needs at least one path");
        }
        let mut changed = HashSet::new();
        let mut added = HashSet::new();
        for spec in &query.paths {
            for path in self.matching(spec)? {
                changed.extend(self.changes[path as usize].iter().copied());
                added.extend(self.additions[path as usize].iter().copied());
            }
        }
        let start = *self
            .index
            .get(&query.start)
            .with_context(|| format!("{} is not in the indexed history", query.start))?;
        let mut seen = vec![false; self.ids.len()];
        let mut queue = BinaryHeap::new();
        let mut queued = 0_u64;
        seen[start as usize] = true;
        queue.push((self.dates[start as usize], Reverse(queued), start));
        let mut listed = Vec::new();
        while let Some((_, _, commit)) = queue.pop() {
            let parents = &self.parents[commit as usize];
            let same = |position: u32| !changed.contains(&(commit, position));
            let (unchanged, followed) = if parents.is_empty() {
                (same(ROOT), &parents[..])
            } else {
                match query.listing {
                    Listing::Changes => match (0..parents.len() as u32).find(|p| same(*p)) {
                        Some(p) => (true, std::slice::from_ref(&parents[p as usize])),
                        None => (false, &parents[..]),
                    },
                    Listing::Additions => ((0..parents.len() as u32).all(same), &parents[..]),
                }
            };
            for parent in followed {
                if !std::mem::replace(&mut seen[*parent as usize], true) {
                    queued += 1;
                    queue.push((self.dates[*parent as usize], Reverse(queued), *parent));
                }
            }
            let shown = !unchanged
                && match query.listing {
                    Listing::Changes => true,
                    Listing::Additions => added.contains(&commit),
                };
            if shown {
                listed.push(self.ids[commit as usize].clone());
            }
        }
        if query.reverse {
            listed.reverse();
        }
        Ok(listed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_freshness::command;
    use std::fs;

    /// A scratch repository whose commits get controlled committer dates.
    struct Repo {
        dir: tempfile::TempDir,
        clock: i64,
        /// Seconds between one commit and the next; negative dates a commit
        /// before its parent and zero gives every commit the same date.
        step: i64,
    }

    impl Repo {
        fn new() -> Self {
            let repo = Self {
                dir: tempfile::tempdir().unwrap(),
                clock: 1_700_000_000,
                step: 60,
            };
            for args in [
                &["init", "-q", "-b", "main"][..],
                &["config", "user.name", "Author"],
                &["config", "user.email", "author@example.test"],
                &["config", "commit.gpgsign", "false"],
            ] {
                repo.git(args);
            }
            repo
        }
        fn path(&self) -> &Path {
            self.dir.path()
        }
        fn git(&self, args: &[&str]) -> String {
            let date = format!("{} +0000", self.clock);
            let output = command(self.path())
                .args(args)
                .env("GIT_COMMITTER_DATE", &date)
                .env("GIT_AUTHOR_DATE", &date)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "git {args:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            String::from_utf8(output.stdout).unwrap().trim().to_owned()
        }
        /// Runs a command that commits, at the next date, and returns HEAD.
        fn committing(&mut self, args: &[&str]) -> String {
            self.clock += self.step;
            self.git(args);
            self.git(&["rev-parse", "HEAD"])
        }
        fn write(&self, path: &str, text: &str) {
            let path = self.path().join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        }
        fn remove(&self, path: &str) {
            let path = self.path().join(path);
            if path.is_dir() {
                fs::remove_dir_all(path).unwrap();
            } else {
                fs::remove_file(path).unwrap();
            }
        }
        fn commit(&mut self, message: &str) -> String {
            self.git(&["add", "-A"]);
            self.committing(&["commit", "-q", "--allow-empty", "-m", message])
        }
        fn checkout(&self, args: &[&str]) {
            let mut all = vec!["checkout", "-q"];
            all.extend(args);
            self.git(&all);
        }
        fn merge(&mut self, args: &[&str]) -> String {
            let mut all = vec!["merge", "-q", "--no-ff", "-m", "merge"];
            all.extend(args);
            self.committing(&all)
        }
    }

    fn issue(id: &str) -> String {
        format!(".atelier/issues/{id}.md")
    }
    fn review(id: &str) -> String {
        format!(".atelier/reviews/{id}.md")
    }

    /// Every query shape, from every commit, over every record path, both
    /// record directories, a path nothing touched and `groups`, answers as
    /// the `git log` it replaces.
    fn assert_parity(repo: &Path, groups: &[Vec<String>]) {
        let head = git_output(repo, &["rev-parse", "HEAD"]).unwrap();
        let head = String::from_utf8(head).unwrap().trim().to_owned();
        let graph = RecordGraph::load(repo, &head).unwrap();
        let mut specs = graph
            .paths
            .keys()
            .map(|path| vec![path.clone()])
            .collect::<Vec<_>>();
        specs.extend(RECORD_DIRS.map(|dir| vec![dir.to_owned()]));
        specs.push(vec![issue("untouched")]);
        specs.extend(groups.iter().cloned());
        let mut compared = 0;
        for start in &graph.ids {
            for paths in &specs {
                for listing in [Listing::Changes, Listing::Additions] {
                    for reverse in [false, true] {
                        let query = PathLog {
                            start: start.clone(),
                            paths: paths.clone(),
                            listing,
                            reverse,
                        };
                        let args = query.git_args();
                        let args = args.iter().map(String::as_str).collect::<Vec<_>>();
                        let git = String::from_utf8(git_output(repo, &args).unwrap()).unwrap();
                        assert_eq!(
                            graph.log(&query).unwrap(),
                            git.lines().collect::<Vec<_>>(),
                            "git {}",
                            args.join(" ")
                        );
                        compared += 1;
                    }
                }
            }
        }
        assert!(compared > 0);
    }

    /// Side branches, clean merges, a merge that keeps only its first
    /// parent's records, a conflict resolved with a review event neither
    /// parent has, an octopus merge and a criss-cross.
    fn branchy(repo: &mut Repo) {
        repo.write(&issue("a"), "a\n");
        repo.write(&review("r1"), "r1\n");
        repo.write("code.txt", "code\n");
        let root = repo.commit("root");
        repo.checkout(&["-b", "side"]);
        repo.write(&review("r2"), "r2\n");
        repo.write(&issue("a"), "a side\n");
        repo.commit("side adds r2");
        repo.checkout(&["main"]);
        repo.write(&issue("b"), "b\n");
        repo.commit("main adds b");
        repo.merge(&["side"]);
        repo.checkout(&["-b", "dropped", &root]);
        repo.write(&issue("a"), "a dropped\n");
        repo.write(&review("r4"), "r4\n");
        repo.commit("dropped work");
        repo.checkout(&["main"]);
        repo.merge(&["-s", "ours", "dropped"]);
        repo.checkout(&["-b", "conflict"]);
        repo.write(&issue("c"), "c conflict\n");
        repo.commit("conflict side");
        repo.checkout(&["main"]);
        repo.write(&issue("c"), "c main\n");
        repo.commit("conflict main");
        let merged = command(repo.path())
            .args(["merge", "-q", "--no-ff", "conflict"])
            .output()
            .unwrap();
        assert!(!merged.status.success());
        repo.write(&issue("c"), "c resolved\n");
        repo.write(&review("r3"), "r3\n");
        repo.commit("resolve with r3");
        let mut legs = Vec::new();
        for leg in ["l1", "l2", "l3"] {
            repo.checkout(&["-b", leg, "main"]);
            repo.write(&review(leg), leg);
            legs.push(repo.commit(leg));
        }
        repo.checkout(&["main"]);
        repo.merge(&["l1", "l2", "l3"]);
        repo.checkout(&["-b", "cross", "main"]);
        repo.write(&issue("x"), "x\n");
        let cross = repo.commit("cross");
        repo.checkout(&["main"]);
        repo.write(&issue("y"), "y\n");
        let main = repo.commit("main y");
        repo.merge(&[&cross]);
        repo.checkout(&["cross"]);
        repo.merge(&[&main]);
        repo.write(&review("r5"), "r5\n");
        repo.commit("cross r5");
        repo.checkout(&["main"]);
        repo.merge(&["cross"]);
    }

    fn scope_groups() -> Vec<Vec<String>> {
        vec![
            vec![issue("a"), issue("b")],
            vec![issue("c"), issue("x"), issue("y")],
            vec![review("r1"), review("r3"), issue("a")],
        ]
    }

    #[test]
    fn matches_git_over_merges_octopus_and_criss_cross() {
        let mut repo = Repo::new();
        branchy(&mut repo);
        assert_parity(repo.path(), &scope_groups());
    }

    #[test]
    fn matches_git_when_dates_run_backwards_or_tie() {
        for step in [-60, 0] {
            let mut repo = Repo::new();
            repo.step = step;
            branchy(&mut repo);
            assert_parity(repo.path(), &scope_groups());
        }
    }

    #[test]
    fn matches_git_for_deleted_and_re_added_paths() {
        let mut repo = Repo::new();
        repo.write(&review("r"), "r\n");
        repo.write(&issue("a"), "a\n");
        let root = repo.commit("add");
        repo.remove(&review("r"));
        repo.commit("delete r");
        repo.write(&review("r"), "r\n");
        repo.commit("re-add r unchanged");
        repo.remove(&review("r"));
        repo.write(&review("r"), "r changed\n");
        repo.commit("rewrite r");
        repo.checkout(&["-b", "side", &root]);
        repo.remove(&issue("a"));
        repo.commit("side deletes a");
        repo.write(&issue("a"), "a again\n");
        repo.commit("side re-adds a");
        repo.checkout(&["main"]);
        repo.write(&issue("a"), "a main\n");
        repo.commit("main edits a");
        repo.merge(&["-X", "theirs", "side"]);
        // A record path that becomes a directory and then a file again.
        repo.remove(&issue("a"));
        repo.write(&format!("{}/inner.md", issue("a")), "inner\n");
        repo.commit("a becomes a directory");
        repo.remove(&issue("a"));
        repo.write(&issue("a"), "a file\n");
        repo.commit("a is a file again");
        assert_parity(repo.path(), &[vec![issue("a"), review("r")]]);
    }

    #[test]
    fn matches_git_for_cherry_picked_review_files() {
        let mut repo = Repo::new();
        repo.write(&issue("a"), "a\n");
        let root = repo.commit("root");
        repo.checkout(&["-b", "side"]);
        repo.write(&review("r"), "r\n");
        let picked = repo.commit("side adds r");
        repo.checkout(&["-b", "other", &root]);
        repo.committing(&["cherry-pick", &picked]);
        repo.checkout(&["main"]);
        repo.committing(&["cherry-pick", &picked]);
        repo.write(&issue("b"), "b\n");
        repo.commit("main adds b");
        repo.merge(&["side"]);
        repo.merge(&["other"]);
        repo.checkout(&["other"]);
        repo.merge(&["side"]);
        repo.checkout(&["main"]);
        repo.merge(&["other"]);
        assert_parity(repo.path(), &[vec![issue("a"), issue("b"), review("r")]]);
    }

    #[test]
    fn matches_git_under_grafts() {
        let mut repo = Repo::new();
        branchy(&mut repo);
        let commits = repo.git(&["rev-list", "--parents", "HEAD"]);
        let lines = commits.lines().collect::<Vec<_>>();
        // Reparent a merge onto one parent, give a commit an extra parent
        // and cut the history at another.
        let merge = lines.iter().find(|line| line.split(' ').count() == 3);
        let merge = merge.unwrap().split(' ').collect::<Vec<_>>();
        let single = lines.iter().filter(|line| line.split(' ').count() == 2);
        let single = single.map(|line| line.split(' ').next().unwrap());
        let single = single.collect::<Vec<_>>();
        let grafts = format!(
            "{} {}\n{} {} {}\n{}\n",
            merge[0],
            merge[2],
            single[2],
            single[4],
            single[6],
            single[single.len() - 3],
        );
        fs::write(repo.path().join(".git/info/grafts"), grafts).unwrap();
        assert_parity(repo.path(), &scope_groups());
    }

    #[test]
    fn matches_git_in_shallow_clones() {
        let mut repo = Repo::new();
        branchy(&mut repo);
        for depth in ["2", "4", "7"] {
            let clone = tempfile::tempdir().unwrap();
            let url = format!("file://{}", repo.path().display());
            let target = clone.path().join("clone");
            let target = target.to_str().unwrap();
            git_output(
                clone.path(),
                &["clone", "-q", "--depth", depth, &url, target],
            )
            .unwrap();
            let shallow = Path::new(target);
            assert!(shallow.join(".git/shallow").exists());
            assert_parity(shallow, &scope_groups());
        }
    }

    #[test]
    fn additions_list_records_resolved_or_renamed_into_being() {
        let mut repo = Repo::new();
        repo.write("code.txt", "code\n");
        repo.write(&issue("old"), &"same line\n".repeat(20));
        repo.commit("root");
        repo.checkout(&["-b", "side"]);
        repo.write("code.txt", "side\n");
        repo.commit("side");
        repo.checkout(&["main"]);
        repo.write("code.txt", "main\n");
        repo.commit("main");
        let merged = command(repo.path())
            .args(["merge", "-q", "--no-ff", "side"])
            .output()
            .unwrap();
        assert!(!merged.status.success());
        repo.write("code.txt", "resolved\n");
        repo.write(&review("first"), "first\n");
        let resolution = repo.commit("resolve with the first review");
        repo.remove(&issue("old"));
        repo.write(&issue("new"), &"same line\n".repeat(20));
        let renamed = repo.commit("rename old to new");
        let head = repo.git(&["rev-parse", "HEAD"]);
        let graph = RecordGraph::load(repo.path(), &head).unwrap();
        let additions = |paths: Vec<String>| {
            graph
                .log(&PathLog {
                    start: head.clone(),
                    paths,
                    listing: Listing::Additions,
                    reverse: true,
                })
                .unwrap()
        };
        // append_only_history's first review event, which the query
        // without merge diffs could not see (atelier-as7i).
        assert_eq!(
            additions(vec![".atelier/reviews/".to_owned()]),
            vec![resolution]
        );
        let without_merge_diffs = repo.git(&[
            "log",
            "--root",
            "--reverse",
            "--diff-filter=A",
            "--format=%H",
            &head,
            "--",
            ".atelier/reviews/",
        ]);
        assert_eq!(without_merge_diffs, "");
        // A record added as another is deleted is an addition, not a rename.
        let scope = vec![issue("old"), issue("new")];
        assert_eq!(additions(scope.clone()).last(), Some(&renamed));
        let with_renames = repo.git(&[
            "log",
            "--root",
            "--reverse",
            "-M",
            "--diff-merges=first-parent",
            "--no-patch",
            "--diff-filter=A",
            "--format=%H",
            &head,
            "--",
            &scope[0],
            &scope[1],
        ]);
        assert!(!with_renames.contains(&renamed));
    }

    #[test]
    fn rejects_paths_and_starts_outside_the_indexed_history() {
        let mut repo = Repo::new();
        repo.write(&issue("a"), "a\n");
        let head = repo.commit("root");
        let graph = RecordGraph::load(repo.path(), &head).unwrap();
        let query = |start: &str, path: &str| PathLog {
            start: start.to_owned(),
            paths: vec![path.to_owned()],
            listing: Listing::Changes,
            reverse: false,
        };
        assert_eq!(
            graph.log(&query(&head, &issue("a"))).unwrap(),
            vec![head.clone()]
        );
        for path in [
            "code.txt",
            ".atelier/",
            ".atelier/issues/*.md",
            ":(glob).atelier/issues/a.md",
        ] {
            assert!(graph.log(&query(&head, path)).is_err(), "{path}");
        }
        assert!(graph.log(&query(&"0".repeat(40), &issue("a"))).is_err());
        let mut unlimited = query(&head, &issue("a"));
        unlimited.paths.clear();
        assert!(graph.log(&unlimited).is_err());
    }
}
