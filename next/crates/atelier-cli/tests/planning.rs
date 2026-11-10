//! The planner's commands, `new`, `set` and `ready`, and what `start`
//! needs of their planning, driven in scratch repositories.

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
            "You are thread p1, on the base branch main.\nPlanning not committed yet, which \
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
        "You are thread t1, on the base branch main.\nNothing is ready to start.\n"
    );
}
