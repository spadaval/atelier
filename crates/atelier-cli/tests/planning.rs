//! The planner's commands, `new`, `set`, `ready`, `drop` and `close`,
//! what `start` needs of their planning, and plan reviews, driven in
//! scratch repositories.

mod common;

use common::*;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const MISSION: &str = "demo-m1ss";

fn mission_path(id: &str) -> String {
    format!(".atelier/missions/{id}/mission.md")
}

fn write_mission(checkout: &Path, id: &str, status: &str) {
    let path = checkout.join(mission_path(id));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!(
            "---\nschema: atelier.mission\nschema_version: 3\nid: {id}\ntitle: Greet\n\
             status: {status}\nlabels: []\nlead: null\nready: null\n\
             created_at: 2026-11-10T09:00:00Z\n---\n\n## Outcome\n\nGreetings.\n"
        ),
    )
    .unwrap();
}

/// Puts the item `id` in the checkout into `mission`.
fn join_mission(checkout: &Path, id: &str, mission: &str) {
    let path = checkout.join(item_path(id));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        text.replace("mission: null", &format!("mission: {mission}")),
    )
    .unwrap();
}

/// Commits everything in `root`; in a detached checkout, as the fixture's
/// is, the base moves to it too, as a backlog branch's merge would move it.
fn commit_all(root: &Path, message: &str) {
    git(root, &["add", "-A"]);
    git(root, &["commit", "-qm", message]);
    if !git_output(root, &["symbolic-ref", "-q", "HEAD"])
        .status
        .success()
    {
        git(root, &["branch", "-f", "main", "HEAD"]);
    }
}

/// The last line of `output`, which must be a `Then:` line, without its
/// prefix.
fn then(output: &str) -> String {
    let last = output.lines().last().unwrap();
    last.strip_prefix("Then: ")
        .unwrap_or_else(|| panic!("{output}"))
        .to_string()
}

/// Runs `line` in a shell at `root`, as an agent pasting it would.
fn shell(root: &Path, line: &str) {
    let output = identity(Command::new("sh").current_dir(root).args(["-c", line]))
        .output()
        .unwrap();
    assert!(output.status.success(), "{line}: {output:?}");
}

/// The paths a planning command printed under `Wrote`, or under
/// `Committed` where it committed them.
fn wrote(output: &str) -> Vec<String> {
    output
        .lines()
        .skip_while(|line| !line.starts_with("Wrote") && !line.starts_with("Committed"))
        .skip(1)
        .map_while(|line| line.strip_prefix("  "))
        .map(str::to_string)
        .collect()
}

/// Whether the checkout's tracked files and index are as `HEAD` holds them.
fn head_and_index(root: &Path) -> (String, String) {
    (
        git(root, &["rev-parse", "HEAD"]),
        git(root, &["diff", "--cached", "--name-only"]),
    )
}

#[test]
fn new_writes_a_draft_item_commits_nothing_and_says_how_to_commit() {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();

    // A mission is made on its own branch, by `start --mission`.
    let refused = atelier(&root, "p1", &["new", "--mission", "Greet everyone"]);
    assert_eq!(refused.code, 1);
    assert_eq!(
        refused.stderr,
        "`atelier new --mission` refused; nothing written.\nA mission is made on a branch of its own, named for it, with: atelier start --mission \"Greet everyone\"\n"
    );
    let mission = atelier(&root, "p1", &["start", "--mission", "Greet everyone"]);
    assert_eq!(mission.code, 0, "{}", mission.stderr);
    let mission_id = mission
        .stdout
        .split_whitespace()
        .nth(3)
        .unwrap()
        .to_string();
    assert!(
        mission.stdout.starts_with(&format!(
            "Created draft mission {mission_id} on branch {mission_id}, with empty sections to write.\n"
        )),
        "{}",
        mission.stdout
    );
    assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), mission_id);
    // One commit on the base's tip, holding the mission alone.
    assert_eq!(
        git(&root, &["rev-parse", "HEAD^"]),
        git(&root, &["rev-parse", "main"])
    );
    assert_eq!(
        git(&root, &["show", "--name-only", "--format=%s", "HEAD"]),
        format!("Plan mission {mission_id}\n\n{}", mission_path(&mission_id))
    );
    let text = read(&root, &mission_path(&mission_id));
    assert_eq!(front(&text, "status"), "draft");
    assert_eq!(front(&text, "title"), "Greet everyone");
    assert!(text.ends_with(
        "---\n\n## Why\n\n## Outcome\n\n## Success criteria\n\n## Scope\n\n## Authority\n\n## Result\n"
    ));

    let before = head_and_index(&root);
    let item = atelier(&root, "p1", &["new", "Say hello", "--in", &mission_id]);
    assert_eq!(item.code, 0, "{}", item.stderr);
    let path = wrote(&item.stdout).pop().unwrap();
    let id = path
        .strip_prefix(".atelier/items/")
        .and_then(|rest| rest.strip_suffix("/item.md"))
        .unwrap()
        .to_string();
    // The config's slug and four characters.
    let suffix = id.strip_prefix("demo-").unwrap();
    assert_eq!(suffix.len(), 4);
    assert!(suffix
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    assert_ne!(id, mission_id);
    let text = read(&root, &path);
    assert_eq!(front(&text, "status"), "draft");
    assert_eq!(front(&text, "mission"), mission_id);
    assert!(text.ends_with("---\n\n## Goal\n\n## Done when\n\n## Scope\n"));
    assert!(item.stdout.starts_with(&format!(
        "Created draft item {id}, with empty Goal, Done when and Scope sections to write.\n"
    )));

    // Nothing is committed or staged; the Then line commits the file alone.
    assert_eq!(head_and_index(&root), before);
    assert_eq!(
        then(&item.stdout),
        format!("git add -- {path} && git commit -m \"Plan {id}\" -- {path}")
    );
    shell(&root, &then(&item.stdout));
    assert_eq!(
        git(&root, &["show", "--name-only", "--format=", "HEAD"]),
        path
    );
    assert_eq!(
        git(&root, &["show", "-s", "--format=%s", "HEAD"]),
        format!("Plan {id}")
    );
    assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");
}

#[test]
fn new_in_a_mission_that_does_not_exist_is_refused_and_writes_nothing() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    // An item is no mission.
    for missing in ["demo-zzzz", ID] {
        let run = atelier(&root, "p1", &["new", "Say hello", "--in", missing]);
        assert_eq!(run.code, 1);
        assert_eq!(
            run.stderr,
            format!(
                "`atelier new` refused; nothing changed.\nNo mission {missing} is in this \
                 checkout. `atelier start --mission \"<title>\"` creates one, on its own branch.\n"
            )
        );
        assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");
    }
}

#[test]
fn the_planning_commands_write_nothing_in_the_bases_checkout_nor_on_an_items_branch() {
    let scratch = Scratch::new(&[(ID, "draft"), ("demo-b2b2", "ready")]);
    let root = scratch.root();
    let base = scratch.worktree("base");
    git(&base, &["switch", "-q", "main"]);
    let commands = [
        &["new", "Say hello"][..],
        &["set", ID, "--title", "Hi"],
        &["ready", ID, "--by", "sumeet"],
        &["drop", ID, "--reason", "Not needed"],
        &["close", ID, "Not needed"],
        &["lead", "demo-zzzz"],
    ];
    for args in commands {
        let run = atelier(&base, "p1", args);
        assert_eq!(run.code, 1, "{args:?}");
        assert_eq!(
            run.stderr,
            format!(
                "`atelier {}` refused; nothing written.\nThis checkout has the base branch main out, and nothing is written there: only a merge writes main, and the pre-commit hook refuses a record change committed on it.\nMake the branch the change belongs on, and run this there: `atelier start --backlog <slug>` for a record change tied to no item or mission, `atelier start --mission \"<title>\"` for a new mission, or `atelier start <id>` for an item's own work. Run in this checkout, `start` makes the branch without switching it, and prints the `git worktree add` that gives the branch a checkout of its own.\n{SKIP}",
                args[0]
            )
        );
        assert_eq!(git(&base, &["status", "--porcelain", "-uall"]), "");
    }
    // On an item's branch, a change that is not the item's is refused.
    assert_eq!(atelier(&root, "t1", &["start", "demo-b2b2"]).code, 0);
    for args in commands {
        let run = atelier(&root, "p1", args);
        assert_eq!(run.code, 1, "{args:?}");
        assert!(
            run.stderr.starts_with(&format!(
                "`atelier {}` refused; nothing written.\nThis checkout is on demo-b2b2, the branch of item demo-b2b2, and this change is not demo-b2b2's",
                args[0]
            )),
            "{}",
            run.stderr
        );
        assert!(run.stderr.ends_with(SKIP), "{}", run.stderr);
        assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");
    }
}

/// An item body with a fence, trailing spaces and no final newline, which
/// a rewrite of its front matter must keep.
const BODY: &str =
    "\n## Goal\n\nGreet.  \n\n```yaml\nstatus: done\n---\n```\n\n## Done when\n\n- [ ] It greets.";

#[test]
fn set_changes_each_named_field_and_nothing_else_in_the_file() {
    let scratch = Scratch::new(&[("demo-b10c", "draft"), (ID, "ready")]);
    let root = scratch.root();
    write_mission(&root, MISSION, "draft");
    let path = root.join(item_path(ID));
    let text = fs::read_to_string(&path).unwrap();
    let (front_matter, _) = text.split_at(text.find("---\n\n## Goal").unwrap() + 4);
    fs::write(&path, format!("{front_matter}{BODY}")).unwrap();
    commit_all(&root, "Plan");
    let before = head_and_index(&root);

    let run = atelier(
        &root,
        "p1",
        &[
            "set",
            ID,
            "--title",
            "Say hello: twice",
            "--priority",
            "low",
            "--labels",
            "cli, greeting",
            "--mission",
            MISSION,
            "--blocked-by",
            "demo-b10c",
            "--caused-by",
            "0123abc",
        ],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.ends_with(&format!("\n---\n{BODY}")), "{text}");
    for (key, value) in [
        ("title", "'Say hello: twice'"),
        ("priority", "low"),
        ("labels", "cli greeting"),
        ("mission", MISSION),
        ("blocked_by", "demo-b10c"),
        ("caused_by", "0123abc"),
        ("status", "ready"),
    ] {
        assert_eq!(front(&text, key), value, "{key}");
    }
    assert_eq!(wrote(&run.stdout), [item_path(ID)]);
    assert_eq!(
        then(&run.stdout),
        format!(
            "git add -- {p} && git commit -m \"Set fields of {ID}\" -- {p}",
            p = item_path(ID)
        )
    );
    assert_eq!(head_and_index(&root), before);

    // An empty value clears; a field not named stays.
    let run = atelier(
        &root,
        "p1",
        &[
            "set",
            ID,
            "--labels",
            "",
            "--mission",
            "",
            "--caused-by",
            "",
            "--blocked-by",
            "",
        ],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let text = fs::read_to_string(&path).unwrap();
    for (key, value) in [
        ("labels", "[]"),
        ("mission", "null"),
        ("caused_by", "null"),
        ("blocked_by", "[]"),
        ("priority", "low"),
    ] {
        assert_eq!(front(&text, key), value, "{key}");
    }
    assert!(text.ends_with(&format!("\n---\n{BODY}")));

    // A mission's title and labels; its body is kept too.
    let mission = root.join(mission_path(MISSION));
    let body = fs::read_to_string(&mission).unwrap();
    let body = body.split_once("\n---\n").unwrap().1.to_string();
    let run = atelier(
        &root,
        "p1",
        &["set", MISSION, "--title", "Greet all", "--labels", "x"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let text = fs::read_to_string(&mission).unwrap();
    assert_eq!(front(&text, "title"), "Greet all");
    assert_eq!(front(&text, "labels"), "x");
    assert!(text.ends_with(&format!("\n---\n{body}")));
}

#[test]
fn set_is_refused_with_nothing_written_for_a_field_or_record_it_cannot_change() {
    let scratch = Scratch::new(&[
        (ID, "draft"),
        ("demo-d0ne", "done"),
        ("demo-dr0p", "dropped"),
    ]);
    let root = scratch.root();
    write_mission(&root, MISSION, "draft");
    write_mission(&root, "demo-m2ss", "done");
    commit_all(&root, "Plan");
    let refusals: [(&[&str], &str); 9] = [
        (
            &["set", MISSION, "--priority", "low", "--title", "T"],
            "demo-m1ss is a mission, and --priority only an item has. A mission's fields are --title and --labels.\n",
        ),
        (
            &["set", ID, "--blocked-by", "demo-b10c,demo-d0ne"],
            "  .atelier/items/demo-ab12/item.md: blocked_by: `demo-b10c` names no item\n",
        ),
        (
            &["set", ID, "--mission", "demo-zzzz"],
            "  .atelier/items/demo-ab12/item.md: mission: `demo-zzzz` names no mission\n",
        ),
        (
            &["set", ID, "--mission", "demo-d0ne"],
            "  .atelier/items/demo-ab12/item.md: mission: `demo-d0ne` names no mission\n",
        ),
        (
            &["set", ID, "--caused-by", "abc12"],
            "  .atelier/items/demo-ab12/item.md: caused_by: `abc12` is neither an item ID nor a commit hash\n",
        ),
        (
            &["set", ID, "--caused-by", "not a hash"],
            "caused_by: `not a hash` is neither an item ID nor a commit hash\n",
        ),
        (
            &["set", "demo-d0ne", "--title", "T"],
            "demo-d0ne is done, and its record no longer changes.\n",
        ),
        (
            &["set", "demo-dr0p", "--title", "T"],
            "demo-dr0p is dropped, and its record no longer changes.\n",
        ),
        (
            &["set", "demo-m2ss", "--labels", "x"],
            "demo-m2ss is done, and its record no longer changes.\n",
        ),
    ];
    for (args, says) in refusals {
        let run = atelier(&root, "p1", args);
        assert_eq!(run.code, 1, "{args:?}: {}", run.stdout);
        // A flag a mission lacks is no rule; the rest end offering the skip.
        let end = if says.contains("only an item has") {
            says
        } else {
            SKIP
        };
        assert!(
            run.stderr.starts_with("`atelier set")
                && run.stderr.contains(says)
                && run.stderr.ends_with(end),
            "{args:?}: {}",
            run.stderr
        );
        assert_eq!(
            git(&root, &["status", "--porcelain", "-uall"]),
            "",
            "{args:?}"
        );
    }

    // A started item's record changes only on its branch, which is named.
    assert_eq!(planned(&root, "p1", &["ready", ID, "--by", "s"]).code, 0);
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    git(&root, &["checkout", "-q", "--detach", "main"]);
    let run = atelier(&root, "p1", &["set", ID, "--title", "T"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier set {ID}` refused; nothing changed.\n{ID} is building: it is started, and \
             its record now changes only on branch {ID}, by its implementer.\n{SKIP}"
        )
    );
    assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");
}

#[test]
fn ready_marks_each_named_draft_with_who_and_when_changes_no_other_item_and_commits_nothing() {
    let scratch = Scratch::new(&[(ID, "draft"), ("demo-b10c", "draft")]);
    let root = scratch.root();
    write_mission(&root, MISSION, "draft");
    join_mission(&root, ID, MISSION);
    join_mission(&root, "demo-b10c", MISSION);
    commit_all(&root, "Plan");
    let before = head_and_index(&root);

    // Readying the mission readies none of its items.
    let run = atelier(&root, "p1", &["ready", MISSION, "--by", "sumeet"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(wrote(&run.stdout), [mission_path(MISSION)]);
    let mission = read(&root, &mission_path(MISSION));
    assert_eq!(front(&mission, "status"), "ready");
    assert!(
        mission.contains("ready:\n  by: sumeet\n  at: "),
        "{mission}"
    );
    assert_eq!(head_and_index(&root), before);
    assert_eq!(git(&root, &["diff", "--name-only"]), mission_path(MISSION));
    assert_eq!(
        then(&run.stdout),
        format!(
            "git add -- {0} && git commit -m \"Ready {MISSION}\" -- {0}",
            mission_path(MISSION)
        )
    );

    // Several IDs at once; one named twice is readied once.
    let run = atelier(
        &root,
        "p1",
        &["ready", ID, "demo-b10c", ID, "--by", "sumeet"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(wrote(&run.stdout), [item_path(ID), item_path("demo-b10c")]);
    for id in [ID, "demo-b10c"] {
        let text = read(&root, &item_path(id));
        assert_eq!(front(&text, "status"), "ready");
        let at = text.lines().find_map(|l| l.strip_prefix("  at: ")).unwrap();
        assert!(chrono::DateTime::parse_from_rfc3339(at).is_ok(), "{at}");
        assert!(text.contains("ready:\n  by: sumeet\n"));
    }
    assert!(run
        .stdout
        .contains(&format!("Made ready, as sumeet said: {ID}, demo-b10c.\n")));
    // No commit, and nothing staged.
    assert_eq!(head_and_index(&root), before);
    let mut changed: Vec<String> = git(&root, &["diff", "--name-only"])
        .lines()
        .map(str::to_string)
        .collect();
    changed.sort();
    let mut expected = vec![mission_path(MISSION), item_path(ID), item_path("demo-b10c")];
    expected.sort();
    assert_eq!(changed, expected);
}

#[test]
fn ready_writes_nothing_when_any_named_id_is_unknown_or_not_a_draft() {
    let scratch = Scratch::new(&[(ID, "draft"), ("demo-r3dy", "ready"), ("demo-d0ne", "done")]);
    let root = scratch.root();
    write_mission(&root, MISSION, "ready");
    commit_all(&root, "Plan");
    let run = atelier(
        &root,
        "p1",
        &[
            "ready",
            ID,
            "demo-zzzz",
            "demo-r3dy",
            MISSION,
            "not-an-id!",
            "demo-d0ne",
            "--by",
            "s",
        ],
    );
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        "`atelier ready` refused; nothing changed.\n  \
         demo-zzzz: no item or mission has this ID in this checkout or on a branch\n  \
         not-an-id!: no item or mission has this ID in this checkout or on a branch\n  \
         demo-r3dy: it is ready, and `atelier ready` readies a draft\n  \
         demo-m1ss: it is ready, and `atelier ready` readies a draft\n  \
         demo-d0ne: it is done, and `atelier ready` readies a draft\n\
         Run it again with only the drafts to make ready.\n"
    );
    assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");

    // `--by` is required, and may not be empty.
    for args in [&["ready", ID][..], &["ready", ID, "--by", ""]] {
        let run = atelier(&root, "p1", args);
        assert_eq!(run.code, 2, "{args:?}");
        assert!(run.stderr.contains("--by"), "{}", run.stderr);
    }
    assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");
}

/// A remote `origin`, a bare repository holding the base, which the
/// config names as the forge's remote.
fn with_remote(scratch: &Scratch) -> std::path::PathBuf {
    let root = scratch.root();
    append(
        &root,
        ".atelier/config.toml",
        "\n[forge]\nprovider = \"github\"\nremote = \"origin\"\n",
    );
    on_base(&root, "Configure");
    let remote = scratch.dir.path().join("remote.git");
    git(
        &root,
        &[
            "init",
            "-q",
            "--bare",
            "-b",
            "main",
            remote.to_str().unwrap(),
        ],
    );
    git(
        &root,
        &["remote", "add", "origin", remote.to_str().unwrap()],
    );
    git(&root, &["push", "-q", "origin", "main"]);
    git(&root, &["fetch", "-q", "origin"]);
    remote
}

#[test]
fn status_names_each_record_change_not_committed_and_each_commit_not_pushed() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let remote = with_remote(&scratch);
    let run = atelier(&root, "p1", &["start", "--backlog", "plan"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        git(&root, &["symbolic-ref", "--short", "HEAD"]),
        "backlog/plan"
    );
    // The branch is on the remote already: `start` pushed it.
    assert_eq!(
        git(&remote, &["rev-parse", "backlog/plan"]),
        git(&root, &["rev-parse", "main"])
    );
    assert_eq!(atelier(&root, "p1", &["ready", ID, "--by", "s"]).code, 0);
    let new = atelier(&root, "p1", &["new", "Say goodbye"]);
    let added = wrote(&new.stdout).pop().unwrap();
    // Not a record: a change outside them.
    fs::write(root.join("notes.txt"), "x").unwrap();

    let status = atelier(&root, "p1", &["status"]).stdout;
    let mut paths = [added.clone(), item_path(ID)];
    paths.sort();
    let paths = paths.join(" ");
    assert!(
        status.contains(&format!(
            "Record changes not committed, which count only once committed and merged:\n  {}\n  {}\nCommit them with: git add -- {paths} && git commit -m \"<what they change>\" -- {paths}\n",
            paths.split(' ').next().unwrap(),
            paths.split(' ').nth(1).unwrap(),
        )),
        "{status}"
    );
    assert!(!status.contains("not pushed"), "{status}");
    assert!(
        status.starts_with(&format!(
            "{NO_HOOKS}You are thread p1, on branch backlog/plan, a backlog branch for record changes.\n"
        )),
        "{status}"
    );

    shell(
        &root,
        &format!("git add -- {paths} && git commit -qm Plan -- {paths}"),
    );
    let status = atelier(&root, "p1", &["status"]).stdout;
    assert!(!status.contains("not committed"), "{status}");
    assert!(
        status.contains("1 commit of backlog/plan not pushed to origin. Push with: git push origin backlog/plan\n"),
        "{status}"
    );
    git(&root, &["push", "-q", "origin", "backlog/plan"]);
    let status = atelier(&root, "p1", &["status"]).stdout;
    assert!(!status.contains("not pushed"), "{status}");
}

#[test]
fn start_is_refused_until_the_items_mission_is_ready_and_names_ready() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    write_mission(&root, MISSION, "draft");
    join_mission(&root, ID, MISSION);
    commit_all(&root, "Plan");

    let run = atelier(&root, "t1", &["start", ID]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier start {ID}` refused; nothing changed.\nTo start it:\n  [ ] Mission \
             {MISSION} is ready (it is draft; a planner makes it ready with: atelier ready \
             {MISSION} --by <name>)\nThen: atelier start {ID}\n{SKIP}"
        )
    );
    assert!(!branch_exists(&root, ID));
    // `status` counts it as an item that cannot start, for the same reason.
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.ends_with(
            "1 ready item cannot start yet: 1 is in a mission that is not ready. \
             See it with: atelier list\nNo ready item can start now.\n"
        ),
        "{status}"
    );

    assert_eq!(
        planned(&root, "p1", &["ready", MISSION, "--by", "s"]).code,
        0
    );
    let run = atelier(&root, "t1", &["start", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

#[test]
fn status_on_the_base_lists_only_what_can_start_and_counts_the_rest_by_cause() {
    let scratch = Scratch::new(&[("demo-a1aa", "ready"), ("demo-c3cc", "draft")]);
    let root = scratch.root();
    write_item(&root, "demo-b2bb", "ready", &[], &["demo-c3cc"], "");
    write_item(&root, "demo-d4dd", "ready", &[], &[], "");
    write_mission(&root, MISSION, "draft");
    join_mission(&root, "demo-d4dd", MISSION);
    commit_all(&root, "Plan");

    let others = "2 other ready items cannot start yet: 1 waits on a blocker that is not done, \
                  1 is in a mission that is not ready. See them with: atelier list\n";
    assert_eq!(
        atelier(&root, "t1", &["status"]).stdout,
        format!(
            "{NO_HOOKS}You are thread t1, on a detached HEAD.\nReady to start:\n  \
             demo-a1aa (high): Say hello\n{others}Then: atelier start demo-a1aa\n"
        )
    );
    for id in ["demo-b2bb", "demo-d4dd"] {
        assert_eq!(atelier(&root, "t1", &["start", id]).code, 1);
    }

    // Once t1 holds the one that could, nothing can start: a thread
    // holding nothing is told so and offered no start, and t1 is still
    // sent back to its item.
    assert_eq!(atelier(&root, "t1", &["start", "demo-a1aa"]).code, 0);
    let base = scratch.worktree("base");
    git(&base, &["switch", "-q", "main"]);
    let count = "2 ready items cannot start yet: 1 waits on a blocker that is not done, \
                 1 is in a mission that is not ready. See them with: atelier list\n";
    assert_eq!(
        atelier(&base, "t2", &["status"]).stdout,
        format!(
            "{NO_HOOKS}You are thread t2, on the base branch main.\n{count}Held by other \
             threads:\n  demo-a1aa (building, held by t1): Say hello\nNo ready item can start now.\n"
        )
    );
    assert_eq!(
        atelier(&base, "t1", &["status"]).stdout,
        format!(
            "{NO_HOOKS}You are thread t1, on the base branch main.\nYou hold:\n  demo-a1aa \
             (building): Say hello\n{count}Then: atelier start demo-a1aa (switches to its branch)\n"
        )
    );
}

/// The ID `start --new` made, from what it printed.
fn started_id(stdout: &str) -> String {
    let rest = stdout.strip_prefix("Created ").unwrap();
    rest.split(' ').next().unwrap().to_string()
}

#[test]
fn start_new_creates_an_item_building_on_its_own_branch_in_one_commit() {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    let base = git(&root, &["rev-parse", "main"]);

    let run = atelier(&root, "t1", &["start", "--new", "Fix the greeting"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let id = started_id(&run.stdout);
    assert!(
        id.starts_with("demo-") && id.len() == "demo-".len() + 4,
        "{id}"
    );

    // One commit on the base's tip, holding the item alone, on its branch.
    assert_eq!(git(&root, &["rev-parse", &format!("{id}^")]), base);
    assert_eq!(git(&root, &["rev-parse", "main"]), base);
    assert_eq!(
        git(&root, &["diff", "--name-only", "main", &id]),
        item_path(&id)
    );
    assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), id);
    assert_eq!(git(&root, &["status", "--porcelain"]), "");
    let text = git(&root, &["show", &format!("{id}:{}", item_path(&id))]);
    assert_eq!(front(&text, "status"), "building");
    assert_eq!(front(&text, "implementers"), "t1");
    assert_eq!(front(&text, "title"), "Fix the greeting");
    assert_eq!(front(&text, "ready"), "null");

    // `status` tells the implementer to define it first, which is no
    // condition of `submit`.
    let path = item_path(&id);
    let write_first = format!(
        "Status: building\nWrite its Goal and its Done when lines in {path} first: they say \
         what the change must do, and the reviewer judges it against them.\nTo submit it for \
         review:\n  [ ] Report: written under `## Report`\n"
    );
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(status.contains(&write_first), "{status}");
    assert!(run.stdout.contains(&write_first), "{}", run.stdout);
    append(&root, &path, "\n## Report\n\nNothing yet.\n");
    let submitted = atelier(&root, "t1", &["submit"]);
    assert_eq!(submitted.code, 0, "{}", submitted.stderr);

    // With its Goal written and its Done when in prose, as many records
    // converted from the old layout are, the line asks only for the
    // lines, in the form they take.
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    let id = started_id(&atelier(&root, "t1", &["start", "--new", "Fix it"]).stdout);
    let path = item_path(&id);
    let text = read(&root, &path).replace("## Goal\n", "## Goal\n\nFix it.\n");
    fs::write(
        root.join(&path),
        text.replace("## Done when\n", "## Done when\n\nIt is fixed.\n"),
    )
    .unwrap();
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.contains(&format!(
            "Status: building\nWrite its Done when lines in {path} first, each as `- [ ] <result>`: \
             they say what the change must do, and the reviewer judges it against them.\n"
        )),
        "{status}"
    );

    // Once a Done when line is written, the line goes.
    let text = read(&root, &path).replace("It is fixed.\n", "- [ ] Fixed.\n");
    fs::write(root.join(&path), text).unwrap();
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(!status.contains("Write its"), "{status}");
}

#[test]
fn a_start_new_stopped_before_its_branch_leaves_no_branch_and_no_item() {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    let release = scratch.dir.path().join("release");
    let mut child = atelier_command(&root, "t1", &["start", "--new", "Fix it"])
        .env("ATELIER_TEST_HOLD_BEFORE_BRANCH", &release)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let since = Instant::now();
    while !release.with_extension("held").exists() {
        assert!(
            since.elapsed() < Duration::from_secs(30),
            "start never held"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    child.kill().unwrap();
    child.wait().unwrap();

    let branches = git(
        &root,
        &["for-each-ref", "--format=%(refname:short)", "refs/heads/"],
    );
    assert_eq!(branches, "main");
    assert_eq!(git(&root, &["rev-parse", "--abbrev-ref", "HEAD"]), "HEAD");
    assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert_eq!(
        status,
        format!("{NO_HOOKS}You are thread t1, on a detached HEAD.\nNothing is ready to start.\n")
    );
}

#[test]
fn the_printed_commit_works_from_a_subdirectory_and_with_paths_a_shell_would_split() {
    // From a subdirectory, the Then line names the checkout's root.
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    let sub = root.join("src");
    fs::create_dir_all(&sub).unwrap();
    let new = atelier(&sub, "p1", &["new", "Say hello"]);
    assert_eq!(new.code, 0, "{}", new.stderr);
    let line = then(&new.stdout);
    assert!(
        line.starts_with(&format!("git -C {} add -- ", root.display())),
        "{line}"
    );
    shell(&sub, &line);
    assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");

    // A backlog branch checked out at a path with a space, with an
    // uncommitted mission document whose name has a space and a quote;
    // `status` is run from a directory under it.
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let base = scratch.dir.path().join("base copy");
    git(
        &root,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "backlog/plan",
            base.to_str().unwrap(),
            "main",
        ],
    );
    write_mission(&base, MISSION, "draft");
    join_mission(&base, ID, MISSION);
    commit_all(&base, "Plan");
    let run = atelier(&base, "p1", &["ready", ID, MISSION, "--by", "s"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    shell(&base, &then(&run.stdout));
    let document = format!(".atelier/missions/{MISSION}/design notes 'v2'.md");
    fs::write(base.join(&document), "# Design\n").unwrap();
    let under = base.join("src");
    fs::create_dir_all(&under).unwrap();

    let status = atelier(&under, "p1", &["status"]).stdout;
    let line = status
        .lines()
        .find_map(|l| l.strip_prefix("Commit them with: "))
        .unwrap();
    assert!(
        line.contains("'.atelier/missions/demo-m1ss/design notes '\\''v2'\\''.md'"),
        "{line}"
    );
    assert!(
        line.starts_with(&format!("git -C '{}' add -- ", base.display())),
        "{line}"
    );
    shell(&under, line);
    assert_eq!(git(&base, &["status", "--porcelain", "-uall"]), "");
    assert!(
        git(&base, &["show", "--name-only", "--format=", "HEAD"]).contains("design notes 'v2'.md")
    );
}

/// The ID of the record a planning command created.
fn created(run: &Run) -> String {
    assert_eq!(run.code, 0, "{}", run.stderr);
    let path = wrote(&run.stdout).pop().unwrap();
    path.split('/').nth(2).unwrap().to_string()
}

/// Runs a planning command as `p1`, and commits what it wrote with the
/// line it printed; in a detached checkout the base moves to that commit,
/// as a backlog branch's merge would move it.
fn plan(root: &Path, args: &[&str]) -> Run {
    let run = atelier(root, "p1", args);
    assert_eq!(run.code, 0, "{args:?}: {}", run.stderr);
    shell(root, &then(&run.stdout));
    if !git_output(root, &["symbolic-ref", "-q", "HEAD"])
        .status
        .success()
    {
        git(root, &["branch", "-f", "main", "HEAD"]);
    }
    run
}

/// A review file of finding `title`, recommended fix now.
fn finding(title: &str) -> String {
    format!("\n## Finding 1: {title}\nRecommendation: fix now\nIt is wrong.\n")
}

/// The review files under `dir` in the checkout, by name.
fn files_in(root: &Path, dir: &str) -> Vec<String> {
    let Ok(listing) = fs::read_dir(root.join(dir)) else {
        return Vec::new();
    };
    let mut names: Vec<String> = listing
        .map(|entry| {
            format!(
                "{dir}/{}",
                entry.unwrap().file_name().into_string().unwrap()
            )
        })
        .collect();
    names.sort();
    names
}

fn plan_reviews(mission: &str) -> String {
    format!(".atelier/missions/{mission}/reviews")
}

fn tip(root: &Path, rev: &str) -> String {
    git(root, &["rev-parse", rev])
}

#[test]
fn drop_puts_its_reason_first_in_the_report_and_needs_a_reason_and_a_record_not_done() {
    let scratch = Scratch::new(&[(ID, "ready"), ("demo-d0ne", "done")]);
    let root = scratch.root();
    // The item has a Report with text and Responses to a finding of its review.
    let report = "It half works.\n\n### Responses\n\n- 1-aaaa.1: fixed in abc1234\n";
    write_item(&root, ID, "ready", &[], &[], report);
    let review = format!(
        "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: abc1234\n\
         reviewers:\n- r1\nmodel: null\nverdict: changes\ncreated_at: 2026-11-10T09:00:00Z\n---\n\
         ## Summary\ns\n{}",
        finding("Wrong")
    );
    fs::create_dir_all(root.join(".atelier/items").join(ID).join("reviews")).unwrap();
    fs::write(
        root.join(format!(".atelier/items/{ID}/reviews/1-aaaa.md")),
        review,
    )
    .unwrap();
    commit_all(&root, "Plan");
    let before = read(&root, &item_path(ID));

    for reason in [None, Some("  "), Some("one\ntwo")] {
        let mut args = vec!["drop", ID];
        args.extend(reason.iter().flat_map(|r| ["--reason", r]));
        let run = atelier(&root, "p1", &args);
        assert_eq!(run.code, 1, "{reason:?}");
        assert_eq!(
            run.stderr,
            format!(
                "`atelier drop` needs a reason, on one line. Nothing changed.\nThen: atelier drop \
                 {ID} --reason \"<why it will not be done>\"\n"
            )
        );
        assert_eq!(read(&root, &item_path(ID)), before);
    }

    let run = plan(&root, &["drop", ID, "--reason", "The site greets instead"]);
    assert!(run.stdout.starts_with(&format!(
        "Dropped item {ID}: The site greets instead\nWrote, and did not commit:\n"
    )));
    let text = read(&root, &item_path(ID));
    assert_eq!(front(&text, "status"), "dropped");
    // Saved only once it validated against the review its Responses name.
    assert!(text.ends_with(
        "## Report\n\nDropped: The site greets instead\n\nIt half works.\n\n### Responses\n\n\
         - 1-aaaa.1: fixed in abc1234\n"
    ));
    assert_eq!(
        git(&root, &["show", "--name-only", "--format=", "HEAD"]),
        item_path(ID)
    );

    // Dropped or done, it is refused, and nothing is written.
    for (id, status) in [(ID, "dropped"), ("demo-d0ne", "done")] {
        let before = read(&root, &item_path(id));
        let run = planned(&root, "p1", &["drop", id, "--reason", "again"]);
        assert_eq!(run.code, 1);
        assert!(
            run.stderr.starts_with(&format!(
                "{id} is {status}, and `atelier drop` runs on an item that is draft, ready, \
                 building, in review, changes requested, approved. Nothing changed.\n"
            )),
            "{}",
            run.stderr
        );
        assert_eq!(read(&root, &item_path(id)), before);
    }
}

#[test]
fn a_dropped_mission_changes_no_item_and_start_on_one_says_it_was_dropped() {
    const STARTED: &str = "demo-st4r";
    let scratch = Scratch::new(&[(ID, "ready"), (STARTED, "ready")]);
    let root = scratch.root();
    write_mission(&root, MISSION, "ready");
    join_mission(&root, ID, MISSION);
    join_mission(&root, STARTED, MISSION);
    commit_all(&root, "Plan");
    let item = read(&root, &item_path(ID));
    // One of its items is started before the mission is dropped.
    let work = scratch.worktree("work");
    assert_eq!(atelier(&work, "t1", &["start", STARTED]).code, 0);
    let branch = tip(&root, STARTED);

    let run = plan(
        &root,
        &["drop", MISSION, "--reason", "No time this quarter"],
    );
    assert_eq!(wrote(&run.stdout), [mission_path(MISSION)]);
    let text = read(&root, &mission_path(MISSION));
    assert_eq!(front(&text, "status"), "dropped");
    // It had no Result section, so one is added, holding the reason.
    assert!(
        text.ends_with("## Outcome\n\nGreetings.\n\n## Result\n\nDropped: No time this quarter\n")
    );
    assert_eq!(read(&root, &item_path(ID)), item);

    // Neither a ready item nor a started one is started, or taken over.
    for id in [ID, STARTED] {
        let run = atelier(&root, "t2", &["start", id]);
        assert_eq!(run.code, 1, "{id}");
        assert!(
            run.stderr.contains(&format!(
                "  [ ] Mission {MISSION} is ready (it was dropped, so none of its items starts)\n"
            )),
            "{}",
            run.stderr
        );
    }
    assert_eq!(tip(&root, STARTED), branch);
    assert_eq!(git(&root, &["branch", "--show-current"]), "");
    let run = planned(&root, "p1", &["drop", MISSION, "--reason", "again"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "Mission {MISSION} is dropped, and `atelier drop` runs on a mission that is draft, \
             ready. Nothing changed.\nNothing is left to do on it.\n{SKIP}"
        )
    );
}

#[test]
fn a_started_item_is_dropped_on_its_branch_and_then_refused_everywhere_with_its_reason() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let work = scratch.worktree("work");
    assert_eq!(atelier(&work, "t1", &["start", ID]).code, 0);
    fs::write(work.join("hello.txt"), "hello\n").unwrap();
    git(&work, &["add", "hello.txt"]);
    git(&work, &["commit", "-qm", "Say hello"]);
    finish_and_submit(&work, "t1");
    // Approved, so only the drop stands between it and its merge.
    assert_eq!(atelier(&work, "t2", &["review", ID]).code, 0);
    assert_eq!(verdict(&work, "t2", &["verdict", "approve"]).code, 0);

    // Anywhere but a checkout of its branch, the drop is refused.
    let reason = "The site greets instead";
    let run = atelier(&root, "p1", &["drop", ID, "--reason", reason]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.starts_with(&format!(
            "`atelier drop {ID}` refused; nothing written.\n{ID} is started, and a started item's record changes only on its branch {ID}; this checkout is on a detached HEAD.\nRun it in {}, which has {ID} checked out.\n",
            work.display()
        )),
        "{}",
        run.stderr
    );
    assert_eq!(git(&root, &["status", "--porcelain"]), "");

    // In the branch's checkout any thread drops it, and commits it.
    let run = atelier(&work, "p1", &["drop", ID, "--reason", reason]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout.starts_with(&format!(
            "Dropped item {ID} on its branch {ID}: {reason}\nOnce it is committed, `submit`, `merge` and a take-over of {ID} are refused.\nThe base's copy of {ID} is dropped on a backlog branch after that: `atelier start --backlog drop-{ID}`, then there `atelier drop {ID} --reason \"{reason}\"`, commit, `atelier submit` and `atelier merge`. Until then `status` and `list` say the drop has not reached the base.\nWrote, and did not commit:\n  {}\n",
            item_path(ID)
        )),
        "{}",
        run.stdout
    );
    let text = read(&work, &item_path(ID));
    assert_eq!(front(&text, "status"), "dropped");
    assert_eq!(front(&text, "implementers"), "t1");
    shell(&work, &then(&run.stdout));
    let branch = tip(&work, ID);
    let base = tip(&root, "main");

    // Until the base's copy is dropped too, `status` and `list` say so.
    let listed = atelier(&root, "t3", &["status"]).stdout;
    assert!(
        listed.contains(&format!(
            "Dropped on their branch, and the drop has not reached the base; carry it on a backlog branch with `atelier drop <id> --reason \"<why>\"` there, then `submit` and `merge`:\n  {ID}: {reason}\n"
        )),
        "{listed}"
    );
    let list = atelier(&root, "t3", &["list"]).stdout;
    assert!(
        list.contains(&format!(
            "{ID} (dropped on its branch; the drop has not reached the base"
        )),
        "{list}"
    );
    let why = format!("It was dropped: {reason}\nNothing is left to do on it.\n");
    let status = atelier(&work, "t1", &["status"]).stdout;
    assert!(
        status.ends_with(&format!("Status: dropped\n{why}")),
        "{status}"
    );
    for (at, thread, command, skip) in [
        (&work, "t1", "submit", SKIP),
        (&work, "t1", "merge", SKIP),
        (&root, "t3", "start", ""),
    ] {
        let args: Vec<&str> = match command {
            "start" => vec!["start", ID],
            _ => vec![command],
        };
        let run = atelier(at, thread, &args);
        assert_eq!(run.code, 1, "{command}: {}", run.stderr);
        assert!(
            run.stderr
                .starts_with(&format!("{ID} is dropped, and `atelier {command}` runs on"))
                && run
                    .stderr
                    .ends_with(&format!("Nothing changed.\n{why}{skip}")),
            "{command}: {}",
            run.stderr
        );
        assert_eq!(
            (tip(&work, ID), tip(&root, "main")),
            (branch.clone(), base.clone())
        );
        assert_eq!(git(&work, &["status", "--porcelain"]), "");
    }

    // The base's copy, on a backlog branch: once it merges, nothing says
    // the drop is waiting, and the branch can go.
    assert_eq!(
        atelier(&root, "p1", &["start", "--backlog", "drop-it"]).code,
        0
    );
    let run = plan(&root, &["drop", ID, "--reason", reason]);
    assert_eq!(wrote(&run.stdout), [item_path(ID)]);
    let merged = atelier(&root, "p1", &["merge"]);
    assert_eq!(merged.code, 0, "{}", merged.stderr);
    assert_eq!(field(&root, "main", "status"), "dropped");
    let listed = atelier(&root, "t3", &["status"]).stdout;
    assert!(!listed.contains("has not reached the base"), "{listed}");
    git(&work, &["checkout", "-q", "--detach"]);
    git(&root, &["branch", "-qD", ID]);
    assert_eq!(field(&root, "main", "status"), "dropped");
    let shown = atelier(&root, "t3", &["show", ID]).stdout;
    assert!(shown.contains("Status: dropped\n"), "{shown}");
}

#[test]
fn a_terminal_status_on_the_base_wins_over_a_branch_record_that_does_not_parse() {
    for terminal in ["dropped", "done"] {
        let scratch = Scratch::new(&[(ID, "ready")]);
        let root = scratch.root();
        let work = scratch.worktree("work");
        assert_eq!(atelier(&work, "t1", &["start", ID]).code, 0);
        let reason = "The site greets instead";
        // The base's copy is final, as its backlog branch's merge or the
        // item's own merge left it.
        let report = match terminal {
            "dropped" => format!("Dropped: {reason}\n"),
            _ => String::new(),
        };
        write_item(&root, ID, terminal, &[], &[], &report);
        commit_all(&root, terminal);
        // The branch then holds a record that does not parse.
        let path = work.join(item_path(ID));
        let broken = read(&work, &item_path(ID)).replace("status: building", "status: broken");
        fs::write(&path, broken).unwrap();
        git(&work, &["commit", "-qam", "Broken"]);

        let why = match terminal {
            "dropped" => format!("It was dropped: {reason}\nNothing is left to do on it.\n"),
            _ => "Nothing is left to do on it.\n".to_string(),
        };
        let status = atelier(&work, "t1", &["status"]);
        assert_eq!(status.code, 0, "{}", status.stderr);
        assert!(
            status
                .stdout
                .ends_with(&format!("Status: {terminal}\n{why}")),
            "{}",
            status.stdout
        );
        let shown = atelier(&work, "t1", &["show", ID]);
        assert_eq!(shown.code, 0, "{}", shown.stderr);
        assert!(
            shown.stdout.contains(&format!("Status: {terminal}\n")),
            "{}",
            shown.stdout
        );
        if terminal == "dropped" {
            assert!(
                shown.stdout.contains(&format!("Dropped: {reason}\n")),
                "{}",
                shown.stdout
            );
        }
        for (at, thread, command) in [
            (&work, "t1", "submit"),
            (&work, "t1", "merge"),
            (&root, "t2", "start"),
        ] {
            let args: Vec<&str> = match command {
                "start" => vec!["start", ID],
                _ => vec![command],
            };
            let run = atelier(at, thread, &args);
            assert_eq!(run.code, 1, "{command}: {}", run.stderr);
            assert!(
                run.stderr.starts_with(&format!(
                    "{ID} is {terminal}, and `atelier {command}` runs on"
                )) && run.stderr.contains(&format!("Nothing changed.\n{why}")),
                "{command}: {}",
                run.stderr
            );
        }

        let listed = atelier(&root, "t3", &["status"]);
        assert_eq!(listed.code, 0, "{}", listed.stderr);
        let named = listed.stdout.contains(&format!(
            "Dropped on the base while started; nothing is left to do on them:\n  {ID}: {reason}\n"
        ));
        assert_eq!(named, terminal == "dropped", "{}", listed.stdout);
        let open = atelier(&root, "t3", &["list"]);
        assert_eq!(open.code, 0, "{}", open.stderr);
        assert!(!open.stdout.contains(ID), "{}", open.stdout);
        let done = atelier(&root, "t3", &["list", "--done"]);
        assert_eq!(done.code, 0, "{}", done.stderr);
        assert!(
            done.stdout
                .contains(&format!("    {ID} ({terminal}, high): Say hello\n")),
            "{}",
            done.stdout
        );
        // `check` on the branch still reports the record it cannot read.
        let check = atelier(&work, "t1", &["check"]);
        let said = check.stdout + &check.stderr;
        assert_eq!(check.code, 1, "{said}");
        assert!(said.contains("unknown variant `broken`"), "{said}");

        // Once the branch is merged, its checkout still holds the broken
        // record, and `show` there still reads the base's.
        if terminal == "done" {
            git(
                &root,
                &["merge", "-q", "--no-ff", "-s", "ours", "-m", "Merge", ID],
            );
            let shown = atelier(&work, "t1", &["show", ID]);
            assert_eq!(shown.code, 0, "{}", shown.stderr);
            assert!(shown.stdout.contains("Status: done\n"), "{}", shown.stdout);
        }
    }
}

#[test]
fn an_item_only_on_its_branch_is_dropped_with_its_definition_and_not_its_reviews() {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    let work = scratch.worktree("work");
    let started = atelier(&work, "t1", &["start", "--new", "Sneeze"]);
    assert_eq!(started.code, 0, "{}", started.stderr);
    let id = git(&work, &["branch", "--show-current"]);
    let path = item_path(&id);
    let text = read(&work, &path)
        .replace("## Goal\n", "## Goal\n\nSneeze.\n")
        .replace("## Done when\n", "## Done when\n\n- [x] It sneezes.\n");
    fs::write(work.join(&path), text + "\n## Report\n\nIt sneezes.\n").unwrap();
    git(&work, &["commit", "-qam", "Sneeze"]);
    assert_eq!(atelier(&work, "t1", &["submit"]).code, 0);
    // Reviewed, with a fix-now finding the Report answers.
    assert_eq!(atelier(&work, "t2", &["review", &id]).code, 0);
    let review = files_in(&work, &format!(".atelier/items/{id}/reviews")).remove(0);
    append(&work, &review, &finding("Too loud"));
    assert_eq!(verdict(&work, "t2", &["verdict", "changes"]).code, 0);
    let stem = review
        .rsplit('/')
        .next()
        .unwrap()
        .strip_suffix(".md")
        .unwrap();
    append(
        &work,
        &path,
        &format!("\n### Responses\n\n- {stem}.1: fixed in abc1234\n"),
    );
    git(&work, &["commit", "-qam", "Answer"]);
    // Dropped on its branch first, by a thread in its checkout.
    let run = atelier(
        &work,
        "p1",
        &["drop", &id, "--reason", "Nobody asked for it"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    shell(&work, &then(&run.stdout));
    let branch = tip(&work, &id);

    // Then the base's copy, which no commit on the base held before.
    let run = plan(&root, &["drop", &id, "--reason", "Nobody asked for it"]);
    assert_eq!(wrote(&run.stdout), std::slice::from_ref(&path));
    // It commits the base record it wrote, which no commit held before.
    assert_eq!(
        git(&root, &["show", "--name-only", "--format=%s", "HEAD"]),
        format!("Drop {id}\n\n{path}")
    );
    assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");
    let text = read(&root, &path);
    assert_eq!(front(&text, "status"), "dropped");
    assert_eq!(front(&text, "implementers"), "t1");
    assert!(
        text.ends_with(
            "\n## Goal\n\nSneeze.\n\n## Done when\n\n- [x] It sneezes.\n\n## Scope\n\n## Report\n\n\
             Dropped: Nobody asked for it\n"
        ),
        "{text}"
    );
    assert!(files_in(&root, &format!(".atelier/items/{id}/reviews")).is_empty());
    assert_eq!(tip(&work, &id), branch);
    let status = atelier(&work, "t1", &["status"]).stdout;
    assert!(
        status.contains("It was dropped: Nobody asked for it\n"),
        "{status}"
    );
}

#[test]
fn close_ends_an_item_with_no_branch_with_its_result_and_names_merge_for_a_started_one() {
    let scratch = Scratch::new(&[(ID, "ready"), ("demo-st4r", "ready"), ("demo-d0ne", "done")]);
    let root = scratch.root();
    assert_eq!(
        atelier(&scratch.worktree("work"), "t1", &["start", "demo-st4r"]).code,
        0
    );
    let before = read(&root, &item_path(ID));

    let run = planned(&root, "p1", &["close", ID]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier close {ID}` needs the item's result, the text its Report gets. Nothing \
             changed.\nThen: atelier close {ID} \"<what came of it>\"\n"
        )
    );
    let run = planned(&root, "p1", &["close", "demo-st4r", "Nothing to do"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        "`atelier close demo-st4r` refused; nothing changed.\nBranch demo-st4r exists, so \
         demo-st4r was started, and a started item ends when `atelier merge` merges it from that \
         branch. If it will not be done, drop it instead.\nThen: atelier drop demo-st4r --reason \
         \"<why>\"\n"
            .to_string()
            + SKIP
    );
    let run = planned(&root, "p1", &["close", "demo-d0ne", "Again"]);
    assert_eq!(run.code, 1);
    assert!(run.stderr.starts_with(
        "demo-d0ne is done, and `atelier close` runs on an item that is draft, ready. Nothing changed.\n"
    ));
    assert_eq!(read(&root, &item_path(ID)), before);

    // An item with no files to change is done on the base, with its result.
    let result = "Nothing to change: the greeting is already printed.";
    let run = plan(&root, &["close", ID, result]);
    assert!(run
        .stdout
        .starts_with(&format!("Closed item {ID}: it is done.\n")));
    // In one commit of its one path.
    assert_eq!(
        git(&root, &["show", "--name-only", "--format=%s", "HEAD"]),
        format!("Close {ID}\n\n{}", item_path(ID))
    );
    let text = read(&root, &item_path(ID));
    assert_eq!(front(&text, "status"), "done");
    assert!(
        text.ends_with(&format!("## Report\n\n{result}\n")),
        "{text}"
    );
    assert_eq!(field(&root, "main", "status"), "done");
    assert!(!branch_exists(&root, ID));
}

#[test]
fn a_mission_goes_from_draft_through_its_items_to_closed() {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    let started = atelier(&root, "p1", &["start", "--mission", "Greet everyone"]);
    assert_eq!(started.code, 0, "{}", started.stderr);
    let mission = started
        .stdout
        .split_whitespace()
        .nth(3)
        .unwrap()
        .to_string();
    // Its items are planned on its branch.
    let a = created(&plan(&root, &["new", "Say hello", "--in", &mission]));
    let b = created(&plan(&root, &["new", "Say goodbye", "--in", &mission]));
    let path = root.join(item_path(&a));
    let text = fs::read_to_string(&path).unwrap();
    let defined = text
        .replace("## Goal\n", "## Goal\n\nSay hello.\n")
        .replace(
            "## Done when\n",
            "## Done when\n\n- [ ] `hello.txt` says hello.\n",
        );
    fs::write(&path, defined).unwrap();
    commit_all(&root, "Define it");
    // A plan's merge, with its reviews, is atelier-2vdq's; `merge` refuses
    // it until then, so a plain merge stands in for it here.
    let refused = atelier(&root, "p1", &["merge"]);
    assert_eq!(refused.code, 1, "{}", refused.stdout);
    assert!(
        refused
            .stderr
            .contains(&format!("  adds the mission, of mission {mission}\n")),
        "{}",
        refused.stderr
    );
    git(&root, &["checkout", "-q", "--detach", "main"]);
    git(
        &root,
        &["merge", "-q", "--no-ff", "-m", "Merge the plan", &mission],
    );
    git(&root, &["branch", "-f", "main", "HEAD"]);
    let run = planned(&root, "p1", &["close", &mission]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "Mission {mission} is draft, and `atelier close` runs on a mission that is ready. \
             Nothing changed.\nA planner makes it ready with: atelier ready {mission} --by <name>\n{SKIP}"
        )
    );
    let ready = plan(&root, &["ready", &mission, &a, "--by", "sumeet"]);
    assert!(ready
        .stdout
        .contains(&format!("Mission {mission}'s plan has no review.\n")));

    // The item's work, through review to its merge.
    let work = scratch.worktree("work");
    assert_eq!(atelier(&work, "t1", &["start", &a]).code, 0);
    fs::write(work.join("hello.txt"), "hello\n").unwrap();
    let done = fs::read_to_string(work.join(item_path(&a)))
        .unwrap()
        .replace("- [ ]", "- [x]");
    fs::write(
        work.join(item_path(&a)),
        done + "\n## Report\n\nIt says hello.\n",
    )
    .unwrap();
    git(&work, &["add", "-A"]);
    git(&work, &["commit", "-qm", "Say hello"]);
    assert_eq!(atelier(&work, "t1", &["submit"]).code, 0);
    assert_eq!(atelier(&work, "t2", &["review", &a]).code, 0);
    assert_eq!(verdict(&work, "t2", &["verdict", "approve"]).code, 0);
    let merged = atelier(&work, "t1", &["merge"]);
    assert_eq!(merged.code, 0, "{}", merged.stderr);
    // Planning goes on from the base the merge left.
    git(&root, &["checkout", "-q", "--detach", "main"]);

    // The other item is a draft and the Result empty: close lists both.
    let run = planned(&root, "p1", &["close", &mission]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier close {mission}` refused; nothing changed.\nTo close it:\n  [ ] {b} is done \
             or dropped (it is draft)\n  [ ] Result: written under `## Result`\nThen: atelier \
             close {mission}\n{SKIP}"
        )
    );
    plan(&root, &["drop", &b, "--reason", "Nobody leaves"]);
    let path = root.join(mission_path(&mission));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text + "\nEveryone is greeted.\n").unwrap();
    commit_all(&root, "Result");
    let run = planned(&root, "p1", &["close", &mission, "text"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.contains("whose result is written in"),
        "{}",
        run.stderr
    );

    let run = plan(&root, &["close", &mission]);
    assert!(run
        .stdout
        .starts_with(&format!("Closed mission {mission}: it is done.\n")));
    let text = git(
        &root,
        &["show", &format!("HEAD:{}", mission_path(&mission))],
    );
    assert_eq!(front(&text, "status"), "done");
}

#[test]
fn review_of_a_mission_opens_a_plan_review_and_its_verdict_commits_only_that_file() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    write_mission(&root, MISSION, "draft");
    join_mission(&root, ID, MISSION);
    fs::write(
        root.join(format!(".atelier/missions/{MISSION}/design.md")),
        "Shape\n",
    )
    .unwrap();
    commit_all(&root, "Plan");
    let head = tip(&root, "HEAD");
    let reviews = plan_reviews(MISSION);

    // Not on an item's branch, where `verdict` is the item's.
    git(&root, &["branch", ID]);
    let on_item = scratch.dir.path().join("on-item");
    git(
        &root,
        &["worktree", "add", "-q", on_item.to_str().unwrap(), ID],
    );
    let refused = atelier(&on_item, "r1", &["review", MISSION]);
    assert_eq!(refused.code, 1);
    assert!(
        refused
            .stderr
            .ends_with(&format!("Then: atelier review {MISSION}\n")),
        "{}",
        refused.stderr
    );
    assert!(files_in(&on_item, &reviews).is_empty());

    let first = atelier(&root, "r1", &["review", MISSION]);
    assert_eq!(first.code, 0, "{}", first.stderr);
    let files = files_in(&root, &reviews);
    assert_eq!(files.len(), 1);
    let file = &files[0];
    let stem = file
        .rsplit('/')
        .next()
        .unwrap()
        .strip_suffix(".md")
        .unwrap()
        .to_string();
    assert!(stem.starts_with("1-"), "{stem}");
    let text = read(&root, file);
    assert_eq!(
        (
            front(&text, "round"),
            front(&text, "commit"),
            front(&text, "reviewers"),
            front(&text, "verdict")
        ),
        ("1".into(), head.clone(), "r1".into(), "null".into())
    );
    for shown in [
        format!("You are a reviewer of the plan of mission {MISSION} (thread r1), round 1.\n"),
        "\n## Outcome\n\nGreetings.\n".to_string(),
        format!("Its other documents:\n  .atelier/missions/{MISSION}/design.md\n"),
        format!("Its items:\n  {} (draft): Say hello\n", item_path(ID)),
        format!("Your review file: {file}\n"),
        "  atelier verdict changes --model <your model>   if the file holds an open fix-now finding\n".into(),
        "  atelier verdict approve --model <your model>   if it holds none\n".into(),
    ] {
        assert!(first.stdout.contains(&shown), "{shown}\n---\n{}", first.stdout);
    }
    // Run again by its reviewer, it prints the brief and opens nothing.
    let again = atelier(&root, "r1", &["review", MISSION]);
    assert_eq!((again.code, &again.stdout), (0, &first.stdout));
    assert_eq!(files_in(&root, &reviews), files);

    // Another thread takes over the file with no verdict, keeping it.
    let taken = atelier(&root, "r2", &["review", MISSION]);
    assert!(
        taken.stdout.starts_with(&format!(
            "Took over round 1's review of {MISSION}; r1 held it."
        )),
        "{}",
        taken.stdout
    );
    assert_eq!(front(&read(&root, file), "reviewers"), "r1 r2");
    assert_eq!(files_in(&root, &reviews), files);

    // An open fix-now finding refuses approve; changes commits the file alone.
    append(&root, file, &finding("No sizing"));
    fs::write(root.join("notes.txt"), "not the review's\n").unwrap();
    let refused = verdict(&root, "r2", &["verdict", "approve"]);
    assert_eq!(refused.code, 1);
    assert!(
        refused.stderr.contains(&format!(
            "  [ ] No fix-now finding is open; {stem}.1 is: No sizing\n"
        )),
        "{}",
        refused.stderr
    );
    assert_eq!(tip(&root, "HEAD"), head);
    let run = atelier(
        &root,
        "r2",
        &["verdict", "changes", "--model", "gpt-6.1-sol"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let message = format!("Review the plan of {MISSION}, round 1: changes requested");
    assert_eq!(
        run.stdout,
        format!(
            "Recorded your verdict on the plan of mission {MISSION}, round 1: changes requested. \
             Wrote, and did not commit:\n  {file}\nCommit it alone, and push it to the branch of \
             the plan you reviewed; your review is done once it is there. Tell whoever started \
             you the commit, and stop.\nThen: git add -- {file} && git commit -m \"{message}\" -- {file}\n"
        )
    );
    // Nothing is committed until the reviewer runs that line.
    assert_eq!(tip(&root, "HEAD"), head);
    shell(&root, &then(&run.stdout));
    assert_eq!(
        git(&root, &["show", "--name-only", "--format=", "HEAD"]),
        *file
    );
    assert_eq!(
        front(&read(&root, &mission_path(MISSION)), "status"),
        "draft"
    );
    assert_eq!(front(&read(&root, file), "verdict"), "changes");

    // Every file of round 1 has its verdict, so the next review opens round
    // 2, carrying the open finding with no author's response.
    let next = atelier(&root, "r3", &["review", MISSION]);
    assert_eq!(next.code, 0, "{}", next.stderr);
    let round2: Vec<String> = files_in(&root, &reviews)
        .into_iter()
        .filter(|f| *f != *file)
        .collect();
    assert_eq!(round2.len(), 1);
    assert!(round2[0].contains("/reviews/2-"), "{round2:?}");
    let text = read(&root, &round2[0]);
    assert!(
        text.contains(&format!(
            "## Finding 1: No sizing\nRecommendation: fix now\nWas: {stem}.1\nNow: kept\n"
        )),
        "{text}"
    );
    assert!(next
        .stdout
        .contains(&format!("\n{stem}.1, carried as Finding 1: No sizing\n")));
}

#[test]
fn a_plan_review_shows_the_plan_as_its_commit_holds_it_and_names_what_is_not_committed() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    write_mission(&root, MISSION, "draft");
    join_mission(&root, ID, MISSION);
    let design = format!(".atelier/missions/{MISSION}/design.md");
    fs::write(root.join(&design), "Shape\n").unwrap();
    commit_all(&root, "Plan");
    let head = tip(&root, "HEAD");

    // Edited and not committed: the Outcome, an item's title and a new
    // document.
    let mission = read(&root, &mission_path(MISSION)).replace("Greetings.", "Farewells.");
    fs::write(root.join(mission_path(MISSION)), mission).unwrap();
    let item = read(&root, &item_path(ID)).replace("title: Say hello", "title: Say goodbye");
    fs::write(root.join(item_path(ID)), item).unwrap();
    let sizing = format!(".atelier/missions/{MISSION}/sizing.md");
    fs::write(root.join(&sizing), "Small\n").unwrap();

    let run = atelier(&root, "r1", &["review", MISSION]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let brief = run.stdout;
    for shown in [
        format!("Review the plan as commit {} holds it", &head[..7]),
        "\n## Outcome\n\nGreetings.\n".to_string(),
        format!(
            "Its other documents:\n  {design}\nIts items:\n  {} (draft): Say hello\n",
            item_path(ID)
        ),
        format!(
            "This checkout changes the plan from commit {}, and this review does not cover those \
             changes: {}, {}, {sizing}\n",
            &head[..7],
            item_path(ID),
            mission_path(MISSION)
        ),
    ] {
        assert!(brief.contains(&shown), "{shown}\n---\n{brief}");
    }
    assert!(
        !brief.contains("Farewells") && !brief.contains("Say goodbye"),
        "{brief}"
    );
    let files = files_in(&root, &plan_reviews(MISSION));
    assert_eq!(front(&read(&root, &files[0]), "commit"), head);

    // A change only the index holds, with the file in the checkout put back
    // as committed, is named too.
    git(&root, &["checkout", "HEAD", "--", &mission_path(MISSION)]);
    fs::remove_file(root.join(&sizing)).unwrap();
    git(&root, &["add", "--", &item_path(ID)]);
    let committed = git_output(&root, &["show", &format!("HEAD:{}", item_path(ID))]).stdout;
    fs::write(root.join(item_path(ID)), committed).unwrap();
    let staged = atelier(&root, "r1", &["review", MISSION]);
    assert_eq!(staged.code, 0, "{}", staged.stderr);
    assert!(
        staged.stdout.contains(&format!(
            "this review does not cover those changes: {}\n",
            item_path(ID)
        )),
        "{}",
        staged.stdout
    );
    git(&root, &["checkout", "HEAD", "--", &item_path(ID)]);

    // An item of the plan whose record here does not parse is named, and the
    // brief still reads the commit.
    let broken = read(&root, &item_path(ID)).replace("status: draft", "status: broken");
    fs::write(root.join(item_path(ID)), broken).unwrap();
    let unread = atelier(&root, "r1", &["review", MISSION]);
    assert_eq!(unread.code, 0, "{}", unread.stderr);
    for shown in [
        format!("  {} (draft): Say hello\n", item_path(ID)),
        format!(
            "this review does not cover those changes: {}\n",
            item_path(ID)
        ),
    ] {
        assert!(
            unread.stdout.contains(&shown),
            "{shown}\n---\n{}",
            unread.stdout
        );
    }
    git(&root, &["checkout", "HEAD", "--", &item_path(ID)]);

    // A mission no commit holds has no plan to review.
    let other = "demo-m2ss";
    write_mission(&root, other, "draft");
    let refused = atelier(&root, "r1", &["review", other]);
    assert_eq!(refused.code, 1);
    assert_eq!(
        refused.stderr,
        format!(
            "`atelier review {other}` reviews the plan as a commit holds it, and commit {} holds \
             no mission {other}. Nothing changed.\nCommit the plan first: git add -- \
             .atelier/missions/{other} && git commit -m \"Plan {other}\" -- .atelier/missions/{other}\n\
             Then: atelier review {other}\n",
            &head[..7]
        )
    );
    assert!(files_in(&root, &plan_reviews(other)).is_empty());
}

#[test]
fn two_blind_plan_reviews_each_commit_their_file_and_ready_says_what_they_found() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    write_mission(&root, MISSION, "draft");
    join_mission(&root, ID, MISSION);
    commit_all(&root, "Plan");
    let reviewed = tip(&root, "HEAD");

    // Each reviewer in a detached worktree of the same commit, blind.
    let mut commits = Vec::new();
    for (name, thread, verdict) in [("a", "r1", "approve"), ("b", "r2", "changes")] {
        let at = scratch.worktree(name);
        let run = atelier(&at, thread, &["review", MISSION]);
        assert_eq!(run.code, 0, "{}", run.stderr);
        let files = files_in(&at, &plan_reviews(MISSION));
        assert_eq!(files.len(), 1, "{name} sees only its own");
        if verdict == "changes" {
            append(&at, &files[0], &finding("No sizing"));
        }
        // The reviewer commits its file with the line `verdict` printed.
        let run = common::verdict(&at, thread, &["verdict", verdict]);
        assert_eq!(run.code, 0, "{}", run.stderr);
        let commit = tip(&at, "HEAD");
        assert_eq!(git(&at, &["rev-parse", "HEAD^"]), reviewed);
        commits.push(commit);
    }
    git(&root, &["cherry-pick", &commits[0], &commits[1]]);
    let files = files_in(&root, &plan_reviews(MISSION));
    assert_eq!(files.len(), 2);
    for file in &files {
        assert_eq!(front(&read(&root, file), "round"), "1", "{file}");
    }

    // An edit to its item since the commit reviewed shows; it refuses nothing.
    let path = root.join(item_path(ID));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace("Print hello.", "Print hello twice.")).unwrap();
    let run = planned(&root, "p1", &["ready", MISSION, "--by", "sumeet"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let short = &reviewed[..7];
    for file in &files {
        let text = read(&root, file);
        let stem = file
            .rsplit('/')
            .next()
            .unwrap()
            .strip_suffix(".md")
            .unwrap();
        let (by, verdict) = match front(&text, "verdict").as_str() {
            "approve" => ("r1", "approved"),
            _ => ("r2", "changes requested"),
        };
        assert!(
            run.stdout
                .contains(&format!("  {stem} by {by}: {verdict}, of commit {short}\n")),
            "{}",
            run.stdout
        );
    }
    assert!(run
        .stdout
        .contains(&format!("Mission {MISSION}'s plan reviews, round 1:\n")));
    assert!(
        run.stdout.contains(&format!(
            "Since {short}, these changed: {}\n",
            item_path(ID)
        )),
        "{}",
        run.stdout
    );
}
