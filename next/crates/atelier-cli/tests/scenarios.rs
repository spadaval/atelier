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
             - [ ] `hello` prints hello.\n- [ ] A test covers it.\n\n## Scope\n\nOnly hello.\n\n\
             ## Report\n\n{report}",
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
    front(
        &git(at, &["show", &format!("{rev}:{}", item_path(ID))]),
        key,
    )
}

/// The front-matter field `key` of the record `text`, as [`field`] gives it.
fn front(text: &str, key: &str) -> String {
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

/// Installs a pre-commit hook that refuses every commit, and returns it.
fn refusing_hook(root: &Path) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let hook = root.join(git(root, &["rev-parse", "--git-path", "hooks/pre-commit"]));
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "#!/bin/sh\necho refused by the hook >&2\nexit 1\n").unwrap();
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
    hook
}

#[test]
fn a_refused_commit_leaves_submit_to_be_run_again() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let head = git(&root, &["rev-parse", "HEAD"]);
    let path = root.join(item_path(ID));
    let written =
        fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]") + "It prints hello.\n";
    fs::write(&path, &written).unwrap();
    fs::write(root.join("notes.txt"), "unrelated\n").unwrap();
    git(&root, &["add", "notes.txt"]);
    let hook = refusing_hook(&root);

    let run = atelier(&root, "t1", &["submit"]);
    assert_eq!(run.code, 2);
    assert!(
        run.stderr
            .contains("The commit failed, so the record is as it was."),
        "{}",
        run.stderr
    );
    assert!(run.stderr.contains("refused by the hook"), "{}", run.stderr);
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    assert_eq!(fs::read_to_string(&path).unwrap(), written);
    assert_eq!(
        git(&root, &["diff", "--cached", "--name-only"]),
        "notes.txt"
    );
    assert!(atelier(&root, "t1", &["status"])
        .stdout
        .contains("Status: building\n"));

    fs::remove_file(hook).unwrap();
    let run = atelier(&root, "t1", &["submit"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(field(&root, "HEAD", "status"), "in_review");
    assert_eq!(
        git(&root, &["show", "--format=", "--name-only", "HEAD"]),
        item_path(ID)
    );
    assert_eq!(
        git(&root, &["diff", "--cached", "--name-only"]),
        "notes.txt"
    );
}

#[test]
fn a_refused_commit_leaves_the_take_over_to_be_run_again() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    git(&root, &["switch", "-q", "-c", ID]);
    write_item(&root, ID, "building", &["t1"], &[], "");
    git(&root, &["commit", "-qam", "Started"]);
    git(&root, &["switch", "-q", "main"]);
    let tip = git(&root, &["rev-parse", ID]);
    let hook = refusing_hook(&root);

    let run = atelier(&root, "t2", &["start", ID]);
    assert_eq!(run.code, 2);
    assert!(
        run.stderr
            .contains("The commit failed, so the record is as it was."),
        "{}",
        run.stderr
    );
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), tip);
    assert_eq!(git(&root, &["status", "--porcelain"]), "");
    assert!(atelier(&root, "t2", &["status"])
        .stdout
        .contains("which is held by t1."));

    fs::remove_file(hook).unwrap();
    let run = atelier(&root, "t2", &["start", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(run
        .stdout
        .starts_with(&format!("Took over {ID}; t1 held it.\n")));
    assert_eq!(field(&root, "HEAD", "implementers"), "t1 t2");
}

#[test]
fn a_refusal_lists_the_conditions_of_the_act_refused() {
    // Approved, so `status` names merge next; submit still has its own
    // conditions, and its refusal lists them.
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    git(&root, &["switch", "-q", "-c", ID]);
    write_item(&root, ID, "approved", &["t1"], &[], "");
    git(&root, &["commit", "-qam", "Approved"]);
    let run = atelier(&root, "t1", &["submit"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        "`atelier submit` refused; nothing changed.\nTo submit it for review:\n  \
         [ ] Done when: `hello` prints hello.\n  [ ] Done when: A test covers it.\n  \
         [ ] Report: written under `## Report`\nThen: atelier submit\n"
    );
}

const REVIEWS: &str = ".atelier/items/demo-ab12/reviews";

/// The review files of the item in the checkout, by name.
fn reviews(root: &Path) -> Vec<String> {
    let Ok(listing) = fs::read_dir(root.join(REVIEWS)) else {
        return Vec::new();
    };
    let mut names: Vec<String> = listing
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

/// The one review file of `round`, as its stem and its path from the root.
fn review_of(root: &Path, round: u32) -> (String, String) {
    let found: Vec<String> = reviews(root)
        .into_iter()
        .filter(|name| name.starts_with(&format!("{round}-")))
        .collect();
    assert_eq!(found.len(), 1, "{found:?}");
    let stem = found[0].strip_suffix(".md").unwrap().to_string();
    assert_eq!(stem.len(), format!("{round}-").len() + 4, "{stem}");
    let path = format!("{REVIEWS}/{stem}.md");
    (stem, path)
}

fn read(root: &Path, path: &str) -> String {
    fs::read_to_string(root.join(path)).unwrap()
}

fn append(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    let before = fs::read_to_string(&path).unwrap();
    fs::write(path, before + text).unwrap();
}

/// Ticks the item's Done when lines, writes its Report and submits it as
/// `thread`.
fn finish_and_submit(root: &Path, thread: &str) {
    let path = root.join(item_path(ID));
    let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, ticked + "It prints hello.\n").unwrap();
    let run = atelier(root, thread, &["submit"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

/// A scratch repository whose item t1 started, changed and submitted.
fn submitted() -> Scratch {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "hello.txt"]);
    git(&root, &["commit", "-qm", "Say hello"]);
    finish_and_submit(&root, "t1");
    scratch
}

const NO_PERIOD: &str = "\n## Finding 1: No period\nRecommendation: fix now\n\
                         hello.txt says `hello`, without the Goal's period.\n\n\
                         ## Finding 2: No test\nRecommendation: defer\n";

#[test]
fn review_opens_the_rounds_file_and_prints_the_brief_and_run_again_only_the_brief() {
    let scratch = submitted();
    let root = scratch.root();
    let head = git(&root, &["rev-parse", "HEAD"]);

    let run = atelier(&root, "t2", &["review", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let (_, path) = review_of(&root, 1);
    let text = read(&root, &path);
    assert_eq!(front(&text, "round"), "1");
    assert_eq!(front(&text, "commit"), head);
    assert_eq!(front(&text, "reviewers"), "t2");
    assert_eq!(front(&text, "model"), "null");
    assert_eq!(front(&text, "verdict"), "null");
    assert!(
        text.ends_with(
            "---\n\n## Summary\n\n<!-- What you checked and how, and what you could not check. -->\n"
        ),
        "{text}"
    );
    // Written, not committed: `verdict` commits it.
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    assert_eq!(
        git(&root, &["status", "--porcelain"]),
        format!("?? {REVIEWS}/")
    );

    let brief = &run.stdout;
    assert!(
        brief.starts_with(&format!(
            "You are the reviewer of {ID} (thread t2), round 1.\n"
        )),
        "{brief}"
    );
    for part in [
        "## Goal\n\nPrint hello.\n",
        "## Done when\n\n- [x] `hello` prints hello.\n- [x] A test covers it.\n",
        "## Scope\n\nOnly hello.\n",
        "## Report\n\nIt prints hello.\n",
        &format!("  git diff main...{head}\n"),
        &format!("Your review file: {path}\n"),
        "  atelier verdict changes --model <your model>",
        "  atelier verdict approve --model <your model>",
    ] {
        assert!(brief.contains(part), "no `{part}` in:\n{brief}");
    }
    assert!(!brief.contains("since the last round"), "{brief}");

    // The same thread again: nothing new, and the same brief.
    let again = atelier(&root, "t2", &["review", ID]);
    assert_eq!(again.code, 0);
    assert_eq!(again.stdout, run.stdout);
    assert_eq!(reviews(&root).len(), 1);
    assert_eq!(read(&root, &path), text);

    let status = atelier(&root, "t2", &["status"]).stdout;
    assert_eq!(
        status,
        format!(
            "You are the reviewer of {ID} (thread t2), round 1.\nReview file: {path}\n\
             `atelier review {ID}` prints the brief again.\nTitle: Say hello\nStatus: in review\n\
             Then: atelier verdict approve (or `atelier verdict changes`, once your file holds a fix-now finding)\n"
        )
    );
}

#[test]
fn review_runs_on_an_item_in_review_in_the_checkout_on_its_branch() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let early = atelier(&root, "t2", &["review", ID]);
    assert_eq!(early.code, 1);
    assert!(
        early.stderr.starts_with(&format!(
            "{ID} is building, and `atelier review` runs on an item that is in review. Nothing changed.\n"
        )),
        "{}",
        early.stderr
    );

    finish_and_submit(&root, "t1");
    let base = scratch.dir.path().join("base");
    git(
        &root,
        &["worktree", "add", "-q", base.to_str().unwrap(), "main"],
    );
    let elsewhere = atelier(&base, "t2", &["review", ID]);
    assert_eq!(elsewhere.code, 1);
    assert_eq!(
        elsewhere.stderr,
        format!(
            "`atelier review {ID}` runs in the checkout on branch {ID}, where its implementer works, \
             and this checkout is on branch main. Nothing changed.\n\
             Run it there, or `git switch {ID}` here if no other checkout has that branch.\n"
        )
    );
    assert!(reviews(&root).is_empty() && reviews(&base).is_empty());
}

#[test]
fn an_author_past_or_present_is_shown_the_brief_and_opens_no_review() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    // t9 takes it over from t1, and submits it.
    assert_eq!(atelier(&root, "t9", &["start", ID]).code, 0);
    finish_and_submit(&root, "t9");
    let head = git(&root, &["rev-parse", "HEAD"]);

    for author in ["t1", "t9"] {
        let run = atelier(&root, author, &["review", ID]);
        assert_eq!(run.code, 1, "{author}: {}", run.stdout);
        assert!(
            run.stderr.starts_with(&format!(
                "You (thread {author}) have implemented {ID}, and an author does not review its own work. Nothing changed.\n"
            )),
            "{}",
            run.stderr
        );
        assert!(run.stderr.contains("## Goal\n\nPrint hello.\n"));
        assert!(run.stderr.contains(&format!("  git diff main...{head}\n")));
        assert!(!run.stderr.contains("Your review file"), "{}", run.stderr);
        assert!(reviews(&root).is_empty());
        assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
        assert_eq!(git(&root, &["status", "--porcelain"]), "");
    }

    // With a reviewer's file open, an author still changes no reviewer.
    assert_eq!(atelier(&root, "t2", &["review", ID]).code, 0);
    let (_, path) = review_of(&root, 1);
    let text = read(&root, &path);
    assert_eq!(atelier(&root, "t1", &["review", ID]).code, 1);
    assert_eq!(read(&root, &path), text);
    assert_eq!(reviews(&root).len(), 1);
}

#[test]
fn a_verdict_needs_its_finding_state_and_commits_the_file_with_the_status() {
    let scratch = submitted();
    let root = scratch.root();
    assert_eq!(atelier(&root, "t2", &["review", ID]).code, 0);
    let (stem, path) = review_of(&root, 1);
    let head = git(&root, &["rev-parse", "HEAD"]);

    // Nothing to fix: changes is refused.
    let opened = read(&root, &path);
    let refused = atelier(&root, "t2", &["verdict", "changes"]);
    assert_eq!(refused.code, 1);
    assert_eq!(
        refused.stderr,
        "`atelier verdict changes` refused; nothing changed.\nTo record your verdict:\n  \
         [ ] Your review file holds an open fix-now finding (it holds none; with nothing to fix, \
         run `atelier verdict approve`)\nThen: atelier verdict changes\n"
    );
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    assert_eq!(read(&root, &path), opened);

    // A fix-now finding open: approve is refused.
    append(&root, &path, NO_PERIOD);
    let written = read(&root, &path);
    let refused = atelier(&root, "t2", &["verdict", "approve"]);
    assert_eq!(refused.code, 1);
    assert!(
        refused.stderr.contains(&format!(
            "  [ ] No fix-now finding is open; {stem}.1 is: No period\n"
        )),
        "{}",
        refused.stderr
    );
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    assert_eq!(read(&root, &path), written);

    let run = atelier(
        &root,
        "t2",
        &["verdict", "changes", "--model", "gpt-6.1-sol"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        format!(
            "Recorded your verdict on {ID}, round 1: changes requested. Committed {path} with it.\n\
             {ID} is back with its implementer, t1, to answer your open findings. \
             Your review is done: say so to whoever started you, and stop.\n"
        )
    );
    // One commit: the item's status and the review file, nothing else.
    assert_eq!(git(&root, &["rev-parse", "HEAD^"]), head);
    assert_eq!(
        git(&root, &["show", "--format=", "--name-only", "HEAD"]),
        format!("{}\n{path}", item_path(ID))
    );
    assert_eq!(field(&root, "HEAD", "status"), "changes_requested");
    let committed = git(&root, &["show", &format!("HEAD:{path}")]);
    assert_eq!(front(&committed, "verdict"), "changes");
    assert_eq!(front(&committed, "model"), "gpt-6.1-sol");
    assert!(committed.contains("## Finding 1: No period\n"));
    assert_eq!(git(&root, &["status", "--porcelain"]), "");

    // The implementer is told each open fix-now finding, and submit waits
    // for a response to it.
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.contains(&format!(
            "Status: changes requested\nTo return it to review:\n  [ ] Finding {stem}.1 (fix now): No period\n"
        )),
        "{status}"
    );
    assert!(!status.contains("No test"), "{status}");
    let head = git(&root, &["rev-parse", "HEAD"]);
    let refused = atelier(&root, "t1", &["submit"]);
    assert_eq!(refused.code, 1);
    assert!(refused
        .stderr
        .contains(&format!("[ ] Finding {stem}.1 (fix now)")));
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    // The reviewer's work is done: its verdict is not taken again.
    let again = atelier(&root, "t2", &["verdict", "approve"]);
    assert_eq!(again.code, 1);
    assert!(again
        .stderr
        .starts_with(&format!("{ID} is changes requested")));
}

#[test]
fn later_rounds_carry_each_open_finding_under_its_latest_name_until_it_closes() {
    let scratch = submitted();
    let root = scratch.root();
    let item = item_path(ID);
    // The commit round 1 reviews.
    let round_1 = git(&root, &["rev-parse", "HEAD"]);
    assert_eq!(atelier(&root, "t2", &["review", ID]).code, 0);
    let (first, path) = review_of(&root, 1);
    append(&root, &path, NO_PERIOD);
    assert_eq!(atelier(&root, "t2", &["verdict", "changes"]).code, 0);

    // Round 2: the author rebuts; the reviewer keeps the finding.
    append(
        &root,
        &item,
        &format!("\n### Responses\n\n- {first}.1: rebutted: the period ends the Goal's sentence\n"),
    );
    let submit = atelier(&root, "t1", &["submit"]);
    assert_eq!(submit.code, 0, "{}", submit.stderr);
    let head = git(&root, &["rev-parse", "HEAD"]);
    let run = atelier(&root, "t3", &["review", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let (second, path) = review_of(&root, 2);
    let carried = format!(
        "## Finding 1: No period\nRecommendation: fix now\nWas: {first}.1\n\
         Response: rebutted: the period ends the Goal's sentence\nNow: kept\n\
         <!-- Set Now: to fixed, withdrawn or kept. -->\n"
    );
    assert!(
        read(&root, &path).ends_with(&carried),
        "{}",
        read(&root, &path)
    );
    for part in [
        format!("What changed since the last round's review:\n  git diff {round_1} {head}\n"),
        format!(
            "{first}.1, carried as Finding 1: No period\n  As {first}.1:\n    Recommendation: fix now\n    \
             hello.txt says `hello`, without the Goal's period.\n  \
             The author's response: rebutted: the period ends the Goal's sentence\n"
        ),
        "numbered on from 2:\n\n## Finding 2: <a one-line title>\n".to_string(),
    ] {
        assert!(run.stdout.contains(&part), "no `{part}` in:\n{}", run.stdout);
    }
    // Kept, it is open: approve is refused, changes is not.
    let refused = atelier(&root, "t3", &["verdict", "approve"]);
    assert_eq!(refused.code, 1);
    assert!(refused
        .stderr
        .contains(&format!("{second}.1 is: No period")));
    assert_eq!(atelier(&root, "t3", &["verdict", "changes"]).code, 0);

    // The open finding now goes by its round-2 name.
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.contains(&format!("  [ ] Finding {second}.1 (fix now): No period\n")),
        "{status}"
    );
    assert!(!status.contains(&format!("{first}.1")), "{status}");

    // Round 3: fixed in a commit; the reviewer agrees and approves.
    fs::write(root.join("hello.txt"), "hello.\n").unwrap();
    git(&root, &["commit", "-qam", "Add the period"]);
    let fix = git(&root, &["rev-parse", "--short", "HEAD"]);
    append(&root, &item, &format!("- {second}.1: fixed in {fix}\n"));
    assert_eq!(atelier(&root, "t1", &["submit"]).code, 0);
    let run = atelier(&root, "t4", &["review", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let (third, path) = review_of(&root, 3);
    assert!(read(&root, &path).contains(&format!(
        "Was: {second}.1\nResponse: fixed in {fix}\nNow: kept\n"
    )));
    assert!(run.stdout.contains(&format!("  As {first}.1:\n")));
    assert!(run.stdout.contains(&format!("  As {second}.1:\n")));
    let text = read(&root, &path).replace("Now: kept", "Now: fixed");
    fs::write(root.join(&path), text).unwrap();
    let approved = atelier(&root, "t4", &["verdict", "approve"]);
    assert_eq!(approved.code, 0, "{}", approved.stderr);
    assert_eq!(field(&root, "HEAD", "status"), "approved");
    assert_eq!(
        front(&git(&root, &["show", &format!("HEAD:{path}")]), "verdict"),
        "approve"
    );
    assert!(atelier(&root, "t1", &["status"])
        .stdout
        .contains("Status: approved\n"));

    // The approval goes stale and the item is submitted again: round 4
    // opens, with nothing left to carry.
    fs::write(root.join("hello.txt"), "hello!\n").unwrap();
    git(&root, &["commit", "-qam", "Change the greeting"]);
    assert_eq!(atelier(&root, "t1", &["submit"]).code, 0);
    let run = atelier(&root, "t5", &["review", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let (_, path) = review_of(&root, 4);
    assert!(!read(&root, &path).contains("## Finding"));
    assert!(!run.stdout.contains("Open findings from earlier rounds"));
    assert_eq!(reviews(&root).len(), 4);
    // A kept finding in round 4 is open for its verdict as in round 2.
    append(
        &root,
        &path,
        "\n## Finding 1: Too loud\nRecommendation: fix now\n",
    );
    let refused = atelier(&root, "t5", &["verdict", "approve"]);
    assert_eq!(refused.code, 1);
    assert_eq!(atelier(&root, "t5", &["verdict", "changes"]).code, 0);
    let _ = third;
}

#[test]
fn a_new_reviewer_takes_over_a_review_with_no_verdict_and_keeps_its_file() {
    let scratch = submitted();
    let root = scratch.root();
    assert_eq!(atelier(&root, "t2", &["review", ID]).code, 0);
    let (_, path) = review_of(&root, 1);
    append(&root, &path, "t2 read the diff.\n");
    let before = read(&root, &path);

    let run = atelier(&root, "t3", &["review", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout.starts_with(&format!(
            "Took over round 1's review of {ID}; t2 held it. The file keeps what t2 wrote.\n\n\
             You are the reviewer of {ID} (thread t3), round 1.\n"
        )),
        "{}",
        run.stdout
    );
    assert_eq!(reviews(&root).len(), 1);
    let after = read(&root, &path);
    assert_eq!(front(&after, "reviewers"), "t2 t3");
    // Only the reviewers changed.
    assert_eq!(after.replace("- t3\n", ""), before);

    let refused = atelier(&root, "t2", &["verdict", "approve"]);
    assert_eq!(refused.code, 1);
    assert_eq!(
        refused.stderr,
        format!(
            "You (thread t2) do not hold this round's review of {ID}; t3 does. Nothing changed.\n\
             To take it over: atelier review {ID}\n"
        )
    );
    let run = atelier(&root, "t3", &["verdict", "approve"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let committed = git(&root, &["show", &format!("HEAD:{path}")]);
    assert_eq!(front(&committed, "reviewers"), "t2 t3");
    assert!(committed.ends_with("t2 read the diff."), "{committed}");
}

#[test]
fn a_reviewer_that_runs_an_implementers_command_is_told_its_role() {
    let scratch = submitted();
    let root = scratch.root();
    assert_eq!(atelier(&root, "t2", &["review", ID]).code, 0);
    // A finding still being written, which does not parse yet, does not
    // hide the role line.
    let (_, path) = review_of(&root, 1);
    append(&root, &path, "\n## Finding 1: Half written\n");
    let head = git(&root, &["rev-parse", "HEAD"]);
    let status = git(&root, &["status", "--porcelain"]);
    for args in [&["submit"][..], &["start", ID][..]] {
        let run = atelier(&root, "t2", args);
        assert_eq!(run.code, 1, "{args:?}");
        assert_eq!(
            run.stderr,
            format!("You are the reviewer of {ID}. Reviewers run `review` and `verdict`.\n")
        );
        assert_eq!(run.stdout, "");
        assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
        assert_eq!(git(&root, &["status", "--porcelain"]), status);
        assert_eq!(field(&root, "HEAD", "implementers"), "t1");
    }
    // And the implementer that runs a reviewer's.
    let run = atelier(&root, "t1", &["verdict", "approve"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "You are the implementer of {ID}. Implementers run `start`, `submit` and `merge`.\n"
        )
    );
}

#[test]
fn a_refused_commit_leaves_the_verdict_to_be_run_again() {
    let scratch = submitted();
    let root = scratch.root();
    assert_eq!(atelier(&root, "t2", &["review", ID]).code, 0);
    let (_, path) = review_of(&root, 1);
    append(&root, &path, NO_PERIOD);
    let review = read(&root, &path);
    let item = read(&root, &item_path(ID));
    let head = git(&root, &["rev-parse", "HEAD"]);
    fs::write(root.join("notes.txt"), "unrelated\n").unwrap();
    git(&root, &["add", "notes.txt"]);
    let hook = refusing_hook(&root);

    let run = atelier(&root, "t2", &["verdict", "changes"]);
    assert_eq!(run.code, 2);
    assert!(
        run.stderr
            .contains("The commit failed, so the record is as it was."),
        "{}",
        run.stderr
    );
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    assert_eq!(read(&root, &path), review);
    assert_eq!(read(&root, &item_path(ID)), item);
    assert_eq!(
        git(&root, &["diff", "--cached", "--name-only"]),
        "notes.txt"
    );
    assert!(atelier(&root, "t2", &["status"])
        .stdout
        .contains("Then: atelier verdict changes\n"));

    fs::remove_file(hook).unwrap();
    let run = atelier(&root, "t2", &["verdict", "changes"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(field(&root, "HEAD", "status"), "changes_requested");
    assert_eq!(
        git(&root, &["diff", "--cached", "--name-only"]),
        "notes.txt"
    );
}
