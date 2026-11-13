//! A disputed finding, driven through the new binary: a rebuttal the next
//! reviewer withdraws, or keeps and then lowers; what `list --disputed`,
//! `show <mission>` and a lead's `status` say of each; and the records
//! that `escalate` and `decide` wrote before they were removed, which still
//! read as they did.

mod common;

use common::*;
use std::fs;
use std::path::Path;

const NO_PERIOD: &str = "\n## Finding 1: No period\nRecommendation: fix now\n\
                         hello.txt says `hello`, without the Goal's period.\n";

const FACT: &str = "the Goal's period ends its sentence";

/// Has t1, on the branch it holds, rebut `finding` and submit again.
fn rebut(root: &Path, finding: &str) {
    append(
        root,
        &item_path(ID),
        &format!("\n### Responses\n\n- {finding}: rebutted: {FACT}\n"),
    );
    let run = atelier(root, "t1", &["submit"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

/// A scratch repository whose item t1 submitted and r1 reviewed in round
/// 1, with one fix-now finding, and the finding's name.
fn changes_requested() -> (Scratch, String) {
    let scratch = submitted();
    let root = scratch.root();
    request_changes(&root);
    (scratch, format!("{}.1", review_of(&root, 1).0))
}

/// r1 reviews round 1 of the item in review on `root`, asking for one fix.
fn request_changes(root: &Path) {
    assert_eq!(atelier(root, "r1", &["review", ID]).code, 0);
    let (_, path) = review_of(root, 1);
    append(root, &path, NO_PERIOD);
    let run = verdict(root, "r1", &["verdict", "changes"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

/// `round`'s review file, opened by `reviewer`, with its carried finding
/// rewritten by `judge`, and its verdict recorded.
fn judge(root: &Path, reviewer: &str, round: u32, judge: impl Fn(&str) -> String, verdict: &str) {
    assert_eq!(atelier(root, reviewer, &["review", ID]).code, 0);
    let (_, path) = review_of(root, round);
    let text = read(root, &path);
    assert!(
        text.contains(&format!("Response: rebutted: {FACT}\nNow: kept\n")),
        "{text}"
    );
    fs::write(root.join(&path), judge(&text)).unwrap();
    let run = common::verdict(root, reviewer, &["verdict", verdict]);
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);
}

fn disputed(root: &Path, args: &[&str]) -> String {
    let run = atelier(root, "t9", &[&["list", "--disputed"], args].concat());
    assert_eq!(run.code, 0, "{}", run.stderr);
    run.stdout
}

const HEADING: &str = "Disputed: each rebutted finding with what the next round made of it, \
                       and each plan-review round whose reviewers' verdicts differ.\n";

#[test]
fn a_rebuttal_the_next_reviewer_withdraws_closes_its_finding() {
    let (scratch, finding) = changes_requested();
    let root = scratch.root();
    assert_eq!(
        disputed(&root, &[]),
        "Nothing is disputed: no finding was rebutted, and no plan-review round's reviewers disagreed.\n"
    );
    rebut(&root, &finding);
    let rebuttal = |next: &str| {
        format!("{HEADING}  {ID}/{finding} ({next}): No period\n    Rebutted: {FACT}\n")
    };
    assert_eq!(disputed(&root, &[]), rebuttal("not yet reviewed"));

    judge(
        &root,
        "r2",
        2,
        |text| text.replace("Now: kept", "Now: withdrawn"),
        "approve",
    );
    assert_eq!(field(&root, "HEAD", "status"), "approved");
    let carrier = format!("{}.1", review_of(&root, 2).0);
    assert_eq!(
        disputed(&root, &[]),
        rebuttal(&format!("withdrawn in {carrier}"))
    );
    assert_eq!(atelier(&root, "t1", &["merge"]).code, 0);
    assert_eq!(field(&root, "main", "status"), "done");
    // A done item's dispute is still listed.
    assert_eq!(
        disputed(&root, &[]),
        rebuttal(&format!("withdrawn in {carrier}"))
    );
}

#[test]
fn a_rebuttal_the_next_reviewer_keeps_has_had_its_one_try_until_a_reviewer_lowers_it() {
    let (scratch, finding) = changes_requested();
    let root = scratch.root();
    rebut(&root, &finding);
    judge(&root, "r2", 2, |text| text.to_string(), "changes");
    let carrier = format!("{}.1", review_of(&root, 2).0);
    assert!(
        disputed(&root, &[]).contains(&format!("{ID}/{finding} (kept in {carrier}): No period\n")),
        "{}",
        disputed(&root, &[])
    );
    // The implementer is told the rebuttal had its try, and is pointed at
    // nothing but a response.
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.contains(&format!(
            "  [ ] Finding {carrier} (fix now): No period (a reviewer kept its rebuttal, and a \
             rebuttal gets one try: fix it)\n"
        )),
        "{status}"
    );
    assert!(!status.contains("escalate"), "{status}");
    assert!(!status.contains("decide"), "{status}");

    // A third reviewer keeps it, lowered to defer, and says why; the item
    // approves and merges, and the finding waits to be settled.
    append(
        &root,
        &item_path(ID),
        &format!("- {carrier}: rebutted: {FACT}, and the next item adds the period\n"),
    );
    assert_eq!(atelier(&root, "t1", &["submit"]).code, 0);
    assert_eq!(atelier(&root, "r3", &["review", ID]).code, 0);
    let (stem, path) = review_of(&root, 3);
    let text = read(&root, &path);
    assert!(
        text.contains("<!-- Set Now: to fixed, withdrawn or kept. To keep it for later work instead, set Recommendation: to defer or no action and add a line that says why. -->"),
        "{text}"
    );
    let lowered = text.replace("Recommendation: fix now", "Recommendation: defer");
    fs::write(root.join(&path), &lowered).unwrap();
    // Without a reason the file does not read, and nothing is recorded.
    let refused = verdict(&root, "r3", &["verdict", "approve"]);
    assert_eq!(refused.code, 2, "{}{}", refused.stdout, refused.stderr);
    assert!(refused.stderr.contains("says why"), "{}", refused.stderr);
    fs::write(
        root.join(&path),
        lowered + "Right, but it belongs to the next item.\n",
    )
    .unwrap();
    let run = verdict(&root, "r3", &["verdict", "approve"]);
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);
    assert!(
        disputed(&root, &[]).contains(&format!(
            "{ID}/{carrier} (kept at defer in {stem}.1): No period\n"
        )),
        "{}",
        disputed(&root, &[])
    );
    let merged = atelier(&root, "t1", &["merge"]);
    assert_eq!(merged.code, 0, "{}", merged.stderr);
    git(&root, &["checkout", "-q", "--detach", "main"]);
    assert!(atelier(&root, "t9", &["list", "--deferred"])
        .stdout
        .contains(&format!("  {ID}/{stem}.1: No period\n")),);
}

#[test]
fn escalate_and_decide_are_gone() {
    let (scratch, finding) = changes_requested();
    let root = scratch.root();
    for args in [
        vec!["escalate", "Worth it?", "--finding", finding.as_str()],
        vec!["decide", ID, "No.", "--defer"],
    ] {
        let run = atelier(&root, "t1", &args);
        assert_eq!(run.code, 2, "{args:?}");
        assert!(
            run.stderr
                .contains(&format!("unrecognized subcommand '{}'", args[0])),
            "{}",
            run.stderr
        );
    }
    let help = atelier(&root, "t1", &["--help"]).stdout;
    assert!(
        !help.contains("escalate") && !help.contains("decide"),
        "{help}"
    );
}

/// What `escalate` and `decide` wrote, as an item finished before they
/// were removed holds it: a question about the round-1 finding, answered
/// with `Outcome: defer`; a round 2 carrying it with the tool's `Decided:`
/// line and approving; and one question asked and never answered.
fn decided_before_removal(root: &Path, finding: &str) -> String {
    let decisions = format!(
        "\n## Decisions\n\n### 2026-11-10: Is the period worth a round?\n\nFinding: {finding}\n\
         Answer by d1: Not now.\nOutcome: defer\n\n### 2026-11-11: Should it say hello twice?\n"
    );
    let item = read(root, &item_path(ID)).replace("status: changes_requested", "status: done");
    fs::write(root.join(item_path(ID)), item + &decisions).unwrap();
    let carrier = format!("{REVIEWS}/2-dcdd.md");
    let commit = git(root, &["rev-parse", "HEAD"]);
    fs::write(
        root.join(&carrier),
        format!(
            "---\nschema: atelier.review\nschema_version: 3\nround: 2\ncommit: {commit}\n\
             reviewers:\n- r2\nmodel: null\nverdict: approve\ncreated_at: 2026-11-10T10:00:00Z\n---\n\n\
             ## Summary\n\nLooked again.\n\n## Finding 1: No period\nRecommendation: fix now\n\
             Was: {finding}\nDecided: defer\n<!-- A decision settled it, and the tool wrote `Decided:`. Leave this finding as it is. -->\n"
        ),
    )
    .unwrap();
    git(root, &["add", "-A"]);
    git(root, &["commit", "-qm", "Decided"]);
    git(root, &["switch", "-q", "main"]);
    git(root, &["merge", "-q", "--no-ff", ID, "-m", "Merge"]);
    // Nothing is written in the base's own checkout.
    git(root, &["checkout", "-q", "--detach"]);
    "2-dcdd.1".into()
}

#[test]
fn an_item_decided_before_escalate_and_decide_were_removed_reads_as_it_did() {
    let (scratch, finding) = changes_requested();
    let root = scratch.root();
    let carrier = decided_before_removal(&root, &finding);
    let run = atelier(&root, "t9", &["check"]);
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);
    // Neither finding is open; the carrier is the deferred one.
    let shown = atelier(&root, "t9", &["show", ID]).stdout;
    assert!(
        shown.contains(&format!("    {finding} (fix now, closed): No period\n")),
        "{shown}"
    );
    assert!(
        shown.contains(&format!("    {carrier} (fix now, closed): No period\n")),
        "{shown}"
    );
    assert!(
        shown.contains("### 2026-11-11: Should it say hello twice?"),
        "{shown}"
    );
    let deferred = atelier(&root, "t9", &["list", "--deferred"]).stdout;
    assert!(
        deferred.starts_with(&format!(
            "Deferred findings of done items that nobody has settled:\n  {ID}/{carrier}: No period\n"
        )),
        "{deferred}"
    );
    let run = atelier(
        &root,
        "t9",
        &[
            "drop",
            &format!("{ID}/{carrier}"),
            "--reason",
            "Punctuation.",
        ],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    // Nothing was rebutted, so nothing is disputed.
    assert!(disputed(&root, &[]).starts_with("Nothing is disputed"));
}

/// A ready mission led by `lead1`, holding the item, started by t1, and
/// two plan reviews of its round 1 that disagree.
fn led() -> Scratch {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let dir = ".atelier/missions/demo-m1ss";
    fs::create_dir_all(root.join(format!("{dir}/reviews"))).unwrap();
    fs::write(
        root.join(format!("{dir}/mission.md")),
        "---\nschema: atelier.mission\nschema_version: 3\nid: demo-m1ss\ntitle: Greet\n\
         status: ready\nlabels: []\nlead: lead1\nready:\n  by: sumeet\n  at: 2026-11-10T09:00:00Z\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Outcome\n\nGreetings.\n",
    )
    .unwrap();
    let review = |verdict: &str, by: &str, model: &str, body: &str| {
        format!(
            "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: abc1234\n\
             reviewers:\n- {by}\nmodel: {model}\nverdict: {verdict}\n\
             created_at: 2026-11-10T09:00:00Z\n---\n\n## Summary\n\ns\n{body}"
        )
    };
    let reviews = [
        ("1-aaaa", review("approve", "p1", "gpt-6.1-sol", "")),
        ("1-bbbb", review("changes", "p2", "null", NO_PERIOD)),
        ("2-cccc", review("approve", "p2", "null", "")),
    ];
    for (stem, text) in reviews {
        fs::write(root.join(format!("{dir}/reviews/{stem}.md")), text).unwrap();
    }
    let item = read(&root, &item_path(ID)).replace("mission: null", "mission: demo-m1ss");
    fs::write(root.join(item_path(ID)), item).unwrap();
    on_base(&root, "Mission");
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    scratch
}

#[test]
fn a_missions_disputes_are_listed_for_it_shown_with_it_and_told_to_its_lead() {
    let scratch = led();
    let root = scratch.root();
    let base = scratch.worktree("base");
    git(&base, &["switch", "-q", "main"]);
    let split = "  demo-m1ss plan, round 1: 1-aaaa approved by p1, gpt-6.1-sol; 1-bbbb changes \
                 requested by p2\n";
    let shown = atelier(&base, "lead1", &["show", "demo-m1ss"]).stdout;
    assert!(
        shown.contains(&format!("\nDisputed:\n{split}\nPlan reviews:\n")),
        "{shown}"
    );

    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "hello.txt"]);
    git(&root, &["commit", "-qm", "Say hello"]);
    finish_and_submit(&root, "t1");
    request_changes(&root);
    let finding = format!("{}.1", review_of(&root, 1).0);
    rebut(&root, &finding);
    let rebuttal =
        format!("  {ID}/{finding} (not yet reviewed): No period\n    Rebutted: {FACT}\n");

    // Read from the item's branch, from anywhere.
    for at in [&root, &base] {
        assert_eq!(
            disputed(at, &["--mission", "demo-m1ss"]),
            format!("Disputed in mission demo-m1ss: each rebutted finding with what the next round made of it, and each plan-review round whose reviewers' verdicts differ.\n{rebuttal}{split}")
        );
    }
    assert_eq!(
        disputed(&base, &["--mission", "demo-zz99"]),
        "Nothing is disputed in mission demo-zz99: no finding was rebutted, and no plan-review round's reviewers disagreed.\n"
    );
    let shown = atelier(&base, "lead1", &["show", "demo-m1ss"]).stdout;
    assert!(
        shown.contains(&format!("\nDisputed:\n{rebuttal}{split}")),
        "{shown}"
    );
    let status = atelier(&base, "lead1", &["status"]).stdout;
    assert!(
        status.contains(&format!(
            "Disputed in this mission (nothing waits on these; a lead steps in by moving disputed \
             work out of an item's Scope into an item of its own):\n{rebuttal}{split}"
        )),
        "{status}"
    );
    // The implementer's own status is not changed by it.
    assert!(!atelier(&root, "t1", &["status"])
        .stdout
        .contains("Disputed"));
}
