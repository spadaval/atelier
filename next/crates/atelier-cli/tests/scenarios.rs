//! The new binary driven in scratch repositories, one thread at a time
//! through `T3CODE_THREAD_ID`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};
use tempfile::TempDir;

const ID: &str = "demo-ab12";

struct Scratch {
    dir: TempDir,
}

impl Scratch {
    /// A repository on `main` whose one commit holds `items`, each an ID
    /// with its status.
    fn new(items: &[(&str, &str)]) -> Scratch {
        let scratch = Scratch {
            dir: TempDir::new().unwrap(),
        };
        let root = scratch.root();
        git(&root, &["init", "-q", "-b", "main"]);
        fs::create_dir_all(root.join(".atelier")).unwrap();
        fs::write(
            root.join(".atelier/config.toml"),
            "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n",
        )
        .unwrap();
        for (id, status) in items {
            write_item(&root, id, status, &[], &[], "");
        }
        git(&root, &["add", "-A"]);
        git(&root, &["commit", "-qm", "Plan"]);
        scratch
    }

    fn root(&self) -> PathBuf {
        self.dir.path().join("repo")
    }

    /// A second worktree of the repository, detached at `main`.
    fn worktree(&self, name: &str) -> PathBuf {
        let path = self.dir.path().join(name);
        git(
            &self.root(),
            &[
                "worktree",
                "add",
                "-q",
                "--detach",
                path.to_str().unwrap(),
                "main",
            ],
        );
        path
    }
}

fn item_path(id: &str) -> String {
    format!(".atelier/items/{id}/item.md")
}

fn write_item(
    checkout: &Path,
    id: &str,
    status: &str,
    implementers: &[&str],
    blocked_by: &[&str],
    report: &str,
) {
    let list = |values: &[&str]| -> String {
        if values.is_empty() {
            " []".into()
        } else {
            values.iter().map(|v| format!("\n- {v}")).collect()
        }
    };
    let path = checkout.join(item_path(id));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!(
            "---\nschema: atelier.item\nschema_version: 3\nid: {id}\ntitle: Say hello\n\
             status: {status}\nmission: null\nlabels: []\npriority: high\nblocked_by:{}\n\
             implementers:{}\nready: null\ntrivial: null\ncaused_by: null\n\
             created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nPrint hello.\n\n## Done when\n\n\
             - [ ] `hello` prints hello.\n- [ ] A test covers it.\n\n## Report\n\n{report}",
            list(blocked_by),
            list(implementers)
        ),
    )
    .unwrap();
}

fn git(at: &Path, args: &[&str]) -> String {
    let output = git_output(at, args);
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn git_output(at: &Path, args: &[&str]) -> Output {
    if !at.exists() {
        fs::create_dir_all(at).unwrap();
    }
    identity(Command::new("git").current_dir(at).args(args))
        .output()
        .unwrap()
}

fn identity(command: &mut Command) -> &mut Command {
    command
        .env("GIT_AUTHOR_NAME", "b")
        .env("GIT_AUTHOR_EMAIL", "b@b")
        .env("GIT_COMMITTER_NAME", "b")
        .env("GIT_COMMITTER_EMAIL", "b@b")
}

fn atelier_command(at: &Path, thread: &str, args: &[&str]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_atelier"));
    command.current_dir(at).args(args);
    for variable in [
        "CLAUDE_CODE_SESSION_ID",
        "CODEX_SESSION_ID",
        "GROK_SESSION_ID",
    ] {
        command.env_remove(variable);
    }
    command.env("T3CODE_THREAD_ID", thread);
    command.env_remove("ATELIER_TEST_HOLD_BEFORE_BRANCH");
    identity(&mut command);
    command
}

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn atelier(at: &Path, thread: &str, args: &[&str]) -> Run {
    let output = atelier_command(at, thread, args).output().unwrap();
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

/// A `start` that stops between its commit and its branch, until released.
struct Held {
    child: Child,
    release: PathBuf,
}

fn start_held(at: &Path, thread: &str, release: PathBuf) -> Held {
    let child = atelier_command(at, thread, &["start", ID])
        .env("ATELIER_TEST_HOLD_BEFORE_BRANCH", &release)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let held = release.with_extension("held");
    let since = Instant::now();
    while !held.exists() {
        assert!(
            since.elapsed() < Duration::from_secs(30),
            "start never held"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    Held { child, release }
}

/// The item's front-matter field `key` as `rev` holds it; a list's entries
/// are joined by spaces.
fn field(at: &Path, rev: &str, key: &str) -> String {
    let text = git(at, &["show", &format!("{rev}:{}", item_path(ID))]);
    let mut lines = text
        .lines()
        .skip_while(|line| !line.starts_with(&format!("{key}:")));
    let value = lines.next().unwrap()[key.len() + 1..].trim().to_string();
    if !value.is_empty() {
        return value;
    }
    let entries: Vec<&str> = lines.map_while(|line| line.strip_prefix("- ")).collect();
    entries.join(" ")
}

fn branch_exists(at: &Path, name: &str) -> bool {
    git_output(
        at,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/{name}"),
        ],
    )
    .status
    .success()
}

#[test]
fn start_commits_building_then_creates_the_branch_and_status_says_what_is_next() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let base = git(&root, &["rev-parse", "main"]);

    let run = atelier(&root, "t1", &["start", ID]);
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);

    // One commit on top of the base, changing the item alone, which the
    // branch points at and the checkout is now on.
    assert_eq!(git(&root, &["rev-parse", &format!("{ID}^")]), base);
    assert_eq!(git(&root, &["rev-parse", "main"]), base);
    assert_eq!(
        git(&root, &["diff", "--name-only", "main", ID]),
        item_path(ID)
    );
    assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), ID);
    assert_eq!(field(&root, ID, "status"), "building");
    assert_eq!(field(&root, ID, "implementers"), "t1");
    assert_eq!(git(&root, &["status", "--porcelain"]), "");

    let status = atelier(&root, "t1", &["status"]);
    assert_eq!(status.code, 0);
    assert_eq!(
        status.stdout,
        format!(
            "You are the implementer of {ID} (thread t1).\nItem: {}\nTitle: Say hello\n\
             Status: building\nTo submit it for review:\n  [ ] Done when: `hello` prints hello.\n\
             \x20 [ ] Done when: A test covers it.\n  [ ] Report: written under `## Report`\n\
             Then: atelier submit\n",
            item_path(ID)
        )
    );
    // `start` prints the same status after what it did.
    assert_eq!(
        run.stdout,
        format!("Started {ID} on branch {ID}.\n\n{}", status.stdout)
    );
}

#[test]
fn submit_refuses_with_the_status_list_and_then_commits_the_report_in_review() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let head = git(&root, &["rev-parse", "HEAD"]);
    let status = atelier(&root, "t1", &["status"]).stdout;
    let list: Vec<&str> = status
        .lines()
        .skip_while(|l| !l.starts_with("To "))
        .collect();

    let refused = atelier(&root, "t1", &["submit"]);
    assert_eq!(refused.code, 1);
    let said: Vec<&str> = refused.stderr.lines().skip(1).collect();
    assert_eq!(said, list);
    assert_eq!(
        refused.stderr.lines().next(),
        Some("`atelier submit` refused; nothing changed.")
    );
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    assert_eq!(git(&root, &["status", "--porcelain"]), "");

    // Both lines ticked, the Report still empty: that alone is listed.
    let path = root.join(item_path(ID));
    let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, &ticked).unwrap();
    let refused = atelier(&root, "t1", &["submit"]);
    assert_eq!(refused.code, 1);
    assert!(refused
        .stderr
        .contains("  [ ] Report: written under `## Report`\n"));
    assert!(!refused.stderr.contains("Done when:"));
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    assert_eq!(fs::read_to_string(&path).unwrap(), ticked);

    fs::write(
        &path,
        format!("{ticked}It prints hello, and a test covers it.\n"),
    )
    .unwrap();
    let submitted = atelier(&root, "t1", &["submit"]);
    assert_eq!(submitted.code, 0, "{}", submitted.stderr);
    assert_eq!(
        submitted.stdout,
        format!(
            "Submitted {ID} for review.\nStart a reviewer with: Run `atelier review {ID}` and follow what it prints.\n"
        )
    );
    // One commit, holding the status and the Report; nothing left over.
    assert_eq!(git(&root, &["rev-parse", "HEAD^"]), head);
    assert_eq!(field(&root, "HEAD", "status"), "in_review");
    assert!(git(&root, &["show", &format!("HEAD:{}", item_path(ID))])
        .contains("It prints hello, and a test covers it."));
    assert_eq!(git(&root, &["status", "--porcelain"]), "");

    // In review, the implementer waits, and a second submit is refused.
    let waiting = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        waiting.contains("Status: in review\nWaiting for a reviewer's verdict."),
        "{waiting}"
    );
    let again = atelier(&root, "t1", &["submit"]);
    assert_eq!(again.code, 1);
    assert!(again.stderr.starts_with(&format!(
        "{ID} is in review, and `atelier submit` runs on an item that is building, changes requested, approved."
    )));
}

#[test]
fn start_on_a_draft_changes_nothing_and_names_ready() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let head = git(&root, &["rev-parse", "HEAD"]);
    let run = atelier(&root, "t1", &["start", ID]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "{ID} is draft, and `atelier start` runs on an item that is ready. Nothing changed.\n\
             It is a draft. A planner makes it ready with: atelier ready {ID} --by <name>\n"
        )
    );
    assert!(!branch_exists(&root, ID));
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), "main");
}

#[test]
fn another_thread_takes_over_a_started_item_in_any_started_status() {
    for status in ["building", "in_review", "changes_requested", "approved"] {
        let scratch = Scratch::new(&[(ID, "ready")]);
        let root = scratch.root();
        // The fixture writes the started item on its branch.
        git(&root, &["switch", "-q", "-c", ID]);
        write_item(&root, ID, status, &["t1"], &[], "");
        git(&root, &["commit", "-qam", "Started"]);
        git(&root, &["switch", "-q", "main"]);
        let tip = git(&root, &["rev-parse", ID]);

        let run = atelier(&root, "t2", &["start", ID]);
        assert_eq!(run.code, 0, "{status}: {}", run.stderr);
        assert!(
            run.stdout
                .starts_with(&format!("Took over {ID}; t1 held it.\n")),
            "{}",
            run.stdout
        );
        assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), ID);
        assert_eq!(git(&root, &["rev-parse", "HEAD^"]), tip);
        assert_eq!(field(&root, "HEAD", "implementers"), "t1 t2");
        assert_eq!(field(&root, "HEAD", "status"), status);
        assert_eq!(git(&root, &["status", "--porcelain"]), "");

        // Running it again as the holder changes nothing.
        let head = git(&root, &["rev-parse", "HEAD"]);
        let again = atelier(&root, "t2", &["start", ID]);
        assert_eq!(again.code, 0);
        assert!(again.stdout.starts_with(&format!("You already hold {ID}")));
        assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    }
}

#[test]
fn of_two_threads_starting_one_item_from_two_worktrees_one_creates_the_branch() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let other = scratch.worktree("other");
    let base = git(&root, &["rev-parse", "main"]);

    // t2 has read the item ready and written its commit when t1 starts it.
    let held = start_held(&other, "t2", scratch.dir.path().join("release"));
    let first = atelier(&root, "t1", &["start", ID]);
    assert_eq!(first.code, 0, "{}", first.stderr);
    fs::write(&held.release, "").unwrap();
    let second = held.child.wait_with_output().unwrap();

    assert_eq!(second.status.code(), Some(1));
    assert_eq!(
        String::from_utf8(second.stderr).unwrap(),
        format!(
            "t1 started {ID} first and holds it. Nothing changed.\n\
             To take it over, run `atelier start {ID}` again.\n"
        )
    );
    assert_eq!(field(&root, ID, "implementers"), "t1");
    assert_eq!(git(&root, &["rev-parse", &format!("{ID}^")]), base);
    assert_eq!(git(&other, &["rev-parse", "HEAD"]), base);
    assert!(git_output(&other, &["symbolic-ref", "-q", "HEAD"])
        .stdout
        .is_empty());
    assert_eq!(git(&other, &["status", "--porcelain"]), "");
}

#[test]
fn a_start_stopped_before_its_branch_leaves_the_item_ready() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let mut held = start_held(&root, "t1", scratch.dir.path().join("release"));
    held.child.kill().unwrap();
    held.child.wait().unwrap();

    assert!(!branch_exists(&root, ID));
    assert_eq!(field(&root, "main", "status"), "ready");
    assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), "main");
    assert_eq!(git(&root, &["status", "--porcelain"]), "");
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.contains(&format!("Ready to start:\n  {ID} (high): Say hello\n")),
        "{status}"
    );

    let run = atelier(&root, "t1", &["start", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(field(&root, ID, "status"), "building");
}

#[test]
fn status_on_the_base_reads_a_started_item_from_its_branch_until_it_merges() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let base = scratch.dir.path().join("base");
    git(
        &root,
        &["worktree", "add", "-q", base.to_str().unwrap(), "main"],
    );

    let status = atelier(&base, "t3", &["status"]);
    assert_eq!(
        status.stdout,
        format!(
            "You are thread t3, on the base branch main.\nHeld by other threads:\n  \
             {ID} (building, held by t1): Say hello\nNothing is ready to start.\n"
        )
    );
    // The thread that holds it is pointed back to it.
    let held = atelier(&base, "t1", &["status"]).stdout;
    assert!(
        held.contains(&format!("You hold:\n  {ID} (building): Say hello\n")),
        "{held}"
    );
    assert!(held.ends_with(&format!(
        "Then: atelier start {ID} (switches to its branch)\n"
    )));

    // Merged, with the base's record moved on: the base's record counts,
    // though the branch still says building.
    git(&base, &["merge", "-q", "--no-ff", "-m", "Merge", ID]);
    write_item(&base, ID, "in_review", &["t1", "t9"], &[], "");
    git(&base, &["commit", "-qam", "Base moves on"]);
    let status = atelier(&base, "t3", &["status"]).stdout;
    assert!(
        status.contains(&format!("  {ID} (in review, held by t9): Say hello\n")),
        "{status}"
    );
}

#[test]
fn a_done_or_dropped_record_on_the_base_wins_over_the_branch() {
    for terminal in ["dropped", "done"] {
        let scratch = Scratch::new(&[(ID, "ready")]);
        let root = scratch.root();
        assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
        let base = scratch.dir.path().join("base");
        git(
            &root,
            &["worktree", "add", "-q", base.to_str().unwrap(), "main"],
        );
        write_item(&base, ID, terminal, &[], &[], "");
        git(&base, &["commit", "-qam", terminal]);

        let status = atelier(&root, "t1", &["status"]).stdout;
        assert!(
            status.contains(&format!("Status: {terminal}\n")),
            "{status}"
        );
        let submit = atelier(&root, "t1", &["submit"]);
        assert_eq!(submit.code, 1);
        assert!(
            submit.stderr.starts_with(&format!("{ID} is {terminal}")),
            "{}",
            submit.stderr
        );
        let take = atelier(&base, "t2", &["start", ID]);
        assert_eq!(take.code, 1);
        assert_eq!(field(&root, ID, "implementers"), "t1");
        // Nobody is offered the item.
        let listed = atelier(&base, "t3", &["status"]).stdout;
        assert!(!listed.contains(ID), "{listed}");
    }
}

#[test]
fn start_refuses_a_ready_item_whose_blocker_is_not_done_or_whose_branch_exists() {
    let scratch = Scratch::new(&[("demo-b10c", "ready")]);
    let root = scratch.root();
    write_item(&root, ID, "ready", &[], &["demo-b10c"], "");
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Blocked item"]);

    let status = atelier(&root, "t1", &["status"]).stdout;
    assert_eq!(
        status,
        format!(
            "You are thread t1, on the base branch main.\nReady to start:\n  \
             demo-b10c (high): Say hello\n  {ID} (high): Say hello\n    \
             [ ] demo-b10c is done (it is ready)\nThen: atelier start demo-b10c\n"
        )
    );
    let run = atelier(&root, "t1", &["start", ID]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier start {ID}` refused; nothing changed.\nTo start it:\n  \
             [ ] demo-b10c is done (it is ready)\nThen: atelier start {ID}\n"
        )
    );
    assert!(!branch_exists(&root, ID));

    // A branch of its name that holds no start of it.
    git(&root, &["branch", "demo-b10c"]);
    let run = atelier(&root, "t1", &["start", "demo-b10c"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.contains("[ ] No branch demo-b10c exists yet"),
        "{}",
        run.stderr
    );
    assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), "main");
}

#[test]
fn only_the_holder_submits_and_others_are_pointed_at_the_take_over() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let head = git(&root, &["rev-parse", "HEAD"]);

    let status = atelier(&root, "t2", &["status"]).stdout;
    assert_eq!(
        status,
        format!(
            "You are thread t2, on the branch of {ID}, which is held by t1.\nTitle: Say hello\n\
             Status: building\nThen: atelier start {ID}\n"
        )
    );
    let run = atelier(&root, "t2", &["submit"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "You (thread t2) do not hold {ID}; t1 does. Nothing changed.\nTo take it over: atelier start {ID}\n"
        )
    );
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);

    // Off any item's branch, submit says where to go.
    git(&root, &["switch", "-q", "--detach", "main"]);
    let run = atelier(&root, "t1", &["submit"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr
            .starts_with("`atelier submit` runs on an item's branch"),
        "{}",
        run.stderr
    );
}
