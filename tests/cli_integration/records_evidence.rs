use super::support::*;

#[test]
fn test_command_result_json_mode_is_rejected_and_human_subset_works() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier_raw(dir.path(), &["--json", "doctor"]);
    assert!(!success, "--json should not be accepted");
    assert!(stderr.contains("unexpected argument '--json'"));

    for args in [
        vec!["issue", "list", "--json"],
        vec!["issue", "show", "1", "--json"],
        vec!["issue", "update", "1", "--priority", "high", "--json"],
        vec!["mission", "list", "--json"],
        vec!["workflow", "check", "--json"],
        vec!["doctor", "--json"],
    ] {
        let (success, _, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} should reject --json");
        assert!(
            stderr.contains("unexpected argument '--json'"),
            "{args:?} stderr did not reject --json: {stderr}"
        );
    }

    for args in [
        vec!["issue", "--help"],
        vec!["mission", "--help"],
        vec!["workflow", "--help"],
        vec!["doctor", "--help"],
    ] {
        let (success, stdout, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(success, "{args:?} help failed: {stderr}");
        assert!(
            !stdout.contains("--json"),
            "{args:?} help still advertises --json:\n{stdout}"
        );
    }

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Agent Factory task",
            "--issue-type",
            "feature",
            "--label",
            "agent-factory",
            "--description",
            "## Description\n\nWork item.\n\n## Outcome\n\nFactory task remains valid for human command-result checks.\n\n## Evidence\n\n- `atelier lint` passes for the command-result fixture.",
        ],
    );
    assert!(success, "create failed: {stderr}");
    assert!(stdout.contains("Created issue atelier-"));
    assert!(stdout.contains("Type:     feature"));
    assert!(stdout.contains("Next Commands"));
    let task_id = issue_id_by_title(dir.path(), "Agent Factory task");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "update", "1", "--claim"]);
    assert!(!success, "issue update --claim unexpectedly succeeded");
    assert!(
        stderr.contains("unexpected argument '--claim'"),
        "issue update --claim should be rejected as unsupported:\n{stderr}"
    );

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "update", "1", "--priority", "high"]);
    assert!(success, "update failed: {stderr}");
    assert!(stdout.contains(&format!("Updated issue {task_id}")));
    assert!(stdout.contains("Priority: high"));

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "note", &task_id, "handoff note"]);
    assert!(success, "issue note failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &task_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains("handoff note"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "ready failed: {stderr}");
    assert!(stdout.contains("1 total"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Blocker",
            "--description",
            "## Description\n\nBlocking fixture issue.\n\n## Outcome\n\nBlocker issue participates in dependency command-result checks.\n\n## Evidence\n\n- `atelier lint` passes for the command-result fixture.",
        ],
    );
    assert!(success, "blocker create failed: {stderr}");
    let blocker_id = issue_ref(dir.path(), 2);
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "block", "1", "2"]);
    assert!(success, "issue block failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains(&blocker_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "blocked", "1"]);
    assert!(success, "issue blocked failed: {stderr}");
    assert!(stdout.contains(&blocker_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "unblock", "1", "2"]);
    assert!(success, "issue unblock failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains(&blocker_id));

    for args in [
        vec!["issue", "list", "--status", "all"],
        vec!["search", "Factory"],
        vec!["lint"],
        vec!["export"],
        vec!["export", "--check"],
        vec!["doctor"],
        vec!["rebuild"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "{args:?} failed: {stderr}");
        assert!(!stdout.trim_start().starts_with('{'), "{args:?}");
    }

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", "missing"]);
    assert!(!success);
    assert!(stderr.contains("missing"));
}

#[test]
fn test_wrong_kind_record_ids_report_actual_kind_and_correct_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Corrective mission",
            "--body",
            "Mission body",
            "--validation",
            "Wrong-kind command output is corrective",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Corrective mission");
    let mission_id = mission_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Corrective issue",
            "--description",
            "## Description\n\nIssue fixture.\n\n## Outcome\n\nWrong-kind command output is corrective.\n\n## Evidence\n\n- Focused CLI checks pass.",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = record_id_by_title(dir.path(), "issues", "Corrective issue");
    let issue_id = issue_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--result",
            "pass",
            "wrong-kind fixture evidence",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    let evidence_id = record_id_by_title(dir.path(), "evidence", "wrong-kind fixture evidence");
    let evidence_id = evidence_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", mission_id]);
    assert!(!success, "mission ID should not resolve as an issue");
    assert!(
        stderr.contains(&format!(
            "{mission_id} is a mission record, not an issue record"
        )),
        "wrong-kind issue read should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier mission show {mission_id}")),
        "wrong-kind issue read should suggest mission show: {stderr}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", mission_id, "--reason", "wrong kind"],
    );
    assert!(!success, "mission ID should not close as an issue");
    assert!(
        stderr.contains(&format!(
            "{mission_id} is a mission record, not an issue record"
        )),
        "wrong-kind issue mutation should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier mission show {mission_id}")),
        "wrong-kind issue mutation should suggest mission show: {stderr}"
    );

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", evidence_id]);
    assert!(!success, "evidence ID should not resolve as an issue");
    assert!(
        stderr.contains(&format!(
            "{evidence_id} is a evidence record, not an issue record"
        )),
        "wrong-kind evidence lookup should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier evidence show {evidence_id}")),
        "wrong-kind evidence lookup should suggest evidence show: {stderr}"
    );

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "status", issue_id]);
    assert!(!success, "issue ID should not resolve as a mission");
    assert!(
        stderr.contains(&format!(
            "{issue_id} is a issue record, not a mission record"
        )),
        "wrong-kind mission read should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier issue show {issue_id}")),
        "wrong-kind mission read should suggest issue show: {stderr}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "note", issue_id, "wrong kind mission note"],
    );
    assert!(!success, "issue ID should not accept a mission note");
    assert!(
        stderr.contains(&format!(
            "{issue_id} is a issue record, not a mission record"
        )),
        "wrong-kind mission mutation should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier issue show {issue_id}")),
        "wrong-kind mission mutation should suggest issue show: {stderr}"
    );

    let unknown_id = "atelier-zzzz";
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", unknown_id]);
    assert!(!success, "unknown ID should fail");
    assert!(
        stderr.contains("was not found"),
        "unknown ID should keep concise not-found error: {stderr}"
    );
    assert!(
        !stderr.contains("record, not"),
        "unknown ID should not imply a wrong-kind match: {stderr}"
    );
}

#[test]
fn test_first_class_records_export_rebuild_and_validate() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Ship records",
            "--body",
            "Mission body",
            "--constraint",
            "Keep issues accountable",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Ship records");
    let mission_id = mission_id.as_str();
    let (success, mission_update, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "update",
            mission_id,
            "--body",
            "Updated mission body",
            "--risk",
            "Projection drift",
            "--validation",
            "Run focused mission checks",
        ],
    );
    assert!(success, "mission update failed: {stderr}");
    assert!(mission_update.contains("Status: ready"));

    let (success, plan_out, stderr) = run_atelier(
        dir.path(),
        &["plan", "create", "Execution plan", "--body", "Do the thing"],
    );
    assert!(success, "plan create failed: {stderr}");
    assert!(plan_out.contains("[plan] open - Execution plan"));
    let plan_id = record_id_by_title(dir.path(), "plans", "Execution plan");
    let plan_id = plan_id.as_str();
    let (success, revise_out, stderr) = run_atelier(
        dir.path(),
        &[
            "plan",
            "revise",
            plan_id,
            "Do the thing, then verify the projection.",
            "--reason",
            "projection-first",
        ],
    );
    assert!(success, "plan revise failed: {stderr}");
    assert!(revise_out.contains("Revision: 2"));

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--result",
            "pass",
            "cargo test passed",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - cargo test passed"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "cargo test passed");
    let evidence_id = evidence_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", mission_id]);
    assert!(!success, "mission ID should not resolve as an issue");
    assert!(
        stderr.contains(&format!(
            "{mission_id} is a mission record, not an issue record"
        )),
        "wrong-kind error should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier mission show {mission_id}")),
        "wrong-kind error should suggest mission show: {stderr}"
    );

    let unknown_id = "atelier-zzzz";
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", unknown_id]);
    assert!(!success, "unknown ID should fail");
    assert!(
        stderr.contains("was not found"),
        "unknown ID should keep concise not-found error: {stderr}"
    );
    assert!(
        !stderr.contains("is a mission record"),
        "unknown ID should not imply a wrong-kind match: {stderr}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "plan",
            "link",
            plan_id,
            "mission",
            mission_id,
            "--type",
            "planned_by",
        ],
    );
    assert!(success, "mission-plan link failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", evidence_id, "mission", mission_id],
    );
    assert!(success, "evidence attach failed: {stderr}");

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Wire mission work",
            "--issue-type",
            "task",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Wire mission work");
    let issue_id = issue_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
    assert!(success, "mission-work link failed: {stderr}");
    let (success, blocker_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Resolve mission blocker",
            "--issue-type",
            "task",
        ],
    );
    assert!(success, "blocker issue create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Resolve mission blocker");
    let blocker_id = blocker_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-blocker", mission_id, blocker_id],
    );
    assert!(success, "mission-blocker link failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed: {stderr}");

    let mission_path = dir
        .path()
        .join(".atelier")
        .join("missions")
        .join(format!("{mission_id}.md"));
    let mission_markdown = std::fs::read_to_string(&mission_path).unwrap();
    assert!(mission_markdown.contains("schema: \"atelier.mission\""));
    assert!(mission_markdown.contains("schema_version: 1"));
    assert!(!mission_markdown.contains("\ndata: "));
    assert!(mission_markdown.contains("labels:\n- \"mission\"\n"));
    assert!(mission_markdown.contains("## Intent\n\nUpdated mission body"));
    assert!(mission_markdown.contains("## Constraints\n\n- Keep issues accountable"));
    assert!(mission_markdown.contains("## Risks\n\n- Projection drift"));
    assert!(mission_markdown.contains("## Validation\n\n- Run focused mission checks"));
    assert!(mission_markdown.contains("relationships:"));
    assert!(!mission_markdown.contains("  attachments:\n  - kind: \"issue\""));
    assert!(mission_markdown.contains(&format!(
        "  - kind: \"issue\"\n    id: \"{blocker_id}\"\n    type: \"blocked_by\""
    )));
    assert!(mission_markdown.contains(&format!(
        "  - kind: \"issue\"\n    id: \"{issue_id}\"\n    type: \"advances\""
    )));

    let plan_path = dir
        .path()
        .join(".atelier")
        .join("plans")
        .join(format!("{plan_id}.md"));
    let plan_markdown = std::fs::read_to_string(&plan_path).unwrap();
    assert!(plan_markdown.contains("schema: \"atelier.plan\""));
    assert!(!plan_markdown.contains("\ndata: "));
    assert!(plan_markdown.contains("Do the thing, then verify the projection."));
    assert!(plan_markdown.contains("revision: 2"));
    assert!(plan_markdown.contains("reason: \"projection-first\""));
    assert!(plan_markdown.contains(&format!("id: \"{mission_id}\"")));

    let evidence_path = dir
        .path()
        .join(".atelier")
        .join("evidence")
        .join(format!("{evidence_id}.md"));
    let evidence_markdown = std::fs::read_to_string(&evidence_path).unwrap();
    assert!(evidence_markdown.contains("schema: \"atelier.evidence\""));
    assert!(!evidence_markdown.contains("\ndata: "));
    assert!(evidence_markdown.contains("evidence_type: \"test\""));
    assert!(evidence_markdown.contains(&format!("id: \"{mission_id}\"")));

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, view_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(view_out.contains("Records: plans=1 milestones=0 evidence=1"));
    assert!(view_out.contains("Work: ready=1 blocked=0 done=0 backlog=0"));
    assert!(view_out.contains(&blocker_id));

    let (success, show_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(show_out.contains("Plans"));
    assert!(show_out.contains("Evidence"));
    assert!(show_out.contains("Mission Blockers"));

    let (success, human_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "human mission show failed: {stderr}");
    assert!(human_out.contains(&format!("Mission {mission_id} [ready] - Ship records")));
    assert!(human_out.contains("Constraints"));
    assert!(human_out.contains("Keep issues accountable"));
    assert!(human_out.contains("Progress"));
    assert!(human_out.contains("Records: plans=1 milestones=0 evidence=1"));
    assert!(human_out.contains("Work: ready=1 blocked=0 done=0 backlog=0"));
    assert!(human_out.contains("Plans"));
    assert!(human_out.contains("Execution plan"));
    assert!(human_out.contains("Evidence"));
    assert!(human_out.contains("cargo test passed"));
    assert!(human_out.contains("Mission Blockers"));
    assert!(human_out.contains("Resolve mission blocker"));
    assert!(human_out.contains("(open blocker)"));
    assert!(human_out.contains("Linked Work"));
    assert!(human_out.contains("Ready (1)"));
    assert!(human_out.contains("Wire mission work"));
    assert!(human_out.contains("Evidence Gaps"));
    assert!(human_out.contains("(none)"));
    assert!(human_out.contains("Next Commands"));
    assert!(human_out.contains("atelier mission status"));

    let (success, plan_show, stderr) = run_atelier(dir.path(), &["plan", "show", plan_id]);
    assert!(success, "human plan show failed: {stderr}");
    assert!(plan_show.contains(&format!("{plan_id} [plan] open - Execution plan")));
    assert!(plan_show.contains("Revision: 2"));
    assert!(plan_show.contains("Body"));
    assert!(plan_show.contains("Do the thing, then verify the projection."));
    assert!(plan_show.contains("Links:"));

    let (success, plan_list, stderr) = run_atelier(dir.path(), &["plan", "list"]);
    assert!(success, "human plan list failed: {stderr}");
    assert!(plan_list.contains("Plans"));
    assert!(plan_list.contains("1 total"));
    assert!(plan_list.contains("Execution plan"));

    let (success, evidence_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", evidence_id]);
    assert!(success, "human evidence show failed: {stderr}");
    assert!(evidence_show.contains(&format!(
        "{evidence_id} [evidence] pass - cargo test passed"
    )));
    assert!(evidence_show.contains("Result:"));
    assert!(evidence_show.contains("Kind:"));
    assert!(evidence_show.contains("Summary"));

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "human evidence list failed: {stderr}");
    assert!(evidence_list.contains("Evidence"));
    assert!(evidence_list.contains("1 total"));
    assert!(evidence_list.contains("cargo test passed"));
}

#[test]
fn test_mission_relationship_filtering_keeps_supporting_records_out_of_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Filtered mission"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Filtered mission");
    let mission_id = mission_id.as_str();

    let (success, work_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Counted work"]);
    assert!(success, "work issue create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Counted work");
    let work_id = work_id.as_str();

    let (success, support_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Supporting reference"]);
    assert!(success, "support issue create failed: {stderr}");
    assert!(support_out.contains("Created issue atelier-"));
    let support_id = issue_id_by_title(dir.path(), "Supporting reference");
    let support_id = support_id.as_str();

    let (success, blocker_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Direct blocker"]);
    assert!(success, "blocker issue create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Direct blocker");
    let blocker_id = blocker_id.as_str();

    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, work_id]);
    assert!(success, "mission add-work failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-blocker", mission_id, blocker_id],
    );
    assert!(success, "mission add-blocker failed: {stderr}");

    let mission_path = dir
        .path()
        .join(".atelier")
        .join("missions")
        .join(format!("{mission_id}.md"));
    let mission_markdown = std::fs::read_to_string(&mission_path).unwrap();
    std::fs::write(
        &mission_path,
        mission_markdown.replace(
            "schema: \"atelier.mission\"",
            &format!(
                "  - kind: \"issue\"\n    id: \"{support_id}\"\n    type: \"related\"\nschema: \"atelier.mission\""
            ),
        ),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(
        success,
        "rebuild after supporting relation edit failed: {stderr}"
    );

    let (success, show_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(show_out.contains("Work: ready=1 blocked=0 done=0 backlog=0"));
    assert!(show_out.contains("Mission Blockers: 1"));
    assert!(show_out.contains("Linked Work"));
    assert!(show_out.contains("Counted work"));
    assert!(show_out.contains("Supporting Records"));
    assert!(show_out.contains("Supporting reference (related)"));

    let linked_work = show_out
        .split("Linked Work")
        .nth(1)
        .and_then(|text| text.split("Supporting Records").next())
        .unwrap_or("");
    assert!(
        !linked_work.contains("Supporting reference"),
        "supporting relation was rendered as linked work:\n{show_out}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Total: 1 ready"));
    assert!(status_out.contains("Mission blockers: 1 open"));
}

#[test]
fn test_mission_unlink_removes_added_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Repair mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Repair mission");
    let mission_id = mission_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Accidental work"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Accidental work");
    let issue_id = issue_id.as_str();

    let (success, add_out, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
    assert!(success, "mission add-work failed: {stderr}");
    assert!(add_out.contains(&format!("Added work {issue_id} to mission {mission_id}")));

    let (success, linked_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show after add-work failed: {stderr}");
    assert!(linked_out.contains("Linked Work"));
    assert!(linked_out.contains("Accidental work"));

    let (success, unlink_out, stderr) =
        run_atelier(dir.path(), &["mission", "unlink", mission_id, issue_id]);
    assert!(success, "mission unlink failed: {stderr}");
    assert!(unlink_out.contains(&format!(
        "Unlinked work {issue_id} from mission {mission_id}"
    )));

    let (success, show_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show after unlink failed: {stderr}");
    assert!(show_out.contains("Linked Work"));
    assert!(!show_out.contains("Accidental work"));
    assert!(show_out.contains("Work: ready=0 blocked=0 done=0 backlog=0"));

    let mission_markdown = read_canonical_record(dir.path(), "missions", mission_id);
    assert!(!mission_markdown.contains(&format!(
        "  - kind: \"issue\"\n    id: \"{issue_id}\"\n    type: \"advances\""
    )));
}

#[test]
fn test_evidence_capture_records_command_metadata_and_attaches_targets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Capture issue", "--issue-type", "task"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Capture issue");
    let issue_id = issue_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Capture epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Capture epic");
    let epic_id = epic_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Capture mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Capture mission");
    let mission_id = mission_id.as_str();

    let (success, issue_capture, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "--summary",
            "issue command proof",
            "--target",
            &format!("issue/{issue_id}"),
            "--",
            "sh",
            "-c",
            "printf 'pass stdout\\n'; printf 'pass stderr\\n' >&2",
        ],
    );
    assert!(success, "issue command record failed: {stderr}");
    assert!(issue_capture.contains("[evidence] pass - issue command proof"));
    assert!(issue_capture.contains("Command:     sh -c"));
    assert!(issue_capture.contains("Exit Status: 0"));
    assert!(issue_capture.contains(&format!("Target:      issue/{issue_id} (validates)")));
    assert!(issue_capture.contains("Captured:"));
    assert!(issue_capture.contains("pass stdout"));
    assert!(issue_capture.contains("pass stderr"));
    let issue_evidence_id = record_id_by_title(dir.path(), "evidence", "issue command proof");
    let issue_evidence_front_matter =
        canonical_evidence_front_matter(dir.path(), &issue_evidence_id);
    assert_eq!(
        issue_evidence_front_matter["proof_scope"],
        "scoped to the attached target or summary"
    );
    assert_eq!(
        issue_evidence_front_matter["independence_level"],
        "unspecified"
    );
    assert_eq!(
        issue_evidence_front_matter["agent_identity"],
        serde_json::Value::Null
    );
    assert_eq!(
        issue_evidence_front_matter["residual_risks"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        issue_evidence_front_matter["follow_up_ids"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(issue_evidence_front_matter["evidence_type"], "validation");
    assert!(issue_evidence_front_matter["command"]
        .as_str()
        .unwrap()
        .starts_with("sh -c"));

    let (success, record_capture, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "--summary",
            "unified command proof",
            "--target",
            &format!("issue/{issue_id}"),
            "--",
            "sh",
            "-c",
            "printf 'record stdout\\n'",
        ],
    );
    assert!(success, "unified command record failed: {stderr}");
    assert!(record_capture.contains("[evidence] pass - unified command proof"));
    assert!(record_capture.contains("Command:     sh -c"));
    assert!(record_capture.contains("Exit Status: 0"));
    assert!(record_capture.contains(&format!("Target:      issue/{issue_id} (validates)")));
    assert!(record_capture.contains("record stdout"));

    let positional_summary = "unified positional manual proof";
    let (success, positional_record_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "deferred",
            "--target",
            &format!("issue/{issue_id}"),
            positional_summary,
        ],
    );
    assert!(success, "positional manual record failed: {stderr}");
    assert!(positional_record_out.contains("[evidence] deferred - unified positional manual proof"));
    assert!(positional_record_out.contains(&format!("Target:      issue/{issue_id} (validates)")));
    let positional_evidence_id = record_id_by_title(dir.path(), "evidence", positional_summary);
    let positional_evidence_front_matter =
        canonical_evidence_front_matter(dir.path(), &positional_evidence_id);
    assert_eq!(
        positional_evidence_front_matter["evidence_type"],
        "validation"
    );
    assert_eq!(
        positional_evidence_front_matter["proof_scope"],
        "scoped to the attached target or summary"
    );
    assert_eq!(
        positional_evidence_front_matter["independence_level"],
        "unspecified"
    );
    assert_eq!(
        positional_evidence_front_matter["agent_identity"],
        serde_json::Value::Null
    );
    assert_eq!(
        positional_evidence_front_matter["residual_risks"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        positional_evidence_front_matter["follow_up_ids"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(positional_evidence_front_matter["status"], "deferred");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "--summary",
            "flag summary",
            "positional summary",
        ],
    );
    assert!(!success, "conflicting summaries should fail");
    assert!(
        stderr.contains("use either --summary or a positional summary"),
        "conflict error should be actionable: {stderr}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "fail",
            "--summary",
            "epic failing command proof",
            "--target",
            &format!("epic/{epic_id}"),
            "--",
            "sh",
            "-c",
            "printf 'failing stdout\\n'; printf 'failing stderr\\n' >&2; exit 7",
        ],
    );
    assert!(success, "epic failing command record failed: {stderr}");
    let epic_evidence_id = record_id_by_title(dir.path(), "evidence", "epic failing command proof");
    let (success, epic_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &epic_evidence_id]);
    assert!(success, "epic evidence show failed: {stderr}");
    assert!(epic_show.contains("Result:      fail"));
    assert!(epic_show.contains("Exit Status: 7"));
    assert!(epic_show.contains(&format!("Target:      epic/{epic_id} (validates)")));
    assert!(epic_show.contains("failing stdout"));
    assert!(epic_show.contains("failing stderr"));

    let manual_epic_summary =
        "manual epic contract audit line-by-line classification maps epic outcome lines to proof";
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            manual_epic_summary,
        ],
    );
    assert!(success, "manual evidence record failed: {stderr}");
    let manual_epic_evidence_id = record_id_by_title(dir.path(), "evidence", manual_epic_summary);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            &manual_epic_evidence_id,
            "epic",
            epic_id,
        ],
    );
    assert!(success, "manual epic evidence attach failed: {stderr}");
    let (success, manual_epic_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &manual_epic_evidence_id]);
    assert!(success, "manual epic evidence show failed: {stderr}");
    assert!(manual_epic_show.contains(&format!("Target:      epic/{epic_id} (validates)")));

    let manual_issue_summary = "unified manual issue proof";
    let (success, manual_record_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "--summary",
            manual_issue_summary,
            "--target",
            &format!("issue/{issue_id}"),
        ],
    );
    assert!(success, "unified manual record failed: {stderr}");
    assert!(manual_record_out.contains("[evidence] pass - unified manual issue proof"));
    assert!(manual_record_out.contains(&format!("Target:      issue/{issue_id} (validates)")));
    let manual_issue_evidence_id = record_id_by_title(dir.path(), "evidence", manual_issue_summary);
    let manual_issue_front_matter =
        canonical_evidence_front_matter(dir.path(), &manual_issue_evidence_id);
    assert_eq!(manual_issue_front_matter["target"]["kind"], "issue");
    assert_eq!(manual_issue_front_matter["target"]["id"], issue_id);
    assert_eq!(manual_issue_front_matter["target"]["role"], "validates");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "blocked",
            "--summary",
            "mission blocked command proof",
            "--target",
            &format!("mission/{mission_id}"),
            "--",
            "sh",
            "-c",
            "i=0; while [ $i -lt 350 ]; do printf 'blocked-line-%03d\\n' \"$i\"; i=$((i + 1)); done; printf 'blocked stderr\\n' >&2; exit 2",
        ],
    );
    assert!(success, "mission blocked command record failed: {stderr}");
    let mission_evidence_id =
        record_id_by_title(dir.path(), "evidence", "mission blocked command proof");
    let (success, mission_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &mission_evidence_id]);
    assert!(success, "mission evidence show failed: {stderr}");
    assert!(mission_show.contains("Result:      blocked"));
    assert!(mission_show.contains("Exit Status: 2"));
    assert!(mission_show.contains(&format!("Target:      mission/{mission_id} (validates)")));
    assert!(mission_show.contains("blocked-line-000"));
    assert!(!mission_show.contains("blocked-line-349"));
    assert!(mission_show.contains("Stdout: "));
    assert!(mission_show.contains("truncated: yes"));

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    assert!(evidence_list.contains(&issue_evidence_id));
    assert!(evidence_list.contains("exit=0"));
    assert!(evidence_list.contains(&format!("target=issue/{issue_id}")));
    assert!(evidence_list.contains("command=sh -c"));
    assert!(evidence_list.contains(&epic_evidence_id));
    assert!(evidence_list.contains("exit=7"));
    assert!(evidence_list.contains(&format!("target=epic/{epic_id}")));
    assert!(evidence_list.contains(&manual_epic_evidence_id));
    assert!(evidence_list.contains(&mission_evidence_id));
    assert!(evidence_list.contains("exit=2"));
    assert!(evidence_list.contains(&format!("target=mission/{mission_id}")));
}

#[test]
fn test_evidence_relation_role_errors_are_corrective() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Corrective evidence target"],
    );
    assert!(success, "issue create failed: {stderr}");
    let target_id = issue_id_by_title(dir.path(), "Corrective evidence target");
    let target_id = target_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "--target",
            &format!("issue/{target_id}"),
            "--role",
            "validation",
            "bad role proof",
        ],
    );
    assert!(!success, "invalid evidence record role should fail");
    assert_corrective_evidence_role_error(&stderr);

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    assert!(
        evidence_list.contains("(none)"),
        "invalid targeted record should not create evidence: {evidence_list}"
    );

    let (success, record_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "--target",
            &format!("issue/{target_id}"),
            "accepted target proof",
        ],
    );
    assert!(success, "accepted evidence record failed: {stderr}");
    assert!(record_out.contains(&format!("Target:      issue/{target_id} (validates)")));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "accepted target proof");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Reused evidence target"]);
    assert!(success, "reuse target issue create failed: {stderr}");
    let reuse_id = issue_id_by_title(dir.path(), "Reused evidence target");
    let reuse_id = reuse_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            &evidence_id,
            "issue",
            reuse_id,
            "--role",
            "validation",
        ],
    );
    assert!(!success, "invalid evidence attach role should fail");
    assert_corrective_evidence_role_error(&stderr);

    let (success, attach_out, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "issue", reuse_id],
    );
    assert!(success, "accepted evidence attach failed: {stderr}");
    assert!(attach_out.contains(&format!(
        "Attached evidence {evidence_id} to issue {reuse_id} (validates)"
    )));

    let (success, show_out, stderr) = run_atelier(dir.path(), &["evidence", "show", &evidence_id]);
    assert!(success, "evidence show failed: {stderr}");
    assert!(show_out.contains(&format!("issue/{target_id} (validates)")));
    assert!(show_out.contains(&format!("issue/{reuse_id} (validates)")));
}

fn assert_corrective_evidence_role_error(stderr: &str) {
    assert!(
        stderr.contains("Invalid evidence relation role 'validation'"),
        "error should name invalid role: {stderr}"
    );
    assert!(
        stderr.contains("Accepted evidence relation vocabulary: validates"),
        "error should name accepted relation vocabulary: {stderr}"
    );
    assert!(
        stderr.contains("Evidence kinds such as validation belong in --kind, not --role"),
        "error should distinguish evidence kind from relation role: {stderr}"
    );
    assert!(
        stderr.contains(
            "atelier evidence record --target issue/<id> --kind validation --result pass \"summary\""
        ),
        "error should name normal targeted record flow: {stderr}"
    );
    assert!(
        stderr.contains("atelier evidence attach <evidence-id> issue <issue-id>"),
        "error should name existing-proof attach flow: {stderr}"
    );
}

#[test]
fn test_evidence_capture_rejects_failed_commands_as_pass_proof() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "--summary",
            "bad pass proof",
            "--",
            "sh",
            "-c",
            "printf 'not a pass\\n'; exit 3",
        ],
    );
    assert!(!success, "nonzero command cannot become pass evidence");
    assert!(stderr.contains("cannot record pass evidence"));

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    assert!(evidence_list.contains("(none)"));
}

#[test]
fn test_issue_closeout_rejects_evidence_attached_to_another_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Target proof"]);
    assert!(success, "target issue create failed: {stderr}");
    let target_id = issue_id_by_title(dir.path(), "Target proof");
    let target_id = target_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Donor proof"]);
    assert!(success, "donor issue create failed: {stderr}");
    let donor_id = issue_id_by_title(dir.path(), "Donor proof");
    let donor_id = donor_id.as_str();

    let evidence_id = attach_issue_pass_evidence(dir.path(), donor_id);

    move_issue_to_validation(dir.path(), target_id);
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", target_id, "--reason", "done"],
    );
    assert!(
        !success,
        "issue closeout must reject evidence linked only to another issue"
    );
    assert!(stderr.contains("expected at least 1 passing evidence record"));
    assert!(stderr.contains(target_id));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "issue", target_id],
    );
    assert!(success, "target evidence attach failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", target_id, "--reason", "target proof"],
    );
    assert!(
        success,
        "target closeout should pass after direct proof: {stderr}"
    );
}

#[test]
fn test_issue_closeout_uses_attached_pass_evidence_not_evidence_text() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let issue_body = "## Description\n\nEvidence gate body.\n\n## Outcome\n\nThe issue can close after workflow proof is attached.\n\n## Evidence\n\n- A focused command transcript proves the workflow change.";
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Evidence gate proof",
            "--description",
            issue_body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Evidence gate proof");

    move_issue_to_validation(dir.path(), &issue_id);
    attach_pass_evidence(
        dir.path(),
        "issue",
        &issue_id,
        "workflow close gate regression transcript recorded",
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "close",
            &issue_id,
            "--reason",
            "workflow proof attached",
        ],
    );
    assert!(
        success,
        "issue closeout should use attached pass evidence rather than Evidence text matching: {stderr}"
    );
}

#[test]
fn test_validation_issue_closeout_uses_workflow_approval_not_contract_audit_terms() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let validation_body = "## Description\n\nValidation item body.\n\n## Outcome\n\nThe validation item can close after independent approval is attached.\n\n## Evidence\n\n- `atelier evidence show <id>` displays the approval record attached to this validation issue.";
    let (success, validation_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "High-risk validation proof",
            "--issue-type",
            "validation",
            "--description",
            validation_body,
        ],
    );
    assert!(success, "validation issue create failed: {stderr}");
    assert!(validation_out.contains("Created issue atelier-"));
    let validation_id = issue_id_by_title(dir.path(), "High-risk validation proof");

    move_issue_to_validation(dir.path(), &validation_id);
    attach_pass_evidence(
        dir.path(),
        "issue",
        &validation_id,
        "independent approval evidence recorded for workflow closeout",
    );

    let (success, transitions, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &validation_id, "--options"],
    );
    assert!(success, "transition options failed: {stderr}");
    assert!(transitions.contains("close"));
    assert!(transitions.contains("pass  proof_attached"));
    assert!(!transitions.contains("contract-audit"));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "close",
            &validation_id,
            "--reason",
            "independent approval attached",
        ],
    );
    assert!(
        success,
        "validation issue closeout should use workflow approval plus attached evidence: {stderr}"
    );
}

#[test]
fn test_validation_issue_closeout_allows_freshly_recorded_evidence() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let validation_body = "## Description\n\nFresh evidence closeout body.\n\n## Outcome\n\nThe issue closes immediately after targeted evidence is recorded.\n\n## Evidence\n\n- A disposable workflow transcript records evidence and closes without an intervening commit.";
    let (success, validation_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Fresh evidence closeout",
            "--issue-type",
            "validation",
            "--description",
            validation_body,
        ],
    );
    assert!(success, "validation issue create failed: {stderr}");
    assert!(validation_out.contains("Created issue atelier-"));
    let validation_id = issue_id_by_title(dir.path(), "Fresh evidence closeout");
    commit_all(dir.path(), "fresh evidence closeout baseline");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &validation_id, "start"],
    );
    assert!(success, "start failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &validation_id, "request_review"],
    );
    assert!(success, "request_review failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &validation_id, "request_validation"],
    );
    assert!(success, "request_validation failed: {stderr}");

    attach_pass_evidence(
        dir.path(),
        "issue",
        &validation_id,
        "fresh validation evidence recorded before close",
    );
    let dirty_before_close = git_status_short(dir.path());
    assert!(
        dirty_before_close.contains(".atelier/evidence/"),
        "evidence record should be dirty before close:\n{dirty_before_close}"
    );
    assert!(
        dirty_before_close.contains(&format!(".atelier/issues/{validation_id}.activity/")),
        "evidence activity should be dirty before close:\n{dirty_before_close}"
    );

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "close",
            &validation_id,
            "--reason",
            "fresh evidence accepted",
        ],
    );
    assert!(
        success,
        "issue close should accept freshly recorded evidence without an intervening commit: {stderr}"
    );
    assert!(
        close_out.contains("Applied transition close"),
        "{close_out}"
    );
    assert!(close_out.contains("To:       done"), "{close_out}");
}

#[test]
fn test_validation_issue_closeout_allows_fresh_issue_and_evidence() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    commit_all(dir.path(), "fresh issue closeout tracker baseline");

    let validation_body = "## Description\n\nFresh issue closeout body.\n\n## Outcome\n\nThe issue closes after create/start/validation/evidence without an intervening commit.\n\n## Evidence\n\n- A disposable workflow transcript records the strict create-to-close path.";
    let (success, validation_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Fresh issue evidence closeout",
            "--issue-type",
            "validation",
            "--description",
            validation_body,
        ],
    );
    assert!(success, "validation issue create failed: {stderr}");
    assert!(validation_out.contains("Created issue atelier-"));
    let validation_id = issue_id_by_title(dir.path(), "Fresh issue evidence closeout");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &validation_id, "start"],
    );
    assert!(success, "start failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &validation_id, "request_review"],
    );
    assert!(success, "request_review failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &validation_id, "request_validation"],
    );
    assert!(success, "request_validation failed: {stderr}");

    attach_pass_evidence(
        dir.path(),
        "issue",
        &validation_id,
        "fresh issue validation evidence recorded before close",
    );
    let dirty_before_close = git_status_short(dir.path());
    assert!(
        dirty_before_close.contains(".atelier/issues/"),
        "new issue tracker state should still be untracked before close:\n{dirty_before_close}"
    );
    assert!(
        dirty_before_close.contains(".atelier/evidence/"),
        "evidence record should be dirty before close:\n{dirty_before_close}"
    );

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "close",
            &validation_id,
            "--reason",
            "fresh issue and evidence accepted",
        ],
    );
    assert!(
        success,
        "issue close should accept freshly created issue bookkeeping: {stderr}"
    );
    assert!(
        close_out.contains("Applied transition close"),
        "{close_out}"
    );
    assert!(close_out.contains("To:       done"), "{close_out}");
}

#[test]
fn test_issue_closeout_requires_passing_evidence_records() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let issue_body = "## Description\n\nValidation blocker body.\n\n## Outcome\n\nThe issue does not close without passing evidence.\n\n## Evidence\n\n- A passing transcript proves closeout readiness.";
    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Blocked validation proof",
            "--description",
            issue_body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Blocked validation proof");

    move_issue_to_validation(dir.path(), &issue_id);
    attach_non_pass_evidence(
        dir.path(),
        "issue",
        &issue_id,
        "blocked",
        "blocked validation transcript recorded",
    );

    let (success, transitions, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(transitions.contains("close"));
    assert!(transitions.contains("expected at least 1 passing evidence record"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "still blocked"],
    );
    assert!(
        !success,
        "closeout must reject evidence that is attached but not passing"
    );
    assert!(stderr.contains("expected at least 1 passing evidence record"));
}

#[test]
fn test_mission_status_shows_ignored_product_behavior_closeout_blocker() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "create", "Ignored blocker mission"],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Ignored blocker mission");
    let mission_id = mission_id.as_str();

    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Finished behavior work"]);
    assert!(success, "work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Finished behavior work");
    let work_id = work_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, work_id]);
    assert!(success, "mission add-work failed: {stderr}");
    close_issue_with_evidence(dir.path(), work_id, Some("done"));

    let (success, followup_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Resolve ignored behavior test"],
    );
    assert!(success, "follow-up create failed: {stderr}");
    assert!(followup_out.contains("Created issue atelier-"));
    let followup_id = issue_id_by_title(dir.path(), "Resolve ignored behavior test");
    let followup_id = followup_id.as_str();

    std::fs::create_dir_all(dir.path().join("tests")).unwrap();
    std::fs::write(
        dir.path().join("tests/product_gap.rs"),
        ignored_test_source(
            &format!(
                "ignore = \"reason: product behavior pending migration; issue: {followup_id}; product: yes\""
            ),
            "ignored_product_closeout_gap",
        ),
    )
    .unwrap();

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "ignored blocker evidence",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - ignored blocker evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "ignored blocker evidence");
    let evidence_id = evidence_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", evidence_id, "mission", mission_id],
    );
    assert!(success, "evidence attach failed: {stderr}");
    commit_all(dir.path(), "ignored test closeout blocker");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Closeout: blocked"));
    assert!(status_out.contains("Ignored Test Review: needed"));
    assert!(status_out.contains("Advanced Validator Detail"));
    assert!(status_out.contains("fail  ignored_tests_reviewed"));
    assert!(status_out.contains("ignored_product_closeout_gap"));
    assert!(status_out.contains(followup_id));
    assert!(status_out.contains("ignored product-behavior test is still blocking closeout"));
    assert!(!status_out.contains(&format!(
        "atelier mission update {mission_id} --status closed"
    )));
}

#[test]
fn test_mission_closeout_blocks_undeferred_obsolete_command_test() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    write_valid_command_guidance(dir.path());

    fs::create_dir_all(dir.path().join("tests")).unwrap();
    fs::write(
        dir.path().join("tests/legacy_session.rs"),
        concat!(
            "#[test]\n",
            "fn legacy_session_still_works() {\n",
            "    let (success, _, _) = run_atelier(dir.path(), &[\"session\", \"start\"]);\n",
            "    assert!(success);\n",
            "}\n"
        ),
    )
    .unwrap();

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Stale test closeout"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Stale test closeout");

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "stale test evidence",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - stale test evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "stale test evidence");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "mission", &mission_id],
    );
    assert!(success, "evidence attach failed: {stderr}");
    commit_all(dir.path(), "stale test closeout baseline");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["mission", "close", &mission_id, "--reason", "done"],
    );

    assert!(
        !success,
        "mission closeout should block undeferred obsolete-command tests"
    );
    assert!(stdout.contains("Mission closeout blocked"));
    assert!(stdout.contains("docs/help drift: detected"));
    assert!(stdout.contains("update docs, help text, or command-surface tests"));
    assert!(stdout.contains("tests/legacy_session.rs"));
    assert!(stdout.contains("legacy_session_still_works"));
    assert!(stdout.contains("atelier session start"));
    assert!(stderr.contains("mission closeout blocked"));
}

#[test]
fn test_mission_audit_reports_shell_closeout_and_explicit_approval() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Approval audit",
            "--validation",
            "Human guidance only.",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Approval audit");
    let mission_id = mission_id.as_str();

    let (success, missing_out, stderr) = run_atelier(dir.path(), &["mission", "audit", mission_id]);
    assert!(!success, "audit without work should fail");
    assert!(missing_out.contains("Mission Closeout Audit"));
    assert!(missing_out.contains("[missing]"));
    assert!(missing_out.contains("No linked mission work exists."));
    assert!(stderr.contains("mission closeout audit found unresolved items"));

    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Finished mission work"]);
    assert!(success, "work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Finished mission work");
    let work_id = work_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, work_id]);
    assert!(success, "mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), work_id, Some("done"));

    let (success, approval_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Mission approval validation",
            "--issue-type",
            "validation",
            "--description",
            "## Description\n\nValidation item body.\n\n## Outcome\n\nMission approval is represented as linked workflow state.\n\n## Evidence\n\n- `atelier evidence show <id>` displays the approval record attached to this issue.",
        ],
    );
    assert!(success, "validation issue create failed: {stderr}");
    assert!(approval_out.contains("Created issue atelier-"));
    let approval_id = issue_id_by_title(dir.path(), "Mission approval validation");
    let approval_id = approval_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", mission_id, approval_id],
    );
    assert!(success, "mission add validation failed: {stderr}");

    let (success, closeout_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", "--closeout", mission_id]);
    assert!(
        !success,
        "closeout view should fail while approval work is open"
    );
    assert!(closeout_out.contains("Mission Closeout Audit"));
    assert!(closeout_out.contains("Workflow Approval"));
    assert!(closeout_out.contains(approval_id));
    assert!(closeout_out.contains("Linked validation work is still"));
    assert!(stderr.contains("mission closeout audit found unresolved items"));

    move_issue_to_validation(dir.path(), approval_id);
    attach_pass_evidence(
        dir.path(),
        "issue",
        approval_id,
        "independent mission approval captured",
    );
    commit_all(dir.path(), "mission approval ready");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", approval_id, "--reason", "approved"],
    );
    assert!(success, "validation issue close failed: {stderr}");
    commit_all(dir.path(), "approval audit ready");

    let (success, ready_out, stderr) = run_atelier(dir.path(), &["mission", "audit", mission_id]);
    assert!(success, "ready audit should pass: {stderr}");
    assert!(ready_out.contains("Mission Closeout Audit"));
    assert!(ready_out.contains("[covered]"));
    assert!(ready_out.contains("Workflow approval closed via linked validation work."));
}

#[test]
fn test_mission_closeout_accepts_shell_mission_without_direct_mission_evidence() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Shell closeout",
            "--validation",
            "Human guidance only.",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Shell closeout");
    let mission_id = mission_id.as_str();

    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Shell mission work"]);
    assert!(success, "work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Shell mission work");
    let work_id = work_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, work_id]);
    assert!(success, "mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), work_id, Some("done"));
    commit_all(dir.path(), "shell mission ready");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Direct mission evidence: none"));
    assert!(status_out.contains("Closeout: ready"));
    assert!(status_out.contains("All required closeout gates pass."));

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "close",
            mission_id,
            "--reason",
            "linked work closed",
        ],
    );
    assert!(
        success,
        "mission closeout should succeed without direct mission evidence: {stderr}"
    );
    assert!(close_out.contains("Status: closed"));
}
