//! `log`, the history of acts, driven through the new binary in scratch
//! repositories: each act found once in Git, with its thread, in order,
//! whether its item's branch is still there or merged and deleted.

mod common;

use common::*;
use std::fs;
use std::path::Path;
use std::process::Command;

const MISSION: &str = "demo-m1ss";
const DONE: &str = "demo-d0ne";
const DONE_REVIEW: &str = ".atelier/items/demo-d0ne/reviews/1-aaaa.md";
const REASON: &str = "Sumeet said to";

/// `atelier log` with `args`, which must succeed; its lines.
fn log(root: &Path, args: &[&str]) -> Vec<String> {
    let mut all = vec!["log"];
    all.extend(args);
    let run = atelier(root, "t9", &all);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(run.stderr, "");
    run.stdout.lines().map(str::to_string).collect()
}

/// `lines` without their time and commit, after checking both: a time in
/// UTC to the second, and a commit Git knows by that name.
fn acts(root: &Path, lines: &[String]) -> Vec<String> {
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
fn the_plain_path_prints_its_acts_in_order_and_the_same_lines_once_its_branch_is_merged_and_deleted(
) {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    ok(atelier(&root, "p1", &["ready", ID, "--by", "sumeet"]));
    ok(atelier(&root, "t1", &["start", ID]));
    hello(&root);
    finish_and_submit(&root, "t1");
    ok(atelier(&root, "t2", &["review", ID]));
    ok(atelier(&root, "t2", &["verdict", "approve"]));
    let (review, _) = review_of(&root, 1);

    // Before the merge, the branch's acts come from its tip.
    let started = [
        format!("{ID} ready - draft -> ready; by sumeet"),
        format!("{ID} start t1 ready -> building"),
        format!("{ID} submit t1 building -> in_review"),
        format!("{ID} review t2 {review}"),
        format!("{ID} verdict t2 {review} approve; in_review -> approved"),
    ];
    assert_eq!(acts(&root, &log(&root, &[])), started);
    assert_eq!(acts(&root, &log(&root, &[ID])), started);

    ok(atelier(&root, "t1", &["merge"]));
    let merged = log(&root, &[]);
    let mut all = started.to_vec();
    all.push(format!("{ID} merge t1 approved -> done"));
    assert_eq!(acts(&root, &merged), all);
    // The merge line is the merge commit's; the others are the branch's.
    assert_eq!(commit_of(&merged[5]), short(&root, "main"));
    assert_eq!(commit_of(&merged[4]), short(&root, ID));

    git(&root, &["switch", "-q", "main"]);
    git(&root, &["branch", "-qD", ID]);
    assert!(!branch_exists(&root, ID));
    assert_eq!(log(&root, &[]), merged);
    assert_eq!(log(&root, &[ID]), merged);
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
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    ok(atelier(&root, "t1", &["start", ID]));
    ok(atelier(&root, "t3", &["start", ID]));
    hello(&root);
    finish_and_submit(&root, "t3");
    ok(atelier(&root, "t2", &["review", ID]));
    ok(codex(&root, "c1", &["review", ID]));
    ok(codex(&root, "c1", &["verdict", "approve"]));
    let (review, _) = review_of(&root, 1);
    assert_eq!(
        acts(&root, &log(&root, &[])),
        [
            // The fixture's first commit adds the item ready.
            format!("{ID} ready - new -> ready"),
            format!("{ID} start t1 ready -> building"),
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
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Merge demo-d0ne"]);

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
    ok(atelier(&root, "r1", &["verdict", "approve"]));
    ok(atelier(&root, "g1", &["merge"]));
    git(&root, &["switch", "-q", "main"]);
    git(&root, &["branch", "-qD", ID]);

    let lines = log(&root, &[DONE]);
    assert_eq!(
        acts(&root, &lines),
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
fn a_mission_shows_its_ready_lead_plan_review_and_close_with_its_items_acts() {
    let scratch = Scratch::new(&[(ID, "draft"), ("demo-0ther", "draft")]);
    let root = scratch.root();
    write_mission(&root);
    git(&root, &["commit", "-qam", "Plan the mission"]);
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Add the mission"]);
    ok(atelier(
        &root,
        "p1",
        &["ready", MISSION, ID, "demo-0ther", "--by", "sumeet"],
    ));
    ok(atelier(&root, "l1", &["lead", MISSION]));
    ok(atelier(&root, "r1", &["review", MISSION]));
    ok(atelier(&root, "r1", &["verdict", "approve"]));
    ok(atelier(
        &root,
        "p1",
        &["drop", ID, "--reason", "Nobody needs it."],
    ));
    append(
        &root,
        &format!(".atelier/missions/{MISSION}/mission.md"),
        "\n## Result\n\nNothing was needed.\n",
    );
    git(&root, &["commit", "-qam", "Result"]);
    ok(atelier(&root, "p1", &["close", MISSION]));
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
        format!("{ID} ready - draft -> ready; by sumeet"),
        format!("{MISSION} ready - draft -> ready; by sumeet"),
        format!("{MISSION} lead l1"),
        format!("{MISSION} review r1 {review}"),
        format!("{MISSION} verdict r1 {review} approve"),
        format!("{ID} drop - ready -> dropped; Nobody needs it."),
        format!("{MISSION} close - ready -> done"),
    ];
    assert_eq!(acts(&root, &log(&root, &[MISSION])), ours);
    // Everything holds the other item too; the item alone holds its own.
    let all = acts(&root, &log(&root, &[]));
    assert_eq!(all.len(), ours.len() + 1);
    assert_eq!(all[0], "demo-0ther ready - draft -> ready; by sumeet");
    assert_eq!(
        acts(&root, &log(&root, &[ID])),
        [ours[0].clone(), ours[5].clone()]
    );
}

#[test]
fn a_skip_ends_the_line_of_the_act_whose_commit_carries_it() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let output = atelier_command(&root, "t1", &["start", ID])
        .env("ATELIER_SKIP", REASON)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        acts(&root, &log(&root, &[])),
        [format!(
            "{ID} start t1 draft -> building; skip {ID} is ready (it is draft): {REASON}"
        )]
    );
}

#[test]
fn since_leaves_out_the_acts_committed_before_it() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let at = |time: &str, thread: &str, args: &[&str]| {
        let output = atelier_command(&root, thread, args)
            .env("GIT_COMMITTER_DATE", time)
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
    };
    at(
        "2026-01-01T00:00:00Z",
        "p1",
        &["ready", ID, "--by", "sumeet"],
    );
    at("2026-02-01T12:00:00Z", "t1", &["start", ID]);
    let ready = format!(
        "2026-01-01T00:00:00Z {} {ID} ready - draft -> ready; by sumeet",
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
fn two_acts_on_one_record_in_one_commit_show_as_their_net_change() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    // Made ready and closed at once: only closed.
    let path = root.join(item_path(ID));
    write_item(&root, ID, "done", &[], &[], "Not needed.\n");
    let text = fs::read_to_string(&path).unwrap().replace(
        "ready: null",
        "ready:\n  by: sumeet\n  at: 2026-11-10T09:00:00Z",
    );
    fs::write(&path, text).unwrap();
    git(&root, &["commit", "-qam", "Ready and close"]);
    assert_eq!(
        acts(&root, &log(&root, &[])),
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
    git(&root, &["commit", "-qam", "Break it"]);
    let broken = short(&root, "HEAD");
    fs::write(&path, draft.replace("status: draft", "status: ready")).unwrap();
    git(&root, &["commit", "-qam", "Mend it"]);

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
    // The acts across it compare the versions on each side.
    let lines: Vec<String> = run.stdout.lines().map(str::to_string).collect();
    assert_eq!(
        acts(&root, &lines),
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
/// branch's acts would be lost with the branch.
#[test]
fn acts_on_a_branch_whose_records_a_plain_merge_matches_survive_its_deletion() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    ok(atelier(&root, "t1", &["start", ID]));
    let started = short(&root, "HEAD");
    hello(&root);
    git(&root, &["revert", "--no-edit", &started]);
    git(&root, &["switch", "-q", "main"]);
    git(
        &root,
        &["merge", "-q", "--no-ff", "-m", "Merge the attempt", ID],
    );
    git(&root, &["branch", "-qD", ID]);
    let lines = log(&root, &[]);
    assert_eq!(
        acts(&root, &lines),
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
    git(&root, &["commit", "-qam", "Break its encoding"]);
    let broken = short(&root, "HEAD");
    fs::write(&path, draft.replace("status: draft", "status: ready")).unwrap();
    git(&root, &["commit", "-qam", "Mend it"]);

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
            acts(&root, &lines),
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
        git(&root, &["commit", "-qam", status]);
    }
    let run = atelier(&root, "t9", &["log"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(run.stderr.lines().count(), 1, "{}", run.stderr);
    let lines: Vec<String> = run.stdout.lines().map(str::to_string).collect();
    assert_eq!(
        acts(&root, &lines),
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
    let acts = acts(&root, &lines);
    assert!(
        acts.contains(&format!("{ID} close - draft -> done")),
        "{acts:?}"
    );
    assert!(
        acts.contains(&format!("{ID} drop - done -> dropped")),
        "{acts:?}"
    );
    assert_eq!(acts.len(), 3, "{acts:?}");
}
