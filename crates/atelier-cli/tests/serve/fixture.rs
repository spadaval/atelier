//! Fixture repositories for `atelier serve`, written record by record with
//! fixed timestamps and commit dates, so their commits and every response
//! read from them are the same on every run.

use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Repo {
    pub dir: PathBuf,
    clock: Cell<i64>,
}

/// 2026-01-01T00:00:00Z.
const EPOCH: i64 = 1_767_225_600;

impl Repo {
    /// A new repository at `dir` on branch `base`, with `slug`'s config and
    /// `policy`, committed.
    pub fn init(dir: &Path, slug: &str, base: &str, policy: &str) -> Self {
        fs::create_dir_all(dir).unwrap();
        let repo = Self {
            dir: dir.to_path_buf(),
            clock: Cell::new(0),
        };
        repo.git(&["init", "-q", "-b", base]);
        repo.write(
            ".atelier/config.toml",
            &format!(
                "schema = \"atelier.project_config\"\nschema_version = 1\nproject_slug = \"{slug}\"\n\n[paths]\nstate_root = \".atelier\"\n"
            ),
        );
        repo.write(".atelier/workflow.yaml", policy);
        repo.commit("Initialize Atelier");
        repo
    }

    /// An existing checkout, such as a worktree, to write and commit in.
    pub fn open(dir: &Path) -> Self {
        Self {
            dir: dir.to_path_buf(),
            clock: Cell::new(0),
        }
    }

    /// Runs git with a fixed identity and the next commit date, failing the
    /// test on a non-zero exit, and returns its trimmed output.
    pub fn git(&self, args: &[&str]) -> String {
        git_in(&self.dir, args, self.clock.get())
    }

    pub fn write(&self, path: &str, text: &str) {
        let path = self.dir.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    /// Commits everything at the next minute and returns the commit.
    pub fn commit(&self, message: &str) -> String {
        self.clock.set(self.clock.get() + 60);
        self.git(&["add", "-A"]);
        self.git(&["commit", "-q", "--no-verify", "-m", message]);
        self.git(&["rev-parse", "HEAD"])
    }

    /// The record time `minutes` after the fixture epoch.
    pub fn at(minutes: i64) -> String {
        chrono::DateTime::from_timestamp(EPOCH + minutes * 60, 0)
            .unwrap()
            .to_rfc3339()
    }
}

/// Runs git in `dir` with a fixed identity, no signing and commit dates
/// `seconds` after the fixture epoch.
pub fn git_in(dir: &Path, args: &[&str], seconds: i64) -> String {
    let date = format!("@{} +0000", EPOCH + seconds);
    let output = Command::new("git")
        .current_dir(dir)
        .args([
            "-c",
            "commit.gpgsign=false",
            "-c",
            "core.hooksPath=/dev/null",
        ])
        .args(args)
        .env("GIT_AUTHOR_NAME", "Fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.test")
        .env("GIT_COMMITTER_NAME", "Fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.test")
        .env("GIT_AUTHOR_DATE", &date)
        .env("GIT_COMMITTER_DATE", &date)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?} in {}: {}",
        dir.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

/// An issue record.
pub struct Issue<'a> {
    pub id: &'a str,
    pub issue_type: &'a str,
    pub status: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub labels: &'a [&'a str],
    pub strategy: Option<&'a str>,
    /// `(issue, link type)` links stored on this record.
    pub relates: &'a [(&'a str, &'a str)],
    pub blocks: &'a [&'a str],
    /// Minutes after the epoch: created, updated, and closed when done.
    pub created: i64,
    pub updated: i64,
    pub closed: Option<i64>,
}

impl Default for Issue<'_> {
    fn default() -> Self {
        Self {
            id: "",
            issue_type: "task",
            status: "todo",
            title: "",
            body: "",
            labels: &[],
            strategy: None,
            relates: &[],
            blocks: &[],
            created: 0,
            updated: 0,
            closed: None,
        }
    }
}

impl Issue<'_> {
    pub fn path(&self) -> String {
        format!(".atelier/issues/{}.md", self.id)
    }

    pub fn text(&self) -> String {
        let mut text = format!(
            "---\ncreated_at: \"{}\"\nid: \"{}\"\nissue_type: \"{}\"\n",
            Repo::at(self.created),
            self.id,
            self.issue_type
        );
        if self.labels.is_empty() {
            text.push_str("labels: []\n");
        } else {
            text.push_str("labels:\n");
            for label in self.labels {
                text.push_str(&format!("- \"{label}\"\n"));
            }
        }
        if let Some(path) = self.strategy {
            text.push_str(&format!("strategy:\n  path: {path}\n"));
        }
        text.push_str("priority: \"P1\"\nrelationships:\n");
        if self.blocks.is_empty() {
            text.push_str("  blocks: []\n");
        } else {
            text.push_str("  blocks:\n");
            for id in self.blocks {
                text.push_str(&format!("  - kind: \"issue\"\n    id: \"{id}\"\n"));
            }
        }
        text.push_str("  children: []\n  attachments: []\n");
        if self.relates.is_empty() {
            text.push_str("  relates: []\n");
        } else {
            text.push_str("  relates:\n");
            for (id, kind) in self.relates {
                text.push_str(&format!(
                    "  - kind: \"issue\"\n    id: \"{id}\"\n    type: \"{kind}\"\n"
                ));
            }
        }
        text.push_str("schema: \"atelier.issue\"\nschema_version: 2\n");
        if let Some(closed) = self.closed {
            text.push_str(&format!("closed_at: \"{}\"\n", Repo::at(closed)));
        }
        text.push_str(&format!(
            "status: \"{}\"\ntitle: \"{}\"\nupdated_at: \"{}\"\n---\n\n{}\n",
            self.status,
            self.title,
            Repo::at(self.updated),
            self.body.trim_end()
        ));
        text
    }

    pub fn write(&self, repo: &Repo) {
        repo.write(&self.path(), &self.text());
    }
}

/// An evidence receipt on `target`, pinned to `commit`, judging `criteria`
/// of `mission`.
pub fn receipt(
    id: &str,
    target: &str,
    commit: &str,
    result: &str,
    criteria: &[(&str, &str)],
    mission: &str,
    minutes: i64,
) -> String {
    let mut judged = String::from("criteria:\n");
    for (slug, result) in criteria {
        judged.push_str(&format!("- slug: \"{slug}\"\n  result: \"{result}\"\n"));
    }
    format!(
        "---\ncreated_at: \"{at}\"\nid: \"{id}\"\nevidence_type: \"test\"\ncommit: \"{commit}\"\nresult: \"{result}\"\ncaptured_at: \"{at}\"\ncommand: \"cargo test\"\nexit_status: \"{exit}\"\nagent_identity: \"fixture-validator\"\ntarget:\n  kind: \"issue\"\n  id: \"{target}\"\n  role: \"validates\"\n{judged}criteria_mission: \"{mission}\"\nrelationships:\n  blocks: []\n  children: []\n  attachments:\n  - kind: \"issue\"\n    id: \"{target}\"\n    role: \"validates\"\n  relates: []\nschema: \"atelier.evidence\"\nschema_version: 2\nstatus: \"recorded\"\ntitle: \"Fixture tests pass\"\nupdated_at: \"{at}\"\n---\n\n## Summary\n\nFixture tests pass\n\n## Command\n\n```console\ncargo test\n```\n\nExit status: {exit}\n\n## Stdout\n\nBytes: 0\nTruncated: no\n\n```text\n```\n\n## Stderr\n\nBytes: 0\nTruncated: no\n\n```text\n```\n",
        at = Repo::at(minutes),
        exit = if result == "pass" { 0 } else { 1 },
    )
}

/// An approval of `target` at `commit`.
pub fn approval(id: &str, target: &str, commit: &str, minutes: i64) -> String {
    format!(
        "---\nschema: atelier.review\nschema_version: 1\nid: {id}\ntitle: approval on {target}\ntarget: {target}\nidentity: fixture-reviewer\ncommit: {commit}\ncreated_at: {}\nevent:\n  event: approval\n---\n\nLooks right.\n",
        Repo::at(minutes)
    )
}

/// The starter policy with `base` as its base branch.
pub fn policy(base: &str) -> String {
    atelier_workflow::starter_policy_yaml(base)
}

/// The starter policy, except that a mission starts `proposed` and moves to
/// `ready`, both in the `todo` category, before it starts.
pub fn proposed_policy(base: &str) -> String {
    let policy = policy(base);
    let (head, rest) = policy.split_once("  objective:\n").unwrap();
    let (objective, tail) = rest.split_once("  work:\n").unwrap();
    let objective = objective
        .replace("initial_status: draft", "initial_status: proposed")
        .replace(
            "      ready:\n        from: [draft]\n        to: todo",
            "      plan:\n        from: [proposed]\n        to: ready",
        )
        .replace(
            "      start:\n        from: [todo]",
            "      start:\n        from: [ready]",
        )
        .replace(
            "[draft, todo, in_progress]",
            "[proposed, ready, in_progress]",
        );
    let head = head.replace(
        "statuses:\n",
        "statuses:\n  proposed:\n    category: todo\n  ready:\n    category: todo\n",
    );
    format!("{head}  objective:\n{objective}  work:\n{tail}")
}

/// Repository `alpha`: a draft mission with a strategy and a done mission
/// without one, with receipts, an approval and code commits between them.
/// Returns the repository and its commits by name.
pub fn alpha(dir: &Path) -> (Repo, Vec<(&'static str, String)>) {
    let repo = Repo::init(dir, "alpha", "main", &policy("main"));
    let mut commits = Vec::new();
    repo.write(
        "docs/strategies/one.md",
        "# Why\n\nPlans go unread.\n\n# Outcome\n\nEvery plan is read on the base.\n\n# Success Criteria\n\n- shown: The brief shows the plan.\n- current: The page reads the base branch.\n\n# Sizing\n\n- Complexity: low.\n\n# Derisking\n\nRead one commit first.\n\n# Risks and Unknowns\n\n- The base may move.\n",
    );
    Issue {
        id: "alpha-m1",
        issue_type: "mission",
        status: "draft",
        title: "Read plans on the base branch",
        body: "## Outcome\n\nEvery plan is read on the base.",
        strategy: Some("docs/strategies/one.md"),
        ..Issue::default()
    }
    .write(&repo);
    Issue {
        id: "alpha-a1",
        title: "Load records from a commit",
        body: "## Goal\n\nLoad them.\n\n## Done when\n\n- [ ] They load.\n\n## Unknowns\n\nWhether it is fast.",
        labels: &["size-m"],
        relates: &[("alpha-m1", "advances")],
        ..Issue::default()
    }
    .write(&repo);
    Issue {
        id: "alpha-a2",
        issue_type: "decision",
        title: "Choose the server",
        body: "## Question\n\nWhich server?",
        relates: &[("alpha-m1", "advances")],
        ..Issue::default()
    }
    .write(&repo);
    let mission_two = |status: &str, updated: i64, closed: Option<i64>, report: &str| {
        Issue {
            id: "alpha-m2",
            issue_type: "mission",
            status,
            title: "Ship the loader",
            body: &format!(
                "## Outcome\n\nThe loader ships.\n\n## Success criteria\n\n- works: The loader reads every record.\n{report}"
            ),
            updated,
            closed,
            ..Issue::default()
        }
        .text()
    };
    repo.write(
        ".atelier/issues/alpha-m2.md",
        &mission_two("draft", 0, None, ""),
    );
    let task = |status: &str, updated: i64, closed: Option<i64>, report: &str| {
        Issue {
            id: "alpha-b1",
            status,
            title: "Write the loader",
            body: &format!(
                "## Goal\n\nWrite it.\n\n## Done when\n\n- [x] It reads records.\n{report}"
            ),
            relates: &[("alpha-m2", "advances")],
            updated,
            closed,
            ..Issue::default()
        }
        .text()
    };
    repo.write(".atelier/issues/alpha-b1.md", &task("todo", 0, None, ""));
    commits.push(("plan", repo.commit("Plan both missions")));
    repo.write(
        ".atelier/issues/alpha-m2.md",
        &mission_two("todo", 10, None, ""),
    );
    commits.push(("ready", repo.commit("Ready alpha-m2")));
    repo.write(
        ".atelier/issues/alpha-m2.md",
        &mission_two("in_progress", 20, None, ""),
    );
    commits.push(("started", repo.commit("Start alpha-m2")));
    repo.write("src/loader.txt", "the loader\n");
    let code = repo.commit("Write the loader");
    commits.push(("code", code.clone()));
    repo.write(
        ".atelier/issues/alpha-b1.md",
        &task(
            "done",
            40,
            Some(40),
            "\n## Report\n\nThe loader reads every record.\n\n### Deviations\n\nNone worth naming.",
        ),
    );
    repo.write(
        ".atelier/evidence/alpha-e1.md",
        &receipt(
            "alpha-e1",
            "alpha-b1",
            &code,
            "pass",
            &[("works", "pass")],
            "alpha-m2",
            35,
        ),
    );
    repo.write(
        ".atelier/evidence/alpha-e2.md",
        &receipt(
            "alpha-e2",
            "alpha-m1",
            &code,
            "pass",
            &[("shown", "pass")],
            "alpha-m1",
            36,
        ),
    );
    repo.write(
        ".atelier/reviews/alpha-r1.md",
        &approval("alpha-r1", "alpha-b1", &code, 38),
    );
    commits.push(("delivered", repo.commit("Merge alpha-b1")));
    repo.write("src/loader.txt", "the loader, faster\n");
    commits.push(("later-code", repo.commit("Speed up the loader")));
    repo.write(
        ".atelier/issues/alpha-m2.md",
        &mission_two(
            "done",
            60,
            Some(60),
            "\n## Report\n\nThe loader shipped and reads every record.",
        ),
    );
    commits.push(("closed", repo.commit("Close alpha-m2")));
    (repo, commits)
}

/// Repository `beta`: a mission workflow whose initial status is
/// `proposed`, with one mission still proposed and one moved on to
/// `ready`, in the same category.
pub fn beta(dir: &Path) -> Repo {
    let repo = Repo::init(dir, "beta", "main", &proposed_policy("main"));
    Issue {
        id: "beta-m1",
        issue_type: "mission",
        status: "proposed",
        title: "Propose the board",
        body: "## Outcome\n\nA board.\n\n## Success criteria\n\n- listed: The board lists work.",
        ..Issue::default()
    }
    .write(&repo);
    Issue {
        id: "beta-m2",
        issue_type: "mission",
        status: "ready",
        title: "Build the inbox",
        body: "## Outcome\n\nAn inbox.\n\n## Success criteria\n\n- queued: The inbox queues asks.\n- linked: Each ask links its page.",
        updated: 5,
        ..Issue::default()
    }
    .write(&repo);
    Issue {
        id: "beta-c1",
        title: "List the asks",
        body: "## Goal\n\nList them.\n\n## Done when\n\n- [ ] They are listed.",
        relates: &[("beta-m2", "advances")],
        ..Issue::default()
    }
    .write(&repo);
    repo.commit("Plan beta");
    repo
}
