use super::*;

// ==================== Issue Creation Tests ====================

#[test]
fn test_create_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "create", "Test issue"]);

    assert!(success);
    assert!(
        stdout.contains("Created issue atelier-"),
        "Expected project-scoped issue id in output, got: {}",
        stdout
    );
    let issue_id = issue_id_by_title(dir.path(), "Test issue");
    assert!(stdout.contains(&format!(".atelier/issues/{issue_id}.md")));
    assert!(stdout.contains(&format!("atelier lint {issue_id}")));
}

#[test]
fn test_create_issue_with_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "create", "High priority issue", "-p", "high"],
    );

    assert!(success);

    // Verify it was created with correct priority
    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(list_out.contains("high"));
}

#[test]
fn test_create_issue_with_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Issue with desc",
            "-d",
            "Detailed description here",
        ],
    );

    assert!(success);

    // Verify description in show
    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("Detailed description"));
}

#[test]
fn test_create_subissue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent issue"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "subissue", "1", "Child issue"]);

    assert!(success);
    assert!(
        stdout.contains("Created subissue atelier-"),
        "Expected project-scoped subissue id in output, got: {}",
        stdout
    );
    let child_id = issue_id_by_title(dir.path(), "Child issue");
    assert!(stdout.contains(&format!(".atelier/issues/{child_id}.md")));
    assert!(stdout.contains(&format!("atelier lint {child_id}")));

    // Verify parent-child relationship in show
    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("Child") || show_out.contains("subissue"));
}

#[test]
fn test_create_issue_rejects_work_flag() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Work path issue", "--work"],
    );
    assert!(!success, "issue create --work should be rejected");
    assert!(stderr.contains("unexpected argument '--work'"));
}

// ==================== Issue Listing Tests ====================

#[test]
fn test_list_empty() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);

    assert!(success);
    assert!(
        stdout.contains("No issues found."),
        "Expected 'No issues found.' in output, got: {}",
        stdout
    );
}

#[test]
fn test_list_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    run_atelier(
        dir.path(),
        &["issue", "subissue", "1", "Issue 2", "-p", "high"],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);

    assert!(success);
    assert!(stdout.contains("Issue Queue"));
    assert!(stdout.contains("2 total"));
    assert!(stdout.contains("atelier-"));
    assert!(stdout.contains("[task] atelier-"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));

    let (success, quiet_out, stderr) = run_atelier(dir.path(), &["--quiet", "issue", "list"]);
    assert!(success, "quiet issue list failed: {stderr}");
    assert!(!quiet_out.contains("Issue Queue"));
    assert!(!quiet_out.contains("Issue 1"));
    assert_eq!(quiet_out.lines().count(), 2);
    assert!(quiet_out.lines().all(|line| line.starts_with("atelier-")));
}

#[test]
fn test_list_filter_by_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Open issue"]);
    run_atelier(dir.path(), &["issue", "create", "Closed issue"]);
    close_issue_with_evidence(dir.path(), "2", None);

    let (_, open_list, _) = run_atelier(dir.path(), &["issue", "list", "-s", "todo"]);
    assert!(open_list.contains("Open issue"));
    assert!(!open_list.contains("Closed issue"));

    let (_, closed_list, _) = run_atelier(dir.path(), &["issue", "list", "-s", "done"]);
    assert!(closed_list.contains("Closed issue"));
    assert!(!closed_list.contains("Open issue"));
}

#[test]
fn test_list_filter_by_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Bug issue"]);
    run_atelier(dir.path(), &["issue", "create", "Feature issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);
    run_atelier(dir.path(), &["issue", "label", "2", "feature"]);

    let (_, bug_list, _) = run_atelier(dir.path(), &["issue", "list", "-l", "bug"]);
    assert!(bug_list.contains("Bug issue"));
    assert!(!bug_list.contains("Feature issue"));
}

// ==================== Issue Show Tests ====================

#[test]
fn test_show_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = "## Description\n\nDescription\n\n## Outcome\n\nThe issue show command renders parsed sections.\n\n## Evidence\n\n- Show output contains the section headings.\n\n## Notes\n\nCLI display context.";

    run_atelier(dir.path(), &["issue", "create", "Test issue", "-d", body]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("Test issue"));
    assert!(stdout.contains("Description"));
    assert!(stdout.contains("Outcome"));
    assert!(stdout.contains("The issue show command renders parsed sections."));
    assert!(stdout.contains("Evidence"));
    assert!(stdout.contains("- Show output contains the section headings."));
    assert!(stdout.contains("Notes"));
    assert!(stdout.contains("CLI display context."));
    assert!(!stdout.contains("Acceptance Criteria"));
}

#[test]
fn test_issue_commands_accept_partial_issue_key() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Partial key issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Partial key issue");
    let key = issue_key(&issue_id);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", key]);

    assert!(success, "show by partial key failed: {stderr}");
    assert!(stdout.contains(&issue_id));
    assert!(stdout.contains("Partial key issue"));
}

#[test]
fn test_issue_reference_surfaces_accept_partial_issue_keys() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent key issue"]);
    run_atelier(dir.path(), &["issue", "create", "Related key issue"]);
    let parent_id = issue_id_by_title(dir.path(), "Parent key issue");
    let related_id = issue_id_by_title(dir.path(), "Related key issue");
    let parent_key = issue_key(&parent_id);
    let related_key = issue_key(&related_id);

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "subissue", parent_key, "Child key issue"],
    );
    assert!(success, "subissue by partial key failed: {stderr}");
    assert!(stdout.contains(&parent_id));
    let child_id = issue_id_by_title(dir.path(), "Child key issue");
    assert!(!child_id.is_empty());

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "relate", parent_key, related_key]);
    assert!(success, "relate by partial keys failed: {stderr}");
    assert!(stdout.contains(&parent_id));
    assert!(stdout.contains(&related_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "related", parent_key]);
    assert!(success, "related by partial key failed: {stderr}");
    assert!(stdout.contains(&related_id));

    migrate_default_issue_workflow(dir.path());
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", parent_key, "--options"],
    );
    assert!(
        success,
        "transition options by partial key failed: {stderr}"
    );
    assert!(stdout.contains(&format!("Issue Transitions {parent_id}")));
}

#[test]
fn test_bulk_plan_apply_accepts_partial_issue_key_refs() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Existing issue target"]);
    let issue_id = issue_id_by_title(dir.path(), "Existing issue target");
    let issue_key = issue_key(&issue_id);
    let bulk_path = dir.path().join("partial-key-plan.json");
    std::fs::write(
        &bulk_path,
        format!(
            r#"{{
  "schema": "atelier.bulk-plan",
  "schema_version": 1,
  "title": "Partial key bulk apply",
  "apply": {{ "export": "auto" }},
  "records": {{
    "issues": [
      {{
        "client_ref": "issue.partial",
        "title": "Partial key dependent",
        "issue_type": "task",
        "priority": "medium",
        "depends_on": [{{ "id": "{issue_key}" }}]
      }}
    ]
  }}
}}"#
        ),
    )
    .unwrap();

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["plan", "apply", bulk_path.to_str().unwrap()]);
    assert!(success, "bulk apply by partial issue key failed: {stderr}");
    assert!(stdout.contains("Bulk plan applied."));

    let dependent_id = issue_id_by_title(dir.path(), "Partial key dependent");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "blocked", &dependent_id]);
    assert!(success, "issue blocked failed: {stderr}");
    assert!(stdout.contains(&issue_id));
}

#[test]
fn test_show_issue_rich_human_output() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent issue", "-p", "high"],
    );
    run_atelier(
        dir.path(),
        &["issue", "subissue", "1", "Target issue", "-p", "medium"],
    );
    run_atelier(
        dir.path(),
        &["issue", "subissue", "2", "Child issue", "-p", "low"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Blocking issue", "-p", "high"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Downstream issue", "-p", "low"],
    );
    run_atelier(dir.path(), &["issue", "block", "2", "4"]);
    run_atelier(dir.path(), &["issue", "block", "5", "2"]);
    run_atelier(dir.path(), &["issue", "note", "2", "Recent note"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "2"]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Target issue"));
    assert!(stdout.contains("Status:   todo"));
    assert!(stdout.contains("Type:     task"));
    assert!(stdout.contains("Priority: medium"));
    let target_id = issue_id_by_title(dir.path(), "Target issue");
    assert!(stdout.contains(&format!(".atelier/issues/{target_id}.md")));
    assert!(stdout.contains("Parent issue"));
    assert!(stdout.contains("1 total | status: todo=1 | priority: low=1"));
    assert!(stdout.contains("Blocking issue"));
    assert!(stdout.contains("(open blocker)"));
    assert!(stdout.contains("Downstream issue"));
    assert!(stdout.contains("Recent Activity"));
    assert!(stdout.contains("Recent note"));
    assert!(stdout.contains("Next Commands"));
    assert!(stdout.contains("atelier issue note"));
    assert!(!stdout.contains("atelier issue comment"));
    assert!(stdout.contains("atelier issue transition"));
}

#[test]
fn test_issue_show_human_shape_exposes_actionable_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "JSON issue", "-d", "JSON description"],
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("JSON issue"));
    assert!(stdout.contains("Description"));
    assert!(stdout.contains("JSON description"));
    assert!(stdout.contains("Blocked by"));
    assert!(stdout.contains("Blocking"));
    assert!(stdout.contains("Recent Activity"));
    assert!(stdout.contains("Next Commands"));
}

#[test]
fn test_issue_show_reads_detail_body_from_record_store() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Canonical detail issue",
            "-d",
            "Canonical Markdown body",
        ],
    );
    let issue_id = issue_id_by_title(dir.path(), "Canonical detail issue");
    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    conn.execute(
        "UPDATE issues SET description = 'SQLite shadow body' WHERE id = ?1",
        [&issue_id],
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Canonical Markdown body"));
    assert!(!stdout.contains("SQLite shadow body"));
}

#[test]
fn test_issue_sections_are_canonical_after_direct_markdown_edit_and_rebuild() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Canonical section source",
            "-d",
            "Original section text",
        ],
    );
    let issue_id = issue_id_by_title(dir.path(), "Canonical section source");
    let issue_path = dir
        .path()
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.md"));
    let edited_body = "Edited direct Markdown section";
    let edited_outcome = "Direct edits are projected from issue body sections.";
    let edited_evidence = "- `atelier rebuild` refreshes derived search text.";
    let issue_text = std::fs::read_to_string(&issue_path)
        .unwrap()
        .replace("Original section text", edited_body)
        .replace("Outcome was not specified.", edited_outcome)
        .replace("Evidence was not specified.", edited_evidence);
    std::fs::write(&issue_path, issue_text).unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains(edited_body), "{stdout}");
    assert!(stdout.contains(edited_outcome), "{stdout}");
    assert!(stdout.contains(edited_evidence), "{stdout}");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["search", "projected from issue body"]);
    assert!(success, "search failed: {stderr}");
    assert!(stdout.contains(&issue_id), "{stdout}");

    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let projected_text: String = conn
        .query_row(
            "SELECT description FROM issues WHERE id = ?1",
            [&issue_id],
            |row| row.get(0),
        )
        .unwrap();
    assert!(projected_text.contains(edited_body));
    assert!(!projected_text.contains(edited_outcome));
    assert!(!projected_text.contains("## Description"));
}

#[test]
fn test_first_class_detail_views_read_payloads_from_record_store() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Canonical mission",
            "--body",
            "Canonical mission body",
            "--constraint",
            "Canonical constraint",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Canonical mission");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "plan",
            "create",
            "Canonical plan",
            "--body",
            "Canonical plan body",
        ],
    );
    assert!(success, "plan create failed: {stderr}");
    let plan_id = record_id_by_title(dir.path(), "plans", "Canonical plan");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--result",
            "pass",
            "Canonical evidence summary",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    let evidence_id = record_id_by_title(dir.path(), "evidence", "Canonical evidence summary");

    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    conn.execute(
        "UPDATE records SET body = 'SQLite mission body', data_json = ?1 WHERE id = ?2",
        [
            r#"{"constraints":["SQLite constraint"],"risks":[],"validation":[],"milestones":[],"plans":[],"evidence":[],"work":[]}"#,
            mission_id.as_str(),
        ],
    )
    .unwrap();
    conn.execute(
        "UPDATE records SET body = 'SQLite plan body', data_json = ?1 WHERE id = ?2",
        [
            r#"{"revision":99,"revisions":[{"revision":99,"reason":"sqlite","body":"SQLite plan body"}]}"#,
            plan_id.as_str(),
        ],
    )
    .unwrap();
    conn.execute(
        "UPDATE records SET body = 'SQLite evidence summary', data_json = ?1 WHERE id = ?2",
        [
            r#"{"kind":"sqlite","result":"fail","path":null,"uri":null,"producer":null,"captured_at":"2000-01-01T00:00:00Z"}"#,
            evidence_id.as_str(),
        ],
    )
    .unwrap();

    let (success, mission_out, stderr) = run_atelier(dir.path(), &["mission", "show", &mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(mission_out.contains("Canonical mission body"));
    assert!(mission_out.contains("Canonical constraint"));
    assert!(!mission_out.contains("SQLite mission body"));
    assert!(!mission_out.contains("SQLite constraint"));

    let (success, plan_out, stderr) = run_atelier(dir.path(), &["plan", "show", &plan_id]);
    assert!(success, "plan show failed: {stderr}");
    assert!(plan_out.contains("Canonical plan body"));
    assert!(plan_out.contains("Revision: 1"));
    assert!(!plan_out.contains("SQLite plan body"));
    assert!(!plan_out.contains("Revision: 99"));

    let (success, evidence_out, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &evidence_id]);
    assert!(success, "evidence show failed: {stderr}");
    assert!(evidence_out.contains("Canonical evidence summary"));
    assert!(evidence_out.contains("Result:      pass"));
    assert!(evidence_out.contains("Kind:        test"));
    assert!(!evidence_out.contains("SQLite evidence summary"));
    assert!(!evidence_out.contains("Kind:        sqlite"));
}

#[test]
fn test_issue_search_reads_payloads_from_record_store_and_activity() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Canonical search issue",
            "-d",
            "canonical body needle",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Canonical search issue");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "comment", &issue_id, "canonical activity needle"],
    );
    assert!(success, "issue comment failed: {stderr}");

    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    conn.execute(
        "UPDATE issues SET description = 'sqlite body needle' WHERE id = ?1",
        [&issue_id],
    )
    .unwrap();

    let (success, body_out, stderr) =
        run_atelier(dir.path(), &["issue", "search", "canonical body needle"]);
    assert!(success, "canonical body search failed: {stderr}");
    assert!(body_out.contains("Canonical search issue"));

    let (success, activity_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "search", "canonical activity needle"],
    );
    assert!(success, "canonical activity search failed: {stderr}");
    assert!(activity_out.contains("Canonical search issue"));

    let (success, shadow_body_out, stderr) =
        run_atelier(dir.path(), &["issue", "search", "sqlite body needle"]);
    assert!(success, "sqlite shadow body search failed: {stderr}");
    assert!(shadow_body_out.contains("No issues found"));
}

#[test]
fn test_show_closed_issue_includes_close_reason() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Closed issue"]);
    close_issue_with_evidence(dir.path(), "1", Some("Done enough"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success, "issue show failed: {stderr}");
    assert!(stdout.contains("Closed issue"));
    assert!(stdout.contains("Closed:"));
    assert!(stdout.contains("Close Reason"));
    assert!(stdout.contains("Done enough"));
}

#[test]
fn test_show_issue_prefers_activity_sidecars_for_recent_activity() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Activity issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Activity issue");
    let activity_dir = dir
        .path()
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.activity"));
    std::fs::create_dir_all(&activity_dir).unwrap();
    std::fs::write(
        activity_dir.join("20260610T181920123456Z.md"),
        format!(
            "---\nschema: \"atelier.activity\"\nschema_version: 1\nid: \"20260610T181920123456Z\"\nsubject_kind: \"issue\"\nsubject_id: \"{issue_id}\"\nevent_type: \"comment\"\nactor: \"tester\"\ncreated_at: \"2026-06-10T18:19:20.123456Z\"\nsummary: \"Canonical activity\"\n---\n\nSidecar body\n"
        ),
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Canonical activity"));
    assert!(stdout.contains("Sidecar body"));
    assert!(!stdout.contains("Legacy note"));
}

#[test]
fn test_history_repo_wide_supports_filters_bounded_output_and_drill_downs() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "First issue"]);
    run_atelier(dir.path(), &["issue", "create", "Second issue"]);
    let first = issue_id_by_title(dir.path(), "First issue");
    let second = issue_id_by_title(dir.path(), "Second issue");
    write_activity_fixture(
        dir.path(),
        &first,
        "20260610T181920123456Z",
        "comment",
        "First comment",
        "First body",
    );
    write_activity_fixture(
        dir.path(),
        &second,
        "20260610T181921123456Z",
        "evidence_attached",
        "Evidence attached",
        "evidence_id: \"ev-1\"\nresult: \"pass\"",
    );
    write_activity_fixture(
        dir.path(),
        &second,
        "20260610T181922123456Z",
        "comment",
        "Second comment",
        "Second body",
    );

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--event-kind",
            "evidence_attached",
            "--limit",
            "1",
        ],
    );
    assert!(success, "history failed: {stderr}");
    assert!(stdout.contains("History"));
    assert!(stdout.contains("Scope:          repository"));
    assert!(stdout.contains("Source:         canonical .atelier"));
    assert!(stdout.contains("Ordering:       newest first"));
    assert!(stdout.contains("Showing:        1 of 1 matching events"));
    assert!(stdout.contains("Evidence attached"));
    assert!(!stdout.contains("First comment"));
    assert!(stdout.contains("Next Commands"));
    assert!(stdout.contains("atelier issue show <id>"));
    assert!(stdout.contains("atelier history --mission <id>"));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            first.as_str(),
            "--event-kind",
            "comment",
            "--since",
            "2026-06-10",
        ],
    );
    assert!(success, "filtered history failed: {stderr}");
    assert!(stdout.contains("First comment"));
    assert!(!stdout.contains("Evidence attached"));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["history", "--event-kind", "comment", "--limit", "1"],
    );
    assert!(success, "history failed: {stderr}");
    assert!(stdout.contains("Second comment"));
    assert!(!stdout.contains("First comment"));
    assert!(stdout.contains("Omitted:"));
}

#[test]
fn test_history_mission_scope_includes_linked_work_descendants_and_evidence() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "History mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "History mission");

    run_atelier(
        dir.path(),
        &["issue", "create", "History epic", "--issue-type", "epic"],
    );
    let epic_id = issue_id_by_title(dir.path(), "History epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "subissue", &epic_id, "History child"],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "History child");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &epic_id]);
    assert!(success, "mission add-work failed: {stderr}");
    let (success, note_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "note", &mission_id, "Mission note body"],
    );
    assert!(success, "mission note failed: {stderr}");
    assert!(note_out.contains("Added note to mission"));
    write_activity_fixture(
        dir.path(),
        &child_id,
        "20260610T191920123456Z",
        "note",
        "Child note",
        "Child body",
    );
    let (success, _evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--result",
            "pass",
            "Cargo test passed",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    let evidence_id = record_id_by_title(dir.path(), "evidence", "Cargo test passed");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            &evidence_id,
            "issue",
            child_id.as_str(),
        ],
    );
    assert!(success, "evidence attach failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--mission",
            mission_id.as_str(),
            "--event-kind",
            "evidence_attached",
        ],
    );

    assert!(success, "history failed: {stderr}");
    assert!(stdout.contains(&format!("Scope:          mission {mission_id}")));
    assert!(stdout.contains(&format!("Attached evidence {evidence_id}")));
    assert!(stdout.contains(&child_id));
    assert!(stdout.contains(&format!("atelier mission show {mission_id}")));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--mission",
            mission_id.as_str(),
            "--event-kind",
            "note",
        ],
    );
    assert!(success, "mission note history failed: {stderr}");
    assert!(stdout.contains("Mission note body"));
    assert!(stdout.contains(&mission_id));
    assert!(stdout.contains("Child note"));
    assert!(stdout.contains(&child_id));
}

#[test]
fn test_history_issue_scope_defaults_single_issue_and_can_include_descendants() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent history"]);
    let parent_id = issue_id_by_title(dir.path(), "Parent history");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "subissue", &parent_id, "Child history"],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Child history");
    write_activity_fixture(
        dir.path(),
        &parent_id,
        "20260610T181920123456Z",
        "note",
        "Parent note",
        "Parent body",
    );
    write_activity_fixture(
        dir.path(),
        &child_id,
        "20260610T181921123456Z",
        "note",
        "Child note",
        "Child body",
    );

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            parent_id.as_str(),
            "--event-kind",
            "note",
        ],
    );
    assert!(success, "issue history failed: {stderr}");
    assert!(stdout.contains(&format!("Scope:          issue {parent_id}")));
    assert!(stdout.contains("Parent note"));
    assert!(!stdout.contains("Child note"));
    assert!(stdout.contains(&format!(
        "atelier history --issue {parent_id} --include-descendants"
    )));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            parent_id.as_str(),
            "--include-descendants",
            "--event-kind",
            "note",
        ],
    );
    assert!(success, "descendant issue history failed: {stderr}");
    assert!(stdout.contains("Parent note"));
    assert!(stdout.contains("Child note"));
}

#[test]
fn test_history_empty_states_and_invalid_limit() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["history"]);
    assert!(success, "empty history failed: {stderr}");
    assert!(stdout.contains("No canonical history found for repository."));
    assert!(stdout.contains("Source:"));
    assert!(stdout.contains("Next Commands"));

    run_atelier(dir.path(), &["issue", "create", "Filtered history"]);
    let issue_id = issue_id_by_title(dir.path(), "Filtered history");
    write_activity_fixture(
        dir.path(),
        &issue_id,
        "20260610T181920123456Z",
        "note",
        "Filter note",
        "Filter body",
    );
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            issue_id.as_str(),
            "--event-kind",
            "evidence_attached",
        ],
    );
    assert!(success, "filtered empty history failed: {stderr}");
    assert!(stdout.contains("History exists for"));
    assert!(stdout.contains("no events matched the current filters"));

    let (success, _, stderr) = run_atelier(dir.path(), &["history", "--limit", "0"]);
    assert!(!success, "zero limit should fail");
    assert!(stderr.contains("--limit must be greater than 0"));
}

#[test]
fn test_show_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", "999"]);

    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

fn write_activity_fixture(
    dir: &Path,
    issue_id: &str,
    activity_id: &str,
    event_type: &str,
    summary: &str,
    body: &str,
) {
    let activity_dir = dir
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.activity"));
    std::fs::create_dir_all(&activity_dir).unwrap();
    std::fs::write(
        activity_dir.join(format!("{activity_id}.md")),
        format!(
            "---\nschema: \"atelier.activity\"\nschema_version: 1\nid: \"{activity_id}\"\nsubject_kind: \"issue\"\nsubject_id: \"{issue_id}\"\nevent_type: \"{event_type}\"\nactor: \"tester\"\ncreated_at: \"{}-{}-{}T{}:{}:{}.123456Z\"\nsummary: \"{summary}\"\n---\n\n{body}\n",
            &activity_id[0..4],
            &activity_id[4..6],
            &activity_id[6..8],
            &activity_id[9..11],
            &activity_id[11..13],
            &activity_id[13..15],
        ),
    )
    .unwrap();
}

// ==================== Issue Update Tests ====================

#[test]
fn test_update_issue_title() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Original title"]);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "update", "1", "--title", "Updated title"],
    );

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("Updated title"));
}

#[test]
fn test_update_issue_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue", "-p", "low"]);
    run_atelier(dir.path(), &["issue", "update", "1", "-p", "critical"]);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("critical"));
}

#[test]
fn test_update_issue_remove_label_replaces_unlabel_helper() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Label lifecycle", "--label", "keep-me"],
    );
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            "1",
            "--label",
            "remove-me",
            "--remove-label",
            "keep-me",
        ],
    );
    assert!(success, "update label replacement failed: {stderr}");
    assert!(stdout.contains("Updated issue"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    let labels_line = show_out
        .lines()
        .find(|line| line.starts_with("Labels:"))
        .unwrap_or("");
    assert!(labels_line.contains("remove-me"), "{show_out}");
    assert!(!labels_line.contains("keep-me"), "{show_out}");
}

// ==================== Issue Close/Reopen Tests ====================

#[test]
fn test_close_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    let issue_id = close_issue_with_evidence(dir.path(), "1", Some("done"));
    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(show_out.contains("Status:   done"), "{show_out}");
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_close_all_is_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Close all one"]);
    assert!(success, "first create failed: {stderr}");
    let first_id = close_issue_with_evidence(dir.path(), "1", Some("done"));
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Close all two"]);
    assert!(success, "second create failed: {stderr}");
    let second_id = close_issue_with_evidence(dir.path(), "2", Some("done"));

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "close-all"]);
    assert!(success, "close-all failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after close-all: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    for issue_id in [first_id, second_id] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
        assert!(success, "show failed for {issue_id}: {stderr}");
        assert!(stdout.contains("Status:   done"), "{stdout}");
    }
}

#[test]
fn test_import_beads_jsonl_fixture_round_trip() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let fixture_path = dir.path().join("issues.manual.jsonl");
    std::fs::write(
        &fixture_path,
        include_str!("../fixtures/beads/issues.manual.jsonl"),
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["import-beads", fixture_path.to_str().unwrap()],
    );
    assert!(success, "import-beads failed: {stderr}");
    assert!(stdout.contains("source records: 3"));
    assert!(stdout.contains("imported issues: 3"));
    assert!(stdout.contains("parent-child relationships: 2"));
    assert!(stdout.contains("blocking relationships: 1"));
    assert!(dir
        .path()
        .join(".atelier")
        .join("issues")
        .join("atelier-0001.md")
        .exists());

    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(list_out.contains("Mission: Replace Beads"));
    assert!(list_out.contains("Dogfood Atelier"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "3"]);
    assert!(show_out.contains("Parent: atelier-0001"));
    assert!(show_out.contains("Blocked by"));
    assert!(show_out.contains("atelier-0002"));
    assert!(show_out.contains("(open blocker)"));
    assert!(show_out.contains("Outcome"));
    assert!(show_out.contains("AGENTFACTORY.md declares Atelier as the tracker"));
    assert!(show_out.contains("Evidence"));
    assert!(show_out.contains("atelier import-beads <path>"));
    assert!(!show_out.contains("Acceptance Criteria"));

    let (updated, _, update_err) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            "2",
            "--title",
            "Imported Beads issue updated",
        ],
    );
    assert!(updated, "update failed: {update_err}");
    close_issue_with_evidence(dir.path(), "2", None);

    let (_, closed_show, _) = run_atelier(dir.path(), &["issue", "show", "2"]);
    assert!(closed_show.contains("Imported Beads issue updated"));
    assert!(closed_show.contains("Status:   done"));

    let (fresh, _, fresh_err) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(
        fresh,
        "export --check validates canonical Markdown/projection state, not SQLite-only drift: {fresh_err}"
    );
}

// ==================== Issue Delete Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_delete_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "To delete"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "delete", "1", "-f"]);

    assert!(success);

    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(!list_out.contains("To delete"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_delete_issue_is_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Durable delete"]);
    assert!(success, "create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "delete", &issue_id, "-f"]);
    assert!(success, "delete failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after delete: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(!success, "deleted issue still exists after rebuild");
    assert!(stderr.contains("was not found"), "{stderr}");
}

// ==================== Labels Tests ====================

#[test]
fn test_add_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "label", "1", "bug"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("bug"));
}

#[test]
fn test_remove_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "unlabel", "1", "bug"]);

    assert!(success);

    let issue_id = issue_id_by_title(dir.path(), "Test issue");
    let issue_text = read_canonical_record(dir.path(), "issues", &issue_id);
    assert!(
        issue_text.contains("labels: []"),
        "removed label should not remain in canonical labels:\n{issue_text}"
    );
}

// ==================== Comments Tests ====================

#[test]
fn test_add_comment() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "comment", "1", "This is a comment"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("This is a comment"));
}

#[test]
fn test_issue_mutations_create_activity_sidecars() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Activity issue"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Activity issue");
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    for (kind, body) in [
        ("human", "Plain comment body"),
        ("note", "Operator note body"),
        ("plan", "Plan body"),
        ("handoff", "Handoff body"),
    ] {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["issue", "comment", &issue_id, body, "--kind", kind],
        );
        assert!(success, "issue comment {kind} failed: {stderr}");
    }

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "comment",
            &issue_id,
            "Invalid body",
            "--kind",
            "decision",
        ],
    );
    assert!(!success, "invalid comment kind should be rejected");
    assert!(stderr.contains("Invalid comment kind 'decision'"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "note", &issue_id, "Append note body"],
    );
    assert!(success, "issue note failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "update", &issue_id, "--claim"]);
    assert!(!success, "issue claim should be rejected");
    assert!(stderr.contains("unexpected argument '--claim'"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &issue_id,
            "--title",
            "Activity issue renamed",
            "--priority",
            "high",
            "--label",
            "activity-label",
        ],
    );
    assert!(success, "issue update fields failed: {stderr}");

    move_issue_to_validation(dir.path(), &issue_id);
    attach_issue_pass_evidence(dir.path(), &issue_id);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "Close reason body"],
    );
    assert!(success, "issue close reason failed: {stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(&activities, "comment", &["Plain comment body"]);
    assert_activity_contains(&activities, "note", &["Operator note body"]);
    assert_activity_contains(&activities, "note", &["Append note body"]);
    assert_activity_contains(&activities, "plan", &["Plan body"]);
    assert_activity_contains(&activities, "handoff", &["Handoff body"]);
    assert_activity_contains(
        &activities,
        "field_changed",
        &[
            "field: \"title\"",
            "old: \"Activity issue\"",
            "new: \"Activity issue renamed\"",
        ],
    );
    assert_activity_contains(
        &activities,
        "field_changed",
        &["field: \"priority\"", "old: \"medium\"", "new: \"high\""],
    );
    assert_activity_contains(
        &activities,
        "field_changed",
        &["field: \"labels\"", "new: \"activity-label\""],
    );
    assert_activity_contains(
        &activities,
        "transition_applied",
        &["transition: \"close\"", "to: \"done\""],
    );
    assert_activity_contains(&activities, "close_reason", &["Close reason body"]);
}

#[test]
fn test_issue_show_json_recovers_activity_fields_after_rebuild() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Rebuild activity"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Rebuild activity");
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "comment",
            &issue_id,
            "Canonical comment",
            "--kind",
            "human",
        ],
    );
    assert!(success, "comment failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "note", &issue_id, "Canonical handoff"],
    );
    assert!(success, "issue note failed: {stderr}");
    move_issue_to_validation(dir.path(), &issue_id);
    attach_issue_pass_evidence(dir.path(), &issue_id);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "Canonical close"],
    );
    assert!(success, "close failed: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Canonical handoff"));
    assert!(stdout.contains("Close Reason"));
    assert!(stdout.contains("Canonical close"));
}

#[test]
fn test_issue_create_is_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Create-only durable",
            "--description",
            "Created body",
            "--priority",
            "high",
        ],
    );
    assert!(success, "create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Create-only durable");

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after create: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed after rebuild: {stderr}");
    assert!(stdout.contains("Create-only durable"));
    assert!(stdout.contains("Created body"));
    assert!(stdout.contains("Priority: high"));
}

#[test]
fn test_issue_mutations_are_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Mutation source"]);
    assert!(success, "source create failed: {stderr}");
    let source_id = issue_id_by_title(dir.path(), "Mutation source");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Mutation target"]);
    assert!(success, "target create failed: {stderr}");
    let target_id = issue_id_by_title(dir.path(), "Mutation target");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &source_id,
            "--title",
            "Mutation source updated",
            "--priority",
            "high",
        ],
    );
    assert!(success, "update failed: {stderr}");

    for args in [
        vec!["issue", "label", &source_id, "remove-me"],
        vec!["issue", "unlabel", &source_id, "remove-me"],
        vec!["issue", "label", &source_id, "keep-me"],
        vec!["issue", "block", &source_id, &target_id],
        vec!["issue", "unblock", &source_id, &target_id],
        vec!["issue", "block", &source_id, &target_id],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "{args:?} failed: {stderr}");
    }

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed before rebuild: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &source_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Mutation source updated"));
    assert!(stdout.contains("Priority: high"));
    assert!(stdout.contains("keep-me"));
    assert!(stdout.contains(&target_id));

    let source_text = read_canonical_record(dir.path(), "issues", &source_id);
    assert!(!source_text.contains("- \"remove-me\""));
}

// ==================== Dependencies Tests ====================

#[test]
fn test_block_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "block", "1", "2"]);

    assert!(success);

    let (_, blocked_out, _) = run_atelier(dir.path(), &["issue", "blocked"]);
    assert!(blocked_out.contains("Blocked issue"));
}

#[test]
fn test_issue_list_blocked_replaces_blocked_helper() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    let blocked_id = issue_ref(dir.path(), 1);
    let blocker_id = issue_ref(dir.path(), 2);
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "block", &blocked_id, &blocker_id]);
    assert!(success, "issue block failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--blocked"]);
    assert!(success, "issue list --blocked failed: {stderr}");
    assert!(stdout.contains("Blocked issue"));
    assert!(stdout.contains(&blocker_id));
}

#[test]
fn test_unblock_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    run_atelier(dir.path(), &["issue", "block", "1", "2"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "unblock", "1", "2"]);

    assert!(success);

    let (_, blocked_out, _) = run_atelier(dir.path(), &["issue", "blocked"]);
    assert!(!blocked_out.contains("Blocked issue"));
}

#[test]
fn test_issue_blocker_mutations_are_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Alias blocked"]);
    assert!(success, "blocked create failed: {stderr}");
    let blocked_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Alias blocker"]);
    assert!(success, "blocker create failed: {stderr}");
    let blocker_id = issue_ref(dir.path(), 2);

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "block", &blocked_id, &blocker_id]);
    assert!(success, "issue block failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after issue block: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after issue block failed: {stderr}");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "blocked", &blocked_id]);
    assert!(success, "issue blocked after issue block failed: {stderr}");
    assert!(stdout.contains(&blocker_id), "{stdout}");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "unblock", &blocked_id, &blocker_id]);
    assert!(success, "issue unblock failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after issue unblock: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after issue unblock failed: {stderr}");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "blocked", &blocked_id]);
    assert!(
        success,
        "issue blocked after issue unblock failed: {stderr}"
    );
    assert!(
        stdout.contains("No dependencies found."),
        "dependency should be removed after rebuild: {stdout}"
    );
}

#[test]
fn test_ready_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    run_atelier(dir.path(), &["issue", "create", "Ready issue"]);
    run_atelier(dir.path(), &["issue", "block", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list", "--ready"]);

    assert!(success);
    assert!(stdout.contains("2 total"));
    assert!(stdout.contains("Ready issue"));
    assert!(stdout.contains("Blocker issue")); // Blocker is also ready
    assert!(!stdout.contains("Blocked issue"));
}

#[test]
fn test_issue_ready_command_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "ready"]);

    assert!(!success);
    assert!(
        stderr.contains("unrecognized subcommand") || stderr.contains("unexpected argument"),
        "expected clap unknown command error, got: {stderr}"
    );
}

#[test]
fn test_quiet_issue_list_ready_outputs_ids_only() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Ready issue"]);

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["--quiet", "issue", "list", "--ready"]);

    assert!(success, "quiet ready list failed: {stderr}");
    assert_eq!(stdout.lines().count(), 1);
    assert!(stdout.lines().all(|line| line.starts_with("atelier-")));
    assert!(!stdout.contains("Ready issue"));
}

#[test]
fn test_issue_list_ready_rejects_closed_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _stderr) = run_atelier(
        dir.path(),
        &["issue", "list", "--ready", "--status", "closed"],
    );

    assert!(!success);
}

#[test]
fn test_issue_list_ready_treats_internal_epic_blockers_as_ready() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "epic"],
    );
    run_atelier(dir.path(), &["issue", "subissue", "1", "Ready child"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Sequenced child"]);
    run_atelier(dir.path(), &["issue", "block", "3", "2"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);

    assert!(success, "ready list failed: {stderr}");
    assert!(stdout.contains("Parent epic"));
    assert!(stdout.contains("Ready child"));
    assert!(!stdout.contains("Sequenced child"));
}

#[test]
fn test_issue_list_ready_marks_blocked_parent_headers_as_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Blocked parent epic",
            "--issue-type",
            "epic",
        ],
    );
    run_atelier(dir.path(), &["issue", "subissue", "1", "Ready child"]);
    run_atelier(dir.path(), &["issue", "create", "Outside blocker"]);
    let parent_id = issue_ref(dir.path(), 1);
    let child_id = issue_ref(dir.path(), 2);
    let blocker_id = issue_ref(dir.path(), 3);
    run_atelier(dir.path(), &["issue", "block", &parent_id, &blocker_id]);

    let (success, ready_out, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "ready list failed: {stderr}");
    assert!(
        ready_out.contains("Blocked parent epic (context; parent blocked)"),
        "{ready_out}"
    );
    assert!(ready_out.contains(&format!("blocked by {blocker_id}")));
    assert!(ready_out.contains(&format!("{child_id} - Ready child")));

    let (success, blocked_out, stderr) = run_atelier(dir.path(), &["issue", "blocked", &parent_id]);
    assert!(success, "blocked detail failed: {stderr}");
    assert!(blocked_out.contains(&blocker_id), "{blocked_out}");
}

#[test]
fn test_issue_list_marks_external_epic_blockers_by_id() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "epic"],
    );
    run_atelier(dir.path(), &["issue", "subissue", "1", "Blocked child"]);
    run_atelier(dir.path(), &["issue", "create", "Outside blocker"]);
    let blocker_id = issue_ref(dir.path(), 3);
    run_atelier(dir.path(), &["issue", "block", "2", "3"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list"]);

    assert!(success, "issue list failed: {stderr}");
    assert!(stdout.contains("Parent epic"));
    assert!(stdout.contains(&format!("blocked by {blocker_id}")));
    assert!(!stdout.contains("open blocker"));
}

#[test]
fn test_issue_update_issue_type_persists_through_rebuild() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Container work"]);

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", "1", "--issue-type", "epic"],
    );
    assert!(success, "issue type update failed: {stderr}");
    assert!(stdout.contains("Type:     epic"));

    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Container work"));
    assert!(stdout.contains("Type:     epic"));
}

#[test]
fn test_removed_issue_type_is_rejected() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Artifact task",
            "--issue-type",
            "decision",
        ],
    );

    assert!(!success, "removed issue type should be rejected");
    assert!(stderr.contains("Invalid issue_type 'decision'"));
}

// ==================== Session Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_start() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["session", "start"]);

    assert!(success);
    assert!(stdout.contains("Session") || stdout.contains("started"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["session", "start"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["session", "status"]);

    assert!(success);
    assert!(stdout.contains("Session") || stdout.contains("active"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Working issue"]);
    run_atelier(dir.path(), &["session", "start"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["session", "work", "1"]);

    assert!(success);
    assert!(stdout.contains("Working") || stdout.contains("#1"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_end() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["session", "start"]);
    let (success, stdout, _) =
        run_atelier(dir.path(), &["session", "end", "--notes", "Finished work"]);

    assert!(success);
    assert!(stdout.contains("ended") || stdout.contains("Session"));
}
