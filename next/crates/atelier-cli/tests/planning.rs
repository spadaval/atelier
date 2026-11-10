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

fn commit_all(root: &Path, message: &str) {
    git(root, &["add", "-A"]);
    git(root, &["commit", "-qm", message]);
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

/// The one path a planning command printed under `Wrote`.
fn wrote(output: &str) -> Vec<String> {
    output
        .lines()
        .skip_while(|line| !line.starts_with("Wrote"))
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
fn new_writes_a_draft_item_or_mission_commits_nothing_and_says_how_to_commit() {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    let before = head_and_index(&root);

    let mission = atelier(&root, "p1", &["new", "--mission", "Greet everyone"]);
    assert_eq!(mission.code, 0, "{}", mission.stderr);
    let paths = wrote(&mission.stdout);
    assert_eq!(paths.len(), 1, "{}", mission.stdout);
    let mission_id = paths[0]
        .strip_prefix(".atelier/missions/")
        .and_then(|rest| rest.strip_suffix("/mission.md"))
        .unwrap()
        .to_string();
    let text = read(&root, &paths[0]);
    assert_eq!(front(&text, "status"), "draft");
    assert_eq!(front(&text, "title"), "Greet everyone");
    assert!(text.ends_with(
        "---\n\n## Why\n\n## Outcome\n\n## Success criteria\n\n## Scope\n\n## Authority\n\n## Result\n"
    ));

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
    let left = git(&root, &["status", "--porcelain", "-uall"]);
    assert_eq!(left, format!("?? {}", mission_path(&mission_id)));
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
                 checkout. `atelier new --mission \"<title>\"` creates one.\n"
            )
        );
        assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");
    }
}

#[test]
fn the_planning_commands_run_only_on_the_base_branch() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    git(&root, &["switch", "-q", "-c", "elsewhere"]);
    for args in [
        &["new", "Say hello"][..],
        &["set", ID, "--title", "Hi"],
        &["ready", ID, "--by", "sumeet"],
    ] {
        let run = atelier(&root, "p1", args);
        assert_eq!(run.code, 1, "{args:?}");
        assert_eq!(
            run.stderr,
            format!(
                "`atelier {}` writes planning, which happens on the base branch main, and this \
                 checkout is on branch elsewhere. Nothing changed.\nRun it in a checkout of \
                 main, as after `git switch main`.\n",
                args[0]
            )
        );
        assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");
    }
    // Where the base is checked out elsewhere, it is named.
    let base = scratch.dir.path().join("base");
    git(
        &root,
        &["worktree", "add", "-q", base.to_str().unwrap(), "main"],
    );
    let run = atelier(&root, "p1", &["new", "Say hello"]);
    assert!(
        run.stderr.ends_with(&format!(
            "Run it in {}, which has main checked out.\n",
            base.display()
        )),
        "{}",
        run.stderr
    );
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
        assert!(
            run.stderr.starts_with("`atelier set") && run.stderr.ends_with(says),
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
    assert_eq!(atelier(&root, "p1", &["ready", ID, "--by", "s"]).code, 0);
    commit_all(&root, "Ready");
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    git(&root, &["switch", "-q", "main"]);
    let run = atelier(&root, "p1", &["set", ID, "--title", "T"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier set {ID}` refused; nothing changed.\n{ID} is building: it is started, and \
             its record now changes only on branch {ID}, by its implementer.\n"
        )
    );
    assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");
}

#[test]
fn ready_marks_each_named_draft_with_who_and_when_and_changes_no_other_item() {
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
    assert_eq!(
        git(&root, &["status", "--porcelain"]),
        format!("M {}", mission_path(MISSION))
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
    assert_eq!(head_and_index(&root), before);
    shell(&root, &then(&run.stdout));
    assert_eq!(
        git(&root, &["show", "--name-only", "--format=%s", "HEAD"]),
        format!(
            "Ready {ID}, demo-b10c\n\n{}\n{}",
            item_path(ID),
            item_path("demo-b10c")
        )
    );
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

#[test]
fn status_on_the_base_lists_planning_not_yet_committed() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "p1", &["ready", ID, "--by", "s"]).code, 0);
    let new = atelier(&root, "p1", &["new", "--mission", "Greet"]);
    let mission = wrote(&new.stdout).pop().unwrap();
    // Not planning: a change outside the records.
    fs::write(root.join("notes.txt"), "x").unwrap();

    let status = atelier(&root, "p1", &["status"]).stdout;
    let paths = [item_path(ID), mission.clone()].join(" ");
    assert_eq!(
        status,
        format!(
            "{NO_HOOKS}You are thread p1, on the base branch main.\nPlanning not committed yet, which \
             `atelier start` does not read until it is: {}, {mission}\nCommit it with: git add \
             -- {paths} && git commit -m \"Plan\" -- {paths}\nNothing is ready to start.\n",
            item_path(ID)
        )
    );
    shell(
        &root,
        &format!("git add -- {paths} && git commit -m \"Plan\" -- {paths}"),
    );
    let status = atelier(&root, "p1", &["status"]).stdout;
    assert!(!status.contains("not committed"), "{status}");
    assert!(
        status.ends_with(&format!("Then: atelier start {ID}\n")),
        "{status}"
    );
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
             {MISSION} --by <name>)\nThen: atelier start {ID}\n"
        )
    );
    assert!(!branch_exists(&root, ID));
    // `status` lists the same condition.
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.contains(&format!("    [ ] Mission {MISSION} is ready (it is draft;")),
        "{status}"
    );
    assert!(status.ends_with("Nothing is ready to start until what blocks it is done.\n"));

    assert_eq!(
        atelier(&root, "p1", &["ready", MISSION, "--by", "s"]).code,
        0
    );
    commit_all(&root, "Ready");
    let run = atelier(&root, "t1", &["start", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

#[test]
fn start_says_when_an_item_or_its_mission_is_ready_in_the_base_checkout_and_not_committed() {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    write_mission(&root, MISSION, "draft");
    commit_all(&root, "Mission");

    // A new item, made ready, nothing committed: the base has no record.
    let new = atelier(&root, "p1", &["new", "Say hello", "--in", MISSION]);
    let item = wrote(&new.stdout).pop().unwrap();
    let id = item.split('/').nth(2).unwrap().to_string();
    assert_eq!(
        atelier(&root, "p1", &["ready", &id, MISSION, "--by", "s"]).code,
        0
    );
    let paths = [item.clone(), mission_path(MISSION)].join(" ");
    let run = atelier(&root, "t1", &["start", &id]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier start {id}` refused; nothing changed.\n{id} is ready in this checkout, but \
             not in main's last commit, which is what `atelier start` reads: the change is not \
             committed yet.\nCommit the planning with: git add -- {paths} && git commit -m \
             \"Plan\" -- {paths}\nThen: atelier start {id}\n"
        )
    );

    // The item committed as a draft, then made ready, and the mission
    // still ready only in the checkout; asked from another worktree.
    let ready = read(&root, &item);
    fs::write(
        root.join(&item),
        ready.replace("status: ready", "status: draft"),
    )
    .unwrap();
    git(&root, &["add", &item]);
    git(&root, &["commit", "-qm", "Draft", "--", &item]);
    fs::write(root.join(&item), ready).unwrap();
    let other = scratch.worktree("other");
    let run = atelier(&other, "t1", &["start", &id]);
    assert_eq!(run.code, 1);
    let git_c = format!("git -C {}", root.display());
    assert_eq!(
        run.stderr,
        format!(
            "`atelier start {id}` refused; nothing changed.\n{id} and mission {MISSION} are \
             ready in the checkout of main at {}, but not in main's last commit, which is what \
             `atelier start` reads: the change is not committed yet.\nCommit the planning with: \
             {git_c} add -- {paths} && {git_c} commit -m \"Plan\" -- {paths}\nThen: atelier \
             start {id}\n",
            root.display()
        )
    );
    assert!(!branch_exists(&root, &id));

    // The item's readiness committed, and the mission's not.
    git(&root, &["commit", "-qm", "Ready item", "--", &item]);
    let run = atelier(&root, "t1", &["start", &id]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.contains(&format!(
            "\nmission {MISSION} is ready in this checkout, but not in main's last commit"
        )),
        "{}",
        run.stderr
    );

    // Committed with the line it gave, `start` goes ahead.
    let line = run
        .stderr
        .lines()
        .find_map(|l| l.strip_prefix("Commit the planning with: "));
    shell(&root, line.unwrap());
    let run = atelier(&root, "t1", &["start", &id]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), id);
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

    // Once its Goal and a Done when line are written, the line goes.
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    let id = started_id(&atelier(&root, "t1", &["start", "--new", "Fix it"]).stdout);
    let path = item_path(&id);
    let text = read(&root, &path).replace("## Goal\n", "## Goal\n\nFix it.\n");
    fs::write(
        root.join(&path),
        text.replace("## Done when\n", "## Done when\n\n- [ ] Fixed.\n"),
    )
    .unwrap();
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(!status.contains("Write its Goal"), "{status}");
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
    assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), "main");
    assert_eq!(git(&root, &["status", "--porcelain", "-uall"]), "");
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert_eq!(
        status,
        format!(
            "{NO_HOOKS}You are thread t1, on the base branch main.\nNothing is ready to start.\n"
        )
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

    // The base checked out at a path with a space, with an uncommitted
    // mission document whose name has a space and a quote; `start` is run
    // from another checkout.
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    git(&root, &["switch", "-q", "--detach"]);
    let base = scratch.dir.path().join("base copy");
    git(
        &root,
        &["worktree", "add", "-q", base.to_str().unwrap(), "main"],
    );
    write_mission(&base, MISSION, "draft");
    join_mission(&base, ID, MISSION);
    commit_all(&base, "Plan");
    assert_eq!(
        atelier(&base, "p1", &["ready", ID, MISSION, "--by", "s"]).code,
        0
    );
    let document = format!(".atelier/missions/{MISSION}/design notes 'v2'.md");
    fs::write(base.join(&document), "# Design\n").unwrap();

    let status = atelier(&base, "p1", &["status"]).stdout;
    let line = status
        .lines()
        .find_map(|l| l.strip_prefix("Commit it with: "))
        .unwrap();
    assert!(
        line.contains("'.atelier/missions/demo-m1ss/design notes '\\''v2'\\''.md'"),
        "{line}"
    );

    let run = atelier(&root, "t1", &["start", ID]);
    assert_eq!(run.code, 1);
    let line = run
        .stderr
        .lines()
        .find_map(|l| l.strip_prefix("Commit the planning with: "))
        .unwrap();
    assert!(
        line.starts_with(&format!("git -C '{}' add -- ", base.display())),
        "{line}"
    );
    shell(&root, line);
    assert_eq!(git(&base, &["status", "--porcelain", "-uall"]), "");
    assert!(
        git(&base, &["show", "--name-only", "--format=", "HEAD"]).contains("design notes 'v2'.md")
    );
    let run = atelier(&root, "t1", &["start", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

/// The ID of the record a planning command created.
fn created(run: &Run) -> String {
    assert_eq!(run.code, 0, "{}", run.stderr);
    let path = wrote(&run.stdout).pop().unwrap();
    path.split('/').nth(2).unwrap().to_string()
}

/// Runs a planning command as `p1` on the base and commits what it wrote
/// with the line it printed.
fn plan(root: &Path, args: &[&str]) -> Run {
    let run = atelier(root, "p1", args);
    assert_eq!(run.code, 0, "{args:?}: {}", run.stderr);
    shell(root, &then(&run.stdout));
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
        "Dropped item {ID}: The site greets instead\nWrote"
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
        let run = atelier(&root, "p1", &["drop", id, "--reason", "again"]);
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
    assert_eq!(git(&root, &["branch", "--show-current"]), "main");
    let run = atelier(&root, "p1", &["drop", MISSION, "--reason", "again"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "Mission {MISSION} is dropped, and `atelier drop` runs on a mission that is draft, \
             ready. Nothing changed.\nNothing is left to do on it.\n"
        )
    );
}

#[test]
fn a_started_item_dropped_on_the_base_is_refused_everywhere_with_its_reason() {
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
    assert_eq!(atelier(&work, "t2", &["verdict", "approve"]).code, 0);
    let branch = tip(&work, ID);

    let reason = "The site greets instead";
    let run = atelier(&root, "p1", &["drop", ID, "--reason", reason]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(run
        .stdout
        .contains(&format!("Its branch {ID} is left as it is.")));
    // The base's record, ready and held by nobody, whatever the branch holds.
    let text = read(&root, &item_path(ID));
    assert_eq!(front(&text, "status"), "dropped");
    assert_eq!(front(&text, "implementers"), "[]");
    assert_eq!(tip(&work, ID), branch);
    shell(&root, &then(&run.stdout));
    let base = tip(&root, "main");

    let listed = atelier(&root, "t3", &["status"]).stdout;
    assert!(
        listed.contains(&format!(
            "Dropped on the base while started; nothing is left to do on them:\n  {ID}: {reason}\n"
        )),
        "{listed}"
    );
    let why = format!("It was dropped on the base: {reason}\nNothing is left to do on it.\n");
    let status = atelier(&work, "t1", &["status"]).stdout;
    assert!(
        status.starts_with(&format!(
            "{NO_HOOKS}You are thread t1, on the branch of {ID}.\n"
        )),
        "{status}"
    );
    assert!(
        status.ends_with(&format!("Status: dropped\n{why}")),
        "{status}"
    );

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
            run.stderr
                .starts_with(&format!("{ID} is dropped, and `atelier {command}` runs on"))
                && run.stderr.ends_with(&format!("Nothing changed.\n{why}")),
            "{command}: {}",
            run.stderr
        );
        assert_eq!(
            (tip(&work, ID), tip(&root, "main")),
            (branch.clone(), base.clone())
        );
        assert_eq!(git(&work, &["status", "--porcelain"]), "");
        assert_eq!(git(&root, &["status", "--porcelain"]), "");
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
    assert_eq!(atelier(&work, "t2", &["verdict", "changes"]).code, 0);
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
    let branch = tip(&work, &id);

    let run = plan(&root, &["drop", &id, "--reason", "Nobody asked for it"]);
    assert_eq!(wrote(&run.stdout), std::slice::from_ref(&path));
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
        status.contains("It was dropped on the base: Nobody asked for it\n"),
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

    let run = atelier(&root, "p1", &["close", ID]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier close {ID}` needs the item's result, the text its Report gets. Nothing \
             changed.\nThen: atelier close {ID} \"<what came of it>\"\n"
        )
    );
    let run = atelier(&root, "p1", &["close", "demo-st4r", "Nothing to do"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        "`atelier close demo-st4r` refused; nothing changed.\nBranch demo-st4r exists, so \
         demo-st4r was started, and a started item ends when `atelier merge` merges it from that \
         branch. If it will not be done, drop it instead.\nThen: atelier drop demo-st4r --reason \
         \"<why>\"\n"
    );
    let run = atelier(&root, "p1", &["close", "demo-d0ne", "Again"]);
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
    let mission = created(&plan(&root, &["new", "--mission", "Greet everyone"]));
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
    let run = atelier(&root, "p1", &["close", &mission]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "Mission {mission} is draft, and `atelier close` runs on a mission that is ready. \
             Nothing changed.\nA planner makes it ready with: atelier ready {mission} --by <name>\n"
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
    assert_eq!(atelier(&work, "t2", &["verdict", "approve"]).code, 0);
    let merged = atelier(&work, "t1", &["merge"]);
    assert_eq!(merged.code, 0, "{}", merged.stderr);

    // The other item is a draft and the Result empty: close lists both.
    let run = atelier(&root, "p1", &["close", &mission]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier close {mission}` refused; nothing changed.\nTo close it:\n  [ ] {b} is done \
             or dropped (it is draft)\n  [ ] Result: written under `## Result`\nThen: atelier \
             close {mission}\n"
        )
    );
    plan(&root, &["drop", &b, "--reason", "Nobody leaves"]);
    let path = root.join(mission_path(&mission));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text + "\nEveryone is greeted.\n").unwrap();
    commit_all(&root, "Result");
    let run = atelier(&root, "p1", &["close", &mission, "text"]);
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
    let refused = atelier(&root, "r2", &["verdict", "approve"]);
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
    let commit = tip(&root, "HEAD");
    assert_eq!(
        run.stdout,
        format!(
            "Recorded your verdict on the plan of mission {MISSION}, round 1: changes requested. \
             Committed {file} alone, in commit {commit}.\nIt is on main.\nYour review is done: \
             tell whoever started you that commit, and stop.\n"
        )
    );
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
fn two_blind_plan_reviews_cherry_pick_onto_the_base_and_ready_says_what_they_found() {
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
        let run = atelier(&at, thread, &["verdict", verdict]);
        assert_eq!(run.code, 0, "{}", run.stderr);
        let commit = tip(&at, "HEAD");
        assert!(run.stdout.contains(&format!(
            "The planner puts it on main with: git cherry-pick {commit}\n"
        )));
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
    let run = atelier(&root, "p1", &["ready", MISSION, "--by", "sumeet"]);
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
