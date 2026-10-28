//! `atelier brief` (atelier-acrt): a mission's plan before work starts, in
//! Markdown and HTML from one view model.

use super::*;

fn ok(dir: &Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier(dir, args);
    assert!(success, "atelier {args:?} failed: {stderr}");
    stdout
}

fn create(dir: &Path, args: &[&str]) -> String {
    let mut full = vec!["issue", "create", "-q"];
    full.extend_from_slice(args);
    ok(dir, &full).trim().to_string()
}

struct Plan {
    mission: String,
    sized: String,
    blocked: String,
    decision: String,
    human: String,
}

fn plan(dir: &Path) -> Plan {
    init_atelier(dir);
    init_git_repo(dir);
    fs::create_dir_all(dir.join("docs/strategies")).unwrap();
    fs::write(
        dir.join("docs/strategies/plan.md"),
        "# Outcome\n\nPlans are <reviewable>.\n\n# Success Criteria\n\n- plan-shown: The brief shows the plan.\n\n# Risks and Unknowns\n\n- The parser may be slow on huge strategies.\n",
    )
    .unwrap();
    let mission = create(
        dir,
        &[
            "--issue-type",
            "mission",
            "--strategy",
            "docs/strategies/plan.md",
            "--body",
            "## Outcome\n\nPlans are <reviewable>.",
            "Planned mission",
        ],
    );
    let sized = create(
        dir,
        &[
            "--label",
            "size-m",
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD\n\n## Unknowns\n\nWhether the cache is needed.",
            "Sized work",
        ],
    );
    let blocked = create(
        dir,
        &[
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD",
            "Blocked work",
        ],
    );
    let decision = create(
        dir,
        &[
            "--issue-type",
            "decision",
            "--body",
            "## Question\n\nWhich store?",
            "Choose the store",
        ],
    );
    let human = create(
        dir,
        &[
            "--label",
            "needs-human",
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD",
            "Approve the account",
        ],
    );
    for id in [&sized, &blocked, &decision, &human] {
        ok(dir, &["issue", "link", "-q", id, &mission]);
    }
    ok(
        dir,
        &[
            "issue",
            "link",
            "-q",
            &blocked,
            &decision,
            "--role",
            "blocked_by",
        ],
    );
    commit_all(dir, "plan");
    Plan {
        mission,
        sized,
        blocked,
        decision,
        human,
    }
}

#[test]
fn brief_markdown_shows_the_plan_and_its_approval() {
    let dir = tempdir().unwrap();
    let p = plan(dir.path());
    let page = ok(dir.path(), &["brief", &p.mission]);
    assert!(
        page.starts_with(&format!(
            "Brief: {} mission draft medium Planned mission\n",
            p.mission
        )),
        "{page}"
    );
    assert!(
        page.contains("Strategy      : docs/strategies/plan.md"),
        "{page}"
    );
    assert!(
        page.contains("plan-shown The brief shows the plan."),
        "{page}"
    );
    assert!(
        page.contains(&format!("{} task todo medium Sized work | size M", p.sized)),
        "{page}"
    );
    assert!(
        page.contains(&format!(
            "{} task todo medium Blocked work | blockers: {}",
            p.blocked, p.decision
        )),
        "{page}"
    );
    assert!(
        page.contains(&format!(
            "Blockers\n--------\n{} blocked by {}",
            p.blocked, p.decision
        )),
        "{page}"
    );
    assert!(
        page.contains(&format!(
            "Unknowns\n--------\n{}: Whether the cache is needed.",
            p.sized
        )),
        "{page}"
    );
    let human = page
        .split("Open Human Decisions\n--------------------\n")
        .nth(1)
        .expect(&page);
    assert!(
        human.contains(&format!(
            "{} decision todo medium Choose the store",
            p.decision
        )),
        "{page}"
    );
    assert!(
        human.contains(&format!("{} task todo medium Approve the account", p.human)),
        "{page}"
    );
    assert!(
        page.contains("Risks\n-----\n- The parser may be slow on huge strategies."),
        "{page}"
    );
    assert!(page.contains("No plan approval recorded."), "{page}");
    assert_eq!(
        ok(dir.path(), &["brief", &p.mission, "-q"]).trim(),
        "criteria=1 items=4 blocked=1 unknowns=1 human_decisions=2 approvals=0"
    );

    ok(
        dir.path(),
        &[
            "review",
            "approve",
            &p.mission,
            "--identity",
            "Human",
            "Plan approved as written",
        ],
    );
    let page = ok(dir.path(), &["brief", &p.mission]);
    assert!(!page.contains("No plan approval recorded."), "{page}");
    assert!(
        page.contains("by Human at") && page.contains("| Plan approved as written"),
        "{page}"
    );
    // The debrief shows the same approval.
    let debrief = ok(dir.path(), &["debrief", &p.mission]);
    assert!(debrief.contains("| Plan approved as written"), "{debrief}");
}

#[test]
fn brief_html_holds_the_same_facts_as_the_markdown() {
    let dir = tempdir().unwrap();
    let p = plan(dir.path());
    ok(
        dir.path(),
        &[
            "review",
            "approve",
            &p.mission,
            "--identity",
            "Human",
            "Plan approved as written",
        ],
    );
    let markdown = ok(dir.path(), &["brief", &p.mission]);
    let path = PathBuf::from(ok(dir.path(), &["brief", &p.mission, "--html", "-q"]).trim());
    assert_eq!(
        path,
        dir.path()
            .canonicalize()
            .unwrap()
            .join(format!(".atelier/runtime/reviews/{}-brief.html", p.mission))
    );
    let html = fs::read_to_string(path).unwrap();
    for forbidden in ["<script", "<form", "<button", "http://", "https://", "src="] {
        assert!(!html.contains(forbidden), "page holds {forbidden}");
    }
    assert!(html.contains("Plans are &lt;reviewable&gt;.") && !html.contains("<reviewable>"));
    for fact in [
        p.mission.as_str(),
        p.sized.as_str(),
        p.blocked.as_str(),
        p.decision.as_str(),
        p.human.as_str(),
        "plan-shown",
        "The brief shows the plan.",
        "Whether the cache is needed.",
        "The parser may be slow on huge strategies.",
        "Plan approved as written",
        "Choose the store",
    ] {
        assert!(markdown.contains(fact) && html.contains(fact), "{fact}");
    }
    assert!(html.contains("<td>M</td>"), "size shown: {html}");
    // The blocked-by cell and the approval's event ID are on the page.
    let blocked_row = html
        .split(&format!("<td class=\"id\">{}</td>", p.blocked))
        .nth(1)
        .unwrap();
    let row = blocked_row.split("</tr>").next().unwrap();
    assert!(
        row.ends_with(&format!("<td class=\"id\">{}</td>", p.decision)),
        "{row}"
    );
    let approval = markdown
        .lines()
        .find(|line| line.starts_with("approval atelier-"))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap();
    assert!(html.contains(approval), "{approval}");
    assert!(!html.contains("No plan approval recorded."));
}

#[test]
fn brief_says_no_plan_approval_in_both_forms_when_only_other_events_exist() {
    let dir = tempdir().unwrap();
    let p = plan(dir.path());
    ok(
        dir.path(),
        &[
            "review",
            "record",
            &p.mission,
            "--identity",
            "Human",
            "Looked at the plan",
        ],
    );
    let markdown = ok(dir.path(), &["brief", &p.mission]);
    let path = PathBuf::from(ok(dir.path(), &["brief", &p.mission, "--html", "-q"]).trim());
    let html = fs::read_to_string(path).unwrap();
    for form in [&markdown, &html] {
        assert!(
            form.contains("Looked at the plan") && form.contains("No plan approval recorded."),
            "{form}"
        );
    }
}
