//! `atelier debrief` (atelier-rx22): a mission's review in Markdown and
//! HTML from one view model, with facts derived from records and Git.

use super::*;

fn ok(dir: &Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier(dir, args);
    assert!(success, "atelier {args:?} failed: {stderr}");
    stdout
}

fn item(dir: &Path, title: &str) -> String {
    ok(
        dir,
        &[
            "issue",
            "create",
            "-q",
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD",
            title,
        ],
    )
    .trim()
    .to_string()
}

const REPORT: &str = "Shipped the parser.\n\n### Deviations\nUsed a manual parser.\n\n### Decisions\nSee the decision note.\n\n### Surprises\nNone.\n\n### Open items\nNone.";

struct Mission {
    id: String,
    shipped: String,
    dropped: String,
    late: String,
    reopened: String,
    open: String,
    follow_up: String,
}

/// A mission whose history holds every kind of deviation.
fn mission(dir: &Path) -> Mission {
    init_atelier(dir);
    init_git_repo(dir);
    let id = ok(
        dir,
        &[
            "issue", "create", "-q", "--issue-type", "mission", "--body",
            "## Outcome\n\nOutput is <safe> & reviewable.\n\n## Success criteria\n\n- pipe-safe: Redirected output has no styling.\n- quiet-tokens: Quiet output prints one token.",
            "Debriefed mission",
        ],
    )
    .trim()
    .to_string();
    let (shipped, dropped, reopened, open) = (
        item(dir, "Shipped"),
        item(dir, "Dropped"),
        item(dir, "Reopened"),
        item(dir, "Still open"),
    );
    for item in [&shipped, &dropped, &reopened, &open] {
        ok(dir, &["issue", "link", "-q", item, &id]);
    }
    commit_all(dir, "plan the mission");
    ok(dir, &["issue", "transition", &id, "ready"]);
    commit_all(dir, "start the mission");
    ok(
        dir,
        &[
            "review",
            "approve",
            &id,
            "--identity",
            "Human",
            "Plan approved",
        ],
    );
    commit_all(dir, "approve the plan");
    let late = item(dir, "Added late");
    ok(dir, &["issue", "link", "-q", &late, &id]);
    commit_all(dir, "add scope after the start");

    ok(dir, &["issue", "transition", &shipped, "start"]);
    ok(
        dir,
        &[
            "issue",
            "note",
            &shipped,
            "--kind",
            "decision",
            "Parse by hand",
            "--reason",
            "No dependency",
            "--rejected",
            "a regex crate",
            "--one-way",
        ],
    );
    ok(
        dir,
        &[
            "issue",
            "note",
            &shipped,
            "--kind",
            "decision",
            "Name it plainly",
            "--reason",
            "Short",
        ],
    );
    fs::write(dir.join("report.md"), REPORT).unwrap();
    ok(
        dir,
        &[
            "issue",
            "transition",
            &shipped,
            "close",
            "--reason-file",
            dir.join("report.md").to_str().unwrap(),
        ],
    );
    fs::remove_file(dir.join("report.md")).unwrap();
    commit_all(dir, "close the shipped item");
    ok(
        dir,
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{shipped}"),
            "--kind",
            "test",
            "--result",
            "pass",
            "--criterion",
            "pipe-safe=pass",
            "Pipe-safe shown",
        ],
    );
    ok(
        dir,
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{open}"),
            "--kind",
            "test",
            "--result",
            "fail",
            "Open item's check fails",
        ],
    );
    let finding = ok(
        dir,
        &[
            "review",
            "finding",
            &shipped,
            "-q",
            "--identity",
            "Reviewer",
            "--recommendation",
            "defer",
            "Edge case left for later",
        ],
    )
    .trim()
    .to_string();
    let follow_up = item(dir, "Follow up the edge case");
    ok(
        dir,
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "defer",
            "--follow-up",
            &follow_up,
            "Deferred",
        ],
    );
    ok(
        dir,
        &[
            "review",
            "trivial",
            &dropped,
            "--identity",
            "Reviewer",
            "Typo fix only",
        ],
    );
    ok(
        dir,
        &[
            "issue",
            "transition",
            &dropped,
            "supersede",
            "--reason",
            "Not needed after the parser change.",
        ],
    );
    commit_all(dir, "review and retire");

    ok(dir, &["issue", "transition", &reopened, "start"]);
    ok(
        dir,
        &[
            "issue",
            "transition",
            &reopened,
            "close",
            "--reason",
            "First claim.",
        ],
    );
    commit_all(dir, "close too early");
    edit_canonical_issue(dir, &reopened, |markdown| {
        replace_front_matter_scalar(&markdown, "status", "in_progress")
    });
    commit_all(dir, "reopen");
    Mission {
        id,
        shipped,
        dropped,
        late,
        reopened,
        open,
        follow_up,
    }
}

/// The text of a Markdown section, up to the next blank line.
fn section(page: &str, title: &str) -> String {
    let heading = format!("{title}\n{}\n", "-".repeat(title.len()));
    page.split(&heading)
        .nth(1)
        .unwrap_or_else(|| panic!("no {title} section: {page}"))
        .split("\n\n")
        .next()
        .unwrap()
        .to_string()
}

#[test]
fn debrief_markdown_reads_as_an_after_action_report() {
    let dir = tempdir().unwrap();
    let m = mission(dir.path());
    let page = ok(dir.path(), &["debrief", &m.id]);
    assert!(
        page.starts_with(&format!(
            "Debrief: {} mission todo medium Debriefed mission\n",
            m.id
        )),
        "{page}"
    );
    assert!(
        page.contains("Status        : in progress | 1 of 2 criteria proven | 1 needs your review | 3 left to do | 1 found along the way\n"),
        "{page}"
    );
    assert!(page.contains("Started       : "), "{page}");

    // The report's order: what we set out to do, what we did, where we are.
    let order = [
        "What We Set Out To Do",
        "Planned Work",
        "What We Did",
        "Results",
        "Validation",
        "Plan vs Actual",
        "Needs Your Review",
        "Left To Do",
        "Found Along The Way",
    ]
    .map(|title| {
        page.find(&format!("\n{title}\n"))
            .unwrap_or_else(|| panic!("no {title}: {page}"))
    });
    assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{page}");
    for gone in [
        "Open Work",
        "One-way Decisions",
        "Reversible Decisions",
        "Mission Reviews",
    ] {
        assert!(!page.contains(&format!("\n{gone}\n")), "{gone}: {page}");
    }

    let intent = section(&page, "What We Set Out To Do");
    assert!(
        intent.starts_with("Output is <safe> & reviewable."),
        "{page}"
    );
    let planned = section(&page, "Planned Work");
    assert!(planned.contains(&format!("{} task ✓ done Shipped", m.shipped)));
    assert!(
        !planned.contains(&m.late) && !planned.contains(&m.follow_up),
        "work added after the start or filed by review is not planned: {page}"
    );

    let did = section(&page, "What We Did");
    let lines = did.lines().collect::<Vec<_>>();
    let at = |needle: &str| {
        lines
            .iter()
            .position(|line| line.contains(needle))
            .unwrap_or_else(|| panic!("no `{needle}`: {page}"))
    };
    assert!(at("started the mission") < at(&format!("delivered {}", m.shipped)));
    assert!(did.contains(&format!("added {} task todo Added late", m.late)));
    assert!(did.contains("by Human at") && did.contains("| Plan approved"));
    assert!(did.contains(&format!(
        "delivered {} task ✓ done Shipped | pipe-safe\n  Outcome: Shipped the parser.\n",
        m.shipped
    )));
    for fact in [
        "  Changed: Used a manual parser.",
        "  Decided: one-way: Parse by hand | because No dependency",
        "  Decided: Name it plainly | because Short",
        "  Review: Not approved; 1 finding: 1 deferred.",
    ] {
        assert!(did.contains(fact), "missing `{fact}`: {page}");
    }
    assert!(did.contains(&format!("  Filed: {} task", m.follow_up)));
    assert!(
        at("one-way: Parse by hand") < at("Name it plainly"),
        "one-way first: {page}"
    );
    assert!(
        !did.contains("Surprised by"),
        "a part that says None is left out: {page}"
    );
    assert!(did.contains("validated by unnamed producer at") && did.contains("| pipe-safe pass"));
    assert!(did.contains(&format!(
        "dropped {} task ✓ superseded Dropped | Not needed after the parser change.",
        m.dropped
    )));

    let results = section(&page, "Results");
    assert!(
        results.contains(&format!("pipe-safe ✓ proven | by {} | atelier-", m.shipped)),
        "{page}"
    );
    assert!(results.contains("quiet-tokens unproven"), "{page}");

    let validation = section(&page, "Validation");
    for line in [
        "Item review: 0 of 1 approved | 1 finding: 1 deferred",
        &format!("Not approved: {}", m.shipped),
        "| 1 receipt",
        "  pipe-safe pass atelier-",
        "Awaiting: quiet-tokens unproven",
    ] {
        assert!(validation.contains(line), "missing `{line}`: {page}");
    }

    let plan = section(&page, "Plan vs Actual");
    for line in [
        "Planned: 1 of 3 items done".to_string(),
        format!("Changed course: {}", m.shipped),
        format!("Added: {} task todo Added late", m.late),
        format!("Dropped: {} task ✓ superseded Dropped | Not needed after the parser change. | Review waived as trivial: Typo fix only", m.dropped),
        "Found: 1 item still open".to_string(),
    ] {
        assert!(plan.contains(&line), "missing `{line}`: {page}");
    }

    let asks = section(&page, "Needs Your Review");
    assert!(
        asks.contains("⚠ 1 criterion without passing proof: quiet-tokens"),
        "{page}"
    );
    let left = section(&page, "Left To Do");
    for expected in [
        format!("{} task in_progress Reopened | Reopened on", m.reopened),
        format!("{} task todo Still open | Newest proof atelier-", m.open),
        "is fail: Open item's check fails".to_string(),
        format!("{} task todo Added late", m.late),
    ] {
        assert!(left.contains(&expected), "missing `{expected}`: {page}");
    }
    assert_eq!(left.lines().count(), 3, "{page}");
    let found = section(&page, "Found Along The Way");
    assert!(
        found.starts_with(&format!(
            "{} task todo Follow up the edge case | from review of {} | filed ",
            m.follow_up, m.shipped
        )),
        "{page}"
    );
    let quiet = ok(dir.path(), &["debrief", &m.id, "-q"]);
    assert_eq!(
        quiet.trim(),
        "criteria=2 proven=1 failing=0 unproven=1 items=6 decisions=2 deviations=5 follow_ups=1"
    );
}

#[test]
fn debrief_html_is_a_self_contained_page_with_the_same_facts() {
    let dir = tempdir().unwrap();
    let m = mission(dir.path());
    let markdown = ok(dir.path(), &["debrief", &m.id]);
    let before = git_status(dir.path());
    let printed = ok(dir.path(), &["debrief", &m.id, "--html", "-q"]);
    let path = PathBuf::from(printed.trim());
    assert_eq!(
        path,
        dir.path()
            .join(format!(".atelier/runtime/reviews/{}-debrief.html", m.id))
    );
    assert_eq!(
        git_status(dir.path()),
        before,
        "debrief writes only the ignored page"
    );
    let html = fs::read_to_string(&path).unwrap();
    assert!(html.starts_with("<!doctype html>"), "{html}");
    for forbidden in [
        "<script", "<form", "<button", "http://", "https://", "<link", "@import", "src=",
    ] {
        assert!(!html.contains(forbidden), "page holds {forbidden}");
    }
    assert!(html.contains("prefers-color-scheme: dark"));
    assert!(
        html.contains("Output is &lt;safe&gt; &amp; reviewable."),
        "record text is escaped"
    );
    assert!(!html.contains("<safe>"));

    // The page's sections in the report's order.
    let order = [
        "id=\"intent\"",
        "id=\"did\"",
        "id=\"results\"",
        "id=\"validation\"",
        "id=\"plan\"",
        "id=\"review\"",
        "id=\"left\"",
        "id=\"found\"",
    ]
    .map(|anchor| html.find(anchor).unwrap_or_else(|| panic!("no {anchor}")));
    assert!(order.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(!html.contains("id=\"decisions\""));

    // Every item the Markdown names is on the page as a chip with its ID,
    // title and status, and a card behind it.
    let items = markdown
        .lines()
        .flat_map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            words
                .windows(2)
                .filter(|pair| pair[0].starts_with("atelier-") && pair[1] == "task")
                .map(|pair| pair[0].to_string())
                .collect::<Vec<_>>()
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(items.len(), 6, "{items:?}");
    for id in &items {
        assert!(
            html.contains(&format!("<span class=\"k\">{id}</span><span class=\"t\">")),
            "page lacks a chip for {id}"
        );
    }
    assert!(
        html.contains(&format!(
            "<span class=\"k\">{}</span><span class=\"t\">Shipped</span><span class=\"st done\">done</span><span class=\"card\"><b>Shipped</b><span class=\"m\">Task · done · {}</span>",
            m.shipped, m.shipped
        )),
        "{html}"
    );
    assert!(
        html.contains("<i>Outcome</i> Shipped the parser."),
        "the card says how it ended"
    );

    // Each criterion's mark, read from the Markdown's results.
    let criteria = section(&markdown, "Results")
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let slug = words.next().unwrap().to_string();
            let mark = words
                .find(|word| ["proven", "failing", "unproven"].contains(word))
                .unwrap()
                .to_string();
            (slug, mark)
        })
        .collect::<Vec<_>>();
    assert_eq!(criteria.len(), 2, "{markdown}");
    for (slug, mark) in criteria {
        let row = html
            .split(&format!(">{slug}</span><span class=\"status"))
            .nth(1)
            .expect(&slug);
        let word = row.split("</span>").next().unwrap().to_lowercase();
        assert!(
            word.ends_with(&mark),
            "{slug} is {mark} in the Markdown: {row}"
        );
    }
    // Each decision, with its reason, where it was made, one-way first; the
    // sidebar indexes the one-way ones.
    let notes = markdown
        .lines()
        .filter_map(|line| line.trim().strip_prefix("Decided: "))
        .map(|line| {
            let (choice, reason) = line.split_once(" | because ").unwrap();
            (
                choice.trim_start_matches("one-way: ").to_string(),
                reason.to_string(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(notes.len(), 2, "{markdown}");
    let entry = &html[html.find(&format!("id=\"{}\"", m.shipped)).expect(&html)..];
    for (choice, reason) in &notes {
        assert!(
            entry.contains(choice.as_str()) && entry.contains(reason.as_str()),
            "{choice}"
        );
    }
    let one_way = entry.find("one-way</span>Parse by hand").expect(entry);
    let reversible = entry.find("Name it plainly").expect(entry);
    assert!(one_way < reversible, "one-way first: {entry}");
    assert!(html.contains(&format!(
        "<a class=\"dec-link\" href=\"#{}\" title=\"Parse by hand\">",
        m.shipped
    )));
    for part in [
        "Shipped the parser.",
        "Used a manual parser.",
        "Not needed after the parser change.",
        "Follow up the edge case",
        "Typo fix only",
        "Plan approved",
        "Pipe-safe shown",
    ] {
        assert!(markdown.contains(part) && html.contains(part), "{part}");
    }
}

#[test]
fn debrief_refuses_items_that_are_not_missions() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let work = item(dir.path(), "Work");
    let (success, _, stderr) = run_atelier(dir.path(), &["debrief", &work]);
    assert!(
        !success && stderr.contains("not a mission; debrief reviews a mission"),
        "{stderr}"
    );
}

fn git_status(dir: &Path) -> String {
    String::from_utf8(
        Command::new("git")
            .current_dir(dir)
            .args(["status", "--porcelain"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
}

fn git(dir: &Path, args: &[&str]) {
    let status = Command::new("git")
        .current_dir(dir)
        .args(args)
        .status()
        .unwrap();
    assert!(status.success(), "git {args:?}");
}

/// A started mission with one planned item, committed.
fn started(dir: &Path) -> (String, String) {
    init_atelier(dir);
    init_git_repo(dir);
    let mission = ok(
        dir,
        &[
            "issue",
            "create",
            "-q",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nO\n\n## Success criteria\n\n- fast: Fast.",
            "Mission",
        ],
    )
    .trim()
    .to_string();
    let planned = item(dir, "Planned");
    ok(dir, &["issue", "link", "-q", &planned, &mission]);
    commit_all(dir, "plan");
    ok(dir, &["issue", "transition", &mission, "ready"]);
    commit_all(dir, "start");
    (mission, planned)
}

/// The sections where departures from the plan show.
fn deviations(page: &str) -> String {
    [
        "What We Did",
        "Plan vs Actual",
        "Left To Do",
        "Found Along The Way",
    ]
    .iter()
    .filter(|title| page.contains(&format!("\n{title}\n")))
    .map(|title| section(page, title))
    .collect::<Vec<_>>()
    .join("\n")
}

#[test]
fn a_child_added_to_planned_scope_after_the_start_is_new_scope() {
    let dir = tempdir().unwrap();
    let (mission, planned) = started(dir.path());
    let child = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--parent",
            &planned,
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD",
            "Child",
        ],
    )
    .trim()
    .to_string();
    commit_all(dir.path(), "add a child");
    let page = ok(dir.path(), &["debrief", &mission]);
    let found = deviations(&page);
    assert!(
        found.contains(&format!("{child} task todo Child | added after the start")),
        "{page}"
    );
    assert!(
        !found.contains(&format!("{planned} task todo Planned | added after")),
        "{page}"
    );
}

#[test]
fn a_link_stored_on_the_mission_dates_from_the_mission_record() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let mission = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nO\n\n## Success criteria\n\n- fast: Fast.",
            "Mission",
        ],
    )
    .trim()
    .to_string();
    let planned = item(dir.path(), "Planned");
    // An older record stored the link on the mission's side.
    edit_canonical_issue(dir.path(), &mission, |markdown| {
        markdown.replacen(
            "  relates: []",
            &format!(
                "  relates:\n  - kind: \"issue\"\n    id: \"{planned}\"\n    type: \"advances\""
            ),
            1,
        )
    });
    commit_all(dir.path(), "plan with the link on the mission");
    ok(dir.path(), &["issue", "transition", &mission, "ready"]);
    commit_all(dir.path(), "start");
    // Later the link moves to the item, as the link storage change did.
    edit_canonical_issue(dir.path(), &mission, |markdown| {
        let start = markdown.find("  relates:\n").unwrap();
        let end = markdown[start..].find("schema:").unwrap() + start;
        format!("{}  relates: []\n{}", &markdown[..start], &markdown[end..])
    });
    ok(dir.path(), &["issue", "link", "-q", &planned, &mission]);
    commit_all(dir.path(), "move the link to the item");
    let page = ok(dir.path(), &["debrief", &mission]);
    assert!(
        !deviations(&page).contains("added after the start"),
        "{page}"
    );
}

#[test]
fn work_merged_from_a_branch_is_not_reported_reopened() {
    let dir = tempdir().unwrap();
    let (mission, planned) = started(dir.path());
    let main = String::from_utf8(
        Command::new("git")
            .current_dir(dir.path())
            .args(["branch", "--show-current"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_string();
    git(dir.path(), &["switch", "-q", "-c", "feature"]);
    ok(dir.path(), &["issue", "transition", &planned, "start"]);
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &planned,
            "close",
            "--reason",
            "Done on the branch.",
        ],
    );
    commit_all(dir.path(), "close on the branch");
    git(dir.path(), &["switch", "-q", &main]);
    std::thread::sleep(std::time::Duration::from_millis(1100));
    ok(
        dir.path(),
        &["issue", "update", &planned, "--label", "later"],
    );
    commit_all(dir.path(), "edit the item on main later");
    git(
        dir.path(),
        &[
            "merge",
            "-q",
            "--no-ff",
            "--no-edit",
            "-X",
            "theirs",
            "feature",
        ],
    );
    let page = ok(dir.path(), &["debrief", &mission]);
    assert!(page.contains(&format!("{planned} task ✓ done")), "{page}");
    assert!(!deviations(&page).contains("Reopened on"), "{page}");
}

#[test]
fn proof_before_later_code_is_aged_up_to_the_close() {
    let dir = tempdir().unwrap();
    let (mission, planned) = started(dir.path());
    ok(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{planned}"),
            "--kind",
            "test",
            "--result",
            "pass",
            "--criterion",
            "fast=pass",
            "Fast shown",
        ],
    );
    commit_all(dir.path(), "proof");
    fs::write(dir.path().join("code.txt"), "later\n").unwrap();
    commit_all(dir.path(), "code after the proof");
    let open = ok(dir.path(), &["debrief", &mission]);
    assert!(open.contains("| 1 code commit since"), "{open}");
    assert!(!open.contains("before close"), "{open}");
    ok(dir.path(), &["issue", "transition", &planned, "start"]);
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &planned,
            "close",
            "--reason",
            "Done.",
        ],
    );
    edit_canonical_issue(dir.path(), &mission, |markdown| {
        replace_front_matter_scalar(&markdown, "status", "done")
    });
    commit_all(dir.path(), "close the mission");
    fs::write(dir.path().join("code.txt"), "after the close\n").unwrap();
    commit_all(dir.path(), "code after the close");
    let closed = ok(dir.path(), &["debrief", &mission]);
    assert!(
        closed.contains("| 1 code commit before close"),
        "code after the close is not counted: {closed}"
    );
    assert!(!closed.contains("code commits before close"), "{closed}");
}

#[test]
fn needs_your_review_lists_human_work_and_criteria_no_ask_already_names() {
    let dir = tempdir().unwrap();
    let (mission, _) = started(dir.path());
    let human = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--label",
            "needs-human",
            "--body",
            "## Goal\n\nShow `fast` to the human.\n\n## Done when\n\nD",
            "Human check",
        ],
    )
    .trim()
    .to_string();
    ok(dir.path(), &["issue", "link", "-q", &human, &mission]);
    let decision = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--issue-type",
            "decision",
            "--body",
            "## Question\n\nWhich store?",
            "Choose the store",
        ],
    )
    .trim()
    .to_string();
    ok(dir.path(), &["issue", "link", "-q", &decision, &mission]);
    commit_all(dir.path(), "asks");
    let page = ok(dir.path(), &["debrief", &mission]);
    let asks = page
        .split("Needs Your Review\n-----------------\n")
        .nth(1)
        .unwrap()
        .split("\n\n")
        .next()
        .unwrap();
    assert!(
        asks.contains(&format!("⚠ {decision} Decide: Choose the store")),
        "{page}"
    );
    assert!(asks.contains(&format!("⚠ {human} Human check")), "{page}");
    assert!(
        !asks.contains("without passing proof"),
        "the human check already names fast: {page}"
    );
}

#[test]
fn a_plain_word_does_not_name_a_one_word_criterion() {
    let dir = tempdir().unwrap();
    let (mission, _) = started(dir.path());
    let human = item(dir.path(), "Human check");
    edit_canonical_issue(dir.path(), &human, |markdown| {
        markdown.replace("labels: []", "labels:\n- \"needs-human\"")
    });
    edit_canonical_issue(dir.path(), &human, |markdown| {
        markdown.replace(
            "## Goal\n\nG",
            "## Goal\n\nMake startup fast for the human.",
        )
    });
    ok(dir.path(), &["issue", "link", "-q", &human, &mission]);
    commit_all(dir.path(), "a plain word");
    let page = ok(dir.path(), &["debrief", &mission]);
    assert!(page.contains("without passing proof: fast"), "{page}");
}

#[test]
fn children_of_open_or_dropped_parents_keep_their_place() {
    let dir = tempdir().unwrap();
    let (mission, planned) = started(dir.path());
    let child = |title: &str| {
        ok(
            dir.path(),
            &[
                "issue",
                "create",
                "-q",
                "--parent",
                &planned,
                "--body",
                "## Goal\n\nG\n\n## Done when\n\nD",
                title,
            ],
        )
        .trim()
        .to_string()
    };
    let (shipped, dropped, waiting) = (
        child("Shipped child"),
        child("Dropped child"),
        child("Waiting child"),
    );
    ok(dir.path(), &["issue", "transition", &shipped, "start"]);
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &shipped,
            "close",
            "--reason",
            "Child shipped.",
        ],
    );
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &dropped,
            "supersede",
            "--reason",
            "Not needed.",
        ],
    );
    commit_all(dir.path(), "children");
    let page = ok(dir.path(), &["debrief", &mission]);
    let did = section(&page, "What We Did");
    assert!(
        did.contains(&format!(
            "delivered {shipped} task ✓ done Shipped child | under {planned}"
        )),
        "{page}"
    );
    assert!(did.contains("  Outcome: Child shipped."), "{page}");
    let left = section(&page, "Left To Do");
    assert!(
        left.contains(&format!("  {waiting} task todo Waiting child")),
        "{page}"
    );
    assert!(
        section(&page, "Plan vs Actual").contains(&format!(
            "Dropped: {dropped} task ✓ superseded Dropped child | Not needed."
        )),
        "{page}"
    );
    let path = PathBuf::from(ok(dir.path(), &["debrief", &mission, "--html", "-q"]).trim());
    let html = fs::read_to_string(path).unwrap();
    assert!(
        html.contains(&format!("id=\"{shipped}\""))
            && html.contains(&format!(
                "part of <span class=\"chip\" tabindex=\"0\"><span class=\"dot todo\"></span><span class=\"k\">{planned}</span><span class=\"t\">Planned</span>"
            )),
        "{html}"
    );
}

#[test]
fn planned_work_counts_only_what_was_linked_by_the_start() {
    let dir = tempdir().unwrap();
    let (mission, _) = started(dir.path());
    let late = item(dir.path(), "Late");
    ok(dir.path(), &["issue", "link", "-q", &late, &mission]);
    commit_all(dir.path(), "late scope");
    let path = PathBuf::from(ok(dir.path(), &["debrief", &mission, "--html", "-q"]).trim());
    let html = fs::read_to_string(path).unwrap();
    assert!(
        html.contains("<span>Planned work done</span><span>0 of 1</span>"),
        "{html}"
    );
}

#[test]
fn follow_ups_come_from_a_finding_latest_disposition_and_from_the_mission() {
    let dir = tempdir().unwrap();
    let (mission, planned) = started(dir.path());
    ok(dir.path(), &["issue", "transition", &planned, "start"]);
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &planned,
            "close",
            "--reason",
            "Done.",
        ],
    );
    commit_all(dir.path(), "close");
    let finding = ok(
        dir.path(),
        &[
            "review",
            "finding",
            &planned,
            "-q",
            "--identity",
            "Reviewer",
            "--recommendation",
            "defer",
            "Later",
        ],
    )
    .trim()
    .to_string();
    let abandoned = item(dir.path(), "Abandoned follow-up");
    ok(
        dir.path(),
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "defer",
            "--follow-up",
            &abandoned,
            "Deferred",
        ],
    );
    commit_all(dir.path(), "defer");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let head = String::from_utf8(
        Command::new("git")
            .current_dir(dir.path())
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    ok(
        dir.path(),
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Author",
            "--decision",
            "fix_now",
            "--resolution-commit",
            head.trim(),
            "Fixed after all",
        ],
    );
    let mission_finding = ok(
        dir.path(),
        &[
            "review",
            "finding",
            &mission,
            "-q",
            "--identity",
            "Reviewer",
            "--recommendation",
            "defer",
            "Plan gap",
        ],
    )
    .trim()
    .to_string();
    let from_mission = item(dir.path(), "Close the plan gap");
    ok(
        dir.path(),
        &[
            "review",
            "disposition",
            &mission_finding,
            "--identity",
            "Reviewer",
            "--decision",
            "defer",
            "--follow-up",
            &from_mission,
            "Deferred",
        ],
    );
    commit_all(dir.path(), "dispositions");
    let page = ok(dir.path(), &["debrief", &mission]);
    assert!(
        !page.contains(&abandoned),
        "a finding later fixed leaves no follow-up: {page}"
    );
    assert!(
        page.contains("Review: Not approved; 1 finding: 1 fixed."),
        "{page}"
    );
    assert!(
        page.contains(&format!(
            "{from_mission} task todo Close the plan gap | from review of the mission"
        )),
        "{page}"
    );
}

#[test]
fn a_failing_receipt_credits_no_work_with_a_criterion() {
    let dir = tempdir().unwrap();
    let (mission, planned) = started(dir.path());
    ok(dir.path(), &["issue", "transition", &planned, "start"]);
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &planned,
            "close",
            "--reason",
            "Done.",
        ],
    );
    ok(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{planned}"),
            "--kind",
            "test",
            "--result",
            "fail",
            "--criterion",
            "fast=fail",
            "Too slow",
        ],
    );
    commit_all(dir.path(), "failing proof");
    let page = ok(dir.path(), &["debrief", &mission]);
    let results = section(&page, "Results");
    let row = results
        .lines()
        .find(|line| line.starts_with("fast "))
        .unwrap();
    assert!(
        row.starts_with("fast ⚠ failing") && !row.contains("| by "),
        "{row}"
    );
}

#[test]
fn record_mentions_are_chips_with_their_id_and_what_they_say() {
    let dir = tempdir().unwrap();
    let (mission, planned) = started(dir.path());
    let finding = ok(
        dir.path(),
        &[
            "review",
            "finding",
            &planned,
            "-q",
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Children of open parents drop out of the page entirely",
        ],
    )
    .trim()
    .to_string();
    ok(dir.path(), &["issue", "transition", &planned, "start"]);
    fs::write(
        dir.path().join("r.md"),
        format!("Done.\n\n### Deviations\nReview {finding} changed the walk."),
    )
    .unwrap();
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &planned,
            "close",
            "--reason-file",
            dir.path().join("r.md").to_str().unwrap(),
        ],
    );
    fs::remove_file(dir.path().join("r.md")).unwrap();
    commit_all(dir.path(), "close");
    let markdown = ok(dir.path(), &["debrief", &mission]);
    assert!(
        markdown.contains(&format!("Review {finding} changed the walk.")),
        "{markdown}"
    );
    let path = PathBuf::from(ok(dir.path(), &["debrief", &mission, "--html", "-q"]).trim());
    let html = fs::read_to_string(path).unwrap();
    assert!(
        html.contains(&format!(
            "Review <span class=\"chip rec\" tabindex=\"0\"><span class=\"k\">{finding}</span><span class=\"t\">finding</span><span class=\"card\"><b>Finding</b><span class=\"s\">Children of open parents drop out of the page entirely</span>"
        )),
        "{html}"
    );
}

#[test]
fn validation_groups_receipts_into_runs_and_names_work_delivered_after_one() {
    let dir = tempdir().unwrap();
    let (mission, planned) = started(dir.path());
    // The raw runner records both against one commit, as a validator does;
    // the usual runner commits before each command.
    for summary in ["First check", "Second check"] {
        let (success, _, stderr) = run_atelier_raw(
            dir.path(),
            &[
                "evidence",
                "record",
                "--target",
                &format!("issue/{mission}"),
                "--kind",
                "validation",
                "--result",
                "pass",
                "--producer",
                "Validator",
                "--criterion",
                "fast=pass",
                summary,
            ],
        );
        assert!(success, "{stderr}");
    }
    commit_all(dir.path(), "validate");
    fs::write(dir.path().join("code.rs"), "fn main() {}\n").unwrap();
    ok(dir.path(), &["issue", "transition", &planned, "start"]);
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &planned,
            "close",
            "--reason",
            "Done after the check.",
        ],
    );
    commit_all(dir.path(), "code after validation");
    let page = ok(dir.path(), &["debrief", &mission]);
    let did = section(&page, "What We Did");
    assert_eq!(
        did.matches("validated by Validator at").count(),
        1,
        "one producer at one commit is one run: {page}"
    );
    assert!(did.contains("| fast pass, fast pass"), "{page}");
    let validation = section(&page, "Validation");
    assert!(
        validation.contains("Validator at")
            && validation.contains("| 2 receipts | 1 code commit since"),
        "{page}"
    );
    for line in [
        "  fast pass atelier-",
        "| First check",
        "| Second check",
        &format!("  Delivered since: {planned}"),
    ] {
        assert!(validation.contains(line), "missing `{line}`: {page}");
    }
    let path = PathBuf::from(ok(dir.path(), &["debrief", &mission, "--html", "-q"]).trim());
    let html = fs::read_to_string(path).unwrap();
    let section_html = &html[html.find("id=\"validation\"").expect(&html)..];
    assert_eq!(section_html.matches("<details class=\"rcpt\">").count(), 2);
    assert!(
        section_html.contains("Delivered after this validation: <span class=\"chip\""),
        "{section_html}"
    );
}

#[test]
fn added_work_parked_in_backlog_is_found_and_taken_on_work_is_left() {
    let dir = tempdir().unwrap();
    let (mission, _) = started(dir.path());
    let parked = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--status",
            "backlog",
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD",
            "Parked",
        ],
    )
    .trim()
    .to_string();
    let taken = item(dir.path(), "Taken on");
    for id in [&parked, &taken] {
        ok(dir.path(), &["issue", "link", "-q", id, &mission]);
    }
    commit_all(dir.path(), "more work");
    let page = ok(dir.path(), &["debrief", &mission]);
    let found = section(&page, "Found Along The Way");
    assert!(
        found.starts_with(&format!("{parked} task backlog Parked | filed ")),
        "{page}"
    );
    assert!(!found.contains(&taken), "{page}");
    let left = section(&page, "Left To Do");
    assert!(
        left.contains(&format!(
            "{taken} task todo Taken on | added after the start"
        )),
        "{page}"
    );
    let did = section(&page, "What We Did");
    assert!(
        did.contains(&format!("filed {parked} task backlog Parked")),
        "{page}"
    );
    assert!(
        did.contains(&format!("added {taken} task todo Taken on")),
        "{page}"
    );
    assert!(
        section(&page, "Plan vs Actual").contains(&format!("Added: {taken} task todo Taken on")),
        "{page}"
    );
}
