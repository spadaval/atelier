//! Filing an item with `new` on the branch its thread is on: it reaches
//! the base with that branch's merge, it makes no approval stale, and a
//! drop of that branch loses none unseen. Driven in clones of one bare
//! remote with a fake `gh` and the Git hooks `setup` installs.

mod common;

use common::hub::*;
use common::*;
use std::fs;
use std::path::{Path, PathBuf};

const MISSION: &str = "demo-m1ss";

/// Runs `line` as an agent that pasted it would: as `thread`, with the
/// binary under test as the `atelier` that it and the hooks find.
fn run(hub: &Hub, at: &Path, thread: &str, line: &str) -> Run {
    let gh = hub.gh.display();
    sh(
        at,
        thread,
        &format!("export PATH='{gh}/bin':\"$PATH\" FAKE_GH='{gh}'; {line}"),
    )
}

/// [`run`], which must succeed; what it printed.
fn done(hub: &Hub, at: &Path, thread: &str, line: &str) -> String {
    let run = run(hub, at, thread, line);
    assert_eq!(run.code, 0, "{line}: {}{}", run.stdout, run.stderr);
    run.stdout
}

/// The Git command `stdout` ends its orders with.
fn then(stdout: &str) -> String {
    let line = stdout
        .lines()
        .find_map(|line| line.strip_prefix("Then: git "));
    format!(
        "git {}",
        line.unwrap_or_else(|| panic!("no Then: git in {stdout}"))
    )
}

/// The ID of the item `new` said it created.
fn created(stdout: &str) -> String {
    let rest = stdout.strip_prefix("Created open item ");
    let id = rest.and_then(|rest| rest.split(',').next());
    id.unwrap_or_else(|| panic!("no item in {stdout}"))
        .to_string()
}

/// A clone with Atelier's hooks, as `setup` installs them.
fn hooked(hub: &Hub, name: &str) -> PathBuf {
    let at = hub.clone(name);
    done(hub, &at, "t0", "atelier setup");
    at
}

/// Files an item as `thread` with the command `new`, writes its Goal, and
/// commits it with the command that printed, through the pre-commit hook.
fn file(hub: &Hub, at: &Path, thread: &str, new: &str) -> String {
    let said = done(hub, at, thread, new);
    let id = created(&said);
    let path = at.join(item_path(&id));
    let text = fs::read_to_string(&path).unwrap();
    let written = text.replace("## Goal\n", "## Goal\n\nThe help is wrong.\n");
    fs::write(&path, written).unwrap();
    done(hub, at, thread, &then(&said));
    assert_eq!(
        git(at, &["status", "--porcelain", "--", &item_path(&id)]),
        ""
    );
    id
}

/// t1's item, started, changed and submitted in `at`.
fn submit(hub: &Hub, at: &Path) {
    done(hub, at, "t1", &format!("atelier start {ID}"));
    fs::write(at.join("hello.txt"), "hello\n").unwrap();
    git(at, &["add", "hello.txt"]);
    done(hub, at, "t1", "git commit -qm 'Say hello'");
    hand_in(hub, at);
}

/// t1's item with its Done when lines ticked and its Report written,
/// submitted.
fn hand_in(hub: &Hub, at: &Path) {
    let path = at.join(item_path(ID));
    let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, ticked + "It prints hello.\n").unwrap();
    done(hub, at, "t1", "atelier submit");
}

/// r1's approval of t1's item, committed.
fn approve(hub: &Hub, at: &Path) {
    done(hub, at, "r1", &format!("atelier review {ID}"));
    let said = done(hub, at, "r1", "atelier verdict approve");
    done(hub, at, "r1", &then(&said));
}

/// The front-matter field `key` of item `id` on the remote's `branch`.
fn field_of(hub: &Hub, branch: &str, id: &str, key: &str) -> String {
    let at = format!("{branch}:{}", item_path(id));
    front(&git(&hub.remote, &["show", &at]), key)
}

/// The command the brief gives for an approval.
fn approval_command(brief: &str) -> String {
    let line = (brief.lines())
        .find(|line| line.starts_with("  atelier verdict approve"))
        .unwrap();
    let command = line.trim().split("   ").next().unwrap();
    command.replace(" --model <your model>", "")
}

/// The command the brief or message `text` gives between backquotes that
/// starts with `starts`, with its placeholder filled in.
fn told(text: &str, starts: &str) -> String {
    let found = text.split('`').find(|quoted| quoted.starts_with(starts));
    let found = found.unwrap_or_else(|| panic!("no `{starts}` in {text}"));
    found
        .replace("<title>", "Fix the help")
        .replace("<slug>", "found")
}

#[test]
fn an_item_filed_on_an_items_branch_is_proposed_until_that_merges_and_then_open() {
    let hub = Hub::new(&[(ID, "open")]);
    // The item is in an agreed mission, which what it files does not join.
    let seed = hub.clone("planner");
    let mission = format!(".atelier/missions/{MISSION}/mission.md");
    fs::create_dir_all(seed.join(&mission).parent().unwrap()).unwrap();
    fs::write(
        seed.join(&mission),
        format!("---\nschema: atelier.mission\nschema_version: 3\nid: {MISSION}\ntitle: Greet\n")
            + "status: open\nlabels: []\nlead: null\nplanners:\n- p1\nready: null\n\
               created_at: 2026-11-22T09:00:00Z\n---\n\n## Why\n\nAsked for.\n\n## Outcome\n\nGreetings.\n",
    )
    .unwrap();
    let path = seed.join(item_path(ID));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        text.replace("mission: null", &format!("mission: {MISSION}")),
    )
    .unwrap();
    git(&seed, &["add", "-A"]);
    git(&seed, &["commit", "-qm", "Agree"]);
    git(&seed, &["push", "-q", "origin", "HEAD:main"]);

    let a = hooked(&hub, "a");
    done(&hub, &a, "t1", &format!("atelier start {ID}"));
    let said = done(&hub, &a, "t1", "atelier new 'Fix the help'");
    let found = created(&said);
    let path = item_path(&found);
    assert_eq!(
        said,
        format!(
            "Created open item {found}, with empty Goal, Done when and Scope sections to write.\n\
             Wrote, and did not commit:\n  {path}\n\
             Write it, then commit it: it reaches the base when {ID} merges, and filing it makes no approval of {ID} stale.\n\
             Then: git add -- {path} && git commit -m \"Plan {found}\" -- {path}\n"
        )
    );
    // The hook judges that commit: it refuses the record while it is invalid.
    let text = read(&a, &path);
    fs::write(
        a.join(&path),
        text.replace("priority: medium", "priority: urgent"),
    )
    .unwrap();
    let refused = run(&hub, &a, "t1", &then(&said));
    assert_ne!(refused.code, 0, "{}", refused.stdout);
    fs::write(a.join(&path), &text).unwrap();
    done(&hub, &a, "t1", &then(&said));
    let joined = file(
        &hub,
        &a,
        "t1",
        &format!("atelier new 'Greet twice' --in {MISSION}"),
    );
    done(&hub, &a, "t1", &format!("git push -q origin {ID}"));
    assert_eq!(field_of(&hub, ID, &found, "status"), "open");
    assert_eq!(field_of(&hub, ID, &found, "mission"), "null");
    assert_eq!(field_of(&hub, ID, &found, "implementers"), "[]");
    assert_eq!(field_of(&hub, ID, &joined, "mission"), MISSION);

    // Run elsewhere, once fetched: proposed, with its branch, and not yet
    // anyone's to start.
    let b = hub.clone("b");
    let listed = done(&hub, &b, "t2", "atelier list --proposed");
    let shown = format!("    {found} (open, medium, on origin/{ID}): Fix the help\n");
    assert!(listed.contains(&shown), "{listed}");
    assert!(!done(&hub, &b, "t2", "atelier list --startable").contains(&found));
    assert_ne!(
        run(&hub, &b, "t2", &format!("atelier start {found}")).code,
        0
    );
    assert!(!done(&hub, &a, "t1", "atelier list --mine").contains(&found));

    hand_in(&hub, &a);
    approve(&hub, &a);
    done(&hub, &a, "t1", "atelier merge");

    assert_eq!(field_of(&hub, "main", &found, "status"), "open");
    assert_eq!(field_of(&hub, "main", &found, "implementers"), "[]");
    // Open on the base is all a start needs.
    git(&b, &["fetch", "-q", "origin"]);
    done(&hub, &b, "t2", &format!("atelier start {found}"));
}

#[test]
fn a_reviewer_files_as_its_brief_says_before_or_after_its_verdict() {
    for after in [false, true] {
        let hub = Hub::new(&[(ID, "open")]);
        let at = hooked(&hub, "a");
        submit(&hub, &at);
        let brief = done(&hub, &at, "r1", &format!("atelier review {ID}"));
        assert!(
            brief.contains("Change no other file, except to file a problem outside this item"),
            "{brief}"
        );
        assert!(!brief.contains("and change no other file."), "{brief}");
        let new = told(&brief, "atelier new ");
        let mut found = None;
        if !after {
            found = Some(file(&hub, &at, "r1", &new));
        }
        let verdict = done(&hub, &at, "r1", &approval_command(&brief));
        done(&hub, &at, "r1", &then(&verdict));
        if after {
            found = Some(file(&hub, &at, "r1", &new));
        }
        // Both are committed, and the merge needs no further round.
        assert_eq!(git(&at, &["status", "--porcelain"]), "");
        let status = done(&hub, &at, "t1", "atelier status");
        assert!(status.contains("Then: atelier merge"), "{status}");
        done(&hub, &at, "t1", "atelier merge");
        assert_eq!(field_of(&hub, "main", &found.unwrap(), "status"), "open");
        let files = git(&hub.remote, &["ls-tree", "-r", "--name-only", "main"]);
        assert!(files.contains(&format!("{REVIEWS}/1-")), "{files}");
    }
}

#[test]
fn an_item_filed_after_approval_by_anyone_leaves_the_approval_standing() {
    for thread in ["t1", "r1", "t3"] {
        let hub = Hub::new(&[(ID, "open")]);
        let at = hooked(&hub, "a");
        submit(&hub, &at);
        approve(&hub, &at);
        let found = file(&hub, &at, thread, "atelier new 'Fix the help'");
        let again = file(&hub, &at, thread, "atelier new 'Fix the manual'");
        let status = done(&hub, &at, "t1", "atelier status");
        assert!(status.contains("Then: atelier merge"), "{status}");
        done(&hub, &at, "t1", "atelier merge");
        for id in [found, again] {
            assert_eq!(field_of(&hub, "main", &id, "status"), "open");
        }
    }
}

#[test]
fn an_item_filed_and_not_committed_is_not_left_behind_by_the_merge() {
    let hub = Hub::new(&[(ID, "open")]);
    let at = hooked(&hub, "a");
    submit(&hub, &at);
    approve(&hub, &at);
    let said = done(&hub, &at, "t1", "atelier new 'Fix the help'");
    let path = item_path(&created(&said));
    let refused = run(&hub, &at, "t1", "atelier merge");
    assert_eq!(refused.code, 1, "{}", refused.stdout);
    assert!(refused.stderr.contains(&path), "{}", refused.stderr);
    done(&hub, &at, "t1", &then(&said));
    done(&hub, &at, "t1", "atelier merge");
    assert_eq!(field_of(&hub, "main", &created(&said), "status"), "open");
}

#[test]
fn new_is_refused_on_the_branch_of_a_dropped_item() {
    // Dropped on its branch, or on the base.
    for on_base in [false, true] {
        let hub = Hub::new(&[(ID, "open")]);
        let at = hooked(&hub, "a");
        done(&hub, &at, "t1", &format!("atelier start {ID}"));
        let drop = format!("atelier drop {ID} --reason 'Not needed'");
        if on_base {
            let other = hub.clone("b");
            done(&hub, &other, "p1", "atelier start --backlog drop-it");
            let said = done(&hub, &other, "p1", &drop);
            done(&hub, &other, "p1", &then(&said));
            done(&hub, &other, "p1", "atelier merge");
            git(&at, &["fetch", "-q", "origin"]);
        } else {
            let said = done(&hub, &at, "t1", &drop);
            done(&hub, &at, "t1", &then(&said));
        }
        let before = git(&at, &["status", "--porcelain", "-uall"]);
        let refused = run(&hub, &at, "t1", "atelier new 'Fix the help'");
        assert_eq!(refused.code, 1, "{}", refused.stdout);
        assert_eq!(
            refused.stderr,
            format!(
                "`atelier new` refused; nothing written.\nThis checkout is on {ID}, the branch of item {ID}, which is dropped: only a started item's branch merges, so an item filed here would not reach the base.\nRun it on a backlog branch, made with `atelier start --backlog <slug>`.\n{SKIP}"
            )
        );
        assert_eq!(git(&at, &["status", "--porcelain", "-uall"]), before);
    }
}

#[test]
fn a_drop_names_what_was_filed_committed_or_not_and_its_commands_carry_them() {
    // The drop written on the item's own branch, or on a backlog branch in
    // another checkout of the clone.
    for own in [true, false] {
        let hub = Hub::new(&[(ID, "open")]);
        let at = hooked(&hub, "a");
        done(&hub, &at, "t1", &format!("atelier start {ID}"));
        let committed = file(&hub, &at, "t1", "atelier new 'Fix the help'");
        let loose = created(&done(&hub, &at, "t1", "atelier new 'Fix the manual'"));

        let backlog = hub.dir.path().join("backlog");
        let add = [
            "worktree",
            "add",
            "-q",
            "--detach",
            backlog.to_str().unwrap(),
            "main",
        ];
        git(&at, &add);
        done(&hub, &backlog, "p1", "atelier start --backlog drop-it");
        let dropped_in = if own { &at } else { &backlog };
        let drop = format!("atelier drop {ID} --reason 'Not needed'");
        let said = done(&hub, dropped_in, "t1", &drop);
        for (id, title) in [(&committed, "Fix the help"), (&loose, "Fix the manual")] {
            assert!(said.contains(&format!("  {id}: {title}\n")), "{said}");
        }
        assert_eq!(
            said.matches("It is not committed as it stands.").count(),
            1,
            "{said}"
        );
        done(&hub, dropped_in, "t1", &then(&said));
        if own {
            let carried = done(&hub, &backlog, "p1", &drop);
            done(&hub, &backlog, "p1", &then(&carried));
        }
        // Each command it printed, in its order, run on the backlog branch.
        for line in said
            .lines()
            .filter_map(|line| line.strip_prefix("    git "))
        {
            done(&hub, &backlog, "p1", &format!("git {line}"));
        }
        done(
            &hub,
            &backlog,
            "p1",
            "git commit -qm 'Carry what was filed'",
        );
        done(&hub, &backlog, "p1", "atelier merge");
        assert_eq!(hub.field("main", "status"), "dropped");
        for id in [&committed, &loose] {
            assert_eq!(field_of(&hub, "main", id, "status"), "open");
        }
    }
}

#[test]
fn a_plan_reviewer_publishes_then_files_on_a_backlog_branch_as_its_brief_says() {
    let hub = Hub::new(&[]);
    let planner = hub.clone("planner");
    done(&hub, &planner, "p1", "atelier start --mission Greet");
    let mission = git(&planner, &["branch", "--show-current"]);
    done(&hub, &planner, "p1", "atelier submit");
    let at = hooked(&hub, "reviewer");
    git(
        &at,
        &["checkout", "-q", "--detach", &format!("origin/{mission}")],
    );

    let brief = done(&hub, &at, "r1", &format!("atelier review {mission}"));
    let push = brief
        .find(&format!("Push it to {mission}: "))
        .expect("a push");
    let filing = brief.find("Only once that push is made").expect("filing");
    assert!(push < filing, "{brief}");
    let recorded = done(&hub, &at, "r1", &approval_command(&brief));
    assert!(
        recorded.contains("Only once that push is made"),
        "{recorded}"
    );
    done(&hub, &at, "r1", &then(&recorded));
    let push = brief[push..].lines().next().unwrap();
    done(&hub, &at, "r1", push.split_once(": ").unwrap().1);
    let published = hub.tip(&mission).unwrap();

    let steps = &brief[filing..];
    done(&hub, &at, "r1", &told(steps, "atelier start --backlog"));
    let found = file(&hub, &at, "r1", &told(steps, "atelier new "));
    done(&hub, &at, "r1", &told(steps, "atelier submit"));
    done(&hub, &at, "r1", &told(steps, "atelier merge"));

    assert_eq!(field_of(&hub, "main", &found, "status"), "open");
    assert_eq!(field_of(&hub, "main", &found, "mission"), "null");
    // The plan and the verdict are as they were published.
    assert_eq!(hub.tip(&mission).unwrap(), published);
    assert_eq!(
        git(&hub.remote, &["rev-parse", &format!("{published}^")]),
        git(&planner, &["rev-parse", "HEAD"])
    );
}
