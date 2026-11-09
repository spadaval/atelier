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
    assert!(
        page.contains(&format!(
            "Record the human's plan approval: atelier review approve {}",
            p.mission
        )),
        "{page}"
    );
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
    // With the plan approved, the next step moves the draft plan on.
    assert!(!page.contains("atelier review approve"), "{page}");
    assert!(
        page.contains(&format!(
            "Move the approved plan on: atelier issue transition {}\n",
            p.mission
        )),
        "{page}"
    );
    ok(dir.path(), &["issue", "transition", &p.mission, "ready"]);
    let page = ok(dir.path(), &["brief", &p.mission]);
    assert!(
        page.contains("List the work ready to start: atelier work ready\n"),
        "{page}"
    );
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
    assert!(
        !html.contains("atelier review approve")
            && html.contains(&format!(
                "Next: move the approved plan on: atelier issue transition {}.",
                p.mission
            )),
        "{html}"
    );
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
            form.contains("Looked at the plan")
                && form.contains("No plan approval recorded.")
                && form.contains(&format!("atelier review approve {}", p.mission)),
            "{form}"
        );
    }
}

#[test]
fn brief_shows_the_strategy_outcome_and_falls_back_to_the_mission_outcome() {
    let dir = tempdir().unwrap();
    let p = plan(dir.path());
    let strategy = dir.path().join("docs/strategies/plan.md");
    fs::write(
        &strategy,
        "# Outcome\n\nThe strategy says plans are <approved>.\n\n# Success Criteria\n\n- plan-shown: The brief shows the plan.\n",
    )
    .unwrap();
    let both = |dir: &Path| {
        let markdown = ok(dir, &["brief", &p.mission]);
        let path = PathBuf::from(ok(dir, &["brief", &p.mission, "--html", "-q"]).trim());
        (markdown, fs::read_to_string(path).unwrap())
    };
    let (markdown, html) = both(dir.path());
    assert!(
        markdown.contains("Outcome       : The strategy says plans are <approved>.\n")
            && !markdown.contains("Plans are <reviewable>."),
        "{markdown}"
    );
    assert!(
        html.contains("The strategy says plans are &lt;approved&gt;.")
            && !html.contains("Plans are &lt;reviewable&gt;."),
        "{html}"
    );

    // A strategy without an Outcome section leaves the mission's.
    fs::write(
        &strategy,
        "# Success Criteria\n\n- plan-shown: The brief shows the plan.\n",
    )
    .unwrap();
    let (markdown, html) = both(dir.path());
    assert!(
        markdown.contains("Outcome       : Plans are <reviewable>.\n"),
        "{markdown}"
    );
    assert!(html.contains("Plans are &lt;reviewable&gt;."), "{html}");

    // So does a mission with no strategy.
    let bare = create(
        dir.path(),
        &[
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nNo strategy here.",
            "Bare mission",
        ],
    );
    let markdown = ok(dir.path(), &["brief", &bare]);
    assert!(
        markdown.contains("Outcome       : No strategy here.\n"),
        "{markdown}"
    );
}

/// The strategy's Sizing and Derisking sit directly under the Outcome in
/// both forms (atelier-cfq9), so the human sees cost, value and risk first.
#[test]
fn brief_shows_sizing_and_derisking_under_the_outcome() {
    let dir = tempdir().unwrap();
    let p = plan(dir.path());
    fs::write(
        dir.path().join("docs/strategies/plan.md"),
        "# Outcome\n\nPlans are <reviewable>.\n\n# Success Criteria\n\n- plan-shown: The brief shows the plan.\n\n# Sizing\n\n- Complexity: small, one <view>.\n- Worth it: yes.\n\n# Derisking\n\n- Riskiest assumption: the parser copes. Tested first.\n",
    )
    .unwrap();
    let markdown = ok(dir.path(), &["brief", &p.mission]);
    let after_header = markdown
        .split("Generated from: ")
        .nth(1)
        .and_then(|rest| rest.split_once("\n\n"))
        .map(|(_, rest)| rest)
        .expect(&markdown);
    assert!(
        after_header.starts_with(
            "Sizing\n------\n- Complexity: small, one <view>.\n- Worth it: yes.\n\nDerisking\n---------\n- Riskiest assumption: the parser copes. Tested first.\n\nSuccess Criteria\n"
        ),
        "{markdown}"
    );
    assert!(!markdown.contains("not written"), "{markdown}");

    let path = PathBuf::from(ok(dir.path(), &["brief", &p.mission, "--html", "-q"]).trim());
    let html = fs::read_to_string(path).unwrap();
    let at = |text: &str| html.find(text).expect(text);
    assert!(
        at("Plans are &lt;reviewable&gt;.") < at("<strong>Sizing</strong>")
            && at("<strong>Sizing</strong>") < at("Complexity: small, one &lt;view&gt;.")
            && at("Complexity: small, one &lt;view&gt;.") < at("<strong>Derisking</strong>")
            && at("<strong>Derisking</strong>") < at("Riskiest assumption: the parser copes.")
            && at("Riskiest assumption: the parser copes.") < at("class=\"tiles\""),
        "{html}"
    );
    assert!(
        !html.contains("<view>") && !html.contains("not written"),
        "{html}"
    );
}

/// A strategy missing either section, or a mission with no strategy, says so
/// under the Outcome instead of leaving the gap unseen; nothing refuses.
#[test]
fn brief_says_when_sizing_or_derisking_is_not_written() {
    let dir = tempdir().unwrap();
    let p = plan(dir.path());
    fs::write(
        dir.path().join("docs/strategies/plan.md"),
        "# Outcome\n\nPlans are <reviewable>.\n\n# Sizing\n\n- Complexity: small.\n",
    )
    .unwrap();
    let markdown = ok(dir.path(), &["brief", &p.mission]);
    assert!(
        markdown.contains(
            "Outcome       : Plans are <reviewable>.\nDerisking     : not written\nStrategy      : "
        ) && markdown.contains("Sizing\n------\n- Complexity: small.\n")
            && !markdown.contains("Sizing        : not written"),
        "{markdown}"
    );
    let path = PathBuf::from(ok(dir.path(), &["brief", &p.mission, "--html", "-q"]).trim());
    let html = fs::read_to_string(path).unwrap();
    assert!(
        html.contains("<strong>Sizing</strong>")
            && html.contains("Derisking: not written")
            && !html.contains("Sizing: not written"),
        "{html}"
    );

    let bare = create(
        dir.path(),
        &[
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nNo strategy here.",
            "Bare mission",
        ],
    );
    let markdown = ok(dir.path(), &["brief", &bare]);
    assert!(
        markdown.contains(
            "Outcome       : No strategy here.\nSizing        : not written\nDerisking     : not written\n"
        ),
        "{markdown}"
    );
    let path = PathBuf::from(ok(dir.path(), &["brief", &bare, "--html", "-q"]).trim());
    let html = fs::read_to_string(path).unwrap();
    assert!(
        html.contains("Sizing: not written") && html.contains("Derisking: not written"),
        "{html}"
    );
}

/// A mission is a themed container whose success criteria are optional
/// (ADR 0024): one with only an Outcome and no strategy passes `check`,
/// becomes ready, and every mission view leaves criteria out rather than
/// showing a placeholder or an empty section.
#[test]
fn mission_without_criteria_is_valid_and_its_views_omit_criteria() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let mission = create(
        dir.path(),
        &[
            "--issue-type",
            "mission",
            "--body",
            "Make the widgets consistent.",
            "Themed mission",
        ],
    );
    let record = fs::read_to_string(canonical_issue_path(dir.path(), &mission)).unwrap();
    assert!(
        record.contains("## Outcome\n\nMake the widgets consistent.")
            && !record.contains("Success criteria"),
        "{record}"
    );
    let item = create(
        dir.path(),
        &[
            "--body",
            "## Goal\n\nFix A.\n\n## Done when\n\nA matches.",
            "Widget A",
        ],
    );
    ok(
        dir.path(),
        &["issue", "link", &item, &mission, "--role", "advances"],
    );
    ok(dir.path(), &["check"]);
    ok(dir.path(), &["check", &mission]);
    ok(dir.path(), &["issue", "transition", &mission, "ready"]);

    let brief = ok(dir.path(), &["brief", &mission]);
    let work = ok(dir.path(), &["work", "mission", &mission]);
    mark_done_with_evidence(dir.path(), &item, Some("Done A."));
    let debrief = ok(dir.path(), &["debrief", &mission]);
    let brief_html =
        fs::read_to_string(ok(dir.path(), &["brief", &mission, "--html", "-q"]).trim()).unwrap();
    let debrief_html =
        fs::read_to_string(ok(dir.path(), &["debrief", &mission, "--html", "-q"]).trim()).unwrap();
    for (view, text) in [
        ("brief", &brief),
        ("work mission", &work),
        ("debrief", &debrief),
        ("brief --html", &brief_html),
        ("debrief --html", &debrief_html),
    ] {
        // `work mission` lists the work, not the Outcome.
        assert!(
            text.contains(&item)
                && (view == "work mission" || text.contains("Make the widgets consistent.")),
            "{view}: {text}"
        );
        assert!(
            !text.to_lowercase().contains("criteri") && !text.contains("not specified"),
            "{view} mentions criteria: {text}"
        );
    }
}

/// A linked strategy with no Success Criteria heading means the mission has
/// no criteria (ADR 0024), so no view reports the missing heading; malformed
/// bullets under a heading that exists are still reported.
#[test]
fn strategy_without_criteria_heading_means_no_criteria() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    fs::create_dir_all(dir.path().join("docs/strategies")).unwrap();
    fs::write(
        dir.path().join("docs/strategies/plain.md"),
        "# Outcome\n\nWidgets are consistent.\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("docs/strategies/malformed.md"),
        "# Outcome\n\nO.\n\n# Success Criteria\n\n- **S1 e2e.** The scenario passes.\n",
    )
    .unwrap();
    let mission = |strategy: &str, title: &str| {
        create(
            dir.path(),
            &[
                "--issue-type",
                "mission",
                "--strategy",
                strategy,
                "--body",
                "Widgets are consistent.",
                title,
            ],
        )
    };
    let plain = mission("docs/strategies/plain.md", "Plain mission");
    let malformed = mission("docs/strategies/malformed.md", "Malformed mission");
    ok(dir.path(), &["check"]);

    for args in [
        vec!["brief", plain.as_str()],
        vec!["work", "mission", plain.as_str()],
        vec!["debrief", plain.as_str()],
    ] {
        let text = ok(dir.path(), &args);
        assert!(
            !text.to_lowercase().contains("criteri") && !text.contains('⚠'),
            "{args:?}: {text}"
        );
    }
    let html =
        fs::read_to_string(ok(dir.path(), &["brief", &plain, "--html", "-q"]).trim()).unwrap();
    assert!(!html.to_lowercase().contains("criteri"), "{html}");

    for args in [
        vec!["brief", malformed.as_str()],
        vec!["work", "mission", malformed.as_str()],
    ] {
        let text = ok(dir.path(), &args);
        assert!(
            text.contains("Success Criteria")
                && text.contains("does not start with a criterion slug"),
            "{args:?}: {text}"
        );
    }
}
