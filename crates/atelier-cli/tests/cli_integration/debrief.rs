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
    std::thread::sleep(std::time::Duration::from_millis(1100));
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
    let decisions = page.split("Decisions\n---------\n").nth(1).expect(&page);
    let one_way = decisions.find("Parse by hand").expect(&page);
    let reversible = decisions.find("Name it plainly").expect(&page);
    assert!(one_way < reversible, "one-way first: {page}");
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
    for (slug, mark) in [("pipe-safe", "proven"), ("quiet-tokens", "unproven")] {
        let row = html
            .split(&format!("<td class=\"id\">{slug}</td>"))
            .nth(1)
            .expect(slug);
        assert!(
            row.starts_with(&format!("<td><span class=\"badge good\">{mark}"))
                || row.starts_with(&format!("<td><span class=\"badge warn\">{mark}")),
            "{slug}: {row}"
        );
    }
    for choice in [
        "Parse by hand",
        "Name it plainly",
        "No dependency",
        "a regex crate",
    ] {
        assert!(
            markdown.contains(choice) && html.contains(choice),
            "{choice}"
        );
    }
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
