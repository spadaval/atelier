//! `log`, the history of actions, driven through the new binary in scratch
//! repositories: each action found once in Git, with its thread, in order,
//! whether its item's branch is still there or merged and deleted.

mod common;

use common::*;
use std::fs;
use std::path::Path;
use std::process::Command;

const MISSION: &str = "demo-m1ss";
const DONE: &str = "demo-d0ne";
const DONE_REVIEW: &str = ".atelier/items/demo-d0ne/reviews/1-aaaa.md";
const REASON: &str = "Ada said to";

/// Writes item `id` as the binary from before items were open made one
/// ready: `ready`, with who said go.
fn readied(root: &Path, id: &str) {
    let path = root.join(item_path(id));
    let text = fs::read_to_string(&path)
        .unwrap()
        .replace("status: draft", "status: ready")
        .replace(
            "ready: null",
            "ready:\n  by: ada\n  at: 2026-11-10T09:00:00Z",
        );
    fs::write(path, text).unwrap();
}

/// `atelier log --all` with `args`, which must succeed; its lines, every
/// action oldest first and each whole.
fn log(root: &Path, args: &[&str]) -> Vec<String> {
    let mut all = vec!["log", "--all"];
    all.extend(args);
    let run = atelier(root, "t9", &all);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(run.stderr, "");
    run.stdout.lines().map(str::to_string).collect()
}

/// `lines` without their time and commit, after checking both: a time in
/// UTC to the second, and a commit Git knows by that name.
fn actions(root: &Path, lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .map(|line| {
            let (time, rest) = line.split_once(' ').unwrap();
            let (commit, rest) = rest.split_once(' ').unwrap();
            assert!(
                chrono::DateTime::parse_from_rfc3339(time).is_ok() && time.ends_with('Z'),
                "{line}"
            );
            assert_eq!(time.len(), "2026-11-11T09:00:00Z".len(), "{line}");
            assert_eq!(
                git(root, &["rev-parse", "--short", commit]),
                commit,
                "{line}"
            );
            rest.to_string()
        })
        .collect()
}

/// The commit of `line`, as it shows it.
fn commit_of(line: &str) -> &str {
    line.split(' ').nth(1).unwrap()
}

fn short(root: &Path, rev: &str) -> String {
    git(root, &["rev-parse", "--short", rev])
}

fn ok(run: Run) {
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);
}

fn hello(root: &Path) {
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(root, &["add", "hello.txt"]);
    git(root, &["commit", "-qm", "Say hello"]);
}

fn shell(root: &Path, line: &str) {
    let output = identity(Command::new("sh").current_dir(root).args(["-c", line]))
        .output()
        .unwrap();
    assert!(output.status.success(), "{line}: {output:?}");
}

/// The last line of `output`, without its `Then: ` prefix.
fn then(output: &str) -> &str {
    let last = output.lines().last().unwrap();
    last.strip_prefix("Then: ")
        .unwrap_or_else(|| panic!("{output}"))
}

#[test]
fn the_plain_path_prints_its_actions_in_order_and_the_same_lines_once_its_branch_is_merged_and_deleted(
) {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    // History from before items were open keeps its go-ahead's line.
    readied(&root, ID);
    on_base(&root, "Ready it");
    ok(atelier(&root, "t1", &["start", ID]));
    hello(&root);
    finish_and_submit(&root, "t1");
    ok(atelier(&root, "t2", &["review", ID]));
    ok(verdict(&root, "t2", &["verdict", "approve"]));
    let (review, _) = review_of(&root, 1);

    // Before the merge, the branch's actions come from its tip.
    let started = [
        format!("{ID} ready - draft -> ready; by ada"),
        format!("{ID} start t1 ready -> building"),
        format!("{ID} submit t1 building -> in_review"),
        format!("{ID} review t2 {review}"),
        format!("{ID} verdict t2 {review} approve; in_review -> approved"),
    ];
    assert_eq!(actions(&root, &log(&root, &[])), started);
    assert_eq!(actions(&root, &log(&root, &[ID])), started);

    ok(atelier(&root, "t1", &["merge"]));
    let merged = log(&root, &[]);
    let mut all = started.to_vec();
    all.push(format!("{ID} merge t1 approved -> done"));
    assert_eq!(actions(&root, &merged), all);
    // The merge line is the merge commit's; the others are the branch's.
    assert_eq!(commit_of(&merged[5]), short(&root, "main"));
    assert_eq!(commit_of(&merged[4]), short(&root, "main^2"));

    // `merge` deleted the branch, and nothing that moves changes the lines.
    assert!(!branch_exists(&root, ID));
    off_base(&root);
    assert_eq!(log(&root, &[]), merged);
    assert_eq!(log(&root, &[ID]), merged);
    // With no option, the same lines newest first, and nothing older.
    let newest: Vec<String> = merged.iter().rev().cloned().collect();
    for args in [&["log"][..], &["log", ID]] {
        let run = atelier(&root, "t9", args);
        assert_eq!(run.code, 0, "{}", run.stderr);
        assert_eq!(run.stdout.lines().collect::<Vec<_>>(), newest);
    }
}

#[test]
fn with_no_option_log_prints_the_newest_thirty_and_says_how_to_see_the_older_ones() {
    let ids: Vec<String> = (0..32).map(|n| format!("demo-{n:04}")).collect();
    let items: Vec<(&str, &str)> = ids.iter().map(|id| (id.as_str(), "draft")).collect();
    let scratch = Scratch::new(&items);
    let root = scratch.root();
    for id in &ids {
        readied(&root, id);
        on_base(&root, &format!("Ready {id}"));
    }
    let every = log(&root, &[]);
    assert_eq!(every.len(), 32);
    assert!(every[0].contains(" demo-0000 ready "), "{}", every[0]);

    let run = atelier(&root, "t9", &["log"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let lines: Vec<&str> = run.stdout.lines().collect();
    assert_eq!(lines.len(), 31, "{}", run.stdout);
    let newest: Vec<&str> = every.iter().rev().take(30).map(String::as_str).collect();
    assert_eq!(lines[..30], newest[..]);
    assert_eq!(
        lines[30],
        "2 older actions not shown. `atelier log --all` prints every action, oldest first, each line whole."
    );

    // `--since` narrows both, and the command it names keeps it.
    let since = "2000-01-01";
    let run = atelier(&root, "t9", &["log", "--since", since]);
    assert!(
        run.stdout.ends_with(
            "\n2 older actions not shown. `atelier log --since 2000-01-01T00:00:00Z --all` prints every action, oldest first, each line whole.\n"
        ),
        "{}",
        run.stdout
    );
}

#[test]
fn a_line_over_300_characters_is_cut_and_says_where_it_is_whole() {
    let scratch = Scratch::new(&[("demo-b10c", "open")]);
    let root = scratch.root();
    write_item(&root, ID, "open", &[], &["demo-b10c"], "");
    on_base(&root, "Plan a blocked item");
    let reason = "Ada said so, at length. ".repeat(20);
    skipping(&root, "t1", &reason, &["start", ID]);

    let whole = log(&root, &[ID]);
    assert_eq!(whole.len(), 1);
    assert!(
        whole[0].len() > 300 && whole[0].ends_with(" (by t1)"),
        "{}",
        whole[0]
    );
    for (args, command) in [
        (&["log"][..], "atelier log --all"),
        (&["log", ID], &*format!("atelier log {ID} --all")),
    ] {
        let run = atelier(&root, "t9", args);
        assert_eq!(run.code, 0, "{}", run.stderr);
        let line = run.stdout.lines().next().unwrap();
        assert_eq!(run.stdout.lines().count(), 1, "{}", run.stdout);
        assert!(line.chars().count() <= 300, "{line}");
        let tail = format!(" [cut; `{command}` prints it whole]");
        let kept = line.strip_suffix(&tail).unwrap_or_else(|| panic!("{line}"));
        assert!(whole[0].starts_with(kept), "{line}");
    }
}

/// `atelier` as a Codex subagent `child` of the session `s1`, in T3 Code's
/// thread `p1`.
fn codex(at: &Path, child: &str, args: &[&str]) -> Run {
    let output = atelier_command(at, "p1", args)
        .env("CODEX_SESSION_ID", "s1")
        .env("CODEX_THREAD_ID", child)
        .output()
        .unwrap();
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

#[test]
fn a_take_over_shows_both_implementers_and_a_review_taken_over_both_reviewers() {
    let scratch = Scratch::new(&[(ID, "open")]);
    let root = scratch.root();
    ok(atelier(&root, "t1", &["start", ID]));
    ok(atelier(&root, "t3", &["start", ID]));
    hello(&root);
    finish_and_submit(&root, "t3");
    ok(atelier(&root, "t2", &["review", ID]));
    ok(codex(&root, "c1", &["review", ID]));
    let run = codex(&root, "c1", &["verdict", "approve"]);
    ok(run.clone());
    run_then(&root, &run.stdout);
    let (review, _) = review_of(&root, 1);
    assert_eq!(
        actions(&root, &log(&root, &[])),
        [
            format!("{ID} start t1 open -> building"),
            format!("{ID} take-over t3 from t1"),
            format!("{ID} submit t3 building -> in_review"),
            format!("{ID} review t2 {review}"),
            format!("{ID} take-over p1/c1 {review}; from t2"),
            format!("{ID} verdict p1/c1 {review} approve; in_review -> approved"),
        ]
    );
}

/// An approving review of `DONE` whose first finding is deferred.
const FINDINGS: &str = "## Summary\nFine.\n\n\
    ## Finding 1: The greeting ignores the locale\nRecommendation: defer\n\
    Greeting is English only.\n";

#[test]
fn a_deferred_finding_settled_on_a_gardeners_branch_shows_once_merged() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    write_item(&root, DONE, "done", &["t1"], &[], "Done.");
    let path = root.join(DONE_REVIEW);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!(
            "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: abc1234\n\
             reviewers:\n- t2\nmodel: null\nverdict: approve\n\
             created_at: 2026-11-10T09:00:00Z\n---\n{FINDINGS}"
        ),
    )
    .unwrap();
    on_base(&root, "Merge demo-d0ne");

    // The gardener settles it on the branch of the item it implements.
    ok(atelier(&root, "g1", &["start", ID]));
    let run = atelier(
        &root,
        "g1",
        &[
            "drop",
            "demo-d0ne/1-aaaa.1",
            "--reason",
            "Nobody reads it abroad.",
        ],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    shell(&root, then(&run.stdout));
    let settling = short(&root, "HEAD");
    hello(&root);
    finish_and_submit(&root, "g1");
    ok(atelier(&root, "r1", &["review", ID]));
    ok(verdict(&root, "r1", &["verdict", "approve"]));
    ok(atelier(&root, "g1", &["merge"]));
    assert!(!branch_exists(&root, ID));

    let lines = log(&root, &[DONE]);
    assert_eq!(
        actions(&root, &lines),
        [
            // Added done, as the cutover's conversion commit adds records.
            format!("{DONE} start t1"),
            format!("{DONE} review t2 1-aaaa"),
            format!("{DONE} verdict t2 1-aaaa approve"),
            format!("{DONE} close - new -> done"),
            format!("{DONE} settle g1 1-aaaa.1 dropped: Nobody reads it abroad."),
        ]
    );
    assert_eq!(commit_of(&lines[4]), settling);
}

fn write_mission(root: &Path) {
    let path = root.join(format!(".atelier/missions/{MISSION}/mission.md"));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!(
            "---\nschema: atelier.mission\nschema_version: 3\nid: {MISSION}\ntitle: Greet\n\
             status: draft\nlabels: []\nlead: null\nready: null\n\
             created_at: 2026-11-10T09:00:00Z\n---\n\n## Outcome\n\nGreetings.\n"
        ),
    )
    .unwrap();
    let item = root.join(item_path(ID));
    let text = fs::read_to_string(&item).unwrap();
    fs::write(
        &item,
        text.replace("mission: null", &format!("mission: {MISSION}")),
    )
    .unwrap();
}

#[test]
fn a_mission_shows_its_ready_lead_plan_review_and_close_with_its_items_actions() {
    let scratch = Scratch::new(&[(ID, "draft"), ("demo-0ther", "draft")]);
    let root = scratch.root();
    write_mission(&root);
    git(&root, &["commit", "-qam", "Plan the mission"]);
    on_base(&root, "Add the mission");
    readied(&root, ID);
    readied(&root, "demo-0ther");
    on_base(&root, "Ready both");
    // Made ready as a mission was before it had no go-ahead, by the old
    // tool, which wrote who said go.
    let path = format!(".atelier/missions/{MISSION}/mission.md");
    let text = fs::read_to_string(root.join(&path)).unwrap().replace(
        "status: draft\nlabels: []\nlead: null\nready: null\n",
        "status: ready\nlabels: []\nlead: null\nready:\n  by: ada\n  at: 2026-11-10T10:00:00Z\n",
    );
    fs::write(root.join(&path), text).unwrap();
    on_base(&root, "Ready the mission");
    ok(planned(&root, "l1", &["lead", MISSION]));
    ok(atelier(&root, "r1", &["review", MISSION]));
    ok(verdict(&root, "r1", &["verdict", "approve"]));
    ok(planned(
        &root,
        "p1",
        &["drop", ID, "--reason", "Nobody needs it."],
    ));
    append(
        &root,
        &format!(".atelier/missions/{MISSION}/mission.md"),
        "\n## Result\n\nNothing was needed.\n",
    );
    on_base(&root, "Result");
    ok(planned(&root, "p1", &["close", MISSION]));
    let stem = fs::read_dir(root.join(format!(".atelier/missions/{MISSION}/reviews")))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .file_name()
        .into_string()
        .unwrap();
    let review = stem.strip_suffix(".md").unwrap();

    let ours = [
        format!("{ID} ready - draft -> ready; by ada"),
        format!("{MISSION} ready - draft -> open; by ada"),
        format!("{MISSION} lead l1"),
        format!("{MISSION} review r1 {review}"),
        format!("{MISSION} verdict r1 {review} approve"),
        format!("{ID} drop - ready -> dropped; Nobody needs it."),
        format!("{MISSION} close - open -> done"),
    ];
    let whole = log(&root, &[MISSION]);
    assert_eq!(actions(&root, &whole), ours);
    // With no option, the mission's lines newest first.
    let run = atelier(&root, "t9", &["log", MISSION]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let newest: Vec<&str> = whole.iter().rev().map(String::as_str).collect();
    assert_eq!(run.stdout.lines().collect::<Vec<_>>(), newest);
    // Everything holds the other item too; the item alone holds its own.
    let all = actions(&root, &log(&root, &[]));
    assert_eq!(all.len(), ours.len() + 1);
    assert_eq!(all[0], "demo-0ther ready - draft -> ready; by ada");
    assert_eq!(
        actions(&root, &log(&root, &[ID])),
        [ours[0].clone(), ours[5].clone()]
    );
}

#[test]
fn a_skip_ends_the_line_of_the_action_whose_commit_carries_it_with_its_thread() {
    let scratch = Scratch::new(&[("demo-b10c", "open")]);
    let root = scratch.root();
    write_item(&root, ID, "open", &[], &["demo-b10c"], "");
    on_base(&root, "Plan a blocked item");
    let output = atelier_command(&root, "t1", &["start", ID])
        .env("ATELIER_SKIP", REASON)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        actions(&root, &log(&root, &[])),
        [format!(
            "{ID} start t1 open -> building; skip demo-b10c is done (it is open): {REASON} (by t1)"
        )]
    );
}

/// The page `show` prints of mission `id`.
fn show(root: &Path, id: &str) -> String {
    let run = atelier(root, "t9", &["show", id]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    run.stdout
}

#[test]
fn a_skip_from_before_its_thread_was_recorded_says_so_in_the_log_and_the_missions_page() {
    let scratch = Scratch::new(&[("demo-b10c", "open")]);
    let root = scratch.root();
    write_item(&root, ID, "open", &[], &["demo-b10c"], "");
    write_mission(&root);
    // Agreed, so that only the item's blocker is a rule its start passes.
    let path = format!(".atelier/missions/{MISSION}/mission.md");
    let agreed = read(&root, &path).replace(
        "status: draft\nlabels: []\nlead: null\nready: null\n",
        "status: ready\nlabels: []\nlead: null\nready:\n  by: ada\n  at: 2026-11-10T09:00:00Z\n",
    );
    fs::write(root.join(&path), agreed).unwrap();
    on_base(&root, "Add the mission");
    // A mission none of whose commits passed a rule says nothing of it.
    assert!(!show(&root, MISSION).contains("Rules passed"));

    let output = atelier_command(&root, "t1", &["start", ID])
        .env("ATELIER_SKIP", REASON)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let rule = format!("demo-b10c is done (it is open): {REASON}");
    let message = git(&root, &["log", "-1", "--format=%B"]);
    assert!(
        message.ends_with(&format!("\n\nAtelier-Skip: {rule}\nAtelier-Skip-By: t1")),
        "{message}"
    );
    let commit = short(&root, "HEAD");
    assert!(
        show(&root, MISSION).contains(&format!(
            "\nRules passed:\n  {ID}, start in {commit}, by t1: {rule}\n\nPlan reviews:\n"
        )),
        "{}",
        show(&root, MISSION)
    );

    // As a binary from before the thread was recorded wrote it.
    let old = message.replace("\nAtelier-Skip-By: t1", "");
    git(&root, &["commit", "-q", "--amend", "-m", &old]);
    let commit = short(&root, "HEAD");
    assert_eq!(
        actions(&root, &log(&root, &[ID])),
        [format!(
            "{ID} start t1 open -> building; skip {rule} (thread not recorded)"
        )]
    );
    assert!(
        show(&root, MISSION).contains(&format!(
            "\nRules passed:\n  {ID}, start in {commit}, thread not recorded: {rule}\n"
        )),
        "{}",
        show(&root, MISSION)
    );
}

#[test]
fn since_leaves_out_the_actions_committed_before_it() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let at = |time: &str, thread: &str, args: &[&str]| {
        let output = atelier_command(&root, thread, args)
            .env("GIT_COMMITTER_DATE", time)
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
    };
    readied(&root, ID);
    // The agent commits it, and its backlog branch's merge moves the base.
    let committed = identity(
        Command::new("git")
            .current_dir(&root)
            .args(["commit", "-qam", "Ready"]),
    )
    .env("GIT_COMMITTER_DATE", "2026-01-01T00:00:00Z")
    .output()
    .unwrap();
    assert!(committed.status.success(), "{committed:?}");
    git(&root, &["branch", "-f", "main", "HEAD"]);
    at("2026-02-01T12:00:00Z", "t1", &["start", ID]);
    let ready = format!(
        "2026-01-01T00:00:00Z {} {ID} ready - draft -> ready; by ada",
        short(&root, "main")
    );
    let start = format!(
        "2026-02-01T12:00:00Z {} {ID} start t1 ready -> building",
        short(&root, ID)
    );
    assert_eq!(log(&root, &[]), [ready.clone(), start.clone()]);
    assert_eq!(
        log(&root, &["--since", "2026-01-01"]),
        [ready, start.clone()]
    );
    assert_eq!(
        log(&root, &["--since", "2026-02-01"]),
        std::slice::from_ref(&start)
    );
    assert_eq!(
        log(&root, &[ID, "--since", "2026-02-01T13:00:00+01:00"]),
        [start]
    );
    assert!(log(&root, &["--since", "2026-02-01T12:00:01Z"]).is_empty());

    let bad = atelier(&root, "t9", &["log", "--since", "yesterday"]);
    assert_eq!(bad.code, 2);
    assert!(
        bad.stderr.contains("`yesterday` is not an RFC 3339 time"),
        "{}",
        bad.stderr
    );
}

#[test]
fn two_actions_on_one_record_in_one_commit_show_as_their_net_change() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    // In old history, made ready and closed at once: only closed.
    let path = root.join(item_path(ID));
    write_item(&root, ID, "done", &[], &[], "Not needed.\n");
    let text = fs::read_to_string(&path).unwrap().replace(
        "ready: null",
        "ready:\n  by: ada\n  at: 2026-11-10T09:00:00Z",
    );
    fs::write(&path, text).unwrap();
    on_base(&root, "Ready and close");
    assert_eq!(
        actions(&root, &log(&root, &[])),
        [format!("{ID} close - draft -> done")]
    );
}

#[test]
fn a_version_that_does_not_read_is_passed_over_and_said_on_stderr() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let path = root.join(item_path(ID));
    let draft = fs::read_to_string(&path).unwrap();
    fs::write(&path, draft.replace("status: draft", "status: bogus")).unwrap();
    on_base(&root, "Break it");
    let broken = short(&root, "HEAD");
    fs::write(&path, draft.replace("status: draft", "status: ready")).unwrap();
    on_base(&root, "Mend it");

    let run = atelier(&root, "t9", &["log"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stderr.starts_with(&format!(
            "atelier log: passed over {} in commit {broken}, which does not read: ",
            item_path(ID)
        )),
        "{}",
        run.stderr
    );
    assert_eq!(run.stderr.lines().count(), 1, "{}", run.stderr);
    // The actions across it compare the versions on each side.
    let lines: Vec<String> = run.stdout.lines().map(str::to_string).collect();
    assert_eq!(
        actions(&root, &lines),
        [format!("{ID} ready - draft -> ready")]
    );
    assert_eq!(commit_of(&lines[0]), short(&root, "HEAD"));
}

#[test]
fn an_id_no_record_has_is_refused_as_show_refuses_it() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let run = atelier(&root, "t9", &["log", "demo-zz99"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        atelier(&root, "t9", &["show", "demo-zz99"]).stderr
    );
    let run = atelier(&root, "t9", &["log", "Not an ID"]);
    assert_eq!(run.code, 2);
    assert_eq!(
        run.stderr,
        atelier(&root, "t9", &["show", "Not an ID"]).stderr
    );
}

/// Git's history simplification drops a merge's side when the merge
/// matches its other parent on the record paths. An Atelier merge never
/// does, since it marks its item done; a plain merge of a branch whose
/// record changes were undone does, and without `--full-history` the
/// branch's actions would be lost with the branch.
#[test]
fn actions_on_a_branch_whose_records_a_plain_merge_matches_survive_its_deletion() {
    // An old record, so that the revert has a line of its own.
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    ok(atelier(&root, "t1", &["start", ID]));
    let started = short(&root, "HEAD");
    hello(&root);
    git(&root, &["revert", "--no-edit", &started]);
    off_base(&root);
    git(
        &root,
        &["merge", "-q", "--no-ff", "-m", "Merge the attempt", ID],
    );
    git(&root, &["branch", "-f", "main", "HEAD"]);
    git(&root, &["branch", "-qD", ID]);
    let lines = log(&root, &[]);
    assert_eq!(
        actions(&root, &lines),
        [
            format!("{ID} ready - new -> ready"),
            format!("{ID} start t1 ready -> building"),
            format!("{ID} ready - building -> ready"),
        ]
    );
    assert_eq!(commit_of(&lines[1]), started);
}

#[test]
fn a_version_that_is_not_utf8_is_passed_over_like_one_that_does_not_parse() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let path = root.join(item_path(ID));
    let draft = fs::read_to_string(&path).unwrap();
    let mut bytes = draft.clone().into_bytes();
    bytes.push(0xff);
    fs::write(&path, bytes).unwrap();
    on_base(&root, "Break its encoding");
    let broken = short(&root, "HEAD");
    fs::write(&path, draft.replace("status: draft", "status: ready")).unwrap();
    on_base(&root, "Mend it");

    for args in [&["log"][..], &["log", ID]] {
        let run = atelier(&root, "t9", args);
        assert_eq!(run.code, 0, "{}", run.stderr);
        assert!(
            run.stderr.starts_with(&format!(
                "atelier log: passed over {} in commit {broken}, which does not read: ",
                item_path(ID)
            )) && run.stderr.contains("not UTF-8"),
            "{}",
            run.stderr
        );
        let lines: Vec<String> = run.stdout.lines().map(str::to_string).collect();
        assert_eq!(
            actions(&root, &lines),
            [format!("{ID} ready - draft -> ready")]
        );
    }
}

#[test]
fn a_version_that_does_not_read_twice_stands_each_time_for_the_version_before_it() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let path = root.join(item_path(ID));
    let draft = fs::read_to_string(&path).unwrap();
    // The two broken versions are one blob, after different versions.
    for status in ["bogus", "ready", "bogus", "ready"] {
        fs::write(
            &path,
            draft.replace("status: draft", &format!("status: {status}")),
        )
        .unwrap();
        on_base(&root, status);
    }
    let run = atelier(&root, "t9", &["log"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(run.stderr.lines().count(), 1, "{}", run.stderr);
    let lines: Vec<String> = run.stdout.lines().map(str::to_string).collect();
    assert_eq!(
        actions(&root, &lines),
        [format!("{ID} ready - draft -> ready")]
    );
    assert_eq!(commit_of(&lines[0]), short(&root, "HEAD~2"));

    // On two branches, each broken version stands for its own branch's.
    git(&root, &["switch", "-q", "-c", "demo-b2b2", "HEAD~4"]);
    fs::write(&path, draft.replace("status: draft", "status: done")).unwrap();
    git(&root, &["commit", "-qam", "done"]);
    fs::write(&path, draft.replace("status: draft", "status: bogus")).unwrap();
    git(&root, &["commit", "-qam", "bogus"]);
    fs::write(&path, draft.replace("status: draft", "status: dropped")).unwrap();
    git(&root, &["commit", "-qam", "dropped"]);
    let run = atelier(&root, "t9", &["log"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let lines: Vec<String> = run.stdout.lines().map(str::to_string).collect();
    let actions = actions(&root, &lines);
    assert!(
        actions.contains(&format!("{ID} close - draft -> done")),
        "{actions:?}"
    );
    assert!(
        actions.contains(&format!("{ID} drop - done -> dropped")),
        "{actions:?}"
    );
    assert_eq!(actions.len(), 3, "{actions:?}");
}

/// `atelier` as `thread` with `ATELIER_SKIP` set to `reason`, which must
/// succeed.
fn skipping(root: &Path, thread: &str, reason: &str, args: &[&str]) -> Run {
    let output = atelier_command(root, thread, args)
        .env("ATELIER_SKIP", reason)
        .output()
        .unwrap();
    let run = Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    };
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);
    run
}

/// The page's `Rules passed:` lines, none when it has no such section.
fn rules_passed(page: &str) -> Vec<String> {
    let Some((_, after)) = page.split_once("\nRules passed:\n") else {
        return Vec::new();
    };
    let section = after.split("\n\n").next().unwrap();
    section.lines().map(str::to_string).collect()
}

#[test]
fn a_proposed_plans_lead_and_merge_and_a_set_of_a_started_item_each_show_their_passed_rule() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    // The plan, proposed on its own branch, with the item in it.
    git(&root, &["switch", "-q", "-c", MISSION]);
    write_mission(&root);
    let path = format!(".atelier/missions/{MISSION}/mission.md");
    fs::write(
        root.join(&path),
        read(&root, &path).replace("status: draft", "status: open"),
    )
    .unwrap();
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Plan the mission"]);

    // Led while still proposed: found on its branch.
    let lead = "Lead it now";
    let run = skipping(&root, "lead1", lead, &["lead", MISSION]);
    run_then(&root, &run.stdout);
    let led = short(&root, "HEAD");
    let page = show(&root, MISSION);
    let passed = rules_passed(&page);
    assert_eq!(passed.len(), 1, "{page}");
    assert!(
        passed[0].starts_with(&format!("  {MISSION}, lead in {led}, by lead1: "))
            && passed[0].ends_with(&format!(": {lead}")),
        "{page}"
    );
    let lines = actions(&root, &log(&root, &[MISSION]));
    assert!(
        lines
            .iter()
            .any(|line| line.starts_with(&format!("{MISSION} lead lead1"))
                && line.ends_with(&format!(": {lead} (by lead1)"))),
        "{lines:?}"
    );

    // Merged without its two plan reviews: the merge changes no status,
    // and its line still shows the rule.
    let unreviewed = "Ada agreed the plan in conversation";
    skipping(&root, "p1", unreviewed, &["merge"]);
    let merged = short(&root, "main");
    let page = show(&root, MISSION);
    let passed = rules_passed(&page);
    assert!(
        passed.iter().any(|line| line
            .starts_with(&format!("  {MISSION}, plan-merge in {merged}, by p1: "))
            && line.ends_with(&format!(": {unreviewed}"))),
        "{page}"
    );
    let lines = actions(&root, &log(&root, &[MISSION]));
    assert!(
        lines.iter().any(
            |line| line.starts_with(&format!("{MISSION} plan-merge p1 skip "))
                && line.ends_with(&format!(": {unreviewed} (by p1)"))
        ),
        "{lines:?}"
    );

    // A started item's priority set on a backlog branch: no action, and
    // its rule still shows.
    off_base(&root);
    ok(atelier(&root, "t1", &["start", ID]));
    off_base(&root);
    ok(atelier(&root, "p1", &["start", "--backlog", "tidy"]));
    let lower = "Ada lowered it";
    let run = skipping(&root, "p1", lower, &["set", ID, "--priority", "low"]);
    run_then(&root, &run.stdout);
    let set = short(&root, "HEAD");
    // Its backlog branch merged past a rule of its own, which the merge
    // commit carries: it names the branch, so the records it changed are
    // the merge's.
    ok(atelier(&root, "p1", &["submit"]));
    append(&root, &item_path(ID), "\nNot committed.\n");
    let dirty = "Ada keeps the note";
    skipping(&root, "p1", dirty, &["merge"]);
    let backlog = short(&root, "main");
    let rule = format!("{ID} is open (it is building)");
    let page = show(&root, MISSION);
    assert!(
        rules_passed(&page).contains(&format!("  {ID}, change in {set}, by p1: {rule}: {lower}")),
        "{page}"
    );
    assert!(
        rules_passed(&page).iter().any(|line| line
            .starts_with(&format!("  {ID}, merge in {backlog}, by p1: "))
            && line.ends_with(&format!(": {dirty}"))),
        "{page}"
    );
    let lines = actions(&root, &log(&root, &[ID]));
    let n = lines.len();
    assert_eq!(
        lines[n - 2],
        format!("{ID} change p1 skip {rule}: {lower} (by p1)"),
        "{lines:?}"
    );
    assert!(
        lines[n - 1].starts_with(&format!("{ID} merge p1 skip "))
            && lines[n - 1].ends_with(&format!(": {dirty} (by p1)")),
        "{lines:?}"
    );
}

#[test]
fn a_record_with_no_action_beside_one_with_an_action_still_shows_the_rule_its_commit_passed() {
    const OTHER: &str = "demo-cd34";
    let scratch = Scratch::new(&[(ID, "ready"), (OTHER, "draft")]);
    let root = scratch.root();
    write_mission(&root);
    on_base(&root, "Add the mission");
    // History from before items were open: one go-ahead in one commit for
    // an item ready already, in the mission, and a draft, in none.
    let again = "Ada reconfirmed the ready item";
    readied(&root, ID);
    readied(&root, OTHER);
    let skipped = format!("Atelier-Skip: {ID} is draft (it is ready): {again}");
    git(
        &root,
        &[
            "commit",
            "-qam",
            "Ready both",
            "--trailer",
            &skipped,
            "--trailer",
            "Atelier-Skip-By: p1",
        ],
    );
    git(&root, &["branch", "-f", "main", "HEAD"]);
    let commit = short(&root, "HEAD");
    let rule = format!("{ID} is draft (it is ready): {again}");
    assert_eq!(
        actions(&root, &log(&root, &[ID])).last().unwrap(),
        &format!("{ID} change p1 skip {rule} (by p1)")
    );
    let page = show(&root, MISSION);
    assert!(
        rules_passed(&page).contains(&format!("  {ID}, change in {commit}, by p1: {rule}")),
        "{page}"
    );
    // The other item's own line carries it too.
    let other = actions(&root, &log(&root, &[OTHER]));
    assert_eq!(
        other.last().unwrap(),
        &format!("{OTHER} ready p1 draft -> ready; by ada; skip {rule} (by p1)")
    );
}

#[test]
fn a_settlement_keeps_its_passed_rule_beside_the_item_its_commit_creates() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    write_mission(&root);
    write_item(&root, DONE, "done", &["t1"], &[], "Done.");
    let item =
        read(&root, &item_path(DONE)).replace("mission: null", &format!("mission: {MISSION}"));
    fs::write(root.join(item_path(DONE)), item).unwrap();
    let path = root.join(DONE_REVIEW);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!(
            "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: abc1234\n\
             reviewers:\n- t2\nmodel: null\nverdict: approve\n\
             created_at: 2026-11-10T09:00:00Z\n---\n{FINDINGS}"
        ),
    )
    .unwrap();
    on_base(&root, "Merge demo-d0ne");

    // Taken into a new item, outside the mission, in the base's checkout.
    git(&root, &["switch", "-q", "main"]);
    let direct = "Ada authorized recording this directly";
    let finding = "demo-d0ne/1-aaaa.1";
    let run = skipping(
        &root,
        "p1",
        direct,
        &["new", "Translate greeting", "--from", finding],
    );
    run_then(&root, &run.stdout);
    let commit = short(&root, "HEAD");
    let message = git(&root, &["log", "-1", "--format=%B"]);
    let rule = message
        .lines()
        .find_map(|line| line.strip_prefix("Atelier-Skip: "))
        .unwrap_or_else(|| panic!("{message}"))
        .to_string();
    assert!(rule.ends_with(&format!(": {direct}")), "{message}");

    let settle = actions(&root, &log(&root, &[DONE]));
    let settle = settle.last().unwrap();
    assert!(
        settle.starts_with(&format!("{DONE} settle p1 1-aaaa.1 taken into "))
            && settle.ends_with(&format!("; skip {rule} (by p1)")),
        "{settle}"
    );
    let page = show(&root, MISSION);
    assert_eq!(
        rules_passed(&page),
        [format!("  {DONE}, settle in {commit}, by p1: {rule}")],
        "{page}"
    );
}

#[test]
fn a_proposed_mission_fetched_alone_or_ahead_of_its_local_branch_shows_its_passed_rules() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    append(
        &root,
        ".atelier/config.toml",
        "\n[forge]\nprovider = \"github\"\nremote = \"origin\"\n",
    );
    on_base(&root, "Configure the forge");
    git(&root, &["switch", "-q", "-c", MISSION]);
    write_mission(&root);
    let path = format!(".atelier/missions/{MISSION}/mission.md");
    fs::write(
        root.join(&path),
        read(&root, &path).replace("status: draft", "status: open"),
    )
    .unwrap();
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Plan the mission"]);
    let lead = "Lead the proposed mission now";
    let run = skipping(&root, "p1", lead, &["lead", MISSION]);
    run_then(&root, &run.stdout);
    let led = short(&root, "HEAD");
    let shows = |root: &Path| {
        let page = show(root, MISSION);
        let passed = rules_passed(&page);
        assert_eq!(passed.len(), 1, "{page}");
        assert!(
            passed[0].starts_with(&format!("  {MISSION}, lead in {led}, by p1: "))
                && passed[0].ends_with(&format!(": {lead}")),
            "{page}"
        );
        let lines = actions(root, &log(root, &[MISSION]));
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with(&format!("{MISSION} lead p1"))
                    && line.ends_with(&format!(": {lead} (by p1)"))),
            "{lines:?}"
        );
    };

    // Only fetched: the remote-tracking branch alone holds it.
    git(
        &root,
        &[
            "update-ref",
            &format!("refs/remotes/origin/{MISSION}"),
            "HEAD",
        ],
    );
    off_base(&root);
    git(&root, &["branch", "-qD", MISSION]);
    shows(&root);

    // The local branch behind the fetched one, before the lead's commit.
    git(
        &root,
        &["branch", "-q", MISSION, &format!("origin/{MISSION}~1")],
    );
    shows(&root);
}

#[test]
fn a_rule_passed_on_a_backlog_branch_not_merged_yet_shows_on_the_missions_page() {
    let scratch = Scratch::new(&[(ID, "done")]);
    let root = scratch.root();
    write_mission(&root);
    on_base(&root, "Add the mission");
    ok(atelier(&root, "p1", &["start", "--backlog", "tidy"]));
    let again = "Ada reconfirmed it";
    let run = skipping(&root, "p1", again, &["set", ID, "--priority", "low"]);
    run_then(&root, &run.stdout);
    let commit = short(&root, "HEAD");
    off_base(&root);
    let rule = format!("{ID} is open (it is done): {again}");
    assert_eq!(
        rules_passed(&show(&root, MISSION)),
        [format!("  {ID}, change in {commit}, by p1: {rule}")]
    );
    assert_eq!(
        actions(&root, &log(&root, &[MISSION])).last().unwrap(),
        &format!("{ID} change p1 skip {rule} (by p1)")
    );
}

#[test]
fn an_item_only_on_a_backlog_branch_not_merged_yet_shows_its_passed_rule_on_its_missions_page() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    write_mission(&root);
    let path = format!(".atelier/missions/{MISSION}/mission.md");
    fs::write(
        root.join(&path),
        read(&root, &path).replace("status: draft", "status: open"),
    )
    .unwrap();
    on_base(&root, "Add the mission");
    ok(atelier(&root, "p1", &["start", "--backlog", "tidy"]));
    let run = atelier(&root, "p1", &["new", "New greeting", "--in", MISSION]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    run_then(&root, &run.stdout);
    let added = git(&root, &["diff", "--name-only", "HEAD~1", "HEAD"]);
    let new = added.split('/').nth(2).unwrap().to_string();
    let run = atelier(&root, "p1", &["close", &new, "Nothing to change."]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    run_then(&root, &run.stdout);
    let again = "Ada reconfirmed it";
    let run = skipping(&root, "p1", again, &["set", &new, "--priority", "low"]);
    run_then(&root, &run.stdout);
    let commit = short(&root, "HEAD");
    off_base(&root);

    let rule = format!("{new} is open (it is done): {again}");
    let page = show(&root, MISSION);
    assert!(page.contains(&format!("  {new} ")), "{page}");
    assert_eq!(
        rules_passed(&page),
        [format!("  {new}, change in {commit}, by p1: {rule}")],
        "{page}"
    );
    assert_eq!(
        actions(&root, &log(&root, &[MISSION])).last().unwrap(),
        &format!("{new} change p1 skip {rule} (by p1)")
    );
}

/// The `plan` and `plan-merge` lines of `lines`, and any `merge`.
fn plan_lines(lines: Vec<String>) -> Vec<String> {
    (lines.into_iter())
        .filter(|line| {
            let action = line.split(' ').nth(1).unwrap();
            ["plan", "plan-merge", "merge"].contains(&action)
        })
        .collect()
}

#[test]
fn a_plan_shows_when_it_was_written_and_merged_and_each_amendment_and_never_as_a_merge() {
    // The base has no record before the plan's merge.
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    let run = atelier(&root, "p1", &["start", "--mission", "Greet"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let id = run.stdout.split_whitespace().nth(3).unwrap().to_string();
    let path = format!(".atelier/missions/{id}/mission.md");
    let edit = |from: &str, to: &str, message: &str| {
        let text = read(&root, &path);
        assert!(text.contains(from), "{text}");
        fs::write(root.join(&path), text.replace(from, to)).unwrap();
        git(&root, &["commit", "-qam", message]);
    };
    let planned = format!("{id} plan p1 new -> open");
    let lines = || plan_lines(actions(&root, &log(&root, &[&id])));
    // Proposed, it is planned and not merged.
    edit(
        "## Outcome\n",
        "## Outcome\n\nGreetings.\n",
        "Write the plan",
    );
    assert_eq!(lines(), [planned.as_str()]);

    // As two reviewers, each in a checkout of its own, land one round.
    let head = short(&root, "HEAD");
    for (stem, reviewer) in [("1-aaaa", "r1"), ("1-bbbb", "r2")] {
        let review = root.join(format!(".atelier/missions/{id}/reviews/{stem}.md"));
        fs::create_dir_all(review.parent().unwrap()).unwrap();
        fs::write(
            review,
            format!(
                "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: {head}\n\
                 reviewers:\n- {reviewer}\nmodel: null\nverdict: approve\n\
                 created_at: 2026-11-10T09:00:00Z\n---\n## Summary\nFine.\n"
            ),
        )
        .unwrap();
    }
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Review the plan"]);
    ok(atelier(&root, "p1", &["merge"]));
    let agreed = format!("{id} plan-merge p1");
    let merged = log(&root, &[&id]);
    assert_eq!(lines(), [planned.as_str(), agreed.as_str()]);
    // The line is the merge commit's, and its action is not `merge`, by
    // which the gardener counts merged items.
    let line = (merged.iter())
        .find(|line| line.split(' ').nth(3) == Some("plan-merge"))
        .unwrap();
    assert_eq!(commit_of(line), short(&root, "main"));
    assert!(merged
        .iter()
        .all(|line| line.split(' ').nth(3) != Some("merge")));

    // Its lead, its notes and its Result change no plan.
    ok(atelier(&root, "l1", &["start", "--backlog", "lead"]));
    edit("lead: null", "lead: l1", "Lead it");
    fs::write(
        root.join(format!(".atelier/missions/{id}/notes.md")),
        "A note.\n",
    )
    .unwrap();
    append(&root, &path, "\n## Result\n\nNot yet.\n");
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Note it"]);
    ok(atelier(&root, "l1", &["merge"]));
    assert_eq!(lines(), [planned.as_str(), agreed.as_str()]);

    // A branch that brings the base in merges no plan.
    git(&root, &["switch", "-q", "-c", "backlog/behind", "main~1"]);
    git(&root, &["merge", "-q", "--no-ff", "-m", "Catch up", "main"]);
    assert_eq!(lines(), [planned.as_str(), agreed.as_str()]);

    // A section changed, and then a document alone, each merged with the
    // rule it passed on its line.
    let amended = |slug: &str, change: &dyn Fn()| {
        off_base(&root);
        ok(atelier(&root, "l1", &["start", "--backlog", slug]));
        change();
        skipping(&root, "l1", "Ada agreed it", &["merge"]);
        lines().pop().unwrap()
    };
    let amendments = [
        amended("outcome", &|| {
            edit("Greetings.", "Warm greetings.", "Amend the Outcome")
        }),
        amended("design", &|| {
            let design = format!(".atelier/missions/{id}/design.md");
            fs::write(root.join(&design), "Say hello.\n").unwrap();
            git(&root, &["add", &design]);
            git(&root, &["commit", "-qm", "Amend the design"]);
        }),
    ];
    for line in &amendments {
        assert!(
            line.starts_with(&format!("{id} plan-merge l1 amendment; skip "))
                && line.ends_with(": Ada agreed it (by l1)"),
            "{line}"
        );
    }
    assert_eq!(
        lines(),
        [
            planned.clone(),
            agreed.clone(),
            amendments[0].clone(),
            amendments[1].clone()
        ]
    );
}

#[test]
fn only_a_new_open_plan_shows_as_planned() {
    let hours = |ago: i64| (chrono::Utc::now() - chrono::Duration::hours(ago)).to_rfc3339();
    let mission = |status: &str, lead: &str, created: &str, body: &str| {
        format!(
            "---\nschema: atelier.mission\nschema_version: 3\nid: {MISSION}\ntitle: Greet\n\
             status: {status}\nlabels: []\nlead: {lead}\ncreated_at: {created}\n---\n\n\
             ## Outcome\n\nGreetings.\n{body}"
        )
    };
    let planned = format!("{MISSION} plan - new -> open");
    for (case, text, expected) in [
        (
            "new, with no planner",
            mission("open", "null", &hours(23), ""),
            true,
        ),
        // As the cutover's conversion wrote atelier-g6rp and atelier-qr5s.
        ("converted", mission("ready", "null", &hours(25), ""), false),
        ("led already", mission("open", "l1", &hours(0), ""), false),
        (
            "with a Result",
            mission("open", "null", &hours(0), "\n## Result\n\nGreeted.\n"),
            false,
        ),
        (
            "an old draft",
            mission("draft", "null", &hours(0), ""),
            false,
        ),
    ] {
        let scratch = Scratch::new(&[]);
        let root = scratch.root();
        let path = root.join(format!(".atelier/missions/{MISSION}/mission.md"));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &text).unwrap();
        on_base(&root, "Add the mission");
        // Removed and added again, it has not first appeared again.
        fs::remove_file(&path).unwrap();
        on_base(&root, "Remove it");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &text).unwrap();
        on_base(&root, "Restore it");
        let found = plan_lines(actions(&root, &log(&root, &[])));
        let expected = if expected {
            vec![planned.clone()]
        } else {
            vec![]
        };
        assert_eq!(found, expected, "{case}");
    }
}
