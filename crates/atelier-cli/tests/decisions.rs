//! A finding's three exits, driven through the new binary: a fix, a
//! rebuttal the next reviewer withdraws or keeps, and a question
//! `escalate` asks and a decider answers with `decide`.

mod common;

use common::*;
use std::fs;
use std::path::Path;

const NO_PERIOD: &str = "\n## Finding 1: No period\nRecommendation: fix now\n\
                         hello.txt says `hello`, without the Goal's period.\n";

/// What `status`, and every refused command, says of a question.
const QUESTION: &str =
    "2026-11-10: Is the period worth a round? The Goal's period ends its sentence.";

/// A scratch repository whose item t1 submitted and r1 reviewed in round
/// 1, with one fix-now finding, and the finding's name.
fn changes_requested() -> (Scratch, String) {
    let scratch = submitted();
    let root = scratch.root();
    assert_eq!(atelier(&root, "r1", &["review", ID]).code, 0);
    let (stem, path) = review_of(&root, 1);
    append(&root, &path, NO_PERIOD);
    let run = atelier(&root, "r1", &["verdict", "changes"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    (scratch, format!("{stem}.1"))
}

/// `escalate`'s question, dated today in UTC as it dates one.
fn asked() -> String {
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    QUESTION.replacen("2026-11-10", &today, 1)
}

fn escalate(root: &Path, finding: Option<&str>) -> Run {
    let question = &QUESTION["2026-11-10: ".len()..];
    let mut args = vec!["escalate", question];
    if let Some(finding) = finding {
        args.extend(["--finding", finding]);
    }
    atelier(root, "t1", &args)
}

/// The handoff for a decider of a question about `finding` in `review`.
fn decider_handoff(finding: &str) -> String {
    let stem = finding.rsplit_once('.').unwrap().0;
    format!(
        "A decider must be a thread other than yours. Under Codex a subagent you start is one; \
         under Claude Code a subagent runs as your thread and is refused, so use your harness's \
         tool for a new agent thread. Start one with fresh context, not as a fork of yours, and \
         give it this task:\n  Run atelier show {ID}, and read the question waiting under its Decisions and \
         finding {finding} in {REVIEWS}/{stem}.md. Answer it with atelier decide {ID} '<your \
         answer>', adding --defer to defer the finding or --no-action to close it without a fix, \
         or neither to leave it to be fixed.\nThen wait for its answer, and run `atelier status`.\n"
    )
}

#[test]
fn a_rebuttal_the_next_reviewer_withdraws_closes_its_finding() {
    let (scratch, finding) = changes_requested();
    let root = scratch.root();
    append(
        &root,
        &item_path(ID),
        &format!("\n### Responses\n\n- {finding}: rebutted: the Goal's period ends its sentence\n"),
    );
    assert_eq!(atelier(&root, "t1", &["submit"]).code, 0);
    assert_eq!(atelier(&root, "r2", &["review", ID]).code, 0);
    let (_, path) = review_of(&root, 2);
    let text = read(&root, &path);
    assert!(
        text.contains("Response: rebutted: the Goal's period ends its sentence\nNow: kept\n"),
        "{text}"
    );
    fs::write(
        root.join(&path),
        text.replace("Now: kept", "Now: withdrawn"),
    )
    .unwrap();
    let run = atelier(&root, "r2", &["verdict", "approve"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(field(&root, "HEAD", "status"), "approved");
    assert_eq!(atelier(&root, "t1", &["merge"]).code, 0);
    assert_eq!(field(&root, "main", "status"), "done");
}

#[test]
fn an_escalation_decided_as_defer_is_carried_as_decided_and_the_item_merges() {
    let (scratch, finding) = changes_requested();
    let root = scratch.root();
    let head = git(&root, &["rev-parse", "HEAD"]);

    // The implementer asks; nothing is committed.
    let run = escalate(&root, Some(&finding));
    assert_eq!(run.code, 0, "{}", run.stderr);
    let asked = asked();
    assert_eq!(
        run.stdout,
        format!(
            "Asked under `## Decisions` in {}, and did not commit it:\n  {asked}\n\
             It is about finding {finding}.\n`atelier submit` and `atelier merge` wait until it \
             is answered, and `atelier submit` commits it with the item.\n{}",
            item_path(ID),
            decider_handoff(&finding)
        )
    );
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    assert_eq!(
        git(&root, &["status", "--porcelain"]),
        format!("M {}", item_path(ID))
    );
    assert!(read(&root, &item_path(ID)).ends_with(&format!(
        "\n## Decisions\n\n### {asked}\n\nFinding: {finding}\n"
    )));

    // Its status shows the question and who decides it; submit waits.
    let status = atelier(&root, "t1", &["status"]).stdout;
    let waits = format!(
        "Status: changes requested in round 1\nTo return it to review:\n  [ ] Question answered: {asked}\n{}",
        decider_handoff(&finding)
    );
    assert!(status.contains(&waits), "{status}");
    let refused = atelier(&root, "t1", &["submit"]);
    assert_eq!(refused.code, 1);
    assert!(
        refused
            .stderr
            .contains(&format!("  [ ] Question answered: {asked}\n")),
        "{}",
        refused.stderr
    );
    assert!(!refused.stderr.contains("(fix now)"), "{}", refused.stderr);
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);

    // Neither its implementer nor the finding's reviewer decides it.
    let before = read(&root, &item_path(ID));
    let implementer = atelier(&root, "t1", &["decide", ID, "No."]);
    assert_eq!(implementer.code, 1);
    assert_eq!(
        implementer.stderr,
        format!(
            "`atelier decide {ID}` refused; nothing changed.\nYou (thread t1) implemented {ID}, \
             and none of its implementers, past or present, decides its questions.\n{}{SKIP}",
            decider_handoff(&finding)
        )
    );
    let reviewer = atelier(&root, "r1", &["decide", ID, "No.", "--no-action"]);
    assert_eq!(reviewer.code, 1);
    assert_eq!(
        reviewer.stderr,
        format!(
            "`atelier decide {ID}` refused; nothing changed.\nYou (thread r1) reviewed finding \
             {finding}, which its question is about, and none of its reviewers decides it.\n\
             Say so to whoever started you, and stop.\n{SKIP}"
        )
    );
    assert_eq!(read(&root, &item_path(ID)), before);

    // A third thread decides it as deferred; nothing is committed.
    let decided = atelier(&root, "d1", &["decide", ID, "Not now.", "--defer"]);
    assert_eq!(decided.code, 0, "{}", decided.stderr);
    assert_eq!(
        decided.stdout,
        format!(
            "Answered a question on {ID}, as thread d1:\n  {asked}\n  Answer: Not now.\n\
             Finding {finding} is settled, `Decided: defer`: the next review carries it so, and \
             no review reopens it.\nWrote {}, and did not commit it.\nNo other question waits. \
             Its implementer, thread t1, runs `atelier submit`, which commits your answer with \
             the item. Your decision is done: say so to whoever started you, and stop.\n",
            item_path(ID)
        )
    );
    assert!(read(&root, &item_path(ID)).ends_with(&format!(
        "Finding: {finding}\nAnswer by d1: Not now.\nOutcome: defer\n"
    )));
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    // Settled, the finding needs no response, and nothing is left to ask.
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.contains("Status: changes requested in round 1\n`atelier submit` commits"),
        "{status}"
    );
    let again = atelier(&root, "d1", &["decide", ID, "Yes."]);
    assert_eq!(again.code, 1);
    assert_eq!(
        again.stderr,
        format!(
            "`atelier decide {ID}` refused; nothing changed.\n{ID} has no unanswered question \
             under `## Decisions`.\n"
        )
    );

    // Submit commits the decision; the next review carries it as decided.
    let submit = atelier(&root, "t1", &["submit"]);
    assert_eq!(submit.code, 0, "{}", submit.stderr);
    assert_eq!(git(&root, &["status", "--porcelain"]), "");
    let brief = atelier(&root, "r2", &["review", ID]);
    assert_eq!(brief.code, 0, "{}", brief.stderr);
    let (_, path) = review_of(&root, 2);
    let carried = format!(
        "## Finding 1: No period\nRecommendation: fix now\nWas: {finding}\nDecided: defer\n\
         <!-- A decision settled it, and the tool wrote `Decided:`. Leave this finding as it \
         is. -->\n"
    );
    let written = read(&root, &path);
    assert!(written.ends_with(&carried), "{written}");
    for part in [
        "One a decision settled has a `Decided:` line instead, which the tool wrote: leave it as \
         it is.\n"
            .to_string(),
        format!(
            "  Decided: defer. The question: {asked}\n  Answer by d1: Not now.\n\nYour review file"
        ),
    ] {
        assert!(
            brief.stdout.contains(&part),
            "no `{part}` in:\n{}",
            brief.stdout
        );
    }

    // The reviewer cannot reopen it: a changed line refuses the verdict.
    fs::write(
        root.join(&path),
        written.replace("Decided: defer", "Now: kept"),
    )
    .unwrap();
    let reopened = atelier(&root, "r2", &["verdict", "changes"]);
    assert_eq!(reopened.code, 1);
    assert_eq!(
        reopened.stderr,
        format!(
            "`atelier verdict changes` refused; nothing changed.\nTo record your verdict:\n  \
             [ ] Your file carries {finding} with the `Decided: defer` line the tool wrote (a \
             decision settled it; put that finding back as it was)\nThen: atelier verdict \
             changes\n{SKIP}"
        )
    );
    fs::write(root.join(&path), &written).unwrap();
    let approved = atelier(&root, "r2", &["verdict", "approve"]);
    assert_eq!(approved.code, 0, "{}", approved.stderr);
    let merged = atelier(&root, "t1", &["merge"]);
    assert_eq!(merged.code, 0, "{}", merged.stderr);
    assert_eq!(field(&root, "main", "status"), "done");
    assert!(git(&root, &["show", &format!("main:{path}")]).contains("Decided: defer\n"));
}

#[test]
fn a_question_about_no_finding_once_answered_lets_submit_pass() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let path = root.join(item_path(ID));
    let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, ticked + "It prints hello.\n").unwrap();
    assert_eq!(escalate(&root, None).code, 0);
    let refused = atelier(&root, "t1", &["submit"]);
    assert_eq!(refused.code, 1);
    assert!(refused
        .stderr
        .contains(&format!("  [ ] Question answered: {}\n", asked())));

    // No finding to settle: --defer is refused, with nothing written.
    let before = read(&root, &item_path(ID));
    let settle = atelier(&root, "d1", &["decide", ID, "Yes.", "--defer"]);
    assert_eq!(settle.code, 1);
    assert_eq!(
        settle.stderr,
        format!(
            "`atelier decide {ID} --defer` refused; nothing changed.\nIts oldest question, which \
             it answers, is about no finding, so there is nothing to settle:\n  {}\n\
             Then: atelier decide {ID} \"<answer>\"\n",
            asked()
        )
    );
    assert_eq!(read(&root, &item_path(ID)), before);

    // Two wait: the oldest is answered first, and the decider is told of
    // the next.
    assert_eq!(
        atelier(&root, "t1", &["escalate", "And how loudly?"]).code,
        0
    );
    let first = atelier(&root, "d1", &["decide", ID, "Yes."]);
    assert_eq!(first.code, 0, "{}", first.stderr);
    assert!(
        first.stdout.ends_with(&format!(
            "Another question waits on {ID}:\n  {}: And how loudly?\nThen: atelier decide {ID} \
             \"<answer>\"\n",
            &asked()[..10]
        )),
        "{}",
        first.stdout
    );
    assert!(read(&root, &item_path(ID)).contains(&format!("{}\n\nAnswer by d1: Yes.\n", asked())));
    let decided = atelier(&root, "d1", &["decide", ID, "Yes, say it loudly."]);
    assert_eq!(decided.code, 0, "{}", decided.stderr);
    assert!(!decided.stdout.contains("settled"), "{}", decided.stdout);
    assert!(
        decided.stdout.contains("No other question waits."),
        "{}",
        decided.stdout
    );
    let submit = atelier(&root, "t1", &["submit"]);
    assert_eq!(submit.code, 0, "{}", submit.stderr);
    let committed = git(&root, &["show", &format!("HEAD:{}", item_path(ID))]);
    assert!(
        committed.ends_with("Answer by d1: Yes, say it loudly."),
        "{committed}"
    );
}

#[test]
fn escalate_is_its_implementers_on_its_branch_and_writes_nothing_when_refused() {
    let (scratch, finding) = changes_requested();
    let root = scratch.root();
    let before = read(&root, &item_path(ID));
    let unchanged = |root: &Path| assert_eq!(read(root, &item_path(ID)), before);

    // A finding no review has, or one that is not open.
    let (stem, path) = review_of(&root, 1);
    for (named, said) in [
        (
            format!("{stem}.9"),
            format!("No review of {ID} has a finding {stem}.9.\n"),
        ),
        (
            "x".to_string(),
            format!("No review of {ID} has a finding x.\n"),
        ),
    ] {
        let run = escalate(&root, Some(&named));
        assert_eq!(run.code, 1, "{named}");
        assert_eq!(
            run.stderr,
            format!(
                "`atelier escalate` refused; nothing changed.\n{said}Its open fix-now findings: \
                 {finding}\n"
            )
        );
        unchanged(&root);
    }
    // A question on more than one line.
    let run = atelier(&root, "t1", &["escalate", "Why?\nBecause."]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        "`atelier escalate \"<question>\"` needs its question on one line. Nothing changed.\n"
    );
    unchanged(&root);
    // Another thread, which does not hold it.
    let run = atelier(&root, "t2", &["escalate", "Why?"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.starts_with("You (thread t2) do not hold"),
        "{}",
        run.stderr
    );
    unchanged(&root);

    // Fixed and closed in round 2, the finding is no longer open.
    append(
        &root,
        &item_path(ID),
        &format!("\n### Responses\n\n- {finding}: fixed in abc1234\n"),
    );
    assert_eq!(atelier(&root, "t1", &["submit"]).code, 0);
    // In review, its implementer asks nothing.
    let run = escalate(&root, None);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.starts_with(&format!(
            "{ID} is in review, and `atelier escalate` runs on an item that is building, changes \
             requested, approved. Nothing changed.\n"
        )),
        "{}",
        run.stderr
    );
    assert_eq!(atelier(&root, "r2", &["review", ID]).code, 0);
    let (_, second) = review_of(&root, 2);
    let text = read(&root, &second).replace("Now: kept", "Now: fixed");
    fs::write(root.join(&second), text).unwrap();
    append(
        &root,
        &second,
        "\n## Finding 2: Loud\nRecommendation: fix now\n",
    );
    assert_eq!(atelier(&root, "r2", &["verdict", "changes"]).code, 0);
    let before = read(&root, &item_path(ID));
    let run = escalate(&root, Some(&finding));
    assert_eq!(run.code, 1);
    let open = format!("{}.2", review_of(&root, 2).0);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier escalate` refused; nothing changed.\nFinding {finding} of {ID} is not an \
             open fix-now finding.\nIts open fix-now findings: {open}\n"
        )
    );
    assert_eq!(read(&root, &item_path(ID)), before);
    let _ = path;

    // Off its branch.
    git(&root, &["switch", "-q", "main"]);
    let run = escalate(&root, None);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr
            .starts_with("`atelier escalate` runs on an item's branch"),
        "{}",
        run.stderr
    );
}

#[test]
fn decide_runs_in_the_checkout_on_the_items_branch_and_names_it() {
    let (scratch, finding) = changes_requested();
    let root = scratch.root();
    assert_eq!(escalate(&root, Some(&finding)).code, 0);
    let base = scratch.worktree("base");
    git(&base, &["switch", "-q", "main"]);
    let run = atelier(&base, "d1", &["decide", ID, "Not now."]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier decide {ID}` runs in the checkout on branch {ID}, where its implementer \
             works, and this checkout is on branch main. Nothing changed.\nRun it in {}, which \
             has branch {ID} checked out.\n",
            root.display()
        )
    );
    assert!(!read(&root, &item_path(ID)).contains("Answer by"));
}

/// A ready mission led by `lead1`, holding the item, started by t1.
fn led() -> Scratch {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let mission = ".atelier/missions/demo-m1ss/mission.md";
    fs::create_dir_all(root.join(".atelier/missions/demo-m1ss")).unwrap();
    fs::write(
        root.join(mission),
        "---\nschema: atelier.mission\nschema_version: 3\nid: demo-m1ss\ntitle: Greet\n\
         status: ready\nlabels: []\nlead: lead1\nready:\n  by: sumeet\n  at: 2026-11-10T09:00:00Z\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Outcome\n\nGreetings.\n",
    )
    .unwrap();
    let item = read(&root, &item_path(ID)).replace("mission: null", "mission: demo-m1ss");
    fs::write(root.join(item_path(ID)), item).unwrap();
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Mission"]);
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    scratch
}

#[test]
fn a_missions_lead_decides_its_items_questions_and_sees_each_and_since_when_one_is_approved() {
    let scratch = led();
    let root = scratch.root();
    let run = escalate(&root, None);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let path = item_path(ID);
    let commit = format!("git add -- {path} && git commit -m \"Ask about {ID}\" -- {path}");
    let handoff = format!(
        "Thread lead1, the lead of mission demo-m1ss, decides it: lead1's `atelier status` lists \
         the question.\nlead1's `atelier status` reads this branch's last commit, so commit the \
         question first: {commit}\nTell whoever started you that it waits, and run `atelier \
         status` once it is answered.\n"
    );
    assert!(run.stdout.ends_with(&handoff), "{}", run.stdout);
    assert!(atelier(&root, "t1", &["status"]).stdout.contains(&handoff));
    let asked = asked();
    let waiting = format!(
        "  {ID}: {asked}\n    Answer it in the checkout on branch {ID}: atelier decide {ID} \
         \"<answer>\"\n"
    );
    // The lead's status and `show` read the item from its branch's tip:
    // the question shows there once the command printed commits it.
    let base = scratch.worktree("base");
    git(&base, &["switch", "-q", "main"]);
    let status = atelier(&base, "lead1", &["status"]).stdout;
    assert!(!status.contains("Questions waiting"), "{status}");
    let committed = identity(std::process::Command::new("sh").current_dir(&root))
        .args(["-c", &commit])
        .output()
        .unwrap();
    assert!(committed.status.success(), "{committed:?}");
    assert_eq!(git(&root, &["status", "--porcelain"]), "");
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(!status.contains("commit the question first"), "{status}");
    // Its implementer, away from the branch, sees the question too.
    let held = atelier(&base, "t1", &["status"]).stdout;
    assert!(
        held.contains(&format!(
            "You hold:\n  {ID} (building): Say hello\n    A question waits on a decider: {asked}\n"
        )),
        "{held}"
    );
    let status = atelier(&base, "lead1", &["status"]).stdout;
    assert!(
        status.contains(&format!(
            "Questions waiting on a decider, oldest first on each item:\n{waiting}"
        )),
        "{status}"
    );
    let shown = atelier(&base, "lead1", &["show", "demo-m1ss"]).stdout;
    assert!(
        shown.contains(&format!("\nQuestions:\n{waiting}")),
        "{shown}"
    );

    // The lead answers it on the item's branch.
    let run = atelier(&root, "lead1", &["decide", ID, "Yes."]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let path = root.join(item_path(ID));
    let finished = fs::read_to_string(&path)
        .unwrap()
        .replace("- [ ]", "- [x]")
        .replace("## Report\n\n", "## Report\n\nIt prints hello.\n\n");
    fs::write(&path, finished).unwrap();
    let submit = atelier(&root, "t1", &["submit"]);
    assert_eq!(submit.code, 0, "{}", submit.stderr);
    assert_eq!(atelier(&root, "r1", &["review", ID]).code, 0);
    assert_eq!(atelier(&root, "r1", &["verdict", "approve"]).code, 0);

    // Approved and not merged: since when, to the lead and in `show`.
    let since = format!(
        "approved since {}",
        git(&root, &["log", "-1", "--format=%cI", ID])
    );
    let status = atelier(&base, "lead1", &["status"]).stdout;
    assert!(
        status.contains(&format!(
            "  Approved:\n    {ID} (high, held by t1, {since}): Say hello\n"
        )),
        "{status}"
    );
    assert!(!status.contains("Questions waiting"), "{status}");
    let shown = atelier(&base, "lead1", &["show", ID]).stdout;
    assert!(
        shown.contains(&format!("Status: approved, held by t1, {since}\n")),
        "{shown}"
    );
    // And to its implementer, away from its branch.
    let held = atelier(&base, "t1", &["status"]).stdout;
    assert!(
        held.contains(&format!("You hold:\n  {ID} ({since}): Say hello\n")),
        "{held}"
    );

    // Code committed after the approval makes it stale: the lead sees the
    // condition its holder's `status` lists, from the branch's last commit.
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "hello.txt"]);
    git(&root, &["commit", "-qm", "Say hello again"]);
    let stale = "The approval still covers the head: since ";
    let own = atelier(&root, "t1", &["status"]).stdout;
    let line = own
        .lines()
        .find(|line| line.contains(stale))
        .unwrap_or_else(|| panic!("{own}"));
    assert!(line.contains("(changed: hello.txt)"), "{own}");
    let status = atelier(&base, "lead1", &["status"]).stdout;
    assert!(
        status.contains(&format!(
            "  Approved:\n    {ID} (high, held by t1, {since}): Say hello\n    {line}\n"
        )),
        "{status}"
    );
}

/// `atelier check` finds every record of the checkout at `root` valid.
fn checked(root: &Path) {
    let run = atelier(root, "t9", &["check"]);
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);
    assert!(
        run.stdout.starts_with("Every record is valid"),
        "{}",
        run.stdout
    );
}

/// An item whose fix-now finding a decider deferred, carried as decided
/// in round 2 and merged, with `check` passing on each record written
/// along the way; and the round-2 carrier's name.
fn deferred_and_merged() -> (Scratch, String) {
    let (scratch, finding) = changes_requested();
    let root = scratch.root();
    assert_eq!(escalate(&root, Some(&finding)).code, 0);
    checked(&root);
    assert_eq!(
        atelier(&root, "d1", &["decide", ID, "Not now.", "--defer"]).code,
        0
    );
    checked(&root);
    assert_eq!(atelier(&root, "t1", &["submit"]).code, 0);
    assert_eq!(atelier(&root, "r2", &["review", ID]).code, 0);
    checked(&root);
    assert_eq!(atelier(&root, "r2", &["verdict", "approve"]).code, 0);
    let merged = atelier(&root, "t1", &["merge"]);
    assert_eq!(merged.code, 0, "{}", merged.stderr);
    git(&root, &["switch", "-q", "main"]);
    checked(&root);
    (scratch, format!("{}.1", review_of(&root, 2).0))
}

#[test]
fn a_finding_deferred_by_a_decision_is_listed_under_its_carriers_name_and_settled_once() {
    for settle in ["new", "drop"] {
        let (scratch, carrier) = deferred_and_merged();
        let root = scratch.root();
        let name = format!("{ID}/{carrier}");
        let run = atelier(&root, "t9", &["list", "--deferred"]);
        assert_eq!(run.code, 0, "{}", run.stderr);
        assert!(
            run.stdout.starts_with(&format!(
                "Deferred findings of done items that nobody has settled:\n  {name}: No period\n"
            )),
            "{}",
            run.stdout
        );
        let args = match settle {
            "new" => vec!["new", "Add the period", "--from", &name],
            _ => vec![
                "drop",
                &name,
                "--reason",
                "Punctuation is not worth an item.",
            ],
        };
        let run = atelier(&root, "t9", &args);
        assert_eq!(run.code, 0, "{settle}: {}", run.stderr);
        checked(&root);
        let (_, path) = review_of(&root, 2);
        let line = match settle {
            "new" => "Taken: ",
            _ => "Dropped by t9: Punctuation is not worth an item.",
        };
        assert!(read(&root, &path).contains(line), "{}", read(&root, &path));
        assert_eq!(
            atelier(&root, "t9", &["list", "--deferred"]).stdout,
            "No deferred finding of a done item is left to settle.\n",
            "{settle}"
        );
        // Settled once, it is not settled again.
        let again = atelier(&root, "t9", &["drop", &name, "--reason", "Again."]);
        assert_eq!(again.code, 1, "{settle}: {}", again.stdout);
    }
}
