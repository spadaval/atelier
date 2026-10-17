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

#[test]
fn debrief_markdown_shows_criteria_reports_decisions_deviations_and_follow_ups() {
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
        page.contains("Outcome       : Output is <safe> & reviewable."),
        "{page}"
    );
    assert!(page.contains("Started       : "), "{page}");
    assert!(
        page.contains("pipe-safe ✓ proven Redirected output has no styling. | atelier-"),
        "{page}"
    );
    assert!(
        page.contains("quiet-tokens unproven Quiet output prints one token."),
        "{page}"
    );
    let items = page.split("Items\n-----\n").nth(1).expect(&page);
    assert!(items.contains(&format!("{} task ✓ done medium Shipped\n  Outcome: Shipped the parser.\n  Deviations: Used a manual parser.\n", m.shipped)), "{page}");
    assert!(items.contains("  Open items: None."), "{page}");
    let one_way = page
        .find("One-way Decisions\n-----------------\n")
        .expect(&page);
    let reversible = page
        .find("Reversible Decisions\n--------------------\n")
        .expect(&page);
    assert!(one_way < reversible, "one-way first: {page}");
    assert!(
        page[one_way..reversible].contains("Parse by hand | because No dependency"),
        "{page}"
    );
    assert!(
        page[reversible..].contains("Name it plainly | because Short"),
        "{page}"
    );
    let deviations = page.split("Deviations\n----------\n").nth(1).expect(&page);
    for expected in [
        format!("scope added {} after the start", m.late),
        format!(
            "scope dropped {} | Not needed after the parser change.",
            m.dropped
        ),
        format!(
            "on {} to {} | Edge case left for later",
            m.shipped, m.follow_up
        ),
        format!("on {} fail | Open item's check fails", m.open),
        format!("on {} | Typo fix only", m.dropped),
        format!("reopened {}", m.reopened),
    ] {
        assert!(
            deviations.contains(&expected),
            "missing `{expected}`: {page}"
        );
    }
    assert!(
        !deviations.contains(&format!("scope added {} ", m.shipped)),
        "planned scope is not a deviation: {page}"
    );
    let reviews = page
        .split("Mission Reviews\n---------------\n")
        .nth(1)
        .expect(&page);
    assert!(
        reviews.starts_with("approval atelier-")
            && reviews.contains("by Human at")
            && reviews.contains("| Plan approved"),
        "{page}"
    );
    let follow_ups = page.split("Follow-ups\n----------\n").nth(1).expect(&page);
    for id in [&m.open, &m.reopened, &m.late, &m.follow_up] {
        assert!(follow_ups.contains(id.as_str()), "follow-up {id}: {page}");
    }
    assert!(!follow_ups.contains(&format!("{} ", m.shipped)), "{page}");
    let quiet = ok(dir.path(), &["debrief", &m.id, "-q"]);
    assert_eq!(
        quiet.trim(),
        "criteria=2 proven=1 failing=0 unproven=1 items=5 decisions=2 deviations=6 follow_ups=4"
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

    // Every item, criterion mark and decision in the Markdown is on the page.
    let ids = markdown
        .split(|c: char| c.is_whitespace() || c == '|' || c == ',')
        .filter(|word| word.starts_with("atelier-") && word.len() == "atelier-xxxx".len())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(ids.len() >= 8, "{ids:?}");
    for id in ids {
        assert!(html.contains(id), "page lacks {id}");
    }
    // Each criterion row's slug and mark, read from the Markdown.
    let criteria = markdown
        .split("Success Criteria\n----------------\n")
        .nth(1)
        .expect(&markdown)
        .lines()
        .take_while(|line| !line.is_empty())
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
            .split(&format!("<td class=\"id\">{slug}</td>"))
            .nth(1)
            .expect(&slug);
        let badge = row.split("</span>").next().unwrap();
        assert!(
            badge.ends_with(&format!(">{mark}")),
            "{slug} is {mark} in the Markdown: {row}"
        );
    }
    // Each decision note's choice and reason, read from the Markdown.
    let notes = markdown
        .lines()
        .filter_map(|line| line.split_once(" | because "))
        .map(|(head, reason)| {
            let choice = head.splitn(3, ' ').nth(2).unwrap().to_string();
            let reason = reason.split(" | ").next().unwrap().to_string();
            (choice, reason)
        })
        .collect::<Vec<_>>();
    assert_eq!(notes.len(), 2, "{markdown}");
    for (choice, reason) in notes {
        assert!(html.contains(&choice) && html.contains(&reason), "{choice}");
    }
    let one_way = html.find(">One-way Decisions</th>").expect(&html);
    let reversible = html.find(">Reversible Decisions</th>").expect(&html);
    assert!(one_way < reversible && html[one_way..reversible].contains("Parse by hand"));
    for part in [
        "Shipped the parser.",
        "Used a manual parser.",
        "Not needed after the parser change.",
        "Edge case left for later",
        "Typo fix only",
        "Plan approved",
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

fn deviations(page: &str) -> &str {
    page.split("Deviations\n----------\n")
        .nth(1)
        .unwrap()
        .split("\n\n")
        .next()
        .unwrap()
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
        found.contains(&format!("scope added {child} after the start")),
        "{page}"
    );
    assert!(
        !found.contains(&format!("scope added {planned} ")),
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
    assert!(!deviations(&page).contains("scope added"), "{page}");
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
    assert!(!deviations(&page).contains("reopened"), "{page}");
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
    assert!(
        deviations(&open).contains("aged proof fast by atelier-") && !open.contains("before close"),
        "{open}"
    );
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
    assert!(
        deviations(&closed).contains("| 1 code commit after it before close"),
        "{closed}"
    );
}
