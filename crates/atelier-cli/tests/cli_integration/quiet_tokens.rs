//! `--quiet` prints one documented token per command, and nothing else on
//! stdout: the Quiet Tokens table in `docs/product/human-cli-output.md`
//! (atelier-pr4k). One fixture repository walks the lifecycle, running every
//! command with `-q`, and a final check fails when a command in the help tree
//! was not run.
use super::*;
use std::collections::BTreeSet;

struct Quiet<'a> {
    dir: &'a Path,
    covered: BTreeSet<String>,
}

impl<'a> Quiet<'a> {
    fn new(dir: &'a Path) -> Self {
        Self {
            dir,
            covered: BTreeSet::new(),
        }
    }

    fn output(&mut self, command: &str, args: &[&str]) -> (bool, String, String) {
        self.covered.insert(command.to_string());
        let mut full = vec!["-q"];
        full.extend(command.split(' '));
        full.extend(args);
        run_atelier_raw(self.dir, &full)
    }

    /// Stdout of a command that must succeed.
    fn ok(&mut self, command: &str, args: &[&str]) -> String {
        let (success, stdout, stderr) = self.output(command, args);
        assert!(success, "atelier -q {command} {args:?} failed: {stderr}");
        stdout
    }

    /// Stderr of a command that must fail with nothing on stdout.
    fn refused(&mut self, command: &str, args: &[&str]) -> String {
        let (success, stdout, stderr) = self.output(command, args);
        assert!(!success, "atelier -q {command} {args:?} passed: {stdout}");
        assert_eq!(stdout, "", "a quiet refusal prints nothing on stdout");
        stderr
    }
}

/// Every line is one record ID.
fn assert_ids(stdout: &str) {
    for line in stdout.lines() {
        assert!(is_record_id(line), "not an ID line {line:?} in:\n{stdout}");
    }
}

/// One line of `key=value` pairs whose values are all numbers.
fn assert_counts(stdout: &str, keys: &[&str]) {
    assert_eq!(stdout.lines().count(), 1, "{stdout}");
    let found: Vec<&str> = stdout
        .split_whitespace()
        .map(|pair| {
            let (key, value) = pair.split_once('=').unwrap_or_else(|| panic!("{stdout}"));
            assert!(value.parse::<usize>().is_ok(), "{pair} in {stdout}");
            key
        })
        .collect();
    assert_eq!(found, keys, "{stdout}");
}

/// Leaf command paths in the help tree, such as `issue show`.
fn leaf_commands(dir: &Path, path: &[&str]) -> Vec<String> {
    let mut args = path.to_vec();
    args.push("--help");
    let (success, help, stderr) = run_atelier_raw(dir, &args);
    assert!(success, "{args:?}: {stderr}");
    let Some(section) = help.split("\nCommands:\n").nth(1) else {
        return vec![path.join(" ")];
    };
    section
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .filter(|name| *name != "help")
        .flat_map(|name| {
            let mut child = path.to_vec();
            child.push(name);
            leaf_commands(dir, &child)
        })
        .collect()
}

const BODY: &str = "## Goal\n\nShip it.\n\n## Done when\n\nIt ships.";

#[test]
fn every_command_prints_its_documented_quiet_token() {
    let dir = tempdir().unwrap();
    let dir = dir.path();
    init_git_repo(dir);
    let mut q = Quiet::new(dir);

    assert_eq!(q.ok("init", &["--slug", "atelier"]), "atelier\n");
    assert_eq!(q.ok("init", &["--force"]), "atelier\n");
    let settings = q.ok("setup", &["claude"]);
    assert!(
        settings.trim_end().ends_with(".claude/settings.json"),
        "{settings}"
    );
    assert_eq!(settings.lines().count(), 1, "{settings}");

    // Records, each create printing its ID.
    let create = |q: &mut Quiet, args: &[&str]| {
        let id = q.ok("issue create", args);
        assert_ids(&id);
        assert_eq!(id.lines().count(), 1, "{id}");
        id.trim().to_string()
    };
    let mission = create(
        &mut q,
        &[
            "Mission",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nDone.\n\n## Success criteria\n\nIt works.",
        ],
    );
    let owner = create(&mut q, &["Owner", "--body", BODY]);
    let other = create(&mut q, &["Other", "--body", BODY]);
    let parent = create(&mut q, &["Parent", "--body", BODY]);
    let child = create(&mut q, &["Child", "--parent", &parent, "--body", BODY]);
    let bare = create(&mut q, &["No sections"]);

    // Mutations print the ID they were given first, whichever record holds
    // the link.
    let line = |id: &str| format!("{id}\n");
    let link = ["--role", "advances"];
    assert_eq!(
        q.ok("issue link", &[&owner, &mission, link[0], link[1]]),
        line(&owner)
    );
    assert_eq!(
        q.ok("issue link", &[&other, &owner, "--role", "blocked_by"]),
        line(&other)
    );
    assert_eq!(
        q.ok("issue unlink", &[&other, &owner, "--role", "blocked_by"]),
        line(&other)
    );
    assert_eq!(
        q.ok("issue link", &[&owner, &other, "--role", "blocks"]),
        line(&owner)
    );
    assert_eq!(
        q.ok("issue update", &[&owner, "--priority", "high"]),
        line(&owner)
    );
    assert_eq!(q.ok("issue note", &[&owner, "A note"]), line(&owner));

    assert_eq!(q.ok("issue show", &[&owner]), format!("{owner} todo\n"));
    assert_eq!(
        q.ok("issue show", &[&owner, &other, "--full"]),
        format!("{owner} todo\n{other} todo\n")
    );
    let listed = q.ok("issue list", &[]);
    assert_ids(&listed);
    assert!(listed.lines().any(|id| id == owner), "{listed}");
    // Allowed names only, counting one that needs only `--reason`.
    let transitions = q.ok("issue transition", &[&owner]);
    assert_eq!(transitions, "block\ndefer\nstart\nsupersede\n");
    let blocked = q.ok("issue transition", &[&other]);
    assert!(!blocked.lines().any(|name| name == "start"), "{blocked}");

    assert_eq!(q.ok("check", &[]), "passed\n");
    let findings = q.refused("check", &[&bare]);
    assert!(findings.contains("Goal must be present"), "{findings}");
    assert_eq!(
        q.ok("issue transition", &[&bare, "supersede", "--reason", "dup"]),
        "superseded\n"
    );

    // Views: IDs, or counts as numbers.
    assert_counts(
        &q.ok("status", &[]),
        &["work", "current_missions", "ready", "human"],
    );
    let (success, prime, stderr) = run_atelier_raw(dir, &["-q", "prime"]);
    assert!(success && stderr.contains("deprecated"), "{stderr}");
    assert_counts(&prime, &["work", "current_missions", "ready", "human"]);
    let views = run_atelier_raw(dir, &["-q", "work"]).1;
    assert!(views.lines().any(|view| view == "ready"), "{views}");
    assert!(views.lines().all(|view| !view.contains(' ')), "{views}");
    for view in ["ready", "blocked", "human", "active", "all", "missions"] {
        let path = format!("work {view}");
        assert_ids(&q.ok(&path, &[]));
    }
    assert!(q.ok("work ready", &[]).lines().any(|id| id == owner));
    assert!(q.ok("work missions", &[]).lines().any(|id| id == mission));
    let counts = [
        "active", "ready", "human", "context", "blocked", "done", "backlog",
    ];
    assert_counts(&q.ok("work mission", &[&mission]), &counts);
    assert_eq!(q.ok("work mission", &[&mission, "--ready"]), line(&owner));
    let mut parent_counts = counts.to_vec();
    parent_counts.push("proof_gaps");
    assert_counts(&q.ok("work parent", &[&parent]), &parent_counts);

    // Lifecycle on the owner's branch.
    commit_all(dir, "plan");
    assert_eq!(q.ok("start", &[&owner]), format!("task/{owner}\n"));
    assert_eq!(git_current_branch(dir), format!("task/{owner}"));
    let target = format!("issue/{owner}");
    let captured = q.ok(
        "evidence record",
        &[
            "--target",
            &target,
            "--kind",
            "test",
            "--summary",
            "s",
            "--",
            "true",
        ],
    );
    assert_ids(&captured);
    let evidence = captured.trim();
    let manual = q.ok(
        "evidence record",
        &[
            "--target", &target, "--kind", "test", "--result", "pass", "manual",
        ],
    );
    assert_ids(&manual);
    assert_eq!(
        q.ok("evidence show", &[evidence]),
        format!("{evidence} pass\n")
    );
    assert_eq!(
        q.ok("evidence attach", &[evidence, "issue", &other]),
        line(evidence)
    );
    let evidence_ids = q.ok("evidence list", &[]);
    assert_ids(&evidence_ids);
    assert_eq!(evidence_ids.lines().count(), 2, "{evidence_ids}");
    commit_all(dir, "implementation and proof");
    assert_eq!(
        q.ok("issue transition", &[&owner, "close", "--reason", "done"]),
        "done\n"
    );
    commit_all(dir, "done claim");

    // A failed gate prints nothing on stdout and says why on stderr.
    let refusal = q.refused("merge", &[&owner, "--check"]);
    assert!(refusal.contains("FAIL"), "{refusal}");

    let review = |q: &mut Quiet, command: &str, args: &[&str]| {
        let id = q.ok(command, args);
        assert_ids(&id);
        assert_eq!(id.lines().count(), 1, "{id}");
        id.trim().to_string()
    };
    let who = ["--identity", "Reviewer"];
    let account = review(
        &mut q,
        "review record",
        &[&owner, who[0], who[1], "Read it"],
    );
    let finding = review(
        &mut q,
        "review finding",
        &[
            &owner,
            who[0],
            who[1],
            "--recommendation",
            "no_action",
            "Nit",
        ],
    );
    review(
        &mut q,
        "review disposition",
        &[
            &finding,
            who[0],
            who[1],
            "--decision",
            "no_action",
            "Fine as is",
        ],
    );
    review(
        &mut q,
        "review trivial",
        &[&owner, who[0], who[1], "One line"],
    );
    review(
        &mut q,
        "review approve",
        &[&owner, who[0], who[1], "Approved"],
    );
    let events = q.ok("review list", &[]);
    assert_ids(&events);
    assert_eq!(events.lines().count(), 5, "{events}");
    assert_eq!(q.ok("review list", &[&owner]), events);
    assert_eq!(
        q.ok("review show", &[&account]),
        format!("{account} review\n")
    );
    assert_eq!(
        q.ok("review show", &[&finding]),
        format!("{finding} finding\n")
    );
    let (success, _, stderr) = run_atelier_raw(dir, &["-q", "review", "list", "--json"]);
    assert!(
        !success && stderr.contains("'--quiet' cannot be used with '--json'"),
        "{stderr}"
    );

    // Gates: `passed`, or nothing and a non-zero exit.
    fs::write(dir.join(".atelier/issues/stray.md"), "stray").unwrap();
    q.refused("land", &[]);
    fs::remove_file(dir.join(".atelier/issues/stray.md")).unwrap();
    commit_all(dir, "reviews");
    assert_eq!(q.ok("land", &[]), "passed\n");
    assert_eq!(q.ok("doctor", &[]), "passed\n");
    assert_eq!(q.ok("merge", &[&owner, "--check"]), "passed\n");
    assert_eq!(q.ok("merge", &[&owner]), "passed\n");
    assert_eq!(git_current_branch(dir), "main");

    let pruned = q.ok("prune", &[]);
    assert!(pruned.starts_with("mode=dry-run eligible="), "{pruned}");
    // Nothing to migrate prints nothing; issues.rs pins a migrated ID.
    assert_eq!(q.ok("migrate epics", &[]), "");

    let fixture = workspace_root().join("docs/spec/bundle/fixtures/valid-bundle.json");
    let fixture = fixture.to_str().unwrap();
    assert_eq!(q.ok("bundle preview", &[fixture]), "passed\n");
    let applied = q.ok("bundle apply", &[fixture, "--yes"]);
    assert_ids(&applied);
    assert_eq!(applied.lines().count(), 5, "{applied}");

    let missing: Vec<String> = leaf_commands(dir, &[])
        .into_iter()
        .filter(|command| !q.covered.contains(command))
        .collect();
    assert!(missing.is_empty(), "no quiet token checked for {missing:?}");
}
